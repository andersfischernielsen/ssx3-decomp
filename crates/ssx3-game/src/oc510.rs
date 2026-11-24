//! Staging module oc510 (wave wave229): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc510`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001d_e378, c_keyboard_popup_set_key_1c_enabled, "cKeyboardPopup_SetKey1CEnabled"),
    (0x001d_e3a0, c_keyboard_popup_set_key_45_enabled, "cKeyboardPopup_SetKey45Enabled"),
    (0x001d_e3c8, c_keyboard_popup_set_key_4f_enabled, "cKeyboardPopup_SetKey4FEnabled"),
    (0x001d_e3f0, c_keyboard_popup_set_key_29_enabled, "cKeyboardPopup_SetKey29Enabled"),
    (0x001d_e418, c_keyboard_popup_set_key_42_enabled, "cKeyboardPopup_SetKey42Enabled"),
    (0x001d_e4b0, c_fe_options_credits_ctor_2, "cFEOptionsCredits_ctor_2"),
    (0x001d_e4e8, c_fe_state_event_select_ctor_2, "cFEStateEventSelect_ctor_2"),
    (0x001d_e590, c_fe_state_background_ctor_3, "cFEStateBackground_ctor_3"),
    (0x001d_e628, c_fe_state_transition_ctor_3, "cFEStateTransition_ctor_3"),
    (0x001d_e690, c_fe_state_title_ctor_2, "cFEStateTitle_ctor_2"),
    (0x001d_e6f0, c_fe_state_main_menu_ctor_2, "cFEStateMainMenu_ctor_2"),
    (0x001d_e750, c_fe_state_bonus_material_ctor_2, "cFEStateBonusMaterial_ctor_2"),
    (0x001d_e788, c_fe_state_bonus_material_dtor, "cFEStateBonusMaterial_dtor"),
    (0x001d_e7b0, net_fe_state_ctor_2, "NetFEState_ctor_2"),
    (0x001d_e8c8, fe_screen_blank_ctor_2, "FEScreenBlank_ctor_2"),
    (0x001d_e918, fe_screen_blank_dtor, "FEScreenBlank_dtor"),
    (0x001d_e950, c_fe_state_mem_card_ctor_2, "cFEStateMemCard_ctor_2"),
    (0x001d_e990, fe_screen_autosave_popup_ctor_2, "FEScreenAutosavePopup_ctor_2"),
    (0x001d_e9c8, fe_screen_autosave_popup_dtor, "FEScreenAutosavePopup_dtor"),
    (0x001d_e9f8, c_fe_popup_option_reset_dtor, "cFEPopupOptionReset_dtor"),
];

/// `cKeyboardPopup_SetKeyEnabled(this, index, value, key)` (0x1ce3c8), the
/// callee of the five `SetKeyNNEnabled` wrappers below.
const FE_KEYBOARD_SET_KEY_ENABLED: Callee = Callee {
    addr: 0x001c_e3c8,
    f: ee::s04::cKeyboardPopup_SetKeyEnabled,
};

/// `MemCardStateBase_ctor` (0x1d53b0): the base constructor of the memory card
/// state, called directly by `cFEStateMemCard_ctor_2`.
const FE_MEMCARD_STATE_BASE_CTOR: Callee = Callee {
    addr: 0x001d_53b0,
    f: ee::s04::MemCardStateBase_ctor,
};

/// `UIState_ctor` (0x39e2a0), reached through the function table.
const FE_UI_STATE_CTOR: u32 = 0x0039_e2a0;

/// `UIState_dtor` (0x39e390), reached through the function table.
const FE_UI_STATE_DTOR: u32 = 0x0039_e390;

/// Calls a target (direct or through the function table) returning to `ret`.
fn call_fe_target(c: &mut Cpu, target: Target, ret: u32) -> Flow {
    match target {
        Target::Direct(f) => f.call(c, ret),
        Target::Table(addr) => call_ptr(c, addr, ret),
    }
}

