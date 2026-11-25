//! Staging module oc691 (wave wave237): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc691`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0039_5730,
        ps2_graphics_man_pop_matrix,
        "PS2GraphicsMan_PopMatrix",
    ),
    (
        0x0039_5c68,
        ps2_graphics_man_get_matrix,
        "PS2GraphicsMan_GetMatrix",
    ),
    (0x003b_4dd8, snd_get_config, "SND_GetConfig"),
    (0x003b_4fc0, snd_set_config, "SND_SetConfig"),
    (0x003b_5158, snd_init, "SND_Init"),
    (
        0x003b_5478,
        snd_start_timer_service,
        "SND_StartTimerService",
    ),
    (
        0x003b_5948,
        snd_remove_tick_callback,
        "SND_RemoveTickCallback",
    ),
    (0x003b_5b20, snd_spu_mem_set_base, "SND_SpuMemSetBase"),
    (0x003b_5b68, snd_spu_mem_get_free, "SND_SpuMemGetFree"),
    (0x003b_6098, snd_bank_add, "SND_BankAdd"),
    (0x003b_6300, snd_bank_remove, "SND_BankRemove"),
    (0x003b_6548, snd_bank_move, "SND_BankMove"),
];

/// `SND` global state (`sndgs`) at 0x50a8e8 (`0x510000 - 0x5718`).
#[cfg(test)]
pub(crate) const OC691_SND_STATE: u32 = 0x0050_a8e8;

/// A direct call to a generated function returning to `ret`.
fn oc691_call(c: &mut Cpu, addr: u32, f: GuestFn, ret: u32) -> Flow {
    Callee { addr, f }.call(c, ret)
}

/// `PS2GraphicsMan::PopMatrix(this)` (0x395730): drop the top matrix of the
/// view stack (`top -= 0x40`, `depth -= 1`) and clear the stack-dirty word.
pub fn ps2_graphics_man_pop_matrix(c: &mut Cpu) {
    c.cycles += 8;
    let this = abi_a0(c);
    c.g[V0][0] = Addr(this).off(0x13e4).lw(c);
    c.g[V1][0] = Addr(this).off(0x13e0).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x40));
    Addr(this).off(0x6b90).sw(c, 0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
    let v0 = c.g[V0][0];
    Addr(this).off(0x13e4).sw(c, v0);
    let t = c.g[RA][0] as u32;
    let v1 = c.g[V1][0];
    Addr(this).off(0x13e0).sw(c, v1);
    c.pc = t;
}

/// `float* PS2GraphicsMan::GetMatrix(this)` (0x395c68): the current top
/// matrix of the view stack.
pub fn ps2_graphics_man_get_matrix(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = Addr(abi_a0(c)).off(0x13e4).lw(c);
    c.pc = t;
}

#[inline]
fn abi_a0(c: &Cpu) -> u32 {
    c.g[A0][0] as u32
}

/// `int SND_GetConfig(SndOptions* out)` (0x3b4dd8): on the first call load the
/// defaults (`SND_SetDefaultConfig`), remember the platform capabilities word
/// (0x44c460) and save a copy of the settings block (0x50a908..0x50a9a8 to
/// +0xd4); then copy the 0xd4-byte options record at the start of the SND
/// state to `out` (`ld/sd` when both sides are 8-aligned, else `ldl/ldr`).
/// Returns the capabilities word.
pub fn snd_get_config(c: &mut Cpu) {
    guest(c, |c| {
        'b_3b4ee0: {
            'b_3b4edc: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S0][0] = 0x450000;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = 0x510000;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S2][0] = c.g[A0][0];
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x3ba4));
                c.g[S3][0] = 0x450000;
                let t = c.g[V0][0] != 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_3b4edc;
                } else {
                    'b_3b4eac: {
                        c.cycles += 19;
                        oc691_call(c, 0x003c0c90, ee::s13::SND_SetDefaultConfig, 0x003b4e10)?;
                        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x5718));
                        c.sw((c.g[S3][0] as u32).wrapping_sub(0x3ba0), c.g[V0][0]);
                        c.g[A1][0] = 0x1;
                        c.g[AT][0] = 0xffffffffbf800000;
                        c.mtc1(1, 0);
                        c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xd4));
                        c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        c.sw((c.g[S0][0] as u32).wrapping_sub(0x3ba4), c.g[A1][0]);
                        c.g[V1][0] = 0x10;
                        c.g[A0][0] = 0x5a;
                        c.g[V0][0] = c.g[A3][0] | c.g[T0][0];
                        c.sh((c.g[A2][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                        c.g[V0][0] = c.g[V0][0] & 0x7;
                        c.sb((c.g[A2][0] as u32).wrapping_add(0x4b), c.g[A0][0]);
                        c.sb((c.g[A2][0] as u32).wrapping_add(0x3c), c.g[A1][0]);
                        let t = c.g[V0][0] == 0u64;
                        c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x34));
                        if t {
                            break 'b_3b4eac;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc0));
                            'l_3b4e58: loop {
                                c.cycles += 19;
                                c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                                c.ldr(3, (c.g[A3][0] as u32));
                                c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0xf));
                                c.ldr(4, (c.g[A3][0] as u32).wrapping_add(0x8));
                                c.ldl(5, (c.g[A3][0] as u32).wrapping_add(0x17));
                                c.ldr(5, (c.g[A3][0] as u32).wrapping_add(0x10));
                                c.ldl(6, (c.g[A3][0] as u32).wrapping_add(0x1f));
                                c.ldr(6, (c.g[A3][0] as u32).wrapping_add(0x18));
                                c.sdl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
                                c.sdr(3, (c.g[T0][0] as u32));
                                c.sdl(4, (c.g[T0][0] as u32).wrapping_add(0xf));
                                c.sdr(4, (c.g[T0][0] as u32).wrapping_add(0x8));
                                c.sdl(5, (c.g[T0][0] as u32).wrapping_add(0x17));
                                c.sdr(5, (c.g[T0][0] as u32).wrapping_add(0x10));
                                c.sdl(6, (c.g[T0][0] as u32).wrapping_add(0x1f));
                                c.sdr(6, (c.g[T0][0] as u32).wrapping_add(0x18));
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                                let t = c.g[A3][0] != c.g[V0][0];
                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003b4e58);
                                    }
                                    continue 'l_3b4e58;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A0][0] = c.g[S2][0];
                                    if t {
                                        break 'b_3b4ee0;
                                    } else {
                                        break 'b_3b4eac;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc0));
                    'l_3b4eb0: loop {
                        c.cycles += 11;
                        c.g[V1][0] = c.ld(c.g[A3][0] as u32);
                        c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                        c.g[A2][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                        c.sd(c.g[T0][0] as u32, c.g[V1][0]);
                        c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                        c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                        c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A2][0]);
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        let t = c.g[A3][0] != c.g[V0][0];
                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x003b4eb0);
                            }
                            continue 'l_3b4eb0;
                        } else {
                            break 'b_3b4edc;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[S2][0];
            break 'b_3b4ee0;
        }
        'b_3b4f74: {
            'b_3b4f48: {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x5718));
                c.g[V0][0] = c.g[A0][0] | c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc0));
                if t {
                    break 'b_3b4f48;
                } else {
                    'l_3b4ef4: loop {
                        c.cycles += 19;
                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(7, (c.g[V1][0] as u32));
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                        c.sdr(7, (c.g[A0][0] as u32));
                        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0xf));
                        c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                        c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x17));
                        c.sdr(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                        c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x1f));
                        c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x003b4ef4);
                            }
                            continue 'l_3b4ef4;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_3b4f74;
                            } else {
                                break 'b_3b4f48;
                            }
                        }
                    }
                }
            }
            'l_3b4f48: loop {
                c.cycles += 11;
                c.g[A3][0] = c.ld(c.g[V1][0] as u32);
                c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A0][0] as u32, c.g[A3][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A2][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x003b4f48);
                    }
                    continue 'l_3b4f48;
                } else {
                    break 'b_3b4f74;
                }
            }
        }
        c.cycles += 18;
        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.ldr(7, (c.g[V1][0] as u32));
        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
        c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
        c.sdr(7, (c.g[A0][0] as u32));
        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0xf));
        c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x8));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x3ba0));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `int SND_SetConfig(const SndOptions* cfg)` (0x3b4fc0): copy the settings
