//! Staging module oc1543 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1543`.

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
    (0x0021_cba0, cov_state_hud1p_ctor_2, "cOVStateHUD1P_ctor_2"),
    (0x0022_0d38, cfe_state_lodge_ctor_2, "cFEStateLodge_ctor_2"),
    (
        0x0022_0d98,
        cfe_state_lodge_rider_detail_ctor_2,
        "cFEStateLodgeRiderDetail_ctor_2",
    ),
    (0x0022_0df8, cfe_state_buy_attrib_ctor_2, "cFEStateBuyAttrib_ctor_2"),
    (
        0x0022_0e58,
        cfe_state_career_stats_ctor_2,
        "cFEStateCareerStats_ctor_2",
    ),
    (
        0x0022_2428,
        cov_state_overlay_global_ctors,
        "cOVStateOverlay_GlobalCtors",
    ),
    (
        0x0023_68a0,
        game_state_event_switch_load_enter,
        "GameStateEventSwitchLoad_Enter",
    ),
    (0x0024_2e58, ssx_app_list_clear, "SSXApp_ListClear"),
];

/// `UIState_ctor` (0x39e2a0), called through the function table.
const UISTATE_CTOR_ADDR: u32 = 0x0039_e2a0;

/// The shape of the four `cFEState*_ctor_2` constructors (14 cycles): a
/// 0x20 frame holding `s0` and `ra`, `UIState_ctor(this, 0)` through the
/// function table returning to `ret`, then the vtable word `vtable` stored at
/// `this + 8`. `this` is read back from `s0` after the call.
fn fe_state_ctor_2_body(c: &mut Cpu, ret: u32, vtable: u32) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = 0;
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, UISTATE_CTOR_ADDR, ret)?;
    c.g[V0][0] = c.g[S0][0];
    c.g[V1][0] = sx(vtable);
    frame.restore_ra(c, 0);
    let this = c.g[S0][0] as u32;
    let value = c.g[V1][0];
    Addr(this.wrapping_add(8)).sw(c, value);
    frame.restore(c, S0, 0x10);
    frame.close(c);
    Ok(())
}

/// `cFEStateLodge_ctor_2(self)` (0x220d38): `UIState_ctor(self, 0)`, then the
/// vtable `0x473aa8` at `+8`.
pub fn cfe_state_lodge_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_state_ctor_2_body(c, 0x0022_0d50, 0x0047_3aa8));
}

/// `cFEStateLodgeRiderDetail_ctor_2(self)` (0x220d98): `UIState_ctor(self, 0)`,
/// then the vtable `0x4739d8` at `+8`.
pub fn cfe_state_lodge_rider_detail_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_state_ctor_2_body(c, 0x0022_0db0, 0x0047_39d8));
}

/// `cFEStateBuyAttrib_ctor_2(self)` (0x220df8): `UIState_ctor(self, 0)`, then
/// the vtable `0x473908` at `+8`.
pub fn cfe_state_buy_attrib_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_state_ctor_2_body(c, 0x0022_0e10, 0x0047_3908));
}

/// `cFEStateCareerStats_ctor_2(self)` (0x220e58): `UIState_ctor(self, 0)`,
/// then the vtable `0x473838` at `+8`.
pub fn cfe_state_career_stats_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_state_ctor_2_body(c, 0x0022_0e70, 0x0047_3838));
}

/// `cOVStateHUD1P_ctor_2(self, ...)` (0x21cba0): `UIState_ctor(self, 0)`, the
/// vtable `0x474160` at `+8`, two `HUDMeter_ctor`s at `self + 0x48 + 8` and
/// `+0x8c` further on (a loop of two), then the HUD fields: `+0x474 = -1`,
/// `+0x478`, `+0x47c`, `+0x488` and `+0x48c` set to 1.0f, `+0x480` and
/// `+0x484` cleared.
pub fn cov_state_hud1p_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x48));
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0x1;
        call_ptr(c, UISTATE_CTOR_ADDR, 0x0021_cbcc)?;
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = sx(0x0047_4160);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        loop {
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
            Callee {
                addr: 0x0020_eaa0,
                f: ee::s05::f_0020eaa0,
            }
            .call(c, 0x0021_cbec)?;
            let more = c.g[S2][0] != c.g[S3][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8c));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0021_cbe0);
            }
        }
        c.cycles += 18;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x474), c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x48c));
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x478));
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x47c));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x480), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x484), 0u64);
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x488));
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cOVStateOverlay_GlobalCtors` (0x222428): the static-init hook
/// `cOVStateOverlay_StaticInitDestroy(1, 0xffff)`.
pub fn cov_state_overlay_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        Callee {
            addr: 0x0021_27e8,
            f: ee::s05::f_002127e8,
        }
        .call(c, 0x0022_243c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SSXApp_ListClear(self)` (0x242e58): deletes every node of the
