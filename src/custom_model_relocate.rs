//! Native model DMA reference relocation and shared setup insertion.

use crate::call::Frame;
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x003a_c048,
    patch_model_dma_chain,
    "cPS2ChunkResolve_PatchModelDmaChain",
)];

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// Relocate the REF pairs of one mesh against `model->chainBase` (+0x24).
/// The mesh descriptor supplies the relative first tag at +4; descriptor
/// flag bits 0x60000 or bit 31 of a REF offset enable shared setup insertion.
/// RET (tag kind 6) is the only terminator. Other non-REF kinds retain the
/// original's non-advancing loop, so malformed assets must be fixed upstream.
pub fn patch_model_dma_chain(c: &mut Cpu) {
    let mut reported_invalid_tag = false;
    c.cycles += 46;
    let frame = Frame::open(c, 0x50);
    c.g[V0][0] = 0x0051_0000;
    frame.save(c, S0, 0x40);
    c.g[V1][0] = 0x0051_0000;
    frame.save(c, S1, 0x30);
    c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x6bf0));
    frame.save(c, S2, 0x20);
    c.g[V1][0] = sx(r(c, V1).wrapping_sub(0x6be0));
    frame.save(c, S3, 0x10);
    c.g[T2][0] = c.g[A2][0];
    frame.save(c, S4, 0);
    c.g[T7][0] = c.g[V0][0] << 32;
    c.g[T6][0] = c.g[V1][0] << 32;
    c.g[A2][0] = 0;
    c.g[A3][0] = c.lw(r(c, A1));
    c.g[V1][0] = 0x0006_0000;
    c.g[V0][0] = 0x0051_0000;
    c.g[T0][0] = c.lw(r(c, A1).wrapping_add(4));
    c.g[A3][0] &= c.g[V1][0];
    c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x6c00));
    c.g[A0][0] = 0;
    c.g[A2][0] = sx(r(c, A2).wrapping_add(0x2270));
    c.g[T3][0] = 1;
    c.g[A2][0] = sx(r(c, A2) >> 3);
    c.g[V1][0] = 0x1400_0000;
    c.g[A0][0] = sx(r(c, A0).wrapping_add(0x22c8));
    c.g[A1][0] = c.lw(r(c, T2).wrapping_add(0x24));
    c.g[S2][0] = c.g[V0][0] << 32;
    c.g[V0][0] = 0x3000_0000;
    c.g[V0][0] |= 1;
    c.g[A0][0] = sx(r(c, A0) >> 3);
    c.g[S1][0] = c.g[A2][0] | c.g[V1][0];
    c.g[T1][0] = c.g[T3][0];
    c.g[T4][0] = 0x7fff_0000;
    c.g[A0][0] |= c.g[V1][0];
    c.g[T5][0] = c.g[T6][0] | c.g[V0][0];
    c.g[S0][0] = c.g[T7][0] | c.g[V0][0];
    c.g[T0][0] = sx(r(c, T0).wrapping_add(r(c, A1)));
    if c.g[A3][0] == 0 {
        c.g[T1][0] = 0;
    }
    c.g[T9][0] = 0;
    c.g[A2][0] = 6;
    c.g[T8][0] = 0xffff_ffff_8000_0000;
    c.g[T4][0] |= 0xffff;
    c.g[S4][0] = 3;
    c.g[S3][0] = 0x6000_0000;
    c.g[A1][0] = c.ld(r(c, T0));

    loop {
        c.cycles += 5;
        c.g[V0][0] = c.g[A1][0] << 4;
        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
        c.g[V0][0] &= 7;
        if c.g[V0][0] == c.g[S4][0] {
            c.g[V1][0] = c.lw(r(c, T0).wrapping_add(4));
            relocate_reference_pair(c);
        } else {
            c.cycles += 2;
            if c.g[V0][0] == c.g[A2][0] {
                c.g[T0][0] = sx(r(c, T0).wrapping_add(0x10));
                c.cycles += 1;
                c.g[T9][0] = 1;
            } else {
                c.cycles += 2;
                if !reported_invalid_tag && crate::custom_tracks::active().is_some() {
                    let model = r(c, T2);
                    eprintln!(
                        "[custom-track] invalid model DMA tag: handle={:08x} model={model:08x} base={:08x} tag_at={:08x} tag={:016x}",
                        c.bus.read32(model), c.bus.read32(model.wrapping_add(0x24)),
                        r(c, T0), c.g[A1][0]
                    );
                    reported_invalid_tag = true;
                }
            }
        }
        c.cycles += 2;
        if c.g[T9][0] != 0 {
            c.cycles += 8;
            frame.restore(c, S0, 0x40);
            c.g[V0][0] = 1;
            frame.restore(c, S1, 0x30);
            frame.restore(c, S2, 0x20);
            frame.restore(c, S3, 0x10);
            frame.restore(c, S4, 0);
            frame.close(c);
            return;
        }
        c.g[A1][0] = c.ld(r(c, T0));
        if event_due(c) {
            deliver_event_here(c, 0x003a_c100);
        }
    }
}

