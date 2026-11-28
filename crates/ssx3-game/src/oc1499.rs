//! Staging module oc1499 (wave wave275): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1499`.

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
    (0x0017_fe2c, lbl_0017fe2c, "lbl_0017FE2C"),
    (0x0015_3200, be_race_state_on_exit, "cBERaceState_onExit"),
    (0x0015_3258, be_race_state_on_enter, "cBERaceState_onEnter"),
    (0x0040_c928, recv_info_copy, "func_0040C928"),
    (0x002d_bee0, fx_loader_dtor, "cFXLoader__cFXLoader"),
    (0x002f_17d0, attached_emitter_fx_load_type_data, "AttachedEmitterFX_LoadTypeData"),
    (0x0019_5de0, felocale_load_game_tables, "FELocale_LoadGameTables"),
    (0x003b_7818, sndstrm_create, "SNDSTRM_Create"),
];

/// `lbl_0017FE2C` (0x0017fe2c): sets `a1 = 0x72` and tail-jumps into the
/// middle of `0x0017fe64` (its dispatch arm at 0x0017fe68), which calls the
/// callee at 0x00320c48 and returns to the original caller. The generated
/// body's other arm (a jump to 0x0017fe34) is dead: its condition is always
/// true.
pub fn lbl_0017fe2c(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A1][0] = 0x72;
    c.pc = 0x0017_fe68;
    ssx3_ee_rt::funcs::call_indirect(c, 0x0017_fe68);
}

/// `void cBERaceState::onExit()` (0x00153200): clears the `bIsActive` word
/// (+0x8) of the ten `cBE*Interface_instance` singletons. Each singleton
/// pointer is a global at a `gp` offset; the register chain (`v0`, `v1`, `a0`)
/// is the generated one, load and store in the same order.
pub fn be_race_state_on_exit(c: &mut Cpu) {
    c.cycles += 21;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1ee4)).lw(c);
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1ea8)).lw(c);
    Addr((c.g[V0][0] as u32).wrapping_add(0x8)).sw(c, 0);
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1f28)).lw(c);
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, 0);
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1f30)).lw(c);
    Addr((c.g[A0][0] as u32).wrapping_add(0x8)).sw(c, 0);
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1f3c)).lw(c);
    Addr((c.g[V0][0] as u32).wrapping_add(0x8)).sw(c, 0);
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1eb8)).lw(c);
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, 0);
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1edc)).lw(c);
    Addr((c.g[A0][0] as u32).wrapping_add(0x8)).sw(c, 0);
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1ea4)).lw(c);
    Addr((c.g[V0][0] as u32).wrapping_add(0x8)).sw(c, 0);
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1e8c)).lw(c);
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, 0);
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1ed4)).lw(c);
    Addr((c.g[A0][0] as u32).wrapping_add(0x8)).sw(c, 0);
    Addr((c.g[V0][0] as u32).wrapping_add(0x8)).sw(c, 0);
    ret(c);
}

/// `void cBERaceState::onEnter()` (0x00153258): sets `bIsActive` (+0x8, to 1)
/// on the eight `cBE*Interface_instance` singletons the race state uses. The
/// same register chain as [`be_race_state_on_exit`].
pub fn be_race_state_on_enter(c: &mut Cpu) {
    c.cycles += 18;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1ee4)).lw(c);
    c.g[V0][0] = 1;
    let one = c.g[V0][0];
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1ea8)).lw(c);
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, one);
    c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1f28)).lw(c);
    Addr((c.g[A0][0] as u32).wrapping_add(0x8)).sw(c, one);
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1eb8)).lw(c);
    Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sw(c, one);
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1edc)).lw(c);
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, one);
    c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1ea4)).lw(c);
    Addr((c.g[A0][0] as u32).wrapping_add(0x8)).sw(c, one);
    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1e8c)).lw(c);
    Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sw(c, one);
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1ed4)).lw(c);
    Addr((c.g[A2][0] as u32).wrapping_add(0x8)).sw(c, one);
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, one);
    ret(c);
}

