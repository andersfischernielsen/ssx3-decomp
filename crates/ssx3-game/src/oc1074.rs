//! Staging module oc1074 (wave wave266): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1074`.

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
    0x002ef2d8,
    oc1074_world_painter_load_view_state,
    "WorldPainter_LoadViewState",
)];

/// `WorldPainter_LoadViewState` (0x002ef2d8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn oc1074_world_painter_load_view_state(c: &mut Cpu) {
    if c.pc != 0x002ef2d8 {
        return ee::s09::f_002ef2d8__generated(c);
    }
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = 0x500000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5c90));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5a0));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
        'l_2ef300: loop {
            c.cycles += 19;
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x208));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x002ef31c);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x002ef31c)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = c.g[S1][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xf0));
            c.g[A2][0] = c.lw(c.g[A0][0] as u32);
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x208));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x002ef340);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x002ef340)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                if event_due(c) {
                    deliver_event_here(c, 0x002ef300);
                }
                continue 'l_2ef300;
            } else {
                c.cycles += 6;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE};

    /// `WorldPainter_LoadViewState` walks the six 0xf0-byte view entries at
    /// 0x4fa370; for each, entry+0x20 holds a pointer to a holder whose object
    /// has a vtable (obj+4) with a slot at +0x208 (delta) / +0x20c (method),
    /// and the method is called twice per entry.
    #[test]
    fn oc1074_world_painter_load_view_state_matches() {
        fakes::covered(
            ee::s09::f_002ef2d8__generated,
            oc1074_world_painter_load_view_state,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1074_2ef2, 12);
                c.g[A0][0] = fakes::ptr(&mut r) as u64;
                for i in 0..6u32 {
                    let entry = 0x4fa370u32 + 0xf0 * i;
                    let holder = fakes::ptr(&mut r);
                    let obj = fakes::ptr(&mut r);
                    let vt = fakes::ptr(&mut r);
                    c.bus.write32(entry + 0x20, holder);
                    c.bus.write32(holder, obj);
                    c.bus.write32(obj + 4, vt);
                    let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                    c.bus.write16(vt + 0x208, delta as u16);
                    c.bus.write32(vt + 0x20c, FAKE);
                }
            },
        );
    }
}
