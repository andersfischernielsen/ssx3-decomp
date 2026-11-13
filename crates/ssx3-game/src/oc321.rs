//! Staging module oc321 (wave wave213): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc321`.

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
    (0x003d_b208, spch_eval_part_choice, "SPCH_EvalPartChoice"),
    (
        0x002b_03b8,
        speech_get_language_from_suffix,
        "Speech_GetLanguageFromSuffix",
    ),
    (
        0x0029_eee0,
        ssxaudiosystem_get_speech_category,
        "SSXAudioSystem_GetSpeechCategory",
    ),
    (
        0x0011_a7a8,
        ridermetrix_bank_pending_score,
        "RiderMetrix_BankPendingScore",
    ),
    (0x003d_8cb0, spch_pick_weighted, "SPCH_PickWeighted"),
    (
        0x0035_5d10,
        modobj_add_dynamic_particle_modifier,
        "ModObj_AddDynamicParticleModifier",
    ),
    (0x002a_d888, bankmonitor_timer_set, "BankMonitor_TimerSet"),
    (0x0032_0448, mem_pool512_alloc, "MEM_Pool512Alloc"),
    (
        0x0029_61f0,
        ssxaudiosystem_play_wipeout_crowd_reaction,
        "SSXAudioSystem_PlayWipeoutCrowdReaction",
    ),
];

/// The guest functions this module's replacements call directly.
pub(crate) mod callee {
    use super::Callee;
    use ssx3_ee as ee;

    pub const RIDER_METRIX_ROUNDED_TRICK_POINTS: Callee = Callee {
        addr: 0x0011_7948,
        f: ee::s00::RiderMetrix_GetRoundedTrickPoints,
    };
    pub const RIDER_METRIX_REMOVE_HUD_MESSAGE: Callee = Callee {
        addr: 0x0011_79e0,
        f: ee::s00::RiderMetrix_RemoveHUDMessage,
    };
    pub const RIDER_METRIX_SHOW_HUD_NUMBER: Callee = Callee {
        addr: 0x0011_7b88,
        f: ee::s00::RiderMetrix_ShowHUDNumber,
    };
    pub const RIDER_METRIX_TRICK_BONUS: Callee = Callee {
        addr: 0x0011_7908,
        f: ee::s00::RiderMetrix_GetTrickBonus,
    };
    pub const SPCH_RANDOM_NO_REPEAT: Callee = Callee {
        addr: 0x003d_b5c0,
        f: ee::s14::SPCH_RandomNoRepeat,
    };
    pub const BANK_MONITOR_TIMER_KILL: Callee = Callee {
        addr: 0x002a_d940,
        f: ee::s08::BankMonitor_TimerKill,
    };
    pub const BANK_MONITOR_TIMER_SET_PERIOD: Callee = Callee {
        addr: 0x002a_d970,
        f: ee::s08::BankMonitor_TimerSetPeriod,
    };
    pub const BANK_MONITOR_TIMER_ENABLE: Callee = Callee {
        addr: 0x002a_d980,
        f: ee::s08::BankMonitor_TimerEnable,
    };
    pub const MEM_POOL_ALLOC: Callee = Callee {
        addr: 0x0031_9e48,
        f: ee::s10::MEM_PoolAlloc,
    };
    pub const DYNAMIC_PARTICLE_MODIFIER_CTOR: Callee = Callee {
        addr: 0x0034_5c90,
        f: ee::s11::DynamicParticleModifier_ctor,
    };
    pub const MODOBJ_LINK_PARTICLE_MODIFIER: Callee = Callee {
        addr: 0x0035_5658,
        f: ee::s11::ModObj_LinkParticleModifier,
    };
}

// ---------------------------------------------------------------------------
// SPCH_EvalPartChoice (0x3db208)
// ---------------------------------------------------------------------------

/// The speech filter callback table `D_0044FF90`: the callback pointer sits
/// at `+4`. Zero means "no callback" (every part entry is rejected).
#[cfg(test)]
pub(crate) const SPCH_EVAL_PART_CHOICE_CB: u32 = 0x0044_ff90;

/// The generated body materialises that table with `lui 0x45`, then adds
/// `0x0 - 0x70`: `v0` is left at `0x450000` and `t0` (the table base the
/// callback is loaded from) at `0x450000 - 0x70`.
pub(crate) const SPCH_EVAL_PART_CHOICE_CB_LUI: u32 = 0x0045_0000;

/// The bank-type table `SPCH_FindBankType` scans (8 eight-byte entries); the
/// test writes it.
#[cfg(test)]
pub(crate) const SPCH_EVAL_PART_CHOICE_TYPE_TABLE: u32 = 0x0044_ffb8;

/// Frame offsets of `SPCH_EvalPartChoice`'s 0x110-byte frame.
pub(crate) const SPCH_EVAL_PART_CHOICE_OUT: u32 = 0x10;
pub(crate) const SPCH_EVAL_PART_CHOICE_HANDLE: u32 = 0x14;
pub(crate) const SPCH_EVAL_PART_CHOICE_VALS: u32 = 0x18;
pub(crate) const SPCH_EVAL_PART_CHOICE_NEGMASK: u32 = 0x1c;
pub(crate) const SPCH_EVAL_PART_CHOICE_TBL: u32 = 0x20;
pub(crate) const SPCH_EVAL_PART_CHOICE_NEXT_K: u32 = 0x24;
/// The current part entry, copied to the frame (`{u8 key; u8 class; u8 data}`).
pub(crate) const SPCH_EVAL_PART_CHOICE_T_A: u32 = 0;
pub(crate) const SPCH_EVAL_PART_CHOICE_T_B: u32 = 4;
pub(crate) const SPCH_EVAL_PART_CHOICE_T_C: u32 = 8;
/// Where the caller-saved q8..q11 (`t0`..`t3`) go across the callback.
pub(crate) const SPCH_EVAL_PART_CHOICE_SAVE_T: u32 = 0x30;

