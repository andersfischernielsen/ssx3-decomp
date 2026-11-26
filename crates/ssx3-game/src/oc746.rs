//! Staging module oc746 (wave wave242): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc746`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0030_45b8,
        wscript_fn_set_rider_config_bits,
        "WScriptFn_SetRiderConfigBits",
    ),
    (
        0x0030_4980,
        wscript_fn_set_object_field24,
        "WScriptFn_SetObjectField24",
    ),
    (
        0x0030_50f0,
        wscript_fn_camera_shake,
        "WScriptFn_CameraShake",
    ),
];

/// `gp + off` (the game's gp-relative globals, `off` as a wrapping offset).
fn w746_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off)
}

/// The script-parameter loop shared by both functions: `a1` entries of
/// 16 bytes at `a2` (`idx`, `val`, -, `type`); each `val` goes into word `idx`
/// of the buffer in register `buf` (a float when the kind table at `t0` says 2
/// and the entry's type differs). `head` is the loop head the event check
/// reports.
fn w746_param_loop(c: &mut Cpu, buf: usize, head: u32) {
    loop {
        c.cycles += 8;
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[buf][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4)).lw(c);
            c.cycles += 1;
            let (dst, v) = (c.g[A0][0] as u32, c.g[V0][0]);
            Addr(dst).sw(c, v);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4)).lw(c);
                c.cycles += 1;
                let (dst, v) = (c.g[A0][0] as u32, c.g[V0][0]);
                Addr(dst).sw(c, v);
            } else {
                c.cycles += 4;
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.cvt_s_w(0, 0);
                c.swc1(0, c.g[A0][0] as u32);
            }
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The fields of the rider-config record (16 words) packed into the 64-bit
/// word at `sp+0x60`: `(word offset, value mask, bit shift)`, in the order
/// the generated body merges them.
const W746_RIDER_CONFIG_FIELDS: [(u32, u64, u32); 16] = [
    (0x00, 0x7, 0),
    (0x04, 0xf, 3),
    (0x08, 0x7, 7),
    (0x0c, 0x3, 10),
    (0x10, 0xf, 12),
    (0x14, 0x7, 16),
    (0x18, 0x7, 19),
    (0x1c, 0x3f, 22),
    (0x20, 0xf, 28),
    (0x24, 0x7, 32),
    (0x28, 0x7f, 35),
    (0x2c, 0x1, 42),
    (0x30, 0x7f, 43),
    (0x34, 0x3, 50),
    (0x38, 0x7, 56),
    (0x3c, 0xf, 52),
];

