//! Keeps plugin events out of the player's career (`docs/plugins.md`, "Career
//! and saves").
//!
//! A plugin event runs on its template native event (say ARA1), so the game
//! would record its result as that event's: high-score table, best times,
//! medals, rewards and unlocks, which the next save writes to the memory card.
//! While a plugin event is the one being played (`event_boot::plugin_event_active`)
//! the hooks here skip those writes and keep the rest of the flow: the race
//! result is still stored for the results screen, Restart, Next Event and Quit
//! work as always. An event opts out with `career = true` in its manifest, a
//! code plugin with [`set_career_allowed`] (from its event listener's `started`).
//!
//! Hooks (installed only when plugins add events; the game without plugins runs
//! its own code, bit for bit):
//!
//! | function | guarded behaviour |
//! | --- | --- |
//! | `BEScoreInt_SubmitRaceResult` 0x154ab8 (via `GameModeMan_OnRiderFinished` 0x238358) | copies the rider's race stats into the score object (the results screen reads them), best = 0, rank = -1; no best stats, no high-score table, no records, no profile score |
//! | `BEScoreInt_SubmitPlacement` 0x154ee8 | stores the place for the results screen; no event-result messages, no medal, no reward |
//! | `BERewardInt_GrantReward` 0x159cd0 (mountain conquered award in `OnRiderFinished`) | nothing granted |
//! | `BERewardInt_OnCollectibleCollected` 0x1599a0, `OnMissionCompleted` 0x159b08 | nothing recorded |
//!
//! The card is written from the career RAM (`BESaveInt_BuildProfileSave`,
//! `BuildSystemSave`), so a save made while or after a plugin event writes what
//! the career was before it. Nothing writes the card on its own in these flows.

use std::sync::atomic::{AtomicBool, Ordering};

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_plugin::hooks::Prev;

use crate::event_boot::call;

const SUBMIT_RACE_RESULT: u32 = 0x0015_4ab8;
const SUBMIT_PLACEMENT: u32 = 0x0015_4ee8;
const GRANT_REWARD: u32 = 0x0015_9cd0;
const ON_COLLECTIBLE: u32 = 0x0015_99a0;
const ON_MISSION: u32 = 0x0015_9b08;
const GET_RIDER_PROFILE: u32 = 0x0014_a0e0;

/// The score object: `best[2]` at +0x10, `rank[2]` at +0x18, two slots at +0x20
/// (0xb0 each: 0xac bytes of race stats, then the place).
const SLOT_BASE: u32 = 0x20;
const SLOT_SIZE: u32 = 0xb0;
const STATS_SIZE: u32 = 0xac;
const PLACE: u32 = 0xac;

static ALLOWED: AtomicBool = AtomicBool::new(false);

/// The plugin event being played may write the template's career record (the
/// guard is off until the event ends). Reset when the event ends.
pub fn set_career_allowed(allowed: bool) {
    ALLOWED.store(allowed, Ordering::Relaxed);
}

/// Career writes are skipped now: a plugin event is being played and did not
/// opt out.
pub fn guarded() -> bool {
    #[cfg(test)]
    if let Some(g) = tests::FORCE.with(|f| f.get()) {
        return g;
    }
    decide(
        crate::event_boot::plugin_event_active(),
        ALLOWED.load(Ordering::Relaxed),
    )
}

/// A plugin event is active and did not opt out.
fn decide(active: bool, allowed: bool) -> bool {
    active && !allowed
}

static RACE_RESULT: Prev = Prev::new();
static PLACEMENT: Prev = Prev::new();
static REWARD: Prev = Prev::new();
static COLLECTIBLE: Prev = Prev::new();
static MISSION: Prev = Prev::new();

/// Install the hooks (call once at boot, only when the plugins add events).
pub fn install() {
    if crate::plugin_host::catalog().events.is_empty() {
        return;
    }
    chain(SUBMIT_RACE_RESULT, submit_race_result, &RACE_RESULT);
    chain(SUBMIT_PLACEMENT, submit_placement, &PLACEMENT);
    chain(GRANT_REWARD, skip_when_guarded, &REWARD);
    chain(ON_COLLECTIBLE, skip_collectible, &COLLECTIBLE);
    chain(ON_MISSION, skip_mission, &MISSION);
}

fn chain(addr: u32, f: ssx3_ee_rt::funcs::GuestFn, prev: &Prev) {
    let was = ssx3_ee::get_override(addr)
        .or_else(|| ssx3_ee::generated_body(addr))
        .expect("career guard: no function to wrap");
    prev.set_for_host(was);
    assert!(
        ssx3_ee::set_override(addr, f),
        "career guard: {addr:#x} has no override slot (hookable.txt)"
    );
}

/// The rider's profile index (0 or 1; 2 and up: not a human profile).
fn profile_of(c: &mut Cpu, rider: u32, back: u32) -> Option<u32> {
    call(c, GET_RIDER_PROFILE, &[rider], back).ok()?;
    Some(c.g[V0][0] as u32)
}

