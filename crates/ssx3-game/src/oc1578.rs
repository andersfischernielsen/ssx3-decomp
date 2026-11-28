//! Staging module oc1578 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1578`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_77c8, rand_seed_main, "Rand_SeedMain"),
    (0x0031_7890, rand_float_spread, "Rand_FloatSpread"),
    (0x0031_7908, rand_restore_game_state, "Rand_RestoreGameState"),
    (0x0031_7930, rand_get_default_state, "Rand_GetDefaultState"),
    (0x0031_9610, ssx_iop_init, "ssx_IopInit"),
    (0x0031_9790, ssx_unload_iop_module, "ssx_UnloadIopModule"),
    (0x0031_9800, ssx_exit_msif_rpc, "ssx_ExitMsifRpc"),
    (0x0031_9820, ssx_iop_reboot, "ssx_IopReboot"),
];

// ---------------------------------------------------------------------------
// Rand_SeedMain (0x3177c8)

/// `void Rand_SeedMain(u32 seed)` (0x3177c8): seeds the main random generator
/// state at `0x4ff018` (`0x500000 - 0xfe8`) with `Rand_Seed(state, seed)`.
/// The seed in a0 is passed on unchanged as a1. Frame 0x10.
pub fn rand_seed_main(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A1][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x0050_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xfe8));
        Callee {
            addr: 0x0031_7958,
            f: ee::s10::f_00317958,
        }
        .call(c, 0x0031_77e0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rand_FloatSpread (0x317890)

/// `float Rand_FloatSpread(float base, float spread)` (0x317890): `base +
/// spread * r`, where `r = Rand_FloatRange(-1.0, 1.0)` is in `[-1, 1)`.
/// Frame 0x20; f20/f21 hold the spread and the base across the call.
pub fn rand_float_spread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save_fpr(c, 21, 0x18);
        frame.save_fpr(c, 20, 0x10);
        c.mov_s(21, 12); // f21 = base
        c.mov_s(20, 13); // f20 = spread
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 12); // f12 = -1.0
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 13); // f13 = 1.0
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0031_7830,
            f: ee::s10::f_00317830,
        }
        .call(c, 0x0031_78c0)?;
        c.f[20] = (FpuF32(c.f[20]) * FpuF32(c.f[0])).0; // spread * r
        frame.restore_ra(c, 0);
        c.f[0] = (FpuF32(c.f[21]) + FpuF32(c.f[20])).0; // base + spread * r
        frame.restore_fpr(c, 21, 0x18);
        frame.restore_fpr(c, 20, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rand_RestoreGameState (0x317908)

/// `void Rand_RestoreGameState(void* src)` (0x317908): copies the 0x18-byte
/// game random state from `src` (a0) to `0x4ff030` (`0x500000 - 0xfd0`) with
/// `EA_MemCopy`. Frame 0x10.
pub fn rand_restore_game_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[A1][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x0050_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xfd0));
        c.g[A2][0] = 0x18;
        call_ptr(c, 0x003e_6574, 0x0031_7924)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rand_GetDefaultState (0x317930)

/// `void Rand_GetDefaultState(void* dst)` (0x317930): copies the 0x18-byte
/// default random state at `0x48dcb0` (`0x490000 - 0x2350`) to `dst` (a0,
/// passed on unchanged) with `EA_MemCopy`. Frame 0x10.
pub fn rand_get_default_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x0049_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2350));
        c.g[A2][0] = 0x18;
        call_ptr(c, 0x003e_6574, 0x0031_7948)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ssx_IopInit (0x319610)

/// `void ssx_IopInit(void* self)` (0x319610): starts the IOP side of the
/// system: the SIF RPC and IOP heap, the reboot of the IOP (`ssx_IopReboot`
/// with `self`), then the CD and file system setup. Frame 0x20; s0 = self.
pub fn ssx_iop_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        Callee {
            addr: 0x0042_6408,
            f: ee::s15::f_00426408,
        }
        .call(c, 0x0031_9628)?;
        Callee {
            addr: 0x0042_ab18,
            f: ee::s15::f_0042ab18,
        }
        .call(c, 0x0031_9630)?;
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0031_9820,
            f: ee::s10::f_00319820,
        }
        .call(c, 0x0031_9638)?;
        c.g[A0][0] = 0;
        Callee {
            addr: 0x0040_1718,
            f: ee::s15::f_00401718,
        }
        .call(c, 0x0031_9640)?;
        c.g[A0][0] = 2;
        Callee {
            addr: 0x0040_2450,
            f: ee::s15::f_00402450,
        }
        .call(c, 0x0031_9648)?;
        Callee {
            addr: 0x0042_7d28,
            f: ee::s15::f_00427d28,
        }
        .call(c, 0x0031_9650)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ssx_UnloadIopModule (0x319790)

