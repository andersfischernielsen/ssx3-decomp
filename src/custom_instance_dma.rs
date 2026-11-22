//! Relocation of the model/colour DMA pairs stored in a world instance.

use crate::call::Frame;
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x003a_c508,
    patch_instance_dma_chain,
    "cPS2ChunkResolve_PatchInstanceDmaChain",
)];

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `void* PatchInstanceDmaChain(resolver, Instance* inst, ModelDesc* model,
///                            DmaTag* chain)` (0x3ac508).
///
/// A normal batch is a pair of REF tags. Geometry offsets are relative to
/// `model->chainBase` (+0x24), while colours are relative to the instance's
/// chain base (+0x98). Patch the last colour qword with the first/subsequent
/// mesh MSCAL. Bit 31 on a geometry offset selects the original's extended
/// shared-reference paths. Return the address just after this group's RET.
pub fn patch_instance_dma_chain(c: &mut Cpu) {
    c.cycles += 28;
    c.g[V0][0] = 0;
    c.g[V1][0] = 0;
    c.g[V0][0] = sx(r(c, V0).wrapping_add(0x2270));
    c.g[V1][0] = sx(r(c, V1).wrapping_add(0x22c8));
    c.g[A0][0] = 0x0051_0000;
    let frame = Frame::open(c, 0x40);
    c.g[T9][0] = sx(r(c, V0) >> 3);
    c.g[T8][0] = sx(r(c, V1) >> 3);
    c.g[A0][0] = sx(r(c, A0).wrapping_sub(0x6c00));
    c.g[V1][0] = 0x1400_0000;
    frame.save(c, S0, 0x30);
    c.g[A0][0] <<= 32;
    frame.save(c, S1, 0x20);
    c.g[T5][0] = 0x7fff_0000;
    frame.save(c, S2, 0x10);
    c.g[V0][0] = 0x3000_0000;
    c.g[V0][0] |= 1;
    frame.save(c, S3, 0);
    c.g[T6][0] = c.g[T8][0] | c.g[V1][0];
    c.g[T7][0] = c.g[T9][0] | c.g[V1][0];
    c.g[S3][0] = c.g[A0][0] | c.g[V0][0];
    c.g[S2][0] = 0;
    c.g[T4][0] = 1;
    c.g[S1][0] = 6;
    c.g[S0][0] = 0xffff_ffff_8000_0000;
    c.g[T5][0] |= 0xffff;
    c.g[T0][0] = c.ld(r(c, A3));
    loop {
        c.cycles += 6;
        c.g[V1][0] = 3;
        c.g[V0][0] = c.g[T0][0] << 4;
        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
        c.g[V0][0] &= 7;
        if c.g[V0][0] == c.g[V1][0] {
            c.g[V1][0] = c.lw(r(c, A3).wrapping_add(4));
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & c.g[S0][0];
            c.g[T2][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & c.g[T5][0];
                c.g[T2][0] = 1;
                c.sw(r(c, A3).wrapping_add(4), c.g[V0][0]);
            }

            // First pair: shared geometry followed by this instance's colours.
            c.cycles += 20;
            c.g[V1][0] = c.lw(r(c, A3).wrapping_add(4));
            c.g[V0][0] = c.lw(r(c, A2).wrapping_add(0x24));
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
            c.sw(r(c, A3).wrapping_add(4), c.g[V0][0]);
            c.g[A3][0] = sx(r(c, A3).wrapping_add(0x10));
            c.g[V1][0] = c.lw(r(c, A3).wrapping_add(4));
            c.g[A0][0] = sx(r(c, A3).wrapping_add(4));
            c.g[V0][0] = c.lw(r(c, A1).wrapping_add(0x98));
            c.g[T0][0] = c.ld(r(c, A3));
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
            c.sw(r(c, A3).wrapping_add(4), c.g[V0][0]);
            c.g[V1][0] = sx(r(c, T0));
            c.g[T1][0] = c.g[V1][0] & 0xffff;
            c.g[A3][0] = sx(r(c, A3).wrapping_add(0x10));
            c.g[T0][0] = c.lw(r(c, A0));
            c.g[V0][0] = sx(r(c, T1).wrapping_sub(1));
            c.g[V0][0] = sx(r(c, V0) << 4);
            c.g[T0][0] = sx(r(c, T0).wrapping_add(r(c, V0)));
            if c.g[T4][0] != 0 {
                c.cycles += 2;
                c.sw(r(c, T0).wrapping_add(8), c.g[T7][0]);
            } else {
                c.cycles += 1;
                c.sw(r(c, T0).wrapping_add(8), c.g[T6][0]);
            }
            c.cycles += 2;
            if c.g[T2][0] == 0 {
                c.g[T4][0] = 0;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.ld(r(c, A3).wrapping_add(0x10));
                c.g[V0][0] <<= 4;
                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                c.g[V0][0] &= 7;
                let next_is_return = c.g[V0][0] == c.g[S1][0];
                c.g[A0][0] = sx(r(c, A3).wrapping_add(0x10));
                if next_is_return {
                    // One extra shared REF: splice in the common setup qword.
                    c.cycles += 11;
                    c.g[T0][0] = c.ld(r(c, A3));
                    c.g[A0][0] = c.lw(r(c, A2).wrapping_add(0x24));
                    c.g[V0][0] = sx(r(c, T0));
                    c.g[V1][0] = c.lw(r(c, A3).wrapping_add(4));
                    c.g[T1][0] = c.g[V0][0] & 0xffff;
                    c.g[V0][0] = sx(r(c, T1).wrapping_sub(1));
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
                    c.g[V0][0] = sx(r(c, V0) << 4);
                    c.g[T0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
                    if c.g[T4][0] != 0 {
                        c.cycles += 2;
                        c.sw(r(c, T0).wrapping_add(8), c.g[T7][0]);
                    } else {
                        c.cycles += 1;
                        c.sw(r(c, T0).wrapping_add(8), c.g[T6][0]);
                    }
                    c.cycles += 13;
                    c.g[V0][0] = 0x3000_0000;
                    c.g[A0][0] <<= 32;
                    c.g[V0][0] |= c.g[T1][0];
                    c.g[V1][0] = 0x6000_0000;
                    c.g[V0][0] |= c.g[A0][0];
                    c.sd(r(c, A3).wrapping_add(0x20), c.g[V1][0]);
                    c.sd(r(c, A3).wrapping_add(0x10), c.g[V0][0]);
                    c.sd(r(c, A3), c.g[S3][0]);
                    c.sd(r(c, A3).wrapping_add(8), 0);
                    c.sd(r(c, A3).wrapping_add(0x18), 0);
                    c.sd(r(c, A3).wrapping_add(0x28), 0);
                    c.g[A3][0] = sx(r(c, A3).wrapping_add(0x20));
                } else {
                    // Extended stream: another geometry/colour pair, then
                    // two shared normal/environment references.
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A3).wrapping_add(4));
                    c.g[V0][0] = c.g[V1][0] & c.g[S0][0];
                    let marked = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[V1][0] & c.g[T5][0];
                    if marked {
                        c.cycles += 2;
                        c.sw(r(c, A3).wrapping_add(4), c.g[V0][0]);
                        c.g[V1][0] = c.lw(r(c, A3).wrapping_add(4));
                    }
                    c.cycles += 36;
                    c.g[T3][0] = 0x1400_0000;
                    c.g[V0][0] = c.lw(r(c, A2).wrapping_add(0x24));
                    c.g[T2][0] = c.g[T8][0] | c.g[T3][0];
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.sw(r(c, A3).wrapping_add(4), c.g[V0][0]);
                    c.g[A3][0] = c.g[A0][0];
                    c.g[V1][0] = c.lw(r(c, A3).wrapping_add(4));
                    c.g[A0][0] = sx(r(c, A3).wrapping_add(4));
                    c.g[V0][0] = c.lw(r(c, A1).wrapping_add(0x98));
                    c.g[T0][0] = c.ld(r(c, A3));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.sw(r(c, A3).wrapping_add(4), c.g[V0][0]);
                    c.g[V1][0] = sx(r(c, T0));
                    c.g[T1][0] = c.g[V1][0] & 0xffff;
                    c.g[A3][0] = sx(r(c, A3).wrapping_add(0x10));
                    c.g[T0][0] = c.lw(r(c, A0));
                    c.g[V0][0] = sx(r(c, T1).wrapping_sub(1));
                    c.g[V0][0] = sx(r(c, V0) << 4);
                    c.g[A0][0] = sx(r(c, A3).wrapping_add(4));
                    c.g[T0][0] = sx(r(c, T0).wrapping_add(r(c, V0)));
                    c.sw(r(c, T0).wrapping_add(8), c.g[T2][0]);
                    c.g[V1][0] = c.lw(r(c, A3).wrapping_add(4));
                    c.g[V0][0] = c.lw(r(c, A2).wrapping_add(0x24));
                    c.g[T0][0] = c.ld(r(c, A3));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.sw(r(c, A3).wrapping_add(4), c.g[V0][0]);
                    c.g[V1][0] = sx(r(c, T0));
                    c.g[T1][0] = c.g[V1][0] & 0xffff;
                    c.g[A3][0] = sx(r(c, A3).wrapping_add(0x10));
                    c.g[T0][0] = c.lw(r(c, A0));
                    c.g[V0][0] = sx(r(c, T1).wrapping_sub(1));
                    c.g[V0][0] = sx(r(c, V0) << 4);
                    c.g[T0][0] = sx(r(c, T0).wrapping_add(r(c, V0)));
                    if c.g[T4][0] != 0 {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[T9][0] | c.g[T3][0];
                        c.sw(r(c, T0).wrapping_add(8), c.g[V0][0]);
                    } else {
                        c.cycles += 1;
                        c.sw(r(c, T0).wrapping_add(8), c.g[T2][0]);
                    }
                    c.cycles += 15;
                    c.g[V1][0] = c.lw(r(c, A3).wrapping_add(4));
                    c.g[A0][0] = sx(r(c, A3).wrapping_add(4));
                    c.g[V0][0] = c.lw(r(c, A2).wrapping_add(0x24));
                    c.g[T0][0] = c.ld(r(c, A3));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.sw(r(c, A3).wrapping_add(4), c.g[V0][0]);
                    c.g[V1][0] = sx(r(c, T0));
                    c.g[T1][0] = c.g[V1][0] & 0xffff;
                    c.g[A3][0] = sx(r(c, A3).wrapping_add(0x10));
                    c.g[T0][0] = c.lw(r(c, A0));
                    c.g[V0][0] = sx(r(c, T1).wrapping_sub(1));
                    c.g[V0][0] = sx(r(c, V0) << 4);
                    c.g[T0][0] = sx(r(c, T0).wrapping_add(r(c, V0)));
                    c.sw(r(c, T0).wrapping_add(8), c.g[T6][0]);
                }
                c.cycles += 2;
                c.g[T4][0] = 0;
            }
        } else {
            c.cycles += 2;
            if c.g[V0][0] == c.g[S1][0] {
                c.g[A3][0] = sx(r(c, A3).wrapping_add(0x10));
                c.cycles += 1;
                c.g[S2][0] = 1;
            } else {
                c.cycles += 2;
            }
        }
        c.cycles += 2;
        if c.g[S2][0] != 0 {
            c.cycles += 7;
            frame.restore(c, S0, 0x30);
            c.g[V0][0] = c.g[A3][0];
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore(c, S3, 0);
            frame.close(c);
            return;
        }
        c.g[T0][0] = c.ld(r(c, A3));
        if event_due(c) {
            deliver_event_here(c, 0x003a_c578);
        }
    }
}