/// The keyboard-popup wrappers (`cKeyboardPopup_setIncludeKey` family): a
/// 0x10 frame holding `ra`, `a2 = a1` (the caller's flag), `a1 = index`,
/// `a3 = 2` (the key), then `cKeyboardPopup_SetKeyEnabled` returning to `ret`.
fn keyboard_popup_set_key_wrapper(c: &mut Cpu, index: u64, ret: u32) -> Flow {
    c.cycles += 9;
    let frame = Frame::open(c, 0x10);
    c.g[A2][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[A1][0] = index;
    c.g[A3][0] = 2;
    FE_KEYBOARD_SET_KEY_ENABLED.call(c, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cKeyboardPopup_SetKey1CEnabled(this, flag)` (0x1de378): key 0x1c (the
/// "include tab" key) enabled from `flag`.
pub fn c_keyboard_popup_set_key_1c_enabled(c: &mut Cpu) {
    guest(c, |c| keyboard_popup_set_key_wrapper(c, 0x1c, 0x001d_e390));
}

/// `cKeyboardPopup_SetKey45Enabled(this, flag)` (0x1de3a0).
pub fn c_keyboard_popup_set_key_45_enabled(c: &mut Cpu) {
    guest(c, |c| keyboard_popup_set_key_wrapper(c, 0x45, 0x001d_e3b8));
}

/// `cKeyboardPopup_SetKey4FEnabled(this, flag)` (0x1de3c8).
pub fn c_keyboard_popup_set_key_4f_enabled(c: &mut Cpu) {
    guest(c, |c| keyboard_popup_set_key_wrapper(c, 0x4f, 0x001d_e3e0));
}

/// `cKeyboardPopup_SetKey29Enabled(this, flag)` (0x1de3f0).
pub fn c_keyboard_popup_set_key_29_enabled(c: &mut Cpu) {
    guest(c, |c| keyboard_popup_set_key_wrapper(c, 0x29, 0x001d_e408));
}

/// `cKeyboardPopup_SetKey42Enabled(this, flag)` (0x1de418).
pub fn c_keyboard_popup_set_key_42_enabled(c: &mut Cpu) {
    guest(c, |c| keyboard_popup_set_key_wrapper(c, 0x42, 0x001d_e430));
}

/// The shape of a `cFE*` constructor: `UIState_ctor`-style base construction
/// (optionally zeroing `a1`/`a2` for it), then the vtable `0x470000 - off`
/// stored at `this + 8`. The base constructor is `callee`, returning to `ret`.
fn fe_ctor_with_vtable(
    c: &mut Cpu,
    cycles: u64,
    zero_a1: bool,
    zero_a2: bool,
    callee: Target,
    ret: u32,
    vtable_off: u32,
) -> Flow {
    c.cycles += cycles;
    let frame = Frame::open(c, 0x20);
    if zero_a1 {
        c.g[A1][0] = 0;
    }
    if zero_a2 {
        c.g[A2][0] = 0;
    }
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_fe_target(c, callee, ret)?;
    c.g[V1][0] = 0x47_0000;
    c.g[V0][0] = c.g[S0][0];
    c.g[V1][0] = sx(0x47_0000u32.wrapping_sub(vtable_off));
    frame.restore_ra(c, 0);
    let vtable = c.g[V1][0];
    Addr(r32(c, S0).wrapping_add(8)).sw(c, vtable);
    frame.restore(c, S0, 0x10);
    frame.close(c);
    Ok(())
}

/// The `cFE*` destructor shape: store the vtable `0x470000 - off` at
/// `this + 8` (`this` is `a0`), then tail through `callee` returning to `ret`.
fn fe_dtor_with_vtable(
    c: &mut Cpu,
    cycles: u64,
    callee: Target,
    vtable_off: u32,
    ret: u32,
) -> Flow {
    c.cycles += cycles;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 0x47_0000;
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = sx(0x47_0000u32.wrapping_sub(vtable_off));
    let vtable = c.g[V0][0];
    Addr(r32(c, V1).wrapping_add(8)).sw(c, vtable);
    call_fe_target(c, callee, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFEOptionsCredits` constructor (0x1de4b0): `UIState_ctor(this, 0)`, then
/// the vtable at `+8`.
pub fn c_fe_options_credits_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(FE_UI_STATE_CTOR),
            0x001d_e4c8,
            0x4208,
        )
    });
}

