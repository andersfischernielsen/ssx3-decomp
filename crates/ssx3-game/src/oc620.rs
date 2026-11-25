//! Staging module oc620 (wave 234): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc620`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0012_8680, oc620_player_load_state, "Player_LoadState"),
    (0x0012_8818, oc620_ai_init, "AI_Init"),
    (0x0012_8ac0, oc620_ai_setup_mission_rider, "AI_SetupMissionRider"),
    (0x0012_a1f0, oc620_ai_is_rider_in_special_list, "AI_IsRiderInSpecialList"),
];

/// `void Player_LoadState(void)` (0x128680): forwards to
/// `RiderBase_LoadState` (0x111d98) in a 16-byte frame that holds `ra`.
pub fn oc620_player_load_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(Callee {
            addr: 0x0011_1d98,
            f: ee::s00::f_00111d98,
        }),
        0x0012_8690,
    );
}

/// `void AI_Init(cAI* self)` (0x128818): reset the focus and danger managers,
/// clear the AI's counters and flags, set the race state to 1, zero the
/// per-rider word tables (`self+0x20`..`+0x70`, `+0x3c`..`+0x28`), load the
/// animation data sources, call the hook at `self->+0xcc` (table entry
/// +0x34, argument `+0x30` of that block as an offset into `self`), and reset
/// the riders.
pub fn oc620_ai_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];

        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0xa4)).lw(c);
        Callee {
            addr: 0x0010_3358,
            f: ee::s00::f_00103358,
        }
        .call(c, 0x0012_8834)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0xa8)).lw(c);
        Callee {
            addr: 0x0010_1688,
            f: ee::s00::f_00101688,
        }
        .call(c, 0x0012_883c)?;

        Addr(r32(c, S0).wrapping_add(0x10)).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        Addr(r32(c, S0).wrapping_add(0x14)).sw(c, 0);
        c.g[A1][0] = 1;
        Addr(r32(c, S0).wrapping_add(0x90)).sw(c, 0);
        Addr(r32(c, S0).wrapping_add(0x78)).sw(c, 0);
        Addr(r32(c, S0).wrapping_add(0x7c)).sw(c, 0);
        Addr(r32(c, S0).wrapping_add(0x80)).sw(c, 0);
        Addr(r32(c, S0).wrapping_add(0x84)).sw(c, 0);
        Addr(r32(c, S0).wrapping_add(0x88)).sw(c, 0);
        Addr(r32(c, S0).wrapping_add(0x1c)).sw(c, 0);
        Addr(r32(c, S0).wrapping_add(0x98)).sw(c, 0);
        Addr(r32(c, S0)).sw(c, 0);
        Callee {
            addr: 0x0011_3b10,
            f: ee::s00::f_00113b10,
        }
        .call(c, 0x0012_8874)?;

        // Loop 1: six words at self+0x3c, walking down.
        c.g[V1][0] = 5;
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(0x3c));
        loop {
            c.cycles += 7;
            Addr(r32(c, V0)).sw(c, 0);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0012_8880);
            }
        }

        c.cycles += 5;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x48));
        c.g[S1][0] = sx(r32(c, S0).wrapping_add(0x20));
        c.g[V1][0] = 0;
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(0x70));
        // Loop 2: one word at self+0x70 (the counter starts at 0).
        loop {
            c.cycles += 7;
            Addr(r32(c, V0)).sw(c, 0);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0012_88b0);
            }
        }

        // Loop 3: five words at (self+0x48)+0x10, walking down.
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x10));
        c.g[V1][0] = 4;
        loop {
            c.cycles += 7;
            Addr(r32(c, V0)).sw(c, 0);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0012_88d8);
            }
        }

        c.cycles += 13;
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0012_b280,
            f: ee::s00::f_0012b280,
        }
        .call(c, 0x0012_88fc)?;
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0xcc)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x30)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x34)).lw(c);
        let hook = r32(c, V1);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, hook, 0x0012_8910)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        Callee {
            addr: 0x0012_97c8,
            f: ee::s00::f_001297c8,
        }
        .call(c, 0x0012_891c)?;

        // Loop 4: two words at self+0x24 and self+0x20.
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(4));
        c.g[V1][0] = 1;
        loop {
            c.cycles += 7;
            Addr(r32(c, V0)).sw(c, 0);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0012_8928);
            }
        }

        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void AI_SetupMissionRider(cAI* self)` (0x128ac0), named
/// `cAI_reinitMissionRiders` in the reference: creates the mission riders
/// (`AI_CreateMissionRider`, a no-op once they are loaded), then resets the
/// riders (`AI_ResetRiders`, 0x1297c8) for `self` with mode 0.
pub fn oc620_ai_setup_mission_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0012_9c00,
            f: ee::s00::f_00129c00,
        }
        .call(c, 0x0012_8ad4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        Callee {
            addr: 0x0012_97c8,
            f: ee::s00::f_001297c8,
        }
        .call(c, 0x0012_8ae0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int func_0012A1F0(cAI* ai, int idx)` (0x12a1f0), named
