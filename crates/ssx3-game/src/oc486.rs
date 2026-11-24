//! Staging module oc486 (wave wave227): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc486`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Callee, Flow, Frame, Unwound};
use crate::events::back_edge_unwinds;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0015_9cd0,
    be_reward_int_grant_reward,
    "BERewardInt_GrantReward",
)];

/// Entry of `BERewardInt_GrantReward` (0x159cd0) and its end (0x15a2b4 is the
/// epilogue, 0x15a2e0 the next function).
const GRANT_ENTRY: u32 = 0x0015_9cd0;
const GRANT_END: u32 = 0x0015_a2e0;

/// The awards-received bits of a character record
/// (`cBELibrary_profileLib[profile][character]`, see the body).
const GRANT_AWARDS_RECEIVED: u32 = 0xf28;

/// Callees (the generated aliases; each goes through its override slot, so a
/// replaced callee is reached too).
const GRANT_IS_REWARD_GRANTED: Callee = Callee {
    addr: 0x0015_a2e0,
    f: ee::s01::f_0015a2e0,
};
const GRANT_PICK_LOCKED_REWARD: Callee = Callee {
    addr: 0x0015_7080,
    f: ee::s01::f_00157080,
};
const GRANT_UNLOCK_TRADING_CARD: Callee = Callee {
    addr: 0x0015_80f8,
    f: ee::s01::f_001580f8,
};
const GRANT_UNLOCK_POSTER: Callee = Callee {
    addr: 0x0015_7fd0,
    f: ee::s01::f_00157fd0,
};
const GRANT_UNLOCK_TOY: Callee = Callee {
    addr: 0x0015_8348,
    f: ee::s01::f_00158348,
};
const GRANT_UNLOCK_CONCEPT_ART: Callee = Callee {
    addr: 0x0015_8220,
    f: ee::s01::f_00158220,
};
const GRANT_UNLOCK_RIDER_ITEM: Callee = Callee {
    addr: 0x0015_8618,
    f: ee::s01::f_00158618,
};
const GRANT_ADD_NEW_REWARD: Callee = Callee {
    addr: 0x0015_a628,
    f: ee::s01::f_0015a628,
};
const GRANT_SET_GRANTED_BIT: Callee = Callee {
    addr: 0x0015_a358,
    f: ee::s01::f_0015a358,
};
const GRANT_RIVAL_REWARDS: Callee = Callee {
    addr: 0x0015_8f60,
    f: ee::s01::f_00158f60,
};
const GRANT_PICK_RANDOM_BOLT: Callee = Callee {
    addr: 0x0015_6c70,
    f: ee::s01::f_00156c70,
};
const GRANT_PICK_RANDOM_BOLT_800: Callee = Callee {
    addr: 0x0015_6ee0,
    f: ee::s01::f_00156ee0,
};
const GRANT_BE_LIBRARY_GET_INSTANCE: Callee = Callee {
    addr: 0x0014_dc80,
    f: ee::s01::f_0014dc80,
};
const GRANT_BE_LIBRARY_GET_INTERFACE: Callee = Callee {
    addr: 0x0014_dd58,
    f: ee::s01::f_0014dd58,
};
const GRANT_UNLOCK_BOLT: Callee = Callee {
    addr: 0x0014_b560,
    f: ee::s01::f_0014b560,
};
const GRANT_IS_PEAK_FULLY_COMPLETED: Callee = Callee {
    addr: 0x0015_77e0,
    f: ee::s01::f_001577e0,
};
const GRANT_REWARD: Callee = Callee {
    addr: 0x0015_9cd0,
    f: ee::s01::f_00159cd0,
};
const GRANT_ARE_ALL_PEAKS_COMPLETE: Callee = Callee {
    addr: 0x0015_78a0,
    f: ee::s01::f_001578a0,
};
const GRANT_IS_PEAK_COMPLETE: Callee = Callee {
    addr: 0x0015_7a78,
    f: ee::s01::f_00157a78,
};
const GRANT_MEDAL_REWARD_ID: Callee = Callee {
    addr: 0x0015_9080,
    f: ee::s01::f_00159080,
};
const GRANT_IS_PEAK_GOAL_COMPLETE: Callee = Callee {
    addr: 0x0015_7bf0,
    f: ee::s01::f_00157bf0,
};
const GRANT_MEDAL_REWARD: Callee = Callee {
    addr: 0x0015_9170,
    f: ee::s01::f_00159170,
};

