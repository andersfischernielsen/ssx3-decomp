//! `src/path/pathsys.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::Flow;
use crate::guest::Addr;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
pub(crate) use crate::game::dirtysock::tags2::at;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_b598, path_data_has_marker, "PathData_HasMarker"),
    (0x0026_b5e0, path_data_find_marker, "PathData_FindMarker"),
    (0x0026_b7d8, path_data_read_float_int_pairs, "PathData_ReadFloatIntPairs"),
];


/// `bool PathData_HasMarker(PathData* self, int a /*a1*/, int b /*a2*/)`
/// (0x26b598): true when one of the `count` markers (0x28 bytes each, a
/// 16-byte head at +4) matches `(b, a)`.
pub fn path_data_has_marker(c: &mut Cpu) {
    c.cycles += 3;
    let count = c.lw(r32(c, A0));
    c.g[A3][0] = count;
    let none = (count as i64) <= 0;
    c.g[V1][0] = 0;
    if !none {
        c.cycles += 1;
        c.g[A0][0] = c.lw(at(c, A0, 0x04));
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, A0, 0x04));
            let mismatch = c.g[V0][0] != c.g[A1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if mismatch {
                c.cycles += 1;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, A0));
                let mismatch = c.g[V0][0] != c.g[A2][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                if !mismatch {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    abi::ret(c);
                    return;
                }
            }
            c.cycles += 2;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0026_b5a8);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}


/// `PathMarker* PathData_FindMarker(PathData* self, int type, int id)`
/// (0x26b5e0): linear search of the 0x28-byte marker array; then the first
/// marker when there is one, else null.
pub fn path_data_find_marker(c: &mut Cpu) {
    c.cycles += 3;
    let list = c.g[A0][0];
    let ty = c.g[A1][0];
    let id = c.g[A2][0];
    c.g[V0][0] = lw(c, list as u32);
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[T0][0] = 0;
    let mut found: Option<u64> = None;
    if !empty {
        c.cycles += 3;
        let mut entry = lw(c, p(list as u32, 4));
        c.g[A3][0] = entry;
        let count = c.g[V0][0];
        c.g[T1][0] = count;
        loop {
            c.cycles += 4;
            c.g[V1][0] = entry;
            c.g[V0][0] = lw(c, p(entry as u32, 4));
            let next = c.g[V0][0] != ty;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
            if !next {
                c.cycles += 3;
                c.g[V0][0] = lw(c, entry as u32);
                let hit = c.g[V0][0] == id;
                if hit {
                    c.cycles += 2;
                    found = Some(entry);
                }
            }
            if found.is_some() {
                c.g[V0][0] = found.unwrap();
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[T1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            entry = sx((entry as u32).wrapping_add(0x28));
            c.g[A3][0] = entry;
            if !more {
                c.cycles += 1;
                c.g[V0][0] = lw(c, list as u32);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0026_b5f8);
            }
        }
    }
    c.cycles += 2;
    let nonempty = (c.g[V0][0] as i64) > 0;
    c.g[V0][0] = 0;
    c.cycles += 2;
    if nonempty {
        c.g[V0][0] = lw(c, p(list as u32, 4));
    }
    abi::ret(c);
}


/// `int PathData_ReadFloatIntPairs(pairs, p)` (0x26b7d8): read a count and
/// then that many (float, int) pairs, returning the bytes consumed.
pub fn path_data_read_float_int_pairs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S2, 0x40);
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, 0x10);
        c.g[S4][0] = c.g[S0][0];
        call_ptr(c, 0x0026_ba68, 0x0026b808)?;
        c.g[S0][0] = sx(p(r32(c, S0), 4));
        c.g[V0][0] = Addr(r32(c, SP)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[S2][0] = sx(p(r32(c, S1), 0x18));
            c.g[S1][0] = sx(p(r32(c, S1), 0x1c));
            loop {
                c.cycles += 14;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ba88, 0x0026b82c)?;
                c.g[S0][0] = sx(p(r32(c, S0), 4));
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ba68, 0x0026b83c)?;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                c.g[V0][0] = Addr(r32(c, SP)).lw(c);
                c.g[S0][0] = sx(p(r32(c, S0), 4));
                c.g[S1][0] = sx(p(r32(c, S1), 8));
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx(p(r32(c, S2), 8));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_b820);
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = sx(r32(c, S0).wrapping_sub(r32(c, S4)));
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::vtable;
    use crate::difftest::Rng;
    use crate::difftest::fill;
    use ssx3_ee as ee;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::be::belibrary::tests::SRC;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base;


    #[test]
    fn path_data_has_marker_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x26b598, 0);
            be_library(c);
            let path = DATA + 0x100;
            let count: u32 = match seed % 5 {
                0 => 0,
                1 => 0x8000_0000,
                _ => 5,
            };
            c.bus.write32(path, count);
            c.bus.write32(path + 4, DATA + 0x1000);
            for i in 0..5u32 {
                let e = DATA + 0x1000 + 0x28 * i;
                c.bus.write32(e, i);
                c.bus.write32(e + 4, 100 + i);
            }
            // Match the first marker, the third, or none.
            let (a, b): (u64, u64) = match seed % 3 {
                0 => (100, 0),
                1 => (102, 2),
                _ => (999, 9),
            };
            c.g[A0][0] = sx(path);
            c.g[A1][0] = a;
            c.g[A2][0] = b;
            let mode = schedule(c, &mut r, seed, 0x0026_b598);
            if mode != Events::Off {
                // The walk is at most five entries: make the event due at the
                // first back edge so the event path is really covered.
                c.next_event = c.cycles + (r.next_u32() % 4) as u64;
            }
            mode
        };
        check_events(ssx3_ee::s07::f_0026b598__generated, path_data_has_marker, 80, &setup);
    }


    #[test]
    fn path_data_find_marker_matches() {
        check(
            ssx3_ee::s07::f_0026b5e0__generated,
            path_data_find_marker,
            150,
            &|c, seed| {
                let mut r = setup(c, seed, 0x26b5e0);
                c.g[A0][0] = DATA as u64;
                let n = seed % 4;
                c.bus.write32(DATA, n);
                c.bus.write32(DATA + 4, DATA + 0x200);
                let ty: u32 = if seed % 2 == 0 { 7 } else { 9 };
                let id = r.next_u32();
                c.g[A1][0] = ty as u64;
                c.g[A2][0] = id as u64;
                for i in 0..n {
                    let e = DATA + 0x200 + i * 0x28;
                    let hit = seed % 3 == 0 && i == n - 1;
                    c.bus.write32(e, if hit { id } else { r.next_u32() });
                    c.bus.write32(e + 4, if hit { ty } else { r.next_u32() });
                    c.bus.write32(e + 8, r.next_u32());
                }
            },
        );
    }


    #[test]
    fn path_data_read_float_int_pairs_matches() {
        check(
            ee::s07::f_0026b7d8__generated,
            path_data_read_float_int_pairs,
            6,
            &|c, seed| {
                let mut r = base(c, seed, 0x26b7d8, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = SRC as u64;
                w(c, SRC, (seed % 4) as u32);
                fill(c, &mut r, SRC + 4, 16);
                w(c, OBJ + 0x18, 0);
                w(c, OBJ + 0x1c, 0);
                w(c, OBJ + 0x20, 0);
                w(c, OBJ + 0x24, 0);
            },
        );
    }
}
