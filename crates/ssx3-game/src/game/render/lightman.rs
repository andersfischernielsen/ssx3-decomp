//! `src/render/lightman.cpp`: hand-written replacements (moved from the
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
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::animation::animsequencer::dtor;
pub(crate) use crate::game::bx::bxstring::gp as gp_3;
pub(crate) use crate::game::fe::debugfe::at as at_2;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::at as at_4;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::movie::movieplayer::at as at_3;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::render::particle::p;
pub(crate) use crate::game::sound::asyncsys::lw;
pub(crate) use crate::game::world::wscriptcache::DELETE_2 as DELETE;
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
    (0x0038_dee8, light_man_v09, "LightMan_v09"),
    (0x0038_df38, light_man_v10, "LightMan_v10"),
    (0x0038_df98, light_man_v11, "LightMan_v11"),
    (0x0038_e008, light_man_v12, "LightMan_v12"),
    (
        0x0038_f2a8,
        vfx_plants_get_sway_matrix,
        "VfxPlants_GetSwayMatrix",
    ),
    (0x0038_f300, spr_stream_set_mode, "SprStream_SetMode"),
    (0x0038_f738, spr_stream_download, "SprStream_Download"),
    (
        0x0038_f768,
        dma_wait_from_scratchpad,
        "Dma_WaitFromScratchpad",
    ),
    (0x0038_f7b0, dma_wait_to_scratchpad, "Dma_WaitToScratchpad"),
    (
        0x0038_f930,
        tex_man_swizzle_t8_block,
        "TexMan_SwizzleT8Block",
    ),
    (
        0x0039_0458,
        ps2_graphics_man_reset_color_filter,
        "PS2GraphicsMan_ResetColorFilter",
    ),
    (0x0039_0c60, vfx_screen_tint_update, "VfxScreenTint_Update"),
    (
        0x0039_0ec8,
        vfx_screen_tint_trigger_flash,
        "VfxScreenTint_TriggerFlash",
    ),
    (
        0x0039_0ef8,
        vfx_screen_tint_poll_flash_sound,
        "VfxScreenTint_PollFlashSound",
    ),
    (0x0039_12a8, vu0_upload_microcode_a, "Vu0_UploadMicrocodeA"),
    (0x0039_1360, vu0_upload_microcode_b, "Vu0_UploadMicrocodeB"),
    (0x0039_14f8, vu0_read_vi1, "Vu0_ReadVI1"),
    (0x0039_16c0, font_dtor, "Font_dtor"),
    (0x0039_1708, font_load, "Font_Load"),
];

// ---------------------------------------------------------------------------
// Callees the generator emitted as direct calls (the readable alias, which
// dispatches through the override slot) and as cross-shard `call_indirect`s.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_IS_TRACK_GROUP0_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_RESET_RACE_RESULT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STORE_RACE_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COLLECTED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COMPLETED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_CHARACTER_2 as BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_SLOT_3 as BE_NEW_PLAYER_INT_GET_SLOT;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_LOAD_STATE_2 as GAME_STATE_LOAD_STATE;
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
    pub(crate) use crate::game::fe::festaterewards::callee::AI_VISUAL_EFFECTS_MAN_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::CUI_TEXT_SCROLL_SET_NAV_BUTTONS;
    pub(crate) use crate::game::fe::festaterewards::callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::fe::festaterewards::callee::IN_SEQ_INSTANCE_SETUP;
    pub(crate) use crate::game::fe::festaterewards::callee::LIGHT_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_FORMAT_FLOAT;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_DRAW_ROW;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_PUSH_PAGE;
    pub(crate) use crate::game::fe::festaterewards::callee::REWARD_GALLERY_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_BINDING_CLEAR_LISTENER;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_LIST_ON_SEQ_START;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_LIST_ON_SEQ_STOPPED;
    pub(crate) use crate::game::fe::festaterewards::callee::SEQ_LISTENER_ON_START;
    pub(crate) use crate::game::fe::festaterewards::callee::SEQ_LISTENER_ON_STOP;
    pub(crate) use crate::game::fe::festaterewards::callee::UI_LIST_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::intersect::riderspheretree::callee::DELETE;
    pub(crate) use crate::game::intersect::riderspheretree::callee::DMA_FROM_SCRATCHPAD;
    pub(crate) use crate::game::intersect::riderspheretree::callee::MOD_OBJ_UPDATE;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_CTOR_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_DTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_BE;
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
    pub(crate) use crate::game::movie::movieplayer::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
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
    pub(crate) use crate::game::movie::movieplayer::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS;
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
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
    pub(crate) use crate::game::world::wscriptcache::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::BANK_MONITOR_DELETE_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_NEW_RACE_INT_GET_TRACK_INFO;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::world::wscriptcache::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_RESET_CHASE_CONTROLLER_SWITCHES;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CHASE_CAMERA_ALGORITHM_CTOR_2 as CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::CINSTANCE_NODE_CTOR_V;
    pub(crate) use crate::game::world::wscriptcache::callee::CROWD_AUDIO_FREE_SAMPLE_LIST;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_BASE_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_INIT;
    pub(crate) use crate::game::world::wscriptcache::callee::C_WORLD_CACHE_STREAMER_DTOR_STUB;
    pub(crate) use crate::game::world::wscriptcache::callee::DELETE as DELETE_2;
    pub(crate) use crate::game::world::wscriptcache::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::FONT_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::INPUT_PREPROC_WRITE_STRING;
    pub(crate) use crate::game::world::wscriptcache::callee::MDF_FILE_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_FLUSH_BUFFER_3 as MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_GET_BITS1;
    pub(crate) use crate::game::world::wscriptcache::callee::PS2_SONY_CODEC_INTERNAL_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::RCMP_SYSTEM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_MAIN_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_SUB_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_SOUND_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::world::wscriptcache::callee::SECTION_ALLOCATOR_FREE_ALL;
    pub(crate) use crate::game::world::wscriptcache::callee::SND_BANK_MONITOR_MEM_SIZE;
    pub(crate) use crate::game::world::wscriptcache::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_PAUSE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_GET_COUNT;
    pub(crate) use crate::game::wscript::wscriptman::callee::ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::wscript::wscriptman::callee::APP_MODULE_UPDATE_FADE;
    pub(crate) use crate::game::wscript::wscriptman::callee::AVALANCHE_BAKE_NODE_HISTORY;
    pub(crate) use crate::game::wscript::wscriptman::callee::AV_PLAYER_INIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_POLL_FREE;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_SET_PITCH;
    pub(crate) use crate::game::wscript::wscriptman::callee::CAM_CONTROLLER_CLEAR_ALGORITHMS;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_CHUNK_RESOLVE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_CREATE_MESHES_2 as CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_FREE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FLAG_MANAGER_GROUP_LOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL;
    pub(crate) use crate::game::wscript::wscriptman::callee::DISABLE_INTC;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_FILE_MOUNT_BIG;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLEX_RAIL_UPDATE_SPACE_NODE;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLOATING_V49;
    pub(crate) use crate::game::wscript::wscriptman::callee::FOCUS_MAN_ADD_FOCUS_BOX;
    pub(crate) use crate::game::wscript::wscriptman::callee::LIGHT_MAN_EMPTY_HOOK_A;
    pub(crate) use crate::game::wscript::wscriptman::callee::MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::MOD_OBJ_DTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::wscript::wscriptman::callee::RAIL_MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::SET_FADE_GAIN;
    pub(crate) use crate::game::wscript::wscriptman::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_GET_STATE_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SET_USER_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::wscript::wscriptman::callee::WSCRIPT_MAN_CALL_FUNC;

    pub const LUNO_TABLE_DTOR: Callee = Callee {
        addr: 0x0022_4df0,
        f: ee::s05::f_00224df0,
    };

    pub const WORLD_GET_FLAG_MANAGER: Callee = Callee {
        addr: 0x002d_1cb0,
        f: ee::s09::World_GetFlagManager,
    };

    pub const WORLD_GET_AI_FRAME_COUNT: Callee = Callee {
        addr: 0x002d_1c98,
        f: ee::s09::World_GetAIFrameCount,
    };

    pub const WORLD_GET_NASH: Callee = Callee {
        addr: 0x002d_1be0,
        f: ee::s09::World_GetNash,
    };

    pub const LIGHT_CONTRIBUTION: Callee = Callee {
        addr: 0x002f_5d30,
        f: ee::s09::WorldLightMan_LightContribution,
    };

    pub const MATH_SIN_COS: Callee = Callee {
        addr: 0x0031_be50,
        f: ee::s10::Math_SinCos,
    };

    pub const SPATIAL_HASH_UPDATE: Callee = Callee {
        addr: 0x0032_91e0,
        f: ee::s10::SpatialHash_Update,
    };

    pub const FLAG_MANAGER_GROUP_COMPUTE_VERTS: Callee = Callee {
        addr: 0x0034_bca0,
        f: ee::s11::cFlagManager_GroupComputeVerts,
    };

    pub const INTERSECT_INFO_EVALUATE: Callee = Callee {
        addr: 0x0035_1538,
        f: ee::s11::IntersectInfo_Evaluate,
    };

    pub const TYPED_PROCESS_SAVE: Callee = Callee {
        addr: 0x0035_4948,
        f: ee::s11::TypedProcess_Save,
    };

    pub const SYNC_PROCESS_SAVE: Callee = Callee {
        addr: 0x0035_4980,
        f: ee::s11::SyncProcess_Save,
    };

    pub const MULTI_SPLINE_MODIFIER_DEBUG_DRAW_PATH: Callee = Callee {
        addr: 0x0034_5430,
        f: ee::s11::MultiSplineModifier_DebugDrawPath,
    };

    pub const EA_MEMSET: Callee = Callee {
        addr: 0x003e_6448,
        f: ee::s14::EA_MemSet,
    };

    pub const DMA_WAIT_FROM_SCRATCHPAD: Callee = Callee {
        addr: 0x0038_f768,
        f: ee::s12::Dma_WaitFromScratchpad,
    };

    pub const DMA_WAIT_TO_SCRATCHPAD: Callee = Callee {
        addr: 0x0038_f7b0,
        f: ee::s12::Dma_WaitToScratchpad,
    };
}

