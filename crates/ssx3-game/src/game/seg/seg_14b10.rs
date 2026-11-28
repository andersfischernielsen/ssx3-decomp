//! `src/seg/seg_14b10.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_3b10, ai_set_race_state, "AI_SetRaceState"),
    (0x0011_6950, trick_build_name, "Trick_BuildName"),
    (
        0x0011_3e18,
        race_state_end_race_enter,
        "RaceStateEndRace_Enter",
    ),
    (
        0x0011_3d18,
        race_state_pre_race_enter_00113d18,
        "RaceStatePreRace_Enter",
    ),
    (
        0x0011_3d28,
        race_state_pre_race_update_00113d28,
        "RaceStatePreRace_Update",
    ),
    (
        0x0011_3d30,
        race_state_pre_race_exit_00113d30,
        "RaceStatePreRace_Exit",
    ),
    (
        0x0011_3e40,
        race_state_end_race_update_00113e40,
        "RaceStateEndRace_Update",
    ),
    (
        0x0011_3e48,
        race_state_end_race_exit_00113e48,
        "RaceStateEndRace_Exit",
    ),
    (
        0x0011_3e50,
        race_state_shutdown_enter_00113e50,
        "RaceStateShutdown_Enter",
    ),
    (
        0x0011_3e70,
        race_state_shutdown_update_00113e70,
        "RaceStateShutdown_Update",
    ),
    (
        0x0011_3e78,
        race_state_shutdown_exit_00113e78,
        "RaceStateShutdown_Exit",
    ),
];

// ---------------------------------------------------------------------------
// AI manager: race-state selection.

pub(crate) const AI_SELECT_RACE_STATE: Callee = Callee {
    addr: 0x0011_3b48,
    f: ssx3_ee::s00::f_00113b48,
};

