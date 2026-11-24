//! Staging module oc488 (wave wave227): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc488`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0015_7fd0,
        reward_unlock_poster,
        "BERewardInt_UnlockPoster",
    ),
    (
        0x0015_80f8,
        reward_unlock_trading_card,
        "BERewardInt_UnlockTradingCard",
    ),
    (
        0x0015_8220,
        reward_unlock_concept_art,
        "BERewardInt_UnlockConceptArt",
    ),
    (0x0015_8348, reward_unlock_toy, "BERewardInt_UnlockToy"),
    (0x0015_8558, reward_unlock_song, "BERewardInt_UnlockSong"),
    (
        0x0015_88b0,
        be_reward_int_get_points_threshold,
        "BERewardInt_GetPointsThreshold",
    ),
    (
        0x0015_9c68,
        reward_grant_reward_for_rider,
        "BERewardInt_GrantRewardForRider",
    ),
    (0x0015_aec0, rewards_file_poll_load, "RewardsFile_PollLoad"),
    (
        0x0015_bc10,
        be_assign_global_string_1bc8,
        "BE_AssignGlobalString1bc8",
    ),
    (
        0x0015_bc30,
        be_copy_global_string_1bc8,
        "BE_CopyGlobalString1bc8",
    ),
    (
        0x0015_c188,
        be_set_name_entry_535617,
        "BE_SetNameEntry535617",
    ),
    (0x0015_c338, be_set_string_534d48, "BE_SetString534d48"),
    (0x0015_c368, be_set_string_534dc0, "BE_SetString534dc0"),
    (0x0015_c398, be_set_string_534e38, "BE_SetString534e38"),
];

/// The rewards file (`BERewardInt` file block, `REWARDS_FILE + 0x3e98`).
const REWARDS_FILE_PTR: u32 = 0x004c_3e98;

/// One reward unlock: the shape the seven `BERewardInt_Unlock*` bodies share.
/// Posters, trading cards, concept art and toys also count the unlocked
/// entries and grant the rider-item bonus when the counts agree; videos and
/// rider items go straight from the award to the money query.
#[derive(Clone, Copy)]
struct RewardUnlock {
    /// Rewards-file offset of the bit table (`lw` in the award branch).
    table: u32,
    /// Offset added to the bit index for `BERewardInt_SetBit`.
    bit_off: u32,
    /// Rider items: the index is `a3 - 10`.
    index_adj: bool,
    /// The count block, for the unlocks that have one.
    count: Option<RewardCount>,
    ret_inst: u32,
    ret_iface: u32,
    ret_setbit: u32,
    ret_file: u32,
    ret_spend: u32,
    ret_money: u32,
}

/// The count block: the unlocked count against the total, then the
/// rider-item unlock when they agree.
#[derive(Clone, Copy)]
struct RewardCount {
    count_fn: u32,
    ret_count: u32,
    get_fn: u32,
    ret_get: u32,
    /// The `a3` of the rider-item unlock made when the counts agree.
    rider_idx: u64,
    ret_rider: u32,
}

const UNLOCK_POSTER: RewardUnlock = RewardUnlock {
    table: 0x14,
    bit_off: 0xf30,
    index_adj: false,
    count: Some(RewardCount {
        count_fn: 0x0015_75c0,
        ret_count: 0x0015_8094,
        get_fn: 0x0015_72b0,
        ret_get: 0x0015_80a0,
        rider_idx: 0x14,
        ret_rider: 0x0015_80c0,
    }),
    ret_inst: 0x0015_8008,
    ret_iface: 0x0015_8014,
    ret_setbit: 0x0015_8048,
    ret_file: 0x0015_8058,
    ret_spend: 0x0015_8084,
    ret_money: 0x0015_80d0,
};

const UNLOCK_TRADING_CARD: RewardUnlock = RewardUnlock {
    table: 0x18,
    bit_off: 0xf36,
    index_adj: false,
    count: Some(RewardCount {
        count_fn: 0x0015_7620,
        ret_count: 0x0015_81bc,
        get_fn: 0x0015_72d0,
        ret_get: 0x0015_81c8,
        rider_idx: 0x11,
        ret_rider: 0x0015_81e8,
    }),
    ret_inst: 0x0015_8130,
    ret_iface: 0x0015_813c,
    ret_setbit: 0x0015_8170,
    ret_file: 0x0015_8180,
    ret_spend: 0x0015_81ac,
    ret_money: 0x0015_81f8,
};

