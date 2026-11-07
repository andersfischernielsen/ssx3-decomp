//! `src/fe/ovstatemission.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::dirtysock::tags2::p;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_2;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_2;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
use crate::guest::Addr;
#[cfg(test)]
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001a_3258,
        cfe_state_mp_circuit_on_create_screen,
        "cFEStateMPCircuit_onCreateScreen",
    ),
    (
        0x001a_3308,
        c_fe_state_mp_circuit_on_activate,
        "cFEStateMPCircuit_OnActivate",
    ),
    (0x001a_3590, fe_async_manager_ctor, "FEAsyncManager_ctor"),
    (0x001a_35e8, fe_async_manager_dtor, "FEAsyncManager_dtor"),
    (0x001a_3648, feasync_manager_init, "FEAsyncManager_Init"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_POPUP_CONFIRM_EMPTY_INIT_HOOK;

    pub(crate) use crate::game::wscript::wscriptman::callee::EA_FILE_MOUNT_BIG;
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {

    pub(crate) use crate::game::wscript::wscriptman::table::BXFILE_EXISTS;
}

/// `void FEAsyncManager_Init(FEAsyncManager* m)` (0x1a3648): initialise the
/// five 0x11c-byte async-file slots at +0, +0x11c, ... +0x470: name byte from
/// `D_004A1408`, +0x100 = 0, +0x104 = +0x110 = +0x118 = -1, +0x108 = 1,
/// +0x10c = +0x114 = 0. 6 + 5×12 + 2 cycles.
pub fn feasync_manager_init(c: &mut Cpu) {
    c.cycles += 6;
    let first = r32(c, A0);
    let end = p(first, 0x58c);
    c.g[A1][0] = sx(end);
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    c.g[A2][0] = 1;
    c.g[V0][0] = Addr(0x004a_1408).lbu(c);
    let mut cur = first;
    loop {
        c.cycles += 12;
        c.sw(p(cur, 0x100), 0);
        Addr(cur).sb(c, c.g[V0][0]);
        c.sw(p(cur, 0x104), c.g[V1][0]);
        c.sw(p(cur, 0x108), c.g[A2][0]);
        c.sw(p(cur, 0x10c), 0);
        c.sw(p(cur, 0x110), c.g[V1][0]);
        c.sw(p(cur, 0x114), 0);
        c.sw(p(cur, 0x118), c.g[V1][0]);
        c.g[A0][0] = sx(cur.wrapping_add(0x11c));
        cur = r32(c, A0);
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(0x004a_1408).lbu(c);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x001a_3660);
            }
            continue;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
}

