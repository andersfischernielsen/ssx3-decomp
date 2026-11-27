//! Staging module oc1014 (wave wave261): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1014`.

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
        0x0022_1fa0,
        ov_screen_rival_pre_dtor,
        "OVScreenRivalPre_dtor",
    ),
    (
        0x0022_1fc8,
        ov_screen_rival_pre_ctor,
        "OVScreenRivalPre_ctor",
    ),
    (
        0x0022_2030,
        ov_screen_save_replay_dtor,
        "OVScreenSaveReplay_dtor",
    ),
    (
        0x0022_2058,
        ov_screen_save_replay_ctor_2,
        "OVScreenSaveReplay_ctor_2",
    ),
    (
        0x0022_2098,
        ov_screen_save_records_dtor,
        "OVScreenSaveRecords_dtor",
    ),
    (
        0x0022_20c0,
        ov_screen_save_records_ctor_2,
        "OVScreenSaveRecords_ctor_2",
    ),
    (
        0x0022_2100,
        ov_screen_rewards_list_popup_ctor,
        "OVScreenRewardsListPopup_ctor",
    ),
    (
        0x0022_2160,
        ov_screen_rewards_list_popup_ctor_2,
        "OVScreenRewardsListPopup_ctor_2",
    ),
];

/// `UIState_ctor` (0x39e2a0): the base constructor every `OVState` runs first.
const OV_UI_STATE_CTOR: u32 = 0x0039_e2a0;
/// `UIState_dtor` (0x39e390): the base destructor the `OVState` destructors end in.
const OV_UI_STATE_DTOR: u32 = 0x0039_e390;
/// `MemCardStateBase_ctor` (0x1d53b0), the base of the save-screen overlay states.
const OV_MEMCARD_CTOR: u32 = 0x001d_53b0;
/// `MemCardStateBase_dtor` (0x1d5428).
const OV_MEMCARD_DTOR: u32 = 0x001d_5428;
/// The game-image data page (`0x4a0000`) holding the chat buffer's first
/// halfword at +0x2088.
const OV_DATA_PAGE: u32 = 0x004a_0000;

/// The stores every constructor of this group makes after its base constructor
/// (`this` is `s0`): the overlay id at +0x98, the transition, pause-menu and
/// PDA flags (+0x48, +0x4c, +0x54) cleared, the base vtable at +0x8 and then
/// `vptr` over it, and the chat buffer's first halfword at +0x58. Leaves `v0` =
/// this, `v1` = `vptr`, `a1` = the data page and `a0` = the halfword, as the
/// generated body does.
fn ov_base_fill_fields(c: &mut Cpu, ov_id: u64, vptr: u32) {
    let obj = c.g[S0][0] as u32;
    c.sw(obj.wrapping_add(0x98), ov_id);
    c.sw(obj.wrapping_add(0x48), 0);
    c.sw(obj.wrapping_add(0x8), 0x0047_4e08);
    c.sw(obj.wrapping_add(0x4c), 0);
    c.sw(obj.wrapping_add(0x54), 0);
    c.g[V1][0] = vptr as u64;
    c.sw(obj.wrapping_add(0x8), c.g[V1][0]);
    c.g[V0][0] = c.g[S0][0];
    c.g[A1][0] = OV_DATA_PAGE as u64;
    c.g[A0][0] = c.lhu(OV_DATA_PAGE.wrapping_add(0x2088));
    c.sh(obj.wrapping_add(0x58), c.g[A0][0]);
}

/// A one-call destructor wrapper: the vtable word `vtbl` at `this+0x8`, then
/// `callee(this)` returning to `ret` (the callee's unwind returns at once, as
/// the generated code does). `this` (a0) is passed through unchanged.
fn ov_wrapper_dtor_body(c: &mut Cpu, vtbl: u32, callee: u32, ret: u32) -> crate::call::Flow {
    c.cycles += 10;
    let f = Frame::open(c, 0x10);
    f.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = vtbl as u64;
    let this = c.g[V1][0] as u32;
    c.sw(this.wrapping_add(0x8), c.g[V0][0]);
    call_ptr(c, callee, ret)?;
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// The save-screen overlay constructor that takes `(this, 0, 0)` into the
/// `MemCardStateBase` constructor and then stores its vtable word at +0x8.
fn ov_memcard_ctor_2_body(c: &mut Cpu, vtbl: u32, ret: u32) -> crate::call::Flow {
    c.cycles += 15;
    let f = Frame::open(c, 0x20);
    c.g[A1][0] = 0;
    f.save(c, 16, 0x10);
    c.g[A2][0] = 0;
    f.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, OV_MEMCARD_CTOR, ret)?;
    c.g[V1][0] = vtbl as u64;
    c.g[V0][0] = c.g[S0][0];
    f.restore_ra(c, 0);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    f.restore(c, 16, 0x10);
    f.close(c);
    Ok(())
}

