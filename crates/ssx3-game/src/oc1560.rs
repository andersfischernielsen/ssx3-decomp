//! Staging module oc1560 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1560`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002c_faf8,
    menu_sys_group_item_v09,
    "MenuSysGroupItem_v09",
)];

/// `MenuSysGroupItem_v09` (0x002cfaf8): a forwarding call on the menu object.
/// The object is `*a0`; its vtable is at `+0x12c`. The call goes to the
/// vtable's `+0x1c` entry with `this = obj + delta`, where `delta` is the
/// halfword at `vtable + 0x18`. `a1` and `a2` pass through unchanged.
pub fn menu_sys_group_item_v09(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = Addr(r32(c, A0)).lw(c);
        c.g[V1][0] = Addr(r32(c, A3)).off(0x12c).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x18).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002c_fb18)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{machine, vtable};
    use crate::difftest::DATA;

    #[test]
    fn menu_sys_group_item_v09_matches() {
        check(
            ee::s09::f_002cfaf8__generated,
            menu_sys_group_item_v09,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002c_faf8, 1);
                let obj = DATA + 0x8000;
                c.bus.write32(DATA, obj);
                c.g[A0][0] = sx(DATA);
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(obj + 0x12c, vt);
            },
        );
    }
}
