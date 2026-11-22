//! Native cBoardWakeFX constructor (0x002dcb88), retained uninstalled.
//!
//! No automated tests, builds, replays or game runs were added or run.
//! Generated EE is authoritative for registers, memory, cycles and calls.

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const STRIPS: u32 = 0x0c;
const RING_ROWS: u32 = 0x10;
const POINT_COUNT: u32 = 0xb4;
const RESET: Callee = Callee {
    addr: 0x002d_cf28,
    f: ssx3_ee::s09::cBoardWakeFX_Reset,
};

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 { c.g[reg][0] as u32 }

#[inline]
fn at(c: &Cpu, reg: usize, offset: u32) -> u32 {
    r(c, reg).wrapping_add(offset)
}

/// `cBoardWakeFX* cBoardWakeFX_ctor(cBoardWakeFX* self, int in_charge)`.
///
/// Construct the virtual rider base when requested, install the three
/// rider vtables, and adjust two stack copies when not in charge. Allocate
/// 32 0x70-byte strip elements and five rows of 32 0x30-byte ring vertices.
/// The element constructors are empty: neither allocation is zeroed here
/// and no RGB component is initialized. Set self+0xb4=-1 and call Reset.
pub fn board_wake_fx_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S0, 0x150);
        frame.save(c, S1, 0x140);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x130);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x120);
        frame.save(c, S4, 0x110);
        frame.save_ra(c, 0x100);
        if c.g[S1][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = sx(at(c, S0, 0xc0));
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x002d_cbc0);
            c.sw(r(c, S0), c.g[V0][0]);
            call_ptr(c, 0x0011_b698, 0x002d_cbc0)?;
        }

        c.cycles += 19;
        c.g[V1][0] = c.lw(r(c, S0));
        c.g[V0][0] = 0x0049_0000;
        c.g[A2][0] = sx(r(c, V0).wrapping_sub(0x7258));
        c.g[A3][0] = c.g[V0][0];
        c.sw(at(c, V1, 0x6e8), c.g[A2][0]);
        c.g[V0][0] = 0x0046_0000;
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x6470));
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = c.lw(r(c, S0));
        c.g[T3][0] = c.g[V1][0];
        c.g[V1][0] = sx(r(c, V1).wrapping_sub(0x7210));
        c.g[S4][0] = sx(at(c, S0, STRIPS));
        c.sw(at(c, A0, 0x6d0), c.g[V0][0]);
        c.g[A1][0] = 0x0048_0000;
        c.g[S2][0] = sx(at(c, S0, RING_ROWS));
        c.g[S3][0] = 0x0048_0000;
        c.g[V0][0] = c.lw(r(c, S0));
        c.sw(at(c, V0, 0x6c0), c.g[V1][0]);
        if c.g[S1][0] == 0 {
            // First vtable: copy 0x48 bytes to sp and fix its base delta.
            c.cycles += 3;
            c.g[V1][0] = c.g[SP][0];
            c.g[V0][0] = sx(at(c, A2, 0x40));
            c.g[T1][0] = sx(frame.at(c, 0x50));
            loop {
                c.cycles += 11;
                c.g[A0][0] = c.ld(r(c, A2));
                c.g[T0][0] = c.ld(at(c, A2, 8));
                c.g[T2][0] = c.ld(at(c, A2, 0x10));
                c.g[T4][0] = c.ld(at(c, A2, 0x18));
                c.sd(r(c, V1), c.g[A0][0]);
                c.sd(at(c, V1, 8), c.g[T0][0]);
                c.sd(at(c, V1, 0x10), c.g[T2][0]);
                c.sd(at(c, V1, 0x18), c.g[T4][0]);
                c.g[A2][0] = sx(at(c, A2, 0x20));
                let more = c.g[A2][0] != c.g[V0][0];
                c.g[V1][0] = sx(at(c, V1, 0x20));
                if !more { break; }
                if event_due(c) { deliver_event_here(c, 0x002d_cc18); }
            }
            c.cycles += 14;
            c.g[V0][0] = c.lw(r(c, S0));
            c.g[A0][0] = sx(r(c, A3).wrapping_sub(0x7258));
            c.g[A3][0] = c.ld(r(c, A2));
            c.sd(r(c, V1), c.g[A3][0]);
            c.sw(at(c, V0, 0x6e8), c.g[SP][0]);
            c.g[A3][0] = sx(r(c, T3).wrapping_sub(0x7210));
            c.g[V1][0] = c.lhu(at(c, A0, 8));
            c.g[T2][0] = sx(at(c, A3, 0xa0));
            c.g[V0][0] = c.lw(r(c, S0));
            c.g[A2][0] = c.g[T1][0];
            c.g[V0][0] = sx(r(c, V0).wrapping_sub(0xc0));
            c.g[T0][0] = sx(r(c, S0).wrapping_sub(r(c, V0)));
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, T0)));
            c.sh(frame.at(c, 8), c.g[V1][0]);

            // Second vtable: copy 0xb0 bytes to sp+0x50, same delta fixup.
            loop {
                c.cycles += 11;
                c.g[V0][0] = c.ld(r(c, A3));
                c.g[V1][0] = c.ld(at(c, A3, 8));
                c.g[A0][0] = c.ld(at(c, A3, 0x10));
                c.g[T4][0] = c.ld(at(c, A3, 0x18));
                c.sd(r(c, A2), c.g[V0][0]);
                c.sd(at(c, A2, 8), c.g[V1][0]);
                c.sd(at(c, A2, 0x10), c.g[A0][0]);
                c.sd(at(c, A2, 0x18), c.g[T4][0]);
                c.g[A3][0] = sx(at(c, A3, 0x20));
                let more = c.g[A3][0] != c.g[T2][0];
                c.g[A2][0] = sx(at(c, A2, 0x20));
                if !more { break; }
                if event_due(c) { deliver_event_here(c, 0x002d_cc7c); }
            }
            c.cycles += 10;
            c.g[V0][0] = sx(r(c, T3).wrapping_sub(0x7210));
            c.g[A0][0] = c.lw(r(c, S0));
            c.g[V1][0] = c.lhu(at(c, V0, 8));
            c.g[V0][0] = c.ld(r(c, A3));
            c.g[T2][0] = c.ld(at(c, A3, 8));
            c.sd(r(c, A2), c.g[V0][0]);
            c.sd(at(c, A2, 8), c.g[T2][0]);
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, T0)));
            c.sw(at(c, A0, 0x6c0), c.g[T1][0]);
            c.sh(frame.at(c, 0x58), c.g[V1][0]);
        }

        c.cycles += 8;
        c.g[A1][0] = sx(at(c, A1, 0x7968));
        c.g[A0][0] = 0xe00;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x002d_cce4)?;
        c.g[A1][0] = u64::MAX;
        c.g[V1][0] = 0x1f;
        // These loops invoke no constructor and write no element memory.
        loop {
            c.cycles += 7;
            c.g[A0][0] = sx(r(c, V1).wrapping_sub(1));
            c.g[V1][0] = 4;
            c.g[V1][0] = c.g[A0][0];
            let more = c.g[V1][0] != c.g[A1][0];
            c.g[A0][0] = 0x1e00;
            if !more { break; }
            if event_due(c) { deliver_event_here(c, 0x002d_ccf0); }
        }
        c.cycles += 7;
        c.sw(r(c, S4), c.g[V0][0]);
        c.g[A1][0] = sx(at(c, S3, 0x7980));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x002d_cd20)?;
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = 0x9f;
        loop {
            c.cycles += 7;
            c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
            let more = c.g[V1][0] != c.g[A0][0];
            c.g[A1][0] = 0x600;
            if !more { break; }
            if event_due(c) { deliver_event_here(c, 0x002d_cd28); }
        }
        c.cycles += 3;
        c.sw(r(c, S2), c.g[V0][0]);
        c.g[V1][0] = sx(at(c, S2, 4));
        c.g[A0][0] = 3;
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.lw(at(c, S0, RING_ROWS));
            c.g[A0][0] = sx(r(c, A0).wrapping_sub(1));
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A1)));
            c.sw(r(c, V1), c.g[V0][0]);
            c.g[A1][0] = sx(at(c, A1, 0x600));
            let more = (c.g[A0][0] as i64) >= 0;
            c.g[V1][0] = sx(at(c, V1, 4));
            if !more { break; }
            if event_due(c) { deliver_event_here(c, 0x002d_cd50); }
        }
        c.cycles += 13;
        c.g[V0][0] = u64::MAX;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x002d_cd7c);
        c.sw(at(c, S0, POINT_COUNT), c.g[V0][0]);
        RESET.call(c, 0x002d_cd7c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x140);
        frame.restore(c, S0, 0x150);
        frame.restore(c, S2, 0x130);
        frame.restore(c, S3, 0x120);
        frame.restore(c, S4, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}
