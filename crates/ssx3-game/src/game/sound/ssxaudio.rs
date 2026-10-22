//! `src/sound/ssxaudio.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::set_reg;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::events::Resume;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee as ee;
use crate::call::Unwound;
use crate::call::VSlot;
use ssx3_ee_rt::funcs::call_indirect;
pub(crate) use crate::game::lib::snd::addiu;
pub(crate) use crate::game::lib::snd::sll;
pub(crate) use crate::game::lib::snd::at;
pub(crate) use crate::game::lib::snd::fpr;
pub(crate) use crate::game::lib::snd::cvt_w;
pub(crate) use crate::game::render::lightman::gp_2 as gp;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::r;
pub(crate) use crate::game::dirtysock::tags2::gp_3 as gp_2;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::sound::bankmonitor::gp as gp_3;
pub(crate) use crate::game::sound::bankmonitor::load_gp;
pub(crate) use crate::game::sound::bankmonitor::add32;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::world::wscriptcache::DELETE_2 as DELETE;
pub(crate) use crate::game::dirtysock::tags2::p;
pub(crate) use crate::game::wscript::wscriptman::gp as gp_4;
pub(crate) use crate::game::scripter::ssxscriptengine::gp as gp_5;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::fe::feasyncfile::low_2;
pub(crate) use crate::game::fe::feasyncfile::gpa;
pub(crate) use crate::game::fe::feasyncfile::lw;
pub(crate) use crate::game::fe::fepopup::lb;
pub(crate) use crate::game::fe::festaterewards::at_4 as at_2;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::wscript::wscriptman::sw;
pub(crate) use crate::game::fe::feasyncfile::at as at_3;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw_2;
pub(crate) use crate::game::fe::feasyncfile::gp_15 as gp_6;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_4;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::lh;
pub(crate) use crate::game::render::graphicsman::RegMem;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0028_4bb0, ssx_audio_system_create, "SSXAudioSystem_Create"),
    (0x0028_4c28, ssx_audio_system_destroy, "SSXAudioSystem_Destroy"),
    (0x0028_54a8, ssx_audio_system_set_output_mode, "SSXAudioSystem_SetOutputMode"),
    (0x0028_5930, ssx_audio_system_compute_3d_pan, "SSXAudioSystem_Compute3DPan"),
    (0x0028_5be0, ssx_audio_system_set_mode, "SSXAudioSystem_SetMode"),
    (0x0028_5f48, audio_stop_all_sounds, "SSXAudioSystem_StopAllSounds"),
    (0x0028_6200, ssx_audio_system_unload_front_end_audio, "SSXAudioSystem_UnloadFrontEndAudio"),
    (0x0028_6c00, ssx_audio_system_unload_world_banks, "SSXAudioSystem_UnloadWorldBanks"),
    (0x0028_6ca8, ssx_audio_system_load_memory_bank, "SSXAudioSystem_LoadMemoryBank"),
    (0x0028_6e20, ssx_audio_system_enter_game_init, "SSXAudioSystem_EnterGameInit"),
    (0x0028_7108, ssx_audio_system_on_reset_race, "SSXAudioSystem_OnResetRace"),
    (0x0028_73d8, ssx_audio_system_level_to_volume, "SSXAudioSystem_LevelToVolume"),
    (0x0028_7410, ssx_audio_system_set_music_volume, "SSXAudioSystem_SetMusicVolume"),
    (0x0028_7488, ssx_audio_system_set_sfx_volume, "SSXAudioSystem_SetSfxVolume"),
    (0x0028_7520, audio_set_speech_volume, "SSXAudioSystem_SetSpeechVolume"),
    (0x0028_7558, ssx_audio_system_set_music_channel_enabled, "SSXAudioSystem_SetMusicChannelEnabled"),
    (0x0028_75d0, ssx_audio_system_set_sfx_sub_channels_enabled, "SSXAudioSystem_SetSfxSubChannelsEnabled"),
    (0x0028_7670, ssx_audio_system_get_music_volume_level, "SSXAudioSystem_GetMusicVolumeLevel"),
    (0x0028_76a0, ssx_audio_system_get_sfx_volume_level, "SSXAudioSystem_GetSfxVolumeLevel"),
    (0x0028_76d0, ssx_audio_system_get_speech_volume_level, "SSXAudioSystem_GetSpeechVolumeLevel"),
    (0x0028_7930, ssx_audio_system_get_channel_gain, "SSXAudioSystem_GetChannelGain"),
    (0x0028_7a10, ssx_audio_system_set_fade_gain, "SSXAudioSystem_SetFadeGain"),
    (0x0028_8370, ssx_audio_system_get_preset_gain, "SSXAudioSystem_GetPresetGain"),
    (0x0028_87a8, ssx_audio_system_set_gain_target, "SSXAudioSystem_SetGainTarget"),
    (0x0028_8ce8, ssx_audio_system_get_listener_count, "SSXAudioSystem_GetListenerCount"),
    (0x0028_9650, ssx_audio_system_crowd_state_callback, "SSXAudioSystem_CrowdStateCallback"),
    (0x0028_9830, ssx_audio_system_set_duck_gain, "SSXAudioSystem_SetDuckGain"),
    (0x0028_99e8, ssx_audio_system_is_sound_mode2, "SSXAudioSystem_IsSoundMode2"),
    (0x0028_9b18, ssx_audio_system_on_anim_sound_event, "SSXAudioSystem_OnAnimSoundEvent"),
    (0x0028_9b70, ssx_audio_system_pause_game_audio, "SSXAudioSystem_PauseGameAudio"),
    (0x0028_9d08, ssx_audio_system_option_to_output_mode, "SSXAudioSystem_OptionToOutputMode"),
    (0x0028_9d60, ssx_audio_system_output_mode_to_option, "SSXAudioSystem_OutputModeToOption"),
    (0x0028_a058, sndloadqueue_ctor, "SndLoadQueue_ctor"),
    (0x0028_a148, snd_load_queue_dtor, "SndLoadQueue_dtor"),
    (0x002a_77f8, ini_parser_ctor, "IniParser_ctor"),
    (0x002a_7ff8, ini_parser_get_string, "IniParser_GetString"),
];

/// gp-relative `.lit4` constants.
pub(crate) mod pan_k {

    /// 0.01: world units to audio units.
    pub const SCALE: i32 = -0x4658;

    /// -pi/2, +pi/2 (azimuth straight left/right).
    pub const AZ_LEFT: i32 = -0x4654;

    pub const AZ_RIGHT: i32 = -0x4650;

    /// pi (azimuth behind).
    pub const AZ_PI_ADD: i32 = -0x464c;

    pub const AZ_PI_SUB: i32 = -0x4648;

    /// 65536 / 2pi: radians to binary angle.
    pub const AZ_TO_ANGLE: i32 = -0x4644;

    /// -pi/2, +pi/2 (straight down/up).
    pub const EL_DOWN: i32 = -0x4640;

    pub const EL_UP: i32 = -0x463c;

    pub const EL_PI_ADD: i32 = -0x4638;

    pub const EL_PI_SUB: i32 = -0x4634;

    pub const EL_TO_ANGLE: i32 = -0x4630;
}

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::be::beintnetwork::callee::MEM_MAN_ALLOC;
    pub(crate) use crate::game::be::beintnetwork::callee::C_BOLTON_FX_REFRESH_EMITTERS;
    pub(crate) use crate::game::be::beintnetwork::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::be::beintnetwork::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME0;
    pub(crate) use crate::game::be::beintnetwork::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME1;
    pub(crate) use crate::game::be::beintnetwork::callee::PATH_QUERY_EVENTS;
    pub(crate) use crate::game::be::beintnetwork::callee::SSX_AUDIO_SYSTEM_GET_APP;
    pub(crate) use crate::game::be::beintnetwork::callee::SSX_AUDIO_SYSTEM_GET_VIEW_COUNT;
    pub(crate) use crate::game::be::beintnetwork::callee::INPUT_VM_INIT;
    pub(crate) use crate::game::be::beintnetwork::callee::INPUT_VM_EXECUTE;
    pub(crate) use crate::game::be::beintnetwork::callee::INPUT_MAP_EVAL_FLOAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_VISIBLE_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_FIND_ROSTER_ENTRY;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_BUDDY_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_BLOCKED_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_TEMP_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_ONLINE_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_SAME_PRODUCT_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_COUNT_MESSAGES_FROM_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_INSTANCE_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_ADD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_SKIP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_EMPTY_CROWD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_GET_BUFFER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_RIDER_SAY_20B9;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_INSTANCE_PLAY_TRICK;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_INSTANCE_PLAY_FALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_INSTANCE_STOP_ANTICIPATE;
    pub(crate) use crate::game::be::beintmission::callee::FIND_MISSION;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_RESET_STATE;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::GET_RIDER_PREVIEW;
    pub(crate) use crate::game::be::beintmission::callee::RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::be::beintmission::callee::LOBBY_MENU_REQUEST_JOIN_ROOM;
    pub(crate) use crate::game::be::beintmission::callee::POPUP_CONFIRM_CFG_RESET;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_CHECK_REWARDS_READY;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_POLL_SHAPE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::be::beintmission::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::be::beintmission::callee::RACE_PROGRESS_RESET_ALL;
    pub(crate) use crate::game::be::beintmission::callee::MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_AUX_LISTS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_DEACTIVATE_BINDING;
    pub(crate) use crate::game::be::beintmission::callee::CROWD_AUDIO_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::BANK_MONITOR_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::SND_BANK_SET_MEMORY;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_IS_GAME_MUSIC_ENABLED;
    pub(crate) use crate::game::be::beintmission::callee::MUSIC_PLAYER_STOP_SONG;
    pub(crate) use crate::game::be::beintmission::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::be::beintmission::callee::BE_NEW_PLAYER_GET_INBOX;
    pub(crate) use crate::game::be::beintmission::callee::WSCRIPT_MAN_EVENT_QUEUE_POP;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_START_PRERACE_MUSIC;
    pub(crate) use crate::game::be::beintmission::callee::SLOGIN_DELETE;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_MAN_UPDATE_MASK1;
    pub(crate) use crate::game::be::beintmission::callee::EA_MEMSET;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_ENGINE_GET_INSTANCE as SCRIPT_ENGINE_GET_INSTANCE_2;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_SOUND_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_PAUSE;
    pub(crate) use crate::game::world::wscriptcache::callee::BANK_MONITOR_DELETE_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_NEW_RACE_INT_GET_TRACK_INFO;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5;
    pub(crate) use crate::game::world::wscriptcache::callee::CROWD_AUDIO_FREE_SAMPLE_LIST;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_BASE_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_INIT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_GET_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS;
    pub(crate) use crate::game::world::wscriptcache::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::MDF_FILE_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::world::wscriptcache::callee::INPUT_PREPROC_WRITE_STRING;
    pub(crate) use crate::game::world::wscriptcache::callee::CINSTANCE_NODE_CTOR_V;
    pub(crate) use crate::game::world::wscriptcache::callee::FONT_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::SECTION_ALLOCATOR_FREE_ALL;
    pub(crate) use crate::game::world::wscriptcache::callee::C_WORLD_CACHE_STREAMER_DTOR_STUB;
    pub(crate) use crate::game::world::wscriptcache::callee::RCMP_SYSTEM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::PS2_SONY_CODEC_INTERNAL_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_GET_BITS1;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_FLUSH_BUFFER_3 as MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_SUB_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_MAIN_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::world::wscriptcache::callee::CHASE_CAMERA_ALGORITHM_CTOR_2 as CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_RESET_CHASE_CONTROLLER_SWITCHES;
    pub(crate) use crate::game::world::wscriptcache::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::world::wscriptcache::callee::DELETE;
    pub(crate) use crate::game::world::wscriptcache::callee::SND_BANK_MONITOR_MEM_SIZE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_DECODE_FRAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_FRAME_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAM_CONTROLLER_SET_ALGORITHM;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_ON_ACTIVATE_2 as FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_REFRESH_LIST;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_NOW_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_BUY_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_EQUIP_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_GET_NEXT_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_MESSENGER_SET_PLAYER_MENU_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_STATS_REFRESH_RESULTS_2 as FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_MAN2D_INIT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_PRE_RACE_MENU_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_COMM_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_DATA_GET_INFO_2 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2 as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MOVIE_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2 as REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_SPAN_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_EVENT_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_RAMP_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_3 as EA_MEM_FREE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::FLOAT_TO_UINT;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_0;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_1;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_UPDATE_BUTTONS_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_MESSAGE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_SET_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_INTERFACE_CALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBXSTRING_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_SHOW;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMORY_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_SET_OUTPUT;
    pub(crate) use crate::game::wscript::wscriptman::callee::SET_FADE_GAIN;
    pub(crate) use crate::game::wscript::wscriptman::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_POLL_FREE;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_SET_PITCH;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::wscript::wscriptman::callee::AVALANCHE_BAKE_NODE_HISTORY;
    pub(crate) use crate::game::wscript::wscriptman::callee::WSCRIPT_MAN_CALL_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_FREE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_CREATE_MESHES_2 as CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLEX_RAIL_UPDATE_SPACE_NODE;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FLAG_MANAGER_GROUP_LOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLOATING_V49;
    pub(crate) use crate::game::wscript::wscriptman::callee::MOD_OBJ_DTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::RAIL_MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::DISABLE_INTC;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::wscript::wscriptman::callee::LIGHT_MAN_EMPTY_HOOK_A;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_GET_STATE_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_CHUNK_RESOLVE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::AV_PLAYER_INIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::FOCUS_MAN_ADD_FOCUS_BOX;
    pub(crate) use crate::game::wscript::wscriptman::callee::CAM_CONTROLLER_CLEAR_ALGORITHMS;
    pub(crate) use crate::game::wscript::wscriptman::callee::ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::wscript::wscriptman::callee::APP_MODULE_UPDATE_FADE;
    pub(crate) use crate::game::wscript::wscriptman::callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SET_USER_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_FILE_MOUNT_BIG;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_CTOR as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_MEM_ALLOC_NAMED as EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUILD_BUDDY_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SHOW_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GOTO_NEXT_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::HANDLE_NET_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ONLINE_CHARACTER_SELECT_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHANNEL_GET_TRACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LIST_GET_ENTRY_TYPE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FIND_FREE_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LOAD_BANK_ASYNC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REQUEST_IMAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SEND_PACKET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_SAY_20B9;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::START_RIDER_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_RACE_MAN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LEVEL_TO_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AUDIO_SYSTEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_END_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SET_CONTROLLER_INDEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD as SND_BANK_UNLOAD_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_BASE_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_HAS_RENDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_HIDE_ROOM_TABS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2 as MC_MANAGER_GET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_ABORT_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_EMAIL_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_LOGIN_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_12 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHALLENGE_POPUP_SET_SETTINGS_LOCKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_11 as NET_FE_STATE_CTOR_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_DEACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_RESET_RACE_RESULT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_IS_TRACK_GROUP0_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STORE_RACE_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_SKIP_WHITESPACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_CHARACTER_2 as BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COLLECTED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COMPLETED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_GET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_4 as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_3 as NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID_2 as POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_CARD_STATE_BASE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEAD_NODE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_GROUP_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_TREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_UPDATE_BOARD_LEAN;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BUILD_WORLD_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_LOAD_STATE_2 as GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_ON_CARD_SCANNED;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_LOAD_UPDATE_PLAYER_RANKING;
    pub(crate) use crate::game::fe::feasyncfile::callee::LIGHT_MAN_EMPTY_HOOK_D;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_SLOT_3 as BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS_2 as MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT_2 as MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_FLUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH_5 as STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_START_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_DISPLAY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_GET_ACTIVE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_NET_CONFIG_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_DRAW_ROW_2 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_POP_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_START_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_PUSH_FREE_NODE_2 as SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ADD_SCRIPT_OBJ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ADD_CAM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_SET_TRANSITION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_INSTANCE_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_FIRE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_LEN_2 as USTR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_CPY_2 as USTR_CPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_ALLOC_NAMED_5 as EA_MEM_ALLOC_NAMED_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_GET_PATH_SIZE_CHECKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_LOAD_FILE_INTO;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_ACQUIRE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARDS_FILE_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_BACKUP_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_ON_RIDER_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_ADD_WIPEOUT_RECOVERY_BOOST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_CHECK_RESET_PATH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_FRAME_DUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_DECODE_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_AUDIO_DONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_3 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_4 as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_3 as UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_FIND_STATE_2 as UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_ALL_TIMERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_STOP_SONG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_FADE_OUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_PLAY_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_TRY_EXIT_2 as APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CLEAR_SKIP_DELAY_ON_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GRAPHICS_MAN_FREE_ALL_TEXTURES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_4 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_GET_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_ADVERT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_POPUP_CONFIRM_EMPTY_INIT_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_LIST_GET_AT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATOI;
    pub(crate) use crate::game::fe::feasyncfile::callee::TAG_FIELD_GET_BINARY;
}

