//! `src/ai/ridermetrix.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::Callee;
use crate::call::Frame;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Flow;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::VSlot;
use ssx3_ee_rt::funcs::call_indirect;
use crate::events::back_edge_unwinds;
pub(crate) use crate::game::ai::rider::p;
pub(crate) use crate::game::ai::rider::gp;
pub(crate) use crate::game::ai::rider::lw;
pub(crate) use crate::game::ai::rider::fr;
pub(crate) use crate::game::ai::rider::setf;
pub(crate) use crate::game::ai::rider::ldf;
pub(crate) use crate::game::ai::rider::stf;
pub(crate) use crate::game::ai::rider::li_f;
pub(crate) use crate::game::ai::rider::fcc;
pub(crate) use crate::game::ai::rider::qcopy;
pub(crate) use crate::game::ai::rider::dot_tail;
pub(crate) use crate::game::ai::rider::v_dot_self;
pub(crate) use crate::game::ai::rider::v_length;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::fe::festateload::glo;
pub(crate) use crate::game::ai::ai::virtual_call_slot;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::fe::debugfe::at as at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::main::gamemode::low as low_2;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::wscript::wscriptman::sw;
pub(crate) use crate::game::fe::feasyncfile::lw_8 as lw_2;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_73b8, rider_metrix_clear_hud_messages, "RiderMetrix_ClearHUDMessages"),
    (0x0011_74e8, rider_metrix_load_state, "RiderMetrix_LoadState"),
    (0x0011_7520, rider_metrix_set_base_score, "RiderMetrix_SetBaseScore"),
    (0x0011_7540, rider_metrix_reset, "RiderMetrix_Reset"),
    (0x0011_76f8, rider_metrix_clear_peak_a4, "RiderMetrix_ClearPeakA4"),
    (0x0011_7708, rider_metrix_update_peak_a4, "RiderMetrix_UpdatePeakA4"),
    (0x0011_7900, rider_metrix_get_trick_points, "RiderMetrix_GetTrickPoints"),
    (0x0011_7948, rider_metrix_get_rounded_trick_points, "RiderMetrix_GetRoundedTrickPoints"),
    (0x0011_7a58, rider_metrix_find_hud_slot, "RiderMetrix_FindHUDSlot"),
    (0x0011_7c28, rider_metrix_update, "RiderMetrix_Update"),
    (0x0011_7fe0, rider_metrix_update_trick_hud, "RiderMetrix_UpdateTrickHUD"),
    (0x0011_9210, rider_metrix_check_score_milestone, "RiderMetrix_CheckScoreMilestone"),
    (0x0011_9368, rider_metrix_on_rider_reset, "RiderMetrix_OnRiderReset"),
    (0x0011_93e0, rider_metrix_on_race_finished, "RiderMetrix_OnRaceFinished"),
    (0x0011_9400, rider_metrix_on_knockdown_opponent, "RiderMetrix_OnKnockdownOpponent"),
    (0x0011_9448, rider_metrix_record_best_jump, "RiderMetrix_RecordBestJump"),
    (0x0011_94c0, rider_metrix_on_split_marker, "RiderMetrix_OnSplitMarker"),
    (0x0011_9958, rider_metrix_chain_uber_grind, "RiderMetrix_ChainUberGrind"),
    (0x0011_99f8, rider_metrix_start_board_press, "RiderMetrix_StartBoardPress"),
    (0x0011_9a38, rider_metrix_finish_board_press, "RiderMetrix_FinishBoardPress"),
    (0x0011_9ad8, rider_metrix_set_press_spin, "RiderMetrix_SetPressSpin"),
    (0x0011_9bf0, rider_metrix_start_handplant, "RiderMetrix_StartHandplant"),
    (0x0011_9c98, rider_metrix_start_spin_trick, "RiderMetrix_StartSpinTrick"),
    (0x0011_a110, rider_metrix_on_mission_completed, "RiderMetrix_OnMissionCompleted"),
];

