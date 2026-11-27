//! Staging module oc1019 (wave wave261): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1019`.

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
        0x0028_9af8,
        ssx_audio_system_rider_release,
        "SSXAudioSystem_RiderRelease",
    ),
    (
        0x0028_9dc0,
        ssx_audio_system_get_output_mode_option,
        "SSXAudioSystem_GetOutputModeOption",
    ),
    (
        0x0028_9de0,
        ssx_audio_system_take_boot_movie_flag,
        "SSXAudioSystem_TakeBootMovieFlag",
    ),
    (
        0x0028_b0f0,
        snd_bank_man_notify_queue_heads,
        "SndBankMan_NotifyQueueHeads",
    ),
    (0x0028_be08, snd_bank_man_link_banks, "SndBankMan_LinkBanks"),
    (
        0x0028_be38,
        snd_bank_man_unlink_bank,
        "SndBankMan_UnlinkBank",
    ),
];

/// `SSXAudioSystem_KillRiderTimers` (0x292ae8).
const RIDER_RELEASE_KILL_TIMERS_CALLEE: Callee = Callee {
    addr: 0x0029_2ae8,
    f: ee::s07::SSXAudioSystem_KillRiderTimers,
};

/// `SSXAudioSystem_OutputModeToOption` (0x289d60).
const OUTPUT_MODE_TO_OPTION_CALLEE: Callee = Callee {
    addr: 0x0028_9d60,
    f: ee::s07::SSXAudioSystem_OutputModeToOption,
};

/// `SndBank_Link` (0x28b6c8).
const SNDBANK_LINK_CALLEE: Callee = Callee {
    addr: 0x0028_b6c8,
    f: ee::s07::SndBank_Link,
};

/// `SndBank_Unlink` (0x28b730).
const SNDBANK_UNLINK_CALLEE: Callee = Callee {
    addr: 0x0028_b730,
    f: ee::s07::SndBank_Unlink,
};

/// `void SSXAudioSystem_RiderRelease(SSXAudioSystem* self)` (0x289af8):
/// stop the rider timers by forwarding to `SSXAudioSystem_KillRiderTimers`.
pub fn ssx_audio_system_rider_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        RIDER_RELEASE_KILL_TIMERS_CALLEE.call(c, 0x0028_9b08)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_GetOutputModeOption(SSXAudioSystem* self)` (0x289dc0):
/// the option index of the output mode stored at `self+0x62b0`.
pub fn ssx_audio_system_get_output_mode_option(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(crate::abi::arg(c, 0).wrapping_add(0x62b0)).lw(c);
        OUTPUT_MODE_TO_OPTION_CALLEE.call(c, 0x0028_9dd0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool SSXAudioSystem_TakeBootMovieFlag(SSXAudioSystem* self)` (0x289de0):
/// return the boot-movie flag at `self+0x6c88` and clear it.
pub fn ssx_audio_system_take_boot_movie_flag(c: &mut Cpu) {
    c.cycles += 3;
    let flag = crate::abi::arg(c, 0).wrapping_add(0x6c88);
    c.g[V0][0] = Addr(flag).lw(c);
    Addr(flag).sw(c, 0);
    ret(c);
}

