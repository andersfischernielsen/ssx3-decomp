//! `src/main/racestate.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::Callee;
use crate::call::Frame;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_3cc0, race_state_game_init_update, "RaceStateGameInit_Update"),
    (0x0011_3cf0, race_state_freeride_update, "RaceStateFreeride_Update"),
    (0x0011_3d48, race_state_countdown_update, "RaceStateCountdown_Update"),
];


/// `void RaceStateGameInit_Update(AIRaceState *state, AIRaceState *next)`
/// (0x113cc0): set the object in `a1` to race state 2.
pub fn race_state_game_init_update(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x2;
        Callee { addr: 0x0011_3b10, f: ssx3_ee::s00::AI_SetRaceState }
            .call(c, 0x0011_3cd4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void RaceStateFreeride_Update(AIRaceState *state, AIRaceState *next)`
/// (0x113cf0): set the object in `a1` to race state 5.
pub fn race_state_freeride_update(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x5;
        Callee { addr: 0x0011_3b10, f: ssx3_ee::s00::AI_SetRaceState }
            .call(c, 0x0011_3d04)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void RaceStateCountdown_Update(AIRaceState *state, AIRaceState *next)`
/// (0x113d48): tick the countdown at `+0x1c`; when it expires, move the
/// object in `a1` to race state 5.
pub fn race_state_countdown_update(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let timer = Addr(c.g[A0][0] as u32).off(0x1c);
        let remaining = timer.lw(c);
        c.g[V0][0] = sx(remaining.wrapping_sub(1) as u32);
        if (remaining as i64) > 0 {
            c.cycles += 2;
            timer.sw(c, c.g[V0][0]);
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 4;
            c.g[A1][0] = 0x5;
            Callee { addr: 0x0011_3b10, f: ssx3_ee::s00::AI_SetRaceState }
                .call(c, 0x0011_3d68)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;


    #[test]
    fn race_state_game_init_update_matches() {
        check(ssx3_ee::s00::f_00113cc0, race_state_game_init_update, 100, &|c, seed| {
            let mut r = Rng(0x3141_5926 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = DATA as u64;
            // state at +0 already equals the 2 the wrapper installs, so the
            // callee (AI_SetRaceState) short-circuits without dispatching.
            c.bus.write32(DATA, 2);
        });
    }


    #[test]
    fn race_state_freeride_update_matches() {
        check(ssx3_ee::s00::f_00113cf0, race_state_freeride_update, 100, &|c, seed| {
            let mut r = Rng(0x2718_2818 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = DATA as u64;
            c.bus.write32(DATA, 5);
        });
    }


    #[test]
    fn race_state_countdown_update_matches() {
        check(ssx3_ee::s00::f_00113d48, race_state_countdown_update, 200, &|c, seed| {
            let mut r = Rng(0x1618_0339 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = DATA as u64;
            // Half the seeds tick a live countdown, half expire into the
            // AI_SetRaceState(5) branch (which short-circuits).
            let countdown = if seed % 2 == 0 { 1 + r.next_u32() % 1000 } else { 0 };
            c.bus.write32(DATA + 0x1c, countdown);
            c.bus.write32(DATA, 5);
        });
    }
}