/// `int SPCH_EvalPartChoice(BankType* handle, SpeechBank* h, int* vals,
/// unsigned char* negmask)` (0x3db208): scores every part entry of `h`
/// against the 20 weight classes (`k`) through the filter callback, returns
/// the bit mask of the entries the callback accepted (one bit per entry,
/// bit `7 - index`) and ORs the rejected ones' bits into `*negmask`.
///
/// The callback is `int cb(handle, entry.key, value, out)`, `value` being
/// `vals[k]` for class `k` and 0 for class 20 (`t.b == 0`); a positive result
/// accepts the entry, 0 skips it and a negative one rejects it.
pub fn spch_eval_part_choice(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S0, 0x70);
        c.g[S0][0] = c.g[A1][0]; // s0 = the bank header
        frame.save(c, FP, 0xf0);
        frame.save(c, S7, 0xe0);
        c.g[A1][0] = sx(frame.at(c, SPCH_EVAL_PART_CHOICE_OUT));
        frame.save(c, S6, 0xd0);
        c.g[FP][0] = 0; // res
        frame.save(c, S3, 0xa0);
        c.g[S7][0] = 0; // neg
        frame.save(c, RA, 0x100);
        c.g[S3][0] = 1; // k
        frame.save(c, S5, 0xc0);
        frame.save(c, S4, 0xb0);
        frame.save(c, S2, 0x90);
        frame.save(c, S1, 0x80);
        c.sw(frame.at(c, SPCH_EVAL_PART_CHOICE_HANDLE), c.g[A0][0]);
        c.sw(frame.at(c, SPCH_EVAL_PART_CHOICE_VALS), c.g[A2][0]);
        c.sw(frame.at(c, SPCH_EVAL_PART_CHOICE_NEGMASK), c.g[A3][0]);
        let find_bank_type = Callee {
            addr: 0x003d_8330,
            f: ee::s14::SPCH_FindBankType,
        };
        find_bank_type.call(c, 0x003d_b258)?;

        // The header: byte +6 is the table size index, the low nibble of byte
        // +7 the entry count; the entries follow at a 4-byte aligned offset.
        let bank = r32(c, S0);
        c.g[V0][0] = Addr(bank).off(6).lbu(c);
        c.g[V1][0] = Addr(bank).off(7).lbu(c);
        c.g[V0][0] = sx(r32(c, V0) << 1);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(3));
        c.g[S6][0] = c.g[V1][0] & 0xf; // n = entry count
        c.g[V0][0] = c.g[V0][0] & 0x3fc;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xc));
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0))); // tbl
        c.sw(frame.at(c, SPCH_EVAL_PART_CHOICE_TBL), c.g[S0][0]);
        c.g[V0][0] = sx(r32(c, S3).wrapping_add(1)); // k + 1 for the tail

        'k: loop {
            c.cycles += 3;
            c.g[S2][0] = 0; // j
            let no_entries = c.g[S6][0] == 0;
            c.sw(frame.at(c, SPCH_EVAL_PART_CHOICE_NEXT_K), c.g[V0][0]);
            if !no_entries {
                c.cycles += 9;
                c.g[V1][0] = c.lw(frame.at(c, SPCH_EVAL_PART_CHOICE_VALS));
                c.g[V0][0] = sx(r32(c, S3) << 2);
                c.g[S1][0] = c.lw(frame.at(c, SPCH_EVAL_PART_CHOICE_TBL));
                c.g[T3][0] = 20; // the "no class" pass
                c.g[S5][0] = sx(r32(c, V0).wrapping_add(r32(c, V1))); // &vals[k]
                c.g[T2][0] = 7;
                c.g[V0][0] = SPCH_EVAL_PART_CHOICE_CB_LUI as u64;
                c.g[T1][0] = 1;
                c.g[T0][0] = sx(SPCH_EVAL_PART_CHOICE_CB_LUI.wrapping_sub(0x70));

                'j: loop {
                    c.cycles += 8;
                    let entry = r32(c, S1);
                    c.g[V1][0] = Addr(entry).lbu(c); // t.a
                    c.g[S4][0] = 0; // bit
                    c.g[A0][0] = Addr(entry.wrapping_add(1)).lbu(c); // t.b
                    c.g[V0][0] = Addr(entry.wrapping_add(2)).lbu(c); // t.c
                    c.sw(frame.at(c, SPCH_EVAL_PART_CHOICE_T_A), c.g[V1][0]);
                    c.sw(frame.at(c, SPCH_EVAL_PART_CHOICE_T_C), c.g[V0][0]);
                    let k_is_max = c.g[S3][0] == c.g[T3][0];
                    c.sw(frame.at(c, SPCH_EVAL_PART_CHOICE_T_B), c.g[A0][0]);

                    'entry: {
                        if !k_is_max {
                            c.cycles += 2;
                            if c.g[A0][0] != c.g[S3][0] {
                                break 'entry; // another class: not this pass
                            }
                            c.cycles += 1;
                            c.g[A2][0] = Addr(r32(c, S5)).lw(c); // vals[k]
                        } else {
                            c.cycles += 2;
                            if c.g[A0][0] != 0 {
                                break 'entry; // a class entry: not the 20th pass
                            }
                            c.cycles += 2;
                            c.g[A2][0] = 0;
                        }

                        c.cycles += 6;
                        c.g[V0][0] = sx(r32(c, T2).wrapping_sub(r32(c, S2)));
                        c.g[V1][0] = Addr(r32(c, T0).wrapping_add(4)).lw(c); // cb
                        c.g[V0][0] = sx(r32(c, T1) << (r32(c, V0) & 31));
                        c.g[A1][0] = c.lw(frame.at(c, SPCH_EVAL_PART_CHOICE_T_A));
                        let no_callback = c.g[V1][0] == 0;
                        c.g[S0][0] = c.g[V0][0] & 0xff; // mask = 1 << (7 - j)
                        if no_callback {
                            c.cycles += 1;
                            c.g[V0][0] = 0xffff_ffff_ffff_ffff; // r = -1
                        } else {
                            c.cycles += 12;
                            c.g[A3][0] = c.lw(frame.at(c, SPCH_EVAL_PART_CHOICE_OUT));
                            c.g[A0][0] = c.lw(frame.at(c, SPCH_EVAL_PART_CHOICE_HANDLE));
                            frame.save(c, T0, SPCH_EVAL_PART_CHOICE_SAVE_T);
                            frame.save(c, T1, SPCH_EVAL_PART_CHOICE_SAVE_T + 0x10);
                            frame.save(c, T2, SPCH_EVAL_PART_CHOICE_SAVE_T + 0x20);
                            frame.save(c, T3, SPCH_EVAL_PART_CHOICE_SAVE_T + 0x30);
                            let target = r32(c, V1);
                            call_ptr(c, target, 0x003d_b320)?;
                            frame.restore(c, T0, SPCH_EVAL_PART_CHOICE_SAVE_T);
                            frame.restore(c, T1, SPCH_EVAL_PART_CHOICE_SAVE_T + 0x10);
                            frame.restore(c, T2, SPCH_EVAL_PART_CHOICE_SAVE_T + 0x20);
                            frame.restore(c, T3, SPCH_EVAL_PART_CHOICE_SAVE_T + 0x30);
                        }

                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            c.cycles += 2;
                            if (c.g[V0][0] as i64) <= 0 {
                                c.g[S7][0] = c.g[S7][0] | c.g[S0][0];
                            } else {
                                c.cycles += 1;
                                c.g[S4][0] = c.g[S0][0];
                            }
                        } else {
                            c.cycles += 2;
                            c.g[S4][0] = 0;
                        }
                        c.cycles += 2;
                        c.g[FP][0] = c.g[FP][0] | c.g[S4][0];
                    }
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));

                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let more_entries = c.g[V0][0] != 0;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(3));
                    if !more_entries {
                        break 'j;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_b2b0);
                    }
                }
            }

            c.cycles += 4;
            c.g[S3][0] = c.lw(frame.at(c, SPCH_EVAL_PART_CHOICE_NEXT_K));
            c.g[V0][0] = ((c.g[S3][0] as i64) < 21) as u64;
            let more_classes = c.g[V0][0] != 0;
            c.g[V0][0] = sx(r32(c, S3).wrapping_add(1));
            if !more_classes {
                break 'k;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_b280);
            }
        }

        c.cycles += 15;
        c.g[V1][0] = c.lw(frame.at(c, SPCH_EVAL_PART_CHOICE_NEGMASK));
        c.g[V0][0] = c.g[FP][0];
        frame.restore(c, RA, 0x100);
        let neg = c.g[S7][0];
        c.sb(r32(c, V1), neg);
        frame.restore(c, FP, 0xf0);
        frame.restore(c, S7, 0xe0);
        frame.restore(c, S6, 0xd0);
        frame.restore(c, S5, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S0, 0x70);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Speech_GetLanguageFromSuffix (0x2b03b8)