/// The audio app / view objects: `world + 0x84` is the audio system, its
/// `+0x84` the app, and the app's `+0x10` the view count.
pub(crate) mod audio {
    pub(crate) use crate::game::sound::asyncsys::audio::APP;
    pub(crate) use crate::game::sound::asyncsys::audio::LISTENER_COUNT;
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_UpdateFade (0x287c48).

/// The fade state of an `sSaMix7C48` (offsets from the object).
pub(crate) mod fade {

    /// Non-zero while a fade is running.
    pub const FADING: u32 = 0x63c0;

    /// Bits: which channels the fade drives (bit 2 = `chan[4]`/the mix).
    pub const MASK: u32 = 0x63c4;

    /// Per-frame step applied to the current value.
    pub const RATE: u32 = 0x63c8;

    /// Where the ramp stops.
    pub const TARGET: u32 = 0x63cc;

    /// The current fade value.
    pub const CUR: u32 = 0x63d0;

    /// `chan[4].out`, scaled by the fade for the mix levels.
    pub const CHAN4: u32 = 0x6300;

    /// Level written with the fade value before the mix levels.
    pub const MIX_LEVEL: u32 = 0x63f4;

    /// The ten output mix levels (`mix[0..10]`).
    pub const MIX: u32 = 0x636c;
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

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::festaterewards::table_callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRLEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRCPY;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OBJECT_PUSH;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_ADD_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_GET_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_GET_STRING4;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUDDY_PRODUCT_COMPARE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SSX_AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_REMOVE_SCRIPT_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SET_SCRIPT_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SCRIPT_CLEANUP;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_RICH_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::CODEC_DECODE_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_INTERFACE_FV;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_GET_BASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUY_POPUP_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_POSTERS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TRADING_CARDS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CONCEPT_ARTS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_VIDEOS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TOYS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CHEAT_CHARS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OPEN_FINISH_SCREEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::AI_SET_AI_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_TEXT_FROM_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BENEW_PLAYER_INT_GET_CHAR_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_SMALL_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_BIG_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::APP_MAN_SET_NEXT_MODULE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UPLOAD_TEXTURE_PIXELS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::MUSIC_SLOT_RELEASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_STATE_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SECTION_TABLE_CTOR;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::fe::feasyncfile::table::STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_MENU_RESET_SCROLL;
    pub(crate) use crate::game::fe::feasyncfile::table::PS2_MEM_CARD_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::table::PATH_STOP_TRACKS;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::table::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_START;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::table::AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::feasyncfile::table::ASYNC_FILE_GET_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::table::SCRIPT_ENGINE_GET_RESULT_VTABLE;
    pub(crate) use crate::game::fe::feasyncfile::table::RIDER_MODEL_FIND;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_GAME_MODE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_EVENT_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_END_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_BEGIN_PLAYBACK;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::table::OPERATOR_NEW_TAG;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::feasyncfile::table::SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_GET_UNPACKED_SIZE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_UNPACK;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_TREE_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE2;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_CALC;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_CASECMP;
}


// ---------------------------------------------------------------------------
// SSX audio system.

/// `float SSXAudioSystem_LevelToVolume(SSXAudioSystem *self, int level)`
/// (0x2873d8): scales the integer level by the constant at `gp-0x462c`, then
/// clamps the result to `[0, 1]`.
pub fn ssx_audio_system_level_to_volume(c: &mut Cpu) {
    c.cycles += 11;
    let level = FpuF32::from_i32(c.g[A1][0] as u32 as i32);
    let scale = Addr((c.g[GP][0] as u32).wrapping_sub(0x462c)).read_fpu(c);
    let scaled = level * scale;
    c.f[2] = scaled.0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[3] = FpuF32::ONE.0;
    let zero = FpuF32::ZERO;
    c.f[1] = zero.0;
    abi::set_fcc(c, zero.le(scaled));
    if zero.le(scaled) {
        c.cycles += 1;
        c.f[1] = scaled.min(FpuF32::ONE).0;
    }
    c.cycles += 2;
    c.f[0] = c.f[1];
    abi::ret(c);
}


/// `float SSXAudioSystem_GetChannelGain(SSXAudioSystem *self, int channel,
/// int sub)` (0x287930): channel 4 returns a per-sub gain from +0x636c
/// (`sub*4`), everything else a per-channel gain from +0x62c8 (`channel*16`).
pub fn ssx_audio_system_get_channel_gain(c: &mut Cpu) {
    const GAIN: u32 = 0x62c8;
    const SUB_GAIN: u32 = 0x636c;
    c.cycles += 3;
    let base = abi::arg(c, 0);
    if c.g[A1][0] != 4 {
        c.g[V0][0] = sx(abi::arg(c, 1) << 4);
        c.cycles += 3;
        let addr = base.wrapping_add(c.g[V0][0] as u32);
        c.g[V0][0] = sx(addr);
        c.f[0] = Addr(addr).off(GAIN).read_fpu(c).0;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    if (c.g[A2][0] as i64) < 10 {
        c.cycles += 3;
        let off = sx(abi::arg(c, 2) << 2);
        let addr = base.wrapping_add(off as u32);
        c.g[V0][0] = sx(addr);
        c.f[0] = Addr(addr).off(SUB_GAIN).read_fpu(c).0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = sx(abi::arg(c, 1) << 4);
    c.cycles += 3;
    let addr = base.wrapping_add(c.g[V0][0] as u32);
    c.g[V0][0] = sx(addr);
    c.f[0] = Addr(addr).off(GAIN).read_fpu(c).0;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// SSXAudioSystem_Compute3DPan.

pub(crate) const GET_APP: Callee = Callee { addr: 0x0028_b1c0, f: ee::s07::SSXAudioSystem_GetApp };

pub(crate) const GET_GAME_MODULE: Callee = Callee { addr: 0x0028_b1c8, f: ee::s07::SSXAudioSystem_GetGameModule };

/// `float Math_Atan(float)` (function table, cross-shard).
pub(crate) const MATH_ATAN: u32 = 0x0031_c228;


/// The listener offset: +25 up, -100 along z.
pub(crate) const PAN_UP: FpuF32 = FpuF32(0x41c8_0000);

pub(crate) const PAN_BACK: FpuF32 = FpuF32(0xc2c8_0000);


/// A gp-relative float.
pub(crate) fn gp_const(c: &mut Cpu, off: i32) -> FpuF32 {
    let gp = reg(c, GP);
    at(gp, off as u32).read_fpu(c)
}


/// `atan(y / x)` folded into a full angle the way the game does it: if
/// `x < 0`, add pi when `y > 0`, else subtract it; `x == 0` gives 0 (y ==
/// 0), `+pi/2` (y >= 0) or `-pi/2` (constants `zero_pos`/`zero_neg` and
/// `pi_add`/`pi_sub`). The FPU registers and FCC end as the original's
/// (`dst` is the register the angle is built in: f3 or f12).
#[allow(clippy::too_many_arguments)]
pub(crate) fn pan_angle(
    c: &mut Cpu,
    y: FpuF32,
    x: FpuF32,
    dst: usize,
    k: [i32; 4],
    ret: u32,
) -> Result<FpuF32, crate::call::Unwound> {
    let [zero_neg, zero_pos, pi_add, pi_sub] = k;
    let zero = FpuF32::ZERO;
    let x_zero = x.eq_s(zero);
    abi::set_fcc(c, x_zero);
    let angle = if x_zero {
        c.cycles += 4;
        let y_zero = y.eq_s(zero);
        abi::set_fcc(c, y_zero);
        if y_zero {
            y
        } else {
            c.cycles += 4;
            let up = zero.le(y);
            abi::set_fcc(c, up);
            let down = gp_const(c, zero_neg);
            if up {
                c.cycles += 3;
                gp_const(c, zero_pos)
            } else {
                down
            }
        }
    } else {
        c.cycles += 7;
        abi::set_fpr(c, 12, y.div(x));
        call_ptr(c, MATH_ATAN, ret)?;
        let behind = x.lt(zero);
        abi::set_fcc(c, behind);
        let mut a = fpr(c, 0);
        if behind {
            c.cycles += 3;
            let positive = zero.lt(y);
            abi::set_fcc(c, positive);
            if positive {
                c.cycles += 3;
                let pi = gp_const(c, pi_add);
                abi::set_fpr(c, 0, pi);
                a = a + pi;
            } else {
                c.cycles += 2;
                let pi = gp_const(c, pi_sub);
                abi::set_fpr(c, 0, pi);
                a = a - pi;
            }
        }
        a
    };
    abi::set_fpr(c, dst, angle);
    Ok(angle)
}


/// `angle * to_binary` as a 16-bit binary angle (`cvt.w.s`; f0 the float,
/// f1 its bits, v0 the value).
pub(crate) fn binary_angle(c: &mut Cpu, angle: FpuF32, k: i32) -> u64 {
    let scale = gp_const(c, k);
    let scaled = angle * scale;
    abi::set_fpr(c, 0, scaled);
    let bits = cvt_w(scaled);
    c.f[1] = bits;
    set_reg(c, V0, sx(bits));
    sx(bits)
}


/// `float SSXAudioSystem_Compute3DPan(SSXAudioSystem* this, Emitter* e,
/// Pan3D* out)` (0x285930): the emitter's position relative to its
/// player's listener and the squared distance.
///
/// ```text
/// if (!app->gameModule) return 1.0
/// p   = listener.matrix (+0x40) * (e->pos, 1)        (VU0 MULA/MADDA/MADD)
/// r   = ((p.x, p.y + 25, p.z - 100) * 0.01)          (stored at sp+0..8)
/// if this->panEnabled (+0x62b0):
///     out->azimuth (+8)    = e->fixedSide (+0xa0) ? (e->right (+0xa4) ? 0x4000 : 0xc000)
///                                                 : angle(r.x, r.z) * 65536/2pi
///     out->elevation (+0xa) = angle(r.y, sqrt(r.x² + r.z²)) * 65536/2pi
/// else out->azimuth = out->elevation = 0
/// return r.x² + r.y² + r.z²
/// ```
///
/// Frame 0xa0 (s0 +0x70, s1 +0x60, s2 +0x50, ra +0x40, f22 +0x90, f21
/// +0x88, f20 +0x80); the transformed point also passes through v0 (all
/// 128 bits, `lq`/`sq` at sp+0x30 / sp+0x10).
pub fn ssx_audio_system_compute_3d_pan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        let emitter = reg(c, A1);
        set_reg(c, S0, emitter);
        frame.save(c, S2, 0x50);
        let out = reg(c, A2);
        set_reg(c, S1, out);
        frame.save_ra(c, 0x40);
        let this = reg(c, A0);
        set_reg(c, S2, this);
        frame.save_fpr(c, 22, 0x90);
        frame.save_fpr(c, 21, 0x88);
        frame.save_fpr(c, 20, 0x80);
        GET_APP.call(c, 0x0028_5960)?;
        let app = reg(c, V0);
        let module = at(app, 0x84).lw(c);
        set_reg(c, V1, module);
        if module == 0 {
            c.cycles += 4;
            set_reg(c, AT, FpuF32::ONE.0 as u64);
            abi::set_fret(c, FpuF32::ONE);
            frame.restore(c, S0, 0x70);
            return pan_exit(c, frame);
        }
        c.cycles += 48;
        GET_GAME_MODULE.call(c, 0x0028_5984)?;
        let module = reg(c, V0);
        let player = at(emitter, 0x84).lw(c);
        let players = at(module, 0x84).lw(c);
        let pos_at = at(emitter, 0x68).lw(c);
        let slot = addu(players, sll(player, 2));
        set_reg(c, A0, slot);
        set_reg(c, AT, FpuF32::ONE.0 as u64);
        let z = at(pos_at, 8).read_fpu(c);
        let x = at(pos_at, 0).read_fpu(c);
        let y = at(pos_at, 4).read_fpu(c);
        let listener = at(slot, 4).lw(c);
        let sp0 = Addr(sp(c));
        sp0.off(0x20).write_fpu(c, x);
        sp0.off(0x24).write_fpu(c, y);
        sp0.off(0x28).write_fpu(c, z);
        sp0.off(0x2c).write_fpu(c, FpuF32::ONE);
        let matrix = addiu(listener, 0x40);
        set_reg(c, V1, matrix);
        // p = M * (x, y, z, 1) on VU0, as the original issues it (its
        // registers and flags are visible to later code).
        c.lqc2(8, sp0.off(0x20).0);
        for i in 0..4u32 {
            c.lqc2(4 + i as usize, (matrix as u32).wrapping_add(0x10 * i));
        }
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, sp0.off(0x30).0);
        c.lq(V0, sp0.off(0x30).0);
        set_reg(c, AT, PAN_UP.0 as u64);
        c.sq(V0, sp0.off(0x10).0);
        // The listener-relative position in audio units.
        let scale = gp_const(c, pan_k::SCALE);
        let ly = sp0.off(0x14).read_fpu(c);
        let lz = sp0.off(0x18).read_fpu(c);
        let lx = sp0.off(0x10).read_fpu(c);
        let raised = ly + PAN_UP;
        let pulled = lz + PAN_BACK;
        let enabled = at(this, 0x62b0).lw(c);
        set_reg(c, V0, enabled);
        let rx = lx * scale;
        let ry = raised * scale;
        let rz = pulled * scale;
        abi::set_fpr(c, 0, ry);
        abi::set_fpr(c, 1, pulled);
        abi::set_fpr(c, 2, scale);
        abi::set_fpr(c, 3, lx);
        abi::set_fpr(c, 4, PAN_UP);
        abi::set_fpr(c, 5, PAN_BACK);
        abi::set_fpr(c, 20, rz);
        abi::set_fpr(c, 21, rx);
        sp0.write_fpu(c, rx);
        sp0.off(4).write_fpu(c, ry);
        sp0.off(8).write_fpu(c, rz);
        if enabled == 0 {
            c.cycles += 3;
            at(out, 0xa).sh(c, 0);
            at(out, 8).sh(c, 0);
        } else {
            // Azimuth.
            c.cycles += 3;
            let fixed = at(emitter, 0xa0).lw(c);
            set_reg(c, V0, fixed);
            let azimuth = if fixed != 0 {
                c.cycles += 3;
                let right = at(emitter, 0xa4).lw(c);
                if right != 0 {
                    0x4000
                } else {
                    c.cycles += 2;
                    0xc000
                }
            } else {
                c.cycles += 4;
                abi::set_fpr(c, 22, FpuF32::ZERO);
                let k = [pan_k::AZ_LEFT, pan_k::AZ_RIGHT, pan_k::AZ_PI_ADD, pan_k::AZ_PI_SUB];
                let angle = pan_angle(c, rx, rz, 3, k, 0x0028_5ab0)?;
                c.cycles += 4;
                binary_angle(c, angle, pan_k::AZ_TO_ANGLE)
            };
            set_reg(c, V0, azimuth);
            // Elevation.
            c.cycles += 12;
            at(out, 8).sh(c, azimuth);
            let dx = sp0.read_fpu(c);
            let dz = sp0.off(8).read_fpu(c);
            let xx = dx * dx;
            let dy = sp0.off(4).read_fpu(c);
            abi::set_fpr(c, 21, dy);
            let zz = dz * dz;
            abi::set_fpr(c, 1, zz);
            abi::set_fpr(c, 22, FpuF32::ZERO);
            let horiz2 = xx + zz;
            abi::set_fpr(c, 0, horiz2);
            let horiz = horiz2.sqrt();
            abi::set_fpr(c, 20, horiz);
            let k = [pan_k::EL_DOWN, pan_k::EL_UP, pan_k::EL_PI_ADD, pan_k::EL_PI_SUB];
            let angle = pan_angle(c, dy, horiz, 12, k, 0x0028_5b58)?;
            c.cycles += 5;
            let elevation = binary_angle(c, angle, pan_k::EL_TO_ANGLE);
            at(out, 0xa).sh(c, elevation);
        }
        // Squared distance.
        c.cycles += 9;
        let dx = sp0.read_fpu(c);
        let dy = sp0.off(4).read_fpu(c);
        let xx = dx * dx;
        let dz = sp0.off(8).read_fpu(c);
        let yy = dy * dy;
        let zz = dz * dz;
        abi::set_fpr(c, 2, yy);
        abi::set_fpr(c, 1, zz);
        abi::set_fret(c, (xx + yy) + zz);
        frame.restore(c, S0, 0x70);
        pan_exit(c, frame)
    })
}


