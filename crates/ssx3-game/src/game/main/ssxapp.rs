//! `src/main/ssxapp.cpp`: hand-written replacements (moved from the
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
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beintscore::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::gp_13 as gp_3;
pub(crate) use crate::game::fe::fepopup3::p_2;
pub(crate) use crate::game::fe::fepopup3::sw;
pub(crate) use crate::game::fe::festateload::glo;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festaterewards::at_4 as at;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::ovstates::at as at_2;
pub(crate) use crate::game::fe::ovstates::close;
pub(crate) use crate::game::fe::ovstates::open;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::movie::movieplayer::p;
pub(crate) use crate::game::render::particle::addi;
pub(crate) use crate::game::render::particle::gp;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0022_7e98, ssx_app_next_tick, "SSXApp_NextTick"),
    (0x0022_7f80, mc_manager_get, "MCManager_Get"),
    (
        0x0022_81d8,
        ssx_app_start_game_module,
        "SSXApp_StartGameModule",
    ),
    (0x0022_9278, crowd_man2d_dtor, "CrowdMan2d_dtor"),
    (0x0022_92e0, crowd_man2d_reset, "CrowdMan2d_Reset"),
    (0x0022_9398, crowd_man2d_remove_all, "CrowdMan2d_RemoveAll"),
    (
        0x0022_9408,
        crowd_man2d_remove_by_type,
        "CrowdMan2d_RemoveByType",
    ),
    (0x0022_9498, crowd_man2d_clear, "CrowdMan2d_Clear"),
    (0x0022_94c8, crowd_man2d_update, "CrowdMan2d_Update"),
    (
        0x0022_9530,
        crowd_man2d_spawn_sprites,
        "CrowdMan2d_SpawnSprites",
    ),
    (0x0022_9738, crowd_man_2d_add_sprite, "CrowdMan2d_AddSprite"),
    (0x0022_9788, crowd_man2d_add, "CrowdMan2d_Add"),
    (0x0022_9b90, crowd_man2d_clear_slot, "CrowdMan2d_ClearSlot"),
    (0x0022_9e58, crowdman2d_load_state, "CrowdMan2d_LoadState"),
    (0x0022_9f80, nash_empty_hook, "Nash_EmptyHook"),
    (0x0022_9f88, vu0_upload_occluder, "Vu0_UploadOccluder"),
    (0x0022_9fc8, nash_classify_instance, "Nash_ClassifyInstance"),
    (0x0022_a128, nash_classify_patch, "Nash_ClassifyPatch"),
    (0x0022_a270, nash_classify_effect, "Nash_ClassifyEffect"),
    (
        0x002b_4b48,
        world_trigger_manager_destroy_instance,
        "WorldTriggerManager_DestroyInstance",
    ),
];

/// Direct and cross-shard callees, by guest address.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::rider::callee::COMM_SYSTEM_UPDATE;
    pub(crate) use crate::game::ai::rider::callee::EMITTER_ADVANCE_DESC_TIME;
    pub(crate) use crate::game::ai::rider::callee::EMITTER_RENDER;
    pub(crate) use crate::game::ai::rider::callee::EMITTER_UPDATE_DESCRIPTOR;
    pub(crate) use crate::game::ai::rider::callee::INPUT_PAD_UPDATE;
    pub(crate) use crate::game::ai::rider::callee::MATH_SIN_COS_2 as MATH_SIN_COS;
    pub(crate) use crate::game::ai::rider::callee::MODIFIER_BLOCK_GET_TRANSFORM_BOUNDS;
    pub(crate) use crate::game::ai::rider::callee::PAD_QUEUE_ADVANCE_WRITE;
    pub(crate) use crate::game::ai::rider::callee::PAD_QUEUE_GET_PAD_DATA;
    pub(crate) use crate::game::ai::rider::callee::PAD_QUEUE_GET_PAD_STATUS;
    pub(crate) use crate::game::ai::rider::callee::USB_KEYBOARD_UPDATE;
    pub(crate) use crate::game::ai::rider::callee::VU0_RESTORE_CONTEXT;
    pub(crate) use crate::game::ai::rider::callee::VU0_SAVE_CONTEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN2D_REMOVE_ALL;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN2D_RESET;
    pub(crate) use crate::game::bx::bxstring2::callee::C_FE_STATE_LOBBY_MENU_FILL_PLAYER_LIST;
    pub(crate) use crate::game::bx::bxstring2::callee::C_FE_STATE_LOBBY_MENU_UPDATE_PLAYER_SCROLL_ARROWS;
    pub(crate) use crate::game::bx::bxstring2::callee::C_LUNO_TABLE_ALLOC;
    pub(crate) use crate::game::bx::bxstring2::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::bx::bxstring2::callee::EVENT_SELECT_ON_ACTIVATE as EVENT_SELECT_ON_ACTIVATE_2;
    pub(crate) use crate::game::bx::bxstring2::callee::EVENT_SELECT_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::FE_MODULE_RIDER_PREVIEW_REQUEST_CHAR;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_PARSE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_STOP_AUX_LISTS;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_GET_ENTRY_TYPE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_PREPARE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_SET_PAUSED_NESTED;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_INIT;
    pub(crate) use crate::game::bx::bxstring2::callee::SNAPSHOT_LIST_REMOVE;
    pub(crate) use crate::game::bx::bxstring2::callee::VID_ENGINE_PLAYER_START_PLAYBACK;
    pub(crate) use crate::game::bx::bxstring2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_SET_OUTPUT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_INTERFACE_CALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_GET_NEXT_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAM_CONTROLLER_SET_ALGORITHM;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBXSTRING_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_BUY_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_EQUIP_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_MAN2D_INIT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_ON_ACTIVATE_2 as FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_STATS_REFRESH_RESULTS_2 as FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_UPDATE_BUTTONS_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FLOAT_TO_UINT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_COMM_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_PRE_RACE_MENU_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_EVENT_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_RAMP_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_SPAN_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_SHOW;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMORY_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_MESSAGE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_MESSENGER_SET_PLAYER_MENU_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_DECODE_FRAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_FRAME_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2 as REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_REFRESH_LIST;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_NOW_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_DATA_GET_INFO_2 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2 as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MOVIE_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_SET_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_0;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_1;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_CREATE_STEP1_ON_EDIT_DONE_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_CREATE_STEP1_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_LOBBY_MENU_ON_CREATE_ROOM_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_LOBBY_MENU_PROMPT_ROOM_PASSWORD;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_NET_QUICK_MESSAGES_ON_RESET_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_NET_QUICK_MESSAGES_ON_SAVE_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_ONLINE_MAIN_MENU_ON_DISCONNECT_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_ONLINE_MAIN_MENU_ON_LEAVE_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_REWARD_GALLERY_RELEASE_TEXTURES;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_REWARD_GALLERY_V29;
    pub(crate) use crate::game::fe::fepopup3::callee::FE_ASYNC_MANAGER_GET_STATE;
    pub(crate) use crate::game::fe::fepopup3::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::fe::fepopup3::callee::FE_MODULE_RIDER_PREVIEW_CTOR;
    pub(crate) use crate::game::fe::fepopup3::callee::GAME_MODULE_CTOR;
    pub(crate) use crate::game::fe::fepopup3::callee::HUD_LAYOUT_ICON_TEXT;
    pub(crate) use crate::game::fe::fepopup3::callee::LOBBY_MAN_GET_ROOM_NAME;
    pub(crate) use crate::game::fe::fepopup3::callee::LOBBY_MAN_LOOKUP_PEER_ADDR;
    pub(crate) use crate::game::fe::fepopup3::callee::LOBBY_MAN_MOVE_TO_ROOM;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_END_PROGRESS;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_REQUEST_CARD_INFO_2 as MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::fe::fepopup3::callee::MESSAGE_CENTER_GET_INBOX_3 as MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::fe::fepopup3::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::fepopup3::callee::NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::fepopup3::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::NET_FE_STATE_SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_SAVE_DATA_SIZE;
    pub(crate) use crate::game::fe::festateload::callee::BOLT_FILE_CACHE_INIT;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SELECT_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::EA_MEM_FREE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_UPDATE;
    pub(crate) use crate::game::fe::festateload::callee::I_SIGNAL_SEMA;
    pub(crate) use crate::game::fe::festateload::callee::MEM_CARD_BASE_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V42;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V43;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_HANDLE_EVENT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_METRIX_SET_SPIN_AMOUNT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_PREVIEW_APPLY_TRANSFORM;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_RESUME;
    pub(crate) use crate::game::fe::festateload::callee::STRLEN_COPY;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::UI_STATE_V13;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_TRIGGER_MANAGER_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_CACHE_SET_DATA;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_MAN_RESET_COMMAND_QUEUE;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_GOTO_TASK;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_JUMP_TO_TASK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE as FE_LOCALE_UNLOAD_TABLE_2;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::CBXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::C_WORLD_SOUND_UNLINK_EMITTERS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EA_MEM_COPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EVENT_SELECT_HANDLE_INPUT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::GRAPHICS_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::LOBBY_MAN_REQUEST_STATS_PAGE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MC_MANAGER_GET_INSTANCE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MENU_SYS_GET_INPUT_CODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_SET_TRACK_CONTROL_VALUE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RIDER_METRIX_BANK_COMBO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_DUMMY_OBJ_READ_FIELDS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SSX_APP_BOOT_LOAD;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_UPDATE_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STREAM_BUFFER_GET_STATUS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STRLEN_AND_STRCPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::TEX_MAN_SET_ALLOC_FLAGS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_NODE_UNLINK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_STATE_MGR_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::USTR_FROM_ASCII;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_RESET_SECTIONS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_INFO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_NAME;
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

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::AI_SET_AI_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::APP_MAN_SET_NEXT_MODULE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BENEW_PLAYER_INT_GET_CHAR_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_INTERFACE_FV;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_TEXT_FROM_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUDDY_PRODUCT_COMPARE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUY_POPUP_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::CODEC_DECODE_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_BIG_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_SMALL_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::festaterewards::table_callee::MUSIC_SLOT_RELEASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CHEAT_CHARS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CONCEPT_ARTS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_GET_BASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_POSTERS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TOYS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TRADING_CARDS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_VIDEOS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_REMOVE_SCRIPT_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SCRIPT_CLEANUP;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SET_SCRIPT_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SSX_AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRCPY;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRLEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_GET_STRING4;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_GET_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OBJECT_PUSH;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OPEN_FINISH_SCREEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_ADD_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_STATE_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_RICH_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UPLOAD_TEXTURE_PIXELS;
}

