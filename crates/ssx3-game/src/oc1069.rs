//! Staging module oc1069 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1069`.

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
    (
        0x002f_6388,
        rider_base_dtor_cheat_code,
        "RiderBase_DtorCheatCode",
    ),
    (
        0x002f_7ab8,
        cboard_wake_fx_reset_thunk,
        "cBoardWakeFX_ResetThunk",
    ),
    (
        0x002f_7ae0,
        cboard_spark_fx_reset_thunk,
        "cBoardSparkFX_ResetThunk",
    ),
    (
        0x002f_7b08,
        cboard_trail_fx_reset_thunk,
        "cBoardTrailFX_ResetThunk",
    ),
    (0x002f_7b30, cang_er_fx_reset_thunk, "cAngerFX_ResetThunk"),
    (
        0x002f_7b58,
        cplayer_pulse_fx_reset_thunk,
        "cPlayerPulseFX_ResetThunk",
    ),
    (
        0x002f_7b80,
        cair_streamer_fx_reset_thunk,
        "cAirStreamerFX_ResetThunk",
    ),
    (0x002f_7c38, vfx_avalanche_ctor, "VfxAvalanche_ctor"),
];

// ---------------------------------------------------------------------------
// RiderBase_DtorCheatCode (0x2f6388)

/// Offsets in the rider's data block (`*self`).
const CHEAT_TABLE_PTR: u32 = 0x6c0;
const CHEAT_HANDLER: u32 = 0x6d0;
const CHEAT_LIST_PTR: u32 = 0x6e8;
/// Table starts in the game image: the cheat list (copied in 0x20-byte
/// chunks up to its end at +0x40), the cheat table (copied up to +0xa0), the
/// handler word and the two relative-offset halfwords.
/// Upper half of the image addresses (`lui 0x49`, then `addiu`).
const CHEAT_IMAGE_HI: u32 = 0x0049_0000;
/// Base destructor (the rider data block, flags 0) and the delete operator.
const RIDER_DTOR: u32 = 0x0011_b978;
const BUILTIN_DELETE: u32 = 0x0031_7e50;