/// `void AI_SetRaceState(AI *ai, int state)` (0x113b10): when the state
/// changes, remember the old one (+4), snapshot the path pointer (+0x98 into
/// +0x9c) and reselect.
pub fn ai_set_race_state(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let ai = c.g[A0][0];
        c.g[V1][0] = ai;
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(ai as u32).lw(c);
        if c.g[A1][0] == c.g[A2][0] {
            c.g[RA][0] = c.ld(frame.at(c, 0));
        } else {
            c.cycles += 5;
            c.g[V0][0] = Addr(ai as u32).off(0x98).lw(c);
            Addr(ai as u32).off(0x4).sw(c, c.g[A2][0]);
            Addr(ai as u32).off(0x9c).sw(c, c.g[V0][0]);
            AI_SELECT_RACE_STATE.call(c, 0x0011_3b38)?;
            c.g[RA][0] = c.ld(frame.at(c, 0));
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Trick_BuildName (0x116950, seg_14B10.cpp `gGenTrickName`)

/// Where a trick-id field is read from (byte offset in the 8-byte id).
#[derive(Clone, Copy)]
pub(crate) enum TbnLoad {
    Word(u32),
    Half(u32),
    Byte(u32),
}

/// The string table a field indexes.
#[derive(Clone, Copy)]
pub(crate) enum TbnTable {
    Abs(u32),
    /// `t0 - 0x2ea0` (t0 = 0x440000: `D_0043D160`).
    T0,
    /// `gp - 0x2100` (`D_004A0FF0`).
    Gp,
}

/// One name part: `table[(load >> shift) & mask (<< 2)]` (the mask and
/// shift already produce a byte offset unless `shl2`), its header cycles
/// (10-cycle headers pay one more cycle before the copy loop), and the copy
/// loop's head.
#[derive(Clone, Copy)]
pub(crate) struct TbnPart {
    cycles: u64,
    load: TbnLoad,
    table: TbnTable,
    shift: u32,
    mask: Option<u64>,
    shl2: bool,
    head: u32,
}

pub(crate) const fn tbn(
    cycles: u64,
    load: TbnLoad,
    table: TbnTable,
    shift: u32,
    mask: Option<u64>,
    shl2: bool,
    head: u32,
) -> TbnPart {
    TbnPart {
        cycles,
        load,
        table,
        shift,
        mask,
        shl2,
        head,
    }
}

/// Name parts 2..17 (part 1, the stance from bits 0-2, has its own shape).
pub(crate) const TBN_PARTS: [TbnPart; 16] = {
    use TbnLoad::*;
    use TbnTable::*;
    [
        tbn(
            10,
            Word(0),
            Abs(0x0043_d130),
            1,
            Some(0x3c),
            false,
            0x0011_69c8,
        ),
        tbn(
            10,
            Word(0),
            Abs(0x0043_cf40),
            5,
            Some(0x1c),
            false,
            0x0011_6a10,
        ),
        tbn(
            10,
            Word(0),
            Abs(0x0043_cf60),
            8,
            Some(0xc),
            false,
            0x0011_6a58,
        ),
        tbn(
            10,
            Word(0),
            Abs(0x0043_cf70),
            10,
            Some(0x3c),
            false,
            0x0011_6aa0,
        ),
        tbn(
            10,
            Half(2),
            Abs(0x0043_cfb0),
            0,
            Some(0x7),
            true,
            0x0011_6ae8,
        ),
        tbn(
            10,
            Word(0),
            Abs(0x0043_cfc8),
            17,
            Some(0x1c),
            false,
            0x0011_6b30,
        ),
        tbn(
            10,
            Word(0),
            Abs(0x0043_cfe0),
            20,
            Some(0xfc),
            false,
            0x0011_6b78,
        ),
        tbn(10, Word(0), Abs(0x0043_d0c0), 28, None, true, 0x0011_6bc0),
        tbn(9, Word(4), T0, 1, Some(0x1fc), false, 0x0011_6c00),
        tbn(9, Word(4), Gp, 8, Some(0x4), false, 0x0011_6c40),
        tbn(9, Word(4), T0, 9, Some(0x1fc), false, 0x0011_6c80),
        tbn(
            10,
            Word(4),
            Abs(0x0043_d2b8),
            0,
            Some(0x7),
            true,
            0x0011_6cc8,
        ),
        tbn(
            9,
            Half(6),
            Abs(0x0043_d2d0),
            0,
            Some(0xc),
            false,
            0x0011_6d08,
        ),
        tbn(
            10,
            Word(4),
            Abs(0x0043_d2e0),
            18,
            Some(0x3c),
            false,
            0x0011_6d50,
        ),
        tbn(
            10,
            Byte(7),
            Abs(0x0043_d100),
            0,
            Some(0x7),
            true,
            0x0011_6d98,
        ),
        tbn(10, Word(4), Abs(0x0043_d320), 27, None, true, 0x0011_6de0),
    ]
};

/// Copy the string at `a1` to `a0` (both advanced). The header loads the
/// first byte into `signed` (`lb`, tested) and `unsigned` (`lbu`, stored);
/// the loop tests `v0` and stores `unsigned`. `pre`: cycles before the loop.
pub(crate) fn tbn_copy(c: &mut Cpu, head: u32, signed: usize, unsigned: usize, pre: u64) {
    let a1 = r32(c, A1);
    c.g[signed][0] = c.lb(a1);
    c.g[unsigned][0] = c.lbu(a1);
    if c.g[signed][0] == 0 {
        return;
    }
    c.cycles += pre;
    loop {
        c.cycles += 7;
        c.sb(r32(c, A0), c.g[unsigned][0]);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
        let p = r32(c, A1);
        c.g[V0][0] = c.lb(p);
        c.g[unsigned][0] = c.lbu(p);
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `void Trick_BuildName(char* out, const TrickId* id)` (0x116950): builds
/// a trick's display name by concatenating 17 strings, each picked from a
/// name-part table by a bit field of the 8-byte trick id (stance, rotation,
/// flip, grab, modifiers, ...), then NUL-terminates it. A leaf.
pub fn trick_build_name(c: &mut Cpu) {
    // Part 1: the stance (bits 0-2 of the first word, `D_0043D118`).
    c.cycles += 12;
    c.g[A3][0] = c.g[A1][0];
    c.g[V0][0] = 0x0044_0000;
    let id = r32(c, A3);
    c.g[V1][0] = c.lw(id);
    c.g[V0][0] = sx(0x0043_d118);
    c.g[T0][0] = 0x0044_0000;
    c.g[V1][0] &= 7;
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.g[A1][0] = c.lw(r32(c, V1));
    tbn_copy(c, 0x0011_6980, V0, V1, 0);
    for part in TBN_PARTS {
        c.cycles += part.cycles;
        let id = r32(c, A3);
        c.g[V0][0] = match part.load {
            TbnLoad::Word(off) => c.lw(id.wrapping_add(off)),
            TbnLoad::Half(off) => c.lhu(id.wrapping_add(off)),
            TbnLoad::Byte(off) => c.lbu(id.wrapping_add(off)),
        };
        c.g[V1][0] = match part.table {
            TbnTable::Abs(a) => sx(a),
            TbnTable::T0 => sx(r32(c, T0).wrapping_sub(0x2ea0)),
            TbnTable::Gp => sx(r32(c, GP).wrapping_sub(0x2100)),
        };
        if part.shift != 0 {
            c.g[V0][0] = sx(r32(c, V0) >> part.shift);
        }
        if let Some(mask) = part.mask {
            c.g[V0][0] &= mask;
        }
        if part.shl2 {
            c.g[V0][0] = sx(r32(c, V0) << 2);
        }
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[A1][0] = c.lw(r32(c, V0));
        tbn_copy(c, part.head, V1, A2, (part.cycles == 10) as u64);
    }
    c.cycles += 2;
    c.sb(r32(c, A0), 0);
    ret(c);
}

/// `void RaceStateEndRace_Enter()` (0x113e18): tell the game-comm object (the
/// global at gp-0x204), if there is one, that the race ended (0x258a48).
pub fn race_state_end_race_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
        let none = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 2;
            call_ptr(c, 0x0025_8a48, 0x0011_3e30)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RaceStatePreRace_Enter` (0x0011_3d18): straight-line leaf: 2 store(s). Mechanical port of the generated body.
pub fn race_state_pre_race_enter_00113d18(c: &mut Cpu) {
    c.cycles += 3;
    c.sw((c.g[A1][0] as u32).wrapping_add(0xc), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), 0u64);
    ret(c);
}

/// `RaceStatePreRace_Update` (0x0011_3d28): straight-line leaf. Mechanical port of the generated body.
pub fn race_state_pre_race_update_00113d28(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RaceStatePreRace_Exit` (0x0011_3d30): straight-line leaf. Mechanical port of the generated body.
pub fn race_state_pre_race_exit_00113d30(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RaceStateEndRace_Update` (0x0011_3e40): straight-line leaf. Mechanical port of the generated body.
pub fn race_state_end_race_update_00113e40(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RaceStateEndRace_Exit` (0x0011_3e48): straight-line leaf. Mechanical port of the generated body.
pub fn race_state_end_race_exit_00113e48(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RaceStateShutdown_Update` (0x0011_3e70): straight-line leaf. Mechanical port of the generated body.
pub fn race_state_shutdown_update_00113e70(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RaceStateShutdown_Exit` (0x0011_3e78): straight-line leaf. Mechanical port of the generated body.
pub fn race_state_shutdown_exit_00113e78(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void RaceStateShutdown_Enter(RaceState* self, AI* ai)` (0x113e50): calls
/// `AI_DestroyAllRiders(ai)` (the second argument moves into `a0`), then
/// returns. A frame of 0x10 holding `ra`.
pub fn race_state_shutdown_enter_00113e50(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        Callee {
            addr: 0x0012_b090,
            f: ee::s00::AI_DestroyAllRiders,
        }
        .call(c, 0x0011_3e60)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;

    pub(crate) use crate::game::ai::ai::tests::race_state_table;
    pub(crate) use crate::game::lib::kernel::tests::SEEDS;

    #[test]
    fn ai_set_race_state_matches() {
        check(
            ssx3_ee::s00::f_00113b10,
            ai_set_race_state,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 74, 0);
                race_state_table(c, seed);
                let ai = DATA + 0x400;
                let state = (seed % 10) as u32;
                let cur = if seed % 4 == 0 {
                    state
                } else {
                    r.next_u32() % 10
                };
                c.bus.write32(ai, cur);
                c.bus.write32(ai + 0x98, r.next_u32());
                c.g[A0][0] = ai as u64;
                c.g[A1][0] = state as u64;
            },
        );
    }

    /// Trick_BuildName: random trick ids into name-part tables
    /// whose slots point at empty or short strings (both copy shapes), and
    /// one seed in five on the game image's real tables with an all-zero id.
    #[test]
    fn trick_build_name_matches() {
        use crate::difftest::{check, elf_image, scramble, Rng, GAME_GP};
        check(
            ee::s00::f_00116950__generated,
            trick_build_name,
            300,
            &|c, seed| {
                let mut r = Rng(0x11_6950 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0011_6950;
                c.g[GP][0] = GAME_GP as u64;
                let id = DATA;
                let out = DATA + 0x100;
                let strings = DATA + 0x1000;
                c.bus.write32(id, r.next_u32());
                c.bus.write32(id + 4, r.next_u32());
                // Tables: every part's slots (largest index 0x1fc / 4).
                let tables = [
                    0x0043_d118u32,
                    0x0043_d130,
                    0x0043_cf40,
                    0x0043_cf60,
                    0x0043_cf70,
                    0x0043_cfb0,
                    0x0043_cfc8,
                    0x0043_cfe0,
                    0x0043_d0c0,
                    0x0043_d160,
                    0x0043_d2b8,
                    0x0043_d2d0,
                    0x0043_d2e0,
                    0x0043_d100,
                    0x0043_d320,
                    GAME_GP - 0x2100,
                ];
                for k in 0..4u32 {
                    let s = strings + 0x10 * k;
                    let len = if k == 0 { 0 } else { k + r.next_u32() % 6 };
                    for i in 0..len {
                        c.bus.write8(s + i, b'a' + (r.next_u32() % 26) as u8);
                    }
                    c.bus.write8(s + len, 0);
                }
                if seed % 5 == 0 {
                    // The real tables, at index 0 of every part.
                    elf_image(c);
                    c.bus.write32(id, 0);
                    c.bus.write32(id + 4, 0);
                } else {
                    for t in tables {
                        for slot in 0..0x80u32 {
                            c.bus
                                .write32(t + 4 * slot, strings + 0x10 * (r.next_u32() % 4));
                        }
                    }
                }
                c.g[A0][0] = out as u64;
                c.g[A1][0] = id as u64;
            },
        );
    }

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::GAME_GP;

    /// `RaceStateEndRace_Enter`: with and without the game-comm object; the
    /// real `0x258a48` runs on an object whose timers (+0x38/+0x3c) are set
    /// and whose state (+0x70) skips the state change.
    #[test]
    fn race_state_end_race_enter_matches() {
        check(
            ee::s00::f_00113e18__generated,
            race_state_end_race_enter,
            60,
            &|c, seed| {
                let mut r = Rng(0x1131_e180 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0011_3e18;
                c.g[GP][0] = GAME_GP as u64;
                let obj = DATA + 0x400;
                c.bus
                    .write32(GAME_GP - 0x204, if seed % 4 == 0 { 0 } else { obj });
                c.bus.write32(obj + 0x70, 15 + r.next_u32() % 4);
                c.bus.write32(obj + 0x38, 1 + r.next_u32() % 100);
                c.bus.write32(obj + 0x3c, 1 + r.next_u32() % 100);
                fill(c, &mut r, obj + 0x40, 2);
            },
        );
    }

    use crate::difftest::fakes;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::leaf_fill;
    pub(crate) use crate::game::be::belibrary::tests::OC1535_MAGIC;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// `RaceStatePreRace_Enter` (0x0011_3d18) against the generated body.
    #[test]
    fn race_state_pre_race_enter_00113d18_matches() {
        check(
            ee::s00::f_00113d18__generated,
            race_state_pre_race_enter_00113d18,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x113d18, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                leaf_fill(c, &mut r, 0x0080_5000, 0x2000, false, 0x0080_5000);
                c.g[A1][0] = sx(0x0080_5000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStatePreRace_Update` (0x0011_3d28) against the generated body.
    #[test]
    fn race_state_pre_race_update_00113d28_matches() {
        check(
            ee::s00::f_00113d28__generated,
            race_state_pre_race_update_00113d28,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113d28, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStatePreRace_Exit` (0x0011_3d30) against the generated body.
    #[test]
    fn race_state_pre_race_exit_00113d30_matches() {
        check(
            ee::s00::f_00113d30__generated,
            race_state_pre_race_exit_00113d30,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113d30, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStateEndRace_Update` (0x0011_3e40) against the generated body.
    #[test]
    fn race_state_end_race_update_00113e40_matches() {
        check(
            ee::s00::f_00113e40__generated,
            race_state_end_race_update_00113e40,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113e40, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStateEndRace_Exit` (0x0011_3e48) against the generated body.
    #[test]
    fn race_state_end_race_exit_00113e48_matches() {
        check(
            ee::s00::f_00113e48__generated,
            race_state_end_race_exit_00113e48,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113e48, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStateShutdown_Update` (0x0011_3e70) against the generated body.
    #[test]
    fn race_state_shutdown_update_00113e70_matches() {
        check(
            ee::s00::f_00113e70__generated,
            race_state_shutdown_update_00113e70,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113e70, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStateShutdown_Exit` (0x0011_3e78) against the generated body.
    #[test]
    fn race_state_shutdown_exit_00113e78_matches() {
        check(
            ee::s00::f_00113e78__generated,
            race_state_shutdown_exit_00113e78,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113e78, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    pub(crate) const OC1535_AI_GATE: u32 = DATA + 0x20_1000;

    pub(crate) static OC1535_AI_PREV: AtomicUsize = AtomicUsize::new(0);

    /// `AI_DestroyAllRiders` (0x12b090), gated: logs the call when the gate is on.
    pub(crate) fn oc1535_fake_destroy_riders(c: &mut Cpu) {
        if c.bus.read32(OC1535_AI_GATE) != OC1535_MAGIC {
            return fakes::fall_back(c, &OC1535_AI_PREV, ee::s00::f_0012b090__generated);
        }
        fakes::fake_call(c, 0x0012_b090);
    }

    #[test]
    fn race_state_shutdown_enter_00113e50_matches() {
        check(
            ee::s00::f_00113e50__generated,
            race_state_shutdown_enter_00113e50,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113e50, 1);
                fakes::hook(
                    addr_of_mut!(ee::s00::OVERRIDE_0012B090),
                    oc1535_fake_destroy_riders,
                    &OC1535_AI_PREV,
                );
                c.bus.write32(OC1535_AI_GATE, OC1535_MAGIC);
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