/// `uint CrowdMan2d_AddSprite(CrowdMan2d *man, const void *sprite, uint value)`
/// (0x229738): appends `sprite` (a 16-byte quad) and `value` to the pool at
/// `man + (man->count << 5) + 0x2220`, wraps the count modulo 40 and returns
/// the new count.
pub fn crowd_man_2d_add_sprite(c: &mut Cpu) {
    let man = Addr(abi::arg(c, 0));
    let sprite = abi::arg(c, 1);
    let value = c.g[A2][0];
    c.cycles += 16;
    c.g[V1][0] = man.off(0x2214).lw(c);
    c.lq(A3, sprite);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
    c.g[A1][0] = 0x28;
    c.g[V1][0] = sx(abi::r32(c, A0).wrapping_add(c.g[V1][0] as u32));
    c.sq(A3, abi::r32(c, V1).wrapping_add(0x2220));
    c.g[V0][0] = man.off(0x2214).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
    c.g[V0][0] = sx(abi::r32(c, A0).wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[V0][0] as u32).off(0x2230).sw(c, value);
    let count = man.off(0x2214).lw(c);
    c.g[V1][0] = sx((count as u32).wrapping_add(1));
    abi::div(c, abi::r32(c, V1), abi::r32(c, A1));
    man.off(0x2214).sw(c, c.g[V1][0]);
    c.cycles += 3;
    c.g[V0][0] = c.hi[0];
    man.off(0x2214).sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Crowd / section manager.

/// `void CrowdMan2d_ClearSlot(CrowdMan2d *self, int slot)` (0x229b90): store
/// all-ones in the 32-bit slot at `self + slot*4 + 8`; leaves that address in
/// `a1` and all-ones in `v0`.
pub fn crowd_man2d_clear_slot(c: &mut Cpu) {
    const SLOT0: u32 = 0x8;
    c.cycles += 6;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] |= 0xffff; // -> all ones
    Addr((c.g[A1][0] as u32).wrapping_add(SLOT0)).sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// The application tick.

/// `int SSXApp_NextTick(App* self)` (0x227e98): advance the pad queue and
/// hand both pads to the input system. Returns 0 when there is no app or the
/// read side of the queue is still ahead of the game, else 1.
pub fn ssx_app_next_tick(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.lw(gp(c, -0x850));
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        let no_app = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        let mut ticked = false;
        if !no_app {
            c.cycles += 4;
            let queue = c.g[V0][0];
            c.g[A0][0] = queue;
            callee::PAD_QUEUE_ADVANCE_WRITE.call(c, 0x0022_7ec0)?;
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = c.lw(gp(c, -0x238));
            if !idle {
                // The comm system and the USB keyboard, when present.
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.g[A0][0] = c.lw(gp(c, 0x300));
                } else {
                    c.cycles += 3;
                    callee::COMM_SYSTEM_UPDATE.call(c, 0x0022_7ed8)?;
                    c.g[A0][0] = c.lw(gp(c, 0x300));
                }
                c.cycles += 2;
                let no_keyboard = c.g[A0][0] == 0;
                c.g[S2][0] = sx(r32(c, S0).wrapping_add(0xa8));
                if !no_keyboard {
                    c.cycles += 2;
                    callee::USB_KEYBOARD_UPDATE.call(c, 0x0022_7eec)?;
                }
                // Both pads.
                c.cycles += 3;
                c.g[S1][0] = 0;
                c.g[A0][0] = c.lw(gp(c, -0x850));
                loop {
                    c.cycles += 15;
                    c.g[A1][0] = c.g[S1][0];
                    callee::PAD_QUEUE_GET_PAD_STATUS.call(c, 0x0022_7f00)?;
                    c.g[A0][0] = c.lw(gp(c, -0x850));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[S0][0] = c.g[V0][0];
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x1));
                    callee::PAD_QUEUE_GET_PAD_DATA.call(c, 0x0022_7f14)?;
                    c.g[A0][0] = c.lw(r32(c, S2));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    addi(c, S2, 0x4);
                    callee::INPUT_PAD_UPDATE.call(c, 0x0022_7f28)?;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = c.lw(gp(c, -0x850));
                    if !more {
                        c.cycles += 2;
                        c.g[V0][0] = 0x1;
                        ticked = true;
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_7ef8);
                    }
                }
            }
        }
        if !ticked {
            c.cycles += 1;
            c.g[V0][0] = 0;
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

// ---------------------------------------------------------------------------
// VU0.

/// `void Vu0_UploadOccluder(int index, const Vec4* v, const Mat4* m)`
/// (0x229f88): copy the vertex and the four matrix rows into VU0 data memory
/// at quadword `index * 5 + 0x50` (the occluder plane slot of a view).
pub fn vu0_upload_occluder(c: &mut Cpu) {
    c.cycles += 16;
    c.g[V0][0] = sx(abi::arg(c, 0) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(abi::arg(c, 0)));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x50));
    c.ctc2(2, 1);
    let v = abi::arg(c, 1);
    let m = abi::arg(c, 2);
    c.lqc2(1, v);
    c.lqc2(2, m);
    c.lqc2(3, p(m, 0x10));
    c.lqc2(4, p(m, 0x20));
    c.lqc2(5, p(m, 0x30));
    c.vu0().sqi(0b1111, 1, 1);
    c.vu0().sqi(0b1111, 2, 1);
    c.vu0().sqi(0b1111, 3, 1);
    c.vu0().sqi(0b1111, 4, 1);
    c.vu0().sqi(0b1111, 5, 1);
    c.pc = c.g[RA][0] as u32;
}

/// `void *MCManager_Get(this)` (0x00227f80): the `cMCOverlayManager`
/// singleton (`MCManager_GetInstance`).
pub fn mc_manager_get(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |_| {},
        callee::MC_MANAGER_GET_INSTANCE,
        0x0022_7f90,
    )
}

// ---------------------------------------------------------------------------
// World triggers.