// ---------------------------------------------------------------------------

/// `int Speech_GetLanguageFromSuffix(const char* name)` (0x2b03b8): a speech
/// bank name ends in `_eng`, `_frn`, `_grm` or `_spn`; return 1..4 for those
/// languages and 0 otherwise. The check is on the last four characters, so a
/// name shorter than four still reads whatever precedes it.
pub fn speech_get_language_from_suffix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0041_6810, 0x002b_03d0)?; // strlen
        c.g[A1][0] = c.g[V0][0]; // len
        c.g[V1][0] = 0x5f; // '_'
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, S0))); // name + len
        c.g[V0][0] = Addr(r32(c, A0).wrapping_sub(4)).lb(c);
        let is_underscore = c.g[V0][0] == c.g[V1][0];
        c.g[S1][0] = 0; // lang
        let mut plus_one = true;
        'suffix: {
            if !is_underscore {
                break 'suffix;
            }
            // "_eng"
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, A0).wrapping_sub(3)).lb(c);
            c.g[V0][0] = 0x65;
            let not_e = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, S0)));
            if !not_e {
                c.cycles += 4;
                c.g[V1][0] = Addr(r32(c, A0).wrapping_sub(2)).lb(c);
                c.g[V0][0] = 0x6e;
                let not_n = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, S0)));
                if !not_n {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(r32(c, A0).wrapping_sub(1)).lb(c);
                    c.g[V0][0] = 0x67;
                    let not_g = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, S0)));
                    if !not_g {
                        c.cycles += 2;
                        c.g[S1][0] = 1;
                        break 'suffix;
                    }
                }
            }
            // "_frn"
            c.cycles += 4;
            c.g[A0][0] = 0x66;
            c.g[V1][0] = Addr(r32(c, V0).wrapping_sub(3)).lb(c);
            let not_f = c.g[V1][0] != c.g[A0][0];
            c.g[A1][0] = c.g[V0][0];
            if not_f {
                c.cycles += 1;
                c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(3)).lb(c);
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(2)).lb(c);
                c.g[V0][0] = 0x72;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(3)).lb(c);
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(1)).lb(c);
                    c.g[V0][0] = 0x6e;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(3)).lb(c);
                    } else {
                        c.cycles += 2;
                        c.g[S1][0] = 2;
                        break 'suffix;
                    }
                }
            }
            // "_grm"
            c.cycles += 3;
            c.g[V0][0] = 0x67;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(3)).lb(c);
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(2)).lb(c);
                c.g[V0][0] = 0x72;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(3)).lb(c);
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(1)).lb(c);
                    c.g[V0][0] = 0x6d;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(3)).lb(c);
                    } else {
                        c.cycles += 2;
                        c.g[S1][0] = 3;
                        break 'suffix;
                    }
                }
            }
            // "_spn"
            c.cycles += 3;
            c.g[V0][0] = 0x73;
            let not_s = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.g[S1][0];
            if not_s {
                plus_one = false;
                break 'suffix;
            }
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(2)).lb(c);
            c.g[V0][0] = 0x70;
            let not_p = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.g[S1][0];
            if not_p {
                plus_one = false;
                break 'suffix;
            }
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, A1).wrapping_sub(1)).lb(c);
            c.g[S1][0] = 4;
            c.g[V0][0] = c.g[V0][0] ^ 0x6e;
            if c.g[V0][0] != 0 {
                c.g[S1][0] = 0;
            }
        }
        if plus_one {
            c.cycles += 1;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_GetSpeechCategory (0x29eee0)
// ---------------------------------------------------------------------------

/// The speech category `SSXAudioSystem_GetSpeechCategory` caches, at `self +
/// 0x5738`.
pub(crate) const SSX_AUDIO_SPEECH_CATEGORY: u32 = 0x5738;

/// `int func_0029EEE0(SSXAudioSystem* self, int known, const char* name)`
/// (0x29eee0): pick the speech category from the bank name prefix — 2 for
/// `DJ_` (11 when the app has no sound-module handle yet), 3 for `PA_`, 10 for
/// `Arcade`, 4 for anything else — cache it in the system and return it. With
/// `known` set nothing is looked at and the answer is -1.
pub fn ssxaudiosystem_get_speech_category(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        if c.g[A1][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        } else {
            let name = r32(c, A2);
            {
                let mut to_prefix_table = true;
                'dj: {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(name).lb(c);
                    c.g[V0][0] = 0x44; // 'D'
                    let not_d = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = Addr(name).lbu(c);
                    if not_d {
                        c.cycles += 1;
                        c.g[V0][0] = sx(r32(c, A0) << 24);
                        break 'dj;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = Addr(name.wrapping_add(1)).lb(c);
                    c.g[V0][0] = 0x4a; // 'J'
                    let not_j = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = sx(r32(c, A0) << 24);
                    if not_j {
                        break 'dj;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = Addr(name.wrapping_add(2)).lb(c);
                    c.g[V0][0] = 0x5f; // '_'
                    let not_underscore = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = sx(r32(c, A0) << 24);
                    if not_underscore {
                        break 'dj;
                    }
                    c.cycles += 4;
                    call_ptr(c, 0x0028_b1b0, 0x0029_ef2c)?; // has a sound module
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = 0xb;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 2;
                    }
                    to_prefix_table = false;
                }
                if to_prefix_table {
                    let mut to_arcade = true;
                    'pa: {
                        c.cycles += 4;
                        c.g[V1][0] = 0x50; // 'P'
                        c.g[V0][0] = sx(((r32(c, V0) as i32) >> 24) as u32);
                        let not_p = c.g[V0][0] != c.g[V1][0];
                        c.g[V0][0] = sx(r32(c, A0) << 24);
                        if not_p {
                            break 'pa;
                        }
                        c.cycles += 4;
                        c.g[V1][0] = Addr(name.wrapping_add(1)).lb(c);
                        c.g[V0][0] = 0x41; // 'A'
                        let not_a = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = sx(r32(c, A0) << 24);
                        if not_a {
                            break 'pa;
                        }
                        c.cycles += 4;
                        c.g[V1][0] = Addr(name.wrapping_add(2)).lb(c);
                        c.g[V0][0] = 0x5f; // '_'
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[V0][0] = sx(r32(c, A0) << 24);
                            break 'pa;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = 3;
                        to_arcade = false;
                    }
                    if to_arcade {
                        c.cycles += 4;
                        c.g[V1][0] = 0x41; // 'A'
                        c.g[V0][0] = sx(((r32(c, V0) as i32) >> 24) as u32);
                        let not_a = c.g[V0][0] != c.g[V1][0];
                        c.g[V0][0] = 4;
                        if !not_a {
                            let rest = [b'r', b'c', b'a', b'd', b'e'];
                            let mut i = 0;
                            while i < rest.len() {
                                c.cycles += 4;
                                c.g[V1][0] = Addr(name.wrapping_add(1 + i as u32)).lb(c);
                                c.g[V0][0] = rest[i] as u64;
                                let differs = c.g[V1][0] != c.g[V0][0];
                                c.g[V0][0] = 4;
                                if differs {
                                    break;
                                }
                                i += 1;
                            }
                            if i == rest.len() {
                                c.cycles += 1;
                                c.g[V0][0] = 0xa;
                            }
                        }
                    }
                }
                c.cycles += 3;
                let cat = Addr(r32(c, S0)).off(SSX_AUDIO_SPEECH_CATEGORY);
                let store = c.g[V0][0];
                cat.sw(c, store);
                let stored = cat.lw(c);
                c.g[V0][0] = stored;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderMetrix_BankPendingScore (0x11a7a8)
// ---------------------------------------------------------------------------

/// The per-slot HUD numbers `RiderMetrix_BankPendingScore` refreshes, and the
/// "1.5x" size it shows them at.
pub(crate) const RIDER_METRIX_BANK_SCALE: u32 = 0x3fc0_0000; // 1.5f

/// `int RiderMetrix_BankPendingScore(cRiderMetrix* self)` (0x11a7a8): refresh
/// the HUD numbers for the tricks landed this jump (slot 0x25), the pending
/// base score (0x27) and the banked total (0x28), each only when its value is
/// positive, and always drop the "wiped out" message (slot 4). Returns the
/// rounded trick points.
pub fn ridermetrix_bank_pending_score(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::RIDER_METRIX_ROUNDED_TRICK_POINTS.call(c, 0x0011_a7c4)?;
        c.g[S1][0] = c.g[V0][0];
        let no_tricks = (c.g[S1][0] as i64) <= 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_tricks {
            c.cycles += 9;
            c.g[A1][0] = 1;
            callee::RIDER_METRIX_REMOVE_HUD_MESSAGE.call(c, 0x0011_a7d8)?;
            c.g[AT][0] = RIDER_METRIX_BANK_SCALE as u64;
            c.mtc1(1, 12);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x25;
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = 0;
            callee::RIDER_METRIX_SHOW_HUD_NUMBER.call(c, 0x0011_a7f4)?;
        }

        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S0)).off(0xa0).lw(c);
        let no_pending = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_pending {
            c.cycles += 9;
            c.g[A1][0] = 3;
            callee::RIDER_METRIX_REMOVE_HUD_MESSAGE.call(c, 0x0011_a808)?;
            c.g[A2][0] = Addr(r32(c, S0)).off(0xa0).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[AT][0] = RIDER_METRIX_BANK_SCALE as u64;
            c.mtc1(1, 12);
            c.g[A1][0] = 0x27;
            c.g[A3][0] = 0;
            callee::RIDER_METRIX_SHOW_HUD_NUMBER.call(c, 0x0011_a824)?;
        }

        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_METRIX_TRICK_BONUS.call(c, 0x0011_a82c)?;
        c.g[V1][0] = Addr(r32(c, S0)).off(0x84).lw(c);
        c.g[S2][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        let no_total = (c.g[S2][0] as i64) <= 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_total {
            c.cycles += 24;
            // The combo messages this total replaces.
            for (slot, ret) in [
                (0x2u64, 0x0011_a844u32),
                (0x1d, 0x0011_a850),
                (0x1c, 0x0011_a85c),
                (0x1e, 0x0011_a868),
                (0x1f, 0x0011_a874),
                (0x20, 0x0011_a880),
            ] {
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = slot;
                callee::RIDER_METRIX_REMOVE_HUD_MESSAGE.call(c, ret)?;
            }
            c.g[AT][0] = RIDER_METRIX_BANK_SCALE as u64;
            c.mtc1(1, 12);
            c.g[A2][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x28;
            c.g[A3][0] = 0;
            callee::RIDER_METRIX_SHOW_HUD_NUMBER.call(c, 0x0011_a89c)?;
        }

        c.cycles += 10;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 4;
        callee::RIDER_METRIX_REMOVE_HUD_MESSAGE.call(c, 0x0011_a8a8)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SPCH_PickWeighted (0x3d8cb0)
// ---------------------------------------------------------------------------

/// The per-class weight table `SPCH_PickWeighted` reads (32-bit words).
pub(crate) const SPCH_PICK_WEIGHT_TABLE: u32 = 0x0045_06e0;

/// Frame offsets of `SPCH_PickWeighted`'s 0x1f0-byte frame.
pub(crate) const SPCH_PICK_WEIGHTED_W: u32 = 0; // int w[]

/// `void SPCH_PickWeighted(SpeechBank* h, unsigned char* out)` (0x3d8cb0):
/// fill `out` with the bank's part indices in weighted-random order — one
/// pass picking a part by its weight (the packed per-part byte's low 5 bits
/// times its class weight) until the total runs out, then a pass appending
/// every part whose weight is zero.
pub fn spch_pick_weighted(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x1f0);
        frame.save(c, S4, 0x1d0);
        frame.save(c, S3, 0x1c0);
        c.g[S4][0] = c.g[A1][0]; // out
        frame.save(c, S2, 0x1b0);
        c.g[S3][0] = c.g[A0][0]; // h
        frame.save(c, RA, 0x1e0);
        frame.save(c, S0, 0x190);
        frame.save(c, S1, 0x1a0);
        c.g[S1][0] = Addr(r32(c, S3)).off(6).lbu(c); // n
        let no_entries = c.g[S1][0] == 0;
        c.g[S2][0] = 0; // total
        if !no_entries {
            c.cycles += 6;
            c.g[V0][0] = 0x450000;
            c.g[T0][0] = c.g[SP][0];
            c.g[T1][0] = sx(SPCH_PICK_WEIGHT_TABLE);
            c.g[A3][0] = sx(r32(c, S3).wrapping_add(0xc));
            c.g[A2][0] = c.g[S1][0];
            loop {
                c.cycles += 23;
                c.g[V0][0] = Addr(r32(c, A3)).lhu(c);
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(2));
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[A1][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                c.g[A0][0] = Addr(r32(c, A1)).lbu(c);
                c.g[V0][0] = sx(r32(c, A0) >> 5);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T1)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                c.mult(0, 4, 4, 3); // a0 = count * weight
                let weight = c.g[A0][0];
                c.sw(r32(c, T0), weight);
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(4));
                c.g[A0][0] = Addr(r32(c, A1)).lbu(c);
                c.g[V0][0] = sx(r32(c, A0) >> 5);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T1)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                c.mult(0, 2, 4, 3);
                let more = c.g[A2][0] != 0;
                c.g[S2][0] = sx(r32(c, V0).wrapping_add(r32(c, S2)));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_8cf8);
                }
            }
        }

        c.cycles += 2;
        let none_left = (c.g[S2][0] as i64) <= 0;
        c.g[S0][0] = 0; // k, the output count
        if !none_left {
            c.cycles += 1;
            loop {
                let mut recount = false;
                c.cycles += 8;
                c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                c.g[A0][0] = c.g[S2][0];
                callee::SPCH_RANDOM_NO_REPEAT.call(c, 0x003d_8d6c)?; // r
                c.g[A3][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[A1][0] = sx(r32(c, S4).wrapping_add(r32(c, S0))); // &out[k]
                c.g[V1][0] = c.g[V0][0];
                let no_entries = c.g[S1][0] == 0;
                c.g[A2][0] = 0; // i
                if no_entries {
                    recount = true;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(frame.at(c, SPCH_PICK_WEIGHTED_W));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, V0)));
                    let negative = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = sx(r32(c, A2) << 2);
                    if negative {
                        // r < w[0]: part 0 wins.
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                        loop {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[S1][0] as i64)) as u64;
                            let out_of_range = c.g[V0][0] == 0;
                            c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
                            if out_of_range {
                                recount = true;
                                break;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = Addr(r32(c, A0)).lw(c);
                            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, V0)));
                            if (c.g[V1][0] as i64) >= 0 {
                                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                                if event_due(c) {
                                    deliver_event_here(c, 0x003d_8d98);
                                }
                            } else {
                                recount = true;
                                break;
                            }
                        }
                    }
                }
                if recount {
                    c.cycles += 1;
                    c.g[V0][0] = sx(r32(c, A2) << 2);
                }
                c.cycles += 7;
                let picked = c.g[A2][0];
                c.sb(r32(c, A1), picked);
                c.g[V1][0] = sx(r32(c, SP).wrapping_add(r32(c, V0)));
                c.g[S0][0] = c.g[A3][0];
                c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                c.g[S2][0] = sx(r32(c, S2).wrapping_sub(r32(c, V0)));
                let still_left = (c.g[S2][0] as i64) > 0;
                c.sw(r32(c, V1), 0); // the picked weight is spent
                if !still_left {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_8d60);
                }
            }
        }

        c.cycles += 2;
        let no_entries = c.g[S1][0] == 0;
        c.g[A2][0] = 0;
        if !no_entries {
            c.cycles += 3;
            c.g[V0][0] = 0x450000;
            c.g[A3][0] = sx(r32(c, S3).wrapping_add(0xc));
            c.g[T0][0] = sx(SPCH_PICK_WEIGHT_TABLE);
            loop {
                c.cycles += 12;
                c.g[V0][0] = Addr(r32(c, A3)).lhu(c);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[A1][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                c.g[A0][0] = Addr(r32(c, A1)).lbu(c);
                c.g[V0][0] = sx(r32(c, A0) >> 5);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T0)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                c.mult(0, 4, 4, 3);
                if c.g[A0][0] != 0 {
                    c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, S0)));
                    let index = c.g[A2][0];
                    c.sb(r32(c, V0), index);
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_8de8);
                }
            }
        }

        c.cycles += 8;
        frame.restore(c, RA, 0x1e0);
        frame.restore(c, S4, 0x1d0);
        frame.restore(c, S3, 0x1c0);
        frame.restore(c, S2, 0x1b0);
        frame.restore(c, S1, 0x1a0);
        frame.restore(c, S0, 0x190);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModObj_AddDynamicParticleModifier (0x355d10)