/// `addiu`: 32-bit wrapping add of a constant, sign-extended to 64 bits.
#[inline(always)]
fn grant_addu(a: u64, k: u32) -> u64 {
    sx((a as u32).wrapping_add(k))
}

/// `addu`: 32-bit wrapping add of two registers, sign-extended.
#[inline(always)]
fn grant_addr(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// `addiu` with a negative constant (`- k`), sign-extended.
#[inline(always)]
fn grant_subu(a: u64, k: u32) -> u64 {
    sx((a as u32).wrapping_sub(k))
}

/// `BERewardInt_GrantReward(self, profile, character, bit)` (0x159cd0): grants
/// reward `bit` to one character of a profile. The bit's own unlock runs
/// first (a message for bits 0-4, a random poster, card or toy pick for the
/// card, toy and poster bits, a concept art pick, a peak pass, a bolt or a
/// rider item for the rest), then the granted bit is set and the reward is
/// logged. Finally the mountain, peak and medal awards that the new state
/// completes are granted, recursively, through this function.
///
/// The body is a resumable dispatcher in the generated code: a callee that
/// unwinds the thread returns through the `?` of its call site, and the
/// resume address is picked up by the generated body (the override wrapper
/// sends any entry other than 0x159cd0 there).
pub fn be_reward_int_grant_reward(c: &mut Cpu) {
    guest(c, grant_reward_body)
}

fn grant_reward_body(c: &mut Cpu) -> Flow {
    c.cycles += 47;
    let frame = Frame::open(c, 0xd0);
    c.g[V1][0] = 0x4a_0000;
    frame.save(c, S5, 0x70);
    c.g[V1][0] = grant_addu(c.g[V1][0], 0x6ca8); // V1 = cBELibrary_profileLib
    frame.save(c, S3, 0x90);
    c.g[S5][0] = c.g[A2][0]; // character
    c.g[S3][0] = c.g[A1][0]; // profile
    c.g[A2][0] = 0xf88;
    c.mult(0, 6, 21, 6); // A2 = character * 0xf88
    c.g[A1][0] = 0x9b50;
    c.mult(0, 5, 19, 5); // A1 = profile * 0x9b50
    frame.save(c, S6, 0x60);
    frame.save(c, S0, 0xc0);
    c.g[S6][0] = c.g[A3][0]; // bit
    frame.save(c, S4, 0x80);
    c.g[A3][0] = 0x46_0000;
    c.g[A2][0] = grant_addr(c.g[A2][0], c.g[V1][0]);
    frame.save(c, S7, 0x50);
    frame.save(c, FP, 0x40);
    c.g[V0][0] = 0x4c_0000;
    frame.save(c, S1, 0xb0);
    c.g[S0][0] = grant_addr(c.g[A1][0], c.g[A2][0]); // rec = &profileLib[profile][character]
    frame.save(c, S2, 0xa0);
    c.g[S4][0] = c.g[A0][0]; // self
    frame.save_ra(c, 0x30);
    c.g[FP][0] = grant_addu(c.g[V0][0], 0x3ef0); // the reward log
    c.g[T1][0] = grant_subu(c.g[A3][0], 0x55d8); // GRANT_SLOTS4_SRC
                                                 // Copy the two slot tables to the frame: v[4] at sp+0, v[2] at sp+0x10.
    let slots4 = c.g[T1][0] as u32;
    c.ldl(5, slots4.wrapping_add(0x7));
    c.ldr(5, slots4);
    c.ldl(6, slots4.wrapping_add(0xf));
    c.ldr(6, slots4.wrapping_add(0x8));
    let a = frame.at(c, 0x7);
    c.sdl(5, a);
    let a = frame.at(c, 0x0);
    c.sdr(5, a);
    let a = frame.at(c, 0xf);
    c.sdl(6, a);
    let a = frame.at(c, 0x8);
    c.sdr(6, a);
    c.g[A0][0] = 0xffff_ffff_ffff_ffff;
    c.g[V1][0] = 0x4a_0000;
    c.g[V0][0] = grant_subu(c.g[S6][0], 0x11); // bit - 17
    let a = frame.at(c, 0x20);
    c.sw(a, c.g[A0][0]); // item = -1
    c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64; // bit in 17..=19: no granted check
    c.g[T0][0] = grant_addu(c.g[V1][0], 0x1268); // GRANT_SLOTS2_SRC
    let slots2 = c.g[T0][0] as u32;
    c.ldl(5, slots2.wrapping_add(0x7));
    c.ldr(5, slots2);
    let a = frame.at(c, 0x17);
    c.sdl(5, a);
    let a = frame.at(c, 0x10);
    c.sdr(5, a);
    c.g[S7][0] = 0; // value (the cheat character, 0 for none)
    if c.g[V0][0] == 0 {
        // Not already granted: the bit's own granted check.
        c.cycles += 7;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S5][0];
        c.g[A3][0] = c.g[S6][0];
        GRANT_IS_REWARD_GRANTED.call(c, 0x0015_9da0)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0xc0);
            return grant_epilogue(c, frame);
        }
    }
    grant_bit_dispatch(c, frame)
}

