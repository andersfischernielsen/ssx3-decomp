//! Staging module oc690 (wave wave237): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc690`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0037_c198,
        ps2_graphics_man_set_dither_matrix,
        "PS2GraphicsMan_SetDitherMatrix",
    ),
    (
        0x0037_c808,
        ps2_graphics_man_snapshot_texture_table,
        "PS2GraphicsMan_SnapshotTextureTable",
    ),
    (
        0x0037_d738,
        ps2_graphics_man_lock_texture,
        "PS2GraphicsMan_LockTexture",
    ),
    (
        0x0038_2740,
        renderer_dma_kick_thread_entry,
        "Renderer_DmaKickThreadEntry",
    ),
    (
        0x0038_2760,
        renderer_dma_kick_thread,
        "Renderer_DmaKickThread",
    ),
    (
        0x0038_66e0,
        ps2_graphics_man_set_phase2_callback,
        "PS2GraphicsMan_SetPhase2Callback",
    ),
    (0x0038_66f0, ps2_graphics_man_v109, "PS2GraphicsMan_v109"),
    (0x0039_5288, ps2_graphics_man_ctor, "PS2GraphicsMan_ctor"),
    (
        0x0039_53b0,
        ps2_graphics_man_get_fog_state,
        "PS2GraphicsMan_GetFogState",
    ),
    (
        0x0039_5420,
        ps2_graphics_man_clear_lights,
        "PS2GraphicsMan_ClearLights",
    ),
    (0x0039_5508, ps2_graphics_man_v110, "PS2GraphicsMan_v110"),
    (
        0x0039_5510,
        ps2_graphics_man_get_flip_count,
        "PS2GraphicsMan_GetFlipCount",
    ),
];

/// `void PS2GraphicsMan_SnapshotTextureTable(PS2GraphicsMan* gfx)` (0x37c808;
/// the reference calls it `PlatformUploadTextures`): copy the 0x1f40-byte
/// texture table at `gfx+0x3838` to `gfx+0x18f8` with `EA_MemCopy(dst, src, n)`.
pub fn ps2_graphics_man_snapshot_texture_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3838));
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x1f40;
        c.g[RA][0] = sx(0x0037_c820);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18f8));
        call_ptr(c, 0x003e_6574, 0x0037_c820)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2GraphicsMan_LockTexture(PS2GraphicsMan* gfx, a1, a2, a3)`
/// (0x37d738): `TexMan_LockTexture(gfx->texman (+0x18f4), a1, a2, a3)`.
pub fn ps2_graphics_man_lock_texture(c: &mut Cpu) {
    tail_wrapper(c, Some(0x18f4), Target::Table(0x0036_7cd0), 0x0037_d748)
}

/// `void Renderer_DmaKickThreadEntry(gfx)` (0x382740): thread entry that runs
/// `Renderer_DmaKickThread(gfx)` (which does not return).
pub fn renderer_dma_kick_thread_entry(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0038_2760), 0x0038_2750)
}

/// `void PS2GraphicsMan_SetPhase2Callback(gfx, fn, arg)` (0x3866e0; the
/// reference calls it `SetLensFXCallBack`): store the phase-2 callback
/// (`gfx+0x6d0c`) and its argument (`+0x6d10`) that the DMA kick thread
/// calls.
pub fn ps2_graphics_man_set_phase2_callback(c: &mut Cpu) {
    c.cycles += 3;
    let gfx = c.g[A0][0] as u32;
    let arg = c.g[A2][0];
    let func = c.g[A1][0];
    Addr(gfx.wrapping_add(0x6d10)).sw(c, arg);
    ret(c);
    Addr(gfx.wrapping_add(0x6d0c)).sw(c, func);
}

/// `void* PS2GraphicsMan_GetFogState(gfx)` (0x3953b0): the fog state block
/// embedded at `gfx+0x6b94`.
pub fn ps2_graphics_man_get_fog_state(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6b94));
}

/// `void PS2GraphicsMan_ClearLights(gfx)` (0x395420): zero the light count
/// at `gfx+0x13ec`.
pub fn ps2_graphics_man_clear_lights(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
    let gfx = c.g[A0][0] as u32;
    Addr(gfx.wrapping_add(0x13ec)).sw(c, 0);
}

/// `void PS2GraphicsMan_v110(gfx)` (0x395508): empty virtual slot.
pub fn ps2_graphics_man_v110(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `int PS2GraphicsMan_GetFlipCount(gfx)` (0x395510): the frame flip counter
/// at `gfx+0x5a74`.
pub fn ps2_graphics_man_get_flip_count(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
    let gfx = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(gfx.wrapping_add(0x5a74)).lw(c);
}

/// `PS2GraphicsMan* PS2GraphicsMan_ctor(PS2GraphicsMan* self)` (0x395288):
/// run the base-class constructor (`cGraphicsMan_ctor`, 0x3691f8), set the
/// vtable word at `self+0x10d8` (0x493260), burn the two short counted
/// loops the compiler left (12 and 4 passes, no stores in the generated
/// body) and return `self`.
pub fn ps2_graphics_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0039_529c);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0036_91f8, 0x0039_529c)?;
        c.g[V0][0] = 0x49_0000;
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3260));
        c.g[V1][0] = 0xb;
        let vtable = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x10d8)).sw(c, vtable);
        'l_3952b0: loop {
            c.cycles += 7;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let more = c.g[V1][0] != c.g[A0][0];
            c.g[V0][0] = 0x7;
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0039_52b0);
                }
                continue 'l_3952b0;
            } else {
                break 'l_3952b0;
            }
        }
        c.cycles += 7;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 0x1;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 0x1;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 0x3;
        c.g[V1][0] = u64::MAX;
        'l_3952e8: loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let more = c.g[V0][0] != c.g[V1][0];
            frame.restore_ra(c, 0);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0039_52e8);
                }
                continue 'l_3952e8;
            } else {
                break 'l_3952e8;
            }
        }
        c.cycles += 4;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2GraphicsMan_SetDitherMatrix(PS2GraphicsMan* gfx, const int* m)`
