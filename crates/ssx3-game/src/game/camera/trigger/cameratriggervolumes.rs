//! `src/camera/trigger/cameratriggervolumes.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call;
use crate::call::call_ptr;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
pub(crate) use crate::game::be::bedbreward::BUILTIN_DELETE_ADDR;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0016_c5d0, active_camera_trigger_list_ctor, "ActiveCameraTriggerList_ctor"),
    (0x0016_c5e0, active_camera_trigger_list_dtor, "ActiveCameraTriggerList_dtor"),
];


/// `ActiveCameraTriggerList* ActiveCameraTriggerList_ctor(ActiveCameraTriggerList *self)`
/// (0x16c5d0): clear the head and return `self`.
pub fn active_camera_trigger_list_ctor(c: &mut Cpu) {
    c.cycles += 3;
    let a0 = abi::arg64(c, 0);
    c.g[V0][0] = a0;
    Addr(a0 as u32).sw(c, 0);
    abi::ret(c);
}

pub(crate) const ACTIVE_CAMERA_TRIGGER_LIST_CLEAR: Callee =
    Callee { addr: 0x0016_c628, f: ee::s02::f_0016c628 };


/// `void ActiveCameraTriggerList_dtor(ActiveCameraTriggerList *list, uint flags)`
/// (0x16c5e0): clear the list, then delete it when `flags & 1`.
pub fn active_camera_trigger_list_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        ACTIVE_CAMERA_TRIGGER_LIST_CLEAR.call(c, 0x0016_c5fc)?;
        c.g[S0][0] &= 1;
        let delete_list = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if delete_list {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, BUILTIN_DELETE_ADDR, 0x0016_c610)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    pub(crate) use crate::game::ai::aianimeventmap::tests::MEM_MUTEX;
    pub(crate) use crate::game::ai::aianimeventmap::tests::alloc_setup;


    #[test]
    fn active_camera_trigger_list_ctor_matches() {
        check(ssx3_ee::s02::f_0016c5d0, active_camera_trigger_list_ctor, 64, &|c, seed| {
            let mut r = Rng(0x16c5 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
        });
    }


    // ---------------------------------------------- active camera trigger list

    /// Build a linked list of `nodes` 0x10-byte nodes at DATA+0x1000 and store
    /// its head at `list`; ids are small so `Find`/`Remove` can match them.
    pub(crate) fn build_list(c: &mut Cpu, r: &mut Rng, list: u32, nodes: u32) -> u32 {
        if nodes == 0 {
            c.bus.write32(list, 0);
            return 0;
        }
        let base = DATA + 0x1000;
        for i in 0..nodes {
            let next = if i + 1 < nodes { base + (i + 1) * 0x10 } else { 0 };
            c.bus.write32(base + i * 0x10, r.next_u32() % 8);
            c.bus.write32(base + i * 0x10 + 4, next);
            c.bus.write32(base + i * 0x10 + 8, r.next_u32());
            c.bus.write32(base + i * 0x10 + 0xc, r.next_u32());
        }
        c.bus.write32(list, base);
        base
    }


    #[test]
    fn active_camera_trigger_list_dtor_matches() {
        check(ee::s02::f_0016c5e0, active_camera_trigger_list_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xc5e0, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 3) as u64;
            build_list(c, &mut r, DATA, seed % 2);
        });
    }
}