/// block (`cfg+0x20..+0xc0`) and the next 0x14 bytes into the SND state, apply
/// them (`SND_ApplyConfig`) and save a copy of the applied settings (to
/// state+0xd4). Returns 0.
pub fn snd_set_config(c: &mut Cpu) {
    guest(c, |c| {
        'b_3b5074: {
            'b_3b5040: {
                c.cycles += 9;
                c.g[V0][0] = 0x510000;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x56f8));
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.g[V0][0] = c.g[V1][0] | c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_3b5040;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc0));
                    c.g[A2][0] = 0x510000;
                    'l_3b4fec: loop {
                        c.cycles += 19;
                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(7, (c.g[V1][0] as u32));
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(10, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(10, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0x7));
                        c.sdr(7, (c.g[A1][0] as u32));
                        c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0xf));
                        c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x17));
                        c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x10));
                        c.sdl(10, (c.g[A1][0] as u32).wrapping_add(0x1f));
                        c.sdr(10, (c.g[A1][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x003b4fec);
                            }
                            continue 'l_3b4fec;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_3b5074;
                            } else {
                                break 'b_3b5040;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc0));
            c.g[A2][0] = 0x510000;
            'l_3b5048: loop {
                c.cycles += 11;
                c.g[A3][0] = c.ld(c.g[V1][0] as u32);
                c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[T2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A1][0] as u32, c.g[A3][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T2][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x003b5048);
                    }
                    continue 'l_3b5048;
                } else {
                    break 'b_3b5074;
                }
            }
        }
        'b_3b5144: {
            'b_3b5114: {
                c.cycles += 19;
                c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5658));
                c.ldl(2, (c.g[A0][0] as u32).wrapping_add(0xc7));
                c.ldr(2, (c.g[A0][0] as u32).wrapping_add(0xc0));
                c.ldl(3, (c.g[A0][0] as u32).wrapping_add(0xcf));
                c.ldr(3, (c.g[A0][0] as u32).wrapping_add(0xc8));
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xd0));
                c.sdl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.sdr(2, (c.g[A3][0] as u32));
                c.sdl(3, (c.g[A3][0] as u32).wrapping_add(0xf));
                c.sdr(3, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.sw((c.g[A3][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5658));
                oc691_call(c, 0x003c0e28, ee::s13::SND_ApplyConfig, 0x003b50a8)?;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xa0));
                c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;

                if t {
                    break 'b_3b5114;
                } else {
                    'l_3b50c0: loop {
                        c.cycles += 19;
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(8, (c.g[V1][0] as u32));
                        c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(10, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(10, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(2, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                        c.sdr(8, (c.g[A0][0] as u32));
                        c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0xf));
                        c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x8));
                        c.sdl(10, (c.g[A0][0] as u32).wrapping_add(0x17));
                        c.sdr(10, (c.g[A0][0] as u32).wrapping_add(0x10));
                        c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x1f));
                        c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[S0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x003b50c0);
                            }
                            continue 'l_3b50c0;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                            if t {
                                break 'b_3b5144;
                            } else {
                                break 'b_3b5114;
                            }
                        }
                    }
                }
            }
            'l_3b5114: loop {
                c.cycles += 11;
                c.g[A1][0] = c.ld(c.g[V1][0] as u32);
                c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A0][0] as u32, c.g[A1][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[S0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x003b5114);
                    }
                    continue 'l_3b5114;
                } else {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    break 'b_3b5144;
                }
            }
        }
        c.cycles += 4;
        c.g[V0][0] = 0u64;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `int SND_Init(void* heap, int size)` (0x3b5158): initialise the sound