/// The epilogue (s0 already restored).
pub(crate) fn pan_exit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 8;
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore_ra(c, 0x40);
    frame.restore_fpr(c, 22, 0x90);
    frame.restore_fpr(c, 21, 0x88);
    frame.restore_fpr(c, 20, 0x80);
    frame.close(c);
    Ok(())
}


/// `int SSXAudioSystem_GetListenerCount()` (0x288ce8): the app's view count,
/// or 1 when the world has no audio app.
pub fn ssx_audio_system_get_listener_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SSX_AUDIO_SYSTEM_GET_APP.call(c, 0x0028_8cf8)?;
        let world = c.g[V0][0] as u32;
        let system = Addr(world).off(audio::APP).lw(c);
        c.g[V1][0] = system;
        c.g[V0][0] = 1;
        if system != 0 {
            c.cycles += 2;
            callee::SSX_AUDIO_SYSTEM_GET_VIEW_COUNT.call(c, 0x0028_8d0c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// One faded channel pair: `fadeCur` is stored at `out`, `gain * fadeCur` at
/// `src + 4` and `gain * fadeCur * tail` at `src + 8`; the mask is left in v0
/// for the next test (as the original does, block by block).
///
/// `out` is the level the channel's fade value is written to; `src` the pair
/// of gains (also where the two scaled values go); `tail` the second factor.
pub(crate) fn scale_channel(c: &mut Cpu, base: u32, out: u32, src: u32, tail: u32) {
    c.cycles += 9;
    c.lwc1(0, base.wrapping_add(fade::CUR));
    c.lwc1(1, base.wrapping_add(src));
    c.lwc1(2, base.wrapping_add(tail));
    c.mul_s(1, 1, 0);
    c.swc1(0, base.wrapping_add(out));
    c.mul_s(2, 1, 2);
    c.swc1(1, base.wrapping_add(src).wrapping_add(4));
    c.swc1(2, base.wrapping_add(src).wrapping_add(8));
    c.g[V0][0] = c.lw(base.wrapping_add(fade::MASK));
}


/// Test mask bit `bit` (+`cost` cycles) and, when set, scale that channel.
/// Every path leaves the mask in v0 (the original reloads it either way).
pub(crate) fn mask_bit(c: &mut Cpu, base: u32, bit: u64, cost: u32, out: u32, src: u32, tail: u32) {
    c.cycles += cost as u64;
    let m = c.lw(base.wrapping_add(fade::MASK));
    if m & bit != 0 {
        scale_channel(c, base, out, src, tail);
    } else {
        c.g[V0][0] = m;
    }
}


/// `void func_00287C48(sSaMix *s)` (0x287c48): advance the audio fade ramp.
///
/// With `fading` set, `cur += rate` is stored and, once the ramp passes
/// `target` (below it for a negative rate, above it for a positive one), the
/// value is clamped to the target and `fading` cleared. Every channel bit of
/// the mask then scales the current value into its gain pair, and bit 2 also
/// rebuilds the ten output mix levels from `chan[2].out` (+0x6300) times the
/// constants at `gp+0x43c..0x460` (`D_004A352C..0x3550`).
pub fn ssx_audio_system_update_fade(c: &mut Cpu) {
    c.cycles += 3;
    let base = abi::arg(c, 0);
    let fading = c.lw(base.wrapping_add(fade::FADING));
    c.g[V0][0] = fading;
    if fading != 0 {
        // `cur += rate`
        c.cycles += 8;
        c.lwc1(0, base.wrapping_add(fade::CUR));
        c.lwc1(1, base.wrapping_add(fade::RATE));
        c.mtc1(0, 2); // f2 = 0.0
        c.add_s(0, 0, 1);
        c.c_lt_s(1, 2); // rate < 0
        let falling = c.fcc();
        c.swc1(0, base.wrapping_add(fade::CUR));
        if falling {
            // a falling ramp stops when cur < target
            c.cycles += 4;
            c.lwc1(1, base.wrapping_add(fade::TARGET));
            c.c_lt_s(0, 1);
        } else {
            // a rising ramp stops when target < cur
            c.cycles += 3;
            c.lwc1(1, base.wrapping_add(fade::TARGET));
            c.c_lt_s(1, 0);
        }
        c.cycles += 2;
        if c.fcc() {
            c.cycles += 3;
            c.swc1(1, base.wrapping_add(fade::CUR));
            c.sw(base.wrapping_add(fade::FADING), 0);
        }
        // Channels 0 and 1 only test their bit (their `set` was inlined away).
        c.cycles += 3;
        c.g[V0][0] = c.lw(base.wrapping_add(fade::MASK));
        c.cycles += 3;
        c.g[V0][0] = c.lw(base.wrapping_add(fade::MASK));

        // Channel 4: the ten mix levels.
        c.cycles += 3;
        c.g[V0][0] &= 4;
        c.lwc1(1, gp(c, 0x43c));
        if c.g[V0][0] != 0 {
            c.cycles += 37;
            c.lwc1(12, base.wrapping_add(fade::CUR));
            c.lwc1(11, base.wrapping_add(fade::CHAN4));
            c.lwc1(0, base.wrapping_add(fade::MIX_LEVEL));
            c.mul_s(11, 11, 12);
            c.lwc1(2, gp(c, 0x440));
            c.lwc1(4, gp(c, 0x444));
            c.mul_s(0, 11, 0);
            c.lwc1(5, gp(c, 0x448));
            c.lwc1(7, gp(c, 0x44c));
            c.lwc1(9, gp(c, 0x450));
            c.lwc1(10, gp(c, 0x454));
            c.mul_s(1, 0, 1);
            c.lwc1(8, gp(c, 0x458));
            c.mul_s(2, 0, 2);
            c.lwc1(6, gp(c, 0x45c));
            c.mul_s(4, 0, 4);
            c.lwc1(3, gp(c, 0x460));
            c.mul_s(5, 0, 5);
            c.mul_s(7, 0, 7);
            c.swc1(1, base.wrapping_add(fade::MIX));
            c.mul_s(3, 0, 3);
            c.swc1(2, base.wrapping_add(fade::MIX).wrapping_add(4));
            c.mul_s(9, 0, 9);
            c.swc1(4, base.wrapping_add(fade::MIX).wrapping_add(8));
            c.mul_s(10, 0, 10);
            c.swc1(5, base.wrapping_add(fade::MIX).wrapping_add(0xc));
            c.mul_s(8, 0, 8);
            c.swc1(7, base.wrapping_add(fade::MIX).wrapping_add(0x10));
            c.mul_s(6, 0, 6);
            c.swc1(9, base.wrapping_add(fade::MIX).wrapping_add(0x14));
            c.swc1(10, base.wrapping_add(fade::MIX).wrapping_add(0x18));
            c.swc1(8, base.wrapping_add(fade::MIX).wrapping_add(0x1c));
            c.swc1(6, base.wrapping_add(fade::MIX).wrapping_add(0x20));
            c.swc1(3, base.wrapping_add(fade::MIX).wrapping_add(0x24));
            c.swc1(12, base.wrapping_add(0x63a4));
            c.swc1(11, base.wrapping_add(fade::CHAN4).wrapping_add(4));
            c.swc1(0, base.wrapping_add(fade::CHAN4).wrapping_add(8));
        }

        // Channel 2 (bit 8, one cycle more: it reloads the mask first).
        mask_bit(c, base, 8, 4, 0x6398, 0x62d0, 0x63e8);
        // Channel 3, 6, 7, 8, 9, 10.
        mask_bit(c, base, 0x80, 3, 0x63a8, 0x6310, 0x63f8);
        mask_bit(c, base, 0x20, 3, 0x63ac, 0x6320, 0x63fc);
        mask_bit(c, base, 0x40, 3, 0x63b0, 0x6330, 0x6400);
        mask_bit(c, base, 0x10, 3, 0x63b4, 0x6340, 0x6404);
        mask_bit(c, base, 0x100, 3, 0x63b8, 0x6350, 0x6408);

        // Channel 11 (bit 0x200): no mask reload, one cycle less.
        c.cycles += 3;
        c.g[V0][0] &= 0x200;
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            c.lwc1(0, base.wrapping_add(fade::CUR));
            c.lwc1(1, base.wrapping_add(0x6360));
            c.lwc1(2, base.wrapping_add(0x640c));
            c.mul_s(1, 1, 0);
            c.swc1(0, base.wrapping_add(0x63bc));
            c.mul_s(2, 1, 2);
            c.swc1(1, base.wrapping_add(0x6364));
            c.swc1(2, base.wrapping_add(0x6368));
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `bool SSXAudioSystem_IsSoundMode2(SSXAudioSystem* self)` (0x2899e8): true
/// when the sound-mode word at `+0x608c` is 2.
pub fn ssx_audio_system_is_sound_mode2(c: &mut Cpu) {
    c.cycles += 4;
    let this = abi::arg(c, 0);
    c.g[V0][0] = c.lw(this.wrapping_add(0x608c));
    c.g[V0][0] = c.g[V0][0] ^ 0x2;
    c.g[V0][0] = (c.g[V0][0] < 1u64) as u64;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Audio system / music player / bank accessors.

/// `void SSXAudioSystem_SetMode(SSXAudioSystem* a, int mode)` (0x285be0): the
/// system mode at +0x6474, stored only when it changed.
pub fn ssx_audio_system_set_mode(c: &mut Cpu) {
    c.cycles += 3;
    let selfp = abi::arg(c, 0);
    let mode = c.g[A1][0];
    c.g[V0][0] = c.lw(selfp.wrapping_add(0x6474));
    if c.g[V0][0] != mode {
        c.sw(selfp.wrapping_add(0x6474), mode);
    }
    c.cycles += 2;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Audio.

/// The shared body of the three volume-level getters: read the channel's
/// volume and scale the 0..1 value to the 0..11 menu level.
pub(crate) fn volume_level(c: &mut Cpu, channel: u32, ret: u32) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A1][0] = channel as u64;
    callee::SSX_AUDIO_GET_CHANNEL_VOLUME.call(c, ret)?;
    // f1 = 11.0, then the usual `(int)(volume * 11.0f)`.
    c.g[AT][0] = 0x4130_0000; // 11.0
    c.mtc1(AT, 1);
    frame.restore_ra(c, 0);
    let level = FpuF32(c.f[0]) * FpuF32(c.f[1]);
    c.f[0] = level.0;
    c.f[1] = level.to_i32() as u32;
    c.mfc1(V0, 1);
    frame.close(c);
    Ok(())
}


/// `int SSXAudioSystem_GetMusicVolumeLevel(SSXAudioSystem *self)` (0x287670).
pub fn ssx_audio_system_get_music_volume_level(c: &mut Cpu) {
    guest(c, |c| volume_level(c, 1, 0x0028_7680))
}


/// `int SSXAudioSystem_GetSfxVolumeLevel(SSXAudioSystem *self)` (0x2876a0).
pub fn ssx_audio_system_get_sfx_volume_level(c: &mut Cpu) {
    guest(c, |c| volume_level(c, 5, 0x0028_76b0))
}


/// `int SSXAudioSystem_GetSpeechVolumeLevel(SSXAudioSystem *self)` (0x2876d0).
pub fn ssx_audio_system_get_speech_volume_level(c: &mut Cpu) {
    guest(c, |c| volume_level(c, 4, 0x0028_76e0))
}


/// `void SSXAudioSystem_CrowdStateCallback(SSXAudioSystem *self, int *state)`
/// (0x289650): when the incoming state is 3 (`idle`) reset the crowd handler.
pub fn ssx_audio_system_crowd_state_callback(c: &mut Cpu) {
    /// `gp + 0x410`: the crowd audio singleton.
    const CROWD: u32 = 0x410;

    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = 3;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        let idle = c.g[V0][0] == c.g[V1][0];
        frame.restore_ra(c, 0);
        if idle {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(CROWD)).lw(c);
            callee::SSX_AUDIO_EMPTY_CROWD_HANDLER.call(c, 0x0028_9674)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Audio system.

/// `void SSXAudioSystem_StopAllSounds(SSXAudioSystem* self)` (0x285f48):
/// stop the crowd and bank-monitor sources, then the sound-system sources.
pub fn audio_stop_all_sounds(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CROWD_AUDIO_STOP_ALL.call(c, 0x0028_5f5c)?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x118)).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
        callee::BANK_MONITOR_STOP_ALL.call(c, 0x0028_5f6c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void SSXAudioSystem_SetSpeechVolume(SSXAudioSystem* self, int level)`
/// (0x287520): convert the level to a volume and apply it to speech
/// channel 4.
pub fn audio_set_speech_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::AUDIO_LEVEL_TO_VOLUME.call(c, 0x0028_7534)?;
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(12, 0);
        c.g[A1][0] = 4;
        callee::AUDIO_SET_CHANNEL_VOLUME.call(c, 0x0028_7544)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// Sound.

/// `float SSXAudioSystem_GetPresetGain(SSXAudioSystem* self, int i, int pct)`
/// (0x288370): a non-negative percentage is scaled by 0.01, otherwise the
/// preset's stored gain (custom table at +0x6440 when +0x6410 is set, the
/// default table at +0x63E4 otherwise).
pub fn ssx_audio_system_get_preset_gain(c: &mut Cpu) {
    c.cycles += 2;
    let this = r(c, A0);
    let index = c.g[A1][0];
    if (c.g[A2][0] as i64) < 0 {
        let custom = c.lw_r_2(V0, this.wrapping_add(0x6410));
        c.cycles += 2;
        c.g[V0][0] = sx((index as u32) << 2);
        let slot = sx(this.wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = slot;
        if custom != 0 {
            c.cycles += 3;
            c.ld_f(0, (slot as u32).wrapping_add(0x6440));
        } else {
            c.cycles += 3;
            c.ld_f(0, (slot as u32).wrapping_add(0x63e4));
        }
    } else {
        c.cycles += 5;
        c.mtc1(A2, 1);
        c.cvt_s_w(1, 1);
        c.ld_f(0, gp_2(c, -0x4618));
        c.mul_s(0, 1, 0);
    }
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Audio.

/// `void SSXAudioSystem_Destroy()` (0x284c28): tell the audio interface
/// object (gp_3+0x410) to shut down (virtual slot 1, argument 3), then clear
/// the singleton.
pub fn ssx_audio_system_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        load_gp(c, V0, 0x410);
        let empty = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !empty {
            c.cycles += 8;
            c.g[V1][0] = Addr(r32(c, V0)).off(0x118).lw(c);
            c.g[A1][0] = 3;
            c.g[A2][0] = Addr(r32(c, V1)).lw(c);
            c.g[V0][0] = Addr(r32(c, A2)).off(0x1d4).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(8).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
            let func = r32(c, V1);
            c.g[RA][0] = sx(0x0028_4c58);
            c.g[A0][0] = add32(c.g[A2][0], c.g[A0][0]);
            c.pc = func;
            call_indirect(c, func);
            if c.pc != 0x0028_4c58 {
                return Err(crate::call::Unwound);
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.sw(gp_3(c, 0x410), 0);
        leave(c, 0x10);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// SSX audio system: pause latches.

/// `void SSXAudioSystem_PauseGameAudio(SSXAudioSystem* s)` (0x289b70): once
/// per session (`s+0x5fb4`), pause the bank monitor and the music stream.
pub fn ssx_audio_system_pause_game_audio(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x5fb4));
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            callee::SSX_AUDIO_SYSTEM_PAUSE.call(c, 0x0028_9b94)?;
            let stream = r32(c, S0).wrapping_add(0x118);
            c.g[A0][0] = sx(stream);
            call_ptr(c, 0x002b_3a70, 0x0028_9b9c)?;
            c.g[V0][0] = 1;
            c.sw(r32(c, S0).wrapping_add(0x5fb4), c.g[V0][0]);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int SSXAudioSystem_SetOutputMode(SSXAudioSystem* sys, int mode)`
/// (0x2854a8): apply `mode` through the audio object at `sys+0x118`'s slot
/// 0x1d8 field, and when that reports 2 remember the mode at +0x62b0.
/// 14 + 6 cycles plus the callee.
pub fn ssx_audio_system_set_output_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        let mode = c.g[A1][0];
        c.g[S0][0] = mode;
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        let obj = c.lw(p(self_, 0x118));
        c.g[V0][0] = obj;
        let vt = c.lw(obj as u32);
        c.g[V1][0] = vt;
        c.g[A0][0] = c.lw(p(vt as u32, 0x1d8));
        call_ptr(c, callee::AUDIO_SET_OUTPUT, 0x0028_54d0)?;
        let result = c.g[V0][0];
        c.g[V1][0] = result;
        c.g[V0][0] = 2;
        if c.g[V1][0] == c.g[V0][0] {
            c.sw(p(self_, 0x62b0), c.g[S0][0]);
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x20);
        c.g[V0][0] = c.g[V1][0];
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Audio.

/// `void SSXAudioSystem_SetGainTarget(sAudioFade* self, int i, int smooth,
/// float target, float time)` (0x2887a8): store the gain target and, when it
/// differs from the current gain, derive the ramp rate from the time (a
/// `smooth` of 0 leaves the rate alone). Equal target zeroes the rate.
pub fn ssx_audio_system_set_gain_target(c: &mut Cpu) {
    c.cycles += 10;
    let this = r32(c, A0);
    c.g[A3][0] = c.g[A0][0];
    // self + i*4 (the per-channel slots).
    let ch = this.wrapping_add(r32(c, A1) << 2);
    c.g[V0][0] = sx(ch);
    c.swc1(12, p(ch, 0x6440)); // target[i]
    c.g[V1][0] = sx(ch);
    c.lwc1(0, p(ch, 0x63e4)); // cur[i]
    c.c_eq_s(12, 0);
    if c.fcc() {
        Addr(p(ch, 0x6414)).sw(c, 0); // rate[i] = 0
    } else {
        c.cycles += 2;
        if c.g[A2][0] != 0 {
            c.cycles += 6;
            c.sub_s(0, 12, 0); // target - cur
            c.lwc1(1, gp_4(c, -0x4614)); // 1/60
            c.div_s(0, 0, 13); // / time
            c.mul_s(0, 0, 1); // * (1/60)
            c.swc1(0, p(ch, 0x6414));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void SSXAudioSystem_SetDuckGain(void* self, float v)` (0x289830): set the
/// duck gain at `+0x6c5c` through the fade system when it changed.
pub fn ssx_audio_system_set_duck_gain(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        c.lwc1(0, p(this, 0x6c5c));
        c.c_eq_s(0, 20);
        if !c.fcc() {
            c.cycles += 5;
            c.mtc1(0, 13); // fade time 0.0
            c.g[A1][0] = 0x1;
            callee::SET_FADE_GAIN.call(c, 0x0028_9868)?;
            c.swc1(20, p(this, 0x6c5c));
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Audio.

/// `int SSXAudioSystem_OptionToOutputMode(option)` (0x289d08): map the audio
/// option byte to its output mode.
pub fn ssx_audio_system_option_to_output_mode(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = c.g[A1][0] & 0xff;
    c.g[V1][0] = 1;
    let is1 = c.g[A1][0] == c.g[V1][0];
    c.g[V0][0] = ((c.g[A1][0] as i64) < 2i64) as u64;
    let mut to_289d50 = true;
    if is1 {
        c.cycles += 2;
        c.g[V1][0] = 2;
    } else {
        c.cycles += 2;
        let small = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0];
        if small {
            to_289d50 = false; // break b_289d54 with v0 = 1
        } else {
            c.cycles += 3;
            c.g[V0][0] = 2;
            let is2 = c.g[A1][0] == c.g[V0][0];
            c.g[V0][0] = 3;
            if is2 {
                c.cycles += 2;
                c.g[V1][0] = 0;
            } else {
                c.cycles += 2;
                let is3 = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = c.g[V1][0];
                if is3 {
                    c.cycles += 1;
                    c.g[V1][0] = 3;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    to_289d50 = false;
                }
            }
        }
    }
    if to_289d50 {
        c.cycles += 1;
        c.g[V0][0] = c.g[V1][0];
    }
    c.cycles += 2;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Audio.

/// `void SSXAudioSystem_OnAnimSoundEvent(void* a0, void* a1, int type)`
/// (0x289b18): dispatch an animation sound event (0x50/0x51/0x52).
pub fn ssx_audio_system_on_anim_sound_event(c: &mut Cpu) {
    c.cycles += 4;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 0x50;
    let not_50 = c.g[A2][0] != c.g[V0][0];
    frame.save_ra(c, 0);
    if not_50 {
        c.cycles += 3;
        c.g[V0][0] = 0x51;
        let not_51 = c.g[A2][0] != c.g[V0][0];
        c.g[V0][0] = 0x52;
        if not_51 {
            c.cycles += 2;
            let not_52 = c.g[A2][0] != c.g[V0][0];
            frame.restore_ra(c, 0);
            if !not_52 {
                c.cycles += 3;
                if call_ptr(c, 0x0029_c088, 0x0028_9b64).is_err() {
                    return;
                }
                frame.restore_ra(c, 0);
            }
        } else {
            c.cycles += 4;
            if call_ptr(c, 0x0029_b968, 0x0028_9b4c).is_err() {
                return;
            }
            frame.restore_ra(c, 0);
        }
    } else {
        c.cycles += 4;
        if call_ptr(c, 0x0029_bcf8, 0x0028_9b30).is_err() {
            return;
        }
        frame.restore_ra(c, 0);
    }
    c.cycles += 2;
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `int SSXAudioSystem_OutputModeToOption(int mode)` (0x289d60): map an
/// output-mode enum to its menu option index (1→0, 2→1, 0→2, 3→3).
pub fn ssx_audio_system_output_mode_to_option(c: &mut Cpu) {
    let mode = c.g[A1][0];
    let (option, extra) = if mode == 1 {
        (0u64, 1u64)
    } else if (mode as i64) < 2 {
        if mode == 0 {
            (2, 8)
        } else {
            (0, 7)
        }
    } else if mode == 2 {
        (1, 8)
    } else if mode == 3 {
        (3, 9)
    } else {
        (0, 9)
    };
    c.cycles += 3 + extra + 2;
    c.g[V1][0] = option;
    c.g[V0][0] = c.g[V1][0];
    abi::ret(c);
}


/// `SSXAudioSystem::LoadMemoryBank(self, id, data)` (0x286ca8): load one
/// memory-resident sound bank and mark it in the loaded/sub tables.
pub fn ssx_audio_system_load_memory_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let absent = c.g[A2][0] == 0;
        c.g[S1][0] = sx((c.g[S2][0] as u32) >> 8);
        if !absent {
            c.cycles += 12;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x118)).lw(c);
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
            c.g[A3][0] = 0x20000;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            call_ptr(c, 0x0028_be60, 0x0028_6ce0)?;
            c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[A1][0] = 1;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = c.g[S2][0] & 0xff;
            c.g[V1][0] = c.g[A0][0];
            Addr(p(c.g[A0][0] as u32, 0x6c70)).sw(c, c.g[A1][0]);
            Addr(p(c.g[V1][0] as u32, 0x6c78)).sw(c, c.g[V0][0]);
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


/// `SSXAudioSystem_Create` (0x284BB0): lazily allocate and construct the sound
/// system singleton, then tick its bank monitor. Returns the singleton.
pub fn ssx_audio_system_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp_5(c, 0x410)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let exists = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if !exists {
            c.cycles += 14;
            c.g[A1][0] = 0x48_0000;
            c.g[A2][0] = 0xffff_ffff_8000_0000;
            c.g[A1][0] = low(c, A1).wrapping_add(0x28b0);
            c.g[A0][0] = 0x7780;
            c.g[A2][0] |= 0x400;
            c.g[A3][0] = 0;
            call_ptr(c, addr::MEM_MAN_ALLOC, 0x0028_4be4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[T0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            c.g[A2][0] = 0x1c;
            c.g[A3][0] = 3;
            callee::AUDIO_SYSTEM_CTOR.call(c, 0x0028_4bfc)?;
            sw_at(c, GP, 0x410, c.g[V0][0]);
        }
        c.cycles += 9;
        c.g[V0][0] = Addr(gp_5(c, 0x410)).lw(c);
        c.g[V1][0] = lw_at(c, V0, 0x118);
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        c.g[A0][0] = low(c, A0).wrapping_add(0x1d8);
        call_ptr(c, addr::BANK_MONITOR_ADD, 0x0028_4c14)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `SSXAudioSystem_SetMusicVolume` (0x287410): set the music volume on the
/// music channels (1, 8 and, when the music is not on the alternative channel,
/// 2).
pub fn ssx_audio_system_set_music_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::LEVEL_TO_VOLUME.call(c, 0x0028_7428)?;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.mov_s(12, 20);
        callee::SET_CHANNEL_VOLUME.call(c, 0x0028_743c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 8;
        c.mov_s(12, 20);
        callee::SET_CHANNEL_VOLUME.call(c, 0x0028_744c)?;
        c.g[V0][0] = lw_at(c, S0, 0x62b8);
        let no_music = c.g[V0][0] == 0;
        c.g[V0][0] = 2;
        if !no_music {
            c.cycles += 3;
            c.g[V1][0] = lw_at(c, S0, 0x608c);
            let alts = c.g[V1][0] == c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            if !alts {
                c.cycles += 3;
                c.mov_s(12, 20);
                c.g[A1][0] = 2;
                callee::SET_CHANNEL_VOLUME.call(c, 0x0028_7470)?;
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}


/// `SSXAudioSystem_SetMusicChannelEnabled` (0x287558): turn the second music
/// channel on (remembering the first channel's volume) or mute it.
pub fn ssx_audio_system_set_music_channel_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let on = c.g[A1][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        let mut enable_path = false;
        if on {
            c.cycles += 4;
            c.g[V1][0] = lw_at(c, S0, 0x608c);
            c.g[V0][0] = 2;
            let is_alt = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 3;
            if !is_alt {
                c.cycles += 2;
                let is_alt2 = c.g[V1][0] == c.g[V0][0];
                if !is_alt2 {
                    c.cycles += 9;
                    c.g[A1][0] = 1;
                    callee::GET_CHANNEL_VOLUME.call(c, 0x0028_758c)?;
                    c.mov_s(12, 0);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 2;
                    callee::SET_CHANNEL_VOLUME.call(c, 0x0028_759c)?;
                    c.g[V0][0] = 1;
                    sw_at(c, S0, 0x62b8, c.g[V0][0]);
                    enable_path = true;
                }
            }
        }
        if !enable_path {
            c.cycles += 5;
            c.mtc1(0, 12);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 2;
            callee::SET_CHANNEL_VOLUME.call(c, 0x0028_75b8)?;
            sw_at(c, S0, 0x62b8, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `func_002A7FF8` (0x2a7ff8): `IniParser_GetString`. Expects an `=`-style
/// token (`0x41aa88(this+0xc)`), then the string `"0x4a3740"` (`IniParser_Expect`),
/// then reads a quoted string and copies it to `a2` (`0x4166f4`); returns 1 on
/// success, 0 otherwise. Signature: `int get_string(IniParser* this, void*,
/// char* out)`.
pub fn ini_parser_get_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        c.g[RA][0] = sx(0x002a_8018);
        c.g[A0][0] = sx(low_2(c, S0).wrapping_add(0xc));
        call_ptr(c, 0x0041_aa88, 0x002a_8018)?;
        let failed = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !failed {
            c.cycles += 6;
            c.g[A1][0] = 0x4a_0000;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002a_8030);
            c.g[A1][0] = sx(low_2(c, A1).wrapping_add(0x3740));
            callee::INI_PARSER_EXPECT.call(c, 0x002a_8030)?;
            let failed = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if !failed {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002a_8040);
                c.g[A0][0] = c.g[S0][0];
                callee::INI_PARSER_READ_QUOTED_STRING.call(c, 0x002a_8040)?;
                let empty = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S1][0];
                if empty {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002a_8050);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0041_66f4, 0x002a_8050)?;
                    c.g[V0][0] = 0x1;
                }
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


/// `void SSXAudioSystem_EnterGameInit(SSXAudioSystem* a)` (0x00286e20): enter
/// game mode (6), reset the world triggers, restart the board loops and store
/// the game module's id; in mode 1 say the "20BB" line.
pub fn ssx_audio_system_enter_game_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[A1][0] = 6;
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0028_5be0, 0x0028_6e38)?; // SSXAudioSystem_SetMode
        c.g[A0][0] = c.lw(gpa(c, 0x21e4));
        call_ptr(c, 0x002b_6900, 0x0028_6e40)?; // WorldTriggerManager_ResetPlayedList
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0029_29d8, 0x0028_6e48)?; // SSXAudioSystem_RestartAllBoardLoops
        call_ptr(c, 0x0028_b1c8, 0x0028_6e50)?; // SSXAudioSystem_GetGameModule
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x582c), c.g[V0][0]);
        call_ptr(c, 0x0014_dc80, 0x0028_6e60)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0028_6e6c)?; // BELibrary_GetInterface
        c.g[V1][0] = sx(0x0053_5c11);
        c.g[V0][0] = 1;
        c.g[V1][0] = c.lb(c.g[V1][0] as u32);
        let one = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 2;
        let run = if one {
            true
        } else {
            c.cycles += 2;
            c.g[V1][0] == c.g[V0][0]
        };
        if run {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_39e0, 0x0028_6e90)?; // SSXAudioSystem_PASay20BB
        }
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}


