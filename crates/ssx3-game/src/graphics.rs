//! Work package E5 graphics manager & draw deferral (docs/investigation/phase2-candidates-3.md).
//!
//! Hand-written, typed replacements for the PS2 graphics manager and draw
//! deferral (sprite/packet stream) functions, each with a differential test
//! against the generated body (docs/investigation/phase2-worker-guide.md).
//!
//! The stream functions drive the scratchpad DMA: `SprStream_BeginBlock`
//! charges the block into the current index slot, `SprStream_EndBlock` kicks
//! the DMA transfer and advances the index, and the `*Upload` helpers wrap
//! `SprStream_BeginBlock` with a `Dma_ToScratchpad`. `DeferMan_*` walk the
//! per-state/pass bucket hash tables that queue deferred draw records.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::deliver_event_here;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0039_56b0,
        ps2_graphics_man_ensure_view_matrices,
        "PS2GraphicsMan_EnsureViewMatrices",
    ),
    (
        0x0036_9cb0,
        graphics_man_draw_model_simple,
        "GraphicsMan_DrawModelSimple",
    ),
    (
        0x0039_53b8,
        ps2_graphics_man_get_world_to_clip_matrix,
        "PS2GraphicsMan_GetWorldToClipMatrix",
    ),
    (0x0038_f708, spr_stream_upload, "SprStream_Upload"),
    (
        0x0038_f668,
        spr_stream_end_block_aligned,
        "SprStream_EndBlockAligned",
    ),
    (0x0038_f460, spr_stream_begin_block, "SprStream_BeginBlock"),
    (
        0x0038_f4f8,
        spr_stream_begin_block_upload,
        "SprStream_BeginBlockUpload",
    ),
    (0x0038_f598, spr_stream_end_block, "SprStream_EndBlock"),
];

// ---------------------------------------------------------------------------
// Callees.

/// Direct calls (same-shard `jal` through the generated function, which
/// dispatches through its override slot when it has one).
mod callee {
    use super::{ee, Callee};

    pub const DMA_TO_SCRATCHPAD: Callee = Callee {
        addr: 0x0037_1d10,
        f: ee::s12::Dma_ToScratchpad,
    };
    pub const DMA_FROM_SCRATCHPAD: Callee = Callee {
        addr: 0x0037_1dd8,
        f: ee::s12::Dma_FromScratchpad,
    };
    pub const DMA_WAIT_FROM_SCRATCHPAD: Callee = Callee {
        addr: 0x0038_f768,
        f: ee::s12::Dma_WaitFromScratchpad,
    };
    pub const SPR_STREAM_BEGIN_BLOCK: Callee = Callee {
        addr: 0x0038_f460,
        f: ee::s12::SprStream_BeginBlock,
    };
    pub const SPR_STREAM_END_BLOCK: Callee = Callee {
        addr: 0x0038_f598,
        f: ee::s12::SprStream_EndBlock,
    };
    pub const SPR_STREAM_NEXT_INDEX: Callee = Callee {
        addr: 0x0038_f6a8,
        f: ee::s12::SprStream_NextIndex,
    };
    pub const PS2_GRAPHICS_MAN_ENSURE_VIEW_MATRICES: Callee = Callee {
        addr: 0x0039_56b0,
        f: ee::s13::PS2GraphicsMan_EnsureViewMatrices,
    };
}

// ---------------------------------------------------------------------------
// Graphics manager.