/// `AI_IsRiderInSpecialList` here: whether the character `iCharacters[idx]`
/// of `SSXApp->GameModeMan` is one of the six ranked characters
/// (`cGameModeMan_iCharRank`, six words at 0x536708). Returns 1 or 0 in v0.
pub fn oc620_ai_is_rider_in_special_list(c: &mut Cpu) {
    c.cycles += 10;
    c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
    c.g[A1][0] = sx(r32(c, A1) << 2);
    c.g[V1][0] = 0x0053_0000;
    c.g[A2][0] = 0;
    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0xc0)).lw(c);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x6708));
    c.g[A3][0] = 1;
    c.g[A0][0] = 5;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
    c.g[A1][0] = Addr(r32(c, V0).wrapping_add(0x18)).lw(c);
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(r32(c, V1)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
        c.g[V0][0] = c.g[A1][0] ^ c.g[V0][0];
        let more = (c.g[A0][0] as i64) >= 0;
        if c.g[V0][0] == 0 {
            c.g[A2][0] = c.g[A3][0];
        }
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0012_a218);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fall_back, hook, machine, prev_cell, FAKE};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// Gate word for the callee fakes below (set in every fixture).
    const OC620_GATE: u32 = DATA + 0x1000;
    const OC620_MAGIC: u32 = 0x6207_2034;

    /// A gated leaf stand-in for a callee that has an override slot: it
    /// returns at once when the gate is set, else runs the body the slot held
    /// before (`prev_cell`, or the generated body if never hooked).
    macro_rules! oc620_gated_leaf {
        ($fake:ident, $generated:path) => {
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC620_GATE) != OC620_MAGIC {
                    return fall_back(c, prev_cell($fake), $generated);
                }
                c.cycles += 5;
                ret(c);
            }
        };
    }

    oc620_gated_leaf!(oc620_fake_rider_base_load_state, ee::s00::f_00111d98__generated);
    oc620_gated_leaf!(oc620_fake_ai_reset_riders, ee::s00::f_001297c8__generated);
    oc620_gated_leaf!(oc620_fake_focus_man_reset, ee::s00::f_00103358__generated);
    oc620_gated_leaf!(oc620_fake_danger_man_reset, ee::s00::f_00101688__generated);
    oc620_gated_leaf!(oc620_fake_ai_set_race_state, ee::s00::f_00113b10__generated);
    oc620_gated_leaf!(
        oc620_fake_ai_load_anim_data_sources,
        ee::s00::f_0012b280__generated
    );

    fn oc620_install_fakes() {
        let slots: [(*mut fn(&mut Cpu), fn(&mut Cpu)); 6] = [
            (
                std::ptr::addr_of_mut!(ee::s00::OVERRIDE_00111D98),
                oc620_fake_rider_base_load_state,
            ),
            (
                std::ptr::addr_of_mut!(ee::s00::OVERRIDE_001297C8),
                oc620_fake_ai_reset_riders,
            ),
            (
                std::ptr::addr_of_mut!(ee::s00::OVERRIDE_00103358),
                oc620_fake_focus_man_reset,
            ),
            (
                std::ptr::addr_of_mut!(ee::s00::OVERRIDE_00101688),
                oc620_fake_danger_man_reset,
            ),
            (
                std::ptr::addr_of_mut!(ee::s00::OVERRIDE_00113B10),
                oc620_fake_ai_set_race_state,
            ),
            (
                std::ptr::addr_of_mut!(ee::s00::OVERRIDE_0012B280),
                oc620_fake_ai_load_anim_data_sources,
            ),
        ];
        for (slot, fake) in slots {
            hook(slot, fake, prev_cell(fake));
        }
    }

    #[test]
    fn player_load_state_matches() {
        oc620_install_fakes();
        check(ee::s00::f_00128680__generated, oc620_player_load_state, 50, &|c, seed| {
            let mut r = Rng(0x6201_2868 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0012_8680;
            c.bus.write32(OC620_GATE, OC620_MAGIC);
        });
    }

    /// Both sides run the same gated fakes for the callees; the hook call
    /// goes to `FAKE` (outside the table: the miss hook) with an offset that
    /// is seed-dependent, so the store loops see different bases.
    #[test]
    fn ai_init_matches() {
        oc620_install_fakes();
        check(ee::s00::f_00128818__generated, oc620_ai_init, 50, &|c, seed| {
            machine(c, seed, 0x0012_8818, 0);
            c.pc = 0x0012_8818;
            c.bus.write32(OC620_GATE, OC620_MAGIC);
            let ai = DATA + 0x6000;
            let block = DATA + 0x7000;
            c.g[A0][0] = sx(ai);
            c.bus.write32(ai + 0xcc, block);
            c.bus.write16(block + 0x30, ((seed % 4) * 0x10) as u16);
            c.bus.write32(block + 0x34, FAKE);
        });
    }

    /// The generated `AI_CreateMissionRider` returns at once when the riders
    /// are loaded (`self->NumberOfMissionRidersLoaded`, +0x88 > 0), so the
    /// fixture takes that short path; the reset is the gated fake.
    #[test]
    fn ai_setup_mission_rider_matches() {
        oc620_install_fakes();
        check(ee::s00::f_00128ac0__generated, oc620_ai_setup_mission_rider, 50, &|c, seed| {
            let mut r = Rng(0x6201_28ac ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0012_8ac0;
            c.bus.write32(OC620_GATE, OC620_MAGIC);
            let ai = DATA + 0x3000;
            c.g[A0][0] = sx(ai);
            c.bus.write32(ai + 0x88, 1 + seed % 3);
        });
    }

    #[test]
    fn ai_is_rider_in_special_list_matches() {
        check(ee::s00::f_0012a1f0__generated, oc620_ai_is_rider_in_special_list, 200, &|c, seed| {
            let mut r = Rng(0x6201_a1f0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0012_a1f0;
            // cGameModeMan_iCharRank = {2, 3, 4, 5, 6, 7}.
            for i in 0..6u32 {
                c.bus.write32(0x0053_6708 + 4 * i, i + 2);
            }
            let world = DATA + 0x5000;
            let table = DATA + 0x4000;
            let idx = seed % 8;
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0xc0, table);
            c.bus.write32(table + 0x18 + 4 * idx, seed % 10);
            c.g[A1][0] = sx(idx);
        });
    }
}