/// `void RiderBase_DtorCheatCode(RiderBase* self, u64 flags)` (0x2f6388): the
/// cheat-code part of the rider destructor. The rider's data block (`*self`)
/// gets its cheat handler, table and list pointed at the defaults in the game
/// image. Unless `flags` is non-zero, the list and the table are first copied
/// to the stack frame and the rider points at the copies, whose
/// offset halfwords are rebased to `self`. Bit 1 of `flags` runs the base
/// destructor `Rider_dtor(data, 0)`; bit 0 deletes `self`.
pub fn rider_base_dtor_cheat_code(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x130);
        c.g[V0][0] = CHEAT_IMAGE_HI as u64;
        frame.save(c, 16, 0x120);
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7920));
        frame.save(c, 17, 0x110);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x100);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = 0x0046_0000;
        c.g[A3][0] = c.g[V0][0];
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6470));
        let list = c.g[A2][0];
        Addr((c.g[V1][0] as u32).wrapping_add(CHEAT_LIST_PTR)).sw(c, list);
        c.g[V1][0] = CHEAT_IMAGE_HI as u64;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[T2][0] = c.g[V1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x78d8));
        let handler = c.g[A0][0];
        Addr((c.g[A1][0] as u32).wrapping_add(CHEAT_HANDLER)).sw(c, handler);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let skip_copy = c.g[S1][0] != 0u64;
        let table = c.g[V1][0];
        Addr((c.g[V0][0] as u32).wrapping_add(CHEAT_TABLE_PTR)).sw(c, table);

        if !skip_copy {
            copy_cheat_tables(c);
        }

        // Bit 1: the base destructor on the data block.
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 0x2;
        let no_base = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if !no_base {
            c.cycles += 4;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A1][0] = 0u64;
            call_ptr(c, RIDER_DTOR, 0x002f_64bc)?;
            c.g[V0][0] = c.g[S1][0] & 0x1;
        }

        // Bit 0: delete `self`.
        c.cycles += 2;
        if c.g[V0][0] != 0u64 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, BUILTIN_DELETE, 0x002f_64d0)?;
        }
        frame.restore(c, 16, 0x120);

        c.cycles += 4;
        frame.restore(c, 17, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// The stack copy of the cheat list and table (the `flags == 0` path of
/// [`rider_base_dtor_cheat_code`]). Loops check events at their back edges.
/// The register values after each loop are the generated body's.
fn copy_cheat_tables(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = c.g[A2][0];
    c.g[V1][0] = c.g[SP][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
    c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    // List: 0x20-byte chunks from the image to the last entry.
    loop {
        c.cycles += 11;
        c.g[A0][0] = Addr(c.g[A1][0] as u32).ld(c);
        c.g[A2][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).ld(c);
        c.g[T1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10)).ld(c);
        c.g[T3][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x18)).ld(c);
        let (w0, w1, w2, w3) = (c.g[A0][0], c.g[A2][0], c.g[T1][0], c.g[T3][0]);
        Addr(c.g[V1][0] as u32).sd(c, w0);
        Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sd(c, w1);
        Addr((c.g[V1][0] as u32).wrapping_add(0x10)).sd(c, w2);
        Addr((c.g[V1][0] as u32).wrapping_add(0x18)).sd(c, w3);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
        let more = c.g[A1][0] != c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002f_63ec);
            }
            continue;
        }
        break;
    }
    // Last list entry, the rebased offset halfword, the list head at the stack.
    c.cycles += 14;
    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7920));
    c.g[A2][0] = Addr(c.g[A1][0] as u32).ld(c);
    let last = c.g[A2][0];
    Addr(c.g[V1][0] as u32).sd(c, last);
    let stack_head = c.g[SP][0];
    Addr((c.g[V0][0] as u32).wrapping_add(CHEAT_LIST_PTR)).sw(c, stack_head);
    c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x78d8));
    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lhu(c);
    c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[A1][0] = c.g[T0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    let rebased = c.g[V1][0];
    Addr(sp(c).wrapping_add(0x8)).sh(c, rebased);
    // Table: 0x20-byte chunks to the end of the table.
    loop {
        c.cycles += 11;
        c.g[V0][0] = Addr(c.g[A2][0] as u32).ld(c);
        c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8)).ld(c);
        c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x10)).ld(c);
        c.g[T3][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x18)).ld(c);
        let (v0, v1, a0, t3) = (c.g[V0][0], c.g[V1][0], c.g[A0][0], c.g[T3][0]);
        Addr(c.g[A1][0] as u32).sd(c, v0);
        Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sd(c, v1);
        Addr((c.g[A1][0] as u32).wrapping_add(0x10)).sd(c, a0);
        Addr((c.g[A1][0] as u32).wrapping_add(0x18)).sd(c, t3);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
        let more = c.g[A2][0] != c.g[T1][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002f_6450);
            }
            continue;
        }
        break;
    }
    c.cycles += 10;
    c.g[V0][0] = Addr(c.g[A2][0] as u32).ld(c);
    c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8)).ld(c);
    let (v0, v1) = (c.g[V0][0], c.g[V1][0]);
    Addr(c.g[A1][0] as u32).sd(c, v0);
    Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sd(c, v1);
    c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x78d8));
    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lhu(c);
    let t0 = c.g[T0][0];
    Addr((c.g[A0][0] as u32).wrapping_add(CHEAT_TABLE_PTR)).sw(c, t0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    let rebased_table = c.g[V1][0];
    Addr(sp(c).wrapping_add(0x58)).sh(c, rebased_table);
}

// ---------------------------------------------------------------------------
// The effect-object reset thunks (0x2f7ab8..0x2f7b90): tail calls to the
// `Reset` of each board effect, 7-cycle forwarding wrappers.

/// `cBoardWakeFX_Reset` (0x2dcf28).
const BOARD_WAKE_RESET: Callee = Callee {
    addr: 0x002d_cf28,
    f: ee::s09::f_002dcf28,
};
/// `cBoardSparkFX_Reset` (0x2daa78).
const BOARD_SPARK_RESET: Callee = Callee {
    addr: 0x002d_aa78,
    f: ee::s09::f_002daa78,
};
/// `cBoardTrailFX_Reset` (0x2e8560).
const BOARD_TRAIL_RESET: Callee = Callee {
    addr: 0x002e_8560,
    f: ee::s09::f_002e8560,
};
/// `cAngerFX_Reset` (0x2d4be0).
const ANGER_RESET: Callee = Callee {
    addr: 0x002d_4be0,
    f: ee::s09::f_002d4be0,
};
/// `cPlayerPulseFX_Reset` (0x2e3930).
const PLAYER_PULSE_RESET: Callee = Callee {
    addr: 0x002e_3930,
    f: ee::s09::f_002e3930,
};
/// `cAirStreamerFX_Reset` (0x2ef6a0).
const AIR_STREAMER_RESET: Callee = Callee {
    addr: 0x002e_f6a0,
    f: ee::s09::f_002ef6a0,
};