/// `int func_0040C928(void)` (0x0040c928): copies the received info block at
/// `D_00529140` (byte n, byte m, then n data bytes) into the struct that the
/// global `D_00529098` points at: word n at +0, word m at +4, the data at +8.
/// Returns 0.
pub fn recv_info_copy(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V1][0] = 0x0053_0000;
    c.g[V0][0] = 0x0053_0000;
    let info_global = (c.g[V0][0] as u32).wrapping_sub(0x6f68);
    c.g[A3][0] = Addr(info_global).lw(c);
    let recv = (c.g[V1][0] as u32).wrapping_sub(0x6ec0);
    c.g[T0][0] = sx(recv);
    c.g[A0][0] = Addr(recv).lbu(c);
    c.g[A1][0] = 0;
    let n = c.g[A0][0];
    Addr(c.g[A3][0] as u32).sw(c, n);
    c.g[V0][0] = Addr(recv.wrapping_add(1)).lbu(c);
    let n_is_zero = c.g[A0][0] == 0;
    let m = c.g[V0][0];
    Addr((c.g[A3][0] as u32).wrapping_add(4)).sw(c, m);
    if !n_is_zero {
        c.cycles += 2;
        c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(8));
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T0][0] as u32));
        loop {
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(2)).lbu(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            let byte = c.g[V1][0];
            Addr(c.g[A0][0] as u32).sb(c, byte);
            c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T0][0] as u32));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0040_c958);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// `FXLoader` destructor (0x002dbee0, `cFXLoader__cFXLoader`): releases the 81
/// texture handles in the graphics manager's slot array (+0xf50, -1 = empty):
/// each live handle is removed through the texture manager (the sub-object at
/// +0x10d8: this-delta at +0x190, method at +0x194) and its slot reset to -1.
/// With bit 0 of `flags` set it then frees the object (`operator delete`).
pub fn fx_loader_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        c.g[S1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a90)).lw(c);
        frame.save(c, S0, 0x60);
        frame.save(c, S2, 0x40);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xf50));
        frame.save(c, S3, 0x30);
        c.g[S2][0] = 0x50;
        frame.save(c, S4, 0x20);
        c.g[S3][0] = u64::MAX;
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S5][0] = c.g[A0][0];
        loop {
            c.cycles += 3;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
            if c.g[A1][0] == c.g[S3][0] {
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            } else {
                c.cycles += 7;
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10d8)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x190)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x194)).lw(c);
                let method = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x002d_bf38)?;
                let empty = c.g[S3][0];
                Addr(c.g[S0][0] as u32).sw(c, empty);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            }
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_bf18);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S4][0] & 1;
        let keep = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x60);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S5][0];
            call_ptr(c, 0x0031_7e50, 0x002d_bf5c)?;
            frame.restore(c, S0, 0x60);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AttachedEmitterFX_LoadTypeData` (0x002f17d0): allocates the 5440-byte
/// emitter type array (`cAttachedEmitterFX_mEmitterDataArray`, gp+0xac0) and
/// fills it in two steps: `AttachedEmitterTypeData_InitPart1`, then
/// `InitPart2`.
pub fn attached_emitter_fx_load_type_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x0048_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x1540;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7c98));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x002f_17f0)?;
        let data = c.g[V0][0];
        Addr((c.g[GP][0] as u32).wrapping_add(0xac0)).sw(c, data);
        Callee {
            addr: 0x002f_83b0,
            f: ee::s09::f_002f83b0,
        }
        .call(c, 0x002f_17f8)?;
        Callee {
            addr: 0x002f_9838,
            f: ee::s09::f_002f9838,
        }
        .call(c, 0x002f_1800)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FELocale_LoadGameTables(FELocale* self)` (0x00195de0, `cFELocale_loadOV`):
