//! Staging module oc509 (wave wave229): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc509`.
//!
//! The front-end options states (`cFEStateOptions*`), the memory-card and
//! profile-select states and the keyboard popup are thin constructors and
//! destructors: each calls its base class (`UIState` at 0x39e2a0 / 0x39e318 /
//! 0x39e390, or `MemCardStateBase` at 0x1d53b0 / 0x1d5428) and then writes its
//! vtable pointer at `this + 8`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001d_dee8, cfe_state_options_ctor_2, "cFEStateOptions_ctor_2"),
    (0x001d_df20, cfe_state_options_dtor, "cFEStateOptions_dtor"),
    (0x001d_df48, cfe_state_options_game_ctor_2, "cFEStateOptionsGame_ctor_2"),
    (0x001d_df80, cfe_state_options_game_dtor, "cFEStateOptionsGame_dtor"),
    (0x001d_dfb0, cfe_state_options_sound_ctor_2, "cFEStateOptionsSound_ctor_2"),
    (0x001d_dfe8, cfe_state_options_sound_dtor, "cFEStateOptionsSound_dtor"),
    (0x001d_e018, cfe_state_options_hud_ctor_2, "cFEStateOptionsHUD_ctor_2"),
    (0x001d_e050, cfe_state_options_hud_dtor, "cFEStateOptionsHUD_dtor"),
    (
        0x001d_e080,
        cfe_state_options_controller_ctor_2,
        "cFEStateOptionsController_ctor_2",
    ),
    (
        0x001d_e0c0,
        cfe_state_options_controller_dtor,
        "cFEStateOptionsController_dtor",
    ),
    (
        0x001d_e0f0,
        cfe_state_options_save_load_ctor_2,
        "cFEStateOptionsSaveLoad_ctor_2",
    ),
    (
        0x001d_e128,
        cfe_state_options_save_load_dtor,
        "cFEStateOptionsSaveLoad_dtor",
    ),
    (
        0x001d_e150,
        cfe_state_options_device_select_dtor,
        "cFEStateOptionsDeviceSelect_dtor",
    ),
    (
        0x001d_e178,
        cfe_state_options_device_select_ctor_2,
        "cFEStateOptionsDeviceSelect_ctor_2",
    ),
    (
        0x001d_e1b8,
        fe_screen_select_profile_ctor_2,
        "FEScreenSelectProfile_ctor_2",
    ),
    (
        0x001d_e1f0,
        fe_screen_select_profile_dtor,
        "FEScreenSelectProfile_dtor",
    ),
    (0x001d_e218, cfe_state_profile_load_dtor, "cFEStateProfileLoad_dtor"),
    (0x001d_e288, ckeyboard_popup_ctor_2, "cKeyboardPopup_ctor_2"),
    (
        0x001d_e2e8,
        ckeyboard_popup_get_text_length,
        "cKeyboardPopup_GetTextLength",
    ),
    (
        0x001d_e350,
        ckeyboard_popup_set_key4b_enabled,
        "cKeyboardPopup_SetKey4BEnabled",
    ),
];

/// `UIState` constructor (`UIState_ctor`, 0x39e2a0).
const FE_UI_STATE_CTOR: u32 = 0x0039_e2a0;
/// `UIState_ctor_2` (0x39e318), the constructor with a2 as an argument.
const FE_UI_STATE_CTOR_2: u32 = 0x0039_e318;
/// `UIState` destructor (0x39e390).
const FE_UI_STATE_DTOR: u32 = 0x0039_e390;
/// `MemCardStateBase_ctor` (0x1d53b0).
const FE_MEMCARD_STATE_CTOR: u32 = 0x001d_53b0;
/// `MemCardStateBase_dtor` (0x1d5428).
const FE_MEMCARD_STATE_DTOR: u32 = 0x001d_5428;
/// `cKeyboardPopup_SetKeyEnabled` (0x1ce3c8).
const FE_KEYBOARD_SET_KEY_ENABLED: u32 = 0x001c_e3c8;
/// `strlen` (0x416810).
const FE_STRLEN: u32 = 0x0041_6810;

/// The vtable pointer a constructor stores: `0x470000 - sub`, sign-extended
/// (the `lui`/`addiu` pair that builds the vtable address).
fn fe_vptr(sub: u32) -> u64 {
    sx(0x0047_0000u32.wrapping_sub(sub))
}

