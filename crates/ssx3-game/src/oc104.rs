//! Phase 2 wave 32 (docs/investigation/phase2-worker-guide.md): typed
//! replacement for `CrowdAudio_UpdateInstance` (0x2a4e88), the per-frame
//! update of one crowd-audio voice.
//!
//! The voice owns three 0x30-byte "motion slots" (`voice+0xe0`, `+0x110`,
//! `+0x140`). The update stages the slot positions in a cleared stack
//! buffer, advances every slot that carries a sound (`slot+0x8 != 0`)
//! through the mixer's per-frame tables, writes the positions back and
//! starts (or retunes) the MIDI player when the voice's player state
//! changes. A voice with no sound to advance only refreshes its cached
//! volume (`voice+0x178`) and the cached instance pitch.
//!
//! Layout of the objects it touches (byte offsets):
//!
//! - **voice** (argument): `+0x0` SND handle, `+0xd4` "has audio" gate,
//!   `+0xd8` sound id, `+0x170` voice kind, `+0x174` MIDI player id (-1
//!   none), `+0x178` cached volume, `+0x17c` loudness, `+0x180` player
//!   state (0 idle, 1 starting, 2 playing, 3 starting again), `+0x18c`
//!   bank-monitor instance, `+0x10..+0x50` play parameters (four quads),
//!   `+0xe0 + 0x30*i` motion slot i.
//! - **SND system** (`*(gp + 0x608)`): `+0x0` the mixer, `+0x4`/`+0x8`
//!   per-frame word arrays, `+0xc` the playback interface (`+0x10` a
//!   signed half `this` delta, `+0x14` the function), `+0x1d8` the bank
//!   monitor (instances at `+0x8ec`, 64 of them).
//! - **mixer** (`*(SND system)`): a ring of `count` (`+0x1f4`) 24-byte
//!   entries at `*(+0x1f8)`, a template at `+0x1dc..+0x1f4`, and per-entry
//!   word tables at `+0x214`, `+0x238`, `+0x23c`, `+0x24c`, `+0x254`.
//! - **motion slot**: `+0x4` signed frame counter, `+0x8` sound, `+0x10`
//!   position quad, `+0x20` mixer slot index.
//!
//! Every block charges the cycles the generated body charges at the same
//! point, the registers keep the generated values (including the scratch
//! ones a callee observes), and the loops deliver events at the same back
//! edges.

use crate::call::{call_ptr, guest, Callee, Flow};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x002a_4e88, crowd_audio_update_instance, "CrowdAudio_UpdateInstance")];

/// `gp + 0x608`: the SND system. (Unlike the game globals this one lives
/// above `gp`.)
const SND_SYSTEM_GP: u32 = 0x608;

mod callee {
    use super::{ee, Callee};

    pub const FIND_INSTANCE: Callee = Callee { addr: 0x002a_d550, f: ee::s08::BankMonitor_FindInstance };
    pub const STOP_SOUND: Callee = Callee { addr: 0x002a_6848, f: ee::s08::CrowdAudio_InstanceStopSound };
    pub const PICK_SAMPLE: Callee = Callee { addr: 0x002a_4ca0, f: ee::s08::CrowdAudio_PickRandomSample };
    pub const PUSH_PLAY_PARAMS: Callee = Callee { addr: 0x002a_8450, f: ee::s08::MIDIBanks_PushPlayParams };
    pub const MIDI_PLAY: Callee = Callee { addr: 0x002a_86b8, f: ee::s08::MIDIBanks_Play };
    pub const SET_PITCH: Callee = Callee { addr: 0x002a_be78, f: ee::s08::BankMonitorInstance_SetPitch };
    pub const PLAY_PUSHED_SOUND: Callee = Callee { addr: 0x0029_06b8, f: ee::s07::Audio_PlayPushedSound };
    pub const MEMSET: Callee = Callee { addr: 0x0041_6210, f: ee::s15::memset };
}

// ---------------------------------------------------------------------------
// The body.

/// `CrowdAudio_UpdateInstance(voice)` 0x2a4e88: advance the voice's motion
/// slots and keep its MIDI player in step with them.
pub fn crowd_audio_update_instance(c: &mut Cpu) {
    guest(c, body);
}

