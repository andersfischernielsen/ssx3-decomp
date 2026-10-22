//! `src/fe/feflowstate.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::p;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001a_0648, fe_flow_state_init, "FEFlowState_Init"),
    (0x001a_06f0, feflowstate_get_menu_selection, "FEFlowState_GetMenuSelection"),
    (0x001a_0708, feflowstate_set_menu_selection, "FEFlowState_SetMenuSelection"),
];


// ---------------------------------------------------------------------------
// Front-end flow state.

/// `int FEFlowState_GetMenuSelection(FEFlowState *self, int index)`
/// (0x1a06f0): signed byte from the selection array at `*(u32*)(self+4)`.
pub fn feflowstate_get_menu_selection(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
    abi::ret(c);
}


/// `void FEFlowState_SetMenuSelection(FEFlowState *self, int index, int value)`
/// (0x1a0708): stores the low byte of `value` into the selection array.
pub fn feflowstate_set_menu_selection(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[V0][0] as u32).sb(c, c.g[A2][0]);
    abi::ret(c);
}


/// `FEFlowState_Init(state, n)` 0x1a0648: free the previous list, remember
/// the sign-extended byte count, allocate the new list and clear it.
pub fn fe_flow_state_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x30);
        let n = (abi::arg(c, 1) as u8 as i8) as i32 as u32;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        c.g[S0][0] = sx(n);
        c.g[S1][0] = this64;
        frame.save_ra(c, 0);
        call_ptr(c, 0x001a_06b0, 0x001a_0668)?;
        Addr(this).sb(c, c.g[S0][0]);
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(0x0046_0b80);
        c.g[A2][0] = 0x100;
        call_ptr(c, 0x0031_7e30, 0x001a_0684)?;
        let list = c.g[V0][0];
        c.g[A2][0] = c.g[S0][0];
        Addr(p(this, 4)).sw(c, list);
        c.g[A0][0] = list;
        c.g[A1][0] = 0;
        call_ptr(c, 0x0041_6210, 0x001a_0698)?;
        frame.restore(c, S0, 0x20);
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
    use crate::difftest::fakes;
    use crate::difftest::GAME_GP;
    use ssx3_ee as ee;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_LIBRARY;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_6 as pool_allocator;


    #[test]
    fn feflowstate_get_menu_selection_matches() {
        check(
            ssx3_ee::s03::f_001a06f0,
            feflowstate_get_menu_selection,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0007 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = seed as u64;
                c.bus.write32(DATA + 0x4, (DATA + 0x100) as u32);
                for i in 0..0x40 {
                    c.bus.write8(DATA + 0x80 + i, r.next_u32() as u8);
                }
            },
        );
    }


    #[test]
    fn feflowstate_set_menu_selection_matches() {
        check(
            ssx3_ee::s03::f_001a0708,
            feflowstate_set_menu_selection,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0008 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(DATA + 0x4, (DATA + 0x100) as u32);
            },
        );
    }


    #[test]
    fn fe_flow_state_init_matches() {
        check(ee::s03::f_001a0648__generated, fe_flow_state_init, 30, &|c, seed| {
            fakes::machine(c, seed, 0x1a0648, 0);
            globals(c);
            pool_allocator(c);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { 0x24 };
            w(c, OBJ + 4, if seed % 3 == 0 { 0 } else { OBJ2 });
        });
    }
}