/// library once: clear and set up the heap, load the default configuration if
/// no voices are configured, seed the random generator, create the mutex,
/// allocate the channel and bank tables, bring up the IOP side and build the
/// pan tables. Returns 0 (also when already installed) or the failing
/// configuration / IOP result.
pub fn snd_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_3b5304: {
            'b_3b5300: {
                'b_3b52fc: {
                    c.cycles += 16;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.g[V0][0] = 0x510000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[A2][0] = 0x450000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = 0x53;
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S3][0] = c.g[V0][0];
                    c.g[S1][0] = c.g[A1][0];
                    c.g[V0][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x174));
                    let t = c.g[V0][0] != 0u64;
                    c.sb((c.g[A2][0] as u32).wrapping_sub(0x1e80), c.g[V1][0]);
                    if t {
                        break 'b_3b52fc;
                    } else {
                        'b_3b51e0: {
                            c.cycles += 10;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = c.g[S1][0];
                            call_ptr(c, 0x00416210, 0x003b51a8)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[S1][0];
                            oc691_call(c, 0x003b5bd0, ee::s13::SND_MemInitHeap, 0x003b51b4)?;
                            c.g[V0][0] = c.lh((c.g[S2][0] as u32).wrapping_add(0x17e));
                            let t = c.g[V0][0] != 0u64;
                            c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5718));
                            if t {
                                break 'b_3b51e0;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S2][0];
                                oc691_call(c, 0x003b4dd8, ee::s13::SND_GetConfig, 0x003b51c8)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = (c.g[S0][0] as i64) < 0;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_3b5304;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.g[S2][0];
                                    oc691_call(c, 0x003b4fc0, ee::s13::SND_SetConfig, 0x003b51dc)?;
                                    c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5718));
                                    break 'b_3b51e0;
                                }
                            }
                        }
                        'b_3b526c: {
                            c.cycles += 28;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                            oc691_call(c, 0x003bc588, ee::s13::SND_RandomSeed, 0x003b51e8)?;
                            oc691_call(c, 0x003c3300, ee::s13::SND_MutexInit, 0x003b51f0)?;
                            c.g[V0][0] = 0x1;
                            c.sb((c.g[S1][0] as u32).wrapping_add(0x174), c.g[V0][0]);
                            oc691_call(c, 0x003b58a0, ee::s13::SND_Lock, 0x003b51fc)?;
                            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x17e));
                            c.g[A0][0] = 0x8c;
                            c.mult(0, 4, 2, 4);
                            oc691_call(c, 0x003b5c60, ee::s13::SND_MemAlloc, 0x003b520c)?;
                            c.g[A0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x28));
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x1f0), c.g[V0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
                            oc691_call(c, 0x003b5c60, ee::s13::SND_MemAlloc, 0x003b521c)?;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
                            oc691_call(c, 0x003b58d8, ee::s13::SND_Unlock, 0x003b5224)?;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x330), 0u64);
                            c.g[V0][0] = 0x7f;
                            c.sb((c.g[S1][0] as u32).wrapping_add(0x175), c.g[V0][0]);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x180), 0u64);
                            c.sb((c.g[S1][0] as u32).wrapping_add(0x179), 0u64);
                            c.sb((c.g[S1][0] as u32).wrapping_add(0x177), 0u64);
                            c.sb((c.g[S1][0] as u32).wrapping_add(0x178), 0u64);
                            oc691_call(c, 0x003c1b80, ee::s13::SND_IopInit, 0x003b5244)?;
                            c.g[S0][0] = c.g[V0][0];
                            if (c.g[S0][0] as i64) >= 0 {
                                c.g[V1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x47));
                                break 'b_3b526c;
                            } else {
                                c.cycles += 7;
                                oc691_call(c, 0x003c2268, ee::s13::SND_IopShutdown, 0x003b5258)?;
                                oc691_call(c, 0x003c3358, ee::s13::SND_MutexDelete, 0x003b5260)?;
                                c.sb((c.g[S1][0] as u32).wrapping_add(0x174), 0u64);
                                let t = true;
                                c.g[V0][0] = c.g[S0][0];
                                if t {
                                    break 'b_3b5300;
                                } else {
                                    break 'b_3b526c;
                                }
                            }
                        }
                        'b_3b5288: {
                            'b_3b5284: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x2;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[V1][0] = 0xe000;
                                if t {
                                    break 'b_3b5284;
                                } else {
                                    c.cycles += 3;
                                    c.g[V1][0] = 0xc000;
                                    let t = true;
                                    c.g[V0][0] = 0x4000;
                                    if t {
                                        break 'b_3b5288;
                                    } else {
                                        break 'b_3b5284;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = 0x2000;
                            break 'b_3b5288;
                        }
                        c.cycles += 29;
                        c.sh((c.g[S1][0] as u32).wrapping_add(0x2ee), c.g[V0][0]);
                        c.sh((c.g[S1][0] as u32).wrapping_add(0x2ec), c.g[V1][0]);
                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5718));
                        c.g[A2][0] = 0xe000;
                        c.g[T1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x94));
                        c.g[A3][0] = 0x2000;
                        c.g[T0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x90));
                        c.g[A1][0] = 0xa000;
                        c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x92));
                        c.g[A0][0] = 0x6000;
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x2f8), c.g[T1][0]);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x2fa), c.g[T0][0]);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x2fc), c.g[V1][0]);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x31c), c.g[A2][0]);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x320), c.g[A3][0]);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x322), c.g[A1][0]);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x324), c.g[A0][0]);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x326), 0u64);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x304), c.g[A2][0]);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x306), c.g[A3][0]);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x308), c.g[A1][0]);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x30a), c.g[A0][0]);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x310), c.g[A2][0]);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x312), 0u64);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x314), c.g[A3][0]);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x316), c.g[A1][0]);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x318), c.g[A0][0]);
                        c.sh((c.g[V0][0] as u32).wrapping_add(0x31e), 0u64);
                        oc691_call(c, 0x003bc0b0, ee::s13::SND_InitPanTables, 0x003b52fc)?;
                        break 'b_3b52fc;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                break 'b_3b5300;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_3b5304;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `int SND_StartTimerService(void)` (0x3b5478): install the abort handler,
/// start the system task once and register the exit handler; returns 0.
pub fn snd_start_timer_service(c: &mut Cpu) {
    guest(c, |c| {
        'b_3b54bc: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.g[V0][0] = 0x3b0000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5470));
            c.g[S0][0] = 0x450000;
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x3b9c));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            let t = c.g[A0][0] != 0u64;
            c.sw((c.g[V1][0] as u32).wrapping_sub(0x5650), c.g[V0][0]);
            if t {
                break 'b_3b54bc;
            } else {
                c.cycles += 7;
                c.g[A0][0] = 0x3b0000;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5450));
                c.g[A2][0] = 0x1;
                call_ptr(c, 0x003e57f8, 0x003b54b4)?;
                c.g[V0][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_sub(0x3b9c), c.g[V0][0]);
                break 'b_3b54bc;
            }
        }
        c.cycles += 8;
        c.g[A0][0] = 0x3c0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x36f0));
        call_ptr(c, 0x003e4a30, 0x003b54c8)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = 0u64;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `void SND_RemoveTickCallback(void (*cb)())` (0x3b5948): remove `cb` from