/// Return from a hook that replaced its function: `v0` as given.
fn leave(c: &mut Cpu, back: u32, v0: u64) {
    c.g[RA][0] = back as u64;
    c.g[V0][0] = v0;
    c.pc = back;
}

/// `(self, rider, stats, score)`: the stats go where the results screen reads
/// them; nothing persistent is touched.
fn submit_race_result(c: &mut Cpu) {
    if !guarded() {
        eprintln!(
            "[career-guard] race result of rider {} recorded",
            c.g[A1][0] as u32
        );
        return RACE_RESULT.call(c);
    }
    eprintln!(
        "[career-guard] plugin event: race result of rider {} kept out of the career",
        c.g[A1][0] as u32
    );
    let (this, rider, stats) = (c.g[A0][0] as u32, c.g[A1][0] as u32, c.g[A2][0] as u32);
    let back = c.g[RA][0] as u32;
    let Some(p) = profile_of(c, rider, back) else {
        return;
    };
    if p < 2 {
        let slot = this + SLOT_BASE + p * SLOT_SIZE;
        for i in 0..STATS_SIZE {
            let b = c.bus.read8(stats + i);
            c.bus.write8(slot + i, b);
        }
        c.bus.write32(slot + PLACE, u32::MAX);
        c.bus.write32(this + 0x10 + 4 * p, 0);
        c.bus.write32(this + 0x18 + 4 * p, u32::MAX);
    }
    leave(c, back, 0);
}

/// `(self, rider, place, score)`.
fn submit_placement(c: &mut Cpu) {
    if !guarded() {
        eprintln!(
            "[career-guard] placement of rider {} recorded",
            c.g[A1][0] as u32
        );
        return PLACEMENT.call(c);
    }
    eprintln!(
        "[career-guard] plugin event: placement of rider {} kept out of the career",
        c.g[A1][0] as u32
    );
    let (this, rider, place) = (c.g[A0][0] as u32, c.g[A1][0] as u32, c.g[A2][0] as u32);
    let back = c.g[RA][0] as u32;
    let Some(p) = profile_of(c, rider, back) else {
        return;
    };
    if p < 2 {
        c.bus
            .write32(this + SLOT_BASE + p * SLOT_SIZE + PLACE, place);
    }
    leave(c, back, 0);
}

fn skip_when_guarded(c: &mut Cpu) {
    if guarded() {
        let back = c.g[RA][0] as u32;
        return leave(c, back, 0);
    }
    REWARD.call(c)
}

fn skip_collectible(c: &mut Cpu) {
    if guarded() {
        let back = c.g[RA][0] as u32;
        return leave(c, back, 0);
    }
    COLLECTIBLE.call(c)
}

fn skip_mission(c: &mut Cpu) {
    if guarded() {
        let back = c.g[RA][0] as u32;
        return leave(c, back, 0);
    }
    MISSION.call(c)
}

#[cfg(test)]
pub(crate) mod tests {
    use std::cell::Cell;

    use ssx3_ee::s01;

    use super::*;
    use crate::difftest::{fresh, run_generated, SP};
    use crate::game::fe::festateloadscreen::tests::{oc444_env, OC444_OBJ, OC444_OBJ2};

    thread_local! {
        /// Forces `guarded()` on this thread (the process-wide state belongs to
        /// the other tests).
        pub(crate) static FORCE: Cell<Option<bool>> = const { Cell::new(None) };
    }

    /// Career memory: the two profiles and the score tables.
    const CAREER: [(u32, u32); 2] = [(0x4a_6ca8, 2 * 0x9b50), (0x53_5c18, 0xa28)];

    fn in_career(a: u32) -> bool {
        CAREER.iter().any(|(s, n)| (*s..*s + *n).contains(&a))
    }

    /// Addresses whose byte differs, the stack frame of the call left out.
    fn changed(before: &[u8], c: &Cpu) -> Vec<u32> {
        let (lo, hi) = (SP as usize - 0x800, SP as usize + 0x40);
        before
            .iter()
            .zip(c.bus.ram.iter())
            .enumerate()
            .filter(|(i, (a, b))| a != b && !(lo..hi).contains(i))
            .map(|(i, _)| i as u32)
            .collect()
    }

    /// Build the fixture machine for one seed: a career-mode race (`b49` = 0)
    /// with `args` set.
    fn machine(
        seed: u32,
        entry: u32,
        args: &dyn Fn(&mut Cpu, &mut crate::difftest::Rng, u32),
    ) -> Box<Cpu> {
        let mut c = fresh();
        let mut r = oc444_env(&mut c, seed, 0xca_ee, 4);
        c.bus.write8(0x0053_05f9, 0);
        args(&mut c, &mut r, seed);
        c.pc = entry;
        c
    }