/// `void WorldTriggerManager_DestroyInstance(void)` (0x2b4b48): destroy the
/// trigger manager singleton (with `a1 = 3`, the delete flavour) and clear
/// the pointer.
pub fn world_trigger_manager_destroy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(glo(c, g::WORLD_TRIGGER_MANAGER)).lw(c);
        let mgr = c.g[A0][0];
        frame.save_ra(c, 0);
        if mgr != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            callee::WORLD_TRIGGER_MANAGER_DTOR.call(c, 0x002b_4b60)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        Addr(glo(c, g::WORLD_TRIGGER_MANAGER)).sw(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `CrowdMan2d_Clear(this)` 0x229498: remove every slot and reset the manager.
pub fn crowd_man2d_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        callee::CROWD_MAN2D_REMOVE_ALL.call(c, 0x0022_94ac)?;
        c.g[A0][0] = c.g[S0][0];
        callee::CROWD_MAN2D_RESET.call(c, 0x0022_94b4)?;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Crowd manager.

/// `void CrowdMan2d_Add(CrowdMan2d* m, int mode)` (0x229788): find the first
/// slot of the 64 entries at +8 whose word is -1 and initialise it with
/// `mode` (and the caller's second argument). 8 + 3 (+3 (+1), +3) per slot +
/// 2 cycles plus the callee. The event check resumes at 0x2297a8.
pub fn crowd_man2d_add(c: &mut Cpu) {
    guest(c, |c| {
        let resume = c.pc == 0x0022_97a8;
        if !resume {
            c.cycles += 8;
            let frame = Frame::open(c, 0x10);
            c.g[A3][0] = 0xffff_ffff_ffff_ffff;
            c.g[A2][0] = c.g[A1][0];
            frame.save_ra(c, 0);
            c.g[A1][0] = 0;
            c.g[V1][0] = sx(r32(c, A0).wrapping_add(8));
        } else {
            c.cycles += 1;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        }
        let frame = Frame { size: 0x10 };
        loop {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 64i64) as u64;
            let exhausted = c.g[V0][0] == 0;
            frame.restore_ra(c, 0);
            if exhausted {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, V1));
            let taken = c.g[V0][0] != c.g[A3][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
            if taken {
                if back_edge_unwinds(c, 0x0022_97a8) {
                    return Ok(());
                }
                c.cycles += 1;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                continue;
            }
            c.cycles += 3;
            callee::CROWD_MAN2D_INIT_SLOT.call(c, 0x0022_97cc)?;
            frame.restore_ra(c, 0);
            break;
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void SSXApp_StartGameModule(SSXApp* self)` (0x2281d8): allocate and
/// construct the game module, hand it to the app manager, and record the game
/// mode manager.
pub fn ssx_app_start_game_module(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x0048_0000;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x57f8));
        c.g[A0][0] = 0x22c;
        call_ptr(c, 0x0031_7d70, 0x0022_8200)?;
        c.g[A0][0] = c.g[V0][0];
        callee::GAME_MODULE_CTOR.call(c, 0x0022_8208)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        sw(c, p_2(c.g[S0][0], 0x84), c.g[V0][0]);
        call_ptr(c, 0x0031_6ef8, 0x0022_8218)?;
        call_ptr(c, 0x0023_7cf8, 0x0022_8220)?;
        sw(c, p_2(c.g[S0][0], 0xc0), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CrowdMan2d_dtor(CrowdMan2d* self, int flags)` (0x229278): empty the crowd,
/// free the render object, then delete the object when the low flag bit is
/// set.
pub fn crowd_man2d_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0022_9398, 0x0022_9294)?;
        c.g[A0][0] = lw(c, p(this as u32, 4));
        sw(c, gp_2(c, -0x6f0), 0);
        call_ptr(c, 0x0031_7e50, 0x0022_92a0)?;
        c.g[A0][0] = lw(c, this as u32);
        let no_render = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !no_render {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x002d_b700, 0x0022_92b4)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = this;
            call_ptr(c, 0x0031_7e50, 0x0022_92c8)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Nash_ClassifyPatch`'s table (`0x480000 - 0x4ca0`), indexed by the heap
/// handle (only handles < 5 are looked up).
pub(crate) const NASH_PATCH_TABLE: u32 = 0x0047_b360;

/// `Nash_ClassifyInstance`'s table (`0x480000 - 0x4cc0`).
pub(crate) const NASH_INSTANCE_TABLE: u32 = 0x0047_b340;

/// `void Nash_ClassifyPatch(Cell *cell, Node *node, int frustum)` (0x22a128):
/// classify one octree node under the cell's heap. A null node takes its
/// handle from VU0 `VI1`; otherwise the node's signed word at `+0x156` is
/// checked for a handle and its byte at `+0x155` indexes the cell's heap
/// (`heap + byte*8 + 0x24`, the slot word must be 6), and the handle selects
/// `heap + handle*0x18 + 0x3f0` whose word must be 3. A hit runs the VU0
/// bounding-box/frustum classification, which leaves the handle in `VI1`
/// (`cfc2`); a handle of five or more stores the node at the cell's list
/// head (`+0x78b0`), a smaller one picks a handler from the cell's table
/// (which appends the old head at `+0x50ac`/`+0x70ac` and then stores too).
///
/// The function makes no calls and checks no events, so the only entry the
/// trampoline can ever use is the top (the generated `__entries` twin's table
/// targets are only reachable from inside).
pub fn nash_classify_patch(c: &mut Cpu) {
    c.cycles += 3;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
    let cell = r32(c, A0);
    c.g[A3][0] = c.g[A0][0];
    let node = r32(c, A1);
    let heap;
    if node == 0 {
        c.cycles += 2;
        c.cfc2(4, 1);
        heap = r32(c, A0);
        c.g[V0][0] = (c.g[A0][0] < 0x5) as u64;
    } else {
        // 0x22a134: the node's handle and slot.
        c.cycles += 5;
        c.g[V0][0] = Addr(cell).off(0x90).lw(c);
        c.g[T0][0] = Addr(node).off(0x156).lh(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        heap = r32(c, A0);
        c.g[V0][0] = 0;
        if (c.g[T0][0] as i64) >= 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr(node).off(0x155).lbu(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.g[V0][0] = sx(heap.wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
            c.g[V1][0] ^= 0x6;
            if c.g[V1][0] == 0 {
                c.cycles += 6;
                c.g[V0][0] = 0x18;
                let i = abi::mult(c, r32(c, T0), 0x18);
                c.g[V1][0] = sx(i);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(heap));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x3f0).lw(c);
                c.g[V1][0] ^= 0x3;
                c.g[V0][0] = (c.g[V1][0] < 0x1) as u64;
            } else {
                c.g[V0][0] = 0;
            }
        }
        // 0x22a17c: a node that did not classify jumps straight to the
        // epilogue, skipping the insert block and its two cycles.
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
            abi::ret(c);
            return;
        }
        // 0x22a184: build the node's box on the stack, classify it with the
        // cell's VU0 routine and keep the handle test for the tail.
        c.cycles += 26;
        for i in 0..6u32 {
            c.lwc1(i as usize, node.wrapping_add(0x158 + 4 * i));
        }
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 6);
        c.swc1(0, sp(c));
        c.swc1(1, sp(c).wrapping_add(0x4));
        c.swc1(2, sp(c).wrapping_add(0x8));
        c.swc1(3, sp(c).wrapping_add(0x10));
        c.swc1(4, sp(c).wrapping_add(0x14));
        c.swc1(5, sp(c).wrapping_add(0x18));
        c.swc1(6, sp(c).wrapping_add(0x1c));
        c.swc1(6, sp(c).wrapping_add(0xc));
        c.cfc2(4, 1);
        c.g[V1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[V0][0] = sx(node.wrapping_add(0x140));
        c.lqc2(20, sp(c));
        c.lqc2(21, r32(c, V1));
        c.lqc2(22, r32(c, V0));
        c.ctc2(6, 14);
        c.vcallms(0xdb8);
        c.g[V0][0] = (c.g[A0][0] < 0x5) as u64;
    }
    // 0x22a1f4/0x22a260: `cfc2` left the VU0 handle in a0. Five or more
    // stores the node at the cell's list head; a smaller handle indexes the
    // cell's table, whose handlers also end in that store.
    c.cycles += 2;
    let high = c.g[V0][0] == 0;
    c.g[V0][0] = 0x0048_0000;
    let mut insert = true;
    if !high {
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx(NASH_PATCH_TABLE);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        match c.g[A0][0] as u32 {
            0x0022_a214 => {
                c.cycles += 3;
                c.g[A0][0] = Addr(cell).off(0x78b0).lw(c);
                if c.g[A0][0] == 0 {
                    Addr(cell).off(0x78b0).sw(c, c.g[A1][0]);
                    insert = false;
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(cell).off(0x50a8).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx(cell.wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    Addr(c.g[V0][0] as u32).off(0x50ac).sw(c, c.g[A0][0]);
                    Addr(cell).off(0x50a8).sw(c, c.g[V1][0]);
                }
            }
            0x0022_a260 => {}
            0x0022_a23c => {
                c.cycles += 3;
                c.g[A0][0] = Addr(cell).off(0x78b0).lw(c);
                if c.g[A0][0] == 0 {
                    Addr(cell).off(0x78b0).sw(c, c.g[A1][0]);
                    insert = false;
                } else {
                    c.cycles += 6;
                    c.g[V1][0] = Addr(cell).off(0x70ac).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx(cell.wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    Addr(c.g[V0][0] as u32).off(0x70b0).sw(c, c.g[A0][0]);
                    Addr(cell).off(0x70ac).sw(c, c.g[V1][0]);
                }
            }
            target => {
                c.jump_out(target, 0x0022_a128, 0x0022_a26c);
                return;
            }
        }
    }
    if insert {
        c.cycles += 1;
        Addr(cell).off(0x78b0).sw(c, c.g[A1][0]);
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
    abi::ret(c);
}

/// `void Nash_ClassifyInstance(Cell *cell, Node *node, int frustum)`
/// (0x229fc8): the same classification for the other node flavour. The node
/// is skipped unless its `+0x8` word ends in 3; its byte at `+0x7d` (0xff
/// means "no slot") indexes `heap + byte*8 + 0x24` (slot word must be 6) and
/// its signed halfword at `+0x7e` selects `heap + handle*0x18 + 0x3f0` (word
/// must be 3). A hit classifies the box at `+0x60`; the `VI1` handle that
/// leaves the VU0 routine then either stores the node at `+0x50a4` (five or
/// more) or dispatches through the same table (appending at `+0x20a0` or
/// `+0x40a0` before that store).
///
/// Like `Nash_ClassifyPatch` it makes no calls and checks no events, so the
/// top is its only entry.
pub fn nash_classify_instance(c: &mut Cpu) {
    c.cycles += 3;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
    let cell = r32(c, A0);
    c.g[A3][0] = c.g[A0][0];
    let node = r32(c, A1);
    let mut heap = 0u32;
    if node == 0 {
        c.cycles += 2;
        c.cfc2(4, 1);
        c.g[V0][0] = (c.g[A0][0] < 0x5) as u64;
    } else {
        // 0x229fd4: only nodes whose type word ends in 3 are classified.
        c.cycles += 5;
        c.g[V0][0] = Addr(node).off(0x8).lw(c);
        c.g[V1][0] = 0x3;
        c.g[V0][0] &= 0x3;
        if c.g[V0][0] != c.g[V1][0] {
            c.g[V0][0] = 0;
        } else {
            // 0x229fe8: the node's slot byte indexes the cell's heap.
            c.cycles += 6;
            c.g[V0][0] = Addr(cell).off(0x90).lw(c);
            c.g[A0][0] = 0xff;
            c.g[V1][0] = Addr(node).off(0x7d).lbu(c);
            c.g[T0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V0][0] = 0x1;
            heap = r32(c, T0);
            if c.g[V1][0] != c.g[A0][0] {
                c.cycles += 7;
                c.g[V0][0] = Addr(node).off(0x7d).lbu(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
                c.g[V1][0] ^= 0x6;
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(node).off(0x7e).lh(c);
                    c.g[A0][0] = 0x18;
                    let i = abi::mult(c, r32(c, V0), 0x18);
                    c.g[V1][0] = sx(i);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(heap));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x3f0).lw(c);
                    c.g[V1][0] ^= 0x3;
                    c.g[V0][0] = (c.g[V1][0] < 0x1) as u64;
                }
            }
        }
        // 0x22a038: a classified node is tested again with the VU0 routine;
        // a miss jumps to the epilogue, skipping the insert block's cycles.
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
            abi::ret(c);
            return;
        }
        c.cycles += 26;
        for i in 0..6u32 {
            c.lwc1(i as usize, node.wrapping_add(0x60 + 4 * i));
        }
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 6);
        c.swc1(0, sp(c));
        c.swc1(1, sp(c).wrapping_add(0x4));
        c.swc1(2, sp(c).wrapping_add(0x8));
        c.swc1(3, sp(c).wrapping_add(0x10));
        c.swc1(4, sp(c).wrapping_add(0x14));
        c.swc1(5, sp(c).wrapping_add(0x18));
        c.swc1(6, sp(c).wrapping_add(0x1c));
        c.swc1(6, sp(c).wrapping_add(0xc));
        c.cfc2(4, 1);
        c.g[V1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[V0][0] = sx(node.wrapping_add(0x50));
        c.lqc2(20, sp(c));
        c.lqc2(21, r32(c, V1));
        c.lqc2(22, r32(c, V0));
        c.ctc2(6, 14);
        c.vcallms(0xdb8);
        c.g[V0][0] = (c.g[A0][0] < 0x5) as u64;
    }
    // 0x22a0b0/0x22a11c: `cfc2` left the VU0 handle in a0. Five or more
    // stores the node at the cell's list head; a smaller handle indexes the
    // cell's table, whose handlers also end in that store.
    c.cycles += 2;
    let high = c.g[V0][0] == 0;
    c.g[V0][0] = 0x0048_0000;
    let mut insert = true;
    if !high {
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx(NASH_INSTANCE_TABLE);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        match c.g[A0][0] as u32 {
            0x0022_a0d0 => {
                c.cycles += 3;
                c.g[A0][0] = Addr(cell).off(0x50a4).lw(c);
                if c.g[A0][0] == 0 {
                    Addr(cell).off(0x50a4).sw(c, c.g[A1][0]);
                    insert = false;
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(cell).off(0x209c).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx(cell.wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    Addr(c.g[V0][0] as u32).off(0x20a0).sw(c, c.g[A0][0]);
                    Addr(cell).off(0x209c).sw(c, c.g[V1][0]);
                }
            }
            0x0022_a11c => {}
            0x0022_a0f8 => {
                c.cycles += 3;
                c.g[A0][0] = Addr(cell).off(0x50a4).lw(c);
                if c.g[A0][0] == 0 {
                    Addr(cell).off(0x50a4).sw(c, c.g[A1][0]);
                    insert = false;
                } else {
                    c.cycles += 6;
                    c.g[V1][0] = Addr(cell).off(0x40a0).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx(cell.wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    Addr(c.g[V0][0] as u32).off(0x40a4).sw(c, c.g[A0][0]);
                    Addr(cell).off(0x40a0).sw(c, c.g[V1][0]);
                }
            }
            target => {
                c.jump_out(target, 0x0022_9fc8, 0x0022_a128);
                return;
            }
        }
    }
    if insert {
        c.cycles += 1;
        Addr(cell).off(0x50a4).sw(c, c.g[A1][0]);
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
    abi::ret(c);
}

/// `void Nash_EmptyHook(void)` (0x229f80): the empty Nash (effect) callback.
pub fn nash_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Nash_ClassifyEffect: a jump-table dispatcher (VU0 setup, then three
// per-effect registration lists).
//
// The port follows the generated `__entries` twin block for block, so a
// thread switch that resumes it at any of the table's case labels behaves
// like the generated function.

/// The two registration lists `Nash_ClassifyEffect` appends to (word at
/// `self + 0x7ab8` is the head; a count at `+0x78b4`/`+0x7abc` indexes the
/// inline arrays at `+0x78b8`/`+0x7ac0`). The effect type also selects one of
/// three lists in the table at `.rodata 0x47b380`.
pub fn nash_classify_effect(c: &mut Cpu) {
    let mut pc = c.pc as u32;
    loop {
        match pc {
            0x0022_a270 => {
                c.cycles += 3;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                let null_node = c.g[A1][0] == 0;
                c.g[A3][0] = c.g[A0][0];
                pc = if null_node { 0x0022_a2e4 } else { 0x0022_a27c };
            }
            0x0022_a27c => {
                c.cycles += 26;
                let node = c.g[A1][0] as u32;
                let sp = c.g[SP][0] as u32;
                c.lwc1(0, node.wrapping_add(0x68));
                c.lwc1(1, node.wrapping_add(0x6c));
                c.lwc1(2, node.wrapping_add(0x70));
                c.lwc1(3, node.wrapping_add(0x74));
                c.lwc1(4, node.wrapping_add(0x78));
                c.lwc1(5, node.wrapping_add(0x7c));
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 6);
                c.swc1(0, sp);
                c.swc1(1, sp.wrapping_add(0x4));
                c.swc1(2, sp.wrapping_add(0x8));
                c.swc1(3, sp.wrapping_add(0x10));
                c.swc1(4, sp.wrapping_add(0x14));
                c.swc1(5, sp.wrapping_add(0x18));
                c.swc1(6, sp.wrapping_add(0x1c));
                c.swc1(6, sp.wrapping_add(0xc));
                c.cfc2(A0, 1);
                c.g[V1][0] = sx(sp.wrapping_add(0x10));
                c.g[V0][0] = sx(node.wrapping_add(0x50));
                c.lqc2(20, sp);
                c.lqc2(21, c.g[V1][0] as u32);
                c.lqc2(22, c.g[V0][0] as u32);
                c.ctc2(6, 14);
                c.vcallms(0xdb8);
                c.g[V0][0] = (c.g[A0][0] < 0x5u64) as u64;
                pc = 0x0022_a2ec;
            }
            0x0022_a2e4 => {
                c.cycles += 2;
                c.cfc2(A0, 1);
                c.g[V0][0] = (c.g[A0][0] < 0x5u64) as u64;
                pc = 0x0022_a2ec;
            }
            0x0022_a2ec => {
                c.cycles += 2;
                let out_of_range = c.g[V0][0] == 0;
                c.g[V0][0] = 0x48_0000;
                pc = if out_of_range {
                    0x0022_a358
                } else {
                    0x0022_a2f4
                };
            }
            0x0022_a2f4 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4c80));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let target = c.g[A0][0] as u32;
                pc = match target {
                    0x0022_a30c => 0x0022_a30c,
                    0x0022_a334 => 0x0022_a334,
                    0x0022_a358 => 0x0022_a358,
                    _ => {
                        c.jump_out(target, 0x0022_a270, 0x0022_a364);
                        return;
                    }
                };
            }
            0x0022_a30c => {
                c.cycles += 3;
                let this = c.g[A3][0] as u32;
                c.g[A0][0] = Addr(this.wrapping_add(0x7ab8)).lw(c);
                if c.g[A0][0] == 0 {
                    let v = c.g[A1][0];
                    Addr(this.wrapping_add(0x7ab8)).sw(c, v);
                    pc = 0x0022_a35c;
                } else {
                    pc = 0x0022_a318;
                }
            }
            0x0022_a318 => {
                c.cycles += 7;
                let head = c.g[A3][0] as u32;
                c.g[V1][0] = Addr(head.wrapping_add(0x78b4)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(head));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                let slot = c.g[V0][0] as u32;
                let prev = c.g[A0][0];
                Addr(slot.wrapping_add(0x78b8)).sw(c, prev);
                let count = c.g[V1][0];
                Addr(head.wrapping_add(0x78b4)).sw(c, count);
                pc = 0x0022_a358;
            }
            0x0022_a334 => {
                c.cycles += 3;
                let this = c.g[A3][0] as u32;
                c.g[A0][0] = Addr(this.wrapping_add(0x7ab8)).lw(c);
                if c.g[A0][0] == 0 {
                    let v = c.g[A1][0];
                    Addr(this.wrapping_add(0x7ab8)).sw(c, v);
                    pc = 0x0022_a35c;
                } else {
                    pc = 0x0022_a340;
                }
            }
            0x0022_a340 => {
                c.cycles += 6;
                let head = c.g[A3][0] as u32;
                c.g[V1][0] = Addr(head.wrapping_add(0x7abc)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(head));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                let slot = c.g[V0][0] as u32;
                let prev = c.g[A0][0];
                Addr(slot.wrapping_add(0x7ac0)).sw(c, prev);
                let count = c.g[V1][0];
                Addr(head.wrapping_add(0x7abc)).sw(c, count);
                pc = 0x0022_a358;
            }
            0x0022_a358 => {
                c.cycles += 1;
                let this = c.g[A3][0] as u32;
                let node = c.g[A1][0];
                Addr(this.wrapping_add(0x7ab8)).sw(c, node);
                pc = 0x0022_a35c;
            }
            0x0022_a35c => {
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x0022_a270, 0x0022_a364);
                return;
            }
        }
    }
}

/// `void CrowdMan2d_RemoveAll(CrowdMan2d* m)` (0x229398): clear all 128 crowd
/// slots that are not already empty.
pub fn crowd_man2d_remove_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        sqsp(c, 18, 0x20);
        sqsp(c, 19, 0x10);
        c.g[S2][0] = 0xffff_ffff_ffff_ffff;
        sqsp(c, 16, 0x40);
        c.g[S3][0] = c.g[A0][0];
        sqsp(c, 17, 0x30);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(8));
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            let empty = c.g[V0][0] == c.g[S2][0];
            c.g[A0][0] = c.g[S3][0];
            if !empty {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                call::call_ptr(c, 0x0022_9b90, 0x0022_93dc)?;
            }
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < 128) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_93c8);
            }
        }
        c.cycles += 7;
        lqsp(c, 16, 0x40);
        lqsp(c, 17, 0x30);
        lqsp(c, 18, 0x20);
        lqsp(c, 19, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// 2D crowd.

/// `void CrowdMan2d_RemoveByType(CrowdMan2d* self, int id)` (0x229408):
/// clear the 128 slots whose type byte is `id`.
pub fn crowd_man2d_remove_by_type(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[S2][0] = c.g[S2][0] | 0xffff;
        frame.save(c, S4, 0x10);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A1][0];
        c.g[S0][0] = sx(p(r32(c, S3), 8));
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            if c.g[V0][0] == c.g[S2][0] {
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S0)).lbu(c);
                if c.g[V0][0] != c.g[S4][0] {
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    callee::CROWD_MAN2D_CLEAR_SLOT.call(c, 0x0022_9464)?;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 128i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_9440);
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

/// `void SSXApp_StartFEModule(self)` (0x228238): start the front-end module
/// (the big one from +0x7C or the small one from +0x80).
pub fn ssx_app_start_fe_module(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        Addr(gp_3(c, -0x10b4)).sw(c, 0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_2(c, S0, 0x60);
        let small = c.g[V0][0] == 0;
        c.g[A1][0] = 0x480000;
        if small {
            c.cycles += 12;
            c.g[A0][0] = 0x238;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x57e8));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, table_callee::EA_MEM_ALLOC_NAMED, 0x0022_82a4)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, table_callee::FE_MODULE_SMALL_CTOR, 0x0022_82ac)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            Addr(c.g[S0][0] as u32).off(0x80).sw(c, c.g[V0][0]);
            call_ptr(c, table_callee::APP_MAN_SET_NEXT_MODULE, 0x0022_82bc)?;
        } else {
            c.cycles += 14;
            c.g[A0][0] = 0xb0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x57e8));
            c.g[A0][0] = c.g[A0][0] | 0x5ae0;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, table_callee::EA_MEM_ALLOC_NAMED, 0x0022_8270)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, table_callee::FE_MODULE_BIG_CTOR, 0x0022_8278)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            Addr(c.g[S0][0] as u32).off(0x7c).sw(c, c.g[V0][0]);
            call_ptr(c, table_callee::APP_MAN_SET_NEXT_MODULE, 0x0022_8288)?;
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Crowd.

/// `void CrowdMan2d_Reset(CrowdMan2d* self)` (0x2292e0): clear the 0x2000 /
/// 0x500 byte blocks at_2 +0x210 / +0x2220, set the 128 ids to -1 with a random
/// 300..599 ms delay each, and clear the counts.
pub fn crowd_man2d_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        open(c, 0x70);
        c.g[A1][0] = 0;
        c.sq(20, at_2(c.g[SP][0] as u32, 0x20));
        c.g[A2][0] = 0x2000;
        let this = c.g[A0][0];
        c.g[S4][0] = this;
        c.sq(16, at_2(c.g[SP][0] as u32, 0x60));
        c.sq(17, at_2(c.g[SP][0] as u32, 0x50));
        c.g[A0][0] = sx((this as u32).wrapping_add(0x210));
        c.sq(18, at_2(c.g[SP][0] as u32, 0x40));
        c.g[S1][0] = sx((this as u32).wrapping_add(0x240));
        c.sq(19, at_2(c.g[SP][0] as u32, 0x30));
        c.g[S0][0] = sx((this as u32).wrapping_add(8));
        c.sq(21, at_2(c.g[SP][0] as u32, 0x10));
        c.g[S3][0] = 0xffff_ffff_ffff_0000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S3][0] = c.g[S3][0] | 0xffff;
        Addr(this as u32).off(0x2210).sw(c, 0);
        c.g[S5][0] = 0x12c;
        c.g[RA][0] = sx(0x0022_9330);
        c.g[S2][0] = 0x7f;
        call_ptr(c, 0x003e_6448, 0x0022_9330)?;
        loop {
            c.cycles += 7;
            Addr(c.g[S0][0] as u32).sw(c, c.g[S3][0]);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            c.g[RA][0] = sx(0x0022_9340);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            call_ptr(c, 0x0031_77f0, 0x0022_9340)?;
            c.divu(0, 2, 21);
            c.cycles += 5;
            c.g[V1][0] = c.hi[0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x12c));
            Addr(c.g[S1][0] as u32).sw(c, c.g[V1][0]);
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x40));
            if !more {
                c.cycles += 14;
                Addr(this as u32).off(0x2214).sw(c, 0);
                c.g[A0][0] = sx((this as u32).wrapping_add(0x2220));
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x500;
                call_ptr(c, 0x003e_6448, 0x0022_9374)?;
                c.lq(16, at_2(c.g[SP][0] as u32, 0x60));
                c.lq(17, at_2(c.g[SP][0] as u32, 0x50));
                c.lq(18, at_2(c.g[SP][0] as u32, 0x40));
                c.lq(19, at_2(c.g[SP][0] as u32, 0x30));
                c.lq(20, at_2(c.g[SP][0] as u32, 0x20));
                c.lq(21, at_2(c.g[SP][0] as u32, 0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                close(c, 0x70);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_9330);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Crowd manager.

/// `void CrowdMan2d_LoadState(CrowdMan2d* self, CrowdState* state)` (0x229e58):
/// clear the crowd, fetch the id table and re-create every live slot.
pub fn crowdman2d_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x250);
        frame.save(c, S3, 0x210);
        frame.save(c, S0, 0x240);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x230);
        frame.save(c, S2, 0x220);
        frame.save_ra(c, 0x200);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0022_9398, 0x0022_9e7c)?;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0022_92e0, 0x0022_9e84)?;
        c.g[V1][0] = 0xffff_ffff_ffff_0000;
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = c.g[SP][0];
        c.g[A0][0] = 0x7f;
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        loop {
            c.cycles += 7;
            Addr(r32(c, V0)).sw(c, c.g[V1][0]);
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
            let more = c.g[A0][0] != c.g[A1][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_9e98);
            }
        }
        c.cycles += 13;
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x200;
        c.g[S2][0] = 0xffff_ffff_ffff_0000;
        c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lh(c);
        c.g[S1][0] = 0;
        c.g[V1][0] = Addr(p(r32(c, V0), 0x14)).lw(c);
        c.g[S2][0] = c.g[S2][0] | 0xffff;
        let read = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, read, 0x0022_9edc)?;
        c.g[S0][0] = c.g[SP][0];
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        loop {
            c.cycles += 6;
            c.g[A1][0] = c.g[S1][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[V0][0];
            let empty = c.g[V0][0] == c.g[S2][0];
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !empty {
                c.cycles += 2;
                call_ptr(c, 0x0022_9910, 0x0022_9f08)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 128i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0022_9ee8);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x240);
        frame.restore(c, S1, 0x230);
        frame.restore(c, S2, 0x220);
        frame.restore(c, S3, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 2D crowd camera flashes.

/// `CrowdMan2d` fields of the flash sprites.
mod crowd2d {
    /// Emitter slots (`+8`, 128 words, -1 = free).
    pub const SLOTS: u32 = 0x8;
    /// Per-emitter spawn timer (`+0x240`, emitters 0x40 apart; their
    /// position and spread vectors at `+0x210`, `+0x220`, `+0x230`).
    pub const TIMER: u32 = 0x240;
    /// The crowd reaction last applied.
    pub const REACTION: u32 = 0x2210;
    /// Flash sprites (`+0x2220`, 40 of 0x20 bytes, lifetime at `+0x10`).
    pub const LIFETIME: u32 = 0x2230;
    /// A flash's lifetime in 60 Hz frames.
    pub const FLASH_FRAMES: u32 = 3;
}

/// `cCrowdAnim2D_Update` (through the function table).
const CROWD_ANIM2D_UPDATE: u32 = 0x002d_bac0;
/// `CrowdAudio_GetInstanceReaction` (through the function table).
const CROWD_AUDIO_GET_INSTANCE_REACTION: u32 = 0x002a_77c8;
const CROWD_MAN2D_SPAWN_SPRITES: Callee = Callee {
    addr: 0x0022_9530,
    f: ee::s05::CrowdMan2d_SpawnSprites,
};
const CROWD_MAN2D_ADD_SPRITE: Callee = Callee {
    addr: 0x0022_9738,
    f: ee::s05::CrowdMan2d_AddSprite,
};

/// `void CrowdMan2d_Update(CrowdMan2d* self)` (0x2294c8): update the crowd
/// animation, publish its frame (`+0x10`) as a half at `0x536690`, age the
/// 40 flash sprites by one tick and spawn new flashes.
///
/// Tick rate: the lifetimes count ticks; [`crowd_man2d_spawn_sprites`] gives
/// new flashes 3 60 Hz frames' worth of ticks, so a flash shows as long at
/// any rate.
pub fn crowd_man2d_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S0)).off(4).lw(c);
        call_ptr(c, CROWD_ANIM2D_UPDATE, 0x0022_94e0)?;
        c.g[V0][0] = Addr(r32(c, S0)).off(4).lw(c);
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0x27;
        c.g[V1][0] = Addr(r32(c, V0)).off(0x10).lw(c);
        c.sh(0x0053_6690, c.g[V1][0]);
        loop {
            c.cycles += 3;
            let life = Addr(r32(c, A1)).off(crowd2d::LIFETIME);
            c.g[V0][0] = life.lw(c);
            let shown = (c.g[V0][0] as i64) > 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            if shown {
                c.cycles += 1;
                life.sw(c, c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            let more = (c.g[A2][0] as i64) >= 0;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_94f8);
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        CROWD_MAN2D_SPAWN_SPRITES.call(c, 0x0022_951c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdMan2d_SpawnSprites(CrowdMan2d* self)` (0x229530): take the
/// crowd's reaction (`|CrowdAudio_GetInstanceReaction(gp+0x410, 0)|`; when it
/// changed, store it and give the animation `max(reaction, 1)`), then count
/// every used emitter's timer down by `reaction * 10 + 4`; an emitter that
/// runs out adds a flash sprite (3 frames) at its position plus a random
/// spread in [-1, 1) of its two spread vectors, and reloads 300..599.
///
/// Tick rate: the timers count down once per 60 Hz frame of this manager
/// ([`crate::tick::frame60_due`]) and a flash lives 3 60 Hz frames in ticks
/// ([`crate::tick::ticks_from_frames60`]), so the flashes per second and
/// their length match 60 Hz. Unchanged at 60.
pub fn crowd_man2d_spawn_sprites(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x110);
        c.g[A1][0] = 0;
        frame.save(c, S1, 0xd0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0xe0);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x410)).lw(c);
        frame.save(c, S2, 0xc0);
        frame.save(c, S3, 0xb0);
        frame.save(c, S4, 0xa0);
        frame.save(c, S5, 0x90);
        frame.save(c, S6, 0x80);
        frame.save(c, S7, 0x70);
        frame.save(c, FP, 0x60);
        frame.save_ra(c, 0x50);
        frame.save_fpr(c, 22, 0x100);
        frame.save_fpr(c, 21, 0xf8);
        frame.save_fpr(c, 20, 0xf0);
        call_ptr(c, CROWD_AUDIO_GET_INSTANCE_REACTION, 0x0022_9578)?;
        // reaction = |v0|
        c.g[A2][0] = c.g[V0][0];
        c.g[V1][0] = (-1i64 < c.g[A2][0] as i64) as u64;
        c.g[A0][0] = sx(0u32.wrapping_sub(r32(c, A2)));
        c.g[A2][0] = c.g[A0][0];
        let this = r32(c, S1);
        c.g[A1][0] = Addr(this).off(crowd2d::REACTION).lw(c);
        if c.g[V1][0] != 0 {
            c.g[A2][0] = c.g[V0][0];
        }
        let same = c.g[A2][0] == c.g[A1][0];
        c.g[V0][0] = 1;
        if !same {
            c.cycles += 5;
            c.g[V1][0] = (0 < c.g[A2][0] as i64) as u64;
            c.g[A0][0] = Addr(this).off(4).lw(c);
            if c.g[V1][0] != 0 {
                c.g[V0][0] = c.g[A2][0];
            }
            Addr(this).off(crowd2d::REACTION).sw(c, c.g[A2][0]);
            Addr(r32(c, A0)).sw(c, c.g[V0][0]);
        }
        if crate::tick::frame60_due(c, this.wrapping_add(crowd2d::TIMER), 0) {
            count_down_emitters(c)?;
        }
        c.cycles += 15;
        frame.restore(c, S0, 0xe0);
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, FP, 0x60);
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 22, 0x100);
        frame.restore_fpr(c, 21, 0xf8);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