/// (0x37c198): pack the 4x4 GS dither matrix. Sixteen 3-bit entries
/// (`m[i] & 7`, i = 0..15) go into the 64-bit DIMX word at `gfx+0x59e0`,
/// entry i at bit `4*i`; the straight-line body rewrites the word after each
/// entry (read-modify-write, as the compiler left it) and the code leaves its
/// mask constants in the scratch registers, which are reproduced.
#[allow(unused_labels, unused_assignments)]
pub fn ps2_graphics_man_set_dither_matrix(c: &mut Cpu) {
    c.cycles += 246;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.g[A2][0] = 0xfffffffffffffff8;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = c.g[V1][0] & 0x7;
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[A3][0] = 0xffffffffffffff8f;
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[T0][0] = 0xfffffffffffff8ff;
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[T1][0] = 0xffffffffffff8fff;
    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[T2][0] = 0xffffffffff8f0000;
    c.g[T2][0] = c.g[T2][0] | 0xffff;
    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[T3][0] = 0xffffffff8fff0000;
    c.g[T3][0] = c.g[T3][0] | 0xffff;
    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[T4][0] = 0xfffffffffffffff8;
    c.g[T4][0] = c.g[T4][0] << 16;
    c.g[T4][0] = c.g[T4][0] | 0xffff;
    c.g[T4][0] = c.g[T4][0] << 16;
    c.g[T4][0] = c.g[T4][0] | 0xffff;
    c.sq(30, (c.g[SP][0] as u32));
    c.g[T5][0] = 0xffffffffffffff8f;
    c.g[T5][0] = c.g[T5][0] << 16;
    c.g[T5][0] = c.g[T5][0] | 0xffff;
    c.g[T5][0] = c.g[T5][0] << 16;
    c.g[T5][0] = c.g[T5][0] | 0xffff;
    c.g[T6][0] = 0xfffffffffffff8ff;
    c.g[T6][0] = c.g[T6][0] << 16;
    c.g[T6][0] = c.g[T6][0] | 0xffff;
    c.g[T6][0] = c.g[T6][0] << 16;
    c.g[T6][0] = c.g[T6][0] | 0xffff;
    c.g[T7][0] = 0xffffffffffff8fff;
    c.g[T7][0] = c.g[T7][0] << 16;
    c.g[T7][0] = c.g[T7][0] | 0xffff;
    c.g[T7][0] = c.g[T7][0] << 16;
    c.g[T7][0] = c.g[T7][0] | 0xffff;
    c.g[V0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x59e0));
    c.g[S0][0] = 0xfffffffffff80000;
    c.g[S0][0] = c.g[S0][0] | 0xffff;
    c.g[S0][0] = c.g[S0][0] << 16;
    c.g[S0][0] = c.g[S0][0] | 0xffff;
    c.g[S0][0] = c.g[S0][0] << 16;
    c.g[S0][0] = c.g[S0][0] | 0xffff;
    c.g[S1][0] = 0xffffffffff8f0000;
    c.g[S1][0] = c.g[S1][0] | 0xffff;
    c.g[S1][0] = c.g[S1][0] << 16;
    c.g[S1][0] = c.g[S1][0] | 0xffff;
    c.g[S1][0] = c.g[S1][0] << 16;
    c.g[S1][0] = c.g[S1][0] | 0xffff;
    c.g[S3][0] = 0xfffffffff8ff0000;
    c.g[S3][0] = c.g[S3][0] | 0xffff;
    c.g[S3][0] = c.g[S3][0] << 16;
    c.g[S3][0] = c.g[S3][0] | 0xffff;
    c.g[S3][0] = c.g[S3][0] << 16;
    c.g[S3][0] = c.g[S3][0] | 0xffff;
    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
    c.g[T9][0] = 0xffffffffff7f0000;
    c.g[T9][0] = c.g[T9][0] | 0xffff;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[A2][0] = 0xfffffffffff80000;
    c.g[A2][0] = c.g[A2][0] | 0xffff;
    c.sd((c.g[A0][0] as u32).wrapping_add(0x59e0), c.g[V0][0]);
    c.g[FP][0] = 0xfffffffff7ff0000;
    c.g[FP][0] = c.g[FP][0] | 0xffff;
    c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
    c.g[S6][0] = 0xffffffffffffffff;
    c.g[S6][0] = c.g[S6][0] << 16;
    c.g[S6][0] = c.g[S6][0] | 0x7fff;
    c.g[S6][0] = c.g[S6][0] << 16;
    c.g[S6][0] = c.g[S6][0] | 0xffff;
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[A3][0] = 0xfffffffff8ff0000;
    c.g[A3][0] = c.g[A3][0] | 0xffff;
    c.g[S7][0] = 0xfffffffffffffff7;
    c.g[S7][0] = c.g[S7][0] << 16;
    c.g[S7][0] = c.g[S7][0] | 0xffff;
    c.g[S7][0] = c.g[S7][0] << 16;
    c.g[S7][0] = c.g[S7][0] | 0xffff;
    c.g[T8][0] = 0xffffffffffffff7f;
    c.g[T8][0] = c.g[T8][0] << 16;
    c.g[T8][0] = c.g[T8][0] | 0xffff;
    c.g[T8][0] = c.g[T8][0] << 16;
    c.g[T8][0] = c.g[T8][0] | 0xffff;
    c.g[V1][0] = c.g[V1][0] & 0x7;
    c.g[S5][0] = 0xfffffffffffff7ff;
    c.g[S5][0] = c.g[S5][0] << 16;
    c.g[S5][0] = c.g[S5][0] | 0xffff;
    c.g[S5][0] = c.g[S5][0] << 16;
    c.g[S5][0] = c.g[S5][0] | 0xffff;
    c.g[V1][0] = c.g[V1][0] << 4;
    c.g[S4][0] = 0xffffffffffff0000;
    c.g[S4][0] = c.g[S4][0] | 0x7fff;
    c.g[S4][0] = c.g[S4][0] << 16;
    c.g[S4][0] = c.g[S4][0] | 0xffff;
    c.g[S4][0] = c.g[S4][0] << 16;
    c.g[S4][0] = c.g[S4][0] | 0xffff;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[S2][0] = 0xfffffffffff70000;
    c.g[S2][0] = c.g[S2][0] | 0xffff;
    c.g[S2][0] = c.g[S2][0] << 16;
    c.g[S2][0] = c.g[S2][0] | 0xffff;
    c.g[S2][0] = c.g[S2][0] << 16;
    c.g[S2][0] = c.g[S2][0] | 0xffff;
    c.sd((c.g[A0][0] as u32).wrapping_add(0x59e0), c.g[V0][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
    c.g[T0][0] = 0xffffffffff7f0000;
    c.g[T0][0] = c.g[T0][0] | 0xffff;
    c.g[T0][0] = c.g[T0][0] << 16;
    c.g[T0][0] = c.g[T0][0] | 0xffff;
    c.g[T0][0] = c.g[T0][0] << 16;
    c.g[T0][0] = c.g[T0][0] | 0xffff;
    c.g[V1][0] = c.g[V1][0] & 0x7;
    c.g[V1][0] = c.g[V1][0] << 8;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd((c.g[A0][0] as u32).wrapping_add(0x59e0), c.g[V0][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
    c.g[T1][0] = 0xfffffffff7ff0000;
    c.g[T1][0] = c.g[T1][0] | 0xffff;
    c.g[T1][0] = c.g[T1][0] << 16;
    c.g[T1][0] = c.g[T1][0] | 0xffff;
    c.g[T1][0] = c.g[T1][0] << 16;
    c.g[T1][0] = c.g[T1][0] | 0xffff;
    c.g[V1][0] = c.g[V1][0] & 0x7;
    c.g[V1][0] = c.g[V1][0] << 12;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd((c.g[A0][0] as u32).wrapping_add(0x59e0), c.g[V0][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
    c.g[A2][0] = 0xffffffffffffffff;
    c.g[A2][0] = c.g[A2][0] >> 1;
    c.g[V1][0] = c.g[V1][0] & 0x7;
    c.g[V1][0] = c.g[V1][0] << 16;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd((c.g[A0][0] as u32).wrapping_add(0x59e0), c.g[V0][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
    c.g[V1][0] = c.g[V1][0] & 0x7;
    c.g[V1][0] = c.g[V1][0] << 20;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd((c.g[A0][0] as u32).wrapping_add(0x59e0), c.g[V0][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
    c.g[V1][0] = c.g[V1][0] & 0x7;
    c.g[V1][0] = c.g[V1][0] << 24;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd((c.g[A0][0] as u32).wrapping_add(0x59e0), c.g[V0][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
    c.g[V1][0] = c.g[V1][0] & 0x7;
    c.g[V1][0] = c.g[V1][0] << 28;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd((c.g[A0][0] as u32).wrapping_add(0x59e0), c.g[V0][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = c.g[V1][0] & 0x7;
    c.g[V1][0] = c.g[V1][0] << 32;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd((c.g[A0][0] as u32).wrapping_add(0x59e0), c.g[V0][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
    c.g[V1][0] = c.g[V1][0] & 0x7;
    c.g[V1][0] = c.g[V1][0] << 36;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd((c.g[A0][0] as u32).wrapping_add(0x59e0), c.g[V0][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[T6][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x28));
    c.g[V1][0] = c.g[V1][0] & 0x7;
    c.g[V1][0] = c.g[V1][0] << 40;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd((c.g[A0][0] as u32).wrapping_add(0x59e0), c.g[V0][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[T7][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
    c.g[V1][0] = c.g[V1][0] & 0x7;
    c.g[V1][0] = c.g[V1][0] << 44;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd((c.g[A0][0] as u32).wrapping_add(0x59e0), c.g[V0][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x30));
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.g[V1][0] = c.g[V1][0] & 0x7;
    c.g[V1][0] = c.g[V1][0] << 48;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd((c.g[A0][0] as u32).wrapping_add(0x59e0), c.g[V0][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x34));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = c.g[V1][0] & 0x7;
    c.g[V1][0] = c.g[V1][0] << 52;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd((c.g[A0][0] as u32).wrapping_add(0x59e0), c.g[V0][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x38));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[V1][0] = c.g[V1][0] & 0x7;
    c.g[V1][0] = c.g[V1][0] << 56;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd((c.g[A0][0] as u32).wrapping_add(0x59e0), c.g[V0][0]);
    c.g[V1][0] = 0xffffffff8fff0000;
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    c.g[V1][0] = c.g[V1][0] << 16;
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    c.g[V1][0] = c.g[V1][0] << 16;
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x3c));
    c.g[V1][0] = c.g[V1][0] & 0x7;
    c.g[V1][0] = c.g[V1][0] << 60;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[V1][0] = 0xfffffffffffffff7;
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V1][0] = 0xffffffffffffff7f;
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V1][0] = 0xfffffffffffff7ff;
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V1][0] = 0xffffffffffff0000;
    c.g[V1][0] = c.g[V1][0] | 0x7fff;
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V1][0] = 0xfffffffffff70000;
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] & c.g[T9][0];
    c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
    c.g[V0][0] = c.g[V0][0] & c.g[S6][0];
    c.lq(30, (c.g[SP][0] as u32));
    c.g[V0][0] = c.g[V0][0] & c.g[S7][0];
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V0][0] = c.g[V0][0] & c.g[T8][0];
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.g[V0][0] & c.g[S5][0];
    c.g[V0][0] = c.g[V0][0] & c.g[S4][0];
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
    c.sd((c.g[A0][0] as u32).wrapping_add(0x59e0), c.g[V0][0]);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = t;
}

/// `void* Renderer_DmaKickThread(PS2GraphicsMan* gfx)` (0x382760): the
/// render DMA kick thread. Never returns normally: each turn waits on the two
/// semaphores at `gfx+0x5ac8` / `+0x5acc`, then by the state word at
/// `gfx+0x5a8c` either finishes a display-register swap (5), starts the next
/// VIF1/GIF display-list DMA after polling the DMA/VIF/GS FIFOs idle (0, when
/// the queue entry says 2), or calls the phase-2 callback (`+0x6d0c`, arg
/// `+0x6d10`) and starts the second list (2); then signals the second
/// semaphore. It leaves only through an unwinding callee (a blocked
/// `WaitSema`) or its dead tail call.
///
/// A transcription of the generated body (every block, register and cycle):
/// it is hardware polling around semaphore calls, with nothing to simplify.
#[allow(unused_labels, unused_assignments)]
pub fn renderer_dma_kick_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5a90));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S1][0] = 0x10000000;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S3][0] = 0x10000000;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S4][0] = 0x10000000;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S5][0] = 0x10000000;
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S6][0] = 0x10000000;
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S7][0] = 0x10000000;
        c.g[FP][0] = 0x10000000;
        c.g[S2][0] = 0x1f000000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[S1][0] = c.g[S1][0] | 0x9000;
        c.sw((c.g[SP][0] as u32), c.g[V0][0]);
        c.g[FP][0] = c.g[FP][0] | 0x5000;
        c.g[S7][0] = c.g[S7][0] | 0x3010;
        c.g[S6][0] = c.g[S6][0] | 0xa030;
        c.g[S5][0] = c.g[S5][0] | 0xa000;
        c.g[S4][0] = c.g[S4][0] | 0x9030;
        c.g[S3][0] = c.g[S3][0] | 0x3c00;
        c.g[S2][0] = c.g[S2][0] | 0x3;
        'l_3827d8: loop {
            'b_382824: {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003827e0);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5ac8));
                call_ptr(c, 0x00423de0, 0x003827e0)?;
                c.g[RA][0] = sx(0x003827e8);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5acc));
                call_ptr(c, 0x00423de0, 0x003827e8)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a8c));
                c.g[V0][0] = 0x5;
                let t = c.g[V1][0] != c.g[V0][0];
                if t {
                    break 'b_382824;
                } else {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a98));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32));
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A2][0] = 0u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x5a8c), 0u64);
                    c.g[RA][0] = sx(0x00382824);
                    call_ptr(c, 0x00382af0, 0x00382824)?;
                    break 'b_382824;
                }
            }
            'b_382ae0: {
                'b_382a18: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a8c));
                    let t = c.g[V0][0] != 0u64;
                    if t {
                        break 'b_382a18;
                    } else {
                        c.cycles += 9;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5aa4));
                        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5a90));
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                        c.g[V0][0] = 0x2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        if t {
                            break 'b_382a18;
                        } else {
                            'b_382884: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                c.g[V0][0] = c.g[V0][0] & 0x100;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V1][0] = 0x10000000;
                                if t {
                                    break 'b_382884;
                                } else {
                                    c.cycles += 1;
                                    c.g[V1][0] = c.g[V1][0] | 0x9000;
                                    'l_382868: loop {
                                        c.cycles += 7;
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                        c.g[V0][0] = c.g[V0][0] & 0x100;
                                        let t = c.g[V0][0] != 0u64;
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00382868);
                                            }
                                            continue 'l_382868;
                                        } else {
                                            break 'b_382884;
                                        }
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V1][0] = 0x10000000;
                            c.g[V1][0] = c.g[V1][0] | 0xa000;
                            'l_382890: loop {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                c.g[V0][0] = c.g[V0][0] & 0x100;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = 0x10000000;
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00382890);
                                    }
                                    continue 'l_382890;
                                } else {
                                    c.cycles += 3;
                                    c.g[V1][0] = 0x1f000000;
                                    c.g[A0][0] = c.g[A0][0] | 0x3c00;
                                    c.g[V1][0] = c.g[V1][0] | 0x3;
                                    'l_3828b8: loop {
                                        c.cycles += 7;
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                        let t = c.g[V0][0] != 0u64;
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x003828b8);
                                            }
                                            continue 'l_3828b8;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V1][0] = 0x12000000;
                                            c.g[A0][0] = 0x4000;
                                            c.g[V1][0] = c.g[V1][0] | 0x1000;
                                            'l_3828e0: loop {
                                                c.cycles += 7;
                                                c.g[V0][0] = c.ld((c.g[V1][0] as u32));
                                                c.g[V0][0] = c.g[V0][0] & 0xc000;
                                                let t = c.g[V0][0] != c.g[A0][0];
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x003828e0);
                                                    }
                                                    continue 'l_3828e0;
                                                } else {
                                                    'b_382974: {
                                                        c.cycles += 21;
                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                            .wrapping_add(0x5aa4));
                                                        c.g[A1][0] = 0x1;
                                                        c.g[A0][0] = 0u64;
                                                        c.sw(
                                                            (c.g[S0][0] as u32)
                                                                .wrapping_add(0x5a98),
                                                            c.g[V1][0],
                                                        );
                                                        c.g[V1][0] = 0x3;
                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                            .wrapping_add(0x5a98));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[A2][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                                                        c.sw(
                                                            (c.g[S0][0] as u32)
                                                                .wrapping_add(0x5a8c),
                                                            c.g[A1][0],
                                                        );
                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                            .wrapping_add(0x5aac));
                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                            .wrapping_add(0x5ab4));
                                                        c.sw(
                                                            (c.g[S0][0] as u32)
                                                                .wrapping_add(0x5a9c),
                                                            c.g[V0][0],
                                                        );
                                                        c.g[RA][0] = sx(0x00382938);
                                                        c.sw(
                                                            (c.g[S0][0] as u32)
                                                                .wrapping_add(0x5aa0),
                                                            c.g[V1][0],
                                                        );
                                                        call_ptr(c, 0x00424020, 0x00382938)?;
                                                        c.g[V0][0] = 0x10000000;
                                                        c.g[V0][0] = c.g[V0][0] | 0xd000;
                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                        c.g[V1][0] = c.g[V1][0] & 0x100;
                                                        let t = c.g[V1][0] == 0u64;
                                                        c.g[V1][0] = 0x10000000;
                                                        if t {
                                                            break 'b_382974;
                                                        } else {
                                                            c.cycles += 2;
                                                            c.g[V1][0] = c.g[V1][0] | 0xd000;
                                                            'l_382958: loop {
                                                                c.cycles += 7;
                                                                c.g[V0][0] =
                                                                    c.lw((c.g[V1][0] as u32));
                                                                c.g[V0][0] = c.g[V0][0] & 0x100;
                                                                let t = c.g[V0][0] != 0u64;
                                                                if t {
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c, 0x00382958,
                                                                        );
                                                                    }
                                                                    continue 'l_382958;
                                                                } else {
                                                                    break 'b_382974;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_3829c4: {
                                                        c.cycles += 10;
                                                        c.g[V1][0] = 0x450000;
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_sub(0x4640));
                                                        c.lq(2, (c.g[V1][0] as u32));
                                                        c.sq(2, (c.g[FP][0] as u32));
                                                        c.g[V0][0] = 0x4;
                                                        c.sw((c.g[S7][0] as u32), c.g[V0][0]);
                                                        c.g[V0][0] = c.lw((c.g[S3][0] as u32));
                                                        c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = 0x10000000;
                                                        if t {
                                                            break 'b_3829c4;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V1][0] = 0x1f000000;
                                                            c.g[A0][0] = c.g[A0][0] | 0x3c00;
                                                            c.g[V1][0] = c.g[V1][0] | 0x3;
                                                            'l_3829a8: loop {
                                                                c.cycles += 7;
                                                                c.g[V0][0] =
                                                                    c.lw((c.g[A0][0] as u32));
                                                                c.g[V0][0] =
                                                                    c.g[V0][0] & c.g[V1][0];
                                                                let t = c.g[V0][0] != 0u64;
                                                                if t {
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c, 0x003829a8,
                                                                        );
                                                                    }
                                                                    continue 'l_3829a8;
                                                                } else {
                                                                    break 'b_3829c4;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[V1][0] = 0x12000000;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[S0][0] as u32)
                                                            .wrapping_add(0x5ab0));
                                                    c.g[V1][0] = c.g[V1][0] | 0x1000;
                                                    c.g[A0][0] = 0x4000;
                                                    'l_3829d8: loop {
                                                        c.cycles += 7;
                                                        c.g[V0][0] = c.ld((c.g[V1][0] as u32));
                                                        c.g[V0][0] = c.g[V0][0] & 0xc000;
                                                        let t = c.g[V0][0] != c.g[A0][0];
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x003829d8);
                                                            }
                                                            continue 'l_3829d8;
                                                        } else {
                                                            c.cycles += 9;
                                                            c.sw((c.g[S6][0] as u32), c.g[A1][0]);
                                                            c.g[V1][0] = 0x105;
                                                            c.sw((c.g[S5][0] as u32), c.g[V1][0]);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x5aa8));
                                                            c.sw((c.g[S4][0] as u32), c.g[V0][0]);
                                                            c.g[V0][0] = 0x185;
                                                            c.sw((c.g[S1][0] as u32), c.g[V0][0]);
                                                            let t = true;
                                                            if t {
                                                                break 'b_382ae0;
                                                            } else {
                                                                break 'b_382a18;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a8c));
                c.g[V1][0] = 0x2;
                let t = c.g[V0][0] != c.g[V1][0];
                if t {
                    break 'b_382ae0;
                } else {
                    'b_382a40: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d0c));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = 0x450000;
                        if t {
                            break 'b_382a40;
                        } else {
                            c.cycles += 3;
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x00382a3c);
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d10));
                            call_ptr(c, t, 0x00382a3c)?;
                            c.g[V1][0] = 0x450000;
                            break 'b_382a40;
                        }
                    }
                    'b_382a8c: {
                        c.cycles += 9;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4640));
                        c.lq(2, (c.g[V1][0] as u32));
                        c.sq(2, (c.g[FP][0] as u32));
                        c.g[V0][0] = 0x4;
                        c.sw((c.g[S7][0] as u32), c.g[V0][0]);
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32));
                        c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = 0x10000000;
                        if t {
                            break 'b_382a8c;
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = 0x1f000000;
                            c.g[A0][0] = c.g[A0][0] | 0x3c00;
                            c.g[V1][0] = c.g[V1][0] | 0x3;
                            'l_382a70: loop {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                let t = c.g[V0][0] != 0u64;
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00382a70);
                                    }
                                    continue 'l_382a70;
                                } else {
                                    break 'b_382a8c;
                                }
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V1][0] = 0x12000000;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5aa0));
                    c.g[V1][0] = c.g[V1][0] | 0x1000;
                    c.g[A0][0] = 0x4000;
                    'l_382aa0: loop {
                        c.cycles += 7;
                        c.g[V0][0] = c.ld((c.g[V1][0] as u32));
                        c.g[V0][0] = c.g[V0][0] & 0xc000;
                        let t = c.g[V0][0] != c.g[A0][0];
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00382aa0);
                            }
                            continue 'l_382aa0;
                        } else {
                            c.cycles += 9;
                            c.sw((c.g[S6][0] as u32), c.g[A1][0]);
                            c.g[V1][0] = 0x105;
                            c.sw((c.g[S5][0] as u32), c.g[V1][0]);
                            c.g[V0][0] = 0x3;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x5a8c), c.g[V0][0]);
                            c.g[V1][0] = 0x185;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a9c));
                            c.sw((c.g[S4][0] as u32), c.g[V0][0]);
                            c.sw((c.g[S1][0] as u32), c.g[V1][0]);
                            break 'b_382ae0;
                        }
                    }
                }
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x00382ae8);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5acc));
            call_ptr(c, 0x00423dc0, 0x00382ae8)?;
            let t = true;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x003827d8);
                }
                continue 'l_3827d8;
            } else {
                c.pc = 0x00382af0;
                call_indirect(c, 0x00382af0);
                return Ok(());
            }
        }
    })
}

