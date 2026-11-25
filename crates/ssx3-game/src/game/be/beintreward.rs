//! `src/be/beintreward.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::be::beintnewrace::TRACK_RECORDS;
pub(crate) use crate::game::be::beintplayer::mul32;
pub(crate) use crate::game::bx::bxstringctor::MESSAGE_MAP;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::fepopup::at as at_3;
pub(crate) use crate::game::fe::fepopup::lb;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::festaterewards::r;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::p_2 as p;
pub(crate) use crate::game::main::ssxapp::oc429_at;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
use crate::guest::Addr;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0015_69c0,
        be_reward_int_get_instance,
        "BERewardInt_GetInstance",
    ),
    (
        0x0015_6a38,
        be_reward_int_is_rewards_file_loaded,
        "BERewardInt_IsRewardsFileLoaded",
    ),
    (
        0x0015_6a60,
        be_reward_int_is_better_place,
        "BERewardInt_IsBetterPlace",
    ),
    (
        0x0015_6a90,
        be_reward_int_get_section0_entry,
        "BERewardInt_GetSection0Entry",
    ),
    (
        0x0015_6ae0,
        be_reward_int_get_section1_entry,
        "BERewardInt_GetSection1Entry",
    ),
    (
        0x0015_6b28,
        be_reward_int_get_section2_entry,
        "BERewardInt_GetSection2Entry",
    ),
    (
        0x0015_6b70,
        be_reward_int_get_section0,
        "BERewardInt_GetSection0",
    ),
    (
        0x0015_6b90,
        be_reward_int_get_section1,
        "BERewardInt_GetSection1",
    ),
    (
        0x0015_6bb0,
        be_reward_int_get_section2,
        "BERewardInt_GetSection2",
    ),
    (
        0x0015_6bd0,
        be_reward_int_get_poster_table,
        "BERewardInt_GetPosterTable",
    ),
    (
        0x0015_6bf0,
        be_reward_int_get_trading_card_table,
        "BERewardInt_GetTradingCardTable",
    ),
    (
        0x0015_6c10,
        be_reward_int_get_concept_art_table,
        "BERewardInt_GetConceptArtTable",
    ),
    (
        0x0015_6c30,
        be_reward_int_get_toy_table,
        "BERewardInt_GetToyTable",
    ),
    (
        0x0015_6c50,
        be_reward_int_get_video_table,
        "BERewardInt_GetVideoTable",
    ),
    (
        0x0015_7210,
        be_reward_int_get_reward_type_count,
        "BERewardInt_GetRewardTypeCount",
    ),
    (
        0x0015_72b0,
        be_reward_int_get_poster_count,
        "BERewardInt_GetPosterCount",
    ),
    (
        0x0015_72d0,
        be_reward_int_get_trading_card_count,
        "BERewardInt_GetTradingCardCount",
    ),
    (
        0x0015_72f0,
        be_reward_int_get_concept_art_count,
        "BERewardInt_GetConceptArtCount",
    ),
    (
        0x0015_7310,
        be_reward_int_get_toy_count,
        "BERewardInt_GetToyCount",
    ),
    (
        0x0015_7330,
        be_reward_int_get_video_count,
        "BERewardInt_GetVideoCount",
    ),
    (
        0x0015_7370,
        be_reward_int_get_cheat_char_count,
        "BERewardInt_GetCheatCharCount",
    ),
    (
        0x0015_7390,
        be_reward_int_count_set_bits,
        "BERewardInt_CountSetBits",
    ),
    (0x0015_73f0, be_reward_int_set_bit, "BERewardInt_SetBit"),
    (0x0015_7430, be_reward_int_test_bit, "BERewardInt_TestBit"),
    (
        0x0015_7468,
        be_reward_int_is_reward_unlocked,
        "BERewardInt_IsRewardUnlocked",
    ),
    (
        0x0015_7518,
        be_reward_int_count_unlocked,
        "BERewardInt_CountUnlocked",
    ),
    (
        0x0015_75c0,
        be_reward_int_count_unlocked_posters,
        "BERewardInt_CountUnlockedPosters",
    ),
    (
        0x0015_7620,
        be_reward_int_count_unlocked_trading_cards,
        "BERewardInt_CountUnlockedTradingCards",
    ),
    (
        0x0015_7680,
        be_reward_int_count_unlocked_concept_arts,
        "BERewardInt_CountUnlockedConceptArts",
    ),
    (
        0x0015_76e0,
        be_reward_int_count_unlocked_toys,
        "BERewardInt_CountUnlockedToys",
    ),
    (
        0x0015_7740,
        be_reward_int_count_unlocked_videos,
        "BERewardInt_CountUnlockedVideos",
    ),
    (
        0x0015_77a0,
        be_reward_int_count_unlocked_rider_items,
        "BERewardInt_CountUnlockedRiderItems",
    ),
    (
        0x0015_78a0,
        be_reward_int_are_all_peaks_complete,
        "BERewardInt_AreAllPeaksComplete",
    ),
    (
        0x0015_7a78,
        be_reward_int_is_peak_complete,
        "BERewardInt_IsPeakComplete",
    ),
    (
        0x0015_7b08,
        be_reward_int_is_rival_reward_pending,
        "BERewardInt_IsRivalRewardPending",
    ),
    (
        0x0015_7b70,
        berewardint_is_rival_reward_pending_for,
        "BERewardInt_IsRivalRewardPendingFor",
    ),
    (
        0x0015_7bf0,
        be_reward_int_is_peak_goal_complete,
        "BERewardInt_IsPeakGoalComplete",
    ),
    (
        0x0015_7d60,
        be_reward_int_is_poster_unlocked,
        "BERewardInt_IsPosterUnlocked",
    ),
    (
        0x0015_7da0,
        be_reward_int_is_trading_card_unlocked,
        "BERewardInt_IsTradingCardUnlocked",
    ),
    (
        0x0015_7de0,
        be_reward_int_is_concept_art_unlocked,
        "BERewardInt_IsConceptArtUnlocked",
    ),
    (
        0x0015_7e20,
        be_reward_int_is_toy_unlocked,
        "BERewardInt_IsToyUnlocked",
    ),
    (
        0x0015_7e60,
        be_reward_int_is_video_unlocked,
        "BERewardInt_IsVideoUnlocked",
    ),
    (
        0x0015_7ee0,
        be_reward_int_is_rider_item_unlocked,
        "BERewardInt_IsRiderItemUnlocked",
    ),
    (
        0x0015_8700,
        be_reward_int_get_audio_setting,
        "BERewardInt_GetAudioSetting",
    ),
    (
        0x0015_8728,
        be_reward_int_get_unlocked_songs,
        "BERewardInt_GetUnlockedSongs",
    ),
    (
        0x0015_8750,
        be_reward_int_get_enabled_songs,
        "BERewardInt_GetEnabledSongs",
    ),
    (
        0x0015_87b8,
        be_reward_int_count_enabled_songs,
        "BERewardInt_CountEnabledSongs",
    ),
    (
        0x0015_87f8,
        be_reward_int_set_audio_setting,
        "BERewardInt_SetAudioSetting",
    ),
    (
        0x0015_8820,
        be_reward_int_set_unlocked_songs,
        "BERewardInt_SetUnlockedSongs",
    ),
    (
        0x0015_8848,
        be_reward_int_set_enabled_songs,
        "BERewardInt_SetEnabledSongs",
    ),
    (
        0x0015_8870,
        be_reward_int_get_collectible_threshold,
        "BERewardInt_GetCollectibleThreshold",
    ),
    (
        0x0015_8890,
        be_reward_int_get_mission_threshold,
        "BERewardInt_GetMissionThreshold",
    ),
    (
        0x0015_8910,
        be_reward_int_get_event_best_place,
        "BERewardInt_GetEventBestPlace",
    ),
    (
        0x0015_8960,
        be_reward_int_get_event_record_value,
        "BERewardInt_GetEventRecordValue",
    ),
    (
        0x0015_89b0,
        be_reward_int_get_collectible_tier,
        "BERewardInt_GetCollectibleTier",
    ),
    (
        0x0015_8a50,
        be_reward_int_get_mission_tier,
        "BERewardInt_GetMissionTier",
    ),
    (
        0x0015_8af0,
        berewardint_has_points_tier,
        "BERewardInt_HasPointsTier",
    ),
    (
        0x0015_8be0,
        be_reward_int_get_next_points_threshold,
        "BERewardInt_GetNextPointsThreshold",
    ),
    (
        0x0015_8f20,
        be_reward_int_get_new_rewards,
        "BERewardInt_GetNewRewards",
    ),
    (
        0x0015_8f30,
        be_reward_int_get_new_reward_total,
        "BERewardInt_GetNewRewardTotal",
    ),
    (
        0x0015_8f60,
        be_reward_int_grant_rival_rewards,
        "BERewardInt_GrantRivalRewards",
    ),
    (
        0x0015_9080,
        bereward_int_get_medal_reward_id,
        "BERewardInt_GetMedalRewardId",
    ),
    (
        0x0015_9170,
        be_reward_int_grant_medal_reward,
        "BERewardInt_GrantMedalReward",
    ),
    (
        0x0015_99a0,
        be_reward_int_on_collectible_collected,
        "BERewardInt_OnCollectibleCollected",
    ),
    (
        0x0015_9b08,
        be_reward_int_on_mission_completed,
        "BERewardInt_OnMissionCompleted",
    ),
    (
        0x0015_a2e0,
        be_reward_int_is_reward_granted,
        "BERewardInt_IsRewardGranted",
    ),
    (
        0x0015_a320,
        reward_test_granted_bit,
        "BERewardInt_TestGrantedBit",
    ),
    (
        0x0015_a358,
        be_reward_int_set_granted_bit,
        "BERewardInt_SetGrantedBit",
    ),
    (
        0x0015_a398,
        be_reward_int_get_track_peak,
        "BERewardInt_GetTrackPeak",
    ),
    (
        0x0015_a478,
        be_reward_int_get_rewards_file,
        "BERewardInt_GetRewardsFile",
    ),
    (
        0x0015_a488,
        be_reward_int_new_rewards_ctor,
        "BERewardInt_NewRewardsCtor",
    ),
    (
        0x0015_a510,
        be_reward_int_clear_new_rewards,
        "BERewardInt_ClearNewRewards",
    ),
    (
        0x0015_a5b0,
        be_reward_int_new_rewards_set_bit,
        "BERewardInt_NewRewardsSetBit",
    ),
    (
        0x0015_a5f0,
        reward_new_rewards_test_bit,
        "BERewardInt_NewRewardsTestBit",
    ),
    (
        0x0015_a628,
        be_reward_int_add_new_reward,
        "BERewardInt_AddNewReward",
    ),
    (
        0x0015_a6f0,
        be_reward_int_count_new_rewards,
        "BERewardInt_CountNewRewards",
    ),
    (
        0x0015_a750,
        be_reward_int_get_new_reward_count,
        "BERewardInt_GetNewRewardCount",
    ),
    (
        0x0015_a768,
        be_reward_int_get_new_reward_bits,
        "BERewardInt_GetNewRewardBits",
    ),
    (0x0015_a778, rewards_file_ctor, "RewardsFile_ctor"),
    (0x0015_a7d0, rewards_file_dtor, "RewardsFile_dtor"),
    (0x0015_a818, rewards_file_parse, "RewardsFile_Parse"),
    (
        0x0015_8c80,
        be_reward_int_get_next_collectible_threshold,
        "BERewardInt_GetNextCollectibleThreshold",
    ),
    (
        0x0015_8d58,
        be_reward_int_get_next_mission_threshold,
        "BERewardInt_GetNextMissionThreshold",
    ),
    (
        0x0015_8e30,
        be_reward_int_begin_reward_tracking,
        "BERewardInt_BeginRewardTracking",
    ),
    (
        0x0015_7350,
        be_reward_int_get_audio_system_field504,
        "BERewardInt_GetAudioSystemField504",
    ),
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
    (
        0x0015_9cd0,
        be_reward_int_grant_reward,
        "BERewardInt_GrantReward",
    ),
    (
        0x0015_6ee0,
        be_reward_int_pick_random_bolt_flag800,
        "BERewardInt_PickRandomBoltFlag800",
    ),
    (
        0x0015_7920,
        be_reward_int_is_peak_gold_complete,
        "BERewardInt_IsPeakGoldComplete",
    ),
    (
        0x0015_8470,
        be_reward_int_unlock_video,
        "BERewardInt_UnlockVideo",
    ),
    (
        0x0015_8618,
        be_reward_int_unlock_rider_item,
        "BERewardInt_UnlockRiderItem",
    ),
    (
        0x0015_97b0,
        be_reward_int_add_points_for_rider,
        "BERewardInt_AddPointsForRider",
    ),
    (
        0x0015_9818,
        be_reward_int_add_points,
        "BERewardInt_AddPoints",
    ),
    (
        0x0015_a3b8,
        be_reward_int_find_requirement_tier_2,
        "BERewardInt_FindRequirementTier",
    ),
    (
        0x0015_6a10,
        be_reward_int_load_rewards_file_3,
        "BERewardInt_LoadRewardsFile",
    ),
    (
        0x0015_6c70,
        be_reward_int_pick_random_bolt,
        "BERewardInt_PickRandomBolt",
    ),
    (
        0x0015_7080,
        be_reward_int_pick_random_locked_reward,
        "BERewardInt_PickRandomLockedReward",
    ),
    (
        0x0015_7f20,
        be_reward_int_unlock_reward,
        "BERewardInt_UnlockReward",
    ),
    (
        0x0015_91e8,
        beint_reward_on_event_result,
        "BERewardInt_OnEventResult",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::be::beintmission::callee::REWARDS_FILE_LOAD;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::REWARDS_FILE_UNLOAD;

    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COLLECTED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COMPLETED_ON_PEAK;

    pub(crate) use crate::game::fe::fepopup::callee::C0014A080;
    pub(crate) use crate::game::fe::fepopup::callee::C0014A0E0;

    pub(crate) use crate::game::fe::fepopup::callee::C0014DC80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014DD58;

    pub(crate) use crate::game::fe::fepopup::callee::C001535C0;
    pub(crate) use crate::game::fe::fepopup::callee::C001543E0;
    pub(crate) use crate::game::fe::fepopup::callee::C00156A60;
    pub(crate) use crate::game::fe::fepopup::callee::C001577E0;
    pub(crate) use crate::game::fe::fepopup::callee::C00157BF0;
    pub(crate) use crate::game::fe::fepopup::callee::C001589B0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158A50;
    pub(crate) use crate::game::fe::fepopup::callee::C00158AF0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158C80;
    pub(crate) use crate::game::fe::fepopup::callee::C00158D58;
    pub(crate) use crate::game::fe::fepopup::callee::C00159CD0;
    pub(crate) use crate::game::fe::fepopup::callee::C0015A2E0;

    pub(crate) use crate::game::fe::fepopup::callee::C00158910;
    pub(crate) use crate::game::fe::fepopup::callee::C00159170;

    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_GET_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_SPEND_MONEY;

    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_GET_FILE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_SET_BIT;

    pub(crate) use crate::game::fe::ovstatepause::callee::BE_REWARD_INT_COUNT_SET_BITS;

    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;

    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;

    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3 as BE_LIBRARY_GET_INTERFACE_2;

    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;

    // Camera / reward front end (shard 1..4).
    pub const BE_GET_CHAR_PROFILE_EVENT_RECORD: Callee = Callee {
        addr: 0x0015_2398,
        f: ee::s01::f_00152398,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {

    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INTERFACE;
}

// ---------------------------------------------------------------------------
// Front-end rewards.

/// `bool BERewardInt_IsRewardsFileLoaded(BERewardInt *self)` (0x156a38): false
/// while the loaded-file handle at 0x4c3e98 is -1, else whether its open count
/// at +4 is positive.
pub fn be_reward_int_is_rewards_file_loaded(c: &mut Cpu) {
    const FILE_HANDLE: u32 = 0x4c3e98;
    c.cycles += 6;
    c.g[V0][0] = 0x4c_0000;
    c.g[A1][0] = 0;
    let handle = Addr(FILE_HANDLE).lw(c);
    c.g[A0][0] = handle;
    c.g[V1][0] = u64::MAX;
    c.g[A2][0] = sx(FILE_HANDLE);
    if handle == u64::MAX {
        c.cycles += 2;
        let open = Addr(FILE_HANDLE + 4).lw(c);
        c.g[V0][0] = open;
        c.g[A1][0] = (open > 0) as u64;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    abi::ret(c);
}

/// `bool BERewardInt_IsBetterPlace(BERewardInt *self, int old, int new)`
/// (0x156a60): false when `new` is -1, true when `old` is -1, else
/// `new < old`. All compared as whole 64-bit registers.
pub fn be_reward_int_is_better_place(c: &mut Cpu) {
    c.cycles += 3;
    let new_place = abi::arg64(c, 2);
    let old_place = abi::arg64(c, 1);
    c.g[V0][0] = u64::MAX;
    if new_place == u64::MAX {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    if old_place == u64::MAX {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = ((new_place as i64) < (old_place as i64)) as u64;
    abi::ret(c);
}

/// `void BERewardInt_SetBit(BERewardInt *self, int bit, u8 *bits)` (0x1573f0):
/// set `bit` (mod 8) in the byte at `bits + bit>>3`.
pub fn be_reward_int_set_bit(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = u64::MAX;
    c.g[V1][0] = sx(abi::arg(c, 1).wrapping_add(7));
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
    c.g[A0][0] = sx(abi::arg(c, 1) >> 3);
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.g[A1][0];
    }
    c.g[A2][0] = sx(abi::arg(c, 2).wrapping_add(c.g[A0][0] as u32));
    c.g[A3][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
    c.g[V0][0] = 0x1;
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 3);
    c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
    c.g[A3][0] = sx(abi::arg(c, 1).wrapping_sub(c.g[A3][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << (c.g[A3][0] as u32 & 31));
    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
    Addr(c.g[A2][0] as u32).sb(c, c.g[V1][0]);
    abi::ret(c);
}

/// `bool BERewardInt_TestBit(BERewardInt *self, int bit, u8 *bits)` (0x157430):
/// whether `bit` (mod 8) is set in the byte at `bits + bit>>3`.
pub fn be_reward_int_test_bit(c: &mut Cpu) {
    c.cycles += 13;
    c.g[V0][0] = u64::MAX;
    c.g[V1][0] = sx(abi::arg(c, 1).wrapping_add(7));
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
    c.g[A0][0] = sx(abi::arg(c, 1) >> 3);
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.g[A1][0];
    }
    c.g[A2][0] = sx(abi::arg(c, 2).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
    c.g[V0][0] = Addr(c.g[A2][0] as u32).lb(c);
    c.g[A0][0] = sx((c.g[V1][0] as u32) << 3);
    c.g[V1][0] = sx(abi::arg(c, 1).wrapping_sub(c.g[A0][0] as u32));
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> (c.g[V1][0] as u32 & 31)) as u32);
    c.g[V0][0] = c.g[V0][0] & 0x1;
    abi::ret(c);
}

/// `int BERewardInt_GetCollectibleThreshold(BERewardInt *self, int a, int b)`
/// (0x158870): word at `a*16 + b*4 + 0x45afe8`.
pub fn be_reward_int_get_collectible_threshold(c: &mut Cpu) {
    c.cycles += 8;
    c.g[A2][0] = sx(abi::arg(c, 2) << 2);
    c.g[A1][0] = sx(abi::arg(c, 1) << 4);
    c.g[V0][0] = 0x46_0000;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = sx(0x46_0000u32.wrapping_sub(0x5018));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
    abi::ret(c);
}

/// `int BERewardInt_GetMissionThreshold(BERewardInt *self, int a, int b)`
/// (0x158890): word at `a*16 + b*4 + 0x45b018`.
pub fn be_reward_int_get_mission_threshold(c: &mut Cpu) {
    c.cycles += 8;
    c.g[A2][0] = sx(abi::arg(c, 2) << 2);
    c.g[A1][0] = sx(abi::arg(c, 1) << 4);
    c.g[V0][0] = 0x46_0000;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = sx(0x46_0000u32.wrapping_sub(0x4fe8));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
    abi::ret(c);
}

/// `NewRewards *BERewardInt_GetNewRewards(BERewardInt *self)` (0x158f20):
/// constant pointer 0x4c3ef0.
pub fn be_reward_int_get_new_rewards(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x4c_0000;
    c.g[V0][0] = sx(0x4c_0000u32.wrapping_add(0x3ef0));
    abi::ret(c);
}

/// The base of the rewards file.
pub(crate) const REWARDS_FILE: u32 = 0x004c_0000;

/// `int BERewardInt_GetTrackPeak(int index)` (0x15a398): the peak score of the
/// `index`-th track record.
pub fn be_reward_int_get_track_peak(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x64;
    c.g[V1][0] = TRACK_RECORDS as u64;
    c.mult(0, A1, A1, V0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x26b0));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x54)).lw(c);
    abi::ret(c);
}

/// `void* BERewardInt_GetRewardsFile(void)` (0x15a478): the rewards file at
/// `0x4c3e98`.
pub fn be_reward_int_get_rewards_file(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = REWARDS_FILE as u64;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e98));
    abi::ret(c);
}

/// `int BERewardInt_GetNewRewardCount(int self, int reward, int sub)`
/// (0x15a750): read a byte out of the reward record.
pub fn be_reward_int_get_new_reward_count(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0xa;
    c.mult(0, V1, A1, V0);
    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x6)).lb(c);
    abi::ret(c);
}

/// `int BERewardInt_GetNewRewardBits(int self, int reward)` (0x15a768): the
/// reward's bit word.
pub fn be_reward_int_get_new_reward_bits(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x174)).lw(c);
    abi::ret(c);
}

