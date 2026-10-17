//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Hand-written, typed replacements for the back-end (BE*) singleton and
//! reward/replay helpers of shard `s01`, each with a differential test
//! against the generated body.

use crate::abi;
use crate::call::{self, call_ptr, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    // BE singletons.
    (0x0015_bb20, be_new_race_int_destroy_instance, "BENewRaceInt_DestroyInstance"),
    (0x0015_bc88, be_new_player_int_destroy_instance, "BENewPlayerInt_DestroyInstance"),
    (0x0015_bd30, be_option_int_destroy_instance, "BEOptionInt_DestroyInstance"),
    (0x0015_c250, be_network_int_destroy_instance, "BENetworkInt_DestroyInstance"),
    (0x0015_c830, be_pre_race_destroy_instance, "BEPreRace_DestroyInstance"),
    (0x0015_c870, be_race_destroy, "BERace_Destroy"),
    (0x0015_c8b0, be_post_race_destroy_instance, "BEPostRace_DestroyInstance"),
    (0x0015_3088, be_race_get_instance, "BERace_GetInstance"),
    (0x0015_6338, be_post_race_get_instance, "BEPostRace_GetInstance"),
    (0x0015_6570, be_pre_race_get_instance, "BEPreRace_GetInstance"),
    (0x0015_69c0, be_reward_int_get_instance, "BERewardInt_GetInstance"),
    // Reward file accessors.
    (0x0015_6b70, be_reward_int_get_section0, "BERewardInt_GetSection0"),
    (0x0015_6b90, be_reward_int_get_section1, "BERewardInt_GetSection1"),
    (0x0015_6bb0, be_reward_int_get_section2, "BERewardInt_GetSection2"),
    (0x0015_6bd0, be_reward_int_get_poster_table, "BERewardInt_GetPosterTable"),
    (0x0015_6bf0, be_reward_int_get_trading_card_table, "BERewardInt_GetTradingCardTable"),
    (0x0015_6c10, be_reward_int_get_concept_art_table, "BERewardInt_GetConceptArtTable"),
    (0x0015_6c30, be_reward_int_get_toy_table, "BERewardInt_GetToyTable"),
    (0x0015_6c50, be_reward_int_get_video_table, "BERewardInt_GetVideoTable"),
    // New-player slots.
    (0x0014_7138, be_new_player_int_set_name_by_slot, "BENewPlayerInt_SetNameBySlot"),
    (0x0014_7220, be_new_player_int_set_pad_index, "BENewPlayerInt_SetPadIndex"),
    (0x0014_7338, be_new_player_int_set_character, "BENewPlayerInt_SetCharacter"),
    (0x0014_74e8, be_new_player_int_get_vibration, "BENewPlayerInt_GetVibration"),
    (0x0014_7528, be_new_player_int_set_vibration, "BENewPlayerInt_SetVibration"),
    (0x0014_7618, be_new_player_int_get_control_map, "BENewPlayerInt_GetControlMap"),
    (0x0014_7658, be_new_player_int_set_control_map, "BENewPlayerInt_SetControlMap"),
    // Options (quick messages, HUD defaults).
    (0x0014_f600, be_option_int_set_hud_defaults, "BEOptionInt_SetHUDDefaults"),
    (0x0014_f678, be_option_int_get_quick_message_key, "BEOptionInt_GetQuickMessageKey"),
    (0x0014_f6a8, be_option_int_get_quick_message, "BEOptionInt_GetQuickMessage"),
    // New-race / rewards / save / score.
    (0x0014_4aa8, be_new_race_int_set_num_human_players, "BENewRaceInt_SetNumHumanPlayers"),
    (0x0014_4b20, be_new_race_int_set_track_strings, "BENewRaceInt_SetTrackStrings"),
    (0x0015_6a90, be_reward_int_get_section0_entry, "BERewardInt_GetSection0Entry"),
    (0x0015_6ae0, be_reward_int_get_section1_entry, "BERewardInt_GetSection1Entry"),
    (0x0015_6b28, be_reward_int_get_section2_entry, "BERewardInt_GetSection2Entry"),
    (0x0015_72b0, be_reward_int_get_poster_count, "BERewardInt_GetPosterCount"),
    (0x0015_72d0, be_reward_int_get_trading_card_count, "BERewardInt_GetTradingCardCount"),
    (0x0015_72f0, be_reward_int_get_concept_art_count, "BERewardInt_GetConceptArtCount"),
    (0x0015_7310, be_reward_int_get_toy_count, "BERewardInt_GetToyCount"),
    (0x0015_7330, be_reward_int_get_video_count, "BERewardInt_GetVideoCount"),
    (0x0015_7370, be_reward_int_get_cheat_char_count, "BERewardInt_GetCheatCharCount"),
    (0x0015_7390, be_reward_int_count_set_bits, "BERewardInt_CountSetBits"),
    (0x0015_a510, be_reward_int_clear_new_rewards, "BERewardInt_ClearNewRewards"),
    (0x0015_2638, be_save_int_get_instance, "BESaveInt_GetInstance"),
    (0x0015_2688, be_save_int_get_save_buffer, "BESaveInt_GetSaveBuffer"),
    (0x0015_2700, be_save_int_free_save_buffer, "BESaveInt_FreeSaveBuffer"),
    (0x0015_3050, be_save_int_destroy, "BESaveInt_Destroy"),
    (0x0015_49a8, be_score_int_ctor, "BEScoreInt_ctor"),
    (0x0015_49e8, be_score_int_v02, "BEScoreInt_v02"),
    // Reward predicates, counts and dispatchers.
    (0x0015_75c0, be_reward_int_count_unlocked_posters, "BERewardInt_CountUnlockedPosters"),
    (0x0015_7620, be_reward_int_count_unlocked_trading_cards, "BERewardInt_CountUnlockedTradingCards"),
    (0x0015_7680, be_reward_int_count_unlocked_concept_arts, "BERewardInt_CountUnlockedConceptArts"),
    (0x0015_76e0, be_reward_int_count_unlocked_toys, "BERewardInt_CountUnlockedToys"),
    (0x0015_7740, be_reward_int_count_unlocked_videos, "BERewardInt_CountUnlockedVideos"),
    (0x0015_77a0, be_reward_int_count_unlocked_rider_items, "BERewardInt_CountUnlockedRiderItems"),
    (0x0015_7d60, be_reward_int_is_poster_unlocked, "BERewardInt_IsPosterUnlocked"),
    (0x0015_7da0, be_reward_int_is_trading_card_unlocked, "BERewardInt_IsTradingCardUnlocked"),
    (0x0015_7de0, be_reward_int_is_concept_art_unlocked, "BERewardInt_IsConceptArtUnlocked"),
    (0x0015_7e20, be_reward_int_is_toy_unlocked, "BERewardInt_IsToyUnlocked"),
    (0x0015_7e60, be_reward_int_is_video_unlocked, "BERewardInt_IsVideoUnlocked"),
    (0x0015_7ee0, be_reward_int_is_rider_item_unlocked, "BERewardInt_IsRiderItemUnlocked"),
    (0x0015_7210, be_reward_int_get_reward_type_count, "BERewardInt_GetRewardTypeCount"),
    (0x0015_7468, be_reward_int_is_reward_unlocked, "BERewardInt_IsRewardUnlocked"),
    (0x0015_7518, be_reward_int_count_unlocked, "BERewardInt_CountUnlocked"),
    (0x0015_8910, be_reward_int_get_event_best_place, "BERewardInt_GetEventBestPlace"),
    (0x0015_8f30, be_reward_int_get_new_reward_total, "BERewardInt_GetNewRewardTotal"),
    (0x0015_a2e0, be_reward_int_is_reward_granted, "BERewardInt_IsRewardGranted"),
    (0x0015_a6f0, be_reward_int_count_new_rewards, "BERewardInt_CountNewRewards"),
    // New player object.
    (0x0014_5930, be_new_player_int_ctor, "BENewPlayerInt_ctor"),
    (0x0014_5b20, be_new_player_int_reset_slots, "BENewPlayerInt_ResetSlots"),
    (0x0014_5c38, be_new_player_int_get_first_time_wc_flag, "BENewPlayerInt_GetFirstTimeWCFlag"),
    (0x0014_5cb0, be_new_player_int_set_first_time_wc_flag, "BENewPlayerInt_SetFirstTimeWCFlag"),
    (0x0014_6f40, be_new_player_int_set_name, "BENewPlayerInt_SetName"),
    // Options (defaults, quick messages).
    (0x0014_f418, be_option_int_ctor, "BEOptionInt_ctor"),
    (0x0014_f458, be_option_int_set_sound_defaults, "BEOptionInt_SetSoundDefaults"),
    (0x0014_f4f8, be_option_int_set_game_defaults, "BEOptionInt_SetGameDefaults"),
    (0x0014_f6e8, be_option_int_reset_quick_message, "BEOptionInt_ResetQuickMessage"),
    (0x0014_f758, be_option_int_reset_quick_messages, "BEOptionInt_ResetQuickMessages"),
    // Score best stats.
    (0x0015_4a58, be_score_int_update_best_stats, "BEScoreInt_UpdateBestStats"),
    // New race game mode and new rewards.
    (0x0014_5108, be_new_race_int_set_game_mode, "BENewRaceInt_SetGameMode"),
    (0x0015_a628, be_reward_int_add_new_reward, "BERewardInt_AddNewReward"),
];

// ---------------------------------------------------------------------------
// Callees.

const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
const BUILTIN_DELETE: u32 = 0x0031_7e50;

/// `BERewardInt_GetRewardsFile` (0x15a478): the loaded rewards file.
const GET_REWARDS_FILE: Callee = Callee { addr: 0x0015_a478, f: ssx3_ee::s01::f_0015a478 };

// ---------------------------------------------------------------------------
// BE singletons.

/// The gp-relative global holding each singleton instance (address
/// `gp - K`), the object size, and the debug name literal EA's allocator
/// records.
const NETWORK_GP: u32 = 0x1eec;
const NEW_PLAYER_GP: u32 = 0x1f30;
const NEW_RACE_GP: u32 = 0x1f3c;
const OPTION_GP: u32 = 0x1ee4;
const PRE_RACE_GP: u32 = 0x1e98;
const RACE_GP: u32 = 0x1ec4;
const POST_RACE_GP: u32 = 0x1e9c;
const REWARD_GP: u32 = 0x1e8c;

/// `BENetworkInt_DestroyInstance()` (0x15c250): `__builtin_delete` the
/// singleton at `gp - 0x1eec` and clear the global.
pub fn be_network_int_destroy_instance(c: &mut Cpu) {
    destroy_instance(c, NETWORK_GP, 0x0015_c264);
}

/// `BENewPlayerInt_DestroyInstance()` (0x15bc88): as above for
/// `gp - 0x1f30`.
pub fn be_new_player_int_destroy_instance(c: &mut Cpu) {
    destroy_instance(c, NEW_PLAYER_GP, 0x0015_bc9c);
}

/// `BENewRaceInt_DestroyInstance()` (0x15bb20): as above for `gp - 0x1f3c`.
pub fn be_new_race_int_destroy_instance(c: &mut Cpu) {
    destroy_instance(c, NEW_RACE_GP, 0x0015_bb34);
}

/// `BEOptionInt_DestroyInstance()` (0x15bd30): as above for `gp - 0x1ee4`.
pub fn be_option_int_destroy_instance(c: &mut Cpu) {
    destroy_instance(c, OPTION_GP, 0x0015_bd44);
}

/// `BEPreRace_DestroyInstance()` (0x15c830): as above for `gp - 0x1e98`.
pub fn be_pre_race_destroy_instance(c: &mut Cpu) {
    destroy_instance(c, PRE_RACE_GP, 0x0015_c844);
}

/// `BERace_Destroy()` (0x15c870): as above for `gp - 0x1ec4`.
pub fn be_race_destroy(c: &mut Cpu) {
    destroy_instance(c, RACE_GP, 0x0015_c884);
}

