//! Staging module oc1475 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1475`.

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
    0x0038_07a0,
    ps2_graphics_man_draw_dynamic_emitter,
    "PS2GraphicsMan_DrawDynamicEmitter",
)];

mod oc1475_callee {
    use super::{ee, Callee};

    pub const UPDATE_VIEW_MATRICES: Callee = Callee {
        addr: 0x0037_d968,
        f: ee::s12::PS2GraphicsMan_UpdateViewMatrices,
    };
    pub const BEGIN_BLOCK: Callee = Callee {
        addr: 0x0038_f460,
        f: ee::s12::SprStream_BeginBlock,
    };
    pub const END_BLOCK_ALIGNED: Callee = Callee {
        addr: 0x0038_f668,
        f: ee::s12::SprStream_EndBlockAligned,
    };
}

/// Low 32 bits of register `r`.
#[inline(always)]
fn dyn_em_w(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// `off(r)` as an address.
#[inline(always)]
fn dyn_em_a(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// `off(sp)`.
#[inline(always)]
fn dyn_em_sp(c: &Cpu, off: u32) -> u32 {
    dyn_em_a(c, SP, off)
}

/// CopyQuads loop of the generated body: `lq v0,(src); sq v0,(dst)` with both
/// pointers stepping 16 until `dst == end`; the back edge checks events at
/// `head`.
fn dyn_em_copy(c: &mut Cpu, src: usize, dst: usize, end: usize, head: u32) {
    loop {
        c.cycles += 7;
        let s = dyn_em_w(c, src);
        c.lq(V0, s);
        let d = dyn_em_w(c, dst);
        c.sq(V0, d);
        c.g[dst][0] = sx(dyn_em_w(c, dst).wrapping_add(0x10));
        let more = c.g[dst][0] != c.g[end][0];
        c.g[src][0] = sx(dyn_em_w(c, src).wrapping_add(0x10));
        if more {
            if event_due(c) {
                deliver_event_here(c, head);
            }
            continue;
        }
        break;
    }
}

/// `void PS2GraphicsMan::DrawDynamicEmitter(gm, emitter)` (0x3807a0): ring
/// emitters drawn in batches of 64 sub-emitters. Each batch opens a
/// scratchpad block, writes the emitter descriptor (21 quads), a header
/// (seg, step, fstep, time), the positions and velocities of the ring slice
/// (with wrap-around and padding to 64) and a VIF/GIF tail, then closes the
/// block and submits a draw-state entry to the defer manager.
pub fn ps2_graphics_man_draw_dynamic_emitter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xc0);
        c.g[V0][0] = c.lw(dyn_em_a(c, GP, 0x140c));
        frame.save(c, S1, 0xa0);
        frame.save(c, S3, 0x80);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0xb0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x90);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save(c, S7, 0x40);
        frame.save(c, FP, 0x30);
        let disabled = c.g[V0][0] != 0;
        let a = frame.at(c, 0x20);
        c.sd(a, c.g[RA][0]);
        if !disabled {
            dyn_em_body(c)?;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// Everything between prologue and epilogue (emitters enabled).
fn dyn_em_body(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 14;
    c.g[A1][0] = c.lw(dyn_em_a(c, S3, 0xe84));
    c.g[V1][0] = 0xffff_ffff_ffff_fc3f;
    c.g[A2][0] = 0xffff_ffff_c000_0000;
    c.g[V0][0] = c.lw(dyn_em_w(c, A1));
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] |= 0x100;
    c.sw(dyn_em_w(c, A1), c.g[V0][0]);
    c.g[A0][0] = c.lw(dyn_em_a(c, S3, 0xe84));
    c.g[V0][0] = c.lw(dyn_em_a(c, A0, 0xc));
    c.g[V0][0] &= c.g[A2][0];
    c.sw(dyn_em_a(c, A0, 0xc), c.g[V0][0]);
    c.g[V1][0] = c.lw(dyn_em_a(c, S3, 0x6b90));
    if c.g[V1][0] != 0 {
        c.g[V1][0] = c.lw(dyn_em_a(c, S1, 0x178));
    } else {
        c.cycles += 3;
        c.g[A0][0] = c.g[S3][0];
        oc1475_callee::UPDATE_VIEW_MATRICES.call(c, 0x0038_081c)?;
        c.g[V1][0] = c.lw(dyn_em_a(c, S1, 0x178));
    }

    // size = numSubEmitters; step = NumParticles / size; batches = size/64+1.
    c.cycles += 9;
    c.g[A0][0] = u64::MAX;
    c.g[A1][0] = c.lw(dyn_em_a(c, S1, 0x20));
    c.g[S4][0] = 0;
    c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.g[V0][0] = sx(dyn_em_w(c, V1).wrapping_add(0x3f));
    c.div(0, A1, V1);
    if c.g[A0][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }

    c.cycles += 9;
    c.g[V0][0] = sx(((dyn_em_w(c, V0)) as i32 >> 6) as u32);
    c.g[S7][0] = sx(dyn_em_w(c, V0).wrapping_add(1));
    c.g[S0][0] = 0;
    c.g[A1][0] = c.lo[0];
    c.mtc1(A1, 0);
    c.cvt_s_w(0, 0);
    c.sw(dyn_em_sp(c, 4), c.g[A1][0]);
    let no_batches = (c.g[S7][0] as i64) <= 0;
    c.swc1(0, dyn_em_sp(c, 8));
    if !no_batches {
        c.cycles += 10;
        c.g[V0][0] = 0;
        c.g[V1][0] = 0x1400_0000;
        c.g[V0][0] = sx(dyn_em_w(c, V0).wrapping_add(0x510));
        c.g[FP][0] = 0x40;
        c.g[V0][0] = sx(dyn_em_w(c, V0) >> 3);
        c.g[S2][0] = sx(dyn_em_sp(c, 0x10));
        c.g[V0][0] |= c.g[V1][0];
        c.g[V0][0] <<= 32;
        c.sd(dyn_em_sp(c, 0x18), c.g[V0][0]);
        loop {
            dyn_em_batch(c)?;
            // loop tail
            c.cycles += 6;
            c.g[V0][0] = c.lw(dyn_em_a(c, S3, 0x5a00));
            c.g[S0][0] = c.g[S6][0];
            c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[S7][0] as i64)) as u64;
            c.g[V0][0] = sx(dyn_em_w(c, V0).wrapping_add(0x10));
            let more = c.g[V1][0] != 0;
            c.sw(dyn_em_a(c, S3, 0x5a00), c.g[V0][0]);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0038_0890);
                }
                continue;
            }
            break;
        }
    }

    // Clear the draw-state bit groups again.
    c.cycles += 10;
    c.g[A0][0] = c.lw(dyn_em_a(c, S3, 0xe84));
    c.g[V1][0] = 0xffff_ffff_ffff_fc3f;
    c.g[A1][0] = 0xffff_ffff_c000_0000;
    c.g[V0][0] = c.lw(dyn_em_w(c, A0));
    c.g[V0][0] &= c.g[V1][0];
    c.sw(dyn_em_w(c, A0), c.g[V0][0]);
    c.g[V1][0] = c.lw(dyn_em_a(c, S3, 0xe84));
    c.g[V0][0] = c.lw(dyn_em_a(c, V1, 0xc));
    c.g[V0][0] &= c.g[A1][0];
    c.sw(dyn_em_a(c, V1, 0xc), c.g[V0][0]);
    Ok(())
}