/// `IniParser* IniParser_ctor(IniParser* self, char* text, u32 flags)`
/// (0x2a77f8): measure the text, copy it into a fresh allocation and point the
/// cursor at_2 the copy's start.
pub fn ini_parser_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        Addr(c.g[S0][0] as u32).off(0x4).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        Addr(c.g[S0][0] as u32).off(0x8).sw(c, 0);
        call_ptr(c, table_callee::STRLEN, 0x002a_7828)?;
        let size = c.g[V0][0];
        Addr(c.g[S0][0] as u32).sw(c, size);
        if size != 0 {
            c.cycles += 16;
            c.g[A2][0] = c.g[S2][0];
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0;
            call_ptr(c, table_callee::OPERATOR_NEW, 0x002a_7844)?;
            let text = c.g[V0][0];
            c.g[A2][0] = lw_2(c, S0, 0);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = text;
            Addr(c.g[S0][0] as u32).off(0x4).sw(c, text);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            call_ptr(c, table_callee::STRCPY, 0x002a_785c)?;
            c.g[V1][0] = lw_2(c, S0, 0x4);
            c.g[V0][0] = lw_2(c, S0, 0);
            Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V1][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            Addr(c.g[V1][0] as u32).sb(c, 0);
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `SSXAudioSystem_SetSfxVolume` (0x287488): convert the level to a volume
/// and apply it to the SFX channels (plus 9/10 when the extra bank exists).
pub fn ssx_audio_system_set_sfx_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0028_74a0);
        c.g[S0][0] = c.g[A0][0];
        callee::AUDIO_LEVEL_TO_VOLUME_2.call(c, 0x0028_74a0)?;
        c.mov_s(20, 0);
        for (channel, ret) in [
            (5u64, 0x0028_74b4u32),
            (3, 0x0028_74c4),
            (6, 0x0028_74d4),
            (7, 0x0028_74e4),
        ] {
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = channel;
            c.g[RA][0] = sx(ret);
            c.mov_s(12, 20);
            callee::AUDIO_SET_CHANNEL_VOLUME_2.call(c, ret)?;
        }
        c.g[V0][0] = Addr(p(r(c, S0), 0x62b4)).lw(c);
        let no_extra = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_extra {
            c.cycles += 7;
            c.g[A1][0] = 9;
            c.g[RA][0] = sx(0x0028_74fc);
            c.mov_s(12, 20);
            callee::AUDIO_SET_CHANNEL_VOLUME_2.call(c, 0x0028_74fc)?;
            c.g[A0][0] = c.g[S0][0];
            c.mov_s(12, 20);
            c.g[RA][0] = sx(0x0028_750c);
            c.g[A1][0] = 0xa;
            callee::AUDIO_SET_CHANNEL_VOLUME_2.call(c, 0x0028_750c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Audio system.

/// `void SSXAudioSystem_SetSfxSubChannelsEnabled(AudioSystem* self, int on)`
/// (0x2875d0): mute sound-effect channels 9 and 10 (volume 0) when `on` is
/// zero; otherwise restore each to the channel-5 volume and remember the
/// setting at `+0x62b4`.
pub fn ssx_audio_system_set_sfx_sub_channels_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let off = c.g[S1][0] == 0;
        frame.save_fpr(c, 20, 0x30);
        if off {
            c.cycles += 10;
            c.mtc1(0, 20);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 9;
            c.mov_s(12, 20);
            call_ptr(c, 0x0028_7700, 0x0028_7640)?; // SetChannelVolume
            c.mov_s(12, 20);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0028_7700, 0x0028_7650)?;
            sw(c, p(c.g[S0][0] as u32, 0x62b4), c.g[S1][0]);
        } else {
            c.cycles += 15;
            c.g[A1][0] = 5;
            call_ptr(c, 0x0028_7920, 0x0028_75f8)?; // GetChannelVolume
            c.mov_s(12, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 9;
            call_ptr(c, 0x0028_7700, 0x0028_7608)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 5;
            call_ptr(c, 0x0028_7920, 0x0028_7614)?;
            c.mov_s(12, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0028_7700, 0x0028_7624)?;
            sw(c, p(c.g[S0][0] as u32, 0x62b4), c.g[S1][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });
}


/// `void SndLoadQueue_dtor(this, flags)` (0x28a148): flush the queue and free
/// the four owned buffers.
pub fn snd_load_queue_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0048_3b10;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        sw_2(c, S0, 0x1d4, c.g[V0][0]);
        callee::SND_LOAD_QUEUE_FLUSH.call(c, 0x0028_a170)?;
        c.g[A0][0] = lw_3(c, S0, 0x1c0);
        let first_null = c.g[A0][0] == 0;
        Addr(gp(c, 0x520)).sw(c, 0);
        if !first_null {
            c.cycles += 2;
            call_ptr(c, table::EA_MEM_FREE, 0x0028_a184)?;
        }
        c.cycles += 3;
        c.g[A0][0] = lw_3(c, S0, 0x1b0);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = lw_3(c, S0, 0x1a0);
        } else {
            c.cycles += 3;
            call_ptr(c, table::EA_MEM_FREE, 0x0028_a198)?;
            c.g[A0][0] = lw_3(c, S0, 0x1a0);
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.g[A0][0] = lw_3(c, S0, 0x190);
        } else {
            c.cycles += 3;
            call_ptr(c, table::EA_MEM_FREE, 0x0028_a1ac)?;
            c.g[A0][0] = lw_3(c, S0, 0x190);
        }
        c.cycles += 2;
        let third_null = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !third_null {
            c.cycles += 3;
            call_ptr(c, table::EA_MEM_FREE, 0x0028_a1c0)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0028_a1d4)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void SSXAudioSystem_UnloadWorldBanks(self)` (0x286c00): finish the
/// current load, unload the world's banks 0..16 (except 13) and free the
/// bank memory.
pub fn ssx_audio_system_unload_world_banks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0;
        call_ptr(c, 0x002a_6f38, 0x00286c20)?;
        c.g[S2][0] = 0xd;
        c.g[V0][0] = Addr(p(r32(c, S1), 0x118)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        call_ptr(c, 0x0028_9df0, 0x00286c34)?;
        loop {
            c.cycles += 2;
            let is13 = c.g[S0][0] == c.g[S2][0];
            c.g[A1][0] = c.g[S0][0];
            if !is13 {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x118)).lw(c);
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, 0x0028_bce8, 0x00286c4c)?;
            }
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 17) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = 3;
            if !more {
                c.cycles += 18;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x118)).lw(c);
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, 0x0028_bde0, 0x00286c68)?;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x118)).lw(c);
                c.g[A1][0] = 0xc;
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, 0x0028_bde0, 0x00286c78)?;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x118)).lw(c);
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, 0x0028_a230, 0x00286c84)?;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0029_f5e0, 0x00286c8c)?;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_6c38);
            }
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void SSXAudioSystem_OnResetRace(self)` (0x287108): reset the race sounds
/// and, once the race manager agrees, restart the race music.
pub fn ssx_audio_system_on_reset_race(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0028_71b0, 0x0028711c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002a_4168, 0x00287124)?;
        let mut proceed = c.g[V0][0] != 0;
        if !proceed {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_41d8, 0x00287134)?;
            proceed = c.g[V0][0] != 0;
        }
        if proceed {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_4040, 0x00287144)?;
            if c.g[V0][0] == 0 {
                c.cycles += 7;
                c.g[A1][0] = Addr(gp_6(c, -0x848)).lw(c);
                c.g[V0][0] = Addr(p(r32(c, S0), 0x5824)).lw(c);
                c.g[V1][0] = Addr(p(r32(c, A1), 0xc0)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V1), 0x70)).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x0029_5028, 0x00287178)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0;
                        c.g[A2][0] = 0;
                        c.g[A3][0] = sx(0xffff_ffff);
                        c.g[T0][0] = 0;
                        c.g[T1][0] = 0;
                        call_ptr(c, 0x0029_5628, 0x0028719c)?;
                    }
                }
            }
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Audio system: unload front-end audio / song changed.