/// `cFEStateEventSelect` constructor (0x1de4e8): `UIState_ctor(this, 0)`, then
/// the vtable at `+8`, then `EventSelect_ctor(this + 0x48)` (the list inside),
/// returning `this`.
pub fn c_fe_state_event_select_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, FE_UI_STATE_CTOR, 0x001d_e500)?;
        c.g[V1][0] = 0x47_0000;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x48));
        c.g[V1][0] = sx(0x47_0000u32.wrapping_sub(0x42d8));
        let vtable = c.g[V1][0];
        Addr(r32(c, S0).wrapping_add(8)).sw(c, vtable);
        call_ptr(c, 0x0020_06b8, 0x001d_e514)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateBackground` constructor (0x1de590): `UIState_ctor` with the
/// caller's `a1`, then the vtable at `+8`.
pub fn c_fe_state_background_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            13,
            false,
            false,
            Target::Table(FE_UI_STATE_CTOR),
            0x001d_e5a4,
            0x43a8,
        )
    });
}

/// `cFEStateTransition` constructor (0x1de628): `UIState_ctor` with the
/// caller's `a1`, then the vtable at `+8` and zeroed words at `+0x48`/`+0x4c`.
pub fn c_fe_state_transition_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, FE_UI_STATE_CTOR, 0x001d_e63c)?;
        c.g[V1][0] = 0x47_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(0x47_0000u32.wrapping_sub(0x4478));
        let s0 = r32(c, S0);
        Addr(s0.wrapping_add(0x48)).sw(c, 0);
        let vtable = c.g[V1][0];
        Addr(s0.wrapping_add(8)).sw(c, vtable);
        Addr(s0.wrapping_add(0x4c)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateTitle` constructor (0x1de690): `UIState_ctor(this, 0)`, vtable at `+8`.
pub fn c_fe_state_title_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(FE_UI_STATE_CTOR),
            0x001d_e6a8,
            0x4548,
        )
    });
}

/// `cFEStateMainMenu` constructor (0x1de6f0): `UIState_ctor(this, 0)`, vtable at `+8`.
pub fn c_fe_state_main_menu_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(FE_UI_STATE_CTOR),
            0x001d_e708,
            0x4618,
        )
    });
}

/// `cFEStateBonusMaterial` constructor (0x1de750): `UIState_ctor(this, 0)`,
/// vtable at `+8`.
pub fn c_fe_state_bonus_material_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(FE_UI_STATE_CTOR),
            0x001d_e768,
            0x46e8,
        )
    });
}

/// `cFEStateBonusMaterial` destructor (0x1de788): vtable at `+8`, then
/// `UIState_dtor(this)`.
pub fn c_fe_state_bonus_material_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_dtor_with_vtable(
            c,
            10,
            Target::Table(FE_UI_STATE_DTOR),
            0x2f30,
            0x001d_e7a4,
        )
    });
}

/// `NetFEState` constructor (0x1de7b0, `cFEStateNet1`): `UIState_ctor_2`, then
/// the vtable and the list/flag fields at `+0x48..+0x6bc`, a string built at
/// `+0x6c0` and a 12-byte block allocated for `+0x6cc`, whose pointer is
/// also stored at the block's `+4` (through `S1 = this + 0x6c8`).
pub fn net_fe_state_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x30);
        c.g[A1][0] = 0;
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e318, 0x001d_e7d0)?;
        let s0 = r32(c, S0);
        c.g[S1][0] = sx(s0.wrapping_add(0x6c8));
        c.g[V0][0] = 0x47_0000;
        c.g[V1][0] = 2;
        Addr(s0.wrapping_add(0x48)).sw(c, 0);
        c.g[V0][0] = sx(0x47_0000u32.wrapping_sub(0x47f8));
        Addr(s0.wrapping_add(0x58)).sw(c, 0);
        c.g[A1][0] = 0x4a_0000;
        Addr(s0.wrapping_add(0x69c)).sw(c, 0);
        c.g[A1][0] = sx(0x4a_0000u32.wrapping_add(0x1390));
        Addr(s0.wrapping_add(0x6a4)).sw(c, 0);
        c.g[A0][0] = sx(s0.wrapping_add(0x6c0));
        Addr(s0.wrapping_add(0x6ac)).sw(c, 0);
        Addr(s0.wrapping_add(0x6b0)).sw(c, 0);
        Addr(s0.wrapping_add(0x6b8)).sw(c, 0);
        let two = c.g[V1][0];
        Addr(s0.wrapping_add(0x6bc)).sw(c, two);
        let vtable = c.g[V0][0];
        Addr(s0.wrapping_add(8)).sw(c, vtable);
        call_ptr(c, 0x0031_82d8, 0x001d_e814)?;
        c.g[A1][0] = 0x46_0000;
        c.g[A0][0] = 0xc;
        c.g[A1][0] = sx(0x46_0000u32.wrapping_sub(0x3218));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x001d_e82c)?;
        c.g[A1][0] = c.g[V0][0];
        frame.restore_ra(c, 0x10);
        let s0 = r32(c, S0);
        let block = c.g[A1][0];
        Addr(s0.wrapping_add(0x6cc)).sw(c, block);
        c.g[V0][0] = c.g[S0][0];
        let s1 = r32(c, S1);
        c.g[V1][0] = Addr(s1.wrapping_add(4)).lw(c);
        let stored = c.g[A1][0];
        Addr(c.g[V1][0] as u32).sw(c, stored);
        c.g[A0][0] = Addr(s1.wrapping_add(4)).lw(c);
        c.g[V1][0] = Addr(s0.wrapping_add(0x6cc)).lw(c);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        let a0 = c.g[A0][0] as u32;
        let v1 = c.g[V1][0];
        Addr(a0.wrapping_add(4)).sw(c, v1);
        frame.close(c);
        Ok(())
    });
}