/// The 56-byte `cFEStateOptions*` constructors: the `UIState` base constructor
/// runs on `this` with a1 = 0, then `this->vptr` (+8) is stored.
fn fe_options_ui_ctor(c: &mut Cpu, call_ret: u32, vptr_sub: u32) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = 0;
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, FE_UI_STATE_CTOR, call_ret)?;
    c.g[V1][0] = fe_vptr(vptr_sub);
    c.g[V0][0] = c.g[S0][0];
    frame.restore_ra(c, 0);
    Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, c.g[V1][0]);
    frame.restore(c, S0, 0x10);
    frame.close(c);
    Ok(())
}

/// The 40-byte destructors: store `this->vptr` (+8), then call the base
/// destructor `target` on the unchanged `this` (a0), returning to `ret`.
fn fe_options_dtor(c: &mut Cpu, target: u32, ret: u32, vptr_sub: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = fe_vptr(vptr_sub);
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
    call_ptr(c, target, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFEStateOptions::cFEStateOptions` (0x1ddee8, the `_ctor_2` alias): the
/// `UIState` base constructor, then the `cFEStateOptions` vtable.
pub fn cfe_state_options_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_options_ui_ctor(c, 0x001d_df00, 0x3840))
}

/// `cFEStateOptions::~cFEStateOptions` (0x1ddf20): the `cFEState` vtable,
/// then the `UIState` destructor.
pub fn cfe_state_options_dtor(c: &mut Cpu) {
    guest(c, |c| fe_options_dtor(c, FE_UI_STATE_DTOR, 0x001d_df3c, 0x2f30))
}

/// `cFEStateOptionsGame::cFEStateOptionsGame` (0x1ddf48, `_ctor_2`).
pub fn cfe_state_options_game_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_options_ui_ctor(c, 0x001d_df60, 0x3910))
}

/// `cFEStateOptionsGame::~cFEStateOptionsGame` (0x1ddf80).
pub fn cfe_state_options_game_dtor(c: &mut Cpu) {
    guest(c, |c| fe_options_dtor(c, FE_UI_STATE_DTOR, 0x001d_df9c, 0x2f30))
}

/// `cFEStateOptionsSound::cFEStateOptionsSound` (0x1ddfb0, `_ctor_2`).
pub fn cfe_state_options_sound_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_options_ui_ctor(c, 0x001d_dfc8, 0x39e0))
}

/// `cFEStateOptionsSound::~cFEStateOptionsSound` (0x1ddfe8).
pub fn cfe_state_options_sound_dtor(c: &mut Cpu) {
    guest(c, |c| fe_options_dtor(c, FE_UI_STATE_DTOR, 0x001d_e004, 0x2f30))
}

/// `cFEStateOptionsHUD::cFEStateOptionsHUD` (0x1de018, `_ctor_2`).
pub fn cfe_state_options_hud_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_options_ui_ctor(c, 0x001d_e030, 0x3ab0))
}

/// `cFEStateOptionsHUD::~cFEStateOptionsHUD` (0x1de050).
pub fn cfe_state_options_hud_dtor(c: &mut Cpu) {
    guest(c, |c| fe_options_dtor(c, FE_UI_STATE_DTOR, 0x001d_e06c, 0x2f30))
}

/// `cFEStateOptionsController::cFEStateOptionsController` (0x1de080, 60
/// bytes): as the 56-byte constructors, and it also clears the byte at +0x50
/// (`this->flag`) before the vptr store.
pub fn cfe_state_options_controller_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, FE_UI_STATE_CTOR, 0x001d_e098)?;
        c.g[V1][0] = fe_vptr(0x3b80);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x50)).sb(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateOptionsController::~cFEStateOptionsController` (0x1de0c0).
pub fn cfe_state_options_controller_dtor(c: &mut Cpu) {
    guest(c, |c| fe_options_dtor(c, FE_UI_STATE_DTOR, 0x001d_e0dc, 0x2f30))
}

/// `cFEStateOptionsSaveLoad::cFEStateOptionsSaveLoad` (0x1de0f0, `_ctor_2`).
pub fn cfe_state_options_save_load_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_options_ui_ctor(c, 0x001d_e108, 0x3c50))
}

/// `cFEStateOptionsSaveLoad::~cFEStateOptionsSaveLoad` (0x1de128).
pub fn cfe_state_options_save_load_dtor(c: &mut Cpu) {
    guest(c, |c| fe_options_dtor(c, FE_UI_STATE_DTOR, 0x001d_e144, 0x2f30))
}