/// The function's epilogue (0x15a2b4): restores the callee-saved registers
/// and returns through the saved `ra`.
fn grant_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 11;
    frame.restore(c, S1, 0xb0);
    frame.restore(c, S2, 0xa0);
    frame.restore(c, S3, 0x90);
    frame.restore(c, S4, 0x80);
    frame.restore(c, S5, 0x70);
    frame.restore(c, S6, 0x60);
    frame.restore(c, S7, 0x50);
    frame.restore(c, FP, 0x40);
    frame.restore_ra(c, 0x30);
    frame.close(c);
    Ok(())
}

/// 0x159da8 and the reward switch (0x159db4): bits 0..31 go to their case,
/// anything above has no unlock and goes straight to the granted bit.
fn grant_bit_dispatch(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = (c.g[S6][0] < 0x20u64) as u64;
    c.g[S1][0] = grant_addu(c.g[S0][0], GRANT_AWARDS_RECEIVED); // &rec->awards
    if c.g[V0][0] == 0 {
        return grant_from_a098(c, frame);
    }
    c.cycles += 7;
    c.g[V0][0] = 0x46_0000;
    c.g[V1][0] = sx((c.g[S6][0] as u32) << 2);
    c.g[V0][0] = grant_subu(c.g[V0][0], 0x55c0); // GRANT_SWITCH_TABLE
    c.g[V1][0] = grant_addr(c.g[V1][0], c.g[V0][0]);
    let entry = c.g[V1][0] as u32;
    c.g[A0][0] = c.lw(entry);
    let target = c.g[A0][0] as u32;
    match target {
        // Bit 0: MessageCenter_PostOnce38 (value 29).
        0x0015_9dd0 => {
            c.cycles += 4;
            c.g[S1][0] = grant_addu(c.g[S0][0], GRANT_AWARDS_RECEIVED);
            call_ptr(c, 0x001e_2578, 0x0015_9dd8)?;
            c.g[S7][0] = 0x1d;
        }
        // Bit 1: MessageCenter_PostOnce3C (value 0x18).
        0x0015_9de0 => {
            c.cycles += 4;
            c.g[S1][0] = grant_addu(c.g[S0][0], GRANT_AWARDS_RECEIVED);
            call_ptr(c, 0x001e_2518, 0x0015_9de8)?;
            c.g[S7][0] = 0x18;
        }
        // Bits 2, 3, 4: MessageCenter_PostPeakUnlockMessage(0, 1, 2).
        0x0015_9df0 => {
            c.cycles += 5;
            c.g[A0][0] = 0;
            c.g[S1][0] = grant_addu(c.g[S0][0], GRANT_AWARDS_RECEIVED);
            call_ptr(c, 0x001e_2428, 0x0015_9dfc)?;
            c.g[S7][0] = 0x12;
        }
        0x0015_9e04 => {
            c.cycles += 5;
            c.g[A0][0] = 1;
            c.g[S1][0] = grant_addu(c.g[S0][0], GRANT_AWARDS_RECEIVED);
            call_ptr(c, 0x001e_2428, 0x0015_9e10)?;
            c.g[S7][0] = 0x19;
        }
        0x0015_9e18 => {
            c.cycles += 5;
            c.g[A0][0] = 2;
            c.g[S1][0] = grant_addu(c.g[S0][0], GRANT_AWARDS_RECEIVED);
            call_ptr(c, 0x001e_2428, 0x0015_9e24)?;
            c.g[S7][0] = 0x1a;
        }
        // Bits 5, 8, 11, 14: trading cards, a poster, a peak pass.
        0x0015_9e2c => return grant_card_bits(c, frame),
        // Bits 6, 9, 12, 15: a toy, concept art, a peak pass.
        0x0015_9f10 => return grant_toy_bits(c, frame),
        // Bit 7: value 0x1b.
        0x0015_9ffc => {
            c.cycles += 3;
            c.g[S1][0] = grant_addu(c.g[S0][0], GRANT_AWARDS_RECEIVED);
            c.g[S7][0] = 0x1b;
        }
        // Bit 10: value 0x17.
        0x0015_a008 => {
            c.cycles += 3;
            c.g[S1][0] = grant_addu(c.g[S0][0], GRANT_AWARDS_RECEIVED);
            c.g[S7][0] = 0x17;
        }
        // Bit 13: value 0x1c.
        0x0015_a014 => {
            c.cycles += 3;
            c.g[S1][0] = grant_addu(c.g[S0][0], GRANT_AWARDS_RECEIVED);
            c.g[S7][0] = 0x1c;
        }
        // Bit 16: value 0x15.
        0x0015_a020 => {
            c.cycles += 3;
            c.g[S1][0] = grant_addu(c.g[S0][0], GRANT_AWARDS_RECEIVED);
            c.g[S7][0] = 0x15;
        }
        // Bits 17, 18, 19: a random bolt of kind 0, 1, 2 (the item).
        0x0015_a02c | 0x0015_a048 | 0x0015_a064 => {
            c.cycles += 7;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S5][0];
            c.g[A3][0] = match target {
                0x0015_a02c => 0,
                0x0015_a048 => 1,
                _ => 2,
            };
            let ret = match target {
                0x0015_a02c => 0x0015_a040,
                0x0015_a048 => 0x0015_a05c,
                _ => 0x0015_a078,
            };
            GRANT_PICK_RANDOM_BOLT.call(c, ret)?;
            c.g[S1][0] = grant_addu(c.g[S0][0], GRANT_AWARDS_RECEIVED);
            return grant_store_item(c, frame);
        }
        // Bits 20..31: a random bolt flagged 0x800 (the item).
        0x0015_a080 => {
            c.cycles += 5;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S5][0];
            c.g[S1][0] = grant_addu(c.g[S0][0], GRANT_AWARDS_RECEIVED);
            GRANT_PICK_RANDOM_BOLT_800.call(c, 0x0015_a094)?;
            return grant_store_item(c, frame);
        }
        _ => {
            c.jump_out(target, GRANT_ENTRY, GRANT_END);
            return Ok(());
        }
    }
    grant_from_a098(c, frame)
}

