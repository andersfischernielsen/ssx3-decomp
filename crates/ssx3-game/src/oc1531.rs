//! Staging module oc1531 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1531`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_a920, audio_play_sound_volume, "Audio_PlaySoundVolume"),
    (0x002b_b928, world_painter_debug_draw_node, "WorldPainter_DebugDrawNode"),
    (0x002b_ea38, twpigd_sun_load_state, "tWPIGD_Sun_LoadState"),
    (0x002b_ee48, twpigd_weather_load_state, "tWPIGD_Weather_LoadState"),
    (0x002b_faa0, twpigd_sun_save_state, "tWPIGD_Sun_SaveState"),
];

/// `Audio_PlaySoundVolume(arg)` (0x2ba920, `cBankSys_SFX_Play2DVol`): push a
/// new voice-option slot on the sound system's play stack. `arg->o` is the
/// sound-system view; its `sub` block (at +0x1d8) holds the slot counter
/// (`sub + 0x1c` = `o + 0x1f4`) and the parallel option arrays (24-byte records
/// at `sub + 0x20`, values at `+0x38`, a 16-byte tuning quad at `+0x70`...).
/// The new slot copies the current record, clears or defaults every array
/// entry (tuning 0/100/90/50, 0x7f, 1), stores the call arguments (`a1`, `a2`
/// into the caller's arrays, `a3` into the value array), clamps that value to
/// 0..127 into byte 0 of the record, and finishes with `func_002906B8(arg)`.
/// `memset(&slot_name, 0, 8)` is the 0x416210 call.
pub fn audio_play_sound_volume(c: &mut Cpu) {
    guest(c, |c| {
        'b_2babfc: {
        'b_2babd0: {
        c.cycles += 166;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S2][0] = 0x18;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[A1][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S4][0] = c.g[A2][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S5][0] = c.g[A3][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S6][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[A1][0] = 0u64;
        c.g[S0][0] = c.lw((c.g[S6][0] as u32));
        c.g[A2][0] = 0x8;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
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
        call_ptr(c, 0x00416210, 0x002baa28)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = 0x64;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
        c.g[A1][0] = 0x5a;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A2][0] = 0x32;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A3][0] = 0x7f;
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[T0][0] = 0x1;
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
        c.sw((c.g[V0][0] as u32), c.g[A0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
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
        c.sw((c.g[V0][0] as u32), c.g[A3][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), c.g[T0][0]);
        c.g[A0][0] = c.lw((c.g[S6][0] as u32));
        c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), c.g[S3][0]);
        c.g[V1][0] = c.lw((c.g[S6][0] as u32));
        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.sw((c.g[V0][0] as u32), c.g[S4][0]);
        c.g[A1][0] = c.lw((c.g[S6][0] as u32));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
        c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), c.g[S5][0]);
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
        c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
        c.mult(0, 4, 4, 18);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        if t {
        break 'b_2babd0;
        } else {
        c.cycles += 6;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        let t = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0u64;
        if t {
        break 'b_2babfc;
        } else {
        break 'b_2babd0;
        }
        }
        }
        c.cycles += 7;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
        if c.g[V1][0] == 0u64 {
        c.g[V0][0] = 0x7f;
        break 'b_2babfc;
        } else {
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
        break 'b_2babfc;
        }
        }
        c.cycles += 13;
        c.sb((c.g[A0][0] as u32), c.g[V0][0]);
        c.g[A0][0] = c.g[S6][0];
        call_ptr(c, 0x002906b8, 0x002bac08)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// `WorldPainter_DebugDrawNode(painter, ctx, node, out, t0, t1)` (0x2bb928,