/// Direct calls (same shard; these go through the shard's override slots, so
/// an installed replacement is reached as well).
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_SUB_STATE;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::ai::rider::callee::RIDER_UPDATE_BOARD_LEAN;
    pub(crate) use crate::game::ai::rider::callee::RIDER_SOLVE_LEG_IK;
    pub(crate) use crate::game::ai::rider::callee::RIDER_UPDATE_SPHERE_TREE;
    pub(crate) use crate::game::ai::rider::callee::RIDER_UPDATE_SPEED_ANIMS;
    pub(crate) use crate::game::ai::rider::callee::AI_GET_FRAME_COUNTER;
    pub(crate) use crate::game::ai::rider::callee::RIDER_METRIX_BANK_COMBO;
    pub(crate) use crate::game::ai::rider::callee::RIDER_METRIX_CHECK_SCORE_MILESTONE;
    pub(crate) use crate::game::ai::rider::callee::RIDER_METRIX_UPDATE_TRICK_HUD;
    pub(crate) use crate::game::ai::rider::callee::CHUDFX_UPDATE;
    pub(crate) use crate::game::ai::rider::callee::RIDER_SPIN_SUB_APPLY_POSE_ROTATION;
    pub(crate) use crate::game::ai::rider::callee::RIDER_MODEL_APPLY_POSE;
    pub(crate) use crate::game::ai::rider::callee::QUAT_SLERP;
    pub(crate) use crate::game::ai::rider::callee::MATH_SIN_COS;
    pub(crate) use crate::game::ai::rider::callee::MATH_ASIN;
    pub(crate) use crate::game::ai::rider::callee::ANIM_CONTROLLER_GET_LAYER_SEQUENCE;
    pub(crate) use crate::game::ai::rider::callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ID;
    pub(crate) use crate::game::ai::rider::callee::ANIM_CONTROLLER_PLAY_ANIM;
    pub(crate) use crate::game::ai::rider::callee::MEMSET;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_SYSTEM_SET_PLAYER_TRICK_SCORE;
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
    pub(crate) use crate::game::movie::movieplayer::callee::BE_LIBRARY_GET_INSTANCE;
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
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EVENT_SELECT_HANDLE_INPUT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MC_MANAGER_GET_INSTANCE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_UPDATE_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_DUMMY_OBJ_READ_FIELDS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EA_MEM_COPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::TEX_MAN_SET_ALLOC_FLAGS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::GRAPHICS_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_STATE_MGR_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_NODE_UNLINK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_RESET_SECTIONS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STRLEN_AND_STRCPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::CBXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::LOBBY_MAN_REQUEST_STATS_PAGE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SSX_APP_BOOT_LOAD;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::USTR_FROM_ASCII;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_SET_TRACK_CONTROL_VALUE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_INFO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_NAME;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STREAM_BUFFER_GET_STATUS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MENU_SYS_GET_INPUT_CODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::C_WORLD_SOUND_UNLINK_EMITTERS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RIDER_METRIX_BANK_COMBO as RIDER_METRIX_BANK_COMBO_2;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_RESUME;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_TRIGGER_MANAGER_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_JUMP_TO_TASK;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_GOTO_TASK;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_MAN_RESET_COMMAND_QUEUE;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V43;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V42;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_HANDLE_EVENT;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_CACHE_SET_DATA;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SELECT_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::BOLT_FILE_CACHE_INIT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_PREVIEW_APPLY_TRANSFORM;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_SAVE_DATA_SIZE;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_METRIX_SET_SPIN_AMOUNT;
    pub(crate) use crate::game::fe::festateload::callee::STRLEN_COPY;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::fe::festateload::callee::I_SIGNAL_SEMA;
    pub(crate) use crate::game::fe::festateload::callee::EA_MEM_FREE;
    pub(crate) use crate::game::fe::festateload::callee::UI_STATE_V13;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_UPDATE;
    pub(crate) use crate::game::fe::festateload::callee::MEM_CARD_BASE_DTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_SHOW_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_DESTROY_MISSION_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_CREATE_COMPUTER_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_REWARD_INT_COUNT_SET_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::fe::ovstatepause::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_BEGIN_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_FORMAT_MONEY;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_ACCOUNT_NAME;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_PASSWORD;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_MOVE_DOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::EVENT_SELECT_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::ovstatepause::callee::VID_ENGINE_PLAYER_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::ovstatepause::callee::MOUNTAIN_ROOM_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_UPDATE_2 as UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_ON_ACTIVATE_2 as UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::PLAY_FE_SOUND;
    pub(crate) use crate::game::fe::ovstatepause::callee::MEM_CARD_STATE_BASE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::ovstatepause::callee::OV_SCREEN_SESSION_DESTROY_INDICATORS;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_SHUTDOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_ASYNC_MANAGER_CANCEL;
    pub(crate) use crate::game::fe::ovstatepause::callee::SECTION_MAN_REQUEST_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRLEN;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PS2_MEMCARD_CREATE;
    pub(crate) use crate::game::sound::bankmonitor::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUDDY_MAN_CONNECT;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;
    pub(crate) use crate::game::sound::bankmonitor::callee::SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMCARD_REQUEST_GET_INFO_NOW;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_VEC3;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_FLOAT_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_LIST_ADD_CHOICE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE as SSX_AUDIO_SYSTEM_GET_INSTANCE_3;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_MEMORY_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::sound::bankmonitor::callee::SPEECH_SYSTEM_STOP_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BANK_MONITOR_TIMER_KILL;
    pub(crate) use crate::game::sound::bankmonitor::callee::SNDSTRM_SET_PREBUFFER;
    pub(crate) use crate::game::sound::bankmonitor::callee::AUDIO_RAND;
    pub(crate) use crate::game::sound::bankmonitor::callee::SONG_INSTANCE_GET_REQUEST_STATUS;
    pub(crate) use crate::game::sound::bankmonitor::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::sound::bankmonitor::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_MAN_CALL_TASK_FUNC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_REMOVE;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INTERFACE as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_MISSION_INT_GET_COLLECTED_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::ADF_DATA_SOURCE_SET_DATA;
    pub(crate) use crate::game::sound::bankmonitor::callee::EA_FILE_LOAD_DECOMPRESSED;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_ANIM_MAP_FIND_HASH;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMSET_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::sound::bankmonitor::callee::TEX_MAN_FREE_TEXTURE;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_MENU_SET_NAV_BUTTONS;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::sound::bankmonitor::callee::SECTION_ALLOCATOR_DTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::STREAM_FILE_RELEASE_CHUNK;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_3;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_CLEAR_PEAK_A4 as RIDER_METRIX_CLEAR_PEAK_A4_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_GET_EVENT_RECORD_INDEX;
    pub(crate) use crate::game::sound::bankmonitor::callee::BOLTS_FREE_DB;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
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
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT as BE_NEW_PLAYER_INT_GET_SLOT_2;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UNLOAD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_ASYNC_MANAGER_SET_FILE_STATUS;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::main::gamemode::callee::RUN_LIST_SEEK;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_LIST_ON_SEQ_ADVANCE;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID as SCRIPT_ENGINE_GET_SEQ_PROCESS_ID_2;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_REQUEST_INSTANT_REPLAY;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII as USTR_TO_ASCII_2;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_2;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_4;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR as MULTI_PARTICLE_DTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::STRLEN as STRLEN_2;
    pub(crate) use crate::game::main::gamemode::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;


    pub const GET_ROUNDED_TRICK_POINTS: Callee = Callee {
        addr: 0x0011_7948,
        f: ee::s00::RiderMetrix_GetRoundedTrickPoints,
    };

    pub const GET_SCORE_TIER: Callee = Callee {
        addr: 0x0011_9310,
        f: ee::s00::RiderMetrix_GetScoreTier,
    };

    pub const SHOW_HUD_NUMBER_2: Callee = Callee {
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

    pub const REMOVE_HUD_MESSAGE_2: Callee = Callee {
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

/// `RiderBase` (and its owner/board sub-objects) fields the HUD reads.
pub(crate) mod rider {
    pub(crate) use crate::game::ai::computer::rider::BOOST_2 as BOOST;
    pub(crate) use crate::game::ai::computer::rider::BOOST_TIMER;
    pub(crate) use crate::game::ai::computer::rider::BOOST_COUNT;
    pub(crate) use crate::game::ai::computer::rider::BOOST_STATE;
    pub(crate) use crate::game::ai::computer::rider::BOOST_MASK;
    pub(crate) use crate::game::ai::computer::rider::BOOST_FLAG;
    pub(crate) use crate::game::ai::computer::rider::CHAR_ID;
    pub(crate) use crate::game::ai::computer::rider::CHAR_COMP;
    pub(crate) use crate::game::ai::computer::rider::METRIC_SCALE;

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

/// `gp`-relative globals used here.
pub(crate) mod g {
    pub(crate) use crate::game::fe::festateload::g::WORLD_TRIGGER_MANAGER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER_END;
    pub(crate) use crate::game::fe::festateload::g::BE_OPTION_SINGLETON;
    pub(crate) use crate::game::fe::festateload::g::FE_BASE_BACK;
    pub(crate) use crate::game::fe::festateload::g::BOLT_FILE_CACHE;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STR_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_SCREEN_START_THREAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CLOSE_UNPINNED_POPUPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCASECMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPRINTF;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEMSET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_POP_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_PUSH_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::FE_NET_SEND;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MC_STATUS_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW_BOLTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::HUD_SHOW_NUMBER_DELAYED;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPATIAL_INSERT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::POSITION_MODIFIER_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_PAINTER_2D;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_OBJECT_BY_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BANK_MONITOR_ADD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_TEARDOWN;
}

// ---------------------------------------------------------------------------
// Field offsets.

/// `RiderMetrix` fields the trick HUD reads and writes (the reference names
/// from `sMetrix17C28`; the ones the C++ left unnamed keep their offset).
pub(crate) mod mx {

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

/// Cross-shard calls the generator emitted as `call_indirect`.
pub(crate) mod table {

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


/// `void RiderMetrix_SetBaseScore(RiderMetrix *m, int base)` (0x117520):
/// `delta = base - m->base (+0x1c8)`, store the new base, and add `delta` to
/// the running total at +0x198.
pub fn rider_metrix_set_base_score(c: &mut Cpu) {
    c.cycles += 7;
    let m = Addr(abi::arg(c, 0));
    let old_base = m.off(0x1c8).lw(c);
    c.g[V0][0] = old_base;
    let total = m.off(0x198).lw(c);
    c.g[V1][0] = total;
    let delta = abi::arg(c, 1).wrapping_sub(old_base as u32);
    c.g[V0][0] = sx(delta);
    m.off(0x1c8).sw(c, c.g[A1][0]);
    let new_total = (total as u32).wrapping_add(delta);
    c.g[V1][0] = sx(new_total);
    m.off(0x198).sw(c, c.g[V1][0]);
    abi::ret(c);
}


/// `int RiderMetrix_GetRoundedTrickPoints(RiderMetrix *m)` (0x117948): compute
/// `round_down((m->pts (+0x1c4) * m->scale (+0x14) * k + 5.0), 10)`, the
/// division being a signed 32-bit `div` whose remainder is subtracted.
pub fn rider_metrix_get_rounded_trick_points(c: &mut Cpu) {
    let m = Addr(abi::arg(c, 0));

    c.cycles += 5;
    let scale = m.off(0x14).read_fpu(c); // f2
    c.f[2] = scale.0;
    c.g[A1][0] = 0xa;
    let points = m.off(0x1c4).read_fpu(c); // f0
    c.f[0] = points.0;
    // (the original's `a1 == 0` guard is dead: a1 is 10)

    c.cycles += 12;
    let k = Addr((c.g[GP][0] as u32).wrapping_sub(0x7b08)).read_fpu(c); // f3
    c.f[3] = k.0;
    let scaled = points * scale;
    c.f[0] = scaled.0;
    c.g[AT][0] = 0x40a0_0000; // 5.0
    c.f[1] = c.g[AT][0] as u32; // mtc1 f1, at
    let scaled = scaled * k;
    c.f[0] = scaled.0;
    let biased = scaled + FpuF32(c.f[1]); // add.s f0, f0, f1
    c.f[0] = biased.0;

    let v = biased.to_i32(); // cvt.w.s f1, f0
    c.f[1] = v as u32;
    let vx = sx(v as u32); // mfc1 v0, f1
    c.g[V0][0] = vx;

    let (_, rem) = abi::div(c, v as u32, 10); // div v0, a1
    c.g[V1][0] = sx(rem);
    c.g[V0][0] = sx((vx as u32).wrapping_sub(rem));
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// RiderMetrix_Update.

/// `void RiderMetrix_Update(RiderMetrix* m)` (0x117c28): per-frame scoring
/// of the rider `+0x1ac`. With `dt = frame_dt * gp-0x7b00` and the speed:
/// the bank-combo timer `+0xa4` runs down (`RiderMetrix_BankCombo` when it
/// expires, unless in a main state or sub-state 1); the running timers
/// (`+0x30` air, `+0x40`, `+0x24` distance, `+0x2c`, `+0x6c`, `+0x78`)
/// advance while non-negative, add to the score `+0x14` and report score
/// milestones (`RiderMetrix_CheckScoreMilestone`, bonus into `+0x84`);
/// distance/peak/frame statistics; the trick HUD and HUD effects; then
/// the score goes to the audio system.
pub fn rider_metrix_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x80);
        let dt_scale = ldf(c, 0, gp(c, -0x7b00));
        frame.save(c, S0, 0x50);
        frame.save_fpr(c, 21, 0x68);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 22, 0x70);
        c.f[21] = 0;
        let m = r32(c, S0);
        c.g[A0][0] = lw(c, p(m, 0x1ac));
        let r = r32(c, A0);
        let bank = ldf(c, 1, p(m, 0xa4));
        let speed = v_length(c, p(r, 0x1e0), V0, 22);
        let frame_dt = ldf(c, 2, p(r, 0x300));
        let zero = fr(c, 21);
        let banking = fcc(c, zero.le(bank));
        let dt = setf(c, 20, frame_dt * dt_scale);

        // The bank-combo timer. 0: timer path (reload air, clear f21),
        // 1: sub-state 1 (clear f21), 2: in a main state.
        let mut path = 0;
        if banking {
            c.cycles += 4;
            callee::RIDER_GET_MAIN_STATE.call(c, 0x0011_7ca4)?;
            if c.g[V0][0] != 0 {
                ldf(c, 0, p(m, 0x30));
                path = 2;
            } else {
                c.cycles += 5;
                c.g[A0][0] = lw(c, p(m, 0x1ac));
                callee::RIDER_GET_SUB_STATE.call(c, 0x0011_7cb4)?;
                c.g[V1][0] = 1;
                if c.g[V0][0] == c.g[V1][0] {
                    ldf(c, 0, p(m, 0x30));
                    path = 1;
                } else {
                    c.cycles += 6;
                    let left = ldf(c, 0, p(m, 0xa4));
                    let left = setf(c, 0, left - dt);
                    let zero = fr(c, 21);
                    let expired = fcc(c, left.le(zero));
                    stf(c, 0, p(m, 0xa4));
                    if expired {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S0][0];
                        callee::RIDER_METRIX_BANK_COMBO.call(c, 0x0011_7ce0)?;
                    }
                }
            }
        }
        if path == 0 {
            c.cycles += 1;
            ldf(c, 0, p(m, 0x30));
        }
        if path <= 1 {
            c.cycles += 1;
            c.f[21] = 0;
        }

        // Air time.
        c.cycles += 4;
        let air = fr(c, 0);
        let zero = fr(c, 21);
        c.g[A2][0] = 0x46_0000;
        if fcc(c, zero.le(air)) {
            c.cycles += 11;
            let air = setf(c, 0, air + dt);
            c.g[A2][0] = sx(0x0045_9e00);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(p(m, 0x88));
            c.g[A3][0] = 0x1c;
            setf(c, 12, air);
            stf(c, 0, p(m, 0x30));
            add_milestone_score(c, m, 0x0011_7d18)?;
        }

        // Timer +0x40, scoring +0x3c per frame.
        c.cycles += 5;
        let t = ldf(c, 2, p(m, 0x40));
        let zero = fr(c, 21);
        c.g[A2][0] = 0x46_0000;
        if fcc(c, zero.le(t)) {
            c.cycles += 15;
            let score = ldf(c, 0, p(m, 0x14));
            let t = setf(c, 2, t + dt);
            let bonus = ldf(c, 1, p(m, 0x3c));
            c.g[A2][0] = sx(0x0045_9e20);
            c.g[A0][0] = c.g[S0][0];
            setf(c, 0, score + bonus);
            setf(c, 12, t);
            stf(c, 2, p(m, 0x40));
            c.g[A1][0] = sx(p(m, 0x8c));
            c.g[A3][0] = 0x20;
            stf(c, 0, p(m, 0x14));
            add_milestone_score(c, m, 0x0011_7d68)?;
        }

        // Distance +0x24 (speed * dt), and +0x1c while descending.
        c.cycles += 5;
        let t = ldf(c, 2, p(m, 0x24));
        let zero = fr(c, 21);
        if fcc(c, zero.le(t)) {
            c.cycles += 16;
            let step = setf(c, 3, speed * dt);
            let rate = ldf(c, 0, gp(c, -0x7afc));
            let score = ldf(c, 1, p(m, 0x14));
            c.g[V1][0] = lw(c, p(m, 0x1ac));
            let rv = r32(c, V1);
            let inc = setf(c, 0, step * rate);
            setf(c, 2, t + step);
            setf(c, 1, score + inc);
            stf(c, 2, p(m, 0x24));
            stf(c, 1, p(m, 0x14));
            let sp = sp(c);
            qcopy(c, V0, p(rv, 0x1c0), sp);
            let vz = ldf(c, 0, p(sp, 8));
            let zero = fr(c, 21);
            let down = fcc(c, vz.lt(zero));
            c.g[A2][0] = 0x46_0000;
            if down {
                c.cycles += 5;
                let k = ldf(c, 0, gp(c, -0x7af8));
                let d = ldf(c, 1, p(m, 0x1c));
                let k = setf(c, 0, step * k);
                setf(c, 1, d + k);
                stf(c, 1, p(m, 0x1c));
            }
            c.cycles += 10;
            ldf(c, 12, p(m, 0x24));
            c.g[A2][0] = sx(0x0045_9f68);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(p(m, 0x98));
            c.g[A3][0] = 0x1d;
            add_milestone_score(c, m, 0x0011_7df4)?;
        }
        ldf(c, 2, p(m, 0x2c));

        // Timer +0x2c.
        c.cycles += 5;
        c.f[21] = 0;
        let t = fr(c, 2);
        let zero = fr(c, 21);
        c.g[A2][0] = 0x46_0000;
        if fcc(c, zero.le(t)) {
            timed_milestone(c, m, 0x2c, -0x7af4, 0x0045_9e40, 0x90, 0x1e, 0x0011_7e4c)?;
        }

        // Timer +0x6c (score only).
        c.cycles += 5;
        let t = ldf(c, 2, p(m, 0x6c));
        let zero = fr(c, 21);
        if fcc(c, zero.le(t)) {
            c.cycles += 8;
            let rate = ldf(c, 0, gp(c, -0x7af0));
            setf(c, 2, t + dt);
            let score = ldf(c, 1, p(m, 0x14));
            let inc = setf(c, 0, dt * rate);
            stf(c, 2, p(m, 0x6c));
            setf(c, 1, score + inc);
            stf(c, 1, p(m, 0x14));
        }
        ldf(c, 2, p(m, 0x78));

        // Timer +0x78.
        c.cycles += 4;
        let t = fr(c, 2);
        let zero = fr(c, 21);
        c.g[A2][0] = 0x46_0000;
        if fcc(c, zero.le(t)) {
            timed_milestone(c, m, 0x78, -0x7aec, 0x0045_9f08, 0x94, 0x1f, 0x0011_7ed0)?;
        }

        // Statistics: smoothed speed peak, distance, frames, top speed.
        c.cycles += 18;
        let avg = ldf(c, 3, p(m, 0x1a8));
        let k_keep = ldf(c, 0, gp(c, -0x7ae8));
        let k_new = ldf(c, 1, gp(c, -0x7ae4));
        let avg = setf(c, 3, avg * k_keep);
        let dist = ldf(c, 2, p(m, 0x164));
        let add = setf(c, 1, speed * k_new);
        let frames = ldf(c, 0, p(m, 0x168));
        let one = li_f(c, 4, 0x3f80_0000);
        setf(c, 2, dist + speed);
        let peak = ldf(c, 5, p(m, 0x160));
        let avg = setf(c, 3, avg + add);
        setf(c, 0, frames + one);
        stf(c, 2, p(m, 0x164));
        let new_peak = fcc(c, peak.lt(avg));
        stf(c, 3, p(m, 0x1a8));
        stf(c, 0, p(m, 0x168));
        if new_peak {
            c.cycles += 1;
            stf(c, 3, p(m, 0x160));
        }
        c.cycles += 5;
        let top = ldf(c, 0, p(m, 0x15c));
        if fcc(c, top.lt(speed)) {
            stf(c, 22, p(m, 0x15c));
        }

        // HUD.
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        c.g[S3][0] = sx(p(m, 0x1b8));
        callee::RIDER_METRIX_UPDATE_TRICK_HUD.call(c, 0x0011_7f48)?;
        c.g[V0][0] = lw(c, p(m, 0x1b0));
        let has_fx = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S3][0];
        if has_fx {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(m, 0x1b4));
            let none = (c.g[V0][0] as i64) <= 0;
            c.g[S1][0] = 0;
            if !none {
                c.cycles += 2;
                c.g[S2][0] = 0;
                loop {
                    c.cycles += 8;
                    let m = r32(c, S0);
                    c.g[A0][0] = lw(c, p(m, 0x1b0));
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    let fx = r32(c, A0).wrapping_add(r32(c, S2));
                    c.g[A0][0] = sx(fx);
                    callee::CHUDFX_UPDATE.call(c, 0x0011_7f78)?;
                    c.g[V0][0] = lw(c, p(m, 0x1b4));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x9c));
                    if !more {
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S3][0];
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0011_7f68);
                    }
                }
            }
        }

        // memset(m + 0x1b8, 0, 8); m->hud_dirty = 1; audio trick score.
        c.cycles += 20;
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        call_ptr(c, callee::MEMSET, 0x0011_7f98)?;
        c.g[V1][0] = 1;
        c.sw(p(m, 0x1c0), 1);
        call_ptr(c, callee::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0011_7fa4)?;
        ldf(c, 12, p(m, 0x14));
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = lw(c, p(m, 0x1ac));
        call_ptr(c, callee::SSX_AUDIO_SYSTEM_SET_PLAYER_TRICK_SCORE, 0x0011_7fb4)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 22, 0x70);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}