/// 0x15a094: the bolt pick's result becomes the item (`sp+0x20`), then the
/// granted bit (0x15a098).
fn grant_store_item(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    let a = frame.at(c, 0x20);
    c.sw(a, c.g[V0][0]);
    grant_from_a098(c, frame)
}

/// Bits 5, 8, 11, 14 (0x159e2c): up to four trading cards (each a random
/// locked card, kept in `v[i]`), then one poster, then the peak pass for
/// bit 11 (MessageCenter_CheckPeakMessages) and GrantRivalRewards.
fn grant_card_bits(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[S2][0] = 0;
    c.g[S1][0] = grant_addu(c.g[S0][0], GRANT_AWARDS_RECEIVED);
    loop {
        // Loop test: i < 4.
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S2][0] as i64) < 4) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A3][0] = 4;
        if !more {
            break;
        }
        // v[i] = random locked card; the loop ends at the first -1.
        c.cycles += 9;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S5][0];
        GRANT_PICK_LOCKED_REWARD.call(c, 0x0015_9e80)?;
        c.g[A3][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
        let slot = frame.at(c, 0);
        c.g[S0][0] = sx(slot.wrapping_add(c.g[V0][0] as u32));
        let found = (c.g[A3][0] as i64) >= 0;
        let s0 = c.g[S0][0] as u32;
        c.sw(s0, c.g[A3][0]);
        if !found {
            break;
        }
        if back_edge_unwinds(c, 0x0015_9e38) {
            return Err(Unwound);
        }
        // Unlock the card, log it.
        c.cycles += 11;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S5][0];
        c.g[T0][0] = 0;
        GRANT_UNLOCK_TRADING_CARD.call(c, 0x0015_9e4c)?;
        c.g[S2][0] = grant_addu(c.g[S2][0], 1);
        let s0 = c.g[S0][0] as u32;
        c.g[A3][0] = c.lw(s0);
        c.g[A0][0] = c.g[FP][0];
        c.g[A1][0] = c.g[S6][0];
        c.g[A2][0] = 4;
        GRANT_ADD_NEW_REWARD.call(c, 0x0015_9e64)?;
    }
    // One poster (kind 3).
    c.cycles += 8;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S5][0];
    c.g[A3][0] = 3;
    GRANT_PICK_LOCKED_REWARD.call(c, 0x0015_9ea8)?;
    c.g[S0][0] = c.g[V0][0];
    let no_poster = (c.g[S0][0] as i64) < 0;
    c.g[A0][0] = c.g[S4][0];
    if !no_poster {
        c.cycles += 10;
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S5][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = 0;
        GRANT_UNLOCK_POSTER.call(c, 0x0015_9ec8)?;
        c.g[A3][0] = c.g[S0][0];
        c.g[A0][0] = c.g[FP][0];
        c.g[A1][0] = c.g[S6][0];
        c.g[A2][0] = 3;
        GRANT_ADD_NEW_REWARD.call(c, 0x0015_9edc)?;
    }
    // Bit 11 also runs the peak messages.
    c.cycles += 3;
    c.g[V0][0] = 0xb;
    let not_eleven = c.g[S6][0] != c.g[V0][0];
    c.g[A0][0] = c.g[S4][0];
    if !not_eleven {
        c.cycles += 3;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x001e_38b8, 0x0015_9ef0)?;
        c.g[A0][0] = c.g[S4][0];
    }
    // GrantRivalRewards(self, profile, character, bit, 1).
    c.cycles += 7;
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S5][0];
    c.g[A3][0] = c.g[S6][0];
    c.g[T0][0] = 1;
    GRANT_RIVAL_REWARDS.call(c, 0x0015_9f08)?;
    c.g[A2][0] = c.g[S1][0];
    grant_from_a09c(c, frame)
}

