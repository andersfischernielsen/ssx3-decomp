//! Staging module oc1045 (wave wave264): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1045`.

#![allow(unused_imports)]

use crate::abi;
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
        0x002a_b0f0,
        stream_array_set_stream_volume,
        "StreamArray_SetStreamVolume",
    ),
    (
        0x002a_b1c8,
        stream_array_set_stream_param,
        "StreamArray_SetStreamParam",
    ),
    (
        0x002a_bd38,
        sound_request_set_bank_sample_2,
        "SoundRequest_SetBankSample_2",
    ),
    (
        0x002a_cac8,
        bank_monitor_instance_pause,
        "BankMonitorInstance_Pause",
    ),
    (0x002a_ce40, bank_monitor_dtor, "BankMonitor_dtor"),
    (
        0x002a_d358,
        bank_monitor_fade_out_handle,
        "BankMonitor_FadeOutHandle",
    ),
    (
        0x002a_d730,
        bank_monitor_count_playing,
        "BankMonitor_CountPlaying",
    ),
    (
        0x002a_d7a8,
        bank_monitor_count_active,
        "BankMonitor_CountActive",
    ),
];

/// `void StreamArray_SetStreamVolume(StreamArray* a, int idx, int vol)`
/// (0x2ab0f0): the stream in slot `idx` of the table at `a+0x8` gets `vol`
/// as its base volume. An empty slot does nothing.
pub fn stream_array_set_stream_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x8).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        frame.restore_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x002a_b118);
            c.g[A1][0] = c.g[A2][0];
            Callee {
                addr: 0x002a_9f30,
                f: ee::s08::f_002a9f30,
            }
            .call(c, 0x002a_b118)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int StreamArray_SetStreamParam(StreamArray* a, int idx, int arg)`
/// (0x2ab1c8): as `StreamArray_SetStreamVolume`, but the stream's parameter
/// setter runs and its result is returned; -1 for an empty slot.
pub fn stream_array_set_stream_param(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x8).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = u64::MAX;
        if !empty {
            c.cycles += 2;
            c.g[RA][0] = sx(0x002a_b1f0);
            c.g[A1][0] = c.g[A2][0];
            Callee {
                addr: 0x002a_a448,
                f: ee::s08::f_002aa448,
            }
            .call(c, 0x002a_b1f0)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SoundRequest_SetBankSample_2(SoundRequest* r, int bank, int sample)`
/// (0x2abd38): the bank goes to +0xc and the sample to +0x10 (the same
/// stores as `SoundRequest_SetBankSample`, which the Ghidra pseudo-C shows).
pub fn sound_request_set_bank_sample_2(c: &mut Cpu) {
    c.cycles += 3;
    let r = abi::arg(c, 0);
    let sample = c.g[A2][0];
    Addr(r.wrapping_add(0x10)).sw(c, sample);
    let bank = c.g[A1][0];
    Addr(r.wrapping_add(0xc)).sw(c, bank);
    abi::ret(c);
}