/// `void cBoardWakeFX_ResetThunk(void)` (0x2f7ab8): `cBoardWakeFX_Reset()`.
pub fn cboard_wake_fx_reset_thunk(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(BOARD_WAKE_RESET), 0x002f_7ac8)
}

/// `void cBoardSparkFX_ResetThunk(void)` (0x2f7ae0): `cBoardSparkFX_Reset()`.
pub fn cboard_spark_fx_reset_thunk(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(BOARD_SPARK_RESET), 0x002f_7af0)
}

/// `void cBoardTrailFX_ResetThunk(void)` (0x2f7b08): `cBoardTrailFX_Reset()`.
pub fn cboard_trail_fx_reset_thunk(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(BOARD_TRAIL_RESET), 0x002f_7b18)
}

/// `void cAngerFX_ResetThunk(void)` (0x2f7b30): `cAngerFX_Reset()`.
pub fn cang_er_fx_reset_thunk(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(ANGER_RESET), 0x002f_7b40)
}

/// `void cPlayerPulseFX_ResetThunk(void)` (0x2f7b58): `cPlayerPulseFX_Reset()`.
pub fn cplayer_pulse_fx_reset_thunk(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(PLAYER_PULSE_RESET), 0x002f_7b68)
}

/// `void cAirStreamerFX_ResetThunk(void)` (0x2f7b80): `cAirStreamerFX_Reset()`.
pub fn cair_streamer_fx_reset_thunk(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(AIR_STREAMER_RESET), 0x002f_7b90)
}

// ---------------------------------------------------------------------------
// VfxAvalanche_ctor (0x2f7c38)

const AVALANCHE_INIT: Callee = Callee {
    addr: 0x002d_6410,
    f: ee::s09::f_002d6410,
};