fn body(c: &mut Cpu) -> Flow {
    'b_2a5ca4: {
        c.cycles += 15;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[S7][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xd4));
        let t = c.g[V0][0] == 0u64;
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        if t {
            break 'b_2a5ca4;
        } else {
            c.cycles += 11;
            c.sw((c.g[SP][0] as u32), 0u64);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8c), c.g[V1][0]);
            c.g[T1][0] = 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
            c.g[T2][0] = 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
            c.g[V0][0] = 0x2;
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
            c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[V1][0] = 0xffffffffffffffff;
            'l_2a4ef0: loop {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                let t = c.g[V0][0] != c.g[V1][0];
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8c));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x002a4ef0);
                    }
                    continue 'l_2a4ef0;
                } else {
                    c.cycles += 3;
                    c.g[V1][0] = c.g[S3][0];
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S5][0] = 0x2;
                    'l_2a4f18: loop {
                        c.cycles += 11;
                        c.sw((c.g[V0][0] as u32), 0u64);
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                        c.sw((c.g[V1][0] as u32), 0u64);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.sw((c.g[A0][0] as u32), 0u64);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                        let t = (c.g[S5][0] as i64) >= 0;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002a4f18);
                            }
                            continue 'l_2a4f18;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = 0xc;
                            c.mult(0, 2, 9, 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                            c.g[A3][0] = c.g[V1][0];
                            'l_2a4f58: loop {
                                'b_2a4f78: {
                                    c.cycles += 5;
                                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xc));
                                    let t = c.g[A0][0] != 0u64;
                                    c.g[S6][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                    if t {
                                        break 'b_2a4f78;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = ((c.g[T1][0] as i64) < 15i64) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[A3][0] = c.g[V1][0];
                                            if event_due(c) {
                                                deliver_event_here(c, 0x002a4f58);
                                            }
                                            continue 'l_2a4f58;
                                        } else {
                                            break 'b_2a4f78;
                                        }
                                    }
                                }
                                'b_2a51d4: {
                                    c.cycles += 16;
                                    c.lq(4, (c.g[A0][0] as u32));
                                    c.g[S5][0] = 0x1;
                                    c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xd4));
                                    c.sq(4, (c.g[SP][0] as u32));
                                    c.g[A1][0] = ((c.g[S5][0] as i64) < (c.g[A1][0] as i64)) as u64;
                                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                    c.g[FP][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[SP][0] as u32)));
                                    c.sq(4, (c.g[V0][0] as u32).wrapping_add(0x10));
                                    c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    let t = c.g[A1][0] == 0u64;
                                    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                    if t {
                                        break 'b_2a51d4;
                                    } else {
                                        c.cycles += 6;
                                        c.g[V0][0] = 0xc;
                                        c.g[T3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8c));
                                        c.mult(0, 2, 9, 2);
                                        c.g[T4][0] = c.g[S3][0];
                                        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
                                        c.lwc1(5, (c.g[SP][0] as u32));
                                        'l_2a4fd0: loop {
                                            c.cycles += 4;
                                            c.g[T5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                            c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                                            c.g[A3][0] = c.g[V1][0];
                                            'l_2a4fe0: loop {
                                                'b_2a5004: {
                                                    c.cycles += 6;
                                                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0xc));
                                                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc));
                                                    c.g[S6][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xc));
                                                    let t = c.g[A0][0] != 0u64;
                                                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                                                    if t {
                                                        break 'b_2a5004;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = ((c.g[T1][0] as i64) < 15i64) as u64;
                                                        if c.g[V0][0] != 0u64 {
                                                            c.g[A3][0] = c.g[V1][0];
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x002a4fe0);
                                                            }
                                                            continue 'l_2a4fe0;
                                                        } else {
                                                            break 'b_2a5004;
                                                        }
                                                    }
                                                }
                                                'b_2a5090: {
                                                    'b_2a5088: {
                                                        c.cycles += 30;
                                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                                        c.lwc1(0, (c.g[A0][0] as u32));
                                                        c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                        c.mtc1(2, 4);
                                                        c.cvt_s_w(4, 4);
                                                        c.mtc1(3, 2);
                                                        c.cvt_s_w(2, 2);
                                                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                        c.sub_s(0, 0, 5);
                                                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                        c.div_s(4, 4, 2);
                                                        c.mul_s(0, 0, 4);
                                                        c.add_s(0, 5, 0);
                                                        c.swc1(0, (c.g[SP][0] as u32));
                                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                                                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                        c.sub_s(0, 0, 1);
                                                        c.mul_s(0, 0, 4);
                                                        c.add_s(1, 1, 0);
                                                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                                                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                        c.sub_s(0, 0, 3);
                                                        c.mul_s(0, 0, 4);
                                                        c.add_s(3, 3, 0);
                                                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                                        c.g[V0][0] = ((c.g[FP][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = sx(((c.g[FP][0] as u32) as i32 >> 2) as u32);
                                                        if t {
                                                            break 'b_2a5088;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[FP][0] = c.g[V1][0];
                                                            break 'b_2a5090;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 2) as u32);
                                                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                    break 'b_2a5090;
                                                }
                                                'b_2a51c4: {
                                                    c.cycles += 4;
                                                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x170));
                                                    c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xd4));
                                                        break 'b_2a51c4;
                                                    } else {
                                                        'b_2a51c0: {
                                                            'b_2a51ac: {
                                                                c.cycles += 57;
                                                                c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                                                c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                c.mtc1(6, 4);
                                                                c.cvt_s_w(4, 4);
                                                                c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                                                c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                                                                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[SP][0] as u32)));
                                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                                                c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                                                                c.mtc1(4, 3);
                                                                c.cvt_s_w(3, 3);
                                                                c.lwc1(0, (c.g[V0][0] as u32));
                                                                c.g[V1][0] = c.g[A1][0];
                                                                c.sub_s(0, 0, 2);
                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                                                                c.div_s(4, 4, 3);
                                                                c.lwc1(1, (c.g[V1][0] as u32));
                                                                c.mul_s(0, 0, 4);
                                                                c.add_s(1, 1, 0);
                                                                c.swc1(1, (c.g[V1][0] as u32));
                                                                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                                                c.g[A0][0] = c.lw((c.g[A3][0] as u32));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                                                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[SP][0] as u32)));
                                                                c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x4));
                                                                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x14));
                                                                c.g[V0][0] = c.g[V1][0];
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                                                                c.sub_s(1, 1, 2);
                                                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                c.mul_s(1, 1, 4);
                                                                c.add_s(0, 0, 1);
                                                                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                                                c.g[A0][0] = c.lw((c.g[A3][0] as u32));
                                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[SP][0] as u32)));
                                                                c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x8));
                                                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                c.g[V1][0] = c.g[V0][0];
                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                                                                c.sub_s(1, 1, 2);
                                                                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                                c.mul_s(1, 1, 4);
                                                                c.add_s(0, 0, 1);
                                                                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                                                c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                                                c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V1][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                c.g[V1][0] = c.lw((c.g[V1][0] as u32));
                                                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 2) as u32);
                                                                if t {
                                                                    break 'b_2a51ac;
                                                                } else {
                                                                    c.cycles += 10;
                                                                    c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                    c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                                                                    c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                                                    break 'b_2a51c0;
                                                                }
                                                            }
                                                            c.cycles += 5;
                                                            c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                            c.g[V1][0] = sx(((c.g[A1][0] as u32) as i32 >> 2) as u32);
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                                                            break 'b_2a51c0;
                                                        }
                                                        c.cycles += 1;
                                                        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xd4));
                                                        break 'b_2a51c4;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[S5][0] = c.g[T5][0];
                                                c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.lwc1(5, (c.g[SP][0] as u32));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002a4fd0);
                                                    }
                                                    continue 'l_2a4fd0;
                                                } else {
                                                    break 'b_2a51d4;
                                                }
                                            }
                                        }
                                    }
                                }
                                'b_2a5a68: {
                                    'b_2a55e8: {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x170));
                                        c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                        c.g[V0][0] = 0x1;
                                        let t = c.g[A0][0] != c.g[V0][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x78), c.g[A0][0]);
                                        if t {
                                            break 'b_2a55e8;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4));
                                            let t = (c.g[A1][0] as i64) < 0;
                                            c.lq(2, (c.g[SP][0] as u32));
                                            if t {
                                                break 'b_2a5a68;
                                            } else {
                                                c.cycles += 10;
                                                c.g[A3][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                c.g[A2][0] = 0x1;
                                                c.sq(2, (c.g[S6][0] as u32).wrapping_add(0x10));
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x7c), c.g[A3][0]);
                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                                callee::FIND_INSTANCE.call(c, 0x002a5214)?;
                                                if c.g[V0][0] != 0u64 {
                                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x64), c.g[FP][0]);
                                                    break 'b_2a5a68;
                                                } else {
                                                    'b_2a54fc: {
                                                        'b_2a54d0: {
                                                            c.cycles += 167;
                                                            c.g[T0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xd8));
                                                            c.g[A0][0] = 0x18;
                                                            c.g[S2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                            c.g[A1][0] = 0u64;
                                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x80), c.g[T0][0]);
                                                            c.g[A2][0] = 0x8;
                                                            c.g[S0][0] = c.lw((c.g[S2][0] as u32));
                                                            c.g[S5][0] = c.lw((c.g[S6][0] as u32));
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                            c.mult(0, 3, 3, 4);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1e3));
                                                            c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1dc));
                                                            c.ldl(8, (c.g[S0][0] as u32).wrapping_add(0x1eb));
                                                            c.ldr(8, (c.g[S0][0] as u32).wrapping_add(0x1e4));
                                                            c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1f3));
                                                            c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1ec));
                                                            c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                            c.sdr(7, (c.g[V1][0] as u32));
                                                            c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                            c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x17));
                                                            c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V1][0] as u32), 0u64);
                                                            c.g[V0][0] = 0x18;
                                                            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                            c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
                                                            c.mult(0, 4, 4, 2);
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                                            c.g[V1][0] = c.lb((c.g[A0][0] as u32));
                                                            c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), 0u64);
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                                                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                                                            callee::MEMSET.call(c, 0x002a5304)?;
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[A0][0] = 0x64;
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                                                            c.g[A1][0] = 0x5a;
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                            c.g[A2][0] = 0x32;
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[A3][0] = 0x7f;
                                                            c.sw((c.g[V1][0] as u32), 0u64);
                                                            c.g[T0][0] = 0x18;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), 0u64);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V1][0] as u32), 0u64);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), 0u64);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V1][0] as u32), 0u64);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), 0u64);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V1][0] as u32), 0u64);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), c.g[A3][0]);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                                                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V1][0] as u32), c.g[A0][0]);
                                                            c.g[A0][0] = c.lw((c.g[S2][0] as u32));
                                                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                            c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                                                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), c.g[S5][0]);
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x7c));
                                                            c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[A2][0]);
                                                            c.g[A1][0] = c.lw((c.g[A0][0] as u32));
                                                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                                                            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), c.g[FP][0]);
                                                            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                                            c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                                                            c.mult(0, 4, 4, 8);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                            c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                            c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                                            if t {
                                                                break 'b_2a54d0;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                                                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                                let t = (c.g[V1][0] as i64) < 0;
                                                                c.g[V0][0] = 0u64;
                                                                if t {
                                                                    break 'b_2a54fc;
                                                                } else {
                                                                    break 'b_2a54d0;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 7;
                                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                        c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
                                                        if c.g[V1][0] == 0u64 {
                                                            c.g[V0][0] = 0x7f;
                                                            break 'b_2a54fc;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                                            c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
                                                            break 'b_2a54fc;
                                                        }
                                                    }
                                                    c.cycles += 59;
                                                    c.sb((c.g[A0][0] as u32), c.g[V0][0]);
                                                    c.g[A2][0] = 0x1;
                                                    c.g[A3][0] = 0x4a0000;
                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                    c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x17c));
                                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                    c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24c));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                    c.sw((c.g[V0][0] as u32), c.g[A2][0]);
                                                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3700));
                                                    c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
                                                    c.ldr(2, (c.g[A2][0] as u32));
                                                    c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x67));
                                                    c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                                                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                    c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x67));
                                                    c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x60));
                                                    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x47));
                                                    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x40));
                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x238));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                    c.sw((c.g[V0][0] as u32), c.g[S7][0]);
                                                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x254));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                    c.sw((c.g[V0][0] as u32), 0u64);
                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                    c.g[A1][0] = c.lw((c.g[S7][0] as u32));
                                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                    c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                    callee::PLAY_PUSHED_SOUND.call(c, 0x002a55e0)?;
                                                    c.sw((c.g[S6][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                                    break 'b_2a5a68;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[S5][0] = 0u64;
                                    c.g[V0][0] = 0x30;
                                    'l_2a55f0: loop {
                                        'b_2a5a5c: {
                                            c.cycles += 6;
                                            c.mult(0, 2, 21, 2);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xe0));
                                            c.g[S6][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                            c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                                            if c.g[V1][0] == 0u64 {
                                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                                break 'b_2a5a5c;
                                            } else {
                                                'b_2a5a58: {
                                                    c.cycles += 3;
                                                    c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4));
                                                    let t = (c.g[A1][0] as i64) < 0;
                                                    c.g[A2][0] = 0x1;
                                                    if t {
                                                        break 'b_2a5a58;
                                                    } else {
                                                        'b_2a5a38: {
                                                            c.cycles += 13;
                                                            c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                                            c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                            c.g[S4][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[SP][0] as u32)));
                                                            c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                                                            c.sq(2, (c.g[S6][0] as u32).wrapping_add(0x10));
                                                            c.g[A0][0] = c.lw((c.g[A3][0] as u32));
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                                            callee::FIND_INSTANCE.call(c, 0x002a563c)?;
                                                            c.g[A2][0] = c.g[V0][0];
                                                            if c.g[A2][0] != 0u64 {
                                                                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                                                break 'b_2a5a38;
                                                            } else {
                                                                'b_2a594c: {
                                                                    'b_2a5920: {
                                                                        c.cycles += 176;
                                                                        c.g[A2][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xd8));
                                                                        c.g[T0][0] = 0x18;
                                                                        c.g[S2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                        c.g[A1][0] = 0u64;
                                                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x84), c.g[A2][0]);
                                                                        c.g[A2][0] = 0x8;
                                                                        c.g[A3][0] = c.lw((c.g[S6][0] as u32));
                                                                        c.g[S0][0] = c.lw((c.g[S2][0] as u32));
                                                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x88), c.g[A3][0]);
                                                                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                        c.mult(0, 3, 3, 8);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
                                                                        c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
                                                                        c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
                                                                        c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
                                                                        c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
                                                                        c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
                                                                        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                                        c.sdr(2, (c.g[V1][0] as u32));
                                                                        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                                        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                                        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                                                                        c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V1][0] as u32), 0u64);
                                                                        c.g[V0][0] = 0x18;
                                                                        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                        c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
                                                                        c.mult(0, 4, 4, 2);
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                                                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                                                        c.g[V1][0] = c.lb((c.g[A0][0] as u32));
                                                                        c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32), 0u64);
                                                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                                                                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                                                                        callee::MEMSET.call(c, 0x002a5734)?;
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[A0][0] = 0x64;
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                                                                        c.g[A1][0] = 0x5a;
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                        c.g[A2][0] = 0x32;
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.g[A3][0] = 0x7f;
                                                                        c.sw((c.g[V1][0] as u32), 0u64);
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32), 0u64);
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V1][0] as u32), 0u64);
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32), 0u64);
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                                                                        c.g[A0][0] = 0x1;
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V1][0] as u32), 0u64);
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32), 0u64);
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V1][0] as u32), 0u64);
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32), c.g[A3][0]);
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V1][0] as u32), c.g[A0][0]);
                                                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32));
                                                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x84));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                                                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x88));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32), c.g[A2][0]);
                                                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                        c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[S4][0]);
                                                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8c));
                                                                        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                                                        c.g[A3][0] = c.lw((c.g[A0][0] as u32));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1d8));
                                                                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                                                        c.g[A1][0] = c.lw((c.g[A2][0] as u32));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x38));
                                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.g[A1][0] = 0x18;
                                                                        c.sw((c.g[V1][0] as u32), c.g[A0][0]);
                                                                        c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x1c));
                                                                        c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x38));
                                                                        c.mult(0, 4, 4, 5);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                        c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x20));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                                        c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                                                        if t {
                                                                            break 'b_2a5920;
                                                                        } else {
                                                                            c.cycles += 6;
                                                                            c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x1c));
                                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                                            let t = (c.g[V1][0] as i64) < 0;
                                                                            c.g[V0][0] = 0u64;
                                                                            if t {
                                                                                break 'b_2a594c;
                                                                            } else {
                                                                                break 'b_2a5920;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 7;
                                                                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x1c));
                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                                    c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
                                                                    if c.g[V1][0] == 0u64 {
                                                                        c.g[V0][0] = 0x7f;
                                                                        break 'b_2a594c;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x1c));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                                                        c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
                                                                        break 'b_2a594c;
                                                                    }
                                                                }
                                                                c.cycles += 59;
                                                                c.sb((c.g[A0][0] as u32), c.g[V0][0]);
                                                                c.g[A3][0] = 0x1;
                                                                c.g[A2][0] = 0x4a0000;
                                                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x17c));
                                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24c));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                c.sw((c.g[V0][0] as u32), c.g[A3][0]);
                                                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                c.g[A1][0] = c.lw((c.g[S7][0] as u32));
                                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                c.g[A1][0] = c.lw((c.g[V1][0] as u32));
                                                                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3700));
                                                                c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                                                                c.ldr(8, (c.g[A0][0] as u32));
                                                                c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x67));
                                                                c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x60));
                                                                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
                                                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x67));
                                                                c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                                                                c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x47));
                                                                c.sdr(5, (c.g[A0][0] as u32).wrapping_add(0x40));
                                                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x238));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                c.sw((c.g[V0][0] as u32), c.g[S7][0]);
                                                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x254));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                c.sw((c.g[V0][0] as u32), 0u64);
                                                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                callee::PLAY_PUSHED_SOUND.call(c, 0x002a5a30)?;
                                                                c.sw((c.g[S6][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                                                break 'b_2a5a58;
                                                            }
                                                        }
                                                        c.cycles += 8;
                                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8c));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[A0][0] = c.lw((c.g[A1][0] as u32));
                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x64), c.g[V1][0]);
                                                        break 'b_2a5a58;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                                break 'b_2a5a5c;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = ((c.g[S5][0] as i64) < 3i64) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = 0x30;
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x002a55f0);
                                            }
                                            continue 'l_2a55f0;
                                        } else {
                                            break 'b_2a5a68;
                                        }
                                    }
                                }
                                'b_2a5c98: {
                                    'b_2a5c3c: {
                                        'b_2a5c38: {
                                            c.cycles += 5;
                                            c.g[V0][0] = 0x1000;
                                            c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x180));
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x74), c.g[V0][0]);
                                            let t = c.g[V1][0] == 0u64;
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                                            if t {
                                                break 'b_2a5c38;
                                            } else {
                                                'b_2a5ab4: {
                                                    c.cycles += 14;
                                                    c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                                                    c.g[A1][0] = c.lw((c.g[S7][0] as u32));
                                                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x74));
                                                    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xc));
                                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x002a5aa0)?;
                                                    c.g[S0][0] = c.g[V0][0];
                                                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.sw((c.g[S7][0] as u32).wrapping_add(0x18c), c.g[S0][0]);
                                                        break 'b_2a5ab4;
                                                    } else {
                                                        break 'b_2a5ab4;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x180));
                                                c.g[V0][0] = 0x1;
                                                if c.g[V1][0] != c.g[V0][0] {
                                                    c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x174));
                                                    break 'b_2a5c3c;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x3;
                                                    if c.g[S0][0] == c.g[V0][0] {
                                                        c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x174));
                                                        break 'b_2a5c3c;
                                                    } else {
                                                        c.cycles += 10;
                                                        c.g[A0][0] = c.g[S7][0];
                                                        callee::STOP_SOUND.call(c, 0x002a5ad8)?;
                                                        c.g[A0][0] = 0x54;
                                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                        c.mult(0, 4, 16, 4);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4c));
                                                        callee::PICK_SAMPLE.call(c, 0x002a5af0)?;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A2][0] = c.g[V0][0];
                                                        if t {
                                                            break 'b_2a5c38;
                                                        } else {
                                                            'b_2a5bbc: {
                                                                'b_2a5b90: {
                                                                    c.cycles += 32;
                                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                    c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xd8));
                                                                    callee::PUSH_PLAY_PARAMS.call(c, 0x002a5b04)?;
                                                                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                    c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x10));
                                                                    c.g[A3][0] = 0x18;
                                                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[A1][0]);
                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x74));
                                                                    c.g[A1][0] = c.lw((c.g[A2][0] as u32));
                                                                    c.mult(0, 4, 30, 4);
                                                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                                                                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
                                                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 12) as u32);
                                                                    c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                                                                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                                                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                                                    c.g[T0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                                                                    c.mult(0, 4, 4, 7);
                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                                    c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                                                    if t {
                                                                        break 'b_2a5b90;
                                                                    } else {
                                                                        c.cycles += 6;
                                                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                                        let t = (c.g[V1][0] as i64) < 0;
                                                                        c.g[V0][0] = 0u64;
                                                                        if t {
                                                                            break 'b_2a5bbc;
                                                                        } else {
                                                                            break 'b_2a5b90;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 7;
                                                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                                                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                                c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
                                                                if c.g[V1][0] == 0u64 {
                                                                    c.g[V0][0] = 0x7f;
                                                                    break 'b_2a5bbc;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                                                    c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
                                                                    break 'b_2a5bbc;
                                                                }
                                                            }
                                                            c.cycles += 29;
                                                            c.sb((c.g[A0][0] as u32), c.g[V0][0]);
                                                            c.g[A2][0] = 0x1;
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                            c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x17c));
                                                            c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24c));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), c.g[A2][0]);
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                            c.g[A1][0] = c.lw((c.g[S7][0] as u32));
                                                            c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                            callee::MIDI_PLAY.call(c, 0x002a5c28)?;
                                                            let t = (c.g[V0][0] as i64) < 0;
                                                            c.sw((c.g[S7][0] as u32).wrapping_add(0x174), c.g[V0][0]);
                                                            if t {
                                                                break 'b_2a5c98;
                                                            } else {
                                                                c.cycles += 2;
                                                                c.g[V0][0] = 0x2;
                                                                c.sw((c.g[S7][0] as u32).wrapping_add(0x180), c.g[V0][0]);
                                                                break 'b_2a5c38;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x174));
                                        break 'b_2a5c3c;
                                    }
                                    c.cycles += 2;
                                    let t = (c.g[A1][0] as i64) < 0;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                    if t {
                                        break 'b_2a5c98;
                                    } else {
                                        'b_2a5c70: {
                                            c.cycles += 7;
                                            c.g[A2][0] = 0x2;
                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                            callee::FIND_INSTANCE.call(c, 0x002a5c54)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            if c.g[S0][0] != 0u64 {
                                                c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x180));
                                                break 'b_2a5c70;
                                            } else {
                                                c.cycles += 4;
                                                c.g[A0][0] = c.g[S7][0];
                                                callee::STOP_SOUND.call(c, 0x002a5c68)?;
                                                c.sw((c.g[S7][0] as u32).wrapping_add(0x18c), 0u64);
                                                break 'b_2a5c98;
                                            }
                                        }
                                        'b_2a5c94: {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[A0][0] = c.g[FP][0];
                                            if t {
                                                break 'b_2a5c94;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x70));
                                                c.g[A0][0] = c.g[S0][0];
                                                callee::SET_PITCH.call(c, 0x002a5c88)?;
                                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x74));
                                                c.mult(0, 2, 30, 2);
                                                c.g[A0][0] = sx(((c.g[V0][0] as u32) as i32 >> 12) as u32);
                                                break 'b_2a5c94;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x64), c.g[A0][0]);
                                        break 'b_2a5c98;
                                    }
                                }
                                c.cycles += 3;
                                c.lq(2, (c.g[SP][0] as u32));
                                c.sw((c.g[S7][0] as u32).wrapping_add(0x178), c.g[FP][0]);
                                c.sq(2, (c.g[S7][0] as u32).wrapping_add(0x10));
                                break 'b_2a5ca4;
                            }
                        }
                    }
                }
            }
        }
    }
    c.cycles += 12;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xc0));
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
    c.pc = t;
    return Ok(());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, fill, put_vec, DATA, GAME_GP};

    /// Everything the function touches lives in the test data area.
    const VOICE: u32 = DATA + 0x3_0000;
    const SND: u32 = DATA + 0x1_0000;
    const MIXER: u32 = DATA + 0x2_0000;
    const RING: u32 = DATA + 0x4_0000;
    const TABLES: u32 = DATA + 0x5_0000;
    const INTERFACE: u32 = DATA + 0x6_0000;
    const SOUND: u32 = DATA + 0x7_0000;
    const CHANNEL: u32 = DATA + 0x7_2000;
    /// `BankMonitor::instances` inside the SND system (`+0x1d8 + 0x8ec`).
    const INSTANCES: u32 = SND + 0xac4;
    /// One bank-monitor instance is 0xc0 bytes.
    const INSTANCE_SIZE: u32 = 0xc0;

    /// A voice, the SND system, its mixer and the objects the motion slots
    /// point at, with the state fields chosen by `seed` so the update
    /// follows different paths (idle voice, motion slots with and without a
    /// sound, each player state).
    fn fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1042_4e88, 8);
        c.bus.write32(GAME_GP + SND_SYSTEM_GP, SND);

        // The SND system: mixer, per-frame word arrays, playback interface,
        // and the bank monitor (all 64 instances free unless written below).
        c.bus.write32(SND, MIXER);
        c.bus.write32(SND + 0x4, TABLES);
        c.bus.write32(SND + 0x8, TABLES + 0x200);
        c.bus.write32(SND + 0xc, INTERFACE);
        c.bus.write16(INTERFACE + 0x10, 0);
        c.bus.write32(INTERFACE + 0x14, FAKE);

        // The mixer's ring: `count` entries at RING, the template copied in
        // front of them, and the per-entry word tables.
        let count = seed % 4;
        c.bus.write32(MIXER + 0x1f4, count);
        c.bus.write32(MIXER + 0x1f8, RING);
        fill(c, &mut r, MIXER + 0x1dc, 6); // 24-byte template
        c.bus.write32(MIXER + 0x20c, TABLES + 0x400);
        c.bus.write32(MIXER + 0x210, TABLES + 0x600);
        c.bus.write32(MIXER + 0x214, TABLES + 0x800);
        c.bus.write32(MIXER + 0x238, TABLES + 0xa00);
        c.bus.write32(MIXER + 0x23c, TABLES + 0xc00);
        c.bus.write32(MIXER + 0x240, TABLES + 0x1100);
        c.bus.write32(MIXER + 0x244, TABLES + 0x1200);
        c.bus.write32(MIXER + 0x248, TABLES + 0x1300);
        c.bus.write32(MIXER + 0x24c, TABLES + 0xe00);
        c.bus.write32(MIXER + 0x250, TABLES + 0x1400);
        c.bus.write32(MIXER + 0x254, TABLES + 0x1000);
        c.bus.write32(MIXER + 0x258, TABLES + 0x1500);
        c.bus.write32(MIXER + 0x25c, TABLES + 0x1600);
        for i in 0..0x1800 / 4 {
            c.bus.write32(TABLES + i * 4, r.next_u32() % 0x4000);
        }
        fill(c, &mut r, RING, 0x40);

        // The voice.
        fill(c, &mut r, VOICE, 0x60);
        c.bus.write32(VOICE + 0x0, r.next_u32() % 0x1000);
        c.bus.write32(VOICE + 0xd4, if seed % 5 == 0 { 0 } else { 1 });
        c.bus.write32(VOICE + 0xd8, r.next_u32() % 4);
        c.bus.write32(VOICE + 0x170, r.next_u32() % 4);
        c.bus.write32(VOICE + 0x174, (r.next_u32() % 5).wrapping_sub(1));
        c.bus.write32(VOICE + 0x178, r.next_u32());
        c.bus.write32(VOICE + 0x17c, r.next_u32() % 0x4000);
        c.bus.write32(VOICE + 0x180, r.next_u32() % 4);
        c.bus.write32(VOICE + 0x18c, INSTANCES + (r.next_u32() % 4) * INSTANCE_SIZE);
        for i in 0..4 {
            let q = [r.f32(1.0), r.f32(2.0), r.f32(3.0), r.f32(4.0)];
            put_vec(c, VOICE + 0x10 + 16 * i, q);
        }

        // The three motion slots: one idle, one with a sound, one with a
        // sound whose counter has run out.
        for i in 0..3 {
            let slot = VOICE + 0xe0 + 0x30 * i;
            fill(c, &mut r, slot, 0xc);
            c.bus.write32(slot, SOUND + 0x100 * i);
            c.bus.write32(
                slot + 0x4,
                match (seed + i) % 3 {
                    0 => 0xffff_ffff, // before the first step
                    1 => r.next_u32() % 0x100,
                    _ => r.next_u32() % 8,
                },
            );
            c.bus.write32(slot + 0x8, if i == 0 { 0 } else { CHANNEL + 0x100 * i });
            c.bus.write32(slot + 0x20, i);
            let q = [r.f32(50.0), r.f32(50.0), r.f32(50.0), r.f32(1.0)];
            put_vec(c, slot + 0x10, q);
            // The sound the slot carries: three floats read as deltas.
            put_vec(c, SOUND + 0x100 * i, [r.f32(10.0), r.f32(10.0), r.f32(10.0), 0]);
            // The mixer channel: its index and frame counter.
            c.bus.write32(CHANNEL + 0x100 * i + 0x4, r.next_u32() % 0x100);
            c.bus.write32(CHANNEL + 0x100 * i + 0x20, i);
            put_vec(c, CHANNEL + 0x100 * i + 0x10, [0, 0, 0, 0]);
        }

        c.g[A0][0] = sx(VOICE);
    }

    #[test]
    fn crowd_audio_update_instance_matches() {
        check(ee::s08::f_002a4e88__generated, crowd_audio_update_instance, 150, &|c, seed| {
            fixture(c, seed);
        });
    }
}
