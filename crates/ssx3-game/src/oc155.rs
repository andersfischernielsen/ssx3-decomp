//! Phase 2 wave 53 module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! - **`RiderMetrix_UpdateTrickHUD`** (0x117fe0): the per-frame trick HUD of
//!   one rider. It shows the pending trick's points (and the score tier's
//!   number), keeps the eight fixed "trick" messages (ids 0x10..0x17), the
//!   combo counter (slot 2 / id 0xc), the boost meter (slot 8 / id 0xa), the
//!   board-wake counters (slots 6 and 9), the "no trick" hint (slots 3, 4, 10,
//!   25, 2), the trick-name banner (slots 13, 5) and the trick history (slot
//!   0x19), then books the combo's bonus points into the metrix.
//!
//! The body is one long chain of forward-jumping blocks: each `'b_mmm: {}` is
//! a jump target of the original (`break` is its `b`/`j`), so the nesting and
//! the labels are kept as the generated code has them. Registers stay in the
//! guest GPRs/FPU registers the original uses: several of them (`v0`/`v1`,
//! `a2`, the FPU scratch registers) survive into the calls below and are
//! observable there, and a few blocks branch on a condition bit latched by an
//! earlier compare.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0011_7fe0,
        rider_metrix_update_trick_hud,
        "RiderMetrix_UpdateTrickHUD",
    ),
    (
        0x002e_6c08,
        c_speed_boost_streamer_fx_update_points,
        "cSpeedBoostStreamerFX_UpdatePoints",
    ),
];

// ---------------------------------------------------------------------------
// Memory helpers (the same widths and sign extension as the original's).

/// `base + off` as a 32-bit address.
#[inline(always)]
fn at(c: &Cpu, base: usize, off: u32) -> u32 {
    (c.g[base][0] as u32).wrapping_add(off)
}

/// `lw rd, off(base)`.
#[inline(always)]
fn lw(c: &mut Cpu, rd: usize, base: usize, off: u32) {
    let a = at(c, base, off);
    c.g[rd][0] = c.lw(a);
}

/// `sw rs, off(base)`.
#[inline(always)]
fn sw(c: &mut Cpu, base: usize, off: u32, rs: usize) {
    let a = at(c, base, off);
    c.sw(a, c.g[rs][0]);
}

/// `swc1 fs, off(base)`.
#[inline(always)]
fn swc1(c: &mut Cpu, base: usize, off: u32, fs: usize) {
    let a = at(c, base, off);
    c.swc1(fs, a);
}

/// `lwc1 fs, off(base)`.
#[inline(always)]
fn lwc1(c: &mut Cpu, fs: usize, base: usize, off: u32) {
    let a = at(c, base, off);
    c.lwc1(fs, a);
}

/// `addiu rd, rs, imm` (32-bit, sign-extended).
#[inline(always)]
fn addiu(c: &mut Cpu, rd: usize, rs: usize, imm: u32) {
    c.g[rd][0] = sx((c.g[rs][0] as u32).wrapping_add(imm));
}

/// `addu rd, rd, rs`.
#[inline(always)]
fn add_reg(c: &mut Cpu, rd: usize, rs: usize) {
    c.g[rd][0] = sx((c.g[rd][0] as u32).wrapping_add(c.g[rs][0] as u32));
}

/// `sw zero, off(base)`.
#[inline(always)]
fn sw_zero(c: &mut Cpu, base: usize, off: u32) {
    let a = at(c, base, off);
    c.sw(a, 0);
}

/// `lui at, hi(bits); mtc1 at, fs`: a float constant through `at`.
#[inline(always)]
fn fc(c: &mut Cpu, fs: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.f[fs] = bits;
}

// ---------------------------------------------------------------------------
// Field offsets.

/// `RiderMetrix` fields the trick HUD reads and writes (the reference names
/// from `sMetrix17C28`; the ones the C++ left unnamed keep their offset).
mod mx {
    /// `+0x14 trickBoost` (float): the multiplier `GetRoundedTrickPoints` uses.
    pub const TRICK_BOOST: u32 = 0x14;
    /// `+0x54`: the running trick counter used by the history slot.
    pub const TRICK_COUNT: u32 = 0x54;
    /// `+0x5c`: "a trick landed" flag for the history slot.
    pub const TRICK_FLAG: u32 = 0x5c;
    /// `+0x84 points`: the bonus booked at the end of the frame.
    pub const BONUS: u32 = 0x84;
    /// `+0x9c combo count`, `+0xa0 combo points`, `+0xa4 combo timer`.
    pub const COMBO_COUNT: u32 = 0x9c;
    pub const COMBO_POINTS: u32 = 0xa0;
    pub const COMBO_TIMER: u32 = 0xa4;
    /// `+0x1ac rider` (`RiderBase*`).
    pub const RIDER: u32 = 0x1ac;
    /// `+0x1b0 slots`: the 0x9c-byte HUD message records.
    pub const SLOTS: u32 = 0x1b0;
    /// A record's size (`index` at `+0x98`).
    pub const SLOT_SIZE: u32 = 0x9c;
}

/// `RiderBase` (and its owner/board sub-objects) fields the HUD reads.
mod rider {
    /// `+0x2f0`: the boost meter (`0` when no boost is available).
    pub const BOOST_METER: u32 = 0x2f0;
    /// `+0x2f4`: the boost level (`< 11` is shown as the number itself).
    pub const BOOST_LEVEL: u32 = 0x2f4;
    /// `+0x304 boostMode`: 3 is "no boost" (`lw & 3`).
    pub const BOOST_MODE: u32 = 0x304;
    /// `+0x2f8 boostStrength` and the owner's meter at `+0x330`.
    pub const BOOST_STRENGTH: u32 = 0x2f8;
    pub const OWNER_METER: u32 = 0x330;
    /// `+0x6c0`: the board object whose `+0x3c` is a virtual method.
    pub const BOARD: u32 = 0x6c0;
    /// `+0x77c owner`: the `RiderBase` (`+0x2c0` is its variant word).
    pub const OWNER: u32 = 0x77c;
    pub const OWNER_MODE: u32 = 0x2c0;
}

/// A `gp`-relative float threshold the HUD compares against.
#[inline(always)]
fn gpf(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// Direct calls (same shard; these go through the shard's override slots, so
/// an installed replacement is reached as well).
mod callee {
    use super::{ee, Callee};

    pub const GET_ROUNDED_TRICK_POINTS: Callee = Callee {
        addr: 0x0011_7948,
        f: ee::s00::RiderMetrix_GetRoundedTrickPoints,
    };
    pub const GET_SCORE_TIER: Callee = Callee {
        addr: 0x0011_9310,
        f: ee::s00::RiderMetrix_GetScoreTier,
    };
    pub const SHOW_HUD_NUMBER: Callee = Callee {
        addr: 0x0011_7b88,
        f: ee::s00::RiderMetrix_ShowHUDNumber,
    };
    pub const SHOW_HUD_TEXT: Callee = Callee {
        addr: 0x0011_7ae8,
        f: ee::s00::RiderMetrix_ShowHUDText,
    };
    pub const SHOW_NUMBER_DELAYED: Callee = Callee {
        addr: 0x0011_71a8,
        f: ee::s00::cHUDFX_ShowNumberDelayed,
    };
    pub const REMOVE_HUD_MESSAGE: Callee = Callee {
        addr: 0x0011_79e0,
        f: ee::s00::RiderMetrix_RemoveHUDMessage,
    };
    pub const GET_TRICK_BONUS: Callee = Callee {
        addr: 0x0011_7908,
        f: ee::s00::RiderMetrix_GetTrickBonus,
    };
    pub const GET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fee8,
        f: ee::s00::Rider_GetSubState,
    };
    pub const GET_HUD_ANGLE: Callee = Callee {
        addr: 0x0012_f118,
        f: ee::s00::RiderPrewindSub_GetHUDAngle,
    };
    pub const GET_FRAME_COUNTER: Callee = Callee {
        addr: 0x0012_98c8,
        f: ee::s00::AI_GetFrameCounter,
    };
}

/// Cross-shard calls the generator emitted as `call_indirect`.
mod table {
    /// `SSXAudioSystem_GetInstance()`.
    pub const AUDIO_SYSTEM: u32 = 0x0028_b180;
    /// `SSXAudioSystem_PlayTrickPointsTick(audio, rider, points)`.
    pub const AUDIO_TRICK_TICK: u32 = 0x0029_a7d8;
    /// Audio calls for the combo/boost counters.
    pub const AUDIO_CALL_A: u32 = 0x0029_b0e0;
    pub const AUDIO_CALL_B: u32 = 0x0029_b3c0;
    pub const AUDIO_CALL_C: u32 = 0x0029_9638;
    pub const AUDIO_CALL_D: u32 = 0x0029_97b8;
    /// `cBE_getBE()` and `cBE_getInterface(be, kind)`.
    pub const BE_GET_BE: u32 = 0x0014_dc80;
    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const BE_CALL: u32 = 0x0015_0960;
    /// The board's wake-effect methods (`rider+0x230` vtable).
    pub const BOARD_WAKE_CLEAR: u32 = 0x0013_5f70;
    /// `bool board_done(board, bool* out)` (0x1360c8): whether the board's
    /// blend has reached its rest value.
    pub const BOARD_DONE: u32 = 0x0013_60c8;
    pub const BOARD_WAKE_A: u32 = 0x0013_6100;
    pub const BOARD_WAKE_B: u32 = 0x0013_6168;
    /// A sine used by the HUD-effect timer.
    pub const SIN: u32 = 0x0031_c040;
    /// `func_00312aa0(anim, int)` (0x312aa0): play an animation.
    pub const PLAY_ANIM: u32 = 0x0031_2aa0;
}

/// A HUD record address: slot `n` of the table at `+0x1b0`.
#[inline(always)]
fn slot(n: u32) -> u32 {
    n * mx::SLOT_SIZE
}

/// `RiderMetrix_RemoveHUDMessage(m, id)`.
#[inline(always)]
fn remove_message(c: &mut Cpu, id: u64, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = id;
    callee::REMOVE_HUD_MESSAGE.call(c, ret)
}