// ---------------------------------------------------------------------------
// VfxScreenTint_Update (0x390c60).

/// The screen-tint animation state (`gp` globals, `D_004A46xx`).
pub(crate) mod tint {

    /// Master enable (`D_004A460C`).
    pub const ENABLED: i32 = 0x151c;

    /// Countdown of the frames left before another flash may start
    /// (`D_004A46B4`).
    pub const COOLDOWN: i32 = 0x15c4;

    /// Flash timer (`D_004A4698`): -1 while idle, then counts up.
    pub const TIMER: i32 = 0x15a8;

    /// Trigger threshold (`D_004A4610`).
    pub const TRIGGER: i32 = 0x1520;

    /// Flash probability factor (`D_004A4614`).
    pub const CHANCE: i32 = 0x1524;

    /// Idle delay (`D_004A4618`).
    pub const D0: i32 = 0x1528;

    /// Second segment (`D_004A461C`).
    pub const D1: i32 = 0x152c;

    /// Third segment (`D_004A4620`).
    pub const D2: i32 = 0x1530;

    /// Fourth segment (`D_004A4624`).
    pub const D3: i32 = 0x1534;
}

// ---------------------------------------------------------------------------
// PS2 graphics manager (colour filter)

/// `void PS2GraphicsMan_ResetColorFilter(PS2GraphicsMan *man, int index)`
/// (0x390458): write the six default colour-filter floats from `gp + 0x14f4`
/// into the record at `man + index*0x18 + 0x6ca4`.
pub fn ps2_graphics_man_reset_color_filter(c: &mut Cpu) {
    c.cycles += 17;
    c.g[V0][0] = 0x18;
    let gp = c.g[GP][0] as u32;
    c.f[1] = Addr(gp.wrapping_add(0x14f4)).read_fpu(c).0;
    c.mult(0, 5, 5, 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6ca4));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let dst = c.g[A0][0] as u32;
    Addr(dst).write_fpu(c, FpuF32(c.f[1]));
    c.f[0] = Addr(gp.wrapping_add(0x14f8)).read_fpu(c).0;
    Addr(dst).off(0x4).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = Addr(gp.wrapping_add(0x14fc)).read_fpu(c).0;
    Addr(dst).off(0x8).write_fpu(c, FpuF32(c.f[1]));
    c.f[0] = Addr(gp.wrapping_add(0x1500)).read_fpu(c).0;
    Addr(dst).off(0xc).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = Addr(gp.wrapping_add(0x1504)).read_fpu(c).0;
    Addr(dst).off(0x10).write_fpu(c, FpuF32(c.f[1]));
    c.f[0] = Addr(gp.wrapping_add(0x1508)).read_fpu(c).0;
    Addr(dst).off(0x14).write_fpu(c, FpuF32(c.f[0]));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Screen tint

/// `void VfxScreenTint_TriggerFlash(float intensity)` (0x390ec8): when the
/// screen tint is active, start a flash: store the scaled intensity and its
/// integer value in the global tint block.
///
/// Tick rate: the thunder delay (`+0x15c4`, ticks counted down by
/// `VfxScreenTint_Update`) is converted from 60 Hz frames with
/// `tick::ticks_from_frames60` (exact at 60).
pub fn vfx_screen_tint_trigger_flash(c: &mut Cpu) {
    c.cycles += 3;
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = Addr(gp.wrapping_add(0x151c)).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        let rate = Addr(gp.wrapping_sub(0x2698)).read_fpu(c);
        c.f[0] = rate.0;
        Addr(gp.wrapping_add(0x15a8)).sw(c, 0);
        let scaled = abi::farg(c, 0) * rate;
        c.f[0] = scaled.0;
        Addr(gp.wrapping_add(0x15ac)).sw(c, 0);
        Addr(gp.wrapping_add(0x15c0)).write_fpu(c, abi::farg(c, 0));
        // The thunder delay is a count of 60 Hz frames (distance times the
        // speed of sound per frame): the same wall-clock delay at any rate.
        let mut i = scaled.to_i32();
        if i > 0 {
            i = crate::tick::ticks_from_frames60(c, i as u32) as i32;
        }
        c.f[1] = i as u32;
        Addr(gp.wrapping_add(0x15c4)).write_fpu(c, FpuF32(c.f[1]));
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool VfxScreenTint_PollFlashSound(void)` (0x390ef8): true once while the
/// flash is active and its countdown has reached zero, clearing the countdown.
pub fn vfx_screen_tint_poll_flash_sound(c: &mut Cpu) {
    c.cycles += 3;
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = Addr(gp.wrapping_add(0x151c)).lw(c);
    let inactive = c.g[V0][0] == 0;
    c.g[V0][0] = Addr(gp.wrapping_add(0x15c4)).lw(c);
    if inactive {
        // fall through to the false return
    } else {
        c.cycles += 2;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        if c.g[V0][0] != 0 {
            // counting down: still false
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            Addr(gp.wrapping_add(0x15c4)).sw(c, c.g[V1][0]);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// SprStream_Download.

/// `void SprStream_Download(u32 madr, int size, int flags)` (0x38f738): send
/// `size >> 4` qwords from the scratchpad to `madr` through the DMA manager's
/// state object (`gp + 0x2a94`).
pub fn spr_stream_download(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[V0][0] = c.g[A0][0];
        c.g[A3][0] = c.g[A2][0];
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = sx(((c.g[A1][0] as u32) as i32 >> 4) as u32);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a94));
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[V0][0];
        callee::DMA_FROM_SCRATCHPAD.call(c, 0x0038_f758)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// DMA.

/// `void Dma_WaitToScratchpad(void)` (0x38f7b0): spin on DMA channel 4's
/// `CHCR` until the transfer to the scratchpad (`0x100`) has completed.
pub fn dma_wait_to_scratchpad(c: &mut Cpu) {
    /// DMA channel 4 control register (the original polls `0x1000d400`).
    const D_CHCR4: u32 = 0x1000_d400;
    /// `CHCR.STR`-side "transfer running" bit the original waits on.
    const BUSY: u64 = 0x100;

    c.cycles += 6;
    c.g[V0][0] = 0x1000_0000;
    c.g[V0][0] |= 0xd400;
    c.g[V1][0] = Addr(D_CHCR4).lw(c);
    c.g[V1][0] &= BUSY;
    let done = c.g[V1][0] == 0;
    c.g[V1][0] = 0x1000_0000;
    if !done {
        c.cycles += 2;
        c.g[V1][0] |= 0xd400;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(D_CHCR4).lw(c);
            c.g[V0][0] &= BUSY;
            if c.g[V0][0] == 0 {
                break;
            }
            if c.cycles >= c.next_event {
                crate::events::deliver_event_here(c, 0x0038_f7d0);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off`, 32-bit wrap.
#[inline(always)]
pub(crate) fn off(a: u32, o: u32) -> u32 {
    a.wrapping_add(o)
}

/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, o: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(o as u32)
}

/// Loads and stores as methods: a method's receiver borrow is two-phased, so
/// an argument may read `c.g` as the original's address arithmetic does.
pub(crate) trait Oc49Mem {
    fn ldw(&mut self, a: u32) -> u64;
    fn ldh(&mut self, a: u32) -> u64;
    fn ldhu(&mut self, a: u32) -> u64;
    fn stw(&mut self, a: u32, v: u64);
    fn sth(&mut self, a: u32, v: u64);
}