/// the per-tick callback table (under the sound lock), shifting the rest down.
pub fn snd_remove_tick_callback(c: &mut Cpu) {
    guest(c, |c| {
        'b_3b5a0c: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = 0x510000;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5718));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x174));
            let t = c.g[V0][0] == 0u64;
            c.g[S1][0] = c.g[A0][0];
            if t {
                break 'b_3b5a0c;
            } else {
                'b_3b5a04: {
                    c.cycles += 5;
                    oc691_call(c, 0x003b58a0, ee::s13::SND_Lock, 0x003b5978)?;
                    c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x177));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[A0][0] = 0u64;
                    if t {
                        break 'b_3b5a04;
                    } else {
                        c.cycles += 5;
                        c.g[A2][0] = c.g[S0][0];
                        c.g[T0][0] = c.g[S2][0];
                        c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x184));
                        c.g[A1][0] = sx((c.g[A0][0] as u32) << 2);
                        'l_3b5998: loop {
                            'b_3b59f4: {
                                c.cycles += 4;
                                c.g[V0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                if c.g[V1][0] != c.g[S1][0] {
                                    c.g[V0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x177));
                                    break 'b_3b59f4;
                                } else {
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lbu((c.g[A2][0] as u32).wrapping_add(0x177));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 24);
                                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
                                    c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    let t = c.g[V1][0] == 0u64;
                                    c.sb((c.g[A2][0] as u32).wrapping_add(0x177), c.g[V0][0]);
                                    if t {
                                        break 'b_3b5a04;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x5718));
                                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x184));
                                        c.g[V1][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                        'l_3b59d0: loop {
                                            c.cycles += 7;
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                                            c.g[V0][0] =
                                                c.lb((c.g[A2][0] as u32).wrapping_add(0x177));
                                            c.g[V0][0] =
                                                ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x003b59d0);
                                                }
                                                continue 'l_3b59d0;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;

                                                if t {
                                                    break 'b_3b5a04;
                                                } else {
                                                    break 'b_3b59f4;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A1][0] = sx((c.g[A0][0] as u32) << 2);
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003b5998);
                                }
                                continue 'l_3b5998;
                            } else {
                                break 'b_3b5a04;
                            }
                        }
                    }
                }
                c.cycles += 2;
                oc691_call(c, 0x003b58d8, ee::s13::SND_Unlock, 0x003b5a0c)?;
                break 'b_3b5a0c;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `int SND_SpuMemSetBase(int start, int end)` (0x3b5b20): `SNDI_SpuMemSetBase`