/// `void ssx_UnloadIopModule(void* self, ...)` (0x319790): stops the IOP
/// module (`sceSifStopModule`, with a2 = 0 and a3 = the frame's scratch
/// word), then unloads it (`sceSifUnloadModule` with `self`). Frame 0x30;
/// s0 = a1.
pub fn ssx_unload_iop_module(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        Callee {
            addr: 0x0042_b438,
            f: ee::s15::f_0042b438,
        }
        .call(c, 0x0031_97b4)?;
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0042_b640,
            f: ee::s15::f_0042b640,
        }
        .call(c, 0x0031_97bc)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ssx_ExitMsifRpc (0x319800)

/// `void ssx_ExitMsifRpc(void* self)` (0x319800): `sceSifMExitRpc`, with a0
/// passed through. Frame 0x10.
pub fn ssx_exit_msif_rpc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0040_b1f8,
            f: ee::s15::f_0040b1f8,
        }
        .call(c, 0x0031_9810)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ssx_IopReboot (0x319820)

/// `void ssx_IopReboot(void* self)` (0x319820): resets the CD and the SIF
/// RPC, builds the IOP module path (`ssx_BuildModulePath` into the frame's
/// buffer, module name at `0x48d9f8`), then reboots the IOP. The reboot is
/// retried while it answers 0; after a non-zero answer the IOP is synced
/// (retried while 0), and the RPC is re-initialised. Frame 0x120; s0 = self.
/// The loops check events at their back edges.
pub fn ssx_iop_reboot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x120);
        frame.save(c, S0, 0x110);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x100);
        c.g[A0][0] = 0;
        Callee {
            addr: 0x0042_6408,
            f: ee::s15::f_00426408,
        }
        .call(c, 0x0031_9838)?;
        c.g[A0][0] = 0;
        Callee {
            addr: 0x0040_1718,
            f: ee::s15::f_00401718,
        }
        .call(c, 0x0031_9840)?;
        c.g[A0][0] = 2;
        Callee {
            addr: 0x0040_2450,
            f: ee::s15::f_00402450,
        }
        .call(c, 0x0031_9848)?;
        c.g[A2][0] = 0x0049_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x2608));
        c.g[A1][0] = c.g[SP][0];
        Callee {
            addr: 0x0031_9690,
            f: ee::s10::f_00319690,
        }
        .call(c, 0x0031_985c)?;
        loop {
            c.cycles += 4;
            c.g[A0][0] = c.g[SP][0];
            Callee {
                addr: 0x0042_bf68,
                f: ee::s15::f_0042bf68,
            }
            .call(c, 0x0031_9868)?;
            if c.g[V0][0] == 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0031_9860);
                }
                continue;
            }
            loop {
                c.cycles += 4;
                Callee {
                    addr: 0x0042_bf30,
                    f: ee::s15::f_0042bf30,
                }
                .call(c, 0x0031_9878)?;
                if c.g[V0][0] == 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_9870);
                    }
                    continue;
                }
                c.cycles += 8;
                c.g[A0][0] = 0;
                Callee {
                    addr: 0x0042_6408,
                    f: ee::s15::f_00426408,
                }
                .call(c, 0x0031_9888)?;
                Callee {
                    addr: 0x0042_ab18,
                    f: ee::s15::f_0042ab18,
                }
                .call(c, 0x0031_9890)?;
                frame.restore(c, S0, 0x110);
                frame.restore_ra(c, 0x100);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// The main random generator state (`0x4ff018`) and the game state (`0x4ff030`).
    const MAIN_STATE: u32 = 0x004f_f018;
    const GAME_STATE: u32 = 0x004f_f030;
    /// The default state the default-state copy reads (`0x48dcb0`).
    const DEFAULT_STATE: u32 = 0x0048_dcb0;

    #[test]
    fn rand_seed_main_matches() {
        check(
            ee::s10::f_003177c8__generated,
            rand_seed_main,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3177c8, 0);
                c.pc = 0x0031_77c8;
                c.g[A0][0] = r.next_u32() as u64;
                fill(c, &mut r, MAIN_STATE, 6);
            },
        );
    }

    /// The random spread of `Rand_FloatSpread`: ordinary values and edge cases
    /// for the base and the spread, over random generator states.
    #[test]
    fn rand_float_spread_matches() {
        check(
            ee::s10::f_00317890__generated,
            rand_float_spread,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x317890, 0);
                c.pc = 0x0031_7890;
                c.g[RA][0] = 0x0012_3450;
                let (base, spread) = if seed % 3 == 0 {
                    (r.edge_f32(), r.edge_f32())
                } else {
                    (r.f32(100.0), r.f32(10.0))
                };
                c.f[12] = base;
                c.f[13] = spread;
                fill(c, &mut r, GAME_STATE, 8);
            },
        );
    }

    #[test]
    fn rand_restore_game_state_matches() {
        check(
            ee::s10::f_00317908__generated,
            rand_restore_game_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x317908, 0);
                c.pc = 0x0031_7908;
                // A saved state at an unaligned or aligned source (EA_MemCopy's
                // byte head or word path).
                let src = DATA + 0x100 + (seed % 4);
                fill(c, &mut r, src & !3, 8);
                c.g[A0][0] = src as u64;
            },
        );
    }

    #[test]
    fn rand_get_default_state_matches() {
        check(
            ee::s10::f_00317930__generated,
            rand_get_default_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x317930, 0);
                c.pc = 0x0031_7930;
                fill(c, &mut r, DEFAULT_STATE, 6);
                // The destination is a0 (kept): unaligned sometimes.
                c.g[A0][0] = (DATA + 0x200 + (seed % 4)) as u64;
            },
        );
    }

    /// The gated IOP fakes. Each answers through `fakes::fake` (logged, the
    /// call index may unwind), and falls back to the body it replaced when the
    /// gate word in the machine's RAM is not set.
    const IOP_GATE: u32 = DATA + 0x2_f000;
    const IOP_MAGIC: u32 = 0x1578_0001;
    /// Calls to `sceSifRebootIop` / `sceSifSyncIop` still answering 0.
    const IOP_REBOOT_LEFT: u32 = DATA + 0x2_f008;
    const IOP_SYNC_LEFT: u32 = DATA + 0x2_f00c;

    fn oc1578_no_prev(_c: &mut Cpu) {
        unreachable!("oc1578 fake called before it was hooked");
    }

    macro_rules! oc1578_fake {
        ($name:ident, $target:literal) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(IOP_GATE) != IOP_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), oc1578_no_prev);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    oc1578_fake!(oc1578_fake_426408, 0x0042_6408); // sceSifInitRpc
    oc1578_fake!(oc1578_fake_42ab18, 0x0042_ab18); // sceSifInitIopHeap
    oc1578_fake!(oc1578_fake_401718, 0x0040_1718); // sceCdInit
    oc1578_fake!(oc1578_fake_402450, 0x0040_2450); // sceCdMmode
    oc1578_fake!(oc1578_fake_427d28, 0x0042_7d28); // sceFsReset
    oc1578_fake!(oc1578_fake_40b1f8, 0x0040_b1f8); // sceSifMExitRpc
    oc1578_fake!(oc1578_fake_42b438, 0x0042_b438); // sceSifStopModule
    oc1578_fake!(oc1578_fake_42b640, 0x0042_b640); // sceSifUnloadModule
    oc1578_fake!(oc1578_fake_319690, 0x0031_9690); // ssx_BuildModulePath

    /// Returns 0 (not ready) while the countdown at `cell` runs, then 1.
    fn oc1578_countdown(c: &mut Cpu, cell: u32) -> u64 {
        let left = c.bus.read32(cell);
        if left > 0 {
            c.bus.write32(cell, left - 1);
            0
        } else {
            1
        }
    }

    fn oc1578_fake_reboot(c: &mut Cpu) {
        if c.bus.read32(IOP_GATE) != IOP_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1578_fake_reboot), oc1578_no_prev);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0042_bf68, ra); // sceSifRebootIop
        c.g[V0][0] = oc1578_countdown(c, IOP_REBOOT_LEFT);
    }

    fn oc1578_fake_sync(c: &mut Cpu) {
        if c.bus.read32(IOP_GATE) != IOP_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1578_fake_sync), oc1578_no_prev);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0042_bf30, ra); // sceSifSyncIop
        c.g[V0][0] = oc1578_countdown(c, IOP_SYNC_LEFT);
    }

    /// Hooks every gated fake into its callee's override slot (once per process).
    fn oc1578_hook() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00426408),
            oc1578_fake_426408,
            fakes::prev_cell(oc1578_fake_426408),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042AB18),
            oc1578_fake_42ab18,
            fakes::prev_cell(oc1578_fake_42ab18),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00401718),
            oc1578_fake_401718,
            fakes::prev_cell(oc1578_fake_401718),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402450),
            oc1578_fake_402450,
            fakes::prev_cell(oc1578_fake_402450),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00427D28),
            oc1578_fake_427d28,
            fakes::prev_cell(oc1578_fake_427d28),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0040B1F8),
            oc1578_fake_40b1f8,
            fakes::prev_cell(oc1578_fake_40b1f8),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042B438),
            oc1578_fake_42b438,
            fakes::prev_cell(oc1578_fake_42b438),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042B640),
            oc1578_fake_42b640,
            fakes::prev_cell(oc1578_fake_42b640),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00319690),
            oc1578_fake_319690,
            fakes::prev_cell(oc1578_fake_319690),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042BF68),
            oc1578_fake_reboot,
            fakes::prev_cell(oc1578_fake_reboot),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042BF30),
            oc1578_fake_sync,
            fakes::prev_cell(oc1578_fake_sync),
        );
    }

    /// A machine with the gate set and the RPC countdowns: `reboot` calls
    /// answer 0 first, then the sync calls.
    fn oc1578_iop_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, reboot: u32, sync: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(IOP_GATE, IOP_MAGIC);
        c.bus.write32(IOP_REBOOT_LEFT, reboot);
        c.bus.write32(IOP_SYNC_LEFT, sync);
        r
    }

    #[test]
    fn ssx_iop_init_matches() {
        oc1578_hook();
        check(
            ee::s10::f_00319610__generated,
            ssx_iop_init,
            200,
            &|c, seed| {
                let _r = oc1578_iop_machine(c, seed, 0x319610, 6, seed % 3, (seed / 3) % 3);
                c.pc = 0x0031_9610;
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }

    #[test]
    fn ssx_unload_iop_module_matches() {
        oc1578_hook();
        check(
            ee::s10::f_00319790__generated,
            ssx_unload_iop_module,
            200,
            &|c, seed| {
                let _r = oc1578_iop_machine(c, seed, 0x319790, 6, 0, 0);
                c.pc = 0x0031_9790;
                c.g[A1][0] = (DATA + 0x200 + 0x10 * (seed % 4)) as u64;
            },
        );
    }

    #[test]
    fn ssx_exit_msif_rpc_matches() {
        oc1578_hook();
        check(
            ee::s10::f_00319800__generated,
            ssx_exit_msif_rpc,
            200,
            &|c, seed| {
                let _r = oc1578_iop_machine(c, seed, 0x319800, 6, 0, 0);
                c.pc = 0x0031_9800;
                c.g[A0][0] = (DATA + 0x300 + 0x10 * (seed % 4)) as u64;
            },
        );
    }

    /// The reboot loops: the reboot answers 0 for `seed % 3` calls, the sync
    /// for `(seed / 3) % 3`, with unwinding fakes at random call indices.
    #[test]
    fn ssx_iop_reboot_matches() {
        oc1578_hook();
        check(
            ee::s10::f_00319820__generated,
            ssx_iop_reboot,
            200,
            &|c, seed| {
                let _r = oc1578_iop_machine(c, seed, 0x319820, 6, seed % 3, (seed / 3) % 3);
                c.pc = 0x0031_9820;
                c.g[A0][0] = (DATA + 0x400) as u64;
            },
        );
    }

    /// Events delivered at the reboot loops' back edges (every seed retries the
    /// reboot at least once, so the event checks run).
    #[test]
    fn ssx_iop_reboot_events() {
        oc1578_hook();
        check_events(
            ee::s10::f_00319820__generated,
            ssx_iop_reboot,
            120,
            &|c, seed| {
                let mut r = oc1578_iop_machine(c, seed, 0x319820, 0, 1 + seed % 3, (seed / 3) % 3);
                c.pc = 0x0031_9820;
                c.g[A0][0] = (DATA + 0x400) as u64;
                schedule(c, &mut r, seed, 0x0031_9820)
            },
        );
    }
}