fn relocate_reference_pair(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[V1][0] & c.g[T8][0];
    let marked = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[V1][0] & c.g[T4][0];
    if marked {
        c.cycles += 2;
        c.g[T1][0] = 1;
        c.sw(r(c, T0).wrapping_add(4), c.g[V0][0]);
    }
    c.cycles += 20;
    c.g[V1][0] = c.lw(r(c, T0).wrapping_add(4));
    c.g[V0][0] = c.lw(r(c, T2).wrapping_add(0x24));
    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
    c.sw(r(c, T0).wrapping_add(4), c.g[V0][0]);
    c.g[T0][0] = sx(r(c, T0).wrapping_add(0x10));
    c.g[V1][0] = c.lw(r(c, T0).wrapping_add(4));
    c.g[A3][0] = sx(r(c, T0).wrapping_add(4));
    c.g[V0][0] = c.lw(r(c, T2).wrapping_add(0x24));
    c.g[A1][0] = c.ld(r(c, T0));
    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
    c.sw(r(c, T0).wrapping_add(4), c.g[V0][0]);
    c.g[V1][0] = c.g[A1][0] << 32;
    c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
    c.g[V0][0] = c.g[V1][0] & 0xffff;
    c.g[T0][0] = sx(r(c, T0).wrapping_add(0x10));
    c.g[A1][0] = c.lw(r(c, A3));
    c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
    c.g[V0][0] = sx(r(c, V0) << 4);
    c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, V0)));
    if c.g[T3][0] != 0 {
        c.cycles += 2;
        c.sw(r(c, A1).wrapping_add(8), c.g[S1][0]);
    } else {
        c.cycles += 1;
        c.sw(r(c, A1).wrapping_add(8), c.g[A0][0]);
    }
    c.cycles += 2;
    if c.g[T1][0] == 0 {
        c.g[T3][0] = 0;
        return;
    }

    c.cycles += 6;
    c.g[V0][0] = c.ld(r(c, T0).wrapping_add(0x10));
    c.g[V0][0] <<= 4;
    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
    c.g[V0][0] &= 7;
    let next_is_return = c.g[V0][0] == c.g[A2][0];
    c.g[A1][0] = sx(r(c, T0).wrapping_add(0x10));
    if next_is_return {
        c.cycles += 6;
        c.g[V1][0] = 0x3000_0000;
        c.g[V1][0] |= 1;
        c.sd(r(c, T0).wrapping_add(8), 0);
        c.g[V0][0] = c.g[S2][0] | c.g[V1][0];
        let first = c.g[T3][0] != 0;
        c.sd(r(c, T0), c.g[V0][0]);
        if first {
            c.cycles += 2;
            c.g[V0][0] = c.g[T7][0] | c.g[V1][0];
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.g[T6][0] | c.g[V1][0];
        }
        c.cycles += 5;
        c.sd(r(c, T0).wrapping_add(0x18), 0);
        c.sd(r(c, T0).wrapping_add(0x10), c.g[V0][0]);
        c.sd(r(c, T0).wrapping_add(0x20), c.g[S3][0]);
        c.sd(r(c, T0).wrapping_add(0x28), 0);
    } else {
        relocate_extended_pair(c);
    }
    c.cycles += 3;
    c.g[T0][0] = sx(r(c, T0).wrapping_add(0x20));
    c.g[T3][0] = 0;
}

fn relocate_extended_pair(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = c.lw(r(c, T0).wrapping_add(4));
    c.g[V0][0] = c.g[V1][0] & c.g[T8][0];
    let marked = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[V1][0] & c.g[T4][0];
    if marked {
        c.cycles += 3;
        c.g[T1][0] = 1;
        c.sw(r(c, T0).wrapping_add(4), c.g[V0][0]);
        c.g[V1][0] = c.lw(r(c, T0).wrapping_add(4));
    }
    c.cycles += 20;
    c.g[V0][0] = c.lw(r(c, T2).wrapping_add(0x24));
    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
    c.sw(r(c, T0).wrapping_add(4), c.g[V0][0]);
    c.g[T0][0] = c.g[A1][0];
    c.g[V1][0] = c.lw(r(c, T0).wrapping_add(4));
    c.g[A3][0] = sx(r(c, T0).wrapping_add(4));
    c.g[V0][0] = c.lw(r(c, T2).wrapping_add(0x24));
    c.g[A1][0] = c.ld(r(c, T0));
    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
    c.sw(r(c, T0).wrapping_add(4), c.g[V0][0]);
    c.g[V1][0] = c.g[A1][0] << 32;
    c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
    c.g[V0][0] = c.g[V1][0] & 0xffff;
    c.g[T0][0] = sx(r(c, T0).wrapping_add(0x10));
    c.g[A1][0] = c.lw(r(c, A3));
    c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
    c.g[V0][0] = sx(r(c, V0) << 4);
    c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, V0)));
    let first = c.g[T3][0] != 0;
    c.sw(r(c, A1).wrapping_add(8), c.g[A0][0]);
    if first {
        c.cycles += 2;
        c.sd(r(c, T0), c.g[S0][0]);
    } else {
        c.cycles += 1;
        c.sd(r(c, T0), c.g[T5][0]);
    }
    c.cycles += 3;
    c.sd(r(c, T0).wrapping_add(8), 0);
    c.sd(r(c, T0).wrapping_add(0x10), c.g[T5][0]);
    c.sd(r(c, T0).wrapping_add(0x18), 0);
}