/// `RewardsFile* RewardsFile_ctor(RewardsFile *self)` (0x15a778): clear the
/// record, set the first word to -1, and return `self`.
pub fn rewards_file_ctor(c: &mut Cpu) {
    c.cycles += 22;
    let a0 = abi::arg64(c, 0);
    let this = Addr(a0 as u32);
    c.g[V0][0] = u64::MAX;
    this.off(0x4).sw(c, 0);
    this.sw(c, u64::MAX);
    c.g[V0][0] = a0;
    for off in [
        0x8u32, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x44,
        0x48, 0x4c,
    ] {
        this.off(off).sw(c, 0);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Callees.

pub(crate) const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;

/// `BERewardInt_GetRewardsFile` (0x15a478): the loaded rewards file.
pub(crate) const GET_REWARDS_FILE: Callee = Callee {
    addr: 0x0015_a478,
    f: ssx3_ee::s01::f_0015a478,
};

pub(crate) const REWARD_GP: u32 = 0x1e8c;

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

/// `EA_MemAllocNamed(size, name, 0, 0)` through the function table; returns
/// the allocated block.
pub(crate) fn alloc_named(
    c: &mut Cpu,
    size: u64,
    name: u32,
    ret: u32,
) -> Result<u64, call::Unwound> {
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
pub(crate) fn rewards_field(c: &mut Cpu, off: u32, ret: u32) {
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
// Reward index arithmetic.
//
// The reward arrays are addressed as `0x4a6ca8 + a*0x9b50 + b*0xf88`; a second
// index adds `+ k*0x9b50`. `TestReward`/`SetReward` then index a bit array at
// that base + a per-type offset.

pub(crate) const REWARD_BASE: u32 = 0x004a_6ca8;

pub(crate) const REWARD_STRIDE_A: u32 = 0x9b50;

pub(crate) const REWARD_STRIDE_B: u32 = 0xf88;

/// `int BERewardInt_IsXUnlocked(...)` (0x157d60 family): test the per-type
/// unlock bit at `REWARD_BASE + a1*0x9b50 + a2*0xf88 + v1 + extra`.
pub(crate) fn is_unlocked(c: &mut Cpu, extra: u32, rider_item: bool, ret: u32) {
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
pub(crate) fn count_unlocked(c: &mut Cpu, field: u32, extra: u32, ret0: u32, ret1: u32) {
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

/// `void BERewardInt_LoadRewardsFile(BERewardInt* r)` (0x156a10): (re)load the
/// rewards database (`D_004C3E98`) through `RewardsFile_Load(db, 1)`.
///
/// Kept out of `OVERRIDES`: `RewardsFile_Load(db, 1)` always takes the
/// bolt-file branch (the `a1 == 0` branch is unreachable from this wrapper),
/// which scans the 256-slot equip table and then calls `BoltFile_LoadSlot` →
/// `BoltFile_EvictBytes`; on a zeroed test machine that eviction walk reads a
/// garbage byte count and loops past the cycle budget, so there is no
/// fixture that both covers the wrapper and terminates.
pub fn be_reward_int_load_rewards_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x004c_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x1;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x3e98));
        callee::REWARDS_FILE_LOAD.call(c, 0x0015_6a28)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The per-player (`a1`, stride 0x9b50) per-slot (`a2`, stride 0xf88) reward
/// record inside `D_004A6CA8`; returns its address.
pub(crate) fn reward_record(c: &mut Cpu) -> u32 {
    c.cycles += 10;
    c.g[V0][0] = 0x0f88;
    c.g[V1][0] = 0x9b50;
    c.mult(0, A2, A2, V0);
    c.mult(0, A1, A1, V1);
    c.g[V0][0] = sx(MESSAGE_MAP.wrapping_add(0x6ca8)); // &D_004A6CA8
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(MESSAGE_MAP.wrapping_add(0x6ca8)));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[A1][0] as u32
}

/// `int BERewardInt_GetAudioSetting(void *this, int player, int slot)`
/// (0x158700): the record's `+0xf80` audio setting.
pub fn be_reward_int_get_audio_setting(c: &mut Cpu) {
    let rec = reward_record(c);
    c.g[V0][0] = Addr(rec.wrapping_add(0xf80)).lw(c);
    abi::ret(c);
}

/// `u64 BERewardInt_GetUnlockedSongs(void *this, int player, int slot)`
/// (0x158728): the record's `+0xf70` unlocked-song mask.
pub fn be_reward_int_get_unlocked_songs(c: &mut Cpu) {
    let rec = reward_record(c);
    c.g[V0][0] = Addr(rec.wrapping_add(0xf70)).ld(c);
    abi::ret(c);
}

/// `u64 BERewardInt_GetEnabledSongs(void *this, int player, int slot)`
/// (0x158750): the record's `+0xf78` enabled-song mask.
pub fn be_reward_int_get_enabled_songs(c: &mut Cpu) {
    let rec = reward_record(c);
    c.g[V0][0] = Addr(rec.wrapping_add(0xf78)).ld(c);
    abi::ret(c);
}

/// `void BERewardInt_SetAudioSetting(void *this, int player, int slot,
/// int value)` (0x1587f8).
pub fn be_reward_int_set_audio_setting(c: &mut Cpu) {
    let rec = reward_record(c);
    Addr(rec.wrapping_add(0xf80)).sw(c, c.g[A3][0]);
    abi::ret(c);
}

/// `void BERewardInt_SetUnlockedSongs(void *this, int player, int slot,
/// u64 mask)` (0x158820).
pub fn be_reward_int_set_unlocked_songs(c: &mut Cpu) {
    let rec = reward_record(c);
    Addr(rec.wrapping_add(0xf70)).sd(c, c.g[A3][0]);
    abi::ret(c);
}

/// `void BERewardInt_SetEnabledSongs(void *this, int player, int slot,
/// u64 mask)` (0x158848).
pub fn be_reward_int_set_enabled_songs(c: &mut Cpu) {
    let rec = reward_record(c);
    Addr(rec.wrapping_add(0xf78)).sd(c, c.g[A3][0]);
    abi::ret(c);
}

/// `bool BERewardInt_TestGrantedBit(BERewardInt* self, int bit,
/// uint8_t* granted)` (0x15a320): bit `bit` of the byte array, with the
/// two's-complement rounding for negative indices.
pub(crate) fn test_bit(c: &mut Cpu, index: u64, base: u64) {
    c.g[V0][0] = u64::MAX;
    c.g[V1][0] = sx((index as u32).wrapping_add(7));
    c.g[V0][0] = ((u64::MAX as i64) < (index as i64)) as u64;
    c.g[A0][0] = sx((index as u32) >> 3);
    if c.g[V0][0] != 0 {
        c.g[V1][0] = index;
    }
    c.g[A2][0] = sx((base as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = sx((((c.g[V1][0] as u32) as i32) >> 3) as u32);
    c.g[V0][0] = Addr(c.g[A2][0] as u32).lbu(c);
    c.g[A0][0] = sx((c.g[V1][0] as u32) << 3);
    c.g[V1][0] = sx((index as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> ((c.g[V1][0] as u32) & 31)) as u32);
    c.g[V0][0] &= 1;
}

/// `bool BERewardInt_TestGrantedBit(BERewardInt* self, int bit,
/// const uint8_t* bits)` (0x15a320).
pub fn reward_test_granted_bit(c: &mut Cpu) {
    c.cycles += 13;
    let (index, base) = (c.g[A1][0], c.g[A2][0]);
    test_bit(c, index, base);
    crate::abi::ret(c);
}

/// `bool BERewardInt_NewRewardsTestBit(BERewardInt* self, int bit,
/// const uint8_t* bits)` (0x15a5f0): identical bit test on the new-reward
/// bitset.
pub fn reward_new_rewards_test_bit(c: &mut Cpu) {
    c.cycles += 13;
    let (index, base) = (c.g[A1][0], c.g[A2][0]);
    test_bit(c, index, base);
    crate::abi::ret(c);
}

/// `void ber_reward_set_bit(bits, int bit)` — the shared body of the two
/// reward-flag setters: `bits[bit >> 3] |= 1 << (bit & 7)`.
pub(crate) fn reward_set_bit(c: &mut Cpu, bits: u64, bit: u64) {
    c.cycles += 15;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.g[V1][0] = sx((bit as u32).wrapping_add(7));
    c.g[V0][0] = ((c.g[V0][0] as i64) < (bit as i64)) as u64;
    c.g[A0][0] = sx((bit as u32) >> 3);
    if c.g[V0][0] != 0 {
        c.g[V1][0] = bit;
    }
    c.g[A2][0] = sx((bits as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[A3][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
    c.g[V0][0] = 1;
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 3);
    c.g[V1][0] = Addr(c.g[A2][0] as u32).lbu(c);
    c.g[A3][0] = sx((bit as u32).wrapping_sub(c.g[A3][0] as u32));
    let shift = (c.g[A3][0] as u32) & 31;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << shift);
    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
    let value = c.g[V1][0];
    c.sb(c.g[A2][0] as u32, value);
    abi::ret(c);
}

/// `void BERewardInt_SetGrantedBit(this, int bit, uchar* bits)` (0x15a358):
/// set the granted bit.
pub fn be_reward_int_set_granted_bit(c: &mut Cpu) {
    let bits = c.g[A2][0];
    let bit = c.g[A1][0];
    reward_set_bit(c, bits, bit);
}

/// `void BERewardInt_NewRewardsSetBit(this, int bit, uchar* bits)` (0x15a5b0):
/// set the "new reward" bit (the generated body is the same as the granted
/// setter, inlined).
pub fn be_reward_int_new_rewards_set_bit(c: &mut Cpu) {
    let bits = c.g[A2][0];
    let bit = c.g[A1][0];
    reward_set_bit(c, bits, bit);
}

/// `int BERewardInt_CountEnabledSongs(BERewardInt* r, int a1, int a2)`
/// (0x1587b8): count the set bits of the 0x40-byte song mask of reward
/// `a1 * 0x9b50 + a2 * 0xf88 + 0xf78` in `D_004A6CA8`.
pub fn be_reward_int_count_enabled_songs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[V0][0] = 0xf88;
        c.g[A3][0] = 0x9b50;
        // `mult a2, a2, v0` and `mult v1, a1, a3`.
        c.mult(0, 6, 6, 2);
        c.mult(0, 3, 5, 7);
        c.g[V0][0] = 0x4a_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(crate::abi::r32(c, V0).wrapping_add(0x6ca8));
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x40;
        c.g[A2][0] = sx(crate::abi::r32(c, A2).wrapping_add(crate::abi::r32(c, V0)));
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_add(crate::abi::r32(c, A2)));
        c.g[A2][0] = sx(crate::abi::r32(c, V1).wrapping_add(0xf78));
        callee::BE_REWARD_INT_COUNT_SET_BITS.call(c, 0x0015_87ec)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rewards file: destructor.

/// `void RewardsFile_dtor(RewardsFile* self, int flags)` (0x15a7d0): unload
/// the file's reward table, then `operator delete` it when `flags & 1`.
pub fn rewards_file_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::REWARDS_FILE_UNLOAD.call(c, 0x0015_a7ec)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE_2, 0x0015_a800)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// BE int rewards / camera.

/// `int BERewardInt_GetEventRecordValue(profile, event, a2, a3, t0, t1)`
/// (0x158960): look the profile's event record up in
/// `D_004A6CA8[...]` and return its second word.
pub fn be_reward_int_get_event_record_value(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        c.g[V0][0] = 0xf88;
        c.g[V1][0] = 0x9b50;
        c.mult(0, A0, A2, V0); // a0 = a2 * sizeof(sProfileSlot)
        c.mult(0, T1, A1, V1); // t1 = a1 * sizeof(record)
        c.g[V0][0] = 0x004a_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(0x004a_0000u32.wrapping_add(0x6ca8)); // D_004A6CA8
        c.g[A1][0] = c.g[T0][0];
        c.g[A2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        BE_GET_CHAR_PROFILE_EVENT_RECORD.call(c, 0x0015_8994)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE reward / score wrappers (rider index -> profile + character).

/// `int BERewardInt_IsRivalRewardPending(BERewardInt* self, int rider,
/// int peak)` (0x157b08): resolve the rider's profile and character, then
/// test the peak's rival-reward bit.
pub fn be_reward_int_is_rival_reward_pending(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x50);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[A2][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0015_7b34);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a0e0, 0x0015_7b34)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0015_7b40);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a080, 0x0015_7b40)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x0015_7b54);
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, 0x0015_7b70, 0x0015_7b54)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x50);
        Ok(())
    })
}

/// `void BERewardInt_GrantMedalReward(self, a1, a2)` (0x159170): compute the
/// medal reward id and grant it unless it was already granted.
pub fn be_reward_int_grant_medal_reward(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x50));
        c.sq(16, sp_2(c).wrapping_add(0x40));
        c.sq(17, sp_2(c).wrapping_add(0x30));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, sp_2(c).wrapping_add(0x20));
        c.g[S1][0] = c.g[A1][0];
        c.sq(19, sp_2(c).wrapping_add(0x10));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S2][0] = c.g[A2][0];
        callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID.call(c, 0x0015_9198)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S3][0];
        callee::BE_REWARD_INT_IS_REWARD_GRANTED.call(c, 0x0015_91b0)?;
        if c.g[V0][0] != 0 {
            c.lq(16, sp_2(c).wrapping_add(0x40));
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S3][0];
            callee::BE_REWARD_INT_GRANT_REWARD.call(c, 0x0015_91cc)?;
            c.lq(16, sp_2(c).wrapping_add(0x40));
        }
        c.cycles += 6;
        c.lq(17, sp_2(c).wrapping_add(0x30));
        c.lq(18, sp_2(c).wrapping_add(0x20));
        c.lq(19, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `bool BERewardInt_AreAllPeaksComplete(self, profile, c)` (0x001578a0):
/// true when every peak of every (up to three) rider is complete.
pub fn be_reward_int_are_all_peaks_complete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        c.g[A0][0] = c.g[S3][0];
        loop {
            c.cycles += 6;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S0][0];
            callee::BEREWARD_INT_IS_PEAK_COMPLETE.call(c, 0x0015_78e0)?;
            let complete = c.g[V0][0] != 0;
            c.g[S0][0] = sx(word(c, S0).wrapping_add(1));
            if !complete {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 3i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S3][0];
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 1;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0015_78d0);
            }
        }
        c.cycles += 7;
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
// Back-end rewards and bolt (grind-rail) lookups.

/// `int BERewardInt_IsRivalRewardPendingFor(a, b, _, peak)` (0x157b70): bit
/// 12 (peak 1) or bit 13 (peak 2) of the 64-bit flag word at
/// `0x4a6ca8 + 0xf88*b + 0x9b50*a + 0x278`; 0 for any other peak.
pub fn berewardint_is_rival_reward_pending_for(c: &mut Cpu) {
    let peak = abi::arg(c, 3);
    c.cycles += 3;
    c.g[V0][0] = 1;
    if peak == 1 {
        c.cycles += 13;
        c.g[V1][0] = 0xf88;
        let bit = rival_flag_bit(c, 20);
        c.g[V0][0] = bit;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let second = peak == 2;
    c.g[V1][0] = 0xf88;
    if second {
        c.cycles += 12;
        let bit = rival_flag_bit(c, 19);
        c.g[V0][0] = bit;
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// The shared tail: `((*(u64*)(0x4a6ca8 + 0xf88*b + 0x9b50*a + 0x278) << shift)
/// >> 32) & 1`, with the multiplications' LO/HI and scratch registers as the
/// original leaves them.
pub(crate) fn rival_flag_bit(c: &mut Cpu, shift: u32) -> u64 {
    c.g[A0][0] = 0x9b50;
    mul32(c, V1, A2, V1); // V1 = b * 0xf88
    mul32(c, A0, A1, A0); // A0 = a * 0x9b50
    c.g[V0][0] = 0x4a0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let word = lw_at(c, p(c.g[V1][0] as u32, 0x278));
    let shifted = (word << shift) as i64 >> 32;
    (shifted as u64) & 1
}

/// `int BERewardInt_HasPointsTier(index, a, b, medal)` (0x158af0): 1 when the
/// rider's total earned points reach `D_0045B048[medal]`, else -1.
pub fn berewardint_has_points_tier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        callee::BE_LIBRARY_GET_INSTANCE_2.call(c, 0x0015_8b14)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xb;
        callee::BE_LIBRARY_GET_INTERFACE_2.call(c, 0x0015_8b20)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A0][0] = c.g[V0][0];
        callee::BE_ECON_INT_GET_TOTAL_EARNED.call(c, 0x0015_8b30)?;
        c.g[V1][0] = 0x460000;
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4fb8));
        c.g[A1][0] = 1;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = lw_at(c, c.g[S0][0] as u32);
        frame.restore(c, 16, 0x30);
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        frame.restore(c, 17, 0x20);
        if c.g[V0][0] == 0 {
            c.g[A0][0] = c.g[A1][0];
        }
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[A0][0];
        frame.close(c);
        Ok(())
    })
}

/// `BERewardInt* BERewardInt_NewRewardsCtor(BERewardInt* self)` (0x15a488):
/// clear the ten new-reward entries and point the string fields at the
/// in-object buffers.
pub fn be_reward_int_new_rewards_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        open(c, 0x20);
        let sp0 = spv(c);
        c.g[V1][0] = 9;
        c.sq(16, at(sp0, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        let this = c.g[S0][0];
        c.g[V0][0] = sx(at(this as u32, 0x198));
        loop {
            c.cycles += 7;
            let slot = c.g[V0][0] as u32;
            c.sw(slot, 0);
            c.g[V1][0] = sx(at(c.g[V1][0] as u32, 0xffff_ffff));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(at(slot, 0xffff_fffc));
            if !more {
                c.cycles += 21;
                c.g[V0][0] = sx(at(this as u32, 0x146));
                c.g[V1][0] = sx(at(this as u32, 0x170));
                c.g[A1][0] = sx(at(this as u32, 0x147));
                c.g[A2][0] = sx(at(this as u32, 0x14d));
                c.g[A3][0] = sx(at(this as u32, 0x15c));
                c.g[T0][0] = sx(at(this as u32, 0x169));
                c.g[T1][0] = sx(at(this as u32, 0x16d));
                c.sw(at(this as u32, 0x174), c.g[V0][0]);
                c.sw(at(this as u32, 0x17c), c.g[V1][0]);
                c.g[A0][0] = c.g[S0][0];
                c.sw(at(this as u32, 0x180), c.g[A1][0]);
                c.sw(at(this as u32, 0x184), c.g[A2][0]);
                c.sw(at(this as u32, 0x188), c.g[A3][0]);
                c.sw(at(this as u32, 0x190), c.g[T0][0]);
                c.sw(at(this as u32, 0x194), c.g[T1][0]);
                callee::CLEAR_NEW_REWARDS.call(c, 0x0015_a4fc)?;
                c.g[V0][0] = c.g[S0][0];
                c.g[RA][0] = c.ld(sp0);
                c.lq(16, at(sp0, 0x10));
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at(sp0, 0x20));
                c.pc = ra;
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0015_a4a0);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// BE reward goals.

/// `int BERewardInt_IsPeakComplete(BERewardInt* self, int a, int b, int c)`
/// (0x157a78): the peak is complete when its four goals are.
pub fn be_reward_int_is_peak_complete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A0][0];
        c.g[A0][0] = c.g[S4][0];
        loop {
            c.cycles += 7;
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S1][0];
            c.g[T0][0] = c.g[S0][0];
            callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE.call(c, 0x0015_7ac4)?;
            let done = c.g[V0][0] != 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            if !done {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 4i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S4][0];
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 1;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0015_7ab0);
            }
        }
        c.cycles += 8;
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

/// `int BERewardInt_GetNextPointsThreshold(BEReward* self, int a, int tier)`
/// (0x158be0): return the points threshold of the first tier (up to three)
/// that the reward has points in and that is better than `a`.
pub fn be_reward_int_get_next_points_threshold(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4fb8));
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A1][0];
        loop {
            c.cycles += 11;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S0][0];
            call_ptr(c, 0x0015_8af0, 0x0015_8c2c)?; // HasPointsTier
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0xffff_ffff_ffff_ffff;
            call_ptr(c, 0x0015_6a60, 0x0015_8c3c)?; // IsBetterPlace
            let better = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            if !better {
                c.cycles += 2;
                c.g[V0][0] = lw(c, c.g[S2][0] as u32);
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 3) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0015_8c18);
                }
                continue;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
            break;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// BE profile/stat helpers.

/// `int BERewardInt_GetCollectibleTier(iface, rider, event, kind)` (0x1589b0):
/// compare the collected count with the four tier thresholds for `kind`.
pub fn be_reward_int_get_collectible_tier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, table::BE_GET_INSTANCE, 0x0015_89d4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xa;
        call_ptr(c, table::BE_GET_INTERFACE, 0x0015_89e0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        callee::BE_COUNT_COLLECTED_ON_PEAK.call(c, 0x0015_89f4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 4);
        c.g[V0][0] = 0x0045_afe8;
        c.g[V1][0] = 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        'tier: loop {
            c.cycles += 4;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[V1][0];
                break 'tier;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 4) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0015_8a10);
                }
                continue 'tier;
            }
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
            break 'tier;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BERewardInt_GetMissionTier(iface, rider, event, kind)` (0x158a50):
/// same tier lookup against the completed-mission thresholds.
pub fn be_reward_int_get_mission_tier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, table::BE_GET_INSTANCE, 0x0015_8a74)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xa;
        call_ptr(c, table::BE_GET_INTERFACE, 0x0015_8a80)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        callee::BE_COUNT_COMPLETED_ON_PEAK.call(c, 0x0015_8a94)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 4);
        c.g[V0][0] = 0x0045_b018;
        c.g[V1][0] = 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        'tier: loop {
            c.cycles += 4;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[V1][0];
                break 'tier;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 4) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0015_8ab0);
                }
                continue 'tier;
            }
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
            break 'tier;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BERewardInt_FindRequirementTier(int ofs, int a, int b)` (0x15a3b8):
/// scan the `[3][4][5]` requirement table at 0x45aad8 (two bytes per entry,
/// 0xa apart per middle index) for the first entry whose `a` matches — and
/// whose `b` matches, or whose `a` is one of 6..10 — returning the middle
/// index, else -1.
pub fn be_reward_int_find_requirement_tier(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0x460000;
    c.g[T1][0] = 0;
    c.g[T8][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5528));
    c.g[T7][0] = 0xa;
    c.g[T6][0] = 6;
    c.g[T5][0] = 7;
    c.g[T4][0] = 8;
    c.g[T3][0] = 9;
    c.g[T2][0] = 0xb;
    c.g[V0][0] = 0x28;
    loop {
        c.cycles += 2;
        c.g[T0][0] = 0;
        c.mult(0, 4, 9, 2);
        loop {
            c.cycles += 2;
            c.g[A3][0] = 0;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T8][0] as u32));
            loop {
                c.cycles += 3;
                let mut hit = Addr(c.g[V1][0] as u32).lb(c) == c.g[A1][0];
                if hit {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(1).lb(c);
                    hit = c.g[V0][0] == c.g[A2][0];
                }
                if !hit {
                    c.cycles += 2;
                    hit = c.g[A1][0] == c.g[T6][0];
                }
                if !hit {
                    c.cycles += 2;
                    hit = c.g[A1][0] == c.g[T5][0];
                }
                if !hit {
                    c.cycles += 2;
                    hit = c.g[A1][0] == c.g[T4][0];
                }
                if !hit {
                    c.cycles += 2;
                    hit = c.g[A1][0] == c.g[T3][0];
                }
                if !hit {
                    c.cycles += 2;
                    hit = c.g[A1][0] == c.g[T7][0];
                }
                if !hit {
                    c.cycles += 2;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                    hit = c.g[A1][0] == c.g[T2][0];
                }
                if hit {
                    c.cycles += 2;
                    let ra = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[T0][0];
                    c.pc = ra;
                    return;
                }
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A3][0] as i64) < 5i64) as u64;
                let again = c.g[V0][0] != 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(2));
                if again {
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_a3f0);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 4;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[T0][0] as i64) < 4i64) as u64;
            let again = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa));
            if again {
                if event_due(c) {
                    deliver_event_here(c, 0x0015_a3e8);
                }
                break;
            }
            c.cycles += 4;
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[T1][0] as i64) < 3i64) as u64;
            let again = c.g[V0][0] != 0;
            c.g[V0][0] = 0x28;
            if !again {
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.g[V0][0] = u64::MAX;
                c.pc = ra;
                return;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0015_a3e0);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// `BERewardInt_IsPeakFullyCompleted` (0x001577e0).
//
// `int BERewardInt::IsPeakFullyCompleted(self, peak, medal, index)`: true when
// all three peak medals are complete and the record levels add up to 24.

pub fn be_reward_int_is_peak_fully_completed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = 0u64;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A2][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S3][0] = c.g[A0][0];
        c.g[A0][0] = c.g[S3][0];
        'l_157810: loop {
            'b_157884: {
                'b_157830: {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00157820);
                    c.g[A3][0] = c.g[S0][0];
                    call_ptr(c, 0x00157920, 0x00157820)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        break 'b_157830;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_157884;
                        } else {
                            break 'b_157830;
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < 3i64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.g[A0][0] = c.g[S3][0];
                    if crate::events::event_due(c) {
                        crate::events::deliver_event_here(c, 0x00157810);
                    }
                    continue 'l_157810;
                } else {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x00157844);
                    c.g[S1][0] = 0u64;
                    call_ptr(c, 0x0014dc80, 0x00157844)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00157850);
                    c.g[A1][0] = 0x8;
                    call_ptr(c, 0x0014dd58, 0x00157850)?;
                    c.g[S0][0] = 0u64;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    'l_157860: loop {
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x00157868);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x001552b0, 0x00157868)?;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V0][0] = ((c.g[S0][0] as i64) < 8i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = c.g[S0][0];
                        if t {
                            if crate::events::event_due(c) {
                                crate::events::deliver_event_here(c, 0x00157860);
                            }
                            continue 'l_157860;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 24i64) as u64;
                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                            break 'b_157884;
                        }
                    }
                }
            }
            c.cycles += 7;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
            c.pc = t;
            return Ok(());
        }
    })
}

// ---------------------------------------------------------------------------
// Reward thresholds.

/// The shared body of the two reward threshold helpers: pick the tier, then
/// find the smallest table value that is below the current best.
pub(crate) fn reward_threshold(c: &mut Cpu, tier_fn: u32, table: u32, ret: u32, head: u32) -> Flow {
    c.cycles += 22;
    let frame = Frame::open(c, 0x90);
    frame.save(c, S4, 0x40);
    frame.save(c, S5, 0x30);
    c.g[S4][0] = c.g[A3][0];
    c.g[S5][0] = c.g[A0][0];
    frame.save(c, S0, 0x80);
    frame.save(c, S1, 0x70);
    frame.save(c, S2, 0x60);
    frame.save(c, S3, 0x50);
    c.g[S2][0] = 0;
    frame.save(c, S6, 0x20);
    frame.save(c, S7, 0x10);
    frame.save_ra(c, 0);
    c.g[S3][0] = 0;
    call_ptr(c, tier_fn, ret)?;
    c.g[S7][0] = c.g[V0][0];
    c.g[V1][0] = sx(r32(c, S4) << 4);
    c.g[V0][0] = 0x460000;
    c.g[S6][0] = sx(r32(c, V0).wrapping_sub(table));
    c.g[S1][0] = sx(p(r32(c, V1), r32(c, S6)));
    loop {
        c.cycles += 6;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S7][0];
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x0015_6a60, ret + 0x28)?;
        if c.g[V0][0] == 0 {
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
        } else {
            c.cycles += 2;
            let empty = c.g[S2][0] == 0;
            c.g[V1][0] = sx(r32(c, S4) << 4);
            if !empty {
                c.cycles += 4;
                c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = sx(p(r32(c, S3), r32(c, V1)));
                    c.g[V0][0] = sx(p(r32(c, V0), r32(c, S6)));
                    c.g[S2][0] = Addr(r32(c, V0)).lw(c);
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                }
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx(p(r32(c, S3), r32(c, V1)));
                c.g[V0][0] = sx(p(r32(c, V0), r32(c, S6)));
                c.g[S2][0] = Addr(r32(c, V0)).lw(c);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            }
        }
        c.cycles += 4;
        c.g[S3][0] = sx(r32(c, S3).wrapping_add(4));
        c.g[V0][0] = ((c.g[S0][0] as i64) < 4i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
    c.cycles += 12;
    c.g[V0][0] = c.g[S2][0];
    frame.restore(c, S0, 0x80);
    frame.restore(c, S1, 0x70);
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore(c, S4, 0x40);
    frame.restore(c, S5, 0x30);
    frame.restore(c, S6, 0x20);
    frame.restore(c, S7, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `int BERewardInt_GetNextCollectibleThreshold(self, a, b, kind)` (0x158c80).
pub fn berewardint_get_next_collectible_threshold(c: &mut Cpu) {
    guest(c, |c| {
        reward_threshold(c, 0x0015_89b0, 0x5018, 0x0015_8cc0, 0x0015_8cd8)
    })
}

/// `int BERewardInt_GetNextMissionThreshold(self, a, b, kind)` (0x158d58).
pub fn berewardint_get_next_mission_threshold(c: &mut Cpu) {
    guest(c, |c| {
        reward_threshold(c, 0x0015_8a50, 0x4fe8, 0x0015_8d98, 0x0015_8db0)
    })
}

// ---------------------------------------------------------------------------
// BE rewards.

/// `void BERewardInt_UnlockVideo(BERewardInt* self, int profile, int c,
/// int idx, int award)` (0x158470): mark video reward `idx` as seen in the
/// profile's video bit table and, when awarded, add its point value (ten
/// times the reward's score) to the player's money.
pub fn bereward_int_unlock_video(c: &mut Cpu) {
    guest(c, |c| {
        bereward_unlock(
            c,
            0x0015_8470,
            0x24,
            0xf52,
            0x0015_84a8,
            0x0015_84b4,
            0x0015_84e8,
            0x0015_84f8,
            0x0015_8524,
            0x0015_8534,
        )
    })
}

/// `void BERewardInt_UnlockRiderItem(...)` (0x158618): as 0x158470 for rider
/// items; `idx` is offset by ten and the bit table is at `+0x28`.
pub fn bereward_int_unlock_rider_item(c: &mut Cpu) {
    guest(c, |c| {
        bereward_unlock(
            c,
            0x0015_8618,
            0x28,
            0xf57,
            0x0015_8650,
            0x0015_865c,
            0x0015_8690,
            0x0015_86a0,
            0x0015_86cc,
            0x0015_86dc,
        )
    })
}

/// The shared body of the two unlock helpers. `idx` is `a3` (`- 10` for the
/// rider-item variant), `table` the rewards-file field holding the bit table
/// and `bit_off` the byte offset of the bit inside a 0x9b50/0xf88 entry.
#[allow(clippy::too_many_arguments)]
pub(crate) fn bereward_unlock(
    c: &mut Cpu,
    _entry: u32,
    table: u32,
    bit_off: u32,
    ret_inst: u32,
    ret_iface: u32,
    ret_setbit: u32,
    ret_file: u32,
    ret_spend: u32,
    ret_money: u32,
) -> Flow {
    c.cycles += 32;
    let frame = Frame::open(c, 0x70);
    frame.save(c, S0, 0x60);
    frame.save(c, S1, 0x50);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S2, 0x40);
    c.g[S1][0] = c.g[T0][0];
    frame.save(c, S3, 0x30);
    c.g[S2][0] = c.g[A1][0];
    frame.save(c, S4, 0x20);
    c.g[S3][0] = c.g[A2][0];
    frame.save(c, S5, 0x10);
    frame.save_ra(c, 0);
    if bit_off == 0xf57 {
        c.g[S4][0] = sx(r(c, A3).wrapping_sub(0xa));
    } else {
        c.g[S4][0] = c.g[A3][0];
    }

    callee::BE_LIBRARY_INSTANCE.call(c, ret_inst)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xb;
    callee::BE_LIBRARY_INTERFACE.call(c, ret_iface)?;

    // econ = interface; entry = D_004A6CA8 + profile * 0x9b50 + c * 0xf88.
    c.g[V1][0] = 0xf88;
    c.g[A2][0] = 0x9b50;
    c.mult(0, V1, S3, V1);
    c.g[S5][0] = c.g[V0][0];
    c.mult(0, A2, S2, A2);
    c.g[V0][0] = 0x4a0000;
    c.g[V0][0] = sx(r(c, V0).wrapping_add(0x6ca8));
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S4][0];
    c.g[V1][0] = sx(p(r(c, V1), r(c, V0)));
    c.g[A2][0] = sx(p(r(c, A2), r(c, V1)));
    c.g[A2][0] = sx(p(r(c, A2), bit_off));
    callee::BE_REWARD_SET_BIT.call(c, ret_setbit)?;

    let no_award = c.g[S1][0] == 0;
    c.g[A0][0] = c.g[S5][0];
    if !no_award {
        c.cycles += 8;
        callee::BE_REWARD_GET_FILE.call(c, ret_file)?;
        c.g[V1][0] = Addr(r(c, V0)).off(table).lw(c);
        c.g[V0][0] = sx(r(c, S4) << 4);
        c.g[V1][0] = sx(p(r(c, V1), r(c, V0)));
        c.g[A3][0] = Addr(r(c, V1)).off(0xc).lh(c);
        let not_scored = (c.g[A3][0] as i64) <= 0;
        c.g[V0][0] = 0xa;
        if !not_scored {
            c.cycles += 1;
            c.mult(0, A3, A3, V0);
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S3][0];
        callee::BE_ECON_SPEND_MONEY.call(c, ret_spend)?;
        c.g[A0][0] = c.g[S5][0];
        return bereward_tail(c, frame);
    }

    c.cycles += 12;
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S3][0];
    callee::BE_ECON_GET_MONEY.call(c, ret_money)?;
    bereward_tail(c, frame)
}