/// `void PS2GraphicsMan_EnsureViewMatrices(PS2GraphicsMan *man)` (0x3956b0):
/// rebuild the cached view matrices with `PS2GraphicsMan_UpdateViewMatrices`
/// when they are not valid (`man+0x6b90 == 0`).
pub fn ps2_graphics_man_ensure_view_matrices(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let man = Addr(abi::arg(c, 0));
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = man.off(0x6b90).lw(c);
        let valid = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !valid {
            c.cycles += 3;
            // Cross-shard: the generator emitted this as `call_indirect`.
            call_ptr(c, 0x0037_d968, 0x0039_56cc)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void GraphicsMan_DrawModelSimple(GraphicsMan *man, void *model, int a,
/// int b, int c)` (0x369cb0): fetch the model's skin from `man+0x10d8`, read
/// the virtual draw method at `skin+0x2fc` (with the `lh` delta at +0x2f8)
/// and call it with `f12 = 1.0`.
pub fn graphics_man_draw_model_simple(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[T2][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[T0][0] = c.g[A2][0];
        c.g[T1][0] = c.g[A3][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 12);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x10d8).lw(c);
        c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = Addr(c.g[A1][0] as u32).off(0x94).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x2f8).lh(c);
        c.g[T3][0] = Addr(c.g[V0][0] as u32).off(0x2fc).lw(c);
        let method = c.g[T3][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, method, 0x0036_9ce8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Vec4 *PS2GraphicsMan_GetWorldToClipMatrix(PS2GraphicsMan *man, Vec4 *out)`
/// (0x3953b8): ensure the view matrices, then copy four quads from
/// `man+0x58a0` to `out` and return `out`.
pub fn ps2_graphics_man_get_world_to_clip_matrix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::PS2_GRAPHICS_MAN_ENSURE_VIEW_MATRICES.call(c, 0x0039_53d8)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58a0));
        c.lqc2(1, c.g[S0][0] as u32);
        c.lqc2(2, (c.g[S0][0] as u32).wrapping_add(0x10));
        c.lqc2(3, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(0x30));
        c.sqc2(1, c.g[S1][0] as u32);
        c.sqc2(2, (c.g[S1][0] as u32).wrapping_add(0x10));
        c.sqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
        c.sqc2(4, (c.g[S1][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sprite/packet stream.

/// `void SprStream_Upload(SprStream *s, void *data, int size, int slot)`
/// (0x38f708): hand the block to `Dma_ToScratchpad` at the scratchpad base
/// (`gp+0x2a94`), with the size in 16-byte units.
pub fn spr_stream_upload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[V0][0] = sx(((c.g[A1][0] as u32) as i32 >> 4) as u32);
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A2][0];
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a94)).lw(c);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[V0][0];
        callee::DMA_TO_SCRATCHPAD.call(c, 0x0038_f724)?;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a94)).lw(c);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SprStream_EndBlockAligned(SprStream *s, int size)` (0x38f668): align
/// `size` up so the block ends on a 16-byte boundary, then `SprStream_EndBlock`.
pub fn spr_stream_end_block_aligned(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A3][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A3][0] as u32));
        c.g[A1][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
        c.g[A1][0] &= 0xf;
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        callee::SPR_STREAM_END_BLOCK.call(c, 0x0038_f698)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SprStream_BeginBlock(SprStream *s, void *src, int size, int flags)`
/// (0x38f460): wait for the previous upload when `flags & 4`, claim the next
/// index slot (bumping the sequence number) and stash `src`/`size` there.
/// Returns the slot's packet address.
pub fn spr_stream_begin_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[A3][0] &= 0x4;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        let wait = c.g[A3][0] != 0;
        c.g[S2][0] = c.g[A1][0];
        if wait {
            c.cycles += 2;
            callee::DMA_WAIT_FROM_SCRATCHPAD.call(c, 0x0038_f490)?;
        }
        // size < 0 selects the fixed size at +0x10, else size/16.
        c.cycles += 2;
        let sized = (c.g[S1][0] as i64) >= 0;
        c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 4) as u32);
        if !sized {
            c.cycles += 1;
            c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
        }
        // claim the slot: index = (index + 1) % count.
        c.cycles += 9;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.div(0, V0, A0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.cycles += 14;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = c.g[V0][0];
        frame.restore_ra(c, 0);
        c.g[A1][0] = c.hi[0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x20), c.g[S2][0]);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x2c), c.g[S1][0]);
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int SprStream_BeginBlockUpload(SprStream *s, void *src, int size,
/// int flags)` (0x38f4f8): `SprStream_BeginBlock` followed by a
/// `Dma_ToScratchpad` of the block. Returns the slot's packet address.
pub fn spr_stream_begin_block_upload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        callee::SPR_STREAM_BEGIN_BLOCK.call(c, 0x0038_f524)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = sx(((c.g[S2][0] as u32) as i32 >> 4) as u32);
        c.g[A3][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S0][0];
        callee::DMA_TO_SCRATCHPAD.call(c, 0x0038_f53c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SprStream_EndBlock(SprStream *s, int mode, int flags)` (0x38f598):
/// either start the DMA of the current block (and advance the index), or,
/// when `mode != 0`, finish the in-flight download and rewind the index.
pub fn spr_stream_end_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        let download = c.g[S2][0] != 0;
        frame.save_ra(c, 0);
        if !download {
            // Start the upload of the current block.
            c.cycles += 3;
            let v0;
            if c.g[A2][0] & 0x6 == 0 {
                v0 = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
            } else {
                c.cycles += 3;
                callee::DMA_WAIT_FROM_SCRATCHPAD.call(c, 0x0038_f5cc)?;
                v0 = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
            }
            c.g[V0][0] = v0;
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x20).lw(c);
            callee::SPR_STREAM_NEXT_INDEX.call(c, 0x0038_f5e4)?;
            c.g[V0][0] = c.g[S0][0];
        } else {
            // Finish the download: kick `Dma_FromScratchpad` and step the
            // index back to the slot just transferred.
            c.cycles += 20;
            c.g[S2][0] = sx(((c.g[S2][0] as u32) as i32 >> 4) as u32);
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[A3][0] = c.g[A2][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A2][0] = c.g[S2][0];
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x14).lw(c);
            callee::DMA_FROM_SCRATCHPAD.call(c, 0x0038_f61c)?;
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
            c.g[A2][0] = sx((c.g[S2][0] as u32) << 4);
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.div(0, A0, V1);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.cycles += 5;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[A1][0] = c.hi[0];
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[A1][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Deferred draw buckets.

/// `DeferManBucket *DeferMan_FindBucket(DeferMan *man, uint hash, uint key1,
/// uint pass, uint key2)` (0x395000): walk the bucket chain for
/// `(key1, key2)` in slot `man + 0x678a0 + pass*4`, returning the matching
/// node (or 0 when absent). A found node is moved to the head of the chain.
pub fn defer_man_find_bucket(c: &mut Cpu) {
    c.cycles += 9;
    c.g[A3][0] &= 0xff;
    c.g[V0][0] = 0x6_0000;
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.g[V0][0] |= 0x78a0;
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[T3][0] as u32).lw(c);
    let empty = c.g[V0][0] == 0;
    c.g[V0][0] = 0x6_0000;
    if empty {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[T4][0] = 0x1;
    c.g[V0][0] |= 0x78a0;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    'l_395030: loop {
        // Search one node's key pair.
        c.cycles += 5;
        c.g[V1][0] = Addr(c.g[T3][0] as u32).lw(c);
        c.g[T2][0] = 0;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
        let has_next = c.g[V0][0] != 0;
        c.g[T1][0] = c.g[V1][0];
        if !has_next {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[T1][0] as u32).off(0x4).lw(c);
            c.g[V1][0] = 0;
            if c.g[V0][0] != c.g[A2][0] {
                c.cycles += 1;
                c.g[T2][0] = c.g[T4][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[T1][0] as u32).off(0x8).lw(c);
                c.g[T2][0] = c.g[T4][0];
                if c.g[V0][0] == c.g[T0][0] {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
                    c.g[V1][0] = c.g[T4][0];
                    c.g[V0][0] ^= c.g[A1][0];
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = 0;
                    }
                    c.cycles += 1;
                    c.g[T2][0] = c.g[T4][0];
                }
            }
            c.cycles += 1;
            if c.g[V1][0] == 0 {
                c.g[T2][0] = 0;
            }
        }
        c.cycles += 2;
        if c.g[T2][0] == 0 {
            c.g[V0][0] = Addr(c.g[T1][0] as u32).off(0x1c).lw(c);
            c.cycles += 2;
            let more = c.g[V0][0] != 0;
            c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1c));
            if more {
                if c.cycles >= c.next_event {
                    c.pc = 0x0039_5030;
                    deliver_event_here(c, c.pc);
                }
                continue 'l_395030;
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.cycles += 4;
        c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
        if c.g[T1][0] != c.g[V0][0] {
            c.g[A0][0] = Addr(c.g[T1][0] as u32).off(0x1c).lw(c);
            c.cycles += 6;
            c.g[V0][0] = c.g[T1][0];
            c.sw(c.g[T3][0] as u32, c.g[A0][0]);
            c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
            c.sw((c.g[T1][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
            c.sw(c.g[A3][0] as u32, c.g[T1][0]);
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[T1][0];
        }
        abi::ret(c);
        return;
    }
}

/// `DeferManRecord *DeferMan_InternState(DeferMan *man, void *state,
/// int pass)` (0x394ed0): dedupe a state record. Look for a bucket whose
/// four-word state matches; on a hit, relink it to the head and return it,
/// otherwise insert a fresh record at `man+0x51480`.
pub fn defer_man_intern_state(c: &mut Cpu) {
    c.cycles += 10;
    c.g[A3][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = 0x6_0000;
    c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
    c.g[V0][0] |= 0x74a0;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[T2][0] = c.g[A0][0];
    c.g[T1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
    let empty = c.g[V0][0] == 0;
    c.g[T3][0] = c.g[A1][0];
    if !empty {
        c.cycles += 4;
        c.g[V0][0] = 0x6_0000;
        c.g[T5][0] = c.g[V1][0];
        c.g[V0][0] |= 0x74a0;
        c.g[T4][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        'l_394f08: loop {
            c.cycles += 4;
            c.g[A2][0] = Addr(c.g[T1][0] as u32).lw(c);
            c.g[T0][0] = 0;
            c.g[A1][0] = c.g[T3][0];
            c.g[A0][0] = c.g[A2][0];
            // Compare up to four words of the state record.
            let all_match = 'compare: loop {
                c.cycles += 4;
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
                let mismatch = c.g[V1][0] != c.g[V0][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                if mismatch {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    break 'compare false;
                }
                c.cycles += 4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                let more = c.g[T0][0] < 0x4;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                if more {
                    if c.cycles >= c.next_event {
                        c.pc = 0x0039_4f18;
                        deliver_event_here(c, c.pc);
                    }
                    continue 'compare;
                }
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                break 'compare true;
            };
            c.cycles += 2;
            c.g[A1][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[T5][0] as u32));
            if !all_match {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x14).lw(c);
                let more = c.g[V0][0] != 0;
                c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x14));
                if more {
                    if c.cycles >= c.next_event {
                        c.pc = 0x0039_4f08;
                        deliver_event_here(c, c.pc);
                    }
                    continue 'l_394f08;
                }
                break 'l_394f08;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            if c.g[A2][0] != c.g[V0][0] {
                c.g[A0][0] = Addr(c.g[A2][0] as u32).off(0x14).lw(c);
                c.cycles += 6;
                c.g[V0][0] = c.g[A2][0];
                c.sw(c.g[T1][0] as u32, c.g[A0][0]);
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.sw((c.g[A2][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                c.sw(c.g[A1][0] as u32, c.g[A2][0]);
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[A2][0];
            }
            abi::ret(c);
            return;
        }
    }
    // Insert a fresh record.
    c.cycles += 31;
    c.g[V1][0] = 0x5_0000;
    c.g[A1][0] = 0x18;
    c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A2][0] = 0x5_0000;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1480).lw(c);
    c.g[A0][0] = 0x6_0000;
    c.g[A2][0] |= 0x1484;
    c.g[A0][0] |= 0x74a0;
    c.mult(0, A1, V0, A1);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x1480), c.g[V0][0]);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[A1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.ldl(2, (c.g[T3][0] as u32).wrapping_add(0x7));
    c.ldr(2, c.g[T3][0] as u32);
    c.ldl(3, (c.g[T3][0] as u32).wrapping_add(0xf));
    c.ldr(3, (c.g[T3][0] as u32).wrapping_add(0x8));
    c.g[A2][0] = Addr(c.g[T3][0] as u32).off(0x10).lw(c);
    c.sdl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
    c.sdr(2, c.g[A1][0] as u32);
    c.sdl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
    c.sdr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
    c.sw((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A2][0]);
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V0][0] = c.g[A1][0];
    c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    abi::ret(c);
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn ensure_view_matrices_matches() {
        check(
            ssx3_ee::s13::f_003956b0,
            ps2_graphics_man_ensure_view_matrices,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_2222 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                // Skip the real (heavy) callee on most seeds; take it otherwise.
                c.bus.write32(
                    DATA + 0x6b90,
                    if seed % 4 == 0 { 0 } else { r.next_u32() | 1 },
                );
            },
        );
    }

    #[test]
    fn draw_model_simple_matches() {
        covered(
            ssx3_ee::s12::f_00369cb0,
            graphics_man_draw_model_simple,
            200,
            &|c, seed| {
                machine(c, seed, 0xabcd, 1);
                let skin = DATA + 0x4000;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.bus.write32(DATA + 0x10d8, skin);
                let delta = (seed as i16) & !3;
                c.bus.write16(skin + 0x2f8, delta as u16);
                c.bus.write32(skin + 0x2fc, FAKE);
                c.bus.write32(DATA + 0x2000 + 0x94, seed | 1);
            },
        );
    }

    #[test]
    fn get_world_to_clip_matrix_matches() {
        check(
            ssx3_ee::s13::f_003953b8,
            ps2_graphics_man_get_world_to_clip_matrix,
            200,
            &|c, seed| {
                let mut r = Rng(0x3333_4444 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.bus.write32(
                    DATA + 0x1000 + 0x6b90,
                    if seed % 4 == 0 { 0 } else { r.next_u32() | 1 },
                );
                for i in 0..16 {
                    c.bus.write32(DATA + 0x1000 + 0x58a0 + i * 4, r.next_u32());
                }
                for i in 0..16 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn spr_stream_upload_matches() {
        check(
            ssx3_ee::s12::f_0038f708,
            spr_stream_upload,
            200,
            &|c, seed| {
                let mut r = Rng(0x5555_6666 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                c.g[A3][0] = ((r.next_u32() as u64) << 32) | (r.next_u32() as u64 | 1);
                c.bus.write32(GAME_GP + 0x2a94, DATA + 0x8000);
            },
        );
    }

    #[test]
    fn spr_stream_end_block_aligned_matches() {
        check(
            ssx3_ee::s12::f_0038f668,
            spr_stream_end_block_aligned,
            200,
            &|c, seed| {
                let mut r = Rng(0x7777_8888 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | (r.next_u32() | 1) as u64;
                let idx = r.next_u32() % 8;
                c.bus.write32(DATA + 0xc, idx);
                // The slot the stream index selects.
                c.bus.write32(DATA + idx * 4 + 0x14, r.next_u32());
                for i in 0..4 {
                    c.bus.write32(DATA + 0x20 + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn spr_stream_begin_block_matches() {
        check(
            ssx3_ee::s12::f_0038f460,
            spr_stream_begin_block,
            200,
            &|c, seed| {
                let mut r = Rng(0x9999_aaaa ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    (r.next_u32() & 0x7fff_ffff) as u64
                } else {
                    0xffff_ffff_0000_0000 | r.next_u32() as u64
                };
                c.g[A3][0] = seed as u64;
                c.bus.write32(DATA + 0x4, 1 + r.next_u32() % 8);
                c.bus.write32(DATA + 0x8, r.next_u32() % 8);
                c.bus.write32(DATA + 0x10, r.next_u32() % 8);
            },
        );
    }

    #[test]
    fn spr_stream_begin_block_upload_matches() {
        check(
            ssx3_ee::s12::f_0038f4f8,
            spr_stream_begin_block_upload,
            200,
            &|c, seed| {
                let mut r = Rng(0xbbbb_cccc ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (r.next_u32() | 1) as u64;
                c.g[A3][0] = seed as u64;
                c.bus.write32(DATA + 0x4, 1 + r.next_u32() % 8);
                c.bus.write32(DATA + 0x8, r.next_u32() % 8);
            },
        );
    }

    #[test]
    fn spr_stream_end_block_matches() {
        check(
            ssx3_ee::s12::f_0038f598,
            spr_stream_end_block,
            300,
            &|c, seed| {
                let mut r = Rng(0xdddd_eeee ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    ((r.next_u32() as u64) << 32) | (r.next_u32() | 1) as u64
                } else {
                    0
                };
                c.g[A2][0] = seed as u64;
                c.bus.write32(DATA + 0x4, 1 + r.next_u32() % 8);
                c.bus.write32(DATA + 0xc, r.next_u32() % 8);
                for i in 0..8 {
                    c.bus.write32(DATA + (i % 8) * 4 + 0x20, r.next_u32());
                }
            },
        );
    }

    /// Build one `DeferMan_FindBucket` scenario; the bucket slot is
    /// `DATA + 0x678a0 + pass*4`, the chain link lives at node+0x1c.
    fn find_bucket_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1122_3344 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        let pass = (seed % 4) as u32;
        let head_loc = DATA + 0x678a0 + pass * 4;
        let n0 = DATA + 0x1000;
        let n1 = DATA + 0x1200;
        let hash = r.next_u32() & 0x7fff_ffff;
        let k1 = r.next_u32() & 0x7fff_ffff;
        let k2 = r.next_u32() & 0x7fff_ffff;
        c.g[A1][0] = hash as u64;
        c.g[A2][0] = k1 as u64;
        c.g[A3][0] = pass as u64;
        c.g[T0][0] = k2 as u64;
        match seed % 6 {
            0 => {} // empty slot (head remains 0)
            1 => {
                c.bus.write32(head_loc, n0);
                c.bus.write32(n0, hash);
                c.bus.write32(n0 + 4, k1 ^ 1);
                c.bus.write32(n0 + 8, k2);
                c.bus.write32(n0 + 0x14, 0);
                c.bus.write32(n0 + 0x1c, 0);
            }
            2 => {
                c.bus.write32(head_loc, n0);
                c.bus.write32(n0, hash);
                c.bus.write32(n0 + 4, k1);
                c.bus.write32(n0 + 8, k2 ^ 1);
                c.bus.write32(n0 + 0x14, 0);
                c.bus.write32(n0 + 0x1c, 0);
            }
            3 => {
                c.bus.write32(head_loc, n0);
                c.bus.write32(n0, hash ^ 1);
                c.bus.write32(n0 + 4, k1);
                c.bus.write32(n0 + 8, k2);
                c.bus.write32(n0 + 0x14, 0);
                c.bus.write32(n0 + 0x1c, 0);
            }
            4 => {
                c.bus.write32(head_loc, n0);
                c.bus.write32(n0, hash);
                c.bus.write32(n0 + 4, k1);
                c.bus.write32(n0 + 8, k2);
                c.bus.write32(n0 + 0x14, 0);
                c.bus.write32(n0 + 0x1c, 0);
            }
            _ => {
                // n1 does not match; its link sends the walk to n0, which
                // does — so the found node is relinked to the head.
                c.bus.write32(head_loc, n1);
                c.bus.write32(n1, hash ^ 2);
                c.bus.write32(n1 + 4, k1 ^ 2);
                c.bus.write32(n1 + 8, k2 ^ 2);
                c.bus.write32(n1 + 0x14, 0);
                c.bus.write32(n1 + 0x1c, n0);
                c.bus.write32(n0, hash);
                c.bus.write32(n0 + 4, k1);
                c.bus.write32(n0 + 8, k2);
                c.bus.write32(n0 + 0x14, 0);
                c.bus.write32(n0 + 0x1c, 0);
            }
        }
    }

    #[test]
    fn defer_man_find_bucket_matches() {
        check(
            ssx3_ee::s13::f_00395000,
            defer_man_find_bucket,
            600,
            &find_bucket_setup,
        );
    }

    /// Write a four-word node plus its chain link.
    fn write_node(c: &mut Cpu, at: u32, words: [u32; 4], next: u32) {
        for i in 0..4 {
            c.bus.write32(at + i * 4, words[i as usize]);
        }
        c.bus.write32(at + 0x14, next);
    }

    /// Build one `DeferMan_InternState` scenario; the bucket slot is
    /// `DATA + 0x674a0 + pass*4`, the chain link lives at node+0x14, and the
    /// four state words are compared against the node's first four words.
    fn intern_state_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x5566_7788 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        let state = DATA + 0x2000;
        c.g[A1][0] = state as u64;
        let pass = (seed % 4) as u32;
        c.g[A2][0] = pass as u64;
        let head_loc = DATA + 0x674a0 + pass * 4;
        let n0 = DATA + 0x3000;
        let n1 = DATA + 0x3100;
        let w: [u32; 4] = [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()];
        for i in 0..4 {
            c.bus.write32(state + i * 4, w[i as usize]);
        }
        match seed % 4 {
            0 => {} // empty slot -> insert
            1 => {
                c.bus.write32(head_loc, n0);
                write_node(c, n0, [w[0] ^ 1, w[1], w[2], w[3]], 0);
            }
            2 => {
                c.bus.write32(head_loc, n0);
                write_node(c, n0, w, 0); // match, and n0 is the head -> early return
            }
            _ => {
                // n1 does not match, n0 does: relink n0 to the head.
                c.bus.write32(head_loc, n1);
                write_node(c, n1, [w[0] ^ 2, w[1] ^ 2, w[2] ^ 2, w[3] ^ 2], n0);
                write_node(c, n0, w, 0);
            }
        }
    }

    #[test]
    fn defer_man_intern_state_matches() {
        check(
            ssx3_ee::s13::f_00394ed0,
            defer_man_intern_state,
            600,
            &intern_state_setup,
        );
    }
}
