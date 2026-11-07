//! `src/ai/computer.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::ai::SPIN_ENTER_ANIM_CTRL;
pub(crate) use crate::game::ai::ai::SPIN_ENTER_CHARACTER;
pub(crate) use crate::game::ai::ai::SPIN_ENTER_PLAYER;
pub(crate) use crate::game::ai::rider::add32;
pub(crate) use crate::game::ai::rider::eq;
pub(crate) use crate::game::ai::rider::fr_2 as fr;
pub(crate) use crate::game::ai::rider::gp_3 as gp;
pub(crate) use crate::game::ai::rider::ldf_2 as ldf;
pub(crate) use crate::game::ai::rider::le;
pub(crate) use crate::game::ai::rider::lt;
pub(crate) use crate::game::ai::rider::p;
pub(crate) use crate::game::ai::rider::setf_2 as setf;
pub(crate) use crate::game::ai::rider::shl32;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_5;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::gpa;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::ldf as ldf_4;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::ofs;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at_3;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::fepopup::at as at_6;
pub(crate) use crate::game::fe::fepopup::lb;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::misc::gp_lit;
pub(crate) use crate::game::movie::movieplayer::at as at_4;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::visualfx::boardtrailfx::add32 as add32_2;
pub(crate) use crate::game::visualfx::boardtrailfx::shl32_2;
pub(crate) use crate::game::visualfx::boardwakefx::at_3 as at_2;
pub(crate) use crate::game::visualfx::boardwakefx::gp as gp_3;
pub(crate) use crate::game::visualfx::boardwakefx::ldf_4 as ldf_3;
pub(crate) use crate::game::visualfx::boardwakefx::lh_2 as lh;
pub(crate) use crate::game::visualfx::boardwakefx::lw_2;
pub(crate) use crate::game::visualfx::boardwakefx::r32_2;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
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
        0x0010_0348,
        computer_actor_rider_compute_steer,
        "ComputerActorRider_ComputeSteer",
    ),
    (
        0x0010_0680,
        computer_actor_rider_cruise_follow_path,
        "ComputerActorRider_CruiseFollowPath",
    ),
    (
        0x0010_8f88,
        computer_actor_rider_init_2,
        "ComputerActorRider_Init_2",
    ),
    (
        0x0010_a768,
        computer_actor_rider_get_control_input,
        "ComputerActorRider_GetControlInput",
    ),
    (
        0x0010_b250,
        computer_actor_rider_think_spin,
        "ComputerActorRider_ThinkSpin",
    ),
    (
        0x0010_b790,
        computer_actor_rider_think_gate,
        "ComputerActorRider_ThinkGate",
    ),
    (
        0x0010_c4f8,
        computer_actor_rider_apply_difficulty,
        "ComputerActorRider_ApplyDifficulty",
    ),
    (
        0x0010_cab8,
        computer_actor_rider_init_difficulty_2,
        "ComputerActorRider_InitDifficulty_2",
    ),
    (
        0x0010_d170,
        computer_actor_rider_reset_trick_counts,
        "ComputerActorRider_ResetTrickCounts",
    ),
    (
        0x0010_d1a0,
        computer_actor_rider_should_reset_path,
        "ComputerActorRider_ShouldResetPath",
    ),
    (
        0x0010_d8f8,
        computer_actor_rider_find_human_rider_index,
        "ComputerActorRider_FindHumanRiderIndex",
    ),
    (
        0x0010_d938,
        computer_actor_rider_is_far_behind_human,
        "ComputerActorRider_IsFarBehindHuman",
    ),
    (
        0x0010_d990,
        computer_actor_rider_is_far_ahead_of_human,
        "ComputerActorRider_IsFarAheadOfHuman",
    ),
    (
        0x0010_d9e8,
        computer_actor_rider_is_rival_target,
        "ComputerActorRider_IsRivalTarget",
    ),
    (
        0x0010_dbf0,
        computer_actor_rider_find_attack_target,
        "ComputerActorRider_FindAttackTarget",
    ),
    (0x0010_e028, rider_set_timed_event, "Rider_SetTimedEvent"),
    (0x0010_e098, rider_add_boost, "Rider_AddBoost"),
    (0x0010_e558, rider_on_split_marker, "Rider_OnSplitMarker"),
    (
        0x0010_e5d8,
        rider_handle_race_path_event,
        "Rider_HandleRacePathEvent",
    ),
    (0x0010_e830, rider_v15, "Rider_v15"),
    (
        0x0010_e8b8,
        rider_award_bonus_boost,
        "Rider_AwardBonusBoost",
    ),
    (
        0x0010_f2d8,
        rider_on_mission_completed,
        "Rider_OnMissionCompleted",
    ),
    (
        0x0010_f338,
        rider_on_item_collected,
        "Rider_OnItemCollected",
    ),
    (
        0x0010_f398,
        ai_reset_rider_relations_2,
        "AI_ResetRiderRelations_2",
    ),
    (0x0010_fc30, ai_count_riders_on_path, "AI_CountRidersOnPath"),
    (
        0x0011_12f8,
        rider_base_exit_main_state,
        "RiderBase_ExitMainState",
    ),
    (
        0x0011_1380,
        rider_base_enter_main_state,
        "RiderBase_EnterMainState",
    ),
    (0x0011_1408, rider_base_update_main_state, "func_00111408"),
    (
        0x0011_14a0,
        rider_base_post_update_main_state,
        "func_001114A0",
    ),
    (
        0x0011_1728,
        rider_base_handle_sub_state_input,
        "func_00111728",
    ),
    (0x0011_1890, rider_base_reset_fx, "RiderBase_ResetFX"),
    (
        0x0011_2338,
        rider_update_race_progress,
        "Rider_UpdateRaceProgress",
    ),
    (
        0x0011_2588,
        rider_get_ai_path_look_ahead_dist,
        "Rider_GetAIPathLookAheadDist",
    ),
    (
        0x0011_25a8,
        rider_save_ai_path_look_ahead,
        "Rider_SaveAIPathLookAhead",
    ),
    (0x0011_25b8, rider_return_zero_25b8, "Rider_ReturnZero_25b8"),
    (
        0x0011_25c0,
        rider_update_ai_path_progress,
        "Rider_UpdateAIPathProgress",
    ),
    (
        0x0011_3128,
        rider_get_dist_to_finish,
        "Rider_GetDistToFinish",
    ),
    (0x0011_3138, air_predictor_ctor, "AirPredictor_ctor"),
    (0x0011_3170, rider_dtor_free, "Rider_dtorFree"),
    (
        0x0011_2fb0,
        rider_update_dist_to_finish,
        "Rider_UpdateDistToFinish",
    ),
    (
        0x0010_b980,
        computer_actor_rider_find_jump_marker,
        "ComputerActorRider_FindJumpMarker",
    ),
    (
        0x0010_da10,
        computer_actor_rider_should_attack_back,
        "ComputerActorRider_ShouldAttackBack",
    ),
    (
        0x0010_f560,
        ai_update_rider_relations,
        "AI_UpdateRiderRelations",
    ),
    (
        0x0010_f998,
        ai_update_race_positions,
        "AI_UpdateRacePositions",
    ),
    (0x0011_1ac0, rider_base_save_state, "RiderBase_SaveState"),
    (0x0011_27f0, rider_select_race_path, "Rider_SelectRacePath"),
    (0x0011_2a50, rider_select_ai_path, "Rider_SelectAIPath"),
    (
        0x0010_d870,
        computer_actor_rider_get_path_class_weight_2,
        "ComputerActorRider_GetPathClassWeight",
    ),
    (
        0x0011_1578,
        rider_base_exit_sub_state,
        "RiderBase_ExitSubState",
    ),
    (
        0x0011_1630,
        rider_base_enter_sub_state,
        "RiderBase_EnterSubState",
    ),
    (
        0x0011_1aa0,
        rider_base_trigger_dynamic_spray,
        "RiderBase_TriggerDynamicSpray",
    ),
    (
        0x0010_a898,
        computer_actor_rider_save_state,
        "ComputerActorRider_SaveState",
    ),
    (
        0x0010_c140,
        computer_actor_rider_crouch_and_boost,
        "ComputerActorRider_CrouchAndBoost",
    ),
    (
        0x0010_cad8,
        computer_actor_rider_choose_trick,
        "ComputerActorRider_ChooseTrick",
    ),
    (
        0x0010_cd20,
        computer_actor_rider_pick_fitted_trick,
        "ComputerActorRider_PickFittedTrick",
    ),
    (
        0x0010_cf68,
        computer_actor_rider_pick_uber_trick,
        "ComputerActorRider_PickUberTrick",
    ),
    (
        0x0010_e228,
        rider_report_rider_hit0,
        "Rider_ReportRiderHit0",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::rider::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::ai::rider::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::ai::rider::callee::EA_MEM_COPY;
    pub(crate) use crate::game::ai::rider::callee::FIXUNSSFSI;
    pub(crate) use crate::game::ai::rider::callee::MATH_ATAN;
    pub(crate) use crate::game::ai::rider::callee::PATH_GET_CLOSEST;
    pub(crate) use crate::game::ai::rider::callee::PATH_GET_CLOSEST_ON_PATH;
    pub(crate) use crate::game::ai::rider::callee::PATH_GET_LENGTH;
    pub(crate) use crate::game::ai::rider::callee::PATH_GET_POINT_AT_DISTANCE;
    pub(crate) use crate::game::ai::rider::callee::RIDER_AI_LOOK_AHEAD;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_SUB_STATE;
    pub(crate) use crate::game::ai::rider::callee::RIDER_HANDLE_RACE_PATH_EVENT;
    pub(crate) use crate::game::ai::rider::callee::RIDER_IS_ON_GROUND;
    pub(crate) use crate::game::ai::rider::callee::RIDER_ON_FINISH_RACE;
    pub(crate) use crate::game::ai::rider::callee::RIDER_RETURN_ZERO;
    pub(crate) use crate::game::ai::rider::callee::RIDER_SAVE_AI_LOOK_AHEAD;
    pub(crate) use crate::game::ai::rider::callee::RIDER_SELECT_AI_PATH;
    pub(crate) use crate::game::ai::rider::callee::RIDER_SELECT_RACE_PATH;
    pub(crate) use crate::game::ai::rider::callee::RIDER_SET_TIMED_EVENT;
    pub(crate) use crate::game::ai::rider::callee::RIDER_UPDATE_DANGER_STATE;
    pub(crate) use crate::game::ai::rider::callee::RIDER_UPDATE_DIST_TO_FINISH;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_ARCADE_SAY;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_UPDATE_RIDER_WORLD_PAINT;
    pub(crate) use crate::game::ai::rider::callee::WORLD_PAINTER_WEATHER20;
    pub(crate) use crate::game::ai::rider::callee::WORLD_PAINTER_WEATHER28;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RESET_FOR_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2 as RIDER_ADD_BOOST_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_APPLY_SCRIPT_TRACK;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE as RIDER_GET_MAIN_STATE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
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
    pub(crate) use crate::game::movie::movieplayer::callee::AIR_APPLY_WIND;
    pub(crate) use crate::game::movie::movieplayer::callee::AIR_PREDICTOR_UPDATE;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::movie::movieplayer::callee::AUDIO_RESET_BOOST_PITCH;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_CHAR_ATTRIB_FACTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILD_OCCLUDER_PLANES;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_CTOR_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_DTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_BE;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_INTERFACE;
    pub(crate) use crate::game::movie::movieplayer::callee::COLL_SEGMENT_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FLAG_MANAGER_GROUP_INIT_KEY;
    pub(crate) use crate::game::movie::movieplayer::callee::C_OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::EVENT_SELECT_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_CONFIG;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_RESET_LAYOUT;
    pub(crate) use crate::game::movie::movieplayer::callee::FILE_LOAD;
    pub(crate) use crate::game::movie::movieplayer::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::movie::movieplayer::callee::FONT_PARSE_GLYPHS;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::MATH_DET3X3;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_END_OPERATION_4 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_ON_OPERATION_DONE;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_SET_MODE_2 as MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_SET_SELECTION;
    pub(crate) use crate::game::movie::movieplayer::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER as MOD_OBJ_SET_TRANSFORM_MODIFIER_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MULTI_SPLINE_MODIFIER_ADD_REF;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_CTOR_5 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::movie::movieplayer::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::OVERLAP_LIST_FIND_CLOSEST;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::PACK_POSITION;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL128_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL16_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL256_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL32_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL512_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL64_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL8_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::QSORT;
    pub(crate) use crate::game::movie::movieplayer::callee::REMOVE_HUD_MESSAGE;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_GET_SUBSTATE;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_ORIENTATION_IMPLICIT;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_UPDATE_ORIENTATION;
    pub(crate) use crate::game::movie::movieplayer::callee::ROLLER_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_FIND_SOUND_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SET_TIMED_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::SHOW_HUD_NUMBER;
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2 as STRCPY;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::VU0_UPLOAD_OCCLUDER;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
    pub(crate) use crate::game::render::particle::callee::ALGORITHM_LIST_DTOR;
    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::AVA_SPLINE_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;
    pub(crate) use crate::game::render::particle::callee::BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::render::particle::callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_COLLECTIBLE_COLLECTED;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;
    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::render::particle::callee::CROWD_MAN3D_GROUP_REMOVE_SECTION;
    pub(crate) use crate::game::render::particle::callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT;
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::render::particle::callee::FLEX_BRIDGE_SPRINGS_SAVE;
    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;
    pub(crate) use crate::game::render::particle::callee::ISIGNAL_SEMA;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_POP;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_SELECT;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::render::particle::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::render::particle::callee::PATH_POST_EVENT;
    pub(crate) use crate::game::render::particle::callee::PATH_SET_PAUSED;
    pub(crate) use crate::game::render::particle::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST as RIDER_ADD_BOOST_2;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS as RIDER_METRIX_AWARD_POINTS_2;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_FINISH_BOARD_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_DELETE;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_MKDIR;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_READ;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_SEEK;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_WRITE;
    pub(crate) use crate::game::render::particle::callee::SNDSTRM_SET_PITCH;
    pub(crate) use crate::game::render::particle::callee::SND_LOCK;
    pub(crate) use crate::game::render::particle::callee::SND_SPU_MEM_SET_BASE;
    pub(crate) use crate::game::render::particle::callee::SND_UNLOCK;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_STREAM_HANDLE;
    pub(crate) use crate::game::render::particle::callee::STRLEN;
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2 as UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::WORLD_CREATE_DEAD_NODE;
    pub(crate) use crate::game::render::particle::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2 as CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2 as DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::NET_FE_STATE_DTOR_3 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2 as RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2 as SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2 as SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_TARGET_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_ON_SEQ_END;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_STOP_2 as SCRIPT_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2 as SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;

    pub const GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::Rider_GetMainState,
    };

    pub const GET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fee8,
        f: ee::s00::Rider_GetSubState,
    };

    pub const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::BELibrary_GetInstance,
    };

    pub const RUBBER_BAND: Callee = Callee {
        addr: 0x0010_def0,
        f: ee::s00::ComputerActorRider_ComputeRubberBand,
    };

    pub const SET_ANIM_TIME_SCALE: Callee = Callee {
        addr: 0x0012_0090,
        f: ee::s00::Rider_SetAnimTimeScale,
    };

    pub const GET_SUB_STATE_2: Callee = Callee {
        addr: 0x0011_fee8,
        f: ee::s00::Rider_GetSubState,
    };

    pub const THINK_CRUISE: Callee = Callee {
        addr: 0x0010_a960,
        f: ee::s00::ComputerActorRider_ThinkCruise,
    };

    pub const THINK_BOARD_PRESS: Callee = Callee {
        addr: 0x0010_ad78,
        f: ee::s00::ComputerActorRider_ThinkBoardPress,
    };

    pub const THINK_PREWIND: Callee = Callee {
        addr: 0x0010_aa70,
        f: ee::s00::ComputerActorRider_ThinkPrewind,
    };

    pub const THINK_NATURAL_AIR: Callee = Callee {
        addr: 0x0010_b590,
        f: ee::s00::ComputerActorRider_ThinkNaturalAir,
    };

    pub const THINK_SPIN: Callee = Callee {
        addr: 0x0010_b250,
        f: ee::s00::ComputerActorRider_ThinkSpin,
    };

    pub const THINK_GATE: Callee = Callee {
        addr: 0x0010_b790,
        f: ee::s00::ComputerActorRider_ThinkGate,
    };

    pub const THINK_RAILSLIDE: Callee = Callee {
        addr: 0x0010_aed8,
        f: ee::s00::ComputerActorRider_ThinkRailslide,
    };

    pub const THINK_WIPEOUT: Callee = Callee {
        addr: 0x0010_b750,
        f: ee::s00::ComputerActorRider_ThinkWipeout,
    };

    pub const VFPRINTF_R: Callee = Callee {
        addr: 0x0041_b728,
        f: ee::s15::_vfprintf_r,
    };

    pub const VFIPRINTF_R: Callee = Callee {
        addr: 0x0041_ca70,
        f: ee::s15::_vfiprintf_r,
    };

    /// `EA_MemSet(block, 0, 8)`: the original calls it through the table.
    pub const EA_MEM_SET: u32 = 0x003e_6448;
}

// ---------------------------------------------------------------------------
// Steering.

/// Rider fields used by the steering computation.
pub(crate) mod rider {
    pub(crate) use crate::game::ai::rider::rider::AI_ARC_LEN;
    pub(crate) use crate::game::ai::rider::rider::AI_HEADING;
    pub(crate) use crate::game::ai::rider::rider::AI_INDEX;
    pub(crate) use crate::game::ai::rider::rider::AI_PATH;
    pub(crate) use crate::game::ai::rider::rider::AI_PATH_CURSOR;
    pub(crate) use crate::game::ai::rider::rider::AI_POINT;
    pub(crate) use crate::game::ai::rider::rider::AI_POINT_PREV;
    pub(crate) use crate::game::ai::rider::rider::AI_POINT_TARGET;
    pub(crate) use crate::game::ai::rider::rider::AI_PROGRESS;
    pub(crate) use crate::game::ai::rider::rider::AI_SEARCH_DIST;
    pub(crate) use crate::game::ai::rider::rider::BASE;
    pub(crate) use crate::game::ai::rider::rider::BBOX0;
    pub(crate) use crate::game::ai::rider::rider::BBOX1;
    pub(crate) use crate::game::ai::rider::rider::BOOST;
    pub(crate) use crate::game::ai::rider::rider::BOOST_A;
    pub(crate) use crate::game::ai::rider::rider::BOOST_B;
    pub(crate) use crate::game::ai::rider::rider::BOOST_KIND;
    pub(crate) use crate::game::ai::rider::rider::BOOST_PHASE;
    pub(crate) use crate::game::ai::rider::rider::BOOST_SCALE;
    pub(crate) use crate::game::ai::rider::rider::BSPHERE;
    pub(crate) use crate::game::ai::rider::rider::DELTA_TIME;
    pub(crate) use crate::game::ai::rider::rider::DID_NOT_FINISH;
    pub(crate) use crate::game::ai::rider::rider::DIST_ESTIMATE;
    pub(crate) use crate::game::ai::rider::rider::DIST_TO_FINISH;
    pub(crate) use crate::game::ai::rider::rider::FINISH_TIME;
    pub(crate) use crate::game::ai::rider::rider::GROUND_NORMAL;
    pub(crate) use crate::game::ai::rider::rider::HANDLER;
    pub(crate) use crate::game::ai::rider::rider::HEADING;
    pub(crate) use crate::game::ai::rider::rider::IS_HUMAN;
    pub(crate) use crate::game::ai::rider::rider::ON_SCREEN;
    pub(crate) use crate::game::ai::rider::rider::POSITION;
    pub(crate) use crate::game::ai::rider::rider::RACE_ENTRIES;
    pub(crate) use crate::game::ai::rider::rider::RACE_ENTRY_COUNT;
    pub(crate) use crate::game::ai::rider::rider::RACE_PATH;
    pub(crate) use crate::game::ai::rider::rider::RACE_PATH_CURSOR;
    pub(crate) use crate::game::ai::rider::rider::RENDER_FLAG;
    pub(crate) use crate::game::ai::rider::rider::RENDER_KIND;
    pub(crate) use crate::game::ai::rider::rider::UNK_150;
    pub(crate) use crate::game::ai::rider::rider::VELOCITY;
    pub(crate) use crate::game::ai::rider::rider::VISIBILITY_BLOCK;

    /// Physical velocity (vec4).
    pub const VELOCITY_2: u32 = 0x1b0;

    /// Speed (the obstacle-phase gate reads it below 1000).
    pub const SPEED: u32 = 0x4c8;

    /// Route heading (fall line).
    pub const HEADING_2: u32 = 0x4cc;

    /// `RiderBase*`.
    pub const BASE_2: u32 = 0x77c;

    /// The air predictor object.
    pub const AIR_PREDICTOR: u32 = 0x788;

    /// The rider's player slot (edging stat lookups).
    pub const PLAYER: u32 = 0x86c;

    /// The per-player value word the attrib factor divides.
    pub const ATTRIB_VALUE: u32 = 0xb34;

    /// Ground speed factor the carve response divides by.
    pub const SPEED_FACTOR: u32 = 0x2fc;

    /// Current boost amount (0..1), scaled for the HUD.
    pub const BOOST_2: u32 = 0x2f8;

    /// Boost HUD timer and its countdown.
    pub const BOOST_TIMER: u32 = 0x2f0;

    pub const BOOST_COUNT: u32 = 0x2f4;

    /// Boost state: 3 means "no boost HUD".
    pub const BOOST_STATE: u32 = 0x304;

    pub const BOOST_MASK: u32 = 0xb28;

    pub const BOOST_FLAG: u32 = 0xb2c;

    pub const CHAR_ID: u32 = 0x86c;

    pub const CHAR_COMP: u32 = 0xb34;

    /// Metric multiplier used for milestone scores.
    pub const METRIC_SCALE: u32 = 0x1c4;

    /// `Rider+0x18`: the rider's model / animation object.
    pub const MODEL: u32 = 0x18;

    /// `RiderModel+0x6c0`: the motion-controller object (has a vtable).
    pub const MOTION: u32 = 0x6c0;
}

pub(crate) mod rider_base {

    /// Main state (`Rider_GetMainState`).
    pub const MAIN_STATE: u32 = 0xde0;

    /// Sub state (`Rider_GetSubState`).
    pub const SUB_STATE: u32 = 0xde4;
}

pub(crate) mod air_predictor {

    /// Predicted landing height.
    pub const LANDING: u32 = 0x98;

    /// Height above the ground the predictor expects.
    pub const HEIGHT: u32 = 0xa0;

    /// Mode: 1 or 3 means "airborne and predicting".
    pub const MODE: u32 = 0xac;
}

/// The computer rider's AI bookkeeping.
pub(crate) mod computer {

    /// The rider it drives.
    pub const RIDER: u32 = 0x18;

    /// Movement direction along the course (< 0 = the wrong way).
    pub const DIR: u32 = 0xe70;

    /// Frames spent stuck.
    pub const STUCK_TIME: u32 = 0xe74;

    /// Frames spent facing the wrong way.
    pub const WRONG_TIME: u32 = 0xe78;
}

/// `gp`-relative globals used here.
pub(crate) mod g {
    pub(crate) use crate::game::fe::festateload::g::BE_OPTION_SINGLETON;
    pub(crate) use crate::game::fe::festateload::g::BOLT_FILE_CACHE;
    pub(crate) use crate::game::fe::festateload::g::FE_BASE_BACK;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER_END;
    pub(crate) use crate::game::fe::festateload::g::WORLD_TRIGGER_MANAGER;
}

/// Cross-shard call targets (the generator emits `call_indirect` for them).
pub(crate) mod table {

    pub const DELETE: u32 = 0x0031_7e50;

    pub const EA_MEM_FREE: u32 = 0x0031_7e98;

    pub const OPERATOR_NEW_NAMED: u32 = 0x0031_7e30;

    pub const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;

    pub const WAIT_SEMA: u32 = 0x0042_3de0;

    pub const WAKEUP_THREAD: u32 = 0x0042_3cd0;

    pub const SLEEP_THREAD: u32 = 0x0042_3cc0;

    pub const STR_HASH: u32 = 0x0031_7670;

    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: u32 = 0x0028_b180;
}

// ---------------------------------------------------------------------------
// ComputerActorRider_CruiseFollowPath (0x00100680)

pub(crate) mod cruise {
    use super::ee;
    use super::Callee;

    pub const GET_STEER_TARGET: Callee = Callee {
        addr: 0x0010_0610,
        f: ee::s00::ComputerActorRider_GetSteerTarget,
    };

    pub const COMPUTE_STEER: Callee = Callee {
        addr: 0x0010_0348,
        f: ee::s00::ComputerActorRider_ComputeSteer,
    };

    pub const FIND_SPEED_MARKER: Callee = Callee {
        addr: 0x0010_bb18,
        f: ee::s00::ComputerActorRider_FindSpeedMarker,
    };

    pub const CROUCH_AND_BOOST: Callee = Callee {
        addr: 0x0010_c140,
        f: ee::s00::ComputerActorRider_CrouchAndBoost,
    };

    pub const SET_CRUISE_CROUCH: Callee = Callee {
        addr: 0x0010_c0a8,
        f: ee::s00::ComputerActorRider_SetCruiseCrouch,
    };

    pub const FIND_BOARD_PRESS_MARKER: Callee = Callee {
        addr: 0x0010_bbf8,
        f: ee::s00::ComputerActorRider_FindBoardPressMarker,
    };

    pub const FIND_JUMP_MARKER: Callee = Callee {
        addr: 0x0010_b980,
        f: ee::s00::ComputerActorRider_FindJumpMarker,
    };

    pub const SELECT_TARGET_RIDER: Callee = Callee {
        addr: 0x0010_bd10,
        f: ee::s00::ComputerActorRider_SelectTargetRider,
    };
}

// ---------------------------------------------------------------------------
// Riders.

/// `bool ComputerActorRider_IsRivalTarget(ComputerActor *self, uint id)`
/// (0x10d9e8): false unless the actor has a rider (+0x18 -> +0xf0) and its id
/// (+0xf8) equals `id`.
pub fn computer_actor_rider_is_rival_target(c: &mut Cpu) {
    c.cycles += 4;
    let actor = Addr(abi::arg(c, 0));
    let rider = actor.off(0x18).lw(c);
    c.g[V1][0] = rider;
    let has_rider = Addr(rider as u32).off(0xf0).lw(c);
    let mut result = 0u64;
    if has_rider != 0 {
        c.cycles += 3;
        let diff = Addr(rider as u32).off(0xf8).lw(c) ^ c.g[A1][0];
        result = (diff == 0) as u64;
    }
    c.cycles += 2;
    c.g[A0][0] = result;
    c.g[V0][0] = result;
    abi::ret(c);
}

/// `float Rider_GetAIPathLookAheadDist(Rider *rider, int flag)` (0x112588):
/// 796.0 when `flag` is set, 200.0 otherwise.
pub fn rider_get_ai_path_look_ahead_dist(c: &mut Cpu) {
    c.cycles += 4;
    let bits = if abi::arg64(c, 1) != 0 {
        0x4447_0000u32
    } else {
        c.cycles += 2;
        0x4348_0000u32
    };
    c.g[AT][0] = bits as u64;
    c.f[0] = bits; // mtc1 f0, at
    c.cycles += 2;
    abi::ret(c);
}

/// `void Rider_SaveAIPathLookAhead(Rider *rider)` (0x1125a8): copies the
/// 16-byte look-ahead quad at +0x4a0 to +0x4b0.
pub fn rider_save_ai_path_look_ahead(c: &mut Cpu) {
    let rider = Addr(abi::arg(c, 0));
    c.cycles += 3;
    let v = rider.off(0x4a0).read_vec4(c);
    abi::set_gpr128(c, V0, v.to_bits());
    rider.off(0x4b0).write_vec4(c, v);
    abi::ret(c);
}

pub(crate) const COMPUTER_ACTOR_RIDER_INIT: Callee = Callee {
    addr: 0x0010_8e88,
    f: ssx3_ee::s00::f_00108e88,
};

pub(crate) const COMPUTER_ACTOR_RIDER_INIT_DIFFICULTY: Callee = Callee {
    addr: 0x0010_c758,
    f: ssx3_ee::s00::f_0010c758,
};

pub(crate) const AI_RESET_RIDER_RELATIONS: Callee = Callee {
    addr: 0x0010_f3b8,
    f: ssx3_ee::s00::f_0010f3b8,
};

pub(crate) const AIR_PREDICTOR_RESET: Callee = Callee {
    addr: 0x0011_3198,
    f: ssx3_ee::s00::f_00113198,
};

pub(crate) const GET_HUMAN_RIDER: Callee = Callee {
    addr: 0x0010_deb0,
    f: ssx3_ee::s00::f_0010deb0,
};

pub(crate) const GET_DIST_TO_FINISH: Callee = Callee {
    addr: 0x0011_3128,
    f: ssx3_ee::s00::f_00113128,
};

// ---------------------------------------------------------------------------
// Computer actor riders.

/// `void ComputerActorRider_Init_2(ComputerActorRider *self)` (0x108f88):
/// tail wrapper around `ComputerActorRider_Init`.
pub fn computer_actor_rider_init_2(c: &mut Cpu) {
    call::tail_wrapper(
        c,
        None,
        Target::Direct(COMPUTER_ACTOR_RIDER_INIT),
        0x0010_8f98,
    );
}

/// `void ComputerActorRider_InitDifficulty_2(ComputerActorRider *self)`
/// (0x10cab8): tail wrapper around `ComputerActorRider_InitDifficulty`.
pub fn computer_actor_rider_init_difficulty_2(c: &mut Cpu) {
    call::tail_wrapper(
        c,
        None,
        Target::Direct(COMPUTER_ACTOR_RIDER_INIT_DIFFICULTY),
        0x0010_cac8,
    );
}

/// `bool ComputerActorRider_IsFarBehindHuman(ComputerActorRider *self)`
/// (0x10d938): true when the human rider's finish distance minus this actor's
/// rider's (+0x4d0) is below the rider's threshold (+0xdc).
pub fn computer_actor_rider_is_far_behind_human(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        GET_HUMAN_RIDER.call(c, 0x0010_d94c)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 10;
            c.g[A0][0] = c.g[V0][0];
            GET_DIST_TO_FINISH.call(c, 0x0010_d95c)?;
            let rider = Addr((c.g[S0][0] as u32).wrapping_add(0x18)).lw(c);
            c.g[V1][0] = rider;
            let limit = Addr(rider as u32).off(0x4d0).read_fpu(c);
            let threshold = Addr(rider as u32).off(0xdc).read_fpu(c);
            abi::set_fpr(c, 1, limit);
            abi::set_fpr(c, 2, threshold);
            let diff = FpuF32(c.f[0]) - limit;
            abi::set_fpr(c, 0, diff);
            let lt = abi::c_lt_s(c, diff, threshold);
            c.g[V0][0] = 0x1;
            if !lt {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool ComputerActorRider_IsFarAheadOfHuman(ComputerActorRider *self)`
/// (0x10d990): true when the this actor's rider's threshold (+0xe0) is below
/// the human rider's finish distance minus the actor's rider's distance
/// (+0x4d0).
pub fn computer_actor_rider_is_far_ahead_of_human(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        GET_HUMAN_RIDER.call(c, 0x0010_d9a4)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 10;
            c.g[A0][0] = c.g[V0][0];
            GET_DIST_TO_FINISH.call(c, 0x0010_d9b4)?;
            let rider = Addr((c.g[S0][0] as u32).wrapping_add(0x18)).lw(c);
            c.g[V1][0] = rider;
            let limit = Addr(rider as u32).off(0x4d0).read_fpu(c);
            let threshold = Addr(rider as u32).off(0xe0).read_fpu(c);
            abi::set_fpr(c, 1, limit);
            abi::set_fpr(c, 2, threshold);
            let diff = FpuF32(c.f[0]) - limit;
            abi::set_fpr(c, 0, diff);
            let lt = abi::c_lt_s(c, threshold, diff);
            c.g[V0][0] = 0x1;
            if !lt {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_ResetRiderRelations_2(void *self)` (0x10f398): tail wrapper around
/// `AI_ResetRiderRelations`.
pub fn ai_reset_rider_relations_2(c: &mut Cpu) {
    call::tail_wrapper(
        c,
        None,
        Target::Direct(AI_RESET_RIDER_RELATIONS),
        0x0010_f3a8,
    );
}

// ---------------------------------------------------------------------------
// Air predictor.

/// `AirPredictor *AirPredictor_ctor(AirPredictor *self)` (0x113138): set
/// `self->+0x30` to -1, run `AirPredictor_Reset`, and return `self`.
pub fn air_predictor_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] |= 0xffff;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(c.g[S0][0] as u32).off(0x30).sw(c, c.g[V0][0]);
        AIR_PREDICTOR_RESET.call(c, 0x0011_3158)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int AI_CountRidersOnPath(AI *ai, int path)` (0x10fc30): count riders
/// whose `vtable[0x48/0x4c]` query returns non-null and whose `+0xab8`
/// path id equals `path`.
pub fn ai_count_riders_on_path(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = 0;
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
        c.g[S4][0] = c.g[A1][0];
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
            loop {
                c.cycles += 9;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x6c0).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x4c).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call::call_ptr(c, target, 0x0010_fc84)?;
                let no_rider = c.g[V0][0] == 0;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                if !no_rider {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xab8).lw(c);
                    c.g[V1][0] ^= c.g[S4][0];
                    if c.g[V1][0] == 0 {
                        c.g[S1][0] = c.g[A0][0];
                    }
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0010_fc68);
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S1][0];
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

// ---------------------------------------------------------------------------
// Race progress.

/// The race-event record list shared by the riders (`D_004A5D60`).
pub(crate) const RACE_EVENTS: u32 = 0x004a_5d60;

/// `void Rider_UpdateRaceProgress(Rider* r)` (0x112338): compares the shared
/// race-event list with the rider's snapshot, refreshes the distance to the
/// finish, picks a new race path when the current one is 500 units behind, ends
/// within 200 units, or its event test reports one, then tells the race-event
/// handler about every entry (matched ones with `found`, the unmatched shared
/// list with 1,0).
///
/// Tick rate: 500 units off the path, the reselect is forced once a second
/// (`now % reselect_period`, 60 ticks at 60 Hz).
pub fn rider_update_race_progress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0xa0);
        let sp = sp(c);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x50);
        c.g[A0][0] = 0x004a_0000;
        frame.save(c, S0, 0x90);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d60));
        frame.save(c, S5, 0x40);
        c.g[A1][0] = sx(p(c.g[S2][0] as u32, rider::RACE_ENTRIES));
        frame.save(c, S6, 0x30);
        frame.save_ra(c, 0x20);
        c.g[A2][0] = c.lw(p(c.g[S2][0] as u32, rider::RACE_ENTRY_COUNT));
        c.g[A2][0] = shl32(c.g[A2][0], 4);
        call_ptr(c, callee::EA_MEM_COPY, 0x00112378)?;
        let rider = c.g[S2][0] as u32;
        c.g[A1][0] = sx(p(sp, 0x10));
        c.g[A2][0] = sx(p(sp, 0x14));
        c.g[S3][0] = c.lw(p(rider, rider::RACE_ENTRY_COUNT));
        c.g[S4][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A3][0] = c.g[SP][0];
        callee::RIDER_UPDATE_DIST_TO_FINISH.call(c, 0x00112398)?;
        let df = ldf(c, 1, p(rider, rider::DIST_TO_FINISH));
        setf(c, 0, FpuF32::ZERO);
        let has_dist = lt(c, FpuF32::ZERO, df);
        ldf(c, 1, p(sp, 0x10));
        if has_dist {
            // --- the path selection
            let mut new_path = 0u64;
            'select: {
                c.cycles += 6;
                c.g[AT][0] = 0x43fa_0000;
                c.mtc1(1, 0);
                let far = lt(c, fr(c, 0), fr(c, 1));
                c.g[S0][0] = c.lw(p(rider, rider::RACE_PATH));
                if far {
                    // The race clock modulo 60 check.
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(gp(c, -0x848));
                    // Once per second: 60 ticks at 60 Hz, `rate` otherwise.
                    c.g[A1][0] = reselect_period(c);
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x84));
                    c.g[A0][0] = c.lw(p(c.g[V1][0] as u32, 0xc));
                    c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 8));
                    c.div(0, 2, 5);
                    c.g[V1][0] = c.hi[0];
                    let not_minute = c.g[V1][0] != 0;
                    c.g[A0][0] = c.g[S2][0];
                    if !not_minute {
                        c.cycles += 4;
                        c.g[A1][0] = 0;
                        callee::RIDER_SELECT_RACE_PATH.call(c, 0x001123fc)?;
                        new_path = c.lw(p(rider, rider::RACE_PATH));
                        break 'select;
                    }
                }
                // b_112440: the path's own end check.
                c.cycles += 10;
                c.g[A0][0] = c.lw(p(rider, rider::RACE_PATH));
                call_ptr(c, callee::PATH_GET_LENGTH, 0x0011240c)?;
                c.g[AT][0] = 0x4348_0000;
                c.mtc1(1, 1);
                ldf(c, 2, p(sp, 0x14));
                let left = fr(c, 0) - fr(c, 1);
                setf(c, 0, left);
                if !lt(c, left, fr(c, 2)) {
                    c.g[A1][0] = c.lw(p(rider, rider::RACE_PATH));
                    // fall into b_11246c
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = 1;
                    callee::RIDER_SELECT_RACE_PATH.call(c, 0x00112438)?;
                    new_path = c.lw(p(rider, rider::RACE_PATH));
                    break 'select;
                }
                // b_11246c: the path's event test.
                c.cycles += 9;
                ldf(c, 12, p(rider, rider::DIST_TO_FINISH));
                c.g[V1][0] = c.lw(p(c.g[A1][0] as u32, 0x34));
                ldf(c, 13, p(rider, rider::DIST_ESTIMATE));
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x10));
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x14));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = add32(c.g[A1][0], c.g[A0][0]);
                call_ptr(c, target, 0x0011245c)?;
                let no_event = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S2][0];
                if !no_event {
                    c.cycles += 2;
                    c.g[A1][0] = 0;
                    callee::RIDER_SELECT_RACE_PATH.call(c, 0x0011246c)?;
                }
                c.cycles += 1;
                new_path = c.lw(p(rider, rider::RACE_PATH));
            }
            // The path changed: reset the new path's cursor and re-measure.
            c.cycles += 2;
            let same = new_path == c.g[S0][0];
            c.g[V0][0] = u64::MAX;
            if !same {
                c.cycles += 7;
                c.g[V1][0] = c.lw(p(rider, rider::RACE_PATH_CURSOR));
                c.g[A1][0] = c.g[S4][0];
                c.g[A2][0] = c.g[S1][0];
                c.sw(p(c.g[V1][0] as u32, 0x14), c.g[V0][0]);
                c.g[A0][0] = c.g[S2][0];
                c.g[A3][0] = c.g[SP][0];
                callee::RIDER_UPDATE_DIST_TO_FINISH.call(c, 0x00112494)?;
            }
        }
        // --- the event loops
        c.cycles += 3;
        c.g[V0][0] = c.lw(p(rider, rider::RACE_ENTRY_COUNT));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = 0;
        if !none {
            c.cycles += 4;
            c.g[S5][0] = sx(p(rider, rider::RACE_ENTRIES));
            c.g[S6][0] = 0x004a_0000;
            c.g[S4][0] = sx(p(rider, rider::RACE_ENTRIES + 4));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            loop {
                c.cycles += 3;
                c.g[A2][0] = 0;
                let no_path = (c.g[S3][0] as i64) <= 0;
                c.g[A1][0] = shl32(c.g[S0][0], 4);
                if !no_path {
                    c.cycles += 3;
                    c.g[T0][0] = add32(c.g[S5][0], c.g[A1][0]);
                    c.g[A3][0] = add32(c.g[S4][0], c.g[A1][0]);
                    c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5d60));
                    loop {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(c.g[T0][0] as u32);
                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                        let mut matched = false;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                        } else {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw(c.g[A3][0] as u32);
                            c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 4));
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                            } else {
                                c.cycles += 2;
                                c.sw(c.g[A0][0] as u32, 0);
                                matched = true;
                            }
                        }
                        if matched {
                            break;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x001124c8);
                        }
                    }
                }
                c.cycles += 11;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4dc));
                c.g[A2][0] = ((c.g[A2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A0][0] = c.g[S2][0];
                c.g[A3][0] = 0x1;
                c.g[S0][0] = c.g[S1][0];
                callee::RIDER_HANDLE_RACE_PATH_EVENT.call(c, 0x00112518)?;
                c.g[V0][0] = c.lw(p(rider, rider::RACE_ENTRY_COUNT));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001124b0);
                }
            }
        }
        // --- the leftover shared list
        c.cycles += 2;
        let none2 = (c.g[S3][0] as i64) <= 0;
        c.g[V0][0] = 0x004a_0000;
        if !none2 {
            c.cycles += 2;
            c.g[S0][0] = c.g[S3][0];
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d60));
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                let empty = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S2][0];
                if !empty {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = 1;
                    c.g[A3][0] = 0;
                    callee::RIDER_HANDLE_RACE_PATH_EVENT.call(c, 0x00112554)?;
                }
                c.cycles += 3;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                let more = c.g[S0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x00112538);
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AI path progress.

/// `void Rider_UpdateAIPathProgress(Rider* r)` (0x1125c0): keeps the AI path
/// progress (arc length along the AI path, +0x4c4) and its look-ahead point
/// current, selects a new AI path when the current one is 500 units behind,
/// ends within 200 units or its event test fires, then computes the AI heading
/// (+0x4cc) from the look-ahead point's offset (`atan2(dy, dx)`, wrapped to
/// (-pi, pi]).
///
/// Tick rate: 500 units off the path, the reselect is forced once a second
/// (`now % reselect_period`, 60 ticks at 60 Hz).
pub fn rider_update_ai_path_progress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        let sp = sp(c);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 22, 0x50);
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        let rider = c.g[S0][0] as u32;
        ldf(c, 0, p(rider, rider::AI_PROGRESS));
        c.swc1(0, p(rider, rider::AI_ARC_LEN));
        callee::RIDER_RETURN_ZERO.call(c, 0x001125ec)?;
        c.g[S1][0] = c.g[V0][0] ^ 0x1;
        let no_search = c.g[S1][0] == 0;
        c.g[A2][0] = sx(p(rider, rider::AI_POINT_TARGET));
        if !no_search {
            c.cycles += 8;
            c.g[A0][0] = c.lw(p(rider, rider::AI_PATH));
            c.g[A1][0] = sx(p(rider, rider::POSITION));
            c.g[T0][0] = c.lw(p(rider, rider::AI_PATH_CURSOR));
            c.g[A3][0] = sx(p(rider, rider::AI_SEARCH_DIST));
            c.g[T1][0] = c.g[S1][0];
            call_ptr(c, callee::PATH_GET_CLOSEST_ON_PATH, 0x00112610)?;
            c.swc1(0, p(rider, rider::AI_PROGRESS));
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.lw(p(rider, rider::AI_PATH));
            c.g[A1][0] = sx(p(rider, rider::POSITION));
            c.g[A3][0] = sx(p(rider, rider::AI_SEARCH_DIST));
            c.g[T0][0] = 0;
            call_ptr(c, callee::PATH_GET_CLOSEST, 0x0011262c)?;
            c.swc1(0, p(rider, rider::AI_PROGRESS));
        }
        // b_11273c: the look-ahead point.
        c.cycles += 20;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::RIDER_AI_LOOK_AHEAD.call(c, 0x0011263c)?;
        let progress = ldf(c, 12, p(rider, rider::AI_PROGRESS));
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.lw(p(rider, rider::AI_PATH));
        c.g[A2][0] = 0;
        setf(c, 12, progress + fr(c, 0));
        call_ptr(c, callee::PATH_GET_POINT_AT_DISTANCE, 0x00112654)?;
        c.lq(2, c.g[SP][0] as u32);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.sq(2, p(rider, rider::AI_POINT));
        callee::RIDER_SAVE_AI_LOOK_AHEAD.call(c, 0x00112668)?;
        ldf(c, 1, p(rider, rider::DIST_TO_FINISH));
        c.mtc1(0, 0);
        let has_dist = lt(c, fr(c, 0), fr(c, 1));
        if has_dist {
            // Re-select the AI path when it ends too soon.
            let mut selected = false;
            'select: {
                c.cycles += 7;
                ldf(c, 1, p(rider, rider::AI_SEARCH_DIST));
                c.g[AT][0] = 0x43fa_0000;
                c.mtc1(1, 0);
                let far = lt(c, fr(c, 0), fr(c, 1));
                c.g[V0][0] = c.lw(gp(c, -0x848));
                // (The generated body runs the minute check when
                // 1000 < search_dist, i.e. when far.)
                if far {
                    // The race clock modulo 60 check.
                    c.cycles += 3;
                    // Once per second: 60 ticks at 60 Hz, `rate` otherwise.
                    c.g[A1][0] = reselect_period(c);
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x84));
                    c.g[A0][0] = c.lw(p(c.g[V1][0] as u32, 0xc));
                    c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 8));
                    c.div(0, 2, 5);
                    c.g[V1][0] = c.hi[0];
                    let not_minute = c.g[V1][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if !not_minute {
                        c.cycles += 4;
                        c.g[A1][0] = 0;
                        callee::RIDER_SELECT_AI_PATH.call(c, 0x001126cc)?;
                        ldf(c, 0, p(rider, rider::AI_POINT_TARGET));
                        selected = true;
                        break 'select;
                    }
                }
                // b_112710: the path's own end check.
                c.cycles += 10;
                c.g[A0][0] = c.lw(p(rider, rider::AI_PATH));
                call_ptr(c, callee::PATH_GET_LENGTH, 0x001126dc)?;
                c.g[AT][0] = 0x4348_0000;
                c.mtc1(1, 1);
                ldf(c, 13, p(rider, rider::AI_PROGRESS));
                let left = fr(c, 0) - fr(c, 1);
                setf(c, 0, left);
                if lt(c, left, fr(c, 13)) {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 1;
                    callee::RIDER_SELECT_AI_PATH.call(c, 0x00112708)?;
                    ldf(c, 0, p(rider, rider::AI_POINT_TARGET));
                    selected = true;
                    break 'select;
                }
                c.g[A1][0] = c.lw(p(rider, rider::AI_PATH));
            }
            if !selected {
                // The path's event test: vtable slot 2 of the path.
                c.cycles += 8;
                ldf(c, 12, p(rider, rider::AI_ARC_LEN));
                c.g[V1][0] = c.lw(p(c.g[A1][0] as u32, 0x34));
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x10));
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x14));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = add32(c.g[A1][0], c.g[A0][0]);
                call_ptr(c, target, 0x00112728)?;
                let no_event = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if !no_event {
                    c.cycles += 2;
                    c.g[A1][0] = 0;
                    callee::RIDER_SELECT_AI_PATH.call(c, 0x00112738)?;
                }
                c.cycles += 1;
                ldf(c, 0, p(rider, rider::AI_POINT_TARGET));
            }
        } else {
            ldf(c, 0, p(rider, rider::AI_POINT_TARGET));
        }
        // b_1127d0: the AI heading from the look-ahead offset.
        'heading: {
            c.cycles += 9;
            let point = ldf(c, 2, p(rider, rider::AI_POINT));
            c.mtc1(0, 22);
            let dx = point - fr(c, 0);
            setf(c, 21, dx);
            let dy_raw = ldf(c, 1, p(rider, rider::AI_POINT + 4));
            let base_y = ldf(c, 0, p(rider, rider::AI_POINT_TARGET + 4));
            let dy = dy_raw - base_y;
            setf(c, 20, dy);
            let dx_zero = eq(c, fr(c, 21), fr(c, 22));
            if dx_zero {
                c.cycles += 4;
                let dy_zero = eq(c, fr(c, 20), fr(c, 22));
                setf(c, 0, fr(c, 20));
                if !dy_zero {
                    c.cycles += 4;
                    if !le(c, fr(c, 22), fr(c, 20)) {
                        let a = gp(c, -0x7c90);
                        let v = ldf(c, 0, a);
                        setf(c, 20, v);
                        c.swc1(20, p(rider, rider::AI_HEADING));
                        break 'heading;
                    }
                    c.cycles += 3;
                    let a = gp(c, -0x7c8c);
                    let v = ldf(c, 0, a);
                    setf(c, 20, v);
                    c.swc1(20, p(rider, rider::AI_HEADING));
                    break 'heading;
                }
            } else {
                c.cycles += 7;
                c.div_s(12, 20, 21);
                call_ptr(c, callee::MATH_ATAN, 0x00112798)?;
                let neg = lt(c, fr(c, 21), fr(c, 22));
                setf(c, 1, fr(c, 0));
                if neg {
                    c.cycles += 3;
                    if lt(c, fr(c, 22), fr(c, 20)) {
                        c.cycles += 3;
                        let pi = ldf(c, 0, gp(c, -0x7c88));
                        setf(c, 1, fr(c, 1) + pi);
                    } else {
                        c.cycles += 2;
                        let pi = ldf(c, 0, gp(c, -0x7c84));
                        setf(c, 1, fr(c, 1) - pi);
                    }
                }
                c.cycles += 1;
                setf(c, 0, fr(c, 1));
            }
            c.cycles += 1;
            c.swc1(0, p(rider, rider::AI_HEADING));
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 22, 0x50);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `Math_Atan`, through the function table (and its override slot).
pub(crate) const MATH_ATAN: u32 = 0x0031_c228;

/// A `gp_2`-relative address (globals and the `.lit4` constant pool).
#[inline(always)]
pub(crate) fn gp_2(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

#[inline(always)]
pub(crate) fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}

/// FPU register `n`.
#[inline(always)]
pub(crate) fn fr_2(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

/// `fN = v`.
#[inline(always)]
pub(crate) fn setf_2(c: &mut Cpu, n: usize, v: FpuF32) -> FpuF32 {
    c.f[n] = v.0;
    v
}

/// `lwc1 fN, a`.
#[inline(always)]
pub(crate) fn ldf_2(c: &mut Cpu, n: usize, a: u32) -> FpuF32 {
    let v = Addr(a).read_fpu(c);
    setf_2(c, n, v)
}

/// `li at, bits; mtc1 at, fN` (a constant moved through `at`, as the original
/// does); returns the register's value.
#[inline(always)]
pub(crate) fn li_f(c: &mut Cpu, n: usize, bits: u32) -> FpuF32 {
    c.g[AT][0] = bits as u64;
    c.mtc1(1, n);
    fr_2(c, n)
}

/// `c.lt.s fA, fB`, leaving FCR31 set.
#[inline(always)]
pub(crate) fn lt_2(c: &mut Cpu, a: usize, b: usize) -> bool {
    let r = fr_2(c, a).lt(fr_2(c, b));
    abi::set_fcc(c, r);
    r
}

/// `c.le.s fA, fB`, leaving FCR31 set.
#[inline(always)]
pub(crate) fn le_2(c: &mut Cpu, a: usize, b: usize) -> bool {
    let r = fr_2(c, a).le(fr_2(c, b));
    abi::set_fcc(c, r);
    r
}

/// `c.eq.s fA, fB`, leaving FCR31 set.
#[inline(always)]
pub(crate) fn eq_2(c: &mut Cpu, a: usize, b: usize) -> bool {
    let r = fr_2(c, a).eq_s(fr_2(c, b));
    abi::set_fcc(c, r);
    r
}

/// `.lit4` constants of the angle wrap (`gp_2` offsets).
pub(crate) const TWO_PI: i32 = -0x7d04;
// 6.2831855
pub(crate) const INV_TWO_PI: i32 = -0x7d08;
// 0.15915494
pub(crate) const PI_SUB: i32 = -0x7d0c;
// 3.1415927, for y <= 0 with x < 0
pub(crate) const PI_ADD: i32 = -0x7d10;
// 3.1415927, for y > 0 with x < 0
pub(crate) const HALF_PI_POS: i32 = -0x7d14;
// 1.5707964
pub(crate) const HALF_PI_NEG: i32 = -0x7d18;
// -1.5707964
pub(crate) const WRONG_WAY_LIMIT: i32 = -0x7d00;
// 1.0471976 (pi/3)

/// The function's epilogue: `lq s0`, the other saved callee-saved registers,
/// `ra`, then `sp += 0x80` and `jr ra`. `extra` carries the block's own cycles
/// before the shared tail (1 on the normal path, 2 at the predictor return).
pub(crate) fn leave(c: &mut Cpu, frame: Frame, extra: u64) {
    c.cycles += extra;
    frame.restore(c, S0, 0x50);
    frame.restore_fpr(c, 22, 0x70);
    frame.restore_fpr(c, 21, 0x68);
    frame.restore_fpr(c, 20, 0x60);
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore_ra(c, 0x20);
    c.cycles += 8;
    frame.close(c);
}

/// `int ComputerActorRider_ShouldResetPath(ComputerActorRider* self)`
/// (0x10d1a0): `d = wrap(heading - atan2(vel.y, vel.x))`. While the course
/// runs the other way (`dir < 0`) a heading error over pi/3 counts a
/// wrong-way frame (`+0xe78`), otherwise the counter clears. Then the rider
/// state machine: main state 2 adds a stuck frame (`+0xe74`); state 4 with
/// wrong-way frames resets (returns 1); state 1 or a speed below 1000 clears
/// the counters, and a predictor height over 30 resets; anything else adds a
/// stuck frame. Reports stuck when the wrong-way frames reach 0x3d or the
/// stuck counter 0xdf, except in sub state 8, where only the wrong-way
/// direction counts.
pub fn computer_actor_rider_should_reset_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x50);
        frame.save_fpr(c, 22, 0x70);
        let actor = r32(c, A0);
        frame.save_fpr(c, 21, 0x68);
        frame.save_fpr(c, 20, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        c.mtc1(0, 22); // f22 = 0
        let rider = lw(c, p(actor, computer::RIDER)) as u32;
        let sp0 = sp(c);
        c.lq(3, p(rider, rider::VELOCITY_2));
        c.sq(3, sp0);
        c.lq(3, p(rider, rider::VELOCITY_2));
        ldf_2(c, 21, p(sp0, 4)); // f21 = vel.y
        c.sq(3, p(sp0, 0x10));
        ldf_2(c, 20, p(sp0, 0x10)); // f20 = vel.x
                                    // The angle in f12: 0 or ±pi/2 when x is 0, else atan(y/x) with the
                                    // quadrant fixup added in line. Reaching the shared tail through
                                    // `b_10d258` costs one cycle and reloads v0.
        let mut via_shared = true;
        if !eq_2(c, 20, 22) {
            c.cycles += 7;
            setf_2(c, 12, fr_2(c, 21).div(fr_2(c, 20))); // y / x
            call_ptr(c, MATH_ATAN, 0x0010_d228)?;
            let x_negative = lt_2(c, 20, 22);
            setf_2(c, 12, fr_2(c, 0)); // mov.s f12, f0
            if x_negative {
                c.cycles += 3;
                if lt_2(c, 22, 21) {
                    c.cycles += 3;
                    let pi = ldf_2(c, 0, gp_2(c, PI_ADD));
                    setf_2(c, 12, fr_2(c, 12) + pi);
                } else {
                    c.cycles += 2;
                    let pi = ldf_2(c, 0, gp_2(c, PI_SUB));
                    setf_2(c, 12, fr_2(c, 12) - pi);
                }
            }
        } else {
            c.cycles += 4;
            let y_zero = eq_2(c, 21, 22);
            setf_2(c, 12, fr_2(c, 21));
            if !y_zero {
                c.cycles += 4;
                let up = le_2(c, 22, 21);
                ldf_2(c, 12, gp_2(c, HALF_PI_NEG));
                c.g[V0][0] = lw(c, p(actor, computer::RIDER));
                via_shared = false;
                if up {
                    c.cycles += 3;
                    ldf_2(c, 12, gp_2(c, HALF_PI_POS));
                }
            }
        }
        if via_shared {
            c.cycles += 1;
            c.g[V0][0] = lw(c, p(actor, computer::RIDER));
        }
        // d = x - floor(x / (2 pi) + 1/2) * 2 pi in f0; f1 keeps the unwrapped
        // error, which the tail only uses for the absolute test.
        c.cycles += 12;
        let inv = ldf_2(c, 0, gp_2(c, INV_TWO_PI));
        ldf_2(c, 1, p(rider, rider::HEADING_2));
        setf_2(c, 2, FpuF32(0x3f00_0000)); // 0.5
        let err = setf_2(c, 1, fr_2(c, 1) - fr_2(c, 12));
        let turns = setf_2(c, 0, err * inv + fr_2(c, 2));
        let whole = setf_2(c, 2, FpuF32::from_i32(turns.to_i32()));
        if lt_2(c, 0, 2) {
            c.cycles += 3;
            li_f(c, 0, 0x3f80_0000); // 1.0
            setf_2(c, 2, whole - fr_2(c, 0));
        }
        c.cycles += 5;
        let two_pi = ldf_2(c, 0, gp_2(c, TWO_PI));
        let dir = lw(c, p(actor, computer::DIR));
        setf_2(c, 0, fr_2(c, 1) - fr_2(c, 2) * two_pi);
        if (dir as i64) >= 0 {
            c.cycles += 1;
            c.sw(p(actor, computer::WRONG_TIME), 0);
        } else {
            c.cycles += 6;
            setf_2(c, 1, fr_2(c, 0).abs());
            let limit = ldf_2(c, 0, gp_2(c, WRONG_WAY_LIMIT));
            if lt_2(c, 0, 1) {
                c.cycles += 4;
                c.g[V0][0] = lw(c, p(actor, computer::WRONG_TIME));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                c.sw(p(actor, computer::WRONG_TIME), c.g[V0][0]);
            } else {
                c.sw(p(actor, computer::WRONG_TIME), 0);
            }
        }
        // wrong = wrongTime >= 0x3d (61 frames, ~1 s: a duration, not a count)
        c.cycles += 8;
        let wrong_time = lw(c, p(actor, computer::WRONG_TIME));
        c.g[V0][0] = wrong_time;
        c.g[A0][0] = lw(c, p(actor, computer::RIDER));
        let wrong_limit = crate::tick::ticks_from_frames60(c, 61) as i64;
        c.g[V0][0] = ((wrong_time as i64) < wrong_limit) as u64;
        c.g[S2][0] = c.g[V0][0] ^ 1;
        callee::GET_MAIN_STATE.call(c, 0x0010_d2ec)?;
        c.g[V1][0] = 2;
        let mut add_stuck_frame = false;
        if c.g[V0][0] != c.g[V1][0] {
            // State 4 with wrong-way frames resets at once.
            c.cycles += 5;
            c.g[A0][0] = lw(c, p(actor, computer::RIDER));
            callee::GET_MAIN_STATE.call(c, 0x0010_d30c)?;
            c.g[V1][0] = 4;
            if c.g[V0][0] == c.g[V1][0] {
                c.cycles += 3;
                c.g[V0][0] = lw(c, p(actor, computer::WRONG_TIME));
                let wrong_way = (c.g[V0][0] as i64) > 0;
                c.g[V0][0] = 1;
                if wrong_way {
                    leave(c, frame, 1);
                    return Ok(());
                }
                c.cycles += 1;
                c.g[A0][0] = lw(c, p(actor, computer::RIDER));
            }
            // State 1 or a speed below 1000 clears the counters.
            c.cycles += 4;
            c.g[S1][0] = 1;
            callee::GET_MAIN_STATE.call(c, 0x0010_d330)?;
            let mut skip_predictor = false;
            if c.g[V0][0] == c.g[S1][0] {
                c.sw(p(actor, computer::STUCK_TIME), 0);
            } else {
                c.cycles += 8;
                c.g[V0][0] = lw(c, p(actor, computer::RIDER));
                let thousand = li_f(c, 1, 0x447a_0000); // 1000.0
                ldf_2(c, 0, p(c.g[V0][0] as u32, rider::SPEED));
                if lt_2(c, 0, 1) {
                    c.cycles += 1;
                    c.sw(p(actor, computer::STUCK_TIME), 0);
                } else {
                    // One more stuck frame instead.
                    c.g[V0][0] = lw(c, p(actor, computer::STUCK_TIME));
                    add_stuck_frame = true;
                    skip_predictor = true;
                }
                let _ = thousand;
            }
            if !skip_predictor {
                // State 1 with a predictor above 30 resets.
                c.cycles += 5;
                c.g[S2][0] = 0;
                c.g[A0][0] = lw(c, p(actor, computer::RIDER));
                let one = c.g[S1][0];
                callee::GET_MAIN_STATE.call(c, 0x0010_d368)?;
                if c.g[V0][0] == one {
                    c.cycles += 9;
                    c.g[V1][0] = lw(c, p(actor, computer::RIDER));
                    li_f(c, 1, 0x41f0_0000); // 30.0
                    c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, rider::AIR_PREDICTOR));
                    ldf_2(c, 0, p(c.g[V0][0] as u32, air_predictor::HEIGHT));
                    let high = lt_2(c, 1, 0);
                    c.g[V0][0] = 1;
                    if high {
                        leave(c, frame, 2);
                        return Ok(());
                    }
                }
            }
        } else {
            // State 2 adds a stuck frame and clears the wrong-way flag.
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(actor, computer::STUCK_TIME));
            c.g[S2][0] = 0;
            add_stuck_frame = true;
        }
        if add_stuck_frame {
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.sw(p(actor, computer::STUCK_TIME), c.g[V0][0]);
        }
        // stuck = wrong || stuckTime >= 0xdf (223 frames, ~3.7 s)
        c.cycles += 2;
        let wrong = c.g[S2][0] != 0;
        c.g[S1][0] = 0;
        if wrong {
            c.cycles += 1;
            c.g[S1][0] = 1;
        } else {
            c.cycles += 4;
            c.g[V0][0] = lw(c, p(actor, computer::STUCK_TIME));
            let stuck_limit = crate::tick::ticks_from_frames60(c, 223) as i64;
            c.g[V0][0] = ((c.g[V0][0] as i64) < stuck_limit) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[S1][0] = 1;
            }
        }
        // Sub state 8 keeps the verdict; any other needs dir < 0.
        c.cycles += 5;
        c.g[A0][0] = lw(c, p(actor, computer::RIDER));
        callee::GET_SUB_STATE.call(c, 0x0010_d3c8)?;
        c.g[V1][0] = 8;
        let reset_sub = c.g[V0][0] == c.g[V1][0];
        c.g[V0][0] = c.g[S1][0];
        if !reset_sub {
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                c.g[V0][0] = 0;
                c.cycles += 2;
                c.g[S1][0] = c.g[V0][0];
                c.g[V0][0] = c.g[S1][0];
            } else {
                c.cycles += 2;
                c.g[V0][0] = lw(c, p(actor, computer::DIR));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 0) as u64;
                c.cycles += 2;
                c.g[S1][0] = c.g[V0][0];
                c.g[V0][0] = c.g[S1][0];
            }
        }
        leave(c, frame, 1);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider base main/sub-state dispatch (`switch` over `.rodata` jump tables).

/// One table entry of the `RiderBase_*MainState` / `..._HandleSubStateInput`
/// dispatchers: the handler a state calls, the sub-object it receives and the
/// block it falls into afterwards.
#[derive(Clone, Copy)]
pub(crate) struct StateCase {
    /// The state handler (a `Rider<State>_*` function).
    handler: u32,
    /// Where the handler returns.
    ret: u32,
    /// Offset from the rider base to the handler's sub-object (0: the rider).
    off: u32,
    /// Cycles charged before the call.
    cycles: u64,
    /// True for the case that falls into the `+1`-cycle tail instead of
    /// straight into the epilogue.
    tail: bool,
}

/// `RiderBase_UpdateMainState` (0x111408): the update handler per main state.
pub(crate) const UPDATE_CASES: [StateCase; 6] = [
    StateCase {
        handler: 0x0013_d818,
        ret: 0x0011_1444,
        off: 0,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0013_9a20,
        ret: 0x0011_1454,
        off: 0x20,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0013_6e98,
        ret: 0x0011_1464,
        off: 0x30,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0013_6958,
        ret: 0x0011_1474,
        off: 0xa0,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0013_af28,
        ret: 0x0011_1484,
        off: 0xb0,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0013_91a8,
        ret: 0x0011_1494,
        off: 0x110,
        cycles: 2,
        tail: true,
    },
];

/// `RiderBase_PostUpdateMainState` (0x1114a0): the post-update handler per
/// main state.
pub(crate) const POST_CASES: [StateCase; 6] = [
    StateCase {
        handler: 0x0013_f178,
        ret: 0x0011_14dc,
        off: 0,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0013_9c88,
        ret: 0x0011_14ec,
        off: 0x20,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0013_6ee0,
        ret: 0x0011_14fc,
        off: 0x30,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0013_6978,
        ret: 0x0011_150c,
        off: 0xa0,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0013_bfa8,
        ret: 0x0011_151c,
        off: 0xb0,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0013_9528,
        ret: 0x0011_152c,
        off: 0x110,
        cycles: 2,
        tail: true,
    },
];

/// `RiderBase_HandleSubStateInput` (0x111728): the input handler per sub
/// state.
pub(crate) const SUB_CASES: [StateCase; 13] = [
    StateCase {
        handler: 0x0013_1620,
        ret: 0x0011_1764,
        off: 0x1c0,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0012_fc80,
        ret: 0x0011_1774,
        off: 0x1d0,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0012_e9b8,
        ret: 0x0011_1784,
        off: 0x1f0,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0012_e778,
        ret: 0x0011_1794,
        off: 0x200,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0012_f730,
        ret: 0x0011_17a4,
        off: 0x210,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0013_3308,
        ret: 0x0011_17b4,
        off: 0x230,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0012_bf68,
        ret: 0x0011_17c4,
        off: 0x290,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0013_1d30,
        ret: 0x0011_17d4,
        off: 0x2b0,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0012_cb68,
        ret: 0x0011_17e4,
        off: 0x2c0,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0012_f398,
        ret: 0x0011_17f4,
        off: 0x350,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0012_c678,
        ret: 0x0011_1804,
        off: 0x360,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0013_2a30,
        ret: 0x0011_1814,
        off: 0x370,
        cycles: 4,
        tail: false,
    },
    StateCase {
        handler: 0x0013_6508,
        ret: 0x0011_1824,
        off: 0x390,
        cycles: 2,
        tail: true,
    },
];

/// One of the state handler calls: `handler(this + off)`.
pub(crate) fn state_case_call(c: &mut Cpu, frame: Frame, case: StateCase) -> Flow {
    c.cycles += case.cycles;
    if case.off == 0 {
        c.g[A0][0] = c.g[A1][0];
    } else {
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(case.off));
    }
    call_ptr(c, case.handler, case.ret)?;
    if case.tail {
        c.cycles += 1;
    }
    frame.restore_ra(c, 0);
    state_case_exit(c, frame);
    Ok(())
}

/// `sp += 0x10; jr ra` (0x111498 / 0x111530 / 0x111828).
pub(crate) fn state_case_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 2;
    frame.close(c);
}

/// `RiderBase_UpdateMainState` (0x111408): run the main-state update handler
/// of `rider->mMainState` (0xde0); states past the table do nothing.
pub fn rider_base_update_main_state(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x10 };
        update_main_state_at(c, frame, c.pc)
    })
}

pub(crate) fn update_main_state_at(c: &mut Cpu, frame: Frame, pc: u32) -> Flow {
    match pc {
        0x0011_1408 => {
            c.cycles += 7;
            Frame::open(c, 0x10);
            c.g[A1][0] = c.g[A0][0];
            frame.save_ra(c, 0);
            c.g[V1][0] = Addr(p(r32(c, A1), 0xde0)).lw(c);
            c.g[V0][0] = (c.g[V1][0] < 6) as u64;
            let in_table = c.g[V0][0] != 0;
            c.g[V0][0] = 0x0045_0000;
            if in_table {
                update_main_state_table(c, frame)
            } else {
                // Not in the table: the `+1` tail and the epilogue.
                c.cycles += 1;
                frame.restore_ra(c, 0);
                state_case_exit(c, frame);
                Ok(())
            }
        }
        0x0011_1424 => update_main_state_table(c, frame),
        0x0011_143c => state_case_call(c, frame, UPDATE_CASES[0]),
        0x0011_144c => state_case_call(c, frame, UPDATE_CASES[1]),
        0x0011_145c => state_case_call(c, frame, UPDATE_CASES[2]),
        0x0011_146c => state_case_call(c, frame, UPDATE_CASES[3]),
        0x0011_147c => state_case_call(c, frame, UPDATE_CASES[4]),
        0x0011_148c => state_case_call(c, frame, UPDATE_CASES[5]),
        0x0011_1494 => {
            c.cycles += 1;
            frame.restore_ra(c, 0);
            state_case_exit(c, frame);
            Ok(())
        }
        0x0011_1498 => {
            state_case_exit(c, frame);
            Ok(())
        }
        other => {
            c.jump_out(other, 0x0011_1408, 0x0011_14a0);
            Ok(())
        }
    }
}

/// The jump-table read at 0x111424 (`0x450000 + 0x6b50 + state * 4`).
pub(crate) fn update_main_state_table(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[V1][0] = shl32_2(c.g[V1][0], 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6b50));
    c.g[V1][0] = add32_2(c.g[V1][0], c.g[V0][0]);
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    match r32(c, A0) {
        0x0011_143c => state_case_call(c, frame, UPDATE_CASES[0]),
        0x0011_144c => state_case_call(c, frame, UPDATE_CASES[1]),
        0x0011_145c => state_case_call(c, frame, UPDATE_CASES[2]),
        0x0011_146c => state_case_call(c, frame, UPDATE_CASES[3]),
        0x0011_147c => state_case_call(c, frame, UPDATE_CASES[4]),
        0x0011_148c => state_case_call(c, frame, UPDATE_CASES[5]),
        t => {
            c.jump_out(t, 0x0011_1408, 0x0011_14a0);
            Ok(())
        }
    }
}

/// `RiderBase_PostUpdateMainState` (0x1114a0): run the main-state
/// post-update handler of `rider->mMainState` (0xde0).
pub fn rider_base_post_update_main_state(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x10 };
        post_main_state_at(c, frame, c.pc)
    })
}

pub(crate) fn post_main_state_at(c: &mut Cpu, frame: Frame, pc: u32) -> Flow {
    match pc {
        0x0011_14a0 => {
            c.cycles += 7;
            Frame::open(c, 0x10);
            c.g[A1][0] = c.g[A0][0];
            frame.save_ra(c, 0);
            c.g[V1][0] = Addr(p(r32(c, A1), 0xde0)).lw(c);
            c.g[V0][0] = (c.g[V1][0] < 6) as u64;
            let in_table = c.g[V0][0] != 0;
            c.g[V0][0] = 0x0045_0000;
            if in_table {
                post_main_state_table(c, frame)
            } else {
                c.cycles += 1;
                frame.restore_ra(c, 0);
                state_case_exit(c, frame);
                Ok(())
            }
        }
        0x0011_14bc => post_main_state_table(c, frame),
        0x0011_14d4 => state_case_call(c, frame, POST_CASES[0]),
        0x0011_14e4 => state_case_call(c, frame, POST_CASES[1]),
        0x0011_14f4 => state_case_call(c, frame, POST_CASES[2]),
        0x0011_1504 => state_case_call(c, frame, POST_CASES[3]),
        0x0011_1514 => state_case_call(c, frame, POST_CASES[4]),
        0x0011_1524 => state_case_call(c, frame, POST_CASES[5]),
        0x0011_152c => {
            c.cycles += 1;
            frame.restore_ra(c, 0);
            state_case_exit(c, frame);
            Ok(())
        }
        0x0011_1530 => {
            state_case_exit(c, frame);
            Ok(())
        }
        other => {
            c.jump_out(other, 0x0011_14a0, 0x0011_1538);
            Ok(())
        }
    }
}

/// The jump-table read at 0x1114bc (`0x450000 + 0x6b70 + state * 4`).
pub(crate) fn post_main_state_table(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[V1][0] = shl32_2(c.g[V1][0], 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6b70));
    c.g[V1][0] = add32_2(c.g[V1][0], c.g[V0][0]);
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    match r32(c, A0) {
        0x0011_14d4 => state_case_call(c, frame, POST_CASES[0]),
        0x0011_14e4 => state_case_call(c, frame, POST_CASES[1]),
        0x0011_14f4 => state_case_call(c, frame, POST_CASES[2]),
        0x0011_1504 => state_case_call(c, frame, POST_CASES[3]),
        0x0011_1514 => state_case_call(c, frame, POST_CASES[4]),
        0x0011_1524 => state_case_call(c, frame, POST_CASES[5]),
        t => {
            c.jump_out(t, 0x0011_14a0, 0x0011_1538);
            Ok(())
        }
    }
}

/// `RiderBase_HandleSubStateInput` (0x111728): hand the input to the
/// sub-state handler of `rider->mSubState` (0xde4).
pub fn rider_base_handle_sub_state_input(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x10 };
        sub_state_input_at(c, frame, c.pc)
    })
}

pub(crate) fn sub_state_input_at(c: &mut Cpu, frame: Frame, pc: u32) -> Flow {
    match pc {
        0x0011_1728 => {
            c.cycles += 7;
            Frame::open(c, 0x10);
            c.g[A2][0] = c.g[A0][0];
            frame.save_ra(c, 0);
            c.g[V1][0] = Addr(p(r32(c, A2), 0xde4)).lw(c);
            c.g[V0][0] = (c.g[V1][0] < 14) as u64;
            let in_table = c.g[V0][0] != 0;
            c.g[V0][0] = 0x0045_0000;
            if in_table {
                sub_state_input_table(c, frame)
            } else {
                c.cycles += 1;
                frame.restore_ra(c, 0);
                state_case_exit(c, frame);
                Ok(())
            }
        }
        0x0011_1744 => sub_state_input_table(c, frame),
        0x0011_175c => sub_case_call(c, frame, 0),
        0x0011_176c => sub_case_call(c, frame, 1),
        0x0011_177c => sub_case_call(c, frame, 2),
        0x0011_178c => sub_case_call(c, frame, 3),
        0x0011_179c => sub_case_call(c, frame, 4),
        0x0011_17ac => sub_case_call(c, frame, 5),
        0x0011_17bc => sub_case_call(c, frame, 6),
        0x0011_17cc => sub_case_call(c, frame, 7),
        0x0011_17dc => sub_case_call(c, frame, 8),
        0x0011_17ec => sub_case_call(c, frame, 9),
        0x0011_17fc => sub_case_call(c, frame, 10),
        0x0011_180c => sub_case_call(c, frame, 11),
        0x0011_181c => sub_case_call(c, frame, 12),
        0x0011_1824 => {
            c.cycles += 1;
            frame.restore_ra(c, 0);
            state_case_exit(c, frame);
            Ok(())
        }
        0x0011_1828 => {
            state_case_exit(c, frame);
            Ok(())
        }
        other => {
            c.jump_out(other, 0x0011_1728, 0x0011_1830);
            Ok(())
        }
    }
}

/// The sub-state input call; the sub-object is at `rider + off` and `a1`
/// (the pad state) is passed through.
pub(crate) fn sub_case_call(c: &mut Cpu, frame: Frame, i: usize) -> Flow {
    let case = SUB_CASES[i];
    c.cycles += case.cycles;
    c.g[A0][0] = sx(r32(c, A2).wrapping_add(case.off));
    call_ptr(c, case.handler, case.ret)?;
    if case.tail {
        c.cycles += 1;
    }
    frame.restore_ra(c, 0);
    state_case_exit(c, frame);
    Ok(())
}

/// The jump-table read at 0x111744 (`0x450000 + 0x6c10 + state * 4`).
pub(crate) fn sub_state_input_table(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[V1][0] = shl32_2(c.g[V1][0], 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6c10));
    c.g[V1][0] = add32_2(c.g[V1][0], c.g[V0][0]);
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    match r32(c, A0) {
        0x0011_175c => sub_case_call(c, frame, 0),
        0x0011_176c => sub_case_call(c, frame, 1),
        0x0011_177c => sub_case_call(c, frame, 2),
        0x0011_178c => sub_case_call(c, frame, 3),
        0x0011_179c => sub_case_call(c, frame, 4),
        0x0011_17ac => sub_case_call(c, frame, 5),
        0x0011_17bc => sub_case_call(c, frame, 6),
        0x0011_17cc => sub_case_call(c, frame, 7),
        0x0011_17dc => sub_case_call(c, frame, 8),
        0x0011_17ec => sub_case_call(c, frame, 9),
        0x0011_17fc => sub_case_call(c, frame, 10),
        0x0011_180c => sub_case_call(c, frame, 11),
        0x0011_181c => sub_case_call(c, frame, 12),
        // The table's 14th slot (sub states 13..) points at the function's
        // `+1`-cycle tail block, which the original's `switch` accepts too.
        0x0011_1824 => {
            c.cycles += 1;
            frame.restore_ra(c, 0);
            state_case_exit(c, frame);
            Ok(())
        }
        t => {
            c.jump_out(t, 0x0011_1728, 0x0011_1830);
            Ok(())
        }
    }
}

/// `ComputerActor`'s six `{ character, model }` rider links.
pub(crate) const ACTOR_RIDERS: u32 = 0x18;

pub(crate) const RIDER_LINKS: u32 = 6;

pub(crate) const RIDER_LINK_SIZE: u32 = 0x24;

// ---------------------------------------------------------------------------
// Computer riders.

/// `int ComputerActorRider_FindHumanRiderIndex(ComputerActor *self)`
/// (0x10d8f8): the first of the six rider links whose character and model
/// are both set, or -1.
pub fn computer_actor_rider_find_human_rider_index(c: &mut Cpu) {
    c.cycles += 2;
    let actor = Addr(abi::arg(c, 0));
    c.g[V1][0] = actor.off(ACTOR_RIDERS).lw(c);
    c.g[A0][0] = 0;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        if c.g[V0][0] == 0 {
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x4).lw(c);
            if c.g[V0][0] == 0 {
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[A0][0];
                abi::ret(c);
                return;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A0][0] as i64) < RIDER_LINKS as i64) as u64;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(RIDER_LINK_SIZE));
        if c.g[V0][0] != 0 {
            if event_due(c) {
                deliver_event_here(c, 0x0010_d900);
            }
            continue;
        }
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        abi::ret(c);
        return;
    }
}

// ---------------------------------------------------------------------------
// Boost.

/// `void Rider_AddBoost(Rider* self, float amt, uint mask)` (0x10e098): add to
/// the boost meter (clamped to 0..1), tick the boost HUD timer and, when the
/// meter fills, start the 20-second boost countdown.
pub fn rider_add_boost(c: &mut Cpu) {
    c.cycles += 12;
    let frame = Frame::open(c, 0x40);
    c.g[V0][0] = 3;
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_fpr(c, 20, 0x30);
    c.g[S1][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    frame.save_fpr(c, 21, 0x38);
    let this = r32(c, S0);
    c.g[V1][0] = c.lw(p(this, rider::BOOST_STATE));
    let no_hud = c.g[V1][0] == c.g[V0][0];
    c.mov_s(20, 12);
    if !no_hud {
        c.cycles += 5;
        c.mtc1(0, 21);
        c.c_eq_s(20, 21);
        if c.fcc() {
            return rider_add_boost_leave_early(c, &frame);
        }
        c.cycles += 3;
        c.c_lt_s(21, 20);
        if c.fcc() {
            // Adding boost: report the character stat and require the mask.
            c.cycles += 13;
            if call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE_2, 0x0010_e0f0).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 3;
            if call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0010_e0fc).is_err() {
                return;
            }
            c.g[A1][0] = c.lw(p(this, rider::CHAR_ID));
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.lw(p(this, rider::CHAR_COMP));
            if call_ptr(c, callee::BE_CHAR_ATTRIB_FACTOR, 0x0010_e10c).is_err() {
                return;
            }
            c.g[V0][0] = c.lw(p(this, rider::BOOST_MASK));
            c.g[V0][0] = c.g[S1][0] & c.g[V0][0];
            if c.g[V0][0] == 0 {
                return rider_add_boost_leave_early(c, &frame);
            }
        }
    } else {
        // `state == 3` returns without touching the meter (`break 'b_10e208`).
        return rider_add_boost_leave_late(c, &frame);
    }
    // Common tail of the meter update.
    c.cycles += 15;
    if call_ptr(c, callee::AUDIO_GET_INSTANCE, 0x0010_e124).is_err() {
        return;
    }
    c.lwc1(12, p(this, rider::BOOST_2));
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[RA][0] = sx(0x0010_e138);
    c.mov_s(13, 20);
    if call_ptr(c, callee::AUDIO_RESET_BOOST_PITCH, 0x0010_e138).is_err() {
        return;
    }
    c.lwc1(0, p(this, rider::BOOST_2));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.add_s(0, 0, 20);
    c.c_le_s(21, 0);
    let below = !c.fcc();
    c.mov_s(2, 21);
    if !below {
        c.cycles += 1;
        c.min_s(2, 0, 1);
    }
    // Clamped meter value in f2.
    'meter: {
        c.cycles += 4;
        c.c_eq_s(2, 1);
        let full = !c.fcc();
        c.swc1(2, p(this, rider::BOOST_2));
        if full {
            break 'meter;
        }
        c.cycles += 3;
        c.c_lt_s(21, 20);
        if !c.fcc() {
            break 'meter;
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(p(this, rider::BOOST_FLAG));
        if c.g[V0][0] == 0 {
            break 'meter;
        }
        // Start the timed boost event once, then run the countdown.
        'timer: {
            c.cycles += 5;
            c.lwc1(0, p(this, rider::BOOST_TIMER));
            c.c_eq_s(0, 21);
            let running = !c.fcc();
            c.mov_s(12, 21);
            if running {
                break 'timer;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 5;
            if callee::SET_TIMED_EVENT.call(c, 0x0010_e1a4).is_err() {
                return;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(p(this, rider::BOOST_COUNT));
        if (c.g[V0][0] as i64) > 0 {
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.sw(p(this, rider::BOOST_COUNT), c.g[V0][0]);
            c.g[V0][0] = c.lw(p(this, rider::BOOST_COUNT));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        }
        c.cycles += 3;
        c.g[V0][0] = (c.g[V0][0] < 9) as u64;
        if c.g[V0][0] == 0 {
            return rider_add_boost_leave_early(c, &frame);
        }
        c.cycles += 5;
        c.lwc1(0, p(this, rider::BOOST_TIMER));
        c.g[AT][0] = 0x41a0_0000;
        c.mtc1(1, 1);
        c.max_s(0, 0, 1);
        c.cycles += 1;
        c.swc1(0, p(this, rider::BOOST_TIMER));
        return rider_add_boost_leave_late(c, &frame);
    }
    // Not full: the meter drains to zero while boost is being removed.
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_lt_s(20, 0);
    if !c.fcc() {
        return rider_add_boost_leave_early(c, &frame);
    }
    c.cycles += 4;
    c.g[V0][0] = c.lw(p(this, rider::BOOST_COUNT));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 10) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.swc1(0, p(this, rider::BOOST_TIMER));
        return rider_add_boost_leave_late(c, &frame);
    }
    rider_add_boost_leave_early(c, &frame);
}

/// `lq s0; jr ra` epilogue of [`rider_add_boost`] (the path that restores s0
/// itself).
#[inline(always)]
pub(crate) fn rider_add_boost_leave_early(c: &mut Cpu, frame: &Frame) {
    let a = frame.at(c, 0x20);
    c.lq(16, a);
    rider_add_boost_finish(c, frame);
}

/// The `break 'b_10e208` epilogue of [`rider_add_boost`]: one extra cycle
/// charged for the restore of s0.
#[inline(always)]
pub(crate) fn rider_add_boost_leave_late(c: &mut Cpu, frame: &Frame) {
    c.cycles += 1;
    let a = frame.at(c, 0x20);
    c.lq(16, a);
    rider_add_boost_finish(c, frame);
}

#[inline(always)]
pub(crate) fn rider_add_boost_finish(c: &mut Cpu, frame: &Frame) {
    c.cycles += 6;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 21, 0x38);
    frame.restore_fpr(c, 20, 0x30);
    frame.close(c);
}

/// `__builtin_delete` (operator delete, 0x317e50): the destructors call it
/// through the function table.
pub(crate) const DELETE: u32 = 0x0031_7e50;

// ---------------------------------------------------------------------------
// Destructors sharing the `if (flags & 1) delete this` shape.

/// `if (flags & 1) operator delete(this)` — the four destructors below.
pub(crate) fn delete_if_flag(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 4;
    let frame = Frame::open(c, 0x10);
    c.g[A1][0] = c.g[A1][0] & 0x1;
    let free = c.g[A1][0] != 0;
    frame.save_ra(c, 0);
    if free {
        c.cycles += 2;
        call_ptr(c, DELETE, ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void Rider_dtorFree(Rider* r, int flags)` (0x113170).
pub fn rider_dtor_free(c: &mut Cpu) {
    guest(c, |c| delete_if_flag(c, 0x0011_3188))
}

// ---------------------------------------------------------------------------
// Rider, AI, flags, BE singletons.

/// `void ComputerActorRider_ResetTrickCounts(Rider* self)` (0x10d170): zero
/// the fifteen trick-count triplets at `+0xef4-0x78`, `+0xef4-0x3c` and
/// `+0xef4`.
pub fn computer_actor_rider_reset_trick_counts(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0xef4));
    c.g[V0][0] = 0xe;
    loop {
        c.cycles += 7;
        let a = c.g[A0][0] as u32;
        Addr(a.wrapping_sub(0x78)).sw(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        Addr(a.wrapping_sub(0x3c)).sw(c, 0);
        Addr(a).sw(c, 0);
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[A0][0] = sx(a.wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0010_d178);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Riders.

/// `void Rider_AwardBonusBoost(Rider* r)` (0x10e8b8): award the bonus points
/// for the metrix at `+0x790`, add 0x10 boost, and play the bonus sound on
/// the audio system (`3, r, 1.0f`).
pub fn rider_award_bonus_boost(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[A0][0] = Addr(p_2(c, S0, 0x790)).lw(c);
    if callee::RIDER_METRIX_AWARD_BONUS_POINTS
        .call(c, 0x0010_e8d0)
        .is_err()
    {
        return;
    }
    c.mov_s(12, 0);
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0x10;
    if callee::RIDER_ADD_BOOST.call(c, 0x0010_e8e0).is_err() {
        return;
    }
    if call_ptr(c, table::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0010_e8e8).is_err() {
        return;
    }
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 12);
    c.g[A2][0] = c.g[S0][0];
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 3;
    if call_ptr(c, 0x0029_ced8, 0x0010_e900).is_err() {
        return;
    }
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Riders, camera and front end (s00-s02).

/// `void Rider_OnMissionCompleted(Rider* self, void* a1, int a2)` (0x10f2d8):
/// forward the mission result to the metric, then to the reward interface.
pub fn rider_on_mission_completed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[A2][0];
        c.g[RA][0] = sx(0x0010_f2fc);
        c.g[A0][0] = Addr(r32(c, S0)).off(0x790).lw(c);
        callee::RIDER_METRIX_ON_MISSION_COMPLETED.call(c, 0x0010_f2fc)?;
        c.g[RA][0] = sx(0x0010_f304);
        call_ptr(c, callee::BELIBRARY_GET_INSTANCE, 0x0010_f304)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0010_f310);
        c.g[A1][0] = 0xd;
        call_ptr(c, callee::BELIBRARY_GET_INTERFACE, 0x0010_f310)?;
        c.g[A1][0] = Addr(r32(c, S0)).off(0x86c).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0010_f320);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::BEREWARD_ON_MISSION_COMPLETED, 0x0010_f320)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Rider_OnItemCollected(Rider* self, void* a1)` (0x10f338): award the
/// collectible points and notify the reward interface.
pub fn rider_on_item_collected(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = 3;
        c.g[RA][0] = sx(0x0010_f35c);
        c.g[A0][0] = Addr(r32(c, S0)).off(0x790).lw(c);
        callee::RIDER_METRIX_AWARD_POINTS_2.call(c, 0x0010_f35c)?;
        c.g[RA][0] = sx(0x0010_f364);
        call_ptr(c, callee::BELIBRARY_GET_INSTANCE, 0x0010_f364)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0010_f370);
        c.g[A1][0] = 0xd;
        call_ptr(c, callee::BELIBRARY_GET_INTERFACE, 0x0010_f370)?;
        c.g[A1][0] = Addr(r32(c, S0)).off(0x86c).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0010_f380);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::BEREWARD_ON_COLLECTIBLE_COLLECTED, 0x0010_f380)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Rider_v13(Rider* rider, float dt)` (0x10e770): advance one of the
/// rider's animation timers (offset 0x2e8) and repaint it.
pub fn rider_v13(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        c.lwc1(0, at(c, S0, 0x2e8));
        c.add_s(0, 0, 12);
        Addr(at(c, S0, 0x2e8)).sw(c, c.f[0] as u64);
        call_ptr(c, 0x0028_b180, 0x0010_e790)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 12);
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0029_ced8, 0x0010_e7a8)?;
        call_ptr(c, 0x0028_b180, 0x0010_e7b0)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = 0x1;
        call_ptr(c, 0x002a_3b18, 0x0010_e7c0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Rider_v14(Rider* rider, float dt)` (0x10e7d0): the same for the
/// rider's second animation timer (offset 0x2ec).
pub fn rider_v14(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        c.lwc1(0, at(c, S0, 0x2ec));
        c.add_s(0, 0, 12);
        Addr(at(c, S0, 0x2ec)).sw(c, c.f[0] as u64);
        call_ptr(c, 0x0028_b180, 0x0010_e7f0)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 12);
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0029_ced8, 0x0010_e808)?;
        call_ptr(c, 0x0028_b180, 0x0010_e810)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = 0x2;
        call_ptr(c, 0x002a_3b18, 0x0010_e820)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Trivial leaves and syscall stubs.

/// `float func_00113128(Rider *r)` (0x113128): the rider's distance to the
/// finish line, a plain field read at `+0x4d0`. (The reference has assembly
/// only; the generated body reads f0 and returns.)
pub fn rider_get_dist_to_finish(c: &mut Cpu) {
    c.cycles += 2;
    c.lwc1(0, r32(c, A0).wrapping_add(0x4d0));
    abi::ret(c);
}

/// Rider state enter/exit methods (cross-shard `call_indirect`).
pub(crate) const RIDER_GROUND_ENTER: u32 = 0x0013_c7a8;

pub(crate) const RIDER_AIR_ENTER: u32 = 0x0013_99e0;

pub(crate) const RIDER_WIPEOUT_ENTER: u32 = 0x0013_6c40;

pub(crate) const RIDER_RAILSLIDE_ENTER: u32 = 0x0013_ad20;

pub(crate) const RIDER_HANDPLANT_ENTER: u32 = 0x0013_8b48;

pub(crate) const RIDER_GROUND_EXIT: u32 = 0x0013_f410;

pub(crate) const RIDER_AIR_EXIT: u32 = 0x0013_9a18;

pub(crate) const RIDER_WIPEOUT_EXIT: u32 = 0x0013_6f28;

pub(crate) const RIDER_RAILSLIDE_EXIT: u32 = 0x0013_c5a0;

pub(crate) const RIDER_HANDPLANT_EXIT: u32 = 0x0013_9178;

// ---------------------------------------------------------------------------
// Helpers.

/// `jalr` through the function table (the generated `call_indirect` form).
#[inline]
pub(crate) fn call_table(c: &mut Cpu, target: u32, ret: u32) -> crate::call::Flow {
    call_ptr(c, target, ret)
}

/// `int Rider_ReturnZero_25b8(void* self)` (0x1125b8): a virtual method that
/// always returns 0.
pub fn rider_return_zero_25b8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// RiderBase state transitions: five virtual enter/exit methods selected by the
// rider's current main-state id (`rider + 0xde0`, 0..5) through a `.rodata`
// table. The ports accept every `__entries` resume label.

/// `void RiderBase_EnterMainState(RiderBase* r)` (0x111380): enter the method
/// of the rider's current state (ground, air, wipeout, railslide, handplant or
/// the base state).
pub fn rider_base_enter_main_state(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x0011_1380 => {
                    c.cycles += 7;
                    let frame = Frame::open(c, 0x10);
                    c.g[A2][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xde0).lw(c);
                    c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                    let out_of_range = c.g[V0][0] == 0;
                    // The generated code reloads the table base into v0
                    // before branching; 0x11139c adds 0x6b30 to it.
                    c.g[V0][0] = 0x45_0000;
                    pc = if out_of_range {
                        0x0011_13fc
                    } else {
                        0x0011_139c
                    };
                }
                0x0011_139c => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6b30));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    pc = match target {
                        0x0011_13b4 => 0x0011_13b4,
                        0x0011_13c4 => 0x0011_13c4,
                        0x0011_13d4 => 0x0011_13d4,
                        0x0011_13e4 => 0x0011_13e4,
                        0x0011_13f4 => 0x0011_13f4,
                        0x0011_13fc => 0x0011_13fc,
                        _ => {
                            c.jump_out(target, 0x0011_1380, 0x0011_1408);
                            return Ok(());
                        }
                    };
                }
                0x0011_13b4 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[A2][0];
                    call_table(c, RIDER_GROUND_ENTER, 0x0011_13bc)?;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1400;
                }
                0x0011_13c4 => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    call_table(c, RIDER_AIR_ENTER, 0x0011_13cc)?;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1400;
                }
                0x0011_13d4 => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x30));
                    call_table(c, RIDER_WIPEOUT_ENTER, 0x0011_13dc)?;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1400;
                }
                0x0011_13e4 => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xb0));
                    call_table(c, RIDER_RAILSLIDE_ENTER, 0x0011_13ec)?;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1400;
                }
                0x0011_13f4 => {
                    c.cycles += 2;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x110));
                    call_table(c, RIDER_HANDPLANT_ENTER, 0x0011_13fc)?;
                    pc = 0x0011_13fc;
                }
                0x0011_13fc => {
                    c.cycles += 1;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1400;
                }
                0x0011_1400 => {
                    c.cycles += 2;
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0011_1380, 0x0011_1408);
                    return Ok(());
                }
            }
        }
    })
}

/// `void RiderBase_ExitMainState(RiderBase* r)` (0x1112f8): leave the method
/// of the rider's current state (same table as the enter side).
pub fn rider_base_exit_main_state(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x0011_12f8 => {
                    c.cycles += 7;
                    let frame = Frame::open(c, 0x10);
                    c.g[A2][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xde0).lw(c);
                    c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                    let out_of_range = c.g[V0][0] == 0;
                    // The generated code reloads the table base into v0
                    // before branching; 0x111314 adds 0x6b10 to it.
                    c.g[V0][0] = 0x45_0000;
                    pc = if out_of_range {
                        0x0011_1374
                    } else {
                        0x0011_1314
                    };
                }
                0x0011_1314 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6b10));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    pc = match target {
                        0x0011_132c => 0x0011_132c,
                        0x0011_133c => 0x0011_133c,
                        0x0011_134c => 0x0011_134c,
                        0x0011_135c => 0x0011_135c,
                        0x0011_136c => 0x0011_136c,
                        0x0011_1374 => 0x0011_1374,
                        _ => {
                            c.jump_out(target, 0x0011_12f8, 0x0011_1380);
                            return Ok(());
                        }
                    };
                }
                0x0011_132c => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[A2][0];
                    call_table(c, RIDER_GROUND_EXIT, 0x0011_1334)?;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1378;
                }
                0x0011_133c => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    call_table(c, RIDER_AIR_EXIT, 0x0011_1344)?;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1378;
                }
                0x0011_134c => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x30));
                    call_table(c, RIDER_WIPEOUT_EXIT, 0x0011_1354)?;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1378;
                }
                0x0011_135c => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xb0));
                    call_table(c, RIDER_RAILSLIDE_EXIT, 0x0011_1364)?;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1378;
                }
                0x0011_136c => {
                    c.cycles += 2;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x110));
                    call_table(c, RIDER_HANDPLANT_EXIT, 0x0011_1374)?;
                    pc = 0x0011_1374;
                }
                0x0011_1374 => {
                    c.cycles += 1;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1378;
                }
                0x0011_1378 => {
                    c.cycles += 2;
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0011_12f8, 0x0011_1380);
                    return Ok(());
                }
            }
        }
    })
}

/// The dot product the original computes with the VU0 add/madd chain
/// (vf3 = a, vf5 = b), left in vf4.x.
#[inline(always)]
pub(crate) fn v_dot(c: &mut Cpu, a: u32, b: u32) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    let vu = c.vu0();
    vu.add(0b1000, 6, 0, Src::Bc(0, W));
    vu.mul(0b1111, 4, 3, Src::Vec(5));
    vu.adda(0b1000, 4, Src::Bc(4, Y));
    vu.madda(0b1000, 6, Src::Bc(4, Z));
    vu.madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// [`v_dot`] of a vector with itself.
#[inline(always)]
pub(crate) fn v_dot_self(c: &mut Cpu, a: u32) {
    c.lqc2(3, a);
    let vu = c.vu0();
    vu.add(0b1000, 6, 0, Src::Bc(0, W));
    vu.mul(0b1111, 4, 3, Src::Vec(3));
    vu.adda(0b1000, 4, Src::Bc(4, Y));
    vu.madda(0b1000, 6, Src::Bc(4, Z));
    vu.madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `out = a / |a|` (the `vrsqrt` + `vmul Q` sequence).
#[inline(always)]
pub(crate) fn v_normalize(c: &mut Cpu, a: u32, out: u32) {
    v_dot_self(c, a);
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    c.sqc2(5, out);
}

/// `float ComputerActorRider_ComputeSteer(Rider* rider /*a0*/,
/// const Vec4* velocity /*a1*/)` (0x00100348): the steering angle between the
/// part of `velocity` perpendicular to the forward axis and a reference axis
/// of the motion controller, normalised and clamped to `0.98` (scaled by 2π)
/// with its sign kept.
///
/// The two indirect calls are virtual methods of the object at_2
/// `rider->model+0x6c0` (slots `+0x10` and `+0x28`), returning the forward
/// vector and the motion basis vector.
pub fn computer_actor_rider_compute_steer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 22, 0xc0);
        frame.save_ra(c, 0x80);
        frame.save_fpr(c, 21, 0xb8);
        frame.save_fpr(c, 20, 0xb0);
        // f22 = 0.
        c.mtc1(0, 22);
        c.g[V0][0] = lw_2(c, p(r32_2(c, S0), rider::MODEL));
        let model = r32_2(c, V0);
        c.lq(3, p(model, 0x1c0));
        c.sq(3, sp(c));
        c.lwc1(0, p(sp(c), 8));
        c.c_le_s(0, 22);
        let backward = !c.fcc();
        c.g[S1][0] = c.g[A1][0];
        if !backward {
            // Not driving backwards: use the motion's forward axis instead.
            c.cycles += 4;
            c.g[V0][0] = 0x0050_0000;
            c.g[V0][0] = sx(r32_2(c, V0).wrapping_sub(0xea0));
            c.lq(3, r32_2(c, V0));
            c.sq(3, sp(c));
        }

        'steer: {
            'math: {
                c.cycles += 100;
                // Forward vector: virtual slot +0x10 of the motion object.
                c.g[A0][0] = lw_2(c, p(r32_2(c, S0), rider::MODEL));
                c.g[V1][0] = lw_2(c, p(r32_2(c, A0), rider::MOTION));
                c.g[A0][0] = sx(p(r32_2(c, A0), rider::MOTION));
                c.g[A1][0] = lh(c, p(r32_2(c, V1), 0x10));
                c.g[V0][0] = lw_2(c, p(r32_2(c, V1), 0x14));
                let target = r32_2(c, V0);
                c.g[A0][0] = sx(p(r32_2(c, A0), r32_2(c, A1)));
                call_ptr(c, target, 0x0010_03b8)?;
                c.lq(3, r32_2(c, V0));
                c.sq(3, at_2(c, 0x10));
                // Side axis = forward × up (vf4 = forward, vf5 = up).
                c.lqc2(4, at_2(c, 0x10));
                c.lqc2(5, at_2(c, 0));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                c.sqc2(6, at_2(c, 0x30));
                c.lq(2, at_2(c, 0x30));
                c.sq(2, at_2(c, 0x20));
                // Motion basis vector: virtual slot +0x28.
                c.g[A0][0] = lw_2(c, p(r32_2(c, S0), rider::MODEL));
                c.g[A1][0] = sx(p(r32_2(c, A0), rider::MOTION));
                c.g[V1][0] = lw_2(c, p(r32_2(c, A0), rider::MOTION));
                c.g[A0][0] = lh(c, p(r32_2(c, V1), 0x28));
                c.g[V0][0] = lw_2(c, p(r32_2(c, V1), 0x2c));
                let target = r32_2(c, V0);
                c.g[A0][0] = sx(p(r32_2(c, A1), r32_2(c, A0)));
                call_ptr(c, target, 0x0010_03fc)?;
                // The velocity's perpendicular component.
                c.lqc2(3, r32_2(c, S1));
                c.lqc2(4, r32_2(c, V0));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at_2(c, 0x40));
                c.lq(2, at_2(c, 0x40));
                c.sq(2, at_2(c, 0x30));
                c.swc1(22, at_2(c, 0x38));
                v_normalize(c, at_2(c, 0x30), at_2(c, 0x50));
                c.lq(2, at_2(c, 0x50));
                c.sq(2, at_2(c, 0x30));
                c.sq(2, at_2(c, 0x40));
                // Remove the forward component.
                v_dot(c, at_2(c, 0x30), at_2(c, 0));
                c.qmfc2(3, 4);
                c.mtc1(3, 0);
                c.mfc1(4, 0);
                c.lqc2(4, at_2(c, 0));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at_2(c, 0x70));
                c.lq(2, at_2(c, 0x70));
                c.sq(2, at_2(c, 0x60));
                c.lqc2(3, at_2(c, 0x30));
                c.lqc2(4, at_2(c, 0x60));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at_2(c, 0x70));
                c.lq(2, at_2(c, 0x70));
                c.sq(2, at_2(c, 0x50));
                v_normalize(c, at_2(c, 0x50), at_2(c, 0x70));
                c.lq(2, at_2(c, 0x70));
                c.sq(2, at_2(c, 0x40));
                c.sq(2, at_2(c, 0x30));
                // f20 = side · forward, f21 = side · basis.
                v_dot(c, at_2(c, 0x30), at_2(c, 0x10));
                c.qmfc2(3, 4);
                c.mtc1(3, 20);
                v_dot(c, at_2(c, 0x30), at_2(c, 0x20));
                c.qmfc2(2, 4);
                c.mtc1(2, 21);
                c.c_eq_s(20, 22);
                if !c.fcc() {
                    // f20 != 0: fall through to the atan path.
                } else {
                    // Degenerate: 0 or ±π/2.
                    c.cycles += 4;
                    c.c_eq_s(21, 22);
                    let axis_aligned = c.fcc();
                    c.mov_s(12, 21);
                    if axis_aligned {
                        break 'steer;
                    }
                    c.cycles += 4;
                    c.c_le_s(22, 21);
                    ldf_3(c, 12, -0x7fec);
                    if !c.fcc() {
                        break 'steer;
                    }
                    c.cycles += 3;
                    ldf_3(c, 12, -0x7fe8);
                    break 'steer;
                }
            }
            c.cycles += 7;
            c.div_s(12, 21, 20);
            call_ptr(c, 0x0031_c228, 0x0010_0564)?; // Math_Atan
            c.c_lt_s(20, 22);
            c.mov_s(12, 0);
            if !c.fcc() {
                break 'steer;
            }
            // f20 < 0: add π/2, or subtract π on the other side.
            'half_turn: {
                c.cycles += 3;
                c.c_lt_s(22, 21);
                if !c.fcc() {
                } else {
                    c.cycles += 3;
                    ldf_3(c, 0, -0x7fe4);
                    c.add_s(12, 12, 0);
                    break 'steer;
                }
            }
            c.cycles += 2;
            ldf_3(c, 0, -0x7fe0);
            c.sub_s(12, 12, 0);
            break 'steer;
        }

        // Scale to the 0..0.98 steering range and restore its sign.
        'clamp: {
            'scale: {
                c.cycles += 8;
                ldf_3(c, 0, -0x7fdc);
                c.abs_s(1, 12);
                ldf_3(c, 2, -0x7fd8);
                c.mul_s(1, 1, 0);
                c.c_lt_s(1, 2);
                c.lq(16, at_2(c, 0xa0));
                if c.fcc() {
                    // Below the dead zone: no clamp needed.
                    c.cycles += 3;
                    c.mtc1(0, 1);
                    c.mov_s(0, 1);
                    break 'clamp;
                }
            }
            'upper: {
                c.cycles += 5;
                ldf_3(c, 0, -0x7fd4);
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.mov_s(1, 0);
                }
            }
            'sign: {
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_lt_s(12, 0);
                if c.fcc() {
                    c.neg_s(1, 1);
                }
            }
            c.cycles += 2;
            c.lq(16, at_2(c, 0xa0));
            c.mov_s(0, 1);
        }

        c.cycles += 7;
        frame.restore(c, 17, 0x90);
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 22, 0xc0);
        frame.restore_fpr(c, 21, 0xb8);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider fields used here (see docs/investigation/structs/rider.md).

/// The rider's `RiderModel*` (animation time scale at `+0x300`).
pub(crate) const RIDER_MODEL: u32 = 0x18;

/// Non-zero while another system has taken the control input over.
pub(crate) const RIDER_CONTROL_OVERRIDE: u32 = 0xe34;

/// The last sub-state, 0xe is past the last handling step.
pub(crate) const SUB_STATE_LAST: u64 = 0xe;

/// The one sub-state that keeps the override word.
pub(crate) const SUB_STATE_SPIN: u64 = 5;

/// `void ComputerActorRider_GetControlInput(Rider* rider, ControlInput* in)`
/// (0x0010a768): fills `in` for the rider's current sub-state.
pub fn computer_actor_rider_get_control_input(c: &mut Cpu) {
    guest(c, |c| {
        // The original's switch dispatcher reloads the sub-state from the
        // function entry for the (unused) state-0 table slot; keep a loop so
        // that edge exists with its event check.
        'restart: loop {
            c.cycles += 22;
            let frame = Frame::open(c, 0x40);
            c.g[A2][0] = 8;
            c.sq(S0, frame.at(c, 0x30));
            c.sq(S1, frame.at(c, 0x20));
            c.g[S0][0] = c.g[A1][0];
            c.g[S1][0] = c.g[A0][0];
            c.sq(S2, frame.at(c, 0x10));
            frame.save_ra(c, 0);
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S0][0];
            call::call_ptr(c, callee::EA_MEM_SET, 0x0010_a794)?;

            c.g[A0][0] = c.g[S1][0];
            callee::RUBBER_BAND.call(c, 0x0010_a79c)?;

            c.g[A0][0] = Addr(c.g[S1][0] as u32).off(RIDER_MODEL).lw(c);
            c.mov_s(12, 0);
            callee::SET_ANIM_TIME_SCALE.call(c, 0x0010_a7a8)?;

            c.g[A0][0] = Addr(c.g[S1][0] as u32).off(RIDER_MODEL).lw(c);
            callee::GET_SUB_STATE_2.call(c, 0x0010_a7b0)?;
            c.g[S2][0] = c.g[V0][0];

            c.g[V0][0] = (c.g[S2][0] < SUB_STATE_LAST) as u64;
            let dispatch = c.g[V0][0] != 0;
            c.g[V0][0] = 0x450000;
            if dispatch {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6a30));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                // The step for each sub-state (the table's target blocks).
                match c.g[A0][0] as u32 {
                    0x0010_a768 => {
                        // Table slot for sub-state 0: re-read the state.
                        if event_due(c) {
                            deliver_event_here(c, 0x0010_a768);
                        }
                        continue 'restart;
                    }
                    0x0010_a7d8 => {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        callee::THINK_CRUISE.call(c, 0x0010_a7e4)?;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    0x0010_a7ec => {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        callee::THINK_PREWIND.call(c, 0x0010_a7f8)?;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    0x0010_a800 => {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        callee::THINK_GATE.call(c, 0x0010_a80c)?;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    0x0010_a814 => {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        callee::THINK_NATURAL_AIR.call(c, 0x0010_a820)?;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    0x0010_a828 => {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        callee::THINK_RAILSLIDE.call(c, 0x0010_a834)?;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    0x0010_a83c => {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        callee::THINK_SPIN.call(c, 0x0010_a848)?;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    0x0010_a850 => {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        callee::THINK_WIPEOUT.call(c, 0x0010_a85c)?;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    0x0010_a864 => {
                        c.cycles += 3;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        callee::THINK_BOARD_PRESS.call(c, 0x0010_a870)?;
                        c.cycles += 1;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    0x0010_a870 => {
                        // Sub-states without a step (3, 9..13).
                        c.cycles += 1;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    other => {
                        c.jump_out(other, 0x0010_a768, 0x0010_a894);
                        return Ok(());
                    }
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = SUB_STATE_SPIN;
            }

            // Only a spinning rider keeps the "control taken over" word.
            c.cycles += 2;
            if c.g[S2][0] != c.g[V0][0] {
                Addr((c.g[S1][0] as u32).wrapping_add(RIDER_CONTROL_OVERRIDE)).sw(c, 0);
            }

            c.cycles += 6;
            c.lq(16, frame.at(c, 0x30));
            c.lq(17, frame.at(c, 0x20));
            c.lq(18, frame.at(c, 0x10));
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    });
}

/// Rider fields the path follower reads and writes.
pub(crate) const R_MODEL: u32 = 0x18;

/// The steering word of the control block (bits 6..11 hold the steering
/// byte, written as `(int)(angle * 31) << 6`).
pub(crate) const R_STEER: u32 = 0x4;

/// The rival slot the AI manager is indexed with.
pub(crate) const R_RIVAL: u32 = 0x80;

/// Distance to the next speed marker (written by the marker search).
pub(crate) const R_MARKER_DIST: u32 = 0xdf0;

/// Distance to the speed marker the crouch check uses.
pub(crate) const R_MARKER_DIST2: u32 = 0xdf8;

/// Board-press countdown.
pub(crate) const R_BOARD_PRESS: u32 = 0xe40;

/// Jump request / jump marker.
pub(crate) const R_JUMP: u32 = 0xe70;

/// `short` jump state.
pub(crate) const R_JUMP_STATE: u32 = 0xf38;

/// The 8-byte jump descriptor copied in.
pub(crate) const R_JUMP_DESC: u32 = 0xf44;

/// The rival's path interface inside the rider entry (`PMF: +0x40 delta,
/// +0x44 function`).
pub(crate) const R_PATH_IFACE: u32 = 0x6c0;

/// Model distances compared with the crouch thresholds.
pub(crate) const M_NEAR: u32 = 0x2f8;

pub(crate) const M_FAR: u32 = 0x2f0;

/// The rider model's forward axis for the distance to a speed marker.
pub(crate) const M_MARKER: u32 = 0x1e0;

/// The steering target the AI wrote into the frame.
pub(crate) const STEER_OUT: u32 = 0x10;

// The jump descriptors the original copies live at `0x450000 - 0x30e0`
// (`0x43cf20`), at `+0x18` (selected) and at `+0x10` (default).

/// `void ComputerActorRider_CruiseFollowPath(ComputerActorRider* rider,
/// ControlInput* in)` (0x00100680): the cruise thinking step. Computes the
/// steering (blending the AI's steering byte into the rider's flags),
/// reacts to speed markers (boost when one is near, crouch otherwise),
/// runs the board-press countdown and then resolves the jump state through
/// the rival's path interface.
///
/// The original's blocks are laid out sequentially in address order and
/// entered by falling out of the previous one; each `'b_…` label below is
/// one of those blocks.
pub fn computer_actor_rider_cruise_follow_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x70);
        c.sq(S0, frame.at(c, 0x50));
        c.sq(S1, frame.at(c, 0x40));
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.swc1(20, frame.at(c, 0x60));
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, 0x30);
        c.g[AT][0] = 0x41f8_0000; // 31.0, the steering scale
        c.mtc1(1, 20);
        c.g[A1][0] = c.g[S0][0];
        cruise::GET_STEER_TARGET.call(c, 0x0010_06b0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        cruise::COMPUTE_STEER.call(c, 0x0010_06bc)?;

        // Pack the steering angle (6 bits) into the rider's flags.
        c.mul_s(0, 0, 20);
        c.g[A3][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V1][0] = 0xffff_ffff_fc0f_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(R_MARKER_DIST));
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[A3][0] = c.g[A3][0] & c.g[V1][0];
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(STEER_OUT));
        c.g[V0][0] = c.g[V0][0] & 0x3f;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 20);
        c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
        c.sw(c.g[S1][0] as u32, c.g[A3][0]);
        cruise::FIND_SPEED_MARKER.call(c, 0x0010_06f8)?;

        let no_marker = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        'b_1008e4: {
            'b_1008e0: {
                if !no_marker {
                    'b_100764: {
                        c.cycles += 18;
                        c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(R_MARKER_DIST));
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x7fcc));
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(R_MODEL).lw(c);
                        c.add_s(0, 2, 3);
                        c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(M_MARKER));
                        vu0_length_sq(c);
                        c.cfc2(3, 22);
                        c.mtc1(3, 1);
                        c.c_lt_s(0, 1);
                        let far = !c.fcc();
                        c.g[V0][0] = Addr(c.g[SP][0] as u32).off(STEER_OUT).lw(c);
                        if far {
                            break 'b_100764;
                        } else {
                            c.cycles += 2;
                            let no_target = c.g[V0][0] == 0;
                            c.g[V1][0] = 0xffff_ffff_ffff_ffc0;
                            if no_target {
                                break 'b_1008e0;
                            } else {
                                // Crouch: clear the boost bits of the flags.
                                c.cycles += 5;
                                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(R_STEER).lw(c);
                                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                c.g[V0][0] = c.g[V0][0] | 0x1f;
                                c.sw((c.g[S1][0] as u32).wrapping_add(R_STEER), c.g[V0][0]);
                                break 'b_1008e0;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.sub_s(0, 2, 3);
                    c.c_lt_s(1, 0);
                    if !c.fcc() {
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(R_MARKER_DIST));
                        break 'b_1008e4;
                    } else {
                        'b_100790: {
                            c.cycles += 2;
                            let no_target = c.g[V0][0] == 0;
                            c.g[A1][0] = c.g[S1][0];
                            if no_target {
                                break 'b_100790;
                            } else {
                                c.cycles += 4;
                                c.g[A0][0] = c.g[S0][0];
                                cruise::CROUCH_AND_BOOST.call(c, 0x0010_0788)?;
                                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(R_MARKER_DIST));
                                break 'b_1008e4;
                            }
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        cruise::SET_CRUISE_CROUCH.call(c, 0x0010_0798)?;
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(R_MARKER_DIST));
                        break 'b_1008e4;
                    }
                }
                {
                    // Board-press marker, written to the frame's +0x14.
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
                    cruise::FIND_BOARD_PRESS_MARKER.call(c, 0x0010_07a8)?;
                    let none = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr(c.g[SP][0] as u32).off(0x14).lw(c);
                    if !none {
                        'b_1007c8: {
                            c.cycles += 4;
                            c.g[AT][0] = 0x3f80_0000; // 1.0
                            c.mtc1(1, 0);
                            let first = c.g[V0][0] == 0;
                            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(R_BOARD_PRESS).lw(c);
                            if !first {
                                c.cycles += 2;
                                c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
                                c.mtc1(1, 0);
                            }
                        }
                        'b_100804: {
                            c.cycles += 2;
                            let idle = c.g[V0][0] != 0;
                            c.g[A0][0] = 0x3c;
                            if !idle {
                                // Start the press: 60 frames, opposite steer.
                                c.cycles += 13;
                                c.neg_s(0, 0);
                                c.g[V1][0] = Addr(c.g[S1][0] as u32).off(R_STEER).lw(c);
                                c.g[V0][0] = 0xffff_ffff_ffff_f03f;
                                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                c.mul_s(0, 0, 20);
                                c.cvt_w_s(1, 0);
                                c.mfc1(2, 1);
                                c.g[V0][0] = c.g[V0][0] & 0x3f;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                c.sw((c.g[S1][0] as u32).wrapping_add(R_STEER), c.g[V1][0]);
                                c.sw((c.g[S0][0] as u32).wrapping_add(R_BOARD_PRESS), c.g[A0][0]);
                                break 'b_1008e0;
                            }
                        }
                        'b_10084c: {
                            // Count the press down, steering opposite.
                            c.cycles += 2;
                            let expired = (c.g[V0][0] as i64) <= 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                            if !expired {
                                'b_10081c: {
                                    c.cycles += 2;
                                    let still = (c.g[V0][0] as i64) > 0;
                                    c.sw(
                                        (c.g[S0][0] as u32).wrapping_add(R_BOARD_PRESS),
                                        c.g[V0][0],
                                    );
                                    if !still {
                                        c.cycles += 2;
                                        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                                        c.sw(
                                            (c.g[S0][0] as u32).wrapping_add(R_BOARD_PRESS),
                                            c.g[V0][0],
                                        );
                                    }
                                }
                                c.cycles += 12;
                                c.neg_s(0, 0);
                                c.g[V1][0] = Addr(c.g[S1][0] as u32).off(R_STEER).lw(c);
                                c.g[V0][0] = 0xffff_ffff_ffff_f03f;
                                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                c.mul_s(0, 0, 20);
                                c.cvt_w_s(1, 0);
                                c.mfc1(2, 1);
                                c.g[V0][0] = c.g[V0][0] & 0x3f;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                c.sw((c.g[S1][0] as u32).wrapping_add(R_STEER), c.g[V1][0]);
                                break 'b_1008e0;
                            }
                        }
                        c.cycles += 11;
                        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(R_STEER).lw(c);
                        c.mul_s(0, 0, 20);
                        c.g[V1][0] = 0xffff_ffff_ffff_f03f;
                        c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
                        c.cvt_w_s(1, 0);
                        c.mfc1(2, 1);
                        c.g[V0][0] = c.g[V0][0] & 0x3f;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                        c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                        c.sw((c.g[S1][0] as u32).wrapping_add(R_STEER), c.g[A0][0]);
                        break 'b_1008e0;
                    }
                }
                {
                    // No board press: drop the counter and crouch when the
                    // marker distance is outside the model's two thresholds.
                    c.cycles += 8;
                    c.sw((c.g[S0][0] as u32).wrapping_add(R_BOARD_PRESS), 0);
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(R_MODEL).lw(c);
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7fc8));
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(M_NEAR));
                    c.c_le_s(1, 0);
                    let skip = !c.fcc();
                    c.g[A1][0] = c.g[S1][0];
                    if !skip {
                        c.cycles += 5;
                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(M_FAR));
                        c.mtc1(0, 0); // f0 = 0.0
                        c.c_lt_s(0, 1);
                        let skip = !c.fcc();
                        if !skip {
                            c.cycles += 7;
                            c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(R_MARKER_DIST2));
                            c.g[AT][0] = 0x4248_0000; // 50.0
                            c.mtc1(1, 0); // f0 = 50.0
                            c.c_lt_s(0, 1);
                            let near = !c.fcc();
                            c.g[V1][0] = 0x2_0000;
                            if !near {
                                c.cycles += 4;
                                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                                c.g[A1][0] = c.g[S1][0];
                            }
                        }
                    }
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                cruise::SET_CRUISE_CROUCH.call(c, 0x0010_08e0)?;
                break 'b_1008e0;
            }
            c.cycles += 1;
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(R_MARKER_DIST));
            break 'b_1008e4;
        }

        // ------------------------------------------------------------ jumps
        'b_1009cc: {
            'b_1009c8: {
                'b_100934: {
                    // Look for a jump marker (five out pointers).
                    c.cycles += 11;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x18));
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1c));
                    c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x24));
                    c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x28));
                    cruise::FIND_JUMP_MARKER.call(c, 0x0010_0908)?;
                    let none = c.g[V0][0] == 0;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    if !none {
                        c.cycles += 9;
                        c.g[V1][0] = 0x44_0000;
                        c.sw((c.g[S0][0] as u32).wrapping_add(R_JUMP), c.g[V0][0]);
                        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30e0));
                        let src = c.g[A2][0] as u32;
                        jump_desc_copy(c, 2, src, 7, 0);
                        c.sh((c.g[S0][0] as u32).wrapping_add(R_JUMP_STATE), 0);
                        break 'b_1009c8;
                    }
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2c));
                cruise::SELECT_TARGET_RIDER.call(c, 0x0010_0940)?;
                let none = c.g[V0][0] == 0;
                c.g[V0][0] = Addr(c.g[SP][0] as u32).off(0x2c).lw(c);
                if !none {
                    'b_1009c4: {
                        'b_10096c: {
                            c.cycles += 2;
                            let empty = c.g[V0][0] == 0;
                            c.g[V0][0] = 0x44_0000;
                            if !empty {
                                c.cycles += 7;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30e0));
                                let src = c.g[V0][0] as u32;
                                jump_desc_copy(c, 7, src, 0x1f, 0x18);
                                c.g[V1][0] = 3;
                                break 'b_1009c4;
                            }
                        }
                        // Ask the rival's path interface whether a jump is on.
                        c.cycles += 15;
                        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(R_RIVAL).lw(c);
                        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0xc).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x28).lw(c);
                        c.g[V1][0] = Addr(c.g[A0][0] as u32).off(R_PATH_IFACE).lw(c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(R_PATH_IFACE));
                        c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x40).lh(c);
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x44).lw(c);
                        let target = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        call::call_ptr(c, target, 0x0010_09a0)?;
                        if c.g[V0][0] != 0 {
                            c.lq(16, frame.at(c, 0x50));
                            break 'b_1009cc;
                        }
                        c.cycles += 7;
                        c.g[V0][0] = 0x44_0000;
                        c.g[V1][0] = 2;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30e0));
                        let src = c.g[V0][0] as u32;
                        jump_desc_copy(c, 7, src, 0x17, 0x10);
                        break 'b_1009c4;
                    }
                    c.cycles += 1;
                    c.sh((c.g[S0][0] as u32).wrapping_add(R_JUMP_STATE), c.g[V1][0]);
                    break 'b_1009c8;
                }
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x50));
            break 'b_1009cc;
        }

        c.cycles += 5;
        c.lq(17, frame.at(c, 0x40));
        frame.restore_ra(c, 0x30);
        c.lwc1(20, frame.at(c, 0x60));
        frame.close(c);
        Ok(())
    });
}

/// The VU0 macro sequence that writes the squared length of the vector
/// loaded into VF3 to the accumulator (`madd` into VF4) and its square root
/// into Q (`sqrt`); the original then reads Q with `cfc2 rt, vi22`.
#[inline]
pub(crate) fn vu0_length_sq(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
}

/// `ldl/ldr` an 8-byte descriptor at `src + lo` / `src + hi` into GPR `reg`,
/// then `sdl/sdr` it to the rider's jump descriptor (the `ldl`/`sdl` at +7,
/// the `ldr`/`sdr` at +0).
#[inline]
pub(crate) fn jump_desc_copy(c: &mut Cpu, reg: usize, src: u32, lo: u32, hi: u32) {
    c.ldl(reg, src.wrapping_add(lo));
    c.ldr(reg, src.wrapping_add(hi));
    let dst = (c.g[S0][0] as u32).wrapping_add(R_JUMP_DESC);
    c.sdl(reg, dst.wrapping_add(7));
    c.sdr(reg, dst);
}

/// `void Rider_SetTimedEvent(Rider* self, int mode, float seconds)`
/// (0x10e028): boost timer; `mode == 0` clears it to -1, otherwise the frame
/// counter plus `seconds * 60` truncated.
///
/// `+0x354`/`+0x358` are compared against `AI_GetFrameCounter` (one count per
/// tick, `docs/investigation/structs/world.md` 0x08), so `seconds * 60` is a
/// tick count: at another rate the deadline (`ticks_from_frames60`) is
/// further away. At 60 the truncated frame count is the tick count, unchanged.
pub fn rider_set_timed_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        let mode = c.g[S2][0] != 0;
        frame.save_ra(c, 0);
        if !mode {
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            Addr(at_3(c, S1, 0x358)).sw(c, 0);
        } else {
            c.cycles += 9;
            c.g[AT][0] = 0x4270_0000;
            c.mtc1(1, 0);
            c.mul_s(0, 12, 0);
            c.cvt_w_s(1, 0);
            c.mfc1(16, 1);
            // `s0` = the seconds at the 60 Hz authoring rate; the counter it
            // is added to counts ticks.
            if crate::tick::rate(c) != 60 && (c.g[S0][0] as i64) > 0 {
                c.g[S0][0] = crate::tick::ticks_from_frames60(c, c.g[S0][0] as u32) as u64;
            }
            call_ptr(c, 0x0012_98c8, 0x0010_e070)?;
            Addr(at_3(c, S1, 0x358)).sw(c, c.g[S2][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        }
        c.cycles += 7;
        Addr(at_3(c, S1, 0x354)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Rider_OnSplitMarker(rider, marker)` (0x0010e558): route a split
/// marker to the rider's metrics when the feature is enabled.
pub fn rider_on_split_marker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x530000);
        c.g[V1][0] = Addr(at_4(c, S0, 0xec)).lw(c);
        Addr(at_4(c, S0, 0xe8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(at_4(c, A0, 0x8d0)).lw(c);
        c.g[V0][0] = sx(((word(c, V0) as i32) >> 9) as u32);
        c.g[V0][0] &= 1;
        let disabled = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !disabled {
            c.cycles += 10;
            callee::CBE_GET_BE.call(c, 0x0010_e594)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            callee::CBE_GET_INTERFACE.call(c, 0x0010_e5a0)?;
            c.g[V1][0] = sx(0x530000);
            c.g[A0][0] = 0x1;
            c.g[V0][0] = Addr(at_4(c, V1, 0x5c12)).lb(c);
            if c.g[V0][0] != c.g[A0][0] {
                c.lq(16, frame.at(c, 0x20));
            } else {
                c.cycles += 3;
                c.g[A0][0] = Addr(at_4(c, S0, 0x790)).lw(c);
                c.g[A1][0] = Addr(at_4(c, S1, 0)).lw(c);
                callee::RIDER_METRIX_ON_SPLIT_MARKER.call(c, 0x0010_e5c0)?;
                c.cycles += 1;
                c.lq(16, frame.at(c, 0x20));
            }
        } else {
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float ComputerActorRider_GetPathClassWeight(Rider* self, int a1, int a2,
/// int a3)` (0x10d870): map the rider's path class (`s16` at_5 `+0xe00`) and the
/// caller flags to a weight, scaled by the `gp-0x7ce8` factor. Returns 0 for
/// the class/flags combinations that select "no weight".
pub fn computer_actor_rider_get_path_class_weight(c: &mut Cpu) {
    c.cycles += 3;
    let this = c.g[A0][0];
    c.g[V1][0] = lh_at(c, this, 0xe00);
    let kind = c.g[V1][0];
    let (a1, a2, a3) = (c.g[A1][0], c.g[A2][0], c.g[A3][0]);
    c.g[V0][0] = 1;
    let weighted;
    if kind != 0 {
        // class 1 (or anything else): the first dispatch charges its block.
        c.cycles += 2;
        if kind == 1 {
            c.g[V1][0] = 3;
            if a2 != 0 {
                weighted = true;
            } else {
                c.cycles += 2;
                c.cycles += 2;
                weighted = false;
            }
        } else {
            c.g[V0][0] = 2;
            c.cycles += 2;
            if kind != 2 {
                c.g[V0][0] = a1 ^ a2;
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
                c.g[V1][0] = 3;
                weighted = c.g[V0][0] == a3;
            } else {
                c.g[V1][0] = 3;
                if a3 != 0 {
                    weighted = true;
                } else {
                    c.cycles += 2;
                    c.cycles += 2;
                    weighted = false;
                }
            }
        }
    } else {
        c.cycles += 2;
        c.g[V1][0] = 3;
        weighted = a1 != 0;
    }
    if !weighted {
        c.cycles += 3;
        c.mtc1(0, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 11;
    c.g[V0][0] = 4;
    if a1 != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    let a = gpa(c, -0x7ce8);
    ldf_4(c, 1, a);
    c.g[V1][0] = sx(at_5(c.g[V0][0] as u32, 0xffff_ffff));
    if a2 != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.g[A0][0] = sx(at_5(c.g[V0][0] as u32, 0xffff_ffff));
    if a3 != 0 {
        c.g[V0][0] = c.g[A0][0];
    }
    c.mtc1(2, 0);
    c.cvt_s_w(0, 0);
    c.mul_s(0, 0, 1);
    abi::ret(c);
}

/// `void Rider_v15(Rider* self, float)` (0x10e830): when the rider's main
/// state is 1 or 4, record the best jump and give the +4 boost, then fire the
/// multiplier sound event.
pub fn rider_v15(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at_5(sp0, 0x10));
        c.swc1(20, at_5(sp0, 0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        c.mov_s(20, 12);
        callee::RIDER_GET_MAIN_STATE_2.call(c, 0x0010_e84c)?;
        c.g[V1][0] = 1;
        let this = c.g[S0][0];
        'body: {
            if c.g[V0][0] != c.g[V1][0] {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                callee::RIDER_GET_MAIN_STATE_2.call(c, 0x0010_e860)?;
                c.g[V1][0] = 4;
                if c.g[V0][0] != c.g[V1][0] {
                    break 'body;
                }
                c.cycles += 1;
            }
            c.g[A0][0] = lw_at(c, this, 0x790);
            c.cycles += 14;
            c.mov_s(12, 20);
            callee::RIDER_RECORD_BEST_JUMP.call(c, 0x0010_e878)?;
            c.mov_s(12, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 4;
            callee::RIDER_ADD_BOOST_3.call(c, 0x0010_e888)?;
            call_ptr(c, callee::SSX_AUDIO_GET_INSTANCE, 0x0010_e890)?;
            let audio = c.g[V0][0];
            c.g[A0][0] = audio;
            c.g[A2][0] = c.g[S0][0];
            c.mov_s(12, 20);
            c.g[A1][0] = 2;
            call_ptr(c, 0x0029_ced8, 0x0010_e8a4)?;
        }
        c.lq(16, at_5(sp0, 0x10));
        c.cycles += 4;
        c.g[RA][0] = c.ld(sp0);
        c.lwc1(20, at_5(sp0, 0x20));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_5(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider FX.

/// `void RiderBase_ResetFX(RiderBase* r)` (0x111890): reset every FX
/// sub-object hanging off the rider, then notify the game module that the
/// rider's FX were reset.
pub fn rider_base_reset_fx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        // The FX sub-objects, in order.
        for (off, target, ret) in [
            (0x3b0, 0x002d_cf28, 0x0011_18a8),
            (0x470, 0x002d_aa78, 0x0011_18b0),
            (0x520, 0x002e_8560, 0x0011_18b8),
            (0x610, 0x002e_6640, 0x0011_18c0),
            (0x9c0, 0x002e_adc0, 0x0011_18c8),
            (0xad0, 0x002e_f6a0, 0x0011_18d0),
            (0xaf0, 0x002d_4be0, 0x0011_18d8),
            (0xb00, 0x002e_3930, 0x0011_18e0),
            (0xb40, 0x002d_f3b0, 0x0011_18e8),
            (0xc70, 0x002f_1148, 0x0011_18f0),
            (0xd20, 0x002f_64e8, 0x0011_18f8),
        ] {
            c.g[RA][0] = sx(ret);
            c.g[A0][0] = sx(p(r(c, S0), off));
            call_ptr(c, target, ret)?;
        }
        c.g[RA][0] = sx(0x0011_1900);
        call_ptr(c, 0x002c_03e8, 0x0011_1900)?;
        'outer: {
            'notify: {
                c.g[V0][0] = Addr(p(r(c, S0), 0x18)).lw(c);
                c.g[A1][0] = Addr(p(r(c, V0), 0x870)).lw(c);
                let skip = (c.g[A1][0] as i64) < 0;
                c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
                if skip {
                    break 'notify;
                }
                c.cycles += 5;
                c.g[RA][0] = sx(0x0011_1918);
                c.g[A0][0] = Addr(p(r(c, V0), 0x84)).lw(c);
                call_ptr(c, 0x0023_06a8, 0x0011_1918)?;
                c.g[A1][0] = c.g[V0][0];
                let none = c.g[A1][0] == 0;
                frame.restore(c, 16, 0x10);
                if none {
                    break 'outer;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r(c, A1), 0xc)).lw(c);
                c.g[A0][0] = Addr(p(r(c, V0), 0x70)).lh(c);
                c.g[V1][0] = Addr(p(r(c, V0), 0x74)).lw(c);
                let target = r(c, V1);
                c.g[RA][0] = sx(0x0011_1938);
                c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                call_ptr(c, target, 0x0011_1938)?;
                break 'notify;
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `lwc1 f, gp+off`, returning the bits.
#[inline(always)]
pub(crate) fn gpf(c: &mut Cpu, f: usize, off: i32) -> u32 {
    c.lwc1(f, gp(c, off));
    c.f[f]
}

// ---------------------------------------------------------------------------
// Rider difficulty.

/// Jump tables of `setIndividualRiderDifficulty` (`.rodata`).
pub(crate) const DIFFICULTY_TABLE_A: u32 = 0x0045_6a90;

pub(crate) const DIFFICULTY_TABLE_B: u32 = 0x0045_6ab0;

pub(crate) const DIFFICULTY_TABLE_C: u32 = 0x0045_6ad0;

/// One full table-A case (levels 2..5): skills A/B (`self+0xdf8`/`+0xdfc`)
/// and the tune record's acceleration (`+0xdc`), speed (`+0xe0`) and boost
/// (`+0xe4`).  `cycles` is the whole case (the original splits it in two
/// blocks; there is no event check between them).
pub(crate) fn difficulty_case_full(
    c: &mut Cpu,
    obj: u32,
    cycles: u64,
    skill_a: u32,
    skill_b: u32,
    accel: u32,
    speed: u32,
    boost: u64,
) {
    c.cycles += cycles;
    c.f[0] = skill_b;
    c.f[2] = skill_a;
    let tune = Addr(obj).off(0x18).lw(c);
    c.f[1] = accel;
    c.g[V0][0] = tune;
    Addr(obj).off(0xdfc).write_fpu(c, FpuF32(skill_b));
    Addr(obj).off(0xdf8).write_fpu(c, FpuF32(skill_a));
    c.f[0] = speed;
    Addr(tune as u32).off(0xdc).write_fpu(c, FpuF32(accel));
    c.g[V0][0] = tune;
    Addr(tune as u32).off(0xe0).write_fpu(c, FpuF32(speed));
    c.g[V1][0] = tune;
    Addr(tune as u32).off(0xe4).sw(c, boost);
}

/// One skill/speed case (modes 0 and 2): skills A/B and the tune speed.
pub(crate) fn difficulty_case_skill_speed(
    c: &mut Cpu,
    obj: u32,
    cycles: u64,
    skill_a: u32,
    skill_b: u32,
    speed: u32,
) {
    c.cycles += cycles;
    c.f[1] = skill_a;
    c.f[2] = skill_b;
    c.f[0] = speed;
    let tune = Addr(obj).off(0x18).lw(c);
    c.g[V0][0] = tune;
    Addr(obj).off(0xdf8).write_fpu(c, FpuF32(skill_a));
    Addr(obj).off(0xdfc).write_fpu(c, FpuF32(skill_b));
    Addr(tune as u32).off(0xe0).write_fpu(c, FpuF32(speed));
}

/// `skillB *= 0.01f` and return (`0x10c748`).
pub(crate) fn difficulty_finish(c: &mut Cpu, obj: u32) {
    c.cycles += 4;
    let b = Addr(obj).off(0xdfc).read_fpu(c);
    let scale = gpf(c, 1, -0x7d20);
    let scaled = b * FpuF32(scale);
    c.f[0] = scaled.0;
    Addr(obj).off(0xdfc).write_fpu(c, scaled);
    abi::ret(c);
}

/// `cComputer_setIndividualRiderDifficulty(cComputer *self, int level, int
/// mode)` (0x10c4f8): choose the rider's skill values and tune record for a
/// difficulty level 1..5.  Table A always applies; mode 0 and mode 2 then
/// override skills A/B and speed.  Finally skill B is scaled by 0.01.
pub fn computer_actor_rider_apply_difficulty(c: &mut Cpu) {
    c.cycles += 4;
    let obj = abi::arg(c, 0);
    let level = abi::arg(c, 1);
    let mode = abi::arg(c, 2);
    c.g[A3][0] = c.g[A0][0];
    c.g[V1][0] = sx(level.wrapping_sub(1));
    let in_range = level.wrapping_sub(1) < 5;
    c.g[V0][0] = in_range as u64;
    if in_range {
        // 0x10c508: dispatch through table A.
        c.cycles += 7;
        c.g[V1][0] = sx(level.wrapping_sub(1) << 2);
        c.g[V0][0] = sx(DIFFICULTY_TABLE_A);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = c.lw(r32(c, V1));
        match level {
            1 => {
                // skillA = 100 via mtc1, skillB / accel / speed from the
                // table, boost 0.
                c.cycles += 14;
                c.g[AT][0] = 0x42c8_0000;
                c.mtc1(1, 0);
                c.lwc1(2, gp(c, -0x7dd4));
                let tune = Addr(obj).off(0x18).lw(c);
                c.lwc1(1, gp(c, -0x7dd0));
                let (a, b, acc) = (c.f[0], c.f[2], c.f[1]);
                Addr(obj).off(0xdf8).write_fpu(c, FpuF32(a));
                Addr(obj).off(0xdfc).write_fpu(c, FpuF32(b));
                Addr(tune as u32).off(0xdc).write_fpu(c, FpuF32(acc));
                c.lwc1(0, gp(c, -0x7dcc));
                c.g[V0][0] = tune;
                let speed = c.f[0];
                Addr(tune as u32).off(0xe0).write_fpu(c, FpuF32(speed));
                c.g[V1][0] = tune;
                Addr(tune as u32).off(0xe4).sw(c, 0);
            }
            2 => {
                let kb = gpf(c, 0, -0x7dc8);
                let ka = gpf(c, 2, -0x7dc4);
                let kacc = gpf(c, 1, -0x7dc0);
                let kspeed = gpf(c, 0, -0x7dbc);
                difficulty_case_full(c, obj, 14, ka, kb, kacc, kspeed, 1);
            }
            3 => {
                let kb = gpf(c, 0, -0x7db8);
                let ka = gpf(c, 2, -0x7db4);
                let kacc = gpf(c, 1, -0x7db0);
                let kspeed = gpf(c, 0, -0x7dac);
                difficulty_case_full(c, obj, 14, ka, kb, kacc, kspeed, 1);
            }
            4 => {
                let kb = gpf(c, 0, -0x7da8);
                let ka = gpf(c, 2, -0x7da4);
                let kacc = gpf(c, 1, -0x7da0);
                let kspeed = gpf(c, 0, -0x7d9c);
                difficulty_case_full(c, obj, 14, ka, kb, kacc, kspeed, 1);
            }
            _ => {
                let kb = gpf(c, 0, -0x7d98);
                let ka = gpf(c, 2, -0x7d94);
                let kacc = gpf(c, 1, -0x7d90);
                let kspeed = gpf(c, 0, -0x7d8c);
                difficulty_case_full(c, obj, 13, ka, kb, kacc, kspeed, 1);
            }
        }
    }
    // 0x10c608: mode dispatch.
    c.cycles += 2;
    c.g[V0][0] = 2;
    if mode != 0 {
        // 0x10c69c: only mode 2 has its own table.
        c.cycles += 2;
        if mode != 2 {
            c.f[0] = Addr(obj).off(0xdfc).read_fpu(c).0;
            difficulty_finish(c, obj);
            return;
        }
        // 0x10c6a4: dispatch through table C.
        c.cycles += 4;
        if !difficulty_dispatch_valid(c, level) {
            c.cycles += 1;
            c.f[0] = Addr(obj).off(0xdfc).read_fpu(c).0;
            difficulty_finish(c, obj);
            return;
        }
        c.cycles += 6;
        c.g[V1][0] = sx(level.wrapping_sub(1) << 2);
        c.g[V0][0] = sx(DIFFICULTY_TABLE_C);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = c.lw(r32(c, V1));
        match level {
            1 => {
                // Both skills are 100, speed from the table.
                c.cycles += 8;
                c.g[AT][0] = 0x42c8_0000;
                c.mtc1(1, 0);
                let tune = Addr(obj).off(0x18).lw(c);
                c.g[V0][0] = tune;
                c.lwc1(1, gp(c, -0x7d54));
                let skill = c.f[0];
                Addr(obj).off(0xdfc).write_fpu(c, FpuF32(skill));
                Addr(obj).off(0xdf8).write_fpu(c, FpuF32(skill));
                let speed = c.f[1];
                Addr(tune as u32).off(0xe0).write_fpu(c, FpuF32(speed));
            }
            2 => {
                let ka = gpf(c, 1, -0x7d50);
                let kb = gpf(c, 2, -0x7d4c);
                let ks = gpf(c, 0, -0x7d48);
                difficulty_case_skill_speed(c, obj, 8, ka, kb, ks);
            }
            3 => {
                let ka = gpf(c, 1, -0x7d44);
                let kb = gpf(c, 2, -0x7d40);
                let ks = gpf(c, 0, -0x7d3c);
                difficulty_case_skill_speed(c, obj, 8, ka, kb, ks);
            }
            4 => {
                let ka = gpf(c, 1, -0x7d38);
                let kb = gpf(c, 2, -0x7d34);
                let ks = gpf(c, 0, -0x7d30);
                difficulty_case_skill_speed(c, obj, 8, ka, kb, ks);
            }
            _ => {
                let ka = gpf(c, 1, -0x7d2c);
                let kb = gpf(c, 2, -0x7d28);
                let ks = gpf(c, 0, -0x7d24);
                difficulty_case_skill_speed(c, obj, 7, ka, kb, ks);
            }
        }
    } else {
        // 0x10c610: dispatch through table B.
        c.cycles += 4;
        if !difficulty_dispatch_valid(c, level) {
            c.cycles += 1;
            c.f[0] = Addr(obj).off(0xdfc).read_fpu(c).0;
            difficulty_finish(c, obj);
            return;
        }
        c.cycles += 6;
        c.g[V1][0] = sx(level.wrapping_sub(1) << 2);
        c.g[V0][0] = sx(DIFFICULTY_TABLE_B);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = c.lw(r32(c, V1));
        match level {
            1 => {
                c.g[AT][0] = 0x42c8_0000;
                c.mtc1(1, 1); // f1 = skill A = 100
                let skill_a = c.f[1];
                let kb = gpf(c, 2, -0x7d88);
                let ks = gpf(c, 0, -0x7d84);
                difficulty_case_skill_speed(c, obj, 9, skill_a, kb, ks);
            }
            2 => {
                let ka = gpf(c, 1, -0x7d80);
                let kb = gpf(c, 2, -0x7d7c);
                let ks = gpf(c, 0, -0x7d78);
                difficulty_case_skill_speed(c, obj, 8, ka, kb, ks);
            }
            3 => {
                let ka = gpf(c, 1, -0x7d74);
                let kb = gpf(c, 2, -0x7d70);
                let ks = gpf(c, 0, -0x7d6c);
                difficulty_case_skill_speed(c, obj, 8, ka, kb, ks);
            }
            4 => {
                let ka = gpf(c, 1, -0x7d68);
                let kb = gpf(c, 2, -0x7d64);
                let ks = gpf(c, 0, -0x7d24);
                difficulty_case_skill_speed(c, obj, 8, ka, kb, ks);
            }
            _ => {
                let ka = gpf(c, 1, -0x7d60);
                let kb = gpf(c, 2, -0x7d5c);
                let ks = gpf(c, 0, -0x7d58);
                difficulty_case_skill_speed(c, obj, 8, ka, kb, ks);
            }
        }
    }
    // 0x10c744.
    c.cycles += 1;
    c.f[0] = Addr(obj).off(0xdfc).read_fpu(c).0;
    difficulty_finish(c, obj);
}

/// The `level - 1 < 5` range check of the mode tables, including the
/// scratch registers the original leaves set.
pub(crate) fn difficulty_dispatch_valid(c: &mut Cpu, level: u32) -> bool {
    c.g[V1][0] = sx(level.wrapping_sub(1));
    c.g[V0][0] = (c.g[V1][0] < 5) as u64;
    let ok = c.g[V0][0] != 0;
    c.g[V0][0] = sx(0x0045_0000);
    ok
}

/// `lh_2` (sign-extended).
#[inline(always)]
pub(crate) fn lh_2(c: &mut Cpu, a: u32) -> u64 {
    c.lh(a)
}

/// `sd`.
#[inline(always)]
pub(crate) fn sd(c: &mut Cpu, a: u32, v: u64) {
    c.sd(a, v);
}

/// `void func_0010E5D8(void* rider, int* arg, int a2, int a3)` (0x10e5d8): a
/// race-path event for a rider: decide whether the race state allows the
/// event, and for event 1 finish a still-running rider, for event 11 forward
/// the event's index to the active camera trigger list.
pub fn rider_handle_race_path_event(c: &mut Cpu) {
    let frame = Frame::open(c, 0x40);
    'b_10e6d8: {
        'b_10e6d4: {
            'b_10e6d0: {
                c.cycles += 21;
                frame.save(c, S0, 0x30);
                frame.save(c, S2, 0x10);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S1, 0x20);
                sd(c, r32(c, SP), c.g[RA][0]);
                c.g[RA][0] = sx(0x0010e5f8);
                c.g[S2][0] = c.g[A1][0];
                if call_ptr(c, 0x0014dc80, 0x0010e5f8).is_err() {
                    return;
                }
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0010e604);
                c.g[A1][0] = 0u64;
                if call_ptr(c, 0x0014dd58, 0x0010e604).is_err() {
                    return;
                }
                c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0010e614);
                c.g[S1][0] = lw(c, at_6(c, V1, 0xc0));
                if call_ptr(c, 0x00144bc0, 0x0010e614).is_err() {
                    return;
                }
                c.g[A0][0] = lw(c, r32(c, V0));
                c.g[V0][0] = 0x530000;
                c.g[T0][0] = sx(at_6(c, V0, 0x5bc8));
                c.g[V1][0] = lb(c, at_6(c, T0, 0x48));
                let t = c.g[V1][0] == 0u64;
                c.g[T1][0] = 0u64;
                if t {
                    break 'b_10e6d0;
                } else {
                    c.cycles += 3;
                    c.g[A3][0] = 0x1;
                    let t = c.g[V1][0] == c.g[A3][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        break 'b_10e6d0;
                    } else {
                        c.cycles += 2;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_10e6d0;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[A2][0] = 0x5;
                            if t {
                                break 'b_10e6d0;
                            } else {
                                'b_10e660: {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != c.g[A2][0];
                                    c.g[A1][0] = 0x6;
                                    if t {
                                        break 'b_10e660;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = lw(c, at_6(c, S1, 0x98));
                                        if c.g[V0][0] != 0u64 {
                                            c.g[T1][0] = 0x1;
                                            break 'b_10e6d4;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A1][0] = 0x6;
                                            break 'b_10e660;
                                        }
                                    }
                                }
                                'b_10e674: {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != c.g[A1][0];

                                    if t {
                                        break 'b_10e674;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = lw(c, at_6(c, S1, 0x98));
                                        if c.g[V0][0] != 0u64 {
                                            c.g[T1][0] = 0x1;
                                            break 'b_10e6d4;
                                        } else {
                                            break 'b_10e674;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                if c.g[A0][0] == c.g[A2][0] {
                                    c.g[T1][0] = 0x1;
                                    break 'b_10e6d4;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A0][0] == c.g[A1][0];
                                    c.g[V0][0] = 0x7;
                                    if t {
                                        break 'b_10e6d0;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[A0][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x8;
                                        if t {
                                            break 'b_10e6d0;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[A0][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x9;
                                            if t {
                                                break 'b_10e6d0;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A0][0] == c.g[V0][0];
                                                c.g[V0][0] = 0xa;
                                                if t {
                                                    break 'b_10e6d0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[A0][0] == c.g[V0][0];
                                                    c.g[V0][0] = 0xb;
                                                    if t {
                                                        break 'b_10e6d0;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[A0][0] == c.g[V0][0];
                                                        c.g[V0][0] = 0xc;
                                                        if t {
                                                            break 'b_10e6d0;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == c.g[V0][0];
                                                            c.g[V0][0] = 0xd;
                                                            if t {
                                                                break 'b_10e6d0;
                                                            } else {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == c.g[V0][0] {
                                                                    c.g[T1][0] = 0x1;
                                                                    break 'b_10e6d4;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == c.g[A3][0] {
                                                                        c.g[T1][0] = 0x1;
                                                                        break 'b_10e6d4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] = lb(
                                                                            c,
                                                                            at_6(c, T0, 0x49),
                                                                        );
                                                                        if c.g[V0][0] == 0u64 {
                                                                            c.g[V1][0] =
                                                                                lw(c, r32(c, S2));
                                                                            break 'b_10e6d8;
                                                                        } else {
                                                                            break 'b_10e6d0;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[T1][0] = 0x1;
            break 'b_10e6d4;
        }
        c.cycles += 1;
        c.g[V1][0] = lw(c, r32(c, S2));
        break 'b_10e6d8;
    }
    'b_10e718: {
        c.cycles += 3;
        c.g[V0][0] = 0x1;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V1][0] = lw(c, r32(c, S2));
            break 'b_10e718;
        } else {
            c.cycles += 2;
            if c.g[T1][0] == 0u64 {
                c.g[V1][0] = lw(c, r32(c, S2));
                break 'b_10e718;
            } else {
                'b_10e704: {
                    c.cycles += 6;
                    c.lwc1(1, at_6(c, S0, 0x470));
                    c.mtc1(0, 0);
                    c.c_le_s(0, 1);
                    if !c.fcc() {
                        c.g[V0][0] = 0u64;
                        break 'b_10e704;
                    } else {
                        break 'b_10e704;
                    }
                }
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    c.g[V1][0] = lw(c, r32(c, S2));
                    break 'b_10e718;
                } else {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0010e714);
                    c.g[A0][0] = c.g[S0][0];
                    if callee::C00125108.call(c, 0x0010e714).is_err() {
                        return;
                    }
                    c.g[V1][0] = lw(c, r32(c, S2));
                    break 'b_10e718;
                }
            }
        }
    }
    'b_10e75c: {
        c.cycles += 3;
        c.g[V0][0] = 0xb;
        if c.g[V1][0] != c.g[V0][0] {
            frame.restore(c, S0, 0x30);
            break 'b_10e75c;
        } else {
            c.cycles += 14;
            c.g[A1][0] = lw(c, at_6(c, S0, 0x6c0));
            c.g[A2][0] = sx(at_6(c, S0, 0x6c0));
            c.g[V0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
            c.g[A0][0] = lh_2(c, at_6(c, A1, 0x38));
            c.g[V1][0] = lw(c, at_6(c, V0, 0x84));
            c.g[V0][0] = lw(c, at_6(c, A1, 0x3c));
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0010e748);
            c.g[S0][0] = lw(c, at_6(c, V1, 0x28));
            if call_ptr(c, t, 0x0010e748).is_err() {
                return;
            }
            c.g[A2][0] = lw(c, at_6(c, S2, 0x4));
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0010e758);
            c.g[A1][0] = c.g[V0][0];
            if call_ptr(c, 0x00270ab0, 0x0010e758).is_err() {
                return;
            }
            frame.restore(c, S0, 0x30);
            break 'b_10e75c;
        }
    }
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_6(c, SP, 0x40));
    c.pc = t;
    return;
}

// ---------------------------------------------------------------------------
// Spin.

/// `ComputerActorRider_UpdateSpinPlan(ai, float height)` (0x10b0e8).
const UPDATE_SPIN_PLAN: Callee = Callee {
    addr: 0x0010_b0e8,
    f: ee::s00::ComputerActorRider_UpdateSpinPlan,
};
/// `int ComputerActorRider_ChooseTrick(ai, float height)` (0x10cad8).
const CHOOSE_TRICK: Callee = Callee {
    addr: 0x0010_cad8,
    f: ee::s00::ComputerActorRider_ChooseTrick,
};
/// `int AnimSet_GetEntryType(set, int slot)` (0x311ae8): the type word of
/// the animation in `slot` (0 for the empty animation 0x1b6).
const ANIM_ENTRY_TYPE: u32 = 0x0031_1ae8;
/// `u32 RandU()` (0x317810).
const RAND_U: u32 = 0x0031_7810;

/// The AI's spin plan (fields of the `ComputerActorRider`).
pub(crate) mod spin {
    /// The rider (`Rider*`): its air state object at `+0x788` (state at
    /// `+0xac`, height at `+0x98`, ground below at `+0xa0`), the ticks spent
    /// spinning high up at `+0x484`, the animation set at `+0x784`.
    pub const RIDER: u32 = 0x18;
    pub const TRICK_STICK: u32 = 0xdf0;
    pub const TRICK: u32 = 0xe0c;
    pub const PLAN_READY: u32 = 0xe14;
    pub const PLAN_RESET: u32 = 0xe18;
    pub const SLOW_SPIN: u32 = 0xe20;
    pub const GRAB_HELD: u32 = 0xe24;
    pub const GRAB: u32 = 0xe28;
    pub const GRAB_START: u32 = 0xe2c;
    pub const GRAB_END: u32 = 0xe30;
    pub const TRICK_HEIGHT: u32 = 0xe34;
    pub const SPIN_RATE: u32 = 0xe3c;
    /// Rider fields.
    pub const AIR: u32 = 0x788;
    pub const SPIN_TICKS: u32 = 0x484;
    pub const ANIMS: u32 = 0x784;
    /// The AI gives up a spin it holds more than 30 60 Hz frames while more
    /// than 15 units above the ground.
    pub const GIVE_UP_FRAMES: u32 = 30;
}

/// `void ComputerActorRider_ThinkSpin(ComputerActorRider *ai, ControlInput
/// *in)` (0x10b250): the AI's input while spinning in the air.
///
/// While in air state 1 or 3 more than 15 units up, count the ticks
/// (rider `+0x484`); past 30 frames give the spin up (input flag 0x1000).
/// Otherwise plan the spin (`UpdateSpinPlan`), pick a trick when high
/// enough, start/stop grabs, or roll a random spin direction, and write the
/// stick (`+0xdf0` +0x48/+0x4c, scaled by 31), the trick byte and the grab
/// flags into `in`.
///
/// Tick rate: `+0x484` counts calls of this function (once per tick per
/// spinning AI rider), so the limit is `tick::ticks_from_frames60(30)`
/// (exact at 60).
/// `ComputerActorRider_ChooseTrick` (the uber/fitted/random trick rolls) is
/// tried once per 60 Hz frame of the AI ([`crate::tick::frame60_due`] keyed by
/// its `+0xe0c`) until a trick is picked, so failed rolls do not retry
/// `rate/60` times as often (every tick at 60).
pub fn computer_actor_rider_think_spin(c: &mut Cpu) {
    use spin::*;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        c.g[V1][0] = 1;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        let ai = c.g[S0][0] as u32;
        c.g[V0][0] = c.lw(ai.wrapping_add(RIDER));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(AIR));
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xac));
        c.g[A2][0] = 0;
        // Air states 1 and 3 count.
        if c.g[A1][0] == c.g[V1][0] {
            c.cycles += 1;
            c.g[A2][0] = 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 3;
            if c.g[A1][0] == c.g[V0][0] {
                c.cycles += 1;
                c.g[A2][0] = 1;
            }
        }
        // f20: the height above the ground (100 when not counting).
        c.cycles += 2;
        let mut high = false;
        if c.g[A2][0] == 0 {
            c.g[V0][0] = c.lw(ai.wrapping_add(RIDER));
            c.cycles += 2;
            c.g[AT][0] = 0x42c8_0000;
            c.mtc1(AT, 20);
        } else {
            c.cycles += 9;
            let air = c.g[A0][0] as u32;
            c.lwc1(0, air.wrapping_add(0xa0));
            c.lwc1(1, air.wrapping_add(0x98));
            c.g[AT][0] = 0x4170_0000;
            c.mtc1(AT, 2);
            c.sub_s(20, 1, 0);
            c.c_lt_s(2, 20);
            high = c.fcc();
            if !high {
                c.g[V0][0] = c.lw(ai.wrapping_add(RIDER));
            }
        }
        if high {
            c.cycles += 5;
            c.g[V1][0] = c.lw(ai.wrapping_add(RIDER));
            let rider = c.g[V1][0] as u32;
            c.g[V0][0] = sx((c.lw(rider.wrapping_add(SPIN_TICKS)) as u32).wrapping_add(1));
            c.sw(rider.wrapping_add(SPIN_TICKS), c.g[V0][0]);
        } else {
            c.cycles += 1;
            c.sw((c.g[V0][0] as u32).wrapping_add(SPIN_TICKS), 0);
        }

        // Held too long: give the spin up.
        c.cycles += 5;
        c.g[V1][0] = c.lw(ai.wrapping_add(RIDER));
        let rider = c.g[V1][0] as u32;
        c.g[V0][0] = c.lw(rider.wrapping_add(SPIN_TICKS));
        let limit = crate::tick::ticks_from_frames60(c, GIVE_UP_FRAMES) as i64 + 1;
        c.g[V0][0] = ((c.g[V0][0] as i64) < limit) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.sw(rider.wrapping_add(SPIN_TICKS), 0);
            let input = c.g[S1][0] as u32;
            c.g[V0][0] = c.lw(input) | 0x1000;
            c.sw(input, c.g[V0][0]);
        } else {
            c.g[V0][0] = c.lw(ai.wrapping_add(PLAN_READY));
            if think_spin_plan(c, ai)? {
                write_spin_input(c, ai);
            } else {
                c.cycles += 1;
                c.g[A2][0] = sx(ai.wrapping_add(TRICK_STICK));
                write_spin_input(c, ai);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// The plan part of `ThinkSpin` (`v0` = `+0xe14`, f20 = height). Returns
/// true when it already set `a2` to the trick stick (the original's direct
/// jumps to the input block), false when the caller does.
fn think_spin_plan(c: &mut Cpu, ai: u32) -> Result<bool, call::Unwound> {
    use spin::*;
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        // Plan the spin; a fresh plan gets a trick height of 15.
        c.cycles += 9;
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(12, 20);
        UPDATE_SPIN_PLAN.call(c, 0x0010_b31c)?;
        c.lwc1(1, ai.wrapping_add(TRICK_HEIGHT));
        c.mtc1(0, 0);
        c.c_le_s(1, 0);
        if c.fcc() {
            c.cycles += 4;
            c.g[AT][0] = 0x4170_0000;
            c.mtc1(AT, 0);
            c.swc1(0, ai.wrapping_add(TRICK_HEIGHT));
        }
        c.g[V0][0] = c.lw(ai.wrapping_add(PLAN_RESET));
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.swc1(20, ai.wrapping_add(TRICK_HEIGHT));
            c.sw(ai.wrapping_add(PLAN_RESET), 0);
            return Ok(false);
        }
        c.g[V1][0] = c.lw(ai.wrapping_add(TRICK));

        // No trick yet, higher than 0.5 and below the trick height: pick one
        // unless the animation in slot 2 is of type 0x14.
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 6;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(AT, 0);
            c.c_lt_s(0, 20);
            if c.fcc() {
                c.cycles += 5;
                c.lwc1(0, ai.wrapping_add(TRICK_HEIGHT));
                c.c_lt_s(20, 0);
                let below = c.fcc();
                c.g[A1][0] = 2;
                if below {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(ai.wrapping_add(RIDER));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(ANIMS));
                    call_ptr(c, ANIM_ENTRY_TYPE, 0x0010_b39c)?;
                    c.g[V1][0] = 0x14;
                    let skip = c.g[V0][0] == c.g[V1][0];
                    c.mov_s(12, 20);
                    if !skip {
                        // One pick (its random rolls) per 60 Hz frame of
                        // this AI: a failed pick retries on the next frame,
                        // not on every tick. Between frames the tick ends as
                        // after a failed pick (no trick yet).
                        if !crate::tick::frame60_due(c, ai.wrapping_add(TRICK), 0) {
                            return Ok(false);
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        CHOOSE_TRICK.call(c, 0x0010_b3b0)?;
                        c.sw(ai.wrapping_add(TRICK), c.g[V0][0]);
                        return Ok(false);
                    }
                }
                c.cycles += 1;
            }
        }
        c.g[V0][0] = c.lw(ai.wrapping_add(SLOW_SPIN));

        // A slow spin under 1 unit up: drop the trick and the grab.
        c.cycles += 2;
        let slow = c.g[V0][0] != 0;
        c.g[V1][0] = 0;
        if slow {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.c_lt_s(20, 0);
            if c.fcc() {
                c.g[V1][0] = 1;
            }
        }
        c.cycles += 2;
        let drop = c.g[V1][0] != 0;
        c.g[V0][0] = u64::MAX;
        if drop {
            c.cycles += 3;
            c.sw(ai.wrapping_add(GRAB), 0);
            c.sw(ai.wrapping_add(TRICK), c.g[V0][0]);
            return Ok(false);
        }

        // The grab window: [start + k, end - 2).
        c.cycles += 7;
        let gp = c.g[GP][0] as u32;
        c.lwc1(0, ai.wrapping_add(GRAB_START));
        c.lwc1(1, gp.wrapping_sub(0x7e10));
        c.add_s(0, 0, 1);
        c.c_le_s(20, 0);
        if c.fcc() {
            c.sw(ai.wrapping_add(GRAB), 0);
        } else {
            c.cycles += 8;
            c.lwc1(0, ai.wrapping_add(GRAB_END));
            c.g[AT][0] = 0x4000_0000;
            c.mtc1(AT, 1);
            c.sub_s(0, 0, 20);
            c.c_lt_s(1, 0);
            let inside = c.fcc();
            c.g[A2][0] = sx(ai.wrapping_add(TRICK_STICK));
            if !inside {
                return Ok(true);
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(ai.wrapping_add(GRAB_HELD));
            let held = c.g[V0][0] != 0;
            c.g[V0][0] = u64::MAX;
            if held {
                return Ok(true);
            }
            c.cycles += 1;
            c.sw(ai.wrapping_add(GRAB), 0);
        }
        c.cycles += 3;
        c.sw(ai.wrapping_add(TRICK), c.g[V0][0]);
        c.sw(ai.wrapping_add(GRAB_HELD), 0);
        return Ok(false);
    }

    // A plan exists: with no spin rate and no trick, roll a trick (0..14)
    // and a grab.
    c.lwc1(1, ai.wrapping_add(SPIN_RATE));
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_eq_s(1, 0);
    let still = c.fcc();
    c.g[V0][0] = u64::MAX;
    if !still {
        return Ok(false);
    }
    c.cycles += 3;
    c.g[V1][0] = c.lw(ai.wrapping_add(TRICK));
    let has_trick = c.g[V1][0] != c.g[V0][0];
    c.g[A2][0] = sx(ai.wrapping_add(TRICK_STICK));
    if has_trick {
        return Ok(true);
    }
    c.cycles += 6;
    call_ptr(c, RAND_U, 0x0010_b470)?;
    c.g[V1][0] = 0xf;
    c.divu(0, V0, V1);
    c.cycles += 6;
    c.g[A0][0] = c.hi[0];
    c.sw(ai.wrapping_add(TRICK), c.g[A0][0]);
    call_ptr(c, RAND_U, 0x0010_b48c)?;
    c.g[V0][0] &= 1;
    let grab = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if grab {
        c.cycles += 1;
        c.sw(ai.wrapping_add(GRAB), c.g[V0][0]);
    }
    Ok(false)
}

/// The input block of `ThinkSpin` (`a2` = the trick stick): the stick
/// scaled by 31 into the low 6 bits of `in[0] >> 24` and `in[1]`, the trick
/// byte at `in + 2`, the grab flag 0x4000, and a slow spin's flag bits.
fn write_spin_input(c: &mut Cpu, ai: u32) {
    use spin::*;
    let input = c.g[S1][0] as u32;
    let stick = c.g[A2][0] as u32;
    c.cycles += 29;
    c.g[AT][0] = 0x41f8_0000;
    c.mtc1(AT, 1);
    c.lwc1(0, stick.wrapping_add(0x48));
    c.g[V0][0] = 0xffff_ffff_c0ff_0000;
    c.g[A0][0] = c.lw(input);
    c.g[V1][0] = 0xffff_ffff_ffff_ffc0;
    c.mul_s(0, 0, 1);
    c.g[A1][0] = c.lw(input.wrapping_add(4));
    c.g[V0][0] |= 0xffff;
    c.g[A1][0] &= c.g[V1][0];
    c.g[A0][0] &= c.g[V0][0];
    c.cvt_w_s(2, 0);
    c.mfc1(V0, 2);
    c.g[V0][0] &= 0x3f;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
    c.g[A0][0] |= c.g[V0][0];
    c.sw(input, c.g[A0][0]);
    c.lwc1(0, stick.wrapping_add(0x4c));
    c.mul_s(0, 0, 1);
    c.cvt_w_s(1, 0);
    c.mfc1(V0, 1);
    c.g[V0][0] &= 0x3f;
    c.g[A1][0] |= c.g[V0][0];
    c.sw(input.wrapping_add(4), c.g[A1][0]);
    c.g[V0][0] = c.lbu(ai.wrapping_add(TRICK));
    c.sb(input.wrapping_add(2), c.g[V0][0]);
    c.g[V1][0] = c.lw(ai.wrapping_add(GRAB_HELD));
    if c.g[V1][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = c.lw(input) | 0x4000;
    } else {
        c.g[V0][0] = c.lw(ai.wrapping_add(GRAB));
        c.cycles += 2;
        let grab = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(input);
        c.cycles += 2;
        if grab {
            c.g[V0][0] |= 0x4000;
        } else {
            c.g[V1][0] = 0xffff_ffff_ffff_bfff;
            c.g[V0][0] &= c.g[V1][0];
        }
    }
    c.cycles += 5;
    c.sw(input, c.g[V0][0]);
    c.g[V1][0] = c.lw(ai.wrapping_add(SLOW_SPIN));
    c.g[V0][0] = 1;
    if c.g[V1][0] == c.g[V0][0] {
        c.cycles += 9;
        c.g[V0][0] = c.lw(input.wrapping_add(4));
        c.g[V1][0] = 0xffff_ffff_fffc_0000;
        c.g[A0][0] = 0x20000;
        c.g[V1][0] |= 0xfff;
        c.g[V0][0] &= c.g[V1][0];
        c.g[A0][0] |= 0x1000;
        c.g[V0][0] |= c.g[A0][0];
        c.sw(input.wrapping_add(4), c.g[V0][0]);
    }
    c.g[V1][0] = c.lw(ai.wrapping_add(TRICK));
    c.cycles += 1;
    c.g[V0][0] = u64::MAX;
}

// ---------------------------------------------------------------------------
// The start gate and attacks (tick-rate T4: AI frame counter readers).

/// `int AI_GetFrameCounter()` (0x1298c8): the AI frame counter (AI `+0x8`,
/// incremented once per tick by `AI_Update`).
const AI_FRAME_COUNTER: u32 = 0x0012_98c8;
/// `BEAggressionInt* BEAggressionInt_GetInstance()` (0x155a50).
const AGGRESSION_INSTANCE: u32 = 0x0015_5a50;
/// `int BEAggressionInt_GetRivalLevel(aggr, int rider, int other)` (0x155b50).
const AGGRESSION_RIVAL_LEVEL: u32 = 0x0015_5b50;

/// Fields read by `ThinkGate` and `FindAttackTarget`.
pub(crate) mod gate {
    /// `ComputerActorRider +0x18`: the rider. Its object at `+0x6c0` has a
    /// vtable whose slot 7 (`+0x38` delta, `+0x3c` function) answers the
    /// rider's index in the race.
    pub const RIDER: u32 = 0x18;
    pub const RIDER_V: u32 = 0x6c0;
    /// `ComputerActorRider +0xdf0`: the gate steering jitter (-1..1, f32).
    pub const STEER_JITTER: u32 = 0xdf0;
    /// The jitter is re-rolled every 20 60 Hz frames.
    pub const JITTER_FRAMES: u32 = 20;
    /// `Rider +0x1b0`: velocity; `+0x8 + 0x24 i` / `+0xc + 0x24 i`: the
    /// distance and direction to rider i; `+0xf4`: the AI frame counter at
    /// the last attack.
    pub const VELOCITY: u32 = 0x1b0;
    pub const OTHER_DIST: u32 = 0x8;
    pub const OTHER_ANGLE: u32 = 0xc;
    pub const OTHER_STRIDE: u32 = 0x24;
    pub const LAST_ATTACK: u32 = 0xf4;
    /// 60 Hz frames between two attacks of one AI (2 s).
    pub const ATTACK_COOLDOWN_FRAMES: u32 = 120;
    /// `AI +0x78`: the number of riders.
    pub const AI_RIDER_COUNT: u32 = 0x78;
}

/// The forced path reselect of `Rider_UpdateRaceProgress` and
/// `Rider_UpdateAIPathProgress` when 500 units off the path runs once a
/// second: `AI frame counter % period == 0`, period = 60 ticks at 60 Hz
/// (`li a1, 0x3c`), `rate` ticks otherwise.
pub(crate) fn reselect_period(c: &mut Cpu) -> u64 {
    crate::tick::ticks_from_frames60(c, 60) as u64
}

/// `void ComputerActorRider_ThinkGate(ComputerActorRider *ai, ControlInput
/// *in)` (0x10b790): the AI's steering input at the start gate.
///
/// `t = 180 - now` (60 Hz frames left of the start countdown, `now` = the AI
/// frame counter, reset at the race start). Riders with index < 4 push off
/// (steer +31) once `t < 33 - (index - 1) * 10`; riders with index < 3 lean
/// back (steer -31) while `t < 72`. Otherwise the gate jitter `+0xdf0` (a
/// random -1..1 re-rolled every 20 frames) is written, scaled by 31.
///
/// Tick rate: `now` counts ticks. Off 60 it is converted to 60 Hz frames
/// (`now * 60 / rate`) before `180 - now`, so the thresholds keep their
/// wall-clock times; the re-roll period is `ticks_from_frames60(20)`. Both
/// are exact at 60.
pub fn computer_actor_rider_think_gate(c: &mut Cpu) {
    use gate::*;
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x38);
        frame.save_fpr(c, 20, 0x30);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let ai = c.g[S0][0] as u32;
        let input = c.g[S1][0] as u32;
        call_ptr(c, AI_FRAME_COUNTER, 0x0010_b7b4)?;

        // f21 = 180 - now (frames left), f20 = 33 - (index - 1) * 10.
        c.g[AT][0] = 0x4204_0000;
        c.mtc1(AT, 20);
        c.g[A0][0] = c.lw(ai.wrapping_add(RIDER));
        c.mtc1(V0, 1);
        c.cvt_s_w(1, 1);
        let rate = crate::tick::rate(c);
        if rate != 60 {
            // The elapsed time in 60 Hz frames.
            let now = c.g[V0][0] as u32 as i32;
            let frames60 = (now as f64 * 60.0 / rate as f64) as f32;
            setf(c, 1, FpuF32::from_f32(frames60));
        }
        c.g[AT][0] = 0x4334_0000;
        c.mtc1(AT, 0);
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(RIDER_V));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(RIDER_V));
        c.sub_s(21, 0, 1);
        c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x38));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x3c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0010_b7ec)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.g[V1][0] = 0xa;
        c.mult(0, V0, V0, V1);
        c.mtc1(V0, 0);
        c.cvt_s_w(0, 0);
        c.sub_s(20, 20, 0);
        c.c_lt_s(21, 20);

        // The control input word's steering field (6-bit signed, bits
        // 12..17): `steer` = write `a0` (+31: 0x1f000, -31: 0x21000) into
        // v0 (the word with the field cleared); else the jitter.
        let mut steer = false;
        if c.fcc() {
            // Push-off time for this rider: indices 1..3 only.
            c.cycles += 10;
            rider_index_call(c, ai, 0x0010_b82c)?;
            c.g[V0][0] = ((c.g[V0][0] as i64) < 4) as u64;
            let not_front = c.g[V0][0] == 0;
            c.g[A0][0] = 0x10000;
            if !not_front {
                c.cycles += 6;
                c.g[V0][0] = c.lw(input);
                c.g[V1][0] = 0xffff_ffff_fffc_0000;
                c.g[V1][0] |= 0xfff;
                c.g[V0][0] &= c.g[V1][0];
                c.g[A0][0] |= 0xf000;
                steer = true;
            }
        }
        if !steer {
            // b_10b850: lean back while fewer than 72 frames are left.
            c.cycles += 5;
            c.g[AT][0] = 0x4290_0000;
            c.mtc1(AT, 0);
            c.c_lt_s(21, 0);
            if c.fcc() {
                c.cycles += 10;
                rider_index_call(c, ai, 0x0010_b880)?;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 3) as u64;
                let not_front = c.g[V0][0] == 0;
                c.g[A0][0] = 0x20000;
                if !not_front {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(input);
                    c.g[V1][0] = 0xffff_ffff_fffc_0000;
                    c.g[V1][0] |= 0xfff;
                    c.g[V0][0] &= c.g[V1][0];
                    c.g[A0][0] |= 0x1000;
                    steer = true;
                }
            }
        }
        if steer {
            // b_10b8a0
            c.cycles += 3;
            c.g[V0][0] |= c.g[A0][0];
            c.sw(input, c.g[V0][0]);
        } else {
            // b_10b8ac: re-roll the jitter every 20 frames.
            c.cycles += 6;
            call_ptr(c, AI_FRAME_COUNTER, 0x0010_b8b4)?;
            c.g[V1][0] = crate::tick::ticks_from_frames60(c, JITTER_FRAMES) as u64;
            c.div(0, V0, V1);
            c.cycles += 3;
            c.g[A0][0] = c.hi[0];
            if c.g[A0][0] != 0 {
                c.lwc1(0, ai.wrapping_add(STEER_JITTER));
            } else {
                // A random 0..1 (unsigned u32 to float), scaled.
                c.cycles += 4;
                call_ptr(c, RAND_U, 0x0010_b8d8)?;
                let high = (c.g[V0][0] as i64) < 0;
                c.g[A0][0] = sx(ai.wrapping_add(STEER_JITTER));
                if !high {
                    c.cycles += 4;
                    c.mtc1(V0, 1);
                    c.cvt_s_w(1, 1);
                } else {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                    c.g[V0][0] &= 1;
                    c.g[V0][0] |= c.g[V1][0];
                    c.mtc1(V0, 1);
                    c.cvt_s_w(1, 1);
                    c.add_s(1, 1, 1);
                }
                c.cycles += 7;
                c.lwc1(0, gp(c, -0x7e0c));
                c.mul_s(0, 1, 0);
                c.swc1(0, c.g[A0][0] as u32);
                // A random sign.
                call_ptr(c, RAND_U, 0x0010_b918)?;
                c.g[V0][0] &= 1;
                let keep = c.g[V0][0] != 0;
                c.lwc1(0, ai.wrapping_add(STEER_JITTER));
                if !keep {
                    c.cycles += 3;
                    c.neg_s(0, 0);
                    c.swc1(0, ai.wrapping_add(STEER_JITTER));
                    c.lwc1(0, ai.wrapping_add(STEER_JITTER));
                }
            }
            // b_10b930: steer = (int)(jitter * 31) & 0x3f.
            c.cycles += 13;
            c.g[A0][0] = 0xffff_ffff_fffc_0000;
            c.g[AT][0] = 0x41f8_0000;
            c.mtc1(AT, 1);
            c.g[A0][0] |= 0xfff;
            c.g[V1][0] = c.lw(input);
            c.mul_s(0, 0, 1);
            c.g[V1][0] &= c.g[A0][0];
            c.cvt_w_s(1, 0);
            c.mfc1(V0, 1);
            c.g[V0][0] &= 0x3f;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 12);
            c.g[V1][0] |= c.g[V0][0];
            c.sw(input, c.g[V1][0]);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `riderV(ai->rider, 7)`: the rider's index in the race (`a0` = the
/// rider's `+0x6c0` object adjusted by the slot's delta, `v1` = vtable, `a1`
/// = delta).
fn rider_index_call(c: &mut Cpu, ai: u32, ret: u32) -> Flow {
    c.g[A0][0] = c.lw(ai.wrapping_add(gate::RIDER));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(gate::RIDER_V));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(gate::RIDER_V));
    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    call_ptr(c, target, ret)
}

/// `bool ComputerActorRider_FindAttackTarget(ComputerActorRider *ai, int
/// *side)` (0x10dbf0): on every 6th tick, look for a rider to knock: another
/// rider closer than 200 whose direction differs from our velocity's heading
/// by 30..150 degrees, against whom our rivalry level is at least 3, when
/// the last attack is 120 frames ago. Writes the side (`dir > 0`) to `*side`,
/// stamps the attack time (rider `+0xf4`) and returns 1; else 0.
///
/// Tick rate: the cooldown is `ticks_from_frames60(120)` ticks (exact at
/// 60). The `% 6` spreads the search over ticks and stays in ticks.
pub fn computer_actor_rider_find_attack_target(c: &mut Cpu) {
    use gate::*;
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xd0);
        let sp = sp(c);
        frame.save(c, S1, 0x80);
        frame.save(c, S6, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 25, 0xc8);
        frame.save_fpr(c, 24, 0xc0);
        frame.save_fpr(c, 23, 0xb8);
        frame.save_fpr(c, 22, 0xb0);
        frame.save_fpr(c, 21, 0xa8);
        frame.save_fpr(c, 20, 0xa0);
        let ai = c.g[S1][0] as u32;
        call_ptr(c, AI_FRAME_COUNTER, 0x0010_dc38)?;
        c.g[V1][0] = 6;
        c.div(0, V0, V1);

        // b_10dc48
        c.cycles += 3;
        c.g[A0][0] = c.hi[0];
        let off_tick = c.g[A0][0] != 0;
        c.g[V0][0] = 0;
        'done: {
            if off_tick {
                break 'done;
            }
            c.cycles += 6;
            c.g[V0][0] = c.lw(gp(c, -0x848));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[S3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            let mgr = c.g[S3][0] as u32;
            c.g[V0][0] = c.lw(mgr.wrapping_add(AI_RIDER_COUNT));
            let none = (c.g[V0][0] as i64) <= 0;
            c.g[S2][0] = 0;
            if !none {
                c.cycles += 5;
                c.lwc1(23, gp(c, -0x7ce0));
                c.g[S5][0] = 0;
                c.lwc1(25, gp(c, -0x7cdc));
                c.g[S4][0] = 0;
                c.lwc1(24, gp(c, -0x7cd8));
                loop {
                    if attack_candidate(c, ai, mgr, sp)? {
                        // Found: v0 = 1.
                        break 'done;
                    }
                    // b_10de58 (v0 = the rider count).
                    c.cycles += 5;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(OTHER_STRIDE));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(OTHER_STRIDE));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0010_dc80);
                    }
                }
            }
            // b_10de6c
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        // b_10de70
        c.cycles += 16;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore_ra(c, 0x20);
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

/// One rider `i = s2` of `FindAttackTarget` (s4 = s5 = `0x24 i`). Ok(true)
/// when it is the target (v0 = 1, `*side` and the attack time written);
/// Ok(false) with v0 = the rider count otherwise.
fn attack_candidate(c: &mut Cpu, ai: u32, mgr: u32, sp: u32) -> Result<bool, call::Unwound> {
    use gate::*;
    let skip = |c: &mut Cpu| -> Result<bool, call::Unwound> {
        c.g[V0][0] = c.lw(mgr.wrapping_add(AI_RIDER_COUNT));
        Ok(false)
    };
    // Not ourselves.
    c.cycles += 9;
    c.g[A0][0] = c.lw(ai.wrapping_add(RIDER));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(RIDER_V));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(RIDER_V));
    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    call_ptr(c, target, 0x0010_dc9c)?;
    if c.g[S2][0] == c.g[V0][0] {
        return skip(c);
    }
    // Closer than 200.
    c.cycles += 9;
    c.g[V1][0] = c.lw(ai.wrapping_add(RIDER));
    let rider = c.g[V1][0] as u32;
    c.g[AT][0] = 0x4348_0000;
    c.mtc1(AT, 0);
    c.g[V0][0] = sx(rider.wrapping_add(c.g[S4][0] as u32));
    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(OTHER_DIST));
    c.c_lt_s(1, 0);
    if !c.fcc() {
        return skip(c);
    }

    // f12 = atan2(vel.y, vel.x) (f20 = x, f21 = y, f22 = 0).
    c.cycles += 10;
    c.lq(2, rider.wrapping_add(VELOCITY));
    c.mtc1(0, 22);
    c.sq(2, sp);
    c.lq(2, rider.wrapping_add(VELOCITY));
    c.lwc1(21, sp.wrapping_add(4));
    c.sq(2, sp.wrapping_add(0x10));
    c.lwc1(20, sp.wrapping_add(0x10));
    c.c_eq_s(20, 22);
    let mut reload_rider = true;
    if c.fcc() {
        // x == 0: 0 when y == 0, else -/+ pi/2.
        c.cycles += 4;
        c.c_eq_s(21, 22);
        let y_zero = c.fcc();
        c.mov_s(12, 21);
        if !y_zero {
            c.cycles += 4;
            c.c_le_s(22, 21);
            c.lwc1(12, gp(c, -0x7cd4));
            let y_neg = !c.fcc();
            c.g[V0][0] = c.lw(ai.wrapping_add(RIDER));
            if !y_neg {
                c.cycles += 3;
                c.lwc1(12, gp(c, -0x7cd0));
            }
            reload_rider = false;
        }
    } else {
        // b_10dd1c
        c.cycles += 7;
        c.div_s(12, 21, 20);
        call_ptr(c, MATH_ATAN, 0x0010_dd28)?;
        c.c_lt_s(20, 22);
        let x_neg = c.fcc();
        c.mov_s(12, 0);
        if x_neg {
            c.cycles += 4;
            c.c_lt_s(22, 21);
            if !c.fcc() {
                c.sub_s(0, 0, 23);
            } else {
                c.cycles += 1;
                c.add_s(0, 0, 23);
            }
            // b_10dd4c
            c.cycles += 1;
            c.mov_s(12, 0);
        }
    }
    if reload_rider {
        // b_10dd50
        c.cycles += 1;
        c.g[V0][0] = c.lw(ai.wrapping_add(RIDER));
    }

    // b_10dd54: d = wrap(heading - direction to i) into [-pi, pi).
    c.cycles += 13;
    c.lwc1(1, gp(c, -0x7ccc));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 2);
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(OTHER_ANGLE));
    c.sub_s(12, 12, 0);
    c.mul_s(1, 12, 1);
    c.add_s(1, 1, 2);
    c.cvt_w_s(2, 1);
    c.cvt_s_w(2, 2);
    c.c_lt_s(1, 2);
    if c.fcc() {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.sub_s(2, 2, 0);
    }
    // b_10dd94: |d| in (lo, hi).
    c.cycles += 8;
    c.lwc1(0, gp(c, -0x7cc8));
    c.mul_s(0, 2, 0);
    c.sub_s(20, 12, 0);
    c.abs_s(0, 20);
    c.c_lt_s(25, 0);
    if !c.fcc() {
        return skip(c);
    }
    c.cycles += 4;
    c.c_lt_s(0, 24);
    if !c.fcc() {
        return skip(c);
    }

    // A rival (level >= 3).
    c.cycles += 18;
    call_ptr(c, AGGRESSION_INSTANCE, 0x0010_ddcc)?;
    c.g[A0][0] = c.lw(ai.wrapping_add(RIDER));
    c.g[S0][0] = c.g[V0][0];
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(RIDER_V));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(RIDER_V));
    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    call_ptr(c, target, 0x0010_ddec)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = c.g[S2][0];
    call_ptr(c, AGGRESSION_RIVAL_LEVEL, 0x0010_ddfc)?;
    c.g[V0][0] = ((c.g[V0][0] as i64) < 3) as u64;
    c.g[V0][0] ^= 1;
    if c.g[V0][0] == 0 {
        return skip(c);
    }

    // The cooldown: last attack + 120 frames < now.
    c.cycles += 7;
    c.g[V0][0] = c.lw(ai.wrapping_add(RIDER));
    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(LAST_ATTACK));
    let cooldown = crate::tick::ticks_from_frames60(c, ATTACK_COOLDOWN_FRAMES);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(cooldown));
    call_ptr(c, AI_FRAME_COUNTER, 0x0010_de1c)?;
    c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[S0][0] == 0 {
        return skip(c);
    }

    // Attack: *side = d > 0; stamp the time.
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_lt_s(0, 20);
    let right = c.fcc();
    c.g[V0][0] = 1;
    if !right {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 6;
    c.sw(c.g[S6][0] as u32, c.g[V0][0]);
    call_ptr(c, AI_FRAME_COUNTER, 0x0010_de48)?;
    c.g[V1][0] = c.lw(ai.wrapping_add(RIDER));
    c.sw((c.g[V1][0] as u32).wrapping_add(LAST_ATTACK), c.g[V0][0]);
    c.g[V0][0] = 1;
    Ok(true)
}

pub(crate) mod dist {
    pub(crate) use crate::game::ai::rider::dist::CLOSEST_POINT;
    pub(crate) use crate::game::ai::rider::dist::DIST;
    pub(crate) use crate::game::ai::rider::dist::PATH;
    pub(crate) use crate::game::ai::rider::dist::PATH_HINT;
    pub(crate) use crate::game::ai::rider::dist::PATH_LENGTH;
    pub(crate) use crate::game::ai::rider::dist::PATH_VTABLE;
    pub(crate) use crate::game::ai::rider::dist::POSITION;
    pub(crate) use crate::game::ai::rider::dist::PREV_DIST;
    pub(crate) use crate::game::ai::rider::dist::SEGMENT;
    pub(crate) use crate::game::ai::rider::dist::SEGMENT_RESULT;
    pub(crate) use crate::game::ai::rider::dist::SPLITS;
}

/// `void Rider_UpdateDistToFinish(Rider* r, float* along, vec* out, vec*
/// closest)` (0x112fb0): project the rider onto the course path, store the
/// distance along it and the distance to the finish; when the rider got
/// closer, fire `Rider_OnSplitMarker` for each split marker (up to six)
/// passed since the last update and update the segment cache with the
/// new range; otherwise only refresh it at the old distance.
pub fn rider_update_dist_to_finish(c: &mut Cpu) {
    use self::dist::*;
    use crate::call::VSlot;
    use crate::ps2f::FpuF32;
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x60);
        c.g[T1][0] = 1;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[A2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x50);
        let rider = Addr(r32(c, S1));
        c.g[A1][0] = sx(rider.off(POSITION).0);
        c.g[A0][0] = rider.off(PATH).lw(c);
        c.g[T0][0] = rider.off(PATH_HINT).lw(c);
        call_ptr(c, CLOSEST_POINT, 0x0011_2ff0)?;

        let along = FpuF32(c.f[0]);
        Addr(r32(c, S0)).write_fpu(c, along);
        let path = rider.off(PATH).lw(c);
        c.g[A3][0] = path;
        let prev = rider.off(PREV_DIST).read_fpu(c);
        c.f[12] = prev.0;
        let dist = Addr(path as u32).off(PATH_LENGTH).read_fpu(c) - along;
        c.f[1] = dist.0;
        let closer = dist.lt(prev);
        abi::set_fcc(c, closer);
        rider.off(DIST).write_fpu(c, dist);

        if !closer {
            // 0x001130e4: refresh the segment cache at the old distance.
            c.cycles += 9;
            let vtable = Addr(path as u32).off(PATH_VTABLE).lw(c);
            c.g[V1][0] = vtable;
            c.f[13] = c.f[12];
            c.g[A1][0] = sx(rider.off(SEGMENT).0);
            c.g[A2][0] = 0xc;
            let slot = VSlot::read(c, vtable, 0x8);
            c.g[V0][0] = slot.func;
            c.g[A0][0] = slot.this(path);
            call_ptr(c, slot.func as u32, 0x0011_3104)?;
            rider.off(SEGMENT_RESULT).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 10;
            c.g[V0][0] = sx(0x004d_33a0);
            c.f[2] = 0;
            c.g[S2][0] = 0;
            let first = Addr(SPLITS).off(4).read_fpu(c);
            c.f[0] = first.0;
            c.g[A1][0] = sx(SPLITS);
            let no_splits = first.eq_s(FpuF32(c.f[2]));
            abi::set_fcc(c, no_splits);
            c.g[S3][0] = sx(rider.off(SEGMENT).0);
            let mut reload_path = true;
            if !no_splits {
                c.cycles += 4;
                let all_ahead = first.lt(FpuF32(c.f[1]));
                abi::set_fcc(c, all_ahead);
                if all_ahead {
                    c.g[A3][0] = rider.off(PATH).lw(c);
                    reload_path = false;
                } else {
                    c.cycles += 3;
                    c.f[20] = c.f[2];
                    c.g[S0][0] = c.g[A1][0];
                    c.f[1] = Addr(r32(c, A1)).off(4).read_fpu(c).0;
                    // 0x00113058: s0 = marker, s2 = count, f1 = its distance.
                    loop {
                        c.cycles += 5;
                        let prev = rider.off(PREV_DIST).read_fpu(c);
                        c.f[0] = prev.0;
                        let crossed = FpuF32(c.f[1]).lt(prev);
                        abi::set_fcc(c, crossed);
                        if crossed {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x0010_e558,
                                f: ee::s00::Rider_OnSplitMarker,
                            }
                            .call(c, 0x0011_3074)?;
                        }
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                        c.cycles += 3;
                        let more = (c.g[S2][0] as i64) < 6;
                        c.g[V0][0] = more as u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
                        if !more {
                            break;
                        }
                        c.cycles += 5;
                        let next = Addr(r32(c, S0)).off(4).read_fpu(c);
                        c.f[1] = next.0;
                        let end = next.eq_s(FpuF32(c.f[20]));
                        abi::set_fcc(c, end);
                        c.g[A1][0] = c.g[S0][0];
                        if end {
                            break;
                        }
                        c.cycles += 5;
                        let now = rider.off(DIST).read_fpu(c);
                        c.f[0] = now.0;
                        let ahead = next.lt(now);
                        abi::set_fcc(c, ahead);
                        if ahead {
                            break;
                        }
                        c.f[1] = Addr(r32(c, A1)).off(4).read_fpu(c).0;
                        if event_due(c) {
                            deliver_event_here(c, 0x0011_3058);
                        }
                    }
                }
            }
            if reload_path {
                c.cycles += 1;
                c.g[A3][0] = rider.off(PATH).lw(c);
            }

            // 0x001130b0: update the segment cache over [dist, prev dist].
            c.cycles += 13;
            c.g[A1][0] = c.g[S3][0];
            c.f[12] = rider.off(PREV_DIST).read_fpu(c).0;
            c.g[A2][0] = 0xc;
            let path = c.g[A3][0];
            let vtable = Addr(path as u32).off(PATH_VTABLE).lw(c);
            c.g[V1][0] = vtable;
            c.f[13] = rider.off(DIST).read_fpu(c).0;
            let slot = VSlot::read(c, vtable, 0x8);
            c.g[V0][0] = slot.func;
            c.g[A0][0] = slot.this(path);
            call_ptr(c, slot.func as u32, 0x0011_30d4)?;
            let now = rider.off(DIST).read_fpu(c);
            c.f[0] = now.0;
            rider.off(SEGMENT_RESULT).sw(c, c.g[V0][0]);
            rider.off(PREV_DIST).write_fpu(c, now);
        }

        c.cycles += 8;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    });
}

pub(crate) mod attack_back {

    /// The AI rider's own rider (its `+0x6c0` interface, its distance table).
    pub const RIDER: u32 = 0x18;

    /// Seconds left of the current decision (float).
    pub const TIMER: u32 = 0xf30;

    /// The current decision: attack back (1) or not (0).
    pub const DECISION: u32 = 0xf34;

    /// AI manager fields: rider count and rider pointer array.
    pub const AI_COUNT: u32 = 0x78;

    pub const AI_RIDERS: u32 = 0x28;

    /// Per-rider distance table entry stride and distance field.
    pub const DIST_STRIDE: u32 = 0x24;

    pub const DIST: u32 = 0x8;

    /// Rider: the AI interface object (vtable at its start).
    pub const IFACE: u32 = 0x6c0;

    /// Rider: animation controller.
    pub const ANIM_CTL: u32 = 0x784;
}

/// `AI_GetFrameCounter()` (0x1298c8).
pub(crate) const AI_GET_FRAME_COUNTER: Callee = Callee {
    addr: 0x0012_98c8,
    f: ee::s00::f_001298c8,
};

/// `AnimController_GetLayerAnimEntry(ctl, layer)` (0x311ae8).
pub(crate) const ANIM_GET_LAYER_ANIM_ENTRY: u32 = 0x0031_1ae8;

/// `Rand_NextGame()` (0x317810).
pub(crate) const RAND_NEXT_GAME: u32 = 0x0031_7810;

/// `int ComputerActorRider_ShouldAttackBack(ComputerActorRider* self)`
/// (0x10da10): whether an AI rider hits back at a rider that is attacking
/// it. A running decision is kept for its timer (2 s), counted down by one
/// tick (`.lit4` gp-0x7ce4) per call; a negative timer is clamped to 0 and
/// answers no. With no decision, every 6th AI frame scans the riders: the
/// first other rider whose interface reports an attack (vtable slot 8),
/// closer than 200 and playing animation-entry kind 13 on layer 1 makes a
/// new decision: attack with a 25% chance (`Rand_NextGame() % 100 < 25`).
pub fn computer_actor_rider_should_attack_back(c: &mut Cpu) {
    use attack_back::*;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        c.f[0] = 0;
        frame.save(c, S2, 0x30);
        frame.save(c, S0, 0x50);
        let me = Addr(r32(c, A0));
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let timer = me.off(TIMER).read_fpu(c);
        c.f[1] = timer.0;
        let zero = FpuF32::ZERO;
        let negative = timer.lt(zero);
        abi::set_fcc(c, negative);
        // false: fall to the shared "return 0" (v0 = 0).
        let mut answered = false;
        if negative {
            c.cycles += 2;
            me.off(TIMER).write_fpu(c, zero);
        } else {
            c.cycles += 3;
            let running = zero.lt(timer);
            abi::set_fcc(c, running);
            if running {
                c.cycles += 5;
                let tick = Addr(r32(c, GP).wrapping_sub(0x7ce4)).read_fpu(c);
                c.g[V0][0] = me.off(DECISION).lw(c);
                let left = timer - tick;
                c.f[0] = left.0;
                me.off(TIMER).write_fpu(c, left);
                answered = true;
            } else {
                c.cycles += 6;
                AI_GET_FRAME_COUNTER.call(c, 0x0010_da74)?;
                c.g[V1][0] = 6;
                let (_, phase) = abi::div(c, r32(c, V0), 6);
                c.cycles += 3;
                c.g[A0][0] = sx(phase);
                c.g[V0][0] = 0;
                if phase != 0 {
                    answered = true;
                } else {
                    c.cycles += 6;
                    let world = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
                    c.g[V0][0] = world;
                    let ai_root = Addr((world as u32).wrapping_add(0x84)).lw(c);
                    c.g[V1][0] = ai_root;
                    let ai = Addr((ai_root as u32).wrapping_add(0xc)).lw(c);
                    c.g[S3][0] = ai;
                    let ai = Addr(ai as u32);
                    c.g[V0][0] = ai.off(AI_COUNT).lw(c);
                    c.g[S0][0] = 0;
                    if c.g[V0][0] as i64 > 0 {
                        c.cycles += 2;
                        c.g[S4][0] = 100;
                        c.g[A0][0] = me.off(RIDER).lw(c);
                        loop {
                            // Index of the rider this AI rides (slot 7).
                            c.cycles += 8;
                            let iface = (c.g[A0][0] as u32).wrapping_add(IFACE);
                            let vt = Addr(iface).lw(c);
                            c.g[V1][0] = vt;
                            c.g[A0][0] = sx(iface);
                            c.g[A1][0] = Addr((vt as u32).wrapping_add(0x38)).lh(c);
                            c.g[V0][0] = Addr((vt as u32).wrapping_add(0x3c)).lw(c);
                            let f = r32(c, V0);
                            c.g[A0][0] = sx(iface.wrapping_add(r32(c, A1)));
                            call_ptr(c, f, 0x0010_dac8)?;
                            let i = r32(c, S0);
                            let mine = c.g[S0][0] == c.g[V0][0];
                            c.g[V1][0] = sx(i << 2);
                            let mut count_loaded = false;
                            if !mine {
                                // Is rider i attacking (slot 8)?
                                c.cycles += 11;
                                c.g[V0][0] = sx(r32(c, S3).wrapping_add(AI_RIDERS));
                                let slot = r32(c, V0).wrapping_add(i << 2);
                                c.g[S1][0] = sx(slot);
                                let other = Addr(slot).lw(c);
                                let iface = (other as u32).wrapping_add(IFACE);
                                let vt = Addr(iface).lw(c);
                                c.g[V1][0] = vt;
                                c.g[A0][0] = sx(iface);
                                c.g[A1][0] = Addr((vt as u32).wrapping_add(0x40)).lh(c);
                                c.g[V0][0] = Addr((vt as u32).wrapping_add(0x44)).lw(c);
                                let f = r32(c, V0);
                                c.g[A0][0] = sx(iface.wrapping_add(r32(c, A1)));
                                call_ptr(c, f, 0x0010_daf4)?;
                                let attacking = c.g[V0][0] != 0;
                                c.g[V0][0] = 0x24;
                                if attacking {
                                    // Distance to rider i from my rider's table.
                                    c.cycles += 10;
                                    let table = me.off(RIDER).lw(c);
                                    let off = abi::mult(c, 0x24, i);
                                    c.g[V0][0] = sx(off);
                                    c.g[AT][0] = 0x4348_0000;
                                    let limit = FpuF32(0x4348_0000); // 200.0
                                    c.f[1] = limit.0;
                                    let entry = (table as u32).wrapping_add(off);
                                    c.g[V1][0] = sx(entry);
                                    let dist = Addr(entry.wrapping_add(DIST)).read_fpu(c);
                                    c.f[0] = dist.0;
                                    let near = dist.lt(limit);
                                    abi::set_fcc(c, near);
                                    c.g[A1][0] = 1;
                                    if near {
                                        c.cycles += 6;
                                        let other = Addr(r32(c, S1)).lw(c);
                                        c.g[V0][0] = other;
                                        c.g[A0][0] =
                                            Addr((other as u32).wrapping_add(ANIM_CTL)).lw(c);
                                        call_ptr(c, ANIM_GET_LAYER_ANIM_ENTRY, 0x0010_db30)?;
                                        c.g[V1][0] = 0xd;
                                        if c.g[V0][0] != 0xd {
                                            c.g[V0][0] = ai.off(AI_COUNT).lw(c);
                                            count_loaded = true; // (skips the +1 below)
                                        } else {
                                            // New decision: 25% attack, for 2 s.
                                            c.cycles += 6;
                                            call_ptr(c, RAND_NEXT_GAME, 0x0010_db44)?;
                                            let (_, roll) = abi::divu(c, r32(c, V0), 100);
                                            c.g[A0][0] = sx(me.off(DECISION).0);
                                            c.cycles += 3;
                                            c.g[V0][0] = sx(roll);
                                            c.g[V1][0] = sx(roll >> 1);
                                            let roll_f = if (roll as i32) < 0 {
                                                c.cycles += 5;
                                                let v = (roll & 1) | (roll >> 1);
                                                c.g[V0][0] = sx(v);
                                                let h = FpuF32::from_i32(v as i32);
                                                h + h
                                            } else {
                                                c.cycles += 4;
                                                FpuF32::from_i32(roll as i32)
                                            };
                                            c.f[1] = roll_f.0;
                                            c.cycles += 6;
                                            c.g[AT][0] = 0x41c8_0000;
                                            let chance = FpuF32(0x41c8_0000); // 25.0
                                            c.f[0] = chance.0;
                                            let attack = roll_f.lt(chance);
                                            abi::set_fcc(c, attack);
                                            c.g[V0][0] = 1;
                                            if !attack {
                                                c.cycles += 1;
                                                c.g[V0][0] = 0;
                                            }
                                            c.cycles += 6;
                                            me.off(DECISION).sw(c, c.g[V0][0]);
                                            c.g[AT][0] = 0x4000_0000;
                                            c.f[0] = 0x4000_0000; // 2.0
                                            c.g[V0][0] = me.off(DECISION).lw(c);
                                            me.off(TIMER).write_fpu(c, FpuF32(0x4000_0000));
                                            answered = true;
                                            break;
                                        }
                                    }
                                }
                            }
                            if !count_loaded {
                                c.cycles += 1;
                                c.g[V0][0] = ai.off(AI_COUNT).lw(c);
                            }
                            c.cycles += 4;
                            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                            let more = (c.g[S0][0] as i64) < (c.g[V0][0] as i64);
                            c.g[V0][0] = more as u64;
                            if !more {
                                break;
                            }
                            c.g[A0][0] = me.off(RIDER).lw(c);
                            if event_due(c) {
                                deliver_event_here(c, 0x0010_dab0);
                            }
                        }
                    }
                }
            }
        }
        if !answered {
            c.cycles += 1;
            c.g[V0][0] = 0;
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

// ---------------------------------------------------------------------------
// ComputerActorRider_FindJumpMarker (0x10b980)

/// ComputerActor: +0x18 the rider; the jump marker's take-off and landing
/// points go to +0xe50 and +0xe60.
pub(crate) const CA_RIDER: u32 = 0x18;

pub(crate) const CA_JUMP_FROM: u32 = 0xe50;

pub(crate) const CA_JUMP_TO: u32 = 0xe60;

/// Rider: track position (+0x4c0 across, +0x4c4 along), +0xab8 the track.
pub(crate) const RIDER_TRACK_X: u32 = 0x4c0;

pub(crate) const RIDER_TRACK_DIST: u32 = 0x4c4;

pub(crate) const RIDER_TRACK: u32 = 0xab8;

/// Track vtable (+0x34): `QueryMarkers(buf, max, x, dist)` at slot +8.
pub(crate) const TRACK_VTABLE: u32 = 0x34;

/// Marker records (0x10): type, flags, start and end distance.
pub(crate) const MARKER_JUMP: u64 = 0x10;

/// Speed units per flag step (gp-0x7e08: 27.77778, 100 km/h in m/s / 1).
pub(crate) const GP_MARKER_SPEED_UNIT: u32 = 0x7e08;

/// `Path_GetPointAtDistance(Vec4* out, Path* path, int* clamped, float d)`
/// (cross-shard: through the table).
pub(crate) const PATH_GET_POINT_AT_DISTANCE: u32 = 0x0026_ab20;

/// `bool ComputerActorRider_FindJumpMarker(ComputerActor* self, float*
/// speed, int* below, int* f3, int* f2, int* f1, int* f0)` (0x10b980): ask
/// the track for up to 12 markers in the 300 units ahead of the rider; for
/// the first jump marker (type 0x10) report its speed (flags >> 4, times
/// 27.78) and flag bits, whether it starts at or behind the rider, and store
/// the track points of its start and end. False when there is none.
pub fn computer_actor_rider_find_jump_marker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x180);
        c.g[AT][0] = 0x4396_0000; // 300.0
        c.mtc1(AT, 0);
        frame.save(c, S1, 0x160);
        frame.save(c, S4, 0x130);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S5, 0x120);
        c.g[S4][0] = c.g[T2][0];
        frame.save(c, S6, 0x110);
        c.g[S5][0] = c.g[T1][0];
        frame.save(c, S7, 0x100);
        c.g[S6][0] = c.g[T0][0];
        frame.save(c, FP, 0xf0);
        c.g[S7][0] = c.g[A3][0];
        frame.save(c, S0, 0x170);
        c.g[FP][0] = c.g[A1][0];
        frame.save(c, S2, 0x150);
        c.g[A1][0] = c.g[SP][0]; // marker buffer at sp
        frame.save(c, S3, 0x140);
        frame.save_ra(c, 0xe0);
        let this = Addr(c.g[S1][0] as u32);
        c.g[V0][0] = this.off(CA_RIDER).lw(c);
        let below_ptr = frame.at(c, 0xd0);
        c.sw(below_ptr, c.g[A2][0]);
        c.g[A2][0] = 0xc;
        let rider = Addr(c.g[V0][0] as u32);
        c.g[S3][0] = rider.off(RIDER_TRACK).lw(c);
        let dist = rider.off(RIDER_TRACK_DIST).read_fpu(c);
        c.g[V1][0] = Addr(c.g[S3][0] as u32).off(TRACK_VTABLE).lw(c);
        let x = rider.off(RIDER_TRACK_X).read_fpu(c);
        abi::set_fpr(c, 12, x);
        abi::set_fpr(c, 13, dist + FpuF32(c.f[0]));
        let vt = Addr(c.g[V1][0] as u32);
        c.g[A0][0] = vt.off(8).lh(c);
        c.g[V0][0] = vt.off(0xc).lw(c);
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = addu(c.g[S3][0], c.g[A0][0]);
        call_ptr(c, method, 0x0010_ba00)?;
        c.g[A1][0] = c.g[V0][0];
        let count = c.g[A1][0] as i64;
        c.g[A0][0] = 0;
        let mut found = false;
        if count > 0 {
            c.cycles += 5;
            let gp = c.g[GP][0] as u32;
            let unit = Addr(gp.wrapping_sub(GP_MARKER_SPEED_UNIT)).read_fpu(c);
            abi::set_fpr(c, 1, unit);
            c.g[A2][0] = MARKER_JUMP;
            c.g[S2][0] = addu(c.g[SP][0], 0xc0); // point result
            c.g[V1][0] = c.g[SP][0]; // marker
            c.g[S0][0] = addu(c.g[SP][0], 0xc); // marker's end distance
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let is_jump = c.g[V0][0] == c.g[A2][0];
                c.g[A0][0] = addu(c.g[A0][0], 1);
                if is_jump {
                    c.cycles += 24;
                    let flags = Addr(c.g[V1][0] as u32).off(4).lw(c);
                    c.g[V1][0] = flags;
                    let start = Addr((c.g[S0][0] as u32).wrapping_sub(4)).read_fpu(c);
                    abi::set_fpr(c, 12, start);
                    let fl = flags as u32 as i32;
                    c.g[V0][0] = sx((fl >> 4) as u32);
                    let speed = FpuF32::from_i32(fl >> 4) * unit;
                    abi::set_fpr(c, 0, speed);
                    c.g[A1][0] = sx((fl >> 3) as u32) & 1;
                    c.g[V0][0] = sx((fl >> 2) as u32) & 1;
                    c.g[A0][0] = sx((fl >> 1) as u32) & 1;
                    c.g[V1][0] &= 1;
                    Addr(c.g[FP][0] as u32).write_fpu(c, speed);
                    Addr(c.g[S7][0] as u32).sw(c, c.g[A1][0]);
                    Addr(c.g[S6][0] as u32).sw(c, c.g[V0][0]);
                    Addr(c.g[S5][0] as u32).sw(c, c.g[A0][0]);
                    Addr(c.g[S4][0] as u32).sw(c, c.g[V1][0]);
                    c.g[V0][0] = this.off(CA_RIDER).lw(c);
                    let now = Addr(c.g[V0][0] as u32).off(RIDER_TRACK_DIST).read_fpu(c);
                    abi::set_fpr(c, 0, now);
                    let behind = start.le(now);
                    abi::set_fcc(c, behind);
                    c.g[A2][0] = 1;
                    if !behind {
                        c.cycles += 1;
                        c.g[A2][0] = 0;
                    }
                    c.cycles += 17;
                    let below = frame.at(c, 0xd0);
                    c.g[V0][0] = c.lw(below);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S3][0];
                    Addr(c.g[V0][0] as u32).sw(c, c.g[A2][0]);
                    c.g[A2][0] = 0;
                    call_ptr(c, PATH_GET_POINT_AT_DISTANCE, 0x0010_baa8)?;
                    let point = frame.at(c, 0xc0);
                    c.lq(V0, point);
                    c.g[A0][0] = c.g[S2][0];
                    let end = Addr(c.g[S0][0] as u32).read_fpu(c);
                    abi::set_fpr(c, 12, end);
                    c.g[A1][0] = c.g[S3][0];
                    c.sq(V0, this.off(CA_JUMP_FROM).0);
                    c.g[A2][0] = 0;
                    call_ptr(c, PATH_GET_POINT_AT_DISTANCE, 0x0010_bac4)?;
                    let point = frame.at(c, 0xc0);
                    c.lq(V1, point);
                    c.g[V0][0] = 1;
                    c.sq(V1, this.off(CA_JUMP_TO).0);
                    found = true;
                    break;
                }
                c.cycles += 4;
                c.g[V1][0] = addu(c.g[V1][0], 0x10);
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = addu(c.g[S0][0], 0x10);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_ba20);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x170);
        frame.restore(c, S1, 0x160);
        frame.restore(c, S2, 0x150);
        frame.restore(c, S3, 0x140);
        frame.restore(c, S4, 0x130);
        frame.restore(c, S5, 0x120);
        frame.restore(c, S6, 0x110);
        frame.restore(c, S7, 0x100);
        frame.restore(c, FP, 0xf0);
        frame.restore_ra(c, 0xe0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod rider_relations {
    use super::ee;
    use super::Callee;

    /// `AI`: frame counter.
    pub const FRAME: u32 = 0x8;

    /// `AI`: `Rider*[]`.
    pub const RIDERS: u32 = 0x28;

    /// `AI`: riders in the race, and riders not taking part (subtracted).
    pub const COUNT: u32 = 0x78;

    pub const SKIP: u32 = 0x84;

    /// `Rider`: per-other-rider info, 0x24 bytes each, at offset 0.
    pub const INFO_SIZE: u32 = 0x24;

    /// Info entry: tracked (non-zero), distance, angle, aggressive flag.
    pub const INFO_ACTIVE: u32 = 0x0;

    pub const INFO_DIST: u32 = 0x8;

    pub const INFO_ANGLE: u32 = 0xc;

    pub const INFO_AGGRESSIVE: u32 = 0x1c;

    /// `Rider`: the distance window reset each pass (-50000 / 50000).
    pub const MIN_DIST: u32 = 0xdc;

    pub const MAX_DIST: u32 = 0xe0;

    /// `Rider`: the embedded object whose virtual slot 0x28 returns the
    /// position (`sVec4*`).
    pub const POS_OBJ: u32 = 0x6c0;

    pub const GET_POS_SLOT: u32 = 0x28;

    /// gp-relative constants: -50000, 50000, pi, -pi/2, pi/2, 1/(2 pi), 2 pi.
    pub const LIT_MIN_DIST: u32 = 0x7cac;

    pub const LIT_MAX_DIST: u32 = 0x7ca8;

    pub const LIT_PI: u32 = 0x7ca4;

    pub const LIT_NEG_HALF_PI: u32 = 0x7ca0;

    pub const LIT_HALF_PI: u32 = 0x7c9c;

    pub const LIT_INV_TWO_PI: u32 = 0x7c98;

    pub const LIT_TWO_PI: u32 = 0x7c94;

    pub const UPDATE_RACE_POSITIONS: Callee = Callee {
        addr: 0x0010_f998,
        f: ee::s00::AI_UpdateRacePositions,
    };

    pub const UPDATE_RIVALS: Callee = Callee {
        addr: 0x0010_f878,
        f: ee::s00::AI_UpdateRivals,
    };

    /// `Math_Atan(x)` (cross-shard).
    pub const ATAN: u32 = 0x0031_c228;

    /// `BEAggressionInt_GetInstance()`, `BEAggressionInt_GetRivalLevel(this, a, b)`.
    pub const AGGRESSION_GET_INSTANCE: u32 = 0x0015_5a50;

    pub const AGGRESSION_GET_RIVAL_LEVEL: u32 = 0x0015_5b50;
}

/// `void AI_UpdateRiderRelations(AI* ai)` (0x10f560): every sixth frame,
/// update the race positions, then for every pair of riders (i < j) that
/// rider i tracks: the distance between them in the ground plane
/// (`sqrt(dx^2 + dy^2)`, stored both ways), the angle from j to i
/// (`atan2(dy, dx)`, stored at j) and its opposite wrapped into
/// [-pi, pi) (stored at i), and whether i's rivalry level towards j is at
/// least 2. Also resets each rider's distance window, then updates rivals.
pub fn ai_update_rider_relations(c: &mut Cpu) {
    use rider_relations::*;
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x100);
        c.g[V0][0] = 6;
        for (reg, off) in [
            (S0, 0xc0),
            (S1, 0xb0),
            (S2, 0xa0),
            (S3, 0x90),
            (S4, 0x80),
            (S5, 0x70),
            (S6, 0x60),
            (S7, 0x50),
            (FP, 0x40),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x30);
        for (f, off) in [(24, 0xf0), (23, 0xe8), (22, 0xe0), (21, 0xd8), (20, 0xd0)] {
            frame.save_fpr(c, f, off);
        }
        // Stack slots: 0x20 the AI, 0x24/0x28 its rider array.
        let slot = |c: &Cpu, off: u32| sp(c).wrapping_add(off);
        c.sw(slot(c, 0x20), c.g[A0][0]);
        'out: {
            // 0x10f5b0: only every sixth frame.
            c.cycles += 5;
            c.g[A0][0] = c.lw(at(c, A0, FRAME));
            c.div(0, A0, V0);
            c.g[V1][0] = c.hi[0];
            frame.restore(c, S0, 0xc0);
            if c.g[V1][0] != 0 {
                break 'out;
            }
            c.cycles += 9;
            c.g[A0][0] = c.lw(slot(c, 0x20));
            UPDATE_RACE_POSITIONS.call(c, 0x0010_f5cc)?;
            c.g[V1][0] = c.lw(slot(c, 0x20));
            c.g[V0][0] = c.lw(at(c, V1, COUNT));
            c.g[V1][0] = c.lw(at(c, V1, SKIP));
            c.g[S6][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
            let few = (c.g[S6][0] as i64) < 2;
            c.g[A0][0] = few as u64;
            frame.restore(c, S0, 0xc0);
            if few {
                break 'out;
            }
            c.cycles += 2;
            c.g[S4][0] = 0;
            if (c.g[S6][0] as i64) > 0 {
                c.cycles += 4;
                c.g[A0][0] = c.lw(slot(c, 0x20));
                c.g[A0][0] = sx(at(c, A0, RIDERS));
                c.sw(slot(c, 0x24), c.g[A0][0]);
                c.g[V0][0] = c.lw(slot(c, 0x24));
                // 0x10f600: rider i = s4 (fp = 4 i); v0 = the rider array.
                loop {
                    c.cycles += 13;
                    c.g[V1][0] = sx(r32(c, S4) << 2);
                    c.g[FP][0] = c.g[V1][0];
                    c.lwc1(1, gp_lit(c, LIT_MIN_DIST));
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    c.lwc1(0, gp_lit(c, LIT_MAX_DIST));
                    c.g[V0][0] = c.lw(r32(c, V1));
                    c.g[S2][0] = sx(r32(c, S4).wrapping_add(1));
                    c.g[S7][0] = c.g[S2][0];
                    let pairs = (c.g[S2][0] as i64) < (c.g[S6][0] as i64);
                    c.g[A0][0] = pairs as u64;
                    c.swc1(1, at(c, V0, MIN_DIST));
                    c.g[V0][0] = c.lw(r32(c, V1));
                    c.swc1(0, at(c, V0, MAX_DIST));
                    if pairs {
                        // s2 = j, s1 = j * 0x24 (i's info on j), s3/s5 =
                        // &riders[j].
                        c.cycles += 9;
                        c.g[V1][0] = c.lw(slot(c, 0x24));
                        c.g[A0][0] = INFO_SIZE as u64;
                        c.mult(0, S1, S7, A0);
                        c.g[V0][0] = sx(r32(c, S7) << 2);
                        c.sw(slot(c, 0x28), c.g[V1][0]);
                        c.lwc1(24, gp_lit(c, LIT_PI));
                        c.g[S3][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                        c.g[S5][0] = c.g[S3][0];
                        loop {
                            let (riders_a, riders_b) = (slot(c, 0x24), slot(c, 0x28));
                            relate_pair(c, riders_a, riders_b)?;
                            c.cycles += 5;
                            c.g[S3][0] = sx(at(c, S3, 4));
                            c.g[S1][0] = sx(at(c, S1, INFO_SIZE));
                            let more = (c.g[S2][0] as i64) < (c.g[S6][0] as i64);
                            c.g[V0][0] = more as u64;
                            c.g[S5][0] = sx(at(c, S5, 4));
                            if !more {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x0010_f658);
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[S4][0] = c.g[S7][0];
                    let more = (c.g[S4][0] as i64) < (c.g[S6][0] as i64);
                    c.g[V0][0] = c.lw(slot(c, 0x24));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0010_f600);
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.lw(slot(c, 0x20));
            UPDATE_RIVALS.call(c, 0x0010_f834)?;
            frame.restore(c, S0, 0xc0);
        }
        c.cycles += 16;
        for (reg, off) in [
            (S1, 0xb0),
            (S2, 0xa0),
            (S3, 0x90),
            (S4, 0x80),
            (S5, 0x70),
            (S6, 0x60),
            (S7, 0x50),
            (FP, 0x40),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x30);
        for (f, off) in [(24, 0xf0), (23, 0xe8), (22, 0xe0), (21, 0xd8), (20, 0xd0)] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

/// One (i, j) pair of [`ai_update_rider_relations`] (0x10f658): rider i
/// is `riders[fp / 4]`, rider j `*s3`/`*s5`, s1 = j * 0x24, s4 = i, s2 = j
/// (advanced here). `riders_a`/`riders_b` are the stack slots holding the
/// rider array (0x24, 0x28).
pub(crate) fn relate_pair(c: &mut Cpu, riders_a: u32, riders_b: u32) -> crate::call::Flow {
    use rider_relations::*;
    c.cycles += 7;
    c.g[V1][0] = c.lw(riders_a);
    c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, FP)));
    c.g[A1][0] = c.lw(r32(c, V0));
    c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, S1)));
    c.g[V0][0] = c.lw(at(c, V1, INFO_ACTIVE));
    if c.g[V0][0] == 0 {
        c.g[S2][0] = sx(at(c, S2, 1));
        return Ok(());
    }

    // 0x10f670: both positions (virtual getPos), their difference.
    c.cycles += 30;
    c.g[V1][0] = c.lw(at(c, A1, POS_OBJ));
    c.g[A1][0] = sx(at(c, A1, POS_OBJ));
    c.mtc1(0, 22);
    c.g[A0][0] = c.lh(at(c, V1, GET_POS_SLOT));
    c.g[V0][0] = c.lw(at(c, V1, GET_POS_SLOT + 4));
    let method = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
    call_ptr(c, method, 0x0010_f690)?;
    c.g[A0][0] = c.lw(r32(c, S5));
    c.g[S0][0] = c.g[V0][0];
    c.g[V1][0] = c.lw(at(c, A0, POS_OBJ));
    c.g[A0][0] = sx(at(c, A0, POS_OBJ));
    c.g[A1][0] = c.lh(at(c, V1, GET_POS_SLOT));
    c.g[V0][0] = c.lw(at(c, V1, GET_POS_SLOT + 4));
    let method = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    call_ptr(c, method, 0x0010_f6b0)?;
    c.lqc2(3, r32(c, S0));
    c.lqc2(4, r32(c, V0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x10));
    c.lq(2, sp(c).wrapping_add(0x10));
    c.sq(2, sp(c));
    // f20 = dx, f21 = dy, f23 = distance.
    c.lwc1(20, sp(c));
    c.lwc1(21, sp(c).wrapping_add(4));
    c.mul_s(0, 20, 20);
    c.mul_s(1, 21, 21);
    c.c_eq_s(20, 22);
    c.add_s(0, 0, 1);
    c.sqrt_s(23, 0);
    let dx_zero = c.fcc();

    // The angle atan2(dy, dx) into f12, f2 = angle + pi, a0 = the riders.
    'angle: {
        if !dx_zero {
            // 0x10f718: atan(dy / dx), corrected by pi for dx < 0.
            c.cycles += 7;
            c.div_s(12, 21, 20);
            call_ptr(c, ATAN, 0x0010_f724)?;
            c.c_lt_s(20, 22);
            let left = c.fcc();
            c.mov_s(12, 0);
            if left {
                c.cycles += 4;
                c.c_lt_s(22, 21);
                if !c.fcc() {
                    c.sub_s(0, 0, 24);
                } else {
                    c.cycles += 1;
                    c.add_s(0, 0, 24);
                }
                c.cycles += 1;
                c.mov_s(12, 0);
            }
        } else {
            // dx == 0: 0 (dy == 0), pi/2 (dy > 0), -pi/2 (dy < 0).
            c.cycles += 4;
            c.c_eq_s(21, 22);
            let dy_zero = c.fcc();
            c.mov_s(12, 21);
            if !dy_zero {
                c.cycles += 4;
                c.c_le_s(22, 21);
                c.lwc1(12, gp_lit(c, LIT_NEG_HALF_PI));
                let up = c.fcc();
                c.g[A0][0] = c.lw(riders_b);
                if up {
                    c.cycles += 3;
                    c.lwc1(12, gp_lit(c, LIT_HALF_PI));
                    c.add_s(2, 12, 24);
                    break 'angle;
                }
            }
        }
        c.cycles += 2;
        c.g[A0][0] = c.lw(riders_b);
        c.add_s(2, 12, 24);
    }

    // 0x10f754: distance both ways; f1 = floor((angle + pi) / 2 pi).
    c.cycles += 18;
    c.lwc1(0, gp_lit(c, LIT_INV_TWO_PI));
    c.g[V1][0] = INFO_SIZE as u64;
    c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, FP)));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 1);
    c.g[V0][0] = c.lw(r32(c, S0));
    c.mul_s(0, 2, 0);
    c.mult(0, A0, S4, V1);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
    c.swc1(23, at(c, V0, INFO_DIST));
    c.add_s(0, 0, 1);
    c.cvt_w_s(1, 0);
    c.cvt_s_w(1, 1);
    c.g[V0][0] = c.lw(r32(c, S3));
    c.c_lt_s(0, 1);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    let rounded_up = c.fcc();
    c.swc1(23, at(c, V0, INFO_DIST));
    if rounded_up {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.sub_s(1, 1, 0);
    }

    // 0x10f7a8: the angles (i's wrapped, j's raw), then the rivalry level.
    c.cycles += 17;
    c.lwc1(0, gp_lit(c, LIT_TWO_PI));
    c.g[V1][0] = c.lw(r32(c, S0));
    c.mul_s(0, 1, 0);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S1)));
    c.sub_s(0, 2, 0);
    c.swc1(0, at(c, V1, INFO_ANGLE));
    c.g[V0][0] = c.lw(r32(c, S3));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    c.swc1(12, at(c, V0, INFO_ANGLE));
    call_ptr(c, AGGRESSION_GET_INSTANCE, 0x0010_f7d0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S4][0];
    c.g[A2][0] = c.g[S2][0];
    call_ptr(c, AGGRESSION_GET_RIVAL_LEVEL, 0x0010_f7e0)?;
    let mild = (c.g[V0][0] as i64) < 2;
    c.g[V0][0] = c.lw(r32(c, S0));
    if mild {
        c.cycles += 2;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
        c.sw(at(c, V0, INFO_AGGRESSIVE), 0);
    } else {
        c.cycles += 4;
        c.g[V1][0] = 1;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
        c.sw(at(c, V0, INFO_AGGRESSIVE), 1);
    }
    c.cycles += 1;
    c.g[S2][0] = sx(at(c, S2, 1));
    Ok(())
}

pub(crate) mod race_positions {
    use super::*;

    /// AI: the rider pointer array.
    pub const AI_RIDERS: u32 = 0x28;

    /// AI: how riders are ranked (0 off, 1 by distance to finish, 2 by score).
    pub const AI_RANK_MODE: u32 = 0x74;

    /// AI: number of riders.
    pub const AI_COUNT: u32 = 0x78;

    /// Rider: race position (0 = first).
    pub const RIDER_POSITION: u32 = 0xec;

    /// Rider: a float compared with 10.0 (the result is dead; only v0 keeps it).
    pub const RIDER_RACE_TIME: u32 = 0x470;

    /// Rider: the score object (its +0x198 is the score).
    pub const RIDER_SCORE_OBJ: u32 = 0x790;

    /// Rider: finished flag.
    pub const RIDER_FINISHED: u32 = 0x878;

    /// Score object: points.
    pub const SCORE_POINTS: u32 = 0x198;

    pub const ARE_ALL_RIDERS_LOADED: Callee = Callee {
        addr: 0x0012_a180,
        f: ee::s00::AI_AreAllRidersLoaded,
    };

    pub const GET_DIST_TO_FINISH: Callee = Callee {
        addr: 0x0011_3128,
        f: ee::s00::Rider_GetDistToFinish,
    };

    /// `EA_ShellSortPairs(n, keys, values)` (cross-shard).
    pub const SHELL_SORT_PAIRS: u32 = 0x003e_6328;

    /// `SSXAudioSystem_GetInstance()` (cross-shard).
    pub const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;

    /// `SSXAudioSystem_OnRacePositionChange(audio, rider, behind, ahead,
    /// old_position, new_position)` (cross-shard).
    pub const AUDIO_ON_POSITION_CHANGE: u32 = 0x0029_9e28;

    /// `base + off` of a register value (32-bit address arithmetic).
    #[inline(always)]
    pub fn race_pos_at(c: &Cpu, reg: usize, off: u32) -> u32 {
        r32(c, reg).wrapping_add(off)
    }
}

// ---------------------------------------------------------------------------
// RiderBase_SaveState (0x111ac0)

/// `RiderBase + 0x18`: the rider (physics/model side) whose parts are saved.
pub(crate) const RB_RIDER: u32 = 0x18;

/// Rider fields saved by pointer.
pub(crate) const RB_RIDER_ANIM: u32 = 0x784;

pub(crate) const RB_RIDER_MODEL: u32 = 0x780;

pub(crate) const RB_RIDER_AIR_PREDICTOR: u32 = 0x788;

pub(crate) const RB_RIDER_METRIX: u32 = 0x790;

pub(crate) const RB_RIDER_TARGET0: u32 = 0x864;

pub(crate) const RB_RIDER_TARGET1: u32 = 0x868;

pub(crate) const RB_RIDER_BLOCK0: u32 = 0xabc;

pub(crate) const RB_RIDER_BLOCK1: u32 = 0xac0;

pub(crate) const RB_RIDER_TRACK_PATH: u32 = 0xab4;

pub(crate) const RB_RIDER_AI_PATH: u32 = 0xab8;

/// `target + 0x150`: the id saved for a non-null target object.
pub(crate) const RB_TARGET_ID: u32 = 0x150;

/// `D_004D33A0`: the course path data.
pub(crate) const RB_PATH_DATA: u32 = 0x004d_33a0;

/// `RiderAnim_SaveState(anim, stream)` (0x104d68).
pub(crate) const RB_RIDER_ANIM_SAVE: Callee = Callee {
    addr: 0x0010_4d68,
    f: ee::s00::f_00104d68,
};

/// `AirPredictor_SaveState` (0x113aa0).
pub(crate) const RB_AIR_PREDICTOR_SAVE: Callee = Callee {
    addr: 0x0011_3aa0,
    f: ee::s00::f_00113aa0,
};

/// `RiderMetrix_SaveState` (0x1174b0).
pub(crate) const RB_RIDER_METRIX_SAVE: Callee = Callee {
    addr: 0x0011_74b0,
    f: ee::s00::f_001174b0,
};

/// `RiderModel_SaveState` (0x310ea8).
pub(crate) const RB_RIDER_MODEL_SAVE: u32 = 0x0031_0ea8;

/// `PathData_GetTrackPathIndex(paths, path)` (0x26b508).
pub(crate) const RB_PATH_TRACK_INDEX: u32 = 0x0026_b508;

/// `PathData_GetAIPathIndex(paths, path)` (0x26b550).
pub(crate) const RB_PATH_AI_INDEX: u32 = 0x0026_b550;

/// How a rider sub-state's `SaveState(self + off, stream)` is reached.
#[derive(Clone, Copy)]
pub(crate) enum RbSub {
    /// `jal` through the generated function.
    Direct(Callee),
    /// The generator's cross-shard `call_indirect`.
    Table(u32),
}

/// The sub-states embedded in `RiderBase`, saved in order:
/// (offset, saver, return address).
pub(crate) const RB_SUBSTATES: [(u32, RbSub, u32); 16] = [
    (0x000, RbSub::Table(0x0013_f848), 0x0011_1c3c), // RiderGroundState
    (0x020, RbSub::Table(0x0013_acb0), 0x0011_1c48),
    (0x030, RbSub::Table(0x0013_8ad8), 0x0011_1c54),
    (0x0b0, RbSub::Table(0x0013_c5e0), 0x0011_1c60),
    (0x110, RbSub::Table(0x0013_9970), 0x0011_1c6c),
    (
        0x1d0,
        RbSub::Direct(Callee {
            addr: 0x0013_1598,
            f: ee::s00::f_00131598,
        }),
        0x0011_1c78,
    ), // BoardPress
    (
        0x210,
        RbSub::Direct(Callee {
            addr: 0x0012_fbf0,
            f: ee::s00::f_0012fbf0,
        }),
        0x0011_1c84,
    ), // NaturalAir
    (0x230, RbSub::Table(0x0013_61f8), 0x0011_1c90),
    (
        0x290,
        RbSub::Direct(Callee {
            addr: 0x0012_c558,
            f: ee::s00::f_0012c558,
        }),
        0x0011_1c9c,
    ), // Gate
    (0x2b0, RbSub::Table(0x0013_2840), 0x0011_1ca8),
    (
        0x2c0,
        RbSub::Direct(Callee {
            addr: 0x0012_e708,
            f: ee::s00::f_0012e708,
        }),
        0x0011_1cb4,
    ), // Wipeout
    (
        0x350,
        RbSub::Direct(Callee {
            addr: 0x0012_f5b0,
            f: ee::s00::f_0012f5b0,
        }),
        0x0011_1cc0,
    ), // Reset
    (
        0x360,
        RbSub::Direct(Callee {
            addr: 0x0012_c9c0,
            f: ee::s00::f_0012c9c0,
        }),
        0x0011_1ccc,
    ), // FinishLine
    (0x370, RbSub::Table(0x0013_30b8), 0x0011_1cd8),
    (0x390, RbSub::Table(0x0013_68e8), 0x0011_1ce4), // UberGrind
    (0xd20, RbSub::Table(0x002f_6a58), 0x0011_1cf0), // cheat code
];

/// `stream->Write(a1, a2)` (vtable slot 0x8/0xc of the stream in s1): the
/// vtable pointer is already in `vt_reg`, the method pointer goes to `fn_reg`.
pub(crate) fn rb_stream_write(
    c: &mut Cpu,
    vt_reg: usize,
    fn_reg: usize,
    ret_addr: u32,
) -> crate::call::Flow {
    let vt = r32(c, vt_reg);
    c.g[A0][0] = Addr(vt.wrapping_add(0x8)).lh(c);
    c.g[fn_reg][0] = Addr(vt.wrapping_add(0xc)).lw(c);
    let target = r32(c, fn_reg);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret_addr)
}

/// `rider = self->rider` (`lw s2+0x18`), into `reg`.
pub(crate) fn rb_rider(c: &mut Cpu, reg: usize) -> u32 {
    c.g[reg][0] = Addr(r32(c, S2).wrapping_add(RB_RIDER)).lw(c);
    r32(c, reg)
}

/// `void RiderBase_SaveState(RiderBase* self, Stream* s)` (0x111ac0): write
/// the rider's 0x6c0-byte block, the anim/model/air-predictor/metrix states,
/// both target ids (-1 for none) with their 16-byte records, the two words at
/// +0xde0/+0xde4, every embedded sub-state, the two 32-byte blocks and the
/// track/AI path indices.
pub fn rider_base_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x60);
        c.g[A2][0] = 0x6c0;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[S2][0] = c.g[A0][0];
        c.g[S0][0] |= 0xffff; // -1: "no target"
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        let rider = Addr(r32(c, S2).wrapping_add(RB_RIDER)).lw(c);
        c.g[A1][0] = rider;
        rb_stream_write(c, V1, V0, 0x0011_1b00)?;

        let rider = rb_rider(c, V1);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = Addr(rider.wrapping_add(RB_RIDER_ANIM)).lw(c);
        RB_RIDER_ANIM_SAVE.call(c, 0x0011_1b10)?;
        let rider = rb_rider(c, V0);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = Addr(rider.wrapping_add(RB_RIDER_MODEL)).lw(c);
        call_ptr(c, RB_RIDER_MODEL_SAVE, 0x0011_1b20)?;
        let rider = rb_rider(c, V0);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = Addr(rider.wrapping_add(RB_RIDER_AIR_PREDICTOR)).lw(c);
        RB_AIR_PREDICTOR_SAVE.call(c, 0x0011_1b30)?;
        let rider = rb_rider(c, V0);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = Addr(rider.wrapping_add(RB_RIDER_METRIX)).lw(c);
        RB_RIDER_METRIX_SAVE.call(c, 0x0011_1b40)?;

        // Target 0: its id (or -1) at sp+0x10, then the 16-byte record.
        rb_target_id(c, RB_RIDER_TARGET0, 0x10);
        c.cycles += 20;
        c.g[A1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 4;
        rb_stream_write(c, V1, V0, 0x0011_1b7c)?;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = 0x10;
        let rider = rb_rider(c, V0);
        c.g[A1][0] = Addr(rider.wrapping_add(RB_RIDER_TARGET0)).lw(c);
        rb_stream_write(c, V1, A3, 0x0011_1b9c)?;

        // Target 1 at sp+0x14.
        rb_target_id(c, RB_RIDER_TARGET1, 0x14);
        c.cycles += 117;
        c.g[A1][0] = sx(sp(c).wrapping_add(0x14));
        c.g[A2][0] = 4;
        rb_stream_write(c, V1, V0, 0x0011_1bd8)?;
        c.g[A3][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = 0x10;
        let rider = rb_rider(c, V0);
        c.g[A1][0] = Addr(rider.wrapping_add(RB_RIDER_TARGET1)).lw(c);
        rb_stream_write(c, A3, V1, 0x0011_1bf8)?;

        c.g[A3][0] = Addr(r32(c, S1)).lw(c);
        c.g[A1][0] = sx(r32(c, S2).wrapping_add(0xde0));
        c.g[A2][0] = 4;
        rb_stream_write(c, A3, V0, 0x0011_1c14)?;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = 4;
        c.g[A1][0] = sx(r32(c, S2).wrapping_add(0xde4));
        rb_stream_write(c, V1, V0, 0x0011_1c30)?;

        for (off, saver, ret_addr) in RB_SUBSTATES {
            c.g[A0][0] = if off == 0 {
                c.g[S2][0]
            } else {
                sx(r32(c, S2).wrapping_add(off))
            };
            c.g[A1][0] = c.g[S1][0];
            match saver {
                RbSub::Direct(f) => f.call(c, ret_addr)?,
                RbSub::Table(addr) => call_ptr(c, addr, ret_addr)?,
            }
        }

        c.g[A3][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = 0x20;
        let rider = rb_rider(c, V0);
        c.g[A1][0] = Addr(rider.wrapping_add(RB_RIDER_BLOCK0)).lw(c);
        rb_stream_write(c, A3, V1, 0x0011_1d10)?;
        c.g[A3][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = 0x20;
        let rider = rb_rider(c, V0);
        c.g[A1][0] = Addr(rider.wrapping_add(RB_RIDER_BLOCK1)).lw(c);
        rb_stream_write(c, A3, V1, 0x0011_1d30)?;

        // The path indices go to sp+0 / sp+4.
        let rider = rb_rider(c, V0);
        c.g[S0][0] = sx(RB_PATH_DATA);
        c.g[A1][0] = Addr(rider.wrapping_add(RB_RIDER_TRACK_PATH)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, RB_PATH_TRACK_INDEX, 0x0011_1d48)?;
        let track = c.g[V0][0];
        Addr(sp(c)).sw(c, track);
        c.g[A0][0] = c.g[S0][0];
        let rider = rb_rider(c, V0);
        c.g[A1][0] = Addr(rider.wrapping_add(RB_RIDER_AI_PATH)).lw(c);
        call_ptr(c, RB_PATH_AI_INDEX, 0x0011_1d5c)?;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A1][0] = c.g[SP][0];
        let ai = c.g[V0][0];
        Addr(sp(c).wrapping_add(4)).sw(c, ai);
        c.g[A2][0] = 8;
        rb_stream_write(c, V1, V0, 0x0011_1d7c)?;

        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `sp[slot] = -1 (s0); if (*rider->field) sp[slot] = (*rider->field)->id;`
/// then `v1 = stream->vtable`.
pub(crate) fn rb_target_id(c: &mut Cpu, field: u32, slot: u32) {
    let none = c.g[S0][0];
    Addr(sp(c).wrapping_add(slot)).sw(c, none);
    let rider = rb_rider(c, V0);
    c.g[V1][0] = Addr(rider.wrapping_add(field)).lw(c);
    c.g[V1][0] = Addr(r32(c, V1)).lw(c);
    if c.g[V1][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(RB_TARGET_ID)).lw(c);
        let id = c.g[V0][0];
        Addr(sp(c).wrapping_add(slot)).sw(c, id);
    }
    c.g[V1][0] = Addr(r32(c, S1)).lw(c);
}

/// `void AI_UpdateRacePositions(AI* ai)` (0x10f998): once every rider is
/// loaded, give each rider a sort key (rank mode 1: finished riders 0xffff,
/// others `-(int)(dist_to_finish + 20 * position)`; mode 2: finished riders
/// -65535, others their score; any other mode leaves the stack slot as it
/// is), shell-sort the rider indices by key, then store each rider's new
/// position, telling the audio system (with the riders just behind and
/// ahead) about every rider whose position changed.
pub fn ai_update_race_positions(c: &mut Cpu) {
    use race_positions::*;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S3, 0xa0);
        frame.save(c, S0, 0xd0);
        c.g[S3][0] = c.g[A0][0];
        for (reg, off) in [
            (S1, 0xc0),
            (S2, 0xb0),
            (S4, 0x90),
            (S5, 0x80),
            (S6, 0x70),
            (S7, 0x60),
            (FP, 0x50),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x40);
        c.g[V0][0] = c.lw(race_pos_at(c, S3, AI_RANK_MODE));
        frame.restore(c, S0, 0xd0);
        'out: {
            if c.g[V0][0] == 0 {
                break 'out;
            }
            c.cycles += 4;
            ARE_ALL_RIDERS_LOADED.call(c, 0x0010_f9dc)?;
            c.g[S1][0] = 0;
            'done: {
                if c.g[V0][0] == 0 {
                    break 'done;
                }
                // Pass 1: indices at sp+0 (s5), keys at sp+0x20 (s2/s7).
                c.cycles += 4;
                c.g[V0][0] = c.lw(race_pos_at(c, S3, AI_COUNT));
                c.g[S7][0] = sx(sp(c).wrapping_add(0x20));
                c.g[A0][0] = c.g[V0][0];
                if (c.g[V0][0] as i64) > 0 {
                    c.cycles += 5;
                    c.g[S6][0] = 0x14;
                    c.g[S2][0] = c.g[S7][0];
                    c.g[S5][0] = c.g[SP][0];
                    c.g[FP][0] = 1;
                    c.g[S4][0] = sx(race_pos_at(c, S3, AI_RIDERS));
                    loop {
                        // 0x10fa08
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(race_pos_at(c, S3, AI_RANK_MODE));
                        c.sw(r32(c, S5), c.g[S1][0]);
                        c.g[S0][0] = c.lw(r32(c, S4));
                        let mut keyed = true;
                        if c.g[V1][0] == c.g[FP][0] {
                            // Rank by distance to the finish.
                            c.cycles += 3;
                            let finished = c.lw(race_pos_at(c, S0, RIDER_FINISHED));
                            c.g[V0][0] = 0xffff;
                            if finished != 0 {
                                c.cycles += 2;
                                c.sw(r32(c, S2), c.g[V0][0]);
                            } else {
                                c.cycles += 7;
                                c.lwc1(1, race_pos_at(c, S0, RIDER_RACE_TIME));
                                c.g[AT][0] = 0x4120_0000; // 10.0
                                c.mtc1(AT, 0);
                                c.c_le_s(0, 1);
                                c.g[V0][0] = 1;
                                if !c.fcc() {
                                    c.cycles += 1;
                                    c.g[V0][0] = 0;
                                }
                                c.cycles += 12;
                                c.g[A0][0] = c.g[S0][0];
                                GET_DIST_TO_FINISH.call(c, 0x0010_fa68)?;
                                // key = -(int)(dist + position * 20)
                                c.g[V0][0] = c.lw(race_pos_at(c, S0, RIDER_POSITION));
                                c.mult(0, V0, V0, S6);
                                c.mtc1(V0, 1);
                                c.cvt_s_w(1, 1);
                                c.add_s(0, 0, 1);
                                c.cvt_w_s(1, 0);
                                c.mfc1(V0, 1);
                                c.g[V0][0] = sx(0u32.wrapping_sub(r32(c, V0)));
                                c.sw(r32(c, S2), c.g[V0][0]);
                            }
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 2;
                            if c.g[V1][0] == c.g[V0][0] {
                                // Rank by score.
                                c.g[V0][0] = c.lw(race_pos_at(c, S0, RIDER_FINISHED));
                                c.cycles += 2;
                                if c.g[V0][0] == 0 {
                                    c.g[V0][0] = c.lw(race_pos_at(c, S0, RIDER_SCORE_OBJ));
                                    c.cycles += 2;
                                    c.g[V1][0] = c.lw(race_pos_at(c, V0, SCORE_POINTS));
                                    c.sw(r32(c, S2), c.g[V1][0]);
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = 0xffff_ffff_ffff_0001;
                                    c.sw(r32(c, S2), c.g[V0][0]);
                                }
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = c.lw(race_pos_at(c, S3, AI_COUNT));
                                keyed = false;
                            }
                        }
                        if keyed {
                            c.cycles += 1;
                            c.g[V0][0] = c.lw(race_pos_at(c, S3, AI_COUNT));
                        }
                        // 0x10fab4
                        c.cycles += 7;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                        c.g[S2][0] = sx(r32(c, S2).wrapping_add(4));
                        c.g[S5][0] = sx(r32(c, S5).wrapping_add(4));
                        c.g[A0][0] = c.g[V0][0];
                        let more = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
                        c.g[V0][0] = more as u64;
                        c.g[S4][0] = sx(r32(c, S4).wrapping_add(4));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0010_fa08);
                        }
                    }
                }
                // 0x10fad0: sort the indices by key.
                c.cycles += 6;
                c.g[A1][0] = c.g[S7][0];
                c.g[A2][0] = c.g[SP][0];
                call_ptr(c, SHELL_SORT_PAIRS, 0x0010_fadc)?;
                c.g[V0][0] = c.lw(race_pos_at(c, S3, AI_COUNT));
                c.g[S0][0] = 0;
                if (c.g[V0][0] as i64) <= 0 {
                    break 'done;
                }
                // Pass 2: s0 = new position, s1 = the rider array.
                c.cycles += 2;
                c.g[S1][0] = sx(race_pos_at(c, S3, AI_RIDERS));
                c.g[V1][0] = sx(r32(c, S0) << 2);
                loop {
                    // 0x10faf0
                    c.cycles += 9;
                    c.g[S2][0] = sx(sp(c).wrapping_add(r32(c, V1)));
                    c.g[S7][0] = c.g[V1][0];
                    c.g[V0][0] = c.lw(r32(c, S2));
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    c.g[A0][0] = c.lw(race_pos_at(c, V1, RIDER_POSITION));
                    let unchanged = c.g[A0][0] == c.g[S0][0];
                    c.g[S6][0] = sx(r32(c, S0).wrapping_add(1));
                    if !unchanged {
                        // s4 = the rider ahead, s5 = the rider behind.
                        c.cycles += 3;
                        c.g[S5][0] = 0;
                        c.g[S4][0] = 0;
                        if (c.g[S0][0] as i64) > 0 {
                            c.cycles += 7;
                            c.g[V0][0] = sx(r32(c, S0).wrapping_sub(1));
                            c.g[V0][0] = sx(r32(c, V0) << 2);
                            c.g[A0][0] = sx(sp(c).wrapping_add(r32(c, V0)));
                            c.g[V1][0] = c.lw(r32(c, A0));
                            c.g[V1][0] = sx(r32(c, V1) << 2);
                            c.g[V1][0] = sx(r32(c, S1).wrapping_add(r32(c, V1)));
                            c.g[S4][0] = c.lw(r32(c, V1));
                        }
                        c.cycles += 5;
                        c.g[V0][0] = c.lw(race_pos_at(c, S3, AI_COUNT));
                        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                        let not_last = (c.g[S0][0] as i64) < (c.g[V0][0] as i64);
                        c.g[V0][0] = sx(r32(c, S6) << 2);
                        if not_last {
                            c.cycles += 5;
                            c.g[A0][0] = sx(sp(c).wrapping_add(r32(c, V0)));
                            c.g[V1][0] = c.lw(r32(c, A0));
                            c.g[V1][0] = sx(r32(c, V1) << 2);
                            c.g[V1][0] = sx(r32(c, S1).wrapping_add(r32(c, V1)));
                            c.g[S5][0] = c.lw(r32(c, V1));
                        }
                        c.cycles += 12;
                        call_ptr(c, AUDIO_GET_INSTANCE, 0x0010_fb6c)?;
                        c.g[V1][0] = c.lw(r32(c, S2));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A2][0] = c.g[S5][0];
                        c.g[A3][0] = c.g[S4][0];
                        c.g[V1][0] = sx(r32(c, V1) << 2);
                        c.g[T1][0] = c.g[S0][0];
                        c.g[V1][0] = sx(r32(c, S1).wrapping_add(r32(c, V1)));
                        c.g[A1][0] = c.lw(r32(c, V1));
                        c.g[T0][0] = c.lw(race_pos_at(c, A1, RIDER_POSITION));
                        call_ptr(c, AUDIO_ON_POSITION_CHANGE, 0x0010_fb94)?;
                    }
                    // 0x10fb94: riders[indices[s0]]->position = s0.
                    c.cycles += 11;
                    c.g[V1][0] = sx(sp(c).wrapping_add(r32(c, S7)));
                    c.g[V0][0] = c.lw(r32(c, V1));
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    c.sw(race_pos_at(c, V1, RIDER_POSITION), c.g[S0][0]);
                    c.g[S0][0] = c.g[S6][0];
                    c.g[V0][0] = c.lw(race_pos_at(c, S3, AI_COUNT));
                    let more = (c.g[S0][0] as i64) < (c.g[V0][0] as i64);
                    c.g[V0][0] = more as u64;
                    c.g[V1][0] = sx(r32(c, S0) << 2);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0010_faf0);
                    }
                }
            }
            // 0x10fbc0
            c.cycles += 1;
            frame.restore(c, S0, 0xd0);
        }
        c.cycles += 11;
        for (reg, off) in [
            (S1, 0xc0),
            (S2, 0xb0),
            (S3, 0xa0),
            (S4, 0x90),
            (S5, 0x80),
            (S6, 0x70),
            (S7, 0x60),
            (FP, 0x50),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_SelectAIPath (0x112a50, computer.cpp)

/// Rider fields: position, velocity, the AI controller (vtable at +0),
/// the followed AI path and its search hint, the path follow state.
pub(crate) const RSA_POS: u32 = 0x110;

pub(crate) const RSA_VEL: u32 = 0x1e0;

pub(crate) const RSA_CLOSEST: u32 = 0x490;

pub(crate) const RSA_AHEAD_POINT: u32 = 0x4a0;

pub(crate) const RSA_PATH_DIST_PREV: u32 = 0x4c0;

pub(crate) const RSA_PATH_DIST: u32 = 0x4c4;

pub(crate) const RSA_CLOSEST_DIST: u32 = 0x4c8;

pub(crate) const RSA_AI: u32 = 0x6c0;

pub(crate) const RSA_AI_PATH: u32 = 0xab8;

pub(crate) const RSA_AI_PATH_HINT: u32 = 0xabc;

/// Path hint field: the current segment (-1: search the whole path).
pub(crate) const RSA_HINT_SEGMENT: u32 = 0x14;

/// AI controller vtable slots: "uses path weights" (int) and the weight of
/// a path (float).
pub(crate) const RSA_VT_USES_WEIGHTS: u32 = 0x48;

pub(crate) const RSA_VT_PATH_WEIGHT: u32 = 0xa0;

/// `PathData` holding the AI paths.
pub(crate) const RSA_PATH_DATA: u32 = 0x004d_33a0;

/// `.lit4` gp-0x7c7c: the initial best distance (~1e37).
pub(crate) const RSA_LIT_FAR: u32 = 0x7c7c;

/// Callees (another shard, through the table).
pub(crate) const RSA_FIND_NEAR_AI_PATHS: u32 = 0x0026_afb8;

pub(crate) const RSA_PATH_GET_CLOSEST_POINT: u32 = 0x0026_a428;

pub(crate) const RSA_PATH_GET_LENGTH: u32 = 0x0026_ac48;

pub(crate) const RSA_PATH_GET_POINT_AT_DISTANCE: u32 = 0x0026_ab20;

pub(crate) const RSA_PATH_GET_CLOSEST_POINT_FROM: u32 = 0x0026_a638;

/// Look-ahead along a path (796.0f), the end margin (200.0f), -1.0f.
pub(crate) const RSA_LOOK_AHEAD: u64 = 0x4447_0000;

pub(crate) const RSA_END_MARGIN: u64 = 0x4348_0000;

pub(crate) const RSA_MINUS_ONE: u64 = 0xffff_ffff_bf80_0000;

/// `|v|²` of the quadword at `addr` (x²+y²+z²+w² through ACC) into `gpr`
/// with `qmfc2`, as the original's inlined `vu0Dot`.
pub(crate) fn rsa_vu0_dot_self(c: &mut Cpu, addr: u32, gpr: usize) {
    c.lqc2(3, addr);
    c.lqc2(5, addr);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(gpr, 4);
}

/// `vf5 = [a] - [b]`, stored through the frame's temporary at sp+0x60 and
/// reloaded into r2 (the original's by-value vector return).
pub(crate) fn rsa_vu0_sub_to_v0(c: &mut Cpu, a: u32, b: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x60));
    c.lq(2, sp(c).wrapping_add(0x60));
}

/// `void Rider_SelectAIPath(Rider* self, bool keep_current)` (0x112a50):
/// the AI's path choice. Looks 796 units ahead along the velocity, finds up
/// to six AI paths near that point and scores each one that has at least
/// 200 units left past the rider: by the controller's path weight when it
/// uses weights (highest wins), else by the summed squared distances of the
/// rider to the path and of the look-ahead point to the path point 796
/// further on (lowest wins). With `keep_current`, the current path is not a
/// candidate; a winner equal to the current path counts as none. A new path
/// becomes current: its closest point and distance are recomputed from a
/// fresh search and the point 796 ahead becomes the steering target.
pub fn rider_select_ai_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x120);
        c.mtc1(0, 0);
        frame.save(c, S0, 0xf0);
        frame.save(c, S2, 0xd0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S5, 0xa0);
        c.g[S2][0] = 0;
        frame.save_fpr(c, 22, 0x110);
        c.g[S5][0] = c.g[A1][0];
        frame.save_fpr(c, 21, 0x108);
        frame.save(c, S1, 0xe0);
        frame.save(c, S3, 0xc0);
        frame.save(c, S4, 0xb0);
        frame.save(c, S6, 0x90);
        frame.save_ra(c, 0x80);
        frame.save_fpr(c, 20, 0x100);
        // |vel| (VU0 length).
        let this = r32(c, S0);
        c.lqc2(3, this.wrapping_add(RSA_VEL));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 1);
        c.lwc1(22, r32(c, GP).wrapping_sub(RSA_LIT_FAR)); // best distance
        c.c_eq_s(1, 0);
        c.g[AT][0] = RSA_MINUS_ONE;
        c.mtc1(1, 21); // best weight
        if c.fcc() {
            // Standing still: look from the rider's position.
            c.cycles += 3;
            c.lq(2, this.wrapping_add(RSA_POS));
            c.sq(2, sp(c).wrapping_add(0x30));
        } else {
            // look = pos + vel * (796 / |vel|)
            c.cycles += 17;
            c.g[AT][0] = RSA_LOOK_AHEAD;
            c.mtc1(1, 0);
            c.div_s(0, 0, 1);
            c.mfc1(3, 0);
            c.lqc2(4, this.wrapping_add(RSA_VEL));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp(c).wrapping_add(0x60));
            c.lq(2, sp(c).wrapping_add(0x60));
            c.sq(2, sp(c).wrapping_add(0x50));
            c.lqc2(3, this.wrapping_add(RSA_POS));
            c.lqc2(4, sp(c).wrapping_add(0x50));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp(c).wrapping_add(0x60));
            c.lq(2, sp(c).wrapping_add(0x60));
            c.sq(2, sp(c).wrapping_add(0x30));
            c.sq(2, sp(c).wrapping_add(0x40));
        }
        // n = PathData_FindNearAIPaths(&ai_paths, &look, paths, 6, 0)
        c.cycles += 10;
        c.g[A1][0] = sx(sp(c).wrapping_add(0x30));
        c.g[S6][0] = c.g[A1][0];
        c.g[A0][0] = sx(RSA_PATH_DATA);
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 6;
        c.g[T0][0] = 0;
        call_ptr(c, RSA_FIND_NEAR_AI_PATHS, 0x0011_2b40)?;
        c.g[S3][0] = c.g[V0][0];
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            c.g[S4][0] = sx(r32(c, S0).wrapping_add(RSA_AI));
            c.g[S1][0] = c.g[SP][0];
            loop {
                // 0x112b50: candidate *s1.
                'candidate: {
                    c.cycles += 4;
                    c.g[A0][0] = Addr(r32(c, S1)).lw(c);
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(RSA_AI_PATH)).lw(c);
                    let is_current = c.g[A0][0] == c.g[V0][0];
                    c.g[A1][0] = c.g[S6][0];
                    if is_current {
                        c.cycles += 2;
                        if c.g[S5][0] != 0 {
                            c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                            break 'candidate;
                        }
                        c.cycles += 1;
                        c.g[A1][0] = c.g[S6][0];
                    }
                    // t = Path_GetClosestPoint(path, &look, &out, &t_out, 1)
                    c.cycles += 14;
                    c.g[A2][0] = sx(sp(c).wrapping_add(0x20));
                    c.g[A3][0] = sx(sp(c).wrapping_add(0x70));
                    c.g[T0][0] = 1;
                    call_ptr(c, RSA_PATH_GET_CLOSEST_POINT, 0x0011_2b7c)?;
                    c.g[A0][0] = Addr(r32(c, S1)).lw(c);
                    c.mov_s(20, 0);
                    call_ptr(c, RSA_PATH_GET_LENGTH, 0x0011_2b88)?;
                    c.g[AT][0] = RSA_END_MARGIN;
                    c.mtc1(1, 1);
                    c.sub_s(0, 0, 1);
                    c.c_lt_s(0, 20);
                    if c.fcc() {
                        // Too close to the end of this path.
                        c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                        break 'candidate;
                    }
                    // ahead = Path_GetPointAtDistance(path, 0, t + 796)
                    c.cycles += 46;
                    c.g[AT][0] = RSA_LOOK_AHEAD;
                    c.mtc1(1, 12);
                    c.g[A0][0] = sx(sp(c).wrapping_add(0x50));
                    c.g[A1][0] = Addr(r32(c, S1)).lw(c);
                    c.g[A2][0] = 0;
                    c.add_s(12, 20, 12);
                    call_ptr(c, RSA_PATH_GET_POINT_AT_DISTANCE, 0x0011_2bc0)?;
                    // d1 = look - ahead (sp+0x40), d0 = pos - out (sp+0x50)
                    let base = sp(c);
                    rsa_vu0_sub_to_v0(c, base.wrapping_add(0x30), base.wrapping_add(0x50));
                    c.sq(2, base.wrapping_add(0x40));
                    rsa_vu0_sub_to_v0(c, r32(c, S0).wrapping_add(RSA_POS), base.wrapping_add(0x20));
                    c.g[A1][0] = Addr(r32(c, S0).wrapping_add(RSA_AI)).lw(c);
                    c.sq(2, base.wrapping_add(0x50));
                    rsa_vu0_dot_self(c, base.wrapping_add(0x40), 2);
                    c.mtc1(2, 0);
                    rsa_vu0_dot_self(c, base.wrapping_add(0x50), 3);
                    c.mtc1(3, 1);
                    // ai->UsesPathWeights()
                    let vt = r32(c, A1);
                    c.g[A0][0] = Addr(vt.wrapping_add(RSA_VT_USES_WEIGHTS)).lh(c);
                    c.add_s(20, 1, 0); // dist = |d0|² + |d1|²
                    c.g[V0][0] = Addr(vt.wrapping_add(RSA_VT_USES_WEIGHTS + 4)).lw(c);
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(r32(c, S4).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0011_2c50)?;
                    c.g[V1][0] = 1;
                    if c.g[V0][0] == c.g[V1][0] {
                        // weight = ai->PathWeight(path); highest wins.
                        c.cycles += 11;
                        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(RSA_AI)).lw(c);
                        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
                        let vt = r32(c, V0);
                        c.g[A0][0] = Addr(vt.wrapping_add(RSA_VT_PATH_WEIGHT)).lh(c);
                        c.g[V1][0] = Addr(vt.wrapping_add(RSA_VT_PATH_WEIGHT + 4)).lw(c);
                        let method = c.g[V1][0] as u32;
                        c.g[A0][0] = sx(r32(c, S0).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, method, 0x0011_2c74)?;
                        c.mov_s(20, 0);
                        c.c_lt_s(21, 20);
                        if !c.fcc() {
                            c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                            break 'candidate;
                        }
                        c.cycles += 3;
                        c.g[S2][0] = Addr(r32(c, S1)).lw(c);
                        c.mov_s(21, 20);
                    } else {
                        // Lowest distance wins.
                        c.cycles += 4;
                        c.c_lt_s(20, 22);
                        if !c.fcc() {
                            c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                            break 'candidate;
                        }
                        c.cycles += 2;
                        c.g[S2][0] = Addr(r32(c, S1)).lw(c);
                        c.mov_s(22, 20);
                    }
                    // 0x112cac: the current path is no new choice.
                    c.cycles += 4;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(RSA_AI_PATH)).lw(c);
                    c.g[V0][0] ^= c.g[S2][0];
                    if c.g[V0][0] == 0 {
                        c.g[S2][0] = 0;
                    }
                    c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                }
                // 0x112cbc
                c.cycles += 2;
                let more = c.g[S3][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0011_2b50);
                }
            }
        }
        // 0x112cc4
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
        if c.g[S2][0] != 0 {
            // Follow the new path from a fresh closest-point search.
            c.cycles += 22;
            let this = r32(c, S0);
            c.g[V1][0] = Addr(this.wrapping_add(RSA_AI_PATH_HINT)).lw(c);
            Addr(this.wrapping_add(RSA_AI_PATH)).sw(c, c.g[S2][0]);
            c.g[A1][0] = sx(this.wrapping_add(RSA_POS));
            Addr(r32(c, V1).wrapping_add(RSA_HINT_SEGMENT)).sw(c, c.g[V0][0]);
            c.g[A2][0] = sx(this.wrapping_add(RSA_CLOSEST));
            c.g[A3][0] = sx(this.wrapping_add(RSA_CLOSEST_DIST));
            c.g[T1][0] = 1;
            c.g[A0][0] = Addr(this.wrapping_add(RSA_AI_PATH)).lw(c);
            c.g[T0][0] = Addr(this.wrapping_add(RSA_AI_PATH_HINT)).lw(c);
            call_ptr(c, RSA_PATH_GET_CLOSEST_POINT_FROM, 0x0011_2cf4)?;
            let this = r32(c, S0);
            c.g[AT][0] = RSA_LOOK_AHEAD;
            c.mtc1(1, 12);
            c.g[A0][0] = sx(sp(c).wrapping_add(0x40));
            c.g[A1][0] = Addr(this.wrapping_add(RSA_AI_PATH)).lw(c);
            c.g[A2][0] = 0;
            c.add_s(12, 0, 12);
            c.swc1(0, this.wrapping_add(RSA_PATH_DIST));
            call_ptr(c, RSA_PATH_GET_POINT_AT_DISTANCE, 0x0011_2d14)?;
            let this = r32(c, S0);
            c.lq(2, sp(c).wrapping_add(0x40));
            c.lwc1(0, this.wrapping_add(RSA_PATH_DIST));
            c.sq(2, this.wrapping_add(RSA_AHEAD_POINT));
            c.swc1(0, this.wrapping_add(RSA_PATH_DIST_PREV));
        }
        c.cycles += 13;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 22, 0x110);
        frame.restore_fpr(c, 21, 0x108);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_SelectRacePath (0x1127f0)

/// Rider fields used by the race-path choice.
pub(crate) const SELECT_PATH_POS: u32 = 0x110;

pub(crate) const SELECT_PATH_VEL: u32 = 0x1e0;

pub(crate) const SELECT_PATH_TRACK_DIST: u32 = 0x4d0;

pub(crate) const SELECT_PATH_CURRENT: u32 = 0xab4;

pub(crate) const SELECT_PATH_AI_STATE: u32 = 0xac0;

/// The path data (`D_004D33A0`).
pub(crate) const SELECT_PATH_DATA: u32 = 0x004d_33a0;

pub(crate) const SELECT_PATH_LOOK_AHEAD: Callee = Callee {
    addr: 0x0011_2588,
    f: ee::s00::f_00112588,
};

pub(crate) const SELECT_PATH_FIND_NEAR: u32 = 0x0026_b178;

pub(crate) const SELECT_PATH_CLOSEST_POINT: u32 = 0x0026_a428;

pub(crate) const SELECT_PATH_POINT_AT: u32 = 0x0026_ab20;

/// `void Rider_SelectRacePath(Rider* self, bool skipCurrent)` (0x1127f0):
/// pick the race path for the AI. The probe point is the position, moved
/// ahead along the velocity by the look-ahead distance
/// (`Rider_GetAIPathLookAheadDist(self, 1)`); of up to three nearby track
/// paths (`PathData_FindNearTrackPaths`), optionally skipping the current
/// one, choose the one minimising |pos - closest point|^2 + |probe - point
/// 796 further along|^2 (`Path_GetClosestPoint`, `Path_GetPointAtDistance`).
/// If the current path's distance disagrees with the rider's by more than
/// 1592, the AI state's `+0x14` is reset to -1. The choice goes to `+0xab4`.
pub fn rider_select_race_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x110);
        c.mtc1(0, 0);
        frame.save(c, S1, 0xd0);
        frame.save(c, S4, 0xa0);
        frame.save_fpr(c, 21, 0xf8);
        frame.save(c, S0, 0xe0);
        frame.save(c, S2, 0xc0);
        frame.save(c, S3, 0xb0);
        frame.save(c, S5, 0x90);
        frame.save(c, S6, 0x80);
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 22, 0x100);
        frame.save_fpr(c, 20, 0xf0);
        c.g[S1][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        let me = r32(c, S1);
        // f20 = |velocity.xyz|
        c.lqc2(3, me.wrapping_add(SELECT_PATH_VEL));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(V0, 22);
        c.mtc1(V0, 20);
        c.lwc1(21, r32(c, GP).wrapping_sub(0x7c80)); // best = 1e37
        c.c_eq_s(20, 0);
        let still = c.fcc();
        c.g[S3][0] = Addr(me.wrapping_add(SELECT_PATH_CURRENT)).lw(c); // best path
        let s = sp(c);
        if still {
            c.cycles += 3;
            c.lq(V0, me.wrapping_add(SELECT_PATH_POS));
            c.sq(V0, s + 0x20);
        } else {
            // probe = pos + vel * (lookahead / |vel|)
            c.cycles += 18;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x1;
            SELECT_PATH_LOOK_AHEAD.call(c, 0x0011_2880)?;
            let me = r32(c, S1);
            c.div_s(0, 0, 20);
            c.mfc1(V1, 0);
            c.lqc2(4, me.wrapping_add(SELECT_PATH_VEL));
            c.qmtc2(V1, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, s + 0x50);
            c.lq(V0, s + 0x50);
            c.sq(V0, s + 0x40);
            c.lqc2(3, me.wrapping_add(SELECT_PATH_POS));
            c.lqc2(4, s + 0x40);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s + 0x50);
            c.lq(V0, s + 0x50);
            c.sq(V0, s + 0x20);
            c.sq(V0, s + 0x30);
        }

        // Up to three track paths near the probe (into sp+0).
        c.cycles += 9;
        c.g[A1][0] = sx(s + 0x20);
        c.g[S6][0] = c.g[A1][0];
        c.g[A0][0] = sx(SELECT_PATH_DATA);
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 0x3;
        call_ptr(c, SELECT_PATH_FIND_NEAR, 0x0011_28d8)?;
        c.g[S2][0] = c.g[V0][0];
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 6;
            c.g[AT][0] = 0x44c7_0000;
            c.mtc1(AT, 22); // 1592.0
            c.g[AT][0] = 0x4447_0000;
            c.mtc1(AT, 20); // 796.0
            c.g[S5][0] = u64::MAX;
            c.g[S0][0] = c.g[SP][0];
            loop {
                let slot = r32(c, S0);
                c.cycles += 2;
                let mut skip = false;
                if c.g[S4][0] != 0 {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(slot).lw(c);
                    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SELECT_PATH_CURRENT)).lw(c);
                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
                        skip = true;
                    } else {
                        c.cycles += 1;
                    }
                }
                if !skip {
                    c.g[A0][0] = Addr(slot).lw(c);
                    // Closest point (sp+0x10) and its distance along the path.
                    c.cycles += 9;
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A2][0] = sx(s + 0x10);
                    c.g[A3][0] = sx(s + 0x60);
                    c.g[T0][0] = 0x1;
                    call_ptr(c, SELECT_PATH_CLOSEST_POINT, 0x0011_2928)?;
                    let me = r32(c, S1);
                    c.g[V1][0] = Addr(slot).lw(c);
                    c.g[V0][0] = Addr(me.wrapping_add(SELECT_PATH_CURRENT)).lw(c);
                    let current = c.g[V1][0] == c.g[V0][0];
                    c.mov_s(12, 0);
                    let mut lost = true;
                    if current {
                        c.cycles += 9;
                        c.lwc1(0, r32(c, V1).wrapping_add(0x38));
                        c.lwc1(1, me.wrapping_add(SELECT_PATH_TRACK_DIST));
                        c.sub_s(0, 0, 1);
                        c.sub_s(0, 0, 12);
                        c.abs_s(0, 0);
                        c.c_lt_s(22, 0);
                        if c.fcc() {
                            c.cycles += 2;
                            c.g[V0][0] = Addr(me.wrapping_add(SELECT_PATH_AI_STATE)).lw(c);
                            Addr(r32(c, V0).wrapping_add(0x14)).sw(c, c.g[S5][0]);
                        } else {
                            lost = false;
                        }
                    }
                    if lost {
                        c.cycles += 1;
                    }
                    c.add_s(12, 12, 20); // t + 796

                    // dist = |pos - closest|^2 + |probe - ahead|^2
                    c.cycles += 39;
                    c.g[A1][0] = Addr(slot).lw(c);
                    c.g[A0][0] = sx(s + 0x40);
                    c.g[A2][0] = 0;
                    call_ptr(c, SELECT_PATH_POINT_AT, 0x0011_2978)?;
                    let me = r32(c, S1);
                    c.lqc2(3, s + 0x20);
                    c.lqc2(4, s + 0x40);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, s + 0x50);
                    c.lq(V0, s + 0x50);
                    c.sq(V0, s + 0x30);
                    c.lqc2(3, me.wrapping_add(SELECT_PATH_POS));
                    c.lqc2(4, s + 0x10);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, s + 0x50);
                    c.lq(V0, s + 0x50);
                    c.sq(V0, s + 0x40);
                    c.lqc2(3, s + 0x40);
                    c.lqc2(5, s + 0x40);
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(V1, 4);
                    c.mtc1(V1, 1);
                    c.lqc2(3, s + 0x30);
                    c.lqc2(5, s + 0x30);
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(V0, 4);
                    c.mtc1(V0, 0);
                    c.add_s(1, 1, 0);
                    c.c_lt_s(1, 21);
                    let better = c.fcc();
                    c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
                    if better {
                        c.cycles += 2;
                        c.g[S3][0] = Addr(slot).lw(c);
                        c.mov_s(21, 1);
                    }
                }
                c.cycles += 2;
                let more = c.g[S2][0] != 0;
                c.g[S0][0] = sx(slot.wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0011_28f8);
                }
            }
        }

        c.cycles += 14;
        Addr(r32(c, S1).wrapping_add(SELECT_PATH_CURRENT)).sw(c, c.g[S3][0]);
        frame.restore(c, S0, 0xe0);
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 22, 0x100);
        frame.restore_fpr(c, 21, 0xf8);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderBase_TriggerDynamicSpray (0x111aa0)
// ---------------------------------------------------------------------------

/// RiderBase: its dynamic spray effect (`cDynamicSprayFX`).
pub(crate) const RIDER_DYNAMIC_SPRAY: u32 = 0xb40;

/// `void RiderBase_TriggerDynamicSpray(RiderBase* self, Vec* pos, Vec* dir,
/// int kind, float speed)` (0x111aa0): queues a spray burst on the rider's
/// dynamic spray effect (`cDynamicSprayFX_QueueBurst(&self->spray, pos, dir,
/// kind, 0, speed)`).
pub fn rider_base_trigger_dynamic_spray(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(RIDER_DYNAMIC_SPRAY));
        frame.save_ra(c, 0);
        c.g[T0][0] = 0;
        call_ptr(c, 0x002e_23e0, 0x0011_1ab4)?; // cDynamicSprayFX_QueueBurst
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderBase_EnterSubState

/// `RiderBase.subState`: the current sub-state id (0..13).
pub(crate) const RIDER_BASE_SUB_STATE: u32 = 0xde4;

/// `.rodata` jump table of the sub-state switch (14 case labels).
pub(crate) const ENTER_SUB_STATE_TABLE: u32 = 0x0045_6bd0;

/// The sub-state `Enter` methods: (case label, sub-object offset in the
/// rider base, callee, return address). `None` callee: a `jalr` through the
/// function table (cross-shard call).
pub(crate) const ENTER_SUB_STATE_ARMS: [(u32, u32, u32, Option<GuestFn>, u32); 12] = [
    (
        0x0011_1664,
        0x1c0,
        0x0013_1608,
        Some(ee::s00::RiderCruiseSub_Enter),
        0x0011_166c,
    ),
    (
        0x0011_1674,
        0x1d0,
        0x0012_fc60,
        Some(ee::s00::RiderBoardPressSub_Enter),
        0x0011_167c,
    ),
    (
        0x0011_1684,
        0x1f0,
        0x0012_e980,
        Some(ee::s00::RiderPrewindSub_Enter),
        0x0011_168c,
    ),
    (
        0x0011_1694,
        0x210,
        0x0012_f620,
        Some(ee::s00::RiderNaturalAirSub_Enter),
        0x0011_169c,
    ),
    (0x0011_16a4, 0x230, 0x0013_3128, None, 0x0011_16ac), // RiderSpinSub_Enter
    (
        0x0011_16b4,
        0x290,
        0x0012_be20,
        Some(ee::s00::RiderGateSub_Enter),
        0x0011_16bc,
    ),
    (
        0x0011_16c4,
        0x2b0,
        0x0013_1d08,
        Some(ee::s00::RiderRailslideSub_Enter),
        0x0011_16cc,
    ),
    (
        0x0011_16d4,
        0x2c0,
        0x0012_ca30,
        Some(ee::s00::RiderWipeoutSub_Enter),
        0x0011_16dc,
    ),
    (
        0x0011_16e4,
        0x350,
        0x0012_f230,
        Some(ee::s00::RiderResetSub_Enter),
        0x0011_16ec,
    ),
    (
        0x0011_16f4,
        0x360,
        0x0012_c5c8,
        Some(ee::s00::RiderFinishLineSub_Enter),
        0x0011_16fc,
    ),
    (0x0011_1704, 0x370, 0x0013_28b0, None, 0x0011_170c), // RiderHandplantSub_Enter
    (0x0011_1714, 0x390, 0x0013_6268, None, 0x0011_171c), // RiderUberGrindSub_Enter
];

/// `void RiderBase_EnterSubState(RiderBase* r)` (0x111630): call the `Enter`
/// method of the rider's current sub-state (cruise, board press, prewind,
/// natural air, spin, gate, railslide, wipeout, reset, finish line,
/// handplant, uber grind; 3 and 13 have none) on its sub-object. Accepts
/// every `__entries` resume label.
pub fn rider_base_enter_sub_state(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0011_1630 => {
                    c.cycles += 7;
                    let frame = Frame::open(c, 0x10);
                    c.g[A2][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[V1][0] = Addr(c.g[A2][0] as u32).off(RIDER_BASE_SUB_STATE).lw(c);
                    let in_range = c.g[V1][0] < 0xe;
                    c.g[V0][0] = 0x45_0000;
                    pc = if in_range { 0x0011_164c } else { 0x0011_171c };
                }
                0x0011_164c => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] =
                        sx((c.g[V0][0] as u32).wrapping_add(ENTER_SUB_STATE_TABLE & 0xffff));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c);
                    c.g[A0][0] = target;
                    let target = target as u32;
                    let known =
                        target == 0x0011_171c || ENTER_SUB_STATE_ARMS.iter().any(|a| a.0 == target);
                    if !known {
                        c.jump_out(target, 0x0011_1630, 0x0011_1728);
                        return Ok(());
                    }
                    pc = target;
                }
                0x0011_171c => {
                    c.cycles += 1;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1720;
                }
                0x0011_1720 => {
                    c.cycles += 2;
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    let Some(&(_, off, addr, f, ret)) =
                        ENTER_SUB_STATE_ARMS.iter().find(|a| a.0 == pc)
                    else {
                        c.jump_out(pc, 0x0011_1630, 0x0011_1728);
                        return Ok(());
                    };
                    // The last arm (uber grind) falls through to 0x11171c.
                    let last = pc == 0x0011_1714;
                    c.cycles += if last { 2 } else { 4 };
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(off));
                    match f {
                        Some(f) => Callee { addr, f }.call(c, ret)?,
                        None => call_ptr(c, addr, ret)?,
                    }
                    if last {
                        pc = 0x0011_171c;
                    } else {
                        let sp = sp(c);
                        c.g[RA][0] = c.ld(sp);
                        pc = 0x0011_1720;
                    }
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// ComputerActorRider_GetPathClassWeight (0x10d870, ai/computer.cpp)

/// ComputerActorRider: the rider's preferred path class (0, 1, 2 or other).
pub(crate) const CAR_PATH_CLASS: u32 = 0xe00;

/// `gp - 0x7ce8`: the weight per unmatched path flag (33.3f).
pub(crate) const CAR_PATH_WEIGHT_STEP_GP: u32 = 0x7ce8;

/// `float ComputerActorRider_GetPathClassWeight(ComputerActorRider* self,
/// int flagA, int flagB, int flagC)` (0x10d870): how attractive a path with
/// the three class flags is to the AI rider. A path is rejected (0.0) when it
/// lacks the flag of the rider's preferred class (class 0: A, 1: B, 2: C;
/// any other class: `(A == B) != C`); an accepted path weighs
/// `(4 - number of set flags) * 33.3`.
pub fn computer_actor_rider_get_path_class_weight_2(c: &mut Cpu) {
    c.cycles += 3;
    let class = Addr(r32(c, A0).wrapping_add(CAR_PATH_CLASS)).lh(c);
    c.g[V1][0] = class;
    c.g[V0][0] = 1;
    let (flag_a, flag_b, flag_c) = (c.g[A1][0], c.g[A2][0], c.g[A3][0]);
    c.cycles += 2;
    let accepted = if class == 0 {
        c.g[V1][0] = 3;
        flag_a != 0
    } else if class == 1 {
        c.cycles += 2;
        c.g[V1][0] = 3;
        if flag_b == 0 {
            c.cycles += 2;
        }
        flag_b != 0
    } else {
        c.g[V0][0] = 2;
        c.cycles += 2;
        if class == 2 {
            c.cycles += 2;
            c.g[V1][0] = 3;
            if flag_c == 0 {
                c.cycles += 2;
            }
            flag_c != 0
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((flag_a ^ flag_b) < 1) as u64;
            c.g[V1][0] = 3;
            c.g[V0][0] == flag_c
        }
    };
    if accepted {
        c.cycles += 11;
        // movn chain: 4, minus one per set flag (v1 holds 3 here).
        let mut n = if flag_a != 0 { c.g[V1][0] } else { 4 };
        let gp = r32(c, GP);
        let step = Addr(gp.wrapping_sub(CAR_PATH_WEIGHT_STEP_GP)).read_fpu(c);
        abi::set_fpr(c, 1, step);
        c.g[V1][0] = sx((n as u32).wrapping_sub(1));
        if flag_b != 0 {
            n = c.g[V1][0];
        }
        c.g[A0][0] = sx((n as u32).wrapping_sub(1));
        if flag_c != 0 {
            n = c.g[A0][0];
        }
        c.g[V0][0] = n;
        abi::set_fret(c, FpuF32::from_i32(n as u32 as i32) * step);
    } else {
        c.cycles += 3;
        c.f[0] = 0;
    }
    ret(c);
}

// ---------------------------------------------------------------------------
// RiderBase_ExitSubState.

/// `Rider::mSubState`.
pub(crate) const RIDER_SUB_STATE: u32 = 0xde4;

/// The `switch (mSubState)` jump table (`.rodata`, 14 entries).
pub(crate) const RIDER_EXIT_SUB_STATE_TABLE: u32 = 0x0045_6b90;

/// How a sub state's exit handler is called.
#[derive(Clone, Copy)]
pub(crate) enum RiderExitSubCall {
    Direct(Callee),
    Table(u32),
}

/// One `case` of the exit switch: its block address (the table target), the
/// sub-object offset in the rider, the handler and its return address.
pub(crate) struct RiderExitSubCase {
    block: u32,
    off: u32,
    handler: RiderExitSubCall,
    ret: u32,
}

/// The sub states with an exit handler (cruise, board press, prewind,
/// natural air, 5, 7, wipeout, 11), in block order.
pub(crate) const RIDER_EXIT_SUB_CASES: [RiderExitSubCase; 8] = [
    RiderExitSubCase {
        block: 0x0011_15ac,
        off: 0x1c0,
        handler: RiderExitSubCall::Direct(Callee {
            addr: 0x0013_1c30,
            f: ee::s00::f_00131c30,
        }),
        ret: 0x0011_15b4,
    },
    RiderExitSubCase {
        block: 0x0011_15bc,
        off: 0x1d0,
        handler: RiderExitSubCall::Direct(Callee {
            addr: 0x0012_fe98,
            f: ee::s00::f_0012fe98,
        }),
        ret: 0x0011_15c4,
    },
    RiderExitSubCase {
        block: 0x0011_15cc,
        off: 0x1f0,
        handler: RiderExitSubCall::Direct(Callee {
            addr: 0x0012_e9b0,
            f: ee::s00::f_0012e9b0,
        }),
        ret: 0x0011_15d4,
    },
    RiderExitSubCase {
        block: 0x0011_15dc,
        off: 0x210,
        handler: RiderExitSubCall::Direct(Callee {
            addr: 0x0012_fb68,
            f: ee::s00::f_0012fb68,
        }),
        ret: 0x0011_15e4,
    },
    RiderExitSubCase {
        block: 0x0011_15ec,
        off: 0x230,
        handler: RiderExitSubCall::Table(0x0013_4cb0),
        ret: 0x0011_15f4,
    },
    RiderExitSubCase {
        block: 0x0011_15fc,
        off: 0x2b0,
        handler: RiderExitSubCall::Table(0x0013_2048),
        ret: 0x0011_1604,
    },
    RiderExitSubCase {
        block: 0x0011_160c,
        off: 0x2c0,
        handler: RiderExitSubCall::Direct(Callee {
            addr: 0x0012_e690,
            f: ee::s00::f_0012e690,
        }),
        ret: 0x0011_1614,
    },
    RiderExitSubCase {
        block: 0x0011_161c,
        off: 0x370,
        handler: RiderExitSubCall::Table(0x0013_2f98),
        ret: 0x0011_1624,
    },
];

/// `void RiderBase_ExitSubState(Rider* self)` (0x111578): run the exit
/// handler of the rider's current sub state (`mSubState`, 0xde4) on its
/// sub-object; sub states without one (3, 6, 9, 10, 12, 13, out of range)
/// do nothing. The switch goes through the `.rodata` jump table.
pub fn rider_base_exit_sub_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(RIDER_SUB_STATE)).lw(c);
        let in_table = c.g[V1][0] < 14;
        c.g[V0][0] = 0x0045_0000;
        let mut tail = !in_table;
        if in_table {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx(RIDER_EXIT_SUB_STATE_TABLE);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            if target == 0x0011_1624 {
                // No handler.
                tail = true;
            } else if let Some(case) = RIDER_EXIT_SUB_CASES.iter().find(|k| k.block == target) {
                let last = case.block == 0x0011_161c;
                c.cycles += if last { 2 } else { 4 };
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(case.off));
                match case.handler {
                    RiderExitSubCall::Direct(f) => f.call(c, case.ret)?,
                    RiderExitSubCall::Table(addr) => call_ptr(c, addr, case.ret)?,
                }
                if last {
                    tail = true;
                } else {
                    frame.restore_ra(c, 0);
                }
            } else {
                c.jump_out(target, 0x0011_1578, 0x0011_1630);
                return Ok(());
            }
        }
        if tail {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ComputerActorRider_PickUberTrick (0x10cf68)

/// ComputerActorRider: the rider, the uber-trick plan (picked flag, the
/// gap before the trick's end pose, the time asked for, the spare time),
/// and per-trick use counts (15 ints).
pub(crate) const UBER_PICK_RIDER: u32 = 0x18;

pub(crate) const UBER_PICK_PICKED: u32 = 0xe24;

pub(crate) const UBER_PICK_PLANNED: u32 = 0xe28;

pub(crate) const UBER_PICK_TAIL: u32 = 0xe2c;

pub(crate) const UBER_PICK_TIME: u32 = 0xe30;

pub(crate) const UBER_PICK_SPARE: u32 = 0xe34;

pub(crate) const UBER_PICK_USES: u32 = 0xeb8;

/// Rider: air time left (float), trick level (>= 5: the second animation
/// set), the trick interface object (vtable slot +0x38: rider index), the
/// animation controller.
pub(crate) const UBER_PICK_AIR_TIME: u32 = 0x2f0;

pub(crate) const UBER_PICK_LEVEL: u32 = 0x2f4;

pub(crate) const UBER_PICK_TRICK_IFACE: u32 = 0x6c0;

pub(crate) const UBER_PICK_IFACE_INDEX_SLOT: u32 = 0x38;

pub(crate) const UBER_PICK_UBER_COUNT: i64 = 15;

/// `BEBAGTInt_GetTrickAnim` result for a trick the rider has no slot for.
pub(crate) const UBER_PICK_NO_ANIM: u64 = 0x1b6;

pub(crate) const UBER_PICK_SKILL_FACTOR: Callee = Callee {
    addr: 0x0012_0038,
    f: ee::s00::Rider_GetTrickSkillFactor,
};

/// `rider->trickIface->GetIndex()` (vtable slot +0x38) with a0 = the rider
/// (as the original leaves it), returning to `ret`.
pub(crate) fn uber_pick_rider_index(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    let iface = r32(c, A0).wrapping_add(UBER_PICK_TRICK_IFACE);
    c.g[V1][0] = Addr(iface).lw(c);
    c.g[A0][0] = sx(iface);
    let vt = c.g[V1][0];
    let slot = VSlot::read(c, vt, UBER_PICK_IFACE_INDEX_SLOT);
    c.g[A1][0] = slot.delta;
    c.g[V0][0] = slot.func;
    c.g[A0][0] = slot.this(c.g[A0][0]);
    call_ptr(c, slot.func as u32, ret)
}

/// `AnimController_GetCurrentAnimTriggerTime(rider->anim, a1, part)` into
/// f0 (`rider_reg` holds the rider; a1 is set by the caller).
pub(crate) fn uber_pick_trigger_time(
    c: &mut Cpu,
    rider_reg: usize,
    part: u64,
    ret: u32,
) -> crate::call::Flow {
    c.g[A2][0] = part;
    c.g[A0][0] = Addr(r32(c, rider_reg).wrapping_add(SPIN_ENTER_ANIM_CTRL)).lw(c);
    call_ptr(c, 0x0031_2820, ret)
}

/// `int ComputerActorRider::PickUberTrick(ComputerActorRider* this, float
/// time)` (0x10cf68): pick the uber trick a CPU rider can finish in `time`
/// seconds of air. -1 if the rider has no air time left (+0x2f0 <= 0).
/// Otherwise, over the 15 uber tricks the rider has an animation for, take
/// the least-used one whose start + end animation times (scaled by the
/// rider's trick skill) fit in `time`; record its tail and spare time,
/// count its use and return its index (-1 if none fits). The loop state
/// lives in s1 (animation), s2 (trick), s4 (best), s5 (best use count),
/// f22 (skill), f23 (time), as in the original.
pub fn computer_actor_rider_pick_uber_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x90);
        c.mtc1(0, 0);
        frame.save(c, S0, 0x60);
        frame.save_fpr(c, 23, 0x88);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x80);
        frame.save_fpr(c, 21, 0x78);
        frame.save_fpr(c, 20, 0x70);
        let this = r32(c, S0);
        Addr(this.wrapping_add(UBER_PICK_PLANNED)).sw(c, 0);
        c.g[V0][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
        c.lwc1(1, r32(c, V0).wrapping_add(UBER_PICK_AIR_TIME));
        c.c_le_s(1, 0);
        let grounded = c.fcc();
        c.mov_s(23, 12);
        if grounded {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            return uber_pick_epilogue(c, frame);
        }

        c.cycles += 13;
        c.swc1(23, this.wrapping_add(UBER_PICK_TIME));
        c.g[S4][0] = u64::MAX;
        c.g[S5][0] = 10000;
        c.g[S2][0] = 0;
        call_ptr(c, 0x0014_dc80, 0x0010_cfd8)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 6;
        call_ptr(c, 0x0014_dd58, 0x0010_cfe4)?; // BELibrary_GetInterface(be, 6): BAGT
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
        c.g[S3][0] = c.g[V0][0];
        UBER_PICK_SKILL_FACTOR.call(c, 0x0010_cff0)?;
        c.mov_s(22, 0);
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
        loop {
            c.cycles += 18;
            uber_pick_rider_index(c, 0x0010_d010)?;
            let this = r32(c, S0);
            c.g[V1][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S2][0];
            let level = Addr(r32(c, V1).wrapping_add(UBER_PICK_LEVEL)).lw(c);
            c.g[A3][0] = (((level as i64) < 5) as u64) ^ 1;
            call_ptr(c, 0x0015_0198, 0x0010_d030)?; // BEBAGTInt_GetTrickAnim
            c.g[S1][0] = c.g[V0][0];
            c.g[V0][0] = UBER_PICK_NO_ANIM;
            if c.g[S1][0] != UBER_PICK_NO_ANIM {
                c.cycles += 34;
                let this = r32(c, S0);
                c.g[A0][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
                uber_pick_rider_index(c, 0x0010_d05c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x0015_00d8, 0x0010_d06c)?; // BEBAGTInt_GetTrickField0
                let this = r32(c, S0);
                c.g[V1][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                uber_pick_trigger_time(c, V1, 1, 0x0010_d080)?;
                let this = r32(c, S0);
                c.g[V0][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
                c.div_s(20, 0, 22); // start of the lead-in
                c.g[A1][0] = c.g[S1][0];
                uber_pick_trigger_time(c, V0, 1, 0x0010_d098)?;
                let this = r32(c, S0);
                c.g[V0][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
                c.div_s(21, 0, 22); // start of the trick
                c.g[A1][0] = c.g[S1][0];
                uber_pick_trigger_time(c, V0, 2, 0x0010_d0b0)?;
                c.div_s(0, 0, 22); // end of the trick
                c.add_s(1, 0, 20);
                c.c_lt_s(1, 23);
                let fits = c.fcc();
                c.g[A0][0] = sx(r32(c, S2) << 2);
                if fits {
                    c.cycles += 6;
                    let this = r32(c, S0);
                    let uses = this.wrapping_add(UBER_PICK_USES).wrapping_add(r32(c, A0));
                    c.g[V1][0] = sx(uses);
                    let n = Addr(uses).lw(c);
                    let fewer = (n as i64) < (c.g[S5][0] as i64);
                    c.g[V0][0] = 1; // (the branch delay slot's li)
                    if fewer {
                        c.cycles += 7;
                        c.sub_s(0, 0, 21);
                        c.sub_s(1, 23, 1);
                        Addr(this.wrapping_add(UBER_PICK_PICKED)).sw(c, 1);
                        c.g[S4][0] = c.g[S2][0];
                        c.swc1(0, this.wrapping_add(UBER_PICK_TAIL));
                        c.swc1(1, this.wrapping_add(UBER_PICK_SPARE));
                        c.g[S5][0] = Addr(uses).lw(c);
                    }
                }
                c.cycles += 1;
            }
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < UBER_PICK_UBER_COUNT) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            let this = r32(c, S0);
            c.g[A0][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0010_cff8);
            }
        }

        // Count the pick.
        c.cycles += 3;
        let best = c.g[S4][0];
        c.g[V0][0] = ((best as i64) < UBER_PICK_UBER_COUNT) as u64;
        if best != u64::MAX {
            c.cycles += 2;
            let in_range = c.g[V0][0] != 0;
            let this = r32(c, S0);
            c.g[V0][0] = sx(this.wrapping_add(UBER_PICK_USES));
            if in_range {
                c.cycles += 5;
                c.g[A0][0] = sx((best as u32) << 2);
                let uses = r32(c, V0).wrapping_add(r32(c, A0));
                c.g[V0][0] = sx(uses);
                c.g[V1][0] = sx((Addr(uses).lw(c) as u32).wrapping_add(1));
                let n = c.g[V1][0];
                Addr(uses).sw(c, n);
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S4][0];
        uber_pick_epilogue(c, frame)
    })
}

/// PickUberTrick's epilogue (13 cycles).
pub(crate) fn uber_pick_epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 13;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 23, 0x88);
    frame.restore_fpr(c, 22, 0x80);
    frame.restore_fpr(c, 21, 0x78);
    frame.restore_fpr(c, 20, 0x70);
    frame.close(c);
    Ok(())
}

/// `RiderBase_SaveState` (0x111ac0).
pub(crate) const RIDER_BASE_SAVE_STATE: Callee = Callee {
    addr: 0x0011_1ac0,
    f: ee::s00::RiderBase_SaveState,
};

/// `ComputerActorRider` field: the AI-specific state block saved after the
/// rider base (0x150 bytes).
pub(crate) const CAR_AI_STATE: u32 = 0xdf0;

pub(crate) const CAR_AI_STATE_SIZE: u64 = 0x150;

/// `void ComputerActorRider_SaveState(ComputerActorRider* self, Stream* s)`
/// (0x10a898): saves the rider base state, then writes the AI rider's own
/// 0x150-byte block at +0xdf0 through the stream's write method.
pub fn computer_actor_rider_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        RIDER_BASE_SAVE_STATE.call(c, 0x0010_a8b4)?;
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(CAR_AI_STATE));
        c.g[A2][0] = CAR_AI_STATE_SIZE;
        // stream->Write(self + 0xdf0, 0x150): vtable slot +0x8 `{delta, fn}`.
        let stream = r32(c, S0);
        c.g[V1][0] = Addr(stream).lw(c);
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt).off(0x8).lh(c);
        c.g[V0][0] = Addr(vt).off(0xc).lw(c);
        let write = r32(c, V0);
        c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
        call_ptr(c, write, 0x0010_a8d0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ComputerActorRider` field: its rider; the rider's velocity (vec4) at +0x1e0.
pub(crate) const CAR_RIDER: u32 = 0x18;

pub(crate) const RIDER_VELOCITY: u32 = 0x1e0;

/// `.lit4` (gp-relative): the speed below which the AI crouches fully
/// (833.33) and the crouch scale above it (0.97).
pub(crate) const CROUCH_FULL_BELOW: u32 = 0x7ddc;

pub(crate) const CROUCH_FAST_SCALE: u32 = 0x7dd8;

/// The AI pad word: the low 26 bits are kept, the crouch amount (0..31)
/// goes into bits 26..31, and bits 16/17 (crouch and boost) are set.
pub(crate) const PAD_KEEP_MASK: u64 = 0x03ff_ffff;

pub(crate) const PAD_CROUCH: u64 = 0x1_0000;

pub(crate) const PAD_BOOST: u64 = 0x2_0000;

/// `void ComputerActorRider_CrouchAndBoost(ComputerActorRider* self, Pad* pad)`
/// (0x10c140): presses crouch and boost on the AI's pad, with a crouch
/// amount of 31 (scaled by 0.97 when the rider's speed is at least 833.33).
/// The speed is the VU0 length of the rider's velocity (Q, as the original).
pub fn computer_actor_rider_crouch_and_boost(c: &mut Cpu) {
    c.cycles += 17;
    c.g[V0][0] = Addr(r32(c, A0)).off(CAR_RIDER).lw(c);
    c.g[A2][0] = c.g[A1][0];
    let gp = r32(c, GP);
    let full_below = Addr(gp.wrapping_sub(CROUCH_FULL_BELOW)).read_fpu(c);
    crate::abi::set_fpr(c, 1, full_below);
    c.lqc2(3, r32(c, V0).wrapping_add(RIDER_VELOCITY));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(3, 22);
    let speed = FpuF32(c.g[V1][0] as u32);
    crate::abi::set_fpr(c, 0, speed);
    let mut scale = Addr(gp.wrapping_sub(CROUCH_FAST_SCALE)).read_fpu(c);
    crate::abi::set_fpr(c, 2, scale);
    if crate::abi::c_lt_s(c, speed, full_below) {
        c.cycles += 2;
        c.g[AT][0] = 0x3f80_0000;
        scale = FpuF32::ONE;
        crate::abi::set_fpr(c, 2, scale);
    }
    c.cycles += 17;
    c.g[AT][0] = 0x41f8_0000;
    let amount = scale * FpuF32(0x41f8_0000); // * 31.0
    crate::abi::set_fpr(c, 0, amount);
    let pad = Addr(r32(c, A2));
    c.g[V1][0] = pad.lw(c) & PAD_KEEP_MASK;
    c.g[A1][0] = PAD_CROUCH;
    c.g[A0][0] = PAD_BOOST;
    let amount_i = FpuF32(amount.to_i32() as u32);
    crate::abi::set_fpr(c, 1, amount_i);
    c.g[V0][0] = sx((amount_i.0) << 26);
    c.g[V1][0] |= c.g[V0][0] | PAD_CROUCH | PAD_BOOST;
    pad.sw(c, c.g[V1][0]);
    ret(c);
}

/// `ComputerActorRider` trick-pick fields.
pub(crate) const CAR_SKIP_PICK: u32 = 0xe20;

/// `int ComputerActorRider_ChooseTrick(ComputerActorRider* ai, float height)`
/// (0x10cad8): the trick to start at `height` above the ground, or -1. No
/// pick when the AI is told to skip (+0xe20) and is below 1 unit, at or
/// below 0.75 units, or at exactly 100 units; otherwise the uber, the
/// fitted and the random trick are rolled in turn, and the first roll that
/// succeeds picks (its picker may still answer -1, which moves on to the
/// next roll). Frame 0x50: ra at 0, s2/s1/s0 at 0x10/0x20/0x30, f20 at 0x40.
pub fn computer_actor_rider_choose_trick(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    const NONE: u64 = u64::MAX;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x40);
        c.g[S1][0] = NONE;
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let ai = Addr(c.g[S0][0] as u32);
        c.g[V0][0] = ai.off(CAR_SKIP_PICK).lw(c);
        c.f[20] = c.f[12];
        let height = FpuF32(c.f[20]);
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.f[0] = 0x3f80_0000;
            let low = height.lt(FpuF32(c.f[0]));
            crate::abi::set_fcc(c, low);
            if low {
                c.g[V1][0] = 1;
            }
        }

        c.cycles += 2;
        c.g[V0][0] = c.g[S1][0];
        let mut picked = true;
        if c.g[V1][0] == 0 {
            c.cycles += 6;
            c.g[AT][0] = 0x3f40_0000;
            c.f[0] = 0x3f40_0000;
            let high_enough = FpuF32(c.f[0]).lt(height);
            crate::abi::set_fcc(c, high_enough);
            if !high_enough {
                picked = false;
            } else {
                c.cycles += 6;
                c.g[AT][0] = 0x42c8_0000;
                c.f[0] = 0x42c8_0000;
                let at_100 = height.eq_s(FpuF32(c.f[0]));
                crate::abi::set_fcc(c, at_100);
                if at_100 {
                    picked = false;
                } else {
                    choose_trick_rolls(c)?;
                }
            }
        }
        if picked {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 6;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// The three rolls of [`computer_actor_rider_choose_trick`]: each
/// `Roll*Trick(ai)` that succeeds calls its `Pick*Trick(ai, height)`; the
/// first pick other than -1 wins. Leaves the pick in s1 and v0.
pub(crate) fn choose_trick_rolls(c: &mut Cpu) -> crate::call::Flow {
    const NONE: u64 = u64::MAX;
    let stages: [(Callee, u32, Callee, u32, u64); 3] = [
        (
            Callee {
                addr: 0x0010_c258,
                f: ee::s00::ComputerActorRider_RollUberTrick,
            },
            0x0010_cb60,
            Callee {
                addr: 0x0010_cf68,
                f: ee::s00::ComputerActorRider_PickUberTrick,
            },
            0x0010_cb70,
            3,
        ),
        (
            Callee {
                addr: 0x0010_c320,
                f: ee::s00::ComputerActorRider_RollFittedTrick,
            },
            0x0010_cb88,
            Callee {
                addr: 0x0010_cd20,
                f: ee::s00::ComputerActorRider_PickFittedTrick,
            },
            0x0010_cb98,
            2,
        ),
        (
            Callee {
                addr: 0x0010_c3b8,
                f: ee::s00::ComputerActorRider_RollRandomTrick,
            },
            0x0010_cbac,
            Callee {
                addr: 0x0010_cbe0,
                f: ee::s00::ComputerActorRider_PickRandomTrick,
            },
            0x0010_cbbc,
            1,
        ),
    ];
    for (n, (roll, roll_ret, pick, pick_ret, tail)) in stages.into_iter().enumerate() {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        roll.call(c, roll_ret)?;
        let rolled = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S0][0];
        if rolled {
            c.cycles += 3;
            c.f[12] = c.f[20];
            pick.call(c, pick_ret)?;
            c.g[S1][0] = c.g[V0][0];
        }
        c.cycles += tail;
        if n == 0 {
            c.g[S2][0] = NONE;
        }
        c.g[V0][0] = c.g[S1][0];
        if n < 2 && c.g[S1][0] != c.g[S2][0] {
            return Ok(());
        }
    }
    Ok(())
}

/// `ComputerActorRider` fields the trick pickers write: uber / fitted
/// flags, the pick's slack before and after, the height it was picked at,
/// and the per-trick fitted pick counts (15 ints).
pub(crate) const CAR_PICK_UBER: u32 = 0xe24;

pub(crate) const CAR_PICK_FITTED: u32 = 0xe28;

pub(crate) const CAR_PICK_LEAD: u32 = 0xe2c;

pub(crate) const CAR_PICK_HEIGHT: u32 = 0xe30;

pub(crate) const CAR_PICK_SLACK: u32 = 0xe34;

pub(crate) const CAR_FITTED_COUNTS: u32 = 0xef4;

/// Rider fields: air height, uber level, animation controller, flags.
pub(crate) const RIDER_AIR_HEIGHT: u32 = 0x2f0;

pub(crate) const RIDER_UBER_LEVEL: u32 = 0x2f4;

pub(crate) const RIDER_VTABLE: u32 = 0x6c0;

pub(crate) const RIDER_ANIM: u32 = 0x784;

pub(crate) const RIDER_FLAGS: u32 = 0xb2c;

/// "No trick / no animation".
pub(crate) const TRICK_NONE: u64 = 0x1b6;

/// The fitted tricks are 1..14 of the BAGT table.
pub(crate) const FITTED_TRICKS: i64 = 15;

/// The rider's virtual "index" query (slot `{delta, fn}` at +0x38 of the
/// vtable at `rider+0x6c0`), as the pickers make it: `a1` holds the rider
/// (`a0` in the uber-animation call), `v1` its vtable; returns to `ret`.
pub(crate) fn pick_fitted_rider_index(
    c: &mut Cpu,
    base: usize,
    delta_reg: usize,
    ret: u32,
) -> crate::call::Flow {
    let vt = Addr(c.g[V1][0] as u32);
    c.g[base][0] = sx((c.g[base][0] as u32).wrapping_add(RIDER_VTABLE));
    c.g[delta_reg][0] = vt.off(0x38).lh(c);
    c.g[V0][0] = vt.off(0x3c).lw(c);
    let f = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[base][0] as u32).wrapping_add(c.g[delta_reg][0] as u32));
    call_ptr(c, f, ret)
}

/// The AI's rider (`*(s0 + 0x18)`, the AI read from s0).
pub(crate) fn pick_fitted_ai_rider(c: &mut Cpu) -> u64 {
    Addr(c.g[S0][0] as u32).off(CAR_RIDER).lw(c)
}

/// `AnimController_GetCurrentAnimTriggerTime(rider->anim, anim, which)`
/// (0x312820) as the pickers call it (`v_reg` holds the AI's rider).
pub(crate) fn pick_fitted_trigger_time(
    c: &mut Cpu,
    v_reg: usize,
    which: u64,
    ret: u32,
) -> crate::call::Flow {
    c.g[A2][0] = which;
    c.g[A0][0] = Addr(c.g[v_reg][0] as u32).off(RIDER_ANIM).lw(c);
    call_ptr(c, 0x0031_2820, ret)
}

/// `int ComputerActorRider_PickFittedTrick(ComputerActorRider* ai, float
/// height)` (0x10cd20): among the fitted tricks 1..14 (those with an
/// animation), the one whose wind-up plus trick time (trigger times over
/// the rider's trick skill factor) fits in `height` and that was picked
/// least often so far (ties: the first). Records the pick (`+0xe28` = 1,
/// lead / slack at `+0xe2c` / `+0xe34`, height at `+0xe30`), counts it and
/// returns its index, or -1. While the rider is in the air with an uber
/// trick ready (flag bit 0), only tricks without an uber animation count.
/// The loop checks events at its head 0x10cd98. Frame 0xa0: ra at 0,
/// s6..s0 at 0x10..0x70, f20..f23 at 0x80..0x98.
pub fn computer_actor_rider_pick_fitted_trick(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x70);
        frame.save_fpr(c, 23, 0x98);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        c.f[23] = c.f[12];
        frame.save(c, S3, 0x40);
        c.g[S2][0] = 1;
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        c.g[S4][0] = u64::MAX;
        frame.save(c, S6, 0x10);
        c.g[S5][0] = 0x2710;
        frame.save_fpr(c, 22, 0x90);
        c.g[S6][0] = TRICK_NONE;
        frame.save(c, S1, 0x60);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x88);
        frame.save_fpr(c, 20, 0x80);
        let ai = Addr(c.g[S0][0] as u32);
        ai.off(CAR_PICK_UBER).sw(c, 0);
        let height = FpuF32(c.f[23]);
        ai.off(CAR_PICK_HEIGHT).write_fpu(c, height);

        // The BAGT interface and the rider's trick skill factor.
        call_ptr(c, 0x0014_dc80, 0x0010_cd78)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 6;
        call_ptr(c, 0x0014_dd58, 0x0010_cd84)?;
        c.g[A0][0] = pick_fitted_ai_rider(c);
        c.g[S3][0] = c.g[V0][0];
        Callee {
            addr: 0x0012_0038,
            f: ee::s00::Rider_GetTrickSkillFactor,
        }
        .call(c, 0x0010_cd90)?;
        c.f[22] = c.f[0];
        c.g[A0][0] = pick_fitted_ai_rider(c);

        loop {
            // The trick's uber animation index (0x1b6: not a fitted trick).
            c.cycles += 13;
            c.g[V1][0] = Addr(c.g[A0][0] as u32).off(RIDER_VTABLE).lw(c);
            pick_fitted_rider_index(c, A0, A1, 0x0010_cdb0)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S2][0];
            call_ptr(c, 0x0015_0138, 0x0010_cdc0)?; // BEBAGTInt_GetTrickField6
            c.g[S1][0] = c.g[V0][0];
            if c.g[S1][0] == c.g[S6][0] {
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            } else {
                pick_fitted_try(c)?;
            }

            c.cycles += 3;
            let more = (c.g[S2][0] as i64) < FITTED_TRICKS;
            c.g[V0][0] = more as u64;
            if !more {
                break;
            }
            c.g[A0][0] = pick_fitted_ai_rider(c);
            if event_due(c) {
                deliver_event_here(c, 0x0010_cd98);
            }
        }

        // Count the pick.
        c.cycles += 3;
        let best = c.g[S4][0];
        c.g[V0][0] = ((best as i64) < FITTED_TRICKS) as u64;
        if best != u64::MAX {
            c.cycles += 2;
            let in_range = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(CAR_FITTED_COUNTS));
            if in_range {
                c.cycles += 5;
                c.g[A0][0] = sx((best as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let slot = Addr(c.g[V0][0] as u32);
                c.g[V1][0] = sx((slot.lw(c) as u32).wrapping_add(1));
                let n = c.g[V1][0];
                slot.sw(c, n);
            }
        }
        c.cycles += 15;
        c.g[V0][0] = c.g[S4][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 23, 0x98);
        frame.restore_fpr(c, 22, 0x90);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// One fitted trick (index s2, uber animation s1 != 0x1b6) of
/// [`computer_actor_rider_pick_fitted_trick`]: skip it while airborne with
/// an uber animation, else time it and keep it when it fits and was picked
/// less often than the best so far. Advances s2.
pub(crate) fn pick_fitted_try(c: &mut Cpu) -> crate::call::Flow {
    use crate::ps2f::FpuF32;
    c.cycles += 5;
    c.g[A1][0] = pick_fitted_ai_rider(c);
    let uber_ready = Addr(c.g[A1][0] as u32).off(RIDER_FLAGS).lw(c) & 1 != 0;
    c.g[V0][0] = TRICK_NONE;
    if uber_ready {
        c.cycles += 15;
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(RIDER_VTABLE).lw(c);
        pick_fitted_rider_index(c, A1, A0, 0x0010_cdf8)?;
        c.g[V1][0] = pick_fitted_ai_rider(c);
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S2][0];
        let level = Addr(c.g[V1][0] as u32).off(RIDER_UBER_LEVEL).lw(c);
        c.g[A3][0] = ((level as i64) < 5) as u64 ^ 1;
        call_ptr(c, 0x0015_0198, 0x0010_ce18)?; // BEBAGTInt_GetTrickAnim
        c.g[A1][0] = pick_fitted_ai_rider(c);
    }

    c.cycles += 6;
    c.f[1] = 0;
    let rider = Addr(c.g[A1][0] as u32);
    c.f[0] = rider.off(RIDER_AIR_HEIGHT).read_fpu(c).0;
    let airborne = FpuF32(c.f[1]).lt(FpuF32(c.f[0]));
    crate::abi::set_fcc(c, airborne);
    if airborne {
        c.cycles += 2;
        if c.g[V0][0] != c.g[S6][0] {
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            return Ok(());
        }
        c.cycles += 1;
    }
    c.g[V1][0] = rider.off(RIDER_VTABLE).lw(c);

    // Wind-up (field 0's animation) and trick time.
    c.cycles += 32;
    pick_fitted_rider_index(c, A1, A0, 0x0010_ce54)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S2][0];
    call_ptr(c, 0x0015_00d8, 0x0010_ce64)?; // BEBAGTInt_GetTrickField0
    c.g[V1][0] = pick_fitted_ai_rider(c);
    c.g[A1][0] = c.g[V0][0];
    pick_fitted_trigger_time(c, V1, 1, 0x0010_ce78)?;
    c.g[V0][0] = pick_fitted_ai_rider(c);
    c.f[20] = FpuF32(c.f[0]).div(FpuF32(c.f[22])).0;
    c.g[A1][0] = c.g[S1][0];
    pick_fitted_trigger_time(c, V0, 1, 0x0010_ce90)?;
    c.g[V0][0] = pick_fitted_ai_rider(c);
    c.f[21] = FpuF32(c.f[0]).div(FpuF32(c.f[22])).0;
    c.g[A1][0] = c.g[S1][0];
    pick_fitted_trigger_time(c, V0, 2, 0x0010_cea8)?;
    let end = FpuF32(c.f[0]).div(FpuF32(c.f[22]));
    c.f[0] = end.0;
    let total = end + FpuF32(c.f[20]);
    c.f[1] = total.0;
    let height = FpuF32(c.f[23]);
    let fits = total.lt(height);
    crate::abi::set_fcc(c, fits);
    c.g[A0][0] = sx((c.g[S2][0] as u32) << 2);
    let ai = Addr(c.g[S0][0] as u32);
    if fits {
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(CAR_FITTED_COUNTS));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let count = Addr(c.g[V1][0] as u32).lw(c);
        let fewer = (count as i64) < (c.g[S5][0] as i64);
        c.g[V0][0] = 1;
        if fewer {
            c.cycles += 7;
            let lead = end - FpuF32(c.f[21]);
            let slack = height - total;
            c.f[0] = lead.0;
            c.f[1] = slack.0;
            ai.off(CAR_PICK_FITTED).sw(c, 1);
            c.g[S4][0] = c.g[S2][0];
            ai.off(CAR_PICK_LEAD).write_fpu(c, lead);
            ai.off(CAR_PICK_SLACK).write_fpu(c, slack);
            c.g[S5][0] = Addr(c.g[V1][0] as u32).lw(c);
        }
    }
    c.cycles += 1;
    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
    Ok(())
}

/// Table callees of `Rider_ReportRiderHit0`: `SSXAudioSystem_GetInstance`,
/// `SSXAudioSystem_PlayRiderCollisionSound(audio, a, b)`,
/// `SSXAudioSystem_SpeechRiderCollision(audio, b, a, 1, 0)`,
/// `BEAggressionInt_GetInstance`, `BEAggressionInt_AddAggression(agg,
/// rider, rival, 0)`.
pub(crate) const HIT_AUDIO_INSTANCE: u32 = 0x0028_b180;

pub(crate) const HIT_PLAY_COLLISION_SOUND: u32 = 0x0029_8488;

pub(crate) const HIT_SPEECH_COLLISION: u32 = 0x002a_0a30;

pub(crate) const HIT_AGGRESSION_INSTANCE: u32 = 0x0015_5a50;

pub(crate) const HIT_ADD_AGGRESSION: u32 = 0x0015_5bf0;

/// The rider's controller sub-object (+0x6c0) and its virtual
/// `GetRiderIndex` slot (vtable +0x38: this delta, +0x3c: function).
pub(crate) const HIT_CONTROLLER: u32 = 0x6c0;

pub(crate) const HIT_INDEX_SLOT: u32 = 0x38;

/// Call `(rider + 0x6c0)->GetRiderIndex()` for the rider in `reg`; `reg`
/// becomes the controller pointer, as in the original.
pub(crate) fn hit_rider_index(c: &mut Cpu, reg: usize, ret: u32) -> crate::call::Flow {
    let ctl = r32(c, reg).wrapping_add(HIT_CONTROLLER);
    let vt = Addr(ctl).lw(c);
    c.g[V1][0] = vt;
    c.g[reg][0] = sx(ctl);
    let delta = Addr((vt as u32).wrapping_add(HIT_INDEX_SLOT)).lh(c);
    let f = Addr((vt as u32).wrapping_add(HIT_INDEX_SLOT + 4)).lw(c);
    c.g[V0][0] = f;
    c.g[A0][0] = sx(ctl.wrapping_add(delta as u32));
    call_ptr(c, f as u32, ret)
}

/// `void Rider_ReportRiderHit0(Rider *a, Rider *b)` (0x10e228): rider `a`
/// hit rider `b`: play the collision sound and the collision speech (kind
/// 1, variant 0), and add aggression from `b`'s index towards `a`'s.
pub fn rider_report_rider_hit0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, HIT_AUDIO_INSTANCE, 0x0010_e248)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, HIT_PLAY_COLLISION_SOUND, 0x0010_e258)?;
        call_ptr(c, HIT_AUDIO_INSTANCE, 0x0010_e260)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[T0][0] = 0;
        c.g[A3][0] = 1;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, HIT_SPEECH_COLLISION, 0x0010_e278)?;
        call_ptr(c, HIT_AGGRESSION_INSTANCE, 0x0010_e280)?;
        c.g[S2][0] = c.g[V0][0];
        hit_rider_index(c, S0, 0x0010_e29c)?;
        c.g[S0][0] = c.g[V0][0];
        hit_rider_index(c, S1, 0x0010_e2b8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = 0;
        call_ptr(c, HIT_ADD_AGGRESSION, 0x0010_e2cc)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as RET;
    use crate::difftest::SP;
    use crate::difftest::SP as STACK;
    use crate::difftest::SP as STACK_TOP;
    pub(crate) use crate::game::ai::ai::tests::belibrary;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::rng;
    pub(crate) use crate::game::ai::rider::tests::atan_constants;
    pub(crate) use crate::game::ai::rider::tests::w;
    pub(crate) use crate::game::ai::rider::tests::wf;
    pub(crate) use crate::game::ai::rider::tests::OBJ;
    pub(crate) use crate::game::ai::rider::tests::RIDER;
    pub(crate) use crate::game::bx::bxstring2::tests::be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_11 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_11 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_9 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_11 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_6 as W_3;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_4;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_2;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::lib::kernel::tests::SEEDS;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_2;
    pub(crate) use crate::game::movie::movieplayer::tests::w as w_2;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2;
    pub(crate) use crate::game::movie::movieplayer::tests::STUB;
    pub(crate) use crate::game::movie::movieplayer::tests::W as W_2;
    pub(crate) use crate::game::render::particle::tests::clear as clear_2;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::put_f32s;
    use ssx3_ee as ee;
    use ssx3_ee::s00;
    use ssx3_ee::s01;
    use ssx3_ee::s02;
    use ssx3_ee::s03;
    use ssx3_ee::s08;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use ssx3_ee::s11;
    use ssx3_ee::s12;
    use ssx3_ee::s13;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn computer_actor_rider_is_rival_target_matches() {
        check(
            ssx3_ee::s00::f_0010d9e8__generated,
            computer_actor_rider_is_rival_target,
            300,
            &|c, seed| {
                let mut r = Rng(0x0f0f_0f0f ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let rider = DATA + 0x100;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(DATA + 0x18, rider);
                c.bus
                    .write32(rider + 0xf0, if seed % 3 == 0 { 0 } else { r.next_u32() });
                let id = if seed % 4 == 0 {
                    // Force the id test to succeed: lw sign-extends into 64 bits, so
                    // keep a1's upper half zero and the value non-negative.
                    let x = r.next_u32() & 0x7fff_ffff;
                    c.g[A1][0] = x as u64;
                    x
                } else {
                    r.next_u32()
                };
                c.bus.write32(rider + 0xf8, id);
            },
        );
    }

    #[test]
    fn rider_get_ai_path_look_ahead_dist_matches() {
        check(
            ssx3_ee::s00::f_00112588__generated,
            rider_get_ai_path_look_ahead_dist,
            200,
            &|c, seed| {
                let mut r = Rng(0x5151_5151 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    ((r.next_u32() as u64) << 32) | (r.next_u32() as u64 | 1)
                };
            },
        );
    }

    #[test]
    fn rider_save_ai_path_look_ahead_matches() {
        check(
            ssx3_ee::s00::f_001125a8__generated,
            rider_save_ai_path_look_ahead,
            200,
            &|c, seed| {
                let mut r = Rng(0x6161_6161 ^ seed as u64);
                scramble(c, &mut r);
                // Odd seeds use a misaligned base to cover `lq`/`sq` masking.
                let base = DATA + (seed % 2) * 8;
                c.g[A0][0] = base as u64;
                for i in 0..8 {
                    c.bus.write32(base + 0x4a0 + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn computer_actor_rider_init_2_matches() {
        check(
            ssx3_ee::s00::f_00108f88,
            computer_actor_rider_init_2,
            60,
            &|c, seed| {
                let mut r = Rng(0x108f_8800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..16 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn computer_actor_rider_init_difficulty_2_matches() {
        check(
            ssx3_ee::s00::f_0010cab8,
            computer_actor_rider_init_difficulty_2,
            60,
            &|c, seed| {
                let mut r = Rng(0x10ca_b800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..16 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    /// Set up the world chain so `ComputerActorRider_GetHumanRider` returns a
    /// rider object, plus the rider/list floats the two "far" predicates read.
    pub(crate) fn far_setup(c: &mut Cpu, seed: u32, threshold_off: u32) {
        let mut r = Rng(0xfab0_0000 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        c.g[GP][0] = GAME_GP as u64;
        let world = GAME_GP.wrapping_sub(0x848);
        let w = DATA + 0x700;
        let x = DATA + 0x740;
        let y = DATA + 0x780;
        let v = DATA + 0x7c0;
        let human = DATA + 0x400;
        let list = DATA + 0x900;
        c.bus.write32(world, w);
        c.bus.write32(w + 0x84, x);
        c.bus.write32(x + 0xc, y);
        c.bus.write32(y, v);
        c.bus.write32(v + 0x28, human);
        c.bus.write32(DATA + 0x18, list);
        // A zero first word makes FindHumanRiderIndex fail -> GetHumanRider 0.
        c.bus.write32(list, if seed % 4 == 0 { 0 } else { 1 });
        c.bus.write32(list + 4, if seed % 4 == 0 { 0 } else { 1 });
        c.bus.write32(human + 0x4d0, r.edge_f32());
        c.bus.write32(list + 0x4d0, r.edge_f32());
        c.bus.write32(list + threshold_off, r.edge_f32());
    }

    #[test]
    fn computer_actor_rider_is_far_behind_human_matches() {
        check(
            ssx3_ee::s00::f_0010d938,
            computer_actor_rider_is_far_behind_human,
            120,
            &|c, seed| {
                far_setup(c, seed, 0xdc);
            },
        );
    }

    #[test]
    fn computer_actor_rider_is_far_ahead_of_human_matches() {
        check(
            ssx3_ee::s00::f_0010d990,
            computer_actor_rider_is_far_ahead_of_human,
            120,
            &|c, seed| {
                far_setup(c, seed, 0xe0);
            },
        );
    }

    #[test]
    fn ai_reset_rider_relations_2_matches() {
        check(
            ssx3_ee::s00::f_0010f398,
            ai_reset_rider_relations_2,
            60,
            &|c, seed| {
                let mut r = Rng(0x10f3_9800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..16 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn air_predictor_ctor_matches() {
        check(
            ssx3_ee::s00::f_00113138,
            air_predictor_ctor,
            60,
            &|c, seed| {
                let mut r = Rng(0x1131_3800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..8 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ai_count_riders_on_path_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 86, 1);
            let ai = DATA + 0x400;
            let count = 1 + r.next_u32() % 4;
            c.bus.write32(ai + 0x78, count);
            let path = r.next_u32() % 4;
            for i in 0..count {
                let rider = DATA + 0x2000 + i * 0x100;
                let vt = DATA + 0x9000;
                c.bus.write32(ai + 0x28 + 4 * i, rider);
                c.bus.write32(rider + 0x6c0, vt);
                c.bus.write16(vt + 0x48, 0);
                c.bus.write32(vt + 0x4c, 0x0000_5000);
                // Half the riders are on the queried path.
                c.bus
                    .write32(rider + 0xab8, if i % 2 == 0 { path } else { r.next_u32() });
            }
            c.g[A0][0] = ai as u64;
            c.g[A1][0] = path as u64;
            schedule(c, &mut r, seed, 0x0010_fc30)
        };
        check_events(
            ssx3_ee::s00::f_0010fc30,
            ai_count_riders_on_path,
            SEEDS,
            &setup,
        );
    }

    #[test]
    fn rider_update_race_progress_matches() {
        check(
            ssx3_ee::s00::f_00112338__generated,
            rider_update_race_progress,
            120,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x112338, 3);
                let rider = RIDER;
                c.g[A0][0] = rider as u64;
                w(c, GAME_GP.wrapping_sub(0x848), 0);
                // A created BE library object (the event handler reaches it).
                w(c, GAME_GP.wrapping_sub(0x1f04), OBJ + 0x3000);
                w(c, GAME_GP.wrapping_sub(0x1efc), 0);
                w(c, 0x0053_08a8, 0);
                // No race path (the null path reads as an empty one).
                w(c, rider + rider::RACE_PATH, 0);
                w(c, rider + rider::RACE_PATH_CURSOR, 0);
                w(c, rider + rider::DIST_TO_FINISH, 0);
                w(c, rider + rider::DIST_ESTIMATE, 0);
                let count = (seed % 3) as u32;
                w(c, rider + rider::RACE_ENTRY_COUNT, count);
                for i in 0..count {
                    w(c, rider + rider::RACE_ENTRIES + 16 * i, 0);
                    w(c, rider + rider::RACE_ENTRIES + 16 * i + 4, 0);
                    w(c, rider + rider::RACE_ENTRIES + 16 * i + 8, r.next_u32());
                }
                // The shared list is empty: no path events to hand out.
                for i in 0..8u32 {
                    w(c, RACE_EVENTS + 4 * i, 0);
                }
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn rider_update_ai_path_progress_matches() {
        check(
            ssx3_ee::s00::f_001125c0__generated,
            rider_update_ai_path_progress,
            120,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1125c0, 4);
                let rider = RIDER;
                c.g[A0][0] = rider as u64;
                atan_constants(c);
                w(c, GAME_GP.wrapping_sub(0x848), 0);
                // A null AI path reads as an empty one.
                w(c, rider + rider::AI_PATH, 0);
                w(c, rider + rider::AI_PATH_CURSOR, 0);
                w(c, rider + rider::DIST_TO_FINISH, 0);
                w(c, rider + rider::AI_ARC_LEN, r.edge_f32());
                w(
                    c,
                    rider + rider::AI_PROGRESS,
                    f32::from_bits(r.f32(100.0)).to_bits(),
                );
                w(
                    c,
                    rider + rider::AI_SEARCH_DIST,
                    f32::from_bits(r.f32(100.0)).to_bits(),
                );
                // The look-ahead offset: sometimes the degenerate branches.
                let target = match seed % 4 {
                    0 => 0.0f32,
                    1 => 1.0,
                    2 => -1.0,
                    _ => f32::from_bits(r.f32(5.0)),
                };
                let point = match seed % 3 {
                    0 => 0.0f32,
                    1 => 1.0,
                    _ => f32::from_bits(r.f32(5.0)),
                };
                w(c, rider + rider::AI_POINT, point.to_bits());
                w(c, rider + rider::AI_POINT + 4, point.to_bits());
                w(c, rider + rider::AI_POINT_TARGET, target.to_bits());
                w(c, rider + rider::AI_POINT_TARGET + 4, target.to_bits());
            },
        );
    }

    /// The `gp_2`-relative `.lit4` constants these functions read (a test
    /// machine's RAM starts empty; the game image holds them).
    pub(crate) fn literals(c: &mut Cpu) {
        put_words(c, GAME_GP - 0x2de8, &[0x5015_02f9, 0xd015_02f9]);
        // The angle-wrap pool around `gp_2 - 0x7d18`.
        put_words(
            c,
            GAME_GP - 0x7d18,
            &[
                0xbfc9_0fdb,
                0x3fc9_0fdb,
                0x4049_0fdb,
                0x4049_0fdb,
                0x3e22_f983,
                0x40c9_0fdb,
                0x3f86_0a93,
            ],
        );
        // 0.036 / 4.0086374
        put_words(c, GAME_GP - 0x7070, &[0x3d13_74bc, 0x4080_46c2]);
    }

    #[test]
    fn computer_actor_rider_should_reset_path_matches() {
        check(
            ssx3_ee::s00::f_0010d1a0__generated,
            computer_actor_rider_should_reset_path,
            300,
            &|c, seed| {
                let mut r = Rng(0x2c1b_3e4d ^ seed as u64);
                scramble(c, &mut r);
                literals(c);
                c.g[GP][0] = GAME_GP as u64;
                let actor = DATA;
                let rider = DATA + 0x200;
                let base = DATA + 0x400;
                let predictor = DATA + 0x300;
                c.g[A0][0] = actor as u64;
                c.bus.write32(actor + computer::RIDER, rider);
                let dir = if seed % 3 == 0 {
                    -(1 + (r.next_u32() % 4) as i32)
                } else {
                    (r.next_u32() % 5) as i32
                };
                c.bus.write32(actor + computer::DIR, dir as u32);
                let stuck = match seed % 4 {
                    0 => 0,
                    1 => 222,
                    2 => 230,
                    _ => r.next_u32() % 400,
                };
                c.bus.write32(actor + computer::STUCK_TIME, stuck);
                let wrong = match seed % 5 {
                    0 => 0,
                    1 => 60,
                    2 => 61,
                    3 => r.next_u32() % 100,
                    _ => 1000,
                };
                c.bus.write32(actor + computer::WRONG_TIME, wrong);
                // Velocity: zero x, zero y, and both signs.
                let (vx, vy) = match seed % 6 {
                    0 => (0, 0),
                    1 => (0, r.edge_f32()),
                    2 => (0, r.edge_f32() | 0x8000_0000),
                    3 => (r.f32(30.0), 0),
                    4 => (r.f32(30.0), r.f32(30.0)),
                    _ => (r.edge_f32(), r.edge_f32()),
                };
                put_vec(
                    c,
                    rider + rider::VELOCITY_2,
                    [vx, vy, r.f32(10.0), r.f32(1.0)],
                );
                let speed = match seed % 4 {
                    0 => r.f32(900.0),
                    1 => 0x447a_0000, // exactly 1000
                    2 => 0x4520_0000, // 2560
                    _ => r.f32(2000.0),
                };
                c.bus.write32(rider + rider::SPEED, speed);
                c.bus.write32(rider + rider::HEADING_2, r.f32(6.3));
                c.bus.write32(rider + rider::BASE_2, base);
                c.bus.write32(rider + rider::AIR_PREDICTOR, predictor);
                c.bus.write32(base + rider_base::MAIN_STATE, seed % 6);
                c.bus.write32(
                    base + rider_base::SUB_STATE,
                    if seed % 5 == 0 { 8 } else { seed % 9 },
                );
                let height = match seed % 3 {
                    0 => r.f32(20.0),
                    1 => 0x41f0_0000, // exactly 30
                    _ => 0x4240_0000, // 48
                };
                c.bus.write32(predictor + air_predictor::HEIGHT, height);
                c.bus
                    .write32(predictor + air_predictor::LANDING, r.f32(50.0));
                c.bus.write32(predictor + air_predictor::MODE, seed % 4);
            },
        );
    }

    #[test]
    fn rider_base_update_main_state_matches() {
        check(
            ee::s00::f_00111408__generated,
            rider_base_update_main_state,
            200,
            &|c, seed| {
                rider_state_table_test(c, seed, 0x0011_1408, 0xde0, 0x0045_6b50, 0x0011_1494);
            },
        );
    }

    #[test]
    fn rider_base_post_update_main_state_matches() {
        check(
            ee::s00::f_001114a0__generated,
            rider_base_post_update_main_state,
            200,
            &|c, seed| {
                rider_state_table_test(c, seed, 0x0011_14a0, 0xde0, 0x0045_6b70, 0x0011_152c);
            },
        );
    }

    #[test]
    fn rider_base_handle_sub_state_input_matches() {
        check(
            ee::s00::f_00111728__generated,
            rider_base_handle_sub_state_input,
            200,
            &|c, seed| {
                rider_state_table_test(c, seed, 0x0011_1728, 0xde4, 0x0045_6c10, 0x0011_1824);
            },
        );
    }

    /// The shared setup of the three state tables: a rider whose state
    /// selects a table entry, and a table that either stays inside the
    /// function (its tail / epilogue block) or is empty (an indirect jump
    /// out, answered by the fake jump hook).
    pub(crate) fn rider_state_table_test(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        state_off: u32,
        table: u32,
        tail: u32,
    ) {
        machine(c, seed, salt, 0);
        let rider = DATA;
        let states = [0u32, 1, 2, 3, 4, 5, 6, 13, 14, 0xffff_ffff];
        c.bus
            .write32(rider + state_off, states[(seed as usize) % states.len()]);
        let case = if seed % 3 == 0 {
            0
        } else if seed % 3 == 1 {
            tail
        } else {
            tail + 4
        };
        put_words(c, table, &[case; 14]);
        c.pc = salt as u32;
        c.g[A0][0] = sx(rider);
    }

    // ------------------------------------------------------------------- riders

    #[test]
    fn computer_actor_rider_find_human_rider_index_matches() {
        check(
            ee::s00::f_0010d8f8__generated,
            computer_actor_rider_find_human_rider_index,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x0010_d8f8 ^ seed as u64);
                scramble(c, &mut r);
                let actor = DATA + 0x1000;
                c.g[A0][0] = actor as u64;
                let riders = DATA + 0x2000;
                c.bus.write32(actor + 0x18, riders);
                // Slot `seed % 8` is the first one with both links set; 6 and 7
                // leave none, so the scan falls out of the table with -1.
                let hit = seed % 8;
                for i in 0..6u32 {
                    let e = riders + 0x24 * i;
                    let (first, second) = if i < hit {
                        if seed % 2 == 0 {
                            (r.next_u32(), 0)
                        } else {
                            (0, r.next_u32())
                        }
                    } else if i == hit {
                        (r.next_u32() | 1, r.next_u32() | 1)
                    } else {
                        (0, 0)
                    };
                    c.bus.write32(e, first);
                    c.bus.write32(e + 4, second);
                }
            },
        );
    }

    #[test]
    fn rider_add_boost_matches() {
        check(
            ee::s00::f_0010e098__generated,
            rider_add_boost,
            240,
            &|c, seed| {
                let mut r = Rng(0x10e0_98 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let this = DATA;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                // BELibrary singleton: a zeroed dummy object, else it loads from
                // disc forever.
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x800);
                // cBE_getInterface indexes a table at 0x5308a8; steer the index
                // into RAM and put a dummy interface (with a do-nothing method)
                // there.
                c.bus.write32(GAME_GP - 0x1efc, 0x03ec_bfd6);
                c.bus.write32(0x0083_0000, DATA + 0x900);
                c.bus.write32(DATA + 0x900, 0);
                c.bus.write32(DATA + 0x904, DATA + 0x940);
                c.bus.write16(DATA + 0x948, 0);
                c.bus.write32(DATA + 0x94c, STUB);
                let (state, amt, boost, timer, count, mask_field, flag) = match seed % 8 {
                    0 => (3, 0.5f32, 0.0f32, 0.0f32, 0u32, 0xffff_ffffu32, 1u32),
                    1 => (0, 0.0, 0.0, 0.0, 0, 0xffff_ffff, 1),
                    2 => (0, -0.5, 0.2, 0.0, 4, 0xffff_ffff, 1),
                    3 => (0, 0.5, 0.0, 0.0, 0, 0, 1),
                    4 => (0, 0.1, 0.2, 0.0, 0, 0xffff_ffff, 1),
                    5 => (0, 0.9, 0.5, 0.0, 5, 0xffff_ffff, 1),
                    6 => (0, 0.9, 0.5, 3.0, 10, 0xffff_ffff, 1),
                    _ => (0, -0.5, 0.2, 0.0, 20, 0xffff_ffff, 1),
                };
                c.bus.write32(this + rider::BOOST_STATE, state);
                c.bus.write32(this + rider::BOOST_2, boost.to_bits());
                c.bus.write32(this + rider::BOOST_TIMER, timer.to_bits());
                c.bus.write32(this + rider::BOOST_COUNT, count);
                c.bus.write32(this + rider::BOOST_MASK, mask_field);
                c.bus.write32(this + rider::BOOST_FLAG, flag);
                c.bus.write32(this + rider::CHAR_ID, r.next_u32());
                c.bus.write32(this + rider::CHAR_COMP, r.next_u32());
                c.bus.write32(this + 0x874, 0);
                c.f[12] = amt.to_bits();
            },
        );
    }

    #[test]
    fn rider_dtor_free_matches() {
        crate::difftest::check(
            ee::s00::f_00113170__generated,
            rider_dtor_free,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_3170, 2);
                be_library(c);
                // A null object keeps `operator delete` on its "nothing to free"
                // path; a real one exercises the heap manager.
                if seed % 2 == 1 {
                    c.g[A0][0] = 0;
                    c.g[A1][0] = 1;
                } else {
                    c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
                    c.g[A1][0] = sx(r.next_u32() & !1);
                }
            },
        );
    }

    // -----------------------------------------------------------------
    // Rider, AI, flags, BE singletons.

    #[test]
    fn computer_actor_rider_reset_trick_counts_matches() {
        check(
            s00::f_0010d170__generated,
            computer_actor_rider_reset_trick_counts,
            200,
            &|c, seed| {
                let mut r = Rng(0x10d1_70aa ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OBJ_2);
                clear(c, OBJ_2, 0x1200);
                // Fifteen triplets of live counters.
                for i in 0..15u32 {
                    let base = OBJ_2 + 0xef4 + 4 * i;
                    c.bus.write32(base - 0x78, r.next_u32());
                    c.bus.write32(base - 0x3c, r.next_u32());
                    c.bus.write32(base, r.next_u32());
                }
            },
        );
    }

    // ------------------------------------------------------------- riders

    #[test]
    fn rider_award_bonus_boost_matches() {
        check(
            ssx3_ee::s00::f_0010e8b8__generated,
            rider_award_bonus_boost,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10e8b8, 4);
                belibrary(c, &mut r);
                let rider = DATA;
                let mtx = DATA + 0x500;
                // `RiderMetrix_AwardBonusPoints` returns early for a non-4 mode.
                c.bus.write8(0x0053_5c48, (seed % 3) as u8);
                c.bus.write32(rider + 0x790, mtx);
                c.bus.write32(mtx + 0x1c4, r.f32(1.0));
                // The multiplier sfx path returns early with a null sfx table.
                c.bus.write32(rider + 0x874, 0);
                c.g[A0][0] = rider as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // Riders, camera and front end.

    #[test]
    fn rider_on_mission_completed_matches() {
        check(
            ee::s00::f_0010f2d8__generated,
            rider_on_mission_completed,
            SEEDS_2,
            &|c, seed| {
                let mut r = common(c, seed, 0x10f2d8, 4);
                let this = DATA + 0x100;
                clear_2(c, this, 0x900 / 4);
                c.bus.write32(this + 0x790, DATA + 0x2000);
                c.bus.write32(this + 0x86c, DATA + 0x3000);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn rider_on_item_collected_matches() {
        check(
            ee::s00::f_0010f338__generated,
            rider_on_item_collected,
            SEEDS_2,
            &|c, seed| {
                let mut r = common(c, seed, 0x10f338, 4);
                let this = DATA + 0x100;
                clear_2(c, this, 0x900 / 4);
                c.bus.write32(this + 0x790, DATA + 0x2000);
                c.bus.write32(this + 0x86c, DATA + 0x3000);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn rider_get_dist_to_finish_matches() {
        check(
            ssx3_ee::s00::f_00113128__generated,
            rider_get_dist_to_finish,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1131_28, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.bus.write32(DATA + 0x100 + 0x4d0, r.next_u32());
            },
        );
    }

    #[test]
    fn rider_return_zero_25b8_matches() {
        leaf(
            ssx3_ee::s00::f_001125b8__generated,
            rider_return_zero_25b8,
            3,
        );
    }

    // ------------------------------------------- RiderBase enter/exit methods

    /// `RiderBase_EnterMainState`/`ExitMainState` fixture: the real dispatch
    /// tables (`.rodata 0x456b30`/`0x456b10`, loaded from the ELF image) pick
    /// the arm from the state word at `owner + 0xde0`; a zeroed owner plus a
    /// zeroed world and back-end singleton keep the real state methods on
    /// their short paths. States 0..5 are all six table entries (in the real
    /// order 0 ground, 1 air, 2 wipeout, 3 base, 4 railslide, 5 handplant);
    /// the others are the `state >= 6` fall-through to the base arm.
    pub(crate) fn state_fixture(c: &mut Cpu, seed: u32, salt: u64, table: u32) {
        machine(c, seed, salt, 1);
        elf_image(c);
        c.g[ssx3_ee_rt::cpu::reg::SP][0] = (STACK - 0x200) as u64;
        for i in 0..0x400 {
            c.bus.write32(DATA + i * 4, 0);
        }
        c.bus.write32(GAME_GP - 0x848, DATA + 0x800); // world
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x900); // BELibrary singleton
        c.bus.write32(GAME_GP - 0x1f28, DATA + 0xa00);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        let state = match seed % 10 {
            0..=5 => seed % 10,
            6 => 6,
            7 => 0x10,
            8 => 0xffff_ffff,
            _ => 0x100,
        };
        c.g[A0][0] = DATA as u64;
        c.bus.write32(DATA + 0xde0, state);
        // The fixture really selects an arm from the game's table, not zero
        // RAM: every in-range state has a non-zero case label there.
        if state < 6 {
            assert_ne!(
                c.bus.read32(table + state * 4),
                0,
                "state table {table:#x} not loaded"
            );
        }
    }

    #[test]
    fn rider_base_enter_main_state_matches() {
        check(
            ssx3_ee::s00::f_00111380__generated,
            rider_base_enter_main_state,
            30,
            &|c, seed| state_fixture(c, seed, 45, 0x0045_6b30),
        );
    }

    #[test]
    fn rider_base_exit_main_state_matches() {
        check(
            ssx3_ee::s00::f_001112f8__generated,
            rider_base_exit_main_state,
            30,
            &|c, seed| state_fixture(c, seed, 46, 0x0045_6b10),
        );
    }

    #[test]
    fn compute_steer_matches() {
        covered(
            ssx3_ee::s00::f_00100348__generated,
            computer_actor_rider_compute_steer,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x51ce, 2);
                c.pc = 0x0010_0348;
                let rider = DATA;
                let model = DATA + 0x1_0000;
                let vt = DATA + 0x2_0000;
                let axis = DATA + 0x3_0000;
                let motion = DATA + 0x3_1000;
                let vel = DATA + 0x3_2000;
                c.g[A0][0] = rider as u64;
                c.g[A1][0] = vel as u64;
                c.bus.write32(rider + 0x18, model);
                c.bus.write32(model + 0x6c0, vt);
                c.bus
                    .write16(vt + 0x10, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x14, FAKE);
                c.bus
                    .write16(vt + 0x28, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x2c, FAKE + 0x10);
                // The two virtual calls return these two quadwords.
                c.bus.write32(fakes::LOG_RESULTS, axis);
                c.bus.write32(fakes::LOG_RESULTS + 4, motion);
                put_f32s(c, motion, [0.0; 4]);
                match seed % 6 {
                    // Exactly axis-aligned: the degenerate f20 == 0 paths.
                    0 => {
                        put_f32s(c, model + 0x1c0, [1.0, 0.0, 0.0, 0.0]);
                        put_f32s(c, axis, [1.0, 0.0, 0.0, 0.0]);
                        put_f32s(c, vel, [0.0, 1.0, 0.0, 0.0]);
                    }
                    1 => {
                        put_f32s(c, model + 0x1c0, [0.0, 1.0, 0.0, 0.0]);
                        put_f32s(c, axis, [0.0, 0.0, -1.0, 0.0]);
                        put_f32s(c, vel, [1.0, 0.0, 0.0, 0.0]);
                    }
                    2 => {
                        put_f32s(c, model + 0x1c0, [0.0, 1.0, 0.0, 0.0]);
                        put_f32s(c, axis, [0.0, 0.0, 1.0, 0.0]);
                        put_f32s(c, vel, [1.0, 0.0, 0.0, 0.0]);
                    }
                    // Random: the atan path; seed % 6 == 4 is the backwards
                    // branch (the model's z above zero).
                    4 => {
                        put_f32s(
                            c,
                            model + 0x1c0,
                            [
                                f32::from_bits(r.f32(2.0)),
                                f32::from_bits(r.f32(2.0)),
                                1.0,
                                1.0,
                            ],
                        );
                        put_f32s(
                            c,
                            axis,
                            [
                                f32::from_bits(r.f32(2.0)),
                                f32::from_bits(r.f32(2.0)),
                                f32::from_bits(r.f32(2.0)),
                                0.0,
                            ],
                        );
                        put_f32s(
                            c,
                            vel,
                            [
                                f32::from_bits(r.f32(2.0)),
                                f32::from_bits(r.f32(2.0)),
                                0.0,
                                0.0,
                            ],
                        );
                    }
                    _ => {
                        put_f32s(
                            c,
                            model + 0x1c0,
                            [
                                f32::from_bits(r.f32(2.0)),
                                f32::from_bits(r.f32(2.0)),
                                -f32::from_bits(r.f32(1.0)),
                                0.0,
                            ],
                        );
                        put_f32s(
                            c,
                            axis,
                            [
                                f32::from_bits(r.f32(2.0)),
                                f32::from_bits(r.f32(2.0)),
                                f32::from_bits(r.f32(2.0)),
                                0.0,
                            ],
                        );
                        put_f32s(
                            c,
                            vel,
                            [
                                f32::from_bits(r.f32(2.0)),
                                f32::from_bits(r.f32(2.0)),
                                0.0,
                                0.0,
                            ],
                        );
                    }
                }
            },
        );
    }

    pub(crate) const SEEDS_3: u32 = 240;

    // ---------------------------------------------------------------- rider

    // Fixture addresses (well above the fake callees' returned pointers,
    // `DATA + (hash & 0x3fff0)`).
    pub(crate) const RIDER_2: u32 = DATA + 0x8_0000;

    pub(crate) const STATE: u32 = DATA + 0x8_1000;

    pub(crate) const MODEL: u32 = DATA + 0x8_2000;

    pub(crate) const INPUT: u32 = DATA + 0x8_3000;

    pub(crate) const HUMAN: u32 = DATA + 0x8_4000;

    pub(crate) const WORLD: u32 = DATA + 0x8_5000;

    pub(crate) const BE_LIBRARY: u32 = DATA + 0x8_6000;

    pub(crate) const BE_IFACE: u32 = DATA + 0x8_7000;

    pub(crate) const BE_VTABLE: u32 = DATA + 0x8_7100;

    /// A CPU rider (at `RIDER_2`) whose model (`MODEL`) is at `+0x18`; the
    /// model's sub-state machine (`STATE`, read through `+0x77c`) announces
    /// sub-state `seed % 16` — every arm of the dispatch plus the two
    /// "no step" cases. The world/AI chain (gp - 0x848) has no riders; the
    /// BE library and its interface table exist so `ComputeRubberBand` can
    /// run both of its paths. Fake callee results are forced small: they are
    /// used as loop bounds by the thinking steps.
    pub(crate) fn rider_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 8);
        put_words(c, LOG_RESULTS, &[1, 2, 1, 3, 1, 2, 1, 4]);

        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x84, WORLD + 0x100);
        c.bus.write32(WORLD + 0x100 + 0xc, WORLD + 0x200);
        c.bus.write32(WORLD + 0x200 + 0x78, 0);
        c.bus.write32(WORLD + 0x200 + 0x7c, 0);
        c.bus
            .write32(WORLD + 0x200 + 0x28, if seed % 2 == 0 { HUMAN } else { 0 });

        // The BE library singleton (its hot path) and interface slot 0,
        // whose virtual method is a fake.
        c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(0x0053_08a8, BE_IFACE);
        c.bus.write32(BE_IFACE + 4, BE_VTABLE);
        c.bus.write16(BE_VTABLE + 8, 0);
        c.bus.write32(BE_VTABLE + 0xc, FAKE + 0x80);

        c.bus.write32(RIDER_2 + 0x18, MODEL);
        // `Rider_GetSubState` is called with the model (`*(rider + 0x18)`),
        // which reads its state machine at `+0x77c` and the sub-state at
        // its `+0xde4`.
        c.bus.write32(MODEL + 0x77c, STATE);
        c.bus.write32(STATE + 0xde4, seed % 16);

        // `ComputeRubberBand`: the model's sub-state (`0` exits early), the
        // "is a human rider" words the rider scan at `MODEL` reads, and the
        // distance that decides between its two tails.
        c.bus.write32(
            MODEL + 0xe4,
            if seed % 3 == 0 {
                0
            } else {
                r.next_u32() % 4 + 1
            },
        );
        let scans = seed % 2 == 0;
        c.bus.write32(MODEL, scans as u32);
        c.bus.write32(MODEL + 4, scans as u32);
        c.bus.write32(
            MODEL + 0x4d0,
            if seed % 4 == 0 {
                r.f32(1000.0)
            } else {
                0xc000_0000
            },
        );
        c.bus.write32(HUMAN + 0x4d0, r.f32(1000.0));
        c.bus.write8(0x0053_5c11, (seed % 3) as u8);
        // The AI frame counter and the shove cooldown the steps read.
        c.bus.write32(WORLD + 0x200 + 8, r.next_u32() % 32);
        c.bus.write32(RIDER_2 + 0xf30, r.edge_f32());
        c.bus.write32(RIDER_2 + 0xf34, r.next_u32() % 2);

        c.g[A0][0] = RIDER_2 as u64;
        c.g[A1][0] = INPUT as u64;
        c.pc = 0x0010_a768;
    }

    #[test]
    fn computer_actor_rider_get_control_input_matches() {
        check(
            ssx3_ee::s00::f_0010a768__generated,
            computer_actor_rider_get_control_input,
            SEEDS_3,
            &|c, seed| rider_setup(c, seed, 0x105),
        );
    }

    // ------------------------------------------------------- cruise follow

    pub(crate) const RIDER_ENTRY: u32 = DATA + 0x8_8000;

    pub(crate) const PMF: u32 = DATA + 0x8_9000;

    /// The cruise step's inputs: the rider (`a0`: model at `+0x18`, the
    /// marker/jump fields) and its control block (`a1`: flags at `+0`,
    /// steering at `+4`, model and rival slot at `+0x18`/`+0x80`). The AI
    /// manager (gp - 0x848 -> world -> game module -> AI) has a few riders
    /// whose `+0x6c0` path interface is a fake PMF, so the "is a jump on"
    /// answer comes from the test machine.
    pub(crate) fn cruise_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x107, 6);
        put_words(c, LOG_RESULTS, &[1, 2, 1, 3, 1, 1, 2, 1]);

        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x84, WORLD + 0x100);
        c.bus.write32(WORLD + 0x100 + 0xc, WORLD + 0x200);
        c.bus.write32(WORLD + 0x200 + 0x28, WORLD + 0x300);
        let riders = if seed % 3 == 0 { 0 } else { 1 + seed % 3 };
        c.bus.write32(WORLD + 0x200 + 0x78, riders);
        c.bus.write32(WORLD + 0x200 + 0x7c, riders);
        c.bus.write32(WORLD + 0x200 + 8, r.next_u32() % 64);
        for i in 0..3u32 {
            let entry = RIDER_ENTRY + 0x100 * i;
            c.bus.write32(WORLD + 0x300 + 4 * i, entry);
            c.bus.write32(entry + 0x6c0, PMF);
            fill(c, &mut r, entry, 8);
        }
        c.bus.write16(PMF + 0x40, 0);
        c.bus.write32(PMF + 0x44, FAKE + 0x20);

        // The rider: model, marker distances, board press, jump state.
        c.bus.write32(RIDER_2 + 0x18, MODEL);
        c.bus.write32(RIDER_2 + 0xdf0, r.edge_f32());
        c.bus.write32(RIDER_2 + 0xdf8, r.edge_f32());
        c.bus.write32(RIDER_2 + 0xe40, r.next_u32() % 4);
        c.bus.write16(RIDER_2 + 0xf38, (r.next_u32() % 4) as u16);
        c.bus.write32(RIDER_2 + 0xe70, r.next_u32());
        // The control block: flags, steering byte, model, rival slot.
        c.bus.write32(STATE, r.next_u32() & !0x000c_0000);
        c.bus.write32(STATE + 0x4, r.next_u32());
        c.bus.write32(STATE + 0x18, MODEL);
        c.bus.write32(STATE + 0x80, r.next_u32() % 3);

        // The model: the marker list owner (a fake virtual at +0xab8), the
        // forward axis, the crouch distances and the steer-target corners.
        c.bus.write32(MODEL + 0xab8, 0);
        put_vec(
            c,
            MODEL + 0x1e0,
            [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0],
        );
        c.bus.write32(MODEL + 0x4c0, r.f32(100.0));
        c.bus.write32(MODEL + 0x4c4, r.f32(100.0));
        c.bus.write32(MODEL + 0x2f0, r.f32(1000.0));
        c.bus.write32(MODEL + 0x2f8, r.f32(1000.0));
        c.bus.write32(MODEL + 0x110, r.edge_f32());
        c.bus.write32(MODEL + 0x490, r.edge_f32());
        c.bus.write32(MODEL + 0x4b0, r.edge_f32());

        c.g[A0][0] = RIDER_2 as u64;
        c.g[A1][0] = STATE as u64;
        c.pc = 0x0010_0680;
    }

    #[test]
    fn computer_actor_rider_cruise_follow_path_matches() {
        check(
            ssx3_ee::s00::f_00100680__generated,
            computer_actor_rider_cruise_follow_path,
            SEEDS_3,
            &cruise_setup,
        );
    }

    #[test]
    fn rider_set_timed_event_matches() {
        check(
            ee::s00::f_0010e028__generated,
            rider_set_timed_event,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x10e0_28);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    (r.next_u32() | 1) as u64
                };
                c.f[12] = if seed % 3 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(5.0)
                };
            },
        );
    }

    /// The timed-event deadline (`+0x354`/`+0x358`) is a count of ticks of
    /// `AI_GetFrameCounter`: `Rider_SetTimedEvent(..., 2.0 s)` must land the
    /// same wall-clock time ahead at every rate (the 60 Hz path is unchanged).
    #[test]
    fn rider_set_timed_event_follows_the_rate() {
        use crate::tick;
        const WORLD: u32 = 0x0081_0000;
        const AI: u32 = 0x0081_1000;
        const RACE: u32 = 0x0081_2000;
        const COUNTER: u32 = 1000;
        let ahead = |hz: u32| -> f64 {
            let mut c = crate::difftest::fresh();
            c.g[GP][0] = GAME_GP as u64;
            // `world -> +0x84 -> +0xc -> +0x8`: the AI frame counter.
            c.bus.write32(GAME_GP - 0x848, WORLD);
            c.bus.write32(WORLD + 0x84, AI);
            c.bus.write32(AI + 0xc, RACE);
            c.bus.write32(RACE + 0x8, COUNTER);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = 1; // mode != 0: set a timed event
            c.f[12] = 2.0f32.to_bits(); // seconds
            tick::set_for_test(&mut c, hz);
            rider_set_timed_event(&mut c);
            let deadline = c.bus.read32(DATA + 0x354) as i32 as i64;
            let mode = c.bus.read32(DATA + 0x358) as i32 as i64;
            assert_eq!(mode, 1);
            (deadline - COUNTER as i64) as f64 / hz as f64
        };
        let at_60 = ahead(60);
        assert!((at_60 - 2.0).abs() < 1e-9, "{at_60} s at 60 Hz");
        for hz in [120u32, 240] {
            let at = ahead(hz);
            assert!(
                (at - at_60).abs() < 1e-9,
                "{hz} Hz: {at} s ahead, 60 Hz: {at_60} s"
            );
        }
    }

    pub(crate) fn wb(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }

    #[test]
    fn rider_on_split_marker_matches() {
        check(
            ee::s00::f_0010e558__generated,
            rider_on_split_marker,
            16,
            &|c, seed| {
                let _ = base_2(c, seed, 0x10e5_58, 6);
                let rider = DATA;
                let marker = DATA + 0x1000;
                w_2(c, rider + 0xec, 0x1234);
                w_2(c, rider + 0x790, DATA + 0x2000);
                w_2(c, marker, 0x55);
                wb(c, 0x0053_5c12, (seed % 2) as u8);
                w_2(c, 0x0053_08d0, (seed % 4 == 0) as u32 * 0x200);
                c.g[A0][0] = rider as u64;
                c.g[A1][0] = marker as u64;
            },
        );
    }

    #[test]
    fn rider_v15_matches() {
        check(ee::s00::f_0010e830__generated, rider_v15, 40, &|c, seed| {
            let _r = machine(c, seed, 0x10e830, 3);
            c.g[A0][0] = sx(DATA);
            c.f[12] = 1.0f32.to_bits();
            // The rider base and its main state (1 = Air, 4 = Railslide).
            c.bus.write32(DATA + 0x77c, DATA + 0x1000);
            let state = [0u32, 1, 4, 2][seed as usize % 4];
            c.bus.write32(DATA + 0x1000 + 0xde0, state);
            c.bus.write32(DATA + 0x790, DATA + 0x2000);
        });
    }

    // ------------------------------------------------------ rider FX reset

    #[test]
    fn rider_base_reset_fx_matches() {
        check(
            ee::s00::f_00111890__generated,
            rider_base_reset_fx,
            40,
            &|c, seed| {
                alloc_base(c, seed, 0x1118, 3);
                c.g[A0][0] = DATA as u64;
                w_3(c, DATA + 0x18, DATA + 0x400);
                w_3(c, DATA + 0x400 + 0x870, if seed % 2 == 0 { 0 } else { 5 });
            },
        );
    }

    #[test]
    fn difficulty_matches() {
        check(
            ssx3_ee::s00::f_0010c4f8__generated,
            computer_actor_rider_apply_difficulty,
            300,
            &|c, seed| {
                let mut r = Rng(0x5177_a9c3 ^ seed as u64);
                // The generated body is a `match pc` dispatcher that also
                // jumps through the difficulty jump tables, so a caller
                // enters it with pc = entry and the tables (`.rodata`) hold
                // the case-block addresses.
                put_words(
                    c,
                    0x0045_6a90,
                    &[
                        0x0010_c524,
                        0x0010_c55c,
                        0x0010_c584,
                        0x0010_c5ac,
                        0x0010_c5d4,
                    ],
                );
                put_words(
                    c,
                    0x0045_6ab0,
                    &[
                        0x0010_c638,
                        0x0010_c650,
                        0x0010_c664,
                        0x0010_c678,
                        0x0010_c688,
                    ],
                );
                put_words(
                    c,
                    0x0045_6ad0,
                    &[
                        0x0010_c6cc,
                        0x0010_c6ec,
                        0x0010_c700,
                        0x0010_c714,
                        0x0010_c728,
                    ],
                );
                scramble(c, &mut r);
                c.pc = 0x0010_c4f8;
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 7) as u64; // 0, 6 and 1..5
                c.g[A2][0] = (seed % 4) as u64; // modes 0, 1, 2, 3
                c.bus.write32(DATA + 0x18, DATA + 0x100);
                // The float constants the tables load from `gp`.
                let g = (GAME_GP as u32).wrapping_sub(0x7dd8);
                for k in 0..0x30u32 {
                    c.bus.write32(g + 4 * k, r.next_u32());
                }
                for k in 0..0x10u32 {
                    c.bus.write32(DATA + 0x100 + 4 * k, r.next_u32());
                    c.bus.write32(DATA + 0xdf8 + 4 * k, r.next_u32());
                }
            },
        );
    }

    pub(crate) const RACE_EVENT_RIDER: u32 = DATA + 0x1_0000;

    pub(crate) const RACE_EVENT_ARG: u32 = DATA + 0x1_1000;

    pub(crate) const RACE_EVENT_STATE: u32 = DATA + 0x1_2000;

    #[test]
    fn rider_handle_race_path_event_matches() {
        check_all(
            ee::s00::f_0010e5d8__generated,
            rider_handle_race_path_event,
            24,
            &|c, seed| {
                let _ = base_4(c, seed, 0x10e5_d8, 10);
                c.g[A0][0] = sx(RACE_EVENT_RIDER);
                c.g[A1][0] = sx(RACE_EVENT_ARG);
                c.g[A2][0] = sx(DATA + 0x1_3000);
                c.g[A3][0] = sx(seed);
                c.bus
                    .write32(RACE_EVENT_ARG, [1u32, 11, 3][seed as usize % 3]);
                c.bus.write32(RACE_EVENT_ARG + 4, seed % 4);
                c.bus.write32(RACE_EVENT_RIDER + 0x470, 0xbf80_0000); // -1.0
                                                                      // world+0xc0 is the race-path state object.
                c.bus.write32(WORLD_2 + 0xc0, RACE_EVENT_STATE);
                c.bus.write32(RACE_EVENT_STATE + 0x98, (seed % 2) as u32);
            },
        );
    }

    // ---------------------------------------------------------- think spin

    const SPIN_AI: u32 = DATA + 0x9_0000;
    const SPIN_RIDER: u32 = DATA + 0x9_1000;
    const SPIN_AIR: u32 = DATA + 0x9_2000;
    const SPIN_STATE: u32 = DATA + 0x9_3000;
    const SPIN_MODEL: u32 = DATA + 0x9_4000;
    const SPIN_ANIMS: u32 = DATA + 0x9_5000;
    const SPIN_INPUT: u32 = DATA + 0x9_6000;
    /// `AnimTable_GetEntry`'s table (0x446990, 0x1c per animation).
    const ANIM_TABLE: u32 = 0x0044_6990;

    /// The AI, its rider (air state, spin counter, animation set, spin
    /// sub-state for `UpdateSpinPlan`), the plan fields and the input block.
    /// Heights stay off the trick pickers' long paths: a trick is only chosen
    /// at 0.6/0.7 units (`ChooseTrick` answers -1 there at once).
    fn think_spin_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let pick = |r: &mut Rng, v: &[u32]| v[(r.next_u32() as usize) % v.len()];
        let f = |x: f32| x.to_bits();
        // The BE library singleton and interface slot 0 (a fake), for the
        // attribute factor `UpdateSpinPlan`'s progress queries ask.
        c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(0x0053_08a8, BE_IFACE);
        c.bus.write32(BE_IFACE + 4, BE_VTABLE);
        c.bus.write16(BE_VTABLE + 8, 0);
        c.bus.write32(BE_VTABLE + 0xc, FAKE + 0x80);
        // `.lit4` constants the plan compares with.
        for off in [0x7e10u32, 0x7e14, 0x7e18, 0x7e1c, 0x7e20] {
            c.bus.write32(
                GAME_GP - off,
                pick(&mut *r, &[f(0.0), f(0.5), f(2.0), f(-1.0)]),
            );
        }
        for i in 0..6u32 {
            c.bus.write32(0x004f_f030 + 4 * i, r.next_u32());
        }

        c.bus.write32(SPIN_AI + 0x18, SPIN_RIDER);
        c.bus.write32(SPIN_RIDER + 0x788, SPIN_AIR);
        c.bus
            .write32(SPIN_AIR + 0xac, [1u32, 3, 0, 2][(seed % 4) as usize]);
        c.bus.write32(SPIN_AIR + 0xa0, 0);
        c.bus.write32(
            SPIN_AIR + 0x98,
            pick(&mut *r, &[f(0.3), f(0.6), f(0.7), f(20.0), f(16.0)]),
        );
        c.bus
            .write32(SPIN_RIDER + 0x484, pick(&mut *r, &[0, 5, 29, 30, 31, 100]));
        c.bus.write32(SPIN_RIDER + 0x77c, SPIN_STATE);
        let x = SPIN_STATE + 0x230;
        for off in [0x10u32, 0x14, 0x18, 0x1c, 0x38, 0x3c] {
            c.bus.write32(x + off, r.f32(4.0));
        }
        c.bus.write32(x + 0x58, SPIN_MODEL);
        c.bus.write32(SPIN_MODEL + 0x2ec, r.f32(2.0));
        c.bus.write32(SPIN_MODEL + 0x86c, r.next_u32() % 4);
        c.bus.write32(SPIN_MODEL + 0xb34, 1 + r.next_u32() % 3);
        // Animation slot 2: the empty animation, or one of type 0x14 / 7.
        c.bus.write32(SPIN_RIDER + 0x784, SPIN_ANIMS);
        c.bus.write32(SPIN_ANIMS + 8, pick(&mut *r, &[0x1b6, 5, 6]));
        c.bus.write32(ANIM_TABLE + 5 * 0x1c, 0x14);
        c.bus.write32(ANIM_TABLE + 6 * 0x1c, 7);

        let ai = SPIN_AI;
        c.bus.write32(ai + 0xe14, r.next_u32() % 2);
        c.bus
            .write32(ai + 0xe18, if r.next_u32() % 4 == 0 { 1 } else { 0 });
        c.bus
            .write32(ai + 0xe0c, pick(&mut *r, &[0xffff_ffff, 0xffff_ffff, 3]));
        c.bus.write32(ai + 0xe20, pick(&mut *r, &[0, 0, 1, 2]));
        c.bus.write32(ai + 0xe24, r.next_u32() % 2);
        c.bus.write32(ai + 0xe28, r.next_u32() % 2);
        c.bus
            .write32(ai + 0xe2c, pick(&mut *r, &[f(0.0), f(10.0), f(-5.0)]));
        c.bus
            .write32(ai + 0xe30, pick(&mut *r, &[f(0.0), f(30.0), f(100.0)]));
        c.bus.write32(
            ai + 0xe34,
            pick(&mut *r, &[f(0.0), f(-1.0), f(0.65), f(50.0)]),
        );
        c.bus.write32(ai + 0xe3c, pick(&mut *r, &[0, 0, f(1.0)]));
        c.bus.write32(ai + 0xdf0 + 0x48, r.f32(1.0));
        c.bus.write32(ai + 0xdf0 + 0x4c, r.f32(1.0));
        c.bus.write32(SPIN_INPUT, r.next_u32());
        c.bus.write32(SPIN_INPUT + 4, r.next_u32());

        c.g[A0][0] = sx(ai);
        c.g[A1][0] = sx(SPIN_INPUT);
    }

    #[test]
    fn computer_actor_rider_think_spin_matches() {
        check_lohi(
            ee::s00::f_0010b250__generated,
            computer_actor_rider_think_spin,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_b250, 4);
                think_spin_world(c, &mut r, seed);
            },
        );
    }

    /// The call (1-based) on which a rider spinning more than 15 units up
    /// gives the spin up (input flag 0x1000), at rate `hz`, with a plan in
    /// place (no callees).
    fn spin_give_up_call(hz: u32) -> u32 {
        let mut c = crate::difftest::fresh();
        let c = &mut *c;
        crate::tick::set_for_test(c, hz);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(SPIN_AI + 0x18, SPIN_RIDER);
        c.bus.write32(SPIN_RIDER + 0x788, SPIN_AIR);
        c.bus.write32(SPIN_AIR + 0xac, 1);
        c.bus.write32(SPIN_AIR + 0xa0, 0);
        c.bus.write32(SPIN_AIR + 0x98, 20.0f32.to_bits());
        c.bus.write32(SPIN_RIDER + 0x484, 0);
        c.bus.write32(SPIN_AI + 0xe14, 1);
        c.bus.write32(SPIN_AI + 0xe3c, 1.0f32.to_bits());
        for n in 1..=10_000 {
            c.bus.write32(SPIN_INPUT, 0);
            c.g[A0][0] = sx(SPIN_AI);
            c.g[A1][0] = sx(SPIN_INPUT);
            c.g[ssx3_ee_rt::cpu::reg::SP][0] = sx(STACK);
            c.g[RA][0] = sx(RET);
            computer_actor_rider_think_spin(c);
            assert_eq!(c.pc, RET, "think did not return");
            if c.bus.read32(SPIN_INPUT) & 0x1000 != 0 {
                return n;
            }
        }
        panic!("{hz} Hz: never gave up");
    }

    /// The AI gives a high spin up after the same wall-clock time (30 frames
    /// at 60 Hz): on call 31 at 60, (31 - 1) * r / 60 + 1 at r.
    #[test]
    fn computer_actor_rider_think_spin_give_up_follows_the_rate() {
        assert_eq!(spin_give_up_call(60), 31);
        assert_eq!(spin_give_up_call(120), 61);
        assert_eq!(spin_give_up_call(240), 121);
    }

    /// One tick of an airborne AI with no trick yet, 0.7 units up (above
    /// the 0.5 pick height, so `ChooseTrick` is called; below 0.75, so it
    /// answers -1 at once), on a freshly built world each tick (the 60 Hz
    /// clocks of [`crate::tick::frame60_due`] live outside the machine):
    /// the cycles the tick took.
    fn think_spin_pick_tick(hz: u32) -> u64 {
        let mut c = crate::difftest::fresh();
        let c = &mut *c;
        let mut r = machine(c, 7, 0x0010_b250, 0);
        think_spin_world(c, &mut r, 0);
        crate::tick::set_for_test(c, hz);
        c.bus.write32(SPIN_AIR + 0xac, 1);
        c.bus.write32(SPIN_AIR + 0xa0, 0);
        c.bus.write32(SPIN_AIR + 0x98, 0.7f32.to_bits());
        c.bus.write32(SPIN_RIDER + 0x484, 0);
        c.bus.write32(SPIN_ANIMS + 8, 0x1b6);
        c.bus.write32(SPIN_AI + 0xe14, 0);
        c.bus.write32(SPIN_AI + 0xe18, 0);
        c.bus.write32(SPIN_AI + 0xe0c, u32::MAX);
        c.bus.write32(SPIN_AI + 0xe20, 0);
        c.bus.write32(SPIN_AI + 0xe34, 50.0f32.to_bits());
        let before = c.cycles;
        computer_actor_rider_think_spin(c);
        assert_eq!(c.pc, RET, "think did not return");
        c.cycles - before
    }

    /// The trick pick (its random rolls) is tried once per 60 Hz frame:
    /// every tick at 60, every other tick at 120, every fourth at 240.
    #[test]
    fn computer_actor_rider_think_spin_picks_once_per_frame() {
        let with_pick = think_spin_pick_tick(60);
        for _ in 0..8 {
            assert_eq!(
                think_spin_pick_tick(60),
                with_pick,
                "60 Hz: a pick every tick"
            );
        }
        for (hz, ticks) in [(120, 120), (240, 240)] {
            let picks = (0..ticks)
                .filter(|_| think_spin_pick_tick(hz) == with_pick)
                .count();
            assert_eq!(picks, 60, "{hz} Hz: one pick per 60 Hz frame");
        }
    }

    // ------------------------------------------- start gate and attacks

    const GATE_AI: u32 = DATA + 0xa_0000;
    const GATE_RIDER: u32 = DATA + 0xa_1000;
    const GATE_WORLD: u32 = DATA + 0xa_2000;
    const GATE_MODULE: u32 = DATA + 0xa_2100;
    const GATE_MGR: u32 = DATA + 0xa_2200;
    const GATE_VT: u32 = DATA + 0xa_2400;
    const GATE_INPUT: u32 = DATA + 0xa_3000;
    const GATE_SIDE: u32 = DATA + 0xa_3010;
    /// The answer of the fake `riderV(.., 7)` in the rate tests.
    const GATE_INDEX: u32 = DATA + 0xa_3020;
    /// Pre-built BE singletons (aggression, race, player, library).
    const GATE_BE: u32 = DATA + 0xa_4000;
    /// `BEAggressionInt_GetRivalLevel`'s byte for profile 0, characters 0/0.
    const RIVALRY_00: u32 = 0x004a_6ca8 + 0xbc2;

    /// The AI frame counter chain (`gp-0x848 -> +0x84 -> +0xc`, counter at
    /// `+8`, rider count at `+0x78`), the AI's rider and its `riderV`
    /// vtable (slot 7 at `FAKE + 0x10`).
    fn gate_world(c: &mut Cpu, now: u32, riders: u32) {
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP - 0x848, GATE_WORLD);
        c.bus.write32(GATE_WORLD + 0x84, GATE_MODULE);
        c.bus.write32(GATE_MODULE + 0xc, GATE_MGR);
        c.bus.write32(GATE_MGR + 8, now);
        c.bus.write32(GATE_MGR + 0x78, riders);
        c.bus.write32(GATE_AI + 0x18, GATE_RIDER);
        c.bus.write32(GATE_RIDER + 0x6c0, GATE_VT);
        c.bus.write16(GATE_VT + 0x38, 0);
        c.bus.write32(GATE_VT + 0x3c, FAKE + 0x10);
    }

    /// `riderV(.., 7)` answering the index stored at [`GATE_INDEX`].
    fn gate_index_hook(c: &mut Cpu, _target: u32) {
        let ra = c.g[RA][0] as u32;
        c.g[V0][0] = sx(c.bus.read32(GATE_INDEX));
        c.pc = ra;
    }

    #[test]
    fn computer_actor_rider_think_gate_matches() {
        check_lohi(
            ee::s00::f_0010b790__generated,
            computer_actor_rider_think_gate,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_b790, 3);
                let pick = |r: &mut Rng, v: &[u32]| v[(r.next_u32() as usize) % v.len()];
                let now = pick(
                    &mut r,
                    &[
                        0,
                        20,
                        40,
                        7,
                        100,
                        108,
                        109,
                        120,
                        140,
                        146,
                        147,
                        150,
                        160,
                        170,
                        175,
                        179,
                        180,
                        200,
                        0x7fff_fff0,
                    ],
                );
                let now = if seed % 7 == 0 { r.next_u32() } else { now };
                gate_world(c, now, 4);
                // The rider index (fake results 0..2; nonzero: 1..5).
                for n in 0..3u32 {
                    c.bus.write32(LOG_RESULTS + 4 * n, 1 + r.next_u32() % 5);
                }
                // The jitter scale and the game RNG state.
                let any = r.next_u32();
                c.bus.write32(
                    GAME_GP - 0x7e0c,
                    pick(&mut r, &[0x2f80_0000, 0x3000_0000, any]),
                );
                for i in 0..6u32 {
                    c.bus.write32(0x004f_f030 + 4 * i, r.next_u32());
                }
                c.bus.write32(
                    GATE_AI + 0xdf0,
                    if seed % 5 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(1.0)
                    },
                );
                c.bus.write32(GATE_INPUT, r.next_u32());
                c.g[A0][0] = sx(GATE_AI);
                c.g[A1][0] = sx(GATE_INPUT);
            },
        );
    }

    /// The steering field `ThinkGate` writes at rate `hz`, `frames60` 60 Hz
    /// frames after the race start, for rider `index` (jitter preset to 0.5,
    /// a fresh RNG): (field, whether the jitter was re-rolled).
    fn gate_steer(hz: u32, frames60: u32, index: u32) -> (u32, bool) {
        let mut c = crate::difftest::fresh();
        let c = &mut *c;
        crate::tick::set_for_test(c, hz);
        c.hooks.miss = gate_index_hook;
        gate_world(c, frames60 * hz / 60, 4);
        c.bus.write32(GATE_INDEX, index);
        const SENTINEL: u32 = 0x7fc0_1234;
        c.bus.write32(GATE_AI + 0xdf0, SENTINEL);
        c.bus.write32(GATE_INPUT, 0);
        c.g[A0][0] = sx(GATE_AI);
        c.g[A1][0] = sx(GATE_INPUT);
        c.g[ssx3_ee_rt::cpu::reg::SP][0] = sx(STACK);
        c.g[RA][0] = sx(RET);
        computer_actor_rider_think_gate(c);
        assert_eq!(c.pc, RET, "think gate did not return");
        let field = (c.bus.read32(GATE_INPUT) >> 12) & 0x3f;
        (field, c.bus.read32(GATE_AI + 0xdf0) != SENTINEL)
    }

    /// The start gate decisions (push off at `t < 33 - (index - 1) * 10`,
    /// lean back at `t < 72`, the jitter re-rolled every 20 frames) fall on
    /// the same wall-clock times at 120 and 240 Hz as at 60, and the jitter
    /// is re-rolled 3 times a second at every rate.
    #[test]
    fn computer_actor_rider_think_gate_follows_the_rate() {
        let frames = [
            0u32, 20, 30, 59, 100, 107, 108, 109, 120, 127, 128, 140, 146, 147, 148, 160, 170, 179,
        ];
        for index in 1..=4u32 {
            for &f in &frames {
                let at60 = gate_steer(60, f, index);
                for hz in [120u32, 240] {
                    assert_eq!(
                        gate_steer(hz, f, index),
                        at60,
                        "{hz} Hz, frame {f}, index {index}"
                    );
                }
            }
        }
        // A rider that waits (index 4): t = 180 - frames > 72 for a second.
        for hz in [60u32, 120, 240] {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            crate::tick::set_for_test(c, hz);
            c.hooks.miss = gate_index_hook;
            gate_world(c, 0, 4);
            c.bus.write32(GATE_INDEX, 4);
            let mut rerolls = 0;
            for now in 0..hz {
                c.bus.write32(GATE_MGR + 8, now);
                c.bus.write32(GATE_AI + 0xdf0, 0x7fc0_1234);
                c.g[A0][0] = sx(GATE_AI);
                c.g[A1][0] = sx(GATE_INPUT);
                c.g[ssx3_ee_rt::cpu::reg::SP][0] = sx(STACK);
                c.g[RA][0] = sx(RET);
                computer_actor_rider_think_gate(c);
                assert_eq!(c.pc, RET);
                rerolls += (c.bus.read32(GATE_AI + 0xdf0) != 0x7fc0_1234) as u32;
            }
            assert_eq!(rerolls, 3, "{hz} Hz");
        }
    }

    /// The angle constants of `FindAttackTarget` (`gp - 0x7ce0..`: pi,
    /// pi/6, 5pi/6, -pi/2, pi/2, 1/(2pi), 2pi), `Math_Atan`'s, and the BE
    /// singletons `GetRivalLevel` reaches, with every rider on slot 0
    /// (profile 0, character 0): its level is the byte at [`RIVALRY_00`].
    fn attack_world(c: &mut Cpu, level: u8) {
        let pi = std::f32::consts::PI;
        for (k, v) in [
            pi,
            pi / 6.0,
            5.0 * pi / 6.0,
            -pi / 2.0,
            pi / 2.0,
            0.5 / pi,
            2.0 * pi,
        ]
        .into_iter()
        .enumerate()
        {
            c.bus.write32(GAME_GP - 0x7ce0 + 4 * k as u32, v.to_bits());
        }
        atan_constants(c);
        c.bus.write32(GAME_GP - 0x1ea4, GATE_BE);
        c.bus.write32(GAME_GP - 0x1f3c, GATE_BE + 0x100);
        c.bus.write32(GAME_GP - 0x1f30, GATE_BE + 0x200);
        c.bus.write32(GAME_GP - 0x1f04, GATE_BE + 0x300);
        for i in 0..8u32 {
            c.bus.write32(0x0053_05b0 + 4 * i, 0);
        }
        for k in 0..7u32 {
            c.bus.write32(0x0053_5b20 + 4 * k, 0);
        }
        c.bus.write8(RIVALRY_00, level);
    }

    #[test]
    fn computer_actor_rider_find_attack_target_matches() {
        check_lohi(
            ee::s00::f_0010dbf0__generated,
            computer_actor_rider_find_attack_target,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_dbf0, 4);
                let pick = |r: &mut Rng, v: &[u32]| v[(r.next_u32() as usize) % v.len()];
                let f = |x: f32| x.to_bits();
                let now = match seed % 8 {
                    0 => r.next_u32(),
                    1 => 1 + 6 * (r.next_u32() % 100),
                    _ => 6 * (r.next_u32() % 200),
                };
                let riders = pick(&mut r, &[0, 1, 2, 3, 3]);
                gate_world(c, now, riders);
                attack_world(c, pick(&mut r, &[1, 2, 3, 4, 0x80]) as u8);
                // Our own index (fake results; nonzero: 1..3).
                for n in 0..8u32 {
                    c.bus.write32(LOG_RESULTS + 4 * n, 1 + r.next_u32() % 3);
                }
                // Last attack: long ago, at the edge of the cooldown, or recent.
                let last = pick(&mut r, &[0, 1, 2, 3]);
                let last = match last {
                    0 => 0,
                    1 => now.wrapping_sub(121),
                    2 => now.wrapping_sub(120),
                    _ => now.wrapping_sub(r.next_u32() % 120),
                };
                c.bus.write32(GATE_RIDER + 0xf4, last);
                let comp = |r: &mut Rng| pick(r, &[0, f(1.0), f(-1.0), f(0.5), f(-2.0)]);
                let (vx, vy) = if seed % 3 == 0 {
                    (r.f32(3.0), r.f32(3.0))
                } else {
                    (comp(&mut r), comp(&mut r))
                };
                c.bus.write32(GATE_RIDER + 0x1b0, vx);
                c.bus.write32(GATE_RIDER + 0x1b4, vy);
                c.bus.write32(GATE_RIDER + 0x1b8, r.next_u32());
                c.bus.write32(GATE_RIDER + 0x1bc, r.next_u32());
                for i in 0..4u32 {
                    let e = GATE_RIDER + 0x24 * i;
                    c.bus.write32(
                        e + 8,
                        pick(&mut r, &[f(50.0), f(150.0), f(199.0), f(250.0)]),
                    );
                    c.bus.write32(
                        e + 0xc,
                        if seed % 4 == 0 {
                            r.f32(4.0)
                        } else {
                            comp(&mut r)
                        },
                    );
                }
                c.bus.write32(GATE_SIDE, r.next_u32());
                c.g[A0][0] = sx(GATE_AI);
                c.g[A1][0] = sx(GATE_SIDE);
            },
        );
    }

    /// The ticks (in seconds) on which an AI attacks a rival in reach, over
    /// `seconds` at rate `hz`.
    fn attack_times(hz: u32, seconds: u32) -> Vec<f64> {
        let mut c = crate::difftest::fresh();
        let c = &mut *c;
        crate::tick::set_for_test(c, hz);
        c.hooks.miss = gate_index_hook;
        gate_world(c, 0, 2);
        attack_world(c, 4);
        // We are rider 0; rider 1 is 50 away at direction 0, and we head
        // pi/2 (velocity (0, 1)): 90 degrees off, on our right.
        c.bus.write32(GATE_INDEX, 0);
        c.bus.write32(GATE_RIDER + 0x1b0, 0);
        c.bus.write32(GATE_RIDER + 0x1b4, 1.0f32.to_bits());
        c.bus.write32(GATE_RIDER + 0x24 + 8, 50.0f32.to_bits());
        c.bus.write32(GATE_RIDER + 0x24 + 0xc, 0);
        c.bus.write32(GATE_RIDER + 8, 1000.0f32.to_bits());
        c.bus.write32(GATE_RIDER + 0xf4, 0);
        let mut times = Vec::new();
        for now in 0..hz * seconds {
            c.bus.write32(GATE_MGR + 8, now);
            c.bus.write32(GATE_SIDE, 0);
            c.g[A0][0] = sx(GATE_AI);
            c.g[A1][0] = sx(GATE_SIDE);
            c.g[ssx3_ee_rt::cpu::reg::SP][0] = sx(STACK);
            c.g[RA][0] = sx(RET);
            computer_actor_rider_find_attack_target(c);
            assert_eq!(c.pc, RET, "find attack target did not return");
            if c.g[V0][0] == 1 {
                assert_eq!(c.bus.read32(GATE_SIDE), 1);
                assert_eq!(c.bus.read32(GATE_RIDER + 0xf4), now);
                times.push(now as f64 / hz as f64);
            }
        }
        times
    }

    /// The 120-frame attack cooldown takes the same wall-clock time at 120
    /// and 240 Hz (the `% 6` search spreading stays in ticks, so the times
    /// differ by under 6 ticks of 60 Hz).
    #[test]
    fn computer_actor_rider_find_attack_target_follows_the_rate() {
        let at60 = attack_times(60, 10);
        assert_eq!(
            at60,
            [126.0 / 60.0, 252.0 / 60.0, 378.0 / 60.0, 504.0 / 60.0]
        );
        for hz in [120u32, 240] {
            let at = attack_times(hz, 10);
            assert_eq!(at.len(), at60.len(), "{hz} Hz: {at:?}");
            for (a, b) in at.iter().zip(&at60) {
                assert!(
                    (a - b).abs() < 0.1 * (1.0 + b / 2.0),
                    "{hz} Hz: {at:?}, 60 Hz: {at60:?}"
                );
            }
        }
    }

    /// The forced path reselect off the path (`Rider_UpdateRaceProgress`,
    /// `Rider_UpdateAIPathProgress`: `now % period == 0`) runs once a
    /// second at every rate; at 60 the period is the original 0x3c.
    #[test]
    fn rider_path_reselect_period_follows_the_rate() {
        for hz in [60u32, 120, 240] {
            let mut c = crate::difftest::fresh();
            crate::tick::set_for_test(&mut c, hz);
            let period = reselect_period(&mut c);
            if hz == 60 {
                assert_eq!(period, 0x3c);
            }
            let per_10s = (0..hz * 10).filter(|t| *t as u64 % period == 0).count();
            assert_eq!(per_10s, 10, "{hz} Hz");
        }
    }
    use super::*;

    // ------------------------------------------------ OverlapList_FindClosestContact

    /// Control block of the fake `OverlapList_CollideVolume`: a magic word
    /// (other tests' machines never have it, so their calls run the real
    /// body), the contact count and the template contacts to copy out.
    pub(crate) const FCC_CTRL: u32 = 0x00a0_0000;

    pub(crate) const FCC_MAGIC: u32 = 0xc011_1de5;

    /// `OverlapList_CollideVolume` for the fixture: copy `count` template
    /// contacts to `a2` and return the count (the real body needs a whole
    /// world of collision objects).
    pub(crate) fn fake_collide_volume(c: &mut Cpu) {
        if c.bus.read32(FCC_CTRL) != FCC_MAGIC {
            return ssx3_ee::s11::f_00333ef8__generated(c);
        }
        let count = c.bus.read32(FCC_CTRL + 4);
        let template = c.bus.read32(FCC_CTRL + 8);
        let out = c.g[A2][0] as u32;
        for i in 0..count * 0x20 {
            let w = c.bus.read32(template + 4 * i);
            c.bus.write32(out + 4 * i, w);
        }
        c.cycles += 40 + count as u64;
        c.g[V0][0] = count as u64;
        c.g[V1][0] = 0x1234;
        abi::ret(c);
    }

    /// Installs the gated fakes ([`fake_collide_volume`],
    /// [`fake_closest_point`]) in their callees' slots once for the whole
    /// test process: without their magic word in a machine's RAM they run
    /// the generated body, so other tests are unaffected (and no test
    /// restores a slot under another one running in parallel).
    pub(crate) fn install_fake_slots() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| unsafe {
            ssx3_ee::s11::OVERRIDE_00333EF8 = fake_collide_volume;
            ssx3_ee::s07::OVERRIDE_0026A638 = fake_closest_point;
        });
    }

    // ------------------------------------------------ Rider_UpdateDistToFinish

    /// Control block of the fake `Path_GetClosestPointFrom`: magic word
    /// (other tests run the real body) and the distance along it returns.
    pub(crate) const UDF_CTRL: u32 = 0x00a1_0000;

    pub(crate) const UDF_MAGIC: u32 = 0x0d15_7f1e;

    /// `Path_GetClosestPointFrom` for the fixture: return the chosen
    /// distance along the path in f0 (the real one walks a course path).
    pub(crate) fn fake_closest_point(c: &mut Cpu) {
        if c.bus.read32(UDF_CTRL) != UDF_MAGIC {
            return ssx3_ee::s07::f_0026a638__generated(c);
        }
        c.f[0] = c.bus.read32(UDF_CTRL + 4);
        c.f[1] = 0x4000_0000;
        c.g[V0][0] = 0x55;
        c.cycles += 31;
        abi::ret(c);
    }

    /// `Rider_UpdateDistToFinish`: farther (old-distance refresh), closer
    /// with no split list, all splits ahead, and crossing up to six split
    /// markers (the list ending on a 0 entry or one still ahead); the path's
    /// segment query is a fake that sometimes unwinds.
    #[test]
    fn rider_update_dist_to_finish_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        install_fake_slots();
        check(
            ssx3_ee::s00::f_00112fb0__generated,
            rider_update_dist_to_finish,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0011_2fb0, 1);
                c.pc = 0x0011_2fb0;
                let rider = DATA + 0x1000;
                let path = DATA + 0x3000;
                let vtable = DATA + 0x3100;
                c.g[A0][0] = sx(rider);
                c.g[A1][0] = sx(DATA);
                c.g[A2][0] = sx(DATA + 0x100);
                c.g[A3][0] = sx(DATA + 0x200);
                c.bus.write32(rider + 0xab4, path);
                c.bus.write32(rider + 0xac0, r.next_u32());
                c.bus.write32(rider + 0xec, r.next_u32());
                c.bus.write32(path + 0x34, vtable);
                c.bus.write16(vtable + 0x8, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vtable + 0xc, FAKE);
                let length = 1000.0f32;
                c.bus.write32(path + 0x38, length.to_bits());
                let pick = |r: &mut Rng, set: &[f32]| set[r.next_u32() as usize % set.len()];
                let dist = pick(&mut r, &[100.0, 180.0, 250.0, 320.0, 400.0]);
                let prev = pick(&mut r, &[150.0, 250.0, 350.0, 450.0, 90.0]);
                c.bus.write32(UDF_CTRL, UDF_MAGIC);
                c.bus.write32(UDF_CTRL + 4, (length - dist).to_bits());
                c.bus.write32(rider + 0x4d4, prev.to_bits());
                // Split markers: distances (0 ends the list), mostly descending.
                let mut d = pick(&mut r, &[0.0, 120.0, 300.0, 440.0, 520.0]);
                for k in 0..8u32 {
                    c.bus.write32(0x004d_33b8 + 8 * k, r.next_u32());
                    c.bus.write32(0x004d_33bc + 8 * k, d.to_bits());
                    d = if r.next_u32() % 6 == 0 {
                        0.0
                    } else {
                        (d - 60.0).max(10.0)
                    };
                }
                // Rider_OnSplitMarker's feature switch: off (its short path).
                c.bus.write32(0x0053_08d0, 1 << 9);
            },
        );
    }
    use super::*;
    use super::*;
    use crate::difftest::check_paths;

    /// Attack-back decision: timer negative / running / zero; AI frame a
    /// multiple of 6 or not; 0-4 riders, each one mine or not, attacking or
    /// not, nearer than 200 or not, playing entry kind 13 or not (or no
    /// animation, 0x1b6). The interface methods (slots 7 and 8) point at the
    /// real `AnimController_GetLayerAnimId` with delta 4, which returns the
    /// word at iface+0x14: the fixture's answer.
    #[test]
    fn computer_actor_rider_should_attack_back_matches() {
        use crate::difftest::GAME_GP;
        use attack_back::*;
        check(
            ee::s00::f_0010da10__generated,
            computer_actor_rider_should_attack_back,
            300,
            &|c, seed| {
                let mut r = Rng(0xda10 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0010_da10;
                let me = DATA;
                let world = DATA + 0x1000;
                let root = DATA + 0x1100;
                let ai = DATA + 0x1200;
                let my_rider = DATA + 0x2000;
                let vt_self = DATA + 0x9000;
                let vt_other = DATA + 0x9100;
                let getter = 0x0031_2aa0; // returns lw(a0 + 0x10) with delta 4
                let timer = match seed % 4 {
                    0 => r.f32(2.0),
                    1 => 0x8000_0000,
                    _ => 0,
                };
                c.bus.write32(me + TIMER, timer);
                c.bus.write32(me + DECISION, r.next_u32());
                c.bus.write32(me + RIDER, my_rider);
                c.bus.write32(GAME_GP - 0x7ce4, 0x3c88_8889); // 1/60
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, root);
                c.bus.write32(root + 0xc, ai);
                let frame = if seed % 3 == 0 {
                    r.next_u32()
                } else {
                    6 * (r.next_u32() % 1000)
                };
                c.bus.write32(ai + 8, frame);
                let count = r.next_u32() % 5;
                c.bus.write32(ai + AI_COUNT, count);
                c.bus.write32(my_rider + IFACE, vt_self);
                c.bus.write16(vt_self + 0x38, 4);
                c.bus.write32(vt_self + 0x3c, getter);
                c.bus.write32(my_rider + IFACE + 0x14, r.next_u32() % 4);
                c.bus.write16(vt_other + 0x40, 4);
                c.bus.write32(vt_other + 0x44, getter);
                for i in 0..4u32 {
                    let rider = DATA + 0x3000 + i * 0x1000;
                    let ctl = DATA + 0x8000 + i * 0x100;
                    c.bus.write32(ai + AI_RIDERS + 4 * i, rider);
                    c.bus.write32(rider + IFACE, vt_other);
                    let attacking = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    };
                    c.bus.write32(rider + IFACE + 0x14, attacking);
                    c.bus.write32(
                        my_rider + i * DIST_STRIDE + DIST,
                        r.f32(400.0) & 0x7fff_ffff,
                    );
                    c.bus.write32(rider + ANIM_CTL, ctl);
                    // Layer 1's animation id; its table entry's first word.
                    let id = if r.next_u32() % 8 == 0 { 0x1b6 } else { 1 + i };
                    c.bus.write32(ctl + 4, id);
                    let kind = if r.next_u32() % 2 == 0 {
                        13
                    } else {
                        r.next_u32() % 20
                    };
                    c.bus.write32(0x0044_6990 + (1 + i) * 0x1c, kind);
                }
                // Rand_NextGame's state.
                fill(c, &mut r, 0x004f_f030, 6);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | me as u64;
            },
        );
    }

    /// ComputerActorRider_FindJumpMarker: the track's marker query is a fake
    /// (forced count; a real return-0 method for "none"), the buffer it
    /// "fills" is written beforehand; jump markers at random places or
    /// absent, before and behind the rider; the real Path_GetPointAtDistance
    /// on a short path.
    #[test]
    fn computer_actor_rider_find_jump_marker_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::SP as STACK;
        covered(
            ee::s00::f_0010b980__generated,
            computer_actor_rider_find_jump_marker,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10b9_80, 1);
                let this = DATA;
                let rider = DATA + 0x1000;
                let track = DATA + 0x2000;
                let vt = DATA + 0x3000;
                let nodes = DATA + 0x4000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x5000);
                c.g[A2][0] = sx(DATA + 0x5004);
                c.g[A3][0] = sx(DATA + 0x5008);
                c.g[T0][0] = sx(DATA + 0x500c);
                c.g[T1][0] = sx(DATA + 0x5010);
                c.g[T2][0] = sx(DATA + 0x5014);
                c.bus.write32(this + CA_RIDER, rider);
                c.bus.write32(rider + RIDER_TRACK, track);
                c.bus.write32(rider + RIDER_TRACK_X, r.f32(50.0));
                c.bus.write32(
                    rider + RIDER_TRACK_DIST,
                    (100.0 + (r.next_u32() % 200) as f32).to_bits(),
                );
                c.bus.write32(track + TRACK_VTABLE, vt);
                let delta = (r.next_u32() % 8) * 4;
                c.bus.write16(vt + 8, delta as u16);
                // Path: start point, node count, nodes.
                for k in 0..3 {
                    c.bus.write32(track + 0xc + 4 * k, r.f32(100.0));
                }
                c.bus.write32(track + 8, r.next_u32() % 3);
                c.bus.write32(track + 0x18, nodes);
                for k in 0..12 {
                    c.bus.write32(nodes + 4 * k, r.f32(400.0));
                }
                c.bus
                    .write32(GAME_GP - GP_MARKER_SPEED_UNIT, 27.777_78f32.to_bits());
                if seed % 9 == 0 {
                    c.bus.write32(vt + 0xc, 0x0011_25b8); // Rider_ReturnZero: none
                } else {
                    c.bus.write32(vt + 0xc, FAKE);
                    c.bus.write32(LOG_RESULTS, 1 + r.next_u32() % 12);
                }
                // The marker records the query writes (buffer at the new sp).
                let buf = STACK - 0x180;
                let jump_at = if seed % 4 == 0 { 99 } else { r.next_u32() % 12 };
                for i in 0..12 {
                    let m = buf + 0x10 * i;
                    let ty = if i == jump_at {
                        0x10
                    } else {
                        r.next_u32() % 0x10
                    };
                    c.bus.write32(m, ty);
                    c.bus.write32(m + 4, r.next_u32() >> (r.next_u32() % 32));
                    c.bus
                        .write32(m + 8, (50.0 + (r.next_u32() % 300) as f32).to_bits());
                    c.bus
                        .write32(m + 0xc, (100.0 + (r.next_u32() % 400) as f32).to_bits());
                }
            },
        );
    }
    use super::*;

    // ------------------------------------------------------ AI_UpdateRiderRelations

    /// The back end's tables the rivalry lookup reads (a copy of the `ai`
    /// tests' fixture): player slots, the player table, rider records
    /// (characters mostly 0..9, some profiles -1), the rivalry table, the
    /// track, and the BE singletons.
    pub(crate) fn be_tables(c: &mut Cpu, r: &mut Rng) {
        use crate::ai::{player_table, RIVALRY};
        for p in 0..8 {
            c.bus.write32(0x0053_05b0 + 4 * p, r.next_u32() % 8);
        }
        fill(c, r, player_table::BASE, 8 * 0x1c / 4);
        let all_cpu = r.next_u32() % 5 == 0;
        for s in 0..8 {
            let v = if all_cpu || r.next_u32() % 2 == 0 {
                r.next_u32() | 0x8000_0000
            } else {
                r.next_u32() % 4
            };
            c.bus.write32(player_table::BASE + s * 0x1c + 0xc, v);
        }
        fill(c, r, 0x0053_5b20, 16 * 0x1c / 4);
        for s in 0..16 {
            let rec = 0x0053_5b20 + s * 0x1c;
            if r.next_u32() % 2 == 0 {
                c.bus.write32(rec + 0xc, u32::MAX);
            }
            let ch = if r.next_u32() % 8 == 0 {
                r.next_u32() as u8
            } else {
                (r.next_u32() % 10) as u8
            };
            c.bus.write8(rec + 0x11, ch);
        }
        for p in 0..3u32 {
            for b in 0..10u32 {
                for a in 0..10u32 {
                    c.bus.write8(
                        RIVALRY + p * 0x9b50 + b * 0xf88 + a * 3,
                        (r.next_u32() % 5) as u8,
                    );
                }
            }
        }
        c.bus.write32(0x0053_5c08, r.next_u32() % 40);
        fill(c, r, 0x0043_d950, 40 * 0x64 / 4);
        for t in 0..40 {
            c.bus
                .write32(0x0043_d950 + t * 0x64 + 0x54, r.next_u32() % 4);
        }
        // NewRace, NewPlayer, Aggression, Char singletons.
        for g in [0x1f3c, 0x1f30, 0x1ea4, 0x1f2c] {
            c.bus.write32(GAME_GP - g, DATA + 0xe_0000 + g * 0x10);
        }
    }

    /// The relation pass over up to six riders: the frame counter is a
    /// multiple of 6 for most seeds; rider count minus skipped is 0..6;
    /// riders track some others; getPos is a fake whose first eight
    /// results are positions with equal x / zero y often enough to reach
    /// every atan2 case (dx = 0 with dy = 0, > 0, < 0; dx < 0 with dy
    /// either sign); the rivalry levels come from the real lookup; the
    /// rivals pass runs for some seeds. Fakes may unwind; events are
    /// delivered at the loop heads.
    #[test]
    fn ai_update_rider_relations_matches() {
        use rider_relations::*;
        const AI: u32 = 0x00a0_0000;
        const VT: u32 = 0x00a0_1000;
        const POS: u32 = 0x00a0_2000;
        const RIDER0: u32 = 0x00a1_0000;
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x2341, 8);
            be_tables(c, &mut r);
            for (off, v) in [
                (LIT_MIN_DIST, -50000.0f32),
                (LIT_MAX_DIST, 50000.0),
                (LIT_PI, std::f32::consts::PI),
                (LIT_NEG_HALF_PI, -std::f32::consts::FRAC_PI_2),
                (LIT_HALF_PI, std::f32::consts::FRAC_PI_2),
                (LIT_INV_TWO_PI, 0.159_154_94),
                (LIT_TWO_PI, std::f32::consts::TAU),
            ] {
                c.bus.write32(GAME_GP - off, v.to_bits());
            }
            // Positions the unforced fakes return (DATA + 0..0x3fff0).
            for k in 0..0x4000u32 {
                let a = DATA + 0x10 * k;
                c.bus.write32(a, r.f32(500.0));
                c.bus.write32(a + 4, r.f32(500.0));
            }
            // The first eight getPos results.
            for n in 0..8u32 {
                let x = [0.0f32, 1.0, -2.0][(r.next_u32() % 3) as usize];
                let y = [0.0f32, 3.0, -1.5][(r.next_u32() % 3) as usize];
                put_vec(c, POS + 0x10 * n, [x.to_bits(), y.to_bits(), r.f32(9.0), 0]);
                c.bus.write32(LOG_RESULTS + 4 * n, POS + 0x10 * n);
            }
            c.bus
                .write16(VT + GET_POS_SLOT, (r.next_u32() % 8 * 4) as u16);
            c.bus.write32(VT + GET_POS_SLOT + 4, FAKE);
            let frame = 6 * (r.next_u32() % 100)
                + if seed % 7 == 3 {
                    1 + r.next_u32() % 5
                } else {
                    0
                };
            c.bus.write32(AI + FRAME, frame);
            c.bus.write32(AI + 0x74, 0); // race positions: nothing to do
            c.bus.write32(AI + COUNT, r.next_u32() % 7);
            c.bus
                .write32(AI + SKIP, if r.next_u32() % 4 == 0 { 1 } else { 0 });
            c.bus.write32(AI + 0x80, (r.next_u32() % 3 == 0) as u32);
            for i in 0..6 {
                let rider = RIDER0 + 0x1000 * i;
                c.bus.write32(AI + RIDERS + 4 * i, rider);
                fill(c, &mut r, rider, 0x100 / 4);
                for j in 0..6 {
                    let tracked = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 3
                    };
                    c.bus.write32(rider + j * INFO_SIZE + INFO_ACTIVE, tracked);
                }
                c.bus.write32(rider + POS_OBJ, VT);
            }
            c.g[A0][0] = AI as u64;
            schedule(c, &mut r, seed, 0x0010_f560)
        };
        check_events(
            ee::s00::f_0010f560__generated,
            ai_update_rider_relations,
            300,
            &setup,
        );
    }
    use super::*;
    use super::*;
    pub(crate) use crate::game::ai::ai::tests::oc258_w32;

    /// RiderBase_SaveState: a stream whose methods are fakes; the real
    /// sub-state savers run on zeroed objects (their writes go to the fake
    /// stream); targets present or not; path indices found or not.
    #[test]
    fn rider_base_save_state_matches() {
        check(
            ee::s00::f_00111ac0__generated,
            rider_base_save_state,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0011_1ac0, 40);
                c.pc = 0x0011_1ac0;
                let stream = DATA + 0x10;
                let vt = vtable(c, &mut r, &[0x8]);
                oc258_w32(c, stream, vt);
                let this = DATA + 0x20_0000;
                crate::difftest::fill(c, &mut r, this, 0xe00 / 4);
                let rider = DATA + 0x21_0000;
                oc258_w32(c, this + RB_RIDER, rider);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
                // Zeroed part objects (anim, model, air predictor, metrix).
                for (k, f) in [
                    RB_RIDER_ANIM,
                    RB_RIDER_MODEL,
                    RB_RIDER_AIR_PREDICTOR,
                    RB_RIDER_METRIX,
                ]
                .into_iter()
                .enumerate()
                {
                    oc258_w32(c, rider + f, DATA + 0x22_0000 + 0x1000 * k as u32);
                }
                // The anim's six sequence lists (8 bytes each, `count <= 0`).
                oc258_w32(c, DATA + 0x22_0000 + 0x50, DATA + 0x22_8000);
                // Targets: `*field` null or an object with an id.
                for (k, f) in [RB_RIDER_TARGET0, RB_RIDER_TARGET1].into_iter().enumerate() {
                    let cell = DATA + 0x23_0000 + 0x400 * k as u32;
                    let obj = cell + 0x100;
                    oc258_w32(c, rider + f, cell);
                    let present = (seed >> k) & 1 != 0;
                    oc258_w32(c, cell, if present { obj } else { 0 });
                    oc258_w32(c, obj + RB_TARGET_ID, r.next_u32());
                }
                oc258_w32(c, rider + RB_RIDER_BLOCK0, r.next_u32());
                oc258_w32(c, rider + RB_RIDER_BLOCK1, r.next_u32());
                // Path data: n track paths of 0x3c bytes (count +0x10, array +0x14)
                // and n AI paths of 0x40 bytes (count +0x8, array +0xc); the
                // rider's paths are one of them, a stranger, or null.
                let n = seed % 5;
                let tracks = DATA + 0x24_0000;
                let ais = DATA + 0x25_0000;
                oc258_w32(c, RB_PATH_DATA + 0x10, n);
                oc258_w32(c, RB_PATH_DATA + 0x14, tracks);
                oc258_w32(c, RB_PATH_DATA + 0x8, n);
                oc258_w32(c, RB_PATH_DATA + 0xc, ais);
                for (f, base, stride) in [
                    (RB_RIDER_TRACK_PATH, tracks, 0x3c),
                    (RB_RIDER_AI_PATH, ais, 0x40),
                ] {
                    let v = match r.next_u32() % 3 {
                        0 => 0,
                        1 => base + stride * (r.next_u32() % 5),
                        _ => r.next_u32(),
                    };
                    oc258_w32(c, rider + f, v);
                }
            },
        );
    }

    /// AI_UpdateRacePositions over 0..8 riders: rank mode 0 (early out),
    /// 1 (distance), 2 (score) or another value (keys left as stack
    /// junk); some riders not loaded (early out), finished or not; the
    /// stored positions sometimes already equal the new rank. The audio
    /// notification runs for real up to BELibrary_GetInterface, whose
    /// interface call is a fake (which may unwind), and stops at the game
    /// module's race-type check. Events are delivered at both loop heads.
    #[test]
    fn ai_update_race_positions_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{Rng, GAME_GP};
        use race_positions::*;
        const AI: u32 = 0x00a0_0000;
        const RIDER0: u32 = 0x00a1_0000;
        const SCORE0: u32 = 0x00a2_0000;
        const IFACE: u32 = 0x00a3_0000;
        const IFACE_VT: u32 = 0x00a3_0100;
        const BELIB: u32 = 0x00a3_0200;
        const WORLD: u32 = 0x00a3_0400;
        const GAME_MODULE: u32 = 0x00a3_0800;
        const AUDIO: u32 = 0x00a3_1000;
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r: Rng = machine(c, seed, 0x0f99_8000, 6);
            let mode = if seed % 9 == 0 {
                0
            } else {
                [1, 1, 2, 2, 3][(r.next_u32() % 5) as usize]
            };
            let count = r.next_u32() % 9;
            c.bus.write32(AI + AI_RANK_MODE, mode);
            c.bus.write32(AI + AI_COUNT, count);
            let not_loaded = if seed % 11 == 5 { r.next_u32() % 8 } else { 99 };
            for i in 0..8u32 {
                let rider = RIDER0 + 0x1000 * i;
                let score = SCORE0 + 0x400 * i;
                c.bus.write32(AI + AI_RIDERS + 4 * i, rider);
                c.bus
                    .write32(rider + 0x880, if i == not_loaded { 3 } else { 7 });
                c.bus.write32(
                    rider + RIDER_FINISHED,
                    if r.next_u32() % 3 == 0 {
                        1 + r.next_u32() % 5
                    } else {
                        0
                    },
                );
                c.bus.write32(rider + RIDER_RACE_TIME, r.f32(20.0));
                c.bus.write32(rider + 0x4d0, r.f32(3000.0));
                let pos = if r.next_u32() % 2 == 0 {
                    i
                } else {
                    r.next_u32() % 8
                };
                c.bus.write32(rider + RIDER_POSITION, pos);
                c.bus.write32(rider + RIDER_SCORE_OBJ, score);
                c.bus.write32(score + SCORE_POINTS, r.next_u32() % 1000);
            }
            // SSXAudioSystem_GetInstance and OnRacePositionChange's path.
            c.bus.write32(GAME_GP + 0x410, AUDIO);
            c.bus.write32(GAME_GP - 0x1f04, BELIB);
            c.bus.write32(GAME_GP - 0x1efc, 0);
            c.bus.write32(0x0053_08a8, IFACE);
            c.bus.write32(IFACE + 4, IFACE_VT);
            c.bus.write16(IFACE_VT + 8, 0);
            c.bus.write32(IFACE_VT + 0xc, FAKE);
            c.bus.write8(0x0053_5c10, 0);
            c.bus.write32(GAME_GP - 0x848, WORLD);
            c.bus.write32(WORLD + 0x84, GAME_MODULE);
            c.bus.write32(GAME_MODULE + 0x214, 0);
            c.g[A0][0] = AI as u64;
            schedule(c, &mut r, seed, 0x0010_f998)
        };
        check_events(
            ee::s00::f_0010f998__generated,
            ai_update_race_positions,
            300,
            &setup,
        );
    }
    use super::*;
    use super::*;

    /// Rider_SelectAIPath: standing still or moving (look-ahead point), 0..4
    /// AI paths of 1..4 segments around the rider (real
    /// PathData_FindNearAIPaths / Path_GetClosestPoint / Path_GetLength /
    /// Path_GetPointAtDistance / Path_GetClosestPointFrom on the game's
    /// `.lit4` constants), the current path among them or not, keep flag on
    /// and off, and the AI controller's two virtual methods as fakes ("uses
    /// weights" forced to 1 or not, a random weight), some unwinding.
    #[test]
    fn rider_select_ai_path_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::LOG_RESULTS;
        use crate::game::worldpainter::quadtree::tests::put_vtable;
        const SELF: u32 = DATA + 0x20_0000;
        const VT: u32 = DATA + 0x20_2000;
        const HINT: u32 = DATA + 0x20_2100;
        const PATHS: u32 = DATA + 0x20_4000;
        const SEGS: u32 = DATA + 0x20_5000;
        check(
            ee::s00::f_00112a50__generated,
            rider_select_ai_path,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_2a50, 4);
                elf_image(c);
                c.pc = 0x0011_2a50;
                let f = |v: f32| v.to_bits();
                let uf = |r: &mut Rng, lo: f32, hi: f32| {
                    lo + (r.next_u32() % 10_000) as f32 / 10_000.0 * (hi - lo)
                };
                // The rider.
                let pos = [
                    uf(&mut r, -100.0, 100.0),
                    uf(&mut r, -100.0, 100.0),
                    uf(&mut r, -10.0, 10.0),
                ];
                put_vec(c, SELF + RSA_POS, [f(pos[0]), f(pos[1]), f(pos[2]), f(1.0)]);
                let vel = if seed % 4 == 0 {
                    [0; 4]
                } else {
                    [
                        f(uf(&mut r, -30.0, 30.0)),
                        f(uf(&mut r, -30.0, 30.0)),
                        f(uf(&mut r, -3.0, 3.0)),
                        0,
                    ]
                };
                put_vec(c, SELF + RSA_VEL, vel);
                c.bus.write32(SELF + RSA_AI, VT);
                put_vtable(c, &mut r, VT, &[RSA_VT_USES_WEIGHTS, RSA_VT_PATH_WEIGHT]);
                c.bus.write32(SELF + RSA_AI_PATH_HINT, HINT);
                for n in 0..8 {
                    c.bus.write32(LOG_RESULTS + 4 * n, r.next_u32() % 2);
                }
                // The AI paths.
                let count = seed % 5;
                c.bus.write32(RSA_PATH_DATA + 8, count);
                c.bus.write32(RSA_PATH_DATA + 0xc, PATHS);
                for i in 0..4u32 {
                    let path = PATHS + 0x40 * i;
                    let segs = SEGS + 0x100 * i;
                    let n = 1 + r.next_u32() % 4;
                    c.bus.write32(path + 8, n);
                    let start = [
                        pos[0] + uf(&mut r, -150.0, 150.0),
                        pos[1] + uf(&mut r, -150.0, 150.0),
                        pos[2],
                    ];
                    for k in 0..3 {
                        c.bus.write32(path + 0xc + 4 * k, f(start[k as usize]));
                    }
                    c.bus.write32(path + 0x18, segs);
                    // Bounds (min, max): around the start, often holding the rider.
                    let reach = if r.next_u32() % 2 == 0 { 400.0 } else { 50.0 };
                    for k in 0..3 {
                        c.bus
                            .write32(path + 0x1c + 4 * k, f(start[k as usize] - reach));
                        c.bus
                            .write32(path + 0x28 + 4 * k, f(start[k as usize] + reach));
                    }
                    for k in 0..n {
                        let a = uf(&mut r, 0.0, 6.28);
                        put_vec(
                            c,
                            segs + 0x10 * k,
                            [
                                f(a.cos()),
                                f(a.sin()),
                                f(uf(&mut r, -0.2, 0.2)),
                                f(uf(&mut r, 100.0, 700.0)),
                            ],
                        );
                    }
                }
                let current = match seed % 3 {
                    0 => 0,
                    1 => PATHS + 0x40 * (r.next_u32() % 4),
                    _ => PATHS,
                };
                c.bus.write32(SELF + RSA_AI_PATH, current);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = ((seed / 3) % 2) as u64;
            },
        );
    }

    /// Rider_SelectRacePath with its real callees (look-ahead distance,
    /// PathData_FindNearTrackPaths, Path_GetClosestPoint,
    /// Path_GetPointAtDistance) on the game's literals: 0..4 track paths of
    /// 1..4 segments whose bounding boxes contain the probe or not; the
    /// rider standing still or moving, its current path one of them (with
    /// a track distance near or far from the closest point's) or another;
    /// skipping the current path or not.
    #[test]
    fn rider_select_race_path_matches() {
        crate::difftest::check(
            ee::s00::f_001127f0__generated,
            rider_select_race_path,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_27f0, 0);
                elf_image(c);
                let me = DATA + 0x1000;
                let ai_state = DATA + 0x3000;
                let tracks = DATA + 0x1_0000;
                let n = match seed % 6 {
                    0 => 0,
                    5 => 4,
                    k => k,
                };
                c.bus.write32(SELECT_PATH_DATA + 0x10, n);
                c.bus.write32(SELECT_PATH_DATA + 0x14, tracks);
                for i in 0..4u32 {
                    let t = tracks + 0x3c * i;
                    let segs = DATA + 0x2_0000 + 0x100 * i;
                    c.bus.write32(t + 8, 1 + r.next_u32() % 4);
                    for k in 0..3 {
                        c.bus.write32(t + 0xc + 4 * k, r.f32(50.0));
                    }
                    c.bus.write32(t + 0x18, segs);
                    let inside = r.next_u32() % 4 != 0;
                    for k in 0..3 {
                        let (lo, hi) = if inside {
                            (-200.0f32, 200.0f32)
                        } else {
                            (300.0, 400.0)
                        };
                        c.bus.write32(t + 0x1c + 4 * k, lo.to_bits());
                        c.bus.write32(t + 0x28 + 4 * k, hi.to_bits());
                    }
                    c.bus
                        .write32(t + 0x38, ((r.next_u32() % 4000) as f32).to_bits());
                    for k in 0..4 {
                        let a = (r.next_u32() % 628) as f32 / 100.0;
                        let z = (r.next_u32() % 100) as f32 / 400.0;
                        let seg = segs + 0x10 * k;
                        c.bus.write32(seg, a.cos().to_bits());
                        c.bus.write32(seg + 4, a.sin().to_bits());
                        c.bus.write32(seg + 8, z.to_bits());
                        c.bus.write32(
                            seg + 0xc,
                            (10.0 + (r.next_u32() % 5000) as f32 / 100.0).to_bits(),
                        );
                    }
                }
                put_vec(
                    c,
                    me + SELECT_PATH_POS,
                    [r.f32(60.0), r.f32(60.0), r.f32(60.0), 1.0f32.to_bits()],
                );
                let vel = if seed % 4 == 0 {
                    [0, 0, 0, 0]
                } else {
                    [r.f32(30.0), r.f32(30.0), r.f32(30.0), 0]
                };
                put_vec(c, me + SELECT_PATH_VEL, vel);
                let current = match r.next_u32() % 3 {
                    0 => DATA + 0x5000,
                    _ => tracks + 0x3c * (r.next_u32() % 3),
                };
                c.bus.write32(me + SELECT_PATH_CURRENT, current);
                c.bus.write32(me + SELECT_PATH_AI_STATE, ai_state);
                c.bus.write32(ai_state + 0x14, r.next_u32());
                // Near (within 1592 of the current path's base) or far.
                let base = f32::from_bits(c.bus.read32(current + 0x38));
                let near = if seed % 3 == 0 {
                    base + 5000.0
                } else {
                    base - 20.0
                };
                c.bus.write32(me + SELECT_PATH_TRACK_DIST, near.to_bits());
                c.g[A0][0] = sx(me);
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
                c.pc = 0x0011_27f0;
            },
        );
    }
    use super::*;
    use super::*;
    use super::*;
    use super::*;

    /// `RiderBase_TriggerDynamicSpray`: a rider whose spray effect (+0xb40)
    /// holds random state (strength, short range, queued position near or
    /// far), random burst position/direction, kind and speed; the real
    /// cDynamicSprayFX_QueueBurst runs.
    #[test]
    fn rider_base_trigger_dynamic_spray_matches() {
        use crate::difftest::{check, put_vec};
        const RIDER: u32 = DATA + 0x1000;
        const SPRAY: u32 = RIDER + RIDER_DYNAMIC_SPRAY;
        const POS: u32 = DATA + 0x400;
        const DIR: u32 = DATA + 0x410;
        check(
            ee::s00::f_00111aa0__generated,
            rider_base_trigger_dynamic_spray,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_1aa0, 1);
                c.pc = 0x0011_1aa0;
                fill(c, &mut r, SPRAY, 0x80);
                let spread = [30.0, 120.0, 300.0, 1000.0][seed as usize % 4];
                c.bus.write32(
                    SPRAY + 0xe0,
                    r.f32(if seed % 5 == 0 { 100.0 } else { 10.0 }),
                );
                c.bus.write32(
                    SPRAY + 0x124,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(SPRAY, RIDER); // its rider, with random carve flags
                fill(c, &mut r, RIDER + 0xac0, 0x12);
                put_vec(
                    c,
                    SPRAY + 0x100,
                    [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                put_vec(
                    c,
                    SPRAY + 0xf0,
                    [r.f32(spread), r.f32(spread), r.f32(spread), r.next_u32()],
                );
                put_vec(
                    c,
                    POS,
                    [r.f32(spread), r.f32(spread), r.f32(spread), r.next_u32()],
                );
                put_vec(
                    c,
                    DIR,
                    [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                c.g[A0][0] = sx(RIDER);
                c.g[A1][0] = sx(POS);
                c.g[A2][0] = sx(DIR);
                c.g[A3][0] = r.next_u32() as u64;
                c.f[12] = if seed % 7 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(50.0)
                };
            },
        );
    }

    // ------------------------------------------------ RiderBase_EnterSubState

    /// `RiderBase_EnterSubState`: the real `.rodata` switch table (from the
    /// ELF image) picks the arm from `rider + 0xde4`; every sub-state 0..13
    /// and out-of-range ids (14, large, negative). The rider base and its
    /// sub-objects are zeroed, with a zeroed world and BELibrary singleton,
    /// so the real `Enter` methods take their short paths; null method
    /// pointers they call through land in the fakes.
    #[test]
    fn rider_base_enter_sub_state_matches() {
        check(
            ee::s00::f_00111630__generated,
            rider_base_enter_sub_state,
            34,
            &|c, seed| {
                machine(c, seed, 0x0011_1630, 0);
                elf_image(c);
                c.pc = 0x0011_1630;
                c.g[ssx3_ee_rt::cpu::reg::SP][0] = (crate::difftest::SP - 0x200) as u64;
                for i in 0..0x400 {
                    c.bus.write32(DATA + i * 4, 0);
                }
                c.bus.write32(GAME_GP - 0x848, DATA + 0x1000); // world
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x1100); // BELibrary
                                                                // Sub states 2, 6, 11 and 12 are not driven: their real Enter
                                                                // methods run away on the zeroed rider (AnimController_PlayAnimImpl
                                                                // at 0x312138, the zeroed-heap consolidate at 0x31ef40). Their arms
                                                                // are the same table call as the others.
                let state = match seed % 17 {
                    2 | 6 | 11 | 12 => 13,
                    s @ 0..=13 => s,
                    14 => 14,
                    15 => 0x100,
                    _ => 0xffff_fff0,
                };
                c.bus.write32(DATA + RIDER_BASE_SUB_STATE, state);
                if state < 14 {
                    assert_ne!(c.bus.read32(ENTER_SUB_STATE_TABLE + 4 * state), 0);
                }
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// GetPathClassWeight: every preferred class (0, 1, 2, other) with each
    /// flag zero or set (full 64-bit values), and a random weight literal.
    #[test]
    fn computer_actor_rider_get_path_class_weight_matches() {
        check(
            ee::s00::f_0010d870__generated,
            computer_actor_rider_get_path_class_weight_2,
            300,
            &|c, seed| {
                let mut r = Rng(0x10d8_7000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0010_d870;
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = sx(DATA);
                let class = match seed % 5 {
                    4 => r.next_u32() as u16,
                    k => k as u16,
                };
                c.bus.write16(DATA + CAR_PATH_CLASS, class);
                c.bus
                    .write32(GAME_GP - CAR_PATH_WEIGHT_STEP_GP, r.edge_f32());
                for (i, reg) in [A1, A2, A3].into_iter().enumerate() {
                    c.g[reg][0] = if (seed / 5 >> i) & 1 == 0 {
                        0
                    } else if r.next_u32() % 2 == 0 {
                        1
                    } else {
                        (r.next_u32() as u64) << 32 | r.next_u32() as u64
                    };
                }
            },
        );
    }

    /// RiderBase_ExitSubState: every sub state 0..13 plus out-of-range ones,
    /// through the game's jump table (ELF image), with the real exit
    /// handlers on short paths: every sub-object points at one actor whose
    /// animation controller has layer 1 idle (`0x1b6`: the cruise and
    /// natural-air exits stop there, the spin exit's fades find empty
    /// lists), and the world's frame counter is odd (the wipeout exit
    /// returns at once). Some seeds zero the table (fake jump out, which
    /// unwinds on some of them).
    #[test]
    fn rider_base_exit_sub_state_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::machine;
        const RIDER: u32 = DATA;
        const ACTOR: u32 = DATA + 0x1_0000;
        const CTRL: u32 = DATA + 0x2_0000;
        const WORLD: u32 = DATA + 0x3_0000;
        check(
            ee::s00::f_00111578__generated,
            rider_base_exit_sub_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_1578, 1);
                elf_image(c);
                c.pc = 0x0011_1578;
                let states = [
                    0u32,
                    1,
                    2,
                    3,
                    4,
                    5,
                    6,
                    7,
                    8,
                    9,
                    10,
                    11,
                    12,
                    13,
                    14,
                    0xffff_ffff,
                ];
                c.bus.write32(
                    RIDER + RIDER_SUB_STATE,
                    states[(seed as usize) % states.len()],
                );
                if seed % 17 == 3 {
                    for i in 0..14 {
                        c.bus.write32(RIDER_EXIT_SUB_STATE_TABLE + 4 * i, 0);
                    }
                }
                // Each handler's link to the rider actor.
                c.bus.write32(RIDER + 0x1c0, ACTOR); // cruise: +0
                c.bus.write32(RIDER + 0x1d0 + 0x14, ACTOR); // board press
                c.bus.write32(RIDER + 0x210 + 0x14, ACTOR); // natural air
                c.bus.write32(RIDER + 0x230 + 0x58, ACTOR); // spin
                c.bus.write32(RIDER + 0x2b0 + 0x8, ACTOR); // 7
                c.bus.write32(RIDER + 0x2c0 + 0x80, ACTOR); // wipeout
                c.bus.write32(RIDER + 0x370 + 0x14, ACTOR); // 11
                c.bus.write32(ACTOR + 0x784, CTRL);
                c.bus.write32(CTRL + 4, 0x1b6);
                // World -> game -> race: an odd frame counter.
                c.bus.write32(GAME_GP - 0x848, WORLD);
                c.bus.write32(WORLD + 0x84, WORLD + 0x100);
                c.bus.write32(WORLD + 0x100 + 0xc, WORLD + 0x200);
                c.bus
                    .write32(WORLD + 0x200 + 8, 2 * (r.next_u32() & 0xffff) + 1);
                c.g[A0][0] = sx(RIDER);
                c.g[A1][0] = (r.next_u32() % 6) as u64;
            },
        );
    }
    use super::*;
    use super::*;
    use super::*;
    use crate::difftest::RA as SENTINEL;

    /// `ComputerActorRider_PickUberTrick`: no air time (early -1) or some;
    /// a time budget in [0, 20] s against random animation trigger times
    /// (some parts missing: -1.0) and random use counts, so tricks fit or
    /// not, are less used or not, and some riders have no animation for a
    /// trick (0x1b6). The real BELibrary lookups (interface getter a fake,
    /// sometimes unwinding), Rider_GetTrickSkillFactor, BEBAGTInt_GetTrickAnim
    /// / GetTrickField0 (fixture as in their own tests) and
    /// AnimController_GetCurrentAnimTriggerTime run; the rider-index and
    /// animation-index virtuals are the real 0-returning leaf 0x1406c0.
    #[test]
    fn computer_actor_rider_pick_uber_trick_matches() {
        use crate::difftest::fakes::{vtable, LOG_RESULTS};
        const THIS: u32 = DATA;
        const RIDER: u32 = DATA + 0x2000;
        const IFACE_VT: u32 = DATA + 0x3000;
        const CTRL: u32 = DATA + 0x3100;
        const CTRL_VT: u32 = DATA + 0x3200;
        const TRIGGERS: u32 = DATA + 0x4000;
        const BANKS: u32 = DATA + 0x6000;
        const BANK: u32 = DATA + 0x6100;
        const ENTRIES: u32 = DATA + 0x6200;
        const TIMES: u32 = DATA + 0x6400;
        const BAGT: u32 = DATA + 0x7000;
        const ANIMS: u32 = DATA + 0x8000;
        const ZERO_LEAF: u32 = 0x0014_06c0;
        const RIDER_PROFILES: u32 = 0x0053_05b0;
        const PROFILES: u32 = 0x0053_5b20;
        const SLOTS: u32 = 0x0053_0ec0;
        const TRICK_TABLE: u32 = 0x0045_aeb8;
        covered(
            ee::s00::f_0010cf68__generated,
            computer_actor_rider_pick_uber_trick,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_cf68, 2);
                elf_image(c);
                c.pc = 0x0010_cf68;
                // The BE singletons; the library's interface getter is a
                // fake (the BAGT interface comes back as BAGT).
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x6_0000);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                for g in [0x1f3c, 0x1f30, 0x1f2c, 0x1ea4] {
                    c.bus.write32(GAME_GP - g, DATA + 0xe_0000 + g * 0x10);
                }
                let lib = DATA + 0x5_0000;
                c.bus.write32(0x0053_08a8, lib);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(lib + 4, vt);
                c.bus.write32(LOG_RESULTS, BAGT);
                // Player profiles and records (skill factor, trick slots).
                for p in 0..8 {
                    c.bus.write32(RIDER_PROFILES + 4 * p, r.next_u32() % 8);
                }
                for s in 0..8 {
                    let rec = PROFILES + s * 0x1c;
                    fill(c, &mut r, rec, 7);
                    if r.next_u32() % 3 == 0 {
                        c.bus.write32(rec + 0xc, u32::MAX);
                    }
                    c.bus.write8(rec + 0x11, (r.next_u32() % 10) as u8);
                    let chars = [0u8, 0x14, 0x18, 0x19, 0x1c, 5, 30, 0xf0];
                    c.bus.write8(rec + 0x12, chars[(r.next_u32() % 8) as usize]);
                }
                for i in 0..0x4000u32 / 4 {
                    let w = r.next_u32() & 0x0707_0707;
                    c.bus.write32(SLOTS + 4 * i, w);
                }
                for t in 0..15u32 {
                    let e = TRICK_TABLE + 0x14 * t;
                    c.bus.write16(e, r.next_u32() as u16);
                    c.bus
                        .write16(e + 0xc, if r.next_u32() % 3 == 0 { 0 } else { 8 });
                    let anims = ANIMS + 0x100 * t;
                    c.bus.write32(e + 0x10, anims);
                    fill(c, &mut r, anims, 0x40);
                }
                // The rider.
                c.bus.write32(THIS + UBER_PICK_RIDER, RIDER);
                let air = match seed % 7 {
                    0 => 0,
                    1 => r.f32(1.0) | 0x8000_0000,
                    _ => (1.0f32 + (r.next_u32() % 100) as f32 / 10.0).to_bits(),
                };
                c.bus.write32(RIDER + UBER_PICK_AIR_TIME, air);
                c.bus.write32(RIDER + UBER_PICK_LEVEL, r.next_u32() % 8);
                c.bus.write32(RIDER + UBER_PICK_TRICK_IFACE, IFACE_VT);
                c.bus
                    .write32(IFACE_VT + UBER_PICK_IFACE_INDEX_SLOT + 4, ZERO_LEAF);
                c.bus.write32(RIDER + SPIN_ENTER_ANIM_CTRL, CTRL);
                c.bus.write32(RIDER + SPIN_ENTER_PLAYER, r.next_u32() % 8);
                let stats = match r.next_u32() % 3 {
                    0 => 0,
                    1 => r.next_u32() % 40,
                    _ => r.next_u32(),
                };
                c.bus.write32(RIDER + SPIN_ENTER_CHARACTER, stats);
                for i in 0..15 {
                    c.bus
                        .write32(THIS + UBER_PICK_USES + 4 * i, r.next_u32() % 20);
                }
                // The animation controller: index 0 -> bank 0, entry e.
                c.bus.write32(CTRL + 0x58, CTRL_VT);
                c.bus.write32(CTRL_VT + 0x24, ZERO_LEAF);
                c.bus.write32(GAME_GP + 0xd8c, TRIGGERS);
                let entry = seed % 3;
                c.bus.write32(TRIGGERS + 0x1030, entry << 8);
                c.bus.write32(GAME_GP + 0xd08, BANKS);
                c.bus.write32(BANKS, BANK);
                c.bus.write32(BANK + 0x8, ENTRIES);
                c.bus.write32(BANK + 0x10, TIMES);
                for e in 0..3 {
                    c.bus
                        .write16(ENTRIES + 0x14 * e + 0x10, (r.next_u32() % 64) as u16);
                    c.bus
                        .write16(ENTRIES + 0x14 * e + 0x12, (r.next_u32() % 4) as u16);
                }
                for i in 0..0x100 {
                    c.bus.write32(TIMES + 4 * i, r.next_u32() % 600);
                }
                c.f[12] = r.f32(20.0) & 0x7fff_ffff;
                c.g[A0][0] = sx(THIS);
            },
        );
    }

    /// A `RiderBase` at DATA+0x200000 that `RiderBase_SaveState` can save
    /// (copied from `rider_base_save_state_matches`: zeroed part objects,
    /// targets present or not, path indices found or not) and a stream at
    /// DATA+0x10 whose write method (slot +0x8) is a fake. Returns
    /// `(rider_base, stream)`.
    pub(crate) fn oc286_rider_base_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) -> (u32, u32) {
        use crate::difftest::fakes::vtable;
        use crate::game::ai::computer::{
            RB_PATH_DATA, RB_RIDER, RB_RIDER_AIR_PREDICTOR, RB_RIDER_AI_PATH, RB_RIDER_ANIM,
            RB_RIDER_BLOCK0, RB_RIDER_BLOCK1, RB_RIDER_METRIX, RB_RIDER_MODEL, RB_RIDER_TARGET0,
            RB_RIDER_TARGET1, RB_RIDER_TRACK_PATH, RB_TARGET_ID,
        };
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        let stream = DATA + 0x10;
        let vt = vtable(c, r, &[0x8]);
        w(c, stream, vt);
        let this = DATA + 0x20_0000;
        crate::difftest::fill(c, r, this, 0xf40 / 4);
        let rider = DATA + 0x21_0000;
        w(c, this + RB_RIDER, rider);
        for (k, f) in [
            RB_RIDER_ANIM,
            RB_RIDER_MODEL,
            RB_RIDER_AIR_PREDICTOR,
            RB_RIDER_METRIX,
        ]
        .into_iter()
        .enumerate()
        {
            w(c, rider + f, DATA + 0x22_0000 + 0x1000 * k as u32);
        }
        w(c, DATA + 0x22_0000 + 0x50, DATA + 0x22_8000);
        for (k, f) in [RB_RIDER_TARGET0, RB_RIDER_TARGET1].into_iter().enumerate() {
            let cell = DATA + 0x23_0000 + 0x400 * k as u32;
            let obj = cell + 0x100;
            w(c, rider + f, cell);
            let present = (seed >> k) & 1 != 0;
            w(c, cell, if present { obj } else { 0 });
            let id = r.next_u32();
            w(c, obj + RB_TARGET_ID, id);
        }
        let b0 = r.next_u32();
        w(c, rider + RB_RIDER_BLOCK0, b0);
        let b1 = r.next_u32();
        w(c, rider + RB_RIDER_BLOCK1, b1);
        let n = seed % 5;
        let tracks = DATA + 0x24_0000;
        let ais = DATA + 0x25_0000;
        w(c, RB_PATH_DATA + 0x10, n);
        w(c, RB_PATH_DATA + 0x14, tracks);
        w(c, RB_PATH_DATA + 0x8, n);
        w(c, RB_PATH_DATA + 0xc, ais);
        for (f, base, stride) in [
            (RB_RIDER_TRACK_PATH, tracks, 0x3c),
            (RB_RIDER_AI_PATH, ais, 0x40),
        ] {
            let v = match r.next_u32() % 3 {
                0 => 0,
                1 => base + stride * (r.next_u32() % 5),
                _ => r.next_u32(),
            };
            w(c, rider + f, v);
        }
        (this, stream)
    }

    /// The real rider-base save, then the AI block write through the
    /// stream's fake write method (unwinding at any of the calls).
    #[test]
    fn computer_actor_rider_save_state_matches() {
        check(
            ee::s00::f_0010a898__generated,
            computer_actor_rider_save_state,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_a898, 41);
                let (this, stream) = oc286_rider_base_fixture(c, &mut r, seed);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
                c.pc = 0x0010_a898;
            },
        );
    }

    /// Rider velocities whose length falls below or above 833.33 (and edge
    /// values), the `.lit4` constants, a random pad word.
    #[test]
    fn computer_actor_rider_crouch_and_boost_matches() {
        check(
            ee::s00::f_0010c140__generated,
            computer_actor_rider_crouch_and_boost,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_c140, 0);
                let this = DATA;
                let rider = DATA + 0x400;
                let pad = DATA + 0x800;
                c.bus.write32(this + CAR_RIDER, rider);
                let mut comp = || {
                    if seed % 7 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(700.0)
                    }
                };
                let v = [comp(), comp(), comp(), comp()];
                crate::difftest::put_vec(c, rider + RIDER_VELOCITY, v);
                c.bus
                    .write32(GAME_GP - CROUCH_FULL_BELOW, 833.333_43f32.to_bits());
                c.bus
                    .write32(GAME_GP - CROUCH_FAST_SCALE, 0.97f32.to_bits());
                let word = r.next_u32();
                c.bus.write32(pad, word);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(pad);
                c.pc = 0x0010_c140;
            },
        );
    }

    pub(crate) const TW_AI: u32 = DATA + 0x1_0000;

    pub(crate) const TW_RIDER: u32 = DATA + 0x2_0000;

    pub(crate) const TW_RIDER_VT: u32 = DATA + 0x3_0000;

    pub(crate) const TW_ANIM: u32 = DATA + 0x3_1000;

    pub(crate) const TW_ANIM_VT: u32 = DATA + 0x3_1100;

    pub(crate) const TW_BE_LIB: u32 = DATA + 0x3_2000;

    pub(crate) const TW_BE_IFACE: u32 = DATA + 0x3_3000;

    pub(crate) const TW_BE_VT: u32 = DATA + 0x3_3100;

    /// What `BELibrary_GetInterface` answers (the BAGT interface).
    pub(crate) const TW_IFACE: u32 = DATA + 0x3_4000;

    /// `AnimController_GetCurrentAnimTriggerTime`'s tables (`gp+0xd8c`,
    /// `gp+0xd08`).
    pub(crate) const TT_BASE: u32 = DATA + 0x3_5000;

    pub(crate) const TT_PTRS: u32 = DATA + 0x3_7000;

    pub(crate) const TT_HDR: u32 = DATA + 0x3_7100;

    pub(crate) const TT_RECS: u32 = DATA + 0x3_7200;

    pub(crate) const TT_TIMES: u32 = DATA + 0x3_7400;

    /// The rider index the rider's virtual answers.
    pub(crate) const TW_IDX: u32 = DATA + 0x3_8000;

    /// Fake calls made, and the call index that unwinds.
    pub(crate) const TW_CALLS: u32 = DATA + 0x3_8004;

    pub(crate) const TW_UNWIND_AT: u32 = DATA + 0x3_8008;

    /// Uber trick animation lists (`trick.+0x10`).
    pub(crate) const TW_UBER_ANIMS: u32 = DATA + 0x3_9000;

    /// The BAGT trick table (0x14 bytes per trick).
    pub(crate) const TRICK_TABLE: u32 = 0x0045_aeb8;

    /// The current track (`BENewRaceInt_GetTrack`), 0xe = no uber tricks.
    pub(crate) const TRACK: u32 = 0x0053_5c08;

    /// `Rand_NextGame`'s state.
    pub(crate) const RAND_STATE: u32 = 0x004f_f030;

    /// The fake callees of the trick world: the rider's index query
    /// (`FAKE+0x10`), the animation controller's anim lookup (`FAKE+0x20`:
    /// ids below 8 map to themselves, others to 0x207, "no trigger time")
    /// and the BE library's interface slot (`FAKE+0x80`). Unwinds at the
    /// call index stored at [`TW_UNWIND_AT`].
    pub(crate) fn trick_world_hook(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(TW_CALLS);
        c.bus.write32(TW_CALLS, n + 1);
        c.g[V0][0] = match target.wrapping_sub(FAKE) {
            0x10 => c.bus.read32(TW_IDX) as u64,
            0x20 => {
                let anim = c.g[A1][0] as u32;
                if anim < 8 {
                    anim as u64
                } else {
                    0x207
                }
            }
            0x80 => TW_IFACE as u64,
            _ => 0,
        };
        c.g[A0][0] = 0x1234;
        c.cycles += 3;
        c.pc = if n == c.bus.read32(TW_UNWIND_AT) {
            0xdead_0000 | n << 4
        } else {
            ra
        };
    }

    /// An AI (`TW_AI`) with its rider, the BE library and BAGT interface,
    /// the trick table (some tricks without an animation, 0x1b6), uber
    /// animation lists, the animation trigger-time tables, the rand state,
    /// the trick counts and the roll / pick inputs, all chosen by `r`.
    pub(crate) fn trick_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let pick = |r: &mut Rng, v: &[u32]| v[(r.next_u32() as usize) % v.len()];
        let f = |x: f32| x.to_bits();
        c.hooks.miss = trick_world_hook;
        c.bus.write32(TW_CALLS, 0);
        let unwind = if seed % 5 == 0 {
            r.next_u32() % 40
        } else {
            u32::MAX
        };
        c.bus.write32(TW_UNWIND_AT, unwind);
        c.bus.write32(TW_IDX, r.next_u32() % 4);
        // BE library singleton and its interface slot.
        c.bus.write32(GAME_GP - 0x1f04, TW_BE_LIB);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(0x0053_08a8, TW_BE_IFACE);
        c.bus.write32(TW_BE_IFACE + 4, TW_BE_VT);
        c.bus.write16(TW_BE_VT + 8, 0);
        c.bus.write32(TW_BE_VT + 0xc, FAKE + 0x80);
        c.bus.write32(TRACK, pick(&mut *r, &[0xe, 3]));
        for i in 0..6 {
            c.bus.write32(RAND_STATE + 4 * i, r.next_u32());
        }
        // Trick skill factor: `lit * attribute + 1`.
        c.bus.write32(GAME_GP - 0x7960, f(0.5));
        c.bus.write8(0x0053_08d8 + 0xa, 2);
        // The trick table.
        for i in 0..16u32 {
            let t = TRICK_TABLE + 0x14 * i;
            c.bus
                .write16(t, pick(&mut *r, &[0, 1, 3, 5, 7, 9, 0x1b6]) as u16);
            c.bus
                .write16(t + 6, pick(&mut *r, &[0x1b6, 2, 4, 6, 8]) as u16);
            let uber = r.next_u32() % 2;
            c.bus.write16(t + 0xc, uber as u16);
            c.bus.write32(t + 0x10, TW_UBER_ANIMS + 0x10 * i);
            c.bus.write16(
                TW_UBER_ANIMS + 0x10 * i,
                pick(&mut *r, &[0x1b6, 3, 6]) as u16,
            );
        }
        // Trigger times: anim id i -> record i, 0..3 times of 0..60 frames.
        c.bus.write32(GAME_GP + 0xd8c, TT_BASE);
        c.bus.write32(GAME_GP + 0xd08, TT_PTRS);
        c.bus.write32(GAME_GP - 0x322c, f(1.0 / 30.0));
        c.bus.write32(TT_PTRS, TT_HDR);
        c.bus.write32(TT_HDR + 8, TT_RECS);
        c.bus.write32(TT_HDR + 0x10, TT_TIMES);
        for i in 0..8u32 {
            c.bus.write32(TT_BASE + 0x1030 + 4 * i, i << 8);
            c.bus.write16(TT_RECS + 0x14 * i + 0x10, (3 * i) as u16);
            c.bus
                .write16(TT_RECS + 0x14 * i + 0x12, (r.next_u32() % 4) as u16);
        }
        for j in 0..32u32 {
            c.bus.write16(TT_TIMES + 4 * j, (r.next_u32() % 60) as u16);
        }
        // The rider.
        let rider = TW_RIDER;
        c.bus.write32(TW_AI + 0x18, rider);
        c.bus.write32(rider + 0x6c0, TW_RIDER_VT);
        c.bus.write16(TW_RIDER_VT + 0x38, 0);
        c.bus.write32(TW_RIDER_VT + 0x3c, FAKE + 0x10);
        c.bus.write32(rider + 0x784, TW_ANIM);
        c.bus.write32(TW_ANIM + 0x58, TW_ANIM_VT);
        c.bus.write16(TW_ANIM_VT + 0x20, 0);
        c.bus.write32(TW_ANIM_VT + 0x24, FAKE + 0x20);
        c.bus
            .write32(rider + 0x2f0, pick(&mut *r, &[f(-1.0), 0, f(2.0), f(2.0)]));
        c.bus.write32(rider + 0x2f4, r.next_u32() % 8);
        c.bus.write32(rider + 0x86c, r.next_u32() % 4);
        c.bus.write32(rider + 0xb34, 1 + r.next_u32() % 3);
        c.bus.write32(rider + 0xb2c, r.next_u32() & 0xff);
        // The AI: roll skill, trick counts.
        let ai = TW_AI;
        c.bus
            .write32(ai + 0xdfc, pick(&mut *r, &[f(-1.0), 0, f(0.5), f(1.0)]));
        c.bus.write32(ai + 0xe20, pick(&mut *r, &[0, 0, 1]));
        for i in 0..15u32 {
            for base in [0xe7cu32, 0xeb8, 0xef4] {
                c.bus.write32(ai + base + 4 * i, r.next_u32() % 20);
            }
        }
    }

    /// `ComputerActorRider_ChooseTrick` on the trick world with the real
    /// rolls and pickers: skipped (+0xe20 below 1 unit), too low, exactly
    /// 100, each roll failing or succeeding and its pick found or not, and
    /// unwinds out of any of the callees.
    #[test]
    fn computer_actor_rider_choose_trick_matches() {
        check(
            ee::s00::f_0010cad8__generated,
            computer_actor_rider_choose_trick,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_cad8, 0);
                trick_world(c, &mut r, seed);
                let heights = [0.5f32, 0.75, 0.9, 1.5, 3.0, 10.0, 100.0, 40.0];
                c.f[12] = heights[(r.next_u32() % 8) as usize].to_bits();
                c.g[A0][0] = sx(TW_AI);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0010_cad8;
            },
        );
    }

    /// `ComputerActorRider_PickFittedTrick` on the trick world: tricks
    /// without an animation, airborne with and without uber animations,
    /// trigger times that fit or not, fewer or more earlier picks, no pick
    /// at all, and unwinds out of any callee.
    #[test]
    fn computer_actor_rider_pick_fitted_trick_matches() {
        check(
            ee::s00::f_0010cd20__generated,
            computer_actor_rider_pick_fitted_trick,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_cd20, 0);
                trick_world(c, &mut r, seed);
                let heights = [0.0f32, 0.5, 1.0, 1.5, 2.0, 3.0, 5.0, -1.0];
                c.f[12] = heights[(r.next_u32() % 8) as usize].to_bits();
                c.g[A0][0] = sx(TW_AI);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0010_cd20;
            },
        );
    }

    /// Event delivery at the trick loop head (0x10cd98).
    #[test]
    fn computer_actor_rider_pick_fitted_trick_events_match() {
        check_events(
            ee::s00::f_0010cd20__generated,
            computer_actor_rider_pick_fitted_trick,
            90,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_cd21, 0);
                trick_world(c, &mut r, 1 + seed % 4);
                c.bus.write32(TW_UNWIND_AT, u32::MAX);
                c.g[A0][0] = sx(TW_AI);
                c.g[RA][0] = SENTINEL as u64;
                let mode = schedule(c, &mut r, seed, 0x0010_cd20);
                c.f[12] = 2.0f32.to_bits();
                mode
            },
        );
    }
    use super::*;

    /// Rider_ReportRiderHit0: the real callees take short paths (no views:
    /// FindViewInRange gives -1 and the collision sound stops; speech volume
    /// 0 stops the speech); the riders' GetRiderIndex methods are fakes
    /// returning rider 1..2 and rival 1..3, and AddAggression runs on
    /// records as in its own test.
    #[test]
    fn rider_report_rider_hit0_matches() {
        use crate::difftest::fakes::{covered, machine, vtable, LOG_RESULTS};
        use crate::game::be::beintaggression::{AGGRESSION_MESSAGES_OFF, AGGRESSION_RECORDS};
        covered(
            ee::s00::f_0010e228__generated,
            rider_report_rider_hit0,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_e228, 3);
                c.pc = 0x0010_e228;
                let gp = crate::difftest::GAME_GP;
                let world = DATA + 0x5_0000;
                let audio = DATA + 0x1_0000;
                let agg = DATA + 0x6_0000;
                let (a, b) = (DATA, DATA + 0x1000);
                // Audio: no views (world+0x84 -> +0x84 -> count +0x10 = 0);
                // speech volume (audio + 0x62fc) 0.
                c.bus.write32(gp + 0x410, audio);
                c.bus.write32(audio + 0x62fc, 0);
                c.bus.write32(gp - 0x848, world);
                c.bus.write32(world + 0x84, world + 0x100);
                c.bus.write32(world + 0x100 + 0x84, world + 0x300);
                c.bus.write32(world + 0x300 + 0x10, 0);
                c.bus.write32(world + 0x100 + 0x28, world + 0x200);
                c.bus.write32(world + 0x200, 1 + r.next_u32() % 9);
                // Aggression singleton exists.
                c.bus.write32(gp - 0x1ea4, agg);
                // Controllers: GetRiderIndex (and the sound's position
                // method at +0x28) are fakes.
                for rider in [a, b] {
                    let vt = vtable(c, &mut r, &[0x28, HIT_INDEX_SLOT]);
                    c.bus.write32(rider + HIT_CONTROLLER, vt);
                }
                let rider = 1 + r.next_u32() % 2;
                let rival = 1 + r.next_u32() % 3;
                c.bus.write32(LOG_RESULTS + 4, rider);
                c.bus.write32(LOG_RESULTS + 8, rival);
                // AddAggression's rider slots, character records and
                // aggression records (as in its own test).
                let pick = |r: &mut Rng, v: &[i32]| v[(r.next_u32() as usize) % v.len()];
                for idx in 0..4u32 {
                    c.bus.write32(0x0053_05b0 + 4 * idx, idx);
                    let rec = 0x0053_5b20 + 0x1c * idx;
                    let flag = pick(&mut r, &[-1, 0, 1]) as u32;
                    c.bus.write32(rec + 0xc, flag);
                    c.bus.write32(rec + 0x10, r.next_u32() % 4);
                    c.bus.write8(rec + 0x11, (r.next_u32() % 5) as u8);
                }
                for profile in 0..3u32 {
                    for ch in 0..5u32 {
                        for rv in 0..5u32 {
                            let e = AGGRESSION_RECORDS + profile * 0x9b50 + ch * 0xf88 + rv * 3;
                            c.bus
                                .write8(e + 0xbc1, pick(&mut r, &[-1, 0, 1, 2, 3, 4]) as u8);
                            c.bus
                                .write8(e + 0xbc2, pick(&mut r, &[-1, 0, 1, 2, 3, 4, 5]) as u8);
                            c.bus.write8(
                                e + 0xbc3,
                                pick(&mut r, &[-3, 0, 4, 9, 14, 15, 19, 24, 30]) as u8,
                            );
                        }
                    }
                }
                c.bus
                    .write8(AGGRESSION_MESSAGES_OFF, (r.next_u32() % 2) as u8);
                c.g[A0][0] = a as u64;
                c.g[A1][0] = b as u64;
            },
        );
    }
}