/// `RiderMetrix_CheckScoreMilestone(...)`, then `m->bonus (+0x84) += v0`.
pub(crate) fn add_milestone_score(c: &mut Cpu, m: u32, ret: u32) -> Flow {
    callee::RIDER_METRIX_CHECK_SCORE_MILESTONE.call(c, ret)?;
    c.g[V1][0] = lw(c, p(m, 0x84));
    let bonus = r32(c, V1).wrapping_add(r32(c, V0));
    c.g[V1][0] = sx(bonus);
    c.sw(p(m, 0x84), sx(bonus));
    Ok(())
}


/// A running timer (f2 = its value, non-negative; 16-cycle block):
/// `timer += dt`, `score += dt * rate`, then the milestone check on the
/// timer (`CheckScoreMilestone(m, m + slot, table, kind)`).
#[allow(clippy::too_many_arguments)]
pub(crate) fn timed_milestone(c: &mut Cpu, m: u32, timer: u32, rate_gp: i32, table: u32, slot: u32, kind: u64, ret: u32) -> Flow {
    c.cycles += 16;
    let dt = fr(c, 20);
    let t = fr(c, 2);
    let rate = ldf(c, 0, gp(c, rate_gp));
    let t = setf(c, 2, t + dt);
    let score = ldf(c, 1, p(m, 0x14));
    let inc = setf(c, 0, dt * rate);
    c.g[A2][0] = sx(table);
    setf(c, 12, t);
    stf(c, 2, p(m, timer));
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx(p(m, slot));
    setf(c, 1, score + inc);
    c.g[A3][0] = kind;
    stf(c, 1, p(m, 0x14));
    add_milestone_score(c, m, ret)
}


/// `int RiderMetrix_CheckScoreMilestone(Rider* self, int* idx,
/// const GradeStep* grades, int type)` (0x119210): when the step at `*idx`
/// has been reached, advance the index, drop the five HUD messages of the
/// previous tier and show the new score; returns the awarded value or 0.
pub fn rider_metrix_check_score_milestone(c: &mut Cpu) {
    c.cycles += 17;
    let frame = Frame::open(c, 0x50);
    let idx = abi::arg(c, 1);
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x30);
    frame.save_ra(c, 0);
    frame.save_fpr(c, 20, 0x40);
    c.mtc1(0, 0);
    c.g[V1][0] = c.lw(idx);
    c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(abi::arg(c, 2)));
    c.lwc1(20, r32(c, V0));
    c.c_lt_s(20, 0);
    let below_zero = c.fcc();
    c.g[S2][0] = c.g[A3][0];
    if !below_zero {
        c.cycles += 4;
        c.c_le_s(20, 12);
        if c.fcc() {
            // The step's threshold has been reached: award the milestone.
            c.lwc1(1, p(r32(c, S1), rider::METRIC_SCALE));
            c.cycles += 33;
            c.g[A0][0] = c.g[S1][0];
            c.lwc1(0, p(r32(c, V0), 0x04));
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 2);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.mul_s(0, 0, 1);
            c.sw(idx, c.g[V0][0]);
            c.add_s(0, 0, 2);
            c.cvt_w_s(1, 0);
            c.mfc1(16, 1);
            // Drop the five HUD messages of the old tier.
            c.g[A1][0] = 0x1c;
            c.g[A0][0] = c.g[S1][0];
            if callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_929c).is_err() {
                return;
            }
            c.g[A1][0] = 0x1d;
            c.g[A0][0] = c.g[S1][0];
            if callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_92a8).is_err() {
                return;
            }
            c.g[A1][0] = 0x1e;
            c.g[A0][0] = c.g[S1][0];
            if callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_92b4).is_err() {
                return;
            }
            c.g[A1][0] = 0x1f;
            c.g[A0][0] = c.g[S1][0];
            if callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_92c0).is_err() {
                return;
            }
            c.g[A1][0] = 0x20;
            c.g[A0][0] = c.g[S1][0];
            if callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_92cc).is_err() {
                return;
            }
            c.cvt_w_s(0, 20);
            c.mfc1(7, 0);
            c.g[AT][0] = 0x3fc0_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            if call_ptr(c, callee::SHOW_HUD_NUMBER, 0x0011_92ec).is_err() {
                return;
            }
            c.g[V0][0] = c.g[S0][0];
            return rider_metrix_leave(c, &frame);
        }
    }
    // Threshold negative, or not reached yet.
    c.cycles += 2;
    c.g[V0][0] = 0;
    rider_metrix_leave(c, &frame);
}


/// The epilogue of [`rider_metrix_check_score_milestone`].
#[inline(always)]
pub(crate) fn rider_metrix_leave(c: &mut Cpu, frame: &Frame) {
    c.cycles += 7;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x40);
    frame.close(c);
}


/// `void RiderMetrix_ClearPeakA4(RiderMetrix* m)` (0x1176f8): resets the peak
/// air metric at +0xa4 to −1 (an unset marker) and leaves −1.0f in f0.
pub fn rider_metrix_clear_peak_a4(c: &mut Cpu) {
    c.cycles += 4;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 0); // f0 = -1.0f
    let selfp = abi::arg(c, 0);
    c.swc1(0, selfp.wrapping_add(0xa4));
    abi::ret(c);
}