const UNLOCK_CONCEPT_ART: RewardUnlock = RewardUnlock {
    table: 0x1c,
    bit_off: 0xf45,
    index_adj: false,
    count: Some(RewardCount {
        count_fn: 0x0015_7680,
        ret_count: 0x0015_82e4,
        get_fn: 0x0015_72f0,
        ret_get: 0x0015_82f0,
        rider_idx: 0x16,
        ret_rider: 0x0015_8310,
    }),
    ret_inst: 0x0015_8258,
    ret_iface: 0x0015_8264,
    ret_setbit: 0x0015_8298,
    ret_file: 0x0015_82a8,
    ret_spend: 0x0015_82d4,
    ret_money: 0x0015_8320,
};

const UNLOCK_TOY: RewardUnlock = RewardUnlock {
    table: 0x20,
    bit_off: 0xf53,
    index_adj: false,
    count: Some(RewardCount {
        count_fn: 0x0015_76e0,
        ret_count: 0x0015_840c,
        get_fn: 0x0015_7310,
        ret_get: 0x0015_8418,
        rider_idx: 0x13,
        ret_rider: 0x0015_8438,
    }),
    ret_inst: 0x0015_8380,
    ret_iface: 0x0015_838c,
    ret_setbit: 0x0015_83c0,
    ret_file: 0x0015_83d0,
    ret_spend: 0x0015_83fc,
    ret_money: 0x0015_8448,
};

const UNLOCK_VIDEO: RewardUnlock = RewardUnlock {
    table: 0x24,
    bit_off: 0xf52,
    index_adj: false,
    count: None,
    ret_inst: 0x0015_84a8,
    ret_iface: 0x0015_84b4,
    ret_setbit: 0x0015_84e8,
    ret_file: 0x0015_84f8,
    ret_spend: 0x0015_8524,
    ret_money: 0x0015_8534,
};

const UNLOCK_RIDER_ITEM: RewardUnlock = RewardUnlock {
    table: 0x28,
    bit_off: 0xf57,
    index_adj: true,
    count: None,
    ret_inst: 0x0015_8650,
    ret_iface: 0x0015_865c,
    ret_setbit: 0x0015_8690,
    ret_file: 0x0015_86a0,
    ret_spend: 0x0015_86cc,
    ret_money: 0x0015_86dc,
};

/// `void BERewardInt_UnlockPoster(self, profile, c, idx, award)` (0x157fd0).
pub fn reward_unlock_poster(c: &mut Cpu) {
    guest(c, |c| reward_unlock_body(c, UNLOCK_POSTER))
}

/// `void BERewardInt_UnlockTradingCard(...)` (0x1580f8).
pub fn reward_unlock_trading_card(c: &mut Cpu) {
    guest(c, |c| reward_unlock_body(c, UNLOCK_TRADING_CARD))
}

/// `void BERewardInt_UnlockConceptArt(...)` (0x158220).
pub fn reward_unlock_concept_art(c: &mut Cpu) {
    guest(c, |c| reward_unlock_body(c, UNLOCK_CONCEPT_ART))
}

/// `void BERewardInt_UnlockToy(...)` (0x158348).
pub fn reward_unlock_toy(c: &mut Cpu) {
    guest(c, |c| reward_unlock_body(c, UNLOCK_TOY))
}

/// `void BERewardInt_UnlockVideo(...)` (0x158470).
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn reward_unlock_video(c: &mut Cpu) {
    guest(c, |c| reward_unlock_body(c, UNLOCK_VIDEO))
}

/// `void BERewardInt_UnlockRiderItem(...)` (0x158618).
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn reward_unlock_rider_item(c: &mut Cpu) {
    guest(c, |c| reward_unlock_body(c, UNLOCK_RIDER_ITEM))
}