/// `cWorldPainterMan_render1`): recursive debug draw of a bounding-box tree
/// node. A node whose first halfword has bit 0 set is an inner node: its
/// centre/extent vectors (`t0`, `t1`) are split by VU0 add/div/mul into four
/// sub-boxes, each recursed into through the child index halfwords
/// (`ctx->children + (hw >> 1) * 8`). Otherwise it is a leaf: the world's
/// draw object is fetched through two vtable calls, the box's four corners are
/// written as 16-byte vectors into `out` (`a3`) at +0x20/+0x50/+0x80/+0xb0/
/// +0xe0, and the box is submitted through the second vtable call.
pub fn world_painter_debug_draw_node(c: &mut Cpu) {
    guest(c, |c| {
        'b_2bbbcc: {
        'b_2bbac8: {
        c.cycles += 19;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S2][0] = c.g[T0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S3][0] = c.g[A1][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S4][0] = c.g[A3][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S5][0] = c.g[T1][0];
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S6][0] = c.g[A2][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
        c.g[V0][0] = c.lhu((c.g[S6][0] as u32));
        c.g[V0][0] = c.g[V0][0] & 0x1;
        let t = c.g[V0][0] == 0u64;
        c.g[S7][0] = c.g[A0][0];
        if t {
        break 'b_2bbac8;
        } else {
        c.cycles += 85;
        c.lqc2(3, (c.g[S2][0] as u32));
        c.lqc2(4, (c.g[S5][0] as u32));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[AT][0] = 0x40000000;
        c.mtc1(1, 0);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 3);
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[T1][0] = c.g[SP][0];
        c.mfc1(2, 0);
        c.qmtc2(2, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.vu0().waitq();
        c.vu0().mul(0b1111, 4, 4, Src::Q);
        c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lwc1(5, (c.g[S2][0] as u32));
        c.sq(2, (c.g[SP][0] as u32));
        c.lwc1(1, (c.g[S5][0] as u32));
        c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
        c.lwc1(2, (c.g[S5][0] as u32).wrapping_add(0x4));
        c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.lwc1(6, (c.g[SP][0] as u32));
        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x24));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x44));
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x2c));
        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x3c));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x48), 0u64);
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4c));
        c.g[A2][0] = c.lhu((c.g[S6][0] as u32));
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
        c.g[A2][0] = sx((c.g[A2][0] as u32) >> 1);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 3);
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        Callee { addr: 0x002bb928, f: ee::s08::f_002bb928 }.call(c, 0x002bba3c)?;
        c.g[A2][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x2));
        c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
        c.g[T0][0] = c.g[S0][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32) >> 1);
        c.g[A0][0] = c.g[S7][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 3);
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.g[A3][0] = c.g[S4][0];
        Callee { addr: 0x002bb928, f: ee::s08::f_002bb928 }.call(c, 0x002bba68)?;
        c.g[A2][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x4));
        c.g[T1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = c.g[S7][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32) >> 1);
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 3);
        c.g[A3][0] = c.g[S4][0];
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        Callee { addr: 0x002bb928, f: ee::s08::f_002bb928 }.call(c, 0x002bba94)?;
        c.g[A2][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x6));
        c.g[A0][0] = c.g[S7][0];
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32) >> 1);
        c.g[A3][0] = c.g[S4][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 3);
        c.g[T1][0] = c.g[S5][0];
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.g[T0][0] = c.g[SP][0];
        Callee { addr: 0x002bb928, f: ee::s08::f_002bb928 }.call(c, 0x002bbac0)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
        break 'b_2bbbcc;
        }
        }
        c.cycles += 65;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        call_ptr(c, t, 0x002bbaf8)?;
        c.lq(3, (c.g[V0][0] as u32));
        c.g[A2][0] = c.g[S4][0];
        c.lwc1(1, (c.g[S2][0] as u32));
        c.g[A1][0] = 0x5;
        c.sq(3, (c.g[SP][0] as u32));
        c.g[A3][0] = 0u64;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 3);
        c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(2, (c.g[S4][0] as u32).wrapping_add(0x20));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.lwc1(0, (c.g[S5][0] as u32));
        c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x4));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.sq(2, (c.g[S4][0] as u32).wrapping_add(0x50));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.lwc1(0, (c.g[S5][0] as u32));
        c.lwc1(1, (c.g[S5][0] as u32).wrapping_add(0x4));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.sq(2, (c.g[S4][0] as u32).wrapping_add(0x80));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.lwc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
        c.lwc1(1, (c.g[S2][0] as u32));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(2, (c.g[S4][0] as u32).wrapping_add(0xb0));
        c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
        c.lwc1(1, (c.g[S2][0] as u32));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(2, (c.g[S4][0] as u32).wrapping_add(0xe0));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x248));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x002bbbc8)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
        break 'b_2bbbcc;
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
        c.pc = t;
        return Ok(());
    });
}