/// `FEScreenBlank` constructor (0x1de8c8): `NetFEState_ctor(this, 0, 0)` (the
/// base at 0x1a8500), then the vtable at `+8`.
pub fn fe_screen_blank_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            15,
            true,
            true,
            Target::Table(0x001a_8500),
            0x001d_e8e4,
            0x4908,
        )
    });
}

/// `FEScreenBlank` destructor (0x1de918): vtable at `+8`, then the base
/// destructor at 0x1a85d0.
pub fn fe_screen_blank_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_dtor_with_vtable(
            c,
            10,
            Target::Table(0x001a_85d0),
            0x4908,
            0x001d_e934,
        )
    });
}

/// `cFEStateMemCard` constructor (0x1de950): `MemCardStateBase_ctor(this, 0, 0)`,
/// then the vtable at `+8`.
pub fn c_fe_state_mem_card_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            15,
            true,
            true,
            Target::Direct(FE_MEMCARD_STATE_BASE_CTOR),
            0x001d_e96c,
            0x4a90,
        )
    });
}

/// `FEScreenAutosavePopup` constructor (0x1de990): `UIState_ctor(this, 0)`,
/// vtable at `+8`.
pub fn fe_screen_autosave_popup_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(FE_UI_STATE_CTOR),
            0x001d_e9a8,
            0x4b60,
        )
    });
}

/// `FEScreenAutosavePopup` destructor (0x1de9c8): vtable at `+8`, then
/// `UIState_dtor(this)`.
pub fn fe_screen_autosave_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_dtor_with_vtable(
            c,
            10,
            Target::Table(FE_UI_STATE_DTOR),
            0x4b60,
            0x001d_e9e4,
        )
    });
}

