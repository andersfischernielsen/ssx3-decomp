//! Staging module oc872 (wave wave251): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc872`.

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
    0x001a_a9b0,
    net_fe_state_update_message_hints,
    "NetFEState_UpdateMessageHints",
)];

/// `void NetFEState_UpdateMessageHints(NetFEState* self)` (0x1aa9b0): the
/// challenge and message hint icons blink (`(now - frame) % 60 > 30`) while
/// their frame word is set and are hidden otherwise; the message count text
/// follows the message icon. `now` is `cAppMan_TheAppMan->TimerTick` (+0x18).
/// Offsets: `self` +0x40 screen, +0x50 message frame, +0x54 challenge frame.
pub fn net_fe_state_update_message_hints(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[A0][0] = 0x0046_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2638));
        frame.save(c, S3, 0x10);
        c.g[S3][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001a_a9e0)?;

        // Challenge hint icon.
        c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001a_a9ec)?;
        c.g[A3][0] = c.g[V0][0];
        let icon_missing = c.g[A3][0] == 0;
        c.g[A0][0] = 0x0046_0000;
        if !icon_missing {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x54)).lw(c);
            let frame_clear = c.g[V0][0] == 0;
            c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            if frame_clear {
                // setVisible(0) through the vtable.
                c.cycles += 7;
                c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001a_aa54)?;
            } else {
                // setVisible((now - frame) % 60 > 30) through the vtable.
                c.cycles += 5;
                c.g[A2][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[V0][0] = 0x3c;
                c.divu(0, 3, 2);
                c.cycles += 9;
                c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x48)).lh(c);
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4c)).lw(c);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[A1][0] = c.hi[0];
                c.g[A1][0] = (c.g[A1][0] < 0x1f) as u64;
                let target = c.g[V0][0] as u32;
                c.g[A1][0] ^= 0x1;
                call_ptr(c, target, 0x001a_aa34)?;
            }
            c.g[A0][0] = 0x0046_0000;
        }

        // Message count text and message hint icon.
        c.cycles += 15;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2650));
        call_ptr(c, 0x0031_7670, 0x001a_aa60)?;
        c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001a_aa6c)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2668));
        call_ptr(c, 0x0031_7670, 0x001a_aa7c)?;
        c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001a_aa88)?;
        c.g[A3][0] = c.g[V0][0];
        let msg_icon_missing = c.g[A3][0] == 0;
        frame.restore(c, S0, 0x40);
        if !msg_icon_missing {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x50)).lw(c);
            let frame_clear = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            if frame_clear {
                // Message icon hidden, then the count text hidden.
                c.cycles += 8;
                c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001a_ab2c)?;
                let count_missing = c.g[S1][0] == 0;
                c.g[A1][0] = 0;
                if !count_missing {
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x8)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001a_ab48)?;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x40);
            } else {
                // Message icon blinks; the count text follows it.
                c.cycles += 5;
                c.g[S0][0] = 0x3c;
                c.divu(0, 2, 16);
                c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x8)).lw(c);
                c.cycles += 9;
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x48)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4c)).lw(c);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[A1][0] = c.hi[0];
                c.g[A1][0] = (c.g[A1][0] < 0x1f) as u64;
                let target = c.g[V0][0] as u32;
                c.g[A1][0] ^= 0x1;
                call_ptr(c, target, 0x001a_aad0)?;
                let count_missing = c.g[S1][0] == 0;
                c.g[V1][0] = 0x3c;
                if count_missing {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x40);
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x50)).lw(c);
                    c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x8)).lw(c);
                    c.cycles += 11;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.divu(0, 2, 16);
                    c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x48)).lh(c);
                    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4c)).lw(c);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A1][0] = c.hi[0];
                    c.g[A1][0] = (c.g[A1][0] < 0x1f) as u64;
                    let target = c.g[V0][0] as u32;
                    c.g[A1][0] ^= 0x1;
                    call_ptr(c, target, 0x001a_ab0c)?;
                    frame.restore(c, S0, 0x40);
                }
            }
        }

        // Epilogue.
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fakes, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word and its mask / lookup counter (both fakes' state).
    const OC872_CTRL: u32 = DATA + 0x5000;
    const OC872_MAGIC: u32 = 0x0872_0872;
    /// The `NetFEState` object, the cAppMan object, the screen.
    const OC872_SELF: u32 = DATA + 0x100;
    const OC872_APP: u32 = DATA + 0x4000;
    const OC872_SCREEN: u32 = DATA + 0x6000;
    /// Three hint objects, 0x40 apart, sharing one vtable.
    const OC872_OBJ: u32 = DATA + 0x2000;
    const OC872_VT: u32 = DATA + 0x3000;

    static OC872_PREV_FIND: AtomicUsize = AtomicUsize::new(0);

    /// Stand-in for `cUIScreen_getObjectByHashName` (0x39d860): the k-th
    /// lookup of a run returns hint object k when bit k of the mask is set,
    /// else null. Gate off, the slot runs the body it held.
    fn oc872_fake_find(c: &mut Cpu) {
        if c.bus.read32(OC872_CTRL) != OC872_MAGIC {
            return fakes::fall_back(c, &OC872_PREV_FIND, ee::s13::f_0039d860__generated);
        }
        let k = c.bus.read32(OC872_CTRL + 8);
        c.bus.write32(OC872_CTRL + 8, k + 1);
        let mask = c.bus.read32(OC872_CTRL + 4);
        c.g[V0][0] = if k < 3 && (mask >> k) & 1 != 0 {
            (OC872_OBJ + 0x40 * k) as u64
        } else {
            0
        };
        c.cycles += 5;
        ret(c);
    }

    /// Random timer and frames around `now`; each branch by `seed`.
    fn oc872_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x872, 0);
        let now = r.next_u32();
        c.bus.write32(GAME_GP.wrapping_add(0x2a74), OC872_APP);
        c.bus.write32(OC872_APP + 0x18, now);
        let msg = if seed % 4 == 0 {
            0
        } else {
            now.wrapping_sub(r.next_u32() % 200)
        };
        let chal = if seed % 3 == 0 {
            0
        } else {
            now.wrapping_sub(r.next_u32() % 200)
        };
        c.bus.write32(OC872_SELF + 0x40, OC872_SCREEN);
        c.bus.write32(OC872_SELF + 0x50, msg);
        c.bus.write32(OC872_SELF + 0x54, chal);
        c.g[A0][0] = sx(OC872_SELF);
        for k in 0..3 {
            c.bus.write32(OC872_OBJ + 0x40 * k + 8, OC872_VT);
        }
        c.bus.write16(OC872_VT + 0x48, 0);
        c.bus.write32(OC872_VT + 0x4c, 0x0014_06c0);
        c.bus.write32(OC872_CTRL, OC872_MAGIC);
        c.bus.write32(OC872_CTRL + 4, seed % 8);
        c.bus.write32(OC872_CTRL + 8, 0);
        c.pc = 0x001a_a9b0;
    }

    #[test]
    fn net_fe_state_update_message_hints_matches() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc872_fake_find,
            &OC872_PREV_FIND,
        );
        check(
            ee::s03::f_001aa9b0__generated,
            net_fe_state_update_message_hints,
            240,
            &oc872_setup,
        );
    }
}