/// The `lq s0-s5; ld ra; addiu sp; jr ra` tail.
pub(crate) fn bereward_tail(c: &mut Cpu, frame: Frame) -> Flow {
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

// ---------------------------------------------------------------------------
// BE medals.

/// `int BERewardInt_GetMedalRewardId(int self, int a1, int kind, int tier)`
/// (0x159080): the reward id for a medal: `5 + tier*3 + kind` for tiers 0-3
/// and kinds 0-2, -1 otherwise.
pub fn bereward_int_get_medal_reward_id(c: &mut Cpu) {
    // A leaf with no frame; tier = t0, kind = a3.
    'outer: {
        'b_159160: {
            'b_15915c: {
                'b_159138: {
                    'b_159110: {
                        'b_1590e8: {
                            c.cycles += 3;
                            c.g[V1][0] = 1;
                            let tier_one = c.g[T0][0] == c.g[V1][0];
                            c.g[A0][0] = u64::MAX;
                            if tier_one {
                                break 'b_1590e8;
                            }
                            'b_1590c0: {
                                'b_1590a8: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[T0][0] as i64) < 2) as u64;
                                    let ge_two = c.g[V0][0] == 0;
                                    c.g[V0][0] = 2;
                                    if ge_two {
                                        break 'b_1590a8;
                                    }
                                    c.cycles += 2;
                                    let tier_zero = c.g[T0][0] == 0;
                                    c.g[V0][0] = c.g[A0][0];
                                    if tier_zero {
                                        break 'b_1590c0;
                                    }
                                    c.cycles += 2;
                                    break 'outer;
                                }
                                c.cycles += 2;
                                let tier_two = c.g[T0][0] == c.g[V0][0];
                                c.g[V0][0] = 3;
                                if tier_two {
                                    break 'b_159110;
                                }
                                c.cycles += 2;
                                let tier_three = c.g[T0][0] == c.g[V0][0];
                                c.g[V0][0] = c.g[A0][0];
                                if tier_three {
                                    break 'b_159138;
                                }
                                c.cycles += 2;
                                break 'outer;
                            }

                            // tier 0
                            c.cycles += 2;
                            if c.g[A3][0] != 0 {
                                c.cycles += 2;
                                let not_one = c.g[A3][0] != c.g[V1][0];
                                c.g[V1][0] = c.g[A3][0] ^ 2;
                                if not_one {
                                    c.cycles += 2;
                                    c.g[V0][0] = 7;
                                    break 'b_15915c;
                                }
                                c.cycles += 2;
                                c.g[A0][0] = 6;
                                break 'b_159160;
                            }
                            c.cycles += 2;
                            c.g[A0][0] = 5;
                            break 'b_159160;
                        }

                        // tier 1
                        c.cycles += 2;
                        if c.g[A3][0] != 0 {
                            c.cycles += 2;
                            let not_one = c.g[A3][0] != c.g[V1][0];
                            c.g[V1][0] = c.g[A3][0] ^ 2;
                            if not_one {
                                c.cycles += 2;
                                c.g[V0][0] = 0xa;
                                break 'b_15915c;
                            }
                            c.cycles += 2;
                            c.g[A0][0] = 9;
                            break 'b_159160;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = 8;
                        break 'b_159160;
                    }

                    // tier 2
                    c.cycles += 2;
                    if c.g[A3][0] != 0 {
                        c.cycles += 2;
                        let not_one = c.g[A3][0] != c.g[V1][0];
                        c.g[V1][0] = c.g[A3][0] ^ 2;
                        if not_one {
                            c.cycles += 2;
                            c.g[V0][0] = 0xd;
                            break 'b_15915c;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = 0xc;
                        break 'b_159160;
                    }
                    c.cycles += 2;
                    c.g[A0][0] = 0xb;
                    break 'b_159160;
                }

                // tier 3
                c.cycles += 2;
                if c.g[A3][0] != 0 {
                    c.cycles += 2;
                    let not_one = c.g[A3][0] != c.g[V1][0];
                    c.g[V1][0] = c.g[A3][0] ^ 2;
                    if not_one {
                        c.cycles += 1;
                        c.g[V0][0] = 0x10;
                        break 'b_15915c;
                    }
                    c.cycles += 2;
                    c.g[A0][0] = 0xf;
                    break 'b_159160;
                }
                c.cycles += 2;
                c.g[A0][0] = 0xe;
                break 'b_159160;
            }
            c.cycles += 1;
            if c.g[V1][0] == 0 {
                c.g[A0][0] = c.g[V0][0];
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
    }
    c.cycles += 2;
    let ra = r(c, RA);
    c.pc = ra;
}

/// `void BERewardInt_GrantRivalRewards(void* self, int profile, int c, int
/// bit, int mode)` (0x158f60): set a rival-reward bit in the character
/// record's flags (`D_004A6CA8 + profile*0x9b50 + c*0xf88 + 0x278`, bit 13
/// for mode 2, bit 12 for mode 1) if it is clear, then post the reward
/// message (`q = D_004C3EF0`, row `c*3+2` or `c*3+1`).
pub fn be_reward_int_grant_rival_rewards(c: &mut Cpu) {
    'b_159068: {
        'b_159008: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.g[V0][0] = 0x4c0000;
            c.sq(19, c.pa(SP, 0x10));
            c.sq(16, c.pa(SP, 0x40));
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3ef0));
            c.sq(17, c.pa(SP, 0x30));
            c.g[S0][0] = c.g[A2][0];
            c.sq(18, c.pa(SP, 0x20));
            c.g[S1][0] = c.g[A1][0];
            c.std(SP, 0, c.g[RA][0]);
            c.g[V0][0] = 0x1;
            let t = c.g[T0][0] == c.g[V0][0];
            c.g[S2][0] = c.g[A3][0];
            if t {
                break 'b_159008;
            } else {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T0][0] as i64) < 2i64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.lq(16, c.pa(SP, 0x40));
                    break 'b_159068;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    if c.g[T0][0] != c.g[V0][0] {
                        c.lq(16, c.pa(SP, 0x40));
                        break 'b_159068;
                    } else {
                        c.cycles += 14;
                        c.g[V1][0] = 0xf88;
                        c.g[A0][0] = 0x9b50;
                        c.mult(0, 4, 17, 4);
                        c.mult(0, 3, 16, 3);
                        c.g[V0][0] = 0x4a0000;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = c.ldd(V1, 0x278);
                        c.g[V0][0] = c.g[A0][0] << 19;
                        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0xffffffffffffdfff;
                        if t {
                            break 'b_159008;
                        } else {
                            c.cycles += 9;
                            c.g[A3][0] = sx((c.g[S0][0] as u32) << 1);
                            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.std(V1, 0x278, c.g[V0][0]);
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2));
                            c.g[A0][0] = c.g[S3][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x00159008);
                            c.g[A2][0] = 0x2;
                            c.pc = 0x0015a628;
                            call_indirect(c, 0x0015a628);
                            if c.pc != 0x00159008 {
                                return;
                            }
                            break 'b_159008;
                        }
                    }
                }
            }
        }
        'b_159064: {
            c.cycles += 14;
            c.g[V1][0] = 0xf88;
            c.g[A0][0] = 0x9b50;
            c.mult(0, 4, 17, 4);
            c.mult(0, 3, 16, 3);
            c.g[V0][0] = 0x4a0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A2][0] = c.ldd(V1, 0x278);
            c.g[V0][0] = c.g[A2][0] << 20;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffefff;
            if t {
                break 'b_159064;
            } else {
                c.cycles += 9;
                c.g[A3][0] = sx((c.g[S0][0] as u32) << 1);
                c.g[V0][0] = c.g[A2][0] & c.g[V0][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.std(V1, 0x278, c.g[V0][0]);
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[RA][0] = sx(0x00159064);
                c.g[A2][0] = 0x2;
                c.pc = 0x0015a628;
                call_indirect(c, 0x0015a628);
                if c.pc != 0x00159064 {
                    return;
                }
                break 'b_159064;
            }
        }
        c.cycles += 1;
        c.lq(16, c.pa(SP, 0x40));
        break 'b_159068;
    }
    c.cycles += 6;
    c.lq(17, c.pa(SP, 0x30));
    c.lq(18, c.pa(SP, 0x20));
    c.lq(19, c.pa(SP, 0x10));
    c.g[RA][0] = c.ldd(SP, 0);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = ra;
    return;
}

/// `int func_00157BF0(void* self, int a, int b, int c, int k)` (0x157bf0):
/// is the peak's goal complete for medal `k`? Kind 0/1 walks the five tracks
/// of `D_0045AAD8[k][k]` and compares their medals; kind 2 compares the
/// collectible and mission tiers; kind 3 compares the earnings medal;
/// anything else is false.
pub fn be_reward_int_is_peak_goal_complete(c: &mut Cpu) {
    let frame = Frame::open(c, 0x60);
    'b_157d40: {
        'b_157d3c: {
            'b_157d38: {
                'b_157d0c: {
                    'b_157cb0: {
                        c.cycles += 13;
                        c.g[V0][0] = 0x2;
                        frame.save(c, S0, 0x50);
                        frame.save(c, S1, 0x40);
                        c.g[S0][0] = c.g[A3][0];
                        frame.save(c, S3, 0x20);
                        c.g[S1][0] = c.g[A0][0];
                        frame.save(c, S4, 0x10);
                        c.g[S3][0] = c.g[A1][0];
                        frame.save(c, S2, 0x30);
                        c.g[S4][0] = c.g[A2][0];
                        let t = c.g[T0][0] == c.g[V0][0];
                        sd(c, r32(c, SP), c.g[RA][0]);
                        if t {
                            break 'b_157cb0;
                        } else {
                            'b_157c40: {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[T0][0] as i64) < 3i64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0x3;
                                if t {
                                    break 'b_157c40;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[T0][0] == c.g[V0][0];
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_157d0c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        frame.restore(c, S0, 0x50);
                                        if t {
                                            break 'b_157d40;
                                        } else {
                                            break 'b_157c40;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = (c.g[T0][0] as i64) < 0;
                            c.g[V1][0] = 0xa;
                            if t {
                                break 'b_157d38;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = 0x28;
                                c.mult(0, A0, S0, A0);
                                c.mult(0, V1, T0, V1);
                                c.g[V0][0] = 0x460000;
                                c.g[S2][0] = 0u64;
                                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5528));
                                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                                c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                                'l_157c68: loop {
                                    'b_157ca8: {
                                        c.cycles += 3;
                                        c.g[A3][0] = lb(c, r32(c, S0));
                                        let t = (c.g[A3][0] as i64) < 0;
                                        c.g[T0][0] = lb(c, at_3(c, S0, 0x1));
                                        if t {
                                            break 'b_157ca8;
                                        } else {
                                            c.cycles += 10;
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A1][0] = c.g[S3][0];
                                            c.g[RA][0] = sx(0x00157c84);
                                            c.g[A2][0] = c.g[S4][0];
                                            if callee::C00158910.call(c, 0x00157c84).is_err() {
                                                return;
                                            }
                                            c.g[A2][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[RA][0] = sx(0x00157c94);
                                            c.g[A1][0] = 0xffffffffffffffff;
                                            if callee::C00156A60.call(c, 0x00157c94).is_err() {
                                                return;
                                            }
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[S2][0] = sx(at_3(c, S2, 0x1));
                                            if t {
                                                break 'b_157d38;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[S0][0] = sx(at_3(c, S0, 0x2));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x00157c68);
                                                    }
                                                    continue 'l_157c68;
                                                } else {
                                                    break 'b_157ca8;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_157d3c;
                                    } else {
                                        break 'b_157cb0;
                                    }
                                }
                            }
                        }
                    }
                    'b_157d04: {
                        c.cycles += 12;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S3][0];
                        c.g[A2][0] = c.g[S4][0];
                        c.g[RA][0] = sx(0x00157cc4);
                        c.g[A3][0] = c.g[S0][0];
                        if callee::C001589B0.call(c, 0x00157cc4).is_err() {
                            return;
                        }
                        c.g[S2][0] = 0u64;
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00157cd8);
                        c.g[A1][0] = 0xffffffffffffffff;
                        if callee::C00156A60.call(c, 0x00157cd8).is_err() {
                            return;
                        }
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = c.g[S3][0];
                        if t {
                            break 'b_157d04;
                        } else {
                            c.cycles += 9;
                            c.g[A2][0] = c.g[S4][0];
                            c.g[A3][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00157cf0);
                            c.g[A0][0] = c.g[S1][0];
                            if callee::C00158A50.call(c, 0x00157cf0).is_err() {
                                return;
                            }
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00157d00);
                            c.g[A1][0] = 0xffffffffffffffff;
                            if callee::C00156A60.call(c, 0x00157d00).is_err() {
                                return;
                            }
                            c.g[S2][0] = (0u64 < c.g[V0][0]) as u64;
                            break 'b_157d04;
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.g[S2][0];
                    if t {
                        break 'b_157d3c;
                    } else {
                        break 'b_157d0c;
                    }
                }
                c.cycles += 11;
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = c.g[S4][0];
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00157d20);
                c.g[A3][0] = c.g[S0][0];
                if callee::C00158AF0.call(c, 0x00157d20).is_err() {
                    return;
                }
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00157d30);
                c.g[A1][0] = 0xffffffffffffffff;
                if callee::C00156A60.call(c, 0x00157d30).is_err() {
                    return;
                }
                let t = true;
                frame.restore(c, S0, 0x50);
                if t {
                    break 'b_157d40;
                } else {
                    break 'b_157d38;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_157d3c;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x50);
        break 'b_157d40;
    }
    c.cycles += 7;
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_3(c, SP, 0x60));
    c.pc = t;
    return;
}

/// `void func_00159B08(void* self, int rider, int k)` (0x159b08): mission `k`
/// was completed for `rider`: when the current count reached the stored
/// threshold, recompute the goal state, store the next threshold, grant the
/// medal when it is the best so far, and grant the mission reward once.
pub fn be_reward_int_on_mission_completed(c: &mut Cpu) {
    let frame = Frame::open(c, 0x80);
    'b_159c44: {
        'b_159c40: {
            c.cycles += 27;
            frame.save(c, S0, 0x70);
            frame.save(c, S3, 0x40);
            c.g[S0][0] = c.g[A1][0];
            frame.save(c, S4, 0x30);
            c.g[S3][0] = c.g[A2][0];
            frame.save(c, S5, 0x20);
            frame.save(c, S6, 0x10);
            frame.save(c, S1, 0x60);
            frame.save(c, S2, 0x50);
            sd(c, r32(c, SP), c.g[RA][0]);
            c.g[RA][0] = sx(0x00159b3c);
            c.g[S4][0] = c.g[A0][0];
            if callee::C0014DC80.call(c, 0x00159b3c).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00159b48);
            c.g[A1][0] = 0xa;
            if callee::C0014DD58.call(c, 0x00159b48).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00159b54);
            c.g[A1][0] = c.g[S3][0];
            if callee::C001543E0.call(c, 0x00159b54).is_err() {
                return;
            }
            c.g[S6][0] = c.g[V0][0];
            c.g[V1][0] = sx(r32(c, S3) << 2);
            c.g[V0][0] = 0x440000;
            c.g[V0][0] = sx(at_3(c, V0, 0xf58));
            c.g[S5][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[V0][0] = lw(c, r32(c, S5));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if t {
                break 'b_159c40;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    frame.restore(c, S0, 0x70);
                    break 'b_159c44;
                } else {
                    'b_159c00: {
                        'b_159bfc: {
                            c.cycles += 20;
                            c.g[RA][0] = sx(0x00159b84);
                            c.g[A0][0] = c.g[S0][0];
                            if callee::C0014A0E0.call(c, 0x00159b84).is_err() {
                                return;
                            }
                            c.g[S2][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00159b90);
                            c.g[A0][0] = c.g[S0][0];
                            if callee::C0014A080.call(c, 0x00159b90).is_err() {
                                return;
                            }
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A3][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x00159bac);
                            c.g[T0][0] = 0x2;
                            if callee::C00157BF0.call(c, 0x00159bac).is_err() {
                                return;
                            }
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00159bc4);
                            c.g[A3][0] = c.g[S3][0];
                            if callee::C00158D58.call(c, 0x00159bc4).is_err() {
                                return;
                            }
                            let t = c.g[S0][0] == 0u64;
                            sw(c, r32(c, S5), c.g[V0][0]);
                            if t {
                                break 'b_159bfc;
                            } else {
                                c.cycles += 7;
                                c.g[V0][0] = 0x460000;
                                c.g[A0][0] = sx(r32(c, S3) << 4);
                                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x4fe8));
                                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                                c.g[V1][0] = lw(c, at_3(c, V0, 0xc));
                                let t = c.g[S6][0] != c.g[V1][0];
                                c.g[A0][0] = c.g[S4][0];
                                if t {
                                    break 'b_159c00;
                                } else {
                                    c.cycles += 5;
                                    c.g[A3][0] = c.g[S3][0];
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A2][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x00159bfc);
                                    c.g[T0][0] = 0x2;
                                    if callee::C00159170.call(c, 0x00159bfc).is_err() {
                                        return;
                                    }
                                    break 'b_159bfc;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S4][0];
                        break 'b_159c00;
                    }
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00159c0c);
                    c.g[A2][0] = c.g[S1][0];
                    if callee::C001577E0.call(c, 0x00159c0c).is_err() {
                        return;
                    }
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S4][0];
                    if t {
                        break 'b_159c40;
                    } else {
                        c.cycles += 6;
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00159c24);
                        c.g[A3][0] = 0u64;
                        if callee::C0015A2E0.call(c, 0x00159c24).is_err() {
                            return;
                        }
                        let t = c.g[V0][0] != 0u64;
                        frame.restore(c, S0, 0x70);
                        if t {
                            break 'b_159c44;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00159c40);
                            c.g[A3][0] = 0u64;
                            if callee::C00159CD0.call(c, 0x00159c40).is_err() {
                                return;
                            }
                            break 'b_159c40;
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x70);
        break 'b_159c44;
    }
    c.cycles += 9;
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore(c, S4, 0x30);
    frame.restore(c, S5, 0x20);
    frame.restore(c, S6, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_3(c, SP, 0x80));
    c.pc = t;
    return;
}

/// `void func_001599A0(void* self, int rider, int k)` (0x1599a0): collectible
/// `k` was collected for `rider`: same shape as the mission-completed
/// handler, with the collectible thresholds at_3 `D_00440F48`.
pub fn be_reward_int_on_collectible_collected(c: &mut Cpu) {
    let frame = Frame::open(c, 0x80);
    'b_159ae0: {
        'b_159adc: {
            c.cycles += 28;
            frame.save(c, S0, 0x70);
            frame.save(c, S3, 0x40);
            c.g[S0][0] = c.g[A1][0];
            frame.save(c, S4, 0x30);
            c.g[S3][0] = c.g[A2][0];
            frame.save(c, S5, 0x20);
            frame.save(c, S6, 0x10);
            frame.save(c, S1, 0x60);
            frame.save(c, S2, 0x50);
            sd(c, r32(c, SP), c.g[RA][0]);
            c.g[RA][0] = sx(0x001599d4);
            c.g[S4][0] = c.g[A0][0];
            if callee::C0014DC80.call(c, 0x001599d4).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001599e0);
            c.g[A1][0] = 0xa;
            if callee::C0014DD58.call(c, 0x001599e0).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001599f0);
            c.g[A2][0] = c.g[S3][0];
            if callee::C001535C0.call(c, 0x001599f0).is_err() {
                return;
            }
            c.g[S6][0] = c.g[V0][0];
            c.g[V1][0] = sx(r32(c, S3) << 2);
            c.g[V0][0] = 0x440000;
            c.g[V0][0] = sx(at_3(c, V0, 0xf48));
            c.g[S5][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[V0][0] = lw(c, r32(c, S5));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if t {
                break 'b_159adc;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    frame.restore(c, S0, 0x70);
                    break 'b_159ae0;
                } else {
                    'b_159a9c: {
                        'b_159a98: {
                            c.cycles += 20;
                            c.g[RA][0] = sx(0x00159a20);
                            c.g[A0][0] = c.g[S0][0];
                            if callee::C0014A0E0.call(c, 0x00159a20).is_err() {
                                return;
                            }
                            c.g[S2][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00159a2c);
                            c.g[A0][0] = c.g[S0][0];
                            if callee::C0014A080.call(c, 0x00159a2c).is_err() {
                                return;
                            }
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A3][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x00159a48);
                            c.g[T0][0] = 0x2;
                            if callee::C00157BF0.call(c, 0x00159a48).is_err() {
                                return;
                            }
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00159a60);
                            c.g[A3][0] = c.g[S3][0];
                            if callee::C00158C80.call(c, 0x00159a60).is_err() {
                                return;
                            }
                            let t = c.g[S0][0] == 0u64;
                            sw(c, r32(c, S5), c.g[V0][0]);
                            if t {
                                break 'b_159a98;
                            } else {
                                c.cycles += 7;
                                c.g[V0][0] = 0x460000;
                                c.g[A0][0] = sx(r32(c, S3) << 4);
                                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5018));
                                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                                c.g[V1][0] = lw(c, at_3(c, V0, 0xc));
                                let t = c.g[S6][0] != c.g[V1][0];
                                c.g[A0][0] = c.g[S4][0];
                                if t {
                                    break 'b_159a9c;
                                } else {
                                    c.cycles += 5;
                                    c.g[A3][0] = c.g[S3][0];
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A2][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x00159a98);
                                    c.g[T0][0] = 0x2;
                                    if callee::C00159170.call(c, 0x00159a98).is_err() {
                                        return;
                                    }
                                    break 'b_159a98;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S4][0];
                        break 'b_159a9c;
                    }
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00159aa8);
                    c.g[A2][0] = c.g[S1][0];
                    if callee::C001577E0.call(c, 0x00159aa8).is_err() {
                        return;
                    }
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S4][0];
                    if t {
                        break 'b_159adc;
                    } else {
                        c.cycles += 6;
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00159ac0);
                        c.g[A3][0] = 0u64;
                        if callee::C0015A2E0.call(c, 0x00159ac0).is_err() {
                            return;
                        }
                        let t = c.g[V0][0] != 0u64;
                        frame.restore(c, S0, 0x70);
                        if t {
                            break 'b_159ae0;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00159adc);
                            c.g[A3][0] = 0u64;
                            if callee::C00159CD0.call(c, 0x00159adc).is_err() {
                                return;
                            }
                            break 'b_159adc;
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x70);
        break 'b_159ae0;
    }
    c.cycles += 9;
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore(c, S4, 0x30);
    frame.restore(c, S5, 0x20);
    frame.restore(c, S6, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_3(c, SP, 0x80));
    c.pc = t;
    return;
}

/// `RewardsFile` field offsets (`self`).
pub(crate) const REWARDS_BASE: u32 = 0x4;

pub(crate) const REWARDS_STRINGS: u32 = 0x2c;

/// Jump table of the section parser: one handler address per section type.
#[cfg(test)]
pub(crate) const REWARDS_JUMP_TABLE: u32 = 0x0045_ab50;

pub(crate) fn rewards_parse_resume_2(c: &mut Cpu) {
    ee::s01::f_0015a818__entries(c)
}

/// One string-offset fixup: the word at `a0 + off`, when non-zero, becomes
/// `strings + word - 1`. `pre` is the cycle charge of the block head.
pub(crate) fn rewards_fixup_word(c: &mut Cpu, off: u32, pre: u64) {
    c.cycles += pre;
    let at = (c.g[A0][0] as u32).wrapping_add(off);
    c.g[V1][0] = c.lw(at);
    let zero = c.g[V1][0] == 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
    if !zero {
        c.cycles += 3;
        let strings = c.lw((c.g[A3][0] as u32).wrapping_add(REWARDS_STRINGS));
        c.g[V0][0] = strings;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(at, c.g[V0][0]);
    }
}