/// `cFEPopupOptionReset` destructor (0x1de9f8): vtable at `+8`, then
/// `UIState_dtor(this)`.
pub fn c_fe_popup_option_reset_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_dtor_with_vtable(
            c,
            10,
            Target::Table(FE_UI_STATE_DTOR),
            0x4c30,
            0x001d_ea14,
        )
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;
    use crate::game::fe::feasyncfile::tests::allocator;
    use crate::game::fe::feasyncfile::tests::be_library_2;
    use crate::game::fe::feasyncfile::tests::call_machine;
    use crate::game::fe::feasyncfile::tests::world;

    /// The five wrappers: a `this` object, a random flag in `a1`.
    fn set_key_case(generated: GuestFn, hand: GuestFn, entry: u32) {
        check(generated, hand, 200, &|c, seed| {
            let mut r = machine(c, seed, entry as u64, 0);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.pc = entry;
        });
    }

    /// A UI-state constructor or destructor: the base helpers are real ports
    /// (`UIState_ctor`, `UIState_dtor`), so the plain machine is enough.
    fn ui_state_case(generated: GuestFn, hand: GuestFn, entry: u32) {
        check(generated, hand, 200, &|c, seed| {
            machine(c, seed, entry as u64, 0);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.pc = entry;
        });
    }

    /// A constructor that calls game code which allocates or reads the
    /// singleton: the pool allocator and a real heap arena (`call_machine`),
    /// the BE library dummy and a world, the allocator mutex when `alloc`.
    fn heap_state_case(generated: GuestFn, hand: GuestFn, entry: u32, alloc: bool) {
        check(generated, hand, 40, &|c, seed| {
            call_machine(c, seed, entry as u64, 0);
            be_library_2(c);
            world(c);
            if alloc {
                allocator(c);
            }
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.pc = entry;
        });
    }

    #[test]
    fn c_keyboard_popup_set_key_1c_enabled_matches() {
        set_key_case(
            ee::s04::f_001de378__generated,
            c_keyboard_popup_set_key_1c_enabled,
            0x001d_e378,
        );
    }

    #[test]
    fn c_keyboard_popup_set_key_45_enabled_matches() {
        set_key_case(
            ee::s04::f_001de3a0__generated,
            c_keyboard_popup_set_key_45_enabled,
            0x001d_e3a0,
        );
    }

    #[test]
    fn c_keyboard_popup_set_key_4f_enabled_matches() {
        set_key_case(
            ee::s04::f_001de3c8__generated,
            c_keyboard_popup_set_key_4f_enabled,
            0x001d_e3c8,
        );
    }

    #[test]
    fn c_keyboard_popup_set_key_29_enabled_matches() {
        set_key_case(
            ee::s04::f_001de3f0__generated,
            c_keyboard_popup_set_key_29_enabled,
            0x001d_e3f0,
        );
    }

    #[test]
    fn c_keyboard_popup_set_key_42_enabled_matches() {
        set_key_case(
            ee::s04::f_001de418__generated,
            c_keyboard_popup_set_key_42_enabled,
            0x001d_e418,
        );
    }

    #[test]
    fn c_fe_options_credits_ctor_2_matches() {
        ui_state_case(
            ee::s04::f_001de4b0__generated,
            c_fe_options_credits_ctor_2,
            0x001d_e4b0,
        );
    }

    #[test]
    fn c_fe_state_event_select_ctor_2_matches() {
        heap_state_case(
            ee::s04::f_001de4e8__generated,
            c_fe_state_event_select_ctor_2,
            0x001d_e4e8,
            false,
        );
    }

    #[test]
    fn c_fe_state_background_ctor_3_matches() {
        ui_state_case(
            ee::s04::f_001de590__generated,
            c_fe_state_background_ctor_3,
            0x001d_e590,
        );
    }

    #[test]
    fn c_fe_state_transition_ctor_3_matches() {
        ui_state_case(
            ee::s04::f_001de628__generated,
            c_fe_state_transition_ctor_3,
            0x001d_e628,
        );
    }

    #[test]
    fn c_fe_state_title_ctor_2_matches() {
        ui_state_case(ee::s04::f_001de690__generated, c_fe_state_title_ctor_2, 0x001d_e690);
    }

    #[test]
    fn c_fe_state_main_menu_ctor_2_matches() {
        ui_state_case(
            ee::s04::f_001de6f0__generated,
            c_fe_state_main_menu_ctor_2,
            0x001d_e6f0,
        );
    }

    #[test]
    fn c_fe_state_bonus_material_ctor_2_matches() {
        ui_state_case(
            ee::s04::f_001de750__generated,
            c_fe_state_bonus_material_ctor_2,
            0x001d_e750,
        );
    }

    #[test]
    fn c_fe_state_bonus_material_dtor_matches() {
        ui_state_case(
            ee::s04::f_001de788__generated,
            c_fe_state_bonus_material_dtor,
            0x001d_e788,
        );
    }

    #[test]
    fn net_fe_state_ctor_2_matches() {
        heap_state_case(
            ee::s04::f_001de7b0__generated,
            net_fe_state_ctor_2,
            0x001d_e7b0,
            true,
        );
    }

    #[test]
    fn fe_screen_blank_ctor_2_matches() {
        heap_state_case(
            ee::s04::f_001de8c8__generated,
            fe_screen_blank_ctor_2,
            0x001d_e8c8,
            true,
        );
    }

    #[test]
    fn fe_screen_blank_dtor_matches() {
        heap_state_case(
            ee::s04::f_001de918__generated,
            fe_screen_blank_dtor,
            0x001d_e918,
            true,
        );
    }

    #[test]
    fn c_fe_state_mem_card_ctor_2_matches() {
        heap_state_case(
            ee::s04::f_001de950__generated,
            c_fe_state_mem_card_ctor_2,
            0x001d_e950,
            false,
        );
    }

    #[test]
    fn fe_screen_autosave_popup_ctor_2_matches() {
        ui_state_case(
            ee::s04::f_001de990__generated,
            fe_screen_autosave_popup_ctor_2,
            0x001d_e990,
        );
    }

    #[test]
    fn fe_screen_autosave_popup_dtor_matches() {
        ui_state_case(
            ee::s04::f_001de9c8__generated,
            fe_screen_autosave_popup_dtor,
            0x001d_e9c8,
        );
    }

    #[test]
    fn c_fe_popup_option_reset_dtor_matches() {
        ui_state_case(
            ee::s04::f_001de9f8__generated,
            c_fe_popup_option_reset_dtor,
            0x001d_e9f8,
        );
    }
}