/// `OVScreenRivalPre_dtor` (0x221fa0): the rival-preview overlay destructor.
/// Stores the base vtable and runs `UIState_dtor`.
pub fn ov_screen_rival_pre_dtor(c: &mut Cpu) {
    guest(c, |c| {
        ov_wrapper_dtor_body(c, 0x0047_4e08, OV_UI_STATE_DTOR, 0x0022_1fbc)
    });
}

/// `OVScreenRivalPre_ctor` (0x221fc8): constructs the rival-preview overlay
/// state in `a0`: `UIState_ctor`, then the stores with overlay id `a2` and the
/// rival-preview vtable.
pub fn ov_screen_rival_pre_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let f = Frame::open(c, 0x30);
        f.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        f.save(c, 17, 0x10);
        f.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, OV_UI_STATE_CTOR, 0x0022_1fe4)?;
        let ov_id = c.g[S1][0];
        ov_base_fill_fields(c, ov_id, 0x0047_25c8);
        f.restore(c, 17, 0x10);
        f.restore(c, 16, 0x20);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `OVScreenSaveReplay_dtor` (0x222030): the save-replay overlay destructor.
/// Stores the save-screen vtable and runs `MemCardStateBase_dtor`.
pub fn ov_screen_save_replay_dtor(c: &mut Cpu) {
    guest(c, |c| {
        ov_wrapper_dtor_body(c, 0x0047_2440, OV_MEMCARD_DTOR, 0x0022_204c)
    });
}

/// `OVScreenSaveReplay_ctor_2` (0x222058): the save-replay overlay constructor
/// (`MemCardStateBase_ctor(this, 0, 0)`, then the vtable word).
pub fn ov_screen_save_replay_ctor_2(c: &mut Cpu) {
    guest(c, |c| ov_memcard_ctor_2_body(c, 0x0047_2440, 0x0022_2074));
}

/// `OVScreenSaveRecords_dtor` (0x222098): the save-records overlay destructor.
/// Stores its vtable and runs `MemCardStateBase_dtor`.
pub fn ov_screen_save_records_dtor(c: &mut Cpu) {
    guest(c, |c| {
        ov_wrapper_dtor_body(c, 0x0047_22b8, OV_MEMCARD_DTOR, 0x0022_20b4)
    });
}

/// `OVScreenSaveRecords_ctor_2` (0x2220c0): the save-records overlay
/// constructor (`MemCardStateBase_ctor(this, 0, 0)`, then its vtable word).
pub fn ov_screen_save_records_ctor_2(c: &mut Cpu) {
    guest(c, |c| ov_memcard_ctor_2_body(c, 0x0047_22b8, 0x0022_20dc));
}

