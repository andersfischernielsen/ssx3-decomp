//! `src/be/beintreward.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::be::beintnewrace::TRACK_RECORDS;
pub(crate) use crate::game::be::beintplayer::mul32;
pub(crate) use crate::game::bx::bxstring2::MESSAGE_MAP;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
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
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
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
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    use super::*;
    pub(crate) use crate::game::be::beintmission::callee::BUY_ATTRIB_UPDATE_BARS;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_EMPTY_STUB;
    pub(crate) use crate::game::be::beintmission::callee::CAREER_STATS_SHOW_HIGHLIGHTS;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::MEMCARD_SCAN_STEP;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_COUNT;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_NTH;
    pub(crate) use crate::game::be::beintmission::callee::MPEG2_ERROR;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_ON_WIDGET_ACTION;
    pub(crate) use crate::game::be::beintmission::callee::REWARDS_FILE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::UBER_TRICK_REFRESH;
    pub(crate) use crate::game::be::beintmission::callee::UITEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::be::beintmission::callee::VIEW_POPUP_SHOW_MESSAGE;
    pub(crate) use crate::game::be::beintmission::callee::WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::QSORT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RESET_FOR_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2 as RIDER_ADD_BOOST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_APPLY_SCRIPT_TRACK;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STRLEN as STRLEN_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH as STR_HASH_4;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST as UI_LIST_FIRST_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING as UI_TEXT_SET_STRING_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::APP_SOMETHING;
    pub(crate) use crate::game::dirtysock::tags2::callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE_RIDER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_CREATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_API_ADD_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LISTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LOBBY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_PEER_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2 as SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_FIELD_B0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SHARED_LOGIN_MAN_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_LIST_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_MAN_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_VIEW_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_IS_TRACK_GROUP0_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_RESET_RACE_RESULT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STORE_RACE_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COLLECTED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COMPLETED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD as BE_GET_CHAR_PROFILE_EVENT_RECORD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_CHARACTER_2 as BE_GET_RIDER_CHARACTER_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_SLOT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHALLENGE_POPUP_SET_SETTINGS_LOCKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_GROUP_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEAD_NODE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_ABORT_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_EMAIL_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_LOGIN_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_LOAD_UPDATE_PLAYER_RANKING;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_LOAD_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_SKIP_WHITESPACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LIGHT_MAN_EMPTY_HOOK_D;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_ON_CARD_SCANNED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_CARD_STATE_BASE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS_2 as MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT_2 as MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_11 as NET_FE_STATE_CTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_4 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_12 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_3 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID_2 as POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_GET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BUILD_WORLD_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_UPDATE_BOARD_LEAN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_FLUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_TREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_DEACTIVATE;
    pub(crate) use crate::game::fe::fepopup::callee::C00113f88;
    pub(crate) use crate::game::fe::fepopup::callee::C00115ab0;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe78;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe98;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fec8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a080;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a0e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0014bdb8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d448;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d988;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d998;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dc80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dd58;
    pub(crate) use crate::game::fe::fepopup::callee::C00150b48;
    pub(crate) use crate::game::fe::fepopup::callee::C00151c48;
    pub(crate) use crate::game::fe::fepopup::callee::C001535c0;
    pub(crate) use crate::game::fe::fepopup::callee::C001543e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00156a60;
    pub(crate) use crate::game::fe::fepopup::callee::C001577e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00157bf0;
    pub(crate) use crate::game::fe::fepopup::callee::C001589b0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158a50;
    pub(crate) use crate::game::fe::fepopup::callee::C00158af0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158c80;
    pub(crate) use crate::game::fe::fepopup::callee::C00158d58;
    pub(crate) use crate::game::fe::fepopup::callee::C00159cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C0015a2e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c6b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c718;
    pub(crate) use crate::game::fe::fepopup::callee::C0017f500;
    pub(crate) use crate::game::fe::fepopup::callee::C0019e2b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0019f878;
    pub(crate) use crate::game::fe::fepopup::callee::C001bb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001c84a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001c8568;
    pub(crate) use crate::game::fe::fepopup::callee::C001c9b68;
    pub(crate) use crate::game::fe::fepopup::callee::C001cb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbc48;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbce8;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbfc0;
    pub(crate) use crate::game::fe::fepopup::callee::C001ce560;
    pub(crate) use crate::game::fe::fepopup::callee::C001d32e0;
    pub(crate) use crate::game::fe::fepopup::callee::C001e1a30;
    pub(crate) use crate::game::fe::fepopup::callee::C001e3388;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91f8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e9290;
    pub(crate) use crate::game::fe::fepopup::callee::C001f10f8;
    pub(crate) use crate::game::fe::fepopup::callee::C0020e900;
    pub(crate) use crate::game::fe::fepopup::callee::C00224c78;
    pub(crate) use crate::game::fe::fepopup::callee::C00224da0;
    pub(crate) use crate::game::fe::fepopup::callee::C00224df0;
    pub(crate) use crate::game::fe::fepopup::callee::C002259a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00227f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00231f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00238bf8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026d4d8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026db88;
    pub(crate) use crate::game::fe::fepopup::callee::C0026e448;
    pub(crate) use crate::game::fe::fepopup::callee::C0026f4a0;
    pub(crate) use crate::game::fe::fepopup::callee::C002702f8;
    pub(crate) use crate::game::fe::fepopup::callee::C002703f0;
    pub(crate) use crate::game::fe::fepopup::callee::C002758c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275b98;
    pub(crate) use crate::game::fe::fepopup::callee::C00275cf8;
    pub(crate) use crate::game::fe::fepopup::callee::C00278e50;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f38;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f68;
    pub(crate) use crate::game::fe::fepopup::callee::C00281ed8;
    pub(crate) use crate::game::fe::fepopup::callee::C002837c8;
    pub(crate) use crate::game::fe::fepopup::callee::C0028b180;
    pub(crate) use crate::game::fe::fepopup::callee::C0028bb10;
    pub(crate) use crate::game::fe::fepopup::callee::C0028e8c0;
    pub(crate) use crate::game::fe::fepopup::callee::C002913d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003457c8;
    pub(crate) use crate::game::fe::fepopup::callee::C003625c0;
    pub(crate) use crate::game::fe::fepopup::callee::C003626d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003627a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00364cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C00366ce0;
    pub(crate) use crate::game::fe::fepopup::callee::C003684f0;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f460;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f668;
    pub(crate) use crate::game::fe::fepopup::callee::C0039c870;
    pub(crate) use crate::game::fe::fepopup::callee::C0039ff78;
    pub(crate) use crate::game::fe::fepopup::callee::C003b1300;
    pub(crate) use crate::game::fe::fepopup::callee::C003b2360;
    pub(crate) use crate::game::fe::fepopup::callee::C00125108;
    pub(crate) use crate::game::fe::fepopup::callee::C00150928;
    pub(crate) use crate::game::fe::fepopup::callee::C00156988;
    pub(crate) use crate::game::fe::fepopup::callee::C00158910;
    pub(crate) use crate::game::fe::fepopup::callee::C00159170;
    pub(crate) use crate::game::fe::fepopup::callee::C00176890;
    pub(crate) use crate::game::fe::fepopup::callee::C00225068;
    pub(crate) use crate::game::fe::fepopup::callee::C00275498;
    pub(crate) use crate::game::fe::fepopup::callee::C00278308;
    pub(crate) use crate::game::fe::fepopup::callee::C00283228;
    pub(crate) use crate::game::fe::fepopup::callee::C00283518;
    pub(crate) use crate::game::fe::fepopup::callee::C00358020;
    pub(crate) use crate::game::fe::fepopup::callee::C00362478;
    pub(crate) use crate::game::fe::fepopup::callee::C00362660;
    pub(crate) use crate::game::fe::fepopup::callee::C00368170;
    pub(crate) use crate::game::fe::fepopup::callee::C00368970;
    pub(crate) use crate::game::fe::festaterewards::callee::BECHAR_GET_OVERALL_RATING;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_IS_SHARING_CHAR_RECORD;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_GET_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_SPEND_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_GET_RIDER_PROFILE_2 as BE_GET_RIDER_PROFILE_4;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_GET_FILE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_SET_BIT;
    pub(crate) use crate::game::fe::festaterewards::callee::BRAGGING_RIGHTS_UPDATE_ROW_COLORS;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_OPPONENT_DISCONNECTED;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SERVER_TIMEOUT;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SHOW_CANCEL;
    pub(crate) use crate::game::fe::festaterewards::callee::GET_LANGUAGE_MASK_2 as GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_ARCADE_SPEECH_ON;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_DJ_SPEECH_ON_2 as IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_IS_FLAG_SET;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_POST;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_SET_FLAG;
    pub(crate) use crate::game::fe::festaterewards::callee::MOD_OBJ_RUN_SCRIPT_EVENT;
    pub(crate) use crate::game::fe::festaterewards::callee::REWARD_GALLERY_REGISTER_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_POP_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_PUSH_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_START_SCRIPT;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_STOP_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_COPY_TAG;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_SET_RAW as TAG_FIELD_SET_RAW_2;
    pub(crate) use crate::game::fe::festaterewards::callee::TEXFLIP_SET_FRAME;
    pub(crate) use crate::game::fe::festaterewards::callee::VFX_PLANTS_GET_SWAY_MATRIX;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_CREATE_COMPUTER_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_DESTROY_MISSION_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_REWARD_INT_COUNT_SET_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_ACCOUNT_NAME;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_PASSWORD;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_SHUTDOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::ovstatepause::callee::EVENT_SELECT_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_ASYNC_MANAGER_CANCEL;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_FORMAT_MONEY;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_MOVE_DOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_BEGIN_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MEM_CARD_STATE_BASE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::MOUNTAIN_ROOM_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_SHOW_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::ovstatepause::callee::OV_SCREEN_SESSION_DESTROY_INDICATORS;
    pub(crate) use crate::game::fe::ovstatepause::callee::PLAY_FE_SOUND;
    pub(crate) use crate::game::fe::ovstatepause::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::fe::ovstatepause::callee::SECTION_MAN_REQUEST_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRLEN;
    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_ON_ACTIVATE_2 as UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_UPDATE_2 as UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::ovstatepause::callee::VID_ENGINE_PLAYER_RENDER;
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT as BE_NEW_PLAYER_INT_GET_SLOT_2;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_3;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UNLOAD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamemode::callee::FE_ASYNC_MANAGER_SET_FILE_STATUS;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_REQUEST_INSTANT_REPLAY;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;
    pub(crate) use crate::game::main::gamemode::callee::RUN_LIST_SEEK;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_LIST_ON_SEQ_ADVANCE;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamemode::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::main::gamemode::callee::STRLEN as STRLEN_2;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3 as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE_3;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE as MEM_FREE_3;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_GET_MESSAGE_COUNT;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_REMOVE_MESSAGE;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_2;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_SCRIPT_OBJ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_STOP_3 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_HAS_BINDING;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_INSTANCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_AABB;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_SPHERE_TREE;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::STR_HASH as STR_HASH_3;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_CTOR_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_DTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_BE as CBE_GET_BE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_INTERFACE;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FLAG_MANAGER_GROUP_INIT_KEY;
    pub(crate) use crate::game::movie::movieplayer::callee::C_OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::EVENT_SELECT_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_CONFIG;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_RESET_LAYOUT;
    pub(crate) use crate::game::movie::movieplayer::callee::FILE_LOAD;
    pub(crate) use crate::game::movie::movieplayer::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES as FLEX_BRIDGE_ALLOC_SPRING_NODES_2;
    pub(crate) use crate::game::movie::movieplayer::callee::FONT_PARSE_GLYPHS;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_END_OPERATION_4 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_ON_OPERATION_DONE;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_SET_MODE_2 as MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_SET_SELECTION;
    pub(crate) use crate::game::movie::movieplayer::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::movie::movieplayer::callee::MULTI_SPLINE_MODIFIER_ADD_REF;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_CTOR_5 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::movie::movieplayer::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS as RIDER_METRIX_RESET_STATS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::ROLLER_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_FIND_SOUND_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2 as STRCPY_3;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR_3;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER as BE_GET_RIDER_CHARACTER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2 as KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET as MC_MANAGER_GET_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;

    // Camera / reward front end (shard 1..4).
    pub const BE_GET_CHAR_PROFILE_EVENT_RECORD: Callee = Callee {
        addr: 0x0015_2398,
        f: ee::s01::f_00152398,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::fe::feasyncfile::table::ASYNC_FILE_GET_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::table::AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_EVENT_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_GAME_MODE;
    pub(crate) use crate::game::fe::feasyncfile::table::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE2;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_CALC;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_TREE_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_GET_UNPACKED_SIZE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_UNPACK;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::table::OPERATOR_NEW_TAG;
    pub(crate) use crate::game::fe::feasyncfile::table::PATH_STOP_TRACKS;
    pub(crate) use crate::game::fe::feasyncfile::table::PS2_MEM_CARD_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_BEGIN_PLAYBACK;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_END_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::table::RIDER_MODEL_FIND;
    pub(crate) use crate::game::fe::feasyncfile::table::SCRIPT_ENGINE_GET_RESULT_VTABLE;
    pub(crate) use crate::game::fe::feasyncfile::table::SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_CASECMP;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_MENU_RESET_SCROLL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_START;
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
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
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
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
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
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
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
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
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
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
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
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
            c.pc = t;
            return Ok(());
        }

        Ok(())
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
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
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
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[S0][0] as u32)));
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
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
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
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[S0][0] as u32)));
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
                                            if callee::C00156a60.call(c, 0x00157c94).is_err() {
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
                        if callee::C001589b0.call(c, 0x00157cc4).is_err() {
                            return;
                        }
                        c.g[S2][0] = 0u64;
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00157cd8);
                        c.g[A1][0] = 0xffffffffffffffff;
                        if callee::C00156a60.call(c, 0x00157cd8).is_err() {
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
                            if callee::C00158a50.call(c, 0x00157cf0).is_err() {
                                return;
                            }
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00157d00);
                            c.g[A1][0] = 0xffffffffffffffff;
                            if callee::C00156a60.call(c, 0x00157d00).is_err() {
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
                if callee::C00158af0.call(c, 0x00157d20).is_err() {
                    return;
                }
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00157d30);
                c.g[A1][0] = 0xffffffffffffffff;
                if callee::C00156a60.call(c, 0x00157d30).is_err() {
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
            if callee::C0014dc80.call(c, 0x00159b3c).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00159b48);
            c.g[A1][0] = 0xa;
            if callee::C0014dd58.call(c, 0x00159b48).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00159b54);
            c.g[A1][0] = c.g[S3][0];
            if callee::C001543e0.call(c, 0x00159b54).is_err() {
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
                            if callee::C0014a0e0.call(c, 0x00159b84).is_err() {
                                return;
                            }
                            c.g[S2][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00159b90);
                            c.g[A0][0] = c.g[S0][0];
                            if callee::C0014a080.call(c, 0x00159b90).is_err() {
                                return;
                            }
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A3][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x00159bac);
                            c.g[T0][0] = 0x2;
                            if callee::C00157bf0.call(c, 0x00159bac).is_err() {
                                return;
                            }
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00159bc4);
                            c.g[A3][0] = c.g[S3][0];
                            if callee::C00158d58.call(c, 0x00159bc4).is_err() {
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
                    if callee::C001577e0.call(c, 0x00159c0c).is_err() {
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
                        if callee::C0015a2e0.call(c, 0x00159c24).is_err() {
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
                            if callee::C00159cd0.call(c, 0x00159c40).is_err() {
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
            if callee::C0014dc80.call(c, 0x001599d4).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001599e0);
            c.g[A1][0] = 0xa;
            if callee::C0014dd58.call(c, 0x001599e0).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001599f0);
            c.g[A2][0] = c.g[S3][0];
            if callee::C001535c0.call(c, 0x001599f0).is_err() {
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
                            if callee::C0014a0e0.call(c, 0x00159a20).is_err() {
                                return;
                            }
                            c.g[S2][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00159a2c);
                            c.g[A0][0] = c.g[S0][0];
                            if callee::C0014a080.call(c, 0x00159a2c).is_err() {
                                return;
                            }
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A3][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x00159a48);
                            c.g[T0][0] = 0x2;
                            if callee::C00157bf0.call(c, 0x00159a48).is_err() {
                                return;
                            }
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00159a60);
                            c.g[A3][0] = c.g[S3][0];
                            if callee::C00158c80.call(c, 0x00159a60).is_err() {
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
                    if callee::C001577e0.call(c, 0x00159aa8).is_err() {
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
                        if callee::C0015a2e0.call(c, 0x00159ac0).is_err() {
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
                            if callee::C00159cd0.call(c, 0x00159adc).is_err() {
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::*;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::ai::tests::generic;
    pub(crate) use crate::game::ai::ai::tests::globals_8 as globals_4;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_8;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_3;
    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_7;
    pub(crate) use crate::game::fe::festaterewards::tests::hook;
    pub(crate) use crate::game::fe::festaterewards::tests::strict;
    pub(crate) use crate::game::fe::festaterewards::tests::HOOKS;
    pub(crate) use crate::game::fe::festaterewards::tests::KEEP_F0;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::w;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2;
    pub(crate) use crate::game::main::gamestate::tests::base_3;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_3;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_2;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2 as BE;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE as BE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_5;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
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
}
