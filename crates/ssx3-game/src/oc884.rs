//! Staging module oc884 (wave wave252): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc884`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x001b_54b8, cfestatenetconfig_v25, "cFEStateNetConfig_v25")];

/// `GetHashValue32(s)` (0x317670) for a name at `s`, returning to `ret`.
fn oc884_hash32(c: &mut Cpu, s: u32, ret: u32) -> crate::call::Flow {
    c.g[A0][0] = sx(s);
    call_ptr(c, 0x0031_7670, ret)
}

/// `cFEStateNetConfig::v25(cMenu* menu)` (0x1b54b8): set the help option count
/// of the menu to 5, store the hash of one string in the config at +0x54, then
/// add the menu's option entries (hashed names, each with its slot id).
pub fn cfestatenetconfig_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 62;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        // cFEStateNetHelpPopup_setNumHelpOptions(menu, 5)
        c.g[A1][0] = 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_a528, 0x001b_54d4)?;
        c.g[A0][0] = 0x0046_0000;
        oc884_hash32(c, 0x0046_4030, 0x001b_54e0)?;
        Addr((c.g[S0][0] as u32).wrapping_add(0x54)).sw(c, c.g[V0][0]);
        c.g[A0][0] = 0x0046_0000;
        oc884_hash32(c, 0x0046_4048, 0x001b_54f0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x001d_a530, 0x001b_5500)?;
        c.g[A0][0] = 0x0046_0000;
        oc884_hash32(c, 0x0045_fe60, 0x001b_550c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A3][0] = 3;
        call_ptr(c, 0x001d_a510, 0x001b_5520)?;
        c.g[A0][0] = 0x0046_0000;
        oc884_hash32(c, 0x0045_fe40, 0x001b_552c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[A3][0] = 4;
        call_ptr(c, 0x001d_a510, 0x001b_5540)?;
        c.g[A0][0] = 0x0046_0000;
        oc884_hash32(c, 0x0046_2c50, 0x001b_554c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        c.g[A3][0] = 0;
        call_ptr(c, 0x001d_a510, 0x001b_5560)?;
        c.g[A0][0] = 0x0046_0000;
        oc884_hash32(c, 0x0046_4068, 0x001b_556c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        c.g[A3][0] = 2;
        call_ptr(c, 0x001d_a510, 0x001b_5580)?;
        c.g[A0][0] = 0x0046_0000;
        oc884_hash32(c, 0x0046_14e8, 0x001b_558c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 4;
        c.g[A3][0] = 1;
        call_ptr(c, 0x001d_a510, 0x001b_55a0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The session and lobby part of the disconnect popup (`'b_1b5788` ..
/// `'b_1b57b4` and the calls after them in 0x1b5740): make sure the session
/// object exists, reset its lobby, then clear the pair settings' static flag.
#[allow(dead_code)]
// Dropped from OVERRIDES: its session callees (0x25b800, 0x2636f0, 0x2591b8,
// 0x2613c8, 0x260f80) have no override slot to fake and run away on test data.
fn oc884_disconnect_main(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x238)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        call_ptr(c, 0x0026_0f80, 0x001b_5788)?;
    }
    c.cycles += 4;
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x238)).lw(c);
    call_ptr(c, 0x0026_1460, 0x001b_5790)?;
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x238)).lw(c);
    if !t {
        c.cycles += 3;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x238)).lw(c);
        call_ptr(c, 0x0026_13c8, 0x001b_57a0)?;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x238)).lw(c);
    }
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
        call_ptr(c, 0x0025_b800, 0x001b_57b4)?;
    }
    c.cycles += 22;
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x238)).lw(c);
    call_ptr(c, 0x0026_1408, 0x001b_57bc)?;
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
    call_ptr(c, 0x0025_b800, 0x001b_57c4)?;
    call_ptr(c, 0x0025_91b8, 0x001b_57cc)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0026_36f0, 0x001b_57d8)?;
    // pair settings->setStatic(0), through the vtable slot at +0x40.
    c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x6ec)).lw(c);
    c.g[A1][0] = 0;
    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
    let slot = VSlot::read(c, c.g[V0][0], 0x40);
    c.g[A0][0] = slot.delta;
    c.g[V1][0] = slot.func;
    let target = slot.func as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001b_57f4)?;
    // cFEStateNet_onPopupEvent(self, popup, event)
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S0][0];
    Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    }
    .call(c, 0x001b_5804)?;
    frame.restore(c, S0, 0x30);
    Ok(())
}

/// `cFEStateNetConfig::OnDisconnectPopupEvent(self, popup, event)` (0x1b5740):
/// for events 0xF and 20 (the disconnect popup's own) it resets the session
/// and its lobby, then forwards to the base popup handler; other events only
/// forward.
#[allow(dead_code)]
// Dropped from OVERRIDES (see oc884_disconnect_main); port kept, untested.
pub fn cfestatenetconfig_on_disconnect_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        let main = if c.g[S0][0] == 0xf {
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x14;
            c.g[A0][0] = c.g[S1][0];
            c.g[S0][0] == 0x14
        };
        if main {
            oc884_disconnect_main(c, frame)?;
        } else {
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            Callee {
                addr: 0x001a_97b8,
                f: ee::s03::NetFEState_OnChildEvent,
            }
            .call(c, 0x001b_5818)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, DATA};

    #[test]
    fn cfestatenetconfig_v25_matches() {
        check(
            ee::s03::f_001b54b8__generated,
            cfestatenetconfig_v25,
            60,
            &|c, seed| {
                machine(c, seed, 0x1b54_b8, 0);
                c.g[A1][0] = sx(DATA);
            },
        );
    }
}
