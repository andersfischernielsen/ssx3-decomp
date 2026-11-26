//! Staging module oc795 (wave wave246): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc795`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003d_8278,
        spch_clear_event_tables,
        "SPCH_ClearEventTables",
    ),
    (0x0038_aef8, ps2_bezier_man_create, "PS2BezierMan_Create"),
    (0x002e_c418, lens_fx_ctor, "LensFX_ctor"),
    (0x002e_c3e0, lens_fx_create, "LensFX_Create"),
    (
        0x0028_9af0,
        ssx_audio_system_rider_init_stub,
        "SSXAudioSystem_RiderInitStub",
    ),
    (0x003d_66c0, spch_add_event_table, "SPCH_AddEventTable"),
];

/// `void SPCH_ClearEventTables(void)` (0x3d8278): zeroes the eight 8-byte
/// speech event-table slots at 0x44ffb8.
pub fn spch_clear_event_tables(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x7;
    c.g[V0][0] = sx(0x0045_0000u32.wrapping_sub(0x48));
    loop {
        c.cycles += 7;
        let slot = c.g[V0][0] as u32;
        c.sw(slot, 0u64);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        c.sw(slot.wrapping_add(0x4), 0u64);
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(slot.wrapping_add(0x8));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003d_8288);
            }
            continue;
        }
        c.cycles += 2;
        ret(c);
        return;
    }
}

/// `PS2BezierMan* PS2BezierMan_Create(void)` (0x38aef8): allocates 0x9c90
/// bytes (`EA_MemAlloc` named, name at 0x492ea0) and runs the constructor.
pub fn ps2_bezier_man_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx(0x0049_0000u32.wrapping_add(0x2ea0));
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x9c90;
        call_ptr(c, 0x0031_7d70, 0x0038_af18)?;
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x0038_ae60,
            f: ee::s12::PS2BezierMan_ctor,
        }
        .call(c, 0x0038_af20)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LensFX* LensFX_Create(void)` (0x2ec3e0): allocates 0x6820 bytes (named
/// at 0x4a3b48) and runs the constructor.
pub fn lens_fx_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx(0x004a_0000u32.wrapping_add(0x3b48));
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x6820;
        call_ptr(c, 0x0031_7d70, 0x002e_c400)?;
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x002e_c418,
            f: ee::s09::LensFX_ctor,
        }
        .call(c, 0x002e_c408)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LensFX* LensFX_ctor(LensFX* self)` (0x2ec418): runs the `LensFXBase`