/// `cFEStateOptionsDeviceSelect::~cFEStateOptionsDeviceSelect` (0x1de150):
/// the vptr, then the `MemCardStateBase` destructor.
pub fn cfe_state_options_device_select_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_options_dtor(c, FE_MEMCARD_STATE_DTOR, 0x001d_e16c, 0x3dd8)
    })
}

/// `cFEStateOptionsDeviceSelect::cFEStateOptionsDeviceSelect` (0x1de178,
/// `_ctor_2`, 60 bytes): the `MemCardStateBase` constructor runs with a1 = a2
/// = 0, then the vptr is stored.
pub fn cfe_state_options_device_select_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, FE_MEMCARD_STATE_CTOR, 0x001d_e194)?;
        c.g[V1][0] = fe_vptr(0x3dd8);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `FEScreenSelectProfile::FEScreenSelectProfile` (0x1de1b8, `_ctor_2`).
pub fn fe_screen_select_profile_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_options_ui_ctor(c, 0x001d_e1d0, 0x3ea8))
}

/// `FEScreenSelectProfile::~FEScreenSelectProfile` (0x1de1f0).
pub fn fe_screen_select_profile_dtor(c: &mut Cpu) {
    guest(c, |c| fe_options_dtor(c, FE_UI_STATE_DTOR, 0x001d_e20c, 0x2f30))
}

/// `cFEStateProfileLoad::~cFEStateProfileLoad` (0x1de218): the vptr, then the
/// `MemCardStateBase` destructor.
pub fn cfe_state_profile_load_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_options_dtor(c, FE_MEMCARD_STATE_DTOR, 0x001d_e234, 0x4030)
    })
}