/// `void* FEAsyncManager_ctor(FEAsyncManager* self)` (0x1a3590): mount the
/// front-end's big file when it exists.
pub fn fe_async_manager_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x46_1418; // D_00461418
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[V0][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, table::BXFILE_EXISTS, 0x001a_35b4)?;
        let missing = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !missing {
            c.cycles += 4;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x64;
            c.g[A3][0] = sx(this.wrapping_add(0x58c));
            callee::EA_FILE_MOUNT_BIG.call(c, 0x001a_35cc)?;
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEAsyncManager_dtor(FEAsyncManager* self, int free)` (0x1a35e8):
/// release the async manager's job list and optionally delete it.
pub fn fe_async_manager_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let self_ = c.g[A0][0];
        c.g[S0][0] = self_;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x0046_1418;
        frame.save_ra(c, 0);
        let free = c.g[A1][0];
        c.g[S1][0] = free;
        call_ptr(c, 0x0031_7f38, 0x001a_360c)?;
        let absent = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !absent {
            c.cycles += 4;
            c.g[A0][0] = Addr(at(c, S0, 0x58c)).lw(c);
            c.g[A1][0] = 0x64;
            call_ptr(c, 0x003d_edc0, 0x001a_3620)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        'delete: {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x20);
                break 'delete;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x001a_3634)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end states.

/// `void cFEStateMPCircuit_OnActivate(cFEState*, void* arg)` (0x1a3308): find
/// the "circuit" widget on the state's screen, and if it is the selected list
/// box play its frame from the selected item's name; then run the empty hook
/// and the base `UIState_OnActivate`.
pub fn c_fe_state_mp_circuit_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x004a_0000;
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19e8));
        call_ptr(c, 0x0031_7670, 0x001a_332c)?; // StrHash
        c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001a_3338)?; // UIScreen_FindObject
        if c.g[V0][0] != 0 {
            c.cycles += 11;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0039_a708, 0x001a_3348)?; // UIListBox_GetSelectedData
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0031_7670, 0x001a_3350)?; // StrHash
            c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_c778, 0x001a_335c)?; // UIScreen_LookupValue
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = 0xffff;
            let no_frame = c.g[A1][0] == c.g[V0][0];
            c.g[A1][0] &= 0xffff;
            if !no_frame {
                c.cycles += 3;
                c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0x40));
                c.g[A2][0] = 1;
                call_ptr(c, 0x0039_c870, 0x001a_3378)?; // UIScreen_StartThread
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001a_34a8, 0x001a_3380)?; // cFEStateMPCircuit_EmptyHook
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e4c0, 0x001a_338c)?; // UIState_OnActivate
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// FE state machine: multiplayer circuit screen creation.

/// `void cFEStateMPCircuit_onCreateScreen(cFEStateMPCircuit* self)` (0x1a3258):
/// create the screen for the state's stored name hash, start its thread and
/// find the state object on it, clearing the "not ready" bit.
pub fn cfe_state_mp_circuit_on_create_screen(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        c.cycles += 16;
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x0046_13c8);
        c.g[RA][0] = sx(0x001a_327c);
        c.g[S0][0] = lw_2(c, at_2(c.g[S1][0], 0x10));
        call_ptr(c, 0x0031_7670, 0x001a_327c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001a_3290);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001a_3290)?;
        let screen = c.g[V0][0];
        Addr(at_2(c.g[S1][0], 0x40)).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001a_32a8);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001a_32a8)?;
        }
        c.cycles += 9;
        c.g[A0][0] = sx(0x004a_19e8);
        c.g[RA][0] = sx(0x001a_32b4);
        call_ptr(c, 0x0031_7670, 0x001a_32b4)?;
        c.g[A0][0] = lw_2(c, at_2(c.g[S1][0], 0x40));
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001a_32c0);
        call_ptr(c, 0x0039_d860, 0x001a_32c0)?;
        let obj = c.g[V0][0];
        let t = obj == 0;
        c.g[V1][0] = sx(0xffff_fffe);
        if !t {
            c.cycles += 7;
            c.g[V0][0] = lw_2(c, at_2(obj, 0x14));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = obj;
            c.g[V0][0] &= c.g[V1][0];
            c.g[V0][0] |= 0x80;
            c.g[RA][0] = sx(0x001a_32e8);
            Addr(at_2(obj, 0x14)).sw(c, c.g[V0][0]);
            callee::C_FE_POPUP_CONFIRM_EMPTY_INIT_HOOK.call(c, 0x001a_32e8)?;
        }
        c.cycles += 5;
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

    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_2;

    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;

    pub(crate) use crate::game::wscript::wscriptman::tests::w as w_2;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS;

    #[test]
    fn feasync_manager_init_matches() {
        check(
            ee::s03::f_001a3648__generated,
            feasync_manager_init,
            80,
            &|c, seed| {
                let mut r = Rng(0x1a36_48a1 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write8(0x004a_1408, (r.next_u32() % 5) as u8);
                fill(c, &mut r, DATA, 0x600 / 4);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn fe_async_manager_ctor_matches() {
        check(
            ee::s03::f_001a3590__generated,
            fe_async_manager_ctor,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1a35_90, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x800);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn fe_async_manager_dtor_matches() {
        check(
            ssx3_ee::s03::f_001a35e8__generated,
            fe_async_manager_dtor,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x1a35e8, 0);
                w(c, OBJ + 0x58c, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    pub(crate) fn str_hash(c: &mut Cpu, addr: u32) -> u64 {
        const RET: u32 = 0x0012_3450;
        c.g[A0][0] = sx(addr);
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7670;
        ssx3_ee_rt::funcs::call_indirect(c, 0x0031_7670);
        assert_eq!(c.pc, RET, "StrHash did not return");
        c.g[V0][0]
    }

    // --------------------------------------------------------------- sound

    // ------------------------------------------------------ front-end state

    #[test]
    fn c_fe_state_mp_circuit_on_activate_matches() {
        check(
            ssx3_ee::s03::f_001a3308__generated,
            c_fe_state_mp_circuit_on_activate,
            12,
            &|c, seed| {
                let (mut r, state) = (base(c, seed, 0x001a_3308), DATA + 0x100);
                clear_2(c, state, 0x100);
                let screen = DATA + 0x2000;
                let objects = DATA + 0x2100;
                let obj = DATA + 0x2200;
                clear_2(c, screen, 0x80);
                clear_2(c, objects, 0x20);
                clear_2(c, obj, 0x340);
                if seed % 3 != 0 {
                    w_2(c, state + 0x40, screen);
                    w_2(c, screen + 0x3c, 1);
                    w_2(c, screen + 0x38, objects);
                    w_2(c, objects, obj);
                    let hv = str_hash(c, 0x004a_19e8) as u32;
                    w_2(c, obj + 0x38, hv);
                }
                w_2(
                    c,
                    state + 0x18,
                    if seed % 6 == 0 { obj } else { r.next_u32() },
                );
                w_2(c, obj + 0x318, 0);
                w_2(c, obj + 0x319, r.next_u32() % 4);
                c.g[A0][0] = sx(state);
                c.g[A1][0] = sx(DATA + 0x9000);
            },
        );
    }

    // --------------------------------------------------------------- FE states

    #[test]
    fn cfe_state_mp_circuit_on_create_screen_matches() {
        check(
            ee::s03::f_001a3258__generated,
            cfe_state_mp_circuit_on_create_screen,
            SEEDS_2,
            &|c, seed| {
                let _ = base_2(c, seed, 0x1a32_58, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                // The engine the screen is created on, with an empty class
                // registry (so no class matches and no screen comes back).
                w_3(c, obj + 0x10, obj + 0x400);
                w_3(c, obj + 0x40, 0);
                if seed % 3 == 0 {
                    // A registry with one class entry.
                    w_3(c, obj + 0x404, obj + 0x500);
                    w_3(c, obj + 0x500 + 0xc, 0);
                }
            },
        );
    }
}