/// `void RiderMetrix_UpdatePeakA4(RiderMetrix* m, float seconds)` (0x117708):
/// raises the peak air metric at +0xa4 to the new value.
pub fn rider_metrix_update_peak_a4(c: &mut Cpu) {
    c.cycles += 4;
    let selfp = abi::arg(c, 0);
    let peak = selfp.wrapping_add(0xa4);
    c.lwc1(0, peak);
    c.max_s(0, 0, 12); // max(current, seconds)
    c.swc1(0, peak);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Forwards with extra arguments or a result the wrapper re-reads.

/// `float RiderMetrix_OnRaceFinished(this)` (0x001193e0): bank the combo, then
/// return 0.0 (the float is materialised with `mtc1`).
pub fn rider_metrix_on_race_finished(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, callee::RIDER_METRIX_BANK_COMBO_2, 0x0011_93f0)?;
        c.mtc1(0, 0);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void RiderMetrix_SetPressSpin(RiderMetrix* self, int on)` (0x119ad8):
/// record the press flag (+0x28) and add pi to the spin amount (+0x34).
pub fn rider_metrix_set_press_spin(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let base = c.g[V0][0] as u32;
        c.lwc1(0, glo(c, -0x7a0c));
        c.lwc1(12, at(base, 0x34));
        let on = c.g[A1][0];
        Addr(at(base, 0x28)).sw(c, on);
        let sum = crate::ps2f::FpuF32(c.f[12]) + crate::ps2f::FpuF32(c.f[0]);
        c.f[12] = sum.0;
        callee::RIDER_METRIX_SET_SPIN_AMOUNT.call(c, 0x0011_9af8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `RiderMetrix_LoadState(self, iface)` (0x1174e8): `iface->v02(self, 0x1ac)`.
pub fn rider_metrix_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0011_7510, Some(0x1ac)))
}


/// `float RiderMetrix_StartBoardPress(RiderMetrix* m, int a1)` (0x1199f8):
/// record the press (`+0x10`, `+0x28`), keep the peak at `+0x2c` at least
/// zero, clear the peak tracker and return 0.0 (f0 and `f20`).
pub fn rider_metrix_start_board_press(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x10);
        c.mtc1(0, 20);
        let this = crate::abi::r32(c, V0);
        c.lwc1(0, p(this, 0x2c));
        Addr(p(this, 0x28)).sw(c, c.g[A1][0]);
        c.max_s(0, 0, 20);
        Addr(p(this, 0x10)).sw(c, c.g[A1][0]);
        c.swc1(0, p(this, 0x2c));
        callee::RIDER_METRIX_CLEAR_PEAK_A4.call(c, 0x0011_9a24)?;
        c.mov_s(0, 20);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x10);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Rider metrics / BE.

/// `float RiderMetrix_OnKnockdownOpponent(RiderMetrix* self)` (0x119400):
/// bump the knockdown count at `+0x128` and pop it up as HUD number 0x2c.
pub fn rider_metrix_on_knockdown_opponent(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(1, 12);
        c.g[A1][0] = 0x2c;
        c.g[V0][0] = Addr(r32(c, V1)).off(0x128).lw(c);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.sw(p(r32(c, V1), 0x128), c.g[V0][0]);
        callee::RIDER_METRIX_SHOW_HUD_NUMBER.call(c, 0x0011_9430)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        frame.restore_ra(c, 0);
        leave(c, 0x10);
        Ok(())
    })
}


/// `float RiderMetrix_StartHandplant(RiderMetrix* self, u32 arg)` (0x119bf0):
/// arm the handplant timer (`+0x7c`, `+0x78`), set `+0x30` to -1 and advance
/// `+0x14` by the frame step at gp-0x7a04.
pub fn rider_metrix_start_handplant(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.lwc1(2, gp(c, -0x7a04));
        c.lwc1(0, p(r32(c, V0), 0x14));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 1);
        c.add_s(0, 0, 2);
        c.sw(p(r32(c, V0), 0x7c), c.g[A1][0]);
        c.swc1(1, p(r32(c, V0), 0x30));
        c.sw(p(r32(c, V0), 0x78), 0);
        c.swc1(0, p(r32(c, V0), 0x14));
        callee::RIDER_METRIX_CLEAR_PEAK_A4_2.call(c, 0x0011_9c24)?;
        c.mtc1(0, 0);
        frame.restore_ra(c, 0);
        leave(c, 0x10);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Rider metrics and state.

/// `void RiderMetrix_ClearHUDMessages(RiderMetrix* m)` (0x1173b8): mark every
/// HUD message (`m+0x1b0`, `m+0x1b4` entries of 0x9c bytes) as state 0x34.
pub fn rider_metrix_clear_hud_messages(c: &mut Cpu) {
    c.cycles += 3;
    let m = r32(c, A0);
    c.g[V0][0] = c.lw(m.wrapping_add(0x1b0));
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = c.lw(m.wrapping_add(0x1b4));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A2][0] = 0x34;
            c.g[A1][0] = 0;
            loop {
                c.cycles += 8;
                c.g[V0][0] = c.lw(m.wrapping_add(0x1b0));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.sw(r32(c, V0), c.g[A2][0]);
                c.g[V0][0] = c.lw(m.wrapping_add(0x1b4));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x9c));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0011_73d8);
                }
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void RiderMetrix_OnMissionCompleted(RiderMetrix* m, int n)` (0x11a110):
/// show the HUD number (1.5f), award 4 points and return 0.0f.
pub fn rider_metrix_on_mission_completed(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x30);
    c.g[AT][0] = 0x3fc0_0000;
    c.mtc1(1, 12);
    frame.save(c, S0, 0x20);
    c.g[A3][0] = 0;
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[A2][0] = c.g[S1][0];
    c.g[A1][0] = 0x1b;
    if callee::RIDER_METRIX_SHOW_HUD_NUMBER_2
        .call(c, 0x0011_a140)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A1][0] = 4;
    if callee::RIDER_METRIX_AWARD_POINTS
        .call(c, 0x0011_a150)
        .is_err()
    {
        return;
    }
    c.mtc1(0, 0);
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `void Rider_FinishHandplant(Rider* rider)` (0x119c38): finish a handplant
/// trick, reset the trick metrix, and return the trick result in `f0`.
pub fn rider_metrix_finish_handplant(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        frame.save_fpr(c, 20, 0x20);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[V0][0] = Addr(at_2(c, S0, 0x80)).lw(c);
        c.g[T1][0] = 0x1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.g[RA][0] = sx(0x0011_9c70);
        Addr(at_2(c, S0, 0x80)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0011_a228, 0x0011_9c70)?;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0011_7838, 0x0011_9c7c)?;
        Addr(at_2(c, S0, 0x30)).sw(c, 0);
        c.mov_s(0, 20);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}


/// `float RiderMetrix_GetTrickPoints(RiderMetrix* m)` (0x117900): the trick
/// point total held at +0x14.
pub fn rider_metrix_get_trick_points(c: &mut Cpu) {
    c.cycles += 2;
    let m = abi::arg(c, 0);
    c.lwc1(0, m.wrapping_add(0x14));
    abi::ret(c);
}


/// `RiderMetrix_RecordBestJump` (0x119448): count a jump; when the rider beat its
/// best jump height, show the number and remember it. Returns 0.0f.
pub fn rider_metrix_record_best_jump(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        c.g[V0][0] = lw_at(c, S0, 0x130);
        c.g[A0][0] = lw_at(c, S0, 0x1b0);
        c.g[V0][0] = low(c, V0).wrapping_add(1);
        let no_hud = c.g[A0][0] == 0;
        sw_at(c, S0, 0x130, c.g[V0][0]);
        if !no_hud {
            c.cycles += 5;
            c.lwc1(0, r32(c, S0).wrapping_add(0x18));
            c.c_lt_s(0, 20);
            let not_better = !c.fcc();
            c.g[A0][0] = low(c, A0).wrapping_add(0x270);
            if !not_better {
                c.cycles += 7;
                c.cvt_w_s(0, 20);
                c.mfc1(6, 0);
                c.mtc1(ZERO, 12);
                c.g[A1][0] = 4;
                c.g[A3][0] = 0;
                call_ptr(c, addr::HUD_SHOW_NUMBER_DELAYED, 0x0011_94a0)?;
                c.swc1(20, r32(c, S0).wrapping_add(0x18));
            }
        }
        c.cycles += 6;
        c.mtc1(ZERO, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}


/// `lwc1 f, off(reg)`.
#[inline]
pub(crate) fn lwc1_r(c: &mut Cpu, f: usize, reg: usize, off: u32) {
    let a = low_2(c, reg).wrapping_add(off);
    c.lwc1(f, a);
}


/// `swc1 f, off(reg)`.
#[inline]
pub(crate) fn swc1_r(c: &mut Cpu, f: usize, reg: usize, off: u32) {
    let a = low_2(c, reg).wrapping_add(off);
    c.swc1(f, a);
}


/// `void RiderMetrix_Reset(self)` (0x117540): reset every metrics counter,
/// HUD message, trick and combo, and the two float/flag fields.
pub fn rider_metrix_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low_2(c, SP).wrapping_sub(0x20));
        c.sq(16, sp_2(c).wrapping_add(0x10));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        callee::RIDER_METRIX_RESET_STATS.call(c, 0x0011_7554)?;
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_METRIX_CLEAR_HUD_MESSAGES.call(c, 0x0011_755c)?;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x50;
        c.g[A0][0] = sx(low_2(c, S0).wrapping_add(0xa8));
        call_ptr(c, callee::MEMSET_3E6448, 0x0011_756c)?;
        sw_r(c, S0, 0xf8, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_METRIX_RESET_TRICK.call(c, 0x0011_7578)?;
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_METRIX_RESET_COMBO.call(c, 0x0011_7580)?;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = sx(low_2(c, S0).wrapping_add(0x1b8));
        sw_r(c, S0, 0x1a8, 0);
        c.g[A1][0] = 0;
        swc1_r(c, 0, S0, 0xa4);
        c.g[A2][0] = 0x8;
        call_ptr(c, callee::MEMSET_3E6448, 0x0011_75a0)?;
        c.g[V0][0] = 0x1;
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        sw_r(c, S0, 0x1c0, c.g[V0][0]);
        c.lq(16, sp_2(c).wrapping_add(0x10));
        let t = low_2(c, RA);
        c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}


/// `float RiderMetrix_OnRiderReset(self, a1)` (0x119368): bank the pending
/// score, reset trick/combo, count the reset and return the penalty multiplier
/// applied to the model's speed value.
pub fn rider_metrix_on_rider_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx(low_2(c, SP).wrapping_sub(0x30));
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        callee::RIDER_METRIX_BANK_PENDING_SCORE.call(c, 0x0011_9384)?;
        c.g[V1][0] = lw_r(c, S1, 0x1a4);
        c.g[V1][0] = sx(low_2(c, V1).wrapping_add(low_2(c, V0)));
        sw_r(c, S1, 0x1a4, c.g[V1][0]);
        c.g[A0][0] = c.g[S1][0];
        callee::RIDER_METRIX_RESET_TRICK.call(c, 0x0011_9398)?;
        c.g[A0][0] = c.g[S1][0];
        callee::RIDER_METRIX_RESET_COMBO.call(c, 0x0011_93a0)?;
        c.g[V0][0] = lw_r(c, S1, 0x120);
        c.g[V0][0] = sx(low_2(c, V0).wrapping_add(1));
        let first = c.g[S0][0] == 0;
        sw_r(c, S1, 0x120, c.g[V0][0]);
        if first {
            c.cycles += 2;
            c.g[V0][0] = lw_r(c, S1, 0x1ac);
            c.lwc1(1, gpo(c, -0x7a24));
        } else {
            c.cycles += 3;
            c.lwc1(1, gpo(c, -0x7a28));
            c.g[V0][0] = lw_r(c, S1, 0x1ac);
        }
        c.cycles += 7;
        lwc1_r(c, 0, V0, 0x2f8);
        c.mul_s(0, 0, 1);
        c.lq(16, sp_2(c).wrapping_add(0x20));
        c.lq(17, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low_2(c, RA);
        c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Rider metrics HUD slots.

/// `int RiderMetrix_FindHUDSlot(RiderMetrix* self, int type)` (0x117a58): the
/// index type itself below 34; otherwise the first free slot at or after 35,
/// else the slot with the smallest value/total ratio (-1 when none).
pub fn rider_metrix_find_hud_slot(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    c.g[V0][0] = ((c.g[A1][0] as i64) < 34i64) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = c.g[A1][0];
        abi::ret(c);
        return;
    }
    c.g[A1][0] = Addr(p(this, 0x1b4)).lw(c);
    c.cycles += 6;
    c.g[V1][0] = 0x23;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 2); // f2 = 1.0
    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
    let empty = c.g[V0][0] == 0;
    c.g[A2][0] = u64::MAX;
    if !empty {
        c.cycles += 3;
        c.g[V0][0] = Addr(p(this, 0x1b0)).lw(c);
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x1554));
        loop {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, A0)).lw(c);
            c.g[V0][0] = c.g[V0][0] ^ 0x34;
            if c.g[V0][0] == 0 {
                // Free slot: take it.
                c.cycles += 2;
                c.g[V0][0] = c.g[V1][0];
                abi::ret(c);
                return;
            }
            c.cycles += 7;
            c.lwc1(1, p(r32(c, A0), 8));
            c.lwc1(0, p(r32(c, A0), 4));
            c.div_s(1, 1, 0);
            c.c_lt_s(1, 2);
            if !c.fcc() {
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.mov_s(2, 1);
                c.g[A2][0] = c.g[V1][0];
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx(p(r32(c, A0), 0x9c));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0011_7a98);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Rider metrics.