/// `void SSXAudioSystem_UnloadFrontEndAudio(self)` (0x286200): stop the
/// front-end music (depending on the two song slots), kill bank timers and
/// unload the front-end bank.
pub fn ssx_audio_system_unload_front_end_audio(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        'end: {
            'second: {
                'first: {
                    c.cycles += 8;
                    c.g[V1][0] = 2;
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = lw_4(c, at_4(c.g[S0][0], 0x608c));
                    let t = c.g[V0][0] == c.g[V1][0];
                    if t {
                        break 'first;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = lw_4(c, at_4(c.g[S0][0], 0x6090));
                    if c.g[V0][0] != c.g[V1][0] {
                        break 'second;
                    }
                    break 'first;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x0028_6234);
                c.g[A0][0] = sx(at_4(c.g[S0][0], 0x118));
                call_ptr(c, 0x002b_3ac0, 0x0028_6234)?;
                c.g[A1][0] = 0;
                break 'end;
            }
            c.cycles += 5;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[RA][0] = sx(0x0028_624c);
            c.g[A0][0] = sx(at_4(c.g[S0][0], 0x118));
            call_ptr(c, 0x002b_3d48, 0x0028_624c)?;
            c.g[A1][0] = 0;
            break 'end;
        }
        c.cycles += 22;
        c.g[RA][0] = sx(0x0028_6258);
        c.g[A0][0] = c.g[S0][0];
        callee::SSX_AUDIO_SYSTEM_SET_MODE.call(c, 0x0028_6258)?;
        c.g[V0][0] = lw_4(c, at_4(c.g[S0][0], 0x118));
        c.g[A0][0] = lw_4(c, c.g[V0][0] as u32);
        c.g[RA][0] = sx(0x0028_6268);
        c.g[A0][0] = sx(at_4(c.g[A0][0], 0x1d8));
        call_ptr(c, 0x002a_dda0, 0x0028_6268)?;
        c.g[V0][0] = lw_4(c, at_4(c.g[S0][0], 0x118));
        c.g[A0][0] = lw_4(c, c.g[V0][0] as u32);
        c.g[V1][0] = lw_4(c, at_4(c.g[A0][0], 0xab0));
        c.g[A0][0] = sx(at_4(c.g[A0][0], 0x1d8));
        c.g[V0][0] = lh(c, at_4(c.g[V1][0], 0x10));
        c.g[A1][0] = lw_4(c, at_4(c.g[V1][0], 0x14));
        let target = c.g[A1][0] as u32;
        c.g[RA][0] = sx(0x0028_6288);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, target, 0x0028_6288)?;
        c.g[V0][0] = lw_4(c, at_4(c.g[S0][0], 0x118));
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x0028_6298);
        c.g[A0][0] = lw_4(c, c.g[V0][0] as u32);
        callee::SND_BANK_MAN_UNLOAD_BANK.call(c, 0x0028_6298)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `SndLoadQueue_ctor(self)` (0x28a058): five 0x18-byte queue nodes with the