/// under the sound lock.
pub fn snd_spu_mem_set_base(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        oc691_call(c, 0x003b58a0, ee::s13::SND_Lock, 0x003b5b3c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        oc691_call(c, 0x003c4e50, ee::s13::SNDI_SpuMemSetBase, 0x003b5b48)?;
        c.g[S0][0] = c.g[V0][0];
        oc691_call(c, 0x003b58d8, ee::s13::SND_Unlock, 0x003b5b50)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `int SND_SpuMemGetFree(int* largest)` (0x3b5b68): `SNDI_SpuMemGetFree`
/// under the sound lock.
pub fn snd_spu_mem_get_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        oc691_call(c, 0x003b58a0, ee::s13::SND_Lock, 0x003b5b7c)?;
        c.g[A0][0] = c.g[S0][0];
        oc691_call(c, 0x003c4ec8, ee::s13::SNDI_SpuMemGetFree, 0x003b5b84)?;
        c.g[S0][0] = c.g[V0][0];
        oc691_call(c, 0x003b58d8, ee::s13::SND_Unlock, 0x003b5b8c)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `int SND_BankAdd(int* handle_out, SndBankHeader* hdr)` (0x3b6098): find a
/// free bank slot, upload the IOP and SPU parts of the bank (0x1000-byte DMA
/// chunks, polled to completion) and relocate their patches. Returns 7 or 8
/// (8 when the bank has neither part) or -6 when an SPU allocation fails.
pub fn snd_bank_add(c: &mut Cpu) {
    guest(c, |c| {
        'b_3b62dc: {
            'b_3b62bc: {
                'b_3b61fc: {
                    'b_3b61ec: {
                        'b_3b61cc: {
                            c.cycles += 26;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                            c.g[V0][0] = 0x3b0000;
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6300));
                            c.g[S0][0] = 0x510000;
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5718));
                            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[S2][0] = c.g[A1][0];
                            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.g[S1][0] = c.g[A0][0];
                            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x1e4), c.g[V0][0]);
                            oc691_call(c, 0x003c6500, ee::s13::SND_BankFindFreeSlot, 0x003b60d4)?;
                            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                            c.g[A0][0] = 0x1;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                            c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.sb((c.g[S4][0] as u32).wrapping_add(0xc), c.g[A0][0]);
                            c.sw(c.g[S4][0] as u32, c.g[S2][0]);
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            let t = c.g[A1][0] == 0u64;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_3b61cc;
                            } else {
                                c.cycles += 4;
                                c.g[A0][0] = 0x100;
                                oc691_call(c, 0x003c4b70, ee::s13::SND_SpuMemAlloc, 0x003b6108)?;
                                let t = c.g[V0][0] == 0u64;
                                c.sw((c.g[S4][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                if t {
                                    break 'b_3b61ec;
                                } else {
                                    'b_3b617c: {
                                        c.cycles += 3;
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        let t = (c.g[V1][0] as i64) <= 0;
                                        c.g[S0][0] = 0u64;
                                        if t {
                                            break 'b_3b617c;
                                        } else {
                                            c.cycles += 3;
                                            c.g[S3][0] = 0x1000;
                                            c.g[A0][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_sub(c.g[S0][0] as u32));
                                            'l_3b6128: loop {
                                                c.cycles += 12;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                c.g[V0][0] = ((c.g[A0][0] as i64) < 4097i64) as u64;
                                                c.g[A2][0] =
                                                    c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                                                c.g[A3][0] = c.g[S3][0];
                                                c.g[A1][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add(c.g[A1][0] as u32));
                                                if c.g[V0][0] != 0 {
                                                    c.g[A3][0] = c.g[A0][0];
                                                }
                                                c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                    .wrapping_add(c.g[S0][0] as u32));
                                                c.g[A2][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[S0][0] as u32));
                                                c.g[A0][0] = 0x100;
                                                c.g[S1][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1000));
                                                oc691_call(
                                                    c,
                                                    0x003c42d8,
                                                    ee::s13::SND_IopUpload,
                                                    0x003b6154,
                                                )?;
                                                c.g[S0][0] = c.g[V0][0];
                                                'l_3b6158: loop {
                                                    c.cycles += 4;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    oc691_call(
                                                        c,
                                                        0x003c42f8,
                                                        ee::s13::SND_IopUploadWait,
                                                        0x003b6160,
                                                    )?;
                                                    let t = c.g[V0][0] == 0u64;

                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x003b6158);
                                                        }
                                                        continue 'l_3b6158;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[S0][0] = c.g[S1][0];
                                                        c.g[V0][0] = ((c.g[S1][0] as i64)
                                                            < (c.g[V1][0] as i64))
                                                            as u64;
                                                        if c.g[V0][0] != 0u64 {
                                                            c.g[A0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(c.g[S0][0] as u32));
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x003b6128);
                                                            }
                                                            continue 'l_3b6128;
                                                        } else {
                                                            break 'b_3b617c;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0x6));
                                    c.g[S0][0] = 0u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.sw(c.g[SP][0] as u32, 0u64);
                                    if t {
                                        break 'b_3b61cc;
                                    } else {
                                        c.cycles += 3;
                                        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x14));
                                        c.g[S3][0] = c.g[S1][0];
                                        'l_3b6198: loop {
                                            'b_3b61b4: {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] = sx((c.g[S3][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                if t {
                                                    break 'b_3b61b4;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[A2][0] =
                                                        c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                                                    c.g[A0][0] = 0x100;
                                                    c.g[A3][0] = c.g[SP][0];
                                                    oc691_call(
                                                        c,
                                                        0x003b6788,
                                                        ee::s13::SND_BankRelocatePatches,
                                                        0x003b61b4,
                                                    )?;
                                                    break 'b_3b61b4;
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[V0][0] =
                                                c.lhu((c.g[S2][0] as u32).wrapping_add(0x6));
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x003b6198);
                                                }
                                                continue 'l_3b6198;
                                            } else {
                                                break 'b_3b61cc;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 4;
                        c.sw((c.g[S4][0] as u32).wrapping_add(0x8), 0u64);
                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                        let t = c.g[A1][0] == 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_3b62bc;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = 0x8;
                            oc691_call(c, 0x003c4b70, ee::s13::SND_SpuMemAlloc, 0x003b61e4)?;
                            let t = c.g[V0][0] != 0u64;
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            if t {
                                break 'b_3b61fc;
                            } else {
                                break 'b_3b61ec;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.sb((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[V0][0] = 0xfffffffffffffffa;
                    let t = true;
                    c.sw(c.g[S4][0] as u32, 0u64);
                    if t {
                        break 'b_3b62dc;
                    } else {
                        break 'b_3b61fc;
                    }
                }
                'b_3b626c: {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.g[S0][0] = 0u64;
                    if t {
                        break 'b_3b626c;
                    } else {
                        c.cycles += 2;
                        c.g[S3][0] = 0x1000;
                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        'l_3b6210: loop {
                            c.cycles += 14;
                            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[V1][0] = ((c.g[A0][0] as i64) < 4097i64) as u64;
                            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                            c.g[A3][0] = c.g[S3][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            if c.g[V1][0] != 0 {
                                c.g[A3][0] = c.g[A0][0];
                            }
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[A0][0] = 0x8;
                            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1000));
                            oc691_call(c, 0x003c42d8, ee::s13::SND_IopUpload, 0x003b6244)?;
                            c.g[S0][0] = c.g[V0][0];
                            'l_3b6248: loop {
                                c.cycles += 4;
                                c.g[A0][0] = c.g[S0][0];
                                oc691_call(c, 0x003c42f8, ee::s13::SND_IopUploadWait, 0x003b6250)?;
                                let t = c.g[V0][0] == 0u64;

                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003b6248);
                                    }
                                    continue 'l_3b6248;
                                } else {
                                    c.cycles += 5;
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                    c.g[S0][0] = c.g[S1][0];
                                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003b6210);
                                        }
                                        continue 'l_3b6210;
                                    } else {
                                        break 'b_3b626c;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 4;
                c.g[V0][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0x6));
                c.g[S0][0] = 0u64;
                let t = c.g[V0][0] == 0u64;
                c.sw(c.g[SP][0] as u32, 0u64);
                if t {
                    break 'b_3b62bc;
                } else {
                    c.cycles += 3;
                    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x14));
                    c.g[S3][0] = c.g[S1][0];
                    'l_3b6288: loop {
                        'b_3b62a4: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            if t {
                                break 'b_3b62a4;
                            } else {
                                c.cycles += 4;
                                c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = 0x8;
                                c.g[A3][0] = c.g[SP][0];
                                oc691_call(
                                    c,
                                    0x003b6788,
                                    ee::s13::SND_BankRelocatePatches,
                                    0x003b62a4,
                                )?;
                                break 'b_3b62a4;
                            }
                        }
                        c.cycles += 6;
                        c.g[V0][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0x6));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x003b6288);
                            }
                            continue 'l_3b6288;
                        } else {
                            break 'b_3b62bc;
                        }
                    }
                }
            }
            c.cycles += 4;
            c.sb((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0x7;
            if t {
                break 'b_3b62dc;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x7;
                if t {
                    break 'b_3b62dc;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 0x8;
                    break 'b_3b62dc;
                }
            }
        }
        c.cycles += 8;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `int SND_BankRemove(int handle)` (0x3b6300): with -1 remove every bank;
/// otherwise validate `handle`, stop the voices playing from the bank, run
/// the user-data free callbacks of its patches, free its IOP and SPU memory
/// and clear the slot. Returns 0 or the validation error (-8).
pub fn snd_bank_remove(c: &mut Cpu) {
    guest(c, |c| {
        'b_3b64b4: {
            'b_3b64b0: {
                'b_3b64ac: {
                    'b_3b636c: {
                        c.cycles += 12;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x160));
                        c.g[V0][0] = 0xffffffffffffffff;
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                        c.g[S2][0] = c.g[A0][0];
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        let t = c.g[S2][0] != c.g[V0][0];
                        c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
                        if t {
                            break 'b_3b636c;
                        } else {
                            c.cycles += 5;
                            c.g[V1][0] = 0x510000;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
                            c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x28));
                            let t = c.g[V0][0] == 0u64;
                            c.g[S0][0] = 0u64;
                            if t {
                                break 'b_3b64ac;
                            } else {
                                c.cycles += 1;
                                c.g[S1][0] = c.g[V1][0];
                                'l_3b6348: loop {
                                    c.cycles += 7;
                                    c.g[A0][0] = c.g[S0][0];
                                    oc691_call(c, 0x003b6300, ee::s13::SND_BankRemove, 0x003b6350)?;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x28));
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003b6348);
                                        }
                                        continue 'l_3b6348;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                        if t {
                                            break 'b_3b64b4;
                                        } else {
                                            break 'b_3b636c;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    oc691_call(c, 0x003b64d8, ee::s13::SND_BankValidate, 0x003b6374)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0xfffffffffffffff8;
                    if t {
                        break 'b_3b64b0;
                    } else {
                        'b_3b63e8: {
                            c.cycles += 12;
                            c.g[S0][0] = 0u64;
                            oc691_call(c, 0x003b58a0, ee::s13::SND_Lock, 0x003b6384)?;
                            c.g[V1][0] = 0x510000;
                            c.g[V0][0] = sx((c.g[S2][0] as u32) << 4);
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
                            c.g[S6][0] = c.g[V1][0];
                            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                            c.g[S5][0] = c.g[V0][0];
                            c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x17e));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            let t = (c.g[V1][0] as i64) <= 0;
                            c.g[S4][0] = c.lw(c.g[V0][0] as u32);
                            if t {
                                break 'b_3b63e8;
                            } else {
                                c.cycles += 3;
                                c.g[S3][0] = c.g[A1][0];
                                c.g[S1][0] = 0u64;
                                'l_3b63b8: loop {
                                    'b_3b63d8: {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1f0));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                                        c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x12));
                                        if c.g[V1][0] != c.g[S2][0] {
                                            c.g[V0][0] =
                                                c.lh((c.g[S3][0] as u32).wrapping_add(0x17e));
                                            break 'b_3b63d8;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                                            oc691_call(
                                                c,
                                                0x003b89e0,
                                                ee::s13::SND_Stop,
                                                0x003b63d4,
                                            )?;
                                            c.g[V0][0] =
                                                c.lh((c.g[S3][0] as u32).wrapping_add(0x17e));
                                            break 'b_3b63d8;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8c));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003b63b8);
                                        }
                                        continue 'l_3b63b8;
                                    } else {
                                        break 'b_3b63e8;
                                    }
                                }
                            }
                        }
                        'b_3b6454: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lhu((c.g[S4][0] as u32).wrapping_add(0x6));
                            let t = c.g[V0][0] == 0u64;
                            c.g[S0][0] = 0u64;
                            if t {
                                break 'b_3b6454;
                            } else {
                                c.cycles += 1;
                                c.g[A0][0] = c.g[S4][0];
                                'l_3b63f8: loop {
                                    'b_3b6440: {
                                        c.cycles += 6;
                                        c.g[A1][0] = c.g[S0][0];
                                        oc691_call(
                                            c,
                                            0x003c6540,
                                            ee::s13::SND_BankGetPatch,
                                            0x003b6400,
                                        )?;
                                        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                        c.g[V1][0] = c.g[V0][0];
                                        let t = c.g[V1][0] == 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[V0][0]);
                                        if t {
                                            break 'b_3b6440;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                                            c.sw(
                                                (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                c.g[V0][0],
                                            );
                                            c.g[A0][0] = c.g[S1][0];
                                            'l_3b6420: loop {
                                                c.cycles += 8;
                                                c.g[A1][0] = c.g[SP][0];
                                                oc691_call(
                                                    c,
                                                    0x003baac0,
                                                    ee::s13::SND_ParsePatch,
                                                    0x003b6428,
                                                )?;
                                                c.g[S0][0] = c.g[V0][0];
                                                c.g[A0][0] = c.g[SP][0];
                                                c.g[A1][0] = 0x2;
                                                oc691_call(
                                                    c,
                                                    0x003b5f98,
                                                    ee::s13::SND_FireEvents,
                                                    0x003b6438,
                                                )?;
                                                let t = c.g[S0][0] != 0u64;
                                                c.g[A0][0] = c.g[S1][0];
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x003b6420);
                                                    }
                                                    continue 'l_3b6420;
                                                } else {
                                                    break 'b_3b6440;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lhu((c.g[S4][0] as u32).wrapping_add(0x6));
                                    c.g[S0][0] = c.g[S2][0];
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = c.g[S4][0];
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003b63f8);
                                        }
                                        continue 'l_3b63f8;
                                    } else {
                                        break 'b_3b6454;
                                    }
                                }
                            }
                        }
                        'b_3b6478: {
                            c.cycles += 6;
                            c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5718));
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                            c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                            if c.g[A1][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                break 'b_3b6478;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = 0x100;
                                oc691_call(c, 0x003c4d78, ee::s13::SND_SpuMemFree, 0x003b6474)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                break 'b_3b6478;
                            }
                        }
                        'b_3b6494: {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                            if c.g[A1][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                break 'b_3b6494;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = 0x8;
                                oc691_call(c, 0x003c4d78, ee::s13::SND_SpuMemFree, 0x003b6490)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                break 'b_3b6494;
                            }
                        }
                        c.cycles += 6;
                        c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw(c.g[V0][0] as u32, 0u64);
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.sb((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                        oc691_call(c, 0x003b58d8, ee::s13::SND_Unlock, 0x003b64ac)?;
                        break 'b_3b64ac;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                break 'b_3b64b0;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
            break 'b_3b64b4;
        }
        c.cycles += 9;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
        c.pc = t;
        Ok(())
    })
}