/// `cKeyboardPopup::cKeyboardPopup` (0x1de288, `_ctor_2`, 76 bytes): the
/// `UIState_ctor_2` base constructor with a1 = a2 = 0, then the vptr, the
/// popup's character limit (0x3f at +0x60) and the `gp`-relative default
/// (`lw gp+0xda0`) at +0x424.
pub fn ckeyboard_popup_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, FE_UI_STATE_CTOR_2, 0x001d_e2a4)?;
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xda0)).lw(c);
        c.g[V1][0] = fe_vptr(0x4138);
        c.g[A0][0] = 0x3f;
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x60)).sw(c, c.g[A0][0]);
        Addr((c.g[S0][0] as u32).wrapping_add(0x424)).sw(c, c.g[A1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cKeyboardPopup::GetTextLength` (0x1de2e8): `strlen(this + 0x74)`, its
/// result returned as is.
pub fn ckeyboard_popup_get_text_length(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
        call_ptr(c, FE_STRLEN, 0x001d_e2f8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cKeyboardPopup::SetKey4BEnabled(popup, enabled)` (0x1de350): the key
/// 0x4b (the 4B key) set through `cKeyboardPopup_SetKeyEnabled` with a2 = the
/// enable flag and a3 = 2.
pub fn ckeyboard_popup_set_key4b_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x4b;
        c.g[A3][0] = 0x2;
        call_ptr(c, FE_KEYBOARD_SET_KEY_ENABLED, 0x001d_e368)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Test fixtures: gated fakes for the base-class callees.

#[cfg(test)]
use crate::difftest::fakes;
#[cfg(test)]
use crate::difftest::{Rng, DATA, GAME_GP};
#[cfg(test)]
use std::ptr::addr_of_mut;
#[cfg(test)]
use std::sync::atomic::AtomicUsize;

/// Gate word for this module's fakes, and its magic value.
#[cfg(test)]
pub(crate) const FE_GATE: u32 = DATA + 0xf_0000;
#[cfg(test)]
pub(crate) const FE_GATE_MAGIC: u32 = 0x0509_0ce5;
/// Seeds per test.
#[cfg(test)]
pub(crate) const FE_SEEDS: u32 = 200;
/// Object pointers land in `DATA..DATA + 0x80000` (away from the gate word).
#[cfg(test)]
pub(crate) const FE_OBJ_MASK: u32 = 0x7_fff0;

#[cfg(test)]
pub(crate) static FE_PREV_UI_CTOR: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
pub(crate) static FE_PREV_UI_CTOR_2: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
pub(crate) static FE_PREV_UI_DTOR: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
pub(crate) static FE_PREV_MEMCARD_CTOR: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
pub(crate) static FE_PREV_MEMCARD_DTOR: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
pub(crate) static FE_PREV_KEYBOARD_SET_KEY: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
pub(crate) static FE_PREV_STRLEN: AtomicUsize = AtomicUsize::new(0);

/// Logs the call (registers, cycles, target) and returns to `ra`; with the
/// gate off, runs the body the slot held before.
#[cfg(test)]
fn fe_gated(c: &mut Cpu, target: u32, prev: &AtomicUsize, generated: GuestFn) {
    if c.bus.read32(FE_GATE) != FE_GATE_MAGIC {
        return fakes::fall_back(c, prev, generated);
    }
    let ra = c.g[RA][0] as u32;
    fakes::fake(c, target, ra);
}

#[cfg(test)]
fn fe_fake_ui_ctor(c: &mut Cpu) {
    fe_gated(c, FE_UI_STATE_CTOR, &FE_PREV_UI_CTOR, ee::s13::f_0039e2a0__generated)
}

#[cfg(test)]
fn fe_fake_ui_ctor_2(c: &mut Cpu) {
    fe_gated(
        c,
        FE_UI_STATE_CTOR_2,
        &FE_PREV_UI_CTOR_2,
        ee::s13::f_0039e318__generated,
    )
}

#[cfg(test)]
fn fe_fake_ui_dtor(c: &mut Cpu) {
    fe_gated(c, FE_UI_STATE_DTOR, &FE_PREV_UI_DTOR, ee::s13::f_0039e390__generated)
}

#[cfg(test)]
fn fe_fake_memcard_ctor(c: &mut Cpu) {
    fe_gated(
        c,
        FE_MEMCARD_STATE_CTOR,
        &FE_PREV_MEMCARD_CTOR,
        ee::s04::f_001d53b0__generated,
    )
}

#[cfg(test)]
fn fe_fake_memcard_dtor(c: &mut Cpu) {
    fe_gated(
        c,
        FE_MEMCARD_STATE_DTOR,
        &FE_PREV_MEMCARD_DTOR,
        ee::s04::f_001d5428__generated,
    )
}

#[cfg(test)]
fn fe_fake_keyboard_set_key(c: &mut Cpu) {
    fe_gated(
        c,
        FE_KEYBOARD_SET_KEY_ENABLED,
        &FE_PREV_KEYBOARD_SET_KEY,
        ee::s04::f_001ce3c8__generated,
    )
}

#[cfg(test)]
fn fe_fake_strlen(c: &mut Cpu) {
    fe_gated(c, FE_STRLEN, &FE_PREV_STRLEN, ee::s15::f_00416810__generated)
}

/// Hooks the fakes into the override slots of their callees (once per process).
#[cfg(test)]
pub(crate) fn fe_install_fakes() {
    fakes::hook(
        addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
        fe_fake_ui_ctor,
        &FE_PREV_UI_CTOR,
    );
    fakes::hook(
        addr_of_mut!(ee::s13::OVERRIDE_0039E318),
        fe_fake_ui_ctor_2,
        &FE_PREV_UI_CTOR_2,
    );
    fakes::hook(
        addr_of_mut!(ee::s13::OVERRIDE_0039E390),
        fe_fake_ui_dtor,
        &FE_PREV_UI_DTOR,
    );
    fakes::hook(
        addr_of_mut!(ee::s04::OVERRIDE_001D53B0),
        fe_fake_memcard_ctor,
        &FE_PREV_MEMCARD_CTOR,
    );
    fakes::hook(
        addr_of_mut!(ee::s04::OVERRIDE_001D5428),
        fe_fake_memcard_dtor,
        &FE_PREV_MEMCARD_DTOR,
    );
    fakes::hook(
        addr_of_mut!(ee::s04::OVERRIDE_001CE3C8),
        fe_fake_keyboard_set_key,
        &FE_PREV_KEYBOARD_SET_KEY,
    );
    fakes::hook(
        addr_of_mut!(ee::s15::OVERRIDE_00416810),
        fe_fake_strlen,
        &FE_PREV_STRLEN,
    );
}

/// The machine for one seed: scrambled registers, the gate word set, a
/// random object pointer in a0 and a game-like gp word. Each callee is
/// called once, and that call may unwind.
#[cfg(test)]
pub(crate) fn fe_setup(c: &mut Cpu, seed: u32, entry: u32, salt: u64) -> Rng {
    fe_install_fakes();
    let mut r = fakes::machine(c, seed, salt, 1);
    c.bus.write32(FE_GATE, FE_GATE_MAGIC);
    c.bus.write32(GAME_GP + 0xda0, r.next_u32());
    c.pc = entry;
    let obj = DATA + (r.next_u32() & FE_OBJ_MASK);
    c.g[A0][0] = sx(obj);
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::covered;

    #[test]
    fn cfe_state_options_ctor_2_matches() {
        covered(
            ee::s04::f_001ddee8__generated,
            cfe_state_options_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_dee8, 1);
            },
        );
    }

    #[test]
    fn cfe_state_options_dtor_matches() {
        covered(
            ee::s04::f_001ddf20__generated,
            cfe_state_options_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_df20, 2);
            },
        );
    }

    #[test]
    fn cfe_state_options_game_ctor_2_matches() {
        covered(
            ee::s04::f_001ddf48__generated,
            cfe_state_options_game_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_df48, 3);
            },
        );
    }

    #[test]
    fn cfe_state_options_game_dtor_matches() {
        covered(
            ee::s04::f_001ddf80__generated,
            cfe_state_options_game_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_df80, 4);
            },
        );
    }

    #[test]
    fn cfe_state_options_sound_ctor_2_matches() {
        covered(
            ee::s04::f_001ddfb0__generated,
            cfe_state_options_sound_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_dfb0, 5);
            },
        );
    }

    #[test]
    fn cfe_state_options_sound_dtor_matches() {
        covered(
            ee::s04::f_001ddfe8__generated,
            cfe_state_options_sound_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_dfe8, 6);
            },
        );
    }

    #[test]
    fn cfe_state_options_hud_ctor_2_matches() {
        covered(
            ee::s04::f_001de018__generated,
            cfe_state_options_hud_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e018, 7);
            },
        );
    }

    #[test]
    fn cfe_state_options_hud_dtor_matches() {
        covered(
            ee::s04::f_001de050__generated,
            cfe_state_options_hud_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e050, 8);
            },
        );
    }

    #[test]
    fn cfe_state_options_controller_ctor_2_matches() {
        covered(
            ee::s04::f_001de080__generated,
            cfe_state_options_controller_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e080, 9);
            },
        );
    }

    #[test]
    fn cfe_state_options_controller_dtor_matches() {
        covered(
            ee::s04::f_001de0c0__generated,
            cfe_state_options_controller_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e0c0, 10);
            },
        );
    }

    #[test]
    fn cfe_state_options_save_load_ctor_2_matches() {
        covered(
            ee::s04::f_001de0f0__generated,
            cfe_state_options_save_load_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e0f0, 11);
            },
        );
    }

    #[test]
    fn cfe_state_options_save_load_dtor_matches() {
        covered(
            ee::s04::f_001de128__generated,
            cfe_state_options_save_load_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e128, 12);
            },
        );
    }

    #[test]
    fn cfe_state_options_device_select_dtor_matches() {
        covered(
            ee::s04::f_001de150__generated,
            cfe_state_options_device_select_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e150, 13);
            },
        );
    }

    #[test]
    fn cfe_state_options_device_select_ctor_2_matches() {
        covered(
            ee::s04::f_001de178__generated,
            cfe_state_options_device_select_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e178, 14);
            },
        );
    }

    #[test]
    fn fe_screen_select_profile_ctor_2_matches() {
        covered(
            ee::s04::f_001de1b8__generated,
            fe_screen_select_profile_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e1b8, 15);
            },
        );
    }

    #[test]
    fn fe_screen_select_profile_dtor_matches() {
        covered(
            ee::s04::f_001de1f0__generated,
            fe_screen_select_profile_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e1f0, 16);
            },
        );
    }

    #[test]
    fn cfe_state_profile_load_dtor_matches() {
        covered(
            ee::s04::f_001de218__generated,
            cfe_state_profile_load_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e218, 17);
            },
        );
    }

    #[test]
    fn ckeyboard_popup_ctor_2_matches() {
        covered(
            ee::s04::f_001de288__generated,
            ckeyboard_popup_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e288, 18);
            },
        );
    }

    #[test]
    fn ckeyboard_popup_get_text_length_matches() {
        covered(
            ee::s04::f_001de2e8__generated,
            ckeyboard_popup_get_text_length,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e2e8, 19);
            },
        );
    }

    #[test]
    fn ckeyboard_popup_set_key4b_enabled_matches() {
        covered(
            ee::s04::f_001de350__generated,
            ckeyboard_popup_set_key4b_enabled,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e350, 20);
            },
        );
    }
}
