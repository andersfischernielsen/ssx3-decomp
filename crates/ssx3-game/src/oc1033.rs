//! Staging module oc1033 (wave wave263): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1033`.

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
        0x0029_9638,
        ssxaudio_on_boost_meter_message,
        "SSXAudioSystem_OnBoostMeterMessage",
    ),
    (
        0x0029_d6d0,
        ssxaudio_set_mission_menu_flag,
        "SSXAudioSystem_SetMissionMenuFlag",
    ),
    (
        0x0029_e438,
        ssxaudio_calc_avalanche_volume,
        "SSXAudioSystem_CalcAvalancheVolume",
    ),
    (
        0x0029_e4a0,
        ssxaudio_avalanche_sound_callback,
        "SSXAudioSystem_AvalancheSoundCallback",
    ),
    (
        0x0029_e560,
        ssxaudio_avalanche_update_stub,
        "SSXAudioSystem_AvalancheUpdateStub",
    ),
    (
        0x002a_1138,
        ssxaudio_rider_say_20d3,
        "SSXAudioSystem_RiderSay20D3",
    ),
    (
        0x002a_1280,
        ssxaudio_rider_say_20d4,
        "SSXAudioSystem_RiderSay20D4",
    ),
    (
        0x002a_1400,
        ssxaudio_rider_say_20b9,
        "SSXAudioSystem_RiderSay20B9",
    ),
];

/// `void SSXAudioSystem_SetMissionMenuFlag(SSXAudioSystem* s)` (0x29d6d0):
/// sets the mission-menu word at `s+0x5fd8` to 1.
pub fn ssxaudio_set_mission_menu_flag(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x1;
    let flag = (c.g[A0][0] as u32).wrapping_add(0x5fd8);
    c.sw(flag, c.g[V0][0]);
    ret(c);
}

/// `void SSXAudioSystem_AvalancheUpdateStub(void)` (0x29e560): an empty
/// per-frame hook; it only returns.
pub fn ssxaudio_avalanche_update_stub(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `int SSXAudioSystem_CalcAvalancheVolume(void* self, Avalanche* a)`
/// (0x29e438): the avalanche sound volume from the two float fields at
/// `a+0x10` and `a+0x14` and three gp-relative constants, clamped to 0..127.
pub fn ssxaudio_calc_avalanche_volume(c: &mut Cpu) {
    c.cycles += 26;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x4588));
    c.g[A2][0] = 0x7f;
    c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x14));
    c.g[A0][0] = u64::MAX;
    c.g[AT][0] = 0x42c8_0000; // 100.0f
    c.mtc1(AT, 1);
    c.mul_s(2, 2, 0);
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4584));
    c.lwc1(4, (c.g[A1][0] as u32).wrapping_add(0x10));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x4580));
    c.sub_s(1, 1, 2);
    c.mul_s(1, 1, 3);
    c.mul_s(1, 1, 4);
    c.mul_s(0, 1, 0);
    c.cvt_w_s(1, 0);
    c.mfc1(V0, 1);
    // Clamp to 0..127: above 127 gives 127, not above -1 gives 0.
    c.g[V1][0] = ((c.g[V0][0] as i64) < 128i64) as u64;
    if c.g[V1][0] == 0 {
        c.g[V0][0] = c.g[A2][0];
    }
    c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[A0][0] == 0 {
        c.g[V0][0] = 0u64;
    }
    c.mtc1(V0, 1);
    c.cvt_s_w(1, 1);
    c.cvt_w_s(0, 1);
    c.mfc1(V0, 0);
    ret(c);
}