/// Fixup loop over the 12-byte entries of the first two tables (`a2` is the
/// byte offset, the table pointer is reloaded from `self + tbl` each time).
/// The count is in `v0` and `a1 = 0` on entry (generated block head).
pub(crate) fn rewards_fix_stride12(c: &mut Cpu, tbl: u32, cnt: u32, head: u32) {
    if (c.g[V0][0] as i64) <= 0 {
        return;
    }
    c.cycles += 2;
    c.g[A2][0] = 0;
    loop {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(tbl));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        rewards_fixup_word(c, 0, 0);
        rewards_fixup_word(c, 4, 3);
        rewards_fixup_word(c, 8, 3);
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(cnt));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// Fixup loop over the 8- or 16-byte entries of the other tables
/// (`v1` holds the table pointer, `shift` the entry size, `words` how many
/// string words each entry has). Same entry state as [`rewards_fix_stride12`].
pub(crate) fn rewards_fix_shifted(
    c: &mut Cpu,
    tbl: u32,
    cnt: u32,
    shift: u32,
    words: u32,
    head: u32,
) {
    if (c.g[V0][0] as i64) <= 0 {
        return;
    }
    c.cycles += 2;
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(tbl));
    loop {
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[A1][0] as u32) << shift);
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        rewards_fixup_word(c, 0, 0);
        rewards_fixup_word(c, 4, 3);
        if words == 3 {
            rewards_fixup_word(c, 8, 3);
        }
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(cnt));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(tbl));
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// Header of a fixup loop: `cycles += pre`, load the count at `self + cnt`
/// into `v0`, `a1 = 0`.
pub(crate) fn rewards_fix_head(c: &mut Cpu, pre: u64, cnt: u32) {
    c.cycles += pre;
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(cnt));
    c.g[A1][0] = 0;
}

/// `void RewardsFile_Parse(RewardsFile* self, int* data)` (0x15a818): parse a
/// rewards file in place. The data starts with nine `(type, count)` headers,
/// each followed by `count` table entries (12 bytes for types 0 and 1, 8 for
/// type 2, 16 for the rest); the jump table at 0x45ab50 sends a type to its
/// handler, which stores the table pointer and entry count in `self`. The
/// string pool follows (after one more word); every non-zero string offset in
/// the entries then becomes a pointer (`strings + offset - 1`).
///
/// Resume entries (`__entries`) go to the generated twin.
pub fn rewards_file_parse(c: &mut Cpu) {
    if c.pc != 0x0015_a818 {
        return rewards_parse_resume_2(c);
    }
    c.cycles += 4;
    c.g[A3][0] = c.g[A0][0];
    c.g[T0][0] = 0;
    c.sw((c.g[A3][0] as u32).wrapping_add(REWARDS_BASE), c.g[A1][0]);
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    loop {
        c.cycles += 4;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
        c.g[A2][0] = c.lw(c.g[A1][0] as u32);
        let empty = c.g[A2][0] == 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
        if !empty {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < 9) as u64;
            let in_range = c.g[V0][0] != 0;
            c.g[V0][0] = 0x46_0000;
            if in_range {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x54b0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                let target = c.g[A0][0] as u32;
                // (table pointer offset, count offset, entry shift)
                let shifted = |off: u32, cnt: u32, shift: u32, c: &mut Cpu| {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << shift);
                    c.sw((c.g[A3][0] as u32).wrapping_add(off), c.g[A1][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[A3][0] as u32).wrapping_add(cnt), c.g[A2][0]);
                };
                // Types 0 and 1: 12-byte entries, `v1 = count * 12`.
                let stride12 = |off: u32, cnt: u32, c: &mut Cpu| {
                    c.cycles += 6;
                    c.g[V0][0] = 0xc;
                    c.sw((c.g[A3][0] as u32).wrapping_add(off), c.g[A1][0]);
                    c.mult(0, 3, 6, 2);
                    c.sw((c.g[A3][0] as u32).wrapping_add(cnt), c.g[A2][0]);
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                };
                match target {
                    0x0015_a85c => stride12(0x8, 0x30, c),
                    0x0015_a874 => stride12(0xc, 0x34, c),
                    0x0015_a88c => shifted(0x10, 0x38, 3, c),
                    0x0015_a8a0 => shifted(0x14, 0x3c, 4, c),
                    0x0015_a8b4 => shifted(0x18, 0x40, 4, c),
                    0x0015_a8c8 => shifted(0x1c, 0x44, 4, c),
                    0x0015_a8dc => shifted(0x24, 0x4c, 4, c),
                    0x0015_a8f0 => shifted(0x20, 0x48, 4, c),
                    0x0015_a904 => {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[A2][0] as u32) << 4);
                        c.sw((c.g[A3][0] as u32).wrapping_add(0x28), c.g[A1][0]);
                        c.sw((c.g[A3][0] as u32).wrapping_add(0x50), c.g[A2][0]);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    }
                    _ => {
                        c.jump_out(target, 0x0015_a818, 0x0015_ad54);
                        return;
                    }
                }
            }
        }
        c.cycles += 4;
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[T0][0] as i64) < 9) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        if event_due(c) {
            deliver_event_here(c, 0x0015_a828);
        }
    }
    // String pool, then the fixups of each table.
    c.cycles += 5;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x30));
    c.sw(
        (c.g[A3][0] as u32).wrapping_add(REWARDS_STRINGS),
        c.g[A1][0],
    );
    c.g[A1][0] = 0;
    rewards_fix_stride12(c, 0x8, 0x30, 0x0015_a940);
    rewards_fix_head(c, 3, 0x34);
    rewards_fix_stride12(c, 0xc, 0x34, 0x0015_a9b8);
    rewards_fix_head(c, 3, 0x38);
    rewards_fix_shifted(c, 0x10, 0x38, 3, 2, 0x0015_aa30);
    rewards_fix_head(c, 3, 0x3c);
    rewards_fix_shifted(c, 0x14, 0x3c, 4, 3, 0x0015_aa90);
    rewards_fix_head(c, 3, 0x40);
    rewards_fix_shifted(c, 0x18, 0x40, 4, 3, 0x0015_ab08);
    rewards_fix_head(c, 3, 0x44);
    rewards_fix_shifted(c, 0x1c, 0x44, 4, 3, 0x0015_ab80);
    rewards_fix_head(c, 3, 0x4c);
    rewards_fix_shifted(c, 0x24, 0x4c, 4, 3, 0x0015_abf8);
    rewards_fix_head(c, 3, 0x48);
    rewards_fix_shifted(c, 0x20, 0x48, 4, 3, 0x0015_ac70);
    rewards_fix_head(c, 3, 0x50);
    rewards_fix_shifted(c, 0x28, 0x50, 4, 3, 0x0015_ace8);
    c.cycles += 2;
    ret(c);
}

/// Shared body of the two `BERewardInt_GetNext*Threshold` functions (they
/// differ in the tier callee, the threshold table and their return
/// addresses): the lowest threshold of `kind` (`a3`, table rows of 4 ints,
/// 16 bytes apart at `0x460000 - table_off`) whose place is better than the
/// player's current one (`IsBetterPlace(self, tier, i)`), 0 when none.
/// `tier_call`/`tier_ret`: the tier getter (called with the caller's a0..a3)
/// and where it returns; `better_ret`: the `IsBetterPlace` return address;
/// `loop_pc`: the loop head the event check resumes at.
pub(crate) fn oc429_next_threshold(
    c: &mut Cpu,
    tier_call: u32,
    tier_ret: u32,
    better_ret: u32,
    loop_pc: u32,
    table_off: u32,
) -> crate::call::Flow {
    c.cycles += 22;
    let frame = Frame::open(c, 0x90);
    frame.save(c, S4, 0x40);
    frame.save(c, S5, 0x30);
    c.g[S4][0] = c.g[A3][0];
    c.g[S5][0] = c.g[A0][0];
    frame.save(c, S0, 0x80);
    frame.save(c, S1, 0x70);
    frame.save(c, S2, 0x60);
    frame.save(c, S3, 0x50);
    c.g[S2][0] = 0;
    frame.save(c, 22, 0x20);
    c.g[S0][0] = 0;
    frame.save(c, S7, 0x10);
    frame.save_ra(c, 0);
    c.g[S3][0] = 0;
    call_ptr(c, tier_call, tier_ret)?;
    c.g[S7][0] = c.g[V0][0];
    c.g[V1][0] = sx((c.g[S4][0] as u32) << 4);
    c.g[V0][0] = 0x46_0000;
    c.g[S6][0] = sx(oc429_at(c.g[V0][0], 0u32.wrapping_sub(table_off)));
    c.g[S1][0] = sx(oc429_at(c.g[V1][0], c.g[S6][0] as u32));
    loop {
        'next: {
            c.cycles += 6;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S7][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0015_6a60, better_ret)?; // BERewardInt_IsBetterPlace
            if c.g[V0][0] == 0 {
                c.g[S0][0] = sx(oc429_at(c.g[S0][0], 1));
                break 'next;
            }
            'take: {
                c.cycles += 2;
                let none_yet = c.g[S2][0] == 0;
                c.g[V1][0] = sx((c.g[S4][0] as u32) << 4);
                if !none_yet {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = sx(oc429_at(c.g[S0][0], 1));
                        break 'next;
                    }
                }
                break 'take;
            }
            c.cycles += 4;
            c.g[V0][0] = sx(oc429_at(c.g[S3][0], c.g[V1][0] as u32));
            c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[S6][0] as u32));
            c.g[S2][0] = c.lw(c.g[V0][0] as u32);
            c.g[S0][0] = sx(oc429_at(c.g[S0][0], 1));
        }
        c.cycles += 4;
        c.g[S3][0] = sx(oc429_at(c.g[S3][0], 4));
        c.g[V0][0] = ((c.g[S0][0] as i64) < 4) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S1][0] = sx(oc429_at(c.g[S1][0], 4));
        if more {
            if event_due(c) {
                deliver_event_here(c, loop_pc);
            }
            continue;
        }
        break;
    }
    c.cycles += 12;
    c.g[V0][0] = c.g[S2][0];
    frame.restore(c, S0, 0x80);
    frame.restore(c, S1, 0x70);
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore(c, S4, 0x40);
    frame.restore(c, S5, 0x30);
    frame.restore(c, 22, 0x20);
    frame.restore(c, S7, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `int BERewardInt_GetNextMissionThreshold(BEReward* self, int profile, int
/// character, int kind)` (0x158d58): the next mission-medal threshold to aim
/// for (table at `0x45b018`), see [`oc429_next_threshold`].
pub fn be_reward_int_get_next_mission_threshold(c: &mut Cpu) {
    guest(c, |c| {
        oc429_next_threshold(
            c,
            0x0015_8a50,
            0x0015_8d98,
            0x0015_8dc0,
            0x0015_8db0,
            0x4fe8,
        )
    })
}

/// `int BERewardInt_GetNextCollectibleThreshold(BEReward* self, int profile,
/// int character, int kind)` (0x158c80): the next collectible-medal
/// threshold (table at `0x45afe8`), see [`oc429_next_threshold`].
pub fn be_reward_int_get_next_collectible_threshold(c: &mut Cpu) {
    guest(c, |c| {
        oc429_next_threshold(
            c,
            0x0015_89b0,
            0x0015_8cc0,
            0x0015_8ce8,
            0x0015_8cd8,
            0x5018,
        )
    })
}

/// `int BERewardInt_BeginRewardTracking(BERewardInt* self)` (0x158e30): clears the new
/// rewards, looks up the first rider's character and stores the next earnings threshold
/// (`gp - 0x1e90`), the three collectible thresholds (`0x440f48`) and the three mission
/// thresholds (`0x440f58`), then resets the reward interface's counters
/// (`+0x10` = 0, `+0x14` = -1). Leaves v0 = -1.
pub fn be_reward_int_begin_reward_tracking(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S3][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A0][0] = 0x4c0000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3ef0));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S1][0] = 0u64;
        Callee {
            addr: 0x0015a510,
            f: ee::s01::BERewardInt_ClearNewRewards,
        }
        .call(c, 0x00158e5c)?;
        c.g[A0][0] = 0u64;
        Callee {
            addr: 0x0014a130,
            f: ee::s01::BE_FindRiderByProfile,
        }
        .call(c, 0x00158e64)?;
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x0014a080,
            f: ee::s01::BE_GetRiderCharacter,
        }
        .call(c, 0x00158e6c)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0u64;
        c.g[A2][0] = c.g[S2][0];
        Callee {
            addr: 0x00158be0,
            f: ee::s01::BERewardInt_GetNextPointsThreshold,
        }
        .call(c, 0x00158e80)?;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1e90), c.g[V0][0]);
        c.g[V0][0] = 0x440000;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf48));
        'l_158e90: loop {
            c.cycles += 10;
            c.g[A3][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0u64;
            c.g[A2][0] = c.g[S2][0];
            Callee {
                addr: 0x00158c80,
                f: ee::s01::BERewardInt_GetNextCollectibleThreshold,
            }
            .call(c, 0x00158ea4)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.g[V0][0] = ((c.g[S1][0] as i64) < 3i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00158e90);
                }
                continue 'l_158e90;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x440000;
                c.g[S0][0] = 0u64;
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf58));
                'l_158ec8: loop {
                    c.cycles += 10;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00158d58,
                        f: ee::s01::BERewardInt_GetNextMissionThreshold,
                    }
                    .call(c, 0x00158edc)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00158ec8);
                        }
                        continue 'l_158ec8;
                    } else {
                        c.cycles += 11;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1e8c));
                        c.g[V0][0] = 0xffffffffffffffff;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

/// `void BERewardInt_LoadRewardsFile(void)` (0x156a10): load the rewards
/// file (`RewardsFile_Load(0x4c3e98, 1)`). Not installed: the real loader
/// runs away on test data, so there is no differential test.
#[allow(dead_code)]
pub fn be_reward_int_load_rewards_file_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x4c0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3e98));
        crate::game::be::beintmission::callee::REWARDS_FILE_LOAD.call(c, 0x0015_6a28)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BERewardInt_GetAudioSystemField504(void)` (0x157350): the word at
/// +0x504 of the object the table function at 0x28b180 returns.
pub fn be_reward_int_get_audio_system_field504(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0028_b180, 0x0015_7360)?;
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x504)).lw(c);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Entry of `BERewardInt_GrantReward` (0x159cd0) and its end (0x15a2b4 is the
/// epilogue, 0x15a2e0 the next function).
pub(crate) const GRANT_ENTRY: u32 = 0x0015_9cd0;

pub(crate) const GRANT_END: u32 = 0x0015_a2e0;

/// The awards-received bits of a character record
/// (`cBELibrary_profileLib[profile][character]`, see the body).
pub(crate) const GRANT_AWARDS_RECEIVED: u32 = 0xf28;

/// Callees (the generated aliases; each goes through its override slot, so a
/// replaced callee is reached too).
pub(crate) const GRANT_IS_REWARD_GRANTED: Callee = Callee {
    addr: 0x0015_a2e0,
    f: ee::s01::f_0015a2e0,
};

pub(crate) const GRANT_PICK_LOCKED_REWARD: Callee = Callee {
    addr: 0x0015_7080,
    f: ee::s01::f_00157080,
};

pub(crate) const GRANT_UNLOCK_TRADING_CARD: Callee = Callee {
    addr: 0x0015_80f8,
    f: ee::s01::f_001580f8,
};

pub(crate) const GRANT_UNLOCK_POSTER: Callee = Callee {
    addr: 0x0015_7fd0,
    f: ee::s01::f_00157fd0,
};

pub(crate) const GRANT_UNLOCK_TOY: Callee = Callee {
    addr: 0x0015_8348,
    f: ee::s01::f_00158348,
};

pub(crate) const GRANT_UNLOCK_CONCEPT_ART: Callee = Callee {
    addr: 0x0015_8220,
    f: ee::s01::f_00158220,
};

pub(crate) const GRANT_UNLOCK_RIDER_ITEM: Callee = Callee {
    addr: 0x0015_8618,
    f: ee::s01::f_00158618,
};

pub(crate) const GRANT_ADD_NEW_REWARD: Callee = Callee {
    addr: 0x0015_a628,
    f: ee::s01::f_0015a628,
};

pub(crate) const GRANT_SET_GRANTED_BIT: Callee = Callee {
    addr: 0x0015_a358,
    f: ee::s01::f_0015a358,
};

pub(crate) const GRANT_RIVAL_REWARDS: Callee = Callee {
    addr: 0x0015_8f60,
    f: ee::s01::f_00158f60,
};

pub(crate) const GRANT_PICK_RANDOM_BOLT: Callee = Callee {
    addr: 0x0015_6c70,
    f: ee::s01::f_00156c70,
};

pub(crate) const GRANT_PICK_RANDOM_BOLT_800: Callee = Callee {
    addr: 0x0015_6ee0,
    f: ee::s01::f_00156ee0,
};

pub(crate) const GRANT_BE_LIBRARY_GET_INSTANCE: Callee = Callee {
    addr: 0x0014_dc80,
    f: ee::s01::f_0014dc80,
};

pub(crate) const GRANT_BE_LIBRARY_GET_INTERFACE: Callee = Callee {
    addr: 0x0014_dd58,
    f: ee::s01::f_0014dd58,
};

pub(crate) const GRANT_UNLOCK_BOLT: Callee = Callee {
    addr: 0x0014_b560,
    f: ee::s01::f_0014b560,
};

pub(crate) const GRANT_IS_PEAK_FULLY_COMPLETED: Callee = Callee {
    addr: 0x0015_77e0,
    f: ee::s01::f_001577e0,
};

pub(crate) const GRANT_REWARD: Callee = Callee {
    addr: 0x0015_9cd0,
    f: ee::s01::f_00159cd0,
};

pub(crate) const GRANT_ARE_ALL_PEAKS_COMPLETE: Callee = Callee {
    addr: 0x0015_78a0,
    f: ee::s01::f_001578a0,
};

pub(crate) const GRANT_IS_PEAK_COMPLETE: Callee = Callee {
    addr: 0x0015_7a78,
    f: ee::s01::f_00157a78,
};

pub(crate) const GRANT_MEDAL_REWARD_ID: Callee = Callee {
    addr: 0x0015_9080,
    f: ee::s01::f_00159080,
};

pub(crate) const GRANT_IS_PEAK_GOAL_COMPLETE: Callee = Callee {
    addr: 0x0015_7bf0,
    f: ee::s01::f_00157bf0,
};

pub(crate) const GRANT_MEDAL_REWARD: Callee = Callee {
    addr: 0x0015_9170,
    f: ee::s01::f_00159170,
};

/// `addiu`: 32-bit wrapping add of a constant, sign-extended to 64 bits.
#[inline(always)]
pub(crate) fn grant_addu(a: u64, k: u32) -> u64 {
    sx((a as u32).wrapping_add(k))
}

/// `addu`: 32-bit wrapping add of two registers, sign-extended.
#[inline(always)]
pub(crate) fn grant_addr(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// `addiu` with a negative constant (`- k`), sign-extended.
#[inline(always)]
pub(crate) fn grant_subu(a: u64, k: u32) -> u64 {
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

pub(crate) fn grant_reward_body(c: &mut Cpu) -> Flow {
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
pub(crate) fn grant_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
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
pub(crate) fn grant_bit_dispatch(c: &mut Cpu, frame: Frame) -> Flow {
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
pub(crate) fn grant_store_item(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    let a = frame.at(c, 0x20);
    c.sw(a, c.g[V0][0]);
    grant_from_a098(c, frame)
}

/// Bits 5, 8, 11, 14 (0x159e2c): up to four trading cards (each a random
/// locked card, kept in `v[i]`), then one poster, then the peak pass for
/// bit 11 (MessageCenter_CheckPeakMessages) and GrantRivalRewards.
pub(crate) fn grant_card_bits(c: &mut Cpu, frame: Frame) -> Flow {
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
pub(crate) fn grant_toy_bits(c: &mut Cpu, frame: Frame) -> Flow {
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
pub(crate) fn grant_from_a098(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.g[A2][0] = c.g[S1][0];
    grant_from_a09c(c, frame)
}

/// 0x15a09c: sets the granted bit, logs the reward and the cheat character
/// (or the item) when there is one, then the rest of the function.
pub(crate) fn grant_from_a09c(c: &mut Cpu, frame: Frame) -> Flow {
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
pub(crate) fn grant_item_tail(c: &mut Cpu, frame: Frame) -> Flow {
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
pub(crate) fn grant_no_item_tail(c: &mut Cpu, frame: Frame) -> Flow {
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
pub(crate) fn grant_mountain_completed(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.g[A0][0] = c.g[S4][0];
    grant_all_peaks(c, frame)
}

/// 0x15a170: `AreAllPeaksComplete`: grant bit 1 when it holds and is not
/// granted yet, then the per-peak awards.
pub(crate) fn grant_all_peaks(c: &mut Cpu, frame: Frame) -> Flow {
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
pub(crate) fn grant_after_peaks_loop(c: &mut Cpu, frame: Frame) -> Flow {
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
pub(crate) fn grant_medal_loop(c: &mut Cpu, frame: Frame) -> Flow {
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

/// The rewards file (`BERewardInt` file block, `REWARDS_FILE + 0x3e98`).
#[cfg(test)]
pub(crate) const REWARDS_FILE_PTR: u32 = 0x004c_3e98;

/// One reward unlock: the shape the seven `BERewardInt_Unlock*` bodies share.
/// Posters, trading cards, concept art and toys also count the unlocked
/// entries and grant the rider-item bonus when the counts agree; videos and
/// rider items go straight from the award to the money query.
#[derive(Clone, Copy)]
pub(crate) struct RewardUnlock {
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
pub(crate) struct RewardCount {
    count_fn: u32,
    ret_count: u32,
    get_fn: u32,
    ret_get: u32,
    /// The `a3` of the rider-item unlock made when the counts agree.
    rider_idx: u64,
    ret_rider: u32,
}

pub(crate) const UNLOCK_POSTER: RewardUnlock = RewardUnlock {
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

pub(crate) const UNLOCK_TRADING_CARD: RewardUnlock = RewardUnlock {
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

pub(crate) const UNLOCK_CONCEPT_ART: RewardUnlock = RewardUnlock {
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

pub(crate) const UNLOCK_TOY: RewardUnlock = RewardUnlock {
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

pub(crate) const UNLOCK_VIDEO: RewardUnlock = RewardUnlock {
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

pub(crate) const UNLOCK_RIDER_ITEM: RewardUnlock = RewardUnlock {
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

/// `int BERewardInt_PickRandomBoltFlag800(self, profile, character)`
/// (0x156ee0): pick one of the character's bolts that carries the 0x800
/// flag, at random (`Rand_NextMain` modulo the flagged count), and return
/// its bolt id; -1 when there is none. Bolt records are 0x38 bytes apart.
pub fn be_reward_int_pick_random_bolt_flag800(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x60);
        'pick: {
            c.g[A0][0] = 0xf88;
            frame.save(c, S3, 0x20);
            c.g[V1][0] = 0x9b50;
            c.g[S3][0] = c.g[A2][0];
            c.mult(0, 5, 5, 3);
            c.mult(0, 4, 19, 4);
            c.g[V0][0] = 0x4a0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
            frame.save(c, S1, 0x40);
            frame.save(c, S0, 0x50);
            frame.save(c, S2, 0x30);
            c.g[S0][0] = 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            frame.save(c, S4, 0x10);
            frame.save_ra(c, 0);
            c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, 0x0014_bdb8, 0x0015_6f28)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0014_d988, 0x0015_6f38)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[S4][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x0014_d998, 0x0015_6f48)?;
            let t = (c.g[S4][0] as i64) <= 0;
            c.g[A0][0] = c.g[V0][0];
            if t {
                break 'pick;
            }
            c.cycles += 2;
            c.g[A1][0] = c.g[S4][0];
            loop {
                'entry: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] &= 0x800;
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        break 'entry;
                    }
                    'flagged: {
                        'slot: {
                            c.cycles += 7;
                            c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x288));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                            let t = (c.g[V0][0] as i64) < 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            if t {
                                break 'slot;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            break 'flagged;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] &= 0x2;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[V1][0];
                    }
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                }
                c.cycles += 2;
                let t = c.g[A1][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
                if !t {
                    break 'pick;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0015_6f58);
                }
            }
        }
        'exit: {
            c.cycles += 2;
            let t = (c.g[S0][0] as i64) <= 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if t {
                break 'exit;
            }
            c.cycles += 5;
            call_ptr(c, 0x0031_77f0, 0x0015_6fc0)?;
            c.divu(0, 2, 16);
            'bolt_pick: {
                c.cycles += 8;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S3][0];
                c.g[V1][0] = c.hi[0];
                c.g[S0][0] = c.g[V1][0];
                call_ptr(c, 0x0014_d998, 0x0015_6fe0)?;
                c.g[A0][0] = c.g[V0][0];
                let t = (c.g[S4][0] as i64) <= 0;
                c.g[A2][0] = 0;
                if t {
                    break 'bolt_pick;
                }
                c.cycles += 1;
                loop {
                    'bolt: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                        c.g[V0][0] &= 0x800;
                        if c.g[V0][0] == 0 {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            break 'bolt;
                        }
                        'bolt_slot: {
                            'bolt_flag: {
                                c.cycles += 7;
                                c.g[A1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x288));
                                c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                                let t = (c.g[V0][0] as i64) < 0;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                if t {
                                    break 'bolt_flag;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                c.g[V0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                break 'bolt_slot;
                            }
                            c.cycles += 1;
                            c.g[V0][0] = 0;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                        c.g[V0][0] &= 0x2;
                        if c.g[V0][0] != 0 {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            break 'bolt;
                        }
                        c.cycles += 3;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                        if (c.g[S0][0] as i64) >= 0 {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            break 'bolt;
                        }
                        // The chosen bolt: its id is the result.
                        c.cycles += 2;
                        c.g[V0][0] = c.g[A1][0];
                        break 'exit;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
                    if !t {
                        break 'bolt_pick;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_6ff0);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        }
        c.cycles += 8;
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

/// `bool BERewardInt_IsPeakGoldComplete(self, profile, character, tier)`
/// (0x157920): walk the peak table of the character (signed-byte entries,
/// a negative entry ends a list); for each entry check the medal or
/// mission tier and the event's best place, and return 1 when the gold
/// goals of every peak are met, 0 as soon as one is not.
pub fn be_reward_int_is_peak_gold_complete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x28;
        frame.save(c, S5, 0x40);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S2, 0x70);
        c.mult(0, 0, 21, 2);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S6, 0x30);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, FP, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S0, 0x90);
        c.g[FP][0] = c.lo[0];
        frame.save(c, S1, 0x80);
        c.g[S6][0] = 0;
        frame.save(c, S7, 0x20);
        frame.save_ra(c, 0);
        'outer: loop {
            c.cycles += 4;
            c.g[V0][0] = 0x460000;
            c.g[S7][0] = 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5528));
            c.g[S1][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[V0][0] as u32));
            'inner: loop {
                'done: {
                    'entry: {
                        c.cycles += 3;
                        c.g[S0][0] = c.lb(c.g[S1][0] as u32);
                        let t = (c.g[S0][0] as i64) < 0;
                        c.g[T0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'entry;
                        }
                        'slot: {
                            'case_a24: {
                                c.cycles += 3;
                                c.g[V0][0] = 3;
                                let t = c.g[S6][0] == c.g[V0][0];
                                c.g[V0][0] = 2;
                                if t {
                                    break 'case_a24;
                                }
                                'case_1579dc: {
                                    c.cycles += 2;
                                    let t = c.g[S6][0] != c.g[V0][0];
                                    c.g[A2][0] = c.g[S3][0];
                                    if t {
                                        break 'case_1579dc;
                                    }
                                    c.cycles += 6;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A1][0] = c.g[S4][0];
                                    c.g[A3][0] = c.g[S5][0];
                                    call_ptr(c, 0x0015_89b0, 0x0015_79b0)?;
                                    let t = c.g[V0][0] != 0;
                                    c.g[V0][0] = 0;
                                    if t {
                                        break 'done;
                                    }
                                    c.cycles += 7;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A1][0] = c.g[S4][0];
                                    c.g[A2][0] = c.g[S3][0];
                                    c.g[A3][0] = c.g[S5][0];
                                    call_ptr(c, 0x0015_8a50, 0x0015_79cc)?;
                                    let t = c.g[V0][0] == 0;
                                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                                    if t {
                                        break 'slot;
                                    }
                                    c.cycles += 2;
                                    c.g[V0][0] = 0;
                                    break 'done;
                                }
                                'case_157a18: {
                                    c.cycles += 9;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A1][0] = c.g[S4][0];
                                    c.g[A3][0] = c.g[S0][0];
                                    call_ptr(c, 0x0015_8910, 0x0015_79ec)?;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4));
                                    c.g[V0][0] = (c.g[V0][0] < 0x8) as u64;
                                    let t = c.g[V0][0] == 0;
                                    c.g[A0][0] = c.g[S2][0];
                                    if t {
                                        break 'case_157a18;
                                    }
                                    c.cycles += 4;
                                    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                                    call_ptr(c, 0x0015_6a60, 0x0015_7a08)?;
                                    let t = c.g[V0][0] != 0;
                                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                                    if t {
                                        break 'slot;
                                    }
                                    c.cycles += 2;
                                    c.g[V0][0] = 0;
                                    break 'done;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = (c.g[A2][0] < 0x2) as u64;
                                let t = c.g[V0][0] == 0;
                                c.g[V0][0] = 0;
                                if t {
                                    break 'done;
                                }
                                break 'case_a24;
                            }
                            c.cycles += 1;
                            c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                            break 'slot;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S7][0] as i64) < 5) as u64;
                        let t = c.g[V0][0] != 0;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0015_7980);
                            }
                            continue 'inner;
                        }
                        break 'entry;
                    }
                    c.cycles += 4;
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S6][0] as i64) < 4) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0xa));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0015_7970);
                        }
                        continue 'outer;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                }
                break 'outer;
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BERewardInt_UnlockVideo(self, profile, character, index, give)`
/// (0x158470): mark the video reward owned, and when `give` is set charge
/// its price (the reward record's halfword at +0xc, times 10) from the money.
pub fn be_reward_int_unlock_video(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[T0][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A3][0];
        call_ptr(c, 0x0014_dc80, 0x0015_84a8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x0014_dd58, 0x0015_84b4)?;
        c.g[V1][0] = 0xf88;
        c.g[A2][0] = 0x9b50;
        c.mult(0, 3, 19, 3);
        c.g[S5][0] = c.g[V0][0];
        c.mult(0, 6, 18, 6);
        c.g[V0][0] = 0x4a0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xf52));
        call_ptr(c, 0x0015_73f0, 0x0015_84e8)?;
        let t = c.g[S1][0] == 0;
        c.g[A0][0] = c.g[S5][0];
        'unlock: {
            if t {
                break 'unlock;
            }
            'price: {
                c.cycles += 8;
                call_ptr(c, 0x0015_a478, 0x0015_84f8)?;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = sx((c.g[S4][0] as u32) << 4);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A3][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc));
                let t = (c.g[A3][0] as i64) <= 0;
                c.g[V0][0] = 0xa;
                if t {
                    break 'price;
                }
                c.cycles += 1;
                c.mult(0, 7, 7, 2);
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            call_ptr(c, 0x0015_0b48, 0x0015_8524)?;
            c.g[A0][0] = c.g[S5][0];
        }
        c.cycles += 12;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S3][0];
        call_ptr(c, 0x0015_0928, 0x0015_8534)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BERewardInt_UnlockRiderItem(self, profile, character, index, give)`
/// (0x158618): the rider-item twin of [`be_reward_int_unlock_video`]: the
/// index is `a3 - 10`, the set-bit offset is 0xf57 and the price record is
/// read at +0x28.
pub fn be_reward_int_unlock_rider_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[T0][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[S4][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xa));
        call_ptr(c, 0x0014_dc80, 0x0015_8650)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x0014_dd58, 0x0015_865c)?;
        c.g[V1][0] = 0xf88;
        c.g[A2][0] = 0x9b50;
        c.mult(0, 3, 19, 3);
        c.g[S5][0] = c.g[V0][0];
        c.mult(0, 6, 18, 6);
        c.g[V0][0] = 0x4a0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xf57));
        call_ptr(c, 0x0015_73f0, 0x0015_8690)?;
        let t = c.g[S1][0] == 0;
        c.g[A0][0] = c.g[S5][0];
        'unlock: {
            if t {
                break 'unlock;
            }
            'price: {
                c.cycles += 8;
                call_ptr(c, 0x0015_a478, 0x0015_86a0)?;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = sx((c.g[S4][0] as u32) << 4);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A3][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc));
                let t = (c.g[A3][0] as i64) <= 0;
                c.g[V0][0] = 0xa;
                if t {
                    break 'price;
                }
                c.cycles += 1;
                c.mult(0, 7, 7, 2);
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            call_ptr(c, 0x0015_0b48, 0x0015_86cc)?;
            c.g[A0][0] = c.g[S5][0];
        }
        c.cycles += 12;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S3][0];
        call_ptr(c, 0x0015_0928, 0x0015_86dc)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BERewardInt_AddPointsForRider(self, rider, points)` (0x1597b0):