/// Bits 6, 9, 12, 15 (0x159f10): one toy (kind 7), up to two concept art
/// pieces (kind 5), the peak messages for bit 12, then GrantRivalRewards.
fn grant_toy_bits(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 8;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S5][0];
    c.g[A3][0] = 7;
    GRANT_PICK_LOCKED_REWARD.call(c, 0x0015_9f24)?;
    c.g[S1][0] = c.g[V0][0];
    let no_toy = (c.g[S1][0] as i64) < 0;
    c.g[A0][0] = c.g[S4][0];
    if !no_toy {
        c.cycles += 10;
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S5][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = 0;
        GRANT_UNLOCK_TOY.call(c, 0x0015_9f44)?;
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[FP][0];
        c.g[A1][0] = c.g[S6][0];
        c.g[A2][0] = 7;
        GRANT_ADD_NEW_REWARD.call(c, 0x0015_9f58)?;
    }
    c.cycles += 3;
    c.g[S2][0] = 0;
    c.g[S1][0] = grant_addu(c.g[S0][0], GRANT_AWARDS_RECEIVED);
    loop {
        // Loop test: i < 2.
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S2][0] as i64) < 2) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A3][0] = 5;
        if !more {
            break;
        }
        // slots2[i] = random locked concept art; the loop ends at the first -1.
        c.cycles += 10;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S5][0];
        GRANT_PICK_LOCKED_REWARD.call(c, 0x0015_9fb0)?;
        c.g[A3][0] = c.g[V0][0];
        c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
        c.g[V0][0] = grant_addu(c.g[SP][0], 0x10);
        c.g[S0][0] = grant_addr(c.g[V0][0], c.g[V1][0]);
        let found = (c.g[A3][0] as i64) >= 0;
        let s0 = c.g[S0][0] as u32;
        c.sw(s0, c.g[A3][0]);
        if !found {
            break;
        }
        if back_edge_unwinds(c, 0x0015_9f68) {
            return Err(Unwound);
        }
        // Unlock the art, log it.
        c.cycles += 11;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S5][0];
        c.g[T0][0] = 0;
        GRANT_UNLOCK_CONCEPT_ART.call(c, 0x0015_9f7c)?;
        c.g[S2][0] = grant_addu(c.g[S2][0], 1);
        let s0 = c.g[S0][0] as u32;
        c.g[A3][0] = c.lw(s0);
        c.g[A0][0] = c.g[FP][0];
        c.g[A1][0] = c.g[S6][0];
        c.g[A2][0] = 5;
        GRANT_ADD_NEW_REWARD.call(c, 0x0015_9f94)?;
    }
    // Bit 12 also runs the peak messages.
    c.cycles += 3;
    c.g[V0][0] = 0xc;
    let not_twelve = c.g[S6][0] != c.g[V0][0];
    c.g[A0][0] = c.g[S4][0];
    if !not_twelve {
        c.cycles += 3;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x001e_38b8, 0x0015_9fdc)?;
        c.g[A0][0] = c.g[S4][0];
    }
    // GrantRivalRewards(self, profile, character, bit, 2).
    c.cycles += 7;
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S5][0];
    c.g[A3][0] = c.g[S6][0];
    c.g[T0][0] = 2;
    GRANT_RIVAL_REWARDS.call(c, 0x0015_9ff4)?;
    c.g[A2][0] = c.g[S1][0];
    grant_from_a09c(c, frame)
}