/// The shared body of the unlocks. Register roles as the generated code
/// reads them: `s0` = award flag (`t0`), `s1` = profile (`a1`), `s2` = self
/// (`a0`), `s3` = entry index (`a2`), `s4` = bit index, `s5` = econ interface.
pub(crate) fn reward_unlock_body(c: &mut Cpu, u: RewardUnlock) -> Flow {
    c.cycles += 32;
    let frame = Frame::open(c, 0x70);
    frame.save(c, S0, 0x60);
    frame.save(c, S1, 0x50);
    frame.save(c, S2, 0x40);
    frame.save(c, S3, 0x30);
    frame.save(c, S4, 0x20);
    frame.save(c, S5, 0x10);
    frame.save_ra(c, 0);
    let has_count = u.count.is_some();
    c.g[S0][0] = c.g[T0][0];
    c.g[S1][0] = c.g[A1][0];
    c.g[S2][0] = c.g[A0][0];
    c.g[S3][0] = c.g[A2][0];
    c.g[S4][0] = if u.index_adj {
        sx(r32(c, A3).wrapping_sub(0xa))
    } else {
        c.g[A3][0]
    };

    call_ptr(c, 0x0014_dc80, u.ret_inst)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xb;
    call_ptr(c, 0x0014_dd58, u.ret_iface)?;

    // econ = interface; entry = 0x4a6ca8 + profile * 0x9b50 + index * 0xf88.
    c.g[V1][0] = 0xf88;
    c.g[A2][0] = 0x9b50;
    c.mult(0, V1, S3, V1);
    c.g[S5][0] = c.g[V0][0];
    c.mult(0, A2, S1, A2);
    c.g[V0][0] = 0x4a_0000;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6ca8));
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S4][0];
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, V1)));
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(u.bit_off));
    call_ptr(c, 0x0015_73f0, u.ret_setbit)?;

    let no_award = c.g[S0][0] == 0;
    c.g[A0][0] = if has_count { c.g[S2][0] } else { c.g[S5][0] };
    if !no_award {
        c.cycles += 8;
        call_ptr(c, 0x0015_a478, u.ret_file)?;
        c.g[V1][0] = Addr(r32(c, V0)).off(u.table).lw(c);
        c.g[V0][0] = sx(r32(c, S4) << 4);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A3][0] = Addr(r32(c, V1)).off(0xc).lh(c);
        let not_scored = (c.g[A3][0] as i64) <= 0;
        c.g[V0][0] = 0xa;
        if !not_scored {
            c.cycles += 1;
            c.mult(0, A3, A3, V0);
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S3][0];
        call_ptr(c, 0x0015_0b48, u.ret_spend)?;
        c.g[A0][0] = if has_count { c.g[S2][0] } else { c.g[S5][0] };
    }

    if let Some(cnt) = u.count {
        c.cycles += 8;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S3][0];
        call_ptr(c, cnt.count_fn, cnt.ret_count)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, cnt.get_fn, cnt.ret_get)?;
        let differs = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = c.g[S5][0];
        if !differs {
            c.cycles += 7;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = cnt.rider_idx;
            c.g[T0][0] = 0;
            call_ptr(c, 0x0015_8618, cnt.ret_rider)?;
            c.g[A0][0] = c.g[S5][0];
        }
    }

    c.cycles += 12;
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = c.g[S3][0];
    call_ptr(c, 0x0015_0928, u.ret_money)?;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void BERewardInt_UnlockSong(self, profile, c, bit, award)` (0x158558): set
/// bit `a3` of the song word at the entry, and spend the award when `t0` is set.
pub fn reward_unlock_song(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A3][0];
        c.g[S1][0] = c.g[T0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A2][0];
        call_ptr(c, 0x0014_dc80, 0x0015_8588)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x0014_dd58, 0x0015_8594)?;
        c.g[A1][0] = 0xf88;
        c.g[A0][0] = 0x9b50;
        c.mult(0, A1, S3, A1);
        c.g[S4][0] = c.g[V0][0];
        c.mult(0, A0, S2, A0);
        c.g[V0][0] = 0x4a_0000;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6ca8));
        c.g[V1][0] = 1;
        c.g[V1][0] = c.g[V1][0] << (c.g[S0][0] & 63);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        c.g[V0][0] = Addr(r32(c, A0)).off(0xf70).ld(c);
        c.g[V0][0] |= c.g[V1][0];
        let no_award = c.g[S1][0] == 0;
        let word = c.g[V0][0];
        Addr(r32(c, A0)).off(0xf70).sd(c, word);
        if !no_award {
            c.cycles += 5;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = 0x1388;
            call_ptr(c, 0x0015_0b48, 0x0015_85e4)?;
        }
        c.cycles += 12;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S3][0];
        call_ptr(c, 0x0015_0928, 0x0015_85f4)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BERewardInt_GetPointsThreshold(self, tier)` (0x1588b0): the word of
/// the threshold table at `0x45b048 + tier * 4`.
pub fn be_reward_int_get_points_threshold(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x46_0000;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4fb8));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
    ret(c);
}