/// circular list whose head sits at `self + 4` (a node's word 0 is its next
/// node; the head's word 0 is the first node) through `__builtin_delete`,
/// then points the head at itself (`head.next = head`, `head.prev = head`).
pub fn ssx_app_list_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
        c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
        if c.g[S0][0] == c.g[V0][0] {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 5;
                c.g[S0][0] = Addr(c.g[S0][0] as u32).lw(c);
                Callee {
                    addr: 0x0031_7e50,
                    f: ee::s10::f_00317e50,
                }
                .call(c, 0x0024_2e88)?;
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
                let more = c.g[S0][0] != c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0024_2e80);
                    }
                    continue;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
                frame.restore(c, S0, 0x20);
                break;
            }
        }
        c.cycles += 7;
        let head = c.g[V0][0];
        Addr(head as u32).sw(c, head);
        frame.restore_ra(c, 0);
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
        frame.restore(c, S1, 0x10);
        let head_again = c.g[V1][0];
        Addr((head_again as u32).wrapping_add(0x4)).sw(c, head_again);
        frame.close(c);
        Ok(())
    });
}

/// `GameStateEventSwitchLoad_Enter(self)` (0x2368a0): `BELibrary_SetPhase(0)`,
/// `Replay_Reset` of the replay at `gp-0x488`'s `+0x28`, then the BE interface
/// work: unless the flag byte at `0x535bc8 + 0x49` is set, one
/// `BENewPlayerInt_SetFirstTimeWCFlag(iface, i, 0)` for each of the `0x535bc8 +
/// 0x30` entries. `self + 0x10` is cleared on every exit that does not unwind.
pub fn game_state_event_switch_load_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        call_ptr(c, 0x0014_de68, 0x0023_68c4)?;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
        call_ptr(c, 0x0026_f228, 0x0023_68d0)?;
        call_ptr(c, 0x0014_dc80, 0x0023_68d8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_68e4)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x49)).lb(c);
        if c.g[V0][0] != 0 {
            c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
        } else {
            c.cycles += 9;
            c.g[S0][0] = 0;
            call_ptr(c, 0x0014_dc80, 0x0023_6900)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014_dd58, 0x0023_690c)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x30)).lw(c);
            if (c.g[V0][0] as i64) <= 0 {
                c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
            } else {
                c.cycles += 1;
                c.g[A1][0] = c.g[S0][0];
                loop {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x0014_5cb0, 0x0023_6930)?;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x30)).lw(c);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[S0][0];
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0023_6920);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
                    break;
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, machine_rt};
    use crate::difftest::{check, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate words of the two gated fakes (unique fixture addresses).
    const OC1543_MAGIC: u32 = 0x1543_0001;
    const OC1543_DELETE_CTRL: u32 = DATA + 0x2e_5000;
    const OC1543_INIT_CTRL: u32 = DATA + 0x2e_6000;

    /// `GameStateEventSwitchLoad`'s globals (`D_00535BC8`).
    const OC1543_SWITCH_GLOBALS: u32 = 0x0053_5bc8;

    static OC1543_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);
    static OC1543_PREV_STATIC_INIT: AtomicUsize = AtomicUsize::new(0);

    /// `__builtin_delete`: a logged no-op while the gate is on.
    fn oc1543_fake_delete(c: &mut Cpu) {
        if c.bus.read32(OC1543_DELETE_CTRL) != OC1543_MAGIC {
            return fakes::fall_back(c, &OC1543_PREV_DELETE, ee::s10::f_00317e50__generated);
        }
        fakes::fake_call(c, 0x0031_7e50);
    }

    /// `cOVStateOverlay_StaticInitDestroy`: a logged no-op while the gate is on.
    fn oc1543_fake_static_init(c: &mut Cpu) {
        if c.bus.read32(OC1543_INIT_CTRL) != OC1543_MAGIC {
            return fakes::fall_back(
                c,
                &OC1543_PREV_STATIC_INIT,
                ee::s05::f_002127e8__generated,
            );
        }
        fakes::fake_call(c, 0x0021_27e8);
    }

    fn oc1543_hook_delete() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1543_fake_delete,
            &OC1543_PREV_DELETE,
        );
    }

    fn oc1543_hook_static_init() {
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_002127E8),
            oc1543_fake_static_init,
            &OC1543_PREV_STATIC_INIT,
        );
    }

    /// A UI-state constructor: the base `UIState_ctor` is a real port, so the
    /// plain machine is enough. `self` lives at `DATA + 0x2000`.
    fn ui_state_ctor_2_case(generated: GuestFn, hand: GuestFn, entry: u32) {
        check(generated, hand, 200, &|c, seed| {
            machine(c, seed, entry as u64, 0);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.pc = entry;
        });
    }

    #[test]
    fn cfe_state_lodge_ctor_2_matches() {
        ui_state_ctor_2_case(
            ee::s05::f_00220d38__generated,
            cfe_state_lodge_ctor_2,
            0x0022_0d38,
        );
    }

    #[test]
    fn cfe_state_lodge_rider_detail_ctor_2_matches() {
        ui_state_ctor_2_case(
            ee::s05::f_00220d98__generated,
            cfe_state_lodge_rider_detail_ctor_2,
            0x0022_0d98,
        );
    }

    #[test]
    fn cfe_state_buy_attrib_ctor_2_matches() {
        ui_state_ctor_2_case(
            ee::s05::f_00220df8__generated,
            cfe_state_buy_attrib_ctor_2,
            0x0022_0df8,
        );
    }

    #[test]
    fn cfe_state_career_stats_ctor_2_matches() {
        ui_state_ctor_2_case(
            ee::s05::f_00220e58__generated,
            cfe_state_career_stats_ctor_2,
            0x0022_0e58,
        );
    }

    /// `cOVStateHUD1P_ctor_2` runs two real `HUDMeter_ctor`s, so the allocator
    /// fixture is needed.
    #[test]
    fn cov_state_hud1p_ctor_2_matches() {
        check(
            ee::s05::f_0021cba0__generated,
            cov_state_hud1p_ctor_2,
            200,
            &|c, seed| {
                machine_rt(c, seed, 0x21cb, 0);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.pc = 0x0021_cba0;
            },
        );
    }

    #[test]
    fn cov_state_overlay_global_ctors_matches() {
        oc1543_hook_static_init();
        check(
            ee::s05::f_00222428__generated,
            cov_state_overlay_global_ctors,
            100,
            &|c, seed| {
                machine(c, seed, 0x2224, 0);
                c.bus.write32(OC1543_INIT_CTRL, OC1543_MAGIC);
                c.pc = 0x0022_2428;
            },
        );
    }

    /// The list is a ring of `seed % 4` nodes (0 means empty), each node's
    /// word 0 pointing at the next and the last at the head.
    #[test]
    fn ssx_app_list_clear_matches() {
        oc1543_hook_delete();
        check(
            ee::s06::f_00242e58__generated,
            ssx_app_list_clear,
            200,
            &|c, seed| {
                machine(c, seed, 0x242e, 0);
                c.bus.write32(OC1543_DELETE_CTRL, OC1543_MAGIC);
                let this = DATA + 0x2000;
                let head = DATA + 0x2100;
                let nodes = seed % 4;
                for k in 0..nodes {
                    let node = DATA + 0x2200 + 0x40 * k;
                    let next = if k + 1 < nodes {
                        node + 0x40
                    } else {
                        head
                    };
                    c.bus.write32(node, next);
                }
                let first = if nodes == 0 { head } else { DATA + 0x2200 };
                c.bus.write32(head, first);
                c.bus.write32(this + 0x4, head);
                c.g[A0][0] = this as u64;
                c.pc = 0x0024_2e58;
            },
        );
    }

    /// The generated loop runs `seed % 4` times (0 skips it), and the
    /// `lb` flag at `+0x49` takes the early exit on every third seed.
    #[test]
    fn game_state_event_switch_load_enter_matches() {
        check(
            ee::s06::f_002368a0__generated,
            game_state_event_switch_load_enter,
            200,
            &|c, seed| {
                machine_rt(c, seed, 0x2368, 0);
                c.bus.write32(GAME_GP.wrapping_sub(0x488), DATA + 0x3000);
                c.bus.write32(DATA + 0x3028, DATA + 0x3100);
                c.bus.write8(OC1543_SWITCH_GLOBALS + 0x49, (seed % 3 == 0) as u8);
                c.bus.write32(OC1543_SWITCH_GLOBALS + 0x30, seed % 4);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.pc = 0x0023_68a0;
            },
        );
    }
}
