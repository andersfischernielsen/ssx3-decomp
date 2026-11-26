//! Staging oc1343 (wave rtti): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1343`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call;
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::lib::gcc::{frame_ld, frame_sd};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0042_e200,
        tf23ps2_sony_codec_internal,
        "__tf23PS2_SONY_CODEC_INTERNAL",
    ),
    (
        0x0042_e4c8,
        tfq24rcmp11rcmp_system,
        "__tfQ24RCMP11RCMP_SYSTEM",
    ),
    (0x0042_e458, tfq24rcmp7decoder, "__tfQ24RCMP7DECODER"),
    (0x0042_e2b0, tfq24rcmp5codec, "__tfQ24RCMP5CODEC"),
    (0x0041_5120, bad_cast_dtor, "bad_cast_dtor"),
    (0x0041_5088, bad_typeid_dtor, "bad_typeid_dtor"),
    (0x0041_5150, bad_cast_ctor, "bad_cast_ctor"),
    (0x0041_50b8, bad_typeid_ctor, "bad_typeid_ctor"),
];

// ---------------------------------------------------------------------------
// Callees (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::Callee;

    /// Fill a single-inheritance type_info (`__rtti_si`, 0x414c00).
    pub(crate) use crate::game::lib::gcc::RTTI_SI;
    /// Fill a user type_info's vtable slot (`__rtti_user`, 0x414c20).
    pub(crate) use crate::game::lib::gcc::RTTI_USER;
}

// ---------------------------------------------------------------------------
// bad_typeid_dtor / bad_cast_dtor

/// `bad_typeid::~bad_typeid(int)` (0x415088): install the vtable, then free
/// the object when the low bit of `a1` says it owns it.
pub fn bad_typeid_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = 0x4a0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6748));
        frame.save_ra(c, 0);
        c.g[A1][0] &= 0x1;
        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0041_50ac);
            call::call_ptr(c, 0x0031_7e50, 0x0041_50ac)?;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

/// `bad_cast::~bad_cast(int)` (0x415120): install the vtable, then free
/// the object when the low bit of `a1` says it owns it.
pub fn bad_cast_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = 0x4a0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6748));
        frame.save_ra(c, 0);
        c.g[A1][0] &= 0x1;
        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0041_5144);
            call::call_ptr(c, 0x0031_7e50, 0x0041_5144)?;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// bad_typeid_ctor / bad_cast_ctor

/// `bad_typeid::bad_typeid(void)` (0x4150b8): install the vtable.
pub fn bad_typeid_ctor(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0x4a0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x65e8));
    let t = c.g[RA][0] as u32;
    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
    c.pc = t;
}

/// `bad_cast::bad_cast(void)` (0x415150): install the vtable.
pub fn bad_cast_ctor(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0x4a0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x65d0));
    let t = c.g[RA][0] as u32;
    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
    c.pc = t;
}

// ---------------------------------------------------------------------------
// RCMP type_info getters

/// Shared body of the single-base RCMP type_info getters: return the
/// registry slot when something installed it, else build it with
/// `__rtti_user` (`slot` and `name` are the offsets from `0x4a0000` and
/// `0x490000`; `ret` is the `__rtti_user` return address).
fn tf_single_base(c: &mut Cpu, slot_off: u32, name_off: u32, ret: u32) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x20);
    c.g[V1][0] = 0x4a0000;
    frame.save(c, S0, 0x10);
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(slot_off));
    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(slot_off));
    frame.save_ra(c, 0);
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.g[A1][0] = 0x490000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(name_off));
        callee::RTTI_USER.call(c, ret)?;
    }
    c.cycles += 5;
    c.g[V0][0] = c.g[S0][0];
    c.g[RA][0] = c.ld(frame.at(c, 0));
    c.g[S0][0] = frame_ld(c, frame, 0x10);
    let t = c.g[RA][0] as u32;
    frame.close(c);
    c.pc = t;
    Ok(())
}

/// `const type_info& __tfQ24RCMP5CODEC(void)` (0x42e2b0): `RCMP::CODEC`'s
/// type_info (built on first use).
pub fn tfq24rcmp5codec(c: &mut Cpu) {
    call::guest(c, |c| tf_single_base(c, 0x5ba0, 0x5410, 0x0042_e2dc))
}

/// `const type_info& __tfQ24RCMP7DECODER(void)` (0x42e458):
/// `RCMP::DECODER`'s type_info (built on first use).
pub fn tfq24rcmp7decoder(c: &mut Cpu) {
    call::guest(c, |c| tf_single_base(c, 0x5b90, 0x53d8, 0x0042_e484))
}

/// `const type_info& __tfQ24RCMP11RCMP_SYSTEM(void)` (0x42e4c8):
/// `RCMP::RCMP_SYSTEM`'s type_info (built on first use).
pub fn tfq24rcmp11rcmp_system(c: &mut Cpu) {
    call::guest(c, |c| tf_single_base(c, 0x5b98, 0x53c0, 0x0042_e4f4))
}