/// resolve the rider's profile and character and add the points to them.
pub fn be_reward_int_add_points_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
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

/// `void BERewardInt_AddPoints(self, profile, character, points, tier)`
/// (0x159818): add the points to the money, and grant each peak medal
/// reward that the new total completes (up to three peaks).
pub fn be_reward_int_add_points(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S6, 0x20);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S5, 0x30);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save_ra(c, 0x10);
        'b_159978: {
            'b_159964: {
                'b_159960: {
                    c.g[S6][0] = c.g[A3][0];
                    call_ptr(c, 0x0014_dc80, 0x0015_9850)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0xb;
                    call_ptr(c, 0x0014_dd58, 0x0015_985c)?;
                    c.g[S5][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1e90));
                    c.g[V0][0] = 0x4c0000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3ef0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S6][0] as u32));
                    let t = c.g[A0][0] == 0;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x3ef0), c.g[V1][0]);
                    if t {
                        break 'b_159960;
                    }
                    c.cycles += 9;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x0015_09c0, 0x0015_9888)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1e90));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S5][0];
                    if t {
                        break 'b_159964;
                    }
                    c.cycles += 3;
                    c.g[S1][0] = 0;
                    c.g[S0][0] = c.g[SP][0];
                    'l_1598a8: loop {
                        c.cycles += 11;
                        c.g[A3][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S4][0];
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[T0][0] = 0x3;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        call_ptr(c, 0x0015_7bf0, 0x0015_98c4)?;
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        c.g[V0][0] = ((c.g[S1][0] as i64) < 3) as u64;
                        let t = c.g[V0][0] != 0;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0015_98a8);
                            }
                            continue 'l_1598a8;
                        }
                        c.cycles += 13;
                        c.g[A0][0] = c.g[S5][0];
                        c.g[A3][0] = c.g[S6][0];
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S3][0];
                        call_ptr(c, 0x0015_0a90, 0x0015_98e8)?;
                        c.g[S1][0] = 0;
                        c.g[A0][0] = c.g[S4][0];
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S3][0];
                        call_ptr(c, 0x0015_8be0, 0x0015_98fc)?;
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1e90), c.g[V0][0]);
                        c.g[S0][0] = c.g[SP][0];
                        'l_159908: loop {
                            'b_15994c: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                if c.g[V0][0] != 0 {
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                    break 'b_15994c;
                                }
                                'b_159948: {
                                    c.cycles += 8;
                                    c.g[A0][0] = c.g[S4][0];
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A2][0] = c.g[S3][0];
                                    c.g[A3][0] = c.g[S1][0];
                                    c.g[T0][0] = 0x3;
                                    call_ptr(c, 0x0015_7bf0, 0x0015_992c)?;
                                    let t = c.g[V0][0] == 0;
                                    c.g[A0][0] = c.g[S4][0];
                                    if t {
                                        break 'b_159948;
                                    }
                                    c.cycles += 5;
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A2][0] = c.g[S3][0];
                                    c.g[A3][0] = c.g[S1][0];
                                    c.g[T0][0] = 0x3;
                                    call_ptr(c, 0x0015_9170, 0x0015_9948)?;
                                    break 'b_159948;
                                }
                                c.cycles += 1;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 3) as u64;
                            let t = c.g[V0][0] != 0;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0015_9908);
                                }
                                continue 'l_159908;
                            }
                            c.cycles += 2;
                            frame.restore(c, S0, 0x80);
                            break 'b_159978;
                        }
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[S5][0];
                break 'b_159964;
            }
            c.cycles += 5;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S6][0];
            call_ptr(c, 0x0015_0a90, 0x0015_9974)?;
            frame.restore(c, S0, 0x80);
            break 'b_159978;
        }
        c.cycles += 9;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `gMedalsInfo[3][4][5]`: 2-byte `{event, location}` entries, the
/// location stride 2, the row stride 0xa and the table stride 0x28.
#[allow(dead_code)]
pub(crate) const BE_MEDALS_INFO: u32 = 0x0045_aad8;

/// `int cBERewardInterface_getTrackTrophyID(cBERewardInterface* self, int eventID, int locationID)`
/// (0x0015a3b8): the index `j` of the first medal entry whose event is
/// `eventID` and whose location is `locationID` (events 6..=11 match on the
/// event alone), searched over `i` (3), `j` (4) and `k` (5 entries, stopping
/// after 5 misses in a row of entries). `-1` when nothing matches.
///
/// Arguments: `a1` = eventID, `a2` = locationID. Scratch: `a0` (row
/// pointer), `a3` (misses in the current row), `t0` = j, `t1` = i, `v1`
/// (entry pointer). `t2..t8` hold the special event ids and the table base.
pub fn be_reward_int_find_requirement_tier_2(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0x0046_0000;
    c.g[T1][0] = 0;
    c.g[T8][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5528));
    c.g[T7][0] = 0xa;
    c.g[T6][0] = 0x6;
    c.g[T5][0] = 0x7;
    c.g[T4][0] = 0x8;
    c.g[T3][0] = 0x9;
    c.g[T2][0] = 0xb;
    c.g[V0][0] = 0x28;
    loop {
        // i: the table of three.
        c.cycles += 2;
        c.g[T0][0] = 0;
        c.mult(0, A0, T1, V0);
        'cols: loop {
            // j: the four rows of the table.
            c.cycles += 2;
            c.g[A3][0] = 0;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T8][0] as u32));
            loop {
                // k: the five entries of the row.
                if medal_entry_matches(c) {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[T0][0];
                    ret(c);
                    return;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A3][0] as i64) < 5i64) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0015_a3f0);
                }
            }
            // Next j.
            c.cycles += 4;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[T0][0] as i64) < 4i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa));
            if !more {
                break 'cols;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0015_a3e8);
            }
        }
        // Next i.
        c.cycles += 4;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = ((c.g[T1][0] as i64) < 3i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = 0x28;
        if !more {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0015_a3e0);
        }
    }
}

/// One entry of the k loop (the `'b_15a440` block): true when the entry at
/// `v1` is the one asked for. A mismatch on the event counts a miss in `a3`.
pub(crate) fn medal_entry_matches(c: &mut Cpu) -> bool {
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
    if c.g[V0][0] != c.g[A1][0] {
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
        return false;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1)).lb(c);
    if c.g[V0][0] == c.g[A2][0] {
        return true;
    }
    for reg in [T6, T5, T4, T3, T7] {
        c.cycles += 2;
        if c.g[A1][0] == c.g[reg][0] {
            return true;
        }
    }
    c.cycles += 2;
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
    c.g[A1][0] == c.g[T2][0]
}

