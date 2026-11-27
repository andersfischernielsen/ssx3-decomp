//! Staging module oc1022 (wave wave262): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1022`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0028_6ea0,
    ssx_audio_system_on_rider_finish,
    "SSXAudioSystem_OnRiderFinish",
)];

/// `reg + off` as a 32-bit address.
#[inline]
fn oc1022_ea(reg: u64, off: u32) -> u32 {
    (reg as u32).wrapping_add(off)
}

/// The address `gp - neg`.
#[inline]
fn oc1022_gp_minus(c: &Cpu, neg: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(neg)
}

/// `void SSXAudioSystem::OnRiderFinish(Rider* rider)` (0x286ea0): `this = a0`,
/// `rider = a1`. Ignored unless the rider is the one the audio system tracks
/// for its player slot (`rider+0x870`). Reports the finish to the sound
/// controller (`gp+0x410`, mode 2, rider id unless a replay is playing),
/// updates the rider's, race and music state, switches the game music (monitor
/// message `0x24` when the finish is in the racing modes 2/3 and the player
/// is ahead, else message `0xa`), then, for a rider that has not finished
/// (`rider+0x480 == 0`), starts the finish speech (`0x2a3708`) or the
/// celebration (`0x2a34d0`) and finally the finish stinger (`0x2a45c0`) in
/// the plain race mode.
pub fn ssx_audio_system_on_rider_finish(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw(oc1022_ea(c.g[S1][0], 0x870));
        Callee {
            addr: 0x0028_5d98,
            f: ee::s07::f_00285d98,
        }
        .call(c, 0x0028_6ed0)?;
        if c.g[S1][0] != c.g[V0][0] {
            frame.restore(c, S0, 0x60);
            return oc1022_orf_finish(c, frame);
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0028_8ae0,
            f: ee::s07::f_00288ae0,
        }
        .call(c, 0x0028_6ee0)?;
        let replay = c.g[V0][0] != 0;
        c.g[A2][0] = 0;
        if !replay {
            c.cycles += 1;
            c.g[A2][0] = c.lw(oc1022_ea(c.g[S1][0], 0x870));
        }
        c.cycles += 29;
        c.g[A0][0] = c.lw(oc1022_ea(c.g[GP][0], 0x410));
        c.g[A1][0] = 2;
        c.g[S4][0] = 1;
        call_ptr(c, 0x002a_73a8, 0x0028_6efc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0029_6e20, 0x0028_6f08)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0029_7438, 0x0028_6f14)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0029_b3c0, 0x0028_6f20)?;
        c.g[V0][0] = c.lw(oc1022_gp_minus(c, 0x848));
        c.g[V1][0] = c.lw(oc1022_ea(c.g[V0][0], 0xc0));
        c.g[S3][0] = c.lw(c.g[V1][0] as u32);
        c.g[S2][0] = c.lw(oc1022_ea(c.g[V1][0], 0x70));
        call_ptr(c, 0x0014_dc80, 0x0028_6f34)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0028_6f40)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[S5][0] = c.g[V1][0];
        c.g[V1][0] = sx(oc1022_ea(c.g[V1][0], 0x5bc8));
        c.g[V0][0] = c.lb(oc1022_ea(c.g[V1][0], 0x48));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(2));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        let not_race_mode = c.g[V0][0] == 0;
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S2][0] as i64)) as u64;
        if !not_race_mode {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x0028_d960,
                    f: ee::s07::f_0028d960,
                }
                .call(c, 0x0028_6f70)?;
                let music_off = c.g[V0][0] == 0;
                c.g[A2][0] = sx(oc1022_ea(c.g[S0][0], 0x118));
                if !music_off {
                    c.cycles += 8;
                    c.g[V1][0] = c.lw(oc1022_ea(c.g[S0][0], 0x5558));
                    c.g[A1][0] = 0x24;
                    c.g[S4][0] = 0;
                    c.g[A0][0] = c.lh(oc1022_ea(c.g[V1][0], 0x20));
                    c.g[V0][0] = c.lw(oc1022_ea(c.g[V1][0], 0x24));
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(oc1022_ea(c.g[A2][0], c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0028_6f94)?;
                    c.sw(oc1022_ea(c.g[S0][0], 0x627c), 0);
                }
            }
        }
        c.cycles += 2;
        if c.g[S4][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0028_d960,
                f: ee::s07::f_0028d960,
            }
            .call(c, 0x0028_6fa8)?;
            let music_off = c.g[V0][0] == 0;
            c.g[A2][0] = sx(oc1022_ea(c.g[S0][0], 0x118));
            if !music_off {
                c.cycles += 6;
                c.g[V1][0] = c.lw(oc1022_ea(c.g[S0][0], 0x5558));
                c.g[A1][0] = 0xa;
                c.g[A0][0] = c.lh(oc1022_ea(c.g[V1][0], 0x20));
                c.g[V0][0] = c.lw(oc1022_ea(c.g[V1][0], 0x24));
                let method = c.g[V0][0] as u32;
                c.g[A0][0] = sx(oc1022_ea(c.g[A2][0], c.g[A0][0] as u32));
                call_ptr(c, method, 0x0028_6fc8)?;
            }
        }
        // the rider's own finish sound
        c.cycles += 8;
        call_ptr(c, 0x0014_dc80, 0x0028_6fd0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0028_6fdc)?;
        c.g[V1][0] = c.lw(oc1022_ea(c.g[S1][0], 0x480));
        let already = c.g[V1][0] != 0;
        c.g[V0][0] = sx(oc1022_ea(c.g[S5][0], 0x5bc8));
        if !already {
            // Which start: the finish speech (two code paths, `a` and `b`, that
            // differ in cycles and return address) or the celebration.
            #[derive(PartialEq)]
            enum Start {
                SpeechA,
                SpeechB,
                Celebrate,
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_4078, 0x0028_6ff0)?;
            let tracked = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S1][0];
            let start;
            if tracked {
                start = Start::SpeechA;
            } else {
                c.cycles += 5;
                c.g[V0][0] = sx(oc1022_ea(c.g[S5][0], 0x5bc8));
                c.g[A0][0] = 2;
                c.g[V1][0] = c.lb(oc1022_ea(c.g[V0][0], 0x49));
                let not_mode2 = c.g[V1][0] != c.g[A0][0];
                c.g[V0][0] = ((c.g[S3][0] as i64) < 3) as u64;
                if !not_mode2 {
                    c.cycles += 6;
                    Callee {
                        addr: 0x0028_b1d8,
                        f: ee::s07::f_0028b1d8,
                    }
                    .call(c, 0x0028_7014)?;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0012_a250, 0x0028_701c)?;
                    let no_race = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S1][0];
                    start = if no_race {
                        Start::Celebrate
                    } else {
                        Start::SpeechA
                    };
                } else {
                    c.cycles += 2;
                    let few_players = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[S1][0];
                    start = if few_players {
                        Start::Celebrate
                    } else {
                        Start::SpeechB
                    };
                }
            }
            match start {
                Start::SpeechA => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002a_3708, 0x0028_702c)?;
                }
                Start::Celebrate => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = 0;
                    c.g[A3][0] = u64::MAX;
                    call_ptr(c, 0x002a_34d0, 0x0028_7050)?;
                }
                Start::SpeechB => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002a_3708, 0x0028_7060)?;
                }
            }
            c.g[V0][0] = sx(oc1022_ea(c.g[S5][0], 0x5bc8));
        }
        c.cycles += 3;
        c.g[V1][0] = c.lb(oc1022_ea(c.g[V0][0], 0x49));
        if c.g[V1][0] != 0 {
            frame.restore(c, S0, 0x60);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_45c0, 0x0028_7078)?;
            frame.restore(c, S0, 0x60);
        }
        oc1022_orf_finish(c, frame)
    });
}