/// `const type_info& __tf23PS2_SONY_CODEC_INTERNAL(void)` (0x42e200):
/// `PS2_SONY_CODEC_INTERNAL`'s type_info (built on first use, on top of
/// `RCMP::CODEC`'s user type_info).
pub fn tf23ps2_sony_codec_internal(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x540000;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1600));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x1600));
        c.g[S2][0] = c.g[V0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        if c.g[V1][0] == 0 {
            c.cycles += 4;
            c.g[V1][0] = 0x4a0000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5ba0));
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5ba0));
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[A1][0] = 0x490000;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5410));
                callee::RTTI_USER.call(c, 0x0042_e248)?;
            }
            c.cycles += 5;
            c.g[A1][0] = 0x490000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5420));
            c.g[A2][0] = c.g[S0][0];
            callee::RTTI_SI.call(c, 0x0042_e25c)?;
        }
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1600));
        c.g[S0][0] = frame_ld(c, frame, 0x30);
        c.g[S1][0] = frame_ld(c, frame, 0x20);
        c.g[S2][0] = frame_ld(c, frame, 0x10);
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_paths;
    use crate::difftest::fakes::machine;
    use crate::difftest::Rng;
    use crate::game::lib::gcc::tests::heap_obj;

    /// Seeds for the RTTI fixtures.
    pub(crate) const SEEDS: u32 = 64;

    /// Fixture base: scrambled machine; `a0`/`a1` from independent digits.
    fn rtti_machine(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = machine(c, seed, 0x4343, 0);
        c.g[A0][0] = (crate::difftest::DATA + 0x400 + (r.next_u32() % 0x100)) as u64;
        c.g[A1][0] = (r.next_u32() % 4) as u64;
        // Non-zero high halves (checklist (a)): slot-vs-neighbor slips show.
        c.g[A0][1] = 0x5a5a_5a5a;
        r
    }

    #[test]
    fn bad_typeid_dtor_matches() {
        check_paths(
            ee::s15::f_00415088,
            bad_typeid_dtor,
            SEEDS,
            &[10, 156, 159, 168],
            &|c, seed| {
                rtti_machine(c, seed);
                c.g[A0][0] = heap_obj(c) as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn bad_cast_dtor_matches() {
        check_paths(
            ee::s15::f_00415120,
            bad_cast_dtor,
            SEEDS,
            &[10, 156, 159, 168],
            &|c, seed| {
                rtti_machine(c, seed);
                c.g[A0][0] = heap_obj(c) as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn bad_typeid_ctor_matches() {
        check_paths(
            ee::s15::f_004150b8,
            bad_typeid_ctor,
            SEEDS,
            &[5],
            &|c, seed| {
                rtti_machine(c, seed);
            },
        );
    }

    #[test]
    fn bad_cast_ctor_matches() {
        check_paths(
            ee::s15::f_00415150,
            bad_cast_ctor,
            SEEDS,
            &[5],
            &|c, seed| {
                rtti_machine(c, seed);
            },
        );
    }

    #[test]
    fn tfq24rcmp5codec_matches() {
        check_paths(
            ee::s15::f_0042e2b0,
            tfq24rcmp5codec,
            SEEDS,
            &[12, 23],
            &|c, seed| {
                rtti_machine(c, seed);
                // Registry slot installed on even seeds (skip the build).
                c.bus.write32(0x004a_5ba0, (seed % 2) as u32);
            },
        );
    }

    #[test]
    fn tfq24rcmp7decoder_matches() {
        check_paths(
            ee::s15::f_0042e458,
            tfq24rcmp7decoder,
            SEEDS,
            &[12, 23],
            &|c, seed| {
                rtti_machine(c, seed);
                c.bus.write32(0x004a_5b90, (seed % 2) as u32);
            },
        );
    }

    #[test]
    fn tfq24rcmp11rcmp_system_matches() {
        check_paths(
            ee::s15::f_0042e4c8,
            tfq24rcmp11rcmp_system,
            SEEDS,
            &[12, 23],
            &|c, seed| {
                rtti_machine(c, seed);
                c.bus.write32(0x004a_5b98, (seed % 2) as u32);
            },
        );
    }

    #[test]
    fn tf23ps2_sony_codec_internal_matches() {
        check_paths(
            ee::s15::f_0042e200,
            tf23ps2_sony_codec_internal,
            SEEDS,
            &[17, 34, 45],
            &|c, seed| {
                rtti_machine(c, seed);
                // Top slot installed every fourth seed (skip everything);
                // the user slot installed on the next (skip the user
                // build); both clear runs the full build.
                c.bus.write32(0x0053_ea00, (seed % 4 == 0) as u32);
                c.bus.write32(0x004a_5ba0, (seed % 4 == 1) as u32);
            },
        );
    }
}
