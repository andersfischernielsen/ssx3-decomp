//! `src/ai/computer.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Target;
use crate::ps2f::FpuF32;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::abi::r32;
use crate::call::Flow;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use ssx3_ee_rt::funcs::call_indirect;
pub(crate) use crate::game::ai::rider::p;
pub(crate) use crate::game::ai::rider::gp_3 as gp;
pub(crate) use crate::game::ai::rider::ldf_2 as ldf;
pub(crate) use crate::game::ai::rider::fr_2 as fr;
pub(crate) use crate::game::ai::rider::setf_2 as setf;
pub(crate) use crate::game::ai::rider::lt;
pub(crate) use crate::game::ai::rider::le;
pub(crate) use crate::game::ai::rider::eq;
pub(crate) use crate::game::ai::rider::add32;
pub(crate) use crate::game::ai::rider::shl32;
pub(crate) use crate::game::visualfx::boardtrailfx::shl32_2;
pub(crate) use crate::game::visualfx::boardtrailfx::add32 as add32_2;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::visualfx::boardwakefx::lw_2;
pub(crate) use crate::game::visualfx::boardwakefx::lh_2 as lh;
pub(crate) use crate::game::visualfx::boardwakefx::r32_2;
pub(crate) use crate::game::visualfx::boardwakefx::gp as gp_3;
pub(crate) use crate::game::visualfx::boardwakefx::at_3 as at_2;
pub(crate) use crate::game::visualfx::boardwakefx::ldf_4 as ldf_3;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at_3;
pub(crate) use crate::game::movie::movieplayer::at as at_4;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_5;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::ofs;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::gpa;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::ldf as ldf_4;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::fepopup::at as at_6;
pub(crate) use crate::game::fe::fepopup::lb;
pub(crate) use crate::game::fe::fepopup::ld;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_0348, computer_actor_rider_compute_steer, "ComputerActorRider_ComputeSteer"),
    (0x0010_0680, computer_actor_rider_cruise_follow_path, "ComputerActorRider_CruiseFollowPath"),
    (0x0010_8f88, computer_actor_rider_init_2, "ComputerActorRider_Init_2"),
    (0x0010_a768, computer_actor_rider_get_control_input, "ComputerActorRider_GetControlInput"),
    (0x0010_b250, computer_actor_rider_think_spin, "ComputerActorRider_ThinkSpin"),
    (0x0010_c4f8, computer_actor_rider_apply_difficulty, "ComputerActorRider_ApplyDifficulty"),
    (0x0010_cab8, computer_actor_rider_init_difficulty_2, "ComputerActorRider_InitDifficulty_2"),
    (0x0010_d170, computer_actor_rider_reset_trick_counts, "ComputerActorRider_ResetTrickCounts"),
    (0x0010_d1a0, computer_actor_rider_should_reset_path, "ComputerActorRider_ShouldResetPath"),
    (0x0010_d8f8, computer_actor_rider_find_human_rider_index, "ComputerActorRider_FindHumanRiderIndex"),
    (0x0010_d938, computer_actor_rider_is_far_behind_human, "ComputerActorRider_IsFarBehindHuman"),
    (0x0010_d990, computer_actor_rider_is_far_ahead_of_human, "ComputerActorRider_IsFarAheadOfHuman"),
    (0x0010_d9e8, computer_actor_rider_is_rival_target, "ComputerActorRider_IsRivalTarget"),
    (0x0010_e028, rider_set_timed_event, "Rider_SetTimedEvent"),
    (0x0010_e098, rider_add_boost, "Rider_AddBoost"),
    (0x0010_e558, rider_on_split_marker, "Rider_OnSplitMarker"),
    (0x0010_e5d8, rider_handle_race_path_event, "Rider_HandleRacePathEvent"),
    (0x0010_e830, rider_v15, "Rider_v15"),
    (0x0010_e8b8, rider_award_bonus_boost, "Rider_AwardBonusBoost"),
    (0x0010_f2d8, rider_on_mission_completed, "Rider_OnMissionCompleted"),
    (0x0010_f338, rider_on_item_collected, "Rider_OnItemCollected"),
    (0x0010_f398, ai_reset_rider_relations_2, "AI_ResetRiderRelations_2"),
    (0x0010_fc30, ai_count_riders_on_path, "AI_CountRidersOnPath"),
    (0x0011_12f8, rider_base_exit_main_state, "RiderBase_ExitMainState"),
    (0x0011_1380, rider_base_enter_main_state, "RiderBase_EnterMainState"),
    (0x0011_1408, rider_base_update_main_state, "func_00111408"),
    (0x0011_14a0, rider_base_post_update_main_state, "func_001114A0"),
    (0x0011_1728, rider_base_handle_sub_state_input, "func_00111728"),
    (0x0011_1890, rider_base_reset_fx, "RiderBase_ResetFX"),
    (0x0011_2338, rider_update_race_progress, "Rider_UpdateRaceProgress"),
    (0x0011_2588, rider_get_ai_path_look_ahead_dist, "Rider_GetAIPathLookAheadDist"),
    (0x0011_25a8, rider_save_ai_path_look_ahead, "Rider_SaveAIPathLookAhead"),
    (0x0011_25b8, rider_return_zero_25b8, "Rider_ReturnZero_25b8"),
    (0x0011_25c0, rider_update_ai_path_progress, "Rider_UpdateAIPathProgress"),
    (0x0011_3128, rider_get_dist_to_finish, "Rider_GetDistToFinish"),
    (0x0011_3138, air_predictor_ctor, "AirPredictor_ctor"),
    (0x0011_3170, rider_dtor_free, "Rider_dtorFree"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_UPDATE_RIDER_WORLD_PAINT;
    pub(crate) use crate::game::ai::rider::callee::MATH_ATAN;
    pub(crate) use crate::game::ai::rider::callee::WORLD_PAINTER_WEATHER28;
    pub(crate) use crate::game::ai::rider::callee::WORLD_PAINTER_WEATHER20;
    pub(crate) use crate::game::ai::rider::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::ai::rider::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::ai::rider::callee::FIXUNSSFSI;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_ARCADE_SAY;
    pub(crate) use crate::game::ai::rider::callee::PATH_GET_CLOSEST_ON_PATH;
    pub(crate) use crate::game::ai::rider::callee::PATH_GET_CLOSEST;
    pub(crate) use crate::game::ai::rider::callee::PATH_GET_POINT_AT_DISTANCE;
    pub(crate) use crate::game::ai::rider::callee::PATH_GET_LENGTH;
    pub(crate) use crate::game::ai::rider::callee::EA_MEM_COPY;
    pub(crate) use crate::game::ai::rider::callee::RIDER_IS_ON_GROUND;
    pub(crate) use crate::game::ai::rider::callee::RIDER_UPDATE_DANGER_STATE;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_SUB_STATE;
    pub(crate) use crate::game::ai::rider::callee::RIDER_SET_TIMED_EVENT;
    pub(crate) use crate::game::ai::rider::callee::RIDER_UPDATE_DIST_TO_FINISH;
    pub(crate) use crate::game::ai::rider::callee::RIDER_SELECT_RACE_PATH;
    pub(crate) use crate::game::ai::rider::callee::RIDER_HANDLE_RACE_PATH_EVENT;
    pub(crate) use crate::game::ai::rider::callee::RIDER_RETURN_ZERO;
    pub(crate) use crate::game::ai::rider::callee::RIDER_AI_LOOK_AHEAD;
    pub(crate) use crate::game::ai::rider::callee::RIDER_SAVE_AI_LOOK_AHEAD;
    pub(crate) use crate::game::ai::rider::callee::RIDER_SELECT_AI_PATH;
    pub(crate) use crate::game::ai::rider::callee::RIDER_ON_FINISH_RACE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL8_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL16_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL32_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL64_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL128_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL256_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL512_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::REMOVE_HUD_MESSAGE;
    pub(crate) use crate::game::movie::movieplayer::callee::SET_TIMED_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::PACK_POSITION;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILD_OCCLUDER_PLANES;
    pub(crate) use crate::game::movie::movieplayer::callee::MATH_DET3X3;
    pub(crate) use crate::game::movie::movieplayer::callee::SHOW_HUD_NUMBER;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_CHAR_ATTRIB_FACTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::movie::movieplayer::callee::AUDIO_RESET_BOOST_PITCH;
    pub(crate) use crate::game::movie::movieplayer::callee::QSORT;
    pub(crate) use crate::game::movie::movieplayer::callee::VU0_UPLOAD_OCCLUDER;
    pub(crate) use crate::game::movie::movieplayer::callee::AIR_APPLY_WIND;
    pub(crate) use crate::game::movie::movieplayer::callee::AIR_PREDICTOR_UPDATE;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_GET_SUBSTATE;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_UPDATE_ORIENTATION;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_ORIENTATION_IMPLICIT;
    pub(crate) use crate::game::movie::movieplayer::callee::COLL_SEGMENT_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::OVERLAP_LIST_FIND_CLOSEST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2 as DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2 as RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2 as CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::NET_FE_STATE_DTOR_3 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_STOP_2 as SCRIPT_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_ON_SEQ_END;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_TARGET_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2 as SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2 as SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2 as SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;
    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_STREAM_HANDLE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT;
    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::render::particle::callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT;
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::render::particle::callee::AVA_SPLINE_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2 as UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS as RIDER_METRIX_AWARD_POINTS_2;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST as RIDER_ADD_BOOST_2;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_FINISH_BOARD_PRESS;
    pub(crate) use crate::game::render::particle::callee::ALGORITHM_LIST_DTOR;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::render::particle::callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::CROWD_MAN3D_GROUP_REMOVE_SECTION;
    pub(crate) use crate::game::render::particle::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::render::particle::callee::FLEX_BRIDGE_SPRINGS_SAVE;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_COLLECTIBLE_COLLECTED;
    pub(crate) use crate::game::render::particle::callee::BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::render::particle::callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX;
    pub(crate) use crate::game::render::particle::callee::SND_LOCK;
    pub(crate) use crate::game::render::particle::callee::SND_UNLOCK;
    pub(crate) use crate::game::render::particle::callee::SND_SPU_MEM_SET_BASE;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_POP;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_SELECT;
    pub(crate) use crate::game::render::particle::callee::SNDSTRM_SET_PITCH;
    pub(crate) use crate::game::render::particle::callee::PATH_SET_PAUSED;
    pub(crate) use crate::game::render::particle::callee::PATH_POST_EVENT;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_SEEK;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_READ;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_WRITE;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_MKDIR;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_DELETE;
    pub(crate) use crate::game::render::particle::callee::ISIGNAL_SEMA;
    pub(crate) use crate::game::render::particle::callee::STRLEN;
    pub(crate) use crate::game::render::particle::callee::WORLD_CREATE_DEAD_NODE;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_CTOR_5 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2 as STRCPY;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_FIND_SOUND_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_SET_SELECTION;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_ON_OPERATION_DONE;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_END_OPERATION_4 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_SET_MODE_2 as MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_RESET_LAYOUT;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_CONFIG;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::ROLLER_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER as MOD_OBJ_SET_TRANSFORM_MODIFIER_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MULTI_SPLINE_MODIFIER_ADD_REF;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_CTOR_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_DTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::FONT_PARSE_GLYPHS;
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::movie::movieplayer::callee::FILE_LOAD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_BE;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_INTERFACE;
    pub(crate) use crate::game::movie::movieplayer::callee::C_OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::EVENT_SELECT_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FLAG_MANAGER_GROUP_INIT_KEY;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RESET_FOR_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE as RIDER_GET_MAIN_STATE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2 as RIDER_ADD_BOOST_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_APPLY_SCRIPT_TRACK;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::fe::fepopup::callee::C00113f88;
    pub(crate) use crate::game::fe::fepopup::callee::C00115ab0;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe78;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe98;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fec8;
    pub(crate) use crate::game::fe::fepopup::callee::C00125108;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a080;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a0e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0014bdb8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d448;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d988;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d998;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dc80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dd58;
    pub(crate) use crate::game::fe::fepopup::callee::C00150928;
    pub(crate) use crate::game::fe::fepopup::callee::C00150b48;
    pub(crate) use crate::game::fe::fepopup::callee::C00151c48;
    pub(crate) use crate::game::fe::fepopup::callee::C001535c0;
    pub(crate) use crate::game::fe::fepopup::callee::C001543e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00156988;
    pub(crate) use crate::game::fe::fepopup::callee::C00156a60;
    pub(crate) use crate::game::fe::fepopup::callee::C001577e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00157bf0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158910;
    pub(crate) use crate::game::fe::fepopup::callee::C001589b0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158a50;
    pub(crate) use crate::game::fe::fepopup::callee::C00158af0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158c80;
    pub(crate) use crate::game::fe::fepopup::callee::C00158d58;
    pub(crate) use crate::game::fe::fepopup::callee::C00159170;
    pub(crate) use crate::game::fe::fepopup::callee::C00159cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C0015a2e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c6b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c718;
    pub(crate) use crate::game::fe::fepopup::callee::C00176890;
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
    pub(crate) use crate::game::fe::fepopup::callee::C00225068;
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
    pub(crate) use crate::game::fe::fepopup::callee::C00275498;
    pub(crate) use crate::game::fe::fepopup::callee::C002758c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275b98;
    pub(crate) use crate::game::fe::fepopup::callee::C00275cf8;
    pub(crate) use crate::game::fe::fepopup::callee::C00278308;
    pub(crate) use crate::game::fe::fepopup::callee::C00278e50;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f38;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f68;
    pub(crate) use crate::game::fe::fepopup::callee::C00281ed8;
    pub(crate) use crate::game::fe::fepopup::callee::C00283228;
    pub(crate) use crate::game::fe::fepopup::callee::C00283518;
    pub(crate) use crate::game::fe::fepopup::callee::C002837c8;
    pub(crate) use crate::game::fe::fepopup::callee::C0028b180;
    pub(crate) use crate::game::fe::fepopup::callee::C0028bb10;
    pub(crate) use crate::game::fe::fepopup::callee::C0028e8c0;
    pub(crate) use crate::game::fe::fepopup::callee::C002913d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003457c8;
    pub(crate) use crate::game::fe::fepopup::callee::C00358020;
    pub(crate) use crate::game::fe::fepopup::callee::C00362478;
    pub(crate) use crate::game::fe::fepopup::callee::C003625c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00362660;
    pub(crate) use crate::game::fe::fepopup::callee::C003626d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003627a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00364cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C00366ce0;
    pub(crate) use crate::game::fe::fepopup::callee::C00368170;
    pub(crate) use crate::game::fe::fepopup::callee::C003684f0;
    pub(crate) use crate::game::fe::fepopup::callee::C00368970;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f460;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f668;
    pub(crate) use crate::game::fe::fepopup::callee::C0039c870;
    pub(crate) use crate::game::fe::fepopup::callee::C0039ff78;
    pub(crate) use crate::game::fe::fepopup::callee::C003b1300;
    pub(crate) use crate::game::fe::fepopup::callee::C003b2360;


    pub const GET_MAIN_STATE: Callee = Callee { addr: 0x0011_fe98, f: ee::s00::Rider_GetMainState };

    pub const GET_SUB_STATE: Callee = Callee { addr: 0x0011_fee8, f: ee::s00::Rider_GetSubState };

    pub const BE_LIBRARY_GET_INSTANCE: Callee =
        Callee { addr: 0x0014_dc80, f: ee::s01::BELibrary_GetInstance };


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
    pub(crate) use crate::game::ai::rider::rider::BASE;
    pub(crate) use crate::game::ai::rider::rider::HANDLER;
    pub(crate) use crate::game::ai::rider::rider::POSITION;
    pub(crate) use crate::game::ai::rider::rider::VELOCITY;
    pub(crate) use crate::game::ai::rider::rider::HEADING;
    pub(crate) use crate::game::ai::rider::rider::GROUND_NORMAL;
    pub(crate) use crate::game::ai::rider::rider::RACE_PATH;
    pub(crate) use crate::game::ai::rider::rider::RACE_PATH_CURSOR;
    pub(crate) use crate::game::ai::rider::rider::AI_PATH;
    pub(crate) use crate::game::ai::rider::rider::AI_PATH_CURSOR;
    pub(crate) use crate::game::ai::rider::rider::RACE_ENTRIES;
    pub(crate) use crate::game::ai::rider::rider::RACE_ENTRY_COUNT;
    pub(crate) use crate::game::ai::rider::rider::DIST_TO_FINISH;
    pub(crate) use crate::game::ai::rider::rider::DIST_ESTIMATE;
    pub(crate) use crate::game::ai::rider::rider::AI_POINT;
    pub(crate) use crate::game::ai::rider::rider::AI_POINT_PREV;
    pub(crate) use crate::game::ai::rider::rider::AI_POINT_TARGET;
    pub(crate) use crate::game::ai::rider::rider::AI_ARC_LEN;
    pub(crate) use crate::game::ai::rider::rider::AI_PROGRESS;
    pub(crate) use crate::game::ai::rider::rider::AI_SEARCH_DIST;
    pub(crate) use crate::game::ai::rider::rider::AI_HEADING;
    pub(crate) use crate::game::ai::rider::rider::RENDER_FLAG;
    pub(crate) use crate::game::ai::rider::rider::RENDER_KIND;
    pub(crate) use crate::game::ai::rider::rider::VISIBILITY_BLOCK;
    pub(crate) use crate::game::ai::rider::rider::ON_SCREEN;
    pub(crate) use crate::game::ai::rider::rider::UNK_150;
    pub(crate) use crate::game::ai::rider::rider::BBOX0;
    pub(crate) use crate::game::ai::rider::rider::BBOX1;
    pub(crate) use crate::game::ai::rider::rider::BSPHERE;
    pub(crate) use crate::game::ai::rider::rider::DELTA_TIME;
    pub(crate) use crate::game::ai::rider::rider::BOOST_A;
    pub(crate) use crate::game::ai::rider::rider::BOOST_B;
    pub(crate) use crate::game::ai::rider::rider::BOOST;
    pub(crate) use crate::game::ai::rider::rider::BOOST_PHASE;
    pub(crate) use crate::game::ai::rider::rider::BOOST_SCALE;
    pub(crate) use crate::game::ai::rider::rider::BOOST_KIND;
    pub(crate) use crate::game::ai::rider::rider::IS_HUMAN;
    pub(crate) use crate::game::ai::rider::rider::AI_INDEX;
    pub(crate) use crate::game::ai::rider::rider::FINISH_TIME;
    pub(crate) use crate::game::ai::rider::rider::DID_NOT_FINISH;

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
    pub(crate) use crate::game::fe::festateload::g::WORLD_TRIGGER_MANAGER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER_END;
    pub(crate) use crate::game::fe::festateload::g::BE_OPTION_SINGLETON;
    pub(crate) use crate::game::fe::festateload::g::FE_BASE_BACK;
    pub(crate) use crate::game::fe::festateload::g::BOLT_FILE_CACHE;
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

pub(crate) const COMPUTER_ACTOR_RIDER_INIT: Callee = Callee { addr: 0x0010_8e88, f: ssx3_ee::s00::f_00108e88 };

pub(crate) const COMPUTER_ACTOR_RIDER_INIT_DIFFICULTY: Callee =
    Callee { addr: 0x0010_c758, f: ssx3_ee::s00::f_0010c758 };

pub(crate) const AI_RESET_RIDER_RELATIONS: Callee = Callee { addr: 0x0010_f3b8, f: ssx3_ee::s00::f_0010f3b8 };

pub(crate) const AIR_PREDICTOR_RESET: Callee = Callee { addr: 0x0011_3198, f: ssx3_ee::s00::f_00113198 };

pub(crate) const GET_HUMAN_RIDER: Callee = Callee { addr: 0x0010_deb0, f: ssx3_ee::s00::f_0010deb0 };

pub(crate) const GET_DIST_TO_FINISH: Callee = Callee { addr: 0x0011_3128, f: ssx3_ee::s00::f_00113128 };


// ---------------------------------------------------------------------------
// Computer actor riders.

/// `void ComputerActorRider_Init_2(ComputerActorRider *self)` (0x108f88):
/// tail wrapper around `ComputerActorRider_Init`.
pub fn computer_actor_rider_init_2(c: &mut Cpu) {
    call::tail_wrapper(c, None, Target::Direct(COMPUTER_ACTOR_RIDER_INIT), 0x0010_8f98);
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
    call::tail_wrapper(c, None, Target::Direct(AI_RESET_RIDER_RELATIONS), 0x0010_f3a8);
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
                    c.g[A1][0] = 0x3c;
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
                    c.g[A1][0] = 0x3c;
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
    StateCase { handler: 0x0013_d818, ret: 0x0011_1444, off: 0, cycles: 4, tail: false },
    StateCase { handler: 0x0013_9a20, ret: 0x0011_1454, off: 0x20, cycles: 4, tail: false },
    StateCase { handler: 0x0013_6e98, ret: 0x0011_1464, off: 0x30, cycles: 4, tail: false },
    StateCase { handler: 0x0013_6958, ret: 0x0011_1474, off: 0xa0, cycles: 4, tail: false },
    StateCase { handler: 0x0013_af28, ret: 0x0011_1484, off: 0xb0, cycles: 4, tail: false },
    StateCase { handler: 0x0013_91a8, ret: 0x0011_1494, off: 0x110, cycles: 2, tail: true },
];


/// `RiderBase_PostUpdateMainState` (0x1114a0): the post-update handler per
/// main state.
pub(crate) const POST_CASES: [StateCase; 6] = [
    StateCase { handler: 0x0013_f178, ret: 0x0011_14dc, off: 0, cycles: 4, tail: false },
    StateCase { handler: 0x0013_9c88, ret: 0x0011_14ec, off: 0x20, cycles: 4, tail: false },
    StateCase { handler: 0x0013_6ee0, ret: 0x0011_14fc, off: 0x30, cycles: 4, tail: false },
    StateCase { handler: 0x0013_6978, ret: 0x0011_150c, off: 0xa0, cycles: 4, tail: false },
    StateCase { handler: 0x0013_bfa8, ret: 0x0011_151c, off: 0xb0, cycles: 4, tail: false },
    StateCase { handler: 0x0013_9528, ret: 0x0011_152c, off: 0x110, cycles: 2, tail: true },
];


/// `RiderBase_HandleSubStateInput` (0x111728): the input handler per sub
/// state.
pub(crate) const SUB_CASES: [StateCase; 13] = [
    StateCase { handler: 0x0013_1620, ret: 0x0011_1764, off: 0x1c0, cycles: 4, tail: false },
    StateCase { handler: 0x0012_fc80, ret: 0x0011_1774, off: 0x1d0, cycles: 4, tail: false },
    StateCase { handler: 0x0012_e9b8, ret: 0x0011_1784, off: 0x1f0, cycles: 4, tail: false },
    StateCase { handler: 0x0012_e778, ret: 0x0011_1794, off: 0x200, cycles: 4, tail: false },
    StateCase { handler: 0x0012_f730, ret: 0x0011_17a4, off: 0x210, cycles: 4, tail: false },
    StateCase { handler: 0x0013_3308, ret: 0x0011_17b4, off: 0x230, cycles: 4, tail: false },
    StateCase { handler: 0x0012_bf68, ret: 0x0011_17c4, off: 0x290, cycles: 4, tail: false },
    StateCase { handler: 0x0013_1d30, ret: 0x0011_17d4, off: 0x2b0, cycles: 4, tail: false },
    StateCase { handler: 0x0012_cb68, ret: 0x0011_17e4, off: 0x2c0, cycles: 4, tail: false },
    StateCase { handler: 0x0012_f398, ret: 0x0011_17f4, off: 0x350, cycles: 4, tail: false },
    StateCase { handler: 0x0012_c678, ret: 0x0011_1804, off: 0x360, cycles: 4, tail: false },
    StateCase { handler: 0x0013_2a30, ret: 0x0011_1814, off: 0x370, cycles: 4, tail: false },
    StateCase { handler: 0x0013_6508, ret: 0x0011_1824, off: 0x390, cycles: 2, tail: true },
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
                    pc = if out_of_range { 0x0011_13fc } else { 0x0011_139c };
                }
                0x0011_139c => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6b30));
                    c.g[V1][0] =
                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
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
                    pc = if out_of_range { 0x0011_1374 } else { 0x0011_1314 };
                }
                0x0011_1314 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6b10));
                    c.g[V1][0] =
                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
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
                                                                        c.g[V0][0] =
                                                                            lb(c, at_6(c, T0, 0x49));
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
const UPDATE_SPIN_PLAN: Callee =
    Callee { addr: 0x0010_b0e8, f: ee::s00::ComputerActorRider_UpdateSpinPlan };