/// The emitter loop of [`crowd_man2d_spawn_sprites`] (`s1` = the manager).
fn count_down_emitters(c: &mut Cpu) -> Flow {
    let this = r32(c, S1);
    c.cycles += 18;
    c.g[V0][0] = Addr(this).off(crowd2d::REACTION).lw(c);
    c.g[V1][0] = 0xa;
    c.g[S4][0] = 0xffff_ffff_ffff_0000;
    c.g[S3][0] = 0x7f_0000;
    c.mult(0, V0, V0, V1);
    c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
    c.mtc1(AT, 22);
    c.g[AT][0] = 0x3f80_0000; // 1.0
    c.mtc1(AT, 21);
    c.g[S4][0] |= 0xffff; // -1: a free slot
    c.g[S3][0] |= 0xffff; // mantissa mask
    c.g[S7][0] = 0x3f80_0000;
    c.g[S6][0] = 300;
    c.g[S0][0] = c.g[S1][0];
    c.g[FP][0] = sx(r32(c, V0).wrapping_add(4));
    c.g[S2][0] = sx(this.wrapping_add(crowd2d::SLOTS));
    c.g[S5][0] = 0x7f;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S2)).lw(c);
        if c.g[V0][0] == c.g[S4][0] {
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x40));
        } else {
            c.cycles += 4;
            let timer = Addr(r32(c, S0)).off(crowd2d::TIMER);
            c.g[V0][0] = timer.lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, FP)));
            let waiting = (c.g[V0][0] as i64) > 0;
            timer.sw(c, c.g[V0][0]);
            if !waiting {
                spawn_flash(c)?;
            }
            c.cycles += 1;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x40));
        }
        c.cycles += 3;
        c.g[S5][0] = sx(r32(c, S5).wrapping_sub(1));
        let more = (c.g[S5][0] as i64) >= 0;
        c.g[S2][0] = sx(r32(c, S2).wrapping_add(4));
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0022_95f8);
        }
    }
}