/// `void WScriptFn_SetRiderConfigBits(sret, int count, Param* params)`
/// (0x3045b8): on the first call zeroes the static 16-word config block
/// (`0x4fbcc8`), copies it (0x44 bytes) to the stack, stores each parameter
/// into its word (a float when the kind table at `0x446650` says 2), packs the
/// words' low bits into one 64-bit word and passes it to
/// `WScriptMan_SetPendingRiderConfig`; returns the nil value in `sret`.
pub fn wscript_fn_set_rider_config_bits(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xe0);
        c.g[V0][0] = Addr(w746_gp(c, 0x271c)).lw(c);
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S5, 0x80);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0xb0);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S3, 0xa0);
        c.g[A1][0] = 0x0050_0000;
        frame.save(c, S4, 0x90);
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x70);
        if !initialised {
            c.cycles += 22;
            let block = (c.g[A1][0] as u32).wrapping_sub(0x4338);
            Addr(block).sw(c, 0);
            c.g[V0][0] = sx(block);
            c.g[V1][0] = 1;
            c.g[A0][0] = Addr(block.wrapping_add(0x40)).lbu(c);
            let flag = c.g[V1][0];
            Addr(w746_gp(c, 0x271c)).sw(c, flag);
            let byte = c.g[A0][0];
            Addr(block.wrapping_add(0x40)).sb(c, byte);
            for off in (4..=0x3cu32).step_by(4) {
                Addr(block.wrapping_add(off)).sw(c, 0);
            }
            Addr(sp(c)).sb(c, byte);
        }
        c.cycles += 8;
        c.g[S4][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x44;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4338));
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x003e_6574, 0x0030_465c)?;
        c.g[T8][0] = sx(sp(c).wrapping_add(0x60));
        let skip = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S1][0];
        if !skip {
            c.cycles += 4;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6650));
            c.g[A3][0] = 2;
            w746_param_loop(c, S4, 0x0030_4678);
        }
        // Pack the config words (the generated body's 176 cycles).
        c.cycles += 176;
        let rec = c.g[S4][0] as u32;
        let masks = [
            A0, A1, A2, A3, T0, T1, T2, T3, T4, T5, T6, T7, S0, S1, A2, S2,
        ];
        let mut v = Addr(sp(c).wrapping_add(0x60)).ld(c);
        let mut last = 0u64;
        for (i, &(off, wm, sh)) in W746_RIDER_CONFIG_FIELDS.iter().enumerate() {
            let mask = !(wm << sh);
            c.g[masks[i]][0] = mask;
            last = (Addr(rec.wrapping_add(off)).lwu(c) & wm) << sh;
            v = (v & mask) | last;
        }
        c.g[V1][0] = last;
        c.g[S3][0] = u64::MAX >> 5;
        c.g[A0][0] = Addr(w746_gp(c, 0xce8)).lw(c);
        c.g[A1][0] = c.g[T8][0];
        v &= c.g[S3][0];
        c.g[V0][0] = v;
        let packed = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x60)).sd(c, packed);
        call_ptr(c, 0x0030_b520, 0x0030_4938)?;
        c.g[V1][0] = 0x004d_0000;
        c.g[V0][0] = c.g[S5][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(A0, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(A0, c.g[A3][0] as u32);
        c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x8)).lw(c);
        c.sdl(A0, (c.g[S5][0] as u32).wrapping_add(0x7));
        c.sdr(A0, c.g[S5][0] as u32);
        let sret = (c.g[S5][0] as u32).wrapping_add(0x8);
        let v = c.g[A1][0];
        Addr(sret).sw(c, v);
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptFn_SetObjectField24(sret, int count, Param* params)`
/// (0x304980): on the first call sets the static 12-byte block (`0x4fbd10`) to
/// `{-1, 1.0f, ...}`, copies it to the stack, stores each parameter into it
/// (the kind table at `gp+0xc78`), resolves the object id in word 0 (`-1`:
/// the WScriptMan's own instance at `+0x290`, else through the world cache's
/// section and data tables) and, when the object's node (`+0xc`) answers the
/// move-node virtual query, calls `0x355878(node, block)` (adds the fade).
/// Returns the nil value in `sret`.
pub fn wscript_fn_set_object_field24(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(w746_gp(c, 0x2720)).lw(c);
        frame.save(c, S3, 0x40);
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[A2][0] = 0x0050_0000;
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x20);
        if !initialised {
            c.cycles += 12;
            let block = (c.g[A2][0] as u32).wrapping_sub(0x42f0);
            c.g[V1][0] = sx(block);
            c.g[A1][0] = Addr(block.wrapping_add(0x8)).lbu(c);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[A0][0] = 1;
            Addr(block).sw(c, u32::MAX as u64);
            let byte = c.g[A1][0];
            Addr(block.wrapping_add(0x8)).sb(c, byte);
            c.swc1(0, block.wrapping_add(0x4));
            Addr(w746_gp(c, 0x2720)).sw(c, 1);
            Addr(sp(c)).sb(c, byte);
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x42f0));
        c.g[A2][0] = 0xc;
        Addr(sp(c).wrapping_add(0x10)).sw(c, u32::MAX as u64);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = 0x004d_0000;
        call_ptr(c, 0x003e_6574, 0x0030_4a08)?;
        let skip = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S3][0];
        if !skip {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(w746_gp(c, 0xc78));
            c.g[A3][0] = 2;
            w746_param_loop(c, S1, 0x0030_4a20);
        }
        // Resolve the object id in word 0 to the object record `v0`.
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        let by_table = c.g[V0][0] != 0;
        c.g[A1][0] = Addr(w746_gp(c, 0x16c8)).lw(c);
        if !by_table {
            c.cycles += 3;
            c.g[V0][0] = Addr(w746_gp(c, 0xce8)).lw(c);
            c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            let no_section = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            let mut found = false;
            if !no_section {
                c.cycles += 7;
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                found = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        // The object's node, if it is a move node: add the fade.
        c.cycles += 3;
        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 7;
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x80)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0030_4ae4)?;
            let no_move = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !no_move {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0035_5878, 0x0030_4af4)?;
            }
        }
        c.cycles += 16;
        c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6f68));
        c.ldl(V0, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(V0, c.g[A1][0] as u32);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lw(c);
        c.sdl(V0, (c.g[S2][0] as u32).wrapping_add(0x7));
        c.sdr(V0, c.g[S2][0] as u32);
        let sret = (c.g[S2][0] as u32).wrapping_add(0x8);
        let v = c.g[V1][0];
        Addr(sret).sw(c, v);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// The id lookup of a script block's object (`SetObjectField24`'s too): the
/// block's id word in `a2`, the world-cache pointer cell in `a1`. Returns with
/// `v0` the object record (0 when missing) and the cycles charged. Only the
/// table path: the caller handles the `-1` (instance) case.
fn w746_lookup_object(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    let no_section = c.g[V1][0] == 0;
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    let mut found = false;
    if !no_section {
        c.cycles += 7;
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        found = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    }
    if !found {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
}

/// `void WScriptFn_CameraShake(sret, int count, Param* params)` (0x3050f0):
/// the block `{keyword, useObj, id, scale}` (defaults `0, 1, -1, 1.0f`, the
/// static at `0x4fbd40`, set up on the first call) is filled from the
/// parameters (kind table at `0x446690`). With keyword 0 it shakes the camera
/// of every local rider at its distance (times `scale`) from the object (the
/// object resolved from `id` when `useObj`), or, without an object, of the
/// rider whose AI index is the script's being. Returns the nil value in
/// `sret`. Has a resume twin: `c.pc` other than the entry goes to
/// `f_003050f0__entries`.
pub fn wscript_fn_camera_shake(c: &mut Cpu) {
    if c.pc != 0x0030_50f0 {
        return ee::s10::f_003050f0__entries(c);
    }
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x100);
        c.g[V0][0] = Addr(w746_gp(c, 0x274c)).lw(c);
        frame.save(c, S2, 0xc0);
        frame.save(c, S1, 0xd0);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S7, 0x70);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0xe0);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, S3, 0xb0);
        c.g[A2][0] = 0x0050_0000;
        frame.save(c, S4, 0xa0);
        frame.save(c, S5, 0x90);
        frame.save(c, S6, 0x80);
        frame.save_ra(c, 0x60);
        let initialised = c.g[V0][0] != 0;
        frame.save_fpr(c, 20, 0xf0);
        if !initialised {
            c.cycles += 14;
            let block = (c.g[A2][0] as u32).wrapping_sub(0x42c0);
            c.g[V1][0] = sx(block);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[A1][0] = Addr(block.wrapping_add(0x10)).lbu(c);
            Addr(block).sw(c, 0);
            c.g[A0][0] = 1;
            let byte = c.g[A1][0];
            Addr(block.wrapping_add(0x10)).sb(c, byte);
            Addr(block.wrapping_add(0x8)).sw(c, u32::MAX as u64);
            c.swc1(0, block.wrapping_add(0xc));
            Addr(w746_gp(c, 0x274c)).sw(c, 1);
            Addr(block.wrapping_add(0x4)).sw(c, 1);
            Addr(sp(c)).sb(c, byte);
        }
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x42c0));
        c.g[A2][0] = 0x14;
        Addr(sp(c).wrapping_add(0x18)).sw(c, u32::MAX as u64);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x0030_518c)?;
        let skip = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !skip {
            c.cycles += 5;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6690));
            c.g[A3][0] = 2;
            w746_param_loop(c, S0, 0x0030_51a8);
        }
        // The object (s5): none without `useObj`, the instance for id -1,
        // else through the world cache.
        c.cycles += 4;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x4)).lw(c);
        c.g[S5][0] = 0;
        let no_obj = c.g[V0][0] == 0;
        c.g[A0][0] = Addr(w746_gp(c, 0xce8)).lw(c);
        if !no_obj {
            c.cycles += 4;
            c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
            c.g[V0][0] = !c.g[A2][0];
            let by_table = c.g[V0][0] != 0;
            c.g[A1][0] = Addr(w746_gp(c, 0x16c8)).lw(c);
            if !by_table {
                c.cycles += 2;
                c.g[S5][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x290)).lw(c);
            } else {
                w746_lookup_object(c);
                c.cycles += 2;
                c.g[A0][0] = Addr(w746_gp(c, 0xce8)).lw(c);
                c.g[S5][0] = c.g[V0][0];
            }
        }
        c.cycles += 4;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[S3][0] = Addr(c.g[A0][0] as u32).lw(c);
        let keyword_set = c.g[A1][0] != 0;
        c.lwc1(20, (c.g[S0][0] as u32).wrapping_add(0xc));
        if keyword_set {
            c.cycles += 1;
            c.g[V1][0] = 0x004d_0000;
        } else {
            c.cycles += 5;
            c.g[V0][0] = Addr(w746_gp(c, 0u32.wrapping_sub(0x848))).lw(c);
            c.g[S2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
            let no_obj = c.g[S5][0] == 0;
            c.g[S4][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x7c)).lw(c);
            if !no_obj {
                w746_shake_by_distance(c)?;
            } else {
                w746_shake_being(c)?;
            }
        }
        c.cycles += 20;
        c.g[V0][0] = c.g[S7][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(A0, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(A0, c.g[A3][0] as u32);
        c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x8)).lw(c);
        c.sdl(A0, (c.g[S7][0] as u32).wrapping_add(0x7));
        c.sdr(A0, c.g[S7][0] as u32);
        let sret = (c.g[S7][0] as u32).wrapping_add(0x8);
        let v = c.g[A1][0];
        Addr(sret).sw(c, v);
        frame.restore(c, S0, 0xe0);
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

/// CameraShake with an object (`s5`): for each player `s3 = 0..s4` shake the
/// local rider's camera by its distance from the object. Leaves `v1` as the
/// generated body does (`0x4d0000` at the end).
fn w746_shake_by_distance(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S4][0] as i64)) as u64;
    let none = c.g[V0][0] == 0;
    c.g[S3][0] = 0;
    if none {
        c.cycles += 1;
        c.g[V1][0] = 0x004d_0000;
        return Ok(());
    }
    c.cycles += 2;
    c.g[S6][0] = sx(w746_gp(c, 0xae8));
    c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
    loop {
        c.cycles += 14;
        c.g[A0][0] = sx((c.g[S3][0] as u32) << 2);
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lw(c);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x6c0)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target, 0x0030_52c4)?;
        c.g[A1][0] = Addr((c.g[S5][0] as u32).wrapping_add(0xc)).lw(c);
        let no_node = c.g[A1][0] == 0;
        c.g[S0][0] = c.g[V0][0];
        if no_node {
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x10));
            c.lq(2, (c.g[V0][0] as u32).wrapping_add(0x30));
        } else {
            c.cycles += 7;
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc0)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc4)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0030_52e4)?;
            c.lq(2, (c.g[V0][0] as u32).wrapping_add(0x30));
        }
        // The distance (VU0): |rider position - object translation|.
        c.cycles += 24;
        c.sq(2, sp(c).wrapping_add(0x40));
        c.lqc2(3, c.g[S0][0] as u32);
        c.lqc2(4, sp(c).wrapping_add(0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c).wrapping_add(0x50));
        c.lq(2, sp(c).wrapping_add(0x50));
        c.sq(2, sp(c).wrapping_add(0x30));
        c.lqc2(3, sp(c).wrapping_add(0x30));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 13);
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lw(c);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x87c)).lw(c);
        let not_local = c.g[V0][0] == 0;
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x870)).lw(c);
        if !not_local {
            c.cycles += 9;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.mul_s(13, 13, 20);
            c.g[A1][0] = Addr(c.g[S6][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4)).lw(c);
            call_ptr(c, 0x0015_e360, 0x0030_5378)?;
        }
        c.cycles += 4;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S4][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0030_5298);
            }
        } else {
            c.cycles += 2;
            c.g[V1][0] = 0x004d_0000;
            return Ok(());
        }
    }
}

/// CameraShake without an object: shake the camera of the local rider whose
/// AI index (virtual call at `+0x3c`) is the script's being (`s3`), once.
fn w746_shake_being(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    let none = (c.g[S4][0] as i64) <= 0;
    c.g[S0][0] = 0;
    if none {
        c.cycles += 1;
        c.g[V1][0] = 0x004d_0000;
        return Ok(());
    }
    c.cycles += 2;
    c.g[S5][0] = sx(w746_gp(c, 0xae8));
    c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
    loop {
        c.cycles += 12;
        c.g[S1][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lw(c);
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x6c0)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x38)).lh(c);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x3c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0030_53c8)?;
        let other = c.g[V0][0] != c.g[S3][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
        if !other {
            c.cycles += 7;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lw(c);
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x87c)).lw(c);
            let not_local = c.g[V0][0] == 0;
            c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x870)).lw(c);
            if not_local {
                c.cycles += 1;
                c.g[V1][0] = 0x004d_0000;
                return Ok(());
            }
            c.cycles += 11;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[A1][0] = Addr(c.g[S5][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.mtc1(0, 13);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4)).lw(c);
            call_ptr(c, 0x0015_e360, 0x0030_5410)?;
            c.g[V1][0] = 0x004d_0000;
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0030_53a0);
            }
        } else {
            c.cycles += 1;
            c.g[V1][0] = 0x004d_0000;
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};

    /// `count` random parameter entries at `DATA + 0x2100` (`idx` below
    /// `idx_range`, random value and type) and the kind table at `table`.
    fn w746_params(c: &mut Cpu, r: &mut Rng, count: u32, idx_range: u32, table: u32) {
        for i in 0..idx_range {
            c.bus.write32(table + 4 * i, r.next_u32() % 3);
        }
        for i in 0..count.max(4) {
            let p = DATA + 0x2100 + 16 * i;
            c.bus.write32(p, r.next_u32() % idx_range);
            c.bus.write32(p + 4, r.next_u32());
            c.bus.write32(p + 0xc, r.next_u32() % 3);
        }
    }

    /// SetRiderConfigBits: first-call flag, kind table at `0x446650`, random
    /// stack junk under the packed word at `sp+0x60`.
    #[test]
    fn wscript_fn_set_rider_config_bits_matches() {
        check(
            ee::s10::f_003045b8__generated,
            wscript_fn_set_rider_config_bits,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x45b8, 0);
                c.bus.write32(GAME_GP.wrapping_add(0xce8), DATA + 0x4000);
                c.bus
                    .write32(GAME_GP.wrapping_add(0x271c), (seed % 2) as u32);
                let count = seed % 5;
                w746_params(c, &mut r, count, 17, 0x0044_6650);
                c.g[A0][0] = DATA as u64 + 0x2000;
                c.g[A1][0] = count as u64;
                c.g[A2][0] = DATA as u64 + 0x2100;
            },
        );
    }

    /// SetObjectField24: the object id (word 0 of the block) either -1 (the
    /// WScriptMan's instance) or a section/slot id into the world cache
    /// tables; the object's node answers the move-node query through a
    /// vtable slot (a fake, or a real leaf returning 0).
    #[test]
    fn wscript_fn_set_object_field24_matches() {
        covered(
            ee::s10::f_00304980__generated,
            wscript_fn_set_object_field24,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x4980, 1);
                let wsm = DATA + 0x4000;
                let (cache, sections, sect, slots) =
                    (DATA + 0x5000, DATA + 0x5100, DATA + 0x5500, DATA + 0x5900);
                let (obj, node, vt) = (DATA + 0x6000, DATA + 0x6100, DATA + 0x6200);
                c.bus.write32(GAME_GP.wrapping_add(0xce8), wsm);
                c.bus
                    .write32(GAME_GP.wrapping_add(0x2720), (seed % 2) as u32);
                c.bus.write32(GAME_GP.wrapping_add(0x16c8), cache);
                c.bus.write32(cache, cache + 0x40);
                c.bus.write32(cache + 0x40 + 8, sections);
                // sections[0..4]: some missing; each has a slot table at +0x1c.
                for sec in 0..4u32 {
                    let s = if (seed >> sec) % 5 == 0 {
                        0
                    } else {
                        sect + 0x40 * sec
                    };
                    c.bus.write32(sections + 4 * sec, s);
                    c.bus.write32(sect + 0x40 * sec + 0x1c, slots + 0x10 * sec);
                    for i in 0..4u32 {
                        let o = if (seed >> (i + 3)) % 3 == 0 { 0 } else { obj };
                        c.bus.write32(
                            slots + 0x10 * sec + 4 * i,
                            ((o >> 2) << 8) | (r.next_u32() & 0xff),
                        );
                    }
                }
                // The object: the instance word at +0x290 of the WScriptMan,
                // or the slot's record. Its node (+0xc) may be null.
                c.bus.write32(wsm + 0x290, obj);
                c.bus
                    .write32(obj + 0xc, if seed % 7 == 0 { 0 } else { node });
                c.bus.write32(node + 0xc, vt);
                c.bus.write16(vt + 0x80, (r.next_u32() & 0xf0) as u16);
                let leaf = if seed % 3 == 0 {
                    0x0014_06c0
                } else {
                    FAKE + 0x20
                };
                c.bus.write32(vt + 0x84, leaf);
                // The parameters: kind table at gp+0xc78 (3 words).
                w746_params(c, &mut r, 3, 3, GAME_GP.wrapping_add(0xc78));
                // Word 0 (the id): -1, or section/slot.
                for i in 0..4u32 {
                    let p = DATA + 0x2100 + 16 * i;
                    if c.bus.read32(p) == 0 {
                        let id = match (seed / 3) % 3 {
                            0 => u32::MAX,
                            _ => ((r.next_u32() % 4) << 8) | (r.next_u32() % 4),
                        };
                        c.bus.write32(p + 4, id);
                        let kind = c.bus.read32(GAME_GP.wrapping_add(0xc78));
                        c.bus.write32(p + 0xc, kind);
                    }
                }
                c.g[A0][0] = DATA as u64 + 0x2000;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = DATA as u64 + 0x2100;
            },
        );
    }

    /// CameraShake: the block parameters (keyword, useObj, id, scale), the
    /// object (the WScriptMan's instance, or one found through the world
    /// cache) with or without a node, and 0..3 riders (local or not) whose
    /// virtual calls are a fake or a real leaf returning 0.
    #[test]
    fn wscript_fn_camera_shake_matches() {
        covered(
            ee::s10::f_003050f0__generated,
            wscript_fn_camera_shake,
            600,
            &|c, seed| {
                c.pc = 0x0030_50f0;
                let mut r = machine(c, seed, 0x50f0, 2);
                c.pc = 0x0030_50f0;
                let wsm = DATA + 0x4000;
                let (cache, sections, sect, slots) =
                    (DATA + 0x4400, DATA + 0x4500, DATA + 0x4900, DATA + 0x4d00);
                let (obj, node, nvt) = (DATA + 0x5000, DATA + 0x5100, DATA + 0x5200);
                let (viewman, game, pb, world) =
                    (DATA + 0x5300, DATA + 0x5400, DATA + 0x5500, DATA + 0x5700);
                c.bus.write32(GAME_GP.wrapping_add(0xce8), wsm);
                c.bus
                    .write32(GAME_GP.wrapping_add(0x274c), (seed % 2) as u32);
                c.bus.write32(GAME_GP.wrapping_add(0x16c8), cache);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                // The being index: 0 (matches a leaf-returning rider) or random.
                c.bus
                    .write32(wsm, if seed % 3 == 0 { 0 } else { r.next_u32() % 4 });
                c.bus.write32(wsm + 0x290, obj);
                c.bus.write32(cache, cache + 0x40);
                c.bus.write32(cache + 0x40 + 8, sections);
                for sec in 0..4u32 {
                    let sec_ptr = if (seed >> sec) % 5 == 0 {
                        0
                    } else {
                        sect + 0x40 * sec
                    };
                    c.bus.write32(sections + 4 * sec, sec_ptr);
                    c.bus.write32(sect + 0x40 * sec + 0x1c, slots + 0x10 * sec);
                    for i in 0..4u32 {
                        let o = if (seed >> (i + 3)) % 3 == 0 { 0 } else { obj };
                        c.bus
                            .write32(slots + 0x10 * sec + 4 * i, ((o >> 2) << 8) | 5);
                    }
                }
                // The object's node (maybe none) and its root-transform call.
                c.bus
                    .write32(obj + 0xc, if seed % 4 == 0 { 0 } else { node });
                c.bus.write32(node + 0xc, nvt);
                c.bus.write16(nvt + 0xc0, (r.next_u32() & 0xf0) as u16);
                c.bus.write32(nvt + 0xc4, FAKE + 0x20);
                // The game: world+0x84 -> game; game+0xc -> player block; game+0x84 -> views.
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0xc, pb);
                c.bus.write32(game + 0x84, viewman);
                c.bus.write32(pb + 0x7c, (seed / 2) % 4);
                for v in 0..4u32 {
                    c.bus.write32(viewman + 4 + 4 * v, DATA + 0x5800 + 0x10 * v);
                }
                for i in 0..3u32 {
                    let (player, rider, vt) = (
                        DATA + 0x5600 + 0x40 * i,
                        DATA + 0x6000 + 0x900 * i,
                        DATA + 0x8000 + 0x80 * i,
                    );
                    c.bus.write32(pb + 0x40 + 4 * i, player);
                    c.bus.write32(player + 0x18, rider);
                    c.bus.write32(rider + 0x6c0, vt);
                    c.bus.write32(rider + 0x870, r.next_u32() % 4);
                    c.bus
                        .write32(rider + 0x87c, ((seed >> (i + 1)) % 3 != 0) as u32);
                    c.bus.write16(vt + 0x28, (r.next_u32() & 0x70) as u16);
                    c.bus.write32(vt + 0x2c, FAKE + 0x20);
                    c.bus.write16(vt + 0x38, (r.next_u32() & 0x70) as u16);
                    let ai = if (seed + i) % 2 == 0 {
                        0x0014_06c0
                    } else {
                        FAKE + 0x20
                    };
                    c.bus.write32(vt + 0x3c, ai);
                }
                // The shake parameter table.
                c.bus.write32(GAME_GP.wrapping_add(0xae8), r.next_u32());
                // The parameters: every field, kind table at 0x446690.
                w746_params(c, &mut r, 4, 4, 0x0044_6690);
                for i in 0..4u32 {
                    let p = DATA + 0x2100 + 16 * i;
                    c.bus.write32(p, i);
                    let kind = c.bus.read32(0x0044_6690 + 4 * i);
                    c.bus.write32(p + 0xc, kind);
                    let v = match i {
                        0 => (r.next_u32() % 4 == 0) as u32,
                        1 => (r.next_u32() % 4 != 0) as u32,
                        2 => match (seed / 5) % 3 {
                            0 => u32::MAX,
                            _ => ((r.next_u32() % 4) << 8) | (r.next_u32() % 4),
                        },
                        _ => r.next_u32(),
                    };
                    c.bus.write32(p + 4, v);
                }
                c.g[A0][0] = DATA as u64 + 0x2000;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = DATA as u64 + 0x2100;
            },
        );
    }
}