/// 0x15a098: the granted-bit call's argument is the awards pointer.
fn grant_from_a098(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.g[A2][0] = c.g[S1][0];
    grant_from_a09c(c, frame)
}

/// 0x15a09c: sets the granted bit, logs the reward and the cheat character
/// (or the item) when there is one, then the rest of the function.
fn grant_from_a09c(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S6][0];
    GRANT_SET_GRANTED_BIT.call(c, 0x0015_a0a8)?;
    c.g[A0][0] = c.g[FP][0];
    if c.g[S7][0] != 0 {
        c.cycles += 10;
        c.g[A1][0] = c.g[S6][0];
        c.g[A2][0] = 8;
        c.g[A3][0] = c.g[S7][0];
        GRANT_ADD_NEW_REWARD.call(c, 0x0015_a0c0)?;
        c.g[A3][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S5][0];
        c.g[T0][0] = 0;
        GRANT_UNLOCK_RIDER_ITEM.call(c, 0x0015_a0d8)?;
    }
    grant_item_tail(c, frame)
}

/// 0x15a0d8: when the switch picked an item (`sp+0x20` is not -1), logs it
/// and unlocks the bolt through the library's stats interface.
fn grant_item_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    let item_slot = frame.at(c, 0x20);
    c.g[T1][0] = c.lw(item_slot);
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    let no_item = c.g[T1][0] == c.g[V0][0];
    c.g[A0][0] = c.g[S4][0];
    if no_item {
        return grant_no_item_tail(c, frame);
    }
    c.cycles += 18;
    GRANT_BE_LIBRARY_GET_INSTANCE.call(c, 0x0015_a0f0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 9;
    GRANT_BE_LIBRARY_GET_INTERFACE.call(c, 0x0015_a0fc)?;
    let item_slot = frame.at(c, 0x20);
    c.g[A3][0] = c.lw(item_slot);
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[FP][0];
    c.g[A1][0] = c.g[S6][0];
    c.g[A2][0] = 9;
    GRANT_ADD_NEW_REWARD.call(c, 0x0015_a114)?;
    let item_slot = frame.at(c, 0x20);
    c.g[A3][0] = c.lw(item_slot);
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S5][0];
    c.g[T0][0] = 0;
    GRANT_UNLOCK_BOLT.call(c, 0x0015_a12c)?;
    c.g[A0][0] = c.g[S4][0];
    grant_no_item_tail(c, frame)
}