/// same vtable, linked into a count-6 ring, then publish the queue in the
/// sound system's global.
pub fn sndloadqueue_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 60;
        let frame = Frame::open(c, 0x40);
        let this = abi::arg(c, 0);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, 16, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3b10));
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        c.g[S1][0] = 0x48_0000;
        frame.save_ra(c, 0);
        c.g[S2][0] = 6;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2820));
        c.sw(p(c.g[S0][0] as u32, 0x18c), c.g[S2][0]);
        c.sw(p(c.g[S0][0] as u32, 4), 0);
        c.g[A1][0] = c.g[S1][0];
        c.sw(p(c.g[S0][0] as u32, 8), 0);
        c.g[A0][0] = 0x18;
        c.sw(p(c.g[S0][0] as u32, 0x194), 0);
        c.g[A2][0] = 0;
        c.sw(p(c.g[S0][0] as u32, 0x198), 0);
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x0028_a0b0);
        c.sw(p(c.g[S0][0] as u32, 0x1d4), c.g[V0][0]);
        call_ptr(c, 0x0031_7e30, 0x0028_a0b0)?; // operator new(0x18, ...)
        c.sw(p(c.g[S0][0] as u32, 0x19c), c.g[S2][0]);
        c.g[A1][0] = c.g[S1][0];
        c.sw(p(c.g[S0][0] as u32, 0x1a4), 0);
        c.g[A0][0] = 0x18;
        c.sw(p(c.g[S0][0] as u32, 0x1a8), 0);
        c.g[A2][0] = 0;
        c.sw(p(c.g[S0][0] as u32, 0x190), c.g[V0][0]);
        c.g[RA][0] = sx(0x0028_a0d4);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0028_a0d4)?;
        c.sw(p(c.g[S0][0] as u32, 0x1ac), c.g[S2][0]);
        c.g[A1][0] = c.g[S1][0];
        c.sw(p(c.g[S0][0] as u32, 0x1b4), 0);
        c.g[A0][0] = 0x18;
        c.sw(p(c.g[S0][0] as u32, 0x1b8), 0);
        c.g[A2][0] = 0;
        c.sw(p(c.g[S0][0] as u32, 0x1a0), c.g[V0][0]);
        c.g[RA][0] = sx(0x0028_a0f8);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0028_a0f8)?;
        c.sw(p(c.g[S0][0] as u32, 0x1bc), c.g[S2][0]);
        c.g[A1][0] = c.g[S1][0];
        c.sw(p(c.g[S0][0] as u32, 0x1c4), 0);
        c.g[A0][0] = 0x18;
        c.sw(p(c.g[S0][0] as u32, 0x1c8), 0);
        c.g[A2][0] = 0;
        c.sw(p(c.g[S0][0] as u32, 0x1b0), c.g[V0][0]);
        c.g[RA][0] = sx(0x0028_a11c);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0028_a11c)?;
        c.sw(p(c.g[S0][0] as u32, 0x1c0), c.g[V0][0]);
        c.sw(p(c.g[S0][0] as u32, 0x1cc), 0);
        c.g[V0][0] = c.g[S0][0];
        c.sw(p(c.g[S0][0] as u32, 0x1d0), 0);
        c.sw(gp(c, 0x520), c.g[S0][0]);
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void SSXAudioSystem_SetFadeGain(SaMix* self, int mask, float time,
/// float v)` (0x287a10): with `time == 0` set the channels selected by
/// `mask` immediately and, for the master channel (bit 2), recompute the ten
/// mix outputs from its value; otherwise store a fade (mask/target/rate) and
/// mark it running when the rate is non-zero.
pub fn ssx_audio_system_set_fade_gain(c: &mut Cpu) {
    'b_287c3c: {
        c.cycles += 5;
        c.mtc1(0, 2);
        c.c_eq_s(13, 2);
        let t = !c.fcc();
        c.g[V0][0] = c.g[A1][0] & 0x1;
        if t {
            c.cycles += 11;
            c.lwc1(0, c.pa(A0, 0x63d0));
            c.lwc1(1, c.pa(GP, -0x4620));
            c.sub_s(0, 12, 0);
            c.stw_2(A0, 0x63c4, c.g[A1][0]);
            c.swc1(12, c.pa(A0, 0x63cc));
            c.div_s(0, 0, 13);
            c.mul_s(0, 0, 1);
            c.c_eq_s(0, 2);
            let t = c.fcc();
            c.swc1(0, c.pa(A0, 0x63c8));
            if t {
                break 'b_287c3c;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                c.stw_2(A0, 0x63c0, c.g[V0][0]);
                break 'b_287c3c;
            }
        } else {
            'b_287a48: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x2;
                if t {
                    break 'b_287a48;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x62d0));
                    c.lwc1(1, c.pa(A0, 0x63e8));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x6398));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x62d4));
                    c.swc1(1, c.pa(A0, 0x62d8));
                    break 'b_287a48;
                }
            }
            'b_287a6c: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x4;
                if t {
                    break 'b_287a6c;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x6310));
                    c.lwc1(1, c.pa(A0, 0x63f8));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63a8));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x6314));
                    c.swc1(1, c.pa(A0, 0x6318));
                    break 'b_287a6c;
                }
            }
            'b_287b04: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.lwc1(1, c.pa(GP, 0x43c));
                if t {
                    break 'b_287b04;
                } else {
                    c.cycles += 36;
                    c.lwc1(11, c.pa(A0, 0x6300));
                    c.lwc1(0, c.pa(A0, 0x63f4));
                    c.mul_s(11, 11, 12);
                    c.lwc1(2, c.pa(GP, 0x440));
                    c.lwc1(4, c.pa(GP, 0x444));
                    c.mul_s(0, 11, 0);
                    c.lwc1(5, c.pa(GP, 0x448));
                    c.lwc1(7, c.pa(GP, 0x44c));
                    c.lwc1(9, c.pa(GP, 0x450));
                    c.lwc1(10, c.pa(GP, 0x454));
                    c.mul_s(1, 0, 1);
                    c.lwc1(8, c.pa(GP, 0x458));
                    c.mul_s(2, 0, 2);
                    c.lwc1(6, c.pa(GP, 0x45c));
                    c.mul_s(4, 0, 4);
                    c.lwc1(3, c.pa(GP, 0x460));
                    c.mul_s(5, 0, 5);
                    c.mul_s(7, 0, 7);
                    c.swc1(1, c.pa(A0, 0x636c));
                    c.mul_s(3, 0, 3);
                    c.swc1(2, c.pa(A0, 0x6370));
                    c.mul_s(9, 0, 9);
                    c.swc1(4, c.pa(A0, 0x6374));
                    c.mul_s(10, 0, 10);
                    c.swc1(5, c.pa(A0, 0x6378));
                    c.mul_s(8, 0, 8);
                    c.swc1(7, c.pa(A0, 0x637c));
                    c.mul_s(6, 0, 6);
                    c.swc1(9, c.pa(A0, 0x6380));
                    c.swc1(10, c.pa(A0, 0x6384));
                    c.swc1(8, c.pa(A0, 0x6388));
                    c.swc1(6, c.pa(A0, 0x638c));
                    c.swc1(3, c.pa(A0, 0x6390));
                    c.swc1(12, c.pa(A0, 0x63a4));
                    c.swc1(11, c.pa(A0, 0x6304));
                    c.swc1(0, c.pa(A0, 0x6308));
                    break 'b_287b04;
                }
            }
            'b_287b2c: {
                c.cycles += 3;
                c.g[V0][0] = c.g[A1][0] & 0x8;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x80;
                if t {
                    break 'b_287b2c;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x62e0));
                    c.lwc1(1, c.pa(A0, 0x63ec));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x639c));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x62e4));
                    c.swc1(1, c.pa(A0, 0x62e8));
                    break 'b_287b2c;
                }
            }
            'b_287b50: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x20;
                if t {
                    break 'b_287b50;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x62f0));
                    c.lwc1(1, c.pa(A0, 0x63f0));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63a0));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x62f4));
                    c.swc1(1, c.pa(A0, 0x62f8));
                    break 'b_287b50;
                }
            }
            'b_287b74: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x40;
                if t {
                    break 'b_287b74;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x6320));
                    c.lwc1(1, c.pa(A0, 0x63fc));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63ac));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x6324));
                    c.swc1(1, c.pa(A0, 0x6328));
                    break 'b_287b74;
                }
            }
            'b_287b98: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x10;
                if t {
                    break 'b_287b98;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x6330));
                    c.lwc1(1, c.pa(A0, 0x6400));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63b0));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x6334));
                    c.swc1(1, c.pa(A0, 0x6338));
                    break 'b_287b98;
                }
            }
            'b_287bbc: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x100;
                if t {
                    break 'b_287bbc;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x6340));
                    c.lwc1(1, c.pa(A0, 0x6404));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63b4));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x6344));
                    c.swc1(1, c.pa(A0, 0x6348));
                    break 'b_287bbc;
                }
            }
            'b_287be0: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x200;
                if t {
                    break 'b_287be0;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x6350));
                    c.lwc1(1, c.pa(A0, 0x6408));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63b8));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x6354));
                    c.swc1(1, c.pa(A0, 0x6358));
                    break 'b_287be0;
                }
            }
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;

            if t {
                break 'b_287c3c;
            } else {
                c.cycles += 8;
                c.lwc1(0, c.pa(A0, 0x6360));
                c.lwc1(1, c.pa(A0, 0x640c));
                c.mul_s(0, 0, 12);
                c.swc1(12, c.pa(A0, 0x63bc));
                c.mul_s(1, 0, 1);
                c.swc1(0, c.pa(A0, 0x6364));
                let ra = c.g[RA][0] as u32;
                c.swc1(1, c.pa(A0, 0x6368));
                c.pc = ra;
                return;
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
    return;
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
    use crate::difftest::elf_image;
    use crate::difftest::fill;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::SP;
    use crate::difftest::put_vec;
    use ssx3_ee as ee;
    use crate::difftest::fakes;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::ptr;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::SP as STACK;
    use crate::difftest::compare;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::sound::asyncsys::tests::audio_world;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::dirtysock::tags2::tests::SCRATCH;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::main::gameload::tests::fb;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::belib_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::string_2 as string;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::fepopup3::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_3;
    pub(crate) use crate::game::fe::festaterewards::tests::w16;
    pub(crate) use crate::game::be::beintplayer::tests::w8;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3 as globals_2;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4 as pool_3;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3 as heap_init_2;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_7;
    pub(crate) use crate::game::sound::bankmonitor::tests::POOL_BLOCK;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2;
    pub(crate) use crate::game::sound::bankmanager::tests::SYS;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH as SCRATCH_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_9;
    pub(crate) use crate::game::ai::ai::tests::strict;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::globals_3 as globals_5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::pool_4 as pool_5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6 as base_10;
    pub(crate) use crate::game::render::graphicsman::tests::same;


    #[test]
    fn ssx_audio_system_level_to_volume_matches() {
        check(ssx3_ee::s07::f_002873d8, ssx_audio_system_level_to_volume, 300, &|c, seed| {
            let mut r = Rng(0x1111_0006 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let scale = match seed % 3 {
                0 => 1.0f32.to_bits(),
                1 => 2.0f32.to_bits(),
                _ => r.edge_f32(),
            };
            c.bus.write32(GAME_GP - 0x462c, scale);
            c.g[A1][0] = match seed % 4 {
                0 => (-5i32) as u32 as u64,
                1 => 0,
                2 => 5,
                _ => r.next_u32() as u64,
            };
        });
    }


    #[test]
    fn ssx_audio_system_get_channel_gain_matches() {
        check(ssx3_ee::s07::f_00287930, ssx_audio_system_get_channel_gain, 300, &|c, seed| {
            let mut r = Rng(0x1111_0008 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let a1 = (seed % 5) as u64; // 4 exercises the sub-channel path
            let a2 = match seed % 4 {
                0 => 3,
                1 => 12,
                2 => 0xffff_ffff_0000_0007, // negative i64 (< 10), low word stays small
                _ => (seed % 6) as u64,
            };
            c.g[A1][0] = a1;
            c.g[A2][0] = a2;
            c.bus.write32(DATA + (a1 as u32) * 0x10 + 0x62c8, r.edge_f32());
            c.bus.write32(DATA + ((a2 as u32) << 2) + 0x636c, r.edge_f32());
        });
    }


    #[test]
    fn ssx_audio_system_compute_3d_pan_matches() {
        check(ee::s07::f_00285930__generated, ssx_audio_system_compute_3d_pan, 240, &|c, seed| {
            let mut r = Rng(0x7a11_0006 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            let app = DATA;
            let module = if seed % 11 == 0 { 0 } else { DATA + 0x100 };
            let players = DATA + 0x200;
            let emitter = DATA + 0x300;
            let pos = DATA + 0x400;
            let listener = DATA + 0x500;
            let this = DATA + 0x1000;
            let out = DATA + 0x8000;
            c.bus.write32(GAME_GP - 0x848, app);
            c.bus.write32(app + 0x84, module);
            c.bus.write32(DATA + 0x100 + 0x84, players);
            let player = r.next_u32() % 4;
            c.bus.write32(emitter + 0x84, player);
            c.bus.write32(emitter + 0x68, pos);
            c.bus.write32(emitter + 0xa0, if seed % 5 == 0 { r.next_u32() | 1 } else { 0 });
            c.bus.write32(emitter + 0xa4, r.next_u32() & 1);
            c.bus.write32(players + 4 * player, listener);
            let identity = seed % 2 == 0;
            for row in 0..4u32 {
                for col in 0..4u32 {
                    let v = if identity {
                        if row == col { 0x3f80_0000 } else { 0 }
                    } else {
                        r.f32(2.0)
                    };
                    c.bus.write32(listener + 0x40 + 16 * row + 4 * col, v);
                }
            }
            let mut x = r.f32(500.0);
            let mut y = r.f32(500.0);
            let mut z = r.f32(500.0);
            if seed % 4 == 0 {
                z = 100.0f32.to_bits();
            }
            if seed % 8 == 0 {
                x = if seed % 16 == 0 { 0 } else { 0x8000_0000 };
            }
            if seed % 16 == 0 || seed % 6 == 2 {
                y = (-25.0f32).to_bits();
            }
            c.bus.write32(pos, x);
            c.bus.write32(pos + 4, y);
            c.bus.write32(pos + 8, z);
            c.bus.write32(this + 0x62b0, (seed % 6 != 0) as u32);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(emitter);
            c.g[A2][0] = sx(out);
        });
    }


    #[test]
    fn ssx_audio_system_get_listener_count_matches() {
        check(ee::s07::f_00288ce8__generated, ssx_audio_system_get_listener_count, SEEDS, &|c, seed| {
            let mut r = Rng(0x0028_8ce8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let world = audio_world(c);
            if seed % 2 == 0 {
                // No audio system: the listener count is the literal 1.
                c.bus.write32(world + 0x84, 0);
            } else {
                c.bus.write32(world + 0x84, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 0x10, r.next_u32());
            }
        });
    }


    #[test]
    fn ssx_audio_system_is_sound_mode2_matches() {
        check(
            ssx3_ee::s07::f_002899e8__generated,
            ssx_audio_system_is_sound_mode2,
            200,
            &|c, seed| {
                let mut r = Rng(0x2899_e801 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let v = match seed % 3 {
                    0 => 2,
                    1 => r.next_u32(),
                    _ => 3,
                };
                c.bus.write32(DATA + 0x608c, v);
                c.pc = 0x0028_99e8;
            },
        );
    }


    #[test]
    fn ssx_audio_system_set_mode_matches() {
        check(ee::s07::f_00285be0__generated, ssx_audio_system_set_mode, SEEDS_2, &|c, seed| {
            let mut r = rng(0x0028_5be0, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            let old = r.next_u32();
            c.bus.write32(DATA + 0x6474, old);
            c.g[A1][0] = match seed % 3 {
                0 => old as u64,
                1 => mixed(&mut r, seed, old),
                _ => r.next_u32() as u64,
            };
        });
    }


    // ---------------------------------------------------------------- audio

    /// A `SSXAudioSystem` at `SCRATCH` with a per-channel volume at
    /// `+0x62bc + channel*16`.
    pub(crate) fn audio_volume_setup(c: &mut Cpu, seed: u32, salt: u64, channel: u32) {
        let mut r = Rng(salt ^ (seed as u64) << 9);
        crate::difftest::scramble(c, &mut r);
        c.g[GP][0] = crate::difftest::GAME_GP as u64;
        c.g[A0][0] = SCRATCH as u64;
        c.bus.write32(
            SCRATCH + 0x62bc + 16 * channel,
            if seed % 3 == 0 { r.edge_f32() } else { r.f32(1.0) },
        );
    }


    #[test]
    fn ssx_audio_system_get_music_volume_level_matches() {
        check(
            ee::s07::f_00287670__generated,
            ssx_audio_system_get_music_volume_level,
            120,
            &|c, seed| audio_volume_setup(c, seed, 0x287670, 1),
        );
    }


    #[test]
    fn ssx_audio_system_get_sfx_volume_level_matches() {
        check(
            ee::s07::f_002876a0__generated,
            ssx_audio_system_get_sfx_volume_level,
            120,
            &|c, seed| audio_volume_setup(c, seed, 0x2876a0, 5),
        );
    }


    #[test]
    fn ssx_audio_system_get_speech_volume_level_matches() {
        check(
            ee::s07::f_002876d0__generated,
            ssx_audio_system_get_speech_volume_level,
            120,
            &|c, seed| audio_volume_setup(c, seed, 0x2876d0, 4),
        );
    }


    #[test]
    fn ssx_audio_system_crowd_state_callback_matches() {
        check(ee::s07::f_00289650__generated, ssx_audio_system_crowd_state_callback, 100, &|c, seed| {
            let _r = machine(c, seed, 0x289650, 1);
            let state = SCRATCH + 0x200;
            c.g[A0][0] = state as u64;
            c.bus.write32(state, if seed % 2 == 0 { 3 } else { (seed % 5) as u32 });
            c.bus.write32(crate::difftest::GAME_GP + 0x410, SCRATCH + 0x300); // D_004A3500
        });
    }


    // ------------------------------------------------------------------
    // Audio.

    #[test]
    fn audio_stop_all_sounds_matches() {
        check(ssx3_ee::s07::f_00285f48__generated, audio_stop_all_sounds, SEEDS, &|c, seed| {
            let _r = machine(c, seed, 0x285f48, 1);
            let sys = DATA + 0x100;
            // No crowd instances and no sound-system source list.
            c.bus.write32(sys + 0x14, 0);
            c.bus.write32(sys + 0x118, 0);
            c.g[A0][0] = sx(sys);
        });
    }


    #[test]
    fn audio_set_speech_volume_matches() {
        check(ssx3_ee::s07::f_00287520__generated, audio_set_speech_volume, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x287520, 2);
            let sys = DATA + 0x100;
            fill(c, &mut r, sys, 0x1000);
            c.bus.write32(GAME_GP.wrapping_sub(0x462c), r.edge_f32());
            c.g[A0][0] = sx(sys);
            c.g[A1][0] = (r.next_u32() % 12) as u64;
        });
    }


    pub(crate) const GP_DB0: u32 = 0x49ea_d8;


    // --------------------------------------------------------------- sound

    #[test]
    fn ssx_audio_system_get_preset_gain_matches() {
        check(ssx3_ee::s07::f_00288370__generated, ssx_audio_system_get_preset_gain, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x2883_70, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            let index = seed % 4;
            c.g[A1][0] = index as u64;
            w(c, GP_DB0, fb(0.01));
            for i in 0..4u32 {
                w(c, DATA + 0x6440 + i * 4, r.edge_f32());
                w(c, DATA + 0x63e4 + i * 4, r.edge_f32());
            }
            match seed % 3 {
                0 => c.g[A2][0] = 0,
                1 => c.g[A2][0] = (r.next_u32() % 1000) as u64,
                _ => {
                    c.g[A2][0] = sx(0xffff_ffff);
                    w(c, DATA + 0x6410, (seed % 2) as u32);
                }
            }
        });
    }


    // ------------------------------------------------------------------
    // Audio.

    #[test]
    fn ssx_audio_system_destroy_matches() {
        check(
            ee::s07::f_00284c28__generated,
            ssx_audio_system_destroy,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x284c28, 0);
                let sys = DATA + 0x300;
                let inner = DATA + 0x400;
                let obj = DATA + 0x500;
                let vt = vtable(c, &mut r, &[8]);
                c.bus
                    .write32(GAME_GP + 0x410, if seed % 3 != 0 { sys } else { 0 });
                c.bus.write32(sys + 0x118, inner);
                c.bus.write32(inner, obj);
                c.bus.write32(obj + 0x1d4, vt);
            },
        );
    }


    #[test]
    fn ssx_audio_system_pause_game_audio_matches() {
        check(
            ee::s07::f_00289b70__generated,
            ssx_audio_system_pause_game_audio,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28_9b70, 0);
                let obj = DATA + 0x1000;
                c.bus
                    .write32(obj + 0x5fb4, if seed % 2 == 0 { 1 } else { 0 });
                c.bus.write32(obj + 0x118, DATA + 0x2000);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }


    #[test]
    fn ssx_audio_system_set_output_mode_matches() {
        check(ee::s07::f_002854a8__generated, ssx_audio_system_set_output_mode, 80, &|c, seed| {
            let mut r = base(c, seed, 0x2854);
            let sys = DATA;
            let obj = DATA + 0x1000;
            c.bus.write32(sys + 0x118, obj);
            c.bus.write32(obj, DATA + 0x2000);
            c.bus.write32(DATA + 0x2000 + 0x1d8, DATA + 0x3000);
            fill(c, &mut r, DATA + 0x3000, 0x20);
            c.bus.write32(sys + 0x62b0, 0xdead);
            c.g[A0][0] = sx(sys);
            c.g[A1][0] = sx(r.next_u32() % 8);
        });
    }


    // ------------------------------------------------------------------
    // Audio.

    #[test]
    fn ssx_audio_system_set_gain_target_matches() {
        check(
            ee::s07::f_002887a8__generated,
            ssx_audio_system_set_gain_target,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2887_a8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let ch = (seed % 5) as u32;
                let target = r.edge_f32();
                c.g[A0][0] = sx(this);
                c.g[A1][0] = ch as u64;
                c.g[A2][0] = (seed % 2) as u64;
                c.f[12] = target;
                c.f[13] = if seed % 7 == 0 { 0 } else { r.edge_f32() };
                c.bus.write32(gp_at(-0x4614), 0x3c88_8893); // 1/60
                let cur = if seed % 3 == 0 { target } else { r.edge_f32() };
                c.bus.write32(this + 0x63e4 + 4 * ch, cur);
            },
        );
    }


    #[test]
    fn ssx_audio_system_set_duck_gain_matches() {
        check(
            ee::s07::f_00289830__generated,
            ssx_audio_system_set_duck_gain,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2898_30, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let v = r.edge_f32();
                c.g[A0][0] = sx(this);
                c.f[12] = v;
                c.bus.write32(gp_at(-0x4620), 0x3c88_8893);
                let cur = if seed % 2 == 0 { v } else { r.edge_f32() };
                c.bus.write32(this + 0x6c5c, cur);
            },
        );
    }


    #[test]
    fn ssx_audio_system_option_to_output_mode_matches() {
        check(
            ee::s07::f_00289d08__generated,
            ssx_audio_system_option_to_output_mode,
            160,
            &|c, seed| {
                let mut r = Rng(0x289d08 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }


    #[test]
    fn ssx_audio_system_on_anim_sound_event_matches() {
        check(
            ssx3_ee::s07::f_00289b18__generated,
            ssx_audio_system_on_anim_sound_event,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x289b18, 3);
                let a0 = DATA + 0x400;
                let a1 = DATA + 0x800;
                // A null sfx table at `+0x874` makes every handler return early.
                c.bus.write32(a1 + 0x874, 0);
                c.g[A0][0] = a0 as u64;
                c.g[A1][0] = a1 as u64;
                c.g[A2][0] = [0x50u64, 0x51, 0x52, 0x53, 0][(seed % 5) as usize];
            },
        );
    }


    #[test]
    fn ssx_audio_system_output_mode_to_option_matches() {
        check(
            ssx3_ee::s07::f_00289d60__generated,
            ssx_audio_system_output_mode_to_option,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x289d60, 0);
                c.g[A1][0] = match seed % 7 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 4,
                    5 => 0x1_0000_0001,
                    _ => u64::MAX,
                };
            },
        );
    }


    #[test]
    fn ssx_audio_system_load_memory_bank_matches() {
        check(
            ee::s07::f_00286ca8__generated,
            ssx_audio_system_load_memory_bank,
            6,
            &|c, seed| {
                let _ = base_2(c, seed, 0x0028_6ca8, 2);
                let id = 0x100 + (seed % 4);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = id as u64;
                c.g[A2][0] = if seed % 2 == 0 { 0 } else { 1 };
                // manager -> bank table: keep the bank header's free link at
                // -1 so the unload path does not walk the test heap.
                let manager = DATA + 0x3200;
                w(c, OBJ + 0x118, DATA + 0x3000);
                w(c, DATA + 0x3000, manager);
                w(c, DATA + 0x3ccc, DATA + 0x4000);
                let handle = (id >> 8) + 8;
                w(c, DATA + 0x4000 + 0x60 * handle + 4, 0xffff_ffff);
            },
        );
    }


    #[test]
    fn ssx_audio_system_create_matches() {
        check(ee::s07::f_00284bb0__generated, ssx_audio_system_create, SEEDS_4, &|c, seed| {
            machine(c, seed, 0x28_4bb0, 3);
            belib_2(c);
            heap_setup(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            // An existing sound system (the singleton create path allocates
            // and constructs the whole object).
            w(c, GAME_GP + 0x410, DATA + 0x3000);
            w(c, DATA + 0x3000 + 0x118, DATA + 0x3200);
            w(c, DATA + 0x3200, DATA + 0x3400);
            c.pc = 0x0028_4bb0;
        });
    }


    #[test]
    fn ssx_audio_system_set_music_volume_matches() {
        check(ee::s07::f_00287410__generated, ssx_audio_system_set_music_volume, SEEDS_4, &|c, seed| {
            let mut r = machine(c, seed, 0x28_7410, 4);
            belib_2(c);
            c.g[A0][0] = (DATA + 0x7000) as u64;
            c.g[A1][0] = (seed % 11) as u64;
            c.f[12] = r.edge_f32();
            w(c, DATA + 0x7000 + 0x62b8, (seed % 2) as u32);
            w(c, DATA + 0x7000 + 0x608c, (seed % 4) as u32);
            c.pc = 0x0028_7410;
        });
    }


    #[test]
    fn ssx_audio_system_set_music_channel_enabled_matches() {
        check(
            ee::s07::f_00287558__generated,
            ssx_audio_system_set_music_channel_enabled,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x28_7558, 3);
                belib_2(c);
                c.g[A0][0] = (DATA + 0x7000) as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, DATA + 0x7000 + 0x608c, (seed % 4) as u32);
                c.pc = 0x0028_7558;
            },
        );
    }


    #[test]
    fn ini_parser_get_string_matches() {
        check(
            ee::s08::f_002a7ff8__generated,
            ini_parser_get_string,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x2a7ff8, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                let stream = DATA + 0x400;
                let key = DATA + 0x500;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0; // compared case-insensitively against `this+0xc`
                c.g[A2][0] = (DATA + 0x800) as u64;
                // `this+0xc` is the key the match compares against address 0 (so
                // it must be the empty string to pass); `this+8` is the character
                // stream. `ReadQuotedString`'s copy loop has no NUL check, so the
                // stream must always contain a closing quote.
                string(c, key, "");
                w(c, this + 0xc, key);
                w(c, this + 8, stream);
                match seed % 4 {
                    0 => string(c, stream, "\""),
                    1 => string(c, stream, "v\""),
                    2 => string(c, stream, "value\""),
                    _ => string(c, stream, "\"\""),
                }
            },
        );
    }


    #[test]
    fn ssx_audio_system_enter_game_init_matches() {
        check(
            ee::s07::f_00286e20__generated,
            ssx_audio_system_enter_game_init,
            60,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x286e, 3);
                w(
                    c,
                    GAME_GP.wrapping_add(0x21e4),
                    if seed % 3 == 0 { 0 } else { DATA + 0x400 },
                );
                wb(
                    c,
                    0x0053_5c11,
                    match seed % 4 {
                        0 => 1,
                        1 => 2,
                        _ => 0,
                    },
                );
                fill(c, &mut r, DATA, 0x40);
                c.g[A0][0] = sx(DATA);
            },
        );
    }


    #[test]
    fn ini_parser_ctor_matches() {
        check(ee::s08::f_002a77f8__generated, ini_parser_ctor, 40, &|c, seed| {
            let mut r = base_4(c, seed, 0x2a77f8, 3);
            let obj = DATA + 0x1000;
            let text = DATA + 0x6000;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = text as u64;
            c.g[A2][0] = r.next_u32() as u64;
            let n = if seed % 3 == 0 { 0 } else { 1 + seed % 40 } as u32;
            for i in 0..n {
                w8(c, text + i, b'a' + (seed as u8 % 26));
            }
            w8(c, text + n, 0);
        });
    }


    #[test]
    fn ssx_audio_system_set_sfx_volume_matches() {
        check(
            ee::s07::f_00287488__generated,
            ssx_audio_system_set_sfx_volume,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                w(c, this + 0x62b4, (seed & 1) as u32);
                c.g[A0][0] = (seed % 100) as u64;
                let _ = this;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 101) as u64;
            }),
        );
    }


    // ------------------------------------------------------------------ audio

    #[test]
    fn ssx_audio_system_set_sfx_sub_channels_enabled_matches() {
        check(
            ssx3_ee::s07::f_002875d0__generated,
            ssx_audio_system_set_sfx_sub_channels_enabled,
            12,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x0028_75d0);
                let audio = DATA + 0x100;
                clear(c, audio, 0x6400);
                for k in 0..11 {
                    w(c, audio + 0x62bc + 0x10 * k, r.edge_f32());
                }
                c.g[A0][0] = sx(audio);
                c.g[A1][0] = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => r.next_u32() as u64,
                };
            },
        );
    }


    #[test]
    fn snd_load_queue_dtor_matches() {
        check(ssx3_ee::s07::f_0028a148__generated, snd_load_queue_dtor, 30, &|c, seed| {
            let _r = base_7(c, seed, 0x28a148, 3);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (seed & 1) as u64;
            for (i, off) in [0x1c0u32, 0x1b0, 0x1a0, 0x190].into_iter().enumerate() {
                let set = (seed >> i) & 1 != 0;
                w_2(c, this + off, if set { POOL_BLOCK } else { 0 });
            }
            w_2(c, GAME_GP.wrapping_add(0x520), 0x55);
        });
    }

    pub(crate) const BANKMAN: u32 = DATA + 0x15000;


    // ---------------------------------------------------------------- scripts

    #[test]
    fn ssx_audio_system_unload_world_banks_matches() {
        check(
            ee::s07::f_00286c00__generated,
            ssx_audio_system_unload_world_banks,
            4,
            &|c, seed| {
                let _ = base_8(c, seed, 0x286c00, 4);
                c.g[A0][0] = SYS as u64;
                w(c, SYS + 0x400, BANKMAN);
                w(c, SYS + 0x118, SYS + 0x400);
            },
        );
    }


    #[test]
    fn ssx_audio_system_on_reset_race_matches() {
        check(
            ee::s07::f_00287108__generated,
            ssx_audio_system_on_reset_race,
            8,
            &|c, seed| {
                let _ = base_8(c, seed, 0x287108, 4);
                c.g[A0][0] = SYS as u64;
                w(c, SYS + 0x5824, (seed * 3) as u32);
                w(c, WORLD + 0xc0, OBJ2);
                w(c, OBJ2 + 0x70, (seed % 4) as u32);
            },
        );
    }


    // ---------------------------------------------------------------- audio

    #[test]
    fn ssx_audio_system_unload_front_end_audio_matches() {
        check(
            ee::s07::f_00286200__generated,
            ssx_audio_system_unload_front_end_audio,
            SEEDS_6,
            &|c, seed| {
                let _ = base_9(c, seed, 0x2862_00, 8);
                let obj = SCRATCH_2;
                c.g[A0][0] = obj as u64;
                w(
                    c,
                    obj + 0x608c,
                    match seed % 3 {
                        0 => 2,
                        1 => 1,
                        _ => 0,
                    },
                );
                w(c, obj + 0x6090, (seed % 2) as u32);
                let player = obj + 0x2000;
                w(c, obj + 0x118, player);
                w(c, player, obj + 0x2100);
                w(c, obj + 0x2100 + 0xab0, obj + 0x2200);
                slot_4(c, obj + 0x2200, 0x10, 0);
                slot_4(c, obj + 0x2200, 0x14, 1);
            },
        );
    }


    #[test]
    fn sndloadqueue_ctor_matches() {
        strict(ee::s07::f_0028a058__generated, sndloadqueue_ctor, SEEDS, &|c, seed| {
            let _r = base_10(c, seed, 0xa058, 0);
            c.g[A0][0] = sx(DATA + 0x1000);
        });
    }


    #[test]
    fn ssx_audio_system_set_fade_gain_matches() {
        same(
            ssx3_ee::s07::f_00287a10__generated,
            ssx_audio_system_set_fade_gain,
            200,
            &|c, seed| {
                let mut r = Rng(0x287a10 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let s = DATA + 0x100;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = (seed % 0x400) as u64;
                c.f[12] = if seed % 3 == 0 { 0 } else { r.f32(2.0) };
                c.f[13] = r.f32(1.0);
                for off in [0x63c4u32, 0x63c8, 0x63cc, 0x63d0] {
                    c.bus.write32(s + off, r.edge_f32());
                }
                for off in [
                    -0x4620i32, 0x43c, 0x440, 0x444, 0x448, 0x44c, 0x450, 0x454, 0x458, 0x45c,
                    0x460,
                ] {
                    c.bus
                        .write32(GAME_GP.wrapping_add(off as u32), r.edge_f32());
                }
            },
        );
    }
}
