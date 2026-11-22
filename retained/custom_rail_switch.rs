//! Native RailMan_SetRailEnabled (0x00358b28), retained uninstalled.
//!
//! Generated EE is the authority. No tests, builds or replays were run.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0035_8b28,
    rail_man_set_rail_enabled,
    "RailMan_SetRailEnabled",
)];

const MANAGER_GP: u32 = 0x0f50;
const SLOT_BASE: u32 = 0x18;
const SLOT_COUNT: i64 = 128;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `void RailMan_SetRailEnabled(u32 key, int enabled)` (0x358b28).
///
/// Lazily construct the native rail manager, then search its 128 raw-key
/// slots, remembering the first free (-1) slot. Enabling only acts on an
/// already present key; disabling only acts on a new key with a free slot.
/// The enable/disable helpers receive `(manager, slotIndex)`. This function
/// never resolves `key` as an instance RID and applies no section/index mask;
/// the helpers own the key's downstream resource interpretation.
pub fn rail_man_set_rail_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.lw(r(c, GP).wrapping_add(MANAGER_GP));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 9;
            c.g[A1][0] = 0x004a_0000;
            c.g[A0][0] = 0x218;
            c.g[A1][0] = sx(r(c, A1).wrapping_add(0x4048));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0035_8b60)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0035_84f0, 0x0035_8b6c)?;
        }

        c.cycles += 9;
        c.g[A0][0] = c.lw(r(c, GP).wrapping_add(MANAGER_GP));
        c.g[A3][0] = 0xffff_ffff_ffff_0000;
        c.g[A1][0] = u64::MAX;
        c.g[A2][0] = 0;
        c.g[T1][0] = u64::MAX;
        c.g[A3][0] |= 0xffff;
        c.g[T0][0] = c.g[S1][0];
        c.g[V1][0] = sx(r(c, A0).wrapping_add(SLOT_BASE));
        loop {
            c.cycles += 2;
            let have_slot = c.g[A1][0] != c.g[T1][0];
            c.g[V0][0] = c.lw(r(c, V1));
            if !have_slot {
                c.cycles += 3;
                c.g[V0][0] ^= c.g[A3][0];
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = c.g[A2][0];
                }
                c.g[V0][0] = c.lw(r(c, V1));
            }
            c.cycles += 2;
            if c.g[V0][0] == c.g[T0][0] {
                c.g[A1][0] = c.g[A2][0];
                break;
            }
            c.cycles += 4;
            c.g[A2][0] = sx(r(c, A2).wrapping_add(1));
            c.g[V0][0] = ((c.g[A2][0] as i64) < SLOT_COUNT) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r(c, V1).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_8b90);
            }
        }

        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        let no_slot = c.g[A1][0] == c.g[V0][0];
        c.g[V0][0] = sx(r(c, A1) << 2);
        let mut restore_s0_without_charge = false;
        if !no_slot {
            c.cycles += 5;
            c.g[A2][0] = c.g[V0][0];
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
            c.g[V1][0] = c.lw(r(c, V0).wrapping_add(SLOT_BASE));
            let same_key = c.g[V1][0] == c.g[S1][0];
            let should_change;
            if same_key {
                c.cycles += 2;
                let disabled = c.g[S0][0] == 0;
                c.g[V0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
                should_change = !disabled;
                if should_change {
                    c.cycles += 2;
                }
            } else {
                c.cycles += 2;
                should_change = c.g[S0][0] == 0;
                if should_change {
                    c.cycles += 1;
                    c.g[V0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
                } else {
                    restore_s0_without_charge = true;
                }
            }
            if should_change {
                c.cycles += 2;
                let disable = c.g[S0][0] == 0;
                c.sw(r(c, V0).wrapping_add(SLOT_BASE), c.g[S1][0]);
                if disable {
                    c.cycles += 2;
                    call_ptr(c, 0x0035_8780, 0x0035_8c18)?;
                } else {
                    c.cycles += 4;
                    call_ptr(c, 0x0035_8998, 0x0035_8c08)?;
                    restore_s0_without_charge = true;
                }
            }
        }
        if !restore_s0_without_charge {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
