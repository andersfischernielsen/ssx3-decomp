//! Staging module oc1010 (wave wave261): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1010`.

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
    0x0028_9df0,
    snd_load_queue_finish_current,
    "SndLoadQueue_FinishCurrent",
)];

/// Pops one entry of the ring at `self + base` after the empty check failed
/// (`v1 = self + base`, `a1` = the tail index): leaves the entry in `v0`,
/// advances (and wraps) the index at `+0xc`. 10 cycles, +1 on the wrap.
#[inline(always)]
fn oc1010_ring_pop(c: &mut Cpu, s1: u32, base: u32) {
    c.cycles += 10;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = c.lw(s1.wrapping_add(base));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
    c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let in_range = c.g[V1][0] != 0;
    c.sw(s1.wrapping_add(base).wrapping_add(0xc), c.g[A1][0]);
    if !in_range {
        c.cycles += 1;
        c.sw(s1.wrapping_add(base).wrapping_add(0xc), 0);
    }
}

/// Pops the ring at `self + base` (second to fourth rings of the async
/// system: names, kinds): `-1` when empty, else the entry. Returns it.
#[inline(always)]
fn oc1010_pop_ring(c: &mut Cpu, s1: u32, base: u32) -> u64 {
    c.cycles += 4;
    c.g[V0][0] = c.lw(s1.wrapping_add(base).wrapping_add(0x8));
    c.g[A1][0] = c.lw(s1.wrapping_add(base).wrapping_add(0xc));
    let nonempty = c.g[V0][0] != c.g[A1][0];
    c.g[V1][0] = sx(s1.wrapping_add(base));
    if !nonempty {
        c.cycles += 2;
        return 0xffff_ffff_ffff_ffff;
    }
    oc1010_ring_pop(c, s1, base);
    c.cycles += 1;
    c.g[V0][0]
}