    /// The guarded `SubmitRaceResult` leaves the career alone, and keeps what
    /// the results screen reads; the original (the control) does write it.
    #[test]
    fn a_plugin_event_keeps_its_race_result_out_of_the_career() {
        let args = |c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32| {
            c.g[A0][0] = OC444_OBJ as u64;
            c.g[A1][0] = (r.next_u32() % 4) as u64;
            c.g[A2][0] = (OC444_OBJ2 + 4 * (seed % 2)) as u64;
            c.g[A3][0] = [1u64, 59, 600, 5000][seed as usize % 4];
        };
        let (mut original_wrote, mut kept_stats) = (0, 0);
        for seed in 0..60 {
            let mut orig = machine(seed, 0x15_4ab8, &args);
            let before = orig.bus.ram.clone();
            FORCE.with(|f| f.set(Some(false)));
            run_generated(&mut orig, s01::f_00154ab8__generated);
            let mut hook = machine(seed, 0x15_4ab8, &args);
            FORCE.with(|f| f.set(Some(true)));
            submit_race_result(&mut hook);
            FORCE.with(|f| f.set(None));
            let orig_changes = changed(&before, &orig);
            let hook_changes = changed(&before, &hook);
            if orig_changes.iter().any(|a| in_career(*a)) {
                original_wrote += 1;
            }
            assert!(
                hook_changes
                    .iter()
                    .all(|a| (OC444_OBJ + 0x10..OC444_OBJ + 0x180).contains(a)),
                "seed {seed}: the guard wrote outside the score object: {:x?}",
                hook_changes
                    .iter()
                    .filter(|a| !(OC444_OBJ + 0x10..OC444_OBJ + 0x180).contains(a))
                    .collect::<Vec<_>>()
            );
            // The stats the results screen reads are what the game copies.
            let slot = |c: &Cpu, p: u32| {
                (0..0xac)
                    .map(|i| c.bus.ram[(OC444_OBJ + 0x20 + p * 0xb0 + i) as usize])
                    .collect::<Vec<_>>()
            };
            for p in 0..2 {
                if orig.bus.read32(OC444_OBJ + 0x20 + p * 0xb0 + 0xac) == u32::MAX
                    && slot(&orig, p) != slot(&before_cpu(&before), p)
                {
                    assert_eq!(slot(&orig, p), slot(&hook, p), "seed {seed}: slot {p}");
                    kept_stats += 1;
                }
            }
            assert_eq!(
                hook.pc,
                crate::difftest::RA,
                "seed {seed}: returns to the caller"
            );
        }
        assert!(
            original_wrote > 0,
            "the control: the original never wrote the career"
        );
        assert!(kept_stats > 0, "the stats copy was never exercised");
    }

    fn before_cpu(ram: &[u8]) -> Box<Cpu> {
        let mut c = fresh();
        c.bus.ram.copy_from_slice(ram);
        c
    }

    /// The guarded `SubmitPlacement` stores the place for the results screen
    /// and nothing else; the original does more (the control).
    #[test]
    fn a_plugin_event_keeps_its_placement_out_of_the_career() {
        let args = |c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32| {
            c.g[A0][0] = OC444_OBJ as u64;
            c.g[A1][0] = (r.next_u32() % 4) as u64;
            c.g[A2][0] = [0u64, 1, 2, 3, 5][seed as usize % 5];
            c.g[A3][0] = [100u64, 6000][seed as usize % 2];
        };
        let mut original_wrote_more = 0;
        let mut stored = 0;
        for seed in 0..60 {
            let mut orig = machine(seed, 0x15_4ee8, &args);
            let before = orig.bus.ram.clone();
            FORCE.with(|f| f.set(Some(false)));
            run_generated(&mut orig, s01::f_00154ee8__generated);
            let mut hook = machine(seed, 0x15_4ee8, &args);
            FORCE.with(|f| f.set(Some(true)));
            submit_placement(&mut hook);
            FORCE.with(|f| f.set(None));
            let place_words = |p: u32| OC444_OBJ + 0x20 + p * 0xb0 + 0xac;
            let orig_changes = changed(&before, &orig);
            let hook_changes = changed(&before, &hook);
            if orig_changes
                .iter()
                .any(|a| !(0..2).any(|p| (place_words(p)..place_words(p) + 4).contains(a)))
            {
                original_wrote_more += 1;
            }
            for a in &hook_changes {
                assert!(
                    (0..2).any(|p| (place_words(p)..place_words(p) + 4).contains(a)),
                    "seed {seed}: the guard wrote {a:#x}"
                );
            }
            for p in 0..2 {
                if hook.bus.read32(place_words(p)) != orig_value(&before, place_words(p)) {
                    assert_eq!(
                        hook.bus.read32(place_words(p)),
                        orig.bus.read32(place_words(p)),
                        "seed {seed}: place {p}"
                    );
                    stored += 1;
                }
            }
        }
        assert!(
            original_wrote_more > 0,
            "the control: the original never wrote more than the place"
        );
        assert!(stored > 0);
    }

    fn orig_value(ram: &[u8], a: u32) -> u32 {
        u32::from_le_bytes(ram[a as usize..a as usize + 4].try_into().unwrap())
    }

    #[test]
    fn the_guard_needs_an_active_plugin_event_that_did_not_opt_out() {
        assert!(decide(true, false));
        assert!(!decide(true, true), "opted out");
        assert!(!decide(false, false), "a game event");
        assert!(!decide(false, true));
    }
}