impl Oc49Mem for Cpu {
    #[inline(always)]
    fn ldw(&mut self, a: u32) -> u64 {
        Addr(a).lw(self)
    }
    #[inline(always)]
    fn ldh(&mut self, a: u32) -> u64 {
        Addr(a).lh(self)
    }
    #[inline(always)]
    fn ldhu(&mut self, a: u32) -> u64 {
        Addr(a).lhu(self)
    }
    #[inline(always)]
    fn stw(&mut self, a: u32, v: u64) {
        Addr(a).sw(self, v);
    }
    #[inline(always)]
    fn sth(&mut self, a: u32, v: u64) {
        Addr(a).sh(self, v);
    }
}

/// `addu`: 32-bit add, sign-extended into the 64-bit register.
#[inline(always)]
pub(crate) fn addu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// `int SprStream_SetMode(SprStream *self, int mode)` (0x0038f300): set the
/// ring-buffer mode, its element count/stride and the buffer addresses, then
/// flush the scratchpad DMA. The buffer loop is re-enterable at
/// `0x0038f3e8` (the trampoline resumes a loop whose event asked to unwind).
pub fn spr_stream_set_mode(c: &mut Cpu) {
    guest(c, |c| {
        let resumed = c.pc == 0x0038_f3e8;
        if !resumed && c.pc != 0x0038_f300 {
            c.jump_out(c.pc, 0x0038_f300, 0x0038_f460);
            return Ok(());
        }
        let frame = if resumed {
            Frame { size: 0x10 }
        } else {
            Frame::open(c, 0x10)
        };
        let mut run_loop = resumed;
        if !resumed {
            c.cycles += 6;
            c.g[A2][0] = c.g[A0][0];
            frame.save_ra(c, 0);
            c.g[V0][0] = c.ldw(c.g[A2][0] as u32);
            let same = c.g[V0][0] == c.g[A1][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            if same {
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 5;
            c.stw(c.g[A2][0] as u32, c.g[A1][0]);
            c.g[A1][0] = c.g[V0][0];
            let in_range = c.g[A1][0] < 7;
            c.g[V0][0] = in_range as u64;
            c.g[V0][0] = 0x49_0000;
            if in_range {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx(0x49_0000u32.wrapping_add(0x2ef0));
                c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
                c.g[A0][0] = c.ldw(c.g[V1][0] as u32);
                let target = c.g[A0][0] as u32;
                match target {
                    0x0038_f344 => {
                        c.cycles += 2;
                        c.g[V0][0] = 1;
                    }
                    0x0038_f34c | 0x0038_f354 | 0x0038_f35c | 0x0038_f364 => {
                        c.cycles += 2;
                        c.g[V0][0] = 2;
                    }
                    0x0038_f36c => {
                        c.cycles += 1;
                        c.g[V0][0] = 3;
                    }
                    _ => {
                        c.jump_out(target, 0x0038_f300, 0x0038_f460);
                        return Ok(());
                    }
                }
                c.cycles += 1;
                c.stw(off(c.g[A2][0] as u32, 4), c.g[V0][0]);
            }
            c.cycles += 4;
            c.stw(off(c.g[A2][0] as u32, 8), 0);
            let stride_range = c.g[A1][0] < 7;
            c.g[V0][0] = stride_range as u64;
            c.stw(off(c.g[A2][0] as u32, 0xc), 0);
            if stride_range {
                c.cycles += 7;
                c.g[V0][0] = 0x49_0000;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx(0x49_0000u32.wrapping_add(0x2f10));
                c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
                c.g[A0][0] = c.ldw(c.g[V1][0] as u32);
                let target = c.g[A0][0] as u32;
                // Each handler block has its own cost (0x38f3c8 is the
                // fall-through tail and costs one cycle, the rest two).
                let (handler, stride) = match target {
                    0x0038_f3a0 => (2u64, 0x400u64),
                    0x0038_f3a8 => (2, 0x200),
                    0x0038_f3b0 => (2, 0x1c0),
                    0x0038_f3b8 => (2, 0x180),
                    0x0038_f3c0 => (2, 0x140),
                    0x0038_f3c8 => (1, 0x100),
                    _ => {
                        c.jump_out(target, 0x0038_f300, 0x0038_f460);
                        return Ok(());
                    }
                };
                c.cycles += handler;
                c.cycles += 1;
                c.g[V0][0] = stride;
                c.stw(off(c.g[A2][0] as u32, 0x10), c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[V0][0] = c.ldw(off(c.g[A2][0] as u32, 4));
            let no_buffers = (c.g[V0][0] as i64) <= 0;
            c.g[A0][0] = 0;
            if !no_buffers {
                c.cycles += 3;
                c.g[A1][0] = 0x7000_0000;
                c.g[V1][0] = sx(off(c.g[A2][0] as u32, 0x2c));
                run_loop = true;
            }
        }
        if run_loop {
            if spr_stream_buffer_loop(c).is_err() {
                return Ok(());
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.ldw(off(c.g[A2][0] as u32, 4));
        c.g[V0][0] = c.ldw(off(c.g[A2][0] as u32, 0x10));
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = ((c.g[V0][0] as i64) < 1024) as u64;
        let fits = c.g[V1][0] == 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        if fits {
            c.cycles += 1;
            c.stw(gp(c, 0x2a94), 0);
        } else {
            c.cycles += 4;
            c.g[V1][0] = 0x7000_0000;
            c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
            c.stw(gp(c, 0x2a94), c.g[V0][0]);
        }
        c.cycles += 4;
        callee::DMA_WAIT_FROM_SCRATCHPAD.call(c, 0x0038_f44c)?;
        callee::DMA_WAIT_TO_SCRATCHPAD.call(c, 0x0038_f454)?;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `0x38f3e8` buffer-fill loop of `SprStream_SetMode`: writes
/// `0x70000000 + stride*i*16` and clears the per-entry words until
/// `i == count`. Returns `Ok` when the loop completed (the caller continues
/// at `0x38f418`) and `Err` when an event asked the thread to unwind.
pub(crate) fn spr_stream_buffer_loop(c: &mut Cpu) -> Flow {
    loop {
        c.cycles += 12;
        c.g[V0][0] = c.ldw(off(c.g[A2][0] as u32, 0x10));
        c.stw((c.g[V1][0] as u32).wrapping_sub(0xc), 0);
        c.mult(0, V0, V0, A0);
        c.stw(c.g[V1][0] as u32, 0);
        c.g[A0][0] = sx(off(c.g[A0][0] as u32, 1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = addu(c.g[V0][0], c.g[A1][0]);
        c.stw((c.g[V1][0] as u32).wrapping_sub(0x18), c.g[V0][0]);
        c.g[V0][0] = c.ldw(off(c.g[A2][0] as u32, 4));
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = sx(off(c.g[V1][0] as u32, 4));
        if !more {
            return Ok(());
        }
        if back_edge_unwinds(c, 0x0038_f3e8) {
            return Err(crate::call::Unwound);
        }
    }
}

// ---------------------------------------------------------------------------
// DMA.

/// `void Dma_WaitFromScratchpad(void)` (0x38f768): spin until the DMA
/// controller's "from scratchpad" channel (D_STAT bit 8) is idle.
pub fn dma_wait_from_scratchpad(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x1000_0000;
    c.g[V0][0] = c.g[V0][0] | 0xd000;
    c.g[V1][0] = c.lw(r32(c, V0));
    c.g[V1][0] = c.g[V1][0] & 0x100;
    let idle = c.g[V1][0] == 0;
    c.g[V1][0] = 0x1000_0000;
    if !idle {
        c.cycles += 2;
        c.g[V1][0] = c.g[V1][0] | 0xd000;
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.lw(r32(c, V1));
            c.g[V0][0] = c.g[V0][0] & 0x100;
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0038_f788);
            }
        }
    }
    c.cycles += 2;
    c.pc = c.g[RA][0] as u32;
}

/// `gp_2 + off` (a gp_2-relative global) as a guest address.
#[inline(always)]
pub(crate) fn gp_2(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `rd = rd + k` on a 32-bit register value (`addiu`).
#[inline(always)]
pub(crate) fn addiu(c: &mut Cpu, rd: usize, k: u32) {
    let v = (c.g[rd][0] as u32).wrapping_add(k);
    c.g[rd][0] = sx(v);
}

/// The frame after the prologue (the part skipped when `ENABLED` is 0).
/// Returns true when it already reloaded `s0` (the original's early return
/// from the flash path, which skips the tail's `lq s0`).
pub(crate) fn tint_body(c: &mut Cpu, frame: Frame) -> Result<bool, Unwound> {
    use tint::*;
    // Per-frame cooldown.
    c.cycles += 3;
    c.g[V0][0] = c.lw(gp_2(c, COOLDOWN));
    let cooled = (c.g[V0][0] as i64) <= 0;
    addiu(c, V0, 0xffff_ffff);
    if !cooled {
        c.cycles += 1;
        c.sw(gp_2(c, COOLDOWN), c.g[V0][0]);
    }

    // A running flash: pick the next one at random, else leave the timer
    // counting up to the trigger.
    c.cycles += 3;
    c.g[A3][0] = c.lw(gp_2(c, TIMER));
    let idle = (c.g[A3][0] as i64) >= 0;
    c.g[A2][0] = c.lw(gp_2(c, TRIGGER));
    if !idle {
        // `RandF() < chance^2` decides whether a flash fires now.
        c.cycles += 16;
        c.g[S0][0] = 0x007f_0000;
        call_ptr(c, 0x0031_77f0, 0x0039_0ca0)?;
        c.g[S0][0] |= 0xffff;
        c.g[S1][0] = 0x3f80_0000;
        let mask = c.g[S0][0];
        c.g[V0][0] &= mask;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20); // f20 = 1.0
        let exp = c.g[S1][0];
        c.g[V0][0] |= exp;
        c.lwc1(0, gp_2(c, CHANCE));
        c.mtc1(2, 2);
        c.mul_s(0, 0, 0);
        // The chance is per 60 Hz frame: at rate r roll with
        // 1 - (1 - p)^(60/r) per tick (the same strikes per second).
        let hz = crate::tick::rate(c);
        if hz != 60 {
            let p = (f32::from_bits(c.f[0]) as f64).clamp(0.0, 1.0);
            c.f[0] = ((1.0 - (1.0 - p).powf(60.0 / hz as f64)) as f32).to_bits();
        }
        c.sub_s(1, 2, 20);
        c.c_lt_s(1, 0);
        if !c.fcc() {
            frame.restore(c, S0, 0x20);
            return Ok(true);
        }
        // Flash: duration `(rand - 1) * K0 + K1` scaled by K2.
        c.cycles += 15;
        call_ptr(c, 0x0031_77f0, 0x0039_0ce0)?;
        let mask = c.g[S0][0];
        c.g[V0][0] &= mask;
        c.lwc1(0, gp_2(c, -0x26a4));
        let exp = c.g[S1][0];
        c.g[V0][0] |= exp;
        c.lwc1(2, gp_2(c, -0x26a0));
        c.mtc1(2, 3);
        c.lwc1(1, gp_2(c, -0x269c));
        c.sub_s(12, 3, 20);
        c.mul_s(12, 12, 0);
        c.add_s(12, 12, 2);
        c.mul_s(12, 12, 1);
        Callee {
            addr: 0x0039_0ec8,
            f: ssx3_ee::s12::VfxScreenTint_TriggerFlash,
        }
        .call(c, 0x0039_0d0c)?;
        frame.restore(c, S0, 0x20);
        return Ok(true);
    }

    // The flash timer: count up to the trigger or clamp to it.
    c.cycles += 2;
    let trig_neg = (c.g[A2][0] as i64) < 0;
    c.g[V0][0] = c.lw(gp_2(c, D0));
    let advance;
    if trig_neg {
        advance = true;
    } else {
        c.cycles += 9;
        c.g[V1][0] = c.lw(gp_2(c, D1));
        c.g[A1][0] = c.lw(gp_2(c, D2));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A0][0] = c.lw(gp_2(c, D3));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.sw(gp_2(c, TIMER), c.g[A2][0]);
            advance = false;
        } else {
            advance = true;
        }
    }
    if advance {
        c.cycles += 2;
        // The four segment lengths are 60 Hz frames: the flash timer steps
        // on its own 60 Hz clock (every tick at 60).
        let t = c.g[A3][0] as u32;
        let key = gp_2(c, TIMER);
        if crate::tick::frame60_due(c, key, t) {
            c.g[V0][0] = sx(t.wrapping_add(1));
            c.sw(gp_2(c, TIMER), c.g[V0][0]);
            crate::tick::frame60_ran(c, key, t.wrapping_add(1));
        }
    }

    // The tint level and colours for the current segment of the flash.
    c.cycles += 5;
    c.g[V1][0] = c.lw(gp_2(c, TIMER));
    c.g[A1][0] = c.lw(gp_2(c, D0));
    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let past_first = c.g[V0][0] != 0;
    c.g[T0][0] = c.lw(gp_2(c, D1));
    if !past_first {
        // Ramp up over the first segment: level = (timer / d0) * K.
        c.cycles += 17;
        c.mtc1(3, 0);
        c.cvt_s_w(0, 0);
        c.lwc1(5, gp_2(c, 0x1538));
        c.mtc1(5, 2);
        c.cvt_s_w(2, 2);
        c.lwc1(3, gp_2(c, 0x1544));
        c.lwc1(4, gp_2(c, 0x1548));
        c.g[V0][0] = 5;
        c.lwc1(1, gp_2(c, 0x154c));
        c.div_s(0, 0, 2);
        c.swc1(3, gp_2(c, 0x15b0));
        c.swc1(4, gp_2(c, 0x15b4));
        c.swc1(1, gp_2(c, 0x15b8));
        c.sw(gp_2(c, 0x15bc), c.g[V0][0]);
        c.mul_s(0, 0, 5);
        c.swc1(0, gp_2(c, 0x15ac));
        return Ok(false);
    } else {
        // Second segment: its peak colours.
        c.cycles += 4;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[T0][0] as u32)));
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let third = c.g[V0][0] != 0;
        c.g[A2][0] = c.lw(gp_2(c, D2));
        if !third {
            c.cycles += 6;
            c.lwc1(3, gp_2(c, 0x1538));
            c.g[V0][0] = 5;
            c.lwc1(0, gp_2(c, 0x1544));
            c.lwc1(1, gp_2(c, 0x1548));
            c.lwc1(2, gp_2(c, 0x154c));
        } else {
            // Third segment.
            c.cycles += 4;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let tail = c.g[V0][0] != 0;
            c.g[A3][0] = c.lw(gp_2(c, D3));
            if !tail {
                // Third segment: its peak colours.
                c.cycles += 5;
                c.lwc1(3, gp_2(c, 0x153c));
                c.g[V0][0] = 5;
                c.lwc1(0, gp_2(c, 0x1550));
                c.lwc1(1, gp_2(c, 0x1554));
                c.lwc1(2, gp_2(c, 0x1558));
            } else {
                // Past the third segment: the fourth, or back to idle.
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let past_end = c.g[V0][0] != 0;
                c.lwc1(2, gp_2(c, 0x14f4));
                if !past_end {
                    // Last segment: level ramps down from 1.
                    c.cycles += 29;
                    c.mtc1(5, 2);
                    c.cvt_s_w(2, 2);
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 1);
                    c.mtc1(3, 0);
                    c.cvt_s_w(0, 0);
                    c.lwc1(8, gp_2(c, 0x1540));
                    c.mtc1(8, 6);
                    c.cvt_s_w(6, 6);
                    c.lwc1(4, gp_2(c, 0x155c));
                    c.mtc1(6, 7);
                    c.cvt_s_w(7, 7);
                    c.lwc1(5, gp_2(c, 0x1560));
                    c.sub_s(0, 0, 2);
                    c.lwc1(3, gp_2(c, 0x1564));
                    c.mtc1(7, 2);
                    c.cvt_s_w(2, 2);
                    c.g[V0][0] = 5;
                    c.swc1(4, gp_2(c, 0x15b0));
                    c.sub_s(0, 0, 6);
                    c.swc1(5, gp_2(c, 0x15b4));
                    c.swc1(3, gp_2(c, 0x15b8));
                    c.sw(gp_2(c, 0x15bc), c.g[V0][0]);
                    c.sub_s(0, 0, 7);
                    c.div_s(0, 0, 2);
                    c.sub_s(1, 1, 0);
                    c.mul_s(1, 1, 8);
                    c.swc1(1, gp_2(c, 0x15ac));
                } else {
                    c.cycles += 10;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    c.lwc1(1, gp_2(c, 0x14f8));
                    c.g[V1][0] = 5;
                    c.lwc1(0, gp_2(c, 0x14fc));
                    c.sw(gp_2(c, TIMER), c.g[V0][0]);
                    c.swc1(2, gp_2(c, 0x15b0));
                    c.swc1(1, gp_2(c, 0x15b4));
                    c.swc1(0, gp_2(c, 0x15b8));
                    c.sw(gp_2(c, 0x15bc), c.g[V1][0]);
                    c.sw(gp_2(c, 0x15ac), 0);
                }
                return Ok(false);
            }
        }
    }

    // Segment colours written for the second, third and fourth segments.
    c.cycles += 6;
    c.swc1(3, gp_2(c, 0x15ac));
    c.swc1(0, gp_2(c, 0x15b0));
    c.swc1(1, gp_2(c, 0x15b4));
    c.swc1(2, gp_2(c, 0x15b8));
    c.sw(gp_2(c, 0x15bc), c.g[V0][0]);
    Ok(false)
}

/// `void func_00390C60(void)` (0x390c60): animate the light manager's screen
/// tint. With `D_004A460C` set, tick the cooldown, and every frame either
/// roll for a new flash (`RandF() < D_004A4614^2`, with a random duration and
/// colour) or step the running flash through its four segments, writing the
/// level (+0x15ac) and colours (+0x15b0..+0x15bc).
///
/// Tick rate: the flash timer advances on a 60 Hz clock
/// (`tick::frame60_due`, keyed by the timer's address), the per-tick strike
/// chance is `1 - (1 - p)^(60/rate)`, and the thunder countdown (+0x15c4)
/// counts the ticks `VfxScreenTint_TriggerFlash` converted. Exact at 60.
///
/// Frame 0x40: s0 at +0x20, s1 at +0x10, ra at 0, f20 at +0x30.
pub fn vfx_screen_tint_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.lw(gp_2(c, tint::ENABLED));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.swc1(20, frame.at(c, 0x30));
        let reloaded = if c.g[V0][0] != 0 {
            tint_body(c, frame)?
        } else {
            false
        };
        if !reloaded {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `base + off` on a guest pointer (the `lw`/`sw` offsets).
#[inline(always)]
pub(crate) fn at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `int Vu0_ReadVI1(void)` (0x3914f8): VU0's VI1 register (`cfc2 $vi1`).
pub fn vu0_read_vi1(c: &mut Cpu) {
    c.cycles += 3;
    c.cfc2(V0, 1);
    abi::ret(c);
}

/// `void Font_dtor(Font* f, int flags)` (0x3916c0): free the font and, when
/// `flags & 1`, the object.
pub fn font_dtor(c: &mut Cpu) {
    guest(c, |c| dtor(c, callee::FONT_FREE, 0x0039_16dc, 0x0039_16f0))
}

/// `void LightMan_v09(void* obj, int a, int arg, float f)` (0x38dee8): resolve
/// the object's light hook and forward the call.
pub fn light_man_v09(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x20);
        c.mov_s(20, 12);
        c.g[A2][0] = Addr(this).lw(c);
        c.g[V1][0] = Addr(r32(c, A2)).off(0x10).lh(c);
        c.g[V0][0] = Addr(r32(c, A2)).off(0x14).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, V1)));
        call_ptr(c, target, 0x0038_df14)?;
        c.g[A1][0] = c.g[S0][0];
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[V0][0];
        callee::LIGHT_MAN_EMPTY_HOOK_A.call(c, 0x0038_df24)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `Matrix* VfxPlants_GetSwayMatrix(const Vec2* pos)` (0x38f2a8): one of eight
