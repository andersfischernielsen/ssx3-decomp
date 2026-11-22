//! Native shadow-volume ribbon packet writer (0x387ec0), retained uninstalled.
//!
//! The generated EE body defines register, stack, cycle and packet order.
//! No differential test is added or run, as requested by the user.

use crate::abi;
use crate::call::Frame;
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const LOOP: u32 = 0x0038_7fb8;
const RIBBON_DIRTY_GP: u32 = 0x12b0;
const CURSOR: u32 = 0x5a00;
const DEFAULT_PAIR: u32 = 0x004a_43a8;

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 { c.g[reg][0] as u32 }

#[inline]
fn at(c: &Cpu, reg: usize, offset: u32) -> u32 { r(c, reg).wrapping_add(offset) }

/// `EmitShadowStrips(alpha, step, gm, rings, first, count, edgeA, edgeB,
/// textured)`: write batches of at most36 pairs from54-vertex rings.
///
/// Each ring is copied as its contiguous tail, optional wrapped head, then
/// its final two vertices in one literal UNPACK. The last batch reserves
/// those two vertices by reducing the REF sections; intermediate batches
/// use the global pair (normally -1/-1). A subsequent batch overlaps one
/// pair. Only the packet writer is retained here: it is not installed as
/// an override and does not change native rendering.
pub fn emit_shadow_strips(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0xc0);
    c.g[V0][0] = c.lw(at(c, GP, RIBBON_DIRTY_GP));
    c.sq(S0, at(c, SP, 0xb0));
    c.g[T4][0] = c.g[A1][0];
    c.sq(S1, at(c, SP, 0xa0));
    c.g[S0][0] = c.g[A2][0];
    c.sq(S2, at(c, SP, 0x90));
    c.sq(S3, at(c, SP, 0x80));
    c.sq(S4, at(c, SP, 0x70));
    c.sq(S5, at(c, SP, 0x60));
    c.sq(S6, at(c, SP, 0x50));
    c.sq(S7, at(c, SP, 0x40));
    c.sq(FP, at(c, SP, 0x30));
    let clean = c.g[V0][0] == 0;
    c.sw(at(c, SP, 0x10), c.g[A0][0]);
    if clean {
        c.cycles += 2;
        c.g[V0][0] = 0x3a08;
    } else {
        c.cycles += 3;
        c.g[V0][0] = 0x3a00;
    }

    c.cycles += 3;
    c.g[T3][0] = sx(r(c, V0) >> 3);
    let textured = c.g[T2][0] != 0;
    c.sw(at(c, GP, RIBBON_DIRTY_GP), 0);
    if textured {
        c.cycles += 3;
        c.g[A2][0] = 0x5c;
        c.g[S7][0] = 0x412;
    } else {
        c.cycles += 2;
        c.g[A2][0] = 4;
        c.g[S7][0] = 0x41f;
    }

    c.cycles += 5;
    c.g[V0][0] = c.lw(at(c, SP, 0x10));
    c.g[V1][0] = c.lw(at(c, V0, CURSOR));
    c.g[V0][0] = 0x3000_0000;
    let empty = c.g[A3][0] == 0;
    c.g[T2][0] = c.g[V1][0] | c.g[V0][0];
    if !empty {
        c.cycles += 29;
        c.mfc1(V1, 13);
        c.g[A0][0] = 0x1400_0000;
        c.g[A0][0] |= c.g[T3][0];
        c.g[A2][0] <<= 47;
        c.g[V0][0] = c.g[V1][0];
        c.g[A0][0] <<= 32;
        c.g[V0][0] <<= 32;
        c.g[A1][0] = sx(r(c, T0) << 2);
        c.g[V0][0] >>= 32;
        c.g[V1][0] = sx(r(c, T1) << 2);
        c.sd(at(c, SP, 0x18), c.g[V0][0]);
        c.g[S3][0] = 0x60000;
        c.g[V0][0] = 0x8000;
        c.g[V0][0] <<= 31;
        c.sd(at(c, SP, 0x20), c.g[A0][0]);
        c.g[T1][0] = sx(r(c, V1).wrapping_add(r(c, T4)));
        c.g[T0][0] = sx(r(c, A1).wrapping_add(r(c, T4)));
        c.g[FP][0] = c.g[A2][0] | c.g[V0][0];
        c.g[T8][0] = 54;
        c.g[S5][0] = 0;
        c.g[S2][0] = 0x1000_0001;
        c.g[S4][0] = 0x3000_0000;
        c.g[S1][0] = 0x30;
        c.g[S6][0] = 0x1000_0007;
        c.g[S3][0] |= 0x8000;
        c.g[V0][0] = ((c.g[A3][0] as i64) < 37) as u64;

        loop {
            // T6=batch size, T4=contiguous tail, T3=wrapped head.
            c.cycles += 17;
            c.g[T6][0] = 36;
            if c.g[V0][0] != 0 { c.g[T6][0] = c.g[A3][0]; }
            c.g[T4][0] = sx(r(c, T8).wrapping_sub(r(c, S0)));
            c.g[V0][0] = ((c.g[T6][0] as i64) < (c.g[T4][0] as i64)) as u64;
            c.g[V1][0] = u64::MAX;
            if c.g[V0][0] != 0 { c.g[T4][0] = c.g[T6][0]; }
            c.g[A0][0] = 0x004a_0000;
            c.g[T3][0] = sx(r(c, T6).wrapping_sub(r(c, T4)));
            c.g[V0][0] = ((c.g[T6][0] as i64) < (c.g[A3][0] as i64)) as u64;
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[T3][0] as i64)) as u64;
            c.g[T7][0] = sx(DEFAULT_PAIR);
            c.ldl(A1, at(c, T7, 7));
            c.ldr(A1, r(c, T7));
            c.sdl(A1, at(c, SP, 7));
            c.sdr(A1, r(c, SP));
            let intermediate = c.g[V0][0] != 0;
            if c.g[V1][0] == 0 { c.g[T3][0] = 0; }
            if !intermediate {
                c.cycles += 3;
                c.g[V0][0] = sx(r(c, S0).wrapping_add(r(c, T6)));
                c.cycles += 9;
                c.g[V1][0] = sx(r(c, V0).wrapping_sub(2));
                c.div(0, V1, T8);
                c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
                c.g[V1][0] = ((c.g[T3][0] as i64) < 2) as u64;
                c.g[A0][0] = c.hi[0];
                c.div(0, V0, T8);
                c.sw(r(c, SP), c.g[A0][0]);
                c.cycles += 3;
                c.g[A1][0] = c.hi[0];
                let pair_straddles = c.g[V1][0] != 0;
                c.sw(at(c, SP, 4), c.g[A1][0]);
                if pair_straddles {
                    c.cycles += 3;
                    c.g[V0][0] = sx(r(c, T4).wrapping_sub(2));
                    c.g[T4][0] = sx(r(c, V0).wrapping_add(r(c, T3)));
                    c.g[T3][0] = 0;
                } else {
                    c.cycles += 2;
                    c.g[T3][0] = sx(r(c, T3).wrapping_sub(2));
                }
            }

            // Header: STCYCL/UNPACK, strip tag, count and alpha ramp.
            c.cycles += 33;
            c.mfc1(V0, 12);
            c.g[A0][0] = sx(r(c, T6) << 1);
            c.g[V1][0] = c.g[T6][0] << 32;
            c.g[T7][0] = 0x0100_0101_0000_0000;
            c.g[A1][0] = c.g[V0][0];
            c.g[T9][0] = 0x6c00_0000;
            c.g[V0][0] = c.g[A0][0] | 0x8000;
            c.g[A1][0] <<= 32;
            c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, T6)));
            c.g[A2][0] = c.g[V0][0] | c.g[FP][0];
            c.g[A0][0] |= c.g[V1][0];
            c.g[V0][0] = 0xc000;
            c.g[V0][0] <<= 46;
            c.g[V1][0] = c.ld(at(c, SP, 0x18));
            c.g[A2][0] |= c.g[V0][0];
            c.g[V0][0] = 0x1000_0003;
            c.sd(at(c, T2, 0x20), c.g[A2][0]);
            c.g[A1][0] |= c.g[V1][0];
            c.sd(r(c, T2), c.g[V0][0]);
            c.g[V1][0] = 0xd805;
            c.g[V1][0] <<= 47;
            c.sd(at(c, T2, 0x30), c.g[A0][0]);
            c.sd(at(c, T2, 0x18), c.g[V1][0]);
            c.g[T5][0] = 2;
            c.sd(at(c, T2, 0x38), c.g[A1][0]);
            c.sd(at(c, T2, 8), 0);
            c.sd(at(c, T2, 0x10), c.g[T7][0]);
            c.sd(at(c, T2, 0x28), c.g[S7][0]);
            let no_tail = c.g[T4][0] == 0;
            c.g[T2][0] = sx(at(c, T2, 0x40));
            if !no_tail {
                c.cycles += 20;
                c.g[V0][0] = sx(r(c, T4) << 1);
                c.sd(r(c, T2), c.g[S2][0]);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T4)));
                c.sd(at(c, T2, 8), 0);
                c.g[V1][0] = sx(r(c, V0) << 16);
                c.sd(at(c, T2, 0x10), c.g[T7][0]);
                c.g[V1][0] |= 0x8002;
                c.mult(0, A0, S0, S1);
                c.g[V1][0] |= c.g[T9][0];
                c.g[A1][0] = c.g[V0][0] | c.g[S4][0];
                c.g[V1][0] <<= 32;
                c.g[T5][0] = sx(r(c, V0).wrapping_add(2));
                c.sd(at(c, T2, 0x18), c.g[V1][0]);
                c.g[V0][0] = c.lw(r(c, T0));
                c.sd(at(c, T2, 0x28), 0);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V0][0] <<= 32;
                c.g[V0][0] |= c.g[A1][0];
                c.sd(at(c, T2, 0x20), c.g[V0][0]);
                c.g[T2][0] = sx(at(c, T2, 0x30));
            }

            c.cycles += 2;
            let no_head = c.g[T3][0] == 0;
            c.g[V1][0] = sx(r(c, T3) << 1);
            if !no_head {
                c.cycles += 18;
                c.sd(r(c, T2), c.g[S2][0]);
                c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, T3)));
                c.sd(at(c, T2, 8), 0);
                c.g[V0][0] = sx(r(c, V1) << 16);
                c.sd(at(c, T2, 0x10), c.g[T7][0]);
                c.g[V0][0] |= 0x8000;
                c.g[A0][0] = c.g[V1][0] | c.g[S4][0];
                c.g[V0][0] |= c.g[T5][0];
                c.g[V0][0] |= c.g[T9][0];
                c.g[T5][0] = sx(r(c, T5).wrapping_add(r(c, V1)));
                c.g[V0][0] <<= 32;
                c.sd(at(c, T2, 0x18), c.g[V0][0]);
                c.g[V1][0] = c.lwu(r(c, T0));
                c.sd(at(c, T2, 0x28), 0);
                c.g[V1][0] <<= 32;
                c.g[V1][0] |= c.g[A0][0];
                c.sd(at(c, T2, 0x20), c.g[V1][0]);
                c.g[T2][0] = sx(at(c, T2, 0x30));
            }

            c.cycles += 3;
            c.g[V0][0] = c.lw(r(c, SP));
            let no_pair = (c.g[V0][0] as i64) < 0;
            c.g[V0][0] = c.g[T5][0] | c.g[S3][0];
            if !no_pair {
                c.cycles += 30;
                c.sd(r(c, T2), c.g[S6][0]);
                c.g[V0][0] |= c.g[T9][0];
                c.sd(at(c, T2, 8), 0);
                c.g[V0][0] <<= 32;
                c.sd(at(c, T2, 0x10), c.g[T7][0]);
                c.sd(at(c, T2, 0x18), c.g[V0][0]);
                c.g[T5][0] = sx(r(c, T5).wrapping_add(6));
                c.g[T2][0] = sx(at(c, T2, 0x20));
                c.g[A0][0] = c.lw(r(c, SP));
                c.g[V1][0] = c.lw(r(c, T0));
                c.mult(0, A0, A0, S1);
                c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
                c.lq(V0, r(c, A0));
                c.sq(V0, r(c, T2));
                c.lq(V1, at(c, A0, 0x10));
                c.sq(V1, at(c, T2, 0x10));
                c.lq(V0, at(c, A0, 0x20));
                c.sq(V0, at(c, T2, 0x20));
                c.g[T2][0] = sx(at(c, T2, 0x30));
                c.g[A0][0] = c.lw(at(c, SP, 4));
                c.g[V1][0] = c.lw(r(c, T0));
                c.mult(0, A0, A0, S1);
                c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
                c.lq(V0, r(c, A0));
                c.sq(V0, r(c, T2));
                c.lq(V1, at(c, A0, 0x10));
                c.sq(V1, at(c, T2, 0x10));
                c.lq(V0, at(c, A0, 0x20));
                c.sq(V0, at(c, T2, 0x20));
                c.g[T2][0] = sx(at(c, T2, 0x30));
            }

            // Edge B starts exactly after the batch-size edge-A vertices.
            c.cycles += 2;
            let no_tail = c.g[T4][0] == 0;
            c.g[V0][0] = sx(r(c, T4) << 1);
            if !no_tail {
                c.cycles += 20;
                c.sd(r(c, T2), c.g[S2][0]);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T4)));
                c.sd(at(c, T2, 8), 0);
                c.g[V1][0] = sx(r(c, V0) << 16);
                c.sd(at(c, T2, 0x10), c.g[T7][0]);
                c.g[V1][0] |= 0x8000;
                c.mult(0, A0, S0, S1);
                c.g[V1][0] |= c.g[T5][0];
                c.g[A1][0] = c.g[V0][0] | c.g[S4][0];
                c.g[V1][0] |= c.g[T9][0];
                c.g[T5][0] = sx(r(c, T5).wrapping_add(r(c, V0)));
                c.g[V1][0] <<= 32;
                c.sd(at(c, T2, 0x18), c.g[V1][0]);
                c.g[V0][0] = c.lw(r(c, T1));
                c.sd(at(c, T2, 0x28), 0);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V0][0] <<= 32;
                c.g[V0][0] |= c.g[A1][0];
                c.sd(at(c, T2, 0x20), c.g[V0][0]);
                c.g[T2][0] = sx(at(c, T2, 0x30));
            }

            c.cycles += 2;
            let no_head = c.g[T3][0] == 0;
            c.g[V1][0] = sx(r(c, T3) << 1);
            if !no_head {
                c.cycles += 18;
                c.sd(r(c, T2), c.g[S2][0]);
                c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, T3)));
                c.sd(at(c, T2, 8), 0);
                c.g[V0][0] = sx(r(c, V1) << 16);
                c.sd(at(c, T2, 0x10), c.g[T7][0]);
                c.g[V0][0] |= 0x8000;
                c.g[A0][0] = c.g[V1][0] | c.g[S4][0];
                c.g[V0][0] |= c.g[T5][0];
                c.g[V0][0] |= c.g[T9][0];
                c.g[T5][0] = sx(r(c, T5).wrapping_add(r(c, V1)));
                c.g[V0][0] <<= 32;
                c.sd(at(c, T2, 0x18), c.g[V0][0]);
                c.g[V1][0] = c.lwu(r(c, T1));
                c.sd(at(c, T2, 0x28), 0);
                c.g[V1][0] <<= 32;
                c.g[V1][0] |= c.g[A0][0];
                c.sd(at(c, T2, 0x20), c.g[V1][0]);
                c.g[T2][0] = sx(at(c, T2, 0x30));
            }

            c.cycles += 3;
            c.g[V0][0] = c.lw(r(c, SP));
            let no_pair = (c.g[V0][0] as i64) < 0;
            c.g[V0][0] = c.g[T5][0] | c.g[S3][0];
            if !no_pair {
                c.cycles += 29;
                c.sd(at(c, T2, 0x10), c.g[T7][0]);
                c.g[V0][0] |= c.g[T9][0];
                c.sd(r(c, T2), c.g[S6][0]);
                c.g[V0][0] <<= 32;
                c.sd(at(c, T2, 8), 0);
                c.sd(at(c, T2, 0x18), c.g[V0][0]);
                c.g[T2][0] = sx(at(c, T2, 0x20));
                c.g[A0][0] = c.lw(r(c, SP));
                c.g[V1][0] = c.lw(r(c, T1));
                c.mult(0, A0, A0, S1);
                c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
                c.lq(V0, r(c, A0));
                c.sq(V0, r(c, T2));
                c.lq(V1, at(c, A0, 0x10));
                c.sq(V1, at(c, T2, 0x10));
                c.lq(V0, at(c, A0, 0x20));
                c.sq(V0, at(c, T2, 0x20));
                c.g[T2][0] = sx(at(c, T2, 0x30));
                c.g[A0][0] = c.lw(at(c, SP, 4));
                c.g[V1][0] = c.lw(r(c, T1));
                c.mult(0, A0, A0, S1);
                c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
                c.lq(V0, r(c, A0));
                c.sq(V0, r(c, T2));
                c.lq(V1, at(c, A0, 0x10));
                c.sq(V1, at(c, T2, 0x10));
                c.lq(V0, at(c, A0, 0x20));
                c.sq(V0, at(c, T2, 0x20));
                c.g[T2][0] = sx(at(c, T2, 0x30));
            }

            // MSCAL, advance alpha, overlap the next batch by one pair.
            c.cycles += 12;
            c.mtc1(T6, 0);
            c.cvt_s_w(0, 0);
            c.sd(r(c, T2), c.g[S2][0]);
            c.sd(at(c, T2, 8), 0);
            c.g[A3][0] = sx(r(c, A3).wrapping_sub(r(c, T6)));
            c.sd(at(c, T2, 0x10), 0);
            c.mul_s(0, 13, 0);
            c.g[A1][0] = c.ld(at(c, SP, 0x20));
            c.sd(at(c, T2, 0x18), c.g[A1][0]);
            c.add_s(12, 12, 0);
            let finished = (c.g[A3][0] as i64) <= 0;
            c.g[T2][0] = sx(at(c, T2, 0x20));
            if !finished {
                c.cycles += 3;
                c.g[V0][0] = sx(r(c, S0).wrapping_add(r(c, T6)));
                c.cycles += 5;
                c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
                c.g[A3][0] = sx(r(c, A3).wrapping_add(1));
                c.div(0, V0, T8);
                c.g[V1][0] = c.hi[0];
                c.g[S0][0] = c.g[V1][0];
            }
            c.cycles += 2;
            let more = c.g[A3][0] != 0;
            c.g[V0][0] = ((c.g[A3][0] as i64) < 37) as u64;
            if !more { break; }
            if event_due(c) { deliver_event_here(c, LOOP); }
        }
    }

    c.cycles += 16;
    c.g[V0][0] = 0x0fff_0000;
    c.g[V1][0] = c.lw(at(c, SP, 0x10));
    c.g[V0][0] |= 0xffff;
    c.lq(S0, at(c, SP, 0xb0));
    c.g[V0][0] &= c.g[T2][0];
    c.lq(S1, at(c, SP, 0xa0));
    c.lq(S2, at(c, SP, 0x90));
    c.lq(S3, at(c, SP, 0x80));
    c.lq(S4, at(c, SP, 0x70));
    c.lq(S5, at(c, SP, 0x60));
    c.lq(S6, at(c, SP, 0x50));
    c.lq(S7, at(c, SP, 0x40));
    c.lq(FP, at(c, SP, 0x30));
    c.sw(at(c, V1, CURSOR), c.g[V0][0]);
    frame.close(c);
    abi::ret(c);
}