/// One batch of the loop (0x380890..0x380c88 before the loop tail).
fn dyn_em_batch(c: &mut Cpu) -> crate::call::Flow {
    // Open the block and write the packet header.
    c.cycles += 39;
    c.g[S5][0] = c.lw(dyn_em_a(c, S3, 0x5a00));
    c.g[A2][0] = u64::MAX;
    c.g[A0][0] = c.lw(dyn_em_a(c, GP, 0x1384));
    c.g[A3][0] = 0;
    c.g[A1][0] = c.g[S5][0];
    c.g[S6][0] = sx(dyn_em_w(c, S0).wrapping_add(1));
    oc1475_callee::BEGIN_BLOCK.call(c, 0x0038_08ac)?;
    c.sw(dyn_em_sp(c, 0x10), c.g[V0][0]);
    c.g[V1][0] = 0x1000_0000;
    c.g[V1][0] |= 0x9a;
    c.sd(dyn_em_w(c, V0), c.g[V1][0]);
    c.g[A0][0] = 0xd931;
    c.g[A0][0] <<= 47;
    c.g[V0][0] = sx(dyn_em_w(c, S0) << 6);
    c.g[A1][0] = sx(dyn_em_a(c, S1, 0x20));
    c.g[V1][0] = c.lw(dyn_em_sp(c, 0x10));
    c.mtc1(V0, 2);
    c.cvt_s_w(2, 2);
    c.sd(dyn_em_a(c, V1, 8), 0);
    c.g[V0][0] = c.lw(dyn_em_sp(c, 0x10));
    c.sd(dyn_em_a(c, V0, 0x10), 0);
    c.g[V1][0] = c.lw(dyn_em_sp(c, 0x10));
    c.sd(dyn_em_a(c, V1, 0x18), c.g[A0][0]);
    c.lwc1(0, dyn_em_sp(c, 8));
    c.lwc1(1, dyn_em_a(c, S1, 0x2c));
    c.mul_s(0, 0, 2);
    c.g[V0][0] = c.lw(dyn_em_a(c, S1, 0x178));
    c.g[A0][0] = c.lw(dyn_em_sp(c, 0x10));
    c.g[V0][0] = sx(dyn_em_w(c, V0).wrapping_sub(dyn_em_w(c, S4)));
    c.mul_s(0, 0, 1);
    c.g[V1][0] = ((c.g[V0][0] as i64) < 65) as u64;
    c.g[A2][0] = sx(dyn_em_a(c, A0, 0x170));
    if c.g[V1][0] == 0 {
        c.g[V0][0] = c.g[FP][0];
    }
    c.g[A0][0] = sx(dyn_em_a(c, A0, 0x20));
    c.sw(dyn_em_sp(c, 0), c.g[V0][0]);
    c.g[V1][0] = c.g[A0][0];
    c.swc1(0, dyn_em_sp(c, 0xc));
    let skip = c.g[V1][0] == c.g[A2][0];
    c.sw(dyn_em_sp(c, 0x10), c.g[A0][0]);
    if !skip {
        c.cycles += 1;
        dyn_em_copy(c, A1, V1, A2, 0x0038_0930);
    }

    // The emitter descriptor is in; now the header quad (from the stack).
    c.cycles += 5;
    c.sw(dyn_em_w(c, S2), c.g[V1][0]);
    c.g[V1][0] = c.lw(dyn_em_sp(c, 0x10));
    c.g[A1][0] = sx(dyn_em_w(c, V1).wrapping_add(0x10));
    let skip = c.g[V1][0] == c.g[A1][0];
    c.g[A0][0] = c.g[SP][0];
    if !skip {
        dyn_em_copy(c, A0, V1, A1, 0x0038_0960);
    }

    // Ring bookkeeping.
    c.cycles += 8;
    c.g[V0][0] = c.lw(dyn_em_a(c, S1, 0x17c));
    c.g[A1][0] = u64::MAX;
    c.g[A3][0] = c.lw(dyn_em_a(c, S1, 0x178));
    c.g[T1][0] = sx(dyn_em_w(c, V0).wrapping_add(1));
    c.sw(dyn_em_w(c, S2), c.g[V1][0]);
    c.g[V0][0] = sx(dyn_em_w(c, S4).wrapping_add(dyn_em_w(c, T1)));

    c.cycles += 16;
    c.div(0, V0, A3);
    c.g[V1][0] = c.lw(dyn_em_sp(c, 0));
    c.g[S4][0] = sx(dyn_em_w(c, S4).wrapping_add(dyn_em_w(c, V1)));
    c.g[A2][0] = ((c.g[S4][0] as i64) < (c.g[A3][0] as i64)) as u64;
    c.g[T3][0] = c.hi[0];
    c.g[V1][0] = sx(dyn_em_w(c, T3).wrapping_add(dyn_em_w(c, V1)));
    c.g[A0][0] = sx(dyn_em_w(c, A3).wrapping_sub(dyn_em_w(c, T3)));
    c.g[T2][0] = sx(dyn_em_w(c, V1).wrapping_sub(dyn_em_w(c, A3)));
    c.g[T0][0] = sx(dyn_em_w(c, V1).wrapping_sub(dyn_em_w(c, T3)));
    c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[T2][0] as i64)) as u64;
    c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[T0][0] as i64)) as u64;
    if c.g[A1][0] == 0 {
        c.g[T2][0] = 0;
    }
    if c.g[V1][0] != 0 {
        c.g[T0][0] = c.g[A0][0];
    }
    c.g[V0][0] = sx(dyn_em_w(c, FP).wrapping_sub(dyn_em_w(c, T2)));
    let last_slot = c.g[A2][0] == 0;
    c.g[T4][0] = sx(dyn_em_w(c, V0).wrapping_sub(dyn_em_w(c, T0)));
    if !last_slot {
        // done < ringSize: the last quad is Wrap(done, ...).
        c.cycles += 3;
        c.g[V0][0] = sx(dyn_em_w(c, S4).wrapping_add(dyn_em_w(c, T1)));
        c.cycles += 9;
        c.div(0, V0, A3);
        c.g[A0][0] = c.lw(dyn_em_a(c, S1, 0x1a0));
        c.g[V0][0] = c.lw(dyn_em_a(c, S1, 0x1a4));
        c.g[T1][0] = c.g[A0][0];
        c.g[V1][0] = c.hi[0];
        c.g[V1][0] = sx(dyn_em_w(c, V1) << 4);
        c.g[T5][0] = sx(dyn_em_w(c, V0).wrapping_add(dyn_em_w(c, V1)));
        c.g[A0][0] = sx(dyn_em_w(c, A0).wrapping_add(dyn_em_w(c, V1)));
    } else {
        // done >= ringSize: Wrap(done - 1, ...).
        c.cycles += 3;
        c.g[V0][0] = sx(dyn_em_w(c, S4).wrapping_sub(1));
        c.cycles += 9;
        c.g[V0][0] = sx(dyn_em_w(c, V0).wrapping_add(dyn_em_w(c, T1)));
        c.g[A0][0] = c.lw(dyn_em_a(c, S1, 0x1a4));
        c.div(0, V0, A3);
        c.g[A1][0] = c.lw(dyn_em_a(c, S1, 0x1a0));
        c.g[T1][0] = c.g[A1][0];
        c.g[V1][0] = c.hi[0];
        c.g[V1][0] = sx(dyn_em_w(c, V1) << 4);
        c.g[T5][0] = sx(dyn_em_w(c, A0).wrapping_add(dyn_em_w(c, V1)));
        c.g[A0][0] = sx(dyn_em_w(c, A1).wrapping_add(dyn_em_w(c, V1)));
    }

    // positions[firstIndex .. firstIndex + first]
    c.cycles += 9;
    c.g[V1][0] = sx(dyn_em_w(c, T3) << 4);
    c.g[V0][0] = sx(dyn_em_w(c, T0) << 4);
    c.g[A2][0] = c.lw(dyn_em_sp(c, 0x10));
    c.g[T0][0] = c.g[V1][0];
    c.g[V1][0] = sx(dyn_em_w(c, V1).wrapping_add(dyn_em_w(c, T1)));
    c.g[T3][0] = c.g[V0][0];
    c.g[A3][0] = sx(dyn_em_w(c, A2).wrapping_add(dyn_em_w(c, V0)));
    let skip = c.g[A2][0] == c.g[A3][0];
    c.g[T1][0] = sx(dyn_em_w(c, T4) << 4);
    if !skip {
        dyn_em_copy(c, V1, A2, A3, 0x0038_0a60);
    }
    // positions[0 .. over]
    c.cycles += 2;
    let no_over = (c.g[T2][0] as i64) <= 0;
    c.sw(dyn_em_w(c, S2), c.g[A2][0]);
    if !no_over {
        c.cycles += 5;
        c.g[V1][0] = c.lw(dyn_em_sp(c, 0x10));
        c.g[V0][0] = sx(dyn_em_w(c, T2) << 4);
        c.g[A3][0] = sx(dyn_em_w(c, V1).wrapping_add(dyn_em_w(c, V0)));
        let skip = c.g[V1][0] == c.g[A3][0];
        c.g[A2][0] = c.lw(dyn_em_a(c, S1, 0x1a0));
        if !skip {
            dyn_em_copy(c, A2, V1, A3, 0x0038_0a98);
        }
        c.cycles += 1;
        c.sw(dyn_em_w(c, S2), c.g[V1][0]);
    }
    // last position quad
    c.cycles += 4;
    c.g[V1][0] = c.lw(dyn_em_sp(c, 0x10));
    c.g[A1][0] = sx(dyn_em_w(c, V1).wrapping_add(0x10));
    if c.g[V1][0] == c.g[A1][0] {
        c.sw(dyn_em_w(c, S2), c.g[V1][0]);
    } else {
        dyn_em_copy(c, A0, V1, A1, 0x0038_0ac8);
        c.cycles += 1;
        c.sw(dyn_em_w(c, S2), c.g[V1][0]);
    }
    // velocities[firstIndex .. firstIndex + first], after the padding
    c.cycles += 8;
    c.g[V1][0] = c.lw(dyn_em_a(c, S1, 0x1a4));
    c.g[V0][0] = c.lw(dyn_em_sp(c, 0x10));
    c.g[V1][0] = sx(dyn_em_w(c, T0).wrapping_add(dyn_em_w(c, V1)));
    c.g[V0][0] = sx(dyn_em_w(c, V0).wrapping_add(dyn_em_w(c, T1)));
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx(dyn_em_w(c, A0).wrapping_add(dyn_em_w(c, T3)));
    let skip = c.g[A0][0] == c.g[A1][0];
    c.sw(dyn_em_sp(c, 0x10), c.g[V0][0]);
    if !skip {
        dyn_em_copy(c, V1, A0, A1, 0x0038_0b08);
    }
    // velocities[0 .. over]
    c.cycles += 2;
    let no_over = (c.g[T2][0] as i64) <= 0;
    c.sw(dyn_em_w(c, S2), c.g[A0][0]);
    if !no_over {
        c.cycles += 5;
        c.g[A0][0] = c.lw(dyn_em_sp(c, 0x10));
        c.g[V0][0] = sx(dyn_em_w(c, T2) << 4);
        c.g[V1][0] = sx(dyn_em_w(c, A0).wrapping_add(dyn_em_w(c, V0)));
        let skip = c.g[A0][0] == c.g[V1][0];
        c.g[A1][0] = c.lw(dyn_em_a(c, S1, 0x1a4));
        if !skip {
            dyn_em_copy(c, A1, A0, V1, 0x0038_0b40);
        }
        c.cycles += 1;
        c.sw(dyn_em_w(c, S2), c.g[A0][0]);
    }
    // last velocity quad
    c.cycles += 4;
    c.g[V1][0] = c.lw(dyn_em_sp(c, 0x10));
    c.g[A1][0] = sx(dyn_em_w(c, V1).wrapping_add(0x10));
    let skip = c.g[V1][0] == c.g[A1][0];
    c.g[A0][0] = c.g[T5][0];
    if !skip {
        dyn_em_copy(c, A0, V1, A1, 0x0038_0b70);
    }

    // Tail packet, close the block and submit the draw state.
    c.cycles += 24;
    c.sw(dyn_em_w(c, S2), c.g[V1][0]);
    c.g[A0][0] = 0x8800;
    c.g[A0][0] <<= 45;
    c.g[V1][0] = c.ld(dyn_em_sp(c, 0x18));
    c.g[A2][0] = 4;
    c.g[V0][0] = c.lw(dyn_em_sp(c, 0x10));
    c.g[V0][0] = sx(dyn_em_w(c, V0).wrapping_add(dyn_em_w(c, T1)));
    c.sw(dyn_em_sp(c, 0x10), c.g[V0][0]);
    c.sd(dyn_em_w(c, V0), c.g[V1][0]);
    c.g[V1][0] = c.lw(dyn_em_sp(c, 0x10));
    c.sd(dyn_em_a(c, V1, 8), c.g[A0][0]);
    c.g[V0][0] = c.lw(dyn_em_sp(c, 0x10));
    c.g[A0][0] = c.lw(dyn_em_a(c, GP, 0x1384));
    c.g[V0][0] = sx(dyn_em_w(c, V0).wrapping_add(0x10));
    c.g[A1][0] = c.g[V0][0];
    c.sw(dyn_em_sp(c, 0x10), c.g[V0][0]);
    oc1475_callee::END_BLOCK_ALIGNED.call(c, 0x0038_0bd0)?;
    c.g[T1][0] = c.g[V0][0];
    c.g[A3][0] = c.lw(dyn_em_a(c, S3, 0x18f0));
    c.sw(dyn_em_a(c, S3, 0x5a00), c.g[T1][0]);
    c.g[V0][0] = c.lw(dyn_em_w(c, A3));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 2600) as u64;
    let full = c.g[V0][0] == 0;
    c.g[T0][0] = c.lw(dyn_em_a(c, S3, 0xe84));
    if !full {
        c.cycles += 33;
        c.g[A1][0] = 0x60000;
        c.g[A0][0] = c.lw(dyn_em_a(c, T0, 8));
        c.g[A1][0] |= 0x8000;
        c.g[V1][0] = 0xffff_ffff_ffff_ffe0;
        c.g[A1][0] = sx(dyn_em_w(c, A3).wrapping_add(dyn_em_w(c, A1)));
        c.g[A0][0] &= c.g[V1][0];
        c.g[V0][0] = c.lw(dyn_em_a(c, A1, 0x1cc4));
        c.g[A2][0] = sx(dyn_em_a(c, A3, 0x80));
        c.g[V1][0] = 0x3000_0000;
        c.g[V0][0] &= 0x1f;
        c.g[A2][0] |= c.g[V1][0];
        c.g[A0][0] |= c.g[V0][0];
        c.sw(dyn_em_a(c, T0, 8), c.g[A0][0]);
        c.g[V1][0] = c.lw(dyn_em_w(c, A3));
        c.g[A0][0] = c.lw(dyn_em_a(c, A1, 0x1cc0));
        c.g[V0][0] = sx(dyn_em_w(c, V1) << 7);
        c.g[V1][0] = sx(dyn_em_w(c, V1).wrapping_add(1));
        c.g[V0][0] = sx(dyn_em_w(c, V0).wrapping_add(dyn_em_w(c, A2)));
        c.sw(dyn_em_w(c, A3), c.g[V1][0]);
        c.ldl(5, dyn_em_a(c, T0, 7));
        c.ldr(5, dyn_em_w(c, T0));
        c.ldl(6, dyn_em_a(c, T0, 0xf));
        c.ldr(6, dyn_em_a(c, T0, 8));
        c.g[A3][0] = c.lw(dyn_em_a(c, T0, 0x10));
        c.sdl(5, dyn_em_a(c, V0, 7));
        c.sdr(5, dyn_em_w(c, V0));
        c.sdl(6, dyn_em_a(c, V0, 0xf));
        c.sdr(6, dyn_em_a(c, V0, 8));
        c.sw(dyn_em_a(c, V0, 0x10), c.g[A3][0]);
        c.sw(dyn_em_a(c, V0, 0x14), c.g[S5][0]);
        c.sw(dyn_em_a(c, V0, 0x18), c.g[T1][0]);
        c.sh(dyn_em_a(c, V0, 0x1c), c.g[A0][0]);
        c.sh(dyn_em_a(c, V0, 0x1e), 0);
    }
    Ok(())
}