/// `void SSXAudioSystem_AvalancheSoundCallback(void* self, Avalanche* a)`
/// (0x29e4a0): the avalanche's sound update. When the race-manager count
/// (`self+0x6070`) is below the count at `race+0x78`, it looks up the sound
/// record at `race + 4 * count` and plays it through its table, then stores
/// the volume the calc returns at `a+0x64`.
pub fn ssxaudio_avalanche_sound_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x30);
        c.g[S2][0] = c.g[A1][0];
        call_ptr(c, 0x0028_b1d8, 0x0029_e4c0)?; // GET_RACE_MAN
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x78)).lw(c);
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x6070)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let nothing_due = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x60);
        if !nothing_due {
            c.cycles += 30;
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6050));
            call_ptr(c, 0x0028_b1d8, 0x0029_e4dc)?;
            c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x6070)).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
            c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x6c0)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2c)).lw(c);
            let play = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, play, 0x0029_e504)?;
            // Copy the record's three floats (+0, +4, +8) into a 3-float
            // vector on the stack, then pass it to the volume calc.
            c.lwc1(1, c.g[V0][0] as u32);
            c.g[A0][0] = c.g[SP][0];
            let (x_slot, y_slot, z_slot) =
                (frame.at(c, 0x20), frame.at(c, 0x24), frame.at(c, 0x28));
            c.g[A1][0] = sx(x_slot);
            c.swc1(1, x_slot);
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
            c.swc1(0, y_slot);
            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
            c.swc1(1, z_slot);
            call_ptr(c, 0x002d_a1c0, 0x0029_e528)?;
            frame.restore(c, V1, 0x0);
            c.g[A0][0] = c.g[S1][0];
            frame.restore(c, V0, 0x10);
            c.g[A1][0] = c.g[S0][0];
            c.sq(V1, (c.g[S1][0] as u32).wrapping_add(0x6050));
            c.sq(V0, (c.g[S1][0] as u32).wrapping_add(0x6060));
            call_ptr(c, 0x0029_e438, 0x0029_e544)?;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x64), c.g[V0][0]);
            frame.restore(c, S0, 0x60);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_OnBoostMeterMessage(SSXAudioSystem* s, Rider* r)`
/// (0x299638): when the boost meter fires for a rider, starts the boost
/// timers and plays the boost say-line (0x8, 0x4, 0x10 or 0x9 variants) unless
/// the rider's boost is already active or the speech is off.
pub fn ssxaudio_on_boost_meter_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        call_ptr(c, 0x0028_8ae0, 0x0029_965c)?;
        let busy = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x40);
        'b_29979c: {
            if busy {
                break 'b_29979c;
            }
            'b_299678: {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x874)).lw(c);
                let zero = c.g[V0][0] == 0;
                c.g[V1][0] = 0;
                if zero {
                    break 'b_299678;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x87c)).lw(c);
                c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
            }
            c.cycles += 2;
            let idle = c.g[V1][0] == 0;
            frame.restore(c, S0, 0x40);
            if idle {
                break 'b_29979c;
            }
            'b_29969c: {
                c.cycles += 6;
                c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x470));
                c.mtc1(ZERO, 0);
                c.c_le_s(0, 1);
                let t = c.fcc();
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_29969c;
                }
                c.cycles += 1;
                c.g[V0][0] = 0u64;
            }
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x40);
                break 'b_29979c;
            }
            'b_299744: {
                'b_299718: {
                    c.cycles += 23;
                    c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x118)).lw(c);
                    c.g[S1][0] = 0x4a_0000;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3678));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[A1][0] = 0x9;
                    c.g[A2][0] = 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                    call_ptr(c, 0x0028_7968, 0x0029_96c8)?; // GET_CHANNEL_GAIN_PTR
                    c.ldl(A2, (c.g[S1][0] as u32).wrapping_add(0x7));
                    c.ldr(A2, c.g[S1][0] as u32);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A3][0] = c.g[V0][0];
                    c.g[A1][0] = 0x190;
                    c.g[T0][0] = 0;
                    c.g[T1][0] = 0x6b;
                    c.g[T2][0] = 0x7f;
                    c.g[T3][0] = 0;
                    call_ptr(c, 0x002a_dca0, 0x0029_96f0)?; // BankMonitor_AddTimer
                    c.g[V1][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x2f4)).lw(c);
                    c.g[V0][0] = 0xa;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 5i64) as u64;
                    if t {
                        break 'b_299718;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A2][0] = 0x10;
                    call_ptr(c, 0x002a_3c00, 0x0029_9710)?; // ArcadeSay20A8
                    c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x118)).lw(c);
                    break 'b_299744;
                }
                'b_299734: {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        break 'b_299734;
                    }
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A2][0] = 0x8;
                    call_ptr(c, 0x002a_3c00, 0x0029_972c)?;
                    c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x118)).lw(c);
                    break 'b_299744;
                }
                c.cycles += 4;
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = 0x4;
                call_ptr(c, 0x002a_3c00, 0x0029_9740)?;
                c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x118)).lw(c);
            }
            c.cycles += 10;
            c.g[V0][0] = 0x4a_0000;
            c.ldl(S0, (c.g[V0][0] as u32).wrapping_add(0x36af));
            c.ldr(S0, (c.g[V0][0] as u32).wrapping_add(0x36a8));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
            call_ptr(c, 0x002a_dde0, 0x0029_9760)?; // BankMonitor_KillTimersByCallback
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x59e4)).lw(c);
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x40);
                break 'b_29979c;
            }
            c.cycles += 12;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x118)).lw(c);
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[S3][0];
            c.g[A1][0] = 0xbb8;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[T0][0] = 0;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
            c.g[T3][0] = 0;
            call_ptr(c, 0x002a_dca0, 0x0029_9798)?; // BankMonitor_AddTimer
            frame.restore(c, S0, 0x40);
            break 'b_29979c;
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_RiderSay20D3(SSXAudioSystem* s, Rider* r, int a2)`
/// (0x2a1138): the 0x20d3 speech line for a rider. Does nothing when speech
/// volume is off or the audio is busy; otherwise sets the event context and
/// the speech and voice masks, then plays the line through its table.
pub fn ssxaudio_rider_say_20d3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A2][0];
        call_ptr(c, 0x0029_f0a0, 0x002a_1160)?; // SSXAudioSystem_IsSpeechVolumeOn
        let speech_off = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x40);
        'b_2a1264: {
            if speech_off {
                break 'b_2a1264;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0028_8ae0, 0x002a_1170)?;
            let busy = c.g[V0][0] != 0;
            frame.restore(c, S0, 0x40);
            if busy {
                break 'b_2a1264;
            }
            'b_2a11a0: {
                c.cycles += 7;
                call_ptr(c, 0x0014_dc80, 0x002a_1180)?; // BELibrary GetInstance
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0014_dd58, 0x002a_118c)?; // GetInterface(be, 0)
                if c.g[S1][0] != 0 {
                    break 'b_2a11a0;
                }
                c.cycles += 3;
                call_ptr(c, 0x0028_b1d8, 0x002a_119c)?; // GET_RACE_MAN
                c.g[S1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x2c)).lw(c);
            }
            'b_2a11b8: {
                c.cycles += 2;
                if c.g[S3][0] != 0 {
                    c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x6c0)).lw(c);
                    break 'b_2a11b8;
                }
                c.cycles += 4;
                call_ptr(c, 0x0028_b1d8, 0x002a_11b0)?;
                c.g[S3][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x6c0)).lw(c);
            }
            c.cycles += 11;
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2c)).lw(c);
            let play = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, play, 0x002a_11cc)?;
            c.lwc1(12, (c.g[GP][0] as u32).wrapping_add(0x5e8));
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0028_8b40, 0x002a_11dc)?;
            let negative = (c.g[V0][0] as i64) < 0;
            c.g[A0][0] = c.g[S1][0];
            if !negative {
                c.cycles += 13;
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5560));
                call_ptr(c, 0x0012_3128, 0x002a_11ec)?;
                c.mtc1(ZERO, 12);
                c.g[A0][0] = c.g[S0][0];
                c.g[T0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20d3;
                c.g[A3][0] = c.g[S1][0];
                c.g[T1][0] = 0;
                c.g[T2][0] = 0;
                call_ptr(c, 0x002b_1458, 0x002a_1210)?; // SpeechSystem_SetEventContext
                let no_event = c.g[V0][0] == 0;
                frame.restore(c, S0, 0x40);
                if no_event {
                    break 'b_2a1264;
                }
                c.cycles += 18;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0029_f330, 0x002a_1220)?; // GetRiderSpeechMask
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x002a_1d48, 0x002a_1230)?; // GetRiderVoiceMask
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 0x1;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20d3;
                call_ptr(c, 0x003d_8008, 0x002a_1244)?; // SPCH_MakeId
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x173c)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = c.g[S0][0];
                c.g[A1][0] = 0x3;
                c.g[T0][0] = 0x4;
                let line = c.g[V1][0] as u32;
                call_ptr(c, line, 0x002a_1260)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_RiderSay20D4(SSXAudioSystem* s, Rider* r, int a2)`
/// (0x2a1280): the 0x20d4 speech line; the same shape as 20D3 without the
/// BELibrary lookup, and a different line id.
pub fn ssxaudio_rider_say_20d4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A2][0];
        call_ptr(c, 0x0029_f0a0, 0x002a_12a8)?; // SSXAudioSystem_IsSpeechVolumeOn
        let speech_off = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x40);
        'b_2a136c: {
            if speech_off {
                break 'b_2a136c;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0028_8ae0, 0x002a_12b8)?;
            let busy = c.g[V0][0] != 0;
            frame.restore(c, S0, 0x40);
            if busy {
                break 'b_2a136c;
            }
            'b_2a1368: {
                c.cycles += 12;
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x6c0)).lw(c);
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2c)).lw(c);
                let play = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, play, 0x002a_12d8)?;
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_add(0x5e8));
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0028_8b40, 0x002a_12e8)?;
                let negative = (c.g[V0][0] as i64) < 0;
                c.g[A0][0] = c.g[S1][0];
                if negative {
                    break 'b_2a1368;
                }
                c.cycles += 13;
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5560));
                call_ptr(c, 0x0012_3128, 0x002a_12f8)?;
                c.mtc1(ZERO, 12);
                c.g[A0][0] = c.g[S0][0];
                c.g[T0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20d4;
                c.g[A3][0] = c.g[S1][0];
                c.g[T1][0] = 0;
                c.g[T2][0] = 0;
                call_ptr(c, 0x002b_1458, 0x002a_131c)?;
                let no_event = c.g[V0][0] == 0;
                frame.restore(c, S0, 0x40);
                if no_event {
                    break 'b_2a136c;
                }
                c.cycles += 17;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0029_f330, 0x002a_132c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x002a_1d48, 0x002a_133c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 0x1;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20d4;
                call_ptr(c, 0x003d_8008, 0x002a_1350)?;
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x173c)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = c.g[S0][0];
                let line = c.g[V1][0] as u32;
                c.g[A1][0] = 0x2;
                call_ptr(c, line, 0x002a_1368)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_RiderSay20B9(SSXAudioSystem* s, Rider* r)` (0x2a1400):