/// `void PS2GraphicsMan_v109(gfx, model, base, tbl, count, slots, x)`
/// (0x3866f0; the reference calls it `ResolveCharModel`): relocate a loaded
/// character model in place. Offsets in the model header (blends, refs, mats,
/// bones) become pointers into `base`; reference index arrays, bone track and
/// key tables are rebased; each bone's name is resolved to a texture id
/// (`GetTextureByName`, 0x3696c8) with its alpha, `ea`/`alph` flag and `envN`
/// environment slot; material texture indices are remapped through `tbl`;
/// each blended-matrix list is rebased, remapped through `tbl`, registered
/// (`AllocBlendedMatrixHistory`) and de-duplicated against the shared `slots`
/// (`MatchBlendedMatrix`, `*count` entries); finally the reference arrays
/// are rewritten from blend index to matrix id.
///
/// A transcription of the generated body (blocks, registers, cycles and call
/// order); the nested loops are the original's.
#[allow(unused_labels, unused_assignments)]
pub fn ps2_graphics_man_v109(c: &mut Cpu) {
    guest(c, |c| {
        'b_3867a0: {
            c.cycles += 32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S4][0] = c.g[A1][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S6][0] = c.g[T0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S7][0] = c.g[A2][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x34));
            c.sw((c.g[SP][0] as u32), c.g[A0][0]);
            c.g[V1][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x30));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A3][0]);
            c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x18));
            c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A0][0]);
            c.g[S3][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x46));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[T1][0]);
            c.sw((c.g[S4][0] as u32).wrapping_add(0x38), c.g[V1][0]);
            c.sw((c.g[S4][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
            let t = (c.g[A2][0] as i64) <= 0;
            c.sw((c.g[S4][0] as u32).wrapping_add(0x40), c.g[T2][0]);
            if t {
                break 'b_3867a0;
            } else {
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x3c));
                'l_386778: loop {
                    c.cycles += 10;
                    c.g[A0][0] = sx((c.g[A1][0] as u32) << 3);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V1][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x46));
                    c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V1][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x3c));
                        if event_due(c) {
                            deliver_event_here(c, 0x00386778);
                        }
                        continue 'l_386778;
                    } else {
                        break 'b_3867a0;
                    }
                }
            }
        }
        'b_3869a8: {
            c.cycles += 3;
            c.g[V0][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x48));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S2][0] = 0u64;
            if t {
                break 'b_3869a8;
            } else {
                c.cycles += 3;
                c.g[FP][0] = 0x14;
                c.mult(0, 3, 18, 30);
                'l_3867b8: loop {
                    'b_386898: {
                        c.cycles += 9;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        c.g[V1][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        let t = (c.g[A0][0] as i64) <= 0;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                        if t {
                            break 'b_386898;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = 0x14;
                            c.mult(0, 2, 18, 2);
                            c.g[T4][0] = c.g[V0][0];
                            c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                            c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10));
                            'l_3867f0: loop {
                                'b_386884: {
                                    c.cycles += 11;
                                    c.g[A3][0] = sx((c.g[A1][0] as u32) << 3);
                                    c.g[T3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                    c.g[V0][0] =
                                        sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                                    c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10));
                                    c.g[V1][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                    let t = (c.g[V0][0] as i64) <= 0;
                                    c.g[T0][0] = 0u64;
                                    if t {
                                        break 'b_386884;
                                    } else {
                                        c.cycles += 3;
                                        c.g[T1][0] =
                                            sx((c.g[T4][0] as u32)
                                                .wrapping_add((c.g[S3][0] as u32)));
                                        c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x10));
                                        'l_386828: loop {
                                            c.cycles += 23;
                                            c.g[A1][0] = sx((c.g[T0][0] as u32) << 3);
                                            c.g[A2][0] = c.g[A3][0];
                                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                            c.g[V0][0] = sx((c.g[A3][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                            c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                            c.g[V0][0] = sx((c.g[S7][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                                            c.g[V1][0] =
                                                c.lw((c.g[T1][0] as u32).wrapping_add(0x10));
                                            c.g[V1][0] = sx((c.g[A3][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[A0][0] =
                                                c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                                            c.g[V1][0] =
                                                c.lw((c.g[T1][0] as u32).wrapping_add(0x10));
                                            c.g[A2][0] = sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[V0][0] =
                                                    c.lw((c.g[T1][0] as u32).wrapping_add(0x10));
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00386828);
                                                }
                                                continue 'l_386828;
                                            } else {
                                                break 'b_386884;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0xc));
                                c.g[A1][0] = c.g[T3][0];
                                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003867f0);
                                    }
                                    continue 'l_3867f0;
                                } else {
                                    break 'b_386898;
                                }
                            }
                        }
                    }
                    'b_3868dc: {
                        'b_3868d8: {
                            c.cycles += 8;
                            c.mult(0, 2, 18, 30);
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                            c.g[S0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x10));
                            let t = c.g[V0][0] == 0u64;
                            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                            if t {
                                break 'b_3868d8;
                            } else {
                                c.cycles += 8;
                                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x14));
                                c.g[AT][0] = 0x43000000;
                                c.mtc1(1, 1);
                                c.mul_s(0, 0, 1);
                                c.cvt_w_s(1, 0);
                                c.mfc1(2, 1);
                                let t = true;
                                c.sh((c.g[V1][0] as u32).wrapping_add(0xa), c.g[V0][0]);
                                if t {
                                    break 'b_3868dc;
                                } else {
                                    break 'b_3868d8;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sh((c.g[V1][0] as u32).wrapping_add(0xa), 0u64);
                        break 'b_3868dc;
                    }
                    'b_38692c: {
                        'b_386914: {
                            c.cycles += 7;
                            c.g[V0][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4398));
                            c.g[RA][0] = sx(0x003868f0);
                            c.g[A2][0] = 0x4;
                            call_ptr(c, 0x00416b18, 0x003868f0)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = 0x65;
                            if t {
                                break 'b_386914;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lb((c.g[S0][0] as u32));
                                if c.g[V0][0] != c.g[V1][0] {
                                    c.g[V0][0] = c.lb((c.g[S1][0] as u32));
                                    break 'b_38692c;
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = 0x61;
                                    if c.g[V1][0] != c.g[V0][0] {
                                        c.g[V0][0] = c.lb((c.g[S1][0] as u32));
                                        break 'b_38692c;
                                    } else {
                                        break 'b_386914;
                                    }
                                }
                            }
                        }
                        c.cycles += 6;
                        c.mult(0, 2, 18, 30);
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                        c.g[V0][0] = c.g[V0][0] | 0x8000;
                        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                        c.g[V0][0] = c.lb((c.g[S1][0] as u32));
                        break 'b_38692c;
                    }
                    'b_38697c: {
                        'b_386978: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[S5][0] = 0u64;
                            if t {
                                break 'b_386978;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = 0x65;
                                let t = c.g[V0][0] != c.g[V1][0];
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32));
                                if t {
                                    break 'b_38697c;
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = 0x6e;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x76;
                                    if t {
                                        break 'b_38697c;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x2));
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        if t {
                                            break 'b_38697c;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V1][0] =
                                                c.lbu((c.g[S1][0] as u32).wrapping_add(0x3));
                                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
                                            c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                                            if t {
                                                break 'b_38697c;
                                            } else {
                                                c.cycles += 2;
                                                c.g[V0][0] =
                                                    sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                                c.g[S5][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x30));
                                                break 'b_386978;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32));
                        break 'b_38697c;
                    }
                    c.cycles += 11;
                    c.g[RA][0] = sx(0x00386984);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x003696c8, 0x00386984)?;
                    c.mult(0, 4, 18, 30);
                    c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                    c.sh((c.g[V1][0] as u32).wrapping_add(0x8), c.g[S5][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V0][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.mult(0, 3, 18, 30);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003867b8);
                        }
                        continue 'l_3867b8;
                    } else {
                        break 'b_3869a8;
                    }
                }
            }
        }
        'b_3869f4: {
            c.cycles += 3;
            c.g[V0][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x4a));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A2][0] = 0u64;
            if t {
                break 'b_3869f4;
            } else {
                c.cycles += 1;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                'l_3869b8: loop {
                    'b_3869e0: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x12));
                        let t = (c.g[V0][0] as i64) < 0;
                        c.g[A1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x12));
                        if t {
                            break 'b_3869e0;
                        } else {
                            c.cycles += 7;
                            c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x10));
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[V1][0] = c.lhu((c.g[V0][0] as u32));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                            c.sh((c.g[A0][0] as u32).wrapping_add(0x12), c.g[V1][0]);
                            break 'b_3869e0;
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x4a));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003869b8);
                        }
                        continue 'l_3869b8;
                    } else {
                        break 'b_3869f4;
                    }
                }
            }
        }
        'b_386b28: {
            c.cycles += 3;
            c.g[V0][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x44));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A2][0] = 0u64;
            if t {
                break 'b_386b28;
            } else {
                c.cycles += 2;
                c.g[V1][0] = 0xc;
                'l_386a08: loop {
                    'b_386a74: {
                        c.cycles += 10;
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x38));
                        c.mult(0, 3, 6, 3);
                        c.g[FP][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        c.g[A2][0] = 0u64;
                        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                        c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        let t = (c.g[V1][0] as i64) <= 0;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        if t {
                            break 'b_386a74;
                        } else {
                            c.cycles += 2;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            'l_386a38: loop {
                                c.cycles += 15;
                                c.g[A0][0] = sx((c.g[A2][0] as u32) << 2);
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x3));
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[A1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x2));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
                                c.g[A1][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.sb((c.g[A0][0] as u32).wrapping_add(0x2), c.g[A1][0]);
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00386a38);
                                    }
                                    continue 'l_386a38;
                                } else {
                                    break 'b_386a74;
                                }
                            }
                        }
                    }
                    c.cycles += 7;
                    c.g[S1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32));
                    c.g[S3][0] = 0u64;
                    c.g[S5][0] = 0xc;
                    c.g[RA][0] = sx(0x00386a90);
                    c.g[S2][0] = c.g[S1][0];
                    call_ptr(c, 0x003698e0, 0x00386a90)?;
                    'l_386a90: loop {
                        'b_386b14: {
                            'b_386ae8: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                                let t = c.g[S3][0] != c.g[V0][0];
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_386ae8;
                                } else {
                                    c.cycles += 19;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                    c.sw((c.g[S2][0] as u32), c.g[V0][0]);
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                    c.g[V1][0] = c.lw((c.g[S6][0] as u32));
                                    c.lo[0] = c.g[V0][0];
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.madd(0, 3, 3, 21);
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                    c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                                    c.lo[0] = c.g[A0][0];
                                    c.madd(0, 3, 2, 21);
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32));
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                                    c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    let t = true;
                                    c.sw((c.g[S6][0] as u32), c.g[V0][0]);
                                    if t {
                                        break 'b_386b14;
                                    } else {
                                        break 'b_386ae8;
                                    }
                                }
                            }
                            c.cycles += 7;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = 0u64;
                            c.g[RA][0] = sx(0x00386afc);
                            c.g[T0][0] = 0u64;
                            call_ptr(c, 0x00369a78, 0x00386afc)?;
                            if c.g[V0][0] != 0u64 {
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                                break 'b_386b14;
                            } else {
                                c.cycles += 4;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                                let t = true;
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00386a90);
                                    }
                                    continue 'l_386a90;
                                } else {
                                    break 'b_386b14;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[V0][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x44));
                        c.g[A2][0] = c.g[FP][0];
                        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V1][0] = 0xc;
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00386a08);
                            }
                            continue 'l_386a08;
                        } else {
                            break 'b_386b28;
                        }
                    }
                }
            }
        }
        'b_386ba0: {
            c.cycles += 3;
            c.g[V0][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x46));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_386ba0;
            } else {
                c.cycles += 1;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x3c));
                'l_386b38: loop {
                    'b_386b8c: {
                        c.cycles += 6;
                        c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
                        c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[A2][0] = 0u64;
                        if t {
                            break 'b_386b8c;
                        } else {
                            c.cycles += 2;
                            c.g[T1][0] = 0xc;
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                            'l_386b58: loop {
                                c.cycles += 13;
                                c.g[A0][0] = sx((c.g[A2][0] as u32) << 2);
                                c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x38));
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                c.mult(0, 2, 2, 9);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.sw((c.g[A0][0] as u32), c.g[V1][0]);
                                c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00386b58);
                                    }
                                    continue 'l_386b58;
                                } else {
                                    break 'b_386b8c;
                                }
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x46));
                    c.g[A1][0] = c.g[T0][0];
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x3c));
                        if event_due(c) {
                            deliver_event_here(c, 0x00386b38);
                        }
                        continue 'l_386b38;
                    } else {
                        break 'b_386ba0;
                    }
                }
            }
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{
        covered, fake, fall_back, hook, machine, prev_cell, LOG_COUNT, LOG_UNWIND_AT,
    };
    use crate::difftest::{check, Rng, DATA};
    use std::ptr::addr_of_mut;

    const SEEDS: u32 = 48;
    /// The graphics manager under test.
    const GFX: u32 = DATA + 0x2000;
    /// Gate word of this module's fakes (machine RAM): they only act with it
    /// set, so other tests that reach the same callees are not affected.
    const OC690_GATE: u32 = DATA + 0xf_0000;
    const OC690_MAGIC: u32 = 0x6f63_3639;
    /// Bit `n` set: the n-th fake call (counted over all fakes) of a
    /// `strncmp` returns 0 (equal) / of `MatchBlendedMatrix` returns 1.
    const OC690_STRNCMP_EQ: u32 = DATA + 0xf_0004;
    const OC690_MATCH_EQ: u32 = DATA + 0xf_0008;

    /// A gated fake for the guest function `$target` (its override slot holds
    /// the fake): with the gate open it logs the call and returns (or
    /// unwinds) through `fakes::fake`, then runs `$post(c, call_index)` when
    /// it returned; with the gate closed it runs what the slot held.
    macro_rules! oc690_fake {
        ($name:ident, $target:expr, $generated:path, $post:expr) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC690_GATE) != OC690_MAGIC {
                    return fall_back(c, prev_cell($name), $generated);
                }
                let ra = c.g[RA][0] as u32;
                fake(c, $target, ra);
                if c.pc == ra {
                    let n = c.bus.read32(LOG_COUNT) - 1;
                    let post: fn(&mut Cpu, u32) = $post;
                    post(c, n);
                }
            }
        };
    }

    fn oc690_nothing(_c: &mut Cpu, _n: u32) {}

    /// `strncmp`: 0 or non-zero by the call-index mask.
    fn oc690_strncmp_result(c: &mut Cpu, n: u32) {
        let mask = c.bus.read32(OC690_STRNCMP_EQ);
        c.g[V0][0] = if mask >> (n & 31) & 1 != 0 { 0 } else { 1 };
    }

    /// `MatchBlendedMatrix`: 1 or 0 by the call-index mask.
    fn oc690_match_result(c: &mut Cpu, n: u32) {
        let mask = c.bus.read32(OC690_MATCH_EQ);
        c.g[V0][0] = (mask >> (n & 31) & 1) as u64;
    }

    oc690_fake!(
        oc690_fake_mem_copy,
        0x003e_6574,
        ee::s14::f_003e6574__generated,
        oc690_nothing
    );
    oc690_fake!(
        oc690_fake_lock_texture,
        0x0036_7cd0,
        ee::s12::f_00367cd0__generated,
        oc690_nothing
    );
    oc690_fake!(
        oc690_fake_wait_sema,
        0x0042_3de0,
        ee::s15::f_00423de0__generated,
        oc690_nothing
    );
    oc690_fake!(
        oc690_fake_signal_sema,
        0x0042_3dc0,
        ee::s15::f_00423dc0__generated,
        oc690_nothing
    );
    oc690_fake!(
        oc690_fake_flush_cache,
        0x0042_4020,
        ee::s15::f_00424020__generated,
        oc690_nothing
    );
    oc690_fake!(
        oc690_fake_set_display_regs,
        0x0038_2af0,
        ee::s12::f_00382af0__generated,
        oc690_nothing
    );
    oc690_fake!(
        oc690_fake_strncmp,
        0x0041_6b18,
        ee::s15::f_00416b18__generated,
        oc690_strncmp_result
    );
    oc690_fake!(
        oc690_fake_alloc_blended,
        0x0036_98e0,
        ee::s12::f_003698e0__generated,
        oc690_nothing
    );
    oc690_fake!(
        oc690_fake_match_blended,
        0x0036_9a78,
        ee::s12::f_00369a78__generated,
        oc690_match_result
    );

    /// Hook every fake once and open the gate in this machine.
    fn oc690_open(c: &mut Cpu) {
        hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6574),
            oc690_fake_mem_copy,
            prev_cell(oc690_fake_mem_copy),
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00367CD0),
            oc690_fake_lock_texture,
            prev_cell(oc690_fake_lock_texture),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423DE0),
            oc690_fake_wait_sema,
            prev_cell(oc690_fake_wait_sema),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423DC0),
            oc690_fake_signal_sema,
            prev_cell(oc690_fake_signal_sema),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00424020),
            oc690_fake_flush_cache,
            prev_cell(oc690_fake_flush_cache),
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00382AF0),
            oc690_fake_set_display_regs,
            prev_cell(oc690_fake_set_display_regs),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00416B18),
            oc690_fake_strncmp,
            prev_cell(oc690_fake_strncmp),
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_003698E0),
            oc690_fake_alloc_blended,
            prev_cell(oc690_fake_alloc_blended),
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00369A78),
            oc690_fake_match_blended,
            prev_cell(oc690_fake_match_blended),
        );
        c.bus.write32(OC690_GATE, OC690_MAGIC);
    }

    /// A leaf-style setup: `gfx` in a0 with random words around the fields the
    /// leaf functions touch.
    fn oc690_gfx_leaf(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.pc = entry;
        c.g[A0][0] = GFX as u64;
        for off in [0x13ec, 0x5a74, 0x6b94, 0x6d0c, 0x6d10] {
            c.bus.write32(GFX + off, r.next_u32());
        }
        r
    }

    /// `SetDitherMatrix`: random 3-bit and wide matrix entries into the DIMX
    /// word (entries are masked with 7, so wide values check the mask).
    #[test]
    fn ps2_graphics_man_set_dither_matrix_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x690_0001, 0);
            c.pc = 0x0037_c198;
            c.g[A0][0] = GFX as u64;
            c.g[A1][0] = (DATA + 0x4000) as u64;
            for i in 0..16 {
                let v = r.next_u32();
                c.bus
                    .write32(DATA + 0x4000 + 4 * i, if seed % 2 == 0 { v } else { v & 7 });
            }
            c.bus.write32(GFX + 0x59e0, r.next_u32());
            c.bus.write32(GFX + 0x59e4, r.next_u32());
        };
        check(
            ee::s12::f_0037c198__generated,
            ps2_graphics_man_set_dither_matrix,
            SEEDS,
            setup,
        );
    }

    /// `SnapshotTextureTable`: one `EA_MemCopy` call (faked), also unwinding.
    #[test]
    fn ps2_graphics_man_snapshot_texture_table_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _ = machine(c, seed, 0x690_0002, 1);
            c.pc = 0x0037_c808;
            c.g[A0][0] = GFX as u64;
            oc690_open(c);
        };
        covered(
            ee::s12::f_0037c808__generated,
            ps2_graphics_man_snapshot_texture_table,
            SEEDS,
            setup,
        );
    }

    /// `LockTexture`: loads `gfx+0x18f4` into a0 and calls `TexMan_LockTexture`
    /// (faked), also unwinding.
    #[test]
    fn ps2_graphics_man_lock_texture_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x690_0003, 1);
            c.pc = 0x0037_d738;
            c.g[A0][0] = GFX as u64;
            c.bus.write32(GFX + 0x18f4, r.next_u32());
            oc690_open(c);
        };
        covered(
            ee::s12::f_0037d738__generated,
            ps2_graphics_man_lock_texture,
            SEEDS,
            setup,
        );
    }

    /// `PS2GraphicsMan_SetPhase2Callback`: two stores.
    #[test]
    fn ps2_graphics_man_set_phase2_callback_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _ = oc690_gfx_leaf(c, seed, 0x690_0004, 0x0038_66e0);
        };
        check(
            ee::s12::f_003866e0__generated,
            ps2_graphics_man_set_phase2_callback,
            SEEDS,
            setup,
        );
    }

    /// `GetFogState`, `ClearLights`, `v110`, `GetFlipCount`: leaf accessors.
    #[test]
    fn ps2_graphics_man_leaf_accessors_match() {
        let fog: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _ = oc690_gfx_leaf(c, seed, 0x690_0005, 0x0039_53b0);
        };
        check(
            ee::s13::f_003953b0__generated,
            ps2_graphics_man_get_fog_state,
            SEEDS,
            fog,
        );
        let lights: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _ = oc690_gfx_leaf(c, seed, 0x690_0006, 0x0039_5420);
        };
        check(
            ee::s13::f_00395420__generated,
            ps2_graphics_man_clear_lights,
            SEEDS,
            lights,
        );
        let v110: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _ = oc690_gfx_leaf(c, seed, 0x690_0007, 0x0039_5508);
        };
        check(
            ee::s13::f_00395508__generated,
            ps2_graphics_man_v110,
            SEEDS,
            v110,
        );
        let flips: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _ = oc690_gfx_leaf(c, seed, 0x690_0008, 0x0039_5510);
        };
        check(
            ee::s13::f_00395510__generated,
            ps2_graphics_man_get_flip_count,
            SEEDS,
            flips,
        );
    }

    /// `PS2GraphicsMan_ctor`: the real base constructor (0x3691f8: short
    /// memset calls on `gfx`) and the two delay loops. Events are not
    /// scheduled (see the report).
    #[test]
    fn ps2_graphics_man_ctor_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _ = machine(c, seed, 0x690_0009, 0);
            c.pc = 0x0039_5288;
            c.g[A0][0] = GFX as u64;
        };
        check(
            ee::s13::f_00395288__generated,
            ps2_graphics_man_ctor,
            16,
            setup,
        );
    }

    /// The semaphore-waiting thread: the entry thunk calls the real body
    /// (`Renderer_DmaKickThread`), whose `WaitSema`/`SignalSema`/`FlushCache`/
    /// `SetDisplayRegs` calls are faked. A `WaitSema` unwinds after a few turns
    /// (the only way out), so the seeds run 0..4 turns through the states
    /// 5 (swap done), 0 (next list; queue entry 2 or not) and 2 (phase-2
    /// callback, faked at `FAKE` for a callback pointer, skipped for 0).
    fn oc690_kick_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = machine(c, seed, 0x690_000a, 0);
        c.pc = entry;
        c.g[A0][0] = GFX as u64;
        c.bus.write32(LOG_UNWIND_AT, 2 + seed % 10);
        let state = [5, 0, 2, 7][(seed / 2 % 4) as usize];
        c.bus.write32(GFX + 0x5a8c, state);
        c.bus.write32(GFX + 0x5a98, 1);
        c.bus.write32(GFX + 0x5aa4, 0);
        // table[0] (gfx+0x5a90) is the queue entry read in state 0.
        c.bus
            .write32(GFX + 0x5a90, if seed % 3 == 0 { 7 } else { 2 });
        c.bus.write32(GFX + 0x5ac8, 1 + r.next_u32() % 8);
        c.bus.write32(GFX + 0x5acc, 1 + r.next_u32() % 8);
        for off in [0x5aa0, 0x5aa8, 0x5aac, 0x5ab0, 0x5ab4] {
            c.bus
                .write32(GFX + off, DATA + 0x8000 + 0x100 * (off & 0xff));
        }
        c.bus.write32(
            GFX + 0x6d0c,
            if seed % 5 == 0 {
                0
            } else {
                crate::difftest::fakes::FAKE
            },
        );
        c.bus.write32(GFX + 0x6d10, r.next_u32());
        oc690_open(c);
    }

    #[test]
    fn renderer_dma_kick_thread_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| oc690_kick_setup(c, seed, 0x0038_2760);
        covered(
            ee::s12::f_00382760__generated,
            renderer_dma_kick_thread,
            SEEDS,
            setup,
        );
    }

    #[test]
    fn renderer_dma_kick_thread_entry_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| oc690_kick_setup(c, seed, 0x0038_2740);
        covered(
            ee::s12::f_00382740__generated,
            renderer_dma_kick_thread_entry,
            SEEDS,
            setup,
        );
    }

    // ---- PS2GraphicsMan_v109 ---------------------------------------------

    /// The model under relocation, and the offsets (relative to the load
    /// base) of its tables.
    const V109_MODEL: u32 = DATA + 0x4000;
    const V109_TBL: u32 = DATA + 0x4100;
    const V109_COUNT: u32 = DATA + 0x4200;
    const V109_SLOTS: u32 = DATA + 0x4300;
    const V109_BASE: u32 = DATA + 0x1_0000;
    const V109_MATS: u32 = 0x100;
    const V109_BONES: u32 = 0x300;
    const V109_REFS: u32 = 0x400;
    const V109_BLENDS: u32 = 0x500;
    const V109_NAMES: u32 = 0x800;
    const V109_TRACKS: u32 = 0x1000;
    const V109_KEYS: u32 = 0x1200;
    const V109_REF_IDX: u32 = 0x1400;
    const V109_LISTS: u32 = 0x1600;

    fn oc690_put_str(c: &mut Cpu, at: u32, s: &[u8]) {
        for i in 0..16u32 {
            c.bus
                .write8(at + i, s.get(i as usize).copied().unwrap_or(0));
        }
    }

    /// Build a model with 0..3 blends, 0..2 refs, bones, mats and random names
    /// (`alph`, `ea`, `env<digit>` ... in the two name halves).
    fn oc690_v109_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let base = V109_BASE;
        let nblends = seed % 4;
        let nrefs = (seed / 4) % 3;
        let nbones = (seed / 3) % 3;
        let nmats = (seed / 5) % 3;
        c.bus.write32(V109_MODEL + 0x18, V109_MATS);
        c.bus.write32(V109_MODEL + 0x20, V109_BONES);
        c.bus.write32(V109_MODEL + 0x30, V109_REFS);
        c.bus.write32(V109_MODEL + 0x34, V109_BLENDS);
        c.bus.write16(V109_MODEL + 0x44, nblends as u16);
        c.bus.write16(V109_MODEL + 0x46, nrefs as u16);
        c.bus.write16(V109_MODEL + 0x48, nbones as u16);
        c.bus.write16(V109_MODEL + 0x4a, nmats as u16);
        for i in 0..4 {
            c.bus.write32(V109_TBL + 4 * i, r.next_u32() & 0xff);
        }
        // materials: mt[8] (+0x10) indexes tbl, mt[9] (+0x12) < 0 is skipped
        for i in 0..nmats {
            let mt = base + V109_MATS + 0x50 * i;
            c.bus.write16(mt + 0x10, (r.next_u32() % 4) as u16);
            let v = r.next_u32() as u16 & 0x7fff;
            c.bus
                .write16(mt + 0x12, if (seed + i) % 2 == 0 { v } else { v | 0x8000 });
        }
        // bones: flags, name record index (+4), ntracks (+0xc), tracks (+0x10)
        let firsts: [&[u8]; 4] = [b"alph", b"ea12", b"tex1", b"body"];
        let seconds: [&[u8]; 5] = [b"", b"env7", b"env", b"enx1", b"xyz"];
        for i in 0..nbones {
            let b = base + V109_BONES + 0x14 * i;
            let name = 0x40 + i;
            let rec = base + name * 0x20;
            c.bus.write32(b, r.next_u32() & 0xff);
            c.bus.write32(b + 4, name);
            c.bus.write32(b + 0xc, (seed + i) % 3);
            c.bus.write32(b + 0x10, V109_TRACKS + 0x40 * i);
            oc690_put_str(c, rec, firsts[((seed + 2 * i) % 4) as usize]);
            oc690_put_str(c, rec + 0x10, seconds[((seed / 2 + i) % 5) as usize]);
            c.bus.write32(rec + 0x14, r.f32(2.0));
            for j in 0..2u32 {
                let t = base + V109_TRACKS + 0x40 * i + 8 * j;
                let keys = V109_KEYS + 0x100 * i + 0x20 * j;
                c.bus.write32(t, keys);
                c.bus.write32(t + 4, (seed + j) % 3);
                for k in 0..2u32 {
                    c.bus.write32(base + keys + 8 * k, r.next_u32() & 0xffff);
                    c.bus
                        .write32(base + keys + 8 * k + 4, r.next_u32() & 0xffff);
                }
            }
        }
        // refs {n, idx offset}: indices into the blends
        for i in 0..nrefs {
            let e = base + V109_REFS + 8 * i;
            c.bus.write32(e, (seed + i) % 4);
            c.bus.write32(e + 4, V109_REF_IDX + 0x20 * i);
            for j in 0..3u32 {
                c.bus
                    .write32(base + V109_REF_IDX + 0x20 * i + 4 * j, r.next_u32() % 3);
            }
        }
        // blends {n, list offset, id}: lists of {_, _, +2 u8, +3 u8 tbl index}
        for i in 0..nblends {
            let e = base + V109_BLENDS + 12 * i;
            let n = (seed / 2 + i) % 4;
            c.bus.write32(e, n);
            c.bus.write32(e + 4, V109_LISTS + 0x20 * i);
            for j in 0..3u32 {
                c.bus.write32(
                    base + V109_LISTS + 0x20 * i + 4 * j,
                    r.next_u32() & 0x0303_ffff,
                );
            }
        }
        c.bus.write32(V109_COUNT, seed / 7 % 3);
        for i in 0..24u32 {
            c.bus.write32(V109_SLOTS + 4 * i, r.next_u32() & 0xff);
        }
        c.bus.write32(OC690_STRNCMP_EQ, r.next_u32());
        c.bus.write32(OC690_MATCH_EQ, r.next_u32());
    }

    /// `PS2GraphicsMan_v109`: relocation of a small random model; the
    /// `strncmp` / `MatchBlendedMatrix` / `AllocBlendedMatrixHistory` callees
    /// are faked (and unwind at a random call), `GetTextureByName` is real
    /// (the gfx texture table starts empty, so every name is added).
    #[test]
    fn ps2_graphics_man_v109_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x690_000b, 12);
            c.pc = 0x0038_66f0;
            c.g[A0][0] = GFX as u64;
            c.g[A1][0] = V109_MODEL as u64;
            c.g[A2][0] = V109_BASE as u64;
            c.g[A3][0] = V109_TBL as u64;
            c.g[T0][0] = V109_COUNT as u64;
            c.g[T1][0] = V109_SLOTS as u64;
            c.g[T2][0] = r.next_u32() as u64;
            oc690_v109_fixture(c, &mut r, seed);
            oc690_open(c);
        };
        covered(
            ee::s12::f_003866f0__generated,
            ps2_graphics_man_v109,
            96,
            setup,
        );
    }
}