/// Emitter `s0` ran out: add a flash at `+0x210 + u * +0x220 + v * +0x230`
/// (u, v random in [-1, 1)) and reload the timer with 300 + rand % 300.
fn spawn_flash(c: &mut Cpu) -> Flow {
    let em = r32(c, S0);
    c.cycles += 50;
    call_ptr(c, callee::RAND_NEXT_MAIN, 0x0022_961c)?;
    // A float in [1, 2) from the random mantissa, then 2 * (x - 1) - 1.
    c.g[V0][0] &= c.g[S3][0];
    c.g[V0][0] |= c.g[S7][0];
    c.mtc1(V0, 0);
    c.sub_s(20, 0, 21);
    c.add_s(20, 20, 20);
    c.add_s(20, 20, 22);
    call_ptr(c, callee::RAND_NEXT_MAIN, 0x0022_9638)?;
    c.g[V0][0] &= c.g[S3][0];
    c.mfc1(A0, 20);
    c.lqc2(4, em.wrapping_add(0x220));
    c.qmtc2(A0, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, sp(c).wrapping_add(0x30));
    c.g[V0][0] |= c.g[S7][0];
    // The flash's lifetime: 3 frames at 60 Hz, in ticks.
    c.g[A2][0] = crate::tick::ticks_from_frames60(c, crowd2d::FLASH_FRAMES) as u64;
    c.mtc1(V0, 1);
    c.g[A0][0] = c.g[S1][0];
    c.lq(V0, sp(c).wrapping_add(0x30));
    c.g[A1][0] = c.g[SP][0];
    c.sub_s(0, 1, 21);
    c.sq(V0, sp(c).wrapping_add(0x20));
    c.lqc2(3, em.wrapping_add(0x210));
    c.lqc2(4, sp(c).wrapping_add(0x20));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x30));
    c.add_s(0, 0, 0);
    c.lq(V0, sp(c).wrapping_add(0x30));
    c.sq(V0, sp(c).wrapping_add(0x10));
    c.add_s(0, 0, 22);
    c.mfc1(V1, 0);
    c.lqc2(4, em.wrapping_add(0x230));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, sp(c).wrapping_add(0x40));
    c.lq(V0, sp(c).wrapping_add(0x40));
    c.sq(V0, sp(c).wrapping_add(0x30));
    c.lqc2(3, sp(c).wrapping_add(0x10));
    c.lqc2(4, sp(c).wrapping_add(0x30));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x40));
    c.lq(V0, sp(c).wrapping_add(0x40));
    c.sq(V0, sp(c));
    CROWD_MAN2D_ADD_SPRITE.call(c, 0x0022_96c8)?;
    call_ptr(c, callee::RAND_NEXT_MAIN, 0x0022_96d0)?;
    c.divu(0, V0, S6);
    c.cycles += 3;
    c.g[V1][0] = c.hi[0];
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(300));
    Addr(r32(c, S0)).off(crowd2d::TIMER).sw(c, c.g[V1][0]);
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::check_paths;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
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
    use crate::difftest::RA as RET;
    use crate::difftest::SP as STACK;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::fake_check;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::rng;
    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::heap_init_5 as heap_init;
    pub(crate) use crate::game::dirtysock::tags2::tests::run_4 as run;
    pub(crate) use crate::game::dirtysock::tags2::tests::ARENA;
    pub(crate) use crate::game::dirtysock::tags2::tests::ARENA_SIZE;
    pub(crate) use crate::game::dirtysock::tags2::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::fepopup3::tests::setup;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::globals;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::ovstates::tests::base_3;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup as setup_2;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::lib::sif::tests::SEEDS;
    pub(crate) use crate::game::movie::movieplayer::tests::globals as globals_2;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::sound::soundsys::tests::alloc_ready;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_4;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_4;
    pub(crate) use crate::game::sound::soundsys::tests::load_2 as load;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool;
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

    #[test]
    fn crowd_man_2d_add_sprite_matches() {
        check_lohi(
            ssx3_ee::s05::f_00229738,
            crowd_man_2d_add_sprite,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c10_0014 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                c.bus.write32(DATA + 0x2214, (seed % 8) as u32);
                for i in 0..4 {
                    c.bus.write32(DATA + 0x100 + i * 4, r.next_u32());
                }
                c.bus.write32(DATA + 0x2230, r.next_u32());
            },
        );
    }

    #[test]
    fn crowd_man2d_clear_slot_matches() {
        check(
            ssx3_ee::s05::f_00229b90,
            crowd_man2d_clear_slot,
            64,
            &|c, seed| {
                let mut r = Rng(0x0c11_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    #[test]
    fn ssx_app_next_tick_matches() {
        check(
            ssx3_ee::s05::f_00227e98__generated,
            ssx_app_next_tick,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 23, 0);
                let queue = DATA + 0x4_0000;
                let app = DATA + 0x300;
                // No app half the time; else a pad queue whose read cursor is
                // either still ahead (early out) or one step behind.
                if seed % 5 == 0 {
                    c.bus.write32(GAME_GP - 0x850, 0);
                } else {
                    c.bus.write32(GAME_GP - 0x850, queue);
                    let cursor = if seed % 3 == 0 { r.next_u32() % 30 } else { 0 };
                    let count = if seed % 3 == 0 {
                        (cursor + 1) % 30
                    } else {
                        r.next_u32() % 30
                    };
                    c.bus.write32(queue + 0x2ee0, cursor);
                    c.bus.write32(queue + 0x2ee4, count);
                }
                c.bus.write32(GAME_GP - 0x238, 0);
                c.bus.write32(GAME_GP + 0x300, 0);
                for i in 0..2u32 {
                    c.bus.write32(app + 0xa8 + i * 4, DATA + 0x800 + i * 0x100);
                }
                c.g[A0][0] = sx(app);
            },
        );
    }

    #[test]
    fn vu0_upload_occluder_matches() {
        check(
            ee::s05::f_00229f88__generated,
            vu0_upload_occluder,
            120,
            &|c, seed| {
                let mut r = Rng(0x51ed_2701 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx((seed % 4) as u32);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x140);
                for i in 0..4 {
                    put_vec(
                        c,
                        DATA + 0x100 + i * 0x10,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    );
                    put_vec(
                        c,
                        DATA + 0x140 + i * 0x10,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    );
                }
            },
        );
    }

    #[test]
    fn mc_manager_get_matches() {
        check(
            ee::s05::f_00227f80__generated,
            mc_manager_get,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22_7f80, 0);
                // The overlay manager already exists: take the cheap path.
                c.bus.write32(GAME_GP.wrapping_sub(0x47c), DATA + 0x500);
                c.g[A0][0] = r.next_u32() as u64;
            },
        );
    }

    // -----------------------------------------------------------------
    // World triggers.

    #[test]
    fn world_trigger_manager_destroy_instance_matches() {
        check(
            s08::f_002b4b48__generated,
            world_trigger_manager_destroy_instance,
            120,
            &|c, seed| {
                machine(c, seed, 0x2b4b48, 1);
                globals(c);
                c.g[A0][0] = sx(OBJ);
                clear(c, CHILD, 0x40);
                // A live singleton (destroyed, then cleared) or none.
                let mgr = if seed % 3 == 0 { 0 } else { CHILD };
                c.bus.write32(GAME_GP.wrapping_add(0x21e4), mgr);
            },
        );
    }

    #[test]
    fn crowd_man2d_clear_matches() {
        check(
            ssx3_ee::s05::f_00229498__generated,
            crowd_man2d_clear,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0xb0, 0);
                globals_2(c);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    // -------------------------------------------------------------- crowd

    #[test]
    fn crowd_man2d_add_matches() {
        check(
            ee::s05::f_00229788__generated,
            crowd_man2d_add,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x2297);
                let m = DATA;
                for i in 0..64u32 {
                    let v = if i == seed % 64 {
                        r.next_u32() & 0xffff_fffe
                    } else {
                        0xffff_ffff
                    };
                    c.bus.write32(m + 8 + 4 * i, v);
                }
                c.g[A0][0] = sx(m);
                c.g[A1][0] = sx(r.next_u32() % 8);
                c.pc = 0x0022_9788;
            },
        );
    }

    #[test]
    fn ssx_app_start_game_module_matches() {
        check(
            ssx3_ee::s05::f_002281d8__generated,
            ssx_app_start_game_module,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x2281d8);
                heap_init(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn crowd_man2d_dtor_matches() {
        check(
            ssx3_ee::s05::f_00229278__generated,
            crowd_man2d_dtor,
            60,
            &|c, seed| {
                let mut r = setup_2(c, seed, 0x229278);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 0x400 + 4, 0);
                c.bus.write32(DATA + 0x400, 0);
                let _ = r.next_u32();
            },
        );
    }

    /// The shipped Nash patch table at [`NASH_PATCH_TABLE`].
    pub(crate) const NASH_PATCH_HANDLERS: [u32; 5] = [
        0x0022_a214,
        0x0022_a260,
        0x0022_a260,
        0x0022_a214,
        0x0022_a23c,
    ];

    /// The shipped Nash instance table at [`NASH_INSTANCE_TABLE`].
    pub(crate) const NASH_INSTANCE_HANDLERS: [u32; 5] = [
        0x0022_a0d0,
        0x0022_a11c,
        0x0022_a11c,
        0x0022_a0d0,
        0x0022_a0f8,
    ];

    // ------------------------------------------- replay / audio jump tables

    /// [`check`] plus soft state and LO/HI: these ports reproduce every
    /// register write of the generated body, so caller-saved temporaries and
    /// the multiply pipeline must match too.
    pub(crate) fn strict_check(
        gen: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let r = crate::difftest::compare(gen, hand, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard {:#?} soft {:#?} lohi {:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }

    // ------------------------------------------------ Nash octree classifiers

    /// Wire a cell's heap word to a heap block: `*(cell+0x90)` = `block` and
    /// `*block` = `heap`, then make a node's slot/handle words classify
    /// (`heap + slot*8 + 0x24` = 6 and `heap + handle*0x18 + 0x3f0` = 3).
    pub(crate) fn nash_link(
        c: &mut Cpu,
        cell: u32,
        block: u32,
        heap: u32,
        node: u32,
        slot: u8,
        handle: u16,
        handle_word: u32,
    ) {
        c.bus.write32(cell + 0x90, block);
        c.bus.write32(block, heap);
        c.bus.write8(node + 0x7d, slot);
        c.bus.write16(node + 0x7e, handle);
        c.bus
            .write32(heap.wrapping_add((slot as u32) << 3).wrapping_add(0x24), 6);
        c.bus.write32(
            heap.wrapping_add((handle as u32) * 0x18)
                .wrapping_add(0x3f0),
            handle_word,
        );
    }

    #[test]
    fn nash_classify_patch_matches() {
        strict_check(
            ssx3_ee::s05::f_0022a128__generated,
            nash_classify_patch,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22a1_28, 0);
                c.pc = 0x0022_a128;
                // A null node takes its "heap" from VU0 VI1 (`cfc2`), which the
                // tail also uses as the table index.
                c.bus.plat.write_ctrl(1, seed % 8);
                put_words(c, NASH_PATCH_TABLE, &NASH_PATCH_HANDLERS);
                let cell = DATA + 0x1000;
                let block = DATA + 0x2800;
                let node = DATA + 0x2000;
                let heap = DATA + 0x3000;
                c.g[A0][0] = sx(cell);
                // The insert heads/counts (both branches of every handler).
                c.bus
                    .write32(cell + 0x78b0, if seed % 2 == 0 { 0 } else { node });
                c.bus.write32(cell + 0x50a8, r.next_u32() & 3);
                c.bus.write32(cell + 0x70ac, r.next_u32() & 3);
                // A classifying node by default; the cases below break one link.
                nash_link(c, cell, block, heap, node, 2, 1, 3);
                c.bus.write16(node + 0x156, 1);
                c.bus.write8(node + 0x155, 2);
                match seed % 6 {
                    // Null node with the real cell as the list owner.
                    0 => c.g[A1][0] = 0,
                    // Null node whose "cell" is a small handle: the tail stores
                    // the node there and indexes the table by VI1.
                    1 => {
                        let k = (seed / 6) % 5;
                        c.g[A0][0] = sx(k);
                        c.bus
                            .write32(0x78b0 + k, if seed % 2 == 0 { 0 } else { node });
                        c.g[A1][0] = 0;
                    }
                    // A negative handle word: an early miss.
                    2 => {
                        c.bus.write16(node + 0x156, 0xffff);
                        c.g[A1][0] = sx(node);
                    }
                    // Slot word is not 6.
                    3 => {
                        c.bus.write32(heap + (2 * 8) + 0x24, 0);
                        c.g[A1][0] = sx(node);
                    }
                    // Handle word is not 3.
                    4 => {
                        c.bus.write32(heap + (1 * 0x18) + 0x3f0, 0);
                        c.g[A1][0] = sx(node);
                    }
                    // Fully classified: the heap block lives at `h`, and the tail
                    // dispatches on the VI1 handle (small: table, else: head).
                    _ => {
                        let h = if (seed / 6) % 2 == 0 {
                            heap
                        } else {
                            (seed / 12) % 5
                        };
                        nash_link(c, cell, block, h, node, 2, 1, 3);
                        c.bus.write16(node + 0x156, 1);
                        c.bus.write8(node + 0x155, 2);
                        c.g[A1][0] = sx(node);
                    }
                }
            },
        );
    }

    #[test]
    fn nash_classify_instance_matches() {
        strict_check(
            ssx3_ee::s05::f_00229fc8__generated,
            nash_classify_instance,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x229f_c8, 0);
                c.pc = 0x0022_9fc8;
                // A null node takes its "heap" from VU0 VI1 (`cfc2`), which the
                // tail also uses as the table index.
                c.bus.plat.write_ctrl(1, seed % 8);
                put_words(c, NASH_INSTANCE_TABLE, &NASH_INSTANCE_HANDLERS);
                let cell = DATA + 0x1000;
                let block = DATA + 0x2800;
                let node = DATA + 0x2000;
                let heap = DATA + 0x3000;
                c.g[A0][0] = sx(cell);
                c.bus
                    .write32(cell + 0x50a4, if seed % 2 == 0 { 0 } else { node });
                c.bus.write32(cell + 0x209c, r.next_u32() & 3);
                c.bus.write32(cell + 0x40a0, r.next_u32() & 3);
                c.bus.write32(node + 0x8, 3);
                nash_link(c, cell, block, heap, node, 2, 1, 3);
                match seed % 6 {
                    // Null node with the real cell as the list owner.
                    0 => c.g[A1][0] = 0,
                    // Null node whose "cell" is a small handle: inserts there.
                    1 => {
                        let k = (seed / 6) % 5;
                        c.g[A0][0] = sx(k);
                        c.bus
                            .write32(0x50a4 + k, if seed % 2 == 0 { 0 } else { node });
                        c.g[A1][0] = 0;
                    }
                    // Type word does not end in 3.
                    2 => {
                        c.bus.write32(node + 0x8, 4);
                        c.g[A1][0] = sx(node);
                    }
                    // Byte 0xff ("no slot"): classified, then the VI1 tail test.
                    3 => {
                        let h = if (seed / 6) % 2 == 0 {
                            heap
                        } else {
                            (seed / 12) % 5
                        };
                        nash_link(c, cell, block, h, node, 2, 1, 3);
                        c.bus.write8(node + 0x7d, 0xff);
                        c.g[A1][0] = sx(node);
                    }
                    // Slot word is not 6.
                    4 => {
                        c.bus.write32(heap + (2 * 8) + 0x24, 0);
                        c.g[A1][0] = sx(node);
                    }
                    // Fully classified: dispatches on the VI1 handle in the tail.
                    _ => {
                        let h = if (seed / 6) % 2 == 0 {
                            heap
                        } else {
                            (seed / 12) % 5
                        };
                        nash_link(c, cell, block, h, node, 2, 1, 3);
                        c.g[A1][0] = sx(node);
                    }
                }
            },
        );
    }

    #[test]
    fn nash_empty_hook_matches() {
        leaf(ssx3_ee::s05::f_00229f80__generated, nash_empty_hook, 9);
    }

    // ---------------------------------------------------- Nash_ClassifyEffect

    #[test]
    fn nash_classify_effect_matches() {
        fake_check(
            ssx3_ee::s05::f_0022a270__generated,
            nash_classify_effect,
            39,
            &|c, seed, r| {
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    (DATA + 0x100) as u64
                };
                // VI1 selects the effect-kind table entry (the code moves it into
                // a0 with cfc2).
                c.bus.plat.write_ctrl(1, seed % 5);
                for i in 0..0x400 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
                // The effect-kind table (`.rodata 0x47b380`).
                put_words(
                    c,
                    0x0047_b380,
                    &[
                        0x0022_a30c,
                        0x0022_a358,
                        0x0022_a358,
                        0x0022_a30c,
                        0x0022_a334,
                    ],
                );
                // Registration heads/counts for the list-append branches.
                c.bus
                    .write32(DATA + 0x7ab8, if seed % 2 == 0 { 0 } else { DATA });
                c.bus.write32(DATA + 0x78b4, r.next_u32() & 3);
                c.bus.write32(DATA + 0x7abc, r.next_u32() & 3);
            },
        );
    }

    #[test]
    fn crowd_man2d_remove_all_matches() {
        check(
            ssx3_ee::s05::f_00229398__generated,
            crowd_man2d_remove_all,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x229398, 0);
                let m = DATA + 0x1000;
                c.g[A0][0] = m as u64;
                for i in 0..128u32 {
                    w32(
                        c,
                        m + 8 + 4 * i,
                        if (seed as u32 + i) % 3 == 0 {
                            0xffff_ffff
                        } else {
                            r.next_u32()
                        },
                    );
                }
            },
        );
    }

    #[test]
    fn crowd_man2d_remove_by_type_matches() {
        check(
            ee::s05::f_00229408__generated,
            crowd_man2d_remove_by_type,
            100,
            &|c, seed| {
                let _ = base_2(c, seed, 0x22_9408);
                globals_3(c);
                for i in 0..128u32 {
                    let slot = DATA + 8 + 4 * i;
                    match (seed.wrapping_add(i)) % 4 {
                        0 => w(c, slot, 0xffff_ffff),
                        1 => w(c, slot, seed & 0xff),
                        _ => w(c, slot, 0x0100_0000u32 | (i & 0xff)),
                    }
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(seed & 0xff);
            },
        );
    }

    // ---------------------------------------------------------------- Crowd.

    #[test]
    fn crowd_man2d_reset_matches() {
        check(
            ee::s05::f_002292e0__generated,
            crowd_man2d_reset,
            5,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x22_92e0);
                c.g[A0][0] = sx(DATA);
                for i in 0..0x2a0u32 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn crowdman2d_load_state_matches() {
        check(
            ee::s05::f_00229e58__generated,
            crowdman2d_load_state,
            8,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x229e58, 2);
                let this = DATA;
                clear_2(c, this, 0x1000);
                let state = DATA + 0x800;
                clear_2(c, state, 0x100);
                let vt = vtable(c, &mut r, &[0x10]);
                w(c, state, vt);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = state as u64;
                for i in 0..128u32 {
                    w(c, SP - 0x250 + 4 * i, 0xffff_ffff);
                }
            },
        );
    }

    /// A `CrowdMan2d` at `man`: its animation takes the short path, the
    /// crowd reaction is `reaction`, every ninth emitter slot is used (timers
    /// 0..59) and the 40 flash lifetimes are -2..4.
    fn crowd_fixture(c: &mut Cpu, r: &mut Rng, seed: u32, man: u32, reaction: i32) {
        let (anim, clock, audio, table) = (
            DATA + 0x3_0000,
            DATA + 0x3_0100,
            DATA + 0x3_0200,
            DATA + 0x3_0300,
        );
        c.bus.write32(man + 4, anim);
        c.bus.write32(anim + 0x10, r.next_u32());
        c.bus.write32(anim + 0x14, 0x7fff_ffff);
        c.bus.write32(anim + 0x18, 0);
        c.bus.write32(GAME_GP + 0x2a74, clock);
        c.bus.write32(clock + 0x1c, 100);
        c.bus.write32(GAME_GP + 0x410, audio);
        c.bus.write32(audio + 0x14, 1);
        c.bus.write32(audio + 0x18, table);
        c.bus.write32(table + 0x18c, reaction as u32);
        let old = if seed % 3 == 0 {
            reaction.unsigned_abs()
        } else {
            r.next_u32() % 5
        };
        c.bus.write32(man + 0x2210, old);
        c.bus.write32(man + 0x2214, seed % 40);
        for i in 0..128u32 {
            let used = (i + seed) % 9 == 0;
            c.bus
                .write32(man + 8 + 4 * i, if used { i } else { 0xffff_ffff });
            let em = man + 0x40 * i;
            for k in 0..12 {
                c.bus.write32(em + 0x210 + 4 * k, r.f32(100.0));
            }
            c.bus.write32(em + 0x240, r.next_u32() % 60);
        }
        for k in 0..40 {
            c.bus
                .write32(man + 0x2230 + 0x20 * k, (r.next_u32() % 7).wrapping_sub(2));
        }
    }

    #[test]
    fn crowd_man2d_spawn_sprites_matches() {
        check(
            ee::s05::f_00229530__generated,
            crowd_man2d_spawn_sprites,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22_9530, 0);
                crowd_fixture(c, &mut r, seed, DATA, (seed % 7) as i32 - 3);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn crowd_man2d_update_matches() {
        check(
            ee::s05::f_002294c8__generated,
            crowd_man2d_update,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22_94c8, 0);
                crowd_fixture(c, &mut r, seed, DATA, (seed % 7) as i32 - 3);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// Over one second of ticks the crowd flashes (how many, and how long
    /// they show in total) are the same at 60, 120 and 240 Hz. Each tick ages
    /// the lifetimes as `CrowdMan2d_Update` does, then spawns.
    #[test]
    fn crowd_man2d_flashes_follow_the_rate() {
        fn second(hz: u32) -> (u32, f64) {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            let mut r = machine(c, 3, 0x22_94c9, 0);
            // (Its own address: frame60_due keeps a clock per manager.)
            let man = DATA + 0x4_0000 + 0x1_0000 * (hz / 60);
            crowd_fixture(c, &mut r, 0, man, 10);
            // After the fixture: `gp+0x2a74` is the app manager pointer.
            crate::tick::set_for_test(c, hz);
            c.bus.write32(man + 0x2210, 10);
            for k in 0..40 {
                c.bus.write32(man + 0x2230 + 0x20 * k, 0);
            }
            let (mut spawned, mut shown) = (0u32, 0u64);
            for _ in 0..hz {
                for k in 0..40 {
                    let life = man + 0x2230 + 0x20 * k;
                    let v = c.bus.read32(life) as i32;
                    if v > 0 {
                        c.bus.write32(life, (v - 1) as u32);
                    }
                }
                let before = c.bus.read32(man + 0x2214);
                c.g[A0][0] = man as u64;
                crowd_man2d_spawn_sprites(c);
                assert_eq!(c.pc, RET);
                spawned += (c.bus.read32(man + 0x2214) + 40 - before) % 40;
                shown += (0..40)
                    .filter(|k| c.bus.read32(man + 0x2230 + 0x20 * k) as i32 > 0)
                    .count() as u64;
            }
            (spawned, shown as f64 / hz as f64)
        }
        let (n60, t60) = second(60);
        assert!(n60 > 20, "{n60} flashes");
        for hz in [120, 240] {
            let (n, t) = second(hz);
            assert_eq!(n, n60, "{hz} Hz: {n} flashes vs {n60} at 60");
            assert!(
                (t - t60).abs() < 1e-9,
                "{hz} Hz: shown {t} s vs {t60} s at 60"
            );
        }
    }
}