/// `BEPostRace_DestroyInstance()` (0x15c8b0): as above for `gp - 0x1e9c`.
pub fn be_post_race_destroy_instance(c: &mut Cpu) {
    destroy_instance(c, POST_RACE_GP, 0x0015_c8c4);
}

/// The shared body of the `*_DestroyInstance` family: load the singleton,
/// `__builtin_delete` it, then clear the global. `ret` is the return address
/// the original passes to the delete call.
fn destroy_instance(c: &mut Cpu, gp_off: u32, ret: u32) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        let global = (c.g[GP][0] as u32).wrapping_sub(gp_off);
        c.g[A0][0] = Addr(global).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, BUILTIN_DELETE, ret)?;
        frame.restore_ra(c, 0);
        Addr(global).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BERace_GetInstance(int id)` (0x153088): the `BERace` singleton, allocated
/// (with `id` in its first word and a name literal at +4) through
/// `EA_MemAllocNamed(8, "BERace", 0, 0)` on first use.
pub fn be_race_get_instance(c: &mut Cpu) {
    get_instance(
        c,
        RACE_GP,
        0x0015_30b8,
        8,
        0x004a_1230,
        &[(0, Arg::A0), (4, Arg::Const(0x0045_abe8))],
    );
}

/// `BEPostRace_GetInstance(int id)` (0x156338): as above with `gp - 0x1e9c`
/// and the "BEPostRace" name.
pub fn be_post_race_get_instance(c: &mut Cpu) {
    get_instance(
        c,
        POST_RACE_GP,
        0x0015_6368,
        8,
        0x0045_a8a8,
        &[(0, Arg::A0), (4, Arg::Const(0x0045_abb8))],
    );
}

/// `BEPreRace_GetInstance(int id)` (0x156570): as above with `gp - 0x1e98`
/// and the "BEPreRace" name.
pub fn be_pre_race_get_instance(c: &mut Cpu) {
    get_instance(
        c,
        PRE_RACE_GP,
        0x0015_65a0,
        8,
        0x0045_a8a8,
        &[(0, Arg::A0), (4, Arg::Const(0x0045_ac18))],
    );
}

/// What to store in an allocated singleton's word.
#[derive(Clone, Copy)]
enum Arg {
    /// The function's first argument (`a0`).
    A0,
    /// A constant word.
    Const(u32),
}

/// `BERewardInt_GetInstance()` (0x1569c0): the `BERewardInt` singleton,
/// allocated with `EA_MemAllocNamed(0x18, "BERewardInt", 0, 0)`: +8 clear,
/// +0xc a name literal.
pub fn be_reward_int_get_instance(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let global = (c.g[GP][0] as u32).wrapping_sub(REWARD_GP);
        let inst = Addr(global).lw(c);
        c.g[V0][0] = inst;
        frame.save_ra(c, 0);
        if inst == 0 {
            c.cycles += 11;
            let block = alloc_named(c, 0x18, 0x0045_a938, 0x0015_69e8)?;
            c.g[V1][0] = 0x0046_0000;
            Addr(block as u32 + 8).sw(c, 0);
            c.g[V1][0] = 0x0045_ae88;
            Addr(block as u32 + 0xc).sw(c, c.g[V1][0]);
            Addr(global).sw(c, block);
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(global).lw(c);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared `*_GetInstance` body: return the cached singleton, allocating
/// it (and storing the given words) when the global is still zero.
fn get_instance(c: &mut Cpu, gp_off: u32, ret: u32, size: u64, name: u32, init: &[(u32, Arg)]) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        let global = (c.g[GP][0] as u32).wrapping_sub(gp_off);
        let inst = Addr(global).lw(c);
        c.g[V0][0] = inst;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let present = inst != 0;
        c.g[S0][0] = c.g[A0][0];
        if !present {
            c.cycles += 11;
            let block = alloc_named(c, size, name, ret)?;
            for &(off, value) in init {
                let v = match value {
                    Arg::A0 => c.g[S0][0],
                    Arg::Const(x) => x as u64,
                };
                c.g[V1][0] = v;
                Addr(block as u32 + off).sw(c, v);
            }
            Addr(global).sw(c, block);
        }
        c.cycles += 5;
        c.g[V0][0] = Addr(global).lw(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `EA_MemAllocNamed(size, name, 0, 0)` through the function table; returns
/// the allocated block.
fn alloc_named(c: &mut Cpu, size: u64, name: u32, ret: u32) -> Result<u64, call::Unwound> {
    c.g[A0][0] = size;
    c.g[A1][0] = name as u64;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, EA_MEM_ALLOC_NAMED, ret)?;
    Ok(c.g[V0][0])
}

// ---------------------------------------------------------------------------
// Rewards file.

/// The shared body of the section/table accessors: call `GetRewardsFile` and
/// return the word at `off` from it.
fn rewards_field(c: &mut Cpu, off: u32, ret: u32) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        GET_REWARDS_FILE.call(c, ret)?;
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(off)).lw(c);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BERewardInt_GetSection0()` (0x156b70): rewards file +8.
pub fn be_reward_int_get_section0(c: &mut Cpu) {
    rewards_field(c, 0x8, 0x0015_6b80);
}

/// `BERewardInt_GetSection1()` (0x156b90): rewards file +0xc.
pub fn be_reward_int_get_section1(c: &mut Cpu) {
    rewards_field(c, 0xc, 0x0015_6ba0);
}

/// `BERewardInt_GetSection2()` (0x156bb0): rewards file +0x10.
pub fn be_reward_int_get_section2(c: &mut Cpu) {
    rewards_field(c, 0x10, 0x0015_6bc0);
}

/// `BERewardInt_GetPosterTable()` (0x156bd0): rewards file +0x14.
pub fn be_reward_int_get_poster_table(c: &mut Cpu) {
    rewards_field(c, 0x14, 0x0015_6be0);
}

/// `BERewardInt_GetTradingCardTable()` (0x156bf0): rewards file +0x18.
pub fn be_reward_int_get_trading_card_table(c: &mut Cpu) {
    rewards_field(c, 0x18, 0x0015_6c00);
}

/// `BERewardInt_GetConceptArtTable()` (0x156c10): rewards file +0x1c.
pub fn be_reward_int_get_concept_art_table(c: &mut Cpu) {
    rewards_field(c, 0x1c, 0x0015_6c20);
}

/// `BERewardInt_GetToyTable()` (0x156c30): rewards file +0x20.
pub fn be_reward_int_get_toy_table(c: &mut Cpu) {
    rewards_field(c, 0x20, 0x0015_6c40);
}

/// `BERewardInt_GetVideoTable()` (0x156c50): rewards file +0x24.
pub fn be_reward_int_get_video_table(c: &mut Cpu) {
    rewards_field(c, 0x24, 0x0015_6c60);
}

// ---------------------------------------------------------------------------
// New-player slots.

/// `BENewPlayerInt_GetSlot` (0x147798): the slot index for a rider id.
const GET_SLOT: Callee = Callee { addr: 0x0014_7798, f: ssx3_ee::s01::f_00147798 };
/// The player-slot array base (`0x530000 + 0x4fe0`), stride 0x1c.
const PLAYER_SLOTS: u32 = 0x0053_4fe0;
const PLAYER_SLOT_STRIDE: u32 = 0x1c;

/// `BENewPlayerInt_SetNameBySlot(self, rider, name)` (0x147138): copy 12 bytes
/// of the name into the rider's slot (`strncpy`).
pub fn be_new_player_int_set_name_by_slot(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        c.g[V0][0] = PLAYER_SLOT_STRIDE as u64;
        c.g[A0][0] = PLAYER_SLOTS as u64;
        c.mult(0, V1, A1, V0);
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0xc;
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(PLAYER_SLOTS));
        call_ptr(c, 0x0041_6ce0, 0x0014_7160)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_SetPadIndex(self, rider, pad)` (0x147220): store the pad
/// index at slot+0xc, and (when non-negative) tag the low flag bits with its
/// parity.
pub fn be_new_player_int_set_pad_index(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        GET_SLOT.call(c, 0x0014_7238)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = PLAYER_SLOTS as u64;
        c.g[V0][0] = PLAYER_SLOT_STRIDE as u64;
        c.mult(0, A2, A0, V0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        let negative = (c.g[S0][0] as i64) < 0;
        Addr(c.g[V0][0] as u32 + 0xc).sw(c, c.g[S0][0]);
        if !negative {
            c.cycles += 9;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A1][0] = 0xffff_ffff_ffff_fffe;
            c.g[V0][0] = Addr(c.g[A2][0] as u32 + 0x10).lw(c);
            c.g[V1][0] = c.g[A0][0] & 0x1;
            c.g[A0][0] = 0xffff_ffff_ffff_fffb;
            c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
            Addr(c.g[A2][0] as u32 + 0x10).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_SetCharacter(self, rider, character)` (0x147338): store the