// ---------------------------------------------------------------------------

/// The pool descriptor `D_0048E990` (`0x490000 - 0x1670`) a dynamic-particle
/// modifier is allocated from.
pub(crate) const MODOBJ_PARTICLE_MODIFIER_POOL: u32 = 0x0048_e990;

/// `void cMoveNode_addDynamicParticle(cMoveNode* self, void* desc, int index,
/// void* a4)` (0x355d10): when `index` names one of the node's dynamic
/// particles, allocate a 0x270-byte modifier, construct it from `desc`, the
/// node and `a4`, and link it into the node's modifier list; an out-of-range
/// index deletes `desc` instead.
pub fn modobj_add_dynamic_particle_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0]; // index
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0]; // self
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0]; // desc
        frame.save_ra(c, 0);
        let negative = (c.g[S0][0] as i64) < 0;
        c.g[S3][0] = c.g[A3][0]; // a4
        let mut in_range = false;
        if !negative {
            c.cycles += 6;
            c.g[V0][0] = Addr(r32(c, S1)).off(0x18).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x80).lw(c);
            c.g[V1][0] = Addr(r32(c, A0)).off(4).lw(c);
            c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            in_range = c.g[V1][0] != 0;
            c.g[A1][0] = 0x490000;
        }
        if in_range {
            c.cycles += 15;
            c.g[A0][0] = 0x270;
            c.g[A1][0] = sx(MODOBJ_PARTICLE_MODIFIER_POOL);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0035_5d78)?; // cMemMan_alloc
            c.g[A2][0] = Addr(r32(c, S1)).off(0x18).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S0][0];
            callee::DYNAMIC_PARTICLE_MODIFIER_CTOR.call(c, 0x0035_5d90)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            callee::MODOBJ_LINK_PARTICLE_MODIFIER.call(c, 0x0035_5d9c)?;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0031_7e50, 0x0035_5d5c)?; // operator delete(desc)
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BankMonitor_TimerSet (0x2ad888)
// ---------------------------------------------------------------------------