/// Epilogue of `OnRiderFinish` (`s0` is restored by the caller).
fn oc1022_orf_finish(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 8;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate of this module's callee fakes, in each test machine's RAM.
    const OC1022_CTRL: u32 = DATA + 0x20_0000;
    const OC1022_MAGIC: u32 = 0x1022_5a7e;
    /// Forced results: per callee a `(MAGIC, v0)` word pair at `addr * 2`.
    const OC1022_MAIL: u32 = DATA + 0x40_0000;

    fn oc1022_mail(addr: u32) -> u32 {
        OC1022_MAIL + (addr & 0xf_fffc) * 2
    }

    /// Force the `v0` of the callee at `addr` for this machine.
    fn oc1022_force(c: &mut Cpu, addr: u32, v0: u32) {
        c.bus.write32(oc1022_mail(addr), OC1022_MAGIC);
        c.bus.write32(oc1022_mail(addr) + 4, v0);
    }

    fn oc1022_fake_body(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let forced = c.bus.read32(oc1022_mail(addr)) == OC1022_MAGIC;
        let v0 = c.bus.read32(oc1022_mail(addr) + 4);
        fakes::fake(c, addr, ra);
        if c.pc == ra && forced {
            c.g[V0][0] = sx(v0);
        }
    }

    macro_rules! oc1022_fakes {
        ($(($fake:ident, $prev:ident, $slot:path, $gen:path, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1022_CTRL) != OC1022_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1022_fake_body(c, $addr);
                }
            )*
            fn oc1022_install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1022_fakes!(
        (
            oc1022_f_285d98,
            OC1022_P_285D98,
            ee::s07::OVERRIDE_00285D98,
            ee::s07::f_00285d98__generated,
            0x0028_5d98
        ),
        (
            oc1022_f_2a73a8,
            OC1022_P_2A73A8,
            ee::s08::OVERRIDE_002A73A8,
            ee::s08::f_002a73a8__generated,
            0x002a_73a8
        ),
        (
            oc1022_f_288ae0,
            OC1022_P_288AE0,
            ee::s07::OVERRIDE_00288AE0,
            ee::s07::f_00288ae0__generated,
            0x0028_8ae0
        ),
        (
            oc1022_f_296e20,
            OC1022_P_296E20,
            ee::s08::OVERRIDE_00296E20,
            ee::s08::f_00296e20__generated,
            0x0029_6e20
        ),
        (
            oc1022_f_297438,
            OC1022_P_297438,
            ee::s08::OVERRIDE_00297438,
            ee::s08::f_00297438__generated,
            0x0029_7438
        ),
        (
            oc1022_f_29b3c0,
            OC1022_P_29B3C0,
            ee::s08::OVERRIDE_0029B3C0,
            ee::s08::f_0029b3c0__generated,
            0x0029_b3c0
        ),
        (
            oc1022_f_14dc80,
            OC1022_P_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc1022_f_14dd58,
            OC1022_P_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc1022_f_28d960,
            OC1022_P_28D960,
            ee::s07::OVERRIDE_0028D960,
            ee::s07::f_0028d960__generated,
            0x0028_d960
        ),
        (
            oc1022_f_2a4078,
            OC1022_P_2A4078,
            ee::s08::OVERRIDE_002A4078,
            ee::s08::f_002a4078__generated,
            0x002a_4078
        ),
        (
            oc1022_f_2a3708,
            OC1022_P_2A3708,
            ee::s08::OVERRIDE_002A3708,
            ee::s08::f_002a3708__generated,
            0x002a_3708
        ),
        (
            oc1022_f_12a250,
            OC1022_P_12A250,
            ee::s00::OVERRIDE_0012A250,
            ee::s00::f_0012a250__generated,
            0x0012_a250
        ),
        (
            oc1022_f_28b1d8,
            OC1022_P_28B1D8,
            ee::s07::OVERRIDE_0028B1D8,
            ee::s07::f_0028b1d8__generated,
            0x0028_b1d8
        ),
        (
            oc1022_f_2a34d0,
            OC1022_P_2A34D0,
            ee::s08::OVERRIDE_002A34D0,
            ee::s08::f_002a34d0__generated,
            0x002a_34d0
        ),
        (
            oc1022_f_2a45c0,
            OC1022_P_2A45C0,
            ee::s08::OVERRIDE_002A45C0,
            ee::s08::f_002a45c0__generated,
            0x002a_45c0
        ),
    );

    const OC1022_AUDIO: u32 = DATA + 0x100;
    const OC1022_RIDER: u32 = DATA + 0x8000;
    const OC1022_WORLD: u32 = DATA + 0x9000;
    const OC1022_CFG: u32 = DATA + 0x9100;
    const OC1022_VT: u32 = DATA + 0xa000;

    /// `SSXAudioSystem::OnRiderFinish`: rider is (or is not) the tracked one;
    /// replay playing or not; race modes 2/3 with the player ahead or not;
    /// music enabled or not; rider already finished or not; and the three
    /// ways to start the finish sound (speech via the tracked check, the
    /// race-manager check in mode 2, or the player count); with the stinger
    /// for state byte 0 only. The callees are gated fakes with forced
    /// results (and some seeds unwind at a call).
    #[test]
    fn ssx_audio_system_on_rider_finish_matches() {
        oc1022_install();
        covered(
            ee::s07::f_00286ea0__generated,
            ssx_audio_system_on_rider_finish,
            400,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x1022_6ea0, 16);
                c.bus.write32(OC1022_CTRL, OC1022_MAGIC);
                c.pc = 0x0028_6ea0;
                c.g[A0][0] = sx(OC1022_AUDIO);
                c.g[A1][0] = sx(OC1022_RIDER);
                c.bus.write32(OC1022_RIDER + 0x870, r.next_u32() % 4);
                let finished = seed % 3 == 0;
                c.bus.write32(
                    OC1022_RIDER + 0x480,
                    if finished { r.next_u32() | 1 } else { 0 },
                );
                // FindRider returns the rider itself, except for 1 seed in 7.
                oc1022_force(
                    c,
                    0x0028_5d98,
                    if seed % 7 == 0 {
                        OC1022_RIDER + 0x40
                    } else {
                        OC1022_RIDER
                    },
                );
                oc1022_force(c, 0x0028_8ae0, (seed / 2 % 2) as u32);
                oc1022_force(c, 0x0028_d960, (seed / 3 % 4 != 0) as u32);
                oc1022_force(c, 0x002a_4078, (seed / 4 % 2) as u32);
                oc1022_force(c, 0x0012_a250, (seed / 5 % 2) as u32);
                c.bus.write32(GAME_GP + 0x410, DATA + 0x7000);
                c.bus.write32(GAME_GP - 0x848, OC1022_WORLD);
                c.bus.write32(OC1022_WORLD + 0xc0, OC1022_CFG);
                // a (cfg+0) against b (cfg+0x70): a < b, a < 3 and not.
                let a = [0u32, 2, 3, 5][(seed / 6 % 4) as usize];
                let b = if seed / 24 % 2 == 0 {
                    a + 1
                } else {
                    a.wrapping_sub(1)
                };
                c.bus.write32(OC1022_CFG, a);
                c.bus.write32(OC1022_CFG + 0x70, b);
                // game mode byte (0x535c10) and state byte (0x535c11).
                c.bus
                    .write8(0x0053_5c10, [2u8, 3, 0, 1][(seed / 48 % 4) as usize]);
                c.bus
                    .write8(0x0053_5c11, [0u8, 2, 1, 2][(seed / 7 % 4) as usize]);
                // the audio system's music monitor vtable slot at +0x20/+0x24
                c.bus.write32(OC1022_AUDIO + 0x5558, OC1022_VT);
                c.bus
                    .write16(OC1022_VT + 0x20, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(OC1022_VT + 0x24, fakes::FAKE);
                c.bus.write32(OC1022_AUDIO + 0x627c, r.next_u32());
            },
        );
    }
}