/// `int ComputerActorRider_ChooseTrick(ai, float height)` (0x10cad8).
const CHOOSE_TRICK: Callee = Callee { addr: 0x0010_cad8, f: ee::s00::ComputerActorRider_ChooseTrick };
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


#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fill;
    use crate::difftest::fakes;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use ssx3_ee as ee;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::SP;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::check_lohi;
    use ssx3_ee::s01;
    use ssx3_ee::s03;
    use ssx3_ee::s00;
    use ssx3_ee::s02;
    use ssx3_ee::s08;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use ssx3_ee::s11;
    use ssx3_ee::s12;
    use ssx3_ee::s13;
    use crate::difftest::fakes::vtable;
    use crate::difftest::elf_image;
    use crate::difftest::SP as STACK;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::RA as RET;
    use crate::difftest::compare;
    pub(crate) use crate::game::lib::kernel::tests::SEEDS;
    pub(crate) use crate::game::ai::rider::tests::RIDER;
    pub(crate) use crate::game::ai::rider::tests::OBJ;
    pub(crate) use crate::game::ai::rider::tests::wf;
    pub(crate) use crate::game::ai::rider::tests::w;
    pub(crate) use crate::game::ai::rider::tests::atan_constants;
    pub(crate) use crate::game::movie::movieplayer::tests::STUB;
    pub(crate) use crate::game::bx::bxstring2::tests::be_library;
    pub(crate) use crate::game::fe::festateload::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::ai::ai::tests::belibrary;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::clear as clear_2;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::ai::ai::tests::rng;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::put_f32s;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2;
    pub(crate) use crate::game::movie::movieplayer::tests::W as W_2;
    pub(crate) use crate::game::movie::movieplayer::tests::w as w_2;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_6 as W_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_11 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_9 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_11 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_11 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_2;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_4;


    #[test]
    fn computer_actor_rider_is_rival_target_matches() {
        check(ssx3_ee::s00::f_0010d9e8__generated, computer_actor_rider_is_rival_target, 300, &|c, seed| {
            let mut r = Rng(0x0f0f_0f0f ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let rider = DATA + 0x100;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA + 0x18, rider);
            c.bus.write32(rider + 0xf0, if seed % 3 == 0 { 0 } else { r.next_u32() });
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
        });
    }


    #[test]
    fn rider_get_ai_path_look_ahead_dist_matches() {
        check(ssx3_ee::s00::f_00112588__generated, rider_get_ai_path_look_ahead_dist, 200, &|c, seed| {
            let mut r = Rng(0x5151_5151 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = if seed % 2 == 0 {
                0
            } else {
                ((r.next_u32() as u64) << 32) | (r.next_u32() as u64 | 1)
            };
        });
    }


    #[test]
    fn rider_save_ai_path_look_ahead_matches() {
        check(ssx3_ee::s00::f_001125a8__generated, rider_save_ai_path_look_ahead, 200, &|c, seed| {
            let mut r = Rng(0x6161_6161 ^ seed as u64);
            scramble(c, &mut r);
            // Odd seeds use a misaligned base to cover `lq`/`sq` masking.
            let base = DATA + (seed % 2) * 8;
            c.g[A0][0] = base as u64;
            for i in 0..8 {
                c.bus.write32(base + 0x4a0 + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn computer_actor_rider_init_2_matches() {
        check(ssx3_ee::s00::f_00108f88, computer_actor_rider_init_2, 60, &|c, seed| {
            let mut r = Rng(0x108f_8800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..16 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn computer_actor_rider_init_difficulty_2_matches() {
        check(ssx3_ee::s00::f_0010cab8, computer_actor_rider_init_difficulty_2, 60, &|c, seed| {
            let mut r = Rng(0x10ca_b800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..16 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
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
        check(ssx3_ee::s00::f_0010d938, computer_actor_rider_is_far_behind_human, 120, &|c, seed| {
            far_setup(c, seed, 0xdc);
        });
    }


    #[test]
    fn computer_actor_rider_is_far_ahead_of_human_matches() {
        check(ssx3_ee::s00::f_0010d990, computer_actor_rider_is_far_ahead_of_human, 120, &|c, seed| {
            far_setup(c, seed, 0xe0);
        });
    }


    #[test]
    fn ai_reset_rider_relations_2_matches() {
        check(ssx3_ee::s00::f_0010f398, ai_reset_rider_relations_2, 60, &|c, seed| {
            let mut r = Rng(0x10f3_9800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..16 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn air_predictor_ctor_matches() {
        check(ssx3_ee::s00::f_00113138, air_predictor_ctor, 60, &|c, seed| {
            let mut r = Rng(0x1131_3800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..8 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
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
                c.bus.write32(rider + 0xab8, if i % 2 == 0 { path } else { r.next_u32() });
            }
            c.g[A0][0] = ai as u64;
            c.g[A1][0] = path as u64;
            schedule(c, &mut r, seed, 0x0010_fc30)
        };
        check_events(ssx3_ee::s00::f_0010fc30, ai_count_riders_on_path, SEEDS, &setup);
    }


    #[test]
    fn rider_update_race_progress_matches() {
        check(ssx3_ee::s00::f_00112338__generated, rider_update_race_progress, 120, &|c, seed| {
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
        });
    }


    #[test]
    fn rider_update_ai_path_progress_matches() {
        check(ssx3_ee::s00::f_001125c0__generated, rider_update_ai_path_progress, 120, &|c, seed| {
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
            w(c, rider + rider::AI_PROGRESS, f32::from_bits(r.f32(100.0)).to_bits());
            w(c, rider + rider::AI_SEARCH_DIST, f32::from_bits(r.f32(100.0)).to_bits());
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
        });
    }


    /// The `gp_2`-relative `.lit4` constants these functions read (a test
    /// machine's RAM starts empty; the game image holds them).
    pub(crate) fn literals(c: &mut Cpu) {
        put_words(c, GAME_GP - 0x2de8, &[0x5015_02f9, 0xd015_02f9]);
        // The angle-wrap pool around `gp_2 - 0x7d18`.
        put_words(
            c,
            GAME_GP - 0x7d18,
            &[0xbfc9_0fdb, 0x3fc9_0fdb, 0x4049_0fdb, 0x4049_0fdb, 0x3e22_f983, 0x40c9_0fdb, 0x3f86_0a93],
        );
        // 0.036 / 4.0086374
        put_words(c, GAME_GP - 0x7070, &[0x3d13_74bc, 0x4080_46c2]);
    }


    #[test]
    fn computer_actor_rider_should_reset_path_matches() {
        check(ssx3_ee::s00::f_0010d1a0__generated, computer_actor_rider_should_reset_path, 300, &|c, seed| {
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
            put_vec(c, rider + rider::VELOCITY_2, [vx, vy, r.f32(10.0), r.f32(1.0)]);
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
            c.bus.write32(base + rider_base::SUB_STATE, if seed % 5 == 0 { 8 } else { seed % 9 });
            let height = match seed % 3 {
                0 => r.f32(20.0),
                1 => 0x41f0_0000, // exactly 30
                _ => 0x4240_0000, // 48
            };
            c.bus.write32(predictor + air_predictor::HEIGHT, height);
            c.bus.write32(predictor + air_predictor::LANDING, r.f32(50.0));
            c.bus.write32(predictor + air_predictor::MODE, seed % 4);
        });
    }


    #[test]
    fn rider_base_update_main_state_matches() {
        check(ee::s00::f_00111408__generated, rider_base_update_main_state, 200, &|c, seed| {
            rider_state_table_test(c, seed, 0x0011_1408, 0xde0, 0x0045_6b50, 0x0011_1494);
        });
    }


    #[test]
    fn rider_base_post_update_main_state_matches() {
        check(ee::s00::f_001114a0__generated, rider_base_post_update_main_state, 200, &|c, seed| {
            rider_state_table_test(c, seed, 0x0011_14a0, 0xde0, 0x0045_6b70, 0x0011_152c);
        });
    }


    #[test]
    fn rider_base_handle_sub_state_input_matches() {
        check(ee::s00::f_00111728__generated, rider_base_handle_sub_state_input, 200, &|c, seed| {
            rider_state_table_test(c, seed, 0x0011_1728, 0xde4, 0x0045_6c10, 0x0011_1824);
        });
    }


    /// The shared setup of the three state tables: a rider whose state
    /// selects a table entry, and a table that either stays inside the
    /// function (its tail / epilogue block) or is empty (an indirect jump
    /// out, answered by the fake jump hook).
    pub(crate) fn rider_state_table_test(c: &mut Cpu, seed: u32, salt: u64, state_off: u32, table: u32, tail: u32) {
        machine(c, seed, salt, 0);
        let rider = DATA;
        let states = [0u32, 1, 2, 3, 4, 5, 6, 13, 14, 0xffff_ffff];
        c.bus.write32(rider + state_off, states[(seed as usize) % states.len()]);
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
        check(ee::s00::f_0010d8f8__generated, computer_actor_rider_find_human_rider_index, SEEDS, &|c, seed| {
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
        });
    }


    #[test]
    fn rider_add_boost_matches() {
        check(ee::s00::f_0010e098__generated, rider_add_boost, 240, &|c, seed| {
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
        });
    }


    #[test]
    fn rider_dtor_free_matches() {
        crate::difftest::check(ee::s00::f_00113170__generated, rider_dtor_free, 60, &|c, seed| {
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
        });
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
        check(ee::s00::f_0010f2d8__generated, rider_on_mission_completed, SEEDS_2, &|c, seed| {
            let mut r = common(c, seed, 0x10f2d8, 4);
            let this = DATA + 0x100;
            clear_2(c, this, 0x900 / 4);
            c.bus.write32(this + 0x790, DATA + 0x2000);
            c.bus.write32(this + 0x86c, DATA + 0x3000);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn rider_on_item_collected_matches() {
        check(ee::s00::f_0010f338__generated, rider_on_item_collected, SEEDS_2, &|c, seed| {
            let mut r = common(c, seed, 0x10f338, 4);
            let this = DATA + 0x100;
            clear_2(c, this, 0x900 / 4);
            c.bus.write32(this + 0x790, DATA + 0x2000);
            c.bus.write32(this + 0x86c, DATA + 0x3000);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = r.next_u32() as u64;
        });
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
        leaf(ssx3_ee::s00::f_001125b8__generated, rider_return_zero_25b8, 3);
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
            assert_ne!(c.bus.read32(table + state * 4), 0, "state table {table:#x} not loaded");
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
                c.bus.write16(vt + 0x10, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x14, FAKE);
                c.bus.write16(vt + 0x28, (r.next_u32() as i16 as i32 & !3) as u16);
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
                            [f32::from_bits(r.f32(2.0)), f32::from_bits(r.f32(2.0)), 1.0, 1.0],
                        );
                        put_f32s(c, axis, [f32::from_bits(r.f32(2.0)), f32::from_bits(r.f32(2.0)), f32::from_bits(r.f32(2.0)), 0.0]);
                        put_f32s(c, vel, [f32::from_bits(r.f32(2.0)), f32::from_bits(r.f32(2.0)), 0.0, 0.0]);
                    }
                    _ => {
                        put_f32s(
                            c,
                            model + 0x1c0,
                            [f32::from_bits(r.f32(2.0)), f32::from_bits(r.f32(2.0)), -f32::from_bits(r.f32(1.0)), 0.0],
                        );
                        put_f32s(c, axis, [f32::from_bits(r.f32(2.0)), f32::from_bits(r.f32(2.0)), f32::from_bits(r.f32(2.0)), 0.0]);
                        put_f32s(c, vel, [f32::from_bits(r.f32(2.0)), f32::from_bits(r.f32(2.0)), 0.0, 0.0]);
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
            assert!((at - at_60).abs() < 1e-9, "{hz} Hz: {at} s ahead, 60 Hz: {at_60} s");
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
        check(ee::s00::f_00111890__generated, rider_base_reset_fx, 40, &|c, seed| {
            alloc_base(c, seed, 0x1118, 3);
            c.g[A0][0] = DATA as u64;
            w_3(c, DATA + 0x18, DATA + 0x400);
            w_3(c, DATA + 0x400 + 0x870, if seed % 2 == 0 { 0 } else { 5 });
        });
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
                    &[0x0010_c524, 0x0010_c55c, 0x0010_c584, 0x0010_c5ac, 0x0010_c5d4],
                );
                put_words(
                    c,
                    0x0045_6ab0,
                    &[0x0010_c638, 0x0010_c650, 0x0010_c664, 0x0010_c678, 0x0010_c688],
                );
                put_words(
                    c,
                    0x0045_6ad0,
                    &[0x0010_c6cc, 0x0010_c6ec, 0x0010_c700, 0x0010_c714, 0x0010_c728],
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
            c.bus.write32(GAME_GP - off, pick(&mut *r, &[f(0.0), f(0.5), f(2.0), f(-1.0)]));
        }
        for i in 0..6u32 {
            c.bus.write32(0x004f_f030 + 4 * i, r.next_u32());
        }

        c.bus.write32(SPIN_AI + 0x18, SPIN_RIDER);
        c.bus.write32(SPIN_RIDER + 0x788, SPIN_AIR);
        c.bus.write32(SPIN_AIR + 0xac, [1u32, 3, 0, 2][(seed % 4) as usize]);
        c.bus.write32(SPIN_AIR + 0xa0, 0);
        c.bus.write32(SPIN_AIR + 0x98, pick(&mut *r, &[f(0.3), f(0.6), f(0.7), f(20.0), f(16.0)]));
        c.bus.write32(SPIN_RIDER + 0x484, pick(&mut *r, &[0, 5, 29, 30, 31, 100]));
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
        c.bus.write32(ai + 0xe18, if r.next_u32() % 4 == 0 { 1 } else { 0 });
        c.bus.write32(ai + 0xe0c, pick(&mut *r, &[0xffff_ffff, 0xffff_ffff, 3]));
        c.bus.write32(ai + 0xe20, pick(&mut *r, &[0, 0, 1, 2]));
        c.bus.write32(ai + 0xe24, r.next_u32() % 2);
        c.bus.write32(ai + 0xe28, r.next_u32() % 2);
        c.bus.write32(ai + 0xe2c, pick(&mut *r, &[f(0.0), f(10.0), f(-5.0)]));
        c.bus.write32(ai + 0xe30, pick(&mut *r, &[f(0.0), f(30.0), f(100.0)]));
        c.bus.write32(ai + 0xe34, pick(&mut *r, &[f(0.0), f(-1.0), f(0.65), f(50.0)]));
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
        check_lohi(ee::s00::f_0010b250__generated, computer_actor_rider_think_spin, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0010_b250, 4);
            think_spin_world(c, &mut r, seed);
        });
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
}