/// `RiderMetrix_UpdateTrickHUD(RiderMetrix* m)` (0x117fe0).
pub fn rider_metrix_update_trick_hud(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S1, 0x80);
        frame.save(c, S0, 0x90);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 27, 0xd8);
        frame.save_fpr(c, 26, 0xd0);
        frame.save_fpr(c, 25, 0xc8);
        frame.save_fpr(c, 24, 0xc0);
        frame.save_fpr(c, 23, 0xb8);
        frame.save_fpr(c, 22, 0xb0);
        frame.save_fpr(c, 21, 0xa8);
        frame.save_fpr(c, 20, 0xa0);
        lw(c, V0, S1, mx::SLOTS);
        let has_hud = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x90);
        if has_hud {
            // ---------------------------------------------------------------
            // The pending trick's points and the score-tier number.
            'b_118088: {
                c.cycles += 5;
                callee::GET_ROUNDED_TRICK_POINTS.call(c, 0x0011_8040)?;
                c.g[S0][0] = c.g[V0][0];
                let positive = (c.g[S0][0] as i64) > 0;
                c.g[A0][0] = c.g[S1][0];
                if positive {
                    c.cycles += 15;
                    c.g[A1][0] = c.g[S0][0];
                    callee::GET_SCORE_TIER.call(c, 0x0011_8054)?;
                    fc(c, 12, 0xbf80_0000);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 1;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A3][0] = c.g[V0][0];
                    callee::SHOW_HUD_NUMBER.call(c, 0x0011_8070)?;
                    call_ptr(c, table::AUDIO_SYSTEM, 0x0011_8078)?;
                    lw(c, A1, S1, mx::RIDER);
                    c.g[A0][0] = c.g[V0][0];
                    // (The points again: the calls above clobbered a2.)
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, table::AUDIO_TRICK_TICK, 0x0011_8088)?;
                }
            }
            // ---------------------------------------------------------------
            // The boost meter's "delayed number" (message 0xb) while the
            // board is in its boost mode.
            'b_1180f0: {
                'b_1180e4: {
                    c.cycles += 5;
                    lw(c, A0, S1, mx::RIDER);
                    callee::GET_SUB_STATE.call(c, 0x0011_8090)?;
                    c.g[V1][0] = 8;
                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[A0][0] = c.g[S1][0];
                        break 'b_1180e4;
                    } else {
                        'b_1180e0: {
                            c.cycles += 6;
                            lw(c, V1, S1, mx::RIDER);
                            lw(c, V1, V1, rider::OWNER);
                            lw(c, V0, V1, rider::OWNER_MODE);
                            c.g[V0][0] ^= 0x3;
                            let t = c.g[V0][0] == 0;
                            c.g[A1][0] = 0xb;
                            if t {
                                break 'b_1180e0;
                            } else {
                                c.cycles += 11;
                                lwc1(c, 12, V1, rider::OWNER_METER);
                                fc(c, 0, 0x3f80_0000);
                                c.g[A2][0] = 0;
                                lw(c, A0, S1, mx::SLOTS);
                                c.g[A3][0] = 0;
                                c.min_s(12, 12, 0);
                                addiu(c, A0, A0, slot(11));
                                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_80d8)?;
                                c.g[A0][0] = c.g[S1][0];
                                break 'b_1180f0;
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S1][0];
                        break 'b_1180e4;
                    }
                }
                c.cycles += 3;
                remove_message(c, 0xb, 0x0011_80ec)?;
                c.g[A0][0] = c.g[S1][0];
                break 'b_1180f0;
            }
            // ---------------------------------------------------------------
            // The trick messages. `'b_1188fc` is the "HUD is up" merge point;
            // `break 'b_1188fc` skips to the next section.
            'b_1188fc: {
                'b_1188f8: {
                    'b_11889c: {
                        'b_118880: {
                            // Fixed messages 0x10..0x17 (the eight trick names
                            // of the running combo): clear them all.
                            'b_118194: {
                                c.cycles += 28;
                                for (i, ret) in [
                                    0x0011_80f8u32,
                                    0x0011_8104,
                                    0x0011_8110,
                                    0x0011_811c,
                                    0x0011_8128,
                                    0x0011_8134,
                                    0x0011_8140,
                                    0x0011_814c,
                                ]
                                .into_iter()
                                .enumerate()
                                {
                                    remove_message(c, 0x10 + i as u64, ret)?;
                                }
                                lw(c, A0, S1, mx::RIDER);
                                callee::GET_SUB_STATE.call(c, 0x0011_8154)?;
                                c.g[V1][0] = 2;
                                if c.g[V0][0] == c.g[V1][0] {
                                    lw(c, A0, S1, mx::RIDER);
                                    break 'b_118194;
                                } else {
                                    // Sub-state 2: the "replay" messages 0xc and
                                    // 0xd are live, otherwise clear them.
                                    'b_118190: {
                                        c.cycles += 5;
                                        lw(c, A0, S1, mx::RIDER);
                                        callee::GET_SUB_STATE.call(c, 0x0011_8168)?;
                                        c.g[V1][0] = 5;
                                        let replay = c.g[V0][0] == c.g[V1][0];
                                        c.g[A0][0] = c.g[S1][0];
                                        if replay {
                                            break 'b_118190;
                                        } else {
                                            c.cycles += 7;
                                            remove_message(c, 0xc, 0x0011_817c)?;
                                            c.g[A0][0] = c.g[S1][0];
                                            remove_message(c, 0xd, 0x0011_8188)?;
                                            c.g[A0][0] = c.g[S1][0];
                                            break 'b_118880;
                                        }
                                    }
                                    c.cycles += 1;
                                    lw(c, A0, S1, mx::RIDER);
                                    break 'b_118194;
                                }
                            }
                            // The HUD angle: slot 0xd while in sub-state 2 (the
                            // prewind), else the board's own method.
                            'b_1181d4: {
                                'b_1181c0: {
                                    c.cycles += 5;
                                    sw_zero(c, SP, 0);
                                    callee::GET_SUB_STATE.call(c, 0x0011_819c)?;
                                    c.g[V1][0] = 2;
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    lw(c, V0, S1, mx::RIDER);
                                    if t {
                                        break 'b_1181c0;
                                    } else {
                                        c.cycles += 6;
                                        c.g[A1][0] = c.g[SP][0];
                                        lw(c, A0, V0, rider::OWNER);
                                        addiu(c, A0, A0, 0x1f0);
                                        callee::GET_HUD_ANGLE.call(c, 0x0011_81b8)?;
                                        c.mov_s(27, 0);
                                        break 'b_1181d4;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A1][0] = c.g[SP][0];
                                lw(c, A0, V0, rider::OWNER);
                                addiu(c, A0, A0, 0x230);
                                call_ptr(c, table::BOARD_DONE, 0x0011_81d0)?;
                                c.mov_s(27, 0);
                                break 'b_1181d4;
                            }
                            // ---------------------------------------------------
                            // The trick-name banner (slot 13, id 0xd) fades in
                            // and out with the HUD-effect timer in f12.
                            'b_11838c: {
                                'b_1181f0: {
                                    c.cycles += 3;
                                    lw(c, V0, SP, 0);
                                    let t = c.g[V0][0] == 0;
                                    c.g[A0][0] = c.g[S1][0];
                                    if t {
                                        break 'b_1181f0;
                                    } else {
                                        c.cycles += 4;
                                        remove_message(c, 0xd, 0x0011_81e8)?;
                                        c.g[A0][0] = c.g[S1][0];
                                        break 'b_11838c;
                                    }
                                }
                                // The timer: a triangle wave over the frame
                                // counter, scaled by two gp constants.
                                'b_118240: {
                                    c.cycles += 17;
                                    callee::GET_FRAME_COUNTER.call(c, 0x0011_81f8)?;
                                    c.mtc1(2, 0);
                                    c.cvt_s_w(0, 0);
                                    c.lwc1(12, gpf(c, 0x7ae0));
                                    c.mul_s(12, 0, 12);
                                    call_ptr(c, table::SIN, 0x0011_820c)?;
                                    c.lwc1(1, gpf(c, 0x7adc));
                                    c.lwc1(2, gpf(c, 0x7ad8));
                                    c.mul_s(1, 27, 1);
                                    c.mul_s(0, 0, 2);
                                    c.mtc1(0, 3);
                                    c.add_s(12, 1, 0);
                                    c.c_lt_s(12, 3);
                                    let t = !c.fcc();
                                    lw(c, V1, S1, mx::SLOTS);
                                    if t {
                                        break 'b_118240;
                                    } else {
                                        c.cycles += 3;
                                        fc(c, 0, 0x3f80_0000);
                                        c.add_s(12, 12, 0);
                                        break 'b_118240;
                                    }
                                }
                                'b_118378: {
                                    c.cycles += 4;
                                    lw(c, V0, V1, slot(13));
                                    c.g[V0][0] ^= 0x34;
                                    if c.g[V0][0] == 0 {
                                        lw(c, A0, S1, mx::SLOTS);
                                        break 'b_118378;
                                    } else {
                                        'b_118374: {
                                            'b_1182d4: {
                                                c.cycles += 9;
                                                lwc1(c, 1, V1, slot(13) + 8);
                                                lwc1(c, 0, V1, slot(13) + 4);
                                                fc(c, 2, 0x3f00_0000);
                                                c.div_s(3, 1, 0);
                                                c.sub_s(1, 3, 12);
                                                c.c_lt_s(2, 1);
                                                let t = !c.fcc();
                                                if t {
                                                    break 'b_1182d4;
                                                } else {
                                                    'b_1182b0: {
                                                        'b_1182a0: {
                                                            c.cycles += 9;
                                                            fc(c, 1, 0x3f80_0000);
                                                            c.lwc1(2, gpf(c, 0x7ad4));
                                                            c.sub_s(1, 3, 1);
                                                            c.add_s(0, 12, 2);
                                                            c.c_lt_s(0, 1);
                                                            if !c.fcc() {
                                                                c.sub_s(0, 12, 2);
                                                                break 'b_1182a0;
                                                            } else {
                                                                c.cycles += 2;
                                                                c.sub_s(12, 1, 2);
                                                                break 'b_1182b0;
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.c_lt_s(1, 0);
                                                        if c.fcc() {
                                                            c.add_s(12, 1, 2);
                                                            break 'b_1182b0;
                                                        } else {
                                                            break 'b_1182b0;
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.mtc1(0, 0);
                                                    c.c_lt_s(12, 0);
                                                    if !c.fcc() {
                                                        lw(c, A0, S1, mx::SLOTS);
                                                        break 'b_118378;
                                                    } else {
                                                        c.cycles += 4;
                                                        fc(c, 0, 0x3f80_0000);
                                                        c.add_s(12, 12, 0);
                                                        break 'b_118374;
                                                    }
                                                }
                                            }
                                            'b_118344: {
                                                c.cycles += 5;
                                                fc(c, 0, 0xbf00_0000);
                                                c.c_lt_s(1, 0);
                                                let t = !c.fcc();
                                                if t {
                                                    break 'b_118344;
                                                } else {
                                                    'b_118324: {
                                                        'b_118314: {
                                                            c.cycles += 9;
                                                            fc(c, 1, 0x3f80_0000);
                                                            c.lwc1(2, gpf(c, 0x7ad0));
                                                            c.add_s(1, 3, 1);
                                                            c.add_s(0, 12, 2);
                                                            c.c_lt_s(0, 1);
                                                            if !c.fcc() {
                                                                c.sub_s(0, 12, 2);
                                                                break 'b_118314;
                                                            } else {
                                                                c.cycles += 2;
                                                                c.sub_s(12, 1, 2);
                                                                break 'b_118324;
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.c_lt_s(1, 0);
                                                        if c.fcc() {
                                                            c.add_s(12, 1, 2);
                                                            break 'b_118324;
                                                        } else {
                                                            break 'b_118324;
                                                        }
                                                    }
                                                    c.cycles += 6;
                                                    fc(c, 0, 0x3f80_0000);
                                                    c.c_lt_s(0, 12);
                                                    if c.fcc() {
                                                        c.sub_s(12, 12, 0);
                                                        break 'b_118374;
                                                    } else {
                                                        c.cycles += 2;
                                                        lw(c, A0, S1, mx::SLOTS);
                                                        break 'b_118378;
                                                    }
                                                }
                                            }
                                            'b_118364: {
                                                c.cycles += 6;
                                                c.lwc1(1, gpf(c, 0x7acc));
                                                c.add_s(0, 12, 1);
                                                c.c_lt_s(0, 3);
                                                if !c.fcc() {
                                                    c.sub_s(0, 12, 1);
                                                    break 'b_118364;
                                                } else {
                                                    c.cycles += 2;
                                                    c.sub_s(12, 3, 1);
                                                    break 'b_118374;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.c_lt_s(3, 0);
                                            if c.fcc() {
                                                c.add_s(12, 3, 1);
                                                break 'b_118374;
                                            } else {
                                                break 'b_118374;
                                            }
                                        }
                                        c.cycles += 1;
                                        lw(c, A0, S1, mx::SLOTS);
                                        break 'b_118378;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A1][0] = 0xd;
                                c.g[A2][0] = 0;
                                c.g[A3][0] = 0;
                                addiu(c, A0, A0, slot(13));
                                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_838c)?;
                                break 'b_11838c;
                            }
                            // ---------------------------------------------------
                            // Sub-state 2 / replay: the combo counter's
                            // messages 0xc and 0xd.
                            'b_1183dc: {
                                c.cycles += 5;
                                lw(c, A0, S1, mx::RIDER);
                                callee::GET_SUB_STATE.call(c, 0x0011_8394)?;
                                c.g[V1][0] = 2;
                                if c.g[V0][0] != c.g[V1][0] {
                                    lw(c, V0, S1, mx::RIDER);
                                    break 'b_1183dc;
                                } else {
                                    'b_1183bc: {
                                        c.cycles += 3;
                                        lw(c, V0, SP, 0);
                                        let t = c.g[V0][0] == 0;
                                        c.g[A0][0] = c.g[S1][0];
                                        if t {
                                            break 'b_1183bc;
                                        } else {
                                            c.cycles += 4;
                                            remove_message(c, 0xc, 0x0011_83b4)?;
                                            lw(c, V1, S1, mx::SLOTS);
                                            break 'b_1188fc;
                                        }
                                    }
                                    c.cycles += 8;
                                    fc(c, 12, 0xbf80_0000);
                                    c.g[A1][0] = 0xc;
                                    c.g[A2][0] = 0;
                                    c.g[A3][0] = 0;
                                    callee::SHOW_HUD_NUMBER.call(c, 0x0011_83d4)?;
                                    lw(c, V1, S1, mx::SLOTS);
                                    break 'b_1188fc;
                                }
                            }
                            // The board-wake effect (vtable `rider+0x230`): clear
                            // messages 0xc/0xd when it reports a wake.
                            'b_11841c: {
                                c.cycles += 8;
                                c.g[A1][0] = c.g[SP][0] | 0x4;
                                sw_zero(c, SP, 0x4);
                                lw(c, A0, V0, rider::OWNER);
                                addiu(c, A0, A0, 0x230);
                                call_ptr(c, table::BOARD_WAKE_CLEAR, 0x0011_83f0)?;
                                lw(c, V0, SP, 0x4);
                                let t = c.g[V0][0] == 0;
                                c.mov_s(21, 0);
                                if t {
                                    break 'b_11841c;
                                } else {
                                    c.cycles += 8;
                                    c.g[A0][0] = c.g[S1][0];
                                    remove_message(c, 0xc, 0x0011_8408)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    remove_message(c, 0xd, 0x0011_8414)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    break 'b_118880;
                                }
                            }
                            // ---------------------------------------------------
                            // The wake/difficulty bars: pick the five message
                            // ids for the current peak-air value (f21).
                            'b_118534: {
                                'b_118468: {
                                    c.cycles += 17;
                                    fc(c, 12, 0xbf80_0000);
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A1][0] = 0xc;
                                    c.g[A2][0] = 0;
                                    c.g[A3][0] = 0;
                                    c.g[S2][0] = 0x34;
                                    callee::SHOW_HUD_NUMBER.call(c, 0x0011_843c)?;
                                    c.g[S3][0] = 0x34;
                                    c.lwc1(1, gpf(c, 0x7ac8));
                                    c.abs_s(0, 21);
                                    c.mov_s(25, 0);
                                    c.g[S4][0] = 0x34;
                                    c.c_lt_s(1, 0);
                                    let t = !c.fcc();
                                    c.g[S5][0] = 0x34;
                                    if t {
                                        break 'b_118468;
                                    } else {
                                        c.cycles += 2;
                                        c.g[S0][0] = 0x10;
                                        break 'b_118534;
                                    }
                                }
                                'b_118490: {
                                    c.cycles += 5;
                                    c.lwc1(0, gpf(c, 0x7ac4));
                                    c.c_lt_s(0, 21);
                                    let t = !c.fcc();
                                    c.g[S0][0] = 0x17;
                                    if t {
                                        break 'b_118490;
                                    } else {
                                        c.cycles += 5;
                                        c.g[S2][0] = 0x15;
                                        c.g[S3][0] = 0x16;
                                        c.g[S4][0] = 0x11;
                                        c.g[S5][0] = 0x10;
                                        break 'b_118534;
                                    }
                                }
                                c.cycles += 5;
                                c.lwc1(0, gpf(c, 0x7ac0));
                                c.c_lt_s(0, 21);
                                let t = c.fcc();
                                c.g[S0][0] = 0x16;
                                if t {
                                    break 'b_118534;
                                } else {
                                    'b_1184cc: {
                                        c.cycles += 5;
                                        c.lwc1(0, gpf(c, 0x7abc));
                                        c.c_lt_s(0, 21);
                                        let t = !c.fcc();
                                        c.g[S0][0] = 0x15;
                                        if t {
                                            break 'b_1184cc;
                                        } else {
                                            c.cycles += 5;
                                            c.g[S2][0] = 0x17;
                                            c.g[S3][0] = 0x16;
                                            c.g[S4][0] = 0x13;
                                            c.g[S5][0] = 0x14;
                                            break 'b_118534;
                                        }
                                    }
                                    'b_1184f4: {
                                        c.cycles += 5;
                                        c.lwc1(0, gpf(c, 0x7ab8));
                                        c.c_lt_s(21, 0);
                                        let t = !c.fcc();
                                        c.g[S0][0] = 0x11;
                                        if t {
                                            break 'b_1184f4;
                                        } else {
                                            c.cycles += 5;
                                            c.g[S2][0] = 0x13;
                                            c.g[S3][0] = 0x12;
                                            c.g[S4][0] = 0x17;
                                            c.g[S5][0] = 0x10;
                                            break 'b_118534;
                                        }
                                    }
                                    c.cycles += 5;
                                    c.lwc1(0, gpf(c, 0x7ab4));
                                    c.c_lt_s(21, 0);
                                    let t = c.fcc();
                                    c.g[S0][0] = 0x12;
                                    if t {
                                        break 'b_118534;
                                    } else {
                                        'b_118530: {
                                            c.cycles += 5;
                                            c.lwc1(0, gpf(c, 0x7ab0));
                                            c.c_lt_s(21, 0);
                                            let t = !c.fcc();
                                            c.g[S0][0] = 0x13;
                                            if t {
                                                break 'b_118530;
                                            } else {
                                                c.cycles += 5;
                                                c.g[S2][0] = 0x11;
                                                c.g[S3][0] = 0x12;
                                                c.g[S4][0] = 0x15;
                                                c.g[S5][0] = 0x14;
                                                break 'b_118534;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[S0][0] = 0x14;
                                        break 'b_118534;
                                    }
                                }
                            }
                            // The five bars: slots S0, S2..S5 of the message
                            // table, at the offset the board reports.
                            'b_1185b0: {
                                c.cycles += 24;
                                lw(c, V0, S1, mx::RIDER);
                                c.g[S6][0] = 0x9c;
                                fc(c, 26, 0x3f80_0000);
                                c.g[S7][0] = 0x34;
                                lw(c, A0, V0, rider::OWNER);
                                addiu(c, A0, A0, 0x230);
                                call_ptr(c, table::BOARD_WAKE_A, 0x0011_8554)?;
                                lw(c, V0, S1, mx::RIDER);
                                c.mov_s(24, 0);
                                lw(c, A0, V0, rider::OWNER);
                                addiu(c, A0, A0, 0x230);
                                call_ptr(c, table::BOARD_WAKE_B, 0x0011_8568)?;
                                c.mult(0, 2, 16, 22);
                                lw(c, A0, S1, mx::SLOTS);
                                c.mov_s(23, 0);
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A2][0] = 0;
                                c.mov_s(12, 26);
                                c.g[A3][0] = 0;
                                add_reg(c, A0, V0);
                                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_858c)?;
                                let t = c.g[S2][0] == c.g[S7][0];
                                c.mult(0, 2, 18, 22);
                                if t {
                                    break 'b_1185b0;
                                } else {
                                    c.cycles += 7;
                                    lw(c, A0, S1, mx::SLOTS);
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A2][0] = 0;
                                    c.mov_s(12, 24);
                                    c.g[A3][0] = 1;
                                    add_reg(c, A0, V0);
                                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_85b0)?;
                                    break 'b_1185b0;
                                }
                            }
                            'b_1185d4: {
                                c.cycles += 2;
                                let t = c.g[S3][0] == c.g[S7][0];
                                c.mult(0, 2, 19, 22);
                                if t {
                                    break 'b_1185d4;
                                } else {
                                    c.cycles += 7;
                                    lw(c, A0, S1, mx::SLOTS);
                                    c.g[A1][0] = c.g[S3][0];
                                    c.g[A2][0] = 0;
                                    c.mov_s(12, 24);
                                    c.g[A3][0] = 1;
                                    add_reg(c, A0, V0);
                                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_85d4)?;
                                    break 'b_1185d4;
                                }
                            }
                            'b_1185f8: {
                                c.cycles += 2;
                                let t = c.g[S4][0] == c.g[S7][0];
                                c.mult(0, 2, 20, 22);
                                if t {
                                    break 'b_1185f8;
                                } else {
                                    c.cycles += 7;
                                    lw(c, A0, S1, mx::SLOTS);
                                    c.g[A1][0] = c.g[S4][0];
                                    c.g[A2][0] = 0;
                                    c.mov_s(12, 23);
                                    c.g[A3][0] = 2;
                                    add_reg(c, A0, V0);
                                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_85f8)?;
                                    break 'b_1185f8;
                                }
                            }
                            'b_11861c: {
                                c.cycles += 2;
                                let t = c.g[S5][0] == c.g[S7][0];
                                c.mult(0, 2, 21, 22);
                                if t {
                                    break 'b_11861c;
                                } else {
                                    c.cycles += 7;
                                    lw(c, A0, S1, mx::SLOTS);
                                    c.g[A1][0] = c.g[S5][0];
                                    c.g[A2][0] = 0;
                                    c.mov_s(12, 23);
                                    c.g[A3][0] = 2;
                                    add_reg(c, A0, V0);
                                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_861c)?;
                                    break 'b_11861c;
                                }
                            }
                            // The fourth wake counter (`slot 14`).
                            'b_118634: {
                                c.cycles += 5;
                                lw(c, V1, S1, mx::SLOTS);
                                lw(c, V0, V1, slot(14));
                                c.g[V0][0] ^= 0x34;
                                let t = c.g[V0][0] == 0;
                                c.g[S2][0] = 0;
                                if t {
                                    break 'b_118634;
                                } else {
                                    c.cycles += 1;
                                    lw(c, S2, V1, slot(14) + 0xc);
                                    break 'b_118634;
                                }
                            }
                            // ---------------------------------------------------
                            // The four bar/percentage messages, chosen from the
                            // wake peak (f25 = |f21|).
                            c.cycles += 3;
                            lw(c, V0, SP, 0);
                            let t = c.g[V0][0] != 0;
                            c.g[S0][0] = 0;
                            if t {
                                break 'b_1188f8;
                            } else {
                                'b_118670: {
                                    c.cycles += 12;
                                    c.sub_s(2, 21, 27);
                                    c.lwc1(0, gpf(c, 0x7aac));
                                    fc(c, 1, 0x3f00_0000);
                                    c.mul_s(0, 2, 0);
                                    c.add_s(0, 0, 1);
                                    c.cvt_w_s(1, 0);
                                    c.cvt_s_w(1, 1);
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.sub_s(1, 1, 26);
                                        break 'b_118670;
                                    } else {
                                        break 'b_118670;
                                    }
                                }
                                'b_11886c: {
                                    'b_118714: {
                                        c.cycles += 10;
                                        c.lwc1(0, gpf(c, 0x7aa8));
                                        c.lwc1(22, gpf(c, 0x7aa4));
                                        c.mul_s(0, 1, 0);
                                        c.mtc1(0, 20);
                                        c.sub_s(2, 2, 0);
                                        c.abs_s(1, 2);
                                        c.c_lt_s(22, 1);
                                        let t = !c.fcc();
                                        c.g[A2][0] = 0x45_0000;
                                        if t {
                                            break 'b_118714;
                                        } else {
                                            'b_1186dc: {
                                                'b_1186d4: {
                                                    c.cycles += 11;
                                                    fc(c, 12, 0xbf80_0000);
                                                    addiu(c, A2, A2, 0x7820);
                                                    c.g[A0][0] = c.g[S1][0];
                                                    c.g[A1][0] = 0xe;
                                                    c.g[A3][0] = 1;
                                                    callee::SHOW_HUD_TEXT.call(c, 0x0011_86b4)?;
                                                    c.c_lt_s(22, 25);
                                                    let t = c.fcc();
                                                    c.g[S0][0] = 1;
                                                    if t {
                                                        break 'b_1186d4;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.lwc1(0, gpf(c, 0x7aa0));
                                                        c.c_lt_s(25, 0);
                                                        let t = !c.fcc();
                                                        if t {
                                                            break 'b_1186dc;
                                                        } else {
                                                            break 'b_1186d4;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                c.mov_s(20, 26);
                                                break 'b_11886c;
                                            }
                                            c.cycles += 6;
                                            c.lwc1(0, gpf(c, 0x7a9c));
                                            fc(c, 20, 0x4000_0000);
                                            c.c_lt_s(0, 25);
                                            let t = !c.fcc();
                                            if t {
                                                break 'b_11886c;
                                            } else {
                                                c.cycles += 4;
                                                c.lwc1(0, gpf(c, 0x7a98));
                                                c.c_lt_s(25, 0);
                                                let t = !c.fcc();
                                                if t {
                                                    break 'b_11886c;
                                                } else {
                                                    c.cycles += 4;
                                                    fc(c, 20, 0x40a0_0000);
                                                    break 'b_11886c;
                                                }
                                            }
                                        }
                                    }
                                    'b_1187e0: {
                                        c.cycles += 4;
                                        c.lwc1(0, gpf(c, 0x7a94));
                                        c.c_lt_s(0, 1);
                                        let t = !c.fcc();
                                        if t {
                                            break 'b_1187e0;
                                        } else {
                                            'b_1187a0: {
                                                'b_118760: {
                                                    'b_118744: {
                                                        c.cycles += 4;
                                                        c.mul_s(1, 2, 21);
                                                        c.c_lt_s(20, 1);
                                                        let t = !c.fcc();
                                                        if t {
                                                            break 'b_118744;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.lwc1(0, gpf(c, 0x7a90));
                                                            c.c_lt_s(0, 25);
                                                            let t = c.fcc();
                                                            if t {
                                                                break 'b_118760;
                                                            } else {
                                                                break 'b_118744;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 3;
                                                    c.c_lt_s(1, 20);
                                                    let t = !c.fcc();
                                                    if t {
                                                        break 'b_1187a0;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.lwc1(0, gpf(c, 0x7a8c));
                                                        c.c_lt_s(25, 0);
                                                        let t = !c.fcc();
                                                        if t {
                                                            break 'b_1187a0;
                                                        } else {
                                                            break 'b_118760;
                                                        }
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.lwc1(0, gpf(c, 0x7a88));
                                                c.c_lt_s(0, 23);
                                                let t = !c.fcc();
                                                c.g[A2][0] = 0x45_0000;
                                                if t {
                                                    break 'b_11886c;
                                                } else {
                                                    c.cycles += 11;
                                                    fc(c, 12, 0xbf80_0000);
                                                    addiu(c, A2, A2, 0x7830);
                                                    c.g[A0][0] = c.g[S1][0];
                                                    c.g[A1][0] = 0xe;
                                                    c.g[A3][0] = 2;
                                                    callee::SHOW_HUD_TEXT.call(c, 0x0011_8790)?;
                                                    fc(c, 20, 0x4040_0000);
                                                    c.g[S0][0] = 2;
                                                    break 'b_11886c;
                                                }
                                            }
                                            c.cycles += 5;
                                            c.lwc1(0, gpf(c, 0x7a84));
                                            c.c_lt_s(0, 24);
                                            let t = !c.fcc();
                                            c.g[A2][0] = 0x45_0000;
                                            if t {
                                                break 'b_11886c;
                                            } else {
                                                c.cycles += 11;
                                                fc(c, 12, 0xbf80_0000);
                                                addiu(c, A2, A2, 0x7840);
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[A1][0] = 0xe;
                                                c.g[A3][0] = 3;
                                                callee::SHOW_HUD_TEXT.call(c, 0x0011_87d0)?;
                                                fc(c, 20, 0x4000_0000);
                                                c.g[S0][0] = 3;
                                                break 'b_11886c;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.lwc1(0, gpf(c, 0x7a80));
                                    c.c_lt_s(0, 1);
                                    let t = !c.fcc();
                                    if t {
                                        break 'b_11886c;
                                    } else {
                                        'b_118844: {
                                            'b_11881c: {
                                                c.cycles += 6;
                                                c.lwc1(0, gpf(c, 0x7a7c));
                                                c.abs_s(1, 27);
                                                c.c_lt_s(1, 0);
                                                let t = c.fcc();
                                                c.g[A2][0] = 0x45_0000;
                                                if t {
                                                    break 'b_11881c;
                                                } else {
                                                    c.cycles += 5;
                                                    c.lwc1(0, gpf(c, 0x7a78));
                                                    c.c_lt_s(0, 1);
                                                    if !c.fcc() {
                                                        c.g[A2][0] = 0x45_0000;
                                                        break 'b_118844;
                                                    } else {
                                                        break 'b_11881c;
                                                    }
                                                }
                                            }
                                            c.cycles += 10;
                                            fc(c, 12, 0xbf80_0000);
                                            addiu(c, A2, A2, 0x7850);
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A1][0] = 0xe;
                                            callee::SHOW_HUD_TEXT.call(c, 0x0011_8838)?;
                                            c.mov_s(20, 26);
                                            c.g[S0][0] = 4;
                                            break 'b_11886c;
                                        }
                                        c.cycles += 10;
                                        fc(c, 12, 0xbf80_0000);
                                        addiu(c, A2, A2, 0x7860);
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A1][0] = 0xe;
                                        fc(c, 20, 0x4000_0000);
                                        c.g[A3][0] = 5;
                                        callee::SHOW_HUD_TEXT.call(c, 0x0011_8868)?;
                                        c.g[S0][0] = 5;
                                        break 'b_11886c;
                                    }
                                }
                                c.cycles += 5;
                                c.mtc1(0, 21);
                                c.c_eq_s(20, 21);
                                let t = !c.fcc();
                                c.g[A0][0] = c.g[S1][0];
                                if t {
                                    break 'b_11889c;
                                } else {
                                    break 'b_118880;
                                }
                            }
                        }
                        c.cycles += 7;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = 0xf;
                        callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_8888)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = 0xe;
                        callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_8894)?;
                        lw(c, V1, S1, mx::SLOTS);
                        break 'b_1188fc;
                    }
                    // -------------------------------------------------------
                    // The trick history: its number (message 0xf), the score
                    // scale, then the combo's bonus points (slot 2).
                    c.cycles += 13;
                    c.cvt_w_s(0, 20);
                    c.mfc1(A2, 0);
                    fc(c, 12, 0xbf80_0000);
                    c.g[A1][0] = 0xf;
                    c.g[A3][0] = 0;
                    callee::SHOW_HUD_NUMBER.call(c, 0x0011_88b8)?;
                    c.lwc1(0, gpf(c, 0x7a74));
                    lwc1(c, 1, S1, 0x1c);
                    c.mul_s(0, 20, 0);
                    c.add_s(1, 1, 0);
                    let t = c.g[S0][0] == c.g[S2][0];
                    swc1(c, S1, 0x1c, 1);
                    if t {
                        break 'b_1188f8;
                    } else {
                        c.cycles += 10;
                        c.g[A0][0] = c.g[S1][0];
                        callee::GET_TRICK_BONUS.call(c, 0x0011_88d8)?;
                        lw(c, A2, S1, mx::BONUS);
                        c.mov_s(12, 21);
                        lw(c, A0, S1, mx::SLOTS);
                        c.g[A3][0] = c.g[V0][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A1][0] = 2;
                        addiu(c, A0, A0, slot(2));
                        callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_88f8)?;
                        break 'b_1188f8;
                    }
                }
                c.cycles += 1;
                lw(c, V1, S1, mx::SLOTS);
                break 'b_1188fc;
            }
            // ---------------------------------------------------------------
            // The boost meter bar: slot 5's fraction (f4), slot 6's value.
            'b_11898c: {
                'b_118918: {
                    c.cycles += 4;
                    lw(c, V0, V1, slot(5));
                    c.g[V0][0] ^= 0x34;
                    let t = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[V1][0];
                    if t {
                        break 'b_118918;
                    } else {
                        c.cycles += 3;
                        lw(c, V0, S1, mx::RIDER);
                        lwc1(c, 4, V0, rider::BOOST_STRENGTH);
                        break 'b_11898c;
                    }
                }
                'b_11893c: {
                    'b_118938: {
                        c.cycles += 5;
                        lw(c, V0, S1, mx::RIDER);
                        c.g[A0][0] = 3;
                        lw(c, V1, V0, rider::BOOST_MODE);
                        let t = c.g[V1][0] == c.g[A0][0];
                        lwc1(c, 1, V0, rider::BOOST_STRENGTH);
                        if t {
                            break 'b_118938;
                        } else {
                            c.cycles += 3;
                            lwc1(c, 0, S1, mx::TRICK_BOOST);
                            c.add_s(3, 1, 0);
                            break 'b_11893c;
                        }
                    }
                    c.cycles += 1;
                    c.mov_s(3, 1);
                    break 'b_11893c;
                }
                'b_118988: {
                    'b_118974: {
                        c.cycles += 12;
                        lwc1(c, 0, A1, slot(5) + 4);
                        lwc1(c, 2, A1, slot(5) + 8);
                        fc(c, 1, 0x3f80_0000);
                        c.div_s(2, 2, 0);
                        c.lwc1(4, gpf(c, 0x7a70));
                        c.min_s(1, 3, 1);
                        c.add_s(0, 1, 4);
                        c.c_lt_s(0, 2);
                        if !c.fcc() {
                            c.sub_s(0, 1, 4);
                            break 'b_118974;
                        } else {
                            c.cycles += 2;
                            c.sub_s(12, 2, 4);
                            break 'b_118988;
                        }
                    }
                    c.cycles += 4;
                    c.c_lt_s(2, 0);
                    let t = !c.fcc();
                    c.mov_s(12, 1);
                    if t {
                        break 'b_118988;
                    } else {
                        c.cycles += 1;
                        c.add_s(12, 2, 4);
                        break 'b_118988;
                    }
                }
                c.cycles += 1;
                c.mov_s(4, 12);
                break 'b_11898c;
            }
            'b_1189f0: {
                'b_1189a8: {
                    c.cycles += 4;
                    lw(c, V0, A1, slot(6));
                    c.g[V0][0] ^= 0x34;
                    if c.g[V0][0] != 0 {
                        lwc1(c, 1, A1, slot(6) + 4);
                        break 'b_1189a8;
                    } else {
                        c.cycles += 3;
                        lw(c, V0, S1, mx::RIDER);
                        lwc1(c, 20, V0, rider::BOOST_STRENGTH);
                        break 'b_1189f0;
                    }
                }
                'b_1189ec: {
                    'b_1189d8: {
                        c.cycles += 10;
                        lwc1(c, 0, A1, slot(6) + 8);
                        lw(c, V0, S1, mx::RIDER);
                        c.div_s(2, 0, 1);
                        c.lwc1(3, gpf(c, 0x7a6c));
                        lwc1(c, 1, V0, rider::BOOST_STRENGTH);
                        c.add_s(0, 1, 3);
                        c.c_lt_s(0, 2);
                        if !c.fcc() {
                            c.sub_s(0, 1, 3);
                            break 'b_1189d8;
                        } else {
                            c.cycles += 2;
                            c.sub_s(12, 2, 3);
                            break 'b_1189ec;
                        }
                    }
                    c.cycles += 4;
                    c.c_lt_s(2, 0);
                    let t = !c.fcc();
                    c.mov_s(12, 1);
                    if t {
                        break 'b_1189ec;
                    } else {
                        c.cycles += 1;
                        c.add_s(12, 2, 3);
                        break 'b_1189ec;
                    }
                }
                c.cycles += 1;
                c.mov_s(20, 12);
                break 'b_1189f0;
            }
            // ---------------------------------------------------------------
            // The fourth counter (messages 5 and 6, slot 8).
            'b_118afc: {
                'b_118a4c: {
                    c.cycles += 18;
                    lw(c, A0, S1, mx::SLOTS);
                    c.mov_s(12, 4);
                    c.g[A1][0] = 5;
                    c.g[A2][0] = 0;
                    addiu(c, A0, A0, slot(5));
                    c.g[A3][0] = 0;
                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8a0c)?;
                    lw(c, A0, S1, mx::SLOTS);
                    c.mov_s(12, 20);
                    c.g[A1][0] = 6;
                    c.g[A2][0] = 0;
                    addiu(c, A0, A0, slot(6));
                    c.g[A3][0] = 0;
                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8a28)?;
                    lw(c, V0, S1, mx::RIDER);
                    lw(c, V1, V0, rider::BOOST_LEVEL);
                    if c.g[V1][0] != 0 {
                        lw(c, V1, S1, mx::SLOTS);
                        break 'b_118a4c;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = 8;
                        callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_8a44)?;
                        lw(c, A0, S1, mx::RIDER);
                        break 'b_118afc;
                    }
                }
                'b_118a70: {
                    c.cycles += 4;
                    lw(c, V0, V1, slot(8));
                    c.g[V0][0] ^= 0x34;
                    let t = c.g[V0][0] != 0;
                    addiu(c, A0, V1, slot(8));
                    if t {
                        break 'b_118a70;
                    } else {
                        c.cycles += 5;
                        c.mtc1(0, 12);
                        c.g[A1][0] = 8;
                        c.g[A2][0] = 0;
                        c.g[A3][0] = 0;
                        callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8a70)?;
                        break 'b_118a70;
                    }
                }
                // The boost level as a fraction of its maximum.
                'b_118ae8: {
                    'b_118ad4: {
                        c.cycles += 23;
                        lw(c, V1, S1, mx::RIDER);
                        c.g[A1][0] = 0xa;
                        lw(c, A0, S1, mx::SLOTS);
                        lw(c, V0, V1, rider::BOOST_LEVEL);
                        c.lwc1(4, gpf(c, 0x7a68));
                        c.g[V1][0] = ((c.g[V0][0] as i64) < 11) as u64;
                        lwc1(c, 1, A0, slot(8) + 8);
                        if c.g[V1][0] == 0 {
                            c.g[V0][0] = c.g[A1][0];
                        }
                        lwc1(c, 3, A0, slot(8) + 4);
                        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        fc(c, 2, 0x3f80_0000);
                        c.mtc1(7, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(5, gpf(c, 0x7a64));
                        c.div_s(3, 1, 3);
                        c.mul_s(0, 0, 4);
                        c.min_s(12, 0, 2);
                        c.add_s(1, 12, 5);
                        c.c_lt_s(1, 3);
                        if !c.fcc() {
                            c.sub_s(0, 12, 5);
                            break 'b_118ad4;
                        } else {
                            c.cycles += 2;
                            c.sub_s(12, 3, 5);
                            break 'b_118ae8;
                        }
                    }
                    'b_118ae4: {
                        c.cycles += 4;
                        c.c_lt_s(3, 0);
                        if c.fcc() {
                            c.add_s(12, 3, 5);
                            break 'b_118ae4;
                        } else {
                            break 'b_118ae4;
                        }
                    }
                    c.cycles += 1;
                    lw(c, A0, S1, mx::SLOTS);
                    break 'b_118ae8;
                }
                c.cycles += 5;
                c.g[A1][0] = 8;
                c.g[A2][0] = 0;
                addiu(c, A0, A0, slot(8));
                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8af8)?;
                lw(c, A0, S1, mx::RIDER);
                break 'b_118afc;
            }
            // ---------------------------------------------------------------
            // Trick-name banner slot 10 (messages 0xa) and its history.
            'b_118c30: {
                'b_118bfc: {
                    'b_118b24: {
                        c.cycles += 7;
                        c.g[A1][0] = 0xa;
                        lw(c, A3, S1, mx::TRICK_FLAG);
                        lw(c, V0, A0, rider::BOOST_LEVEL);
                        c.g[V1][0] = ((c.g[V0][0] as i64) < 11) as u64;
                        if c.g[V1][0] == 0 {
                            c.g[V0][0] = c.g[A1][0];
                        }
                        let t = c.g[A3][0] != 0;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        if t {
                            break 'b_118b24;
                        } else {
                            c.cycles += 3;
                            lw(c, V0, S1, mx::TRICK_COUNT);
                            if c.g[V0][0] == 0 {
                                lw(c, V1, S1, mx::SLOTS);
                                break 'b_118bfc;
                            } else {
                                break 'b_118b24;
                            }
                        }
                    }
                    c.cycles += 2;
                    if (c.g[A1][0] as i64) <= 0 {
                        lw(c, V1, S1, mx::SLOTS);
                        break 'b_118bfc;
                    } else {
                        'b_118b80: {
                            c.cycles += 10;
                            lw(c, S0, S1, mx::TRICK_COUNT);
                            lw(c, A0, S1, mx::SLOTS);
                            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                            if c.g[A3][0] != 0 {
                                c.g[S0][0] = c.g[V1][0];
                            }
                            lw(c, V0, A0, slot(10));
                            c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[S0][0] as i64)) as u64;
                            c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(slot(10)));
                            c.g[V0][0] ^= 0x34;
                            let t = c.g[V0][0] != 0;
                            if c.g[V1][0] != 0 {
                                c.g[S0][0] = c.g[A1][0];
                            }
                            if t {
                                break 'b_118b80;
                            } else {
                                c.cycles += 11;
                                c.g[A0][0] = c.g[A2][0];
                                c.mtc1(0, 12);
                                c.g[A1][0] = 0xa;
                                c.g[A2][0] = 0;
                                c.g[A3][0] = 0;
                                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8b6c)?;
                                call_ptr(c, table::AUDIO_SYSTEM, 0x0011_8b74)?;
                                lw(c, A1, S1, mx::RIDER);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, table::AUDIO_CALL_A, 0x0011_8b80)?;
                                break 'b_118b80;
                            }
                        }
                        // The trick-name timer (slot 10's fraction).
                        'b_118bdc: {
                            'b_118bcc: {
                                c.cycles += 17;
                                lw(c, V0, S1, mx::SLOTS);
                                c.mtc1(16, 1);
                                c.cvt_s_w(1, 1);
                                c.lwc1(0, gpf(c, 0x7a60));
                                lwc1(c, 4, V0, slot(10) + 4);
                                lwc1(c, 3, V0, slot(10) + 8);
                                c.mul_s(1, 1, 0);
                                fc(c, 2, 0x3f80_0000);
                                c.div_s(3, 3, 4);
                                c.lwc1(4, gpf(c, 0x7a5c));
                                c.min_s(12, 1, 2);
                                c.add_s(0, 12, 4);
                                c.c_lt_s(0, 3);
                                if !c.fcc() {
                                    c.sub_s(0, 12, 4);
                                    break 'b_118bcc;
                                } else {
                                    c.cycles += 2;
                                    c.sub_s(12, 3, 4);
                                    break 'b_118bdc;
                                }
                            }
                            c.cycles += 4;
                            c.c_lt_s(3, 0);
                            if c.fcc() {
                                c.add_s(12, 3, 4);
                                break 'b_118bdc;
                            } else {
                                break 'b_118bdc;
                            }
                        }
                        c.cycles += 8;
                        lw(c, A0, S1, mx::SLOTS);
                        c.g[A3][0] = c.g[S0][0];
                        c.g[A1][0] = 0xa;
                        c.g[A2][0] = 0;
                        addiu(c, A0, A0, slot(10));
                        callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8bf4)?;
                        lw(c, V1, S1, mx::SLOTS);
                        break 'b_118c30;
                    }
                }
                'b_118c24: {
                    c.cycles += 4;
                    lw(c, V0, V1, slot(10));
                    c.g[V0][0] ^= 0x34;
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = c.g[S1][0];
                        break 'b_118c24;
                    } else {
                        c.cycles += 6;
                        call_ptr(c, table::AUDIO_SYSTEM, 0x0011_8c14)?;
                        lw(c, A1, S1, mx::RIDER);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, table::AUDIO_CALL_B, 0x0011_8c20)?;
                        c.g[A0][0] = c.g[S1][0];
                        break 'b_118c24;
                    }
                }
                c.cycles += 3;
                c.g[A1][0] = 0xa;
                callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_8c2c)?;
                lw(c, V1, S1, mx::SLOTS);
                break 'b_118c30;
            }
            // The board-wake meter: slot 9 (message 9) and its fraction.
            'b_118d54: {
                'b_118d50: {
                    'b_118c84: {
                        c.cycles += 4;
                        lw(c, V0, V1, slot(9));
                        c.g[V0][0] ^= 0x34;
                        let t = c.g[V0][0] != 0;
                        addiu(c, A0, V1, slot(9));
                        if t {
                            break 'b_118c84;
                        } else {
                            c.cycles += 7;
                            lw(c, V0, S1, mx::RIDER);
                            c.mtc1(0, 12);
                            lwc1(c, 0, V0, rider::BOOST_METER);
                            c.c_lt_s(12, 0);
                            let t = !c.fcc();
                            c.g[A1][0] = 9;
                            if t {
                                break 'b_118d50;
                            } else {
                                c.cycles += 10;
                                c.g[A2][0] = 0;
                                c.g[A3][0] = 0;
                                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8c68)?;
                                call_ptr(c, table::AUDIO_SYSTEM, 0x0011_8c70)?;
                                lw(c, A1, S1, mx::RIDER);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, table::AUDIO_CALL_C, 0x0011_8c7c)?;
                                lw(c, A0, S1, mx::SLOTS);
                                break 'b_118d54;
                            }
                        }
                    }
                    'b_118ce4: {
                        c.cycles += 9;
                        lwc1(c, 0, V1, slot(9) + 4);
                        lwc1(c, 1, V1, slot(9) + 8);
                        fc(c, 2, 0x3f80_0000);
                        c.div_s(3, 1, 0);
                        c.c_eq_s(3, 2);
                        let t = !c.fcc();
                        lw(c, V1, S1, mx::RIDER);
                        if t {
                            break 'b_118ce4;
                        } else {
                            c.cycles += 5;
                            lwc1(c, 1, V1, rider::BOOST_METER);
                            c.mtc1(0, 0);
                            c.c_eq_s(1, 0);
                            let t = !c.fcc();
                            if t {
                                break 'b_118ce4;
                            } else {
                                c.cycles += 10;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = 9;
                                callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_8cc8)?;
                                call_ptr(c, table::AUDIO_SYSTEM, 0x0011_8cd0)?;
                                lw(c, A1, S1, mx::RIDER);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, table::AUDIO_CALL_D, 0x0011_8cdc)?;
                                lw(c, A0, S1, mx::SLOTS);
                                break 'b_118d54;
                            }
                        }
                    }
                    'b_118d34: {
                        'b_118d20: {
                            c.cycles += 13;
                            c.lwc1(1, gpf(c, 0x7a58));
                            lwc1(c, 0, V1, rider::BOOST_METER);
                            fc(c, 2, 0x3f80_0000);
                            c.mul_s(0, 0, 1);
                            lw(c, V0, S1, mx::SLOTS);
                            c.lwc1(1, gpf(c, 0x7a54));
                            c.sub_s(12, 2, 0);
                            c.add_s(0, 12, 1);
                            c.c_lt_s(0, 3);
                            let t = !c.fcc();
                            addiu(c, A0, V0, slot(9));
                            if t {
                                break 'b_118d20;
                            } else {
                                c.cycles += 2;
                                c.sub_s(12, 3, 1);
                                break 'b_118d34;
                            }
                        }
                        c.cycles += 5;
                        c.sub_s(0, 12, 1);
                        c.c_lt_s(3, 0);
                        if c.fcc() {
                            c.add_s(12, 3, 1);
                            break 'b_118d34;
                        } else {
                            break 'b_118d34;
                        }
                    }
                    c.cycles += 7;
                    lw(c, A3, V1, rider::BOOST_LEVEL);
                    c.g[A1][0] = 9;
                    c.g[V1][0] = 0xb;
                    c.g[A2][0] = 0;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < 12) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A3][0] = c.g[V1][0];
                    }
                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8d50)?;
                    break 'b_118d50;
                }
                c.cycles += 1;
                lw(c, A0, S1, mx::SLOTS);
                break 'b_118d54;
            }
            // ---------------------------------------------------------------
            // The combo timer's slot 3 (message 3, its count at +0x10) and
            // slot 4's fraction.
            'b_118e30: {
                c.cycles += 11;
                c.g[A1][0] = 7;
                lw(c, A2, S1, 0x198);
                c.g[A3][0] = 0;
                c.mtc1(0, 12);
                addiu(c, A0, A0, slot(7));
                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8d6c)?;
                lw(c, V1, S1, mx::SLOTS);
                lw(c, V0, V1, slot(3));
                c.g[V0][0] ^= 0x34;
                let t = c.g[V0][0] == 0;
                addiu(c, A0, V1, slot(3));
                if t {
                    break 'b_118e30;
                } else {
                    'b_118db4: {
                        c.cycles += 9;
                        lw(c, A3, S1, mx::COMBO_POINTS);
                        lwc1(c, 0, V1, slot(3) + 4);
                        lw(c, A1, V1, slot(3) + 0x14);
                        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x46));
                        lwc1(c, 1, V1, slot(3) + 8);
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        c.div_s(1, 1, 0);
                        let t = c.g[V0][0] != 0;
                        c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x46));
                        if t {
                            break 'b_118db4;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x46));
                            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x46));
                            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                c.g[A2][0] = c.g[A3][0];
                            }
                            break 'b_118db4;
                        }
                    }
                    'b_118dec: {
                        'b_118dd0: {
                            c.cycles += 5;
                            c.lwc1(0, gpf(c, 0x7a50));
                            c.lwc1(2, gpf(c, 0x7a4c));
                            c.c_lt_s(0, 1);
                            let t = !c.fcc();
                            if t {
                                break 'b_118dd0;
                            } else {
                                c.cycles += 2;
                                c.sub_s(12, 1, 2);
                                break 'b_118dec;
                            }
                        }
                        c.cycles += 7;
                        c.lwc1(0, gpf(c, 0x7a48));
                        fc(c, 12, 0x3f80_0000);
                        c.c_lt_s(1, 0);
                        if c.fcc() {
                            c.add_s(12, 1, 2);
                            break 'b_118dec;
                        } else {
                            break 'b_118dec;
                        }
                    }
                    'b_118e1c: {
                        c.cycles += 9;
                        lw(c, A3, S1, mx::COMBO_COUNT);
                        c.g[A1][0] = 3;
                        callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8df8)?;
                        lwc1(c, 1, S1, mx::COMBO_TIMER);
                        c.mtc1(0, 0);
                        c.c_lt_s(1, 0);
                        let t = !c.fcc();
                        c.g[V0][0] = 4;
                        if t {
                            break 'b_118e1c;
                        } else {
                            c.cycles += 3;
                            lw(c, V1, S1, mx::SLOTS);
                            sw(c, V1, slot(3) + 0x10, V0);
                            break 'b_118e30;
                        }
                    }
                    c.cycles += 5;
                    c.lwc1(0, gpf(c, 0x7a44));
                    lw(c, V0, S1, mx::SLOTS);
                    c.mul_s(0, 1, 0);
                    c.cvt_w_s(1, 0);
                    swc1(c, V0, slot(3) + 0x10, 1);
                    break 'b_118e30;
                }
            }
            // ---------------------------------------------------------------
            // The combo's bonus number (slot 2 / message 2): the slot's
            // fraction is its own progress, else the new bonus.
            'b_118ee0: {
                'b_118ecc: {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S1][0];
                    callee::GET_TRICK_BONUS.call(c, 0x0011_8e38)?;
                    lw(c, A1, S1, mx::SLOTS);
                    c.g[A3][0] = c.g[V0][0];
                    lw(c, T0, S1, mx::BONUS);
                    lw(c, V0, A1, slot(2));
                    addiu(c, A0, A1, slot(2));
                    c.g[V0][0] ^= 0x34;
                    let t = c.g[V0][0] == 0;
                    c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    if t {
                        break 'b_118ecc;
                    } else {
                        'b_118ebc: {
                            'b_118eb8: {
                                c.cycles += 5;
                                lw(c, V0, A1, slot(2) + 0x14);
                                lw(c, V1, A1, slot(2) + 0xc);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                                let t = c.g[V0][0] != c.g[T0][0];
                                if t {
                                    break 'b_118eb8;
                                } else {
                                    'b_118e94: {
                                        c.cycles += 8;
                                        lwc1(c, 0, A1, slot(2) + 4);
                                        lwc1(c, 1, A1, slot(2) + 8);
                                        c.lwc1(2, gpf(c, 0x7a40));
                                        c.div_s(1, 1, 0);
                                        c.lwc1(3, gpf(c, 0x7a3c));
                                        c.c_lt_s(2, 1);
                                        let t = !c.fcc();
                                        if t {
                                            break 'b_118e94;
                                        } else {
                                            c.cycles += 2;
                                            c.sub_s(12, 1, 3);
                                            break 'b_118ebc;
                                        }
                                    }
                                    c.cycles += 7;
                                    c.lwc1(0, gpf(c, 0x7a38));
                                    fc(c, 12, 0x3f80_0000);
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.add_s(12, 1, 3);
                                        break 'b_118ebc;
                                    } else {
                                        c.cycles += 2;
                                        break 'b_118ebc;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.mtc1(0, 12);
                            break 'b_118ebc;
                        }
                        c.cycles += 4;
                        c.g[A1][0] = 2;
                        callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8ec4)?;
                        break 'b_118ee0;
                    }
                }
                c.cycles += 2;
                if (c.g[A2][0] as i64) <= 0 {
                    break 'b_118ee0;
                } else {
                    c.cycles += 3;
                    c.mtc1(0, 12);
                    c.g[A1][0] = 2;
                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8ee0)?;
                    break 'b_118ee0;
                }
            }
            // ---------------------------------------------------------------
            // The "special" message: the BE interface's name goes to slot 25
            // (message 0x19), slot 4's fraction to message 4.
            'b_118fa8: {
                c.cycles += 28;
                call_ptr(c, table::BE_GET_BE, 0x0011_8ee8)?;
                c.g[A1][0] = 0xb;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, table::BE_GET_INTERFACE, 0x0011_8ef4)?;
                lw(c, A0, S1, mx::RIDER);
                c.g[S0][0] = c.g[V0][0];
                lw(c, V1, A0, rider::BOARD);
                addiu(c, A0, A0, rider::BOARD);
                c.g[A1][0] = c.lh(at(c, V1, 0x38));
                lw(c, V0, V1, 0x3c);
                let t = c.g[V0][0] as u32;
                addiu(c, A0, A0, c.g[A1][0] as u32);
                call_ptr(c, t, 0x0011_8f14)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, table::BE_CALL, 0x0011_8f20)?;
                lw(c, A0, S1, mx::SLOTS);
                c.g[A2][0] = c.g[V0][0];
                c.mtc1(0, 12);
                c.g[A1][0] = 0x19;
                addiu(c, A0, A0, slot(25));
                c.g[A3][0] = 0;
                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8f3c)?;
                lw(c, A0, S1, mx::SLOTS);
                lw(c, V0, A0, slot(4));
                c.g[V0][0] ^= 0x34;
                let t = c.g[V0][0] == 0;
                addiu(c, V1, A0, slot(4));
                if t {
                    break 'b_118fa8;
                } else {
                    'b_118f98: {
                        'b_118f7c: {
                            c.cycles += 9;
                            lwc1(c, 0, A0, slot(4) + 4);
                            lwc1(c, 1, A0, slot(4) + 8);
                            c.lwc1(2, gpf(c, 0x7a34));
                            c.div_s(1, 1, 0);
                            c.lwc1(3, gpf(c, 0x7a30));
                            c.c_lt_s(2, 1);
                            let t = !c.fcc();
                            lw(c, A2, A0, slot(4) + 0x14);
                            if t {
                                break 'b_118f7c;
                            } else {
                                c.cycles += 2;
                                c.sub_s(12, 1, 3);
                                break 'b_118f98;
                            }
                        }
                        c.cycles += 7;
                        c.lwc1(0, gpf(c, 0x7a2c));
                        fc(c, 12, 0x3f80_0000);
                        c.c_lt_s(1, 0);
                        if c.fcc() {
                            c.add_s(12, 1, 3);
                            break 'b_118f98;
                        } else {
                            break 'b_118f98;
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[V1][0];
                    c.g[A1][0] = 4;
                    c.g[A3][0] = 0;
                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8fa8)?;
                    break 'b_118fa8;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x90);
        }
        c.cycles += 18;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 27, 0xd8);
        frame.restore_fpr(c, 26, 0xd0);
        frame.restore_fpr(c, 25, 0xc8);
        frame.restore_fpr(c, 24, 0xc0);
        frame.restore_fpr(c, 23, 0xb8);
        frame.restore_fpr(c, 22, 0xb0);
        frame.restore_fpr(c, 21, 0xa8);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cSpeedBoostStreamerFX_UpdatePoints.