/// 0x15a130: the peak-completion awards (the mountain, then the peaks).
fn grant_no_item_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S5][0];
    GRANT_IS_PEAK_FULLY_COMPLETED.call(c, 0x0015_a13c)?;
    c.g[A0][0] = c.g[S4][0];
    if c.g[V0][0] == 0 {
        return grant_mountain_completed(c, frame);
    }
    // The mountain is conquered: grant bit 0 unless it is granted already.
    c.cycles += 6;
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S5][0];
    c.g[A3][0] = 0;
    GRANT_IS_REWARD_GRANTED.call(c, 0x0015_a154)?;
    c.g[A0][0] = c.g[S4][0];
    if c.g[V0][0] != 0 {
        return grant_all_peaks(c, frame);
    }
    c.cycles += 4;
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S5][0];
    c.g[A3][0] = 0;
    GRANT_REWARD.call(c, 0x0015_a16c)?;
    grant_mountain_completed(c, frame)
}

/// 0x15a16c: the mountain-completed check (`ARE_ALL_PEAKS_COMPLETE`), grant
/// bit 1 when it holds and bit 1 is not granted yet.
fn grant_mountain_completed(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.g[A0][0] = c.g[S4][0];
    grant_all_peaks(c, frame)
}

/// 0x15a170: `AreAllPeaksComplete`: grant bit 1 when it holds and is not
/// granted yet, then the per-peak awards.
fn grant_all_peaks(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S5][0];
    GRANT_ARE_ALL_PEAKS_COMPLETE.call(c, 0x0015_a17c)?;
    c.g[A0][0] = c.g[S4][0];
    if c.g[V0][0] == 0 {
        c.cycles += 2; // 0x15a1b0
        c.g[S1][0] = 0;
        return grant_after_peaks_loop(c, frame);
    }
    c.cycles += 6;
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S5][0];
    c.g[A3][0] = 1;
    GRANT_IS_REWARD_GRANTED.call(c, 0x0015_a194)?;
    c.g[S1][0] = 0;
    if c.g[V0][0] != 0 {
        return grant_after_peaks_loop(c, frame);
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S5][0];
    c.g[A3][0] = 1;
    GRANT_REWARD.call(c, 0x0015_a1b0)?;
    c.cycles += 2; // 0x15a1b0
    c.g[S1][0] = 0;
    grant_after_peaks_loop(c, frame)
}

/// 0x15a1b8 and the three peaks (0x15a1bc): each completed peak grants its
/// award (bit i + 2) when that is not granted yet; then the medal rewards.
fn grant_after_peaks_loop(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.g[A0][0] = c.g[S4][0];
    loop {
        c.cycles += 6;
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S5][0];
        c.g[A3][0] = c.g[S1][0];
        GRANT_IS_PEAK_COMPLETE.call(c, 0x0015_a1cc)?;
        c.g[S0][0] = grant_addu(c.g[S1][0], 2);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[S1][0] = grant_addu(c.g[S1][0], 1);
        } else {
            c.cycles += 7;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S5][0];
            c.g[A3][0] = c.g[S0][0];
            GRANT_IS_REWARD_GRANTED.call(c, 0x0015_a1e8)?;
            if c.g[V0][0] != 0 {
                c.g[S1][0] = grant_addu(c.g[S1][0], 1);
            } else {
                c.cycles += 5;
                c.g[A3][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = c.g[S5][0];
                GRANT_REWARD.call(c, 0x0015_a204)?;
                c.cycles += 1;
                c.g[S1][0] = grant_addu(c.g[S1][0], 1);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S1][0] as i64) < 3) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S4][0];
        if !more {
            break;
        }
        if back_edge_unwinds(c, 0x0015_a1bc) {
            return Err(Unwound);
        }
    }
    grant_medal_loop(c, frame)
}

