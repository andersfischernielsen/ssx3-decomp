//! Staging module oc1057 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1057`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0036_8970, texman_emit_tex0, "TexMan_EmitTex0")];

/// Offsets in `cTextureMan` and `tTexInfo` used by `TexMan_EmitTex0`.
#[cfg(test)]
pub(crate) const TEXMAN_BOUND: u32 = 0x1f4c;
#[cfg(test)]
pub(crate) const TEXMAN_CLUT_TEX: u32 = 0x1f54;

/// Picks the GS page size (width, height in texels) of one mip level from the
/// 29-entry jump table at `0x490000 + tab`, indexed by the pixel format minus
/// 4 (`idx`); out of range (`in_range` false) takes the 64x32 default. The
/// four table targets (`base`, +0xc, +0x18, +0x24) set (128,128), (128,64),
/// (64,64) and (64,32) into `rx`/`ry`. Returns false when the table holds
/// anything else (the generated body leaves through `jump_out`).
fn tm_pick(
    c: &mut Cpu,
    in_range: bool,
    idx: usize,
    tab: u32,
    rx: usize,
    ry: usize,
    base: u32,
) -> bool {
    if !in_range {
        c.cycles += 2;
        c.g[rx][0] = 0x40;
        c.g[ry][0] = 0x20;
        return true;
    }
    c.cycles += 7;
    c.g[V0][0] = 0x490000;
    c.g[V1][0] = sx((c.g[idx][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(tab));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
    let target = c.g[A0][0] as u32;
    let (cycles, w, h) = if target == base {
        (3, 0x80, 0x80)
    } else if target == base + 0xc {
        (3, 0x80, 0x40)
    } else if target == base + 0x18 {
        (3, 0x40, 0x40)
    } else if target == base + 0x24 {
        (2, 0x40, 0x20)
    } else {
        c.jump_out(target, 0x0036_8970, 0x0036_9098);
        return false;
    };
    c.cycles += cycles;
    c.g[rx][0] = w;
    c.g[ry][0] = h;
    true
}

/// `void TexMan_EmitTex0(cTextureMan* self, int idx, int slot, ulong** dma)`
/// (0x368970): binds texture `idx` to TEX slot 0 or 1 and appends the GIF
/// packet to the DMA stream. If the slot already holds `idx` and its CLUT is
/// current nothing is written; if only the CLUT differs, a 2-register CLUT
/// load (TEX0_n with the CLUT address) is emitted. Otherwise a packet with
/// MIPTBP1/2 (mip level base pointers, each level's size from the page-size
/// jump table and the pixel format shift), TEX1 and TEX0 is written, and
/// `mCLUTTex` is updated when the texture has a CLUT.
pub fn texman_emit_tex0(c: &mut Cpu) {
    c.cycles += 16;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    c.g[T8][0] = c.g[A0][0];
    c.g[T1][0] = c.g[A2][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[V1][0] = sx((c.g[T1][0] as u32) << 2);
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V0][0] = sx((c.g[T8][0] as u32).wrapping_add(0x1f4c));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sq(20, (c.g[SP][0] as u32));
    c.g[S1][0] = c.g[A3][0];
    c.g[T9][0] = c.g[A1][0];
    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
    let t = c.g[A0][0] == c.g[T9][0];
    c.g[T0][0] = c.lw((c.g[S1][0] as u32));
    'epi: {
        if t {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x1f54));
            let t = c.g[V0][0] == c.g[A0][0];
            c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
            if t {
                break 'epi;
            }
            c.cycles += 5;
            c.g[V0][0] = sx((c.g[T8][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x2c));
            let t = (c.g[V1][0] as i64) < 0;
            c.g[A2][0] = 0xe;
            if t {
                break 'epi;
            }
            c.cycles += 25;
            c.sw((c.g[T8][0] as u32).wrapping_add(0x1f54), c.g[A0][0]);
            c.g[V0][0] = 0x10000000;
            c.g[V0][0] = c.g[V0][0] | 0x3;
            c.sd((c.g[T0][0] as u32), c.g[V0][0]);
            c.g[V1][0] = 0x50000000;
            c.g[V1][0] = c.g[V1][0] | 0x2;
            c.g[V1][0] = c.g[V1][0] << 32;
            c.g[A1][0] = 0x10000000;
            c.g[A1][0] = c.g[A1][0] << 32;
            c.g[A1][0] = c.g[A1][0] | 0x8001;
            c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
            c.g[V0][0] = 0x16;
            c.sd((c.g[T0][0] as u32).wrapping_add(0x20), c.g[A1][0]);
            c.g[A0][0] = 0x17;
            c.sd((c.g[T0][0] as u32).wrapping_add(0x28), c.g[A2][0]);
            if c.g[T1][0] == 0 {
                c.g[A0][0] = c.g[V0][0];
            }
            c.sd((c.g[T0][0] as u32).wrapping_add(0x8), 0u64);
            c.sd((c.g[T0][0] as u32).wrapping_add(0x10), 0u64);
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x2c));
            c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = c.g[V0][0] << 37;
            c.sd((c.g[T0][0] as u32).wrapping_add(0x38), c.g[A0][0]);
            c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
            c.sd((c.g[T0][0] as u32).wrapping_add(0x30), c.g[V1][0]);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x40));
            break 'epi;
        }
        c.cycles += 5;
        c.sw((c.g[V0][0] as u32), c.g[T9][0]);
        c.g[V0][0] = sx((c.g[T9][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[T8][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        let t = c.g[T1][0] != 0u64;
        c.g[T2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        if t {
            c.cycles += 4;
            c.g[S4][0] = 0x7;
            c.g[S3][0] = 0x15;
            c.g[S0][0] = 0x35;
            c.g[S2][0] = 0x37;
        } else {
            c.cycles += 5;
            c.g[S4][0] = 0x6;
            c.g[S3][0] = 0x14;
            c.g[S0][0] = 0x34;
            c.g[S2][0] = 0x36;
        }
        c.cycles += 9;
        c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0xc));
        c.g[T7][0] = 0x8;
        c.g[A1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = 0x6;
        c.g[V0][0] = c.g[V0][0] ^ 0x9;
        c.g[A2][0] = 0x4;
        c.g[V1][0] = ((c.g[A1][0] as i64) < 4i64) as u64;
        let t = c.g[V1][0] == 0u64;
        if c.g[V0][0] == 0 {
            c.g[T7][0] = c.g[A0][0];
        }
        if !t {
            c.cycles += 4;
            c.g[V1][0] = ((c.g[A1][0] as i64) < 1i64) as u64;
            c.g[A2][0] = 0x2;
            c.g[V0][0] = 0x3;
            if c.g[V1][0] == 0 {
                c.g[A2][0] = c.g[V0][0];
            }
        }
        c.cycles += 21;
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = 0x50000000;
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        c.g[A1][0] = c.g[A2][0] | 0x8000;
        c.g[A0][0] = 0x8000;
        c.g[A0][0] = c.g[A0][0] << 45;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2));
        c.g[A1][0] = c.g[A1][0] | c.g[A0][0];
        c.g[V0][0] = 0x10000000;
        c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
        c.g[V1][0] = c.g[V1][0] << 32;
        c.g[A0][0] = 0xe;
        c.sd((c.g[T0][0] as u32), c.g[V0][0]);
        c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
        c.sd((c.g[T0][0] as u32).wrapping_add(0x20), c.g[A1][0]);
        c.sd((c.g[T0][0] as u32).wrapping_add(0x28), c.g[A0][0]);
        c.sd((c.g[T0][0] as u32).wrapping_add(0x8), 0u64);
        c.sd((c.g[T0][0] as u32).wrapping_add(0x10), 0u64);
        c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x24));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x30));
        if !t {
            c.cycles += 18;
            c.g[A0][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x38));
            c.g[A1][0] = 0x1;
            c.g[T1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = c.g[A0][0] >> 26;
            c.g[V1][0] = c.g[A0][0] >> 30;
            c.g[V0][0] = c.g[V0][0] & 0xf;
            c.g[V1][0] = c.g[V1][0] & 0xf;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            c.g[A2][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x4));
            c.g[V1][0] = c.g[V1][0] << 32;
            c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
            c.g[T4][0] = sx((c.g[A1][0] as u32) << ((c.g[V0][0] as u32) & 31));
            c.g[T5][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
            c.g[A3][0] = c.g[A0][0];
            c.g[V0][0] = (c.g[A2][0] < 0x1du64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[T3][0] = c.g[T1][0];
            if !tm_pick(c, !t, A2, 0x2340, A0, A1, 0x00368ad4) {
                return;
            }
            c.cycles += 34;
            c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[A0][0] as i64)) as u64;
            c.g[V1][0] = ((c.g[T5][0] as i64) < (c.g[A1][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = c.g[T4][0];
            }
            if c.g[V1][0] == 0 {
                c.g[A1][0] = c.g[T5][0];
            }
            c.mult(0, 5, 4, 5);
            c.g[V0][0] = c.g[A3][0] >> 30;
            c.g[A0][0] = c.g[A3][0] >> 26;
            c.g[V0][0] = c.g[V0][0] & 0xf;
            c.g[V1][0] = 0x1;
            c.g[A0][0] = c.g[A0][0] & 0xf;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            c.g[A0][0] = c.g[A0][0] << 32;
            c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64;
            c.mult(0, 5, 5, 9);
            c.g[V0][0] = sx((c.g[V1][0] as u32) << ((c.g[V0][0] as u32) & 31));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[A0][0] as u32) & 31));
            c.g[A2][0] = 0xffffffffffffffff;
            c.g[T4][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
            c.g[T5][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
            c.g[A3][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x28));
            c.g[T1][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x4));
            c.g[A2][0] = ((c.g[A2][0] as i64) < (c.g[A1][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7));
            if c.g[A2][0] != 0 {
                c.g[V1][0] = c.g[A1][0];
            }
            c.g[T6][0] = c.g[T3][0];
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
            c.g[A0][0] = (c.g[T1][0] < 0x1du64) as u64;
            c.g[V0][0] = sx(0u32.wrapping_sub((c.g[V1][0] as u32)));
            c.g[V0][0] = c.g[V0][0] & 0xff;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> ((c.g[T7][0] as u32) & 31)) as u32);
            let t = c.g[A0][0] == 0u64;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            if !tm_pick(c, !t, T1, 0x23c0, A1, A0, 0x00368ba4) {
                return;
            }
            c.cycles += 34;
            c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[A1][0] as i64)) as u64;
            c.g[V1][0] = ((c.g[T5][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A1][0] = c.g[T4][0];
            }
            if c.g[V1][0] == 0 {
                c.g[A0][0] = c.g[T5][0];
            }
            c.mult(0, 4, 5, 4);
            c.g[V0][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x38));
            c.g[A1][0] = 0x1;
            c.g[A2][0] = 0xffffffffffffffff;
            c.g[V1][0] = c.g[V0][0] >> 30;
            c.g[T5][0] = c.g[T3][0];
            c.g[V0][0] = c.g[V0][0] >> 26;
            c.g[V1][0] = c.g[V1][0] & 0xf;
            c.mult(0, 4, 4, 14);
            c.g[V0][0] = c.g[V0][0] & 0xf;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            c.g[V1][0] = c.g[V1][0] << 32;
            c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << ((c.g[V0][0] as u32) & 31));
            c.g[T1][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x4));
            c.g[A1][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
            c.g[T3][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
            c.g[A2][0] = ((c.g[A2][0] as i64) < (c.g[A0][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7));
            if c.g[A2][0] != 0 {
                c.g[V1][0] = c.g[A0][0];
            }
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 2) as u32);
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
            c.g[A0][0] = (c.g[T1][0] < 0x1du64) as u64;
            c.g[V0][0] = sx(0u32.wrapping_sub((c.g[V1][0] as u32)));
            c.g[V0][0] = c.g[V0][0] & 0xff;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> ((c.g[T7][0] as u32) & 31)) as u32);
            let t = c.g[A0][0] == 0u64;
            c.g[T4][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            if !tm_pick(c, !t, T1, 0x2440, A2, V1, 0x00368c74) {
                return;
            }
            c.cycles += 29;
            c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            c.g[V0][0] = ((c.g[T3][0] as i64) < (c.g[A2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A2][0] = c.g[T3][0];
            }
            if c.g[A0][0] == 0 {
                c.g[V1][0] = c.g[A1][0];
            }
            c.mult(0, 3, 6, 3);
            c.g[A1][0] = 0xffffffffffffffff;
            c.g[A0][0] = c.g[T4][0];
            c.g[A2][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x48));
            c.g[A0][0] = c.g[A0][0] << 20;
            c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[S0][0]);
            c.g[A0][0] = c.g[A3][0] | c.g[A0][0];
            c.mult(0, 3, 3, 13);
            c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7));
            if c.g[A1][0] != 0 {
                c.g[V0][0] = c.g[V1][0];
            }
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
            c.g[V1][0] = sx(0u32.wrapping_sub((c.g[V0][0] as u32)));
            c.g[V1][0] = c.g[V1][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T7][0] as u32) & 31)) as u32);
            c.g[T6][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V1][0] = c.g[T6][0] << 40;
            c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
            c.g[A2][0] = c.g[A2][0] | c.g[A0][0];
            c.sd((c.g[T0][0] as u32), c.g[A2][0]);
            c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x24));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x10));
            if !t {
                c.cycles += 20;
                c.g[A1][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x38));
                c.g[A0][0] = 0x1;
                c.g[T1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = c.g[A1][0] >> 26;
                c.g[V1][0] = c.g[A1][0] >> 30;
                c.g[V0][0] = c.g[V0][0] & 0xf;
                c.g[V1][0] = c.g[V1][0] & 0xf;
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                c.g[V1][0] = c.g[V1][0] << 32;
                c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                c.g[V0][0] = sx((c.g[A0][0] as u32) << ((c.g[V0][0] as u32) & 31));
                c.g[A2][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x4));
                c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[V1][0] as u32) & 31));
                c.g[T4][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                c.g[T5][0] = sx(((c.g[A0][0] as u32) as i32 >> 3) as u32);
                c.g[A3][0] = c.g[A1][0];
                c.g[V0][0] = (c.g[A2][0] < 0x1du64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[T3][0] = c.g[T1][0];
                if !tm_pick(c, !t, A2, 0x24c0, A0, A1, 0x00368d80) {
                    return;
                }
                c.cycles += 33;
                c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[A0][0] as i64)) as u64;
                c.g[V1][0] = ((c.g[T5][0] as i64) < (c.g[A1][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A0][0] = c.g[T4][0];
                }
                if c.g[V1][0] == 0 {
                    c.g[A1][0] = c.g[T5][0];
                }
                c.mult(0, 5, 4, 5);
                c.g[V1][0] = c.g[A3][0] >> 30;
                c.g[A0][0] = c.g[A3][0] >> 26;
                c.g[V1][0] = c.g[V1][0] & 0xf;
                c.g[V0][0] = 0x1;
                c.g[A0][0] = c.g[A0][0] & 0xf;
                c.g[V1][0] = c.g[V1][0] << 32;
                c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                c.g[A0][0] = c.g[A0][0] << 32;
                c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64;
                c.mult(0, 5, 5, 9);
                c.g[V1][0] = sx((c.g[V0][0] as u32) << ((c.g[V1][0] as u32) & 31));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A0][0] as u32) & 31));
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[T1][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                c.g[T4][0] = sx(((c.g[V1][0] as u32) as i32 >> 4) as u32);
                c.g[A3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x4));
                c.g[T5][0] = c.g[T3][0];
                c.g[A2][0] = ((c.g[A2][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7));
                if c.g[A2][0] != 0 {
                    c.g[V0][0] = c.g[A1][0];
                }
                c.g[A0][0] = (c.g[A3][0] < 0x1du64) as u64;
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                c.g[V1][0] = sx(0u32.wrapping_sub((c.g[V0][0] as u32)));
                c.g[V1][0] = c.g[V1][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T7][0] as u32) & 31)) as u32);
                let t = c.g[A0][0] == 0u64;
                c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                if !tm_pick(c, !t, A3, 0x2540, A1, A0, 0x00368e4c) {
                    return;
                }
                c.cycles += 34;
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[V1][0] = ((c.g[T4][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = c.g[T1][0];
                }
                if c.g[V1][0] == 0 {
                    c.g[A0][0] = c.g[T4][0];
                }
                c.mult(0, 4, 5, 4);
                c.g[V0][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x38));
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[V1][0] = c.g[V0][0] >> 30;
                c.g[T4][0] = c.g[T3][0];
                c.g[V0][0] = c.g[V0][0] >> 26;
                c.g[V1][0] = c.g[V1][0] & 0xf;
                c.mult(0, 4, 4, 13);
                c.g[V0][0] = c.g[V0][0] & 0xf;
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                c.g[V1][0] = c.g[V1][0] << 32;
                c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                c.g[V0][0] = sx((c.g[A1][0] as u32) << ((c.g[V0][0] as u32) & 31));
                c.g[A3][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x4));
                c.g[A1][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
                c.g[T1][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                c.g[A2][0] = ((c.g[A2][0] as i64) < (c.g[A0][0] as i64)) as u64;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7));
                if c.g[A2][0] != 0 {
                    c.g[V1][0] = c.g[A0][0];
                }
                c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 5) as u32);
                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
                c.g[A0][0] = (c.g[A3][0] < 0x1du64) as u64;
                c.g[V0][0] = sx(0u32.wrapping_sub((c.g[V1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0xff;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> ((c.g[T7][0] as u32) & 31)) as u32);
                let t = c.g[A0][0] == 0u64;
                c.g[T3][0] = sx((c.g[T6][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                if !tm_pick(c, !t, A3, 0x25c0, A2, V1, 0x00368f1c) {
                    return;
                }
                c.cycles += 26;
                c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A2][0] = c.g[T1][0];
                }
                if c.g[A0][0] == 0 {
                    c.g[V1][0] = c.g[A1][0];
                }
                c.mult(0, 3, 6, 3);
                c.g[A1][0] = 0xffffffffffffffff;
                c.g[A0][0] = c.g[T3][0];
                c.g[A2][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x50));
                c.g[A0][0] = c.g[A0][0] << 20;
                c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[S2][0]);
                c.g[A0][0] = c.g[T6][0] | c.g[A0][0];
                c.mult(0, 3, 3, 12);
                c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7));
                if c.g[A1][0] != 0 {
                    c.g[V0][0] = c.g[V1][0];
                }
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                c.g[V1][0] = sx(0u32.wrapping_sub((c.g[V0][0] as u32)));
                c.g[V1][0] = c.g[V1][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T7][0] as u32) & 31)) as u32);
                c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V0][0] = c.g[V0][0] << 40;
                c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                c.g[A2][0] = c.g[A2][0] | c.g[A0][0];
                c.sd((c.g[T0][0] as u32), c.g[A2][0]);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x10));
            }
        }
        c.cycles += 6;
        c.g[V0][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x40));
        c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[S3][0]);
        c.sd((c.g[T0][0] as u32), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x2c));
        let t = (c.g[V0][0] as i64) < 0;
        c.g[A0][0] = 0u64;
        if !t {
            c.cycles += 2;
            c.sw((c.g[T8][0] as u32).wrapping_add(0x1f54), c.g[T9][0]);
            c.g[A0][0] = c.g[V0][0];
        }
        c.cycles += 9;
        c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x28));
        c.g[A0][0] = c.g[A0][0] << 37;
        c.g[V1][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x38));
        c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
        c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[S4][0]);
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
    }
    c.cycles += 8;
    c.sw((c.g[S1][0] as u32), c.g[T0][0]);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(20, (c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t;
    return;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{self, Rng, DATA};

    /// `TexMan_EmitTex0` against the generated body. Random texture infos
    /// (pixel format 0..40 so the jump table's default arm is hit as well,
    /// random TEX0 size bits, 0..6 mip levels, CLUT address negative or not,
    /// format 9 or not) bound to slot 0/1, with the slot already holding the
    /// texture (CLUT current or not) or not. The page-size jump tables come
    /// from the game image.
    #[test]
    fn texman_emit_tex0_matches() {
        difftest::check(
            ee::s12::f_00368970__generated,
            texman_emit_tex0,
            600,
            &|c, seed| {
                let mut r = Rng(0x368970 + seed as u64 * 7919);
                difftest::elf_image(c);
                let this = DATA;
                let dma_ptr = DATA + 0x1000;
                let packet = DATA + 0x4000;
                let idx = r.next_u32() % 8;
                let slot = r.next_u32() & 1;
                for i in 0..10u32 {
                    let tex = DATA + 0x2000 + i * 0x80;
                    c.bus.write32(this + 8 + 4 * i, tex);
                    let psm = if seed % 5 == 0 {
                        r.next_u32() % 40
                    } else {
                        4 + r.next_u32() % 29
                    };
                    c.bus.write32(
                        tex + 0xc,
                        if r.next_u32() & 1 == 0 {
                            9
                        } else {
                            r.next_u32() % 16
                        },
                    );
                    c.bus.write32(tex + 0x18, psm);
                    c.bus.write32(tex + 0x24, r.next_u32() % 7);
                    c.bus.write32(tex + 0x28, r.next_u32() & 0x3fff);
                    let clut = if r.next_u32() & 1 == 0 {
                        r.next_u32() & 0x3fff
                    } else {
                        0xffff_fff0
                    };
                    c.bus.write32(tex + 0x2c, clut);
                    let tex0 = ((r.next_u32() as u64) << 32 | r.next_u32() as u64)
                        & 0xffff_ffff_0000_0000
                        | ((r.next_u32() % 11) as u64) << 26
                        | ((r.next_u32() % 11) as u64) << 30;
                    c.bus.write64(tex + 0x38, tex0);
                    c.bus.write64(
                        tex + 0x40,
                        (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    );
                    c.bus.write64(
                        tex + 0x48,
                        (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    );
                    c.bus.write64(
                        tex + 0x50,
                        (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    );
                }
                let bound = match seed % 4 {
                    0 | 1 => idx,
                    _ => idx + 1,
                };
                c.bus.write32(this + TEXMAN_BOUND + 4 * slot, bound);
                c.bus
                    .write32(this + TEXMAN_BOUND + 4 * (slot ^ 1), r.next_u32() % 8);
                c.bus.write32(
                    this + TEXMAN_CLUT_TEX,
                    if seed % 8 == 1 || seed % 8 == 4 {
                        idx
                    } else {
                        idx + 2
                    },
                );
                c.bus.write32(dma_ptr, packet);
                c.g[SP][0] = sx(difftest::SP);
                c.g[GP][0] = sx(difftest::GAME_GP);
                c.g[RA][0] = sx(difftest::RA);
                c.pc = 0x0036_8970;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = slot as u64;
                c.g[A3][0] = sx(dma_ptr);
            },
        );
    }
}
