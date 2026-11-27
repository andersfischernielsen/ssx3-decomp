//! Staging module oc1059 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1059`.

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
    0x0036_b9d8,
    ps2_graphics_man_emit_blur_downsample,
    "PS2GraphicsMan_EmitBlurDownsample",
)];

/// `PS2GraphicsMan` blur-downsample emitters' callees.
pub(crate) mod oc1059_blur {
    use crate::call::Callee;

    /// `SprStream_BeginBlock(stream, handle, -1, 0)`.
    pub const BEGIN_BLOCK: Callee = Callee {
        addr: 0x0038_f460,
        f: ssx3_ee::s12::SprStream_BeginBlock,
    };

    /// `SprStream_EndBlockAligned(stream, end, 4)`.
    pub const END_BLOCK_ALIGNED: Callee = Callee {
        addr: 0x0038_f668,
        f: ssx3_ee::s12::SprStream_EndBlockAligned,
    };
}

/// One column-strip pass of the blur: how the stack constants are offset
/// and which registers the generated code uses for the strip count, the
/// running texture offset and the three packed coordinate words.
struct BlurPass {
    /// Strips left to emit (`T1`, or `T2` in the last pass).
    cnt: usize,
    /// Running `i * du << 9` accumulator (`T2`, or `T1` in the last pass).
    acc: usize,
    /// Register holding `height << 4 << 16`.
    hh: usize,
    /// Register holding the packed u and v offsets of the strip's first and
    /// second vertex (`uv0`, `uv1`).
    uv0: usize,
    uv1: usize,
    /// The strip set-up adds (`true`) or subtracts the blur offset.
    setup_add: bool,
    /// The per-strip loop adds or subtracts the blur offset.
    body_add: bool,
    /// Cycles of the set-up block.
    setup_cycles: u64,
    /// Loop head, for the event check at the back edge.
    head: u32,
}