/// `void VfxAvalanche_ctor(VfxAvalanche* self)` (0x2f7c38): stores the
/// avalanche vtable in `self` and calls `VfxAvalanche_Init(self)`; returns
/// `self`.
pub fn vfx_avalanche_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = CHEAT_IMAGE_HI as u64;
        frame.save(c, 16, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x79b8));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let vtable = c.g[V0][0];
        Addr(c.g[S0][0] as u32).sw(c, vtable);
        AVALANCHE_INIT.call(c, 0x002f_7c58)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, elf_image, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes (own address and magic in the machine's
    /// RAM: other tests' machines run the real bodies).
    const OC1069_CTRL: u32 = DATA + 0x21_0000;
    const OC1069_MAGIC: u32 = 0x1069_f00d;
    /// Fixture objects: the rider's `*self` holder and the rider data block,
    /// and the avalanche object (clear of the fakes' pointer results).
    const OC1069_THIS: u32 = DATA + 0x3000;
    const OC1069_RIDER: u32 = DATA + 0x4000;
    const OC1069_AVALANCHE: u32 = DATA + 0x5000;

    /// Every callee of the three bodies: a gated fake in its override slot that
    /// logs the call and returns (or unwinds, as the machine chooses).
    macro_rules! oc1069_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1069_CTRL) != OC1069_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            /// Puts every fake in its override slot (once per process).
            fn oc1069_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc1069_fakes! {
        oc1069_fake_002dcf28, 0x002d_cf28, ee::s09::OVERRIDE_002DCF28, ee::s09::f_002dcf28__generated;
        oc1069_fake_002daa78, 0x002d_aa78, ee::s09::OVERRIDE_002DAA78, ee::s09::f_002daa78__generated;
        oc1069_fake_002e8560, 0x002e_8560, ee::s09::OVERRIDE_002E8560, ee::s09::f_002e8560__generated;
        oc1069_fake_002d4be0, 0x002d_4be0, ee::s09::OVERRIDE_002D4BE0, ee::s09::f_002d4be0__generated;
        oc1069_fake_002e3930, 0x002e_3930, ee::s09::OVERRIDE_002E3930, ee::s09::f_002e3930__generated;
        oc1069_fake_002ef6a0, 0x002e_f6a0, ee::s09::OVERRIDE_002EF6A0, ee::s09::f_002ef6a0__generated;
        oc1069_fake_002d6410, 0x002d_6410, ee::s09::OVERRIDE_002D6410, ee::s09::f_002d6410__generated;
        oc1069_fake_0011b978, 0x0011_b978, ee::s00::OVERRIDE_0011B978, ee::s00::f_0011b978__generated;
        oc1069_fake_00317e50, 0x0031_7e50, ee::s10::OVERRIDE_00317E50, ee::s10::f_00317e50__generated;
    }

    /// Hooked fakes, scrambled registers, the gate on.
    fn oc1069_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1069_hook();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1069_CTRL, OC1069_MAGIC);
        r
    }

    /// The rider case: `flags` covers the copy path (zero, half of the seeds),
    /// each destructor bit, and high bits that only the 64-bit tests see.
    fn oc1069_dtor_setup(c: &mut Cpu, seed: u32) -> Events {
        oc1069_hook();
        let mut r = machine(c, seed, 0x2f_6388, 0);
        elf_image(c);
        c.bus.write32(OC1069_CTRL, OC1069_MAGIC);
        let mut flags = if seed % 2 == 0 {
            0u64
        } else {
            [1u64, 2, 3][((seed / 2) % 3) as usize]
        };
        if seed % 16 == 9 {
            flags |= (r.next_u32() as u64) << 32;
        }
        c.bus.write32(OC1069_THIS, OC1069_RIDER);
        c.g[A0][0] = sx(OC1069_THIS);
        c.g[A1][0] = flags;
        schedule(c, &mut r, seed, 0x002f_6388)
    }

    /// The copy path, both destructor bits, and the event deliveries in the
    /// two copy loops, against the generated body.
    #[test]
    fn rider_base_dtor_cheat_code_matches() {
        check_events(
            ee::s09::f_002f6388__generated,
            rider_base_dtor_cheat_code,
            300,
            &oc1069_dtor_setup,
        );
    }

    /// The thunks forward to their `Reset`, with the callee's return or unwind.
    fn oc1069_thunk_setup(c: &mut Cpu, seed: u32, salt: u64) {
        oc1069_machine(c, seed, salt, 1);
    }

    #[test]
    fn cboard_wake_fx_reset_thunk_matches() {
        check(
            ee::s09::f_002f7ab8__generated,
            cboard_wake_fx_reset_thunk,
            40,
            &|c, seed| oc1069_thunk_setup(c, seed, 0x2f_7ab8),
        );
    }

    #[test]
    fn cboard_spark_fx_reset_thunk_matches() {
        check(
            ee::s09::f_002f7ae0__generated,
            cboard_spark_fx_reset_thunk,
            40,
            &|c, seed| oc1069_thunk_setup(c, seed, 0x2f_7ae0),
        );
    }

    #[test]
    fn cboard_trail_fx_reset_thunk_matches() {
        check(
            ee::s09::f_002f7b08__generated,
            cboard_trail_fx_reset_thunk,
            40,
            &|c, seed| oc1069_thunk_setup(c, seed, 0x2f_7b08),
        );
    }

    #[test]
    fn cang_er_fx_reset_thunk_matches() {
        check(
            ee::s09::f_002f7b30__generated,
            cang_er_fx_reset_thunk,
            40,
            &|c, seed| oc1069_thunk_setup(c, seed, 0x2f_7b30),
        );
    }

    #[test]
    fn cplayer_pulse_fx_reset_thunk_matches() {
        check(
            ee::s09::f_002f7b58__generated,
            cplayer_pulse_fx_reset_thunk,
            40,
            &|c, seed| oc1069_thunk_setup(c, seed, 0x2f_7b58),
        );
    }

    #[test]
    fn cair_streamer_fx_reset_thunk_matches() {
        check(
            ee::s09::f_002f7b80__generated,
            cair_streamer_fx_reset_thunk,
            40,
            &|c, seed| oc1069_thunk_setup(c, seed, 0x2f_7b80),
        );
    }

    /// The avalanche object gets its vtable, `Init` runs, `self` comes back.
    #[test]
    fn vfx_avalanche_ctor_matches() {
        check(
            ee::s09::f_002f7c38__generated,
            vfx_avalanche_ctor,
            40,
            &|c, seed| {
                oc1069_machine(c, seed, 0x2f_7c38, 1);
                c.bus.write32(OC1069_AVALANCHE, seed);
                c.g[A0][0] = sx(OC1069_AVALANCHE);
            },
        );
    }
}