//
// The original is a dispatcher (it has an `__entries` twin): most of its blocks
// are separate resume points. Per docs/investigation/phase2-overrides.md the
// override stub sends every entry other than the function start to the
// generated twin, so this replacement handles the start only (`pc` is left at
// the loop head when an event asks to unwind, and the stub resumes there).
// The VU0 work is issued as the original's own COP2 macro ops.

/// `void cSpeedBoostStreamerFX_UpdatePoints(cSpeedBoostStreamerFX* self)`
/// (0x2e6c08): one step of the speed-boost wake particle streamer. It
/// integrates the point buffers from the rider's velocity and board, advances
/// the two wake shapes, and fades the timer at `+0x18`.
pub fn c_speed_boost_streamer_fx_update_points(c: &mut Cpu) {
    guest(c, |c| {
        'b_2e7950: {
            'b_2e6c4c: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x150));
                c.mtc1(0, 1);
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.g[S0][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x120), c.g[RA][0]);
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                    break 'b_2e6c4c;
                } else {
                    c.cycles += 5;
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x18));
                    c.c_lt_s(1, 0);
                    if !c.fcc() {
                        c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x4));
                        break 'b_2e7950;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                        break 'b_2e6c4c;
                    }
                }
            }
            'b_2e794c: {
                c.cycles += 14;
                c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x1e0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 0);
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[V0][0] = 0x1e;
                if t {
                    break 'b_2e794c;
                } else {
                    'b_2e6cdc: {
                        'b_2e6cd4: {
                            c.cycles += 3;
                            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            let t = c.g[A3][0] != c.g[V0][0];
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                            if t {
                                break 'b_2e6cd4;
                            } else {
                                'b_2e6cb4: {
                                    c.cycles += 9;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                                    c.g[A2][0] = 0x1c;
                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                                    c.g[A1][0] = 0x1e;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1d));
                                    c.g[S1][0] = c.g[A0][0];
                                    c.div(0, 3, 7);
                                    if c.g[A1][0] == 0u64 {
                                        break 'b_2e6cb4;
                                    } else {
                                        break 'b_2e6cb4;
                                    }
                                }
                                c.cycles += 8;
                                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x18));
                                c.g[V0][0] = c.hi[0];
                                c.mult(0, 2, 2, 6);
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.lwc1(0, (c.g[A0][0] as u32));
                                c.sub_s(1, 1, 0);
                                let t = true;
                                c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x18));
                                if t {
                                    break 'b_2e6cdc;
                                } else {
                                    break 'b_2e6cd4;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                        break 'b_2e6cdc;
                    }
                    'b_2e6cec: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = 0x1e;
                        if c.g[V1][0] == 0u64 {
                            break 'b_2e6cec;
                        } else {
                            break 'b_2e6cec;
                        }
                    }
                    'b_2e6db0: {
                        c.cycles += 38;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1d));
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                        c.div(0, 2, 3);
                        c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c0));
                        c.g[A0][0] = c.hi[0];
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[A0][0]);
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x6c0));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x002e_6d18)?;
                        c.lq(3, (c.g[V0][0] as u32));
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                        c.sq(3, (c.g[SP][0] as u32));
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8a4));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8a4));
                        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x20));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8a4));
                        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.lq(2, (c.g[A0][0] as u32));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x320));
                        if c.g[V1][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            break 'b_2e6db0;
                        } else {
                            c.cycles += 11;
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 0);
                            c.mfc1(3, 0);
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            break 'b_2e6db0;
                        }
                    }
                    'b_2e6e08: {
                        'b_2e6e04: {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                                break 'b_2e6e04;
                            } else {
                                c.cycles += 18;
                                c.lqc2(3, (c.g[SP][0] as u32));
                                c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(0x20));
                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                c.vu0().sqrt(4, 0);
                                c.vu0().waitq();
                                c.cfc2(3, 22);
                                c.mtc1(3, 3);
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_2e6e08;
                                } else {
                                    break 'b_2e6e04;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.mtc1(0, 3);
                        break 'b_2e6e08;
                    }
                    'b_2e6e40: {
                        c.cycles += 14;
                        c.g[A1][0] = 0x1c;
                        c.lq(2, (c.g[SP][0] as u32));
                        c.g[A0][0] = 0x1e;
                        c.mult(0, 6, 3, 5);
                        c.mtc1(0, 4);
                        c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
                        c.g[AT][0] = 0x42960000;
                        c.mtc1(1, 1);
                        c.g[AT][0] = 0x418c0000;
                        c.mtc1(1, 2);
                        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                        c.swc1(4, (c.g[V1][0] as u32));
                        if c.g[A0][0] == 0u64 {
                            break 'b_2e6e40;
                        } else {
                            break 'b_2e6e40;
                        }
                    }
                    'b_2e7060: {
                        c.cycles += 118;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.div(0, 2, 4);
                        c.g[V1][0] = c.hi[0];
                        c.mult(0, 2, 3, 5);
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                        c.swc1(3, (c.g[A1][0] as u32));
                        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x18));
                        c.add_s(0, 0, 3);
                        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x18));
                        c.mfc1(3, 1);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.mfc1(3, 2);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.mfc1(3, 1);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.mfc1(3, 2);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.mfc1(3, 1);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.mfc1(3, 2);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.mfc1(3, 1);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.mfc1(3, 2);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x330));
                        let t = c.g[V1][0] == 0u64;

                        if t {
                            break 'b_2e7060;
                        } else {
                            'b_2e7058: {
                                'b_2e7048: {
                                    c.cycles += 8;
                                    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x784));
                                    c.g[A1][0] = 0x2;
                                    call_ptr(c, table::PLAY_ANIM, 0x002e_7024)?;
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x17));
                                    c.g[V0][0] = (c.g[V0][0] < 0x6u64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = 0x25;
                                    if t {
                                        break 'b_2e7048;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x26;
                                        if t {
                                            break 'b_2e7048;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];

                                            if t {
                                                break 'b_2e7058;
                                            } else {
                                                break 'b_2e7048;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 4);
                                let t = true;

                                if t {
                                    break 'b_2e7060;
                                } else {
                                    break 'b_2e7058;
                                }
                            }
                            c.cycles += 2;
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 4);
                            break 'b_2e7060;
                        }
                    }
                    'b_2e71bc: {
                        'b_2e7110: {
                            c.cycles += 4;
                            c.mtc1(0, 0);
                            c.c_lt_s(0, 4);
                            let t = !c.fcc();

                            if t {
                                break 'b_2e7110;
                            } else {
                                c.cycles += 40;
                                c.g[AT][0] = 0x43160000;
                                c.mtc1(1, 0);
                                c.mfc1(3, 0);
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.qmtc2(3, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                                c.mfc1(3, 4);
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                                c.qmtc2(3, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.mfc1(3, 0);
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.qmtc2(3, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                                c.mfc1(3, 4);
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                                c.qmtc2(3, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                if t {
                                    break 'b_2e71bc;
                                } else {
                                    break 'b_2e7110;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.c_lt_s(4, 0);
                        if !c.fcc() {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                            break 'b_2e71bc;
                        } else {
                            c.cycles += 39;
                            c.g[AT][0] = 0x43160000;
                            c.mtc1(1, 0);
                            c.mfc1(3, 0);
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.mfc1(3, 4);
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.mfc1(3, 0);
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.mfc1(3, 4);
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                            break 'b_2e71bc;
                        }
                    }
                    'b_2e7240: {
                        c.cycles += 22;
                        c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x1e0));
                        c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x370));
                        c.vu0().opmula(4, 5);
                        c.vu0().opmsub(6, 5, 4);
                        c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                        c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.mtc1(0, 1);
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lqc2(5, (c.g[S0][0] as u32).wrapping_add(0x40));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.qmfc2(3, 4);
                        c.mtc1(3, 0);
                        c.c_lt_s(0, 1);
                        let t = !c.fcc();
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        if t {
                            break 'b_2e7240;
                        } else {
                            c.cycles += 11;
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 0);
                            c.mfc1(3, 0);
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                            break 'b_2e7240;
                        }
                    }
                    'b_2e7464: {
                        'b_2e73f0: {
                            'b_2e73d8: {
                                'b_2e73a0: {
                                    'b_2e7380: {
                                        'b_2e7374: {
                                            c.cycles += 72;
                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                            c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x40));
                                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                            c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x460));
                                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x460));
                                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                                            c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x460));
                                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                            c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x460));
                                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x90));
                                            c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(3, 4);
                                            c.mtc1(3, 8);
                                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                            c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(2, 4);
                                            c.mtc1(2, 6);
                                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                            c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(3, 4);
                                            c.mtc1(3, 7);
                                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                            c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(2, 4);
                                            c.mtc1(2, 5);
                                            c.abs_s(2, 7);
                                            c.abs_s(1, 5);
                                            c.c_lt_s(1, 2);
                                            c.abs_s(4, 8);
                                            c.abs_s(0, 6);
                                            let t = !c.fcc();

                                            if t {
                                                break 'b_2e7374;
                                            } else {
                                                c.cycles += 3;
                                                c.c_lt_s(2, 0);
                                                let t = c.fcc();

                                                if t {
                                                    break 'b_2e7380;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;

                                                    if t {
                                                        break 'b_2e73a0;
                                                    } else {
                                                        break 'b_2e7374;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.c_lt_s(1, 0);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_2e73a0;
                                        } else {
                                            break 'b_2e7380;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.abs_s(0, 6);
                                    c.c_lt_s(0, 4);
                                    let t = c.fcc();

                                    if t {
                                        break 'b_2e73d8;
                                    } else {
                                        c.cycles += 4;
                                        c.abs_s(3, 7);
                                        c.abs_s(2, 5);
                                        let t = true;

                                        if t {
                                            break 'b_2e73f0;
                                        } else {
                                            break 'b_2e73a0;
                                        }
                                    }
                                }
                                'b_2e73cc: {
                                    c.cycles += 6;
                                    c.abs_s(0, 7);
                                    c.abs_s(1, 5);
                                    c.c_lt_s(1, 0);
                                    c.mov_s(3, 0);
                                    let t = !c.fcc();
                                    c.mov_s(2, 1);
                                    if t {
                                        break 'b_2e73cc;
                                    } else {
                                        c.cycles += 3;
                                        c.c_lt_s(3, 4);
                                        let t = c.fcc();

                                        if t {
                                            break 'b_2e73d8;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;

                                            if t {
                                                break 'b_2e73f0;
                                            } else {
                                                break 'b_2e73cc;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.c_lt_s(2, 4);
                                let t = !c.fcc();

                                if t {
                                    break 'b_2e73f0;
                                } else {
                                    break 'b_2e73d8;
                                }
                            }
                            c.cycles += 6;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 1);
                            c.abs_s(0, 8);
                            c.div_s(0, 1, 0);
                            let t = true;
                            c.mul_s(2, 8, 0);
                            if t {
                                break 'b_2e7464;
                            } else {
                                break 'b_2e73f0;
                            }
                        }
                        'b_2e7434: {
                            'b_2e7420: {
                                'b_2e7414: {
                                    c.cycles += 4;
                                    c.c_lt_s(2, 3);
                                    c.abs_s(1, 6);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_2e7414;
                                    } else {
                                        c.cycles += 3;
                                        c.c_lt_s(3, 1);
                                        let t = c.fcc();

                                        if t {
                                            break 'b_2e7420;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;

                                            if t {
                                                break 'b_2e7434;
                                            } else {
                                                break 'b_2e7414;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.c_lt_s(2, 1);
                                let t = !c.fcc();

                                if t {
                                    break 'b_2e7434;
                                } else {
                                    break 'b_2e7420;
                                }
                            }
                            c.cycles += 5;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.div_s(0, 0, 1);
                            let t = true;
                            c.mul_s(2, 8, 0);
                            if t {
                                break 'b_2e7464;
                            } else {
                                break 'b_2e7434;
                            }
                        }
                        'b_2e7454: {
                            c.cycles += 3;
                            c.c_lt_s(2, 3);
                            let t = !c.fcc();

                            if t {
                                break 'b_2e7454;
                            } else {
                                c.cycles += 5;
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 0);
                                c.div_s(0, 0, 3);
                                let t = true;
                                c.mul_s(2, 8, 0);
                                if t {
                                    break 'b_2e7464;
                                } else {
                                    break 'b_2e7454;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.div_s(0, 0, 2);
                        c.mul_s(2, 8, 0);
                        break 'b_2e7464;
                    }
                    'b_2e7698: {
                        'b_2e750c: {
                            c.cycles += 6;
                            c.mul_s(3, 6, 0);
                            c.mul_s(5, 5, 0);
                            c.c_lt_s(2, 3);
                            let t = !c.fcc();
                            c.mul_s(4, 7, 0);
                            if t {
                                break 'b_2e750c;
                            } else {
                                c.cycles += 6;
                                c.g[AT][0] = 0x3f000000;
                                c.mtc1(1, 1);
                                c.add_s(0, 4, 1);
                                c.c_lt_s(2, 0);
                                let t = !c.fcc();

                                if t {
                                    break 'b_2e750c;
                                } else {
                                    c.cycles += 4;
                                    c.add_s(0, 5, 1);
                                    c.c_lt_s(2, 0);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_2e750c;
                                    } else {
                                        c.cycles += 26;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                        c.g[AT][0] = 0x41200000;
                                        c.mtc1(1, 1);
                                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.sub_s(0, 0, 1);
                                        c.mfc1(4, 0);
                                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.qmtc2(4, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        let t = true;
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        if t {
                                            break 'b_2e7698;
                                        } else {
                                            break 'b_2e750c;
                                        }
                                    }
                                }
                            }
                        }
                        'b_2e75a8: {
                            c.cycles += 3;
                            c.c_lt_s(3, 2);
                            let t = !c.fcc();

                            if t {
                                break 'b_2e75a8;
                            } else {
                                c.cycles += 6;
                                c.g[AT][0] = 0x3f000000;
                                c.mtc1(1, 1);
                                c.add_s(0, 4, 1);
                                c.c_lt_s(3, 0);
                                let t = !c.fcc();

                                if t {
                                    break 'b_2e75a8;
                                } else {
                                    c.cycles += 4;
                                    c.add_s(0, 5, 1);
                                    c.c_lt_s(3, 0);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_2e75a8;
                                    } else {
                                        c.cycles += 26;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                        c.g[AT][0] = 0x41200000;
                                        c.mtc1(1, 1);
                                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.sub_s(0, 0, 1);
                                        c.mfc1(4, 0);
                                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.qmtc2(4, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        let t = true;
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        if t {
                                            break 'b_2e7698;
                                        } else {
                                            break 'b_2e75a8;
                                        }
                                    }
                                }
                            }
                        }
                        'b_2e7638: {
                            c.cycles += 4;
                            c.c_lt_s(4, 2);
                            if !c.fcc() {
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                break 'b_2e7638;
                            } else {
                                c.cycles += 4;
                                c.c_lt_s(4, 3);
                                let t = !c.fcc();
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                if t {
                                    break 'b_2e7638;
                                } else {
                                    c.cycles += 3;
                                    c.c_lt_s(4, 5);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_2e7638;
                                    } else {
                                        c.cycles += 25;
                                        c.g[AT][0] = 0x41200000;
                                        c.mtc1(1, 1);
                                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.sub_s(0, 0, 1);
                                        c.mfc1(4, 0);
                                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.qmtc2(4, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        let t = true;
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        if t {
                                            break 'b_2e7698;
                                        } else {
                                            break 'b_2e7638;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 24;
                        c.g[AT][0] = 0x41200000;
                        c.mtc1(1, 1);
                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.qmfc2(2, 4);
                        c.mtc1(2, 0);
                        c.sub_s(0, 0, 1);
                        c.mfc1(4, 0);
                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                        c.qmtc2(4, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                        break 'b_2e7698;
                    }
                    'b_2e78c8: {
                        'b_2e773c: {
                            c.cycles += 5;
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                            c.c_le_s(3, 2);
                            let t = !c.fcc();
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                            if t {
                                break 'b_2e773c;
                            } else {
                                c.cycles += 6;
                                c.g[AT][0] = 0x3f000000;
                                c.mtc1(1, 1);
                                c.sub_s(0, 4, 1);
                                c.c_le_s(0, 2);
                                let t = !c.fcc();

                                if t {
                                    break 'b_2e773c;
                                } else {
                                    c.cycles += 4;
                                    c.sub_s(0, 5, 1);
                                    c.c_le_s(0, 2);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_2e773c;
                                    } else {
                                        c.cycles += 26;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                        c.g[AT][0] = 0x41200000;
                                        c.mtc1(1, 1);
                                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.sub_s(0, 0, 1);
                                        c.mfc1(4, 0);
                                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.qmtc2(4, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        let t = true;
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        if t {
                                            break 'b_2e78c8;
                                        } else {
                                            break 'b_2e773c;
                                        }
                                    }
                                }
                            }
                        }
                        'b_2e77d8: {
                            c.cycles += 3;
                            c.c_le_s(2, 3);
                            let t = !c.fcc();

                            if t {
                                break 'b_2e77d8;
                            } else {
                                c.cycles += 6;
                                c.g[AT][0] = 0x3f000000;
                                c.mtc1(1, 1);
                                c.sub_s(0, 4, 1);
                                c.c_le_s(0, 3);
                                let t = !c.fcc();

                                if t {
                                    break 'b_2e77d8;
                                } else {
                                    c.cycles += 4;
                                    c.sub_s(0, 5, 1);
                                    c.c_le_s(0, 3);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_2e77d8;
                                    } else {
                                        c.cycles += 26;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                        c.g[AT][0] = 0x41200000;
                                        c.mtc1(1, 1);
                                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.sub_s(0, 0, 1);
                                        c.mfc1(4, 0);
                                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.qmtc2(4, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        let t = true;
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        if t {
                                            break 'b_2e78c8;
                                        } else {
                                            break 'b_2e77d8;
                                        }
                                    }
                                }
                            }
                        }
                        'b_2e7868: {
                            c.cycles += 4;
                            c.c_le_s(2, 4);
                            if !c.fcc() {
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                break 'b_2e7868;
                            } else {
                                c.cycles += 4;
                                c.c_le_s(3, 4);
                                let t = !c.fcc();
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                if t {
                                    break 'b_2e7868;
                                } else {
                                    c.cycles += 3;
                                    c.c_le_s(5, 4);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_2e7868;
                                    } else {
                                        c.cycles += 25;
                                        c.g[AT][0] = 0x41200000;
                                        c.mtc1(1, 1);
                                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.sub_s(0, 0, 1);
                                        c.mfc1(4, 0);
                                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.qmtc2(4, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        let t = true;
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        if t {
                                            break 'b_2e78c8;
                                        } else {
                                            break 'b_2e7868;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 24;
                        c.g[AT][0] = 0x41200000;
                        c.mtc1(1, 1);
                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.qmfc2(2, 4);
                        c.mtc1(2, 0);
                        c.sub_s(0, 0, 1);
                        c.mfc1(4, 0);
                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                        c.qmtc2(4, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                        break 'b_2e78c8;
                    }
                    c.cycles += 33;
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = 0x1c;
                    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.mult(0, 2, 3, 4);
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xd4));
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xd8));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf4));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xf8));
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0xf7));
                    c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf8));
                    c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0x5b));
                    c.sdr(5, (c.g[V1][0] as u32).wrapping_add(0x54));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x5c), c.g[A2][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.mult(0, 3, 2, 4);
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xe4));
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf4));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xf8));
                    c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0xf7));
                    c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf8));
                    c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x67));
                    c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x60));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x68), c.g[A1][0]);
                    break 'b_2e794c;
                }
            }
            c.cycles += 1;
            c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x4));
            break 'b_2e7950;
        }
        'b_2e796c: {
            c.cycles += 6;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 0);
            c.c_lt_s(0, 2);
            let t = !c.fcc();
            c.g[T1][0] = 0u64;
            if t {
                break 'b_2e796c;
            } else {
                c.cycles += 1;
                c.g[T1][0] = 0x3;
                break 'b_2e796c;
            }
        }
        'b_2e79fc: {
            c.cycles += 4;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                break 'b_2e79fc;
            } else {
                c.cycles += 5;
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x18));
                c.c_lt_s(2, 1);
                if !c.fcc() {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                    break 'b_2e79fc;
                } else {
                    c.cycles += 6;
                    c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                    c.g[A3][0] = 0x1e;
                    c.g[T4][0] = 0u64;
                    c.g[T3][0] = 0x1c;
                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    'l_2e79a8: loop {
                        'b_2e79b0: {
                            c.cycles += 2;
                            if c.g[A3][0] == c.g[T4][0] {
                                break 'b_2e79b0;
                            } else {
                                break 'b_2e79b0;
                            }
                        }
                        'b_2e79f8: {
                            c.cycles += 13;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                            c.div(0, 2, 7);
                            c.g[A2][0] = c.g[A0][0];
                            c.g[A1][0] = ((c.g[T1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                            c.g[V1][0] = c.hi[0];
                            c.mult(0, 13, 3, 11);
                            c.g[V0][0] = sx((c.g[T5][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                            c.lwc1(0, (c.g[V0][0] as u32));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                            c.sub_s(1, 1, 0);
                            let t = c.g[A1][0] == 0u64;
                            c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x18));
                            if t {
                                break 'b_2e79f8;
                            } else {
                                c.cycles += 5;
                                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                                c.c_lt_s(0, 1);
                                let t = c.fcc();
                                c.g[V0][0] =
                                    sx((c.g[T2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002e_79a8);
                                    }
                                    continue 'l_2e79a8;
                                } else {
                                    break 'b_2e79f8;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                        break 'b_2e79fc;
                    }
                }
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x120));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
        abi::ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check_lohi, elf_image, put_vec, DATA, GAME_GP};

    /// The metrix, its rider and the objects the HUD callees walk through.
    const M: u32 = DATA + 0x4000;
    const RIDER: u32 = DATA + 0x6000;
    const OWNER: u32 = DATA + 0x6800;
    const BOARD: u32 = DATA + 0x6a00;
    const ANGLE_OBJ: u32 = DATA + 0x6c00;
    const BOARD_VT: u32 = DATA + 0x6d00;
    const SLOTS: u32 = DATA + 0x10000;
    const SLOT_COUNT: u32 = 0x2c;
    /// The BE-library singleton and the audio system the HUD calls into.
    const BELIB: u32 = DATA + 0x30000;
    const AUDIO: u32 = DATA + 0x31000;
    /// `world -> +0x84 manager -> +0xc frames`, and the audio "playing" list
    /// `manager+0x28 -> (a value < 10)` that makes every audio helper return
    /// at once.
    const WORLD: u32 = DATA + 0x32000;
    const MANAGER: u32 = DATA + 0x32100;
    const FRAMES: u32 = DATA + 0x32200;
    const PLAYING: u32 = DATA + 0x32300;

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    /// The speed-boost streamer's own object, its rider and model.
    const FX_RIDER: u32 = DATA + 0x9000;
    const MODEL: u32 = DATA + 0xa000;
    const BONES: u32 = DATA + 0xb000;
    const FX_VT: u32 = DATA + 0xb800;
    const ANIM: u32 = DATA + 0xbc00;

    #[test]
    fn speed_boost_streamer_fx_update_points_matches() {
        check_lohi(
            ee::s09::f_002e6c08__generated,
            c_speed_boost_streamer_fx_update_points,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x6c08, 1);
                elf_image(c);
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), BELIB);
                // The timer/fade decide whether the update runs at all.
                let (timer, fade): (f32, f32) = match seed % 4 {
                    0 => (0.5, 0.5),
                    1 => (-0.5, 0.5),
                    2 => (-0.5, -0.5),
                    _ => (0.0, 0.1),
                };
                c.bus.write32(M, FX_RIDER);
                c.bus.write32(M + 0x4, f(timer));
                c.bus.write32(M + 0x18, f(fade));
                c.bus.write32(M + 0x14, r.next_u32() % 40);
                c.bus.write32(M + 0x30, r.next_u32() % 4);
                for i in 0..0x30 {
                    c.bus.write32(M + 0x50 + 4 * i, r.edge_f32());
                }
                // The rider, its board (a vtable whose slot is the fake
                // method), its animation and its model with the bone array
                // the streamer indexes.
                c.bus.write32(FX_RIDER + 0x6c0, FX_VT);
                c.bus.write32(FX_RIDER + 0x780, MODEL);
                c.bus.write32(FX_RIDER + 0x784, ANIM);
                c.bus.write32(FX_RIDER + 0x8a4, r.next_u32() % 8);
                c.bus.write32(FX_RIDER + 0x320, (seed % 2) as u32);
                c.bus.write32(FX_RIDER + 0x330, r.f32(1.0));
                c.bus.write16(FX_VT + 0x28, 0);
                c.bus.write32(FX_VT + 0x2c, FAKE);
                c.bus.write32(MODEL + 0x30, BONES);
                for i in 0..0x100 {
                    c.bus.write32(BONES + 4 * i, r.edge_f32());
                }
                for off in [0x1e0, 0x370, 0x460] {
                    put_vec(
                        c,
                        FX_RIDER + off,
                        [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.f32(1.0)],
                    );
                }
                c.bus.write32(M + 0x20, r.f32(1.0));
                c.bus.write32(M + 0x24, r.f32(1.0));
                c.g[A0][0] = sx(M);
                c.pc = 0x002e_6c08;
            },
        );
    }

    #[test]
    fn rider_metrix_update_trick_hud_matches() {
        check_lohi(
            ee::s00::f_00117fe0__generated,
            rider_metrix_update_trick_hud,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7fe0, 0);
                elf_image(c);
                // Globals: the BE library (0x14dc80), the audio system
                // (0x28b180) and the world chain (0x1298c8).
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), BELIB);
                c.bus.write32(GAME_GP.wrapping_add(0x410), AUDIO);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
                c.bus.write32(WORLD + 0x84, MANAGER);
                c.bus.write32(MANAGER + 0xc, FRAMES);
                c.bus.write32(MANAGER + 0x28, PLAYING);
                c.bus.write32(PLAYING, 1);
                c.bus.write32(FRAMES + 8, r.next_u32() % 4096);

                // The metrix: a slot table (so the body runs) and the
                // trick-score inputs (`GetRoundedTrickPoints` +0x1c4 * +0x14).
                c.bus.write32(M + mx::SLOTS, SLOTS);
                c.bus.write32(M + 0x1b4, SLOT_COUNT);
                c.bus.write32(M + mx::RIDER, RIDER);
                c.bus.write32(M + mx::TRICK_BOOST, f(1.0));
                let scale: f32 = match seed % 3 {
                    0 => 0.0,
                    1 => 0.001,
                    _ => -0.001,
                };
                c.bus.write32(M + 0x1c4, f(scale));
                c.bus.write32(M + 0x1c, r.f32(2.0));
                c.bus.write32(M + mx::TRICK_COUNT, r.next_u32() % 4);
                c.bus.write32(M + mx::TRICK_FLAG, (seed % 3 == 0) as u32);
                c.bus.write32(M + mx::BONUS, r.next_u32() % 5000);
                c.bus.write32(M + mx::COMBO_COUNT, r.next_u32() % 5);
                c.bus.write32(M + mx::COMBO_POINTS, r.next_u32() % 8000);
                c.bus.write32(M + mx::COMBO_TIMER, r.f32(1.0));
                c.bus.write32(M + 0x198, r.next_u32() % 5000);

                // The HUD records: some are free (id 0x34), some are live.
                for i in 0..SLOT_COUNT {
                    let rec = SLOTS + i * mx::SLOT_SIZE;
                    let id = if r.next_u32() % 3 == 0 { 0x34 } else { i };
                    c.bus.write32(rec, id);
                    c.bus.write32(rec + 4, r.f32(1.0));
                    c.bus.write32(rec + 8, r.f32(1.0));
                    c.bus.write32(rec + 0xc, r.next_u32() % 10);
                    c.bus.write32(rec + 0x14, r.next_u32() % 1000);
                    c.bus.write32(rec + 0x1c, r.f32(1.0));
                    c.bus.write32(rec + 0x20, r.f32(2.0));
                    c.bus.write32(rec + 0x98, i);
                }

                // The rider and its owner: sub-state 2/5, boost mode 3 and
                // the meter/blend inputs the board and HUD-angle calls read.
                c.bus.write32(RIDER + rider::OWNER, OWNER);
                c.bus.write32(RIDER + rider::BOARD, BOARD_VT);
                c.bus.write32(RIDER + rider::BOOST_LEVEL, r.next_u32() % 14);
                c.bus.write32(RIDER + rider::BOOST_MODE, r.next_u32() % 8);
                c.bus.write32(RIDER + rider::BOOST_METER, r.f32(1.0));
                c.bus.write32(RIDER + rider::BOOST_STRENGTH, r.f32(1.0));
                c.bus.write16(BOARD_VT + 0x38, 0);
                c.bus.write32(BOARD_VT + 0x3c, FAKE);

                let sub = [2, 5, 8, 0][(seed % 4) as usize];
                c.bus.write32(OWNER + 0xde4, sub);
                c.bus.write32(OWNER + rider::OWNER_MODE, r.next_u32() % 8);
                c.bus.write32(OWNER + rider::OWNER_METER, r.f32(1.0));
                c.bus.write32(OWNER + 0x1f0, ANGLE_OBJ);
                c.bus.write32(OWNER + 0x230, BOARD);
                c.bus.write32(ANGLE_OBJ + 0x2a4, r.f32(0.5));
                c.bus.write32(ANGLE_OBJ + 0x2b0, r.f32(0.5));
                for off in [0x18, 0x1c, 0x50] {
                    c.bus.write32(BOARD + off, r.f32(1.0));
                }

                c.g[A0][0] = sx(M);
            },
        );
    }
}