/// Emit the strips of one blur pass into the stream block at `T9` (the
/// generated block after "any strips?"): per strip two vertices (`uv`
/// packed words, texture coordinates `x0` and `x1 | height << 20`) as
/// four 16-byte GIF primitives. Leaves every register as the generated
/// loop does (T9 past the packets, `T0`/`A3` advanced).
fn oc1059_blur_strips(c: &mut Cpu, p: &BlurPass) {
    let sp = |c: &Cpu, off: u32| (c.g[SP][0] as u32).wrapping_add(off);
    let off_arg = c.lw(sp(c, 0x100)); // blur offset
    c.cycles += p.setup_cycles;
    let hh_arg = c.lw(sp(c, 0xf8));
    c.g[T5][0] = 0;
    c.g[T4][0] = 3;
    let height16 = sx((hh_arg as u32) << 4);
    let a0 = c.lw(sp(c, 0x10));
    let a2 = c.lw(sp(c, 0x14));
    let step = |a: u64, o: u64, add: bool| {
        if add {
            sx((a as u32).wrapping_add(o as u32))
        } else {
            sx((a as u32).wrapping_sub(o as u32))
        }
    };
    let u_setup = step(a0, off_arg, p.setup_add);
    let v_setup = step(a2, off_arg, p.setup_add);
    c.g[p.hh][0] = height16 << 16;
    c.g[p.uv0][0] = u_setup << 16;
    let du = c.lw(sp(c, 0xc));
    c.g[p.cnt][0] = c.lw(sp(c, 0x18));
    c.g[p.uv1][0] = v_setup << 16;
    c.g[T3][0] = 5;
    c.g[p.acc][0] = 0;
    c.g[T6][0] = sx((du as u32) << 9);
    // The loop overwrites V0, V1, A0, A1 and A2 on its first pass.
    c.g[V0][0] = du;
    c.g[V1][0] = v_setup;
    c.g[A0][0] = a0;
    c.g[A1][0] = off_arg;
    c.g[A2][0] = a2;
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.lw(sp(c, 0xc));
        c.cycles += 2;
        c.cycles += 31;
        c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x200));
        c.mult(0, 3, 8, 2);
        let t9 = c.g[T9][0] as u32;
        c.sd(t9.wrapping_add(0x10), c.g[A3][0]);
        c.div(0, p.acc, 30);
        c.g[V0][0] = c.g[T0][0] | c.g[p.hh][0];
        c.sd(t9.wrapping_add(0x8), c.g[T4][0]);
        c.g[A1][0] = ((c.g[A2][0] as i64) < (c.g[FP][0] as i64)) as u64;
        c.sd(t9.wrapping_add(0x18), c.g[T3][0]);
        c.g[T0][0] = c.g[A2][0];
        c.sd(t9.wrapping_add(0x28), c.g[T4][0]);
        if c.g[A1][0] == 0 {
            c.g[T0][0] = c.g[FP][0];
        }
        c.sd(t9.wrapping_add(0x38), c.g[T3][0]);
        c.g[p.acc][0] = sx((c.g[p.acc][0] as u32).wrapping_add(c.g[T6][0] as u32));
        c.sd(t9.wrapping_add(0x30), c.g[V0][0]);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x200));
        c.g[V0][0] = c.lw(sp(c, 0x8));
        c.g[p.cnt][0] = sx((c.g[p.cnt][0] as u32).wrapping_sub(1));
        c.g[A1][0] = c.lw(sp(c, 0x100));
        c.g[A0][0] = c.lo[0];
        c.div(0, 3, 30);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A0][0] = step(c.g[A0][0], c.g[A1][0], p.body_add);
        c.g[A0][0] |= c.g[p.uv0][0];
        c.sd(t9, c.g[A0][0]);
        c.g[V1][0] = c.lo[0];
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = step(c.g[V1][0], c.g[A1][0], p.body_add);
        c.g[V1][0] |= c.g[p.uv1][0];
        c.sd(t9.wrapping_add(0x20), c.g[V1][0]);
        let more = c.g[p.cnt][0] != 0;
        c.g[T9][0] = sx(t9.wrapping_add(0x40));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, p.head);
        }
    }
}