/// character at slot+0x11, unless the "character locked" flag (bit 2 of the
/// slot flags) is set, in which case keep the slot's default byte.
pub fn be_new_player_int_set_character(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        GET_SLOT.call(c, 0x0014_7350)?;
        c.g[V1][0] = PLAYER_SLOT_STRIDE as u64;
        c.g[A0][0] = PLAYER_SLOTS as u64;
        c.mult(0, V0, V0, V1);
        c.g[V1][0] = 1;
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(PLAYER_SLOTS));
        c.g[V0][0] = Addr(c.g[A1][0] as u32 + 0x10).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        if c.g[V0][0] != c.g[V1][0] {
            Addr(c.g[A1][0] as u32 + 0x11).sb(c, c.g[S0][0]);
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(PLAYER_SLOTS + 0x11).lbu(c);
            Addr(c.g[A1][0] as u32 + 0x11).sb(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_GetVibration(self, rider)` (0x1474e8): bit 1 of the slot
/// flags at +0x10.
pub fn be_new_player_int_get_vibration(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        GET_SLOT.call(c, 0x0014_74f8)?;
        c.g[A0][0] = PLAYER_SLOT_STRIDE as u64;
        c.g[V1][0] = PLAYER_SLOTS as u64;
        c.mult(0, V0, V0, A0);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(PLAYER_SLOTS));
        c.g[V0][0] = Addr(c.g[V0][0] as u32 + 0x10).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_SetVibration(self, rider, on)` (0x147528): write bit 1 of
/// the slot flags from `on & 1`.
pub fn be_new_player_int_set_vibration(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        GET_SLOT.call(c, 0x0014_7540)?;
        c.g[V1][0] = PLAYER_SLOT_STRIDE as u64;
        c.g[A0][0] = PLAYER_SLOTS as u64;
        c.mult(0, V0, V0, V1);
        c.g[A1][0] = 0xffff_ffff_ffff_fffd;
        c.g[S0][0] = c.g[S0][0] & 0x1;
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 1);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(PLAYER_SLOTS));
        c.g[V1][0] = Addr(c.g[V0][0] as u32 + 0x10).lw(c);
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V1][0] = c.g[V1][0] | c.g[S0][0];
        frame.restore(c, S0, 0x10);
        Addr(c.g[V0][0] as u32 + 0x10).sw(c, c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_GetControlMap(self, rider)` (0x147618): the signed byte at
/// slot+0x13.
pub fn be_new_player_int_get_control_map(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[S0][0] = PLAYER_SLOTS as u64;
        frame.save_ra(c, 0);
        GET_SLOT.call(c, 0x0014_7634)?;
        c.g[V1][0] = PLAYER_SLOT_STRIDE as u64;
        frame.restore_ra(c, 0);
        c.mult(0, V0, V0, V1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32 + 0x13).lb(c);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_SetControlMap(self, rider, map)` (0x147658): store the
/// control map byte at slot+0x13.
pub fn be_new_player_int_set_control_map(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        GET_SLOT.call(c, 0x0014_7670)?;
        c.g[A0][0] = PLAYER_SLOT_STRIDE as u64;
        c.g[V1][0] = PLAYER_SLOTS as u64;
        c.mult(0, V0, V0, A0);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(PLAYER_SLOTS));
        Addr(c.g[V0][0] as u32 + 0x13).sb(c, c.g[S0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Options.

/// `BEOptionInt_SetHUDDefaults(self)` (0x14f600): set bits 0x3fff of the HUD
/// flags word at 0x53610, then notify the current profile (`self+0xc` vtable
/// slot +0xc).
pub fn be_option_int_set_hud_defaults(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 17;
        c.g[A0][0] = 0x0053_0000;
        c.g[V0][0] = 0x3fff_0000;
        c.g[V1][0] = Addr(0x0053_5610).lw(c);
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(OPTION_GP)).lw(c);
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        frame.save_ra(c, 0);
        Addr(0x0053_5610).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(c.g[A1][0] as u32 + 0xc).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32 + 0x8).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32 + 0xc).lw(c);
        c.g[RA][0] = 0x0014_f638;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, 0x0014_f638)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BEOptionInt_GetQuickMessageKey(self, index)` (0x14f678): format the index
/// into a small stack buffer and look the key up.
pub fn be_option_int_get_quick_message_key(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        c.g[A2][0] = c.g[A1][0];
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0x0046_0000;
        frame.save_ra(c, 0x20);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(0x0046_0000u32.wrapping_sub(0x5940));
        call_ptr(c, 0x0041_7828, 0x0014_f694)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = 0x0014_f69c;
        call_ptr(c, 0x0031_7670, 0x0014_f69c)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `BEOptionInt_GetQuickMessage(self, index)` (0x14f6a8): the quick-message
/// string for `index` at `0x535617 + index*0x40`, or 0 when empty.
pub fn be_option_int_get_quick_message(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0053_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(0x0053_0000u32.wrapping_add(0x5617));
        c.g[S0][0] = sx((c.g[A1][0] as u32) << 6);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_6810, 0x0014_f6cc)?;
        if c.g[V0][0] == 0 {
            c.g[S0][0] = 0;
        }
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// New race.

/// `BENewRaceInt_SetNumHumanPlayers(self, n)` (0x144aa8): set the human
/// count and the derived total at +0x34/+0x30, then clear six pairs of words
/// (the per-player slots at +0 and +0x18).
pub fn be_new_race_int_set_num_human_players(c: &mut Cpu) {
    const INFO: u32 = 0x0053_5bc8;
    c.cycles += 10;
    c.g[V0][0] = 0x0053_0000;
    c.g[A2][0] = 0;
    c.g[V0][0] = sx(0x0053_0000u32.wrapping_add(0x5bc8));
    c.g[V1][0] = Addr(INFO + 0x3c).lw(c);
    c.g[A3][0] = c.g[V0][0];
    c.g[A0][0] = Addr(INFO + 0x38).lw(c);
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    Addr(INFO + 0x34).sw(c, c.g[A1][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    Addr(INFO + 0x30).sw(c, c.g[V1][0]);
    loop {
        c.cycles += 7;
        Addr(c.g[A3][0] as u32).sw(c, c.g[A2][0]);
        Addr(c.g[A3][0] as u32 + 0x18).sw(c, c.g[A2][0]);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[A2][0] as i64) < 6) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0014_4ad0);
            }
        } else {
            c.cycles += 2;
            return abi::ret(c);
        }
    }
}

/// `BENewRaceInt_SetTrackStrings(self, name)` (0x144b20): append the name to
/// four track-name buffers inside the new-race info block.
pub fn be_new_race_int_set_track_strings(c: &mut Cpu) {
    const INFO: u32 = 0x0053_5bc8;
    call::guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = INFO as u64;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = sx(INFO);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = 0x0044_0000;
        frame.save_ra(c, 0);
        c.g[S3][0] = 0x64;
        c.g[S2][0] = c.g[A1][0];
        c.g[S0][0] = sx(0x0044_0000u32.wrapping_sub(0x267c));
        // 1: dst = *(info+0x40) * 0x64 + s0.
        c.g[A0][0] = Addr(INFO + 0x40).lw(c);
        c.mult(0, A0, A0, S3);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        call_ptr(c, 0x0041_66f4, 0x0014_4b60)?;
        // 2: dst = *(info+0x40) * 0x64 + s0 + 0x10.
        c.g[A0][0] = Addr(INFO + 0x40).lw(c);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[S2][0];
        c.mult(0, A0, A0, S3);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, 0x0041_66f4, 0x0014_4b78)?;
        // 3: dst = *(info+0x40) * 0x64 + s0 - 0x30.
        c.g[A0][0] = Addr(INFO + 0x40).lw(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x30));
        c.g[A1][0] = c.g[S2][0];
        c.mult(0, A0, A0, S3);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        call_ptr(c, 0x0041_66f4, 0x0014_4b90)?;
        // 4: a fixed buffer.
        c.g[A0][0] = 0x0044_0000;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx(0x0044_0000u32.wrapping_sub(0x1914));
        call_ptr(c, 0x0041_66f4, 0x0014_4ba0)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Reward entries and counts.

/// `BERewardInt_GetSection0Entry(file, a, b)` (0x156a90): word at section0
/// (`file+8`) indexed by `(b*3 + a)*0xc`.
pub fn be_reward_int_get_section0_entry(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        GET_REWARDS_FILE.call(c, 0x0015_6aac)?;
        c.g[V1][0] = sx((c.g[S0][0] as u32) << 1);
        c.g[A0][0] = 0xc;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32 + 8).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
        frame.restore(c, S0, 0x20);
        c.mult(0, V1, V1, A0);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        frame.close(c);
        Ok(())
    })
}

/// `BERewardInt_GetSection1Entry(file, a, b)` (0x156ae0): word at section1
/// (`file+0xc`) indexed by `(a*4 + b)*0xc`.
pub fn be_reward_int_get_section1_entry(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        GET_REWARDS_FILE.call(c, 0x0015_6afc)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V1][0] = 0xc;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32 + 0xc).lw(c);
        c.mult(0, S0, S0, V1);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `BERewardInt_GetSection2Entry(file, a, b)` (0x156b28): word at section2
/// (`file+0x10`) indexed by `(b*3 + a)*8`.
pub fn be_reward_int_get_section2_entry(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        GET_REWARDS_FILE.call(c, 0x0015_6b44)?;
        c.g[V1][0] = sx((c.g[S0][0] as u32) << 1);
        c.g[V0][0] = Addr(c.g[V0][0] as u32 + 0x10).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
        frame.restore(c, S0, 0x20);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
        frame.restore(c, S1, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        frame.close(c);
        Ok(())
    })
}

/// `BERewardInt_GetPosterCount()` (0x1572b0): rewards file +0x3c.
pub fn be_reward_int_get_poster_count(c: &mut Cpu) {
    rewards_field(c, 0x3c, 0x0015_72c0);
}

/// `BERewardInt_GetTradingCardCount()` (0x1572d0): rewards file +0x40.
pub fn be_reward_int_get_trading_card_count(c: &mut Cpu) {
    rewards_field(c, 0x40, 0x0015_72e0);
}

/// `BERewardInt_GetConceptArtCount()` (0x1572f0): rewards file +0x44.
pub fn be_reward_int_get_concept_art_count(c: &mut Cpu) {
    rewards_field(c, 0x44, 0x0015_7300);
}

/// `BERewardInt_GetToyCount()` (0x157310): rewards file +0x48.
pub fn be_reward_int_get_toy_count(c: &mut Cpu) {
    rewards_field(c, 0x48, 0x0015_7320);
}

/// `BERewardInt_GetVideoCount()` (0x157330): rewards file +0x4c.
pub fn be_reward_int_get_video_count(c: &mut Cpu) {
    rewards_field(c, 0x4c, 0x0015_7340);
}

/// `BERewardInt_GetCheatCharCount()` (0x157370): rewards file +0x50.
pub fn be_reward_int_get_cheat_char_count(c: &mut Cpu) {
    rewards_field(c, 0x50, 0x0015_7380);
}

/// `BERewardInt_CountSetBits(data, bits)` (0x157390): count the set bits in
/// the first `bits` bits of `data` (bytes read one at a time).
pub fn be_reward_int_count_set_bits(c: &mut Cpu) {
    c.cycles += 3;
    c.g[T0][0] = 0;
    let t = (c.g[A1][0] as i64) <= 0;
    c.g[A3][0] = 0;
    if !t {
        c.cycles += 1;
        c.g[T1][0] = c.lb(c.g[A2][0] as u32);
        'outer: loop {
            c.cycles += 4;
            c.g[A0][0] = 0;
            c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let done = c.g[V0][0] == 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            if done {
                break 'outer;
            }
            c.cycles += 2;
            c.g[V1][0] = sx(((c.g[T1][0] as u32) as i32 >> (c.g[A0][0] as u32 & 31)) as u32);
            'inner: loop {
                c.cycles += 6;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                c.g[V1][0] = c.g[V1][0] & 1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < 8) as u64;
                let byte_done = c.g[V0][0] == 0;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                if byte_done {
                    break 'inner;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] =
                        sx(((c.g[T1][0] as u32) as i32 >> (c.g[A0][0] as u32 & 31)) as u32);
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_73b8);
                    }
                    continue 'inner;
                } else {
                    break 'inner;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[T1][0] = c.lb(c.g[A2][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0015_73a0);
                }
                continue 'outer;
            } else {
                break 'outer;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A3][0];
    abi::ret(c);
}

/// `BERewardInt_ClearNewRewards(self)` (0x15a510): clear 0x174 bytes and set
/// the "new" count at +4 to -1.
pub fn be_reward_int_clear_new_rewards(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x174;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x003e_6448, 0x0015_a52c)?;
        c.g[V0][0] = u64::MAX;
        frame.restore_ra(c, 0);
        Addr(c.g[S0][0] as u32 + 4).sh(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Save singletons.

const SAVE_GP: u32 = 0x1ed4;
const SAVE_BUFFER_GP: u32 = 0x1ecc;
/// The separately freed save block (`gp - 0x1ed0`).
const SAVE_FREE_GP: u32 = 0x1ed0;

/// `BESaveInt_GetInstance()` (0x152638): the `BESaveInt` singleton, allocated
/// with `EA_MemAllocNamed(0x10, "BESaveInt", 0, 0)`.
pub fn be_save_int_get_instance(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let global = (c.g[GP][0] as u32).wrapping_sub(SAVE_GP);
        let inst = Addr(global).lw(c);
        c.g[V0][0] = inst;
        frame.save_ra(c, 0);
        if inst == 0 {
            c.cycles += 12;
            let block = alloc_named(c, 0x10, 0x0045_a7e0, 0x0015_2660)?;
            c.g[V1][0] = 0x0046_0000;
            Addr(block as u32 + 8).sw(c, 0);
            c.g[V1][0] = 0x0045_ad08;
            Addr(block as u32 + 0xc).sw(c, c.g[V1][0]);
            Addr(global).sw(c, block);
            c.sw(Addr((c.g[GP][0] as u32).wrapping_sub(0x1ec8)).0, 0);
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(global).lw(c);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BESaveInt_GetSaveBuffer()` (0x152688): the 0x80000-byte save buffer,
/// allocated (flags 0x100) on first use; sets the "buffer active" flag.
pub fn be_save_int_get_save_buffer(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let buffer = (c.g[GP][0] as u32).wrapping_sub(SAVE_BUFFER_GP);
        c.g[V0][0] = 1;
        c.g[V1][0] = Addr(buffer).lw(c);
        frame.save_ra(c, 0);
        let present = c.g[V1][0] != 0;
        c.sw(Addr((c.g[GP][0] as u32).wrapping_sub(0x1ec8)).0, c.g[V0][0]);
        if !present {
            c.cycles += 7;
            c.g[A1][0] = 0x0046_0000;
            c.g[A0][0] = 0x8_0000;
            c.g[A1][0] = 0x0045_a7f0;
            c.g[A2][0] = 0x100;
            c.g[RA][0] = 0x0015_26b8;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x0015_26b8)?;
            c.sw(Addr(buffer).0, c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(buffer).lw(c);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BESaveInt_FreeSaveBuffer()` (0x152700): free the save buffer and clear
/// its global.
pub fn be_save_int_free_save_buffer(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let buffer = (c.g[GP][0] as u32).wrapping_sub(SAVE_FREE_GP);
        c.g[A0][0] = Addr(buffer).lw(c);
        let absent = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !absent {
            c.cycles += 2;
            c.g[RA][0] = 0x0015_2718;
            call_ptr(c, 0x0031_7e98, 0x0015_2718)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.sw(Addr(buffer).0, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BESaveInt_Destroy()` (0x153050): delete the singleton and free the save
/// buffer.
pub fn be_save_int_destroy(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        let global = (c.g[GP][0] as u32).wrapping_sub(SAVE_GP);
        c.g[A0][0] = Addr(global).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0015_3064)?;
        let buffer = (c.g[GP][0] as u32).wrapping_sub(SAVE_FREE_GP);
        c.g[A0][0] = Addr(buffer).lw(c);
        let absent = c.g[A0][0] == 0;
        c.sw(Addr(global).0, 0);
        if !absent {
            c.cycles += 2;
            c.g[RA][0] = 0x0015_3078;
            call_ptr(c, 0x0031_7e98, 0x0015_3078)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.sw(Addr(buffer).0, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Score singleton.

/// `BEScoreInt_ctor(self)` (0x1549a8): set the name at +0xc and reset.
pub fn be_score_int_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(0x5358));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sw(Addr(c.g[S0][0] as u32 + 0xc).0, c.g[V0][0]);
        c.pc = 0x0015_49e8;
        BE_SCORE_INT_V02.call(c, 0x0015_49c8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `BEScoreInt_v02(self)` (0x1549e8): zero 0x160 bytes from +0x20, set
/// +0x10/+0x14 to 0 and +0x18/+0x1c to -1, then set two stride-0xb0 records
/// to -1.
pub fn be_score_int_v02(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(Addr(c.g[S0][0] as u32 + 0x10).0, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
        c.sw(Addr(c.g[S0][0] as u32 + 0x14).0, 0);
        c.g[A2][0] = 0x160;
        c.sw(Addr(c.g[S0][0] as u32 + 0x1c).0, c.g[V0][0]);
        c.g[RA][0] = 0x0015_4a1c;
        c.sw(Addr(c.g[S0][0] as u32 + 0x18).0, c.g[V0][0]);
        call_ptr(c, 0x003e_6448, 0x0015_4a1c)?;
        c.g[V1][0] = u64::MAX;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x17c));
        c.g[V0][0] = 1;
        loop {
            c.cycles += 7;
            c.sw(Addr(c.g[S0][0] as u32).0, c.g[V1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let t = (c.g[V0][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xb0));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0015_4a28);
                }
                continue;
            }
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

const BE_SCORE_INT_V02: Callee = Callee { addr: 0x0015_49e8, f: ssx3_ee::s01::f_001549e8 };

// ---------------------------------------------------------------------------
// Reward index arithmetic.
//
// The reward arrays are addressed as `0x4a6ca8 + a*0x9b50 + b*0xf88`; a second
// index adds `+ k*0x9b50`. `TestReward`/`SetReward` then index a bit array at
// that base + a per-type offset.

const REWARD_BASE: u32 = 0x004a_6ca8;
const REWARD_STRIDE_A: u32 = 0x9b50;
const REWARD_STRIDE_B: u32 = 0xf88;

/// `int BERewardInt_IsXUnlocked(...)` (0x157d60 family): test the per-type
/// unlock bit at `REWARD_BASE + a1*0x9b50 + a2*0xf88 + v1 + extra`.
fn is_unlocked(c: &mut Cpu, extra: u32, rider_item: bool, ret: u32) {
    call::guest(c, |c| {
        c.cycles += 16;
        c.g[V0][0] = REWARD_STRIDE_B as u64;
        c.g[T0][0] = REWARD_STRIDE_A as u64;
        c.mult(0, A2, A2, V0);
        c.mult(0, V1, A1, T0);
        c.g[V0][0] = 0x004a_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(REWARD_BASE);
        frame.save_ra(c, 0);
        c.g[A1][0] = if rider_item {
            sx((c.g[A3][0] as u32).wrapping_sub(0xa))
        } else {
            c.g[A3][0]
        };
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(extra));
        call_ptr(c, 0x0015_7430, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BERewardInt_IsPosterUnlocked(...)` (0x157d60).
pub fn be_reward_int_is_poster_unlocked(c: &mut Cpu) {
    is_unlocked(c, 0xf30, false, 0x0015_7d94);
}

/// `BERewardInt_IsTradingCardUnlocked(...)` (0x157da0).
pub fn be_reward_int_is_trading_card_unlocked(c: &mut Cpu) {
    is_unlocked(c, 0xf36, false, 0x0015_7dd4);
}

/// `BERewardInt_IsConceptArtUnlocked(...)` (0x157de0).
pub fn be_reward_int_is_concept_art_unlocked(c: &mut Cpu) {
    is_unlocked(c, 0xf45, false, 0x0015_7e14);
}

/// `BERewardInt_IsToyUnlocked(...)` (0x157e20).
pub fn be_reward_int_is_toy_unlocked(c: &mut Cpu) {
    is_unlocked(c, 0xf53, false, 0x0015_7e54);
}

/// `BERewardInt_IsVideoUnlocked(...)` (0x157e60).
pub fn be_reward_int_is_video_unlocked(c: &mut Cpu) {
    is_unlocked(c, 0xf52, false, 0x0015_7e94);
}

/// `BERewardInt_IsRiderItemUnlocked(...)` (0x157ee0): the rider-item bit uses
/// `a3 - 0xa`.
pub fn be_reward_int_is_rider_item_unlocked(c: &mut Cpu) {
    is_unlocked(c, 0xf57, true, 0x0015_7f14);
}

/// `BERewardInt_IsRewardGranted(...)` (0x15a2e0): the "granted" bit at
/// `+0xf28`.
pub fn be_reward_int_is_reward_granted(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 16;
        c.g[V0][0] = REWARD_STRIDE_B as u64;
        c.g[T0][0] = REWARD_STRIDE_A as u64;
        c.mult(0, A2, A2, V0);
        c.mult(0, V1, A1, T0);
        c.g[V0][0] = 0x004a_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(REWARD_BASE);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A3][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf28));
        call_ptr(c, 0x0015_a320, 0x0015_a314)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BERewardInt_CountUnlockedX(self, a, b)` (0x1575c0 family): count the
/// set bits of the per-type unlock array.
fn count_unlocked(c: &mut Cpu, field: u32, extra: u32, ret0: u32, ret1: u32) {
    call::guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = REWARD_STRIDE_B as u64;
        c.mult(0, A2, A2, V0);
        frame.save(c, S0, 0x20);
        c.g[V1][0] = REWARD_STRIDE_A as u64;
        c.g[V0][0] = 0x004a_0000;
        c.mult(0, S0, A1, V1);
        c.g[V0][0] = sx(REWARD_BASE);
        frame.save(c, S1, 0x10);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        call_ptr(c, 0x0015_a478, ret0)?;
        c.g[A1][0] = Addr(c.g[V0][0] as u32 + field).lw(c);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(extra));
        call_ptr(c, 0x0015_7390, ret1)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BERewardInt_CountUnlockedPosters` (0x1575c0).
pub fn be_reward_int_count_unlocked_posters(c: &mut Cpu) {
    count_unlocked(c, 0x3c, 0xf30, 0x0015_75f8, 0x0015_7608);
}

/// `BERewardInt_CountUnlockedTradingCards` (0x157620).
pub fn be_reward_int_count_unlocked_trading_cards(c: &mut Cpu) {
    count_unlocked(c, 0x40, 0xf36, 0x0015_7658, 0x0015_7668);
}

/// `BERewardInt_CountUnlockedConceptArts` (0x157680).
pub fn be_reward_int_count_unlocked_concept_arts(c: &mut Cpu) {
    count_unlocked(c, 0x44, 0xf45, 0x0015_76b8, 0x0015_76c8);
}

/// `BERewardInt_CountUnlockedToys` (0x1576e0).
pub fn be_reward_int_count_unlocked_toys(c: &mut Cpu) {
    count_unlocked(c, 0x48, 0xf53, 0x0015_7718, 0x0015_7728);
}

/// `BERewardInt_CountUnlockedVideos` (0x157740).
pub fn be_reward_int_count_unlocked_videos(c: &mut Cpu) {
    count_unlocked(c, 0x4c, 0xf52, 0x0015_7778, 0x0015_7788);
}

/// `BERewardInt_CountUnlockedRiderItems(self, a, b)` (0x1577a0): no file
/// lookup; counts 20 bits at `REWARD_BASE + a2*0xf88 + v1 + 0xf57`.
pub fn be_reward_int_count_unlocked_rider_items(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 16;
        c.g[V0][0] = REWARD_STRIDE_B as u64;
        c.g[A3][0] = REWARD_STRIDE_A as u64;
        c.mult(0, A2, A2, V0);
        c.mult(0, V1, A1, A3);
        c.g[V0][0] = 0x004a_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(REWARD_BASE);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x14;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf57));
        call_ptr(c, 0x0015_7390, 0x0015_77d4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BERewardInt_GetRewardTypeCount(self, type)` (0x157210): dispatch on
/// `type - 3` through the 7-entry jump table to the matching count getter.
pub fn be_reward_int_get_reward_type_count(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(3));
        frame.save_ra(c, 0);
        c.g[V0][0] = (c.g[V1][0] < 7) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[A2][0] = c.g[A0][0];
        if !in_range {
            c.cycles += 1;
            c.g[V0][0] = 0;
            c.cycles += 1;
        } else {
            c.cycles += 7;
            c.g[V0][0] = 0x0046_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(0x5690));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            let call = match target {
                0x0015_7244 => Some((0x0015_72b0, 0x0015_724c)),
                0x0015_7254 => Some((0x0015_72d0, 0x0015_725c)),
                0x0015_7264 => Some((0x0015_72f0, 0x0015_726c)),
                0x0015_7274 => Some((0x0015_7330, 0x0015_727c)),
                0x0015_7284 => Some((0x0015_7310, 0x0015_728c)),
                0x0015_7294 | 0x0015_729c => None,
                t => {
                    c.jump_out(t, 0x0015_7210, 0x0015_72ac);
                    return Ok(());
                }
            };
            if let Some((addr, ret)) = call {
                c.cycles += 4;
                c.g[A0][0] = c.g[A2][0];
                call_ptr(c, addr, ret)?;
            } else if target == 0x0015_7294 {
                c.cycles += 2;
                c.g[V0][0] = 0x14;
                c.cycles += 1;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
                c.cycles += 1;
            }
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `BERewardInt_IsRewardUnlocked(self, a1, a2, type)` (0x157468): dispatch on
/// `type - 3` to the matching `IsXUnlocked`.
pub fn be_reward_int_is_reward_unlocked(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(3));
        frame.save_ra(c, 0);
        c.g[T1][0] = c.g[A0][0];
        c.g[V0][0] = (c.g[V1][0] < 6) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[A3][0] = c.g[T0][0];
        if !in_range {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 7;
            c.g[V0][0] = 0x0046_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(0x5670));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            let call = match target {
                0x0015_74a0 => Some((0x0015_7d60, 0x0015_74a8)),
                0x0015_74b0 => Some((0x0015_7da0, 0x0015_74b8)),
                0x0015_74c0 => Some((0x0015_7de0, 0x0015_74c8)),
                0x0015_74d0 => Some((0x0015_7e60, 0x0015_74d8)),
                0x0015_74e0 => Some((0x0015_7e20, 0x0015_74e8)),
                0x0015_74f0 => Some((0x0015_7ee0, 0x0015_74fc)),
                t => {
                    c.jump_out(t, 0x0015_7468, 0x0015_7514);
                    return Ok(());
                }
            };
            let (addr, ret) = call.unwrap();
            if target == 0x0015_74f0 {
                c.cycles += 5;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0xa));
            } else {
                c.cycles += 4;
            }
            c.g[A0][0] = c.g[T1][0];
            call_ptr(c, addr, ret)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `BERewardInt_CountUnlocked(self, a1, a2, type)` (0x157518): dispatch on
/// `type - 3` to the matching `CountUnlockedX`.
pub fn be_reward_int_count_unlocked(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(3));
        frame.save_ra(c, 0);
        c.g[V0][0] = (c.g[V1][0] < 6) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[T0][0] = c.g[A0][0];
        if !in_range {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 7;
            c.g[V0][0] = 0x0046_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(0x5650));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            let (addr, ret) = match target {
                0x0015_754c => (0x0015_75c0, 0x0015_7554),
                0x0015_755c => (0x0015_7620, 0x0015_7564),
                0x0015_756c => (0x0015_7680, 0x0015_7574),
                0x0015_757c => (0x0015_7740, 0x0015_7584),
                0x0015_758c => (0x0015_76e0, 0x0015_7594),
                0x0015_759c => (0x0015_77a0, 0x0015_75a4),
                t => {
                    c.jump_out(t, 0x0015_7518, 0x0015_75bc);
                    return Ok(());
                }
            };
            c.cycles += 4;
            c.g[A0][0] = c.g[T0][0];
            call_ptr(c, addr, ret)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `BERewardInt_GetEventBestPlace(...)` (0x158910): read the best-place byte
/// for an event, or -1 when the event has no record.
pub fn be_reward_int_get_event_best_place(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 15;
        c.g[V0][0] = REWARD_STRIDE_B as u64;
        c.g[V1][0] = REWARD_STRIDE_A as u64;
        c.mult(0, A0, A2, V0);
        c.mult(0, T1, A1, V1);
        c.g[V0][0] = 0x004a_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(REWARD_BASE);
        c.g[A1][0] = c.g[T0][0];
        c.g[A2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, 0x0015_2398, 0x0015_8944)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.lb(c.g[V0][0] as u32 + 1);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BERewardInt_GetNewRewardTotal()` (0x158f30): count the bits of the
/// new-reward array at 0x4c3ef0.
pub fn be_reward_int_get_new_reward_total(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x004c_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x004c_0000u32.wrapping_add(0x3ef0));
        call_ptr(c, 0x0015_a6f0, 0x0015_8f44)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BERewardInt_CountNewRewards(bits)` (0x15a6f0): sum the 0xa-byte counts for
/// 31 reward types.
pub fn be_reward_int_count_new_rewards(c: &mut Cpu) {
    c.cycles += 6;
    c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(6));
    c.g[A2][0] = 0;
    c.g[A0][0] = 0;
    c.g[A3][0] = 0xa;
    c.mult(0, V0, A0, A3);
    'outer: loop {
        c.cycles += 4;
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        c.g[A0][0] = 9;
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        'inner: loop {
            c.cycles += 7;
            c.g[V0][0] = c.lb(c.g[V1][0] as u32);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            let more = (c.g[A0][0] as i64) >= 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0015_a718);
                }
                continue 'inner;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[A1][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < 32) as u64;
            let more_types = c.g[V0][0] != 0;
            c.mult(0, V0, A0, A3);
            if more_types {
                if event_due(c) {
                    deliver_event_here(c, 0x0015_a708);
                }
                continue 'outer;
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[A2][0];
            return abi::ret(c);
        }
    }
}

// ---------------------------------------------------------------------------
// New-player object.

/// `BENewPlayerInt_ctor(self)` (0x145930): clear the state at +8, store the
/// name literal at +0xc, `ResetSlots`, then the shared slot initialiser, and
/// return `self`.
pub fn be_new_player_int_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(0x51d8));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sw(c.g[S0][0] as u32 + 8, 0);
        c.sw(c.g[S0][0] as u32 + 0xc, c.g[V0][0]);
        call_ptr(c, 0x0014_5b20, 0x0014_5954)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_5bd8, 0x0014_595c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_ResetSlots(self)` (0x145b20): copy the 0xa0-byte template
/// at 0x535b20 over the 20 player slots at 0x534fe0 (plus the 8-byte tail),
/// with the original's alignment check.
pub fn be_new_player_int_reset_slots(c: &mut Cpu) {
    const SRC: u32 = 0x0053_5b20;
    const DST: u32 = 0x0053_4fe0;
    const END: u32 = SRC + 0xa0;
    c.cycles += 8;
    c.g[V0][0] = 0x0053_0000;
    c.g[V1][0] = 0x0053_0000;
    c.g[A0][0] = DST as u64;
    c.g[V1][0] = SRC as u64;
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] = c.g[V0][0] & 7;
    let use_ldl = c.g[V0][0] != 0;
    c.g[V0][0] = sx(END);
    if use_ldl {
        loop {
            c.cycles += 19;
            for (r, k) in [(5usize, 7u32), (6, 0xf), (7, 0x17), (8, 0x1f)] {
                c.ldl(r, (c.g[V1][0] as u32).wrapping_add(k));
                c.ldr(r, (c.g[V1][0] as u32).wrapping_add(k - 7));
            }
            for (r, k) in [(5usize, 7u32), (6, 0xf), (7, 0x17), (8, 0x1f)] {
                c.sdl(r, (c.g[A0][0] as u32).wrapping_add(k));
                c.sdr(r, (c.g[A0][0] as u32).wrapping_add(k - 7));
            }
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0014_5b40);
                }
            } else {
                c.cycles += 2;
                break;
            }
        }
    } else {
        loop {
            c.cycles += 11;
            c.g[A1][0] = c.ld(c.g[V1][0] as u32);
            c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(8));
            c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
            c.sd(c.g[A0][0] as u32, c.g[A1][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(8), c.g[A2][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_5b94);
            }
        }
    }
    c.cycles += 5;
    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(7));
    c.ldr(2, c.g[V1][0] as u32);
    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(7));
    c.sdr(2, c.g[A0][0] as u32);
    abi::ret(c);
}

/// `BENewPlayerInt_GetFirstTimeWCFlag(self, rider)` (0x145c38): the first-time
/// world-circuit flag for `rider` from the reward array.
pub fn be_new_player_int_get_first_time_wc_flag(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a0e0, 0x0014_5c54)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, 0x0014_7798, 0x0014_5c60)?;
        c.g[V1][0] = 0x1c;
        c.g[A1][0] = 0x0053_0000;
        c.mult(0, V0, V0, V1);
        c.g[A1][0] = PLAYER_SLOTS as u64;
        c.g[V1][0] = REWARD_STRIDE_A as u64;
        c.g[A2][0] = REWARD_STRIDE_B as u64;
        c.mult(0, S1, S1, V1);
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx(REWARD_BASE);
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.restore_ra(c, 0);
        c.g[V1][0] = Addr(c.g[V0][0] as u32 + 0x11).lb(c);
        c.mult(0, V1, V1, A2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        frame.restore(c, S1, 0x10);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_SetFirstTimeWCFlag(self, rider, value)` (0x145cb0): store
/// the first-time world-circuit flag.
pub fn be_new_player_int_set_first_time_wc_flag(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a0e0, 0x0014_5cd4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, 0x0014_7798, 0x0014_5ce0)?;
        c.g[V1][0] = 0x1c;
        c.g[A1][0] = 0x0053_0000;
        c.mult(0, V0, V0, V1);
        c.g[A1][0] = PLAYER_SLOTS as u64;
        c.g[V1][0] = REWARD_STRIDE_A as u64;
        c.g[A2][0] = REWARD_STRIDE_B as u64;
        c.mult(0, S1, S1, V1);
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx(REWARD_BASE);
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.restore_ra(c, 0);
        c.g[V1][0] = Addr(c.g[V0][0] as u32 + 0x11).lb(c);
        c.mult(0, V1, V1, A2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        frame.restore(c, S1, 0x20);
        Addr(c.g[V1][0] as u32).sw(c, c.g[S2][0]);
        frame.restore(c, S2, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_SetName(self, rider, name)` (0x146f40): copy 12 bytes of
/// `name` into the rider's slot.
pub fn be_new_player_int_set_name(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        call_ptr(c, 0x0014_7798, 0x0014_6f58)?;
        c.g[V1][0] = PLAYER_SLOT_STRIDE as u64;
        c.g[A0][0] = PLAYER_SLOTS as u64;
        c.mult(0, V0, V0, V1);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0xc;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(PLAYER_SLOTS));
        call_ptr(c, 0x0041_6ce0, 0x0014_6f78)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Option defaults and quick messages.

/// `BEOptionInt_ctor(self)` (0x14f418): run the base initialiser, then set the
/// HUD, game and sound defaults.
pub fn be_option_int_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_f648, 0x0014_f42c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_f600, 0x0014_f434)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_f4f8, 0x0014_f43c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_f458, 0x0014_f444)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BEOptionInt_SetSoundDefaults(self)` (0x14f458): clear and set the sound
/// bits of the option flags word at 0x53610, then notify the profile.
pub fn be_option_int_set_sound_defaults(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 40;
        c.g[A2][0] = 0x0053_0000;
        c.g[V1][0] = 0xffff_ffff_ffff_fff0;
        c.g[V0][0] = Addr(0x0053_5610).lw(c);
        c.g[A0][0] = 0xffff_ffff_ffff_ff0f;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = 0xffff_ffff_ffff_f0ff;
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.g[V0][0] | 0xa;
        c.g[T1][0] = 0x0053_5610;
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[T2][0] = 0xffff_ffff_ffff_cfff;
        c.g[V0][0] = c.g[V0][0] | 0xa0;
        c.g[A1][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
        c.g[A0][0] = Addr(0x0053_5614).lw(c);
        c.g[V0][0] = c.g[V0][0] | 0xa00;
        c.g[A1][0] = c.g[A1][0] | 0x3fff;
        c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
        c.g[A3][0] = 0x1_0000;
        c.g[V1][0] = 0xffff_ffff_ffff_fffc;
        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
        c.g[T0][0] = 0x2_0000;
        c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
        c.g[V1][0] = 0x4_0000;
        c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
        c.g[A0][0] = c.g[A0][0] | 1;
        Addr(0x0053_5614).sw(c, c.g[A0][0]);
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        Addr(0x0053_5610).sw(c, c.g[V0][0]);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(OPTION_GP)).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32 + 0xc).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32 + 8).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32 + 0xc).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0014_f4ec)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BEOptionInt_SetGameDefaults(self)` (0x14f4f8): set the game-mode bits of
/// the option flags word from the current mode, then notify the profile.
pub fn be_option_int_set_game_defaults(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[A0][0] = 1;
        c.g[V1][0] = Addr(c.g[V0][0] as u32 + 0x9c).lw(c);
        c.g[V1][0] = Addr(c.g[V1][0] as u32 + 0x24).lw(c);
        let eq1 = c.g[V1][0] == c.g[A0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        if eq1 {
            c.cycles += 6;
            c.g[A0][0] = 0x0053_0000;
            c.g[V1][0] = 0xffff_ffff_ffcf_0000;
            c.g[V0][0] = Addr(0x0053_5610).lw(c);
            c.g[V1][0] = c.g[V1][0] | 0xffff;
            c.g[A1][0] = 0x20_0000;
            apply_game_mask(c);
        } else {
            c.cycles += 2;
            let lt2 = c.g[V0][0] != 0;
            c.g[V0][0] = 0x0053_0000;
            if lt2 {
                c.cycles += 6;
                clear_game_mask(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                let eq2 = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x0053_0000;
                if eq2 {
                    c.cycles += 5;
                    c.g[A0][0] = 0x0053_0000;
                    c.g[V1][0] = 0xffff_ffff_ffcf_0000;
                    c.g[V0][0] = Addr(0x0053_5610).lw(c);
                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                    c.g[A1][0] = 0x10_0000;
                    apply_game_mask(c);
                } else {
                    c.cycles += 2;
                    c.cycles += 6;
                    clear_game_mask(c);
                }
            }
        }
        c.cycles += 28;
        c.g[V1][0] = Addr(c.g[A2][0] as u32 + 0x5610).lw(c);
        c.g[V0][0] = 0xffff_ffff_fe3f_0000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[A0][0] = 0xffff_ffff_fff7_0000;
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        c.g[A1][0] = 0xffff_ffff_dfff_0000;
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        c.g[A1][0] = c.g[A1][0] | 0xffff;
        c.g[V0][0] = 0xffff_ffff_e1ff_0000;
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.g[A0][0] = 0x1200_0000;
        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5610));
        Addr(0x0053_5610).sw(c, c.g[V1][0]);
        c.sb(c.g[V0][0] as u32 + 6, 0);
        c.sb(c.g[V0][0] as u32 + 5, 0);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(OPTION_GP)).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32 + 0xc).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32 + 8).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32 + 0xc).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0014_f5f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared "apply the mode bits" tail of `SetGameDefaults`.
fn apply_game_mask(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
    Addr(0x0053_5610).sw(c, c.g[V0][0]);
}

/// The "clear the mode bits" tail of `SetGameDefaults` (v0 already holds the
/// flags word for the 0/>2 path, or the base for the inner path).
fn clear_game_mask(c: &mut Cpu) {
    c.g[V1][0] = 0xffff_ffff_ffcf_0000;
    c.g[A0][0] = Addr(0x0053_5610).lw(c);
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
    Addr(0x0053_5610).sw(c, c.g[A0][0]);
}

/// `BEOptionInt_ResetQuickMessage(self, index)` (0x14f6e8): format the index
/// into the controller's key buffer and copy it into the quick-message slot.
pub fn be_option_int_reset_quick_message(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = Addr(c.g[V0][0] as u32 + 0x8c).lw(c);
        c.g[S1][0] = Addr(c.g[S0][0] as u32 + 4).lw(c);
        c.g[V0][0] = Addr(c.g[S1][0] as u32 + 0x20).lh(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, 0x0014_f678, 0x0014_f718)?;
        c.g[V1][0] = Addr(c.g[S1][0] as u32 + 0x24).lw(c);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0014_f728)?;
        c.g[A0][0] = 0x0053_0000;
        c.g[S2][0] = sx((c.g[S2][0] as u32) << 6);
        c.g[A0][0] = sx(0x0053_0000u32.wrapping_add(0x5617));
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, 0x003d_cb20, 0x0014_f740)?;
        frame.restore(c, S2, 0x10);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BEOptionInt_ResetQuickMessages(self)` (0x14f758): reset all 10 quick