/// constructor, then installs the LensFX vtable (0x487f00) at `+0x649c`.
/// Returns `self`.
pub fn lens_fx_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002e_2e18,
            f: ee::s09::LensFXBase_ctor,
        }
        .call(c, 0x002e_c42c)?;
        c.g[V1][0] = sx(0x0048_0000u32.wrapping_add(0x7f00));
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x649c), c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_RiderInitStub(void)` (0x289af0): empty.
pub fn ssx_audio_system_rider_init_stub(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `int SPCH_AddEventTable(unsigned short* bankData, unsigned bankKey)`
/// (0x3d66c0): `SPCH_RegisterEventTable(bankData, bankKey)` when `bankKey < 8`,
/// else 0.
pub fn spch_add_event_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0;
        let in_range = c.g[A1][0] < 0x8;
        c.g[V0][0] = in_range as u64;
        frame.save(c, RA, 0);
        if in_range {
            c.cycles += 3;
            Callee {
                addr: 0x003d_6498,
                f: ee::s14::SPCH_RegisterEventTable,
            }
            .call(c, 0x003d_66dc)?;
            c.g[V1][0] = c.g[V0][0];
        }
        c.cycles += 4;
        frame.restore(c, RA, 0);
        c.g[V0][0] = c.g[V1][0];
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the allocator fake (in each machine's own RAM).
    const OC795_CTRL: u32 = DATA + 0x20_0010;
    const OC795_MAGIC: u32 = 0x0795_f4c3;

    static OC795_PREV_ALLOC: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake of the allocator (`0x317d70`): logs the call like
    /// `fakes::fake_call`, may unwind, returns a pointer into the data area.
    fn oc795_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(OC795_CTRL) != OC795_MAGIC {
            return fakes::fall_back(c, &OC795_PREV_ALLOC, ee::s10::f_00317d70__generated);
        }
        fakes::fake_call(c, 0x0031_7d70);
    }

    fn oc795_hook() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc795_fake_alloc,
            &OC795_PREV_ALLOC,
        );
    }

    /// `SPCH_ClearEventTables`: the eight slots at 0x44ffb8, with events
    /// off, delivered and unwinding at the loop back edge.
    #[test]
    fn spch_clear_event_tables_matches() {
        check(
            ee::s14::f_003d8278__generated,
            spch_clear_event_tables,
            60,
            &|c, seed| {
                let mut r = Rng(0x7950_0001 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                for i in 0..0x44u32 {
                    c.bus.write32(0x0044_ffa0 + 4 * i, r.next_u32() | 1);
                }
                c.g[RA][0] = crate::difftest::RA as u64;
                schedule(c, &mut r, seed, 0x003d_8278);
            },
        );
    }

    /// `PS2BezierMan_Create`: the allocator is faked (and unwinds on some
    /// seeds); the constructor runs for real on the fixture.
    #[test]
    fn ps2_bezier_man_create_matches() {
        oc795_hook();
        fakes::covered(
            ee::s12::f_0038aef8__generated,
            ps2_bezier_man_create,
            30,
            &|c, seed| {
                let _ = machine(c, seed, 0x7950_0002, 1);
                c.bus.write32(OC795_CTRL, OC795_MAGIC);
            },
        );
    }

    /// `LensFX_Create`: as above, with the lens-flare constructor.
    #[test]
    fn lens_fx_create_matches() {
        oc795_hook();
        fakes::covered(
            ee::s09::f_002ec3e0__generated,
            lens_fx_create,
            30,
            &|c, seed| {
                let _ = machine(c, seed, 0x7950_0003, 1);
                c.bus.write32(OC795_CTRL, OC795_MAGIC);
            },
        );
    }

    /// `LensFX_ctor` on a zeroed object (the base constructor runs for real).
    #[test]
    fn lens_fx_ctor_matches() {
        check(
            ee::s09::f_002ec418__generated,
            lens_fx_ctor,
            20,
            &|c, seed| {
                let _ = machine(c, seed, 0x7950_0004, 0);
                c.g[A0][0] = sx(DATA + 0x1000 + 0x10 * (seed % 8));
            },
        );
    }

    #[test]
    fn ssx_audio_system_rider_init_stub_matches() {
        check(
            ee::s07::f_00289af0__generated,
            ssx_audio_system_rider_init_stub,
            20,
            &|c, seed| {
                let _ = machine(c, seed, 0x7950_0005, 0);
            },
        );
    }

    /// `SPCH_AddEventTable`: key below and not below 8; the registration
    /// itself (real callee) with and without the 0xc03 header.
    #[test]
    fn spch_add_event_table_matches() {
        check(
            ee::s14::f_003d66c0__generated,
            spch_add_event_table,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7950_0006, 0);
                let table = 0x0044_ffb8u32;
                let bank = DATA + 0x1000;
                c.bus
                    .write16(bank, if seed % 7 == 0 { 0x1234 } else { 0xc03 });
                c.bus.write8(bank + 8, (r.next_u32() % 2) as u8);
                c.bus.write8(bank + 9, (r.next_u32() % 2) as u8);
                for i in 0..8u32 {
                    let slot = if r.next_u32() % 3 != 0 {
                        DATA + 0x2000 + 0x400 * i
                    } else {
                        0
                    };
                    c.bus.write32(table + 8 * i, slot);
                    c.bus
                        .write32(table + 8 * i + 4, [0x11, 0x22][(r.next_u32() % 2) as usize]);
                    if slot != 0 {
                        c.bus.write8(slot + 8, (r.next_u32() % 2) as u8);
                        c.bus.write8(slot + 9, (r.next_u32() % 2) as u8);
                        c.bus.write16(slot + 0x10, (r.next_u32() % 6) as u16);
                        c.bus.write8(slot + 0x12, (r.next_u32() % 5) as u8);
                    }
                }
                c.g[A0][0] = sx(bank);
                c.g[A1][0] = match seed % 4 {
                    0 => 8 + (r.next_u32() % 100) as u64,
                    1 => 0xffff_ffff_0000_0003,
                    _ => [0x11u64, 0x22][(r.next_u32() % 2) as usize] % 8,
                };
            },
        );
    }
}