/// `void SndLoadQueue_FinishCurrent(SndLoadQueue* self, int flush)` (0x289df0),
/// the C++ `cAsyncSys_DoCancel`: pops one entry of each of the four request
/// rings (user handle +0x18c, name slot +0x19c, kind +0x1ac, flags +0x1bc),
/// and when a load is in flight (+4) asks the async file layer for its result
/// and notifies through the vtable (+0x1d4, entry at +0x28/+0x2c) that the
/// bank finished (kinds 0 and 2: result 1 means ok; kind 1 uses the kind
/// itself), then clears busy. With `flush` the tails are set to the heads.
/// Finally starts the next queued load.
pub fn snd_load_queue_finish_current(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S4, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S5, 0x10);
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18c));
        frame.save(c, S0, 0x60);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save_ra(c, 0);
        let this = c.g[S1][0] as u32;
        c.g[A1][0] = c.lw(this.wrapping_add(0x198));
        c.g[V0][0] = c.lw(this.wrapping_add(0x194));
        let empty = c.g[V0][0] == c.g[A1][0];
        c.g[S4][0] = 0xffff_ffff_ffff_ffff;
        if !empty {
            oc1010_ring_pop(c, this, 0x18c);
            c.cycles += 1;
            c.g[S4][0] = c.g[V0][0];
        }
        c.g[S3][0] = oc1010_pop_ring(c, this, 0x19c);
        c.g[S2][0] = oc1010_pop_ring(c, this, 0x1ac);
        // Fourth ring (flags): popped and dropped.
        c.cycles += 4;
        c.g[V0][0] = c.lw(this.wrapping_add(0x1c4));
        c.g[A0][0] = c.lw(this.wrapping_add(0x1c8));
        let empty = c.g[V0][0] == c.g[A0][0];
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        if !empty {
            c.cycles += 5;
            c.g[V1][0] = c.lw(this.wrapping_add(0x1bc));
            c.sw(this.wrapping_add(0x1c8), c.g[V0][0]);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.sw(this.wrapping_add(0x1c8), 0);
            }
        }
        // The in-flight load: finish it according to its kind.
        c.cycles += 3;
        c.g[V0][0] = c.lw(this.wrapping_add(0x4));
        let idle = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        if !idle {
            c.cycles += 2;
            let kind_is_1 = c.g[S2][0] == c.g[V0][0];
            c.g[V0][0] = ((c.g[S2][0] as i64) < 2) as u64;
            // true: kind 0 or 2 arm (copy A); false: kind 1 arm (copy B).
            let arm_a;
            if kind_is_1 {
                arm_a = None;
            } else {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 0x2;
                    c.cycles += 2;
                    arm_a = if c.g[S2][0] != c.g[V0][0] {
                        Some(false)
                    } else {
                        Some(true)
                    };
                } else {
                    c.cycles += 2;
                    if c.g[S2][0] == 0 {
                        arm_a = Some(true);
                    } else {
                        c.cycles += 2;
                        arm_a = Some(false);
                    }
                }
            }
            match arm_a {
                Some(true) => {
                    c.cycles += 25;
                    c.g[A0][0] = c.lw(this);
                    call_ptr(c, 0x003d_faf0, 0x0028_9f4c)?;
                    c.g[A0][0] = c.lw(this);
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x003d_f9d8, 0x0028_9f60)?;
                    c.g[V1][0] = c.lw(this.wrapping_add(0x1d4));
                    c.g[T0][0] = sx((c.g[S3][0] as u32) << 6);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[S0][0] ^= 0x1;
                    c.g[T3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    c.g[A3][0] = 0x2;
                    if c.g[S0][0] == 0 {
                        c.g[A3][0] = c.g[V0][0];
                    }
                    c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[T0][0] = sx(this.wrapping_add(c.g[T0][0] as u32));
                    c.g[A2][0] = 0xffff_ffff_ffff_ffff;
                    c.g[T1][0] = 0;
                    let target = c.g[T3][0] as u32;
                    c.g[T2][0] = 0;
                    call_ptr(c, target, 0x0028_9fa0)?;
                    c.sw(this.wrapping_add(0x4), 0);
                }
                None => {
                    c.cycles += 23;
                    c.g[A0][0] = c.lw(this);
                    call_ptr(c, 0x003d_faf0, 0x0028_9fb0)?;
                    c.g[A0][0] = c.lw(this);
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x003d_f9d8, 0x0028_9fc4)?;
                    c.g[V1][0] = c.lw(this.wrapping_add(0x1d4));
                    c.g[T0][0] = sx((c.g[S3][0] as u32) << 6);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0xc));
                    c.g[S0][0] ^= 0x1;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[A3][0] = 0x2;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    if c.g[S0][0] == 0 {
                        c.g[A3][0] = c.g[S2][0];
                    }
                    c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[T0][0] = sx(this.wrapping_add(c.g[T0][0] as u32));
                    c.g[A2][0] = 0xffff_ffff_ffff_ffff;
                    c.g[T1][0] = 0;
                    let target = c.g[V0][0] as u32;
                    c.g[T2][0] = 0;
                    call_ptr(c, target, 0x0028_a000)?;
                    c.sw(this.wrapping_add(0x4), 0);
                }
                Some(false) => {}
            }
        }
        c.cycles += 2;
        if c.g[S5][0] != 0 {
            c.cycles += 8;
            c.g[V0][0] = c.lw(this.wrapping_add(0x194));
            c.g[V1][0] = c.lw(this.wrapping_add(0x1a4));
            c.g[A0][0] = c.lw(this.wrapping_add(0x1b4));
            c.g[A1][0] = c.lw(this.wrapping_add(0x1c4));
            c.sw(this.wrapping_add(0x198), c.g[V0][0]);
            c.sw(this.wrapping_add(0x1a8), c.g[V1][0]);
            c.sw(this.wrapping_add(0x1b8), c.g[A0][0]);
            c.sw(this.wrapping_add(0x1c8), c.g[A1][0]);
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0028_a298, 0x0028_a034)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::DATA;
    use crate::game::sound::soundsys::tests::{oc739_gate, oc739_install, OC739_CTRL};

    /// `SndLoadQueue_FinishCurrent`: four rings (size, data, head, tail at
    /// +0x18c/+0x19c/+0x1ac/+0x1bc; empty, wrapping and plain pops), busy or
    /// not, every kind (0, 1, 2, other, empty ring), the async layer's
    /// cancel and result (gated fakes), the vtable notification, with and
    /// without flush; the next load starts through the real `StartNext`.
    #[test]
    fn snd_load_queue_finish_current_matches() {
        oc739_install();
        fakes::covered(
            ee::s07::f_00289df0__generated,
            snd_load_queue_finish_current,
            500,
            &|c, seed| {
                let mut r = machine(c, seed, 0x289df0, 6);
                oc739_gate(c);
                c.bus.write32(OC739_CTRL + 0x20, r.next_u32() % 3);
                c.bus.write32(OC739_CTRL + 0x24, r.next_u32() % 2);
                let this = DATA + 0x100;
                let vt = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 3 == 0 { 1 } else { 0 };
                c.bus.write32(this, DATA + 0x5000);
                c.bus.write32(this + 4, (seed % 4 != 0) as u32);
                let kinds = [0u32, 1, 2, 3, u32::MAX, 1, 0, 2, 1, 2];
                for k in 0..4u32 {
                    let base = this + 0x18c + 0x10 * k;
                    let size = 1 + r.next_u32() % 4;
                    let data = DATA + 0x1000 + 0x40 * k;
                    let head = r.next_u32() % size;
                    let mut tail = r.next_u32() % size;
                    if r.next_u32() % 4 != 0 && head == tail {
                        tail = (tail + 1) % size;
                    }
                    c.bus.write32(base, size);
                    c.bus.write32(base + 4, data);
                    c.bus.write32(base + 8, head);
                    c.bus.write32(base + 0xc, tail);
                    for i in 0..size {
                        let v = if k == 2 {
                            kinds[(r.next_u32() % 10) as usize]
                        } else {
                            r.next_u32() % 4
                        };
                        c.bus.write32(data + 4 * i, v);
                    }
                }
                c.bus.write32(this + 0x1cc, r.next_u32());
                c.bus.write32(this + 0x1d0, r.next_u32());
                c.bus.write32(this + 0x1d4, vt);
                for off in (0..0x40u32).step_by(4) {
                    let lh_slot = matches!(off, 0x10 | 0x18 | 0x20 | 0x28);
                    c.bus
                        .write32(vt + off, if lh_slot { 0 } else { fakes::FAKE });
                }
                c.bus
                    .write16(vt + 0x28, (r.next_u32() as i16 as i32 & !3) as u16);
                c.pc = 0x0028_9df0;
            },
        );
    }
}