/// the 0x20b9 speech line. Its voice and finish masks feed the line, and the
/// line id is 2 or 4 depending on the race type (`IsRaceType4or5`, the
/// `0x530000` language byte).
pub fn ssxaudio_rider_say_20b9(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0029_f0a0, 0x002a_1424)?; // SSXAudioSystem_IsSpeechVolumeOn
        let speech_off = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x40);
        'b_2a1544: {
            if speech_off {
                break 'b_2a1544;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0028_8ae0, 0x002a_1434)?;
            let busy = c.g[V0][0] != 0;
            frame.restore(c, S0, 0x40);
            if busy {
                break 'b_2a1544;
            }
            'b_2a1540: {
                c.cycles += 12;
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x6c0)).lw(c);
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2c)).lw(c);
                let play = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, play, 0x002a_1454)?;
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_add(0x5e8));
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0028_8b40, 0x002a_1464)?;
                let negative = (c.g[V0][0] as i64) < 0;
                c.g[A0][0] = c.g[S1][0];
                if negative {
                    break 'b_2a1540;
                }
                c.cycles += 13;
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5560));
                call_ptr(c, 0x0012_3128, 0x002a_1474)?;
                c.mtc1(ZERO, 12);
                c.g[A0][0] = c.g[S0][0];
                c.g[T0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20b9;
                c.g[A3][0] = c.g[S1][0];
                c.g[T1][0] = 0;
                c.g[T2][0] = 0;
                call_ptr(c, 0x002b_1458, 0x002a_1498)?;
                let no_event = c.g[V0][0] == 0;
                frame.restore(c, S0, 0x40);
                if no_event {
                    break 'b_2a1544;
                }
                'b_2a1518: {
                    'b_2a14e0: {
                        c.cycles += 14;
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x0029_f330, 0x002a_14a8)?;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S2][0];
                        c.g[S3][0] = c.g[V0][0];
                        call_ptr(c, 0x002a_1e68, 0x002a_14b8)?; // GetFinishSpeechMask
                        c.g[S0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014_dc80, 0x002a_14c0)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0;
                        call_ptr(c, 0x0014_dd58, 0x002a_14cc)?;
                        c.g[V1][0] = 0x1;
                        if c.g[S0][0] != c.g[V1][0] {
                            break 'b_2a14e0;
                        }
                        c.cycles += 2;
                        c.g[S0][0] = 0x1;
                        break 'b_2a1518;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x002a_4248, 0x002a_14e8)?; // IsRaceType4or5
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = 0x2;
                        break 'b_2a1518;
                    }
                    c.cycles += 5;
                    c.g[V1][0] = 0x53_0000;
                    c.g[A0][0] = 0x2;
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c11)).lb(c);
                    if c.g[V0][0] == c.g[A0][0] {
                        c.g[S0][0] = 0x2;
                        break 'b_2a1518;
                    }
                    'b_2a1514: {
                        c.cycles += 2;
                        let same = c.g[S0][0] == c.g[A0][0];
                        c.g[V0][0] = 0x4;
                        if same {
                            break 'b_2a1514;
                        }
                        c.cycles += 2;
                        let differs = c.g[S0][0] != c.g[V0][0];
                        c.g[S0][0] = 0x2;
                        if differs {
                            break 'b_2a1518;
                        }
                    }
                    c.cycles += 1;
                    c.g[S0][0] = 0x4;
                }
                c.cycles += 10;
                c.g[A0][0] = 0x1;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20b9;
                call_ptr(c, 0x003d_8008, 0x002a_1528)?;
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x173c)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[S0][0];
                c.g[A1][0] = 0x2;
                let line = c.g[V1][0] as u32;
                call_ptr(c, line, 0x002a_1540)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
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
    use crate::difftest::fakes::{machine, machine_rt, FAKE};
    use crate::difftest::{check, elf_image, Rng, DATA, GAME_GP};

    /// The rider's speech setup shared by the three say-line tests: the
    /// audio at DATA, the rider at DATA+0x4000 with its voice record at +0x6c0
    /// (its method call goes to a fake at FAKE), and the line table pointer at
    /// gp+0x173c (a fake too). `a2` is the caller's third argument.
    pub(crate) fn rider_say_setup(c: &mut Cpu, seed: u32, salt: u64, a2: u32) -> Rng {
        let mut r = machine_rt(c, seed, salt, 0);
        elf_image(c);
        let audio = DATA;
        let rider = DATA + 0x4000;
        c.g[A0][0] = audio as u64;
        c.g[A1][0] = rider as u64;
        c.g[A2][0] = a2 as u64;
        let rec = DATA + 0x3000;
        c.bus.write32(rider + 0x6c0, rec);
        c.bus.write16(rec + 0x28, (r.next_u32() & 0xf) as u16);
        c.bus.write32(rec + 0x2c, FAKE + 0x10);
        c.bus.write32(GAME_GP + 0x173c, FAKE + 0x20);
        c.bus.write32(audio + 0x5560, r.next_u32());
        r
    }

    #[test]
    fn ssxaudio_set_mission_menu_flag_matches() {
        check(
            ssx3_ee::s08::f_0029d6d0__generated,
            ssxaudio_set_mission_menu_flag,
            50,
            &|c, seed| {
                let _ = machine(c, seed, 0x29d6, 0);
                c.g[A0][0] = DATA as u64 + ((seed as u64 & 3) << 4);
            },
        );
    }

    #[test]
    fn ssxaudio_avalanche_update_stub_matches() {
        check(
            ssx3_ee::s08::f_0029e560__generated,
            ssxaudio_avalanche_update_stub,
            20,
            &|c, seed| {
                let _ = machine(c, seed, 0x29e5, 0);
            },
        );
    }

    #[test]
    fn ssxaudio_calc_avalanche_volume_matches() {
        check(
            ssx3_ee::s08::f_0029e438__generated,
            ssxaudio_calc_avalanche_volume,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29e4, 0);
                // The three gp-relative constants (a float each).
                c.bus.write32(GAME_GP - 0x4588, r.edge_f32());
                c.bus.write32(GAME_GP - 0x4584, r.edge_f32());
                c.bus.write32(GAME_GP - 0x4580, r.edge_f32());
                let avalanche = DATA + 0x100;
                c.g[A1][0] = avalanche as u64;
                c.bus.write32(avalanche + 0x10, r.edge_f32());
                c.bus.write32(
                    avalanche + 0x14,
                    match seed % 4 {
                        0 => r.f32(1.0),
                        _ => r.edge_f32(),
                    },
                );
            },
        );
    }

    #[test]
    fn ssxaudio_avalanche_sound_callback_matches() {
        check(
            ssx3_ee::s08::f_0029e4a0__generated,
            ssxaudio_avalanche_sound_callback,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x29e4_a0, 0);
                elf_image(c);
                let avalanche = DATA + 0x2000;
                let inst = DATA + 0x3000;
                c.g[A0][0] = avalanche as u64;
                c.g[A1][0] = inst as u64;
                c.bus.write32(avalanche + 0x6070, r.next_u32() % 4);
                c.bus.write32(avalanche + 0x6050, r.next_u32());
                c.bus.write32(avalanche + 0x6060, r.next_u32());
                let rec = DATA + 0x3800;
                c.bus.write32(rec + 0x78, r.next_u32() % 4);
                c.bus.write16(rec + 0x28, (r.next_u32() & 0xf) as u16);
                c.bus.write32(rec + 0x2c, FAKE + 0x30);
                c.bus.write32(avalanche + 0x6c0, rec);
                c.bus.write32(GAME_GP + 0x173c, FAKE + 0x20);
            },
        );
    }

    #[test]
    fn ssxaudio_on_boost_meter_message_matches() {
        check(
            ssx3_ee::s08::f_00299638__generated,
            ssxaudio_on_boost_meter_message,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x2996_38, 0);
                elf_image(c);
                let audio = DATA;
                let rider = DATA + 0x4000;
                c.g[A0][0] = audio as u64;
                c.g[A1][0] = rider as u64;
                c.bus.write32(audio + 0x118, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, DATA + 0x2000);
                c.bus
                    .write32(audio + 0x59e4, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus
                    .write32(rider + 0x874, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus
                    .write32(rider + 0x87c, if seed % 4 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(rider + 0x470, r.edge_f32());
                c.bus.write32(
                    rider + 0x2f4,
                    match seed % 4 {
                        0 => 10,
                        1 => 5,
                        _ => r.next_u32() % 12,
                    },
                );
            },
        );
    }

    #[test]
    fn ssxaudio_rider_say_20d3_matches() {
        check(
            ssx3_ee::s08::f_002a1138__generated,
            ssxaudio_rider_say_20d3,
            200,
            &|c, seed| {
                let mut r =
                    rider_say_setup(c, seed, 0x2a11_38, if seed % 2 == 0 { 0 } else { seed + 1 });
                c.bus.write32(DATA + 0x4000 + 0x2c, r.next_u32());
            },
        );
    }

    #[test]
    fn ssxaudio_rider_say_20d4_matches() {
        check(
            ssx3_ee::s08::f_002a1280__generated,
            ssxaudio_rider_say_20d4,
            200,
            &|c, seed| {
                let mut r =
                    rider_say_setup(c, seed, 0x2a12_80, if seed % 2 == 0 { 0 } else { seed + 1 });
                c.bus.write32(DATA + 0x4000 + 0x2c, r.next_u32());
            },
        );
    }

    #[test]
    fn ssxaudio_rider_say_20b9_matches() {
        check(
            ssx3_ee::s08::f_002a1400__generated,
            ssxaudio_rider_say_20b9,
            200,
            &|c, seed| {
                let mut r = rider_say_setup(c, seed, 0x2a14_00, 0);
                c.bus.write32(0x0053_0000 + 0x5c11, r.next_u32() % 5);
            },
        );
    }
}