/// `OVScreenRewardsListPopup_ctor` (0x222100): the rewards-list popup state in
/// `a0` with overlay id 0, after `UIState_ctor(this, 0)`.
pub fn ov_screen_rewards_list_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        f.save(c, 16, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OV_UI_STATE_CTOR, 0x0022_2118)?;
        ov_base_fill_fields(c, 0, 0x0047_21e0);
        f.restore(c, 16, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `OVScreenRewardsListPopup_ctor_2` (0x222160): as the constructor above, with
/// the overlay id taken from `a2`.
pub fn ov_screen_rewards_list_popup_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let f = Frame::open(c, 0x30);
        f.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        f.save(c, 17, 0x10);
        f.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, OV_UI_STATE_CTOR, 0x0022_217c)?;
        let ov_id = c.g[S1][0];
        ov_base_fill_fields(c, ov_id, 0x0047_21e0);
        f.restore(c, 17, 0x10);
        f.restore(c, 16, 0x20);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes (its own address and magic in the
    /// machine's RAM, so other tests' fakes stay off).
    const OC1014_CTRL: u32 = DATA + 0x0020_a140;
    const OC1014_MAGIC: u32 = 0x1014_0c0d;

    static OC1014_PREV_UI_CTOR: AtomicUsize = AtomicUsize::new(0);
    static OC1014_PREV_UI_DTOR: AtomicUsize = AtomicUsize::new(0);
    static OC1014_PREV_MEMCARD_CTOR: AtomicUsize = AtomicUsize::new(0);
    static OC1014_PREV_MEMCARD_DTOR: AtomicUsize = AtomicUsize::new(0);

    /// `UIState_ctor` with its gate on: logs the call and returns to the
    /// caller, so only the wrappers' own work is compared.
    fn oc1014_fake_ui_ctor(c: &mut Cpu) {
        if c.bus.read32(OC1014_CTRL) != OC1014_MAGIC {
            return fakes::fall_back(c, &OC1014_PREV_UI_CTOR, ee::s13::f_0039e2a0__generated);
        }
        fakes::fake_call(c, 0x0039_e2a0);
    }

    /// `UIState_dtor` with its gate on (see `oc1014_fake_ui_ctor`).
    fn oc1014_fake_ui_dtor(c: &mut Cpu) {
        if c.bus.read32(OC1014_CTRL) != OC1014_MAGIC {
            return fakes::fall_back(c, &OC1014_PREV_UI_DTOR, ee::s13::f_0039e390__generated);
        }
        fakes::fake_call(c, 0x0039_e390);
    }

    /// `MemCardStateBase_ctor` with its gate on (see `oc1014_fake_ui_ctor`).
    fn oc1014_fake_memcard_ctor(c: &mut Cpu) {
        if c.bus.read32(OC1014_CTRL) != OC1014_MAGIC {
            return fakes::fall_back(c, &OC1014_PREV_MEMCARD_CTOR, ee::s04::f_001d53b0__generated);
        }
        fakes::fake_call(c, 0x001d_53b0);
    }

    /// `MemCardStateBase_dtor` with its gate on (see `oc1014_fake_ui_ctor`).
    fn oc1014_fake_memcard_dtor(c: &mut Cpu) {
        if c.bus.read32(OC1014_CTRL) != OC1014_MAGIC {
            return fakes::fall_back(c, &OC1014_PREV_MEMCARD_DTOR, ee::s04::f_001d5428__generated);
        }
        fakes::fake_call(c, 0x001d_5428);
    }

    /// Puts this module's four gated fakes in their override slots (once per
    /// process; with the gate off they run the body they replaced).
    fn oc1014_hook() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
            oc1014_fake_ui_ctor,
            &OC1014_PREV_UI_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E390),
            oc1014_fake_ui_dtor,
            &OC1014_PREV_UI_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001D53B0),
            oc1014_fake_memcard_ctor,
            &OC1014_PREV_MEMCARD_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001D5428),
            oc1014_fake_memcard_dtor,
            &OC1014_PREV_MEMCARD_DTOR,
        );
    }

    /// Scrambled registers, the gate on, and the object (junk in its first
    /// 0x100 bytes, so every stored field shows) at `DATA + 0x2000` (varied by
    /// seed) as `this`. Each function makes one call, so one unwind index.
    fn oc1014_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 1);
        c.bus.write32(OC1014_CTRL, OC1014_MAGIC);
        let obj = DATA + 0x2000 + (seed % 4) * 0x100;
        crate::difftest::fill(c, &mut r, obj, 0x40);
        c.g[A0][0] = obj as u64;
    }

    #[test]
    fn ov_screen_rival_pre_dtor_matches() {
        oc1014_hook();
        check(
            ee::s05::f_00221fa0__generated,
            ov_screen_rival_pre_dtor,
            200,
            &|c, seed| oc1014_setup(c, seed, 0x0022_1fa0),
        );
    }

    #[test]
    fn ov_screen_rival_pre_ctor_matches() {
        oc1014_hook();
        check(
            ee::s05::f_00221fc8__generated,
            ov_screen_rival_pre_ctor,
            200,
            &|c, seed| oc1014_setup(c, seed, 0x0022_1fc8),
        );
    }

    #[test]
    fn ov_screen_save_replay_dtor_matches() {
        oc1014_hook();
        check(
            ee::s05::f_00222030__generated,
            ov_screen_save_replay_dtor,
            200,
            &|c, seed| oc1014_setup(c, seed, 0x0022_2030),
        );
    }

    #[test]
    fn ov_screen_save_replay_ctor_2_matches() {
        oc1014_hook();
        check(
            ee::s05::f_00222058__generated,
            ov_screen_save_replay_ctor_2,
            200,
            &|c, seed| oc1014_setup(c, seed, 0x0022_2058),
        );
    }

    #[test]
    fn ov_screen_save_records_dtor_matches() {
        oc1014_hook();
        check(
            ee::s05::f_00222098__generated,
            ov_screen_save_records_dtor,
            200,
            &|c, seed| oc1014_setup(c, seed, 0x0022_2098),
        );
    }

    #[test]
    fn ov_screen_save_records_ctor_2_matches() {
        oc1014_hook();
        check(
            ee::s05::f_002220c0__generated,
            ov_screen_save_records_ctor_2,
            200,
            &|c, seed| oc1014_setup(c, seed, 0x0022_20c0),
        );
    }

    #[test]
    fn ov_screen_rewards_list_popup_ctor_matches() {
        oc1014_hook();
        check(
            ee::s05::f_00222100__generated,
            ov_screen_rewards_list_popup_ctor,
            200,
            &|c, seed| oc1014_setup(c, seed, 0x0022_2100),
        );
    }

    #[test]
    fn ov_screen_rewards_list_popup_ctor_2_matches() {
        oc1014_hook();
        check(
            ee::s05::f_00222160__generated,
            ov_screen_rewards_list_popup_ctor_2,
            200,
            &|c, seed| oc1014_setup(c, seed, 0x0022_2160),
        );
    }
}
