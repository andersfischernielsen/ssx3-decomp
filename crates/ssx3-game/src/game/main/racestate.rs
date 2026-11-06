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
    (
        0x0011_3cc0,
        race_state_game_init_update,
        "RaceStateGameInit_Update",
    ),
    (
        0x0011_3cf0,
        race_state_freeride_update,
        "RaceStateFreeride_Update",
    ),
    (
        0x0011_3d48,
        race_state_countdown_update,
        "RaceStateCountdown_Update",
    ),
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
        Callee {
            addr: 0x0011_3b10,
            f: ssx3_ee::s00::AI_SetRaceState,
        }
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
        Callee {
            addr: 0x0011_3b10,
            f: ssx3_ee::s00::AI_SetRaceState,
        }
        .call(c, 0x0011_3d04)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RaceStateCountdown_Update(AIRaceState *state, AIRaceState *next)`
/// (0x113d48): tick the countdown at `+0x1c`; when it expires, move the
/// object in `a1` to race state 5.
///
/// The countdown is in 60 Hz frames (`RaceStateCountdown_Enter` sets 180, the
/// start lights come at multiples of 60, `GameStateCountdown_Update`): above
/// 60 Hz it counts down once per `rate / 60` ticks, so the start takes 3 s at
/// any rate. Every call counts at 60.
///
/// The phase comes from the AI update counter (`+0x8`), which a replay restores
/// with the timer. A host-side credit is not restored, and the replayed inputs
/// would reach the race one tick late.
pub fn race_state_countdown_update(c: &mut Cpu) {
    let state = c.g[A1][0] as u32;
    let value = c.bus.read32(state.wrapping_add(0x1c));
    let count = c.bus.read32(state.wrapping_add(0x8));
    if (value as i32) > 0 && !crate::tick::frame60_by_count(c, count) {
        abi::ret(c);
        return;
    }
    race_state_countdown_step(c);
}

/// The step of [`race_state_countdown_update`] (the original).
fn race_state_countdown_step(c: &mut Cpu) {
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
            Callee {
                addr: 0x0011_3b10,
                f: ssx3_ee::s00::AI_SetRaceState,
            }
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
        check(
            ssx3_ee::s00::f_00113cc0,
            race_state_game_init_update,
            100,
            &|c, seed| {
                let mut r = Rng(0x3141_5926 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = DATA as u64;
                // state at +0 already equals the 2 the wrapper installs, so the
                // callee (AI_SetRaceState) short-circuits without dispatching.
                c.bus.write32(DATA, 2);
            },
        );
    }

    #[test]
    fn race_state_freeride_update_matches() {
        check(
            ssx3_ee::s00::f_00113cf0,
            race_state_freeride_update,
            100,
            &|c, seed| {
                let mut r = Rng(0x2718_2818 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = DATA as u64;
                c.bus.write32(DATA, 5);
            },
        );
    }

    #[test]
    fn race_state_countdown_update_matches() {
        check(
            ssx3_ee::s00::f_00113d48,
            race_state_countdown_update,
            200,
            &|c, seed| {
                let mut r = Rng(0x1618_0339 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = DATA as u64;
                // Half the seeds tick a live countdown, half expire into the
                // AI_SetRaceState(5) branch (which short-circuits).
                let countdown = if seed % 2 == 0 {
                    1 + r.next_u32() % 1000
                } else {
                    0
                };
                c.bus.write32(DATA + 0x1c, countdown);
                c.bus.write32(DATA, 5);
            },
        );
    }

    /// One countdown tick; bumps the AI counter (+0x8) as the game's update does.
    fn countdown_tick(c: &mut Cpu, state: u32) {
        c.g[A1][0] = state as u64;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0011_3d48;
        race_state_countdown_update(c);
        let count = c.bus.read32(state + 8);
        c.bus.write32(state + 8, count + 1);
    }

    /// 180 frames take 3 s at 60, 120, 144 and 240 Hz: down by 60 each second.
    #[test]
    fn race_state_countdown_update_rate() {
        for hz in [60u32, 120, 144, 240] {
            let mut c = crate::difftest::fresh();
            crate::tick::set_for_test(&mut c, hz);
            c.bus.write32(DATA, 4);
            c.bus.write32(DATA + 0x1c, 180);
            c.bus.write32(DATA + 8, 0);
            for sec in 1..=3u32 {
                for _ in 0..hz {
                    countdown_tick(&mut c, DATA);
                }
                assert_eq!(
                    c.bus.read32(DATA + 0x1c),
                    180 - 60 * sec,
                    "{hz} Hz, {sec} s"
                );
            }
        }
    }

    /// Restored at any tick of a recorded countdown (timer and counter), it
    /// counts down on the same ticks as the recording.
    #[test]
    fn race_state_countdown_update_resumes_from_a_snapshot() {
        for hz in [120u32, 144, 240] {
            let mut live = crate::difftest::fresh();
            crate::tick::set_for_test(&mut live, hz);
            live.bus.write32(DATA, 4);
            live.bus.write32(DATA + 0x1c, 180);
            live.bus.write32(DATA + 8, 0);
            // (timer, counter) before each tick of the recorded countdown.
            let mut states = Vec::new();
            for _ in 0..3 * hz {
                states.push((live.bus.read32(DATA + 0x1c), live.bus.read32(DATA + 8)));
                countdown_tick(&mut live, DATA);
            }
            states.push((live.bus.read32(DATA + 0x1c), live.bus.read32(DATA + 8)));
            for start in [1usize, 2, 3, 5, hz as usize - 1, 2 * hz as usize + 1] {
                let mut replay = crate::difftest::fresh();
                crate::tick::set_for_test(&mut replay, hz);
                replay.bus.write32(DATA, 4);
                replay.bus.write32(DATA + 0x1c, states[start].0);
                replay.bus.write32(DATA + 8, states[start].1);
                for (k, want) in states.iter().enumerate().skip(start) {
                    let got = (replay.bus.read32(DATA + 0x1c), replay.bus.read32(DATA + 8));
                    assert_eq!(got, *want, "{hz} Hz, restored at tick {start}, tick {k}");
                    countdown_tick(&mut replay, DATA);
                }
            }
        }
    }
}