/// 0x15a214 .. 0x15a2b0: for each of the three medal kinds (S2) and each of
/// its four tiers (S1), the medal reward (`GetMedalRewardId`) is granted
/// when its peak goal is complete and the bit is not granted yet.
fn grant_medal_loop(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[S2][0] = 0;
    c.g[S1][0] = 0;
    loop {
        // Kind loop head.
        c.cycles += 2;
        c.g[S6][0] = grant_addu(c.g[S2][0], 1);
        c.g[A0][0] = c.g[S4][0];
        loop {
            // Tier loop body.
            c.cycles += 14;
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S5][0];
            c.g[A3][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S1][0];
            GRANT_MEDAL_REWARD_ID.call(c, 0x0015_a23c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S5][0];
            c.g[A3][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S1][0];
            GRANT_IS_PEAK_GOAL_COMPLETE.call(c, 0x0015_a258)?;
            let goal_missing = c.g[V0][0] == 0;
            c.g[A3][0] = c.g[S0][0];
            if goal_missing {
                c.cycles += 1;
                c.g[S1][0] = grant_addu(c.g[S1][0], 1);
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = c.g[S5][0];
                GRANT_IS_REWARD_GRANTED.call(c, 0x0015_a270)?;
                if c.g[V0][0] != 0 {
                    c.g[S1][0] = grant_addu(c.g[S1][0], 1);
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A2][0] = c.g[S5][0];
                    c.g[A3][0] = c.g[S2][0];
                    c.g[T0][0] = c.g[S1][0];
                    GRANT_MEDAL_REWARD.call(c, 0x0015_a290)?;
                    c.cycles += 1;
                    c.g[S1][0] = grant_addu(c.g[S1][0], 1);
                }
            }
            // Tier loop test: i < 4.
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 4) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S4][0];
            if !more {
                break;
            }
            if back_edge_unwinds(c, 0x0015_a228) {
                return Err(Unwound);
            }
        }
        // Kind loop test: S2 + 1 < 3 (S2 is the next kind).
        c.cycles += 4;
        c.g[S2][0] = c.g[S6][0];
        c.g[V0][0] = ((c.g[S2][0] as i64) < 3) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S1][0] = 0;
        if !more {
            break;
        }
        if back_edge_unwinds(c, 0x0015_a220) {
            return Err(Unwound);
        }
    }
    // 0x15a2b0: restore the first callee-saved register, then the epilogue.
    c.cycles += 1;
    frame.restore(c, S0, 0xc0);
    grant_epilogue(c, frame)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::machine;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    /// `cBELibrary_profileLib` (`[profile][character]`, 0x9b50 per profile,
    /// 0xf88 per character).
    const PROFILE_LIB: u32 = 0x004a_6ca8;
    const PROFILE_STRIDE: u32 = 0x9b50;
    const CHAR_STRIDE: u32 = 0xf88;

    /// Fixture: the game image (jump table, slot tables, `.lit4`), the back
    /// end singletons at gp-0x1f04 / gp-0x1f28 as zeroed objects, and a
    /// character record whose award bits and owned-reward bits are random,
    /// so the pickers scan real bit arrays.
    fn grant_world(c: &mut Cpu, r: &mut Rng, profile: u32, character: u32) {
        elf_image(c);
        for off in [0x1f04, 0x1f28] {
            c.bus.write32(GAME_GP - off, DATA + 0x20_0000 + off * 0x10);
        }
        c.bus.write32(GAME_GP - 0x1efc, 0);
        let rec = PROFILE_LIB + profile * PROFILE_STRIDE + character * CHAR_STRIDE;
        for i in 0..0x40u32 {
            c.bus
                .write8(rec + GRANT_AWARDS_RECEIVED + i, r.next_u32() as u8 & 0x0f);
        }
    }

    /// `BERewardInt_GrantReward` against the generated body: every bit (the
    /// per-bit switch cases, the 17..=19 skip, the >= 32 path), with the
    /// picks, the unlocks and the medal and peak checks answered by the
    /// fakes, which unwind at random call indices.
    #[test]
    fn be_reward_int_grant_reward_matches() {
        check(
            ee::s01::f_00159cd0__generated,
            be_reward_int_grant_reward,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0159_cd0, 12);
                let profile = seed % 3;
                let character = (seed / 3) % 4;
                let bit = match seed % 5 {
                    0 => 17 + r.next_u32() % 3,
                    1 => r.next_u32() % 40,
                    _ => r.next_u32() % 20,
                };
                grant_world(c, &mut r, profile, character);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = profile as u64;
                c.g[A2][0] = character as u64;
                c.g[A3][0] = bit as u64;
                c.pc = GRANT_ENTRY;
            },
        );
    }
}
