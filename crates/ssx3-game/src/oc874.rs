//! Staging module oc874 (wave wave251): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc874`.

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
        0x001a_d178,
        select_persona_init,
        "cFEStateSelectPersona_Init",
    ),
    (
        0x001a_d1e8,
        select_persona_update,
        "cFEStateSelectPersona_Update",
    ),
    (
        0x001a_d208,
        select_persona_submit,
        "cFEStateSelectPersona_SubmitPersona",
    ),
    (
        0x001a_d3e8,
        select_persona_fill_list,
        "cFEStateSelectPersona_FillPersonaList",
    ),
    (0x001a_d4f0, select_persona_v25, "cFEStateSelectPersona_v25"),
    (
        0x001a_d8c0,
        select_persona_on_child_event,
        "cFEStateSelectPersona_OnChildEvent",
    ),
];

/// `base + off` as a guest address (`addiu`), from a 64-bit register value.
#[inline(always)]
fn oc874_at(base: u64, off: u32) -> Addr {
    Addr((base as u32).wrapping_add(off))
}

/// `cFEStateSelectPersona_Init(self)` (0x1ad178): the persona screen's init.
/// Looks up the screen by its name hash and, when that finds a screen, plays
/// it; then runs the base `NetFEState_Init` on `self`.
pub fn select_persona_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 17, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, 16, 0x20);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x2dc8));
        c.g[S0][0] = oc874_at(c.g[S1][0], 0x10).lw(c);
        call_ptr(c, 0x0031_7670, 0x001a_d19c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001a_d1b0)?;
        let found = c.g[V0][0] != 0;
        let v = c.g[V0][0];
        oc874_at(c.g[S1][0], 0x40).sw(c, v);
        if found {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001a_d1c8)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001a_8768, 0x001a_d1d0)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateSelectPersona_Update(self)` (0x1ad1e8): the base `NetFEState_Update`.
pub fn select_persona_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x001a_8770, 0x001a_d1f8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateSelectPersona_SubmitPersona(self, msg)` (0x1ad208): the persona
/// choice. When the query at `0x416810` is below 3 it opens the confirm popup
/// (its option texts, then the popup's show call); otherwise it shows the
/// other popup with the persona manager's first text. Both end by running
/// the shared epilogue.
pub fn select_persona_submit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 16, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0041_6810, 0x001a_d22c)?;
        let small = (c.g[V0][0] as i64) < 3;
        c.g[V0][0] = small as u64;
        if small {
            c.cycles += 26;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x001a_9c50, 0x001a_d240)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x15f8));
            c.g[V0][0] = 1;
            let v = c.g[V0][0];
            oc874_at(c.g[S0][0], 0xbc).sw(c, v);
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
            call_ptr(c, 0x0031_7670, 0x001a_d25c)?;
            c.g[V1][0] = 3;
            c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x1618));
            let v = c.g[V1][0];
            oc874_at(c.g[S0][0], 0xc0).sw(c, v);
            let v = c.g[V0][0];
            oc874_at(c.g[S0][0], 0xc4).sw(c, v);
            call_ptr(c, 0x0031_7670, 0x001a_d274)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x001c_57c0, 0x001a_d284)?;
            c.g[V0][0] = 0x814;
            c.g[A0][0] = c.g[S2][0];
            let v = c.g[V0][0];
            oc874_at(c.g[S0][0], 0x150).sw(c, v);
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x001a_9bc0, 0x001a_d298)?;
            frame.restore(c, 16, 0x30);
        } else {
            c.cycles += 20;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x001a_9c50, 0x001a_d2a8)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x2de0));
            oc874_at(c.g[S0][0], 0xbc).sw(c, 0);
            call_ptr(c, 0x0031_7670, 0x001a_d2bc)?;
            c.g[V1][0] = 3;
            c.g[A2][0] = 0x815;
            oc874_at(c.g[S0][0], 0x164).sw(c, 0);
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S0][0];
            let v = c.g[V0][0];
            oc874_at(c.g[S0][0], 0xc4).sw(c, v);
            let v = c.g[V1][0];
            oc874_at(c.g[S0][0], 0xc0).sw(c, v);
            let v = c.g[A2][0];
            oc874_at(c.g[S0][0], 0x150).sw(c, v);
            call_ptr(c, 0x001a_9bc0, 0x001a_d2e0)?;
            c.g[A0][0] = oc874_at(c.g[GP][0], 0u32.wrapping_sub(0xc8)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0025_d1b8, 0x001a_d2ec)?;
            frame.restore(c, 16, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateSelectPersona_FillPersonaList(self, list)` (0x1ad3e8): fill the
/// list box with one entry per persona in the global persona manager (its
/// count at `+0x6c`, its id array at `+0x70`), giving each entry its index,
/// then the four fixed persona slots.
pub fn select_persona_fill_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 19, 0x20);
        frame.save(c, 17, 0x40);
        frame.save(c, 20, 0x10);
        c.g[S1][0] = 0;
        frame.save(c, 16, 0x50);
        frame.save(c, 18, 0x30);
        frame.save_ra(c, 0);
        c.g[S3][0] = oc874_at(c.g[GP][0], 0u32.wrapping_sub(0xc8)).lw(c);
        c.g[V0][0] = oc874_at(c.g[S3][0], 0x6c).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S4][0] = c.g[A1][0];
        if !empty {
            c.cycles += 2;
            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x70));
            c.g[A1][0] = c.g[S1][0];
            loop {
                c.cycles += 19;
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x74));
                call_ptr(c, 0x0039_7870, 0x001a_d428)?;
                c.g[S0][0] = c.g[V0][0];
                let v = c.g[S1][0];
                oc874_at(c.g[S0][0], 0x18).sw(c, v);
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[A1][0] = oc874_at(c.g[S2][0], 0).lw(c);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
                call_ptr(c, 0x003a_0c50, 0x001a_d444)?;
                c.g[V0][0] = oc874_at(c.g[S0][0], 8).lw(c);
                c.g[A1][0] = 1;
                c.g[A0][0] = oc874_at(c.g[V0][0], 0x48).lh(c);
                c.g[V1][0] = oc874_at(c.g[V0][0], 0x4c).lw(c);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let target = c.g[V1][0] as u32;
                call_ptr(c, target, 0x001a_d45c)?;
                c.g[V0][0] = oc874_at(c.g[S3][0], 0x6c).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001a_d420);
                }
            }
        }
        // The four fixed persona slots.
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S1][0] as i64) < 4) as u64;
        let slots_left = c.g[V0][0] != 0;
        c.g[S2][0] = 0x0046_0000;
        if slots_left {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            loop {
                c.cycles += 19;
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x74));
                call_ptr(c, 0x0039_7870, 0x001a_d488)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2e58));
                let v = c.g[S1][0];
                oc874_at(c.g[S0][0], 0x18).sw(c, v);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                call_ptr(c, 0x0031_7670, 0x001a_d49c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003a_1000, 0x001a_d4a8)?;
                c.g[V0][0] = oc874_at(c.g[S0][0], 8).lw(c);
                c.g[A1][0] = 1;
                c.g[A0][0] = oc874_at(c.g[V0][0], 0x48).lh(c);
                c.g[V1][0] = oc874_at(c.g[V0][0], 0x4c).lw(c);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let target = c.g[V1][0] as u32;
                call_ptr(c, target, 0x001a_d4c0)?;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 4) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001a_d480);
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateSelectPersona_v25(self, a1)` (0x1ad4f0): the two-row popup's
/// text and option setup: a run of text lookups and `0x1da510`/`0x1da530`
/// widget calls on `self`.
pub fn select_persona_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 61;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x4;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_a528, 0x001a_d50c)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x2e70));
        call_ptr(c, 0x0031_7670, 0x001a_d518)?;
        let v = c.g[V0][0];
        oc874_at(c.g[S0][0], 0x54).sw(c, v);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x2e88));
        call_ptr(c, 0x0031_7670, 0x001a_d528)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x001d_a530, 0x001a_d538)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x2ea0));
        call_ptr(c, 0x0031_7670, 0x001a_d544)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x001d_a530, 0x001a_d554)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_sub(0x1a0));
        call_ptr(c, 0x0031_7670, 0x001a_d560)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A3][0] = 3;
        call_ptr(c, 0x001d_a510, 0x001a_d574)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x2eb8));
        call_ptr(c, 0x0031_7670, 0x001a_d580)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[A3][0] = 2;
        call_ptr(c, 0x001d_a510, 0x001a_d594)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x2c50));
        call_ptr(c, 0x0031_7670, 0x001a_d5a0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        c.g[A3][0] = 0;
        call_ptr(c, 0x001d_a510, 0x001a_d5b4)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x14e8));
        call_ptr(c, 0x0031_7670, 0x001a_d5c0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        c.g[A3][0] = 1;
        call_ptr(c, 0x001d_a510, 0x001a_d5d4)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateSelectPersona_OnChildEvent(self, popup, msg)` (0x1ad8c0): the
/// popup's messages. Popup code 0x816 calls the popup's own handler through
/// its vtable slot; 0x819 forwards to `cFEStateSelectPersona_OnSaveAccountPopupEvent`;
/// anything else goes to the base `NetFEState_OnChildEvent`.
pub fn select_persona_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x816;
        c.g[V1][0] = oc874_at(c.g[A1][0], 0x18).lw(c);
        let is_816 = c.g[V1][0] == c.g[V0][0];
        c.g[A3][0] = c.g[A0][0];
        if is_816 {
            c.cycles += 7;
            c.g[V0][0] = oc874_at(c.g[A3][0], 8).lw(c);
            c.g[A0][0] = oc874_at(c.g[V0][0], 0x108).lh(c);
            c.g[V1][0] = oc874_at(c.g[V0][0], 0x10c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001a_d900)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x819;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 4;
                c.g[A0][0] = c.g[A3][0];
                call_ptr(c, 0x001a_d930, 0x001a_d910)?;
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 2;
                c.cycles += 3;
                c.g[A0][0] = c.g[A3][0];
                call_ptr(c, 0x001a_97b8, 0x001a_d920)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// A scrambled machine at `entry`, with GP as the game has it.
    fn oc874_machine(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = Rng(0x0874_5eed ^ ((seed as u64) << 16) ^ entry as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.pc = entry;
    }

    #[test]
    fn select_persona_init_matches() {
        check(
            ee::s03::f_001ad178__generated,
            select_persona_init,
            40,
            &|c, seed| {
                oc874_machine(c, seed, 0x001a_d178);
                let obj = DATA + 0x1000;
                c.g[A0][0] = obj as u64;
                c.bus.write32(obj + 0x10, 0x0080_2000 + seed);
            },
        );
    }

    #[test]
    fn select_persona_update_matches() {
        check(
            ee::s03::f_001ad1e8__generated,
            select_persona_update,
            40,
            &|c, seed| {
                oc874_machine(c, seed, 0x001a_d1e8);
                c.g[A0][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    #[test]
    fn select_persona_submit_matches() {
        check(
            ee::s03::f_001ad208__generated,
            select_persona_submit,
            40,
            &|c, seed| {
                oc874_machine(c, seed, 0x001a_d208);
                let obj = DATA + 0x1000;
                let msg = DATA + 0x2000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = msg as u64;
                c.bus.write32(GAME_GP - 0xc8, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn select_persona_fill_list_matches() {
        check(
            ee::s03::f_001ad3e8__generated,
            select_persona_fill_list,
            40,
            &|c, seed| {
                oc874_machine(c, seed, 0x001a_d3e8);
                let mgr = DATA + 0x1000;
                let list = DATA + 0x5000;
                c.g[A0][0] = (DATA + 0x4000) as u64;
                c.g[A1][0] = list as u64;
                c.bus.write32(GAME_GP - 0xc8, mgr);
                // Persona count: 0 (empty), then up to 6 entries.
                c.bus.write32(mgr + 0x6c, seed % 7);
                for i in 0..6 {
                    c.bus.write32(mgr + 0x70 + 4 * i, 0x0080_7000 + i);
                }
            },
        );
    }

    #[test]
    fn select_persona_v25_matches() {
        check(
            ee::s03::f_001ad4f0__generated,
            select_persona_v25,
            30,
            &|c, seed| {
                oc874_machine(c, seed, 0x001a_d4f0);
                c.g[A1][0] = (DATA + 0x1000 + (seed % 4) * 0x100) as u64;
            },
        );
    }

    #[test]
    fn select_persona_on_child_event_matches() {
        check(
            ee::s03::f_001ad8c0__generated,
            select_persona_on_child_event,
            40,
            &|c, seed| {
                oc874_machine(c, seed, 0x001a_d8c0);
                let obj = DATA + 0x1000;
                let msg = DATA + 0x2000;
                let inner = DATA + 0x3000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = msg as u64;
                c.g[A2][0] = (seed % 5) as u64;
                // The popup code: 0x816, 0x819, or another value.
                let code = [0x816, 0x819, 0x817][(seed % 3) as usize];
                c.bus.write32(msg + 0x18, code);
                c.bus.write32(obj + 8, inner);
                // Leaf (returns 0) as the vtable slot function.
                c.bus.write32(inner + 0x10c, 0x0014_06c0);
            },
        );
    }
}