/// `void BERewardInt_AddPointsForRider(self, rider, points, ...)` (0x1597b0):
/// look up the rider's profile and character, then `AddPoints` with them.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn reward_add_points_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A2][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a0e0, 0x0015_97dc)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a080, 0x0015_97e8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, 0x0015_9818, 0x0015_97fc)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BERewardInt_GrantRewardForRider(self, rider, ...)` (0x159c68): as
/// `AddPointsForRider`, but grants the reward.
pub fn reward_grant_reward_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A2][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a0e0, 0x0015_9c94)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a080, 0x0015_9ca0)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, 0x0015_9cd0, 0x0015_9cb4)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RewardsFile_PollLoad(RewardsFile *self)` (0x15aec0): when the file
/// is not parsed yet and the loader's handle is ready, parse it; the status
/// word becomes -1 once it is done.
pub fn rewards_file_poll_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        let parsed = (c.g[A2][0] as i64) < 0;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        if !parsed {
            c.cycles += 7;
            c.g[A1][0] = 0;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x7c)).lw(c);
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a70));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0019_e238, 0x0015_aef4)?;
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !idle {
                c.cycles += 12;
                c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[A1][0] = 0;
                call_ptr(c, 0x0019_e330, 0x0015_af08)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0015_a818, 0x0015_af14)?;
                c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0019_dc20, 0x0015_af24)?;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                let done = c.g[V0][0];
                Addr(c.g[S1][0] as u32).sw(c, done);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_AssignGlobalString1bc8(void)` (0x15bc10): the global string at