/// `void RiderMetrix_ChainUberGrind(RiderMetrix* self, int points)` (0x119958):
/// count the chained uber, finish the trick with no extra bonus and reset it,
/// then record the points as the trick's value.
pub fn rider_metrix_chain_uber_grind(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        c.g[A1][0] = 0;
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 21, 0x38);
        c.g[T0][0] = 0;
        c.lwc1(21, p(c.g[S0][0] as u32, 0x30));
        c.g[T1][0] = 0;
        c.g[V1][0] = lw(c, p(c.g[S0][0] as u32, 0x54));
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.lwc1(22, p(c.g[S0][0] as u32, 0x24));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        sw(c, p(c.g[S0][0] as u32, 0x54), c.g[V1][0]);
        sw(c, p(c.g[S0][0] as u32, 0x74), c.g[V0][0]);
        call_ptr(c, 0x0011_a228, 0x0011_99b4)?; // RiderMetrix_FinishTrick
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0011_7838, 0x0011_99c0)?; // RiderMetrix_ResetTrick
        c.swc1(21, p(c.g[S0][0] as u32, 0x30));
        c.mov_s(0, 20);
        sw(c, p(c.g[S0][0] as u32, 0x20), c.g[S1][0]);
        c.swc1(22, p(c.g[S0][0] as u32, 0x24));
        sw(c, p(c.g[S0][0] as u32, 0xc), c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });
}


/// `void RiderMetrix_FinishBoardPress(RiderMetrix* self)` (0x119a38): finish
/// the board press trick (with a zero bonus unless the press value at
/// `+0x30` is negative) and reset it.
pub fn rider_metrix_finish_board_press(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.mtc1(0, 0);
        frame.save(c, S0, 0x20);
        c.g[T1][0] = 1;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 20, 0x30);
        frame.save_fpr(c, 21, 0x38);
        c.lwc1(21, p(c.g[S0][0] as u32, 0x30));
        c.lwc1(22, p(c.g[S0][0] as u32, 0x24));
        c.c_le_s(0, 21);
        let not_negative = c.fcc();
        c.g[S1][0] = lw(c, p(c.g[S0][0] as u32, 0x20));
        if !not_negative {
            c.cycles += 1;
            c.g[T1][0] = 0;
        }
        c.cycles += 22;
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, 0x0011_a228, 0x0011_9a94)?;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0011_7838, 0x0011_9aa0)?;
        c.swc1(21, p(c.g[S0][0] as u32, 0x30));
        c.mov_s(0, 20);
        sw(c, p(c.g[S0][0] as u32, 0x20), c.g[S1][0]);
        c.swc1(22, p(c.g[S0][0] as u32, 0x24));
        sw(c, p(c.g[S0][0] as u32, 0xc), c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// Rider tricks and effects.

/// `float RiderMetrix_StartSpinTrick(self, int trick)` (0x119c98): announce a
/// spin trick, and when the rider is spinning start the trick's timer.
pub fn rider_metrix_start_spin_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, 0x0011_a168, 0x00119cc8)?;
        c.mtc1(0, 20);
        c.lwc1(0, p(r32(c, S0), 0x14));
        c.c_lt_s(20, 0);
        let spinning = c.fcc();
        c.g[V0][0] = 1;
        if spinning {
            c.cycles += 14;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 0);
            Addr(r32(c, S0)).sw(c, c.g[S1][0]);
            c.g[A0][0] = sx(p(r32(c, S0), 0x60));
            c.swc1(20, p(r32(c, S0), 0x34));
            c.g[A1][0] = 0;
            c.swc1(20, p(r32(c, S0), 0x38));
            c.g[A2][0] = 0xc;
            Addr(p(r32(c, S0), 4)).sw(c, 0);
            Addr(p(r32(c, S0), 0xc)).sw(c, 0);
            Addr(p(r32(c, S0), 0x10)).sw(c, 0);
            c.swc1(0, p(r32(c, S0), 0x40));
            Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x003e_6448, 0x00119d18)?;
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0011_76f8, 0x00119d20)?;
        c.mov_s(0, 20);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// RiderMetrix_OnSplitMarker (0x00001194c0).

/// `RiderMetrix_OnSplitMarker`: record a split time for the rider and update the split HUD.
pub fn rider_metrix_on_split_marker(c: &mut Cpu) {
    guest(c, |c| {
    'b_119588: {
    c.cycles += 16;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[S1][0] = c.g[A0][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S2][0] = c.g[A1][0];
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ac));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[RA][0] = sx(0x001194f8);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x001194f8 { return Err(crate::call::Unwound); }
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    if t {
    break 'b_119588;
    } else {
    c.cycles += 14;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ac));
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x38));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x3c));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[RA][0] = sx(0x00119520);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x00119520 { return Err(crate::call::Unwound); }
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = c.g[S2][0];
    call_ptr(c, 0x00238510, 0x00119530)?;
    let t = c.g[V0][0] == 0u64;
    c.g[A0][0] = c.g[S1][0];
    if t {
    break 'b_119588;
    } else {
    'b_119568: {
    c.cycles += 10;
    c.g[AT][0] = 0x40200000;
    c.mtc1(1, 12);
    c.g[A2][0] = c.g[S2][0];
    c.g[A1][0] = 0x29;
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x00117b88, 0x00119550)?;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ac));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x874));
    let t = c.g[V0][0] == 0u64;
    c.g[A0][0] = 0u64;
    if t {
    break 'b_119568;
    } else {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x87c));
    c.g[A0][0] = (0u64 < c.g[V0][0]) as u64;
    break 'b_119568;
    }
    }
    c.cycles += 2;
    let t = c.g[A0][0] == 0u64;

    if t {
    break 'b_119588;
    } else {
    c.cycles += 6;
    call_ptr(c, 0x0028b180, 0x00119578)?;
    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ac));
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = 0x1;
    call_ptr(c, 0x002a3ce8, 0x00119588)?;
    break 'b_119588;
    }
    }
    }
    }
    c.cycles += 7;
    c.mtc1(0, 0);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t; return Ok(());
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// Memory helpers (the same widths and sign extension as the original's).

/// `base + off` as a 32-bit address.
#[inline(always)]
pub(crate) fn at_3(c: &Cpu, base: usize, off: u32) -> u32 {
    (c.g[base][0] as u32).wrapping_add(off)
}


/// `lw_3 rd, off(base)`.
#[inline(always)]
pub(crate) fn lw_3(c: &mut Cpu, rd: usize, base: usize, off: u32) {
    let a = at_3(c, base, off);
    c.g[rd][0] = c.lw(a);
}


/// `sw_2 rs, off(base)`.
#[inline(always)]
pub(crate) fn sw_2(c: &mut Cpu, base: usize, off: u32, rs: usize) {
    let a = at_3(c, base, off);
    c.sw(a, c.g[rs][0]);
}


/// `swc1 fs, off(base)`.
#[inline(always)]
pub(crate) fn swc1(c: &mut Cpu, base: usize, off: u32, fs: usize) {
    let a = at_3(c, base, off);
    c.swc1(fs, a);
}


/// `lwc1 fs, off(base)`.
#[inline(always)]
pub(crate) fn lwc1(c: &mut Cpu, fs: usize, base: usize, off: u32) {
    let a = at_3(c, base, off);
    c.lwc1(fs, a);
}


/// `addiu rd, rs, imm` (32-bit, sign-extended).
#[inline(always)]
pub(crate) fn addiu(c: &mut Cpu, rd: usize, rs: usize, imm: u32) {
    c.g[rd][0] = sx((c.g[rs][0] as u32).wrapping_add(imm));
}


/// `addu rd, rd, rs`.
#[inline(always)]
pub(crate) fn add_reg(c: &mut Cpu, rd: usize, rs: usize) {
    c.g[rd][0] = sx((c.g[rd][0] as u32).wrapping_add(c.g[rs][0] as u32));
}


/// `sw_2 zero, off(base)`.
#[inline(always)]
pub(crate) fn sw_zero(c: &mut Cpu, base: usize, off: u32) {
    let a = at_3(c, base, off);
    c.sw(a, 0);
}


/// `lui at_3, hi(bits); mtc1 at_3, fs`: a float constant through `at_3`.
#[inline(always)]
pub(crate) fn fc(c: &mut Cpu, fs: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.f[fs] = bits;
}


/// A `gp`-relative float threshold the HUD compares against.
#[inline(always)]
pub(crate) fn gpf(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}


/// A HUD record address: slot `n` of the table at_3 `+0x1b0`.
#[inline(always)]
pub(crate) fn slot(n: u32) -> u32 {
    n * mx::SLOT_SIZE
}