/// `void BankMonitorInstance_Pause(BankMonitorInstance* self)` (0x2acac8):
/// unless the instance is free or already fading, reset its pitch and
/// volume and, for a type-2 instance, pause its sequence handle.
pub fn bank_monitor_instance_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x002a_cadc);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002a_bb80,
            f: ee::s08::f_002abb80,
        }
        .call(c, 0x002a_cadc)?;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x54).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 10;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x002a_cafc);
                c.g[A1][0] = 0;
                Callee {
                    addr: 0x002a_be78,
                    f: ee::s08::f_002abe78,
                }
                .call(c, 0x002a_cafc)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x002a_cb08);
                c.g[A1][0] = 0;
                Callee {
                    addr: 0x002a_bf60,
                    f: ee::s08::f_002abf60,
                }
                .call(c, 0x002a_cb08)?;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V0][0] = 2;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x002a_cb20);
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
                    call_ptr(c, 0x003b_9d00, 0x002a_cb20)?;
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitor_dtor(BankMonitor* self, int flags)` (0x2ace40): with a
/// non-zero flag the monitor's instance table (`self+0x8ec`, count in the
/// word before it) is destroyed from the last instance down to the first,
/// then the base and the sound manager reference, when `flags` asks for them.
/// With a zero flag the monitor's vtable slot is set and the sound
/// manager's table is cleared instead.
pub fn bank_monitor_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S1, 0x50);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3b48));
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[A0][0] = 0x0048_0000;
        frame.save(c, S3, 0x30);
        c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3b88));
        frame.save_ra(c, 0x20);
        c.g[T0][0] = c.g[A1][0];
        let this = c.g[S1][0] as u32;
        let stamp = c.g[V0][0];
        Addr(this.wrapping_add(0x8d8)).sw(c, stamp);
        c.g[V1][0] = Addr(this).lw(c);
        let flags_zero = c.g[T0][0] == 0;
        let obj = c.g[V1][0] as u32;
        let a3 = c.g[A3][0];
        Addr(obj.wrapping_add(4)).sw(c, a3);
        if flags_zero {
            c.cycles += 16;
            c.g[A2][0] = Addr(c.g[A0][0] as u32).off(0x3b88).ld(c);
            c.g[V0][0] = Addr(c.g[A3][0] as u32).off(8).ld(c);
            c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x10).ld(c);
            c.g[A0][0] = Addr(c.g[A3][0] as u32).off(0x18).ld(c);
            c.g[A1][0] = Addr(this).lw(c);
            let (v0_val, v1_val, a0_val, a2_val) = (c.g[V0][0], c.g[V1][0], c.g[A0][0], c.g[A2][0]);
            Addr(frame.at(c, 8)).sd(c, v0_val);
            Addr(frame.at(c, 0x10)).sd(c, v1_val);
            Addr(frame.at(c, 0)).sd(c, a2_val);
            Addr(frame.at(c, 0x18)).sd(c, a0_val);
            let sp_now = c.g[SP][0];
            Addr((c.g[A1][0] as u32).wrapping_add(4)).sw(c, sp_now);
            c.g[V1][0] = Addr(c.g[A3][0] as u32).off(8).lhu(c);
            c.g[V0][0] = Addr(this).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8f0));
            c.g[V0][0] = sx(this.wrapping_sub(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let half = c.g[V1][0];
            Addr(frame.at(c, 8)).sh(c, half);
        }
        // 'b_2acf14: the instance table.
        c.cycles += 4;
        c.g[A1][0] = Addr(this.wrapping_add(0x8ec)).lw(c);
        c.g[S3][0] = c.g[T0][0] & 0x1;
        let no_table = c.g[A1][0] == 0;
        c.g[S2][0] = c.g[T0][0] & 0x2;
        if !no_table {
            c.cycles += 6;
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_sub(0x10)).lw(c);
            c.g[V0][0] = 0xc0;
            // a0 = v1 * v0 (the instance count times the instance size)
            c.mult(0, A0, V1, V0);
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = Addr(this.wrapping_add(0x8ec)).lw(c);
            } else {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc0));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0;
                    c.g[RA][0] = sx(0x002a_cefc);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x002a_ba18,
                        f: ee::s08::f_002aba18,
                    }
                    .call(c, 0x002a_cefc)?;
                    c.g[V0][0] = Addr(this.wrapping_add(0x8ec)).lw(c);
                    if c.g[V0][0] != c.g[S0][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc0));
                        if event_due(c) {
                            deliver_event_here(c, 0x002a_cef0);
                        }
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = Addr(this.wrapping_add(0x8ec)).lw(c);
                        break;
                    }
                }
            }
            c.cycles += 2;
            c.g[RA][0] = sx(0x002a_cf14);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x002a_cf14)?;
        }
        // 'b_2acf34: the base class.
        c.cycles += 5;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x002a_cf20);
        c.g[A1][0] = 0;
        Callee {
            addr: 0x002a_dac8,
            f: ee::s08::f_002adac8,
        }
        .call(c, 0x002a_cf20)?;
        if c.g[S2][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(this).lw(c);
            c.g[RA][0] = sx(0x002a_cf34);
            c.g[A1][0] = 0;
            Callee {
                addr: 0x002a_dee8,
                f: ee::s08::f_002adee8,
            }
            .call(c, 0x002a_cf34)?;
        }
        // 'b_2acf48: the optional table call.
        c.cycles += 2;
        frame.restore(c, S0, 0x60);
        if c.g[S3][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x002a_cf44);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x002a_cf44)?;
            frame.restore(c, S0, 0x60);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitor_FadeOutHandle(BankMonitor* self, float secs)`