#[cfg(test)]
pub(crate) const OC1475_CTRL: u32 = crate::difftest::DATA + 0xf0000;
#[cfg(test)]
pub(crate) const OC1475_MAGIC: u32 = 0x1475_0001;
#[cfg(test)]
pub(crate) const OC1475_BUF: u32 = crate::difftest::DATA + 0x230000;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered};
    use crate::difftest::{fill, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    static OC1475_PREV_BEGIN: AtomicUsize = AtomicUsize::new(0);
    static OC1475_PREV_END: AtomicUsize = AtomicUsize::new(0);
    static OC1475_PREV_VIEW: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc1475_fake_begin(c: &mut Cpu) {
        if c.bus.read32(OC1475_CTRL) != OC1475_MAGIC {
            return fakes::fall_back(c, &OC1475_PREV_BEGIN, ee::s12::f_0038f460__generated);
        }
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake_call(c, 0x0038_f460);
        c.g[V0][0] = (OC1475_BUF + (n % 16) * 0x1000) as u64;
    }

    pub(crate) fn oc1475_fake_end(c: &mut Cpu) {
        if c.bus.read32(OC1475_CTRL) != OC1475_MAGIC {
            return fakes::fall_back(c, &OC1475_PREV_END, ee::s12::f_0038f668__generated);
        }
        fakes::fake_call(c, 0x0038_f668);
    }

    pub(crate) fn oc1475_fake_view(c: &mut Cpu) {
        if c.bus.read32(OC1475_CTRL) != OC1475_MAGIC {
            return fakes::fall_back(c, &OC1475_PREV_VIEW, ee::s12::f_0037d968__generated);
        }
        fakes::fake_call(c, 0x0037_d968);
    }

    fn oc1475_install() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038F460),
            oc1475_fake_begin,
            &OC1475_PREV_BEGIN,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038F668),
            oc1475_fake_end,
            &OC1475_PREV_END,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0037D968),
            oc1475_fake_view,
            &OC1475_PREV_VIEW,
        );
    }

    /// `DrawDynamicEmitter`: disabled flag, matrices valid or not, empty
    /// (negative) rings, rings of 1..200 sub-emitters (single and several
    /// batches, wrap-around, padding), a full or open defer manager, and
    /// unwinding inside each of the three callees.
    #[test]
    fn ps2_graphics_man_draw_dynamic_emitter_matches() {
        oc1475_install();
        const SIZES: [i32; 12] = [1, 5, 63, 64, 65, 127, 128, 129, 200, -100, 70, 33];
        covered(
            ee::s12::f_003807a0__generated,
            ps2_graphics_man_draw_dynamic_emitter,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1475, 7);
                c.bus.write32(OC1475_CTRL, OC1475_MAGIC);
                let gm = DATA;
                let e = DATA + 0x20_0000;
                let state = DATA + 0x8000;
                let defer = DATA + 0x1_0000;
                let gp = c.g[GP][0] as u32;
                c.g[A0][0] = gm as u64;
                c.g[A1][0] = e as u64;
                let toggle = if seed % 8 == 0 { 1 } else { 0 };
                c.bus.write32(gp + 0x140c, toggle);
                c.bus.write32(gp + 0x1384, r.next_u32());
                c.bus.write32(gm + 0xe84, state);
                fill(c, &mut r, state, 8);
                c.bus.write32(gm + 0x6b90, (seed % 3 != 0) as u32);
                c.bus.write32(gm + 0x5a00, r.next_u32());
                c.bus.write32(gm + 0x18f0, defer);
                let count = match seed % 4 {
                    0 => 2600 + r.next_u32() % 3,
                    1 => 2599,
                    _ => r.next_u32() % 20,
                };
                c.bus.write32(defer, count);
                fill(c, &mut r, defer + 0x6_8000 + 0x1cc0, 2);
                let size = SIZES[(seed / 2 % 12) as usize];
                fill(c, &mut r, e + 0x20, 0x170 / 4);
                c.bus.write32(e + 0x20, r.next_u32() % 10000);
                c.bus.write32(e + 0x2c, (1.0f32 + (seed % 5) as f32).to_bits());
                c.bus.write32(e + 0x178, size as u32);
                c.bus.write32(e + 0x17c, r.next_u32() % 300);
                c.bus.write32(e + 0x1a0, e + 0x1_0000);
                c.bus.write32(e + 0x1a4, e + 0x2_0000);
                fill(c, &mut r, e + 0x1_0000, 200 * 4);
                fill(c, &mut r, e + 0x2_0000, 200 * 4);
            },
        );
    }
}