/// `RiderMetrix_RemoveHUDMessage(m, id)`.
#[inline(always)]
pub(crate) fn remove_message(c: &mut Cpu, id: u64, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = id;
    callee::REMOVE_HUD_MESSAGE_2.call(c, ret)
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
        lw_3(c, V0, S1, mx::SLOTS);
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
                    callee::SHOW_HUD_NUMBER_2.call(c, 0x0011_8070)?;
                    call_ptr(c, table::AUDIO_SYSTEM, 0x0011_8078)?;
                    lw_3(c, A1, S1, mx::RIDER);
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
                    lw_3(c, A0, S1, mx::RIDER);
                    callee::GET_SUB_STATE.call(c, 0x0011_8090)?;
                    c.g[V1][0] = 8;
                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[A0][0] = c.g[S1][0];
                        break 'b_1180e4;
                    } else {
                        'b_1180e0: {
                            c.cycles += 6;
                            lw_3(c, V1, S1, mx::RIDER);
                            lw_3(c, V1, V1, rider::OWNER);
                            lw_3(c, V0, V1, rider::OWNER_MODE);
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
                                lw_3(c, A0, S1, mx::SLOTS);
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
                                lw_3(c, A0, S1, mx::RIDER);
                                callee::GET_SUB_STATE.call(c, 0x0011_8154)?;
                                c.g[V1][0] = 2;
                                if c.g[V0][0] == c.g[V1][0] {
                                    lw_3(c, A0, S1, mx::RIDER);
                                    break 'b_118194;
                                } else {
                                    // Sub-state 2: the "replay" messages 0xc and
                                    // 0xd are live, otherwise clear them.
                                    'b_118190: {
                                        c.cycles += 5;
                                        lw_3(c, A0, S1, mx::RIDER);
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
                                    lw_3(c, A0, S1, mx::RIDER);
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
                                    lw_3(c, V0, S1, mx::RIDER);
                                    if t {
                                        break 'b_1181c0;
                                    } else {
                                        c.cycles += 6;
                                        c.g[A1][0] = c.g[SP][0];
                                        lw_3(c, A0, V0, rider::OWNER);
                                        addiu(c, A0, A0, 0x1f0);
                                        callee::GET_HUD_ANGLE.call(c, 0x0011_81b8)?;
                                        c.mov_s(27, 0);
                                        break 'b_1181d4;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A1][0] = c.g[SP][0];
                                lw_3(c, A0, V0, rider::OWNER);
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
                                    lw_3(c, V0, SP, 0);
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
                                    lw_3(c, V1, S1, mx::SLOTS);
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
                                    lw_3(c, V0, V1, slot(13));
                                    c.g[V0][0] ^= 0x34;
                                    if c.g[V0][0] == 0 {
                                        lw_3(c, A0, S1, mx::SLOTS);
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
                                                        lw_3(c, A0, S1, mx::SLOTS);
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
                                                        lw_3(c, A0, S1, mx::SLOTS);
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
                                        lw_3(c, A0, S1, mx::SLOTS);
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
                                lw_3(c, A0, S1, mx::RIDER);
                                callee::GET_SUB_STATE.call(c, 0x0011_8394)?;
                                c.g[V1][0] = 2;
                                if c.g[V0][0] != c.g[V1][0] {
                                    lw_3(c, V0, S1, mx::RIDER);
                                    break 'b_1183dc;
                                } else {
                                    'b_1183bc: {
                                        c.cycles += 3;
                                        lw_3(c, V0, SP, 0);
                                        let t = c.g[V0][0] == 0;
                                        c.g[A0][0] = c.g[S1][0];
                                        if t {
                                            break 'b_1183bc;
                                        } else {
                                            c.cycles += 4;
                                            remove_message(c, 0xc, 0x0011_83b4)?;
                                            lw_3(c, V1, S1, mx::SLOTS);
                                            break 'b_1188fc;
                                        }
                                    }
                                    c.cycles += 8;
                                    fc(c, 12, 0xbf80_0000);
                                    c.g[A1][0] = 0xc;
                                    c.g[A2][0] = 0;
                                    c.g[A3][0] = 0;
                                    callee::SHOW_HUD_NUMBER_2.call(c, 0x0011_83d4)?;
                                    lw_3(c, V1, S1, mx::SLOTS);
                                    break 'b_1188fc;
                                }
                            }
                            // The board-wake effect (vtable `rider+0x230`): clear
                            // messages 0xc/0xd when it reports a wake.
                            'b_11841c: {
                                c.cycles += 8;
                                c.g[A1][0] = c.g[SP][0] | 0x4;
                                sw_zero(c, SP, 0x4);
                                lw_3(c, A0, V0, rider::OWNER);
                                addiu(c, A0, A0, 0x230);
                                call_ptr(c, table::BOARD_WAKE_CLEAR, 0x0011_83f0)?;
                                lw_3(c, V0, SP, 0x4);
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
                                    callee::SHOW_HUD_NUMBER_2.call(c, 0x0011_843c)?;
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
                            // table, at_3 the offset the board reports.
                            'b_1185b0: {
                                c.cycles += 24;
                                lw_3(c, V0, S1, mx::RIDER);
                                c.g[S6][0] = 0x9c;
                                fc(c, 26, 0x3f80_0000);
                                c.g[S7][0] = 0x34;
                                lw_3(c, A0, V0, rider::OWNER);
                                addiu(c, A0, A0, 0x230);
                                call_ptr(c, table::BOARD_WAKE_A, 0x0011_8554)?;
                                lw_3(c, V0, S1, mx::RIDER);
                                c.mov_s(24, 0);
                                lw_3(c, A0, V0, rider::OWNER);
                                addiu(c, A0, A0, 0x230);
                                call_ptr(c, table::BOARD_WAKE_B, 0x0011_8568)?;
                                c.mult(0, 2, 16, 22);
                                lw_3(c, A0, S1, mx::SLOTS);
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
                                    lw_3(c, A0, S1, mx::SLOTS);
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
                                    lw_3(c, A0, S1, mx::SLOTS);
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
                                    lw_3(c, A0, S1, mx::SLOTS);
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
                                    lw_3(c, A0, S1, mx::SLOTS);
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
                                lw_3(c, V1, S1, mx::SLOTS);
                                lw_3(c, V0, V1, slot(14));
                                c.g[V0][0] ^= 0x34;
                                let t = c.g[V0][0] == 0;
                                c.g[S2][0] = 0;
                                if t {
                                    break 'b_118634;
                                } else {
                                    c.cycles += 1;
                                    lw_3(c, S2, V1, slot(14) + 0xc);
                                    break 'b_118634;
                                }
                            }
                            // ---------------------------------------------------
                            // The four bar/percentage messages, chosen from the
                            // wake peak (f25 = |f21|).
                            c.cycles += 3;
                            lw_3(c, V0, SP, 0);
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
                        callee::REMOVE_HUD_MESSAGE_2.call(c, 0x0011_8888)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = 0xe;
                        callee::REMOVE_HUD_MESSAGE_2.call(c, 0x0011_8894)?;
                        lw_3(c, V1, S1, mx::SLOTS);
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
                    callee::SHOW_HUD_NUMBER_2.call(c, 0x0011_88b8)?;
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
                        lw_3(c, A2, S1, mx::BONUS);
                        c.mov_s(12, 21);
                        lw_3(c, A0, S1, mx::SLOTS);
                        c.g[A3][0] = c.g[V0][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A1][0] = 2;
                        addiu(c, A0, A0, slot(2));
                        callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_88f8)?;
                        break 'b_1188f8;
                    }
                }
                c.cycles += 1;
                lw_3(c, V1, S1, mx::SLOTS);
                break 'b_1188fc;
            }
            // ---------------------------------------------------------------
            // The boost meter bar: slot 5's fraction (f4), slot 6's value.
            'b_11898c: {
                'b_118918: {
                    c.cycles += 4;
                    lw_3(c, V0, V1, slot(5));
                    c.g[V0][0] ^= 0x34;
                    let t = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[V1][0];
                    if t {
                        break 'b_118918;
                    } else {
                        c.cycles += 3;
                        lw_3(c, V0, S1, mx::RIDER);
                        lwc1(c, 4, V0, rider::BOOST_STRENGTH);
                        break 'b_11898c;
                    }
                }
                'b_11893c: {
                    'b_118938: {
                        c.cycles += 5;
                        lw_3(c, V0, S1, mx::RIDER);
                        c.g[A0][0] = 3;
                        lw_3(c, V1, V0, rider::BOOST_MODE);
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
                    lw_3(c, V0, A1, slot(6));
                    c.g[V0][0] ^= 0x34;
                    if c.g[V0][0] != 0 {
                        lwc1(c, 1, A1, slot(6) + 4);
                        break 'b_1189a8;
                    } else {
                        c.cycles += 3;
                        lw_3(c, V0, S1, mx::RIDER);
                        lwc1(c, 20, V0, rider::BOOST_STRENGTH);
                        break 'b_1189f0;
                    }
                }
                'b_1189ec: {
                    'b_1189d8: {
                        c.cycles += 10;
                        lwc1(c, 0, A1, slot(6) + 8);
                        lw_3(c, V0, S1, mx::RIDER);
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
                    lw_3(c, A0, S1, mx::SLOTS);
                    c.mov_s(12, 4);
                    c.g[A1][0] = 5;
                    c.g[A2][0] = 0;
                    addiu(c, A0, A0, slot(5));
                    c.g[A3][0] = 0;
                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8a0c)?;
                    lw_3(c, A0, S1, mx::SLOTS);
                    c.mov_s(12, 20);
                    c.g[A1][0] = 6;
                    c.g[A2][0] = 0;
                    addiu(c, A0, A0, slot(6));
                    c.g[A3][0] = 0;
                    callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8a28)?;
                    lw_3(c, V0, S1, mx::RIDER);
                    lw_3(c, V1, V0, rider::BOOST_LEVEL);
                    if c.g[V1][0] != 0 {
                        lw_3(c, V1, S1, mx::SLOTS);
                        break 'b_118a4c;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = 8;
                        callee::REMOVE_HUD_MESSAGE_2.call(c, 0x0011_8a44)?;
                        lw_3(c, A0, S1, mx::RIDER);
                        break 'b_118afc;
                    }
                }
                'b_118a70: {
                    c.cycles += 4;
                    lw_3(c, V0, V1, slot(8));
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
                        lw_3(c, V1, S1, mx::RIDER);
                        c.g[A1][0] = 0xa;
                        lw_3(c, A0, S1, mx::SLOTS);
                        lw_3(c, V0, V1, rider::BOOST_LEVEL);
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
                    lw_3(c, A0, S1, mx::SLOTS);
                    break 'b_118ae8;
                }
                c.cycles += 5;
                c.g[A1][0] = 8;
                c.g[A2][0] = 0;
                addiu(c, A0, A0, slot(8));
                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8af8)?;
                lw_3(c, A0, S1, mx::RIDER);
                break 'b_118afc;
            }
            // ---------------------------------------------------------------
            // Trick-name banner slot 10 (messages 0xa) and its history.
            'b_118c30: {
                'b_118bfc: {
                    'b_118b24: {
                        c.cycles += 7;
                        c.g[A1][0] = 0xa;
                        lw_3(c, A3, S1, mx::TRICK_FLAG);
                        lw_3(c, V0, A0, rider::BOOST_LEVEL);
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
                            lw_3(c, V0, S1, mx::TRICK_COUNT);
                            if c.g[V0][0] == 0 {
                                lw_3(c, V1, S1, mx::SLOTS);
                                break 'b_118bfc;
                            } else {
                                break 'b_118b24;
                            }
                        }
                    }
                    c.cycles += 2;
                    if (c.g[A1][0] as i64) <= 0 {
                        lw_3(c, V1, S1, mx::SLOTS);
                        break 'b_118bfc;
                    } else {
                        'b_118b80: {
                            c.cycles += 10;
                            lw_3(c, S0, S1, mx::TRICK_COUNT);
                            lw_3(c, A0, S1, mx::SLOTS);
                            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                            if c.g[A3][0] != 0 {
                                c.g[S0][0] = c.g[V1][0];
                            }
                            lw_3(c, V0, A0, slot(10));
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
                                lw_3(c, A1, S1, mx::RIDER);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, table::AUDIO_CALL_A, 0x0011_8b80)?;
                                break 'b_118b80;
                            }
                        }
                        // The trick-name timer (slot 10's fraction).
                        'b_118bdc: {
                            'b_118bcc: {
                                c.cycles += 17;
                                lw_3(c, V0, S1, mx::SLOTS);
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
                        lw_3(c, A0, S1, mx::SLOTS);
                        c.g[A3][0] = c.g[S0][0];
                        c.g[A1][0] = 0xa;
                        c.g[A2][0] = 0;
                        addiu(c, A0, A0, slot(10));
                        callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8bf4)?;
                        lw_3(c, V1, S1, mx::SLOTS);
                        break 'b_118c30;
                    }
                }
                'b_118c24: {
                    c.cycles += 4;
                    lw_3(c, V0, V1, slot(10));
                    c.g[V0][0] ^= 0x34;
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = c.g[S1][0];
                        break 'b_118c24;
                    } else {
                        c.cycles += 6;
                        call_ptr(c, table::AUDIO_SYSTEM, 0x0011_8c14)?;
                        lw_3(c, A1, S1, mx::RIDER);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, table::AUDIO_CALL_B, 0x0011_8c20)?;
                        c.g[A0][0] = c.g[S1][0];
                        break 'b_118c24;
                    }
                }
                c.cycles += 3;
                c.g[A1][0] = 0xa;
                callee::REMOVE_HUD_MESSAGE_2.call(c, 0x0011_8c2c)?;
                lw_3(c, V1, S1, mx::SLOTS);
                break 'b_118c30;
            }
            // The board-wake meter: slot 9 (message 9) and its fraction.
            'b_118d54: {
                'b_118d50: {
                    'b_118c84: {
                        c.cycles += 4;
                        lw_3(c, V0, V1, slot(9));
                        c.g[V0][0] ^= 0x34;
                        let t = c.g[V0][0] != 0;
                        addiu(c, A0, V1, slot(9));
                        if t {
                            break 'b_118c84;
                        } else {
                            c.cycles += 7;
                            lw_3(c, V0, S1, mx::RIDER);
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
                                lw_3(c, A1, S1, mx::RIDER);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, table::AUDIO_CALL_C, 0x0011_8c7c)?;
                                lw_3(c, A0, S1, mx::SLOTS);
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
                        lw_3(c, V1, S1, mx::RIDER);
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
                                callee::REMOVE_HUD_MESSAGE_2.call(c, 0x0011_8cc8)?;
                                call_ptr(c, table::AUDIO_SYSTEM, 0x0011_8cd0)?;
                                lw_3(c, A1, S1, mx::RIDER);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, table::AUDIO_CALL_D, 0x0011_8cdc)?;
                                lw_3(c, A0, S1, mx::SLOTS);
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
                            lw_3(c, V0, S1, mx::SLOTS);
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
                    lw_3(c, A3, V1, rider::BOOST_LEVEL);
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
                lw_3(c, A0, S1, mx::SLOTS);
                break 'b_118d54;
            }
            // ---------------------------------------------------------------
            // The combo timer's slot 3 (message 3, its count at_3 +0x10) and
            // slot 4's fraction.
            'b_118e30: {
                c.cycles += 11;
                c.g[A1][0] = 7;
                lw_3(c, A2, S1, 0x198);
                c.g[A3][0] = 0;
                c.mtc1(0, 12);
                addiu(c, A0, A0, slot(7));
                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8d6c)?;
                lw_3(c, V1, S1, mx::SLOTS);
                lw_3(c, V0, V1, slot(3));
                c.g[V0][0] ^= 0x34;
                let t = c.g[V0][0] == 0;
                addiu(c, A0, V1, slot(3));
                if t {
                    break 'b_118e30;
                } else {
                    'b_118db4: {
                        c.cycles += 9;
                        lw_3(c, A3, S1, mx::COMBO_POINTS);
                        lwc1(c, 0, V1, slot(3) + 4);
                        lw_3(c, A1, V1, slot(3) + 0x14);
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
                        lw_3(c, A3, S1, mx::COMBO_COUNT);
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
                            lw_3(c, V1, S1, mx::SLOTS);
                            sw_2(c, V1, slot(3) + 0x10, V0);
                            break 'b_118e30;
                        }
                    }
                    c.cycles += 5;
                    c.lwc1(0, gpf(c, 0x7a44));
                    lw_3(c, V0, S1, mx::SLOTS);
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
                    lw_3(c, A1, S1, mx::SLOTS);
                    c.g[A3][0] = c.g[V0][0];
                    lw_3(c, T0, S1, mx::BONUS);
                    lw_3(c, V0, A1, slot(2));
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
                                lw_3(c, V0, A1, slot(2) + 0x14);
                                lw_3(c, V1, A1, slot(2) + 0xc);
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
                lw_3(c, A0, S1, mx::RIDER);
                c.g[S0][0] = c.g[V0][0];
                lw_3(c, V1, A0, rider::BOARD);
                addiu(c, A0, A0, rider::BOARD);
                c.g[A1][0] = c.lh(at_3(c, V1, 0x38));
                lw_3(c, V0, V1, 0x3c);
                let t = c.g[V0][0] as u32;
                addiu(c, A0, A0, c.g[A1][0] as u32);
                call_ptr(c, t, 0x0011_8f14)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, table::BE_CALL, 0x0011_8f20)?;
                lw_3(c, A0, S1, mx::SLOTS);
                c.g[A2][0] = c.g[V0][0];
                c.mtc1(0, 12);
                c.g[A1][0] = 0x19;
                addiu(c, A0, A0, slot(25));
                c.g[A3][0] = 0;
                callee::SHOW_NUMBER_DELAYED.call(c, 0x0011_8f3c)?;
                lw_3(c, A0, S1, mx::SLOTS);
                lw_3(c, V0, A0, slot(4));
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
                            lw_3(c, A2, A0, slot(4) + 0x14);
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::elf_image;
    use crate::difftest::put_vec;
    use ssx3_ee as ee;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::FAKE;
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
    use crate::difftest::fakes;
    use crate::difftest::put_words;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::SP as STACK;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::compare;
    use crate::difftest::seed_count;
    pub(crate) use crate::game::ai::rider::tests::SEEDS;
    pub(crate) use crate::game::ai::rider::tests::RIDER;
    pub(crate) use crate::game::ai::rider::tests::STATE;
    pub(crate) use crate::game::ai::rider::tests::f;
    pub(crate) use crate::game::ai::rider::tests::uf;
    pub(crate) use crate::game::ai::rider::tests::rand_vec;
    pub(crate) use crate::game::ai::rider::tests::put_state;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::ai::ai::tests::be_library;
    pub(crate) use crate::game::ai::ai::tests::virtual_setup;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4;
    pub(crate) use crate::game::ai::ai::tests::belibrary;
    pub(crate) use crate::game::lib::sif::tests::SEEDS as SEEDS_5;
    pub(crate) use crate::game::ai::ai::tests::rng as rng_2;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_6;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    pub(crate) use crate::game::ai::rider::tests::fb;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_7;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::ai::ai::tests::wf;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;


    #[test]
    fn rider_metrix_set_base_score_matches() {
        check(ssx3_ee::s00::f_00117520, rider_metrix_set_base_score, 200, &|c, seed| {
            let mut r = Rng(0x5566_7788 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA + 0x1c8, r.next_u32());
            c.bus.write32(DATA + 0x198, r.next_u32());
        });
    }


    #[test]
    fn rider_metrix_get_rounded_trick_points_matches() {
        check_lohi(ssx3_ee::s00::f_00117948, rider_metrix_get_rounded_trick_points, 300, &|c, seed| {
            let mut r = Rng(0x6677_8899 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x14, r.edge_f32());
            c.bus.write32(DATA + 0x1c4, r.edge_f32());
            c.bus.write32(GAME_GP - 0x7b08, r.edge_f32());
        });
    }


    #[test]
    fn rider_metrix_update_matches() {
        check(ee::s00::f_00117c28__generated, rider_metrix_update, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x7c28, 0);
            elf_image(c);
            let m = DATA + 0x8000;
            // The BELibrary singleton (gp-0x1f04) already exists: otherwise
            // `BELibrary_GetInstance` (reached from the trick HUD and
            // `RiderMetrix_BankCombo` → `RiderMetrix_AwardPoints`) builds it
            // and waits forever on its rewards file (`EAFile_QueueOpen` /
            // `EAFile_AllocRequest` with no request slots on a test machine).
            c.bus.write32(GAME_GP - 0x1f04, DATA + 0xa000);
            c.bus.write32(m + 0x1ac, RIDER);
            rand_vec(c, &mut r, RIDER + 0x1e0, 30.0);
            c.bus.write32(RIDER + 0x300, uf(&mut r, 0.5, 2.0));
            rand_vec(c, &mut r, RIDER + 0x1c0, 5.0);
            put_state(c, (seed % 3 == 1) as u32, if seed % 4 == 2 { 1 } else { 3 });
            for off in [0xa4, 0x30, 0x40, 0x24, 0x2c, 0x6c, 0x78] {
                let v = if r.next_u32() % 3 == 0 { r.f32(0.05) } else { r.f32(10.0) };
                c.bus.write32(m + off, v);
            }
            for off in [0x14, 0x3c, 0x1c, 0x1a8, 0x164, 0x168, 0x160, 0x15c] {
                c.bus.write32(m + off, r.f32(50.0));
            }
            c.bus.write32(m + 0x84, r.next_u32() % 1000);
            let n = [0, 1, 3, 0xffff_fffe][(seed % 4) as usize];
            c.bus.write32(m + 0x1b0, if seed % 5 == 0 { 0 } else { DATA + 0x9000 });
            c.bus.write32(m + 0x1b4, n);
            c.g[A0][0] = sx(m);
        });
    }


    #[test]
    fn rider_metrix_check_score_milestone_matches() {
        check(ee::s00::f_00119210__generated, rider_metrix_check_score_milestone, 120, &|c, seed| {
            let mut r = Rng(0x1192_10 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let this = DATA;
            let idx = DATA + 0x100;
            let table = DATA + 0x200;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(idx);
            c.g[A2][0] = sx(table);
            c.g[A3][0] = sx(2);
            let i = if seed % 3 == 0 { 0 } else { r.next_u32() % 3 };
            c.bus.write32(idx, i);
            // Seeds cover a negative threshold, an unreached step and a hit.
            let (threshold, x) = match seed % 3 {
                0 => ((-1.0f32).to_bits(), 1.0f32.to_bits()),
                1 => (5.0f32.to_bits(), 1.0f32.to_bits()),
                _ => (0.5f32.to_bits(), 1.0f32.to_bits()),
            };
            c.bus.write32(table + i * 8, threshold);
            c.bus.write32(table + i * 8 + 4, r.f32(100.0));
            c.bus.write32(this + rider::METRIC_SCALE, r.edge_f32());
            c.f[12] = x;
        });
    }


    #[test]
    fn rider_metrix_clear_peak_a4_matches() {
        check(ee::s00::f_001176f8__generated, rider_metrix_clear_peak_a4, SEEDS_2, &|c, seed| {
            let mut r = rng(0x0011_76f8, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA + 0xa4, r.next_u32());
        });
    }


    #[test]
    fn rider_metrix_update_peak_a4_matches() {
        check(ee::s00::f_00117708__generated, rider_metrix_update_peak_a4, SEEDS_2, &|c, seed| {
            let mut r = rng(0x0011_7708, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            // Half the seeds take a larger "seconds", half a smaller one.
            let (cur, new) = if seed % 2 == 0 {
                (r.f32(10.0), r.f32(100.0))
            } else {
                (r.f32(100.0), r.f32(10.0))
            };
            c.bus.write32(DATA + 0xa4, cur);
            c.f[12] = new;
        });
    }


    #[test]
    fn rider_metrix_on_race_finished_matches() {
        check(ee::s00::f_001193e0__generated, rider_metrix_on_race_finished, 200, &|c, seed| {
            machine(c, seed, 0x11_93e0, 0);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            // Both exits of `RiderMetrix_BankCombo` are early: no combo banked
            // (trick count < 2) or no points.
            if seed % 2 == 0 {
                c.bus.write32(DATA + 0x1000 + 0x9c, 0);
            } else {
                c.bus.write32(DATA + 0x1000 + 0x9c, 5);
                c.bus.write32(DATA + 0x1000 + 0xa0, 0);
            }
        });
    }


    #[test]
    fn rider_metrix_set_press_spin_matches() {
        check(s00::f_00119ad8__generated, rider_metrix_set_press_spin, 150, &|c, seed| {
            machine(c, seed, 0x119ad8, 1);
            clear(c, OBJ, 0x80);
            c.g[A0][0] = sx(OBJ);
            c.g[A1][0] = sx(seed as u32 & 1);
            // The pi constant lives at `gp-0x7a0c` in `.lit4`; both bodies load
            // it from there, so a zeroed page is fine (and deterministic).
            c.bus.write32(GAME_GP.wrapping_sub(0x7a0c), 0x4049_0fdb);
            c.bus.write32(OBJ + 0x34, seed.wrapping_mul(0x3dcc_cccd));
            c.bus.write32(OBJ + 0x14, 0);
        });
    }


    #[test]
    fn rider_metrix_load_state_matches() {
        check(ee::s00::f_001174e8__generated, rider_metrix_load_state, SEEDS_3, &|c, seed| {
            virtual_setup(c, seed, 0x1174_e8, 0x10);
        });
    }


    #[test]
    fn rider_metrix_start_board_press_matches() {
        check(
            ssx3_ee::s00::f_001199f8__generated,
            rider_metrix_start_board_press,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x06, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                // The press peak `+0x2c` can be negative or positive.
                c.bus.write32(
                    DATA + 0x2c,
                    if seed % 3 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(1.0)
                    },
                );
            },
        );
    }


    // ------------------------------------------------------------------
    // Rider metrics / BE.

    #[test]
    fn rider_metrix_on_knockdown_opponent_matches() {
        check(
            ee::s00::f_00119400__generated,
            rider_metrix_on_knockdown_opponent,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x119400, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at + 0x128, (seed % 7) as u32);
            },
        );
    }


    #[test]
    fn rider_metrix_start_handplant_matches() {
        check(
            ee::s00::f_00119bf0__generated,
            rider_metrix_start_handplant,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x119bf0, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.bus.write32(self_at + 0x14, 0x4120_0000 | (seed & 0xfff));
                c.bus.write32(GAME_GP - 0x7a04, 0x3ca3_d70a);
            },
        );
    }


    #[test]
    fn rider_metrix_clear_hud_messages_matches() {
        check(
            ee::s00::f_001173b8__generated,
            rider_metrix_clear_hud_messages,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_73b8, 0);
                let m = DATA + 0x1000;
                let entries = DATA + 0x2000;
                match seed % 4 {
                    0 => {}
                    1 => c.bus.write32(m + 0x1b0, entries),
                    2 => {
                        c.bus.write32(m + 0x1b0, entries);
                        c.bus.write32(m + 0x1b4, u32::MAX);
                    }
                    _ => {
                        c.bus.write32(m + 0x1b0, entries);
                        c.bus.write32(m + 0x1b4, 1 + r.next_u32() % 6);
                    }
                }
                c.g[A0][0] = sx(m);
            },
        );
    }


    #[test]
    fn rider_metrix_on_mission_completed_matches() {
        check(
            ssx3_ee::s00::f_0011a110__generated,
            rider_metrix_on_mission_completed,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11a110, 2);
                belibrary(c, &mut r);
                let mtx = DATA + 0x500;
                // The HUD slot array is empty: `ShowHUDNumber` returns at once.
                c.bus.write32(mtx + 0x1b0, 0);
                c.g[A0][0] = mtx as u64;
                // A zero amount makes `RiderMetrix_AwardPoints` return at once.
                c.g[A1][0] = 0;
            },
        );
    }


    #[test]
    fn rider_metrix_get_trick_points_matches() {
        leaf(ssx3_ee::s00::f_00117900__generated, rider_metrix_get_trick_points, 31);
    }


    #[test]
    fn rider_metrix_record_best_jump_matches() {
        check(ee::s00::f_00119448__generated, rider_metrix_record_best_jump, SEEDS_6, &|c, seed| {
            let mut r = machine(c, seed, 0x11_9448, 2);
            c.f[12] = r.f32(500.0);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            w(c, DATA + 0x2000 + 0x130, seed % 9);
            w(c, DATA + 0x2000 + 0x1b0, if seed % 3 == 0 { 0 } else { DATA + 0x2200 });
            w(c, DATA + 0x2000 + 0x18, if seed % 2 == 0 { fb(-100.0) } else { fb(1e6) });
            c.pc = 0x0011_9448;
        });
    }


    #[test]
    fn rider_metrix_reset_matches() {
        check(ee::s00::f_00117540__generated, rider_metrix_reset, SEEDS_7, &|c, seed| {
            let mut r = machine(c, seed, 0x117540, 6);
            be_library_2(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            w(c, obj + 0x114, 0);
            w(c, obj + 0x118, 0);
            for i in 0..16u32 {
                w(c, obj + 4 * i, r.next_u32());
            }
        });
    }


    #[test]
    fn rider_metrix_on_rider_reset_matches() {
        check(ee::s00::f_00119368__generated, rider_metrix_on_rider_reset, SEEDS_7, &|c, seed| {
            let mut r = machine(c, seed, 0x119368, 3);
            be_library_2(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 2) as u64;
            w(c, obj + 0x1ac, DATA + 0x1000);
            w(c, DATA + 0x1000 + 0x2f8, r.edge_f32());
        });
    }


    #[test]
    fn rider_metrix_find_hud_slot_matches() {
        check(ee::s00::f_00117a58__generated, rider_metrix_find_hud_slot, 200, &|c, seed| {
            let mut r = base(c, seed, 0x11_7a58);
            globals(c);
            const SLOTS: u32 = DATA + 0x1554;
            let count = 34 + (seed % 5);
            w(c, DATA + 0x1b4, count);
            w(c, DATA + 0x1b0, SLOTS - 0x1554);
            for i in 0..39u32 {
                if i < 35 {
                    w(c, SLOTS + 0x9c * i, 0);
                    continue;
                }
                // 0x34 marks a free slot; otherwise value/total drives the
                // "smallest ratio" search.
                let free = (seed + i) % 5 == 0;
                w(c, SLOTS + 0x9c * i, if free { 0x34 } else { 0 });
                c.bus.write32(SLOTS + 0x9c * i + 4, 0x3f80_0000u32 | (r.next_u32() & 0x003f_ffff));
                c.bus.write32(SLOTS + 0x9c * i + 8, 0x3f00_0000u32 | (r.next_u32() & 0x007f_ffff));
            }
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = [0u64, 17, 33, 34, 36][seed as usize % 5];
        });
    }


    // ---------------------------------------------------------- rider metrics

    #[test]
    fn rider_metrix_chain_uber_grind_matches() {
        check(ssx3_ee::s00::f_00119958__generated, rider_metrix_chain_uber_grind, 8, &|c, seed| {
            let mut r = base_2(c, seed, 0x0011_9958);
            let m = DATA + 0x100;
            clear_2(c, m, 0x200);
            w(c, m + 0x54, r.next_u32() % 8);
            w(c, m + 0x74, r.next_u32() % 8);
            w(c, m + 0x30, r.f32(4.0));
            w(c, m + 0x24, r.f32(4.0));
            w(c, m + 0x20, r.next_u32());
            w(c, m + 0xc, r.next_u32());
            c.g[A0][0] = sx(m);
            c.g[A1][0] = (seed % 500) as u64;
        });
    }


    #[test]
    fn rider_metrix_finish_board_press_matches() {
        check(ssx3_ee::s00::f_00119a38__generated, rider_metrix_finish_board_press, 8, &|c, seed| {
            let mut r = base_2(c, seed, 0x0011_9a38);
            let m = DATA + 0x100;
            clear_2(c, m, 0x200);
            w(c, m + 0x30, if seed % 2 == 0 { r.f32(1.0) } else { r.next_u32() | 0x8000_0000 });
            w(c, m + 0x24, r.f32(4.0));
            w(c, m + 0x20, r.next_u32());
            w(c, m + 0xc, r.next_u32());
            c.g[A0][0] = sx(m);
        });
    }


        #[test]
    fn rider_metrix_start_spin_trick_matches() {
        check(
            ee::s00::f_00119c98__generated,
            rider_metrix_start_spin_trick,
            8,
            &|c, seed| {
                let _ = base_3(c, seed, 0x119c98, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 6) as u64;
                wf(c, OBJ + 0x14, if seed % 3 == 0 { 0.0 } else { 1.0 });
                w(c, OBJ + 0x60, 0);
            },
        );
    }


    #[test]
    fn rider_metrix_on_split_marker_matches() {
        diff(ee::s00::f_001194c0__generated, rider_metrix_on_split_marker, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x11_94c0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 4) as u64;
            w(c, DATA + 0x1ac, DATA + 0x2000);
            w(c, DATA + 0x2000 + 0x6c0, 0);
            w(c, DATA + 0x2000 + 0x6c4, r.next_u32());
            w(c, DATA + 0x2000 + 0x6c8, r.next_u32());
            w(c, DATA + 0x2000 + 0x6cc, r.next_u32());
        });
    }


    /// The metrix, its rider and the objects the HUD callees walk through.
    pub(crate) const M: u32 = DATA + 0x4000;

    pub(crate) const RIDER_2: u32 = DATA + 0x6000;

    pub(crate) const OWNER: u32 = DATA + 0x6800;

    pub(crate) const BOARD: u32 = DATA + 0x6a00;

    pub(crate) const ANGLE_OBJ: u32 = DATA + 0x6c00;

    pub(crate) const BOARD_VT: u32 = DATA + 0x6d00;

    pub(crate) const SLOTS: u32 = DATA + 0x10000;

    pub(crate) const SLOT_COUNT: u32 = 0x2c;

    /// The BE-library singleton and the audio system the HUD calls into.
    pub(crate) const BELIB: u32 = DATA + 0x30000;

    pub(crate) const AUDIO: u32 = DATA + 0x31000;

    /// `world -> +0x84 manager -> +0xc frames`, and the audio "playing" list
    /// `manager+0x28 -> (a value < 10)` that makes every audio helper return
    /// at_3 once.
    pub(crate) const WORLD_3: u32 = DATA + 0x32000;

    pub(crate) const MANAGER: u32 = DATA + 0x32100;

    pub(crate) const FRAMES: u32 = DATA + 0x32200;

    pub(crate) const PLAYING: u32 = DATA + 0x32300;


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
                c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD_3);
                c.bus.write32(WORLD_3 + 0x84, MANAGER);
                c.bus.write32(MANAGER + 0xc, FRAMES);
                c.bus.write32(MANAGER + 0x28, PLAYING);
                c.bus.write32(PLAYING, 1);
                c.bus.write32(FRAMES + 8, r.next_u32() % 4096);

                // The metrix: a slot table (so the body runs) and the
                // trick-score inputs (`GetRoundedTrickPoints` +0x1c4 * +0x14).
                c.bus.write32(M + mx::SLOTS, SLOTS);
                c.bus.write32(M + 0x1b4, SLOT_COUNT);
                c.bus.write32(M + mx::RIDER, RIDER_2);
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
                c.bus.write32(RIDER_2 + rider::OWNER, OWNER);
                c.bus.write32(RIDER_2 + rider::BOARD, BOARD_VT);
                c.bus.write32(RIDER_2 + rider::BOOST_LEVEL, r.next_u32() % 14);
                c.bus.write32(RIDER_2 + rider::BOOST_MODE, r.next_u32() % 8);
                c.bus.write32(RIDER_2 + rider::BOOST_METER, r.f32(1.0));
                c.bus.write32(RIDER_2 + rider::BOOST_STRENGTH, r.f32(1.0));
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