/// messages.
pub fn be_option_int_reset_quick_messages(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0014_f6e8, 0x0014_f780)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 10) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if !more {
                c.cycles += 5;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_f778);
            }
        }
    })
}

/// `BEScoreInt_UpdateBestStats(self, rider)` (0x154a58): ask the rider's
/// profile whether stats should update, then `BEScoreInt_UpdateRecords`.
pub fn be_score_int_update_best_stats(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0014_a0e0, 0x0015_4a74)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x0053_0000;
        c.g[V1][0] = Addr(0x0053_05f9).lb(c);
        if c.g[V1][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < 2) as u64;
            let update = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if update {
                c.cycles += 4;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = u64::MAX;
                c.g[A3][0] = u64::MAX;
                call_ptr(c, 0x0015_5420, 0x0015_4aa4)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BENewRaceInt_SetGameMode(self, mode)` (0x145108): record the mode byte,
/// then set the human/AI rider counts for modes 0, 1 and 2.
pub fn be_new_race_int_set_game_mode(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0053_0000;
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = sx(0x0053_0000u32.wrapping_add(0x5bc8));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = 1;
        let eq1 = c.g[A1][0] == c.g[V0][0];
        c.sb((c.g[S1][0] as u32).wrapping_add(0x49), c.g[A1][0]);
        let mode = c.g[A1][0];
        let case: u8;
        if eq1 {
            case = 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((mode as i64) < 2) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 2;
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                if mode == c.g[V0][0] {
                    case = 2;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x20);
                    c.cycles += 4;
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                if mode == 0 {
                    case = 0;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x20);
                    c.cycles += 4;
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
        }
        match case {
            0 => {
                c.cycles += 7;
                c.g[A1][0] = 1;
                call_ptr(c, 0x0014_4aa8, 0x0014_5164)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0014_4d98, 0x0014_5170)?;
            }
            1 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 1;
                call_ptr(c, 0x0014_4aa8, 0x0014_5184)?;
                c.g[V0][0] = Addr(c.g[S1][0] as u32 + 0x3c).lw(c);
                let ge = (c.g[V0][0] as i64) >= 0;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 6) as u64;
                if !ge {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0014_4d98, 0x0014_519c)?;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32 + 0x3c).lw(c);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 6) as u64;
                }
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 5;
                    call_ptr(c, 0x0014_4d98, 0x0014_51b8)?;
                }
            }
            _ => {
                c.cycles += 6;
                c.g[A1][0] = 2;
                call_ptr(c, 0x0014_4aa8, 0x0014_51c8)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0014_4d98, 0x0014_51d4)?;
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BERewardInt_AddNewReward(self, index, type_arg, value)` (0x15a628): bump
/// the per-type new-reward counter and, for indices 0..8, set the reward bit.
pub fn be_reward_int_add_new_reward(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 8;
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let eq8 = c.g[S0][0] == c.g[V0][0];
        c.g[A1][0] = c.g[A3][0];
        let index = c.g[S0][0];
        if eq8 {
            c.cycles += 5;
            c.g[V0][0] = 0xa;
            c.mult(0, V1, S2, V0);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.sb(c.g[V0][0] as u32 + 0xe, c.g[A1][0]);
        } else {
            c.cycles += 3;
            if (index as i64) < 9 {
                c.cycles += 2;
                if (index as i64) >= 0 {
                    c.cycles += 13;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A2][0] = Addr(c.g[V0][0] as u32 + 0x174).lw(c);
                    call_ptr(c, 0x0015_a5b0, 0x0015_a688)?;
                    c.g[V1][0] = 0xa;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(6));
                    c.mult(0, V0, S2, V1);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = 9;
                let eq9 = c.g[S0][0] == c.g[V0][0];
                frame.restore(c, S0, 0x30);
                if eq9 {
                    c.cycles += 6;
                    c.g[V0][0] = 0xa;
                    c.g[V1][0] = 1;
                    c.mult(0, A0, S2, V0);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.sb(c.g[V0][0] as u32 + 0xf, c.g[V1][0]);
                    c.sh(c.g[S1][0] as u32 + 4, c.g[A1][0]);
                } else {
                    c.cycles += 2;
                    frame.restore(c, S1, 0x20);
                    c.cycles += 4;
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
        }
        c.cycles += 2;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        c.cycles += 4;
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, ptr, vtable, LOG_RESULTS};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// The allocator mutex; make it look already held by "this thread" so the
    /// real `EA_MemAllocNamed`/`__builtin_delete` take their fast path.
    const MEM_MUTEX: u32 = 0x0053_8b00;

    fn alloc_setup(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(MEM_MUTEX + 4, 0x42);
        c.bus.write32(LOG_RESULTS, 0x42);
        c.bus.write32(MEM_MUTEX + 8, r.next_u32() % 4);
    }

    fn destroy_test(generated: GuestFn, hand: GuestFn, gp_off: u32, salt: u64) {
        check(generated, hand, 60, &|c, seed| {
            let mut r = machine(c, seed, salt, 6);
            let _ = &mut r;
            alloc_setup(c, &mut r);
            c.bus.write32(
                GAME_GP - gp_off,
                if seed % 3 == 0 { 0 } else { 0x0080_0000u32 + seed * 0x40 },
            );
        });
    }

    #[test]
    fn be_destroy_instances_match() {
        destroy_test(
            ssx3_ee::s01::f_0015c250,
            be_network_int_destroy_instance,
            NETWORK_GP,
            1,
        );
        destroy_test(
            ssx3_ee::s01::f_0015bc88,
            be_new_player_int_destroy_instance,
            NEW_PLAYER_GP,
            2,
        );
        destroy_test(
            ssx3_ee::s01::f_0015bb20,
            be_new_race_int_destroy_instance,
            NEW_RACE_GP,
            3,
        );
        destroy_test(
            ssx3_ee::s01::f_0015bd30,
            be_option_int_destroy_instance,
            OPTION_GP,
            4,
        );
        destroy_test(ssx3_ee::s01::f_0015c830, be_pre_race_destroy_instance, PRE_RACE_GP, 5);
        destroy_test(ssx3_ee::s01::f_0015c870, be_race_destroy, RACE_GP, 6);
        destroy_test(
            ssx3_ee::s01::f_0015c8b0,
            be_post_race_destroy_instance,
            POST_RACE_GP,
            7,
        );
    }

    fn get_instance_test(generated: GuestFn, hand: GuestFn, gp_off: u32, salt: u64) {
        check(generated, hand, 60, &|c, seed| {
            let mut r = machine(c, seed, salt, 6);
            alloc_setup(c, &mut r);
            c.bus.write32(GAME_GP - gp_off, if seed % 2 == 0 { 0 } else { 0x0080_0000u32 + seed * 8 });
            c.g[A0][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        });
    }

    #[test]
    fn be_get_instances_match() {
        get_instance_test(ssx3_ee::s01::f_00153088, be_race_get_instance, RACE_GP, 11);
        get_instance_test(ssx3_ee::s01::f_00156338, be_post_race_get_instance, POST_RACE_GP, 12);
        get_instance_test(ssx3_ee::s01::f_00156570, be_pre_race_get_instance, PRE_RACE_GP, 13);
        get_instance_test(ssx3_ee::s01::f_001569c0, be_reward_int_get_instance, REWARD_GP, 14);
    }

    /// The rewards file itself: point the global at test data.
    fn rewards_file(c: &mut Cpu, r: &mut Rng) {
        c.g[GP][0] = GAME_GP as u64;
        let f = 0x0080_0000;
        c.bus.write32(0x004c_3e98, f);
        for i in 0..16 {
            c.bus.write32(f + 4 * i, r.next_u32());
        }
    }

    #[test]
    fn be_rewards_field_getters_match() {
        for (gen, hand, off, salt) in [
            (ssx3_ee::s01::f_00156b70 as GuestFn, be_reward_int_get_section0 as GuestFn, 0x8u32, 21u64),
            (ssx3_ee::s01::f_00156b90, be_reward_int_get_section1, 0xc, 22),
            (ssx3_ee::s01::f_00156bb0, be_reward_int_get_section2, 0x10, 23),
            (ssx3_ee::s01::f_00156bd0, be_reward_int_get_poster_table, 0x14, 24),
            (ssx3_ee::s01::f_00156bf0, be_reward_int_get_trading_card_table, 0x18, 25),
            (ssx3_ee::s01::f_00156c10, be_reward_int_get_concept_art_table, 0x1c, 26),
            (ssx3_ee::s01::f_00156c30, be_reward_int_get_toy_table, 0x20, 27),
            (ssx3_ee::s01::f_00156c50, be_reward_int_get_video_table, 0x24, 28),
        ] {
            let _ = off;
            check(gen, hand, 120, &|c, seed| {
                let mut r = Rng(salt << 8 ^ seed as u64);
                scramble(c, &mut r);
                rewards_file(c, &mut r);
            });
        }
    }

    /// Point a rider id at a slot index and fill that slot.
    fn slot_setup(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        scramble(c, r);
        c.g[GP][0] = GAME_GP as u64;
        let rider = seed % 4;
        let idx = (seed / 4) % 3;
        c.bus.write32(0x0053_5b0 + rider * 4, idx);
        c.g[A1][0] = rider as u64;
        let base = PLAYER_SLOTS + idx * PLAYER_SLOT_STRIDE;
        for i in 0..8 {
            c.bus.write32(base + i * 4, r.next_u32());
        }
        base
    }

    #[test]
    fn be_new_player_slot_accessors_match() {
        check(
            ssx3_ee::s01::f_00147138,
            be_new_player_int_set_name_by_slot,
            120,
            &|c, seed| {
                let mut r = Rng(0x7138_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let idx = seed % 3;
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = 0x0080_0200u64;
                for i in 0..4 {
                    c.bus.write32(0x0080_0200 + i * 4, r.next_u32());
                }
                for i in 0..8 {
                    c.bus.write32(PLAYER_SLOTS + idx * PLAYER_SLOT_STRIDE + i * 4, r.next_u32());
                }
            },
        );
        check(
            ssx3_ee::s01::f_00147220,
            be_new_player_int_set_pad_index,
            200,
            &|c, seed| {
                let mut r = Rng(0x7220_0000 ^ seed as u64);
                slot_setup(c, &mut r, seed);
                c.g[A2][0] = if seed % 2 == 0 {
                    r.next_u32() as u64
                } else {
                    ((r.next_u32() as u64) << 32) | r.next_u32() as u64
                };
            },
        );
        check(
            ssx3_ee::s01::f_00147338,
            be_new_player_int_set_character,
            200,
            &|c, seed| {
                let mut r = Rng(0x7338_0000 ^ seed as u64);
                slot_setup(c, &mut r, seed);
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
        check(
            ssx3_ee::s01::f_001474e8,
            be_new_player_int_get_vibration,
            150,
            &|c, seed| {
                let mut r = Rng(0x74e8_0000 ^ seed as u64);
                slot_setup(c, &mut r, seed);
            },
        );
        check(
            ssx3_ee::s01::f_00147528,
            be_new_player_int_set_vibration,
            150,
            &|c, seed| {
                let mut r = Rng(0x7528_0000 ^ seed as u64);
                slot_setup(c, &mut r, seed);
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
        check(
            ssx3_ee::s01::f_00147618,
            be_new_player_int_get_control_map,
            150,
            &|c, seed| {
                let mut r = Rng(0x7618_0000 ^ seed as u64);
                slot_setup(c, &mut r, seed);
            },
        );
        check(
            ssx3_ee::s01::f_00147658,
            be_new_player_int_set_control_map,
            150,
            &|c, seed| {
                let mut r = Rng(0x7658_0000 ^ seed as u64);
                slot_setup(c, &mut r, seed);
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn be_option_int_set_hud_defaults_matches() {
        covered(
            ssx3_ee::s01::f_0014f600,
            be_option_int_set_hud_defaults,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0xf600, 1);
                let opt = ptr(&mut r);
                c.bus.write32(GAME_GP - OPTION_GP, opt);
                let profile = ptr(&mut r);
                c.bus.write32(opt + 0xc, profile);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(profile + 0xc, vt);
                c.bus.write32(0x0053_5610, r.next_u32());
            },
        );
    }

    #[test]
    fn be_option_int_get_quick_message_matches() {
        check(
            ssx3_ee::s01::f_0014f6a8,
            be_option_int_get_quick_message,
            150,
            &|c, seed| {
                let mut r = Rng(0xf6a8_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let idx = seed % 4;
                c.g[A1][0] = idx as u64;
                let p = 0x0053_5617 + idx * 0x40;
                if seed % 3 == 0 {
                    c.bus.write8(p, 0);
                } else {
                    for i in 0..8 {
                        c.bus.write8(p + i, (r.next_u32() | 1) as u8);
                    }
                    c.bus.write8(p + 8, 0);
                }
            },
        );
    }

    #[test]
    fn be_option_int_get_quick_message_key_matches() {
        check(
            ssx3_ee::s01::f_0014f678,
            be_option_int_get_quick_message_key,
            100,
            &|c, seed| {
                let mut r = Rng(0xf678_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    (seed % 100) as u64
                } else {
                    r.next_u32() as u64
                };
                // A tiny format string at the address the original uses.
                for (i, b) in [b'%', b'd', 0, 0].into_iter().enumerate() {
                    c.bus.write8(0x0045_a6c0 + i as u32, b);
                }
            },
        );
    }

    #[test]
    fn be_rewards_counts_match() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_001572b0 as GuestFn, be_reward_int_get_poster_count as GuestFn, 31u64),
            (ssx3_ee::s01::f_001572d0, be_reward_int_get_trading_card_count, 32),
            (ssx3_ee::s01::f_001572f0, be_reward_int_get_concept_art_count, 33),
            (ssx3_ee::s01::f_00157310, be_reward_int_get_toy_count, 34),
            (ssx3_ee::s01::f_00157330, be_reward_int_get_video_count, 35),
            (ssx3_ee::s01::f_00157370, be_reward_int_get_cheat_char_count, 36),
        ] {
            check(gen, hand, 100, &|c, seed| {
                let mut r = Rng(salt << 8 ^ seed as u64);
                scramble(c, &mut r);
                rewards_file(c, &mut r);
            });
        }
    }

    #[test]
    fn be_reward_entries_match() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_00156a90 as GuestFn, be_reward_int_get_section0_entry as GuestFn, 41u64),
            (ssx3_ee::s01::f_00156ae0, be_reward_int_get_section1_entry, 42),
            (ssx3_ee::s01::f_00156b28, be_reward_int_get_section2_entry, 43),
        ] {
            check(gen, hand, 100, &|c, seed| {
                let mut r = Rng(salt << 8 ^ seed as u64);
                scramble(c, &mut r);
                rewards_file(c, &mut r);
                c.g[A1][0] = (seed % 7) as u64;
                c.g[A2][0] = ((seed / 7) % 5) as u64;
            });
        }
    }

    #[test]
    fn be_reward_count_set_bits_matches() {
        check(ssx3_ee::s01::f_00157390, be_reward_int_count_set_bits, 200, &|c, seed| {
            let mut r = Rng(0x7390_0000 ^ seed as u64);
            scramble(c, &mut r);
            let n = match seed % 8 {
                0 => 0,
                1 => 1,
                _ => (seed % 60) as u64 + 2,
            };
            c.g[A1][0] = n;
            c.g[A2][0] = (DATA + 0x600) as u64;
            for i in 0..32 {
                c.bus.write8(DATA + 0x600 + i, r.next_u32() as u8);
            }
        });
    }

    #[test]
    fn be_reward_clear_new_rewards_matches() {
        check(ssx3_ee::s01::f_0015a510, be_reward_int_clear_new_rewards, 60, &|c, seed| {
            let mut r = Rng(0xa510_0000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            for i in 0..0x60 {
                c.bus.write32(DATA + 0x1000 + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn be_new_race_match() {
        check(ssx3_ee::s01::f_00144aa8, be_new_race_int_set_num_human_players, 120, &|c, seed| {
            let mut r = Rng(0x44aa_8000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | (seed % 8) as u64;
            for i in 0..0x80 {
                c.bus.write32(0x0053_5bc8 + i * 4, r.next_u32());
            }
        });
        check(ssx3_ee::s01::f_00144b20, be_new_race_int_set_track_strings, 60, &|c, seed| {
            let mut r = Rng(0x44b2_0000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = (DATA + 0x200) as u64;
            for i in 0..8 {
                c.bus.write8(DATA + 0x200 + i, (r.next_u32() | 1) as u8);
            }
            c.bus.write8(DATA + 0x208, 0);
            c.bus.write32(0x0053_5bc8 + 0x40, 0);
            for i in 0..0x60 {
                c.bus.write32(0x0043_d900 + i * 4, 0);
            }
        });
    }

    #[test]
    fn be_save_score_match() {
        for (gen, hand, off, salt) in [
            (ssx3_ee::s01::f_00152638 as GuestFn, be_save_int_get_instance as GuestFn, 0x1ed4u32, 51u64),
            (ssx3_ee::s01::f_00152700, be_save_int_free_save_buffer, 0x1ed0, 52),
            (ssx3_ee::s01::f_00153050, be_save_int_destroy, 0x1ed4, 53),
        ] {
            check(gen, hand, 60, &|c, seed| {
                let mut r = machine(c, seed, salt, 6);
                alloc_setup(c, &mut r);
                c.bus.write32(GAME_GP - off, if seed % 2 == 0 { 0 } else { DATA + 0x400 });
                if seed % 3 == 0 {
                    c.bus.write32(GAME_GP - 0x1ed0, DATA + 0x800);
                }
            });
        }
        covered(ssx3_ee::s01::f_00152688, be_save_int_get_save_buffer, 60, &|c, seed| {
            let mut r = machine(c, seed, 54, 6);
            alloc_setup(c, &mut r);
            c.bus.write32(GAME_GP - 0x1ecc, if seed % 2 == 0 { 0 } else { DATA + 0x400 });
        });
    }

    #[test]
    fn be_reward_predicates_match() {
        fn pred(generated: GuestFn, hand: GuestFn, extra: u32, salt: u64, rider: bool) {
            check(generated, hand, 120, &|c, seed| {
                let mut r = Rng(salt << 8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[A3][0] = if rider { (seed % 200 + 10) as u64 } else { (seed % 200) as u64 };
                c.g[V1][0] = (seed % 0x100) as u64;
                let base = REWARD_BASE + extra;
                for i in 0..64 {
                    c.bus.write8(base + i, r.next_u32() as u8);
                }
            });
        }
        pred(ssx3_ee::s01::f_00157d60, be_reward_int_is_poster_unlocked, 0xf30, 71, false);
        pred(ssx3_ee::s01::f_00157da0, be_reward_int_is_trading_card_unlocked, 0xf36, 72, false);
        pred(ssx3_ee::s01::f_00157de0, be_reward_int_is_concept_art_unlocked, 0xf45, 73, false);
        pred(ssx3_ee::s01::f_00157e20, be_reward_int_is_toy_unlocked, 0xf53, 74, false);
        pred(ssx3_ee::s01::f_00157e60, be_reward_int_is_video_unlocked, 0xf52, 75, false);
        pred(ssx3_ee::s01::f_00157ee0, be_reward_int_is_rider_item_unlocked, 0xf57, 76, true);
        predeg(ssx3_ee::s01::f_0015a2e0, be_reward_int_is_reward_granted, 0xf28);
    }

    fn predeg(generated: GuestFn, hand: GuestFn, extra: u32) {
        check(generated, hand, 120, &|c, seed| {
            let mut r = Rng(0xa2e0_0000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.g[A3][0] = (seed % 200) as u64;
            c.g[V1][0] = (seed % 0x100) as u64;
            let base = REWARD_BASE + extra;
            for i in 0..64 {
                c.bus.write8(base + i, r.next_u32() as u8);
            }
        });
    }

    #[test]
    fn be_reward_count_unlocked_match() {
        fn counts(generated: GuestFn, hand: GuestFn, extra: u32, salt: u64, _rider: bool) {
            check(generated, hand, 120, &|c, seed| {
                let mut r = Rng(salt << 8 ^ seed as u64);
                scramble(c, &mut r);
                rewards_file(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[V1][0] = (seed % 0x100) as u64;
                let bits = (seed % 60) as u32;
                let f = 0x0080_0000;
                for off in [0x3c, 0x40, 0x44, 0x48, 0x4c] {
                    c.bus.write32(f + off, bits);
                }
                let base = REWARD_BASE + extra;
                for i in 0..64 {
                    c.bus.write8(base + i, r.next_u32() as u8);
                }
            });
        }
        counts(ssx3_ee::s01::f_001575c0, be_reward_int_count_unlocked_posters, 0xf30, 51, false);
        counts(ssx3_ee::s01::f_00157620, be_reward_int_count_unlocked_trading_cards, 0xf36, 52, false);
        counts(ssx3_ee::s01::f_00157680, be_reward_int_count_unlocked_concept_arts, 0xf45, 53, false);
        counts(ssx3_ee::s01::f_001576e0, be_reward_int_count_unlocked_toys, 0xf53, 54, false);
        counts(ssx3_ee::s01::f_00157740, be_reward_int_count_unlocked_videos, 0xf52, 55, false);
        counts(ssx3_ee::s01::f_001577a0, be_reward_int_count_unlocked_rider_items, 0xf57, 56, true);
    }

    #[test]
    fn be_reward_type_count_matches() {
        check(
            ssx3_ee::s01::f_00157210,
            be_reward_int_get_reward_type_count,
            120,
            &|c, seed| {
                let mut r = Rng(0x7210_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0015_7210;
                rewards_file(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let ty = match seed % 10 {
                    0 => 0,
                    1 => 2,
                    2 => 10,
                    _ => 3 + (seed % 7),
                };
                c.g[A1][0] = ty as u64;
                for (i, v) in [
                    0x0015_7244u32,
                    0x0015_7254,
                    0x0015_7264,
                    0x0015_7274,
                    0x0015_7284,
                    0x0015_7294,
                    0x0015_729c,
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus.write32(0x0045_a970 + 4 * i as u32, v);
                }
            },
        );
    }

    #[test]
    fn be_reward_unlocked_dispatch_match() {
        check(
            ssx3_ee::s01::f_00157468,
            be_reward_int_is_reward_unlocked,
            120,
            &|c, seed| {
                let mut r = Rng(0x7468_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0015_7468;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[T0][0] = (seed % 200) as u64;
                c.g[V1][0] = (seed % 0x100) as u64;
                let ty = match seed % 9 {
                    0 => 0,
                    1 => 9,
                    _ => 3 + (seed % 6),
                };
                c.g[A3][0] = ty as u64;
                for (i, v) in [
                    0x0015_74a0u32,
                    0x0015_74b0,
                    0x0015_74c0,
                    0x0015_74d0,
                    0x0015_74e0,
                    0x0015_74f0,
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus.write32(0x0045_a990 + 4 * i as u32, v);
                }
                for extra in [0xf30u32, 0xf36, 0xf45, 0xf53, 0xf52, 0xf57] {
                    let base = REWARD_BASE + extra;
                    for i in 0..64 {
                        c.bus.write8(base + i, r.next_u32() as u8);
                    }
                }
            },
        );
        check(
            ssx3_ee::s01::f_00157518,
            be_reward_int_count_unlocked,
            120,
            &|c, seed| {
                let mut r = Rng(0x7518_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0015_7518;
                rewards_file(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[V1][0] = (seed % 0x100) as u64;
                let ty = match seed % 9 {
                    0 => 0,
                    1 => 9,
                    _ => 3 + (seed % 6),
                };
                c.g[A3][0] = ty as u64;
                let f = 0x0080_0000;
                for off in [0x3c, 0x40, 0x44, 0x48, 0x4c] {
                    c.bus.write32(f + off, (seed % 60) as u32);
                }
                for (i, v) in [
                    0x0015_754cu32,
                    0x0015_755c,
                    0x0015_756c,
                    0x0015_757c,
                    0x0015_758c,
                    0x0015_759c,
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus.write32(0x0045_a9b0 + 4 * i as u32, v);
                }
                for extra in [0xf30u32, 0xf36, 0xf45, 0xf53, 0xf52, 0xf57] {
                    let base = REWARD_BASE + extra;
                    for i in 0..64 {
                        c.bus.write8(base + i, r.next_u32() as u8);
                    }
                }
            },
        );
    }

    #[test]
    fn be_reward_get_new_reward_total_matches() {
        check(
            ssx3_ee::s01::f_00158f30,
            be_reward_int_get_new_reward_total,
            60,
            &|c, seed| {
                let mut r = Rng(0x8f30_0000 ^ seed as u64);
                scramble(c, &mut r);
                for i in 0..0x200 {
                    c.bus.write8(0x004c_3ef0 + i, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn be_reward_count_new_rewards_matches() {
        check(
            ssx3_ee::s01::f_0015a6f0,
            be_reward_int_count_new_rewards,
            100,
            &|c, seed| {
                let mut r = Rng(0xa6f0_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA + 0x3000) as u64;
                for i in 0..0x200 {
                    c.bus.write8(DATA + 0x3000 + i, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn be_reward_get_event_best_place_matches() {
        check(
            ssx3_ee::s01::f_00158910,
            be_reward_int_get_event_best_place,
            40,
            &|c, seed| {
                let mut r = Rng(0x8910_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[T0][0] = (seed % 8) as u64;
                c.g[A3][0] = (seed % 4) as u64;
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn be_new_player_object_match() {
        covered(ssx3_ee::s01::f_00145930, be_new_player_int_ctor, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x5930, 1);
            c.g[A0][0] = DATA as u64;
            for i in 0..0x40 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
            // The template ResetSlots copies from.
            for i in 0..0x30 {
                c.bus.write32(0x0053_5b20 + i * 4, r.next_u32());
            }
        });
        check(
            ssx3_ee::s01::f_00145b20,
            be_new_player_int_reset_slots,
            80,
            &|c, seed| {
                let mut r = Rng(0x5b20_0000 ^ seed as u64);
                scramble(c, &mut r);
                for i in 0..0x30 {
                    c.bus.write32(0x0053_5b20 + i * 4, r.next_u32());
                }
                for i in 0..0x30 {
                    c.bus.write32(0x0053_4fe0 + i * 4, r.next_u32());
                }
            },
        );
        check(ssx3_ee::s01::f_00146f40, be_new_player_int_set_name, 80, &|c, seed| {
            let mut r = Rng(0x6f40_0000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
            c.bus.write32(0x0053_5b0 + (seed % 4) * 4, (seed % 3) as u32);
            for i in 0..4 {
                c.bus.write32(DATA + 0x200 + i * 4, r.next_u32());
            }
            for i in 0..0x30 {
                c.bus.write32(PLAYER_SLOTS + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn be_new_player_first_time_wc_match() {
        fn setup(c: &mut Cpu, seed: u32) {
            let mut r = Rng(0x5c38_0000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A1][0] = 0;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(0x0053_5b0, 0);
            c.bus.write32(PLAYER_SLOTS + 0xc, 0xffff_ffff);
            c.bus.write8(PLAYER_SLOTS + 0x11, 0);
            // profile 2 -> +2*0x9b50.
            for i in 0..8 {
                c.bus.write32(REWARD_BASE + 2 * REWARD_STRIDE_A + i * 4, r.next_u32());
            }
        }
        check(
            ssx3_ee::s01::f_00145c38,
            be_new_player_int_get_first_time_wc_flag,
            80,
            &|c, seed| setup(c, seed),
        );
        check(
            ssx3_ee::s01::f_00145cb0,
            be_new_player_int_set_first_time_wc_flag,
            80,
            &|c, seed| setup(c, seed),
        );
    }

    #[test]
    fn be_option_int_ctor_matches() {
        covered(ssx3_ee::s01::f_0014f418, be_option_int_ctor, 60, &|c, seed| {
            let mut r = machine(c, seed, 0xf418, 3);
            let world = ptr(&mut r);
            c.bus.write32(GAME_GP - 0x848, world);
            let sub = ptr(&mut r);
            c.bus.write32(world + 0x9c, sub);
            c.bus.write32(sub + 0x24, (seed % 4) as u32);
            let opt = ptr(&mut r);
            c.bus.write32(GAME_GP - OPTION_GP, opt);
            let profile = ptr(&mut r);
            c.bus.write32(opt + 0xc, profile);
            let vt = vtable(c, &mut r, &[8]);
            c.bus.write32(profile + 0xc, vt);
            c.bus.write32(0x0053_5610, r.next_u32());
            c.bus.write32(0x0053_5614, r.next_u32());
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn be_option_defaults_match() {
        covered(
            ssx3_ee::s01::f_0014f458,
            be_option_int_set_sound_defaults,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0xf458, 1);
                let opt = ptr(&mut r);
                c.bus.write32(GAME_GP - OPTION_GP, opt);
                let profile = ptr(&mut r);
                c.bus.write32(opt + 0xc, profile);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(profile + 0xc, vt);
                c.bus.write32(0x0053_5610, r.next_u32());
                c.bus.write32(0x0053_5614, r.next_u32());
            },
        );
        covered(
            ssx3_ee::s01::f_0014f4f8,
            be_option_int_set_game_defaults,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0xf4f8, 1);
                let world = ptr(&mut r);
                c.bus.write32(GAME_GP - 0x848, world);
                let sub = ptr(&mut r);
                c.bus.write32(world + 0x9c, sub);
                c.bus.write32(sub + 0x24, (seed % 4) as u32);
                let opt = ptr(&mut r);
                c.bus.write32(GAME_GP - OPTION_GP, opt);
                let profile = ptr(&mut r);
                c.bus.write32(opt + 0xc, profile);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(profile + 0xc, vt);
                c.bus.write32(0x0053_5610, r.next_u32());
                c.bus.write32(0x0053_5614, r.next_u32());
            },
        );
    }

    #[test]
    fn be_option_quick_messages_match() {
        fn qm_setup(c: &mut Cpu, r: &mut Rng, seed: u32) {
            let world = ptr(r);
            c.bus.write32(GAME_GP - 0x848, world);
            let obj = ptr(r);
            c.bus.write32(world + 0x8c, obj);
            let ctrl = ptr(r);
            c.bus.write32(obj + 4, ctrl);
            c.bus.write16(ctrl + 0x20, (r.next_u32() % 4) as u16);
            let vt = vtable(c, r, &[0x24]);
            c.bus.write32(ctrl + 0x24, vt);
            // GetQuickMessageKey's format string.
            for (i, b) in [b'%', b'd', 0, 0].into_iter().enumerate() {
                c.bus.write8(0x0045_a6c0 + i as u32, b);
            }
            let _ = seed;
        }
        check(
            ssx3_ee::s01::f_0014f6e8,
            be_option_int_reset_quick_message,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xf6e8, 1);
                qm_setup(c, &mut r, seed);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 10) as u64;
            },
        );
        check(
            ssx3_ee::s01::f_0014f758,
            be_option_int_reset_quick_messages,
            20,
            &|c, seed| {
                let mut r = machine(c, seed, 0xf758, 1);
                qm_setup(c, &mut r, seed);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn be_score_int_update_best_stats_match() {
        check(
            ssx3_ee::s01::f_00154a58,
            be_score_int_update_best_stats,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x4a58, 1);
                c.g[A0][0] = 0;
                c.g[A1][0] = DATA as u64;
                c.bus.write32(0x0053_5b0, 0);
                c.bus.write32(0x0053_5b20 + 0xc, if seed % 3 == 0 { 0 } else { 0xffff_ffff });
                c.bus.write32(0x0053_5b20 + 0x10, (seed % 2) as u32);
                c.bus.write8(0x0053_05f9, (seed % 5 == 0) as u8);
            },
        );
    }

    #[test]
    fn be_new_race_game_mode_match() {
        check(
            ssx3_ee::s01::f_00145108,
            be_new_race_int_set_game_mode,
            100,
            &|c, seed| {
                let mut r = Rng(0x5108_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = (DATA + 0x800) as u64;
                c.g[A1][0] = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 0xffff_ffff_ffff_ffff,
                    _ => 0x10,
                };
                for i in 0..0x30 {
                    c.bus.write32(0x0053_5bc8 + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn be_reward_add_new_reward_match() {
        check(
            ssx3_ee::s01::f_0015a628,
            be_reward_int_add_new_reward,
            100,
            &|c, seed| {
                let mut r = Rng(0xa628_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 12) as u64;
                c.g[A3][0] = (seed % 200) as u64;
                for i in 0..0x60 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
                for index in 0..10 {
                    c.bus.write32(DATA + index * 4 + 0x174, DATA + 0x3000);
                }
                for i in 0..64 {
                    c.bus.write8(DATA + 0x3000 + i, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn be_score_int_match() {
        check(ssx3_ee::s01::f_001549a8, be_score_int_ctor, 60, &|c, seed| {
            let mut r = machine(c, seed, 61, 6);
            alloc_setup(c, &mut r);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            for i in 0..0x80 {
                c.bus.write32(DATA + 0x2000 + i * 4, r.next_u32());
            }
        });
        check(ssx3_ee::s01::f_001549e8, be_score_int_v02, 60, &|c, seed| {
            let mut r = Rng(0x49e8_0000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            for i in 0..0x80 {
                c.bus.write32(DATA + 0x2000 + i * 4, r.next_u32());
            }
        });
    }
}
