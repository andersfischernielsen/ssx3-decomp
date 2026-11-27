//! Staging module oc1047 (wave wave264): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1047`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002b_07c0,
        speech_instance_is_playing,
        "SpeechInstance_IsPlaying",
    ),
    (
        0x002b_07e8,
        speech_instance_set_param8c4,
        "SpeechInstance_SetParam8c4",
    ),
    (
        0x002b_07f0,
        speech_instance_set_param8c8,
        "SpeechInstance_SetParam8c8",
    ),
    (
        0x002b_0f10,
        speech_system_load_speech_inf,
        "SpeechSystem_LoadSpeechInf",
    ),
    (
        0x002b_0fe8,
        speech_system_set_instance_param8c4,
        "SpeechSystem_SetInstanceParam8c4",
    ),
    (0x002a_fb38, speech_instance_dtor, "SpeechInstance_dtor"),
    (0x002b_0ae8, speech_system_dtor, "SpeechSystem_dtor"),
];

/// `base + off` as a guest address (32-bit wrapping, as the generated code).
fn speech_addr(base: u64, off: u32) -> Addr {
    Addr((base as u32).wrapping_add(off))
}

/// `bool SpeechInstance_IsPlaying(SpeechInstance* self)` (0x2b07c0): whether the
/// stream array (global at `gp+0x6c4`) reports this instance's stream as playing.
pub fn speech_instance_is_playing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = speech_addr(c.g[GP][0], 0x6c4).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = speech_addr(c.g[A0][0], 4).lw(c);
        c.g[A0][0] = speech_addr(c.g[V0][0], 0).lw(c);
        call_ptr(c, 0x002a_b028, 0x002b_07d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SpeechInstance_SetParam8c4(SpeechInstance* self, int v)` (0x2b07e8):
/// stores `v` at `self+0x8c4`.
pub fn speech_instance_set_param8c4(c: &mut Cpu) {
    c.cycles += 2;
    let v = c.g[A1][0];
    speech_addr(c.g[A0][0], 0x8c4).sw(c, v);
    ret(c);
}

/// `void SpeechInstance_SetParam8c8(SpeechInstance* self, int v)` (0x2b07f0):
/// stores `v` at `self+0x8c8`.
pub fn speech_instance_set_param8c8(c: &mut Cpu) {
    c.cycles += 2;
    let v = c.g[A1][0];
    speech_addr(c.g[A0][0], 0x8c8).sw(c, v);
    ret(c);
}

/// `void SpeechSystem_LoadSpeechInf(SpeechSystem* self, int index)` (0x2b0f10):
/// `SpeechInstance_LoadSpeechInf(inst, index, 0)` on `self->instances[index]`,
/// unless that instance pointer is null.
pub fn speech_system_load_speech_inf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V1][0] = speech_addr(c.g[A0][0], 4).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = speech_addr(c.g[V0][0], 0).lw(c);
        frame.restore_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A3][0] = 0;
            call_ptr(c, 0x002b_0088, 0x002b_0f38)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void SpeechSystem_SetInstanceParam8c4(SpeechSystem* self, int index, int v)`
/// (0x2b0fe8): `SpeechInstance_SetParam8c4(inst, v)` on `self->instances[index]`,
/// unless that instance pointer is null.
pub fn speech_system_set_instance_param8c4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = speech_addr(c.g[A0][0], 4).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = speech_addr(c.g[A1][0], 0).lw(c);
        frame.restore_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[A2][0];
            call_ptr(c, 0x002b_07e8, 0x002b_1010)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `cSpeechInstance::~cSpeechInstance` (0x2afb38; a0 = self, a1 = flags, bit 0
/// frees the object through `__builtin_delete`). Destroys the bank objects
/// (count at `+0xf0`, pointers at `+0xf4`), the stream at `gp+0x6c4`, the
/// per-entry pointer pairs at `+0x8d4` / `+0xd0` / `+0xcc` (count `+0xec`) and
/// updates the counter at `gp+0x6c8`.
pub fn speech_instance_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S1, 0x80);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0;
        c.g[S3][0] = c.g[A0][0];
        c.g[V0][0] = speech_addr(c.g[S3][0], 0xf0).lw(c);
        c.g[S2][0] = c.g[A1][0];
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0xf4));
            // Bank loop: destroy each bank object.
            loop {
                c.cycles += 3;
                c.g[A0][0] = speech_addr(c.g[S1][0], 0).lw(c);
                if c.g[A0][0] == 0 {
                    c.g[V0][0] = speech_addr(c.g[S3][0], 0xf0).lw(c);
                } else {
                    c.cycles += 3;
                    c.g[A1][0] = 3;
                    call_ptr(c, 0x002a_f900, 0x002a_fb94)?;
                    c.g[V0][0] = speech_addr(c.g[S3][0], 0xf0).lw(c);
                }
                c.cycles += 4;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                let more = (c.g[S0][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002a_fb80);
                }
            }
        }
        // 'b_2afc38: the counter at gp+0x6c8 and the pointer-pair tables.
        c.cycles += 8;
        c.g[V0][0] = speech_addr(c.g[GP][0], 0x6c8).lw(c);
        c.g[FP][0] = c.g[S2][0] & 1;
        c.g[V1][0] = speech_addr(c.g[S3][0], 0xf0).lw(c);
        c.g[S0][0] = 0;
        c.g[A0][0] = speech_addr(c.g[S3][0], 0xec).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        let entries = (c.g[A0][0] as i64) > 0;
        let counter = c.g[V0][0];
        speech_addr(c.g[GP][0], 0x6c8).sw(c, counter);
        if entries {
            c.cycles += 4;
            c.g[S7][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8d4));
            c.g[S6][0] = sx((c.g[S3][0] as u32).wrapping_add(0xd0));
            c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0xcc));
            c.g[S2][0] = sx((c.g[S0][0] as u32) << 3);
            // Entry loop: free the two pointers of each entry.
            loop {
                c.cycles += 4;
                c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S7][0] as u32));
                c.g[S1][0] = 1;
                loop {
                    c.cycles += 3;
                    c.g[A0][0] = speech_addr(c.g[S0][0], 0).lw(c);
                    let null = c.g[A0][0] == 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                    if !null {
                        c.cycles += 2;
                        call_ptr(c, 0x0025_23a8, 0x002a_fbfc)?;
                    }
                    c.cycles += 2;
                    let more = (c.g[S1][0] as i64) >= 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x002a_fbe8);
                        }
                        continue;
                    }
                    break;
                }
                // 'b_2afc24: the second pointer of the entry, then clear both slots.
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[V0][0] = speech_addr(c.g[S1][0], 0).lw(c);
                let zero = c.g[V0][0] == 0;
                c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S2][0] as u32));
                if !zero {
                    c.cycles += 4;
                    c.g[A0][0] = speech_addr(c.g[S0][0], 0).lw(c);
                    call_ptr(c, 0x0025_23a8, 0x002a_fc1c)?;
                    speech_addr(c.g[S1][0], 0).sw(c, 0);
                    speech_addr(c.g[S0][0], 0).sw(c, 0);
                }
                c.cycles += 5;
                c.g[V0][0] = speech_addr(c.g[S3][0], 0xec).lw(c);
                c.g[S0][0] = c.g[S4][0];
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S0][0] as u32) << 3);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002a_fbd8);
                    }
                    continue;
                }
                break;
            }
        }
        // 'b_2afc5c: the stream, then the object itself when a1 & 1.
        c.cycles += 6;
        c.g[V0][0] = speech_addr(c.g[GP][0], 0x6c4).lw(c);
        c.g[A1][0] = speech_addr(c.g[S3][0], 4).lw(c);
        c.g[A0][0] = speech_addr(c.g[V0][0], 0).lw(c);
        call_ptr(c, 0x002a_abd0, 0x002a_fc48)?;
        let flag_clear = c.g[FP][0] == 0;
        frame.restore(c, S0, 0x90);
        if !flag_clear {
            c.cycles += 3;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0031_7e50, 0x002a_fc58)?;
            frame.restore(c, S0, 0x90);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cSpeechSys::~cSpeechSys` (0x2b0ae8; a0 = self, a1 = flags: bit 1 destroys
/// the stream array, the bank manager and the sound manager reference; bit 0
/// frees the object). When a1 is zero, the vtable words of the system and of
/// its sub-objects are (re)written first.
pub fn speech_system_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S0, 0x80);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3870));
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x60);
        c.g[A3][0] = 0x0048_0000;
        let vt = c.g[V0][0];
        speech_addr(c.g[S0][0], 0x1bc).sw(c, vt);
        c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x38a0));
        c.g[V1][0] = speech_addr(c.g[S0][0], 0).lw(c);
        c.g[A0][0] = 0x0048_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3b48));
        c.g[T0][0] = 0x0048_0000;
        c.g[V0][0] = speech_addr(c.g[V1][0], 0).lw(c);
        c.g[T5][0] = sx((c.g[T0][0] as u32).wrapping_add(0x38c0));
        c.g[S1][0] = c.g[A1][0];
        c.g[V1][0] = speech_addr(c.g[V0][0], 0x1d8).lw(c);
        let a2 = c.g[A2][0];
        speech_addr(c.g[V1][0], 4).sw(c, a2);
        c.g[V0][0] = speech_addr(c.g[S0][0], 0).lw(c);
        c.g[V1][0] = speech_addr(c.g[V0][0], 0).lw(c);
        let a0 = c.g[A0][0];
        speech_addr(c.g[V1][0], 0xab0).sw(c, a0);
        c.g[V0][0] = speech_addr(c.g[S0][0], 0).lw(c);
        c.g[V1][0] = speech_addr(c.g[V0][0], 0).lw(c);
        let t5 = c.g[T5][0];
        speech_addr(c.g[V1][0], 0x1d4).sw(c, t5);
        if c.g[S1][0] == 0 {
            c.cycles += 44;
            c.g[A1][0] = speech_addr(c.g[A3][0], 0x38a0).ld(c);
            c.g[T4][0] = sx(sp(c).wrapping_add(0x20));
            c.g[V1][0] = speech_addr(c.g[A2][0], 0x10).ld(c);
            c.g[A0][0] = speech_addr(c.g[A2][0], 0x18).ld(c);
            c.g[V0][0] = speech_addr(c.g[A2][0], 0x8).ld(c);
            let a1 = c.g[A1][0];
            speech_addr(c.g[SP][0], 0).sd(c, a1);
            let a0 = c.g[A0][0];
            speech_addr(c.g[SP][0], 0x18).sd(c, a0);
            let v1 = c.g[V1][0];
            speech_addr(c.g[SP][0], 0x10).sd(c, v1);
            let v0 = c.g[V0][0];
            speech_addr(c.g[SP][0], 0x8).sd(c, v0);
            c.g[V0][0] = speech_addr(c.g[S0][0], 0).lw(c);
            c.g[A0][0] = speech_addr(c.g[A2][0], 0x8).lhu(c);
            c.g[V1][0] = speech_addr(c.g[V0][0], 0).lw(c);
            c.g[T1][0] = speech_addr(c.g[T0][0], 0x38c0).ld(c);
            c.g[V0][0] = speech_addr(c.g[V1][0], 0x1d8).lw(c);
            c.g[T2][0] = speech_addr(c.g[T5][0], 8).ld(c);
            let spv = c.g[SP][0];
            speech_addr(c.g[V0][0], 4).sw(c, spv);
            c.g[T3][0] = speech_addr(c.g[T5][0], 0x10).ld(c);
            c.g[A3][0] = speech_addr(c.g[S0][0], 0).lw(c);
            c.g[T0][0] = speech_addr(c.g[T5][0], 0x18).ld(c);
            c.g[V1][0] = speech_addr(c.g[A3][0], 0).lw(c);
            c.g[A2][0] = speech_addr(c.g[T5][0], 0x20).ld(c);
            c.g[V0][0] = speech_addr(c.g[V1][0], 0x1d8).lw(c);
            c.g[V1][0] = speech_addr(c.g[T5][0], 0x30).ld(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1c0));
            c.g[A1][0] = speech_addr(c.g[T5][0], 0x28).ld(c);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            let v1 = c.g[V1][0];
            speech_addr(c.g[SP][0], 0x50).sd(c, v1);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let t1 = c.g[T1][0];
            speech_addr(c.g[SP][0], 0x20).sd(c, t1);
            let a0 = c.g[A0][0];
            speech_addr(c.g[SP][0], 0x8).sh(c, a0);
            let t2 = c.g[T2][0];
            speech_addr(c.g[SP][0], 0x28).sd(c, t2);
            let t3 = c.g[T3][0];
            speech_addr(c.g[SP][0], 0x30).sd(c, t3);
            let t0 = c.g[T0][0];
            speech_addr(c.g[SP][0], 0x38).sd(c, t0);
            let a2 = c.g[A2][0];
            speech_addr(c.g[SP][0], 0x40).sd(c, a2);
            let a1 = c.g[A1][0];
            speech_addr(c.g[SP][0], 0x48).sd(c, a1);
            c.g[A0][0] = speech_addr(c.g[T5][0], 8).lhu(c);
            c.g[V0][0] = speech_addr(c.g[A3][0], 0).lw(c);
            let t4 = c.g[T4][0];
            speech_addr(c.g[V0][0], 0x1d4).sw(c, t4);
            c.g[V1][0] = speech_addr(c.g[S0][0], 0).lw(c);
            c.g[V0][0] = speech_addr(c.g[V1][0], 0).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1c8));
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let a0 = c.g[A0][0];
            speech_addr(c.g[SP][0], 0x28).sh(c, a0);
        }
        // 'b_2b0c18: the instance at this+4 (a nonzero pointer is destroyed).
        c.cycles += 3;
        c.g[A0][0] = speech_addr(c.g[S0][0], 4).lw(c);
        let has_part = c.g[A0][0] != 0;
        c.g[V0][0] = c.g[S1][0] & 2;
        if has_part {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x002b_0c14)?;
            c.g[V0][0] = c.g[S1][0] & 2;
        }
        // 'b_2b0c50: clear the global, then the stream array, the bank manager
        // and the sound manager reference when a1 & 2.
        c.cycles += 2;
        let no_managers = c.g[V0][0] == 0;
        speech_addr(c.g[GP][0], 0x6c4).sw(c, 0);
        if !no_managers {
            c.cycles += 12;
            c.g[A0][0] = speech_addr(c.g[S0][0], 0).lw(c);
            c.g[A1][0] = 0;
            call_ptr(c, 0x002a_a648, 0x002b_0c2c)?;
            c.g[V0][0] = speech_addr(c.g[S0][0], 0).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = speech_addr(c.g[V0][0], 0).lw(c);
            call_ptr(c, 0x0028_bb10, 0x002b_0c3c)?;
            c.g[V1][0] = speech_addr(c.g[S0][0], 0).lw(c);
            c.g[A1][0] = 0;
            c.g[V0][0] = speech_addr(c.g[V1][0], 0).lw(c);
            c.g[A0][0] = speech_addr(c.g[V0][0], 0x1d8).lw(c);
            call_ptr(c, 0x002a_dee8, 0x002b_0c50)?;
        }
        // 'b_2b0c68: free the object when a1 & 1.
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x80);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x002b_0c64)?;
            frame.restore(c, S0, 0x80);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes in the machine's RAM.
    const OC1047_CTRL: u32 = DATA + 0x21_0200;
    const OC1047_MAGIC: u32 = 0x1047_c0de;

    // Each fake logs the call (`fakes::fake_call`) when the gate word is set,
    // and otherwise runs the body it replaced.
    macro_rules! oc1047_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1047_CTRL) != OC1047_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc1047_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1047_fakes!(
        (
            oc1047_fk_enter_critical,
            OC1047_PREV_ENTER_CRITICAL,
            ee::s13::OVERRIDE_003B58A0,
            ee::s13::f_003b58a0,
            0x003b_58a0
        ),
        (
            oc1047_fk_leave_critical,
            OC1047_PREV_LEAVE_CRITICAL,
            ee::s13::OVERRIDE_003B58D8,
            ee::s13::f_003b58d8,
            0x003b_58d8
        ),
        (
            oc1047_fk_snd_stop,
            OC1047_PREV_SND_STOP,
            ee::s13::OVERRIDE_003B89E0,
            ee::s13::f_003b89e0,
            0x003b_89e0
        ),
        (
            oc1047_fk_snd_purge,
            OC1047_PREV_SND_PURGE,
            ee::s13::OVERRIDE_003B78F8,
            ee::s13::f_003b78f8,
            0x003b_78f8
        ),
        (
            oc1047_fk_stream_playing,
            OC1047_PREV_STREAM_PLAYING,
            ee::s08::OVERRIDE_002AB028,
            ee::s08::f_002ab028,
            0x002a_b028
        ),
        (
            oc1047_fk_bank_dtor,
            OC1047_PREV_BANK_DTOR,
            ee::s08::OVERRIDE_002AF900,
            ee::s08::f_002af900,
            0x002a_f900
        ),
        (
            oc1047_fk_mem_free,
            OC1047_PREV_MEM_FREE,
            ee::s06::OVERRIDE_002523A8,
            ee::s06::f_002523a8,
            0x0025_23a8
        ),
        (
            oc1047_fk_stream_destroy,
            OC1047_PREV_STREAM_DESTROY,
            ee::s08::OVERRIDE_002AABD0,
            ee::s08::f_002aabd0,
            0x002a_abd0
        ),
        (
            oc1047_fk_builtin_delete,
            OC1047_PREV_BUILTIN_DELETE,
            ee::s10::OVERRIDE_00317E50,
            ee::s10::f_00317e50,
            0x0031_7e50
        ),
        (
            oc1047_fk_vec_delete,
            OC1047_PREV_VEC_DELETE,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98,
            0x0031_7e98
        ),
        (
            oc1047_fk_stream_arr_dtor,
            OC1047_PREV_STREAM_ARR_DTOR,
            ee::s08::OVERRIDE_002AA648,
            ee::s08::f_002aa648,
            0x002a_a648
        ),
        (
            oc1047_fk_bank_mgr_dtor,
            OC1047_PREV_BANK_MGR_DTOR,
            ee::s07::OVERRIDE_0028BB10,
            ee::s07::f_0028bb10,
            0x0028_bb10
        ),
        (
            oc1047_fk_soundref_dtor,
            OC1047_PREV_SOUNDREF_DTOR,
            ee::s08::OVERRIDE_002ADEE8,
            ee::s08::f_002adee8,
            0x002a_dee8
        ),
        (
            oc1047_fk_load_inf,
            OC1047_PREV_LOAD_INF,
            ee::s08::OVERRIDE_002B0088,
            ee::s08::f_002b0088,
            0x002b_0088
        ),
    );

    /// Install the fakes and open the gate in this fixture's machine.
    fn oc1047_arm(c: &mut Cpu) {
        oc1047_install_fakes();
        c.bus.write32(OC1047_CTRL, OC1047_MAGIC);
    }

    /// `PATH_PlatStopTrackSound` (0x2af838): the hand-written port in
    /// `game::sound::bankmonitor`, checked against the generated body (the
    /// port is not in `OVERRIDES` here). Flag clear or set, all four track
    /// slots, the stop and purge paths.
    #[test]
    fn oc1047_path_plat_stop_track_sound_matches() {
        check(
            ee::s08::f_002af838__generated,
            crate::game::sound::bankmonitor::path_plat_stop_track_sound,
            300,
            &|c, seed| {
                machine(c, seed, 0x1047_01, 3);
                oc1047_arm(c);
                let i = seed % 4;
                let obj = DATA + 0x1_0000 + i * 0x100;
                c.bus.write32(0x004d_3e98 + 4 * i, obj);
                let flag = if seed % 3 == 0 {
                    0
                } else {
                    1 + (seed % 100) as u8
                };
                c.bus.write8(obj + 0x64, flag);
                c.bus.write32(obj + 0x6c, DATA + 0x2_0000);
                c.bus.write32(DATA + 0x2_0004, DATA + 0x2_1000);
                c.g[A0][0] = sx(i);
            },
        );
    }

    /// `SpeechInstance_IsPlaying`: the stream array at `gp+0x6c4` answers for
    /// the instance's stream at `+4`; the fake may unwind.
    #[test]
    fn oc1047_speech_instance_is_playing_matches() {
        check(
            ee::s08::f_002b07c0__generated,
            speech_instance_is_playing,
            200,
            &|c, seed| {
                machine(c, seed, 0x1047_02, 1);
                oc1047_arm(c);
                c.bus.write32(GAME_GP + 0x6c4, DATA + 0x1_0000);
                c.bus.write32(DATA + 0x1_0000, DATA + 0x1_1000);
                let inst = DATA + 0x1_2000;
                c.bus.write32(inst + 4, seed.wrapping_mul(0x9e37));
                c.g[A0][0] = sx(inst);
            },
        );
    }

    /// `SpeechInstance_SetParam8c4` and `_SetParam8c8`: plain stores of the
    /// argument into the instance, whatever the value.
    #[test]
    fn oc1047_speech_instance_set_param8c4_matches() {
        check(
            ee::s08::f_002b07e8__generated,
            speech_instance_set_param8c4,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1047_03, 0);
                c.g[A0][0] = sx(DATA + 0x1_0000);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn oc1047_speech_instance_set_param8c8_matches() {
        check(
            ee::s08::f_002b07f0__generated,
            speech_instance_set_param8c8,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1047_04, 0);
                c.g[A0][0] = sx(DATA + 0x1_0000);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    /// Fixture for the two `SpeechSystem` index functions: the instance table
    /// at `self+4`, entry `index` null or not.
    fn oc1047_system_fixture(c: &mut Cpu, seed: u32) -> u32 {
        let sys = DATA + 0x1_0000;
        let table = DATA + 0x1_1000;
        let index = seed % 4;
        c.bus.write32(sys + 4, table);
        let inst = if seed % 3 == 0 {
            0
        } else {
            DATA + 0x1_2000 + index * 0x100
        };
        c.bus.write32(table + 4 * index, inst);
        c.g[A0][0] = sx(sys);
        c.g[A1][0] = sx(index);
        index
    }

    /// `SpeechSystem_LoadSpeechInf`: the instance at `index` (null or not) is
    /// loaded with a3 = 0; the loader is faked and may unwind.
    #[test]
    fn oc1047_speech_system_load_speech_inf_matches() {
        check(
            ee::s08::f_002b0f10__generated,
            speech_system_load_speech_inf,
            300,
            &|c, seed| {
                machine(c, seed, 0x1047_05, 2);
                oc1047_arm(c);
                oc1047_system_fixture(c, seed);
            },
        );
    }

    /// `SpeechSystem_SetInstanceParam8c4`: the real `SetParam8c4` runs on the
    /// instance at `index` (null or not) with the value in a2.
    #[test]
    fn oc1047_speech_system_set_instance_param8c4_matches() {
        check(
            ee::s08::f_002b0fe8__generated,
            speech_system_set_instance_param8c4,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1047_06, 0);
                oc1047_system_fixture(c, seed);
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    /// `SpeechInstance_dtor`: bank count 0..=3 (null or not per bank), entry
    /// count 0..=3 with null or not pointers in the three tables, the stream
    /// at `gp+0x6c4`, flag bit 0 with the object freed; the callees are faked
    /// and may unwind.
    #[test]
    fn oc1047_speech_instance_dtor_matches() {
        check(
            ee::s08::f_002afb38__generated,
            speech_instance_dtor,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1047_07, 6);
                oc1047_arm(c);
                let this = DATA + 0x1_0000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                for j in 0..4u32 {
                    for k in 0..2u32 {
                        let off = 8 * j + 4 * k;
                        for base in [0x8d4u32, 0xd0, 0xcc] {
                            let p = if r.next_u32() % 3 == 0 {
                                0
                            } else {
                                DATA + 0x3_0000 + base + off
                            };
                            c.bus.write32(this + base + off, p);
                        }
                    }
                }
                for i in 0..4u32 {
                    let bank = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        DATA + 0x2_0000 + i * 0x100
                    };
                    c.bus.write32(this + 0xf4 + 4 * i, bank);
                }
                c.bus.write32(this + 4, DATA + 0x3_3000);
                c.bus.write32(GAME_GP + 0x6c4, DATA + 0x3_1000);
                c.bus.write32(DATA + 0x3_1000, DATA + 0x3_2000);
                c.bus.write32(GAME_GP + 0x6c8, r.next_u32());
                // Written last: these words overlap the tables above.
                c.bus.write32(this + 0xec, r.next_u32() % 4);
                c.bus.write32(this + 0xf0, r.next_u32() % 4);
            },
        );
    }

    /// `SpeechSystem_dtor`: the flag word covers all four values (bit 1 the
    /// managers, bit 0 the object, zero the copy path); the nested instance at
    /// `this+4` null or not; the vtable chain `this -> p1 -> p2 -> p3`; the
    /// callees are faked and may unwind.
    #[test]
    fn oc1047_speech_system_dtor_matches() {
        check(
            ee::s08::f_002b0ae8__generated,
            speech_system_dtor,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1047_08, 5);
                oc1047_arm(c);
                let this = DATA + 0x1_0000;
                let p1 = DATA + 0x1_1000;
                let p2 = DATA + 0x1_2000;
                let p3 = DATA + 0x1_3000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(this, p1);
                c.bus.write32(p1, p2);
                c.bus.write32(p2 + 0x1d8, p3);
                let inst = if r.next_u32() % 3 == 0 {
                    0
                } else {
                    DATA + 0x1_4000
                };
                c.bus.write32(this + 4, inst);
            },
        );
    }
}