/// The shared body of the `tWPIGD_*_LoadState/SaveState` replay-frame
/// walkers: `stream->vt[vt_fn](stream + vt_adj, field, 4)` for each of
/// `fields` four-byte fields at `self + 8, +0x10, ...`, every field followed
/// by a four-byte transfer of the word at `self + 0`. Calls return to
/// `first_ret + 0x1c * n`.
fn wp_ingame_stream_fields(
    c: &mut Cpu,
    fields: u32,
    first_ret: u32,
    vt_adj_off: u32,
    vt_fn_off: u32,
) -> Flow {
    let frame = Frame::open(c, 0x30);
    c.g[A2][0] = 0x4;
    frame.save(c, 16, 0x20);
    frame.save(c, 17, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    for call in 0..2 * fields {
        // Even calls move field `call / 2` (self + 8 * (n + 1)), odd calls
        // the word at self + 0.
        c.g[A1][0] = if call % 2 == 0 {
            sx((c.g[S1][0] as u32).wrapping_add(8 * (call / 2 + 1)))
        } else {
            c.g[S1][0]
        };
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A2][0] = 0x4;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(vt_adj_off));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(vt_fn_off));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, first_ret + 0x1c * call)?;
    }
    frame.restore(c, 16, 0x20);
    frame.restore(c, 17, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `tWPIGD_Sun_LoadState(self, stream)` (0x2bea38,
/// `tWPInGameData_Sun_readFromReplayFrame`): read the 9 sun fields (pitch,
/// yaw, colour, intensity, type, lens...) from the replay stream (vtable
/// +0x10 offset adjust, +0x14 read function), 4 bytes each.
pub fn twpigd_sun_load_state(c: &mut Cpu) {
    c.cycles += 137;
    guest(c, |c| wp_ingame_stream_fields(c, 9, 0x002b_ea6c, 0x10, 0x14));
}

/// `tWPIGD_Weather_LoadState(self, stream)` (0x2bee48,
/// `tWPInGameData_Weather_readFromReplayFrame`): the 19 weather fields.
pub fn twpigd_weather_load_state(c: &mut Cpu) {
    c.cycles += 277;
    guest(c, |c| wp_ingame_stream_fields(c, 19, 0x002b_ee7c, 0x10, 0x14));
}

/// `tWPIGD_Sun_SaveState(self, stream)` (0x2bfaa0,
/// `tWPInGameData_Sun_writeToReplayFrame`): the same 9 sun fields written to
/// the stream (vtable +0x8 offset adjust, +0xc write function).
pub fn twpigd_sun_save_state(c: &mut Cpu) {
    c.cycles += 137;
    guest(c, |c| wp_ingame_stream_fields(c, 9, 0x002b_fad4, 0x8, 0xc));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{fill, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (a unique cell of the
    /// machine's own RAM; the fakes only answer when it holds the magic).
    const OC1531_CTRL: u32 = DATA + 0x20_0100;
    const OC1531_MAGIC: u32 = 0x1531_a920;
    static OC1531_PREV_MEMSET: AtomicUsize = AtomicUsize::new(0);
    static OC1531_PREV_SND: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake of `memset` (0x416210).
    fn oc1531_fake_memset(c: &mut Cpu) {
        if c.bus.read32(OC1531_CTRL) != OC1531_MAGIC {
            return fakes::fall_back(c, &OC1531_PREV_MEMSET, ee::s15::f_00416210__generated);
        }
        fakes::fake_call(c, 0x0041_6210);
    }

    /// Gated fake of `func_002906B8` (the sound system update).
    fn oc1531_fake_snd(c: &mut Cpu) {
        if c.bus.read32(OC1531_CTRL) != OC1531_MAGIC {
            return fakes::fall_back(c, &OC1531_PREV_SND, ee::s07::f_002906b8__generated);
        }
        fakes::fake_call(c, 0x0029_06b8);
    }

    /// Audio_PlaySoundVolume: slot counter 0..2, volume arguments below, in
    /// and above the 0..127 range, both fake calls (and unwinds in each).
    #[test]
    fn audio_play_sound_volume_matches() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00416210),
            oc1531_fake_memset,
            &OC1531_PREV_MEMSET,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002906B8),
            oc1531_fake_snd,
            &OC1531_PREV_SND,
        );
        const ARG: u32 = DATA + 0x1000;
        const OBJ: u32 = DATA + 0x2000;
        fakes::covered(
            ee::s08::f_002ba920__generated,
            audio_play_sound_volume,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x002b_a920, 2);
                c.bus.write32(OC1531_CTRL, OC1531_MAGIC);
                let sub = OBJ + 0x1d8;
                c.bus.write32(ARG, OBJ);
                c.bus.write32(ARG + 4, DATA + 0x1100);
                c.bus.write32(ARG + 8, DATA + 0x1200);
                // The current record (24 bytes at +0x1dc, unaligned copy).
                fill(c, &mut r, OBJ + 0x1d8, 8);
                c.bus.write32(OBJ + 0x1f4, seed % 3);
                // Pointer fields of the option block, each to its own array.
                for (k, off) in [
                    0x20u32, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c,
                    0x80, 0x84,
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus.write32(sub + off, DATA + 0x4000 + 0x400 * k as u32);
                }
                c.g[A0][0] = sx(ARG);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
                let volumes = [200i32, -5, 50, 126, 127, 0, -1, 1000];
                c.g[A3][0] = sx(volumes[seed as usize % volumes.len()] as u32);
            },
        );
    }

    /// WorldPainter_DebugDrawNode: leaf boxes (bit 0 of the node clear: two
    /// vtable calls through the world's draw object) and inner nodes (VU0
    /// split and four recursive calls into leaf children), unwinds in each.
    #[test]
    fn world_painter_debug_draw_node_matches() {
        const CTX: u32 = DATA + 0x1100;
        const CHILDREN: u32 = DATA + 0x1300;
        const VEC0: u32 = DATA + 0x1400;
        const VEC1: u32 = DATA + 0x1420;
        const OUT: u32 = DATA + 0x8000;
        const WORLD: u32 = DATA + 0x2000;
        const X1: u32 = DATA + 0x2200;
        const X2: u32 = DATA + 0x2400;
        const X3: u32 = DATA + 0x2600;
        const X4: u32 = DATA + 0x2800;
        const X5: u32 = DATA + 0x3200;
        const G: u32 = DATA + 0x3400;
        const Y1: u32 = DATA + 0x5000;
        fakes::covered(
            ee::s08::f_002bb928__generated,
            world_painter_debug_draw_node,
            240,
            &|c, seed| {
                let inner = seed % 2 == 1;
                let mut r = fakes::machine(c, seed, 0x002b_b928, if inner { 8 } else { 2 });
                // Node: inner nodes have bit 0 of the first halfword set; the
                // halfwords index four leaf children (even first halfword).
                let node = DATA + 0x1200;
                let idx = |r: &mut Rng| (r.next_u32() % 4) << 1;
                let hu0 = idx(&mut r) | inner as u32;
                c.bus.write16(node, hu0 as u16);
                for k in 1..4u32 {
                    let v = idx(&mut r) | (r.next_u32() & 1);
                    c.bus.write16(node + 2 * k, v as u16);
                }
                c.bus.write32(CTX + 0x20, CHILDREN);
                for k in 0..4u32 {
                    c.bus.write16(CHILDREN + 8 * k, (r.next_u32() & 0xfffe) as u16);
                }
                // Two vectors; some seeds with edge values (zero, inf, NaN).
                for k in 0..4u32 {
                    let (a, b) = if seed % 7 == 0 {
                        (r.edge_f32(), r.edge_f32())
                    } else {
                        (r.f32(8.0), r.f32(8.0))
                    };
                    c.bus.write32(VEC0 + 4 * k, a);
                    c.bus.write32(VEC1 + 4 * k, b);
                }
                // The leaf: world -> +0x84 -> +0xc -> +0x40 -> +0x18 -> +0x6c0
                // object whose vtable (+0x28 adjust, +0x2c fn) draws.
                c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
                c.bus.write32(WORLD + 0x84, X1);
                c.bus.write32(X1 + 0xc, X2);
                c.bus.write32(X2 + 0x40, X3);
                c.bus.write32(X3 + 0x18, X4);
                c.bus.write32(X4 + 0x6c0, X5);
                c.bus.write16(X5 + 0x28, (r.next_u32() & 0xf0) as u16);
                c.bus.write32(X5 + 0x2c, fakes::FAKE);
                c.bus.write32(GAME_GP + 0x2a90, G);
                c.bus.write32(G + 0x10d8, Y1);
                c.bus.write16(Y1 + 0x248, (r.next_u32() & 0xf0) as u16);
                c.bus.write32(Y1 + 0x24c, fakes::FAKE + 0x10);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(CTX);
                c.g[A2][0] = sx(node);
                c.g[A3][0] = sx(OUT);
                c.g[T0][0] = sx(VEC0);
                c.g[T1][0] = sx(VEC1);
            },
        );
    }

    /// Replay-frame walkers: a stream whose vtable holds the offset adjust
    /// and the (fake) transfer function; unwinds at random calls.
    fn oc1531_stream_fixture(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        calls: u32,
        adj_off: u32,
        fn_off: u32,
    ) {
        const OBJ: u32 = DATA + 0x1000;
        const STREAM: u32 = DATA + 0x1100;
        const VT: u32 = DATA + 0x1200;
        let mut r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(STREAM, VT);
        c.bus.write16(VT + adj_off, r.next_u32() as u16);
        c.bus.write32(VT + fn_off, fakes::FAKE + 0x10 * (seed % 4));
        fill(c, &mut r, OBJ, 8);
        c.g[A0][0] = sx(OBJ);
        c.g[A1][0] = sx(STREAM);
    }

    #[test]
    fn twpigd_sun_load_state_matches() {
        fakes::covered(
            ee::s08::f_002bea38__generated,
            twpigd_sun_load_state,
            120,
            &|c, seed| oc1531_stream_fixture(c, seed, 0x002b_ea38, 18, 0x10, 0x14),
        );
    }

    #[test]
    fn twpigd_weather_load_state_matches() {
        fakes::covered(
            ee::s08::f_002bee48__generated,
            twpigd_weather_load_state,
            120,
            &|c, seed| oc1531_stream_fixture(c, seed, 0x002b_ee48, 38, 0x10, 0x14),
        );
    }

    #[test]
    fn twpigd_sun_save_state_matches() {
        fakes::covered(
            ee::s08::f_002bfaa0__generated,
            twpigd_sun_save_state,
            120,
            &|c, seed| oc1531_stream_fixture(c, seed, 0x002b_faa0, 18, 0x8, 0xc),
        );
    }
}
