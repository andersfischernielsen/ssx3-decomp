//! `src/seg/seg_1f1548.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002f_1768,
        c_attached_dynamic_emitter_ctor,
        "cAttachedDynamicEmitter_ctor",
    ),
    (
        0x002f_17a8,
        c_attached_dynamic_emitter_v04,
        "cAttachedDynamicEmitter_v04",
    ),
    (
        0x002f_2130,
        c_attached_dynamic_emitter_create,
        "cAttachedDynamicEmitter_Create",
    ),
];

/// `cAttachedDynamicEmitter::cAttachedDynamicEmitter(this)` (0x2f1768): runs
/// the base constructor (0x370b60), then clears the emitter's counters and
/// stores its vtable at `+0x1f8`.
pub fn c_attached_dynamic_emitter_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0037_0b60, 0x002f_177c)?;
        c.g[V1][0] = 0x490000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7e18));
        let this = c.g[S0][0] as u32;
        Addr(this.wrapping_add(0x200)).sw(c, 0);
        let vtable = c.g[V1][0];
        Addr(this.wrapping_add(0x1f8)).sw(c, vtable);
        Addr(this.wrapping_add(0x204)).sw(c, 0);
        Addr(this.wrapping_add(0x208)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cAttachedDynamicEmitter::v04(this)` (0x2f17a8): stores the vtable at
/// `+0x1f8` and calls the virtual-slot helper 0x370c08.
pub fn c_attached_dynamic_emitter_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x490000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7e18));
        let vtable = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0x1f8)).sw(c, vtable);
        call_ptr(c, 0x0037_0c08, 0x002f_17c4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cAttachedDynamicEmitter::Create(this, a1, a2)` (0x2f2130): finds the
/// first empty slot of the ten at `this+0x6c` (counting the used ones in
/// s1). In an empty slot it allocates a 0x210-byte emitter, constructs it
/// and wires it up (calls through its vtable); returns the slot count, or
/// -1 when all ten slots are full.
pub fn c_attached_dynamic_emitter_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x110;
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.mult(0, S3, A1, V0); // s3 = a1 * 0x110
        frame.save(c, S1, 0x30);
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c));
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        c.g[A1][0] = 0x480000;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 31;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7cb0));
                c.g[A0][0] = 0x210;
                call_ptr(c, 0x0031_7d70, 0x002f_2180)?;
                c.g[A0][0] = c.g[V0][0];
                Callee {
                    addr: 0x002f_1768,
                    f: ee::s09::f_002f1768__generated,
                }
                .call(c, 0x002f_2188)?;
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xac0)).lw(c);
                let slot = c.g[S0][0] as u32;
                let v = c.g[V0][0];
                Addr(slot).sw(c, v);
                let v = c.g[S2][0];
                Addr((c.g[V0][0] as u32).wrapping_add(0x200)).sw(c, v);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                c.g[V0][0] = Addr(slot).lw(c);
                Addr((c.g[V0][0] as u32).wrapping_add(0x204)).sw(c, 0);
                c.g[V1][0] = Addr(slot).lw(c);
                let v = c.g[A0][0];
                Addr((c.g[V1][0] as u32).wrapping_add(0x20c)).sw(c, v);
                c.g[A0][0] = Addr(slot).lw(c);
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1f8)).lw(c);
                c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x20c)).lw(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lh(c);
                c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x58));
                c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1c)).lw(c);
                let target = c.g[A2][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                call_ptr(c, target, 0x002f_21c8)?;
                c.g[A1][0] = Addr(slot).lw(c);
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x1f8)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x002f_21e0)?;
                c.g[V0][0] = c.g[S1][0];
                break;
            }
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 10) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x002f_2160);
                }
                continue;
            }
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            break;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine_rt;

    use crate::difftest::DATA;

    #[test]
    fn c_attached_dynamic_emitter_ctor_matches() {
        check(
            ee::s09::f_002f1768__generated,
            c_attached_dynamic_emitter_ctor,
            200,
            &|c, seed| {
                let _r = machine_rt(c, seed, 0x002f_1768, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn c_attached_dynamic_emitter_v04_matches() {
        check(
            ee::s09::f_002f17a8__generated,
            c_attached_dynamic_emitter_v04,
            200,
            &|c, seed| {
                let _r = machine_rt(c, seed, 0x002f_17a8, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn c_attached_dynamic_emitter_create_matches() {
        check(
            ee::s09::f_002f2130__generated,
            c_attached_dynamic_emitter_create,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x002f_2130, 0);
                let this = DATA + 0x1000;
                // The first `seed % 11` slots are used: 0..=10 passes.
                for i in 0..10u32 {
                    let slot = if i < seed % 11 {
                        DATA + 0x5000 + 0x40 * i
                    } else {
                        0
                    };
                    c.bus.write32(this + 0x6c + 4 * i, slot);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() & 0xff);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }
}