/// `void BERewardInt_OnEventResult(self, profile, charID, mode, event, medal)`
/// (0x001591e8; reference `cBERewardInterface_calculateUserTrackMedalRewards`):
/// the reward bookkeeping after an event result. It looks up the character's
/// score record, the track's tier and peak group, the cash reward for the
/// medal, grants the "first medal" award (medal < 2, no earlier result) or the
/// peak foreshadow messages (medal 2/3 on mountain events), awards cash
/// (halved when the medal is no improvement) and the trophy when it became
/// owned, records the place, grants the mountain-conquered award, stores the
/// last medal and cash for the result screen and finally clears the character
/// flag bits (at `record + 0x278`) of the medal and goal checks of the five
/// tracks of the group's tier.
///
/// Registers: `a0` self, `a1` profile, `a2` charID, `a3` mode, `t0` event,
/// `t1` medal. The body is the generated one, every register, cycle and call
/// kept; resume entries (`pc` != entry) run the generated twin.
#[allow(
    unused_labels,
    unused_variables,
    unused_assignments,
    unused_mut,
    unreachable_code,
    clippy::all
)]
pub fn beint_reward_on_event_result(c: &mut Cpu) {
    if c.pc != 0x001591e8 {
        return ee::s01::f_001591e8__generated(c);
    }
    guest(c, |c| {
        'b_1592f4: {
            'b_1592f0: {
                'b_1592c4: {
                    'b_1592bc: {
                        c.cycles += 50;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                        c.g[V1][0] = 0xf88;
                        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[V0][0] = 0x4a0000;
                        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.g[S5][0] = c.g[A1][0];
                        c.g[S6][0] = c.g[A2][0];
                        c.g[A1][0] = 0x9b50;
                        c.mult(0, 3, 22, 3);
                        c.mult(0, 5, 21, 5);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.g[S1][0] = c.g[A3][0];
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.g[S2][0] = c.g[A0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[S4][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[S7][0] = c.g[T0][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S3][0] = c.g[T1][0];
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                        c.g[RA][0] = sx(0x00159254);
                        c.g[FP][0] = 0xffffffffffffffff;
                        Callee {
                            addr: 0x0014dc80,
                            f: ee::s01::BELibrary_GetInstance,
                        }
                        .call(c, 0x00159254)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00159260);
                        c.g[A1][0] = 0xb;
                        Callee {
                            addr: 0x0014dd58,
                            f: ee::s01::BELibrary_GetInterface,
                        }
                        .call(c, 0x00159260)?;
                        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                        c.g[A0][0] = c.g[S4][0];
                        c.g[A1][0] = c.g[S7][0];
                        c.g[RA][0] = sx(0x00159274);
                        c.g[A2][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00152398,
                            f: ee::s01::BE_GetCharProfileEventRecord,
                        }
                        .call(c, 0x00159274)?;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00159288);
                        c.g[A2][0] = c.g[S7][0];
                        Callee {
                            addr: 0x0015a3b8,
                            f: ee::s01::BERewardInt_FindRequirementTier,
                        }
                        .call(c, 0x00159288)?;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x00159298);
                        c.g[A1][0] = c.g[S7][0];
                        Callee {
                            addr: 0x0015a398,
                            f: ee::s01::BERewardInt_GetTrackPeak,
                        }
                        .call(c, 0x00159298)?;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[S0][0] = c.g[V0][0];
                        c.g[V0][0] = 0x6;
                        c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x1));
                        let t = c.g[S1][0] == c.g[V0][0];
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                        if t {
                            break 'b_1592bc;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x9;
                            let t = c.g[S1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x7;
                            if t {
                                break 'b_1592c4;
                            } else {
                                break 'b_1592bc;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = 0u64;
                    if t {
                        break 'b_1592f0;
                    } else {
                        break 'b_1592c4;
                    }
                }
                'b_1592dc: {
                    'b_1592d4: {
                        c.cycles += 2;
                        let t = c.g[S1][0] == c.g[V0][0];
                        c.g[V0][0] = 0xa;
                        if t {
                            break 'b_1592d4;
                        } else {
                            c.cycles += 2;
                            let t = c.g[S1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x8;
                            if t {
                                break 'b_1592dc;
                            } else {
                                break 'b_1592d4;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = 0x1;
                    if t {
                        break 'b_1592f0;
                    } else {
                        break 'b_1592dc;
                    }
                }
                'b_1592ec: {
                    c.cycles += 2;
                    let t = c.g[S1][0] == c.g[V0][0];
                    c.g[V0][0] = 0xb;
                    if t {
                        break 'b_1592ec;
                    } else {
                        c.cycles += 2;
                        let t = c.g[S1][0] != c.g[V0][0];
                        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        if t {
                            break 'b_1592f4;
                        } else {
                            break 'b_1592ec;
                        }
                    }
                }
                c.cycles += 1;
                c.g[S0][0] = 0x2;
                break 'b_1592f0;
            }
            c.cycles += 1;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            break 'b_1592f4;
        }
        'b_159350: {
            c.cycles += 8;
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S7][0];
            c.g[RA][0] = sx(0x00159304);
            c.g[A3][0] = c.g[S1][0];
            Callee {
                addr: 0x00151040,
                f: ee::s01::BERewardInt_GetEventRewardPoints,
            }
            .call(c, 0x00159304)?;
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.g[V0][0] = (c.g[S1][0] < 0xeu64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x460000;
            if t {
                break 'b_159350;
            } else {
                'b_15934c: {
                    'b_159344: {
                        'b_15933c: {
                            'b_159334: {
                                c.cycles += 6;
                                c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5610));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                let t = c.g[A0][0] as u32;
                                match t {
                                    0x0015932c => {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x11));
                                        if t {
                                            break 'b_159350;
                                        } else {
                                            break 'b_159334;
                                        }
                                    }
                                    0x00159334 => {
                                        break 'b_159334;
                                    }
                                    0x0015933c => {
                                        break 'b_15933c;
                                    }
                                    0x00159344 => {
                                        break 'b_159344;
                                    }
                                    0x0015934c => {
                                        break 'b_15934c;
                                    }
                                    0x00159350 => {
                                        break 'b_159350;
                                    }
                                    _ => {
                                        c.jump_out(t, 0x001591e8, 0x001597ac);
                                        return Ok(());
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                            if t {
                                break 'b_159350;
                            } else {
                                break 'b_15933c;
                            }
                        }
                        c.cycles += 2;
                        let t = true;
                        c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x17));
                        if t {
                            break 'b_159350;
                        } else {
                            break 'b_159344;
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1a));
                    if t {
                        break 'b_159350;
                    } else {
                        break 'b_15934c;
                    }
                }
                c.cycles += 1;
                c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d));
                break 'b_159350;
            }
        }
        'b_1593e0: {
            'b_159384: {
                c.cycles += 3;
                c.g[V0][0] = (c.g[S3][0] < 0x2u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                if t {
                    break 'b_159384;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_1593e0;
                    } else {
                        c.cycles += 7;
                        c.g[A3][0] = c.g[FP][0];
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S5][0];
                        c.g[RA][0] = sx(0x0015937c);
                        c.g[A2][0] = c.g[S6][0];
                        Callee {
                            addr: 0x00159cd0,
                            f: ee::s01::BERewardInt_GrantReward,
                        }
                        .call(c, 0x0015937c)?;
                        let t = true;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_1593e0;
                        } else {
                            break 'b_159384;
                        }
                    }
                }
            }
            'b_1593dc: {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x6));
                c.g[V0][0] = (c.g[V0][0] < 0x6u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x530000;
                if t {
                    break 'b_1593dc;
                } else {
                    c.cycles += 3;
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5f9));
                    let t = c.g[V1][0] != 0u64;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_1593e0;
                    } else {
                        'b_1593c4: {
                            c.cycles += 3;
                            c.g[V0][0] = 0x3;
                            let t = c.g[S3][0] != c.g[V0][0];
                            c.g[V0][0] = 0x2;
                            if t {
                                break 'b_1593c4;
                            } else {
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001593bc);
                                c.g[A2][0] = 0u64;
                                call_ptr(c, 0x001e1dd0, 0x001593bc)?;
                                let t = true;
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_1593e0;
                                } else {
                                    break 'b_1593c4;
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S3][0] != c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_1593e0;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001593dc);
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x001e1dd0, 0x001593dc)?;
                            break 'b_1593dc;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1593e0;
        }
        'b_1594b4: {
            'b_1594b0: {
                'b_159478: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001593ec);
                    c.g[A2][0] = c.g[S3][0];
                    Callee {
                        addr: 0x00156a60,
                        f: ee::s01::BERewardInt_IsBetterPlace,
                    }
                    .call(c, 0x001593ec)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    if t {
                        break 'b_159478;
                    } else {
                        c.cycles += 24;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S5][0];
                        c.g[A2][0] = c.g[S6][0];
                        c.g[RA][0] = sx(0x00159408);
                        c.g[A3][0] = c.g[S0][0];
                        Callee {
                            addr: 0x00157bf0,
                            f: ee::s01::BERewardInt_IsPeakGoalComplete,
                        }
                        .call(c, 0x00159408)?;
                        c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        c.g[FP][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S5][0];
                        c.g[RA][0] = sx(0x00159420);
                        c.g[A2][0] = c.g[S6][0];
                        Callee {
                            addr: 0x00159818,
                            f: ee::s01::BERewardInt_AddPoints,
                        }
                        .call(c, 0x00159420)?;
                        c.g[A1][0] = c.g[S7][0];
                        c.g[A2][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S4][0];
                        c.g[RA][0] = sx(0x00159434);
                        c.g[A3][0] = c.g[S3][0];
                        Callee {
                            addr: 0x00152528,
                            f: ee::s01::BE_RecordCharProfileEventPlace,
                        }
                        .call(c, 0x00159434)?;
                        c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S5][0];
                        c.g[A2][0] = c.g[S6][0];
                        c.g[RA][0] = sx(0x0015944c);
                        c.g[A3][0] = c.g[S0][0];
                        Callee {
                            addr: 0x00157bf0,
                            f: ee::s01::BERewardInt_IsPeakGoalComplete,
                        }
                        .call(c, 0x0015944c)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S2][0];
                        if t {
                            break 'b_1594b0;
                        } else {
                            c.cycles += 2;
                            let t = c.g[FP][0] != 0u64;
                            c.g[A1][0] = c.g[S5][0];
                            if t {
                                break 'b_1594b4;
                            } else {
                                c.cycles += 7;
                                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A2][0] = c.g[S6][0];
                                c.g[RA][0] = sx(0x00159470);
                                c.g[A3][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00159170,
                                    f: ee::s01::BERewardInt_GrantMedalReward,
                                }
                                .call(c, 0x00159470)?;
                                let t = true;
                                c.g[A0][0] = c.g[S2][0];
                                if t {
                                    break 'b_1594b0;
                                } else {
                                    break 'b_159478;
                                }
                            }
                        }
                    }
                }
                c.cycles += 14;
                c.g[A1][0] = c.g[S7][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x0015948c);
                c.g[A3][0] = c.g[S3][0];
                Callee {
                    addr: 0x00152528,
                    f: ee::s01::BE_RecordCharProfileEventPlace,
                }
                .call(c, 0x0015948c)?;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = c.g[S6][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.g[RA][0] = sx(0x001594ac);
                c.g[A3][0] = c.g[V0][0];
                Callee {
                    addr: 0x00159818,
                    f: ee::s01::BERewardInt_AddPoints,
                }
                .call(c, 0x001594ac)?;
                c.g[A0][0] = c.g[S2][0];
                break 'b_1594b0;
            }
            c.cycles += 1;
            c.g[A1][0] = c.g[S5][0];
            break 'b_1594b4;
        }
        'b_1594f4: {
            'b_1594f0: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001594bc);
                c.g[A2][0] = c.g[S6][0];
                Callee {
                    addr: 0x001577e0,
                    f: ee::s01::BERewardInt_IsPeakFullyCompleted,
                }
                .call(c, 0x001594bc)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S2][0];
                if t {
                    break 'b_1594f0;
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S5][0];
                    c.g[A2][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x001594d4);
                    c.g[A3][0] = 0u64;
                    Callee {
                        addr: 0x0015a2e0,
                        f: ee::s01::BERewardInt_IsRewardGranted,
                    }
                    .call(c, 0x001594d4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1e8c));
                    if t {
                        break 'b_1594f4;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S5][0];
                        c.g[A2][0] = c.g[S6][0];
                        c.g[RA][0] = sx(0x001594f0);
                        c.g[A3][0] = 0u64;
                        Callee {
                            addr: 0x00159cd0,
                            f: ee::s01::BERewardInt_GrantReward,
                        }
                        .call(c, 0x001594f0)?;
                        break 'b_1594f0;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1e8c));
            break 'b_1594f4;
        }
        'b_159780: {
            'b_15977c: {
                c.cycles += 9;
                c.g[A2][0] = c.g[S3][0];
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = 0xffffffffffffffff;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[S3][0]);
                c.g[RA][0] = sx(0x00159510);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                Callee {
                    addr: 0x00156a60,
                    f: ee::s01::BERewardInt_IsBetterPlace,
                }
                .call(c, 0x00159510)?;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                if t {
                    break 'b_15977c;
                } else {
                    c.cycles += 14;
                    c.g[V0][0] = 0xa;
                    c.g[A1][0] = 0x28;
                    c.g[FP][0] = 0u64;
                    c.mult(0, 2, 3, 2);
                    c.mult(0, 5, 16, 5);
                    c.g[V1][0] = 0x1;
                    c.g[S7][0] = 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5528));
                    c.g[S3][0] = c.g[V0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    'l_159550: loop {
                        'b_159634: {
                            'b_159590: {
                                c.cycles += 7;
                                c.g[V0][0] = 0x460000;
                                c.g[A3][0] = c.lb(c.g[S1][0] as u32);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5528));
                                c.g[V1][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                c.g[V0][0] = 0xffffffffffffffff;
                                let t = c.g[A3][0] != c.g[V0][0];
                                c.g[T0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_159590;
                                } else {
                                    c.cycles += 5;
                                    c.g[V1][0] = 0x530000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5b0));
                                    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                    if t {
                                        break 'b_159634;
                                    } else {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x00159588);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x001e1c10, 0x00159588)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                        if t {
                                            break 'b_159634;
                                        } else {
                                            break 'b_159590;
                                        }
                                    }
                                }
                            }
                            'b_159630: {
                                'b_159620: {
                                    'b_1595d0: {
                                        c.cycles += 4;
                                        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4));
                                        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A0][0] = c.g[S2][0];
                                        if t {
                                            break 'b_1595d0;
                                        } else {
                                            c.cycles += 10;
                                            c.g[A1][0] = c.g[S5][0];
                                            c.g[A2][0] = c.g[S6][0];
                                            c.g[RA][0] = sx(0x001595b0);
                                            c.g[FP][0] = 0x1;
                                            Callee {
                                                addr: 0x00158910,
                                                f: ee::s01::BERewardInt_GetEventBestPlace,
                                            }
                                            .call(c, 0x001595b0)?;
                                            c.g[A2][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x001595c0);
                                            c.g[A1][0] = 0xffffffffffffffff;
                                            Callee {
                                                addr: 0x00156a60,
                                                f: ee::s01::BERewardInt_IsBetterPlace,
                                            }
                                            .call(c, 0x001595c0)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[S7][0] =
                                                    sx((c.g[S7][0] as u32).wrapping_add(0x1));
                                                break 'b_159620;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                                if t {
                                                    break 'b_159630;
                                                } else {
                                                    break 'b_1595d0;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 9;
                                    c.g[A1][0] = c.g[S5][0];
                                    c.g[RA][0] = sx(0x001595dc);
                                    c.g[A2][0] = c.g[S6][0];
                                    Callee {
                                        addr: 0x00158910,
                                        f: ee::s01::BERewardInt_GetEventBestPlace,
                                    }
                                    .call(c, 0x001595dc)?;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001595ec);
                                    c.g[A1][0] = 0xffffffffffffffff;
                                    Callee {
                                        addr: 0x00156a60,
                                        f: ee::s01::BERewardInt_IsBetterPlace,
                                    }
                                    .call(c, 0x001595ec)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                                        break 'b_159620;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[FP][0] != 0u64;
                                        c.g[V1][0] = 0x530000;
                                        if t {
                                            break 'b_159630;
                                        } else {
                                            'b_159618: {
                                                c.cycles += 4;
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x5b0));
                                                c.g[V0][0] =
                                                    c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                                                if c.g[V0][0] != 0u64 {
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x14),
                                                        0u64,
                                                    );
                                                    break 'b_159618;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[RA][0] = sx(0x00159614);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x001e1c10, 0x00159614)?;
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x14),
                                                        0u64,
                                                    );
                                                    break 'b_159618;
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = true;
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                            if t {
                                                break 'b_159630;
                                            } else {
                                                break 'b_159620;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                                c.g[V0][0] = ((c.g[S7][0] as i64) < 5i64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00159550);
                                    }
                                    continue 'l_159550;
                                } else {
                                    break 'b_159630;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                            break 'b_159634;
                        }
                        'b_1596dc: {
                            'b_1596d8: {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_1596d8;
                                } else {
                                    'b_1596c4: {
                                        'b_1596b8: {
                                            'b_159680: {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != 0u64;

                                                if t {
                                                    break 'b_159680;
                                                } else {
                                                    'b_159658: {
                                                        c.cycles += 2;
                                                        let t = c.g[S0][0] != 0u64;
                                                        c.g[V0][0] = 0x1;
                                                        if t {
                                                            break 'b_159658;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.ld((c.g[S4][0] as u32)
                                                                .wrapping_add(0x278));
                                                            let t = true;
                                                            c.g[V1][0] = 0xffffffffffffffbf;
                                                            if t {
                                                                break 'b_1596b8;
                                                            } else {
                                                                break 'b_159658;
                                                            }
                                                        }
                                                    }
                                                    'b_15966c: {
                                                        c.cycles += 2;
                                                        let t = c.g[S0][0] != c.g[V0][0];
                                                        c.g[V0][0] = 0x2;
                                                        if t {
                                                            break 'b_15966c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.ld((c.g[S4][0] as u32)
                                                                .wrapping_add(0x278));
                                                            let t = true;
                                                            c.g[V1][0] = 0xffffffffffffff7f;
                                                            if t {
                                                                break 'b_1596b8;
                                                            } else {
                                                                break 'b_15966c;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = c.g[S0][0] != c.g[V0][0];
                                                    c.g[V0][0] = 0x530000;
                                                    if t {
                                                        break 'b_1596c4;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .ld((c.g[S4][0] as u32)
                                                                .wrapping_add(0x278));
                                                        let t = true;
                                                        c.g[V1][0] = 0xfffffffffffffeff;
                                                        if t {
                                                            break 'b_1596b8;
                                                        } else {
                                                            break 'b_159680;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_159694: {
                                                c.cycles += 2;
                                                let t = c.g[S0][0] != 0u64;
                                                c.g[V0][0] = 0x1;
                                                if t {
                                                    break 'b_159694;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.ld(
                                                            (c.g[S4][0] as u32).wrapping_add(0x278)
                                                        );
                                                    let t = true;
                                                    c.g[V1][0] = 0xfffffffffffffdff;
                                                    if t {
                                                        break 'b_1596b8;
                                                    } else {
                                                        break 'b_159694;
                                                    }
                                                }
                                            }
                                            'b_1596a8: {
                                                c.cycles += 2;
                                                let t = c.g[S0][0] != c.g[V0][0];
                                                c.g[V0][0] = 0x2;
                                                if t {
                                                    break 'b_1596a8;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.ld(
                                                            (c.g[S4][0] as u32).wrapping_add(0x278)
                                                        );
                                                    let t = true;
                                                    c.g[V1][0] = 0xfffffffffffffbff;
                                                    if t {
                                                        break 'b_1596b8;
                                                    } else {
                                                        break 'b_1596a8;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = c.g[S0][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x530000;
                                            if t {
                                                break 'b_1596c4;
                                            } else {
                                                c.cycles += 2;
                                                c.g[V0][0] =
                                                    c.ld((c.g[S4][0] as u32).wrapping_add(0x278));
                                                c.g[V1][0] = 0xfffffffffffff7ff;
                                                break 'b_1596b8;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                        c.sd((c.g[S4][0] as u32).wrapping_add(0x278), c.g[V0][0]);
                                        c.g[V0][0] = 0x530000;
                                        break 'b_1596c4;
                                    }
                                    c.cycles += 3;
                                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5f9));
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                    if t {
                                        break 'b_1596dc;
                                    } else {
                                        c.cycles += 2;
                                        c.g[RA][0] = sx(0x001596d8);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x001e2370, 0x001596d8)?;
                                        break 'b_1596d8;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                            break 'b_1596dc;
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_15977c;
                        } else {
                            'b_159774: {
                                'b_159730: {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != 0u64;

                                    if t {
                                        break 'b_159730;
                                    } else {
                                        'b_159700: {
                                            c.cycles += 2;
                                            let t = c.g[S0][0] != 0u64;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_159700;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.ld((c.g[S4][0] as u32).wrapping_add(0x278));
                                                let t = true;
                                                c.g[V1][0] = 0xffffffffffffbfff;
                                                if t {
                                                    break 'b_159774;
                                                } else {
                                                    break 'b_159700;
                                                }
                                            }
                                        }
                                        'b_159718: {
                                            c.cycles += 2;
                                            let t = c.g[S0][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_159718;
                                            } else {
                                                c.cycles += 4;
                                                c.g[V1][0] = 0xffffffffffff0000;
                                                c.g[V1][0] = c.g[V1][0] | 0x7fff;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.ld((c.g[S4][0] as u32).wrapping_add(0x278));
                                                if t {
                                                    break 'b_159774;
                                                } else {
                                                    break 'b_159718;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[S0][0] != c.g[V0][0];
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                        if t {
                                            break 'b_159780;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V1][0] = 0xfffffffffffe0000;
                                            c.g[V1][0] = c.g[V1][0] | 0xffff;
                                            let t = true;
                                            c.g[V0][0] =
                                                c.ld((c.g[S4][0] as u32).wrapping_add(0x278));
                                            if t {
                                                break 'b_159774;
                                            } else {
                                                break 'b_159730;
                                            }
                                        }
                                    }
                                }
                                'b_159748: {
                                    c.cycles += 2;
                                    let t = c.g[S0][0] != 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_159748;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V1][0] = 0xfffffffffffd0000;
                                        c.g[V1][0] = c.g[V1][0] | 0xffff;
                                        let t = true;
                                        c.g[V0][0] = c.ld((c.g[S4][0] as u32).wrapping_add(0x278));
                                        if t {
                                            break 'b_159774;
                                        } else {
                                            break 'b_159748;
                                        }
                                    }
                                }
                                'b_159760: {
                                    c.cycles += 2;
                                    let t = c.g[S0][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x2;
                                    if t {
                                        break 'b_159760;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V1][0] = 0xfffffffffffb0000;
                                        c.g[V1][0] = c.g[V1][0] | 0xffff;
                                        let t = true;
                                        c.g[V0][0] = c.ld((c.g[S4][0] as u32).wrapping_add(0x278));
                                        if t {
                                            break 'b_159774;
                                        } else {
                                            break 'b_159760;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S0][0] != c.g[V0][0];
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                if t {
                                    break 'b_159780;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.ld((c.g[S4][0] as u32).wrapping_add(0x278));
                                    c.g[V1][0] = 0xfffffffffff70000;
                                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                                    break 'b_159774;
                                }
                            }
                            c.cycles += 2;
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            c.sd((c.g[S4][0] as u32).wrapping_add(0x278), c.g[V0][0]);
                            break 'b_15977c;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            break 'b_159780;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

/// Gate word of this module's fakes (in the machine's RAM).
#[cfg(test)]
pub(crate) const OC677_CTRL: u32 = crate::difftest::DATA + 0xf_e000;

#[cfg(test)]
pub(crate) const OC677_MAGIC: u32 = 0x0677_a11e;

/// Scripted results of the fakes (by call index & 31).
#[cfg(test)]
pub(crate) const OC677_RET: u32 = OC677_CTRL + 0x10;

/// The score-stat record `BE_GetCharProfileEventRecord` is faked to return.
#[cfg(test)]
pub(crate) const OC677_STATS: u32 = crate::difftest::DATA + 0x2800;

/// `void BERewardInt_LoadRewardsFile(void)` (0x156a10): (re)load the rewards
/// database (`0x4c3e98`) through `RewardsFile_Load(db, 1)`.
///
/// The real loader runs away on zeroed test RAM (the `a1 == 1` branch walks the
/// bolt-file equip table), so its test fakes `RewardsFile_Load` (gated). Two
/// older dead copies of this function sit in `game/be/beintreward.rs`
/// (`be_reward_int_load_rewards_file`, `..._2`): delete them at the reorg.
pub fn be_reward_int_load_rewards_file_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x004c_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3e98));
        Callee {
            addr: 0x0015_ad58,
            f: ee::s01::RewardsFile_Load,
        }
        .call(c, 0x0015_6a28)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BERewardInt_PickRandomBolt` (0x00156c70): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn be_reward_int_pick_random_bolt(c: &mut Cpu) {
    if c.pc != 0x00156c70 {
        return ee::s01::f_00156c70__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00156c70;
        loop {
            match pc {
                0x00156c70 => {
                    c.cycles += 30;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.g[A0][0] = 0xf88;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = 0x9b50;
                    c.g[S3][0] = c.g[A2][0];
                    c.mult(0, 5, 5, 2);
                    c.mult(0, 4, 19, 4);
                    c.g[V1][0] = 0x4a0000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S2][0] = c.g[A3][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[RA][0] = sx(0x00156cc0);
                    c.g[S0][0] = 0u64;
                    Callee {
                        addr: 0x0014bdb8,
                        f: ee::s01::Bolts_GetDB,
                    }
                    .call(c, 0x00156cc0)?;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00156cd0);
                    c.g[A0][0] = c.g[S4][0];
                    Callee {
                        addr: 0x0014d988,
                        f: ee::s01::Bolts_GetCharBoltCount,
                    }
                    .call(c, 0x00156cd0)?;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[S5][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00156ce0);
                    c.g[A1][0] = c.g[S3][0];
                    Callee {
                        addr: 0x0014d998,
                        f: ee::s01::Bolts_GetCharBolts,
                    }
                    .call(c, 0x00156ce0)?;
                    let t = (c.g[S5][0] as i64) <= 0;
                    c.g[A0][0] = c.g[V0][0];
                    if t {
                        pc = 0x00156dac;
                        continue;
                    }
                    pc = 0x00156ce8;
                    continue;
                }
                0x00156ce8 => {
                    c.cycles += 4;
                    c.g[A3][0] = 0x1;
                    c.g[A2][0] = 0x2;
                    c.g[A1][0] = c.g[S5][0];
                    pc = 0x00156cf8;
                    continue;
                }
                0x00156cf8 => {
                    c.cycles += 2;
                    let t = c.g[S2][0] != 0u64;

                    if t {
                        pc = 0x00156d34;
                        continue;
                    }
                    pc = 0x00156d00;
                    continue;
                }
                0x00156d00 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x35));
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        pc = 0x00156d34;
                        continue;
                    }
                    pc = 0x00156d10;
                    continue;
                }
                0x00156d10 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                    pc = 0x00156d14;
                    continue;
                }
                0x00156d14 => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x288));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        pc = 0x00156d80;
                        continue;
                    }
                    pc = 0x00156d2c;
                    continue;
                }
                0x00156d2c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00156d90;
                        continue;
                    }
                    pc = 0x00156d34;
                    continue;
                }
                0x00156d34 => {
                    c.cycles += 2;
                    let t = c.g[S2][0] != c.g[A3][0];

                    if t {
                        pc = 0x00156d4c;
                        continue;
                    }
                    pc = 0x00156d3c;
                    continue;
                }
                0x00156d3c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] = c.g[V0][0] & 0x200;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                        if back_edge_unwinds(c, 0x00156d14) {
                            return Ok(());
                        }
                        pc = 0x00156d14;
                        continue;
                    }
                    pc = 0x00156d4c;
                    continue;
                }
                0x00156d4c => {
                    c.cycles += 2;

                    if c.g[S2][0] != c.g[A2][0] {
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        pc = 0x00156da4;
                        continue;
                    }
                    pc = 0x00156d54;
                    continue;
                }
                0x00156d54 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] = c.g[V0][0] & 0x400;

                    if c.g[V0][0] == 0u64 {
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        pc = 0x00156da4;
                        continue;
                    }
                    pc = 0x00156d64;
                    continue;
                }
                0x00156d64 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x288));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        pc = 0x00156d8c;
                        continue;
                    }
                    pc = 0x00156d80;
                    continue;
                }
                0x00156d80 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                    let t = true;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        pc = 0x00156d90;
                        continue;
                    }
                    pc = 0x00156d8c;
                    continue;
                }
                0x00156d8c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x00156d90;
                    continue;
                }
                0x00156d90 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.g[V0][0] & 0x2;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[V1][0];
                    }
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    pc = 0x00156da4;
                    continue;
                }
                0x00156da4 => {
                    c.cycles += 2;
                    let t = c.g[A1][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
                    if t {
                        if back_edge_unwinds(c, 0x00156cf8) {
                            return Ok(());
                        }
                        pc = 0x00156cf8;
                        continue;
                    }
                    pc = 0x00156dac;
                    continue;
                }
                0x00156dac => {
                    c.cycles += 2;
                    let t = (c.g[S0][0] as i64) <= 0;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x00156ebc;
                        continue;
                    }
                    pc = 0x00156db4;
                    continue;
                }
                0x00156db4 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00156dbc);
                    call_ptr(c, 0x003177f0, 0x00156dbc)?;
                    c.divu(0, 2, 16);

                    if c.g[S0][0] == 0u64 {
                        pc = 0x00156dc8;
                        continue;
                    }
                    pc = 0x00156dc8;
                    continue;
                }
                0x00156dc8 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[V1][0] = c.hi[0];
                    c.g[RA][0] = sx(0x00156ddc);
                    c.g[S0][0] = c.g[V1][0];
                    Callee {
                        addr: 0x0014d998,
                        f: ee::s01::Bolts_GetCharBolts,
                    }
                    .call(c, 0x00156ddc)?;
                    c.g[A0][0] = c.g[V0][0];
                    let t = (c.g[S5][0] as i64) <= 0;
                    c.g[A2][0] = 0u64;
                    if t {
                        pc = 0x00156eb8;
                        continue;
                    }
                    pc = 0x00156de8;
                    continue;
                }
                0x00156de8 => {
                    c.cycles += 2;
                    c.g[T0][0] = 0x1;
                    c.g[A3][0] = 0x2;
                    pc = 0x00156df0;
                    continue;
                }
                0x00156df0 => {
                    c.cycles += 2;
                    let t = c.g[S2][0] != 0u64;

                    if t {
                        pc = 0x00156e2c;
                        continue;
                    }
                    pc = 0x00156df8;
                    continue;
                }
                0x00156df8 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x35));
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        pc = 0x00156e2c;
                        continue;
                    }
                    pc = 0x00156e08;
                    continue;
                }
                0x00156e08 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                    pc = 0x00156e0c;
                    continue;
                }
                0x00156e0c => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x288));
                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        pc = 0x00156e78;
                        continue;
                    }
                    pc = 0x00156e24;
                    continue;
                }
                0x00156e24 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00156e88;
                        continue;
                    }
                    pc = 0x00156e2c;
                    continue;
                }
                0x00156e2c => {
                    c.cycles += 2;
                    let t = c.g[S2][0] != c.g[T0][0];

                    if t {
                        pc = 0x00156e44;
                        continue;
                    }
                    pc = 0x00156e34;
                    continue;
                }
                0x00156e34 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] = c.g[V0][0] & 0x200;

                    if c.g[V0][0] != 0u64 {
                        c.g[A1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                        if back_edge_unwinds(c, 0x00156e0c) {
                            return Ok(());
                        }
                        pc = 0x00156e0c;
                        continue;
                    }
                    pc = 0x00156e44;
                    continue;
                }
                0x00156e44 => {
                    c.cycles += 2;

                    if c.g[S2][0] != c.g[A3][0] {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        pc = 0x00156eac;
                        continue;
                    }
                    pc = 0x00156e4c;
                    continue;
                }
                0x00156e4c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] = c.g[V0][0] & 0x400;

                    if c.g[V0][0] == 0u64 {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        pc = 0x00156eac;
                        continue;
                    }
                    pc = 0x00156e5c;
                    continue;
                }
                0x00156e5c => {
                    c.cycles += 7;
                    c.g[A1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x288));
                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        pc = 0x00156e84;
                        continue;
                    }
                    pc = 0x00156e78;
                    continue;
                }
                0x00156e78 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                    let t = true;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        pc = 0x00156e88;
                        continue;
                    }
                    pc = 0x00156e84;
                    continue;
                }
                0x00156e84 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x00156e88;
                    continue;
                }
                0x00156e88 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = c.g[V0][0] & 0x2;

                    if c.g[V0][0] != 0u64 {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        pc = 0x00156eac;
                        continue;
                    }
                    pc = 0x00156e98;
                    continue;
                }
                0x00156e98 => {
                    c.cycles += 3;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));

                    if (c.g[S0][0] as i64) >= 0 {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        pc = 0x00156eac;
                        continue;
                    }
                    pc = 0x00156ea4;
                    continue;
                }
                0x00156ea4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.g[A1][0];
                    if t {
                        pc = 0x00156ebc;
                        continue;
                    }
                    pc = 0x00156eac;
                    continue;
                }
                0x00156eac => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[S5][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
                    if t {
                        if back_edge_unwinds(c, 0x00156df0) {
                            return Ok(());
                        }
                        pc = 0x00156df0;
                        continue;
                    }
                    pc = 0x00156eb8;
                    continue;
                }
                0x00156eb8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xffffffffffffffff;
                    pc = 0x00156ebc;
                    continue;
                }
                0x00156ebc => {
                    c.cycles += 9;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00156c70, 0x00156ee0);
                    return Ok(());
                }
            }
        }
    });
}

/// `BERewardInt_PickRandomLockedReward` (0x00157080): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn be_reward_int_pick_random_locked_reward(c: &mut Cpu) {
    if c.pc != 0x00157080 {
        return ee::s01::f_00157080__generated(c);
    }
    guest(c, |c| {
        'b_1571ec: {
            c.cycles += 25;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S0][0] = c.g[A0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A3][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S5][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S4][0] = c.g[A2][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x001570b8);
            c.g[A1][0] = c.g[S1][0];
            Callee {
                addr: 0x00157210,
                f: ee::s01::BERewardInt_GetRewardTypeCount,
            }
            .call(c, 0x001570b8)?;
            c.g[S3][0] = 0xffffffffffffffff;
            c.g[S2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A2][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x001570d4);
            c.g[A3][0] = c.g[S1][0];
            Callee {
                addr: 0x00157518,
                f: ee::s01::BERewardInt_CountUnlocked,
            }
            .call(c, 0x001570d4)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffffff;
            if t {
                break 'b_1571ec;
            } else {
                'b_157110: {
                    c.cycles += 11;
                    c.g[RA][0] = sx(0x001570ec);
                    call_ptr(c, 0x003177f0, 0x001570ec)?;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                    c.g[A0][0] = 0xf88;
                    c.divu(0, 2, 3);
                    c.g[A1][0] = 0x9b50;
                    c.g[V0][0] = 0x4a0000;
                    c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x3));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                    if c.g[V1][0] == 0u64 {
                        break 'b_157110;
                    } else {
                        break 'b_157110;
                    }
                }
                'b_157170: {
                    c.cycles += 8;
                    c.g[T0][0] = 0u64;
                    c.g[V1][0] = (c.g[A3][0] < 0x5u64) as u64;
                    c.g[A2][0] = c.hi[0];
                    c.mult(0, 4, 20, 4);
                    c.mult(0, 5, 21, 5);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = c.g[V1][0] == 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    if t {
                        break 'b_157170;
                    } else {
                        'b_15716c: {
                            'b_157164: {
                                'b_15715c: {
                                    'b_157154: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x56b0));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x0015714c => {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[T0][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0xf30));
                                                if t {
                                                    break 'b_157170;
                                                } else {
                                                    break 'b_157154;
                                                }
                                            }
                                            0x00157154 => {
                                                break 'b_157154;
                                            }
                                            0x0015715c => {
                                                break 'b_15715c;
                                            }
                                            0x00157164 => {
                                                break 'b_157164;
                                            }
                                            0x0015716c => {
                                                break 'b_15716c;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x00157080, 0x00157210);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf36));
                                    if t {
                                        break 'b_157170;
                                    } else {
                                        break 'b_15715c;
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf45));
                                if t {
                                    break 'b_157170;
                                } else {
                                    break 'b_157164;
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf52));
                            if t {
                                break 'b_157170;
                            } else {
                                break 'b_15716c;
                            }
                        }
                        c.cycles += 1;
                        c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf53));
                        break 'b_157170;
                    }
                }
                c.cycles += 2;
                c.g[A1][0] = 0u64;
                'l_157178: loop {
                    'b_1571e0: {
                        'b_1571a8: {
                            c.cycles += 8;
                            c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            c.g[A0][0] = 0u64;
                            c.g[V0][0] = c.lb(c.g[V1][0] as u32);
                            c.g[A3][0] = c.g[V1][0];
                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                            c.g[V0][0] = c.g[V0][0] & 0x1;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                            if t {
                                break 'b_1571a8;
                            } else {
                                c.cycles += 2;
                                if c.g[A2][0] != 0u64 {
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                    break 'b_1571a8;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[S3][0] = sx((c.g[A1][0] as u32) << 3);
                                    if t {
                                        break 'b_1571e0;
                                    } else {
                                        break 'b_1571a8;
                                    }
                                }
                            }
                        }
                        'l_1571a8: loop {
                            c.cycles += 1;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            'l_1571ac: loop {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A0][0] as i64) < 8i64) as u64;
                                let t = c.g[V0][0] == 0u64;

                                if t {
                                    break 'b_1571e0;
                                } else {
                                    c.cycles += 6;
                                    c.g[V0][0] = c.lb(c.g[A3][0] as u32);
                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32
                                        >> ((c.g[A0][0] as u32) & 31))
                                        as u32);
                                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x001571ac);
                                        }
                                        continue 'l_1571ac;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[A2][0] != 0u64 {
                                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x001571a8);
                                            }
                                            continue 'l_1571a8;
                                        } else {
                                            c.cycles += 2;
                                            c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
                                            c.g[S3][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            break 'b_1571e0;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = (c.g[S3][0] as i64) < 0;
                    c.g[A1][0] = c.g[V1][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00157178);
                        }
                        continue 'l_157178;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S3][0];
                        break 'b_1571ec;
                    }
                }
            }
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `BERewardInt_UnlockReward` (0x00157f20): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn be_reward_int_unlock_reward(c: &mut Cpu) {
    if c.pc != 0x00157f20 {
        return ee::s01::f_00157f20__generated(c);
    }
    guest(c, |c| {
        'b_157fc8: {
            'b_157fc0: {
                c.cycles += 8;
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x3));
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[A3][0] = c.g[T0][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[T2][0] = c.g[A0][0];
                c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[T0][0] = c.g[T1][0];
                if t {
                    break 'b_157fc0;
                } else {
                    'b_157fac: {
                        'b_157f9c: {
                            'b_157f8c: {
                                'b_157f7c: {
                                    'b_157f6c: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5630));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x00157f5c => {
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x00157f64);
                                                c.g[A0][0] = c.g[T2][0];
                                                Callee {
                                                    addr: 0x00157fd0,
                                                    f: ee::s01::BERewardInt_UnlockPoster,
                                                }
                                                .call(c, 0x00157f64)?;
                                                let t = true;
                                                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                                if t {
                                                    break 'b_157fc8;
                                                } else {
                                                    break 'b_157f6c;
                                                }
                                            }
                                            0x00157f6c => {
                                                break 'b_157f6c;
                                            }
                                            0x00157f7c => {
                                                break 'b_157f7c;
                                            }
                                            0x00157f8c => {
                                                break 'b_157f8c;
                                            }
                                            0x00157f9c => {
                                                break 'b_157f9c;
                                            }
                                            0x00157fac => {
                                                break 'b_157fac;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x00157f20, 0x00157fd0);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x00157f74);
                                    c.g[A0][0] = c.g[T2][0];
                                    Callee {
                                        addr: 0x001580f8,
                                        f: ee::s01::BERewardInt_UnlockTradingCard,
                                    }
                                    .call(c, 0x00157f74)?;
                                    let t = true;
                                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                    if t {
                                        break 'b_157fc8;
                                    } else {
                                        break 'b_157f7c;
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x00157f84);
                                c.g[A0][0] = c.g[T2][0];
                                Callee {
                                    addr: 0x00158220,
                                    f: ee::s01::BERewardInt_UnlockConceptArt,
                                }
                                .call(c, 0x00157f84)?;
                                let t = true;
                                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                if t {
                                    break 'b_157fc8;
                                } else {
                                    break 'b_157f8c;
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x00157f94);
                            c.g[A0][0] = c.g[T2][0];
                            Callee {
                                addr: 0x00158470,
                                f: ee::s01::BERewardInt_UnlockVideo,
                            }
                            .call(c, 0x00157f94)?;
                            let t = true;
                            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                            if t {
                                break 'b_157fc8;
                            } else {
                                break 'b_157f9c;
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x00157fa4);
                        c.g[A0][0] = c.g[T2][0];
                        Callee {
                            addr: 0x00158348,
                            f: ee::s01::BERewardInt_UnlockToy,
                        }
                        .call(c, 0x00157fa4)?;
                        let t = true;
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        if t {
                            break 'b_157fc8;
                        } else {
                            break 'b_157fac;
                        }
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[T2][0];
                    c.g[RA][0] = sx(0x00157fb8);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0xa));
                    Callee {
                        addr: 0x00158618,
                        f: ee::s01::BERewardInt_UnlockRiderItem,
                    }
                    .call(c, 0x00157fb8)?;
                    let t = true;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if t {
                        break 'b_157fc8;
                    } else {
                        break 'b_157fc0;
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0u64;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            break 'b_157fc8;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::generic;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_6;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_8;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_7;

    pub(crate) use crate::game::fe::festaterewards::tests::strict;

    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::w;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2;
    pub(crate) use crate::game::main::gamestate::tests::base_3;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_3;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_2;

    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;

    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_5;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;

    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn be_reward_int_is_rewards_file_loaded_matches() {
        check(
            ssx3_ee::s01::f_00156a38,
            be_reward_int_is_rewards_file_loaded,
            300,
            &|c, seed| {
                let mut r = Rng(0xd000_000e ^ seed as u64);
                scramble(c, &mut r);
                if seed % 2 == 0 {
                    c.bus.write32(0x4c3e98, r.next_u32());
                } else {
                    c.bus.write32(0x4c3e98, 0xffff_ffff);
                }
                let open = match seed % 3 {
                    0 => 0,
                    1 => r.next_u32() | 0x8000_0000,
                    _ => r.next_u32() & 0x7fff_ffff,
                };
                c.bus.write32(0x4c3e9c, open);
            },
        );
    }

    #[test]
    fn be_reward_int_is_better_place_matches() {
        check(
            ssx3_ee::s01::f_00156a60,
            be_reward_int_is_better_place,
            300,
            &|c, seed| {
                let mut r = Rng(0xe000_000f ^ seed as u64);
                scramble(c, &mut r);
                c.g[A2][0] = match seed % 3 {
                    0 => u64::MAX,
                    _ => ((r.next_u32() as u64) << 32) | r.next_u32() as u64,
                };
                c.g[A1][0] = match seed % 4 {
                    0 => u64::MAX,
                    _ => ((r.next_u32() as u64) << 32) | r.next_u32() as u64,
                };
            },
        );
    }

    #[test]
    fn be_reward_int_set_bit_matches() {
        check(
            ssx3_ee::s01::f_001573f0,
            be_reward_int_set_bit,
            300,
            &|c, seed| {
                let mut r = Rng(0xf000_0010 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A2][0] = DATA as u64;
                let bit = (seed % 32) as u64;
                // Odd seeds are negative as a whole 64-bit value, so the
                // "index < 0" adjustment path is exercised too.
                c.g[A1][0] = if seed % 2 == 0 {
                    bit
                } else {
                    0x8000_0000_0000_0000u64 | bit
                };
                let byte_addr = DATA + ((c.g[A1][0] as u32) >> 3);
                c.bus.write8(byte_addr, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn be_reward_int_test_bit_matches() {
        check(
            ssx3_ee::s01::f_00157430,
            be_reward_int_test_bit,
            300,
            &|c, seed| {
                let mut r = Rng(0x1_0000_0011u64 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A2][0] = DATA as u64;
                let bit = (seed % 32) as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    bit
                } else {
                    0x8000_0000_0000_0000u64 | bit
                };
                let byte_addr = DATA + ((c.g[A1][0] as u32) >> 3);
                c.bus.write8(byte_addr, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn be_reward_int_get_collectible_threshold_matches() {
        check(
            ssx3_ee::s01::f_00158870,
            be_reward_int_get_collectible_threshold,
            200,
            &|c, seed| {
                let mut r = Rng(0x1_1000_0012u64 ^ seed as u64);
                scramble(c, &mut r);
                let a = seed % 4;
                let b = seed % 4;
                c.g[A1][0] = a as u64;
                c.g[A2][0] = b as u64;
                let addr = 0x45afe8u32.wrapping_add(b * 4).wrapping_add(a * 16);
                c.bus.write32(addr, r.next_u32());
            },
        );
    }

    #[test]
    fn be_reward_int_get_mission_threshold_matches() {
        check(
            ssx3_ee::s01::f_00158890,
            be_reward_int_get_mission_threshold,
            200,
            &|c, seed| {
                let mut r = Rng(0x1_2000_0013u64 ^ seed as u64);
                scramble(c, &mut r);
                let a = seed % 4;
                let b = seed % 4;
                c.g[A1][0] = a as u64;
                c.g[A2][0] = b as u64;
                let addr = 0x45b018u32.wrapping_add(b * 4).wrapping_add(a * 16);
                c.bus.write32(addr, r.next_u32());
            },
        );
    }

    #[test]
    fn be_reward_int_get_new_rewards_matches() {
        check(
            ssx3_ee::s01::f_00158f20,
            be_reward_int_get_new_rewards,
            20,
            &|c, seed| {
                let mut r = Rng(0x1_3000_0014u64 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn be_reward_int_get_track_peak_matches() {
        check_lohi(
            ssx3_ee::s01::f_0015a398,
            be_reward_int_get_track_peak,
            200,
            &|c, seed| {
                let mut r = Rng(0x15a3 ^ seed as u64);
                scramble(c, &mut r);
                let index = (r.next_u32() % 8) as u32;
                c.g[A1][0] = index as u64;
                c.bus.write32(
                    TRACK_RECORDS
                        .wrapping_sub(0x26b0)
                        .wrapping_add(index.wrapping_mul(100))
                        .wrapping_add(0x54),
                    r.next_u32(),
                );
            },
        );
    }

    #[test]
    fn be_reward_int_get_rewards_file_matches() {
        check(
            ssx3_ee::s01::f_0015a478,
            be_reward_int_get_rewards_file,
            32,
            &|c, seed| {
                let mut r = Rng(0x15a4 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn be_reward_int_get_new_reward_count_matches() {
        check_lohi(
            ssx3_ee::s01::f_0015a750,
            be_reward_int_get_new_reward_count,
            200,
            &|c, seed| {
                let mut r = Rng(0x15a7 ^ seed as u64);
                scramble(c, &mut r);
                let a1 = (r.next_u32() % 8) as u32;
                let a2 = (r.next_u32() % 8) as u32;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = a1 as u64;
                c.g[A2][0] = a2 as u64;
                for k in 0..0x40 {
                    c.bus.write8(DATA + k, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn be_reward_int_get_new_reward_bits_matches() {
        check(
            ssx3_ee::s01::f_0015a768,
            be_reward_int_get_new_reward_bits,
            200,
            &|c, seed| {
                let mut r = Rng(0x15a8 ^ seed as u64);
                scramble(c, &mut r);
                let a1 = (r.next_u32() % 8) as u32;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = a1 as u64;
                for k in 0..0x10 {
                    c.bus.write32(DATA + 0x170 + k * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn rewards_file_ctor_matches() {
        check(
            ssx3_ee::s01::f_0015a778,
            rewards_file_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x15a9 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// The rewards file itself: point the global at test data.
    pub(crate) fn rewards_file(c: &mut Cpu, r: &mut Rng) {
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
            (
                ssx3_ee::s01::f_00156b70 as GuestFn,
                be_reward_int_get_section0 as GuestFn,
                0x8u32,
                21u64,
            ),
            (
                ssx3_ee::s01::f_00156b90,
                be_reward_int_get_section1,
                0xc,
                22,
            ),
            (
                ssx3_ee::s01::f_00156bb0,
                be_reward_int_get_section2,
                0x10,
                23,
            ),
            (
                ssx3_ee::s01::f_00156bd0,
                be_reward_int_get_poster_table,
                0x14,
                24,
            ),
            (
                ssx3_ee::s01::f_00156bf0,
                be_reward_int_get_trading_card_table,
                0x18,
                25,
            ),
            (
                ssx3_ee::s01::f_00156c10,
                be_reward_int_get_concept_art_table,
                0x1c,
                26,
            ),
            (
                ssx3_ee::s01::f_00156c30,
                be_reward_int_get_toy_table,
                0x20,
                27,
            ),
            (
                ssx3_ee::s01::f_00156c50,
                be_reward_int_get_video_table,
                0x24,
                28,
            ),
        ] {
            let _ = off;
            check(gen, hand, 120, &|c, seed| {
                let mut r = Rng(salt << 8 ^ seed as u64);
                scramble(c, &mut r);
                rewards_file(c, &mut r);
            });
        }
    }

    #[test]
    fn be_rewards_counts_match() {
        for (gen, hand, salt) in [
            (
                ssx3_ee::s01::f_001572b0 as GuestFn,
                be_reward_int_get_poster_count as GuestFn,
                31u64,
            ),
            (
                ssx3_ee::s01::f_001572d0,
                be_reward_int_get_trading_card_count,
                32,
            ),
            (
                ssx3_ee::s01::f_001572f0,
                be_reward_int_get_concept_art_count,
                33,
            ),
            (ssx3_ee::s01::f_00157310, be_reward_int_get_toy_count, 34),
            (ssx3_ee::s01::f_00157330, be_reward_int_get_video_count, 35),
            (
                ssx3_ee::s01::f_00157370,
                be_reward_int_get_cheat_char_count,
                36,
            ),
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
            (
                ssx3_ee::s01::f_00156a90 as GuestFn,
                be_reward_int_get_section0_entry as GuestFn,
                41u64,
            ),
            (
                ssx3_ee::s01::f_00156ae0,
                be_reward_int_get_section1_entry,
                42,
            ),
            (
                ssx3_ee::s01::f_00156b28,
                be_reward_int_get_section2_entry,
                43,
            ),
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
        check(
            ssx3_ee::s01::f_00157390,
            be_reward_int_count_set_bits,
            200,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn be_reward_clear_new_rewards_matches() {
        check(
            ssx3_ee::s01::f_0015a510,
            be_reward_int_clear_new_rewards,
            60,
            &|c, seed| {
                let mut r = Rng(0xa510_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                for i in 0..0x60 {
                    c.bus.write32(DATA + 0x1000 + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn be_reward_predicates_match() {
        fn pred(generated: GuestFn, hand: GuestFn, extra: u32, salt: u64, rider: bool) {
            check(generated, hand, 120, &|c, seed| {
                let mut r = Rng(salt << 8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[A3][0] = if rider {
                    (seed % 200 + 10) as u64
                } else {
                    (seed % 200) as u64
                };
                c.g[V1][0] = (seed % 0x100) as u64;
                let base = REWARD_BASE + extra;
                for i in 0..64 {
                    c.bus.write8(base + i, r.next_u32() as u8);
                }
            });
        }
        pred(
            ssx3_ee::s01::f_00157d60,
            be_reward_int_is_poster_unlocked,
            0xf30,
            71,
            false,
        );
        pred(
            ssx3_ee::s01::f_00157da0,
            be_reward_int_is_trading_card_unlocked,
            0xf36,
            72,
            false,
        );
        pred(
            ssx3_ee::s01::f_00157de0,
            be_reward_int_is_concept_art_unlocked,
            0xf45,
            73,
            false,
        );
        pred(
            ssx3_ee::s01::f_00157e20,
            be_reward_int_is_toy_unlocked,
            0xf53,
            74,
            false,
        );
        pred(
            ssx3_ee::s01::f_00157e60,
            be_reward_int_is_video_unlocked,
            0xf52,
            75,
            false,
        );
        pred(
            ssx3_ee::s01::f_00157ee0,
            be_reward_int_is_rider_item_unlocked,
            0xf57,
            76,
            true,
        );
        predeg(
            ssx3_ee::s01::f_0015a2e0,
            be_reward_int_is_reward_granted,
            0xf28,
        );
    }

    pub(crate) fn predeg(generated: GuestFn, hand: GuestFn, extra: u32) {
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
        counts(
            ssx3_ee::s01::f_001575c0,
            be_reward_int_count_unlocked_posters,
            0xf30,
            51,
            false,
        );
        counts(
            ssx3_ee::s01::f_00157620,
            be_reward_int_count_unlocked_trading_cards,
            0xf36,
            52,
            false,
        );
        counts(
            ssx3_ee::s01::f_00157680,
            be_reward_int_count_unlocked_concept_arts,
            0xf45,
            53,
            false,
        );
        counts(
            ssx3_ee::s01::f_001576e0,
            be_reward_int_count_unlocked_toys,
            0xf53,
            54,
            false,
        );
        counts(
            ssx3_ee::s01::f_00157740,
            be_reward_int_count_unlocked_videos,
            0xf52,
            55,
            false,
        );
        counts(
            ssx3_ee::s01::f_001577a0,
            be_reward_int_count_unlocked_rider_items,
            0xf57,
            56,
            true,
        );
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

    /// The reward record for `a1 = player`, `a2 = slot`.
    pub(crate) fn reward_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = Rng(salt ^ (seed as u64) << 8);
        crate::difftest::scramble(c, &mut r);
        let player = seed % 4;
        let slot = seed % 8;
        c.g[A1][0] = player as u64;
        c.g[A2][0] = slot as u64;
        c.g[A3][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        let rec = 0x004a_6ca8u32 + player * 0x9b50 + slot * 0xf88;
        c.bus.write32(rec + 0xf80, r.next_u32());
        c.bus.write64(
            rec + 0xf70,
            (r.next_u32() as u64) << 32 | r.next_u32() as u64,
        );
        c.bus.write64(
            rec + 0xf78,
            (r.next_u32() as u64) << 32 | r.next_u32() as u64,
        );
    }

    #[test]
    fn be_reward_int_get_audio_setting_matches() {
        check(
            ee::s01::f_00158700__generated,
            be_reward_int_get_audio_setting,
            120,
            &|c, seed| reward_setup(c, seed, 0x1587_0000),
        );
    }

    #[test]
    fn be_reward_int_get_unlocked_songs_matches() {
        check(
            ee::s01::f_00158728__generated,
            be_reward_int_get_unlocked_songs,
            120,
            &|c, seed| reward_setup(c, seed, 0x1587_0028),
        );
    }

    #[test]
    fn be_reward_int_get_enabled_songs_matches() {
        check(
            ee::s01::f_00158750__generated,
            be_reward_int_get_enabled_songs,
            120,
            &|c, seed| reward_setup(c, seed, 0x1587_0050),
        );
    }

    #[test]
    fn be_reward_int_set_audio_setting_matches() {
        check(
            ee::s01::f_001587f8__generated,
            be_reward_int_set_audio_setting,
            120,
            &|c, seed| reward_setup(c, seed, 0x1587_00f8),
        );
    }

    #[test]
    fn be_reward_int_set_unlocked_songs_matches() {
        check(
            ee::s01::f_00158820__generated,
            be_reward_int_set_unlocked_songs,
            120,
            &|c, seed| reward_setup(c, seed, 0x1587_0120),
        );
    }

    #[test]
    fn be_reward_int_set_enabled_songs_matches() {
        check(
            ee::s01::f_00158848__generated,
            be_reward_int_set_enabled_songs,
            120,
            &|c, seed| reward_setup(c, seed, 0x1587_0148),
        );
    }

    pub(crate) fn bit_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let idx: [u64; 9] = [0, 1, 2, 7, 8, 63, 255, u64::MAX, 0xffff_ffff];
        c.g[A1][0] = idx[seed as usize % idx.len()];
        c.g[A2][0] = sx(DATA + 0x2000);
        for i in 0..64u32 {
            c.bus.write8(DATA + 0x2000 + i, r.next_u32() as u8);
        }
    }

    #[test]
    fn reward_test_granted_bit_matches() {
        check(
            ssx3_ee::s01::f_0015a320__generated,
            reward_test_granted_bit,
            SEEDS,
            &|c, seed| {
                bit_setup(c, seed, 0xa320);
            },
        );
    }

    #[test]
    fn reward_new_rewards_test_bit_matches() {
        check(
            ssx3_ee::s01::f_0015a5f0__generated,
            reward_new_rewards_test_bit,
            SEEDS,
            &|c, seed| {
                bit_setup(c, seed, 0xa5f0);
            },
        );
    }

    pub(crate) fn reward_setup_2(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x15a3_58, 0);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = (r.next_u32() % 64) as u64;
        let bits = DATA + 0x800;
        c.g[A2][0] = bits as u64;
        for k in 0..8u32 {
            c.bus.write8(bits + k, r.next_u32() as u8);
        }
    }

    #[test]
    fn be_reward_int_set_granted_bit_matches() {
        check(
            ssx3_ee::s01::f_0015a358__generated,
            be_reward_int_set_granted_bit,
            64,
            &reward_setup_2,
        );
    }

    #[test]
    fn be_reward_int_new_rewards_set_bit_matches() {
        check(
            ssx3_ee::s01::f_0015a5b0__generated,
            be_reward_int_new_rewards_set_bit,
            64,
            &reward_setup_2,
        );
    }

    #[test]
    fn be_reward_int_count_enabled_songs_matches() {
        check(
            ssx3_ee::s01::f_001587b8__generated,
            be_reward_int_count_enabled_songs,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0d, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = (seed % 3) as u64;
                c.bus.write32(DATA + 0x200, r.next_u32());
            },
        );
    }

    #[test]
    fn rewards_file_dtor_matches() {
        check(
            ssx3_ee::s01::f_0015a7d0__generated,
            rewards_file_dtor,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15a7d0, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                // A non-negative tag runs the unload path; the nested pointer
                // (+4) stays null, so only the outer object is deleted.
                c.bus.write32(this_, r.next_u32() & 0x7fff_ffff);
                c.bus.write32(this_ + 4, 0);
                c.bus.write32(this_ + 0x4c, r.next_u32());
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x158960 BERewardInt_GetEventRecordValue.

    #[test]
    fn be_reward_int_get_event_record_value_matches() {
        check(
            ee::s01::f_00158960__generated,
            be_reward_int_get_event_record_value,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x1589_60, 1);
                c.g[A0][0] = (seed % 10) as u64;
                c.g[A1][0] = (seed % 7) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[A3][0] = if seed % 2 == 0 { 8 } else { 0x20 }; // record index / "missing"
                c.g[T0][0] = 20; // >= 17: the cheap index path
                c.g[T1][0] = (seed % 3) as u64;
                c.bus.write32(0x004a_6ca8, 1);
            },
        );
    }

    #[test]
    fn be_reward_int_is_rival_reward_pending_matches() {
        check(
            ee::s01::f_00157b08__generated,
            be_reward_int_is_rival_reward_pending,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x15_7b08);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = (seed % 4) as u64;
                // The profile table stores offsets from 0x5305b20, not pointers.
                for i in 0..3u32 {
                    let rec = DATA + 0x100 + 0x40 * i;
                    c.bus
                        .write32(0x0053_05b0 + 4 * i, rec.wrapping_sub(0x0530_5b20));
                    c.bus
                        .write32(rec + 0xc, if (seed + i) % 2 == 0 { 0xffff_ffff } else { 0 });
                    c.bus
                        .write32(rec + 0x10, if (seed + i) % 3 == 0 { 1 } else { 0 });
                    c.bus.write8(rec + 0x11, (r.next_u32() & 7) as u8);
                }
            },
        );
    }

    /// The dummy world at `gp-0x848` with the race-manager chain, so rider
    /// loops terminate.
    pub(crate) fn world(c: &mut Cpu) {
        let wld = DATA + 0x1000;
        w(c, GAME_GP.wrapping_sub(0x848), wld);
        w(c, wld + 0x84, DATA + 0x1200);
        w(c, DATA + 0x1200 + 0xc, DATA + 0x1400);
        w(c, DATA + 0x1400 + 0x7c, 0);
    }

    #[test]
    fn be_reward_int_grant_medal_reward_matches() {
        check(
            ee::s01::f_00159170__generated,
            be_reward_int_grant_medal_reward,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x159170, 3);
                be_library_2(c);
                world(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[A3][0] = (seed % 3) as u64;
                for i in 0..0x400u32 {
                    w(
                        c,
                        0x004a_6ca8 + 4 * i,
                        (seed.wrapping_mul(2654435761) ^ i) & 0x3,
                    );
                }
            },
        );
    }

    #[test]
    fn be_reward_int_are_all_peaks_complete_matches() {
        check(
            ee::s01::f_001578a0__generated,
            be_reward_int_are_all_peaks_complete,
            6,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1578_a0, 6);
                let self_ = DATA;
                let profile = DATA + 0x1000;
                let ctx = DATA + 0x2000;
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = profile as u64;
                c.g[A2][0] = ctx as u64;
                let _ = r.next_u32();
            },
        );
    }

    /// The BE library table and index global.
    pub(crate) const BE_LIBRARY: u32 = 0x0053_08a8;

    pub(crate) const BE_LIBRARY_INDEX_GP: u32 = 0x1efc;

    // -----------------------------------------------------------------------
    // Rewards, bolts.

    #[test]
    fn berewardint_is_rival_reward_pending_for_matches() {
        check(
            ee::s01::f_00157b70__generated,
            berewardint_is_rival_reward_pending_for,
            SEEDS_3,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x7b70, 0);
                let a = (r.next_u32() % 4) as u64;
                let b = (r.next_u32() % 4) as u64;
                c.g[A1][0] = a;
                c.g[A2][0] = b;
                c.g[A3][0] = (seed % 4) as u64;
                let off = (b as u32).wrapping_mul(0xf88) + (a as u32).wrapping_mul(0x9b50);
                let addr = 0x004a_6ca8u32.wrapping_add(off).wrapping_add(0x278);
                c.bus
                    .write64(addr, (r.next_u32() as u64) << 12 | (seed as u64));
            },
        );
    }

    #[test]
    fn berewardint_has_points_tier_matches() {
        covered(
            ee::s01::f_00158af0__generated,
            berewardint_has_points_tier,
            SEEDS_3,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x8af0, 3);
                let index = (seed % 4) as u32;
                c.bus.write32(GAME_GP - BE_LIBRARY_INDEX_GP, index);
                let lib = ptr(&mut r);
                c.bus.write32(BE_LIBRARY + 4 * index, lib);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(lib + 4, vt);
                let a = ((seed / 4) % 3) as u64;
                let b = ((seed / 12) % 3) as u64;
                let medal = ((seed / 36) % 4) as u32;
                c.g[A1][0] = a;
                c.g[A2][0] = b;
                c.g[A3][0] = medal as u64;
                let off = (b as u32).wrapping_mul(0xf88) + (a as u32).wrapping_mul(0x9b50);
                c.bus.write32(0x004a_6ca8 + off + 0xac8, r.next_u32());
                c.bus.write32(
                    0x0045_b048 + 4 * medal,
                    if seed % 2 == 0 { 0 } else { 0xffff_ffff },
                );
            },
        );
    }

    #[test]
    fn be_reward_int_new_rewards_ctor_matches() {
        check(
            ee::s01::f_0015a488__generated,
            be_reward_int_new_rewards_ctor,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x15a488, 1);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn be_reward_int_is_peak_complete_matches() {
        check(
            ee::s01::f_00157a78__generated,
            be_reward_int_is_peak_complete,
            120,
            &|c, seed| {
                let _ = base_4(c, seed, 0x15_7a78);
                globals(c);
                // A zeroed BE_2 reward: the goal check only reads it.
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(seed ^ 1);
                c.g[A2][0] = sx(seed ^ 2);
                c.g[A3][0] = sx(seed ^ 3);
            },
        );
    }

    #[test]
    fn be_reward_int_get_next_points_threshold_matches() {
        check(
            ssx3_ee::s01::f_00158be0__generated,
            be_reward_int_get_next_points_threshold,
            12,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x0015_8be0);
                let reward = DATA + 0x100;
                clear(c, reward, 0x100);
                for i in 0..3u32 {
                    w(c, 0x0045_b048 + 4 * i, r.next_u32() % 100000);
                }
                c.g[A0][0] = sx(reward);
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (seed % 7) as u64;
            },
        );
    }

    #[test]
    fn be_reward_int_get_collectible_tier_matches() {
        check(
            ssx3_ee::s01::f_001589b0__generated,
            be_reward_int_get_collectible_tier,
            10,
            &|c, seed| {
                let _r = base_6(c, seed, 0x1589b0, 3);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 6) as u64;
                c.g[A2][0] = ((seed / 6) % 6) as u64;
                c.g[A3][0] = (seed % 4) as u64;
                for i in 0..16u32 {
                    w_2(c, 0x0045_afe8 + 4 * i, i % 5);
                }
            },
        );
    }

    #[test]
    fn be_reward_int_get_mission_tier_matches() {
        check(
            ssx3_ee::s01::f_00158a50__generated,
            be_reward_int_get_mission_tier,
            10,
            &|c, seed| {
                let _r = base_6(c, seed, 0x158a50, 3);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 6) as u64;
                c.g[A2][0] = ((seed / 6) % 6) as u64;
                c.g[A3][0] = (seed % 4) as u64;
                for i in 0..16u32 {
                    w_2(c, 0x0045_b018 + 4 * i, i % 5);
                }
            },
        );
    }

    // --------------------------------------------------------- memory card

    #[test]
    fn bereward_int_get_medal_reward_id_matches() {
        strict(
            ee::s01::f_00159080__generated,
            bereward_int_get_medal_reward_id,
            16,
            &|c, seed| {
                let _ = base_7(c, seed, 0x1590, 0);
                c.g[A1][0] = seed as u64;
                c.g[T0][0] = (seed % 6) as u64;
                c.g[A3][0] = ((seed / 6) % 4) as u64;
            },
        );
    }

    #[test]
    fn be_reward_int_grant_rival_rewards_matches() {
        same(
            ssx3_ee::s01::f_00158f60__generated,
            be_reward_int_grant_rival_rewards,
            200,
            &|c, seed| {
                let mut r = Rng(0x158f60 ^ seed as u64);
                generic(c, seed, 0x10);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 3) as u64;
                c.g[A3][0] = (seed % 2) as u64;
                c.g[T0][0] = (seed % 5) as u64;
                for profile in 0..4u32 {
                    for ch in 0..3u32 {
                        let p = 0x004a_6ca8 + profile * 0x9b50 + ch * 0xf88 + 0x278;
                        c.bus.write32(p, r.next_u32());
                        c.bus.write32(p + 4, r.next_u32());
                    }
                }
            },
        );
    }

    pub(crate) const REWARD: u32 = DATA + 0x1_0000;

    #[test]
    fn be_reward_int_is_peak_goal_complete_matches() {
        check_all(
            ee::s01::f_00157bf0__generated,
            be_reward_int_is_peak_goal_complete,
            30,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x157b_f0, 12);
                c.g[A0][0] = sx(REWARD);
                c.g[A1][0] = sx(seed % 3);
                c.g[A2][0] = sx((seed / 3) % 3);
                c.g[A3][0] = sx((seed / 9) % 4);
                c.g[T0][0] = sx(seed % 5); // the medal kind
                                           // The five-track table stays empty, so the loop starts on track 0
                                           // and runs until the medal comparison fails.
                c.bus.write32(REWARD + 0x54, r.next_u32() % 8);
            },
        );
    }

    pub(crate) const REWARD2: u32 = DATA + 0x1_0000;

    #[test]
    fn be_reward_int_on_mission_completed_matches() {
        check_all(
            ee::s01::f_00159b08__generated,
            be_reward_int_on_mission_completed,
            24,
            &|c, seed| {
                let _ = base_8(c, seed, 0x159b_08, 14);
                c.g[A0][0] = sx(REWARD2);
                c.g[A1][0] = sx(seed % 3);
                c.g[A2][0] = sx((seed / 3) % 4);
                // The stored mission threshold at_3 `D_00440F58[k]`.
                c.bus.write32(
                    0x0044_0f58 + 4 * (((seed / 3) % 4) as u32),
                    if seed % 2 == 0 { 0 } else { seed % 5 },
                );
                c.bus.write32(REWARD2 + 0x54, seed % 4);
            },
        );
    }

    pub(crate) const REWARD3: u32 = DATA + 0x1_0000;

    #[test]
    fn be_reward_int_on_collectible_collected_matches() {
        check_all(
            ee::s01::f_001599a0__generated,
            be_reward_int_on_collectible_collected,
            24,
            &|c, seed| {
                let _ = base_8(c, seed, 0x1599_a0, 14);
                c.g[A0][0] = sx(REWARD3);
                c.g[A1][0] = sx(seed % 3);
                c.g[A2][0] = sx((seed / 3) % 4);
                c.bus.write32(
                    0x0044_0f48 + 4 * (((seed / 3) % 4) as u32),
                    if seed % 2 == 0 { 0 } else { seed % 5 },
                );
                c.bus.write32(REWARD3 + 0x54, seed % 4);
            },
        );
    }

    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::RA as SENTINEL;

    /// Fixture: a rewards file at `DATA + 0x1000` (nine headers in random
    /// type order, some empty or out of range, 0..3 entries each with
    /// random zero / non-zero string offsets), `self` at `DATA`.
    pub(crate) fn rewards_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x15a8_1800 ^ (seed as u64) << 8);
        scramble(c, &mut r);
        let handlers = [
            0x15a85cu32,
            0x15a874,
            0x15a88c,
            0x15a8a0,
            0x15a8b4,
            0x15a8c8,
            0x15a8dc,
            0x15a8f0,
            0x15a904,
        ];
        for (k, h) in handlers.iter().enumerate() {
            c.bus.write32(REWARDS_JUMP_TABLE + 4 * k as u32, *h);
        }
        let mut order: Vec<u32> = (0..9).collect();
        for i in (1..9).rev() {
            order.swap(i, r.next_u32() as usize % (i + 1));
        }
        let data = DATA + 0x1000;
        let mut p = data;
        for n in 0..9 {
            let mut ty = order[n];
            if seed % 4 == 1 && r.next_u32() % 3 == 0 {
                ty = 9 + r.next_u32() % 3;
            }
            let count = match r.next_u32() % 4 {
                0 => 0,
                k => k,
            };
            c.bus.write32(p, ty);
            c.bus.write32(p + 4, count);
            p += 8;
            let stride = match ty {
                0 | 1 => 12,
                2 => 8,
                _ => 16,
            };
            if count != 0 && ty < 9 {
                for w in 0..count * stride / 4 {
                    let v = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 0x100
                    };
                    c.bus.write32(p + 4 * w, v);
                }
                p += count * stride;
            }
        }
        // the word before the string pool
        c.bus.write32(p, 0x1234);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(data);
    }

    /// `RewardsFile_Parse`: random section orders and counts, out-of-range
    /// types, zero and non-zero string offsets (LO/HI too).
    #[test]
    fn rewards_file_parse_matches() {
        check_lohi(
            ee::s01::f_0015a818__generated,
            rewards_file_parse,
            300,
            &|c, seed| {
                rewards_fixture(c, seed);
                c.pc = 0x0015_a818;
            },
        );
    }

    /// Event delivery at the parse loop and the nine fixup loop heads.
    #[test]
    fn rewards_file_parse_events_match() {
        check_events(
            ee::s01::f_0015a818__generated,
            rewards_file_parse,
            240,
            &|c, seed| {
                rewards_fixture(c, seed);
                let mut r = Rng(0x15a8_1801 ^ seed as u64);
                let mode = schedule(c, &mut r, seed, 0x0015_a818);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[RA][0] = SENTINEL as u64;
                mode
            },
        );
    }

    pub(crate) use crate::game::main::ssxapp::tests::oc429_force;
    pub(crate) use crate::game::main::ssxapp::tests::oc429_machine;

    pub(crate) use crate::game::main::ssxapp::tests::oc429_w;

    /// Setup of the two `BERewardInt_GetNext*Threshold` tests: the table (4
    /// rows of 4 thresholds), the tier getter and `IsBetterPlace` as fakes
    /// with forced answers.
    pub(crate) fn oc429_threshold_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32, table: u32) {
        let mut r = oc429_machine(c, seed, salt, 5, entry);
        for i in 0..16u32 {
            // Positive thresholds; a few zero ones.
            let v = if r.next_u32() % 6 == 0 {
                0
            } else {
                r.next_u32() % 500
            };
            oc429_w(c, table + 4 * i, v);
        }
        let better: Vec<u64> = (0..4).map(|_| (r.next_u32() & 1) as u64).collect();
        oc429_force(c, 0x0015_6a60, &better);
        c.g[A0][0] = r.next_u32() as u64;
        c.g[A1][0] = r.next_u32() as u64;
        c.g[A2][0] = r.next_u32() as u64;
        c.g[A3][0] = (r.next_u32() % 4) as u64;
    }

    #[test]
    fn be_reward_int_get_next_mission_threshold_matches() {
        covered(
            ee::s01::f_00158d58__generated,
            be_reward_int_get_next_mission_threshold,
            120,
            &|c, seed| oc429_threshold_setup(c, seed, 0x158d58, 0x0015_8d58, 0x0045_b018),
        );
    }

    #[test]
    fn be_reward_int_get_next_collectible_threshold_matches() {
        covered(
            ee::s01::f_00158c80__generated,
            be_reward_int_get_next_collectible_threshold,
            120,
            &|c, seed| oc429_threshold_setup(c, seed, 0x158c80, 0x0015_8c80, 0x0045_afe8),
        );
    }

    use crate::difftest::fakes;

    pub(crate) use crate::game::ai::ai::tests::oc434_setup;
    pub(crate) use crate::game::ai::ai::tests::oc434_w;

    /// `BERewardInt_BeginRewardTracking`: all callees are fakes (ten calls).
    #[test]
    fn be_reward_int_begin_reward_tracking_matches() {
        fakes::covered(
            ee::s01::f_00158e30__generated,
            be_reward_int_begin_reward_tracking,
            24,
            &|c, seed| {
                let _ = oc434_setup(c, seed, 0x158e30, 10, 0x0015_8e30);
                c.g[A0][0] = (DATA + 0x100) as u64;
                oc434_w(c, GAME_GP.wrapping_sub(0x1e8c), DATA + 0x800);
            },
        );
    }

    use crate::difftest::fill;

    #[test]
    fn be_reward_int_get_audio_system_field504_matches() {
        check(
            ee::s01::f_00157350__generated,
            be_reward_int_get_audio_system_field504,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7350, 0);
                fill(c, &mut r, DATA + 0x3000, 0x400);
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3000);
            },
        );
    }

    use crate::difftest::elf_image;

    /// `cBELibrary_profileLib` (`[profile][character]`, 0x9b50 per profile,
    /// 0xf88 per character).
    pub(crate) const PROFILE_LIB: u32 = 0x004a_6ca8;

    pub(crate) const PROFILE_STRIDE: u32 = 0x9b50;

    pub(crate) const CHAR_STRIDE: u32 = 0xf88;

    /// Fixture: the game image (jump table, slot tables, `.lit4`), the back
    /// end singletons at gp-0x1f04 / gp-0x1f28 as zeroed objects, and a
    /// character record whose award bits and owned-reward bits are random,
    /// so the pickers scan real bit arrays.
    pub(crate) fn grant_world(c: &mut Cpu, r: &mut Rng, profile: u32, character: u32) {
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
        check(
            ee::s01::f_00158348__generated,
            reward_unlock_toy,
            120,
            &|c, seed| {
                let _ = unlock_fixture(c, seed, 0x158348, 0x20);
                unlock_inputs(c, seed, 0x0015_8348, 0);
            },
        );
    }

    #[test]
    fn reward_unlock_song_matches() {
        check(
            ee::s01::f_00158558__generated,
            reward_unlock_song,
            120,
            &|c, seed| {
                let _ = unlock_fixture(c, seed, 0x158558, 0x24);
                unlock_inputs(c, seed, 0x0015_8558, 0);
                // The bit index runs past 63 on some seeds (the shift masks it).
                c.g[A3][0] = (seed as u64).wrapping_mul(7) % 70;
            },
        );
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

    /// The BE library dummy the reward code reaches through `cBE_getBE`.
    pub(crate) fn be_library_3(c: &mut Cpu) {
        c.bus
            .write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    }

    #[test]
    fn be_reward_int_pick_random_bolt_flag800_matches() {
        check(
            ssx3_ee::s01::f_00156ee0__generated,
            be_reward_int_pick_random_bolt_flag800,
            120,
            &|c, seed| {
                let mut r = Rng(0x6ee0_0000 ^ seed as u64);
                machine(c, seed, 0x156ee0, 0);
                let ch = seed % 8;
                let player = (seed / 8) % 4;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = player as u64;
                c.g[A2][0] = ch as u64;
                // Bolts_GetDB is the constant 0x4a6750: count, base and index.
                let db = 0x004a_6750u32;
                c.bus.write32(db + ch * 4 + 0x2c, seed % 6);
                c.bus.write32(db + ch * 4 + 0xa4, (seed / 3) % 4);
                c.bus.write32(db + 4, DATA + 0x20_0000);
                fill(c, &mut r, DATA + 0x20_0000, 0x400);
                let rec = 0x004a_6ca8 + player * 0x9b50 + ch * 0xf88;
                c.bus.write32(rec + 0x288, DATA + 0x40_0000);
                fill(c, &mut r, DATA + 0x40_0000, 0x400);
            },
        );
    }

    #[test]
    fn be_reward_int_is_peak_gold_complete_matches() {
        check(
            ssx3_ee::s01::f_00157920__generated,
            be_reward_int_is_peak_gold_complete,
            80,
            &|c, seed| {
                let mut r = Rng(0x7920_0000 ^ seed as u64);
                machine(c, seed, 0x157920, 0);
                be_library_3(c);
                c.g[A0][0] = (DATA + 0x20_0000) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = ((seed / 4) % 8) as u64;
                c.g[A3][0] = ((seed / 32) % 8) as u64;
                // The peak tables at fp + 0x45aad8 (signed-byte entries).
                fill(c, &mut r, 0x0045_a000, 0x200);
                fill(c, &mut r, DATA + 0x20_0000, 0x1000);
            },
        );
    }

    #[test]
    fn be_reward_int_unlock_video_matches() {
        check(
            ssx3_ee::s01::f_00158470__generated,
            be_reward_int_unlock_video,
            80,
            &|c, seed| {
                let mut r = Rng(0x8470_0000 ^ seed as u64);
                machine(c, seed, 0x158470, 0);
                be_library_3(c);
                c.g[A0][0] = (DATA + 0x20_0000) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = ((seed / 4) % 8) as u64;
                c.g[A3][0] = ((seed / 32) % 6) as u64;
                c.g[T0][0] = (seed % 2) as u64;
                fill(c, &mut r, DATA + 0x20_0000, 0x1000);
                fill(c, &mut r, 0x004a_6ca8, 0x2000);
            },
        );
    }

    #[test]
    fn be_reward_int_unlock_rider_item_matches() {
        check(
            ssx3_ee::s01::f_00158618__generated,
            be_reward_int_unlock_rider_item,
            80,
            &|c, seed| {
                let mut r = Rng(0x8618_0000 ^ seed as u64);
                machine(c, seed, 0x158618, 0);
                be_library_3(c);
                c.g[A0][0] = (DATA + 0x20_0000) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = ((seed / 4) % 8) as u64;
                c.g[A3][0] = (10 + (seed / 32) % 6) as u64;
                c.g[T0][0] = (seed % 2) as u64;
                fill(c, &mut r, DATA + 0x20_0000, 0x1000);
                fill(c, &mut r, 0x004a_6ca8, 0x2000);
            },
        );
    }

    #[test]
    fn be_reward_int_add_points_for_rider_matches() {
        check(
            ssx3_ee::s01::f_001597b0__generated,
            be_reward_int_add_points_for_rider,
            80,
            &|c, seed| {
                let mut r = Rng(0x97b0_0000 ^ seed as u64);
                machine(c, seed, 0x1597b0, 0);
                be_library_3(c);
                c.bus.write32(GAME_GP.wrapping_sub(0x1e90), seed % 40);
                c.g[A0][0] = (DATA + 0x20_0000) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 1000) as u64;
                c.g[A3][0] = (seed % 2) as u64;
                fill(c, &mut r, DATA + 0x20_0000, 0x1000);
            },
        );
    }

    #[test]
    fn be_reward_int_add_points_matches() {
        check(
            ssx3_ee::s01::f_00159818__generated,
            be_reward_int_add_points,
            80,
            &|c, seed| {
                let mut r = Rng(0x9818_0000 ^ seed as u64);
                machine(c, seed, 0x159818, 0);
                be_library_3(c);
                c.bus.write32(GAME_GP.wrapping_sub(0x1e90), seed % 40);
                c.g[A0][0] = (DATA + 0x20_0000) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = ((seed / 4) % 3) as u64;
                c.g[A3][0] = ((seed / 12) % 5) as u64;
                fill(c, &mut r, DATA + 0x20_0000, 0x1000);
            },
        );
    }

    #[test]
    fn be_reward_int_find_requirement_tier_matches() {
        check(
            ee::s01::f_0015a3b8__generated,
            be_reward_int_find_requirement_tier_2,
            300,
            &|c, seed| {
                let mut r = Rng(0x2f5a_3b80 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0015_a3b8;
                // A table of small event ids (6..=11 and the others) and
                // location ids, so every branch is reached.
                for n in 0..60u32 {
                    let ev = (r.next_u32() % 14) as u8;
                    let loc = (r.next_u32() % 5) as u8;
                    c.bus.write8(BE_MEDALS_INFO + 2 * n, ev);
                    c.bus.write8(BE_MEDALS_INFO + 2 * n + 1, loc);
                }
                c.g[A1][0] = sx(r.next_u32() % 14);
                c.g[A2][0] = sx(r.next_u32() % 5);
            },
        );
    }

    use crate::difftest::fakes::fake;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use crate::difftest::fakes::LOG_COUNT;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    pub(crate) use crate::game::camera::camera::tests::OC678_CTRL;
    pub(crate) use crate::game::camera::camera::tests::OC678_MAGIC;

    pub(crate) use crate::game::camera::camera::tests::oc678_install;

    pub(crate) fn w_3(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Log the call like every fake, then return the scripted result of this
    /// call index (shaped per callee so the data it feeds stays in range).
    pub(crate) fn oc677_fake_call(c: &mut Cpu, addr: u32) {
        let n = c.bus.read32(LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        fake(c, addr, ra);
        if c.pc == ra {
            let word = c.bus.read32(OC677_RET + 4 * (n & 31));
            let v = match addr {
                0x0015_2398 => OC677_STATS,
                0x0015_a3b8 => word & 1,
                0x0015_a398 => word % 3,
                0x001e_1ad0 => 0x100,
                0x0014_7398 => word % 10,
                0x001e_3a30 => word & 1,
                _ => word,
            };
            c.g[V0][0] = sx(v);
        }
    }

    macro_rules! oc677_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC677_CTRL) != OC677_MAGIC {
                    return fall_back(c, &$prev, $gen);
                }
                oc677_fake_call(c, $addr);
            }
        };
    }

    oc677_fake!(
        oc677_fake_0014dc80,
        OC677_PREV_0014DC80,
        0x0014dc80,
        ee::s01::f_0014dc80__generated
    );

    oc677_fake!(
        oc677_fake_0014dd58,
        OC677_PREV_0014DD58,
        0x0014dd58,
        ee::s01::f_0014dd58__generated
    );

    oc677_fake!(
        oc677_fake_00152398,
        OC677_PREV_00152398,
        0x00152398,
        ee::s01::f_00152398__generated
    );

    oc677_fake!(
        oc677_fake_0015a3b8,
        OC677_PREV_0015A3B8,
        0x0015a3b8,
        ee::s01::f_0015a3b8__generated
    );

    oc677_fake!(
        oc677_fake_0015a398,
        OC677_PREV_0015A398,
        0x0015a398,
        ee::s01::f_0015a398__generated
    );

    oc677_fake!(
        oc677_fake_00159cd0,
        OC677_PREV_00159CD0,
        0x00159cd0,
        ee::s01::f_00159cd0__generated
    );

    oc677_fake!(
        oc677_fake_00156a60,
        OC677_PREV_00156A60,
        0x00156a60,
        ee::s01::f_00156a60__generated
    );

    oc677_fake!(
        oc677_fake_00157bf0,
        OC677_PREV_00157BF0,
        0x00157bf0,
        ee::s01::f_00157bf0__generated
    );

    oc677_fake!(
        oc677_fake_00159818,
        OC677_PREV_00159818,
        0x00159818,
        ee::s01::f_00159818__generated
    );

    oc677_fake!(
        oc677_fake_00152528,
        OC677_PREV_00152528,
        0x00152528,
        ee::s01::f_00152528__generated
    );

    oc677_fake!(
        oc677_fake_00159170,
        OC677_PREV_00159170,
        0x00159170,
        ee::s01::f_00159170__generated
    );

    oc677_fake!(
        oc677_fake_001577e0,
        OC677_PREV_001577E0,
        0x001577e0,
        ee::s01::f_001577e0__generated
    );

    oc677_fake!(
        oc677_fake_0015a2e0,
        OC677_PREV_0015A2E0,
        0x0015a2e0,
        ee::s01::f_0015a2e0__generated
    );

    oc677_fake!(
        oc677_fake_00158910,
        OC677_PREV_00158910,
        0x00158910,
        ee::s01::f_00158910__generated
    );

    oc677_fake!(
        oc677_fake_00145630,
        OC677_PREV_00145630,
        0x00145630,
        ee::s01::f_00145630__generated
    );

    oc677_fake!(
        oc677_fake_001e3a30,
        OC677_PREV_001E3A30,
        0x001e3a30,
        ee::s04::f_001e3a30__generated
    );

    oc677_fake!(
        oc677_fake_001e3a78,
        OC677_PREV_001E3A78,
        0x001e3a78,
        ee::s04::f_001e3a78__generated
    );

    oc677_fake!(
        oc677_fake_001e1ad0,
        OC677_PREV_001E1AD0,
        0x001e1ad0,
        ee::s04::f_001e1ad0__generated
    );

    oc677_fake!(
        oc677_fake_00146e98,
        OC677_PREV_00146E98,
        0x00146e98,
        ee::s01::f_00146e98__generated
    );

    oc677_fake!(
        oc677_fake_00147398,
        OC677_PREV_00147398,
        0x00147398,
        ee::s01::f_00147398__generated
    );

    oc677_fake!(
        oc677_fake_001e2370,
        OC677_PREV_001E2370,
        0x001e2370,
        ee::s04::f_001e2370__generated
    );

    pub(crate) fn oc677_install() {
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc677_fake_0014dc80,
            &OC677_PREV_0014DC80,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc677_fake_0014dd58,
            &OC677_PREV_0014DD58,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00152398),
            oc677_fake_00152398,
            &OC677_PREV_00152398,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015A3B8),
            oc677_fake_0015a3b8,
            &OC677_PREV_0015A3B8,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015A398),
            oc677_fake_0015a398,
            &OC677_PREV_0015A398,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00159CD0),
            oc677_fake_00159cd0,
            &OC677_PREV_00159CD0,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00156A60),
            oc677_fake_00156a60,
            &OC677_PREV_00156A60,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00157BF0),
            oc677_fake_00157bf0,
            &OC677_PREV_00157BF0,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00159818),
            oc677_fake_00159818,
            &OC677_PREV_00159818,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00152528),
            oc677_fake_00152528,
            &OC677_PREV_00152528,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00159170),
            oc677_fake_00159170,
            &OC677_PREV_00159170,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_001577E0),
            oc677_fake_001577e0,
            &OC677_PREV_001577E0,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015A2E0),
            oc677_fake_0015a2e0,
            &OC677_PREV_0015A2E0,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00158910),
            oc677_fake_00158910,
            &OC677_PREV_00158910,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00145630),
            oc677_fake_00145630,
            &OC677_PREV_00145630,
        );
        hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E3A30),
            oc677_fake_001e3a30,
            &OC677_PREV_001E3A30,
        );
        hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E3A78),
            oc677_fake_001e3a78,
            &OC677_PREV_001E3A78,
        );
        hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E1AD0),
            oc677_fake_001e1ad0,
            &OC677_PREV_001E1AD0,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00146E98),
            oc677_fake_00146e98,
            &OC677_PREV_00146E98,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00147398),
            oc677_fake_00147398,
            &OC677_PREV_00147398,
        );
        hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E2370),
            oc677_fake_001e2370,
            &OC677_PREV_001E2370,
        );
    }

    /// `BERewardInt_OnEventResult` against the generated body: every mode
    /// 0..=14, every medal 0..=4, both settings of the peak-message flag
    /// (`D_005305F9`), random medal tables (tracks -1 and 4..5, kinds), random
    /// flag words, stats records with and without a count, and callees
    /// (back-end lookups, awards, place and peak checks, the message centre)
    /// answered by gated fakes, which unwind at random call indices.
    #[test]
    fn beint_reward_on_event_result_matches() {
        oc677_install();
        fakes::covered(
            ee::s01::f_001591e8__generated,
            beint_reward_on_event_result,
            450,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_91e8, 30);
                elf_image(c);
                w_3(c, OC677_CTRL, OC677_MAGIC);
                for i in 0..32u32 {
                    let word = r.next_u32();
                    w_3(c, OC677_RET + 4 * i, word);
                }
                // `gMedalsInfo`: [group][tier][5] of (track, kind) bytes.
                for i in 0..0x80u32 {
                    let b = if i % 2 == 0 {
                        [0xff, 0, 1, 2, 3, 4, 5, 6, 0xff][(r.next_u32() % 9) as usize]
                    } else {
                        (r.next_u32() % 4) as u8
                    };
                    c.bus.write8(0x0045_aad8 + i, b);
                }
                // The world: `world->+0x84->+0x28 -> int` (message state).
                let wld = DATA + 0x1000;
                w_3(c, GAME_GP.wrapping_sub(0x848), wld);
                w_3(c, wld + 0x84, DATA + 0x1200);
                w_3(c, DATA + 0x1228, DATA + 0x1300);
                w_3(c, DATA + 0x1300, [0, 5, 12][(seed % 3) as usize]);
                // The reward interface's last-result record.
                w_3(c, GAME_GP.wrapping_sub(0x1e8c), DATA + 0x2000);
                // The medal stats: old medal byte at +1, count at +2.
                c.bus.write8(
                    OC677_STATS + 1,
                    [0xff, 0, 1, 2, 3, 4][(r.next_u32() % 6) as usize],
                );
                c.bus
                    .write16(OC677_STATS + 2, if seed % 3 == 0 { 0 } else { 2 });
                let profile = seed % 3;
                let ch = (seed / 3) % 3;
                let mode = seed % 15;
                let medal = (seed / 15) % 5;
                c.bus.write8(0x0053_05f9, ((seed / 75) % 2) as u8);
                let rec = 0x004a_6ca8 + profile * 0x9b50 + ch * 0xf88;
                c.bus.write32(rec + 0x278, r.next_u32());
                c.bus.write32(rec + 0x27c, r.next_u32());
                c.pc = 0x0015_91e8;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = profile as u64;
                c.g[A2][0] = ch as u64;
                c.g[A3][0] = mode as u64;
                c.g[T0][0] = (r.next_u32() % 20) as u64;
                c.g[T1][0] = medal as u64;
            },
        );
    }

    /// `BERewardInt_LoadRewardsFile`: one call to the (faked) loader, which
    /// may unwind.
    #[test]
    fn be_reward_int_load_rewards_file_3_matches() {
        oc678_install();
        covered(
            ee::s01::f_00156a10__generated,
            be_reward_int_load_rewards_file_3,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x0015_6a10, 1);
                c.bus.write32(OC678_CTRL, OC678_MAGIC);
                c.pc = 0x0015_6a10;
            },
        );
    }

    #[test]
    fn be_reward_int_pick_random_bolt_matches() {
        check(
            ee::s01::f_00156c70__generated,
            be_reward_int_pick_random_bolt,
            200,
            &|c, seed| {
                machine(c, seed, 0x6c70, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00156c70;
            },
        );
    }

    #[test]
    fn be_reward_int_pick_random_locked_reward_matches() {
        check(
            ee::s01::f_00157080__generated,
            be_reward_int_pick_random_locked_reward,
            200,
            &|c, seed| {
                machine(c, seed, 0x7080, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00157080;
            },
        );
    }

    #[test]
    fn be_reward_int_unlock_reward_matches() {
        check(
            ee::s01::f_00157f20__generated,
            be_reward_int_unlock_reward,
            200,
            &|c, seed| {
                machine(c, seed, 0x7f20, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00157f20;
            },
        );
    }
}