/// sway matrices chosen from the plant's distance to the origin.
pub fn vfx_plants_get_sway_matrix(c: &mut Cpu) {
    c.cycles += 22;
    let a0 = r32(c, A0);
    c.lwc1(1, p(a0, 0x4));
    c.g[A1][0] = u64::MAX;
    c.lwc1(0, p(a0, 0x0));
    c.mul_s(1, 1, 1);
    c.lwc1(2, gp_3(c, 0x1368));
    c.mul_s(0, 0, 0);
    c.g[A0][0] = 0x0050_0000;
    c.g[A0][0] = sx(0x0050_4d80);
    c.add_s(0, 0, 1);
    c.sqrt_s(0, 0);
    c.mul_s(2, 2, 0);
    c.cvt_w_s(0, 2);
    c.mfc1(2, 0);
    c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[V1][0] = sx(r32(c, V0).wrapping_add(7));
    if c.g[A1][0] != 0 {
        c.g[V1][0] = c.g[V0][0];
    }
    c.g[V1][0] = sx(((r32(c, V1) as i32) >> 3) as u32);
    c.g[V1][0] = sx(r32(c, V1) << 3);
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
    c.g[V0][0] = sx(r32(c, V0) << 6);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    abi::ret(c);
}

/// `void LightMan_v10(LightMan* man, void* obj, int a2, int a3)` (0x38df38):
/// resolve the object's light hook and run it.
pub fn light_man_v10(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let a3 = c.g[A3][0];
        let a2 = c.g[A2][0];
        c.g[S0][0] = a3;
        frame.save_ra(c, 0);
        c.g[S1][0] = a2;
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(20, 12);
        c.g[A2][0] = Addr(r32_2(c, A0)).lw(c);
        c.g[V1][0] = Addr(r32_2(c, A2)).off(0x10).lh(c);
        c.g[V0][0] = Addr(r32_2(c, A2)).off(0x14).lw(c);
        let target = r32_2(c, V0);
        c.g[A0][0] = abi::addu(c.g[A0][0], c.g[V1][0]);
        call_ptr(c, target, 0x0038_df6c)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0038_dc58, 0x0038_df80)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void LightMan_v11(void* obj, int b, int c, int d, float f)` (0x38df98):
/// look up the light manager's virtual method and hand the arguments on.
pub fn light_man_v11(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x50);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[A3][0];
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x40);
        c.mov_s(20, 12);
        c.g[A2][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[V1][0] = Addr(p_2(c.g[A2][0] as u32, 0x10)).lh(c);
        c.g[V0][0] = Addr(p_2(c.g[A2][0] as u32, 0x14)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = abi::addu(c.g[A0][0], c.g[V1][0]);
        call::call_ptr(c, target, 0x0038_dfd4)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S0][0];
        c.mov_s(12, 20);
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        callee::LIGHT_MAN_EMPTY_HOOK.call(c, 0x0038_dfec)?;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    });
}

/// `void Font_Load(font, name, flags)` (0x00391708): load the font file and
/// parse its glyphs, freeing the file image afterwards.
pub fn font_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[A1][0] = sx(0x490000);
        c.g[A2][0] = c.g[S0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(0x490000u32.wrapping_add(0x2f30));
        c.g[A0][0] = sx(word(c, S1).wrapping_add(0x64));
        callee::SPRINTF.call(c, 0x0039_173c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0] ^ 0x100;
        callee::FILE_LOAD.call(c, 0x0039_1748)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        callee::FONT_PARSE_GLYPHS.call(c, 0x0039_175c)?;
        if c.g[S0][0] == 0 {
            c.lq(16, frame.at(c, 0x30));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::MEM_FREE_2.call(c, 0x0039_176c)?;
            c.lq(16, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Texture swizzling.

/// `int TexMan_SwizzleT8Block(void* unused, u8* src, u8* dst)` (0x38f930):
/// reorder a 16x16 T8 block through the swizzle table at `D_0044BDE0`.
pub fn tex_man_swizzle_t8_block(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x0045_0000;
    c.g[T3][0] = 0;
    c.g[T6][0] = sx(r32(c, V0).wrapping_sub(0x4220));
    c.g[V1][0] = 0;
    'outer: loop {
        c.cycles += 6;
        c.g[V0][0] = c.g[V1][0] & 1;
        c.g[T5][0] = sx(r32(c, V1).wrapping_add(1));
        c.g[T1][0] = sx(r32(c, V0) << 6);
        c.g[V1][0] = 0;
        c.g[T4][0] = sx(r32(c, A1).wrapping_add(0x40));
        c.g[V0][0] = sx(r32(c, T1) << 2);
        'middle: loop {
            c.cycles += 4;
            c.g[T2][0] = sx(r32(c, V1).wrapping_add(1));
            c.g[A3][0] = sx(r32(c, V0).wrapping_add(r32(c, T6)));
            c.g[T0][0] = 0;
            'inner: loop {
                c.cycles += 11;
                c.g[V0][0] = Addr(r32(c, A3)).lw(c);
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, T3)));
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
                c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                c.g[T3][0] = sx(r32(c, T3).wrapping_add(1));
                c.g[V0][0] = (c.g[T0][0] < 4u64) as u64;
                let more = c.g[V0][0] != 0;
                Addr(r32(c, A0)).sb(c, c.g[V1][0]);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0038_f968);
                    }
                    continue 'inner;
                }
                c.cycles += 4;
                c.g[V1][0] = c.g[T2][0];
                c.g[V0][0] = (c.g[V1][0] < 0x10u64) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, T1) << 2);
                if !more {
                    c.cycles += 4;
                    c.g[V1][0] = c.g[T5][0];
                    c.g[V0][0] = (c.g[V1][0] < 4u64) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[T4][0];
                    if !more {
                        c.cycles += 2;
                        c.g[V0][0] = 0;
                        abi::ret(c);
                        return;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0038_f940);
                    }
                    continue 'outer;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0038_f958);
                }
                continue 'middle;
            }
        }
    }
}

/// `void LightMan_v12(obj, a, b, c, d, e, f12, f13, f14)` (0x38e008): call the
/// object's hook with the eight parameters, then the default empty hook.
pub fn light_man_v12(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[T1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A2][0];
        frame.save_fpr(c, 22, 0x60);
        frame.save_fpr(c, 21, 0x58);
        c.mov_s(22, 12);
        frame.save_fpr(c, 20, 0x50);
        c.mov_s(21, 13);
        c.mov_s(20, 14);
        c.g[V1][0] = Addr(abi::arg(c, 0)).lw(c);
        c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
        c.g[V0][0] = lw_2(c, V1, 0x14);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        call_ptr(c, target, 0x0038_e05c)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.mov_s(12, 22);
        c.mov_s(13, 21);
        c.g[T0][0] = c.g[S0][0];
        c.mov_s(14, 20);
        c.g[A0][0] = c.g[V0][0];
        callee::LIGHT_MAN_EMPTY_HOOK_D.call(c, 0x0038_e080)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x60);
        frame.restore_fpr(c, 21, 0x58);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// VU0 microcode upload.

/// `void Vu0_UploadMicrocodeA(void)` (0x3912a8): wait for the VU0 micro
/// program to stop, install the microcode entry point at `0x43bd00`, start it
/// and flush the instruction cache.
pub fn vu0_upload_microcode_a(c: &mut Cpu) {
    guest(c, |c| {
        'first: {
            c.cycles += 8;
            c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x10));
            c.g[V0][0] = sx(0x1000_8000);
            c.sd(r(c, SP), c.g[RA][0]);
            c.g[V1][0] = Addr(r(c, V0)).lw(c);
            c.g[V1][0] = c.g[V1][0] & 0x100;
            let idle = c.g[V1][0] == 0;
            c.g[V1][0] = sx(0x1000_0000);
            if idle {
                break 'first;
            }
            c.cycles += 2;
            c.g[V1][0] = sx(0x1000_8000);
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(r(c, V1)).lw(c);
                c.g[V0][0] = c.g[V0][0] & 0x100;
                let busy = c.g[V0][0] != 0;
                if !busy {
                    break 'first;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_12d0);
                }
            }
        }
        'second: {
            c.cycles += 16;
            c.g[A0][0] = sx(0x1000_8030);
            c.g[V0][0] = sx(0x0043_bd00);
            c.g[V1][0] = sx(0x1000_0000);
            Addr(r(c, A0)).sw(c, c.g[V0][0]);
            c.g[V1][0] = sx(0x1000_8020);
            c.g[A0][0] = sx(0x1000_0000);
            Addr(r(c, V1)).sw(c, 0);
            c.g[A0][0] = sx(0x1000_8000);
            c.g[V1][0] = 0x104;
            Addr(r(c, A0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr(r(c, A0)).lw(c);
            c.g[V0][0] = c.g[V0][0] & 0x100;
            let idle = c.g[V0][0] == 0;
            c.g[V1][0] = sx(0x1000_0000);
            if idle {
                break 'second;
            }
            c.cycles += 1;
            c.g[V1][0] = sx(0x1000_8000);
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(r(c, V1)).lw(c);
                c.g[V0][0] = c.g[V0][0] & 0x100;
                let busy = c.g[V0][0] != 0;
                if !busy {
                    break 'second;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_1330);
                }
            }
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x0039_1354);
        c.g[A0][0] = 0;
        call_ptr(c, 0x0042_4020, 0x0039_1354)?;
        c.g[RA][0] = c.ld(r(c, SP));
        let ra = r(c, RA);
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

/// `void Vu0_UploadMicrocodeB(void)` (0x391360): the second half of the VU0
/// upload — install the entry point at `0x43cca0` and start the program.
pub fn vu0_upload_microcode_b(c: &mut Cpu) {
    guest(c, |c| {
        'first: {
            c.cycles += 8;
            c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x10));
            c.g[V0][0] = sx(0x1000_8000);
            c.sd(r(c, SP), c.g[RA][0]);
            c.g[V1][0] = Addr(r(c, V0)).lw(c);
            c.g[V1][0] = c.g[V1][0] & 0x100;
            let idle = c.g[V1][0] == 0;
            c.g[V1][0] = sx(0x1000_0000);
            if idle {
                break 'first;
            }
            c.cycles += 2;
            c.g[V1][0] = sx(0x1000_8000);
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(r(c, V1)).lw(c);
                c.g[V0][0] = c.g[V0][0] & 0x100;
                let busy = c.g[V0][0] != 0;
                if !busy {
                    break 'first;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_1388);
                }
            }
        }
        'second: {
            c.cycles += 16;
            c.g[A0][0] = sx(0x1000_8030);
            c.g[V0][0] = sx(0x0043_cca0);
            c.g[V1][0] = sx(0x1000_0000);
            Addr(r(c, A0)).sw(c, c.g[V0][0]);
            c.g[V1][0] = sx(0x1000_8020);
            c.g[A0][0] = sx(0x1000_0000);
            Addr(r(c, V1)).sw(c, 0);
            c.g[A0][0] = sx(0x1000_8000);
            c.g[V1][0] = 0x104;
            Addr(r(c, A0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr(r(c, A0)).lw(c);
            c.g[V0][0] = c.g[V0][0] & 0x100;
            let idle = c.g[V0][0] == 0;
            c.g[V1][0] = sx(0x1000_0000);
            if idle {
                break 'second;
            }
            c.cycles += 1;
            c.g[V1][0] = sx(0x1000_8000);
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(r(c, V1)).lw(c);
                c.g[V0][0] = c.g[V0][0] & 0x100;
                let busy = c.g[V0][0] != 0;
                if !busy {
                    break 'second;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_13e8);
                }
            }
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x0039_140c);
        c.g[A0][0] = 0;
        call_ptr(c, 0x0042_4020, 0x0039_140c)?;
        c.g[RA][0] = c.ld(r(c, SP));
        let ra = r(c, RA);
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Font.

/// `Font* Font_ctor(Font* self, char* name, int flags)` (0x3915e8): set the
/// default UV transforms and colours, then load the font.
pub fn font_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 54;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x40));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.sq(S0, p(r32(c, SP), 0x30));
        c.sw(r32(c, SP), 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p(r32(c, SP), 4)).sw(c, 0);
        c.sd(p(r32(c, SP), 0x20), c.g[RA][0]);
        Addr(p(r32(c, S0), 8)).sw(c, 0);
        c.ldl(2, p(r32(c, SP), 7));
        c.ldr(2, r32(c, SP));
        c.sdl(2, p(r32(c, S0), 0x2f));
        c.sdr(2, p(r32(c, S0), 0x28));
        c.swc1(0, r32(c, SP));
        c.swc1(0, p(r32(c, SP), 4));
        c.ldl(2, p(r32(c, SP), 7));
        c.ldr(2, r32(c, SP));
        c.sdl(2, p(r32(c, S0), 0x37));
        c.sdr(2, p(r32(c, S0), 0x30));
        c.swc1(0, p(r32(c, SP), 0x10));
        c.swc1(0, p(r32(c, SP), 0x14));
        c.swc1(0, p(r32(c, SP), 0x18));
        c.swc1(0, p(r32(c, SP), 0x1c));
        c.ldl(2, p(r32(c, S0), 0x37));
        c.ldr(2, p(r32(c, S0), 0x30));
        c.sdl(2, p(r32(c, S0), 0x3f));
        c.sdr(2, p(r32(c, S0), 0x38));
        c.ldl(2, p(r32(c, SP), 0x17));
        c.ldr(2, p(r32(c, SP), 0x10));
        c.ldl(3, p(r32(c, SP), 0x1f));
        c.ldr(3, p(r32(c, SP), 0x18));
        c.sdl(2, p(r32(c, S0), 0x47));
        c.sdr(2, p(r32(c, S0), 0x40));
        c.sdl(3, p(r32(c, S0), 0x4f));
        c.sdr(3, p(r32(c, S0), 0x48));
        c.swc1(0, r32(c, SP));
        Addr(p(r32(c, SP), 4)).sw(c, 0);
        Addr(p(r32(c, SP), 8)).sw(c, 0);
        Addr(p(r32(c, SP), 0xc)).sw(c, 0);
        c.ldl(2, p(r32(c, SP), 7));
        c.ldr(2, r32(c, SP));
        c.ldl(3, p(r32(c, SP), 0xf));
        c.ldr(3, p(r32(c, SP), 8));
        c.sdl(2, p(r32(c, S0), 0x57));
        c.sdr(2, p(r32(c, S0), 0x50));
        c.sdl(3, p(r32(c, S0), 0x5f));
        c.sdr(3, p(r32(c, S0), 0x58));
        Addr(p(r32(c, S0), 0x64)).sb(c, 0);
        call_ptr(c, 0x0039_1708, 0x0039_16ac)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(p(r32(c, SP), 0x20));
        c.lq(S0, p(r32(c, SP), 0x30));
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
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
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::ai::tests::slot as slot_2;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::ai::computer::tests::wb;
    pub(crate) use crate::game::dirtysock::tags::tests::put_str;
    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_4;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::fe::ovstatelodge::tests::slot;
    pub(crate) use crate::game::fe::ovstatelodge::tests::VT;
    pub(crate) use crate::game::input::inputparse::tests::EMPTY;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_2;
    pub(crate) use crate::game::movie::movieplayer::tests::heap_init_2 as heap_init;
    pub(crate) use crate::game::movie::movieplayer::tests::pool_allocator;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2 as run;
    pub(crate) use crate::game::movie::movieplayer::tests::w as w_2;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2 as BE;
    pub(crate) use crate::game::movie::movieplayer::tests::MEM_HEAP;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::render::particle::tests::base;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE as BE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn ps2_graphics_man_reset_color_filter_matches() {
        check_lohi(
            ssx3_ee::s12::f_00390458,
            ps2_graphics_man_reset_color_filter,
            200,
            &|c, seed| {
                let mut r = Rng(0x3904_5800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                for off in [0x14f4, 0x14f8, 0x14fc, 0x1500, 0x1504, 0x1508] {
                    c.bus.write32(GAME_GP + off, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn vfx_screen_tint_trigger_flash_matches() {
        check(
            ssx3_ee::s12::f_00390ec8,
            vfx_screen_tint_trigger_flash,
            200,
            &|c, seed| {
                let mut r = Rng(0x390e_c800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.f[12] = r.edge_f32();
                c.bus.write32(
                    GAME_GP + 0x151c,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
                c.bus.write32(GAME_GP - 0x2698, r.edge_f32());
            },
        );
    }

    #[test]
    fn vfx_screen_tint_poll_flash_sound_matches() {
        check(
            ssx3_ee::s12::f_00390ef8,
            vfx_screen_tint_poll_flash_sound,
            200,
            &|c, seed| {
                let mut r = Rng(0x390e_f800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(
                    GAME_GP + 0x151c,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
                c.bus.write32(
                    GAME_GP + 0x15c4,
                    if seed % 4 == 0 { 0 } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn dma_wait_to_scratchpad_matches() {
        check(
            ee::s12::f_0038f7b0__generated,
            dma_wait_to_scratchpad,
            64,
            &|c, seed| {
                let mut r = Rng(0x38f7_b0 ^ (seed as u64) << 9 | 1);
                scramble(c, &mut r);
                // DMA channel 4 is idle (CHCR bit 8 clear) on a fresh machine; a
                // busy channel spins until the DMAC clears it, which no test
                // machine does, so only the idle path is exercised.
            },
        );
    }

    #[test]
    fn spr_stream_set_mode_matches() {
        driven!(gen_driven, ssx3_ee::s12::f_0038f300__generated);
        driven!(hand_driven, spr_stream_set_mode);
        fn setup(c: &mut Cpu, seed: u32) -> Events {
            let mut r = Rng(0x38f3_00 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let selfp = DATA;
            c.g[A0][0] = selfp as u64;
            let mode = 1 + seed % 8;
            c.g[A1][0] = mode as u64;
            c.bus.write32(selfp, if seed % 5 == 0 { mode } else { 0 });
            put_words(
                c,
                0x49_0000 + 0x2ef0,
                &[
                    0x0038_f344,
                    0x0038_f34c,
                    0x0038_f354,
                    0x0038_f35c,
                    0x0038_f364,
                    0x0038_f36c,
                    0x0038_f36c,
                ],
            );
            put_words(
                c,
                0x49_0000 + 0x2f10,
                &[
                    0x0038_f3a0,
                    0x0038_f3a8,
                    0x0038_f3b0,
                    0x0038_f3b8,
                    0x0038_f3c8,
                    0x0038_f3c0,
                    0x0038_f3c8,
                ],
            );
            schedule(c, &mut r, seed, 0x0038_f300)
        }
        check_events(gen_driven, hand_driven, 60, &setup);
    }

    #[test]
    fn dma_wait_from_scratchpad_matches() {
        check(
            ee::s12::f_0038f768__generated,
            dma_wait_from_scratchpad,
            40,
            &|c, seed| {
                let mut r = Rng(0x9e37_79b9 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn vfx_screen_tint_update_matches() {
        check(
            ssx3_ee::s12::f_00390c60__generated,
            vfx_screen_tint_update,
            300,
            &|c, seed| {
                let mut r = Rng(0x3912_0c60 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let g = GAME_GP;
                // A quiet PRNG state so `Rand_Next` returns immediately.
                for i in 0..0x24u32 {
                    c.bus.write32(0x004f_f018 + i * 4, 0);
                }
                c.bus.write32(g + 0x151c, if seed % 6 == 0 { 0 } else { 1 });
                c.bus.write32(
                    g + 0x15c4,
                    match seed % 3 {
                        0 => 0,
                        1 => 0xffff_ffff,
                        _ => (seed % 20) as u32,
                    },
                );
                // Timer: idle (-1) on some seeds, otherwise inside or past the
                // segment lengths.
                let (d0, d1, d2, d3) = (4u32, 3, 2, 5);
                let timer = match seed % 8 {
                    0 => 0xffff_ffff,
                    1 => 0,
                    2 => 2,
                    3 => 5,
                    4 => 8,
                    5 => 11,
                    6 => 13,
                    _ => (r.next_u32() % 18) as u32,
                };
                c.bus.write32(g + 0x15a8, timer);
                c.bus.write32(
                    g + 0x1520,
                    match seed % 5 {
                        0 => 0xffff_ffff,
                        1 => 0,
                        2 => 20,
                        _ => r.next_u32() & 0x3f,
                    },
                );
                c.bus.write32(g + 0x1524, r.edge_f32());
                c.bus.write32(g + 0x1528, d0);
                c.bus.write32(g + 0x152c, d1);
                c.bus.write32(g + 0x1530, d2);
                c.bus.write32(g + 0x1534, d3);
                for off in [
                    0x1538u32, 0x153c, 0x1540, 0x1544, 0x1548, 0x154c, 0x1550, 0x1554, 0x1558,
                    0x155c, 0x1560, 0x1564, 0x14f4, 0x14f8, 0x14fc,
                ] {
                    c.bus.write32(g + off, r.edge_f32());
                }
                c.bus.write32(g + 0x15ac, r.next_u32());
                c.bus.write32(g + 0x15b0, r.next_u32());
                c.bus.write32(g + 0x15b4, r.next_u32());
                c.bus.write32(g + 0x15b8, r.next_u32());
                c.bus.write32(g + 0x15bc, r.next_u32());
            },
        );
    }

    #[test]
    fn vu0_read_vi1_matches() {
        check(
            ee::s12::f_003914f8__generated,
            vu0_read_vi1,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0039_14f8, seed);
                scramble(c, &mut r);
                c.bus.plat.write_ctrl(1, r.next_u32());
            },
        );
    }

    #[test]
    fn font_dtor_matches() {
        check(ee::s12::f_003916c0__generated, font_dtor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x39_16c0, 0);
            let obj = DATA + 0x1000;
            c.bus
                .write32(obj + 4, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
            c.bus.write32(obj + 0x60, r.next_u32());
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = (seed % 4 == 0) as u64;
        });
    }

    #[test]
    fn light_man_v09_matches() {
        check(
            ee::s12::f_0038dee8__generated,
            light_man_v09,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x38de_e8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let vt = vtable(c, &mut r, &[0x10]);
                clear(c, this, 0x40);
                c.bus.write32(this, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn vfx_plants_get_sway_matrix_matches() {
        check(
            ee::s12::f_0038f2a8__generated,
            vfx_plants_get_sway_matrix,
            SEEDS_2,
            &|c, seed| {
                let mut r = base(c, seed, 0x38f2a8);
                let pos = DATA + 0x100;
                clear(c, DATA, 0x200);
                c.bus.write32(pos, r.edge_f32());
                c.bus.write32(pos + 4, r.edge_f32());
                c.bus.write32(gp_at(0x1368), r.f32(100.0));
                c.g[A0][0] = sx(pos);
            },
        );
    }

    // ------------------------------------------------------------- cameras

    #[test]
    fn light_man_v10_matches() {
        check(
            ssx3_ee::s12::f_0038df38__generated,
            light_man_v10,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x38df38, 0);
                w(c, OBJ, VT);
                slot(c, VT, 0x10, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = seed as u64;
                c.g[A3][0] = OBJ3 as u64;
                c.f[12] = ((seed as f32) * 0.5).to_bits();
            },
        );
    }

    #[test]
    fn light_man_v11_matches() {
        check(
            ssx3_ee::s12::f_0038df98__generated,
            light_man_v11,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x38df98, 0);
                let obj = DATA + 0x1000;
                let vt = DATA + 0x5000;
                c.g[A0][0] = obj as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.f[12] = r.edge_f32();
                w32(c, obj, vt);
                slot_2(c, vt, 0x10, 0, EMPTY);
            },
        );
    }

    /// EAFile's request and file-handle pools (counts `0x519ab0`/`0x519ab4`,
    /// tables `0x519ad8`/`0x519adc`, entries 0x30/0x110 bytes): with no free
    /// entry the allocators spin forever (`0x3de768`).
    pub(crate) fn eafile_pools(c: &mut Cpu) {
        for (count, table, at_3, size) in [
            (0x0051_9ab0, 0x0051_9ad8, 0xf_0000, 0x30),
            (0x0051_9ab4, 0x0051_9adc, 0xf_1000, 0x110),
        ] {
            let base_2 = DATA + at_3;
            c.bus.write32(count, 4);
            c.bus.write32(table, base_2);
            for i in 0..size {
                c.bus.write32(base_2 + 4 * i, 0);
            }
        }
    }

    #[test]
    fn font_load_matches() {
        check(ee::s12::f_00391708__generated, font_load, 8, &|c, seed| {
            let _ = alloc_base(c, seed, 0x3917_08, 8);
            eafile_pools(c);
            let font = DATA;
            let name = DATA + 0x1000;
            put_str(c, name, b"font.x");
            c.g[A0][0] = font as u64;
            c.g[A1][0] = name as u64;
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { 0x100 };
        });
    }

    #[test]
    fn tex_man_swizzle_t8_block_matches() {
        check(
            ee::s12::f_0038f930__generated,
            tex_man_swizzle_t8_block,
            60,
            &|c, seed| {
                let _ = base_3(c, seed, 0x38_f930);
                globals(c);
                for i in 0..0x100u32 {
                    let v = if seed % 2 == 0 {
                        i
                    } else {
                        (seed.wrapping_mul(7) + i) & 0x1ff
                    };
                    w_3(c, 0x0044_bde0 + 4 * i, v);
                    w_3(c, DATA + 0x1000 + 4 * i, i.wrapping_mul(0x0101_0101));
                }
                c.g[A0][0] = 0;
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x2000);
            },
        );
    }

    #[test]
    fn light_man_v12_matches() {
        check(
            ssx3_ee::s12::f_0038e008__generated,
            light_man_v12,
            20,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x38e008, 2);
                let obj = DATA + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                c.g[A3][0] = (DATA + 0x3000) as u64;
                c.g[T0][0] = (DATA + 0x4000) as u64;
                c.g[T1][0] = (DATA + 0x5000) as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                c.f[14] = r.edge_f32();
                w(c, obj, DATA + 0x6000);
                c.bus.write16(DATA + 0x6000 + 0x10, 0);
                w(c, DATA + 0x6000 + 0x14, 0);
            },
        );
    }

    // ------------------------------------------------------------ VU0

    #[test]
    fn vu0_upload_microcode_a_matches() {
        check(
            ee::s12::f_003912a8__generated,
            vu0_upload_microcode_a,
            20,
            &|c, seed| {
                base_5(c, seed, 0x3912, 1);
                w_4(c, 0x1000_8000, 0);
                w_4(c, 0x1000_8020, 0);
                w_4(c, 0x1000_8030, 0);
            },
        );
    }

    #[test]
    fn vu0_upload_microcode_b_matches() {
        check(
            ee::s12::f_00391360__generated,
            vu0_upload_microcode_b,
            20,
            &|c, seed| {
                base_5(c, seed, 0x3913, 1);
                w_4(c, 0x1000_8000, 0);
                w_4(c, 0x1000_8020, 0);
                w_4(c, 0x1000_8030, 0);
            },
        );
    }

    // ------------------------------------------------- screen tint at a rate

    /// A lightning screen tint at rate `hz`: enabled, segments 4/3/2/5
    /// frames, no hold point, a seeded `Rand_Next` state, idle.
    fn tint_machine(hz: u32) -> Box<Cpu> {
        let mut c = crate::difftest::fresh();
        crate::tick::set_for_test(&mut c, hz);
        c.g[GP][0] = GAME_GP as u64;
        let g = GAME_GP;
        let mut r = Rng(0x0039_0c60);
        for i in 0..6u32 {
            c.bus.write32(0x004f_f018 + i * 4, r.next_u32() | 1);
        }
        c.bus.write32(g + 0x151c, 1);
        c.bus.write32(g + 0x15c4, 0);
        c.bus.write32(g + 0x15a8, 0xffff_ffff);
        c.bus.write32(g + 0x1520, 0xffff_ffff);
        c.bus.write32(g + 0x1524, 0);
        for (off, d) in [(0x1528u32, 4u32), (0x152c, 3), (0x1530, 2), (0x1534, 5)] {
            c.bus.write32(g + off, d);
        }
        for (off, v) in [(0x1538u32, 1.0f32), (0x153c, 0.5), (0x1540, 0.75)] {
            c.bus.write32(g + off, v.to_bits());
        }
        c
    }

    fn tint_tick(c: &mut Cpu) {
        c.g[SP][0] = STACK as u64;
        c.g[RA][0] = TEST_RA as u64;
        vfx_screen_tint_update(c);
        assert_eq!(c.pc, TEST_RA, "update did not return");
    }

    /// A flash from its start (timer 0) at rate `hz`: (timer, level) after
    /// each update until the timer is back to idle.
    fn flash_at(hz: u32) -> Vec<(u32, u32)> {
        let mut c = tint_machine(hz);
        c.bus.write32(GAME_GP + 0x15a8, 0);
        let mut out = Vec::new();
        for _ in 0..1000 {
            tint_tick(&mut c);
            let timer = c.bus.read32(GAME_GP + 0x15a8);
            out.push((timer, c.bus.read32(GAME_GP + 0x15ac)));
            if timer == 0xffff_ffff {
                return out;
            }
        }
        panic!("{hz} Hz: the flash never ended");
    }

    /// The flash runs through its four segments in the same wall-clock time
    /// at every rate (its timer steps on a 60 Hz clock): the state after
    /// 60 Hz update i is the state after update (i - 1) * r / 60 + 1 at r.
    #[test]
    fn vfx_screen_tint_flash_follows_the_rate() {
        let at60 = flash_at(60);
        assert_eq!(at60.len(), 15);
        for hz in [120u32, 240] {
            let q = (hz / 60) as usize;
            let at = flash_at(hz);
            assert_eq!(at.len(), (at60.len() - 1) * q + 1, "{hz} Hz: flash length");
            for (i, e) in at60.iter().enumerate() {
                assert_eq!(at[i * q], *e, "{hz} Hz: 60 Hz update {}", i + 1);
            }
        }
    }

    /// The thunder follows the lightning after the same wall-clock delay:
    /// `VfxScreenTint_TriggerFlash` stores r/60 times the 60 Hz tick count.
    #[test]
    fn vfx_screen_tint_thunder_delay_follows_the_rate() {
        let delay = |hz: u32, intensity: f32| {
            let mut c = tint_machine(hz);
            c.bus
                .write32(GAME_GP - 0x2698, 0.001_807_228_9f32.to_bits());
            c.f[12] = intensity.to_bits();
            c.g[RA][0] = TEST_RA as u64;
            vfx_screen_tint_trigger_flash(&mut c);
            assert_eq!(c.pc, TEST_RA);
            c.bus.read32(GAME_GP + 0x15c4)
        };
        for intensity in [2000.0f32, 50_000.0, 199_999.0] {
            let d60 = delay(60, intensity);
            assert!(d60 > 0);
            for hz in [120u32, 240] {
                assert_eq!(delay(hz, intensity), d60 * hz / 60, "{hz} Hz, {intensity}");
            }
        }
    }

    /// Strikes per second while idle (chance 0.2: p = 0.04 per 60 Hz frame)
    /// over 100 s of updates at rate `hz`, forcing the tint back to idle after
    /// each update.
    fn strikes_at(hz: u32) -> u32 {
        let mut c = tint_machine(hz);
        c.bus.write32(GAME_GP + 0x1524, 0.2f32.to_bits());
        let mut n = 0;
        for _ in 0..100 * hz {
            c.bus.write32(GAME_GP + 0x15a8, 0xffff_ffff);
            tint_tick(&mut c);
            if c.bus.read32(GAME_GP + 0x15a8) == 0 {
                n += 1;
            }
        }
        n
    }

    /// The random strike chance is per 60 Hz frame: as many strikes per
    /// second at every rate (about 240 in 100 s; 25% tolerance).
    #[test]
    fn vfx_screen_tint_strike_chance_follows_the_rate() {
        let n60 = strikes_at(60) as f64;
        assert!((170.0..310.0).contains(&n60), "60 Hz: {n60} strikes");
        for hz in [120u32, 240] {
            let n = strikes_at(hz) as f64;
            assert!(
                (n / n60 - 1.0).abs() < 0.25,
                "{hz} Hz: {n} strikes vs {n60} at 60"
            );
        }
    }
}