/// marks `self` (+0x58) as 2, picks the locale file from the byte at +1 (0 ->
/// 0xc, 1 -> 0xd, 2 -> 0xe, 3 -> 0xf, anything else -> -1), and loads it through
/// `FELocale_LoadTable` (0x001959a8, `cFELocale_addFile`).
pub fn felocale_load_game_tables(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        c.g[V1][0] = 2;
        let two = c.g[V1][0];
        Addr(this.wrapping_add(0x58)).sb(c, two);
        c.g[V0][0] = 1;
        c.g[A2][0] = Addr(this.wrapping_add(1)).lb(c);
        let kind = c.g[A2][0];
        c.g[A1][0] = u64::MAX;
        if kind == 1 {
            c.cycles += 2;
            c.g[A1][0] = 0xd;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((kind as i64) < 2) as u64;
            let below_two = c.g[V0][0] == 1;
            c.g[V0][0] = 2;
            if below_two {
                c.cycles += 2;
                if kind == 0 {
                    c.g[A1][0] = 0xc;
                } else {
                    c.cycles += 2;
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = 3;
                if kind == 2 {
                    c.cycles += 1;
                    c.g[A1][0] = 0xe;
                } else {
                    c.cycles += 2;
                    if kind == 3 {
                        c.g[A1][0] = 0xf;
                    } else {
                        c.cycles += 2;
                    }
                }
            }
        }
        c.cycles += 5;
        Callee {
            addr: 0x0019_59a8,
            f: ee::s02::f_001959a8,
        }
        .call(c, 0x0019_5e48)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SNDSTRM_create(...)` (0x003b7818): forwards to `SNDSTRMI_Open` with
/// the two trailing arguments (t1, t2) set to 0.
pub fn sndstrm_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[T1][0] = 0;
        frame.save_ra(c, 0);
        c.g[T2][0] = 0;
        Callee {
            addr: 0x003b_7450,
            f: ee::s13::f_003b7450,
        }
        .call(c, 0x003b_782c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Magic word that turns a gated fake on in this fixture's machine.
    const MAGIC: u32 = 0x1499_c0de;
    /// One gate word per fake (each fake answers only in its own machine).
    const GATE_NEW: u32 = DATA + 0x20_1000;
    const GATE_DELETE: u32 = DATA + 0x20_1040;
    const GATE_INIT1: u32 = DATA + 0x20_1080;
    const GATE_INIT2: u32 = DATA + 0x20_10c0;
    const GATE_LOADTABLE: u32 = DATA + 0x20_1100;
    const GATE_SNDOPEN: u32 = DATA + 0x20_1140;
    const GATE_EVAL: u32 = DATA + 0x20_1180;

    /// Gated fakes for the callees the replacements call directly or through
    /// a function pointer: each one logs the call (`fakes::fake`) when its gate
    /// word is set, and runs the body it replaced otherwise.
    macro_rules! gated_fakes {
        ($( ($fake:ident, $gate:expr, $target:expr, $slot:path, $generated:path) ),* $(,)?) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32($gate) != MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake(c, $target, ra);
                }
            )*

            /// Hooks the fakes above (once per process).
            fn install_fakes() {
                $(
                    fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake));
                )*
            }
        };
    }

    gated_fakes! {
        (oc1499_fake_new, GATE_NEW, 0x0031_7e30, ee::s10::OVERRIDE_00317E30, ee::s10::f_00317e30__generated),
        (oc1499_fake_delete, GATE_DELETE, 0x0031_7e50, ee::s10::OVERRIDE_00317E50, ee::s10::f_00317e50__generated),
        (oc1499_fake_init1, GATE_INIT1, 0x002f_83b0, ee::s09::OVERRIDE_002F83B0, ee::s09::f_002f83b0__generated),
        (oc1499_fake_init2, GATE_INIT2, 0x002f_9838, ee::s09::OVERRIDE_002F9838, ee::s09::f_002f9838__generated),
        (oc1499_fake_loadtable, GATE_LOADTABLE, 0x0019_59a8, ee::s02::OVERRIDE_001959A8, ee::s02::f_001959a8__generated),
        (oc1499_fake_sndopen, GATE_SNDOPEN, 0x003b_7450, ee::s13::OVERRIDE_003B7450, ee::s13::f_003b7450__generated),
        (oc1499_fake_eval, GATE_EVAL, 0x0032_0c48, ee::s10::OVERRIDE_00320C48, ee::s10::f_00320c48__generated),
    }

    /// Writes the `gp` globals of the singleton pointers (`gp - off` holds the
    /// address of an object at `DATA + 0x7000 + 0x40 * k`, whose +8 word starts
    /// random).
    fn interface_fixture(c: &mut Cpu, r: &mut Rng, offs: &[u32]) {
        for (k, off) in offs.iter().enumerate() {
            let obj = DATA + 0x7000 + 0x40 * k as u32;
            c.bus.write32(GAME_GP.wrapping_sub(*off), obj);
            c.bus.write32(obj + 8, r.next_u32());
        }
    }

    /// The `gp` offsets of the ten `cBE*Interface_instance` globals that
    /// `onExit` clears, in the order the generated body loads them.
    const EXIT_OFFS: [u32; 10] = [
        0x1ee4, 0x1ea8, 0x1f28, 0x1f30, 0x1f3c, 0x1eb8, 0x1edc, 0x1ea4, 0x1e8c, 0x1ed4,
    ];
    /// The eight globals `onEnter` sets.
    const ENTER_OFFS: [u32; 8] = [0x1ee4, 0x1ea8, 0x1f28, 0x1eb8, 0x1edc, 0x1ea4, 0x1e8c, 0x1ed4];

    #[test]
    fn lbl_0017fe2c_matches() {
        install_fakes();
        check(
            ee::s02::f_0017fe2c__generated,
            lbl_0017fe2c,
            200,
            &|c: &mut Cpu, seed: u32| {
                let mut r = fakes::machine(c, seed, 0x17_fe2c, 0);
                c.bus.write32(GATE_EVAL, MAGIC);
                c.g[A2][0] = (DATA + 0x6000) as u64;
                c.bus.write32(DATA + 0x6018, r.next_u32());
            },
        );
    }

    #[test]
    fn be_race_state_on_exit_matches() {
        check(
            ee::s01::f_00153200__generated,
            be_race_state_on_exit,
            200,
            &|c: &mut Cpu, seed: u32| {
                let mut r = fakes::machine(c, seed, 0x1532_00, 0);
                interface_fixture(c, &mut r, &EXIT_OFFS);
                // Half the seeds: every singleton already points at one object.
                if seed % 2 == 0 {
                    let shared = DATA + 0x7000;
                    for off in EXIT_OFFS {
                        c.bus.write32(GAME_GP.wrapping_sub(off), shared);
                    }
                }
            },
        );
    }

    #[test]
    fn be_race_state_on_enter_matches() {
        check(
            ee::s01::f_00153258__generated,
            be_race_state_on_enter,
            200,
            &|c: &mut Cpu, seed: u32| {
                let mut r = fakes::machine(c, seed, 0x1532_58, 0);
                interface_fixture(c, &mut r, &ENTER_OFFS);
                if seed % 3 == 0 {
                    let shared = DATA + 0x7040;
                    for off in ENTER_OFFS {
                        c.bus.write32(GAME_GP.wrapping_sub(off), shared);
                    }
                }
            },
        );
    }

    #[test]
    fn recv_info_copy_matches() {
        check(
            ee::s15::f_0040c928__generated,
            recv_info_copy,
            300,
            &|c: &mut Cpu, seed: u32| {
                let mut r = fakes::machine(c, seed, 0x40_c928, 0);
                // The destination struct the global points at.
                c.bus.write32(0x0052_9098, DATA + 0x8000);
                // n: zero, one, the full byte range, and small values.
                let n: u8 = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 255,
                    _ => (r.next_u32() % 40) as u8,
                };
                c.bus.write8(0x0052_9140, n);
                c.bus.write8(0x0052_9141, r.next_u32() as u8);
                for i in 0..n as u32 {
                    c.bus.write8(0x0052_9142 + i, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn fx_loader_dtor_matches() {
        install_fakes();
        check(
            ee::s09::f_002dbee0__generated,
            fx_loader_dtor,
            200,
            &|c: &mut Cpu, seed: u32| {
                let mut r = fakes::machine(c, seed, 0x2d_bee0, 0);
                c.bus.write32(GATE_DELETE, MAGIC);
                let grman = DATA + 0x3000;
                c.bus.write32(GAME_GP.wrapping_add(0x2a90), grman);
                // The texture manager at +0x10d8: this-delta at +0x190, method
                // at +0x194 (a miss-hook fake at FAKE).
                let texman = DATA + 0x4000;
                c.bus.write16(texman + 0x190, r.next_u32() as u16);
                c.bus.write32(texman + 0x194, fakes::FAKE);
                c.bus.write32(grman + 0x10d8, texman);
                // The 81 texture slots: about half empty (-1).
                for i in 0..81u32 {
                    let v = if r.next_u32() % 2 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 1000
                    };
                    c.bus.write32(grman + 0xf50 + 4 * i, v);
                }
                c.g[A0][0] = (DATA + 0x6000) as u64;
                // Bit 0 of flags: the object is deleted on half of the seeds.
                c.g[A1][0] = (r.next_u32() as u64 & !1) | (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn attached_emitter_fx_load_type_data_matches() {
        install_fakes();
        check(
            ee::s09::f_002f17d0__generated,
            attached_emitter_fx_load_type_data,
            200,
            &|c: &mut Cpu, seed: u32| {
                let _r = fakes::machine(c, seed, 0x2f_17d0, 0);
                c.bus.write32(GATE_NEW, MAGIC);
                c.bus.write32(GATE_INIT1, MAGIC);
                c.bus.write32(GATE_INIT2, MAGIC);
            },
        );
    }

    #[test]
    fn felocale_load_game_tables_matches() {
        install_fakes();
        check(
            ee::s02::f_00195de0__generated,
            felocale_load_game_tables,
            200,
            &|c: &mut Cpu, seed: u32| {
                let mut r = fakes::machine(c, seed, 0x19_5de0, 0);
                c.bus.write32(GATE_LOADTABLE, MAGIC);
                let this = DATA + 0x6000;
                c.g[A0][0] = this as u64;
                // The locale byte: each switch arm, then the out-of-range values.
                let kind: u8 = match seed % 9 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 4,
                    5 => 0xff,
                    6 => 0x80,
                    7 => 0x7f,
                    _ => r.next_u32() as u8,
                };
                c.bus.write8(this + 1, kind);
            },
        );
    }

    #[test]
    fn sndstrm_create_matches() {
        install_fakes();
        check(
            ee::s13::f_003b7818__generated,
            sndstrm_create,
            200,
            &|c: &mut Cpu, seed: u32| {
                let _r = fakes::machine(c, seed, 0x3b_7818, 0);
                c.bus.write32(GATE_SNDOPEN, MAGIC);
            },
        );
    }
}