/// `void PS2GraphicsMan::EmitBlurDownsample(...)` (0x36b9d8): writes the
/// GIF packets of a blur-by-downsample into the scratchpad stream
/// (`gp+0x1384`, via `SprStream_BeginBlock` / `EndBlockAligned`). A header
/// block sets up the GS (frame, z-buffer, scissor, texture, clamp, the grey
/// vertex colour `intensity / passes`), then up to four passes
/// (`passes` at `sp+0x108`, a blur offset at `sp+0x100`) each draw the
/// source rectangle as 32-pixel-wide sprite strips, shifted by
/// (-off,-off), (+off,+off), (+off,-off) and (-off,+off) texels, which
/// the GS blends together.
pub fn ps2_graphics_man_emit_blur_downsample(c: &mut Cpu) {
    use oc1059_blur::{BEGIN_BLOCK, END_BLOCK_ALIGNED};
    guest(c, |c| {
        c.cycles += 198;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S2][0] = c.g[T0][0];
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[S4][0] = c.g[A3][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[T4][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[T2][0] as u32)));
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[V0][0] = sx((c.g[S4][0] as u32) << 4);
        c.g[S1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[T1][0] as u32)));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = sx((c.g[S0][0] as u32) << 4);
        c.sw((c.g[SP][0] as u32), c.g[A1][0]);
        c.g[S5][0] = 0xffffffffffffffff;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1f));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[T0][0] as u32)));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S7][0] = ((c.g[S5][0] as i64) < (c.g[S1][0] as i64)) as u64;
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
        if c.g[S7][0] != 0 {
            c.g[A1][0] = c.g[S1][0];
        }
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 5) as u32);
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 4);
        c.g[S6][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5a80));
        c.g[S3][0] = c.g[A2][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[V1][0] = sx((c.g[T4][0] as u32) << 4);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[T0][0] as u32)));
        c.g[A1][0] = c.lw((c.g[A0][0] as u32));
        c.g[FP][0] = sx((c.g[S1][0] as u32) << 4);
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
        c.mov_s(20, 12);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[T3][0]);
        c.g[A3][0] = 0u64;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V1][0]);
        c.g[A2][0] = 0xffffffffffffffff;
        c.sq(12, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = sx(0x0036bab0);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        BEGIN_BLOCK.call(c, 0x0036bab0)?;
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x108));
        c.cvt_s_w(0, 0);
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x108));
        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
        c.g[S5][0] = ((c.g[S5][0] as i64) < (c.g[S3][0] as i64)) as u64;
        c.g[T2][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3f));
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3f));
        if c.g[S5][0] != 0 {
            c.g[A0][0] = c.g[S3][0];
        }
        c.div_s(20, 20, 0);
        c.mult(0, 10, 6, 10);
        if c.g[S7][0] != 0 {
            c.g[A1][0] = c.g[S1][0];
        }
        c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 6) as u32);
        c.g[V1][0] = 0u64;
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 6) as u32);
        c.g[T1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[A3][0] = 0x1;
        c.g[T9][0] = c.g[V0][0];
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[T3][0] = c.g[A3][0];
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        if c.g[T1][0] != 0 {
            c.g[T3][0] = c.g[A0][0];
        }
        if c.g[V1][0] != 0 {
            c.g[A3][0] = c.g[A1][0];
        }
        c.g[A1][0] = sx((c.g[T2][0] as u32).wrapping_add(0xe));
        c.g[V1][0] = 0x10000000;
        c.g[A0][0] = 0x1000000;
        c.g[A1][0] = c.g[A1][0] | c.g[V1][0];
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
        c.cvt_w_s(0, 20);
        c.mfc1(8, 0);
        c.g[S6][0] = c.g[S6][0] | c.g[A0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf8));
        c.g[A0][0] = c.g[V0][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
        c.g[S0][0] = c.g[S0][0] << 14;
        c.g[A2][0] = 0xa;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32));
        c.lq(12, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.g[S0][0] | c.g[A2][0];
        c.sd((c.g[T9][0] as u32), c.g[A1][0]);
        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        c.g[A3][0] = c.g[A3][0] << 16;
        c.g[T3][0] = c.g[T3][0] << 14;
        c.g[A3][0] = c.g[A0][0] | c.g[A3][0];
        c.g[T3][0] = c.g[V0][0] | c.g[T3][0];
        c.g[V1][0] = 0xffffffffff000000;
        c.g[V1][0] = c.g[V1][0] << 16;
        c.g[V1][0] = c.g[V1][0] | 0x100;
        c.g[V1][0] = c.g[V1][0] << 16;
        c.g[A1][0] = 0xd502;
        c.g[A1][0] = c.g[A1][0] << 19;
        c.sd((c.g[T9][0] as u32).wrapping_add(0x8), 0u64);
        c.g[A3][0] = c.g[A3][0] | c.g[V1][0];
        c.g[T3][0] = c.g[T3][0] | c.g[A1][0];
        c.g[S4][0] = c.g[S4][0] << 4;
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0xd));
        c.g[V0][0] = 0x8000;
        c.g[V0][0] = c.g[V0][0] << 45;
        c.g[A0][0] = 0x8000;
        c.g[A0][0] = c.g[A0][0] << 17;
        c.g[T9][0] = sx((c.g[T9][0] as u32).wrapping_add(0x10));
        c.g[T2][0] = c.g[T2][0] | c.g[V0][0];
        c.g[S4][0] = c.g[S4][0] | c.g[S0][0];
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
        c.g[S2][0] = c.g[S2][0] << 24;
        c.g[T1][0] = c.g[T0][0] << 8;
        c.g[S6][0] = c.g[S6][0] | c.g[A0][0];
        c.g[V1][0] = 0xe;
        c.g[A1][0] = 0x4e;
        c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x1));
        c.sd((c.g[T9][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x10), c.g[A3][0]);
        c.g[S4][0] = c.g[S4][0] | c.g[S2][0];
        c.sd((c.g[T9][0] as u32), c.g[T2][0]);
        c.g[A2][0] = c.g[A2][0] << 48;
        c.sd((c.g[T9][0] as u32).wrapping_add(0x28), c.g[A1][0]);
        c.g[T4][0] = c.g[T4][0] << 34;
        c.sd((c.g[T9][0] as u32).wrapping_add(0x20), c.g[S6][0]);
        c.g[T1][0] = c.g[T0][0] | c.g[T1][0];
        c.g[S1][0] = c.g[S1][0] << 16;
        c.g[A0][0] = 0x4c;
        c.sd((c.g[T9][0] as u32).wrapping_add(0x18), c.g[A0][0]);
        c.g[T0][0] = c.g[T0][0] << 16;
        c.g[S1][0] = c.g[S1][0] | c.g[A2][0];
        c.g[S4][0] = c.g[S4][0] | c.g[T4][0];
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
        c.g[T1][0] = c.g[T1][0] | c.g[T0][0];
        c.g[T4][0] = 0x47;
        c.g[T5][0] = 0x18;
        c.g[T6][0] = 0x80;
        c.g[T6][0] = c.g[T6][0] << 32;
        c.g[T6][0] = c.g[T6][0] | 0x68;
        c.g[T7][0] = 0x42;
        c.g[T8][0] = 0x6;
        c.g[A2][0] = 0x30000;
        c.g[V0][0] = 0x3f800000;
        c.g[V0][0] = c.g[V0][0] << 16;
        c.g[V0][0] = c.g[V0][0] | 0x8000;
        c.g[V0][0] = c.g[V0][0] << 16;
        c.g[A1][0] = 0x8;
        c.g[S0][0] = 0x3f;
        c.g[S2][0] = 0x40;
        c.g[S3][0] = 0x60;
        c.g[S5][0] = 0x14;
        c.g[T1][0] = c.g[T1][0] | c.g[V0][0];
        c.sd((c.g[T9][0] as u32).wrapping_add(0x30), c.g[A2][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x38), c.g[T4][0]);
        c.g[V0][0] = 0x116;
        c.sd((c.g[T9][0] as u32).wrapping_add(0x48), c.g[T5][0]);
        c.g[V1][0] = 0x1;
        c.sd((c.g[T9][0] as u32).wrapping_add(0x50), c.g[T6][0]);
        c.g[A2][0] = 0x4;
        c.sd((c.g[T9][0] as u32).wrapping_add(0x58), c.g[T7][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x90), c.g[T3][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x98), c.g[T8][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0xa8), c.g[A1][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x68), c.g[S0][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x70), c.g[S1][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x78), c.g[S2][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x80), c.g[S3][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x88), c.g[S5][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0xa0), c.g[S4][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x40), 0u64);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x60), 0u64);
        c.g[T9][0] = sx((c.g[T9][0] as u32).wrapping_add(0xb0));
        c.sd((c.g[T9][0] as u32), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[T9][0] as u32).wrapping_add(0x20));
        c.sd((c.g[T9][0] as u32).wrapping_add(0x10), c.g[T1][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x18), c.g[V1][0]);
        c.g[RA][0] = sx(0x0036bcc0);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x8), 0u64);
        END_BLOCK_ALIGNED.call(c, 0x0036bcc0)?;
        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[A3][0] = 0u64;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
        c.g[A1][0] = c.g[V0][0];
        c.sw((c.g[A2][0] as u32), c.g[V0][0]);
        c.g[RA][0] = sx(0x0036bcdc);
        c.g[A2][0] = 0xffffffffffffffff;
        BEGIN_BLOCK.call(c, 0x0036bcdc)?;
        c.g[T9][0] = c.g[V0][0];
        c.g[T0][0] = 0x200;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0u64;
        if !t {
            oc1059_blur_strips(
                c,
                &BlurPass {
                    cnt: T1,
                    acc: T2,
                    hh: S0,
                    uv0: S1,
                    uv1: T7,
                    setup_add: false,
                    body_add: false,
                    setup_cycles: 18,
                    head: 0x0036_bd38,
                },
            );
        }
        'epilogue: {
            // End pass 1, then pass 2 if `passes >= 2`.
            c.cycles += 13;
            c.g[V0][0] = 0x156;
            c.g[A1][0] = sx((c.g[T9][0] as u32).wrapping_add(0x10));
            c.sd((c.g[T9][0] as u32), c.g[V0][0]);
            c.g[A2][0] = 0x4;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
            c.g[RA][0] = sx(0x0036bde4);
            c.sd((c.g[T9][0] as u32).wrapping_add(0x8), 0u64);
            END_BLOCK_ALIGNED.call(c, 0x0036bde4)?;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x108));
            c.g[A1][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x110));
            c.g[V0][0] = ((c.g[A2][0] as i64) < 2i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.sw((c.g[V1][0] as u32), c.g[A1][0]);
            if t {
                break 'epilogue;
            }
            // Pass 2: (+off, +off).
            c.cycles += 11;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
            c.g[A3][0] = 0u64;
            c.g[RA][0] = sx(0x0036be0c);
            c.g[A2][0] = 0xffffffffffffffff;
            BEGIN_BLOCK.call(c, 0x0036be0c)?;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x108));
            c.g[T9][0] = c.g[V0][0];
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            c.g[T0][0] = 0x200;
            c.g[S2][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
            let t = (c.g[A1][0] as i64) <= 0;
            c.g[A3][0] = 0u64;
            if !t {
                oc1059_blur_strips(
                    c,
                    &BlurPass {
                        cnt: T1,
                        acc: T2,
                        hh: S0,
                        uv0: S1,
                        uv1: T7,
                        setup_add: true,
                        body_add: true,
                        setup_cycles: 18,
                        head: 0x0036_be70,
                    },
                );
            }
            c.cycles += 7;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
            c.g[A2][0] = 0x4;
            c.g[RA][0] = sx(0x0036bf10);
            c.g[A1][0] = c.g[T9][0];
            END_BLOCK_ALIGNED.call(c, 0x0036bf10)?;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x110));
            let t = c.g[S2][0] != 0u64;
            c.sw((c.g[A2][0] as u32), c.g[V0][0]);
            if t {
                break 'epilogue;
            }
            // Pass 3: (+off, -off).
            c.cycles += 12;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
            c.g[A3][0] = 0u64;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0036bf30);
            c.g[A2][0] = 0xffffffffffffffff;
            BEGIN_BLOCK.call(c, 0x0036bf30)?;
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x108));
            c.g[T9][0] = c.g[V0][0];
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            c.g[T0][0] = 0x200;
            c.g[S2][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
            let t = (c.g[A0][0] as i64) <= 0;
            c.g[A3][0] = 0u64;
            if !t {
                oc1059_blur_strips(
                    c,
                    &BlurPass {
                        cnt: T1,
                        acc: T2,
                        hh: S0,
                        uv0: S1,
                        uv1: T7,
                        setup_add: false,
                        body_add: true,
                        setup_cycles: 17,
                        head: 0x0036_bf90,
                    },
                );
            }
            c.cycles += 7;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
            c.g[A2][0] = 0x4;
            c.g[RA][0] = sx(0x0036c030);
            c.g[A1][0] = c.g[T9][0];
            END_BLOCK_ALIGNED.call(c, 0x0036c030)?;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x110));
            let t = c.g[S2][0] != 0u64;
            c.sw((c.g[A2][0] as u32), c.g[V0][0]);
            if t {
                break 'epilogue;
            }
            // Pass 4: (-off, +off).
            c.cycles += 10;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
            c.g[A3][0] = 0u64;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0036c050);
            c.g[A2][0] = 0xffffffffffffffff;
            BEGIN_BLOCK.call(c, 0x0036c050)?;
            c.g[T9][0] = c.g[V0][0];
            c.g[A3][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[T0][0] = 0x200;
            if !t {
                oc1059_blur_strips(
                    c,
                    &BlurPass {
                        cnt: T2,
                        acc: T1,
                        hh: T7,
                        uv0: S1,
                        uv1: S0,
                        setup_add: true,
                        body_add: false,
                        setup_cycles: 17,
                        head: 0x0036_c0a8,
                    },
                );
            }
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
            c.g[A2][0] = 0x4;
            c.g[RA][0] = sx(0x0036c148);
            c.g[A1][0] = c.g[T9][0];
            END_BLOCK_ALIGNED.call(c, 0x0036c148)?;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x110));
            c.sw((c.g[A2][0] as u32), c.g[V0][0]);
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP, SP as STACK_TOP};

    /// `PS2GraphicsMan_EmitBlurDownsample`: random sizes (including zero and
    /// negative widths, so no strips), `passes` 0..=5 (each exit after
    /// passes 1-4), random intensity, offsets and rectangle; a scratchpad
    /// stream (two slots whose buffers are in RAM) at gp+0x1384 with the
    /// real `SprStream_BeginBlock` / `EndBlockAligned` callees.
    #[test]
    fn ps2_graphics_man_emit_blur_downsample_matches() {
        check(
            ee::s12::f_0036b9d8__generated,
            ps2_graphics_man_emit_blur_downsample,
            250,
            &|c, seed| {
                let mut r = Rng(0x0036_b9d8 ^ (seed as u64) << 13);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[SP][0] = sx(STACK_TOP);
                let man = DATA + 0x1_0000;
                let stream = DATA + 0x7_0000;
                let handle = DATA + 0x7_8000;
                c.bus.write32(GAME_GP + 0x1384, stream);
                let slot = r.next_u32() % 2;
                c.bus.write32(stream + 4, 2);
                c.bus.write32(stream + 8, slot);
                c.bus.write32(stream + 0xc, slot);
                c.bus.write32(stream + 0x10, 0x101);
                c.bus.write32(stream + 0x14, DATA + 0x7_1000);
                c.bus.write32(stream + 0x18, DATA + 0x7_3000);
                fill(c, &mut r, DATA + 0x7_1000, 0x400);
                fill(c, &mut r, DATA + 0x7_3000, 0x400);
                fill(c, &mut r, man + 0x5a70, 0x20);
                c.bus.write32(handle, DATA + 0x7_4000);
                let width = match seed % 7 {
                    0 => 0,
                    1 => -40i32 as u32,
                    2 => 32,
                    3 => 256,
                    4 => 100,
                    _ => 1 + r.next_u32() % 256,
                };
                let passes = [1u32, 2, 3, 4, 5, 0, 4, 3][(seed / 7 % 8) as usize];
                let fb_w = [0u32, 64, 640, 100, -64i32 as u32][(seed % 5) as usize];
                c.g[A0][0] = sx(man);
                c.g[A1][0] = sx(r.next_u32() & 0x3fff_ffff);
                c.g[A2][0] = sx(fb_w);
                c.g[A3][0] = r.next_u32() as u64 % 600;
                c.g[T0][0] = r.next_u32() as u64 % 400;
                c.g[T1][0] = r.next_u32() as u64 % 600;
                c.g[T2][0] = r.next_u32() as u64 % 400;
                c.g[T3][0] = sx(r.next_u32() & 0x3fff_ffff);
                c.f[12] = r.f32(255.0);
                let args = STACK_TOP;
                c.bus.write32(args, width); // width
                c.bus.write32(args + 0x8, 1 + r.next_u32() % 300); // height
                c.bus.write32(args + 0x10, r.next_u32() % 4); // blur offset
                c.bus.write32(args + 0x18, passes);
                c.bus.write32(args + 0x20, handle);
            },
        );
    }
}