/// `void BankMonitor_TimerSet(BankMonitorTimer* t, int period, sBmKey key,
/// int b, int c, int d, int e, int f)` (0x2ad888): re-arm a bank-monitor
/// timer — kill it, set its period, store the 8-byte key (an unaligned
/// copy through the stack) and five payload words, then enable it.
pub fn bankmonitor_timer_set(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 42;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        c.g[S0][0] = c.g[A0][0]; // the timer
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A1][0]; // period
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A3][0]; // b
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[T0][0]; // c
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[T1][0]; // d
        frame.save(c, S6, 0x20);
        c.g[S5][0] = c.g[T2][0]; // e
        c.g[S6][0] = c.g[T3][0]; // f
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        let key = c.g[A2][0];
        c.sd(frame.at(c, 0), key);
        callee::BANK_MONITOR_TIMER_KILL.call(c, 0x002a_d8d0)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        callee::BANK_MONITOR_TIMER_SET_PERIOD.call(c, 0x002a_d8dc)?;
        // The key is copied 8 bytes at a time, unaligned.
        c.ldl(2, frame.at(c, 7));
        c.ldr(2, frame.at(c, 0));
        let key_src = Addr(r32(c, S0));
        c.sdl(2, key_src.off(0x13).0);
        c.sdr(2, key_src.off(0xc).0);
        let timer = Addr(r32(c, S0));
        let b = c.g[S2][0];
        timer.off(0x18).sw(c, b);
        c.g[A0][0] = c.g[S0][0];
        let cw = c.g[S3][0];
        timer.off(0x1c).sw(c, cw);
        let dw = c.g[S4][0];
        timer.off(0x20).sw(c, dw);
        let ew = c.g[S5][0];
        timer.off(0x24).sw(c, ew);
        let fw = c.g[S6][0];
        timer.off(0x28).sw(c, fw);
        callee::BANK_MONITOR_TIMER_ENABLE.call(c, 0x002a_d908)?;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x90));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MEM_Pool512Alloc (0x320448)