/// `int SND_BankMove(void* dest, int handle)` (0x3b6548): copy the bank header
/// into `dest` and point the bank slot at it; returns 0 or the size error.
pub fn snd_bank_move(c: &mut Cpu) {
    guest(c, |c| {
        'b_3b65b0: {
            'b_3b6584: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A1][0];
                c.g[S2][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.g[S1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S3][0] = 0u64;
                oc691_call(c, 0x003b6528, ee::s13::SND_BankGetSize, 0x003b6574)?;
                let t = (c.g[V0][0] as i64) >= 0;
                c.g[S0][0] = 0x510000;
                if t {
                    break 'b_3b6584;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[S3][0] = c.g[V0][0];
                    if t {
                        break 'b_3b65b0;
                    } else {
                        break 'b_3b6584;
                    }
                }
            }
            c.cycles += 11;
            c.g[A2][0] = c.g[V0][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5718));
            c.g[S1][0] = sx((c.g[S1][0] as u32) << 4);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = c.g[S2][0];
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
            call_ptr(c, 0x0041610c, 0x003b65a4)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[S1][0] as u32, c.g[S2][0]);
            break 'b_3b65b0;
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S3][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, fill, scramble, Rng, DATA};

    /// `sndgs` and the words around it that the SND functions read.
    const OC691_STATE: u32 = OC691_SND_STATE;
    /// `SND_Lock` / `SND_Unlock` call through these hooks.
    const OC691_LOCK_HOOK: u32 = 0x0044_c46c;
    const OC691_UNLOCK_HOOK: u32 = 0x0044_c470;
    /// `SND_GetConfig`: "capabilities fetched" flag and the capabilities word.
    const OC691_GOT_CAPS: u32 = 0x0044_c45c;
    const OC691_CAPS: u32 = 0x0044_c460;
    /// `SND_StartTimerService`: "system task added" flag and the exit
    /// handler list (64 words) that `REAL_addexit` fills.
    const OC691_TASK_ADDED: u32 = 0x0044_c464;
    const OC691_EXIT_LIST: u32 = 0x0045_0c78;
    const OC691_EXIT_HANDLER: u32 = 0x003b_c910;
    /// The SPU memory manager state (`0x510000 - 0x5300`).
    const OC691_SPU_STATE: u32 = 0x0050_ad00;

    /// A machine with the fake hooks, the game's gp and the `SND_Lock` /
    /// `SND_Unlock` hooks pointing at fake callees (logged and compared).
    fn oc691_machine(c: &mut Cpu, seed: u32, entry: u32, calls: u32) -> Rng {
        let r = machine(c, seed, entry as u64, calls);
        c.pc = entry;
        c.bus.write32(OC691_LOCK_HOOK, FAKE);
        c.bus.write32(OC691_UNLOCK_HOOK, FAKE + 0x10);
        r
    }

    /// `PopMatrix`: any stack pointer and depth (the arithmetic is 32-bit).
    #[test]
    fn ps2_graphics_man_pop_matrix_matches() {
        check(
            ee::s13::f_00395730__generated,
            ps2_graphics_man_pop_matrix,
            100,
            &|c, seed| {
                let mut r = Rng(0x0039_5730 ^ ((seed as u64) << 8));
                scramble(c, &mut r);
                c.pc = 0x0039_5730;
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x13e4, r.next_u32());
                c.bus.write32(this + 0x13e0, r.next_u32());
                c.bus.write32(this + 0x6b90, r.next_u32());
            },
        );
    }

    /// `GetMatrix`: returns the top-of-stack pointer.
    #[test]
    fn ps2_graphics_man_get_matrix_matches() {
        check(
            ee::s13::f_00395c68__generated,
            ps2_graphics_man_get_matrix,
            100,
            &|c, seed| {
                let mut r = Rng(0x0039_5c68 ^ ((seed as u64) << 8));
                scramble(c, &mut r);
                c.pc = 0x0039_5c68;
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x13e4, r.next_u32());
            },
        );
    }

    /// `SND_GetConfig`: capabilities fetched or not (the defaults are
    /// loaded first), the output buffer 8-aligned or not, events off,
    /// delivered and unwinding inside the copy loops.
    #[test]
    fn snd_get_config_matches() {
        check_events(
            ee::s13::f_003b4dd8__generated,
            snd_get_config,
            200,
            &|c, seed| {
                let mut r = Rng(0x003b_4dd8 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                fill(c, &mut r, OC691_STATE, 0x35);
                c.bus.write32(OC691_GOT_CAPS, (seed / 3) % 2);
                c.bus.write32(OC691_CAPS, r.next_u32());
                let skew = [0u32, 0, 1, 3, 4, 7][(seed / 6 % 6) as usize];
                c.g[A0][0] = sx(DATA + 0x2000 + skew);
                schedule(c, &mut r, seed, 0x003b_4dd8)
            },
        );
    }

    /// `SND_SetConfig`: the configuration 8-aligned or not, the library
    /// installed (voices and channels present) or not, events as above.
    #[test]
    fn snd_set_config_matches() {
        check_events(
            ee::s13::f_003b4fc0__generated,
            snd_set_config,
            200,
            &|c, seed| {
                let mut r = Rng(0x003b_4fc0 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                let skew = [0u32, 0, 1, 2, 4, 5, 7][(seed / 6 % 7) as usize];
                let cfg = DATA + 0x2000 + skew;
                fill(c, &mut r, cfg, 0x36);
                c.g[A0][0] = sx(cfg);
                let installed = (seed / 3) % 4 == 0;
                c.bus.write8(OC691_STATE + 0x174, installed as u8);
                c.bus.write16(OC691_STATE + 0x17e, (seed / 12 % 3) as u16);
                c.bus.write32(OC691_STATE + 0x1f0, DATA + 0x3000);
                schedule(c, &mut r, seed, 0x003b_4fc0)
            },
        );
    }

    /// `SND_Init`: already installed (returns 0), and a configuration
    /// fetch that fails (negative capabilities word, no voices configured).
    /// The path on to `SND_IopInit` (RPC to the IOP) cannot be bounded here:
    /// the race replay gate covers it.
    #[test]
    fn snd_init_matches() {
        check(ee::s13::f_003b5158__generated, snd_init, 200, &|c, seed| {
            let mut r = oc691_machine(c, seed, 0x003b_5158, 0);
            c.g[A0][0] = sx(DATA + 0x4000);
            c.g[A1][0] = sx(0x400 + 0x100 * (seed % 8));
            let installed = seed % 3 == 0;
            c.bus.write8(OC691_STATE + 0x174, installed as u8);
            c.bus.write16(OC691_STATE + 0x17e, 0);
            c.bus.write32(OC691_GOT_CAPS, 1);
            c.bus.write32(OC691_CAPS, 0xffff_ff00 | r.next_u32() & 0xff);
        });
    }

    /// `SND_StartTimerService`: the system task already added or not, the
    /// exit handler list empty, already holding the handler, or full.
    #[test]
    fn snd_start_timer_service_matches() {
        check(
            ee::s13::f_003b5478__generated,
            snd_start_timer_service,
            200,
            &|c, seed| {
                let mut r = oc691_machine(c, seed, 0x003b_5478, 0);
                c.bus.write32(OC691_TASK_ADDED, seed % 2);
                match (seed / 2) % 3 {
                    1 => c.bus.write32(
                        OC691_EXIT_LIST + 4 * (r.next_u32() % 64),
                        OC691_EXIT_HANDLER,
                    ),
                    2 => {
                        for i in 0..64 {
                            c.bus.write32(OC691_EXIT_LIST + 4 * i, 0x0012_3450 + 4 * i);
                        }
                    }
                    _ => {}
                }
            },
        );
    }

    /// `SND_RemoveTickCallback`: subsystem off, no callbacks, a handler
    /// somewhere in the table (or not), negative and large counts.
    #[test]
    fn snd_remove_tick_callback_matches() {
        covered(
            ee::s13::f_003b5948__generated,
            snd_remove_tick_callback,
            200,
            &|c, seed| {
                let mut r = oc691_machine(c, seed, 0x003b_5948, 2);
                let on = seed % 5 != 0;
                c.bus.write8(OC691_STATE + 0x174, on as u8);
                c.bus.write8(OC691_STATE + 0x176, r.next_u32() as u8);
                let count: u32 = match seed % 6 {
                    0 => 0,
                    1 => 0x80 | (r.next_u32() & 0x7f),
                    2 => 100,
                    _ => 1 + r.next_u32() % 7,
                };
                c.bus.write8(OC691_STATE + 0x177, count as u8);
                let handlers = [0x0012_3450u32, 0x0012_3460, 0x8123_4560];
                let cb = handlers[(r.next_u32() % 3) as usize];
                for i in 0..110u32 {
                    let h = if r.next_u32() % 4 == 0 {
                        cb
                    } else {
                        handlers[(r.next_u32() % 3) as usize]
                    };
                    c.bus.write32(OC691_STATE + 0x184 + 4 * i, h);
                }
                c.g[A0][0] = sx(cb);
            },
        );
    }

    /// `SND_SpuMemSetBase` under the (faked) sound lock: -1 and ordinary
    /// start and end addresses.
    #[test]
    fn snd_spu_mem_set_base_matches() {
        covered(
            ee::s13::f_003b5b20__generated,
            snd_spu_mem_set_base,
            200,
            &|c, seed| {
                let mut r = oc691_machine(c, seed, 0x003b_5b20, 2);
                let starts = [u32::MAX, 0, 0x1000, 0x1_0000, 0x7_fff0, 0x20_0000];
                let ends = [u32::MAX, 0x8_0000, 0x1f_ffff, 0x4000];
                c.g[A0][0] = sx(starts[(r.next_u32() % 6) as usize]);
                c.g[A1][0] = sx(ends[(r.next_u32() % 4) as usize]);
            },
        );
    }

    /// `SND_SpuMemGetFree` under the (faked) sound lock: an empty block list
    /// and lists of one to three blocks.
    #[test]
    fn snd_spu_mem_get_free_matches() {
        covered(
            ee::s13::f_003b5b68__generated,
            snd_spu_mem_get_free,
            200,
            &|c, seed| {
                let mut r = oc691_machine(c, seed, 0x003b_5b68, 2);
                let count = if seed % 2 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 3
                };
                c.bus.write16(OC691_SPU_STATE + 0x1a98, count as u16);
                c.bus
                    .write16(OC691_SPU_STATE + 0x1a9a, (r.next_u32() & 0xff) as u16);
                c.bus
                    .write16(OC691_SPU_STATE + 0x1a9c, (r.next_u32() & 0x1fff) as u16);
                c.bus
                    .write16(OC691_SPU_STATE + 0x1a9e, (r.next_u32() & 0x1fff) as u16);
                c.bus.write32(OC691_SPU_STATE + 0x1aa0, DATA + 0x400);
                for i in 0..8u32 {
                    c.bus
                        .write16(DATA + 0x400 + 2 * i, (r.next_u32() & 0x1fff) as u16);
                }
                c.g[A0][0] = sx(DATA + 0x200);
            },
        );
    }

    /// `SND_BankMove`: a bank slot with a sane header size (including
    /// overlapping source and destination) or a negative one.
    #[test]
    fn snd_bank_move_matches() {
        check(
            ee::s13::f_003b6548__generated,
            snd_bank_move,
            200,
            &|c, seed| {
                let mut r = oc691_machine(c, seed, 0x003b_6548, 0);
                let list = DATA + 0x1000;
                c.bus.write32(OC691_STATE + 0x1f4, list);
                let n = seed % 4;
                let hdr = DATA + 0x2000;
                for i in 0..4u32 {
                    c.bus.write32(list + 16 * i, hdr + 0x100 * i);
                }
                fill(c, &mut r, DATA + 0x2000, 0x80);
                let size: u32 = if seed % 5 == 0 {
                    (-1 - (r.next_u32() % 3) as i32) as u32
                } else {
                    0x20 + r.next_u32() % 0x60
                };
                c.bus.write32(hdr + 0x100 * n + 8, size);
                let dest = match seed % 3 {
                    0 => DATA + 0x6000,
                    1 => hdr + 0x100 * n + 8,
                    _ => hdr + 0x100 * n - 4,
                };
                c.g[A0][0] = sx(dest);
                c.g[A1][0] = sx(n);
            },
        );
    }

    /// Sets up the bank table the `SND_BankAdd` and `SND_BankRemove` tests
    /// share: four slots (some free, some busy, SPU memory present), voices
    /// and channels that are never the removed bank's.
    fn oc691_bank_table(c: &mut Cpu, r: &mut Rng) {
        let list = DATA + 0x1000;
        c.bus.write16(OC691_STATE + 0x28, 4);
        c.bus.write32(OC691_STATE + 0x1f4, list);
        c.bus.write32(OC691_STATE + 0x1f0, DATA + 0x3000);
        c.bus
            .write16(OC691_STATE + 0x17e, (r.next_u32() % 4) as u16);
        for i in 0..4u32 {
            let entry = list + 16 * i;
            let obj = DATA + 0x2000 + 0x400 * i;
            c.bus
                .write32(entry, if r.next_u32() % 4 == 0 { 0 } else { obj });
            c.bus.write32(entry + 4, 0);
            c.bus.write32(
                entry + 8,
                if r.next_u32() % 2 == 0 {
                    0
                } else {
                    DATA + 0x5000
                },
            );
            c.bus.write8(entry + 0xc, (r.next_u32() % 5 == 0) as u8);
            c.bus.write16(obj + 6, (r.next_u32() % 3) as u16);
            // The channels (0x8c bytes each): bank handles 0x40.. never match.
            c.bus
                .write16(DATA + 0x3000 + 0x8c * i + 0x12, 0x40 + (i as u16));
        }
    }

    /// `SND_BankAdd`: free slot found or not, a bank with neither an IOP nor
    /// an SPU part (returns 8) and one whose SPU allocation fails (-6, with
    /// the SPU memory manager empty). The upload and relocation legs (IOP
    /// heap allocation by RPC, SIF DMA) cannot be bounded here: the race
    /// replay gate covers them.
    #[test]
    fn snd_bank_add_matches() {
        check(
            ee::s13::f_003b6098__generated,
            snd_bank_add,
            200,
            &|c, seed| {
                let mut r = oc691_machine(c, seed, 0x003b_6098, 0);
                oc691_bank_table(c, &mut r);
                let hdr = DATA + 0x6000;
                fill(c, &mut r, hdr, 0x10);
                c.bus.write32(hdr + 0x10, 0);
                let spu = if seed % 3 == 0 {
                    0
                } else {
                    0x40 + r.next_u32() % 0x1000
                };
                c.bus.write32(hdr + 0xc, spu);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(hdr);
            },
        );
    }

    /// `SND_BankRemove`: -1 (all banks), invalid handles, busy or free
    /// slots, banks with patch tables of no patches, an SPU block to free.
    #[test]
    fn snd_bank_remove_matches() {
        covered(
            ee::s13::f_003b6300__generated,
            snd_bank_remove,
            240,
            &|c, seed| {
                let mut r = oc691_machine(c, seed, 0x003b_6300, 3);
                oc691_bank_table(c, &mut r);
                c.g[A0][0] = match seed % 8 {
                    0 => u64::MAX,
                    1 => sx(0xffff_fffe),
                    2 => 100,
                    k => (k % 4) as u64,
                };
            },
        );
    }
}