/// `void SndBankMan_NotifyQueueHeads(SndBankMan* self, int queue)` (0x28b0f0):
/// pick the head entries of the two queues at `self+0x18c` and `self+0x1bc`
/// (or -1 when a queue has none) and hand the first to the function pointer
/// stored in the queue's dispatch table, with the chosen entry as argument.
pub fn snd_bank_man_notify_queue_heads(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        let bank = c.g[A3][0] as u32;
        c.g[T0][0] = sx(bank.wrapping_add(0x18c));
        c.g[V0][0] = Addr(bank.wrapping_add(0x1d4)).lw(c);
        c.g[A1][0] = Addr(bank.wrapping_add(0x194)).lw(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lh(c);
        c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = Addr(bank.wrapping_add(0x198)).lw(c);
        c.g[T1][0] = sx(bank.wrapping_add(c.g[V1][0] as u32));
        if c.g[A1][0] != c.g[A0][0] {
            c.cycles += 5;
            c.g[V0][0] = Addr((c.g[T0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[V1][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
        } else {
            c.cycles += 2;
            c.g[A1][0] = u64::MAX;
        }
        c.cycles += 4;
        c.g[V1][0] = Addr(bank.wrapping_add(0x1c8)).lw(c);
        c.g[V0][0] = Addr(bank.wrapping_add(0x1c4)).lw(c);
        let differ = c.g[V0][0] != c.g[V1][0];
        c.g[A0][0] = sx(bank.wrapping_add(0x1bc));
        if differ {
            c.cycles += 5;
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
        } else {
            c.cycles += 2;
            c.g[A3][0] = u64::MAX;
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(c.g[T2][0] as u32).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = c.g[T1][0];
        call_ptr(c, target, 0x0028_b174)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SndBankMan_LinkBanks(SndBankMan* self, int i, int j)` (0x28be08):
/// link bank `i` after bank `j` in the `0x60`-byte bank array at `self+0xacc`.
pub fn snd_bank_man_link_banks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x60;
        c.mult(0, V1, A1, V0);
        frame.save_ra(c, 0);
        let bank_array = Addr(crate::abi::arg(c, 0).wrapping_add(0xacc)).lw(c);
        c.g[A3][0] = bank_array;
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(bank_array as u32));
        c.mult(0, V1, A2, V0);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(bank_array as u32));
        SNDBANK_LINK_CALLEE.call(c, 0x0028_be2c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SndBankMan_UnlinkBank(SndBankMan* self, int i)` (0x28be38): unlink
/// bank `i` from the bank array at `self+0xacc`.
pub fn snd_bank_man_unlink_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x60;
        frame.save_ra(c, 0);
        c.mult(0, A1, A1, V0);
        let bank_array = Addr(crate::abi::arg(c, 0).wrapping_add(0xacc)).lw(c);
        c.g[A0][0] = sx((bank_array as u32).wrapping_add(c.g[A1][0] as u32));
        SNDBANK_UNLINK_CALLEE.call(c, 0x0028_be54)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{machine, vtable, FAKE};
    use crate::difftest::DATA;
    use crate::game::be::beprepoststate::tests::belib;

    #[test]
    fn ssx_audio_system_rider_release_matches() {
        check(
            ee::s07::f_00289af8__generated,
            ssx_audio_system_rider_release,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_9af8, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 7;
                c.bus.write32(DATA + 0x118, DATA + 0x100);
                c.bus.write32(DATA + 0x100, DATA + 0x200);
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(DATA + 0x200 + 0xab0, vt);
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_output_mode_option_matches() {
        check(
            ee::s07::f_00289dc0__generated,
            ssx_audio_system_get_output_mode_option,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x0028_9dc0, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x62b0, seed % 5);
            },
        );
    }

    #[test]
    fn ssx_audio_system_take_boot_movie_flag_matches() {
        check(
            ee::s07::f_00289de0__generated,
            ssx_audio_system_take_boot_movie_flag,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_9de0, 0);
                c.g[A0][0] = DATA as u64;
                let flag = if seed % 3 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(DATA + 0x6c88, flag);
            },
        );
    }

    #[test]
    fn snd_bank_man_notify_queue_heads_matches() {
        check(
            ee::s07::f_0028b0f0__generated,
            snd_bank_man_notify_queue_heads,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x0028_b0f0, 0);
                let bank = DATA + 0x1000;
                let queue = DATA + 0x2000;
                let table = DATA + 0x3000;
                let idx = seed % 4;
                c.g[A0][0] = bank as u64;
                c.g[A1][0] = (seed as u64).wrapping_mul(7);
                // The queue record: the entry offset and the call target.
                c.bus.write32(bank + 0x1d4, queue);
                c.bus.write16(queue + 0x18, 0x40);
                c.bus.write32(queue + 0x1c, FAKE);
                // First queue: the array base and index at +0x190/+0x198;
                // the head is `a1` (+0x194) against `a0` (+0x198).
                c.bus.write32(bank + 0x190, table);
                c.bus.write32(bank + 0x198, idx);
                c.bus
                    .write32(bank + 0x194, if seed % 2 == 0 { idx } else { idx + 1 });
                // Second queue: the same at +0x1c0/+0x1c8 and +0x1c4.
                c.bus.write32(bank + 0x1c0, table);
                c.bus.write32(bank + 0x1c8, idx);
                c.bus.write32(
                    bank + 0x1c4,
                    if (seed / 2) % 2 == 0 { idx } else { idx + 1 },
                );
                c.bus.write32(table + 4 * idx, 0x1234 + seed);
            },
        );
    }

    #[test]
    fn snd_bank_man_link_banks_matches() {
        check(
            ee::s07::f_0028be08__generated,
            snd_bank_man_link_banks,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_be08, 0);
                let bank = DATA + 0x1000;
                let array = DATA + 0x2000;
                c.g[A0][0] = bank as u64;
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = ((seed / 3) % 3) as u64;
                c.bus.write32(bank + 0xacc, array);
                for k in 0..4u32 {
                    let node = array + 0x60 * k;
                    let prev = if seed % 2 == 0 { 0 } else { DATA + 0x3000 };
                    c.bus.write32(node + 0x18, prev);
                    c.bus.write32(node + 0x1c, DATA + 0x3800);
                    let data = if seed % 3 == 0 { 0 } else { r.next_u32() };
                    c.bus.write32(node + 0x8, data);
                    c.bus.write32(node + 0x10, r.next_u32());
                    c.bus.write32(node + 0x14, r.next_u32());
                    c.bus.write32(node + 0xc, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn snd_bank_man_unlink_bank_matches() {
        check(
            ee::s07::f_0028be38__generated,
            snd_bank_man_unlink_bank,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x0028_be38, 0);
                let bank = DATA + 0x1000;
                let array = DATA + 0x2000;
                c.g[A0][0] = bank as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(bank + 0xacc, array);
                let node = array + 0x60 * (seed % 4);
                let (next, prev) = match seed % 3 {
                    0 => (0, DATA + 0x600),
                    1 => (DATA + 0x400, DATA + 0x400),
                    _ => (DATA + 0x400, DATA + 0x500),
                };
                c.bus.write32(node + 0x18, next);
                c.bus.write32(node + 0x1c, prev);
            },
        );
    }
}