/// `gp - 0x...1bc8` is passed to the string-assign helper.
pub fn be_assign_global_string_1bc8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1bc8));
        frame.save_ra(c, 0);
        call_ptr(c, 0x0031_84e8, 0x0015_bc24)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_CopyGlobalString1bc8(void *dst)` (0x15bc30): copies the global
/// string at `gp + 0x1bc8` into `dst`, returns `dst`.
pub fn be_copy_global_string_1bc8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1bc8));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0031_7fe8, 0x0015_bc48)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_SetNameEntry535617(int index, int a1, int a2)` (0x15c188): the
/// name slot `0x530000 + 0x5617 + index * 0x40`, through the setter.
pub fn be_set_name_entry_535617(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[A0][0] = 0x53_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 6);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5617));
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, 0x0041_66f4, 0x0015_c1a8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_SetString534d48(void)` (0x15c338): the global string at
/// `0x530000 + 0x4d48` through the string setter.
pub fn be_set_string_534d48(c: &mut Cpu) {
    set_global_string(c, 0x4d48, 0x0015_c34c);
}

/// `void BE_SetString534dc0(void)` (0x15c368): as `534d48`, at `0x4dc0`.
pub fn be_set_string_534dc0(c: &mut Cpu) {
    set_global_string(c, 0x4dc0, 0x0015_c37c);
}

/// `void BE_SetString534e38(void)` (0x15c398): as `534d48`, at `0x4e38`.
pub fn be_set_string_534e38(c: &mut Cpu) {
    set_global_string(c, 0x4e38, 0x0015_c3ac);
}

/// The shared body of the three `BE_SetString*` wrappers: `setter(0x530000 + off)`.
fn set_global_string(c: &mut Cpu, off: u32, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x53_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(off));
        call_ptr(c, 0x0041_66f4, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    /// Scrambled machine, the BELibrary dummy at `gp-0x1f04` and a
    /// rewards file whose bit table at `table` points at a record array.
    pub(crate) fn unlock_fixture(c: &mut Cpu, seed: u32, salt: u64, table: u32) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
        c.bus.write32(REWARDS_FILE_PTR + table, DATA + 0x5000);
        for k in 0..0x40u32 {
            c.bus.write16(
                DATA + 0x5000 + 16 * k + 0xc,
                ((k * 7 + seed) % 5) as u16 * 3,
            );
        }
        r
    }

    /// Unlock inputs: `self` at DATA, small profile / entry / bit values and
    /// the award flag from the seed.
    pub(crate) fn unlock_inputs(c: &mut Cpu, seed: u32, entry: u32, idx_base: u64) {
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = (seed % 3) as u64;
        c.g[A2][0] = (seed / 3 % 4) as u64;
        c.g[A3][0] = idx_base + (seed % 0x10) as u64;
        c.g[T0][0] = (seed % 2) as u64;
        c.pc = entry;
    }

    #[test]
    fn reward_unlock_poster_matches() {
        check(
            ee::s01::f_00157fd0__generated,
            reward_unlock_poster,
            120,
            &|c, seed| {
                let _ = unlock_fixture(c, seed, 0x157f_d0, 0x14);
                unlock_inputs(c, seed, 0x0015_7fd0, 0);
            },
        );
    }

    #[test]
    fn reward_unlock_trading_card_matches() {
        check(
            ee::s01::f_001580f8__generated,
            reward_unlock_trading_card,
            120,
            &|c, seed| {
                let _ = unlock_fixture(c, seed, 0x1580_f8, 0x18);
                unlock_inputs(c, seed, 0x0015_80f8, 0);
            },
        );
    }

    #[test]
    fn reward_unlock_concept_art_matches() {
        check(
            ee::s01::f_00158220__generated,
            reward_unlock_concept_art,
            120,
            &|c, seed| {
                let _ = unlock_fixture(c, seed, 0x158220, 0x1c);
                unlock_inputs(c, seed, 0x0015_8220, 0);
            },
        );
    }

    #[test]
    fn reward_unlock_toy_matches() {
        check(ee::s01::f_00158348__generated, reward_unlock_toy, 120, &|c, seed| {
            let _ = unlock_fixture(c, seed, 0x158348, 0x20);
            unlock_inputs(c, seed, 0x0015_8348, 0);
        });
    }

    #[test]
    fn reward_unlock_song_matches() {
        check(ee::s01::f_00158558__generated, reward_unlock_song, 120, &|c, seed| {
            let _ = unlock_fixture(c, seed, 0x158558, 0x24);
            unlock_inputs(c, seed, 0x0015_8558, 0);
            // The bit index runs past 63 on some seeds (the shift masks it).
            c.g[A3][0] = (seed as u64).wrapping_mul(7) % 70;
        });
    }

    #[test]
    fn be_reward_int_get_points_threshold_matches() {
        check(
            ee::s01::f_001588b0__generated,
            be_reward_int_get_points_threshold,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1588b0, 0);
                c.g[A1][0] = (seed % 64) as u64;
                c.pc = 0x0015_88b0;
            },
        );
    }

    #[test]
    fn reward_grant_reward_for_rider_matches() {
        check(
            ee::s01::f_00159c68__generated,
            reward_grant_reward_for_rider,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x159c68, 0);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1_0000);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = (seed % 7) as u64;
                c.pc = 0x0015_9c68;
            },
        );
    }

    #[test]
    fn rewards_file_poll_load_matches() {
        check(
            ee::s01::f_0015aec0__generated,
            rewards_file_poll_load,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x15aec0, 0);
                // The loader's world: game at +0x7c, shape manager at +0x1a70.
                let world = DATA + 0x1_0000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x7c, DATA + 0x2_0000);
                c.bus.write32(DATA + 0x2_0000 + 0x1a70, seed % 2);
                let file = DATA + 0x3_0000;
                c.bus
                    .write32(file, if seed % 3 == 0 { u32::MAX } else { seed % 4 });
                c.g[A0][0] = sx(file);
                c.pc = 0x0015_aec0;
            },
        );
    }

    #[test]
    fn be_assign_global_string_1bc8_matches() {
        check(
            ee::s01::f_0015bc10__generated,
            be_assign_global_string_1bc8,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x15bc10, 0);
                c.pc = 0x0015_bc10;
            },
        );
    }

    #[test]
    fn be_copy_global_string_1bc8_matches() {
        check(
            ee::s01::f_0015bc30__generated,
            be_copy_global_string_1bc8,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x15bc30, 0);
                c.g[A0][0] = sx(DATA + 0x100 * (seed % 8));
                c.pc = 0x0015_bc30;
            },
        );
    }

    #[test]
    fn be_set_name_entry_535617_matches() {
        check(
            ee::s01::f_0015c188__generated,
            be_set_name_entry_535617,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x15c188, 0);
                c.g[A1][0] = (seed % 0x40) as u64;
                c.g[A2][0] = sx(DATA + seed * 8);
                c.pc = 0x0015_c188;
            },
        );
    }

    #[test]
    fn be_set_string_534d48_matches() {
        check(ee::s01::f_0015c338__generated, be_set_string_534d48, 20, &|c, seed| {
            let _ = machine(c, seed, 0x15c338, 0);
            c.pc = 0x0015_c338;
        });
    }

    #[test]
    fn be_set_string_534dc0_matches() {
        check(ee::s01::f_0015c368__generated, be_set_string_534dc0, 20, &|c, seed| {
            let _ = machine(c, seed, 0x15c368, 0);
            c.pc = 0x0015_c368;
        });
    }

    #[test]
    fn be_set_string_534e38_matches() {
        check(ee::s01::f_0015c398__generated, be_set_string_534e38, 20, &|c, seed| {
            let _ = machine(c, seed, 0x15c398, 0);
            c.pc = 0x0015_c398;
        });
    }
}