/// (0x2ad358): find the instance playing the handle; if there is one, fade
/// its volume out over `secs` (the same float, kept in f20).
pub fn bank_monitor_fade_out_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_fpr(c, 20, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x002a_d36c);
        c.mov_s(20, 12);
        Callee {
            addr: 0x002a_d550,
            f: ee::s08::f_002ad550,
        }
        .call(c, 0x002a_d36c)?;
        let missing = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[V0][0];
        if !missing {
            c.cycles += 4;
            c.mov_s(12, 20);
            c.g[A1][0] = u64::MAX;
            c.g[RA][0] = sx(0x002a_d384);
            c.g[A2][0] = 0;
            Callee {
                addr: 0x002a_cb80,
                f: ee::s08::f_002acb80,
            }
            .call(c, 0x002a_d384)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int BankMonitor_CountPlaying(BankMonitor* self)` (0x2ad730): the number
/// of instances (of the 64 slots) whose handle at +0x8 is set and whose
/// `BankMonitorInstance_PollFree` is reached busy.
pub fn bank_monitor_count_playing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = 0x3f;
        loop {
            c.cycles += 5;
            c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x8ec)).lw(c);
            c.g[RA][0] = sx(0x002a_d764);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            Callee {
                addr: 0x002a_bb80,
                f: ee::s08::f_002abb80,
            }
            .call(c, 0x002a_d764)?;
            let busy = c.g[V0][0] != 0;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            if !busy {
                c.cycles += 5;
                c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x8ec)).lw(c);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lw(c);
                if c.g[V1][0] != 0 {
                    c.g[S1][0] = c.g[A0][0];
                }
            }
            c.cycles += 2;
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc0));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_d758);
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BankMonitor_CountActive(BankMonitor* self)` (0x2ad7a8): the number
/// of the 64 instance slots whose `BankMonitorInstance_PollFree` returns 0
/// (free).
pub fn bank_monitor_count_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0x3f;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        loop {
            c.cycles += 8;
            c.g[A0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x8ec)).lw(c);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            c.g[RA][0] = sx(0x002a_d7e0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            Callee {
                addr: 0x002a_bb80,
                f: ee::s08::f_002abb80,
            }
            .call(c, 0x002a_d7e0)?;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc0));
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            let more = (c.g[S1][0] as i64) >= 0;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = c.g[V1][0];
            }
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_d7d0);
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, fill, scramble, Rng, DATA};
    use ssx3_ee as ee;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word magic of the fakes below (written into the machine's RAM).
    const OC1045_MAGIC: u32 = 0x1045_c0de;
    /// Fake k keeps its gate, call counter and forced results in its own
    /// block of machine RAM (never in statics).
    const OC1045_BASE: u32 = DATA + 0x20_0000;

    fn oc1045_gate(k: u32) -> u32 {
        OC1045_BASE + k * 0x1000
    }

    /// Arm fake `k`: gate on, counter zero, forced results for its first calls.
    fn oc1045_arm(c: &mut Cpu, k: u32, results: &[u32]) {
        let g = oc1045_gate(k);
        c.bus.write32(g, OC1045_MAGIC);
        c.bus.write32(g + 4, 0);
        for (i, v) in results.iter().enumerate() {
            c.bus.write32(g + 0x100 + 4 * (i as u32), *v);
        }
    }

    /// Take fake `k`'s next call number and its forced result (0 after the
    /// armed results run out).
    fn oc1045_next(c: &mut Cpu, k: u32) -> u32 {
        let g = oc1045_gate(k);
        let n = c.bus.read32(g + 4);
        c.bus.write32(g + 4, n + 1);
        if n < 16 {
            c.bus.read32(g + 0x100 + 4 * n)
        } else {
            0
        }
    }

    /// A callee fake for an override slot. Gate off: the body the slot held
    /// when the fake was hooked. Gate on: a forced result in v0, return to ra.
    macro_rules! oc1045_fake {
        ($fake:ident, $prev:ident, $k:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(oc1045_gate($k)) != OC1045_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let v = oc1045_next(c, $k);
                c.g[V0][0] = sx(v);
                c.cycles += 4;
                c.pc = c.g[RA][0] as u32;
            }
        };
    }

    oc1045_fake!(
        oc1045_fake_stream_param,
        OC1045_PREV_STREAM_PARAM,
        0,
        ee::s08::f_002aa448__generated
    );
    oc1045_fake!(
        oc1045_fake_poll,
        OC1045_PREV_POLL,
        1,
        ee::s08::f_002abb80__generated
    );
    oc1045_fake!(
        oc1045_fake_pitch,
        OC1045_PREV_PITCH,
        2,
        ee::s08::f_002abe78__generated
    );
    oc1045_fake!(
        oc1045_fake_volume,
        OC1045_PREV_VOLUME,
        3,
        ee::s08::f_002abf60__generated
    );
    oc1045_fake!(
        oc1045_fake_seqpause,
        OC1045_PREV_SEQPAUSE,
        4,
        ee::s13::f_003b9d00__generated
    );
    oc1045_fake!(
        oc1045_fake_inst_dtor,
        OC1045_PREV_INST_DTOR,
        5,
        ee::s08::f_002aba18__generated
    );
    oc1045_fake!(
        oc1045_fake_base_dtor,
        OC1045_PREV_BASE_DTOR,
        6,
        ee::s08::f_002adac8__generated
    );
    oc1045_fake!(
        oc1045_fake_sm_dtor,
        OC1045_PREV_SM_DTOR,
        7,
        ee::s08::f_002adee8__generated
    );
    oc1045_fake!(
        oc1045_fake_table_a,
        OC1045_PREV_TABLE_A,
        8,
        ee::s10::f_00317e98__generated
    );
    oc1045_fake!(
        oc1045_fake_table_b,
        OC1045_PREV_TABLE_B,
        9,
        ee::s10::f_00317e50__generated
    );
    oc1045_fake!(
        oc1045_fake_find,
        OC1045_PREV_FIND,
        10,
        ee::s08::f_002ad550__generated
    );
    oc1045_fake!(
        oc1045_fake_fade,
        OC1045_PREV_FADE,
        11,
        ee::s08::f_002acb80__generated
    );

    /// Hook every fake once per process (gates are off unless a test arms them).
    fn oc1045_hook_all() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AA448),
            oc1045_fake_stream_param,
            &OC1045_PREV_STREAM_PARAM,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ABB80),
            oc1045_fake_poll,
            &OC1045_PREV_POLL,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ABE78),
            oc1045_fake_pitch,
            &OC1045_PREV_PITCH,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ABF60),
            oc1045_fake_volume,
            &OC1045_PREV_VOLUME,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B9D00),
            oc1045_fake_seqpause,
            &OC1045_PREV_SEQPAUSE,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ABA18),
            oc1045_fake_inst_dtor,
            &OC1045_PREV_INST_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADAC8),
            oc1045_fake_base_dtor,
            &OC1045_PREV_BASE_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADEE8),
            oc1045_fake_sm_dtor,
            &OC1045_PREV_SM_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1045_fake_table_a,
            &OC1045_PREV_TABLE_A,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1045_fake_table_b,
            &OC1045_PREV_TABLE_B,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AD550),
            oc1045_fake_find,
            &OC1045_PREV_FIND,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ACB80),
            oc1045_fake_fade,
            &OC1045_PREV_FADE,
        );
    }

    #[test]
    fn stream_array_set_stream_volume_matches() {
        check(
            ssx3_ee::s08::f_002ab0f0__generated,
            stream_array_set_stream_volume,
            300,
            &|c, seed| {
                let mut r = Rng(0x2ab0_f0 ^ seed as u64);
                scramble(c, &mut r);
                let array = DATA;
                let table = DATA + 0x100;
                c.g[A0][0] = array as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.bus.write32(array + 8, table);
                for i in 0..4u32 {
                    let slot = if (seed >> i) & 1 == 0 {
                        0
                    } else {
                        DATA + 0x300 + i * 0x20
                    };
                    c.bus.write32(table + 4 * i, slot);
                }
                fill(c, &mut r, DATA + 0x300, 0x100);
            },
        );
    }

    #[test]
    fn stream_array_set_stream_param_matches() {
        oc1045_hook_all();
        check(
            ssx3_ee::s08::f_002ab1c8__generated,
            stream_array_set_stream_param,
            300,
            &|c, seed| {
                let mut r = Rng(0x2ab1_c8 ^ seed as u64);
                scramble(c, &mut r);
                let array = DATA;
                let table = DATA + 0x100;
                c.g[A0][0] = array as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.bus.write32(array + 8, table);
                for i in 0..4u32 {
                    let slot = if (seed >> i) & 1 == 0 {
                        0
                    } else {
                        DATA + 0x300 + i * 0x20
                    };
                    c.bus.write32(table + 4 * i, slot);
                }
                fill(c, &mut r, DATA + 0x300, 0x100);
                oc1045_arm(c, 0, &[r.next_u32()]);
            },
        );
    }

    #[test]
    fn sound_request_set_bank_sample_2_matches() {
        check(
            ssx3_ee::s08::f_002abd38__generated,
            sound_request_set_bank_sample_2,
            30,
            &|c, seed| {
                let mut r = Rng(0x2abd_38 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn bank_monitor_instance_pause_matches() {
        oc1045_hook_all();
        check(
            ssx3_ee::s08::f_002acac8__generated,
            bank_monitor_instance_pause,
            300,
            &|c, seed| {
                let mut r = Rng(0x2aca_c8 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this, if seed % 5 == 0 { 2 } else { 1 });
                c.bus.write32(this + 0x54, (seed % 4 == 1) as u32);
                c.bus.write32(this + 4, DATA + 0x2000);
                oc1045_arm(c, 1, &[(seed % 3 == 0) as u32]);
                oc1045_arm(c, 2, &[]);
                oc1045_arm(c, 3, &[]);
                oc1045_arm(c, 4, &[]);
            },
        );
    }

    #[test]
    fn bank_monitor_dtor_matches() {
        oc1045_hook_all();
        check(
            ssx3_ee::s08::f_002ace40__generated,
            bank_monitor_dtor,
            300,
            &|c, seed| {
                let mut r = Rng(0x2ace_40 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA + 0x1000;
                let obj = DATA + 0x2000;
                let arr = DATA + 0x3000;
                c.g[A0][0] = this as u64;
                // The entry a1 is the flags: bit 0 and bit 1 pick the
                // sound manager and the base-class steps; zero takes the
                // copy path of the monitor's vtable.
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(this, obj);
                c.bus
                    .write32(this + 0x8ec, if seed % 7 == 0 { 0 } else { arr });
                // Count 1 is the no-loop case; 2 and 3 loop (a count of 0
                // never ends, in the original as in the port).
                c.bus.write32(arr - 0x10, 1 + seed % 3);
                for k in 5..=9 {
                    oc1045_arm(c, k, &[]);
                }
            },
        );
    }

    #[test]
    fn bank_monitor_fade_out_handle_matches() {
        oc1045_hook_all();
        check(
            ssx3_ee::s08::f_002ad358__generated,
            bank_monitor_fade_out_handle,
            60,
            &|c, seed| {
                let mut r = Rng(0x2ad3_58 ^ seed as u64);
                scramble(c, &mut r);
                c.f[12] = r.next_u32();
                c.g[A0][0] = (DATA + 0x1000) as u64;
                let found = if seed % 3 == 0 { 0 } else { DATA + 0x4000 };
                oc1045_arm(c, 10, &[found]);
                oc1045_arm(c, 11, &[]);
            },
        );
    }

    #[test]
    fn bank_monitor_count_playing_matches() {
        oc1045_hook_all();
        check(
            ssx3_ee::s08::f_002ad730__generated,
            bank_monitor_count_playing,
            60,
            &|c, seed| {
                let mut r = Rng(0x2ad7_30 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA + 0x1000;
                let arr = DATA + 0x3000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x8ec, arr);
                fill(c, &mut r, arr, 0x800);
                let results: Vec<u32> = (0..16).map(|_| (r.next_u32() % 3) & 1).collect();
                oc1045_arm(c, 1, &results);
                let _ = seed;
            },
        );
    }

    #[test]
    fn bank_monitor_count_active_matches() {
        oc1045_hook_all();
        check(
            ssx3_ee::s08::f_002ad7a8__generated,
            bank_monitor_count_active,
            60,
            &|c, seed| {
                let mut r = Rng(0x2ad7_a8 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA + 0x1000;
                let arr = DATA + 0x3000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x8ec, arr);
                let results: Vec<u32> = (0..16).map(|_| (r.next_u32() % 3) & 1).collect();
                oc1045_arm(c, 1, &results);
                let _ = seed;
            },
        );
    }
}