// ---------------------------------------------------------------------------

/// `void* MEM_Pool512Alloc(void* pool)` (0x320448): take one 512-byte block
/// from the pool's free list (a singly linked list at `pool + 0x20000`);
/// when the list is empty, ask the allocator for a fresh 0x204-byte block.
pub fn mem_pool512_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x20000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        c.g[V0][0] = c.lw(r32(c, A0));
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = 0x204;
            callee::MEM_POOL_ALLOC.call(c, 0x0032_046c)?;
            frame.restore_ra(c, 0);
        } else {
            c.g[V0][0] = c.lw(r32(c, A0));
            c.cycles += 3;
            c.g[V1][0] = c.lw(r32(c, V0));
            let next = c.g[V1][0];
            c.sw(r32(c, A0), next);
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_PlayWipeoutCrowdReaction (0x2961f0)
// ---------------------------------------------------------------------------

/// `void func_002961F0(SSXAudioSystem* sys, cRider* rider, float v)`
/// (0x2961f0): the crowd's reaction to a wipeout — resolve the rider's voice
/// index (0 while a replay plays, otherwise the tracked rider's), bail out
/// when the system's current rider is someone else, then play the crowd's
/// fall (or stop the anticipation) by how hard the wipeout was, and reset the
/// music fade-in.
pub fn ssxaudiosystem_play_wipeout_crowd_reaction(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0]; // sys
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0xffff_ffff_ffff_ffff; // voice index
        frame.save_fpr(c, 20, 0x40);
        c.g[S2][0] = c.g[A1][0]; // rider
        frame.save_ra(c, 0);
        c.mov_s(20, 12); // f20 = v
        call_ptr(c, 0x0028_8ae0, 0x0029_621c)?; // IsReplayPlaying(sys)
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(r32(c, S2)).off(0x874).lw(c);
            c.cycles += 2;
            let no_tracking = c.g[V0][0] == 0;
            c.g[V1][0] = 0;
            if !no_tracking {
                c.cycles += 2;
                c.g[V0][0] = Addr(r32(c, S2)).off(0x87c).lw(c);
                c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
            }
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                c.g[S1][0] = Addr(r32(c, S2)).off(0x870).lw(c);
            }
        } else {
            c.cycles += 2;
            c.g[S1][0] = 0;
        }

        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0028_5d98, 0x0029_6250)?; // FindRider(sys, index)
        if c.g[V0][0] == c.g[S2][0] {
            'reaction: {
                c.cycles += 6;
                c.mtc1(0, 0);
                c.abs_s(12, 20);
                c.c_le_s(12, 0);
                let louder = !c.fcc();
                c.g[A1][0] = c.g[S1][0];
                if !louder {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002a_7438, 0x0029_6278)?; // StopAnticipate
                    c.g[A0][0] = c.g[S0][0];
                    break 'reaction;
                }
                'quiet: {
                    c.cycles += 6;
                    c.g[AT][0] = 0x3e80_0000;
                    c.mtc1(1, 0);
                    c.c_lt_s(12, 0);
                    let below_quarter = !c.fcc();
                    c.g[A2][0] = c.g[S1][0];
                    if !below_quarter {
                        break 'quiet;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x002a_73d8, 0x0029_62a4)?; // PlayFall(0)
                    c.g[A0][0] = c.g[S0][0];
                    break 'reaction;
                }
                'medium: {
                    c.cycles += 5;
                    let gp = r32(c, GP);
                    c.lwc1(0, gp.wrapping_sub(0x45a0));
                    c.c_lt_s(12, 0);
                    let below_soft = !c.fcc();
                    c.g[A0][0] = c.g[S0][0];
                    if !below_soft {
                        break 'medium;
                    }
                    c.cycles += 4;
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x002a_73d8, 0x0029_62c8)?; // PlayFall(1)
                    c.g[A0][0] = c.g[S0][0];
                    break 'reaction;
                }
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = 2;
                call_ptr(c, 0x002a_73d8, 0x0029_62dc)?; // PlayFall(2)
                c.g[A0][0] = c.g[S0][0];
            }
            c.cycles += 3;
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x0028_f108, 0x0029_62e8)?; // ResetMusicFadeIn
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 6;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        c.g[RA][0] = c.ld(frame.at(c, 0));
        frame.restore_fpr(c, 20, 0x40);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, DATA, GAME_GP};
    use crate::game::dirtysock::tagsunk::tests::put_arena;

    /// SPCH_EvalPartChoice: a bank header with 0..12 three-byte part entries,
    /// a bank-type table that matches (the type writes `out`) or does not,
    /// and a filter callback that is absent (r = -1), a fake (r > 0) or the
    /// real 0-returning leaf 0x1406c0 (r = 0).
    #[test]
    fn spch_eval_part_choice_matches() {
        covered(
            ee::s14::f_003db208__generated,
            spch_eval_part_choice,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_b208, 4);
                c.pc = 0x003d_b208;
                let handle = DATA + 0x2000; // bank-type descriptor
                let ty = DATA + 0x3000; // a table entry's type struct
                let bank = DATA + 0x4000; // header and part entries
                let vals = DATA + 0x5000;
                let negmask = DATA + 0x6000;
                c.g[A0][0] = sx(handle);
                c.g[A1][0] = sx(bank);
                c.g[A2][0] = sx(vals);
                c.g[A3][0] = sx(negmask);

                // The type table: on a match the entry's type pointer is
                // stored and the type's byte +9 becomes `out`.
                for i in 0..8 {
                    c.bus.write32(SPCH_EVAL_PART_CHOICE_TYPE_TABLE + 8 * i, 0);
                }
                c.bus.write8(ty + 8, seed as u8);
                c.bus.write8(ty + 9, (seed >> 8) as u8);
                c.bus.write8(handle + 2, seed as u8);
                c.bus.write8(handle + 3, (seed >> 8) as u8);
                if seed % 2 == 0 {
                    c.bus.write32(SPCH_EVAL_PART_CHOICE_TYPE_TABLE, ty);
                }

                // Header: table index 0 (entries at +0xc), entry count.
                c.bus.write8(bank + 6, 0);
                let n = if seed % 7 == 0 { 0 } else { seed % 12 + 1 };
                c.bus.write8(bank + 7, n as u8);
                for j in 0..n {
                    let e = bank + 0xc + 3 * j;
                    c.bus.write8(e, r.next_u32() as u8); // t.a: callback key
                    c.bus.write8(e + 1, ((j + seed) % 22) as u8); // t.b: class
                    c.bus.write8(e + 2, r.next_u32() as u8); // t.c
                }
                for k in 0..21 {
                    c.bus.write32(vals + 4 * k, r.next_u32());
                }

                match seed % 4 {
                    0 => c.bus.write32(SPCH_EVAL_PART_CHOICE_CB + 4, 0),
                    3 => c.bus.write32(SPCH_EVAL_PART_CHOICE_CB + 4, 0x0014_06c0),
                    _ => {
                        c.bus.write32(SPCH_EVAL_PART_CHOICE_CB + 4, FAKE);
                        for i in 0..8 {
                            c.bus.write32(LOG_RESULTS + 4 * i, r.next_u32() % 5 + 1);
                        }
                    }
                }
            },
        );
    }

    /// Speech_GetLanguageFromSuffix: names ending in `_eng`, `_frn`, `_grm`,
    /// `_spn`, an unknown suffix, and no `_` suffix at all (with and without
    /// a name prefix).
    #[test]
    fn speech_get_language_from_suffix_matches() {
        check(
            ee::s08::f_002b03b8__generated,
            speech_get_language_from_suffix,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_03b8, 0);
                let name = DATA + 0x100;
                c.g[A0][0] = sx(name);
                let suffix: &[u8] = match seed % 6 {
                    0 => &b"eng"[..],
                    1 => &b"frn"[..],
                    2 => &b"grm"[..],
                    3 => &b"spn"[..],
                    4 => &b"xyz"[..],
                    _ => &b""[..],
                };
                let mut p = name;
                for _ in 0..(seed / 6) % 5 {
                    c.bus.write8(p, b'a' + (r.next_u32() % 26) as u8);
                    p += 1;
                }
                if seed % 2 == 0 {
                    c.bus.write8(p, b'_');
                    p += 1;
                }
                for (i, b) in suffix.iter().enumerate() {
                    c.bus.write8(p + i as u32, *b);
                }
                c.bus.write8(p + suffix.len() as u32, 0);
            },
        );
    }

    /// SSXAudioSystem_GetSpeechCategory: the `DJ_`, `PA_`, `Arcade` and
    /// other prefixes (with the sound-module handle present, absent, and a
    /// zero world), plus the "already known" argument and a stale cached
    /// category in the system.
    #[test]
    fn ssxaudiosystem_get_speech_category_matches() {
        check(
            ee::s08::f_0029eee0__generated,
            ssxaudiosystem_get_speech_category,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29_eee0, 0);
                let this = DATA;
                let name = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 5 == 0 { 1 } else { 0 };
                c.g[A2][0] = sx(name);
                // The world and the sound-module handle behind `gp-0x848`.
                let world = if seed % 2 == 0 { DATA + 0x2000 } else { 0 };
                c.bus.write32(GAME_GP - 0x848, world);
                let module = if seed % 3 == 0 { 0 } else { r.next_u32() | 1 };
                c.bus.write32(world + 0x84, module);
                let s: &[u8] = match seed % 8 {
                    0 => &b"DJ_pick1"[..],
                    1 => &b"PA_x"[..],
                    2 => &b"Arcade"[..],
                    3 => &b"Arcadf"[..],
                    4 => &b"Arcax"[..],
                    5 => &b"DJ"[..],
                    6 => &b"Az"[..],
                    _ => &b"Zzz"[..],
                };
                for (i, b) in s.iter().enumerate() {
                    c.bus.write8(name + i as u32, *b);
                }
                c.bus.write8(name + s.len() as u32, 0);
                c.bus
                    .write32(this + SSX_AUDIO_SPEECH_CATEGORY, r.next_u32());
            },
        );
    }

    /// RiderMetrix_BankPendingScore: the trick points, the pending score and
    /// the banked total each negative, zero or positive (the real HUD
    /// functions return at once on an empty message list).
    #[test]
    fn ridermetrix_bank_pending_score_matches() {
        check(
            ee::s00::f_0011a7a8__generated,
            ridermetrix_bank_pending_score,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_a7a8, 0);
                let this = DATA;
                c.g[A0][0] = sx(this);
                // No HUD message list: Remove/Show return immediately.
                c.bus.write32(this + 0x1b0, 0);
                c.bus.write32(this + 0x1b4, 0);
                // Trick points = count (+0x14) * factor (+0x1c4) * gp const.
                c.bus.write32(
                    this + 0x14,
                    if seed % 3 == 0 {
                        0
                    } else {
                        (r.next_u32() % 0x1000) as f32 as u32
                    },
                );
                c.bus.write32(this + 0x1c4, 1.0f32.to_bits());
                c.bus
                    .write32(this + 0x18, (r.next_u32() % 0x1000) as f32 as u32);
                c.bus.write32(GAME_GP - 0x7b08, 1.0f32.to_bits());
                c.bus
                    .write32(this + 0xa0, [0, 1, 0xffff_ffff, 77][(seed % 4) as usize]);
                c.bus.write32(this + 0x84, (seed % 3) as u32);
            },
        );
    }

    /// SPCH_PickWeighted: banks of 0..6 parts with class weights that are
    /// zero or not, so the weighted pass picks parts until the total runs
    /// out and the zero-weight pass appends the rest.
    #[test]
    fn spch_pick_weighted_matches() {
        check(
            ee::s14::f_003d8cb0__generated,
            spch_pick_weighted,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_8cb0, 0);
                let h = DATA;
                let out = DATA + 0x8000;
                c.g[A0][0] = sx(h);
                c.g[A1][0] = sx(out);
                for i in 0..8u32 {
                    let weight = match (seed + i) % 4 {
                        0 => 0,
                        1 => 1,
                        2 => 3,
                        _ => 0x11,
                    };
                    c.bus.write32(SPCH_PICK_WEIGHT_TABLE + 4 * i, weight);
                }
                let n = if seed % 9 == 0 { 0 } else { seed % 6 + 1 };
                c.bus.write8(h + 6, n as u8); // the part count
                                              // Part ids 8..13 keep their packed bytes clear of the id list.
                for j in 0..n {
                    c.bus.write16(h + 0xc + 2 * j, (8 + j) as u16);
                    let class = ((seed + j * 3) % 8) as u32;
                    let count = (seed / 6 + j) % 32;
                    c.bus.write8(h + 4 * (8 + j), ((class << 5) | count) as u8);
                }
                for i in 0..8u32 {
                    c.bus.write8(out + i, 0xee);
                }
            },
        );
    }

    /// ModObj_AddDynamicParticleModifier: a negative index and an index past
    /// the particle table delete the descriptor (a null one, so the free
    /// path stays short), a valid index allocates from the main heap and
    /// links the modifier.
    #[test]
    fn modobj_add_dynamic_particle_modifier_matches() {
        check(
            ee::s11::f_00355d10__generated,
            modobj_add_dynamic_particle_modifier,
            90,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_5d10, 8);
                put_arena(c);
                let node = DATA + 0x1000;
                let group = DATA + 0x2000;
                let table = DATA + 0x3000;
                let count = seed % 4;
                c.bus.write32(node + 0x18, group);
                c.bus.write32(group + 0x80, table);
                c.bus.write32(table + 4, count);
                // The node array the modifier's `TransformPoints` walks
                // (`Instance_GetNodeMatrix` reads `lw(table + 8)` as its base):
                // a 16-byte entry per index, the next index at +0 and the
                // node's matrix at +0xc. -1 ends the chain, so the walk's back
                // edge runs once and then stops (zeroed entries never end).
                let nodes = DATA + 0x8000;
                c.bus.write32(table + 8, nodes);
                for (i, next) in [1u32, u32::MAX].into_iter().enumerate() {
                    let e = nodes + 0x10 * i as u32;
                    c.bus.write32(e, next);
                    c.bus.write32(e + 0xc, DATA + 0x9000 + 0x40 * i as u32);
                }
                // A modifier block already exists, so nothing is created.
                c.bus.write32(node + 0x1c, table + 0x100);
                let index = match seed % 3 {
                    0 => -1i32,
                    1 => count as i32 + 1,
                    _ => 0,
                };
                let valid = index >= 0 && (index as u32) < count;
                c.bus.write32(DATA + 0x4000 + 0x48, r.next_u32());
                c.bus.write32(DATA + 0x4000 + 0x50, r.next_u32());
                c.g[A0][0] = sx(node);
                // Only a valid index keeps the descriptor: the delete path
                // must not free a random pointer.
                c.g[A1][0] = sx(if valid && seed % 5 != 0 {
                    DATA + 0x4000
                } else {
                    0
                });
                c.g[A2][0] = sx(index as u32);
                c.g[A3][0] = sx(table + 0x200);
            },
        );
    }

    /// BankMonitor_TimerSet: a timer record with random fields; the real
    /// kill/set-period/enable leaves store the period, the 8-byte key and
    /// the five payload words.
    #[test]
    fn bankmonitor_timer_set_matches() {
        check(
            ee::s08::f_002ad888__generated,
            bankmonitor_timer_set,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_d888, 0);
                let timer = DATA + 0x100;
                for i in 0..0x30u32 {
                    c.bus.write32(timer + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(timer);
                c.g[A1][0] = sx(r.next_u32() % 5000);
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                c.g[A3][0] = sx(r.next_u32());
                c.g[T0][0] = sx(r.next_u32());
                c.g[T1][0] = sx(r.next_u32());
                c.g[T2][0] = sx(r.next_u32());
                c.g[T3][0] = sx(r.next_u32());
            },
        );
    }

    /// MEM_Pool512Alloc: a pool with a free block (whose `next` is zero or
    /// another block) takes it from the list; an empty list asks the
    /// allocator over the main heap for a 0x204-byte block.
    #[test]
    fn mem_pool512_alloc_matches() {
        check(
            ee::s10::f_00320448__generated,
            mem_pool512_alloc,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x32_0448, 0);
                put_arena(c);
                let pool = DATA + 0x2000;
                let head = pool + 0x20000;
                let block = DATA + 0x4000;
                let next = DATA + 0x5000;
                c.g[A0][0] = sx(pool);
                c.g[A1][0] = 8;
                if seed % 3 == 0 {
                    c.bus.write32(head, 0); // empty free list
                    c.bus.write32(GAME_GP + 0xdd8, (seed % 2) as u32);
                } else {
                    c.bus.write32(head, block);
                    c.bus.write32(block, if seed % 2 == 0 { 0 } else { next });
                    c.bus.write32(next, DATA + 0x6000);
                }
            },
        );
    }

    /// SSXAudioSystem_PlayWipeoutCrowdReaction: the rider-index resolution
    /// (not in a replay, tracking or not) and the early return when
    /// FindRider answers another rider, plus the four reaction levels when
    /// it answers the rider itself (rider = -1, the answer of a FindRider
    /// outside a replay) with the crowd instance absent.
    #[test]
    fn ssxaudiosystem_play_wipeout_crowd_reaction_matches() {
        check(
            ee::s08::f_002961f0__generated,
            ssxaudiosystem_play_wipeout_crowd_reaction,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29_61f0, 8);
                // The BELibrary singleton `GetRiderRatingTier` asks for.
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x3e000);
                let sys = DATA;
                // No crowd instance: the crowd calls return at once.
                c.bus.write32(sys + 0x18, 0);
                let rider = if seed % 3 == 0 {
                    0xffff_ffffu32
                } else {
                    DATA + 0x1000
                };
                c.bus.write32(rider.wrapping_add(0x874), (seed % 2) as u32);
                c.bus
                    .write32(rider.wrapping_add(0x87c), (seed % 4 != 0) as u32);
                c.bus.write32(rider.wrapping_add(0x870), r.next_u32() % 4);
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = sx(rider);
                c.f[12] = match seed % 7 {
                    0 => 0.0f32.to_bits(),
                    1 => 0.125f32.to_bits(),
                    2 => 0.25f32.to_bits(),
                    3 => 0.65f32.to_bits(),
                    4 => 2.0f32.to_bits(),
                    5 => (-0.3f32).to_bits(),
                    _ => r.edge_f32(),
                };
            },
        );
    }
}
