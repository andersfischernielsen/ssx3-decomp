//! `src/visualfx/boardtrailfx.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::dirtysock::tags2::gp_3 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_4;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::fe::ovstates::at as at_2;
pub(crate) use crate::game::fe::ovstates::close;
pub(crate) use crate::game::fe::ovstates::open;
pub(crate) use crate::game::render::particle::this_with_delta;
pub(crate) use crate::game::util::menu::gp_5;
pub(crate) use crate::game::visualfx::angerfx::shl32;
pub(crate) use crate::game::visualfx::boardwakefx::addiu;
pub(crate) use crate::game::visualfx::boardwakefx::at;
pub(crate) use crate::game::visualfx::boardwakefx::back_edge;
pub(crate) use crate::game::visualfx::boardwakefx::copy_record;
pub(crate) use crate::game::visualfx::boardwakefx::draw;
pub(crate) use crate::game::visualfx::boardwakefx::fconst;
pub(crate) use crate::game::visualfx::boardwakefx::gfx;
pub(crate) use crate::game::visualfx::boardwakefx::gfx_method;
pub(crate) use crate::game::visualfx::boardwakefx::gp;
pub(crate) use crate::game::visualfx::boardwakefx::gs_field;
pub(crate) use crate::game::visualfx::boardwakefx::p;
pub(crate) use crate::game::visualfx::boardwakefx::pop_record;
pub(crate) use crate::game::visualfx::boardwakefx::DRAW;
pub(crate) use crate::game::visualfx::boardwakefx::GFX_MAN;
pub(crate) use crate::game::visualfx::boardwakefx::GFX_VTABLE;
pub(crate) use crate::game::visualfx::boardwakefx::GS_DEFAULT;
pub(crate) use crate::game::visualfx::boardwakefx::GS_TOP;
pub(crate) use crate::game::visualfx::boardwakefx::RECORD;
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
    (
        0x002e_87e8,
        c_board_trail_fx_is_active,
        "cBoardTrailFX_IsActive",
    ),
    (0x002e_8938, c_board_trail_fx_update, "cBoardTrailFX_Update"),
    (0x002e_a480, c_board_trail_fx_render, "cBoardTrailFX_Render"),
    (
        0x002e_a670,
        c_cut_scene_transition_man_ctor,
        "cCutSceneTransitionMan_ctor",
    ),
    (
        0x002e_a6b8,
        c_cut_scene_transition_man_start_in,
        "cCutSceneTransitionMan_StartIn",
    ),
    (
        0x002e_a780,
        c_cut_scene_transition_man_start_out,
        "cCutSceneTransitionMan_StartOut",
    ),
    (
        0x002e_a820,
        cut_scene_transition_man_force_off,
        "cCutSceneTransitionMan_ForceOff",
    ),
    (
        0x002e_a860,
        c_cut_scene_transition_man_force_on,
        "cCutSceneTransitionMan_ForceOn",
    ),
    (
        0x002e_a8b8,
        c_cut_scene_transition_man_set_paused,
        "cCutSceneTransitionMan_SetPaused",
    ),
    (
        0x002e_aa28,
        ccutscene_transition_man_save_viewport,
        "cCutSceneTransitionMan_SaveViewport",
    ),
    (
        0x002e_adc0,
        rider_base_reset_power_up_trail,
        "RiderBase_ResetPowerUpTrail",
    ),
    (
        0x002e_add0,
        rider_base_update_power_up_trail,
        "RiderBase_UpdatePowerUpTrail",
    ),
    (
        0x002e_b198,
        rider_base_render_power_up_trail,
        "RiderBase_RenderPowerUpTrail",
    ),
    (
        0x002e_bb10,
        transition_effect_draw_fade,
        "TransitionEffect_DrawFade",
    ),
    (0x002e_bba8, transition_wipe_ctor, "TransitionWipe_ctor"),
    (0x002e_bc40, transition_wipe_draw, "TransitionWipe_Draw"),
    (
        0x002e_be20,
        transition_effect_ctor_random2,
        "TransitionEffect_ctorRandom2",
    ),
];

// ---------------------------------------------------------------------------
// RiderBase_RenderPowerUpTrail.

/// `RiderBase` fields used by the power-up trail.
pub(crate) mod trail {

    /// Number of trail sections.
    pub const COUNT: u32 = 0xc;

    /// Ring index of the first section (sections are `index % 3`).
    pub const FIRST: u32 = 0x10;

    /// Ring of 3 sections, 0x40 bytes each: four points from +0x20.
    pub const POINTS: u32 = 0x20;

    /// The two points of the trail's head.
    pub const HEAD_A: u32 = 0xe0;

    pub const HEAD_B: u32 = 0xf0;
}

/// Stack layout of the trail: four vertex arrays of 7 vertices (0x150
/// bytes) from sp+0, their counts at sp+0x540.., the colour at sp+0x550
/// (alpha, r, g, b).
pub(crate) mod trail_sp {

    pub const ARRAY: u32 = 0x150;

    pub const COUNTS: u32 = 0x540;

    pub const ALPHA: u32 = 0x550;

    pub const R: u32 = 0x554;

    pub const G: u32 = 0x558;

    pub const B: u32 = 0x55c;
}

// ---------------------------------------------------------------------------
// Rider power-up trail.

/// `RiderBase` trail state used by the update.
pub(crate) mod power_trail {

    /// The rider.
    pub const RIDER: u32 = 0x00;

    /// Number of live trail points (0..3).
    pub const COUNT: u32 = 0x0c;

    /// Ring slot being written.
    pub const CURSOR: u32 = 0x10;

    /// Ring of 0x40-byte slots at `cursor * 0x40` (four vertices each).
    pub const SLOTS: u32 = 0x20;

    /// The two 12-unit-wide edges of the current cross-section.
    pub const EDGE0: u32 = 0xe0;

    pub const EDGE1: u32 = 0xf0;
}

/// `Rider` fields read here.
pub(crate) mod pt_rider {

    pub const SPEED: u32 = 0x2ec;

    pub const MODEL: u32 = 0x77c;

    /// Index of the board bone in the skeleton.
    pub const BONE: u32 = 0x8a4;

    pub const SKELETON: u32 = 0x780;

    /// The skeleton's array of 0x40-byte node transforms.
    pub const NODES: u32 = 0x30;
}

// ---------------------------------------------------------------------------
// Board-trail wake effect.

/// `cBoardTrailFX` (a wake tail attached to a rider) field offsets.
pub(crate) mod board_trail {

    /// The rider (entity) the effect follows (`*(self)`).
    pub const OWNER: u32 = 0;

    /// The rider's animation controller.
    pub const ANIM: u32 = 0x784;

    /// The rider's state object.
    pub const STATE: u32 = 0x77c;

    /// The board-trail intensity the `switch` writes.
    pub const INTENSITY: u32 = 0x9c;

    /// Rider main state.
    pub const MAIN_STATE: u32 = 0x438;
}

// ---------------------------------------------------------------------------
// Callees the generator emitted as direct `jal`s (one shard-local alias each).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2 as MC_MANAGER_GET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::render::particle::callee::ALGORITHM_LIST_DTOR;
    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::AVA_SPLINE_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INSTANCE as BELIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INTERFACE as BELIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::render::particle::callee::BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::render::particle::callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_COLLECTIBLE_COLLECTED;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR as CHASE_CAMERA_ALGORITHM_CTOR_2;
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
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS;
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
    pub(crate) use crate::game::world::wscriptcache::callee::DELETE;
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

    /// `cBoardTrailFX_IsActive(self)` (0x2e87e8).
    pub const IS_ACTIVE: Callee = Callee {
        addr: 0x002e_87e8,
        f: ee::s09::cBoardTrailFX_IsActive,
    };

    /// `cBoardTrailFX_ShouldEmit(self)` (0x2e86f0).
    pub const SHOULD_EMIT: Callee = Callee {
        addr: 0x002e_86f0,
        f: ee::s09::cBoardTrailFX_ShouldEmit,
    };
}

// ---------------------------------------------------------------------------
// Cut-scene transitions.

/// `void cCutSceneTransitionMan_StartOut(cCutSceneTransitionMan *this,
/// float speed)` (0x2ea780): kick off a transition depending on the state at
/// +0x10 (0 idle, 1 in, 2 out, 3 pending), writing the phase and rates.
pub fn c_cut_scene_transition_man_start_out(c: &mut Cpu) {
    let this = abi::arg(c, 0);
    'b_2ea814: {
        'b_2ea80c: {
            'b_2ea7e8: {
                'b_2ea7cc: {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(this).off(0x10).lw(c);
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                    if t {
                        c.cycles += 3;
                        c.swc1(12, this.wrapping_add(0x18));
                        c.sw(this.wrapping_add(0x14), 0);
                        break 'b_2ea80c;
                    } else {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            c.cycles += 2;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = 0x3;
                            if t {
                                break 'b_2ea7cc;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_2ea7e8;
                                } else {
                                    c.cycles += 2;
                                    Addr(this).off(0x10).sw(c, c.g[V0][0]);
                                    abi::ret(c);
                                    return;
                                }
                            }
                        } else {
                            c.cycles += 2;
                            let t = c.g[V1][0] == 0u64;
                            if t {
                                break 'b_2ea814;
                            } else {
                                c.cycles += 2;
                                Addr(this).off(0x10).sw(c, c.g[V0][0]);
                                abi::ret(c);
                                return;
                            }
                        }
                    }
                }
                c.cycles += 7;
                c.lwc1(0, this.wrapping_add(0x14));
                c.lwc1(1, this.wrapping_add(0x18));
                c.swc1(12, this.wrapping_add(0x18));
                c.div_s(0, 0, 1);
                c.mul_s(0, 0, 12);
                c.swc1(0, this.wrapping_add(0x14));
                break 'b_2ea80c;
            }
            c.cycles += 9;
            c.lwc1(0, this.wrapping_add(0x14));
            c.lwc1(2, this.wrapping_add(0x18));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 1); // f1 = 1.0
            c.div_s(0, 0, 2);
            c.swc1(12, this.wrapping_add(0x18));
            c.sub_s(1, 1, 0);
            c.mul_s(1, 1, 12);
            c.swc1(1, this.wrapping_add(0x14));
            break 'b_2ea80c;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x2;
        Addr(this).off(0x10).sw(c, c.g[V0][0]);
        break 'b_2ea814;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void cCutSceneTransitionMan_SetPaused(cCutSceneTransitionMan *this,
/// int paused)` (0x2ea8b8): while fading (state 2/3), bump or drop the hold
/// counter at +0x1c.
pub fn c_cut_scene_transition_man_set_paused(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x10).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(2));
    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        let paused = c.g[A1][0];
        c.g[V0][0] = Addr(this).off(0x1c).lw(c);
        if paused == 0 {
            c.cycles += 2;
            let positive = (c.g[V0][0] as i64) > 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            if positive {
                c.cycles += 1;
                Addr(this).off(0x1c).sw(c, c.g[V0][0]);
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            Addr(this).off(0x1c).sw(c, c.g[V0][0]);
            abi::ret(c);
            return;
        }
    } else {
        c.cycles += 2;
        Addr(this).off(0x1c).sw(c, 0);
        abi::ret(c);
        return;
    }
}

/// `void RiderBase_ResetPowerUpTrail(Rider *rider)` (0x2eadc0): clear the
/// power-up trail state.
pub fn rider_base_reset_power_up_trail(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    Addr(this).off(0x10).sw(c, 0);
    Addr(this).off(0xc).sw(c, 0);
    abi::ret(c);
}

/// `void RiderBase_RenderPowerUpTrail(RiderBase *r)` (0x2eb198): with at
/// least two trail sections, push a GS state (texture `gfx+0x1048`, alpha
/// blend), build four triangle strips (one per edge of the trail's
/// cross-section: the head points `+0xe0`/`+0xf0`, then for each section
/// `i` the points of ring slot `(first + i) % 3` with `v = 1 - i/(n-1)`)
/// and draw them; pop the state.
///
/// Frame 0x5b0: s0-s3 at +0x5a0..+0x570, ra at +0x560.
pub fn rider_base_render_power_up_trail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x5b0);
        frame.save(c, S0, 0x5a0);
        frame.save(c, S1, 0x590);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x580);
        frame.save(c, S3, 0x570);
        frame.save_ra(c, 0x560);
        let n = c.lw(at(c.g[S0][0], trail::COUNT));
        c.g[V0][0] = ((n as i64) < 2) as u64;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x5a0);
        } else {
            trail_draw(c, frame)?;
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x590);
        frame.restore(c, S2, 0x580);
        frame.restore(c, S3, 0x570);
        frame.restore_ra(c, 0x560);
        frame.close(c);
        Ok(())
    })
}

/// The trail's drawing (from the GS state push to the pop and the `s0`
/// reload).
pub(crate) fn trail_draw(c: &mut Cpu, frame: Frame) -> Flow {
    use trail_sp::*;
    let sp = r32(c, SP);
    c.cycles += 95;
    // Push: copy the top record up, load the default into the new top.
    let g = gfx(c);
    c.g[V1][0] = g;
    c.g[A1][0] = 0x50_0000;
    c.g[A2][0] = 0xffff_ffff_fe7f_ffff;
    c.g[T0][0] = 0xffff_ffff_ffcf_ffff;
    let top = c.lw(at(g, GS_TOP));
    c.g[V0][0] = top;
    c.g[T1][0] = 0xffff_ffff_fff0_0fff;
    copy_record(c, top as u32, at(top, RECORD), A0, A3, T2);
    c.g[A3][0] = 0x1_4000;
    c.g[T4][0] = 0xffff_ffff_ffff_fffc;
    let top = c.lw(at(g, GS_TOP));
    c.g[A0][0] = gfx(c);
    c.g[T2][0] = 0xffff_ffff_e000_03ff;
    c.g[V0][0] = sx(at(top, RECORD));
    let new_top = c.g[V0][0];
    c.sw(at(g, GS_TOP), new_top);
    c.g[T5][0] = 0xffff_ffff_ffff_fc1f;
    c.g[T6][0] = 0xffff_ffff_ffff_ff83;
    c.g[T3][0] = 0xffff_ffff_ffbf_ffff;
    let top = c.lw(at(c.g[A0][0], GS_TOP));
    c.g[V0][0] = top;
    c.g[T7][0] = 0x40_0000;
    c.g[S1][0] = 3;
    c.g[A0][0] = sx(GS_DEFAULT);
    copy_record(c, GS_DEFAULT, top as u32, S2, S3, V1);
    c.g[T8][0] = sx(sp.wrapping_add(COUNTS));
    c.g[S2][0] = u64::MAX;
    // Texture and state bits.
    let g = gfx(c);
    c.g[V1][0] = g;
    let top = c.lw(at(g, GS_TOP));
    c.g[A0][0] = top;
    let tex = c.lw(at(g, 0x1048));
    c.g[V0][0] = tex;
    c.sh(at(top, 0x10), tex);
    let g = gfx(c);
    c.g[A1][0] = g;
    gs_field(c, g, V1, 0, u64::MAX, 0xc);
    gs_field(c, g, A0, 4, 0xffff_ffff_fe7f_ffff, 0);
    gs_field(c, g, V1, 4, 0xffff_ffff_ffcf_ffff, 0);
    gs_field(c, g, A0, 4, 0xffff_ffff_fff0_0fff, 0x1_4000);
    gs_field(c, g, A2, 4, 0xffff_ffff_ffff_fffc, 2);
    gs_field(c, g, V1, 8, 0xffff_ffff_e000_03ff, 0);
    gs_field(c, g, A0, 8, 0xffff_ffff_ffff_fc1f, 0xe0);
    gs_field(c, g, V1, 4, 0xffff_ffff_ffff_ff83, 0x1c);
    gs_field(c, g, A0, 4, 0xffff_ffff_ffbf_ffff, 0x40_0000);
    // gcc's empty constructor loops over the 4 x 7 vertices.
    c.g[V1][0] = sx(r32(c, S1).wrapping_sub(1));
    loop {
        c.cycles += 2;
        c.g[V0][0] = 6;
        loop {
            c.cycles += 7;
            addiu(c, V0, V0, u32::MAX);
            let more = c.g[V0][0] != c.g[S2][0];
            c.g[S1][0] = c.g[V1][0];
            if !more {
                break;
            }
            back_edge(c, 0x002e_b348);
        }
        c.cycles += 2;
        let more = c.g[S1][0] != c.g[S2][0];
        addiu(c, V1, S1, u32::MAX);
        if !more {
            break;
        }
        back_edge(c, 0x002e_b340);
    }
    // Zero the four counts.
    c.cycles += 3;
    c.g[V1][0] = 3;
    addiu(c, V0, T8, 0xc);
    loop {
        c.cycles += 7;
        let a = r32(c, V0);
        c.sw(a, 0);
        addiu(c, V1, V1, u32::MAX);
        let more = (c.g[V1][0] as i64) >= 0;
        addiu(c, V0, V0, (-4i32) as u32);
        if !more {
            break;
        }
        back_edge(c, 0x002e_b378);
    }
    // The head: one vertex each into strips 1 and 3 (colour 1.0, 255).
    c.cycles += 68;
    c.g[V0][0] = c.lw(sp.wrapping_add(COUNTS + 4));
    c.g[A0][0] = 0x30;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 2);
    c.g[A1][0] = 0xff;
    c.mult(0, A2, V0, A0);
    c.g[AT][0] = 0x437f_0000;
    c.mtc1(AT, 3);
    addiu(c, V0, V0, 1);
    for off in [ALPHA, G, B, R] {
        c.swc1(2, sp.wrapping_add(off));
    }
    c.g[V1][0] = sx(r32(c, A2).wrapping_add(sp));
    let v0 = c.g[V0][0];
    c.sw(sp.wrapping_add(COUNTS + 4), v0);
    addiu(c, V1, V1, ARRAY);
    let v = r32(c, V1);
    let a1 = c.g[A1][0];
    c.sw(v.wrapping_add(0x10), a1);
    let this = c.g[S0][0];
    c.lq(A1, at(this, trail::HEAD_A));
    c.lwc1(1, sp.wrapping_add(G));
    c.lq(V0, at(this, trail::HEAD_B));
    c.mul_s(1, 1, 3);
    c.g[A3][0] = c.lw(at(this, trail::COUNT));
    c.g[AT][0] = 0x4300_0000;
    c.mtc1(AT, 5);
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 4);
    c.cvt_w_s(0, 1);
    c.swc1(0, v.wrapping_add(0x14));
    c.lwc1(0, sp.wrapping_add(B));
    c.mul_s(0, 0, 3);
    c.cvt_w_s(1, 0);
    c.swc1(1, v.wrapping_add(0x18));
    c.lwc1(1, sp.wrapping_add(ALPHA));
    c.sq(V0, v.wrapping_add(0x20));
    c.mul_s(1, 1, 5);
    c.sw(v.wrapping_add(4), 0);
    c.swc1(2, v.wrapping_add(8));
    c.swc1(4, v);
    c.cvt_w_s(0, 1);
    c.swc1(0, v.wrapping_add(0x1c));
    c.g[V0][0] = c.lw(sp.wrapping_add(COUNTS + 0xc));
    c.lwc1(1, sp.wrapping_add(R));
    c.mult(0, V1, V0, A0);
    c.mul_s(1, 1, 3);
    addiu(c, V0, V0, 1);
    let v0 = c.g[V0][0];
    c.sw(sp.wrapping_add(COUNTS + 0xc), v0);
    c.g[A0][0] = sx(r32(c, V1).wrapping_add(sp));
    addiu(c, A0, A0, 3 * ARRAY);
    let v = r32(c, A0);
    trail_color(c, sp, v);
    c.sq(A1, v.wrapping_add(0x20));
    c.sw(v.wrapping_add(4), 0);
    c.swc1(2, v.wrapping_add(8));
    c.swc1(4, v);
    let none = (c.g[A3][0] as i64) <= 0;
    c.g[T4][0] = 0;
    if !none {
        c.cycles += 5;
        c.mov_s(6, 4);
        c.g[T5][0] = 3;
        c.mov_s(4, 2);
        c.g[T3][0] = 0x30;
        loop {
            trail_section(c, sp);
            let more = c.g[A0][0] != 0;
            if !more {
                break;
            }
            back_edge(c, 0x002e_b4b8);
        }
    }
    // Draw the four strips.
    c.cycles += 4;
    c.g[S1][0] = c.g[T8][0];
    c.g[S0][0] = c.g[SP][0];
    c.g[S2][0] = 3;
    c.g[T0][0] = gfx(c);
    loop {
        c.cycles += 13;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = c.lw(r32(c, S1));
        c.g[A3][0] = 0;
        addiu(c, S1, S1, 4);
        addiu(c, S0, S0, ARRAY);
        addiu(c, S2, S2, u32::MAX);
        draw(c, T0, V1, V0, 0x002e_b8bc)?;
        let more = (c.g[S2][0] as i64) >= 0;
        c.g[T0][0] = gfx(c);
        if !more {
            break;
        }
        back_edge(c, 0x002e_b890);
    }
    c.cycles += 5;
    pop_record(c, V1);
    frame.restore(c, S0, 0x5a0);
    Ok(())
}

/// The trail colour of vertex `v` from sp+0x550.. (alpha * f5 = 128,
/// rgb * f3 = 255, as integers at v+0x10..0x1c), r already scaled in f1.
pub(crate) fn trail_color(c: &mut Cpu, sp: u32, v: u32) {
    use trail_sp::*;
    c.cvt_w_s(0, 1);
    c.swc1(0, v.wrapping_add(0x10));
    c.lwc1(0, sp.wrapping_add(G));
    c.mul_s(0, 0, 3);
    c.cvt_w_s(1, 0);
    c.swc1(1, v.wrapping_add(0x14));
    c.lwc1(1, sp.wrapping_add(B));
    c.mul_s(1, 1, 3);
    c.cvt_w_s(0, 1);
    c.swc1(0, v.wrapping_add(0x18));
    c.lwc1(0, sp.wrapping_add(ALPHA));
    c.mul_s(0, 0, 5);
    c.cvt_w_s(1, 0);
    c.swc1(1, v.wrapping_add(0x1c));
}

/// Append one vertex to strip `strip` (count at sp+0x540+4*strip, bumped;
/// the offset through `rd`): colour, `v` (f2) at +4, the point at `src` (via
/// v0) at +0x20, f4 at +8 and, when `one`, at +0 (else 0). v1 = the vertex.
pub(crate) fn trail_vertex(c: &mut Cpu, sp: u32, strip: u32, rd: usize, src: u32, one: bool) {
    use trail_sp::*;
    let count = sp.wrapping_add(COUNTS + 4 * strip);
    c.g[V0][0] = c.lw(count);
    c.lwc1(1, sp.wrapping_add(R));
    c.mult(0, rd, V0, T3);
    c.mul_s(1, 1, 3);
    addiu(c, V0, V0, 1);
    let n = c.g[V0][0];
    c.sw(count, n);
    let v = r32(c, rd).wrapping_add(sp).wrapping_add(strip * ARRAY);
    c.g[V1][0] = sx(v);
    trail_color(c, sp, v);
    c.swc1(4, v.wrapping_add(8));
    c.lq(V0, src);
    c.swc1(2, v.wrapping_add(4));
    c.sq(V0, v.wrapping_add(0x20));
    if one {
        c.swc1(4, v);
    } else {
        c.sw(v, 0);
    }
}

/// One trail section (loop body at 0x2eb4b8): section `i = t4` of ring
/// slot `(first + i) % 3`, two vertices per strip. Leaves a0 = `i+1 < n`.
pub(crate) fn trail_section(c: &mut Cpu, sp: u32) {
    use trail_sp::*;
    c.cycles += 3;
    c.g[V0][0] = c.lw(at(c.g[S0][0], trail::FIRST));
    c.cycles += 239;
    // Strip 0 gets the section's first vertex: alpha 0x7f, v = 1 - i/(n-1).
    c.g[V1][0] = c.lw(sp.wrapping_add(COUNTS));
    c.g[A1][0] = 0x7f;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T4)));
    c.swc1(6, sp.wrapping_add(ALPHA));
    c.div(0, V0, T5);
    c.swc1(6, sp.wrapping_add(G));
    c.swc1(6, sp.wrapping_add(B));
    c.mtc1(T4, 2);
    c.cvt_s_w(2, 2);
    c.swc1(6, sp.wrapping_add(R));
    addiu(c, T4, T4, 1);
    c.g[A2][0] = c.hi[0];
    c.mult(0, V0, V1, T3);
    c.g[A2][0] = sx(r32(c, A2) << 6);
    addiu(c, V1, V1, 1);
    let n = c.g[V1][0];
    c.sw(sp.wrapping_add(COUNTS), n);
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(sp));
    let v = r32(c, A0);
    let a1 = c.g[A1][0];
    c.sw(v.wrapping_add(0x10), a1);
    addiu(c, V0, A3, u32::MAX);
    c.mtc1(V0, 1);
    c.cvt_s_w(1, 1);
    c.lwc1(0, sp.wrapping_add(G));
    c.mul_s(0, 0, 3);
    c.div_s(2, 2, 1);
    c.cvt_w_s(1, 0);
    c.swc1(1, v.wrapping_add(0x14));
    c.lwc1(0, sp.wrapping_add(B));
    c.sub_s(2, 4, 2);
    c.mul_s(0, 0, 3);
    c.cvt_w_s(1, 0);
    c.swc1(1, v.wrapping_add(0x18));
    c.lwc1(1, sp.wrapping_add(ALPHA));
    c.swc1(4, v.wrapping_add(8));
    c.mul_s(1, 1, 5);
    c.cvt_w_s(0, 1);
    c.swc1(0, v.wrapping_add(0x1c));
    let section = sx(r32(c, A2).wrapping_add(r32(c, S0)));
    for r in [V0, A3, A1] {
        c.g[r][0] = section;
    }
    // The section's four points.
    let p = (section as u32).wrapping_add(trail::POINTS);
    let point = |k: u32| p.wrapping_add(0x10 * k);
    c.lq(V1, point(0));
    c.swc1(2, v.wrapping_add(4));
    c.sq(V1, v.wrapping_add(0x20));
    c.sw(v, 0);
    for r in [T2, T0, T1, A2, A0] {
        c.g[r][0] = section;
    }
    // The rest of the section: (strip, point) pairs.
    trail_vertex(c, sp, 0, T6, point(1), true);
    trail_vertex(c, sp, 1, A1, point(1), true);
    trail_vertex(c, sp, 1, A1, point(2), false);
    trail_vertex(c, sp, 2, A1, point(2), false);
    trail_vertex(c, sp, 2, A1, point(3), true);
    trail_vertex(c, sp, 3, A1, point(3), true);
    c.g[A3][0] = c.lw(at(c.g[S0][0], trail::COUNT));
    c.g[A0][0] = ((c.g[T4][0] as i64) < (c.g[A3][0] as i64)) as u64;
    trail_vertex(c, sp, 3, A1, point(0), false);
}

/// `mfc1 rt, f; lqc2 vf4, v; qmtc2 rt, vf3; vmulx vf5, vf4, vf3x; sqc2 vf5,
/// out; lq r2, out; sq r2, store`: scale vector `v` by FPU register `f` and
/// leave a copy at `store`.
#[inline(always)]
pub(crate) fn scale_store(c: &mut Cpu, f: usize, v: u32, out: u32, store: u32) {
    c.mfc1(2, f);
    c.lqc2(4, v);
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, out);
    c.lq(2, out);
    c.sq(2, store);
}

/// `vadd` plus the `lq`/`sq` copy to `store`.
#[inline(always)]
pub(crate) fn add_store(c: &mut Cpu, a: u32, b: u32, out: u32, store: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
    c.lq(2, out);
    c.sq(2, store);
}

/// `vsub` plus the `lq`/`sq` copy to `store`.
#[inline(always)]
pub(crate) fn sub_store(c: &mut Cpu, a: u32, b: u32, out: u32, store: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
    c.lq(2, out);
    c.sq(2, store);
}

/// `void RiderBase_UpdatePowerUpTrail(RiderBase* self)` (0x2eadd0): while the
/// rider is fast (or the debug override is set) keep a four-point trail
/// cross-section around the board: the two 12-unit side edges and four ring
/// slots (70 units forward/back ± 18 units sideways) rebuilt from the board
/// bone's frame each frame; otherwise clear the trail.
pub fn rider_base_update_power_up_trail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0xc0);
        c.g[V0][0] = c.lw(gp(c, 0x1630));
        frame.save(c, S0, 0xa0);
        frame.save_ra(c, 0x90);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0xb8);
        frame.save_fpr(c, 20, 0xb0);
        let self_at = r32(c, S0);
        let sp0 = sp(c);
        c.g[V1][0] = c.lw(self_at);
        let hidden = c.g[V0][0] == 0;
        c.lwc1(20, p(r32(c, V1), pt_rider::SPEED));
        if !hidden {
            c.cycles += 2;
            fconst(c, 20, 0x4120_0000); // 10
        }
        // `zero < speed` (the zero comes from the flag register, a
        // denormal after `mtc1`, which the PS2 compares as 0).
        c.cycles += 5;
        c.mtc1(0, 21);
        c.c_lt_s(21, 20);
        let no_speed = !c.fcc();
        c.g[A1][0] = 1;
        if !no_speed {
            c.cycles += 22;
            c.g[A0][0] = c.lw(p(r32(c, V1), pt_rider::MODEL));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xd30));
            call_ptr(c, 0x0039_2de8, 0x002e_ae20)?;
            // Draw the glow: colour (0, 0, 1, 0), size 2.
            fconst(c, 0, 0x4000_0000); // 2
            c.g[A1][0] = sx(p(sp0, 0x10));
            c.g[V0][0] = c.lw(self_at);
            c.g[A2][0] = sx(sp0);
            c.swc1(0, p(sp0, 8));
            c.swc1(0, sp0);
            c.swc1(0, p(sp0, 4));
            fconst(c, 1, 0x3f80_0000); // 1
            c.g[A0][0] = c.lw(p(r32(c, V0), pt_rider::MODEL));
            c.swc1(1, p(sp0, 0x18));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xd30));
            c.swc1(21, p(sp0, 0x10));
            c.swc1(21, p(sp0, 0x14));
            c.swc1(21, p(sp0, 0x1c));
            call_ptr(c, 0x0039_2d90, 0x002e_ae64)?;
            c.c_lt_s(21, 20);
        }
        'out: {
            c.cycles += 2;
            if !c.fcc() {
                c.sw(p(self_at, power_trail::COUNT), 0);
                break 'out;
            }
            c.cycles += 5;
            c.g[A0][0] = c.lw(self_at);
            call_ptr(c, 0x0011_fe98, 0x002e_ae7c)?;
            c.g[V1][0] = 1;
            if c.g[V0][0] != c.g[V1][0] {
                c.sw(p(self_at, power_trail::COUNT), 0);
                break 'out;
            }
            // Advance the ring and rebuild the cross-section.
            c.cycles += 4;
            c.g[V1][0] = c.lw(p(self_at, power_trail::CURSOR));
            c.g[V0][0] = 3;
            c.cycles += 186;
            c.g[A1][0] = c.g[V0][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(2));
            c.g[A0][0] = c.lw(p(self_at, power_trail::COUNT));
            c.div(0, 3, 2); // (cursor + 2) % 3
            c.g[A2][0] = c.lw(self_at);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
            fconst(c, 1, 0x4140_0000); // 12
            c.g[V0][0] = ((c.g[A0][0] as i64) < 4) as u64;
            fconst(c, 0, 0x428c_0000); // 70
            if c.g[V0][0] != 0 {
                c.g[A1][0] = c.g[A0][0];
            }
            fconst(c, 2, 0x4190_0000); // 18
            c.sw(p(self_at, power_trail::COUNT), c.g[A1][0]);
            c.g[A3][0] = c.hi[0];
            c.sw(p(self_at, power_trail::CURSOR), c.g[A3][0]);
            // node = skeleton->nodes[bone]
            c.g[V0][0] = c.lw(p(r32(c, A2), pt_rider::SKELETON));
            c.g[V1][0] = c.lw(p(r32(c, A2), pt_rider::BONE));
            c.g[A0][0] = c.lw(p(r32(c, V0), pt_rider::NODES));
            c.g[V1][0] = shl32(c.g[V1][0], 6);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            c.lq(2, p(r32(c, A0), 0x30));
            c.sq(2, sp0);
            c.g[A1][0] = c.lw(p(r32(c, A2), pt_rider::SKELETON));
            c.g[V1][0] = c.lw(p(r32(c, A2), pt_rider::BONE));
            c.g[A0][0] = c.lw(p(r32(c, A1), pt_rider::NODES));
            c.g[V1][0] = shl32(c.g[V1][0], 6);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            c.lq(2, r32(c, A0));
            c.sq(2, p(sp0, 0x20));
            c.g[A1][0] = c.lw(p(r32(c, A2), pt_rider::SKELETON));
            c.g[V1][0] = c.lw(p(r32(c, A2), pt_rider::BONE));
            c.g[A0][0] = c.lw(p(r32(c, A1), pt_rider::NODES));
            c.g[V1][0] = shl32(c.g[V1][0], 6);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            c.lq(2, p(r32(c, A0), 0x20));
            c.sq(2, p(sp0, 0x30));
            // edge0 = pos - side * 12
            scale_store(c, 1, p(sp0, 0x30), p(sp0, 0x60), p(sp0, 0x50));
            sub_store(
                c,
                sp0,
                p(sp0, 0x50),
                p(sp0, 0x60),
                p(self_at, power_trail::EDGE0),
            );
            c.sq(2, p(sp0, 0x40));
            // edge1 = pos + side * 12
            scale_store(c, 1, p(sp0, 0x30), p(sp0, 0x60), p(sp0, 0x50));
            add_store(
                c,
                sp0,
                p(sp0, 0x50),
                p(sp0, 0x60),
                p(self_at, power_trail::EDGE1),
            );
            c.sq(2, p(sp0, 0x40));
            // slot v0 = pos + fwd * 70 - side * 18
            scale_store(c, 0, p(sp0, 0x20), p(sp0, 0x70), p(sp0, 0x60));
            add_store(c, sp0, p(sp0, 0x60), p(sp0, 0x70), p(sp0, 0x50));
            scale_store(c, 2, p(sp0, 0x30), p(sp0, 0x80), p(sp0, 0x70));
            sub_store(c, p(sp0, 0x50), p(sp0, 0x70), p(sp0, 0x80), p(sp0, 0x40));
            c.g[A0][0] = c.lw(p(self_at, power_trail::CURSOR));
            c.g[A0][0] = shl32(c.g[A0][0], 6);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(self_at));
            c.sq(2, p(r32(c, A0), power_trail::SLOTS));
            // slot v1 = pos + fwd * 70 + side * 18
            scale_store(c, 0, p(sp0, 0x20), p(sp0, 0x70), p(sp0, 0x60));
            add_store(c, sp0, p(sp0, 0x60), p(sp0, 0x70), p(sp0, 0x50));
            scale_store(c, 2, p(sp0, 0x30), p(sp0, 0x80), p(sp0, 0x70));
            add_store(c, p(sp0, 0x50), p(sp0, 0x70), p(sp0, 0x80), p(sp0, 0x40));
            c.g[V1][0] = c.lw(p(self_at, power_trail::CURSOR));
            c.g[V1][0] = shl32(c.g[V1][0], 6);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(self_at));
            c.sq(2, p(r32(c, V1), power_trail::SLOTS + 0x10));
            // slot v2 = pos - fwd * 70 + side * 18
            scale_store(c, 0, p(sp0, 0x20), p(sp0, 0x70), p(sp0, 0x60));
            sub_store(c, sp0, p(sp0, 0x60), p(sp0, 0x70), p(sp0, 0x50));
            scale_store(c, 2, p(sp0, 0x30), p(sp0, 0x80), p(sp0, 0x70));
            add_store(c, p(sp0, 0x50), p(sp0, 0x70), p(sp0, 0x80), p(sp0, 0x40));
            c.g[A0][0] = c.lw(p(self_at, power_trail::CURSOR));
            c.g[A0][0] = shl32(c.g[A0][0], 6);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(self_at));
            c.sq(2, p(r32(c, A0), power_trail::SLOTS + 0x20));
            // slot v3 = pos - fwd * 70 - side * 18
            scale_store(c, 0, p(sp0, 0x20), p(sp0, 0x70), p(sp0, 0x60));
            sub_store(c, sp0, p(sp0, 0x60), p(sp0, 0x70), p(sp0, 0x50));
            scale_store(c, 2, p(sp0, 0x30), p(sp0, 0x80), p(sp0, 0x70));
            sub_store(c, p(sp0, 0x50), p(sp0, 0x70), p(sp0, 0x80), p(sp0, 0x40));
            c.g[V1][0] = c.lw(p(self_at, power_trail::CURSOR));
            c.g[V1][0] = shl32(c.g[V1][0], 6);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(self_at));
            c.sq(2, p(r32(c, V1), power_trail::SLOTS + 0x30));
            break 'out;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.restore_fpr(c, 21, 0xb8);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    });
}

/// `sll` on a register value (`sx` of the 32-bit result, as the generator
/// writes it).
#[inline(always)]
pub(crate) fn shl32_2(v: u64, n: u32) -> u64 {
    sx((v as u32) << n)
}

/// 32-bit wrapping add of two register values.
#[inline(always)]
pub(crate) fn add32(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// The blocks of `cBoardTrailFX_IsActive`'s state `switch` (its `.rodata`
/// jump table sits at `0x480000 + 0x7b40`).
pub(crate) const CBFX_ENTRY: u32 = 0x002e_87e8;

pub(crate) const CBFX_END: u32 = 0x002e_8934;

pub(crate) const CBFX_TABLE_OFF: u32 = 0x7b40;

pub(crate) const CBFX_GET_ANIM_ID: u32 = 0x0031_2aa0;

pub(crate) const CBFX_GET_SEQUENCE: u32 = 0x0031_1b20;

pub(crate) const CBFX_TEST_FLAGS: u32 = 0x0014_46a0;

/// `bool cBoardTrailFX_IsActive(cBoardTrailFX* self)` (0x2e87e8): whether the
/// wake tail should draw this frame — the rider must be in a drawable state
/// (or hidden behind the state's flag), playing the board-trail animation
/// sequence, and the function stores the per-state trail intensity into
/// `self+0x9c` when it is active.
pub fn c_board_trail_fx_is_active(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x20 };
        board_trail_fx_at(c, frame, c.pc)
    })
}

pub(crate) fn board_trail_fx_at(c: &mut Cpu, frame: Frame, pc: u32) -> Flow {
    match pc {
        CBFX_ENTRY => cbfx_entry(c, frame),
        0x002e_880c => cbfx_state_two(c, frame),
        0x002e_8820 => {
            cbfx_pop(c, frame);
            Ok(())
        }
        0x002e_8828 => cbfx_ask_active(c, frame),
        0x002e_8838 => cbfx_can_draw(c, frame),
        0x002e_8848 => cbfx_can_draw_sub(c, frame),
        0x002e_8854 => cbfx_can_draw_flag(c, frame),
        0x002e_885c => cbfx_a1_known(c, frame),
        0x002e_8864 => cbfx_b00_check(c, frame),
        0x002e_8870 => cbfx_anim_check(c, frame),
        0x002e_888c => cbfx_sequence(c, frame),
        0x002e_88ac => cbfx_sequence_second(c, frame),
        0x002e_88cc => {
            c.cycles += 1;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            cbfx_switch(c, frame)
        }
        0x002e_88d0 => cbfx_switch(c, frame),
        0x002e_88e0 => cbfx_table(c, frame),
        0x002e_88f8 => cbfx_case_f8(c, frame),
        0x002e_8900 => cbfx_case_00(c, frame),
        0x002e_8908 => cbfx_case_08(c, frame),
        0x002e_8910 => cbfx_case_10(c, frame),
        0x002e_8914 => {
            cbfx_store_active(c, frame);
            Ok(())
        }
        0x002e_8920 => {
            cbfx_return_false(c, frame);
            Ok(())
        }
        0x002e_8924 => {
            cbfx_return(c, frame);
            Ok(())
        }
        0x002e_8928 => {
            cbfx_exit(c, frame);
            Ok(())
        }
        other => {
            c.jump_out(other, CBFX_ENTRY, CBFX_END);
            Ok(())
        }
    }
}

/// 0x2e87e8: the entry.
pub(crate) fn cbfx_entry(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 9;
    Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    let fx = abi::arg(c, 0);
    c.g[S0][0] = sx(fx);
    c.g[A0][0] = Addr(p(fx, board_trail::OWNER)).lw(c);
    call_ptr(c, 0x0011_fe98, 0x002e_8800)?;
    c.g[V1][0] = 2;
    if c.g[V0][0] != c.g[V1][0] {
        cbfx_ask_active(c, frame)
    } else {
        cbfx_state_two(c, frame)
    }
}

/// 0x2e880c: state 2: inactive when the state's hidden flag is set.
pub(crate) fn cbfx_state_two(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
    c.g[V1][0] = Addr(p(r32(c, V0), board_trail::STATE)).lw(c);
    c.g[A0][0] = Addr(p(r32(c, V1), 0x30)).lw(c);
    let hidden = c.g[A0][0] != 0;
    c.g[V0][0] = 0;
    if hidden {
        cbfx_pop(c, frame);
        return Ok(());
    }
    cbfx_anim_check(c, frame)
}

/// 0x2e8820: pop `s0` and return with the caller's `v0` (0).
pub(crate) fn cbfx_pop(c: &mut Cpu, frame: Frame) {
    c.cycles += 2;
    frame.restore(c, S0, 0x10);
    cbfx_exit(c, frame);
}

/// 0x2e8828: the rider is in some other state: state 0 draws, others need
/// the drawable check.
pub(crate) fn cbfx_ask_active(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = Addr(r32(c, S0)).lw(c);
    call_ptr(c, 0x0011_fe98, 0x002e_8830)?;
    let first_state = c.g[V0][0] == 0;
    c.g[A1][0] = 0;
    if first_state {
        cbfx_anim_check(c, frame)
    } else {
        cbfx_can_draw(c, frame)
    }
}

/// 0x2e8838: `boardTrailCanDraw` inlined (the rider's spray link).
pub(crate) fn cbfx_can_draw(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
    c.g[V0][0] = Addr(p(r32(c, V1), 0xac4)).lw(c);
    let link = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[V1][0];
    if link {
        cbfx_a1_known(c, frame)
    } else {
        cbfx_can_draw_sub(c, frame)
    }
}

/// 0x2e8848.
pub(crate) fn cbfx_can_draw_sub(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = Addr(p(r32(c, A0), 0xad0)).lw(c);
    if c.g[V0][0] != 0 {
        cbfx_a1_known(c, frame)
    } else {
        cbfx_can_draw_flag(c, frame)
    }
}

/// 0x2e8854.
pub(crate) fn cbfx_can_draw_flag(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    c.g[V0][0] = Addr(p(r32(c, A0), 0xafc)).lw(c);
    c.g[A1][0] = (0 < c.g[V0][0]) as u64;
    cbfx_a1_known(c, frame)
}

/// 0x2e885c: `a1` says whether the rider is in a spray-tracked state.
pub(crate) fn cbfx_a1_known(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    let not_tracked = c.g[A1][0] == 0;
    c.g[V0][0] = 0;
    if not_tracked {
        cbfx_return(c, frame);
        return Ok(());
    }
    cbfx_b00_check(c, frame)
}

/// 0x2e8864.
pub(crate) fn cbfx_b00_check(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = Addr(p(r32(c, A0), 0xb00)).lw(c);
    let empty = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if empty {
        cbfx_return(c, frame);
        return Ok(());
    }
    cbfx_anim_check(c, frame)
}

/// 0x2e8870: is the rider playing the board-trail animation (0x16)?
pub(crate) fn cbfx_anim_check(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
    c.g[A1][0] = 2;
    c.g[A0][0] = Addr(p(r32(c, V0), board_trail::ANIM)).lw(c);
    call_ptr(c, CBFX_GET_ANIM_ID, 0x002e_8880)?;
    c.g[V1][0] = 0x16;
    let other_anim = c.g[V0][0] != c.g[V1][0];
    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
    if other_anim {
        cbfx_switch(c, frame)
    } else {
        cbfx_sequence(c, frame)
    }
}

/// 0x2e888c: the trail sequence's first flag.
pub(crate) fn cbfx_sequence(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 8;
    c.g[A1][0] = 2;
    c.g[A0][0] = Addr(p(r32(c, V0), board_trail::ANIM)).lw(c);
    call_ptr(c, CBFX_GET_SEQUENCE, 0x002e_8898)?;
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(0xb0));
    c.g[A1][0] = 0;
    call_ptr(c, CBFX_TEST_FLAGS, 0x002e_88a4)?;
    let unset = c.g[V0][0] == 0;
    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
    if unset {
        cbfx_switch(c, frame)
    } else {
        cbfx_sequence_second(c, frame)
    }
}

/// 0x2e88ac: the trail sequence's second flag.
pub(crate) fn cbfx_sequence_second(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 8;
    c.g[A1][0] = 2;
    c.g[A0][0] = Addr(p(r32(c, V0), board_trail::ANIM)).lw(c);
    call_ptr(c, CBFX_GET_SEQUENCE, 0x002e_88b8)?;
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(0xb0));
    c.g[A1][0] = 1;
    call_ptr(c, CBFX_TEST_FLAGS, 0x002e_88c4)?;
    let unset = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if unset {
        cbfx_return(c, frame);
        return Ok(());
    }
    c.cycles += 1;
    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
    cbfx_switch(c, frame)
}

/// 0x2e88d0: the per-state trail intensity `switch` (state < 14).
pub(crate) fn cbfx_switch(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = Addr(p(r32(c, V0), board_trail::MAIN_STATE)).lw(c);
    c.g[V1][0] = (c.g[A0][0] < 0xe) as u64;
    let in_table = c.g[V1][0] != 0;
    c.g[V0][0] = 0x0048_0000;
    if !in_table {
        cbfx_return_false(c, frame);
        return Ok(());
    }
    cbfx_table(c, frame)
}

/// 0x2e88e0: the jump-table read; only the switch's own case blocks are
/// accepted (any other target is an indirect jump out of the function).
pub(crate) fn cbfx_table(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[V1][0] = shl32_2(c.g[A0][0], 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(CBFX_TABLE_OFF));
    c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    match r32(c, A0) {
        0x002e_88f8 => cbfx_case_f8(c, frame),
        0x002e_8900 => cbfx_case_00(c, frame),
        0x002e_8908 => cbfx_case_08(c, frame),
        0x002e_8910 => cbfx_case_10(c, frame),
        0x002e_8920 => {
            cbfx_return_false(c, frame);
            Ok(())
        }
        t => {
            c.jump_out(t, CBFX_ENTRY, CBFX_END);
            Ok(())
        }
    }
}

/// 0x2e88f8.
pub(crate) fn cbfx_case_f8(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    c.lwc1(0, gp(c, 0x14a4));
    cbfx_store_active(c, frame);
    Ok(())
}

/// 0x2e8900.
pub(crate) fn cbfx_case_00(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    c.lwc1(0, gp(c, 0x14a0));
    cbfx_store_active(c, frame);
    Ok(())
}

/// 0x2e8908.
pub(crate) fn cbfx_case_08(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    c.lwc1(0, gp(c, 0x14a8));
    cbfx_store_active(c, frame);
    Ok(())
}

/// 0x2e8910.
pub(crate) fn cbfx_case_10(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.lwc1(0, gp(c, 0x14ac));
    cbfx_store_active(c, frame);
    Ok(())
}

/// 0x2e8914: store the intensity and report active.
pub(crate) fn cbfx_store_active(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    c.g[V0][0] = 1;
    c.swc1(0, p(r32(c, S0), board_trail::INTENSITY));
    cbfx_return(c, frame);
}

/// 0x2e8920: report inactive.
pub(crate) fn cbfx_return_false(c: &mut Cpu, frame: Frame) {
    c.cycles += 1;
    c.g[V0][0] = 0;
    cbfx_return(c, frame);
}

/// 0x2e8924: pop `s0`.
pub(crate) fn cbfx_return(c: &mut Cpu, frame: Frame) {
    c.cycles += 1;
    frame.restore(c, S0, 0x10);
    cbfx_exit(c, frame);
}

/// 0x2e8928: the epilogue.
pub(crate) fn cbfx_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Visual effects, memory manager, input, animation, render.

/// `void cCutSceneTransitionMan_ForceOff(cCutSceneTransitionMan* self)`
/// (0x2EA820): end a running cut-scene transition at once (virtual call with
/// this+0x20 as the argument) and clear the active flag.
pub fn cut_scene_transition_man_force_off(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        let fading = c.lw_r_2(V0, (this as u32).wrapping_add(0x10));
        c.g[A1][0] = sx((this as u32).wrapping_add(0x20));
        if fading != 0 {
            c.cycles += 7;
            Addr((this as u32).wrapping_add(0x10)).sw(c, 0);
            let manager = c.lw_r_2(A2, gp_2(c, 0x2a90));
            let vt = c.lw_r_2(V0, (manager as u32).wrapping_add(0x10d8));
            let delta = c.lh_r(A0, (vt as u32).wrapping_add(0x148));
            let target = c.lw_r_2(V1, (vt as u32).wrapping_add(0x14c));
            let t = target as u32;
            let a0 = sx((manager as u32).wrapping_add(delta as u32));
            c.g[A0][0] = a0;
            call_ptr(c, t, 0x002e_a850)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cCutSceneTransitionMan_ctor(cCutSceneTransitionMan* m)` (0x2ea670):
/// the cut-scene transition manager: base process ctor, vtable 0x487f30,
/// `+0x10 = +0x1c = 0`, then `SaveViewport`. Returns `m`.
pub fn c_cut_scene_transition_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CUT_SCENE_TRANSITION_MAN_BASE_CTOR.call(c, 0x002e_a684)?;
        c.g[V1][0] = 0x480000;
        c.sw(r32(c, S0).wrapping_add(0x10), 0);
        c.g[V1][0] = sx(0x0048_0000u32.wrapping_add(0x7f30));
        c.sw(r32(c, S0).wrapping_add(0x1c), 0);
        c.sw(r32(c, S0).wrapping_add(0xc), c.g[V1][0]);
        c.g[A0][0] = c.g[S0][0];
        callee::CUT_SCENE_TRANSITION_MAN_INIT.call(c, 0x002e_a6a0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cCutSceneTransitionMan_ForceOn(Manager* self)` (0x2ea860): force the
/// transition manager on, saving the viewport once and telling the app's
/// viewport slot to take over.
pub fn c_cut_scene_transition_man_force_on(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(this).off(0x10).lw(c);
        let already_on = c.g[V0][0] != 0;
        c.g[A2][0] = Addr(gp(c, 0x2a90)).lw(c);
        if !already_on {
            c.cycles += 3;
            c.g[RA][0] = sx(0x002e_a884);
            callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT.call(c, 0x002e_a884)?;
            c.g[A2][0] = Addr(gp(c, 0x2a90)).lw(c);
        }
        c.cycles += 12;
        c.g[V0][0] = 1;
        Addr(this).off(0x10).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(this.wrapping_add(0x34));
        c.g[V0][0] = Addr(r32(c, A2)).off(0x10d8).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x148).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x14c).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x002e_a8a8);
        c.g[A0][0] = this_with_delta(c.g[A2][0], c.g[A0][0]);
        call_ptr(c, target, 0x002e_a8a8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `TransitionWipe_ctor` (0x2ebba8): construct the 16-byte parameter struct
/// passed by value (`a1`) into TransitionEffect's constructor, then install
/// the wipe vtable and store the two trailing words.
pub fn transition_wipe_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        let arg = r(c, A1);
        c.ldl(2, p(arg, 7));
        c.ldr(2, arg);
        c.ldl(3, p(arg, 0xf));
        c.ldr(3, p(arg, 8));
        c.sdl(2, p(r(c, SP), 7));
        c.sdr(2, r(c, SP));
        c.sdl(3, p(r(c, SP), 0xf));
        c.g[RA][0] = sx(0x002e_bbec);
        c.sdr(3, p(r(c, SP), 8));
        callee::TRANSITION_EFFECT_CTOR.call(c, 0x002e_bbec)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[V0][0] = sx(this);
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_sub(0x7f60));
        frame.restore_ra(c, 0x10);
        Addr(this).sw(c, c.g[V1][0]);
        let sp = r(c, SP);
        c.ldl(3, p(sp, 7));
        c.ldr(3, sp);
        c.ldl(4, p(sp, 0xf));
        c.ldr(4, p(sp, 8));
        c.sdl(3, p(this, 0xb));
        c.sdr(3, p(this, 4));
        c.sdl(4, p(this, 0x13));
        c.sdr(4, p(this, 0xc));
        Addr(p(this, 0x14)).sw(c, c.g[S1][0]);
        Addr(p(this, 0x18)).sw(c, c.g[S2][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `TransitionEffect_DrawFade` (0x2ebb10): draw the fade rectangle (the
/// effect's 16-byte colour quad, its alpha scaled by `f12`).
pub fn transition_effect_draw_fade(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x40);
        let this = r(c, A0);
        c.g[V0][0] = sx(this);
        frame.save_ra(c, 0x30);
        c.g[A0][0] = sx(r(c, SP).wrapping_add(0x10));
        c.g[AT][0] = 0x4420_0000;
        c.mtc1(1, 6);
        c.lwc1(0, p(this, 4));
        c.ldl(3, p(this, 0xb));
        c.ldr(3, p(this, 4));
        c.ldl(5, p(this, 0x13));
        c.ldr(5, p(this, 0xc));
        c.sdl(3, p(r(c, SP), 7));
        c.sdr(3, r(c, SP));
        c.sdl(5, p(r(c, SP), 0xf));
        c.sdr(5, p(r(c, SP), 8));
        c.mul_s(0, 0, 12);
        Addr(p(r(c, SP), 0x10)).sw(c, 0);
        c.g[AT][0] = 0x43f0_0000;
        c.mtc1(1, 5);
        c.g[A1][0] = sx(r(c, SP).wrapping_add(0x20));
        c.lwc1(1, gp_3(c, 0xa40));
        c.g[A2][0] = c.g[SP][0];
        c.lwc1(2, gp_3(c, 0xa4c));
        c.lwc1(3, gp_3(c, 0xa48));
        c.lwc1(4, gp_3(c, 0xa54));
        c.lwc1(12, p(r(c, SP), 0x10));
        c.swc1(0, r(c, SP));
        c.swc1(6, p(r(c, SP), 0x18));
        c.swc1(5, p(r(c, SP), 0x1c));
        c.swc1(1, p(r(c, SP), 0x20));
        c.swc1(2, p(r(c, SP), 0x24));
        c.swc1(3, p(r(c, SP), 0x28));
        c.swc1(4, p(r(c, SP), 0x2c));
        c.g[RA][0] = sx(0x002e_bb9c);
        Addr(p(r(c, SP), 0x14)).sw(c, 0);
        callee::TRANSITION_EFFECT_DRAW_RECT.call(c, 0x002e_bb9c)?;
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Board trail FX.

/// `void cBoardTrailFX_Render(cBoardTrailFX* fx)` (0x2ea480): flush the trail's
/// queued nodes when the effect is enabled and the current render pass owns
/// it, otherwise drop the oldest entry when the queue is at least two deep.
pub fn c_board_trail_fx_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        'outer: {
            'mid: {
                frame.save(c, S0, 0x10);
                c.g[V0][0] = Addr(gp_4(c, 0x1428)).lw(c);
                let this = r(c, A0);
                frame.save_ra(c, 0);
                let enabled = c.g[V0][0] != 0;
                c.g[S0][0] = sx(this);
                if enabled {
                    break 'mid;
                }
                'inner: {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(gp_4(c, 0x142c)).lw(c);
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(p(r(c, S0), 0x90)).lw(c);
                        break 'inner;
                    }
                    c.cycles += 9;
                    c.g[A0][0] = Addr(r(c, S0)).lw(c);
                    c.g[V1][0] = Addr(p(r(c, A0), 0x6c0)).lw(c);
                    c.g[A0][0] = sx(p(r(c, A0), 0x6c0));
                    c.g[A1][0] = Addr(p(r(c, V1), 0x48)).lh(c);
                    c.g[V0][0] = Addr(p(r(c, V1), 0x4c)).lw(c);
                    let target = r(c, V0);
                    c.g[RA][0] = sx(0x002e_a4c0);
                    c.g[A0][0] = sx(p(r(c, A0), r(c, A1)));
                    call_ptr(c, target, 0x002e_a4c0)?;
                    if c.g[V0][0] != 0 {
                        frame.restore(c, 16, 0x10);
                        break 'outer;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(r(c, S0), 0x90)).lw(c);
                    break 'inner;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
                let few = c.g[V0][0] != 0;
                frame.restore(c, 16, 0x10);
                if few {
                    break 'outer;
                }
                // Push the oldest node: shift 0x14 bytes of the node up by
                // 0x14 and drop its last word into the new slot.
                c.cycles += 19;
                c.g[A0][0] = Addr(gp_4(c, -0x854)).lw(c);
                c.g[V0][0] = Addr(p(r(c, A0), 0xe84)).lw(c);
                c.ldl(3, p(r(c, V0), 7));
                c.ldr(3, r(c, V0));
                c.ldl(5, p(r(c, V0), 0xf));
                c.ldr(5, p(r(c, V0), 8));
                c.g[A2][0] = Addr(p(r(c, V0), 0x10)).lw(c);
                c.sdl(3, p(r(c, V0), 0x1b));
                c.sdr(3, p(r(c, V0), 0x14));
                c.sdl(5, p(r(c, V0), 0x23));
                c.sdr(5, p(r(c, V0), 0x1c));
                c.sw(p(r(c, V0), 0x24), c.g[A2][0]);
                c.g[V1][0] = Addr(p(r(c, A0), 0xe84)).lw(c);
                c.g[A1][0] = Addr(gp_4(c, -0x854)).lw(c);
                c.g[V1][0] = sx(r(c, V1).wrapping_add(0x14));
                c.sw(p(r(c, A0), 0xe84), c.g[V1][0]);
                c.g[V0][0] = Addr(p(r(c, A1), 0xe84)).lw(c);
                c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x14));
                c.sw(p(r(c, A1), 0xe84), c.g[V0][0]);
                break 'mid;
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

/// `void cCutSceneTransitionMan_SaveViewport(cCutSceneTransitionMan* self)`
/// (0x2eaa28): snapshot the current viewport (16-byte quad + word at_2 +0x10)
/// and the fade constants into +0x20.., choosing the "snap" variant when the
/// saved mode is 2.
pub fn ccutscene_transition_man_save_viewport(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        open(c, 0x20);
        c.g[A1][0] = Addr(c.g[GP][0] as u32).off(0x2a90).lw(c);
        c.sq(16, at_2(c.g[SP][0] as u32, 0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x10d8).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x150).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x154).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002e_aa50)?;
        c.ldl(4, at_2(c.g[V0][0] as u32, 7));
        c.ldr(4, c.g[V0][0] as u32);
        c.ldl(5, at_2(c.g[V0][0] as u32, 0xf));
        c.ldr(5, at_2(c.g[V0][0] as u32, 8));
        c.g[A2][0] = Addr(c.g[V0][0] as u32).off(0x10).lw(c);
        c.sdl(4, at_2(c.g[S0][0] as u32, 0x27));
        c.sdr(4, at_2(c.g[S0][0] as u32, 0x20));
        c.sdl(5, at_2(c.g[S0][0] as u32, 0x2f));
        c.sdr(5, at_2(c.g[S0][0] as u32, 0x28));
        Addr(c.g[S0][0] as u32).off(0x30).sw(c, c.g[A2][0]);
        c.g[V1][0] = 2;
        c.g[AT][0] = 0x3f40_0000;
        c.mtc1(1, 1);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x20).lw(c);
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x39a8));
        if c.g[V0][0] != c.g[V1][0] {
            c.g[V0][0] = 1;
            c.cycles += 7;
            c.g[AT][0] = 0x3e00_0000;
            c.mtc1(1, 0);
            c.swc1(2, at_2(c.g[S0][0] as u32, 0x44));
            Addr(c.g[S0][0] as u32).off(0x34).sw(c, c.g[V0][0]);
            c.swc1(0, at_2(c.g[S0][0] as u32, 0x38));
            c.swc1(1, at_2(c.g[S0][0] as u32, 0x40));
            c.swc1(1, at_2(c.g[S0][0] as u32, 0x3c));
        } else {
            c.cycles += 8;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.swc1(2, at_2(c.g[S0][0] as u32, 0x44));
            Addr(c.g[S0][0] as u32).off(0x34).sw(c, c.g[V0][0]);
            c.swc1(0, at_2(c.g[S0][0] as u32, 0x3c));
            c.swc1(1, at_2(c.g[S0][0] as u32, 0x40));
            Addr(c.g[S0][0] as u32).off(0x38).sw(c, 0);
        }
        c.cycles += 4;
        c.lq(16, at_2(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x20);
        Ok(())
    })
}

/// `cCutSceneTransitionMan_StartIn(this, dur)` (0x2ea6b8): fade a cut-scene
/// transition in — save the viewport for the first stage and rescale the
/// remaining time by the new duration for the others.
pub fn c_cut_scene_transition_man_start_in(c: &mut Cpu) {
    'b_2ea768: {
        c.cycles += 9;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = 0x1;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        let t = c.g[V1][0] == c.g[V0][0];
        c.mov_s(20, 12);
        if t {
            break 'b_2ea768;
        } else {
            'b_2ea760: {
                'b_2ea74c: {
                    'b_2ea724: {
                        'b_2ea710: {
                            'b_2ea6f8: {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_2ea6f8;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] == 0u64;
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_2ea710;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                        if t {
                                            break 'b_2ea768;
                                        } else {
                                            break 'b_2ea6f8;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = 0x3;
                            if t {
                                break 'b_2ea724;
                            } else {
                                c.cycles += 2;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x14));
                                    break 'b_2ea74c;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                    if t {
                                        break 'b_2ea768;
                                    } else {
                                        break 'b_2ea710;
                                    }
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x002ea718);
                        c.g[A0][0] = c.g[S0][0];
                        c.pc = 0x002eaa28;
                        ee::s09::cCutSceneTransitionMan_SaveViewport(c);
                        if c.pc != 0x002ea718 {
                            return;
                        }
                        c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x18));
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
                        if t {
                            break 'b_2ea760;
                        } else {
                            break 'b_2ea724;
                        }
                    }
                    c.cycles += 10;
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x14));
                    c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 1);
                    c.div_s(0, 0, 2);
                    c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x18));
                    c.sub_s(1, 1, 0);
                    c.mul_s(1, 1, 20);
                    let t = true;
                    c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x14));
                    if t {
                        break 'b_2ea760;
                    } else {
                        break 'b_2ea74c;
                    }
                }
                c.cycles += 5;
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x18));
                c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x18));
                c.div_s(0, 0, 1);
                c.mul_s(0, 0, 20);
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x14));
                break 'b_2ea760;
            }
            c.cycles += 2;
            c.g[V0][0] = 0x3;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            break 'b_2ea768;
        }
    }
    c.cycles += 5;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t;
    return;
}

// ---------------------------------------------------------------------------
// 0x002ebe20 TransitionEffect_ctorRandom2
// ---------------------------------------------------------------------------

/// `void* TransitionEffect_ctorRandom2(TransitionEffect* self, const Vec4* v)`
/// (0x2ebe20).
///
/// Constructs the base effect, overrides its vtable, copies the four-float
/// vector argument into `+4` and stores two random durations (0..1000 ms) at
/// `+0x14` and `+0x18`. `v0` is the new object.
pub fn transition_effect_ctor_random2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.ldl(2, p(c.g[A1][0] as u32, 7));
        c.ldr(2, c.g[A1][0] as u32);
        c.ldl(3, p(c.g[A1][0] as u32, 0xf));
        c.ldr(3, p(c.g[A1][0] as u32, 8));
        c.sdl(2, p(c.g[SP][0] as u32, 7));
        c.sdr(2, c.g[SP][0] as u32);
        c.sdl(3, p(c.g[SP][0] as u32, 0xf));
        c.sdr(3, p(c.g[SP][0] as u32, 8));
        c.g[RA][0] = sx(0x002e_be58);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002e_4d70, 0x002e_be58)?; // TransitionEffect_ctor
        c.g[V1][0] = 0x0049_0000;
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_sub(0x7f80));
        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
        c.ldl(2, p(c.g[SP][0] as u32, 7));
        c.ldr(2, c.g[SP][0] as u32);
        c.ldl(3, p(c.g[SP][0] as u32, 0xf));
        c.ldr(3, p(c.g[SP][0] as u32, 8));
        c.sdl(2, p(c.g[S0][0] as u32, 0xb));
        c.sdr(2, p(c.g[S0][0] as u32, 4));
        c.sdl(3, p(c.g[S0][0] as u32, 0x13));
        c.sdr(3, p(c.g[S0][0] as u32, 0xc));
        c.g[RA][0] = sx(0x002e_be8c);
        call_ptr(c, 0x0031_77f0, 0x002e_be8c)?; // BXrand
        c.g[V1][0] = 0x3e8;
        c.g[A0][0] = sx(c.g[S0][0] as u32 + 0x14);
        c.divu(0, 2, 3);

        // 1000 * rand() / 0x7fff, as a signed conversion.
        'ms: {
            'conv: {
                c.cycles += 3;
                c.g[V0][0] = c.hi[0];
                let t = (c.g[V0][0] as i64) < 0;
                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                if t {
                    break 'conv;
                }
                c.cycles += 4;
                c.mtc1(2, 1);
                c.cvt_s_w(1, 1);
                let t = true;
                if t {
                    break 'ms;
                }
                break 'conv;
            }
            c.cycles += 5;
            c.g[V0][0] = c.g[V0][0] & 1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
            break 'ms;
        }
        'second: {
            c.cycles += 9;
            c.lwc1(0, gp_5(c, -0x3994));
            c.mul_s(0, 1, 0);
            c.g[RA][0] = sx(0x002e_bee0);
            c.swc1(0, c.g[A0][0] as u32);
            call_ptr(c, 0x0031_77f0, 0x002e_bee0)?; // BXrand
            c.g[V1][0] = 0x3e8;
            c.g[A0][0] = sx(c.g[S0][0] as u32 + 0x18);
            c.divu(0, 2, 3);
            'ms2: {
                'conv2: {
                    c.cycles += 3;
                    c.g[V0][0] = c.hi[0];
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                    if t {
                        break 'conv2;
                    }
                    c.cycles += 4;
                    c.mtc1(2, 1);
                    c.cvt_s_w(1, 1);
                    let t = true;
                    if t {
                        break 'ms2;
                    }
                    break 'conv2;
                }
                c.cycles += 5;
                c.g[V0][0] = c.g[V0][0] & 1;
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.mtc1(2, 1);
                c.cvt_s_w(1, 1);
                c.add_s(1, 1, 1);
                break 'ms2;
            }
            c.cycles += 8;
            c.lwc1(0, gp_5(c, -0x3990));
            c.g[V0][0] = c.g[S0][0];
            c.lq(16, p(c.g[SP][0] as u32, 0x20));
            c.mul_s(0, 1, 0);
            c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
            c.swc1(0, c.g[A0][0] as u32);
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            c.pc = t;
            Ok(())
        }
    })
}

// ---------------------------------------------------------------------------
// Register and address helpers.

/// The low 32 bits of a GPR (what the generated code's `as u32` reads).
#[inline(always)]
pub(crate) fn g(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// A `gp_6`-relative global.
#[inline(always)]
pub(crate) fn gp_6(c: &Cpu, off: i32) -> u32 {
    g(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// cBoardTrailFX_Update.

/// `void cBoardTrailFX_Update(cBoardTrailFX* self)` (0x2e8938): advance the
/// board trail of one rider by a frame (see the module comment).
pub fn c_board_trail_fx_update(c: &mut Cpu) {
    guest(c, body)
}

/// The body, split out so `?` on the call protocol has a `Flow` to return.
pub(crate) fn body(c: &mut Cpu) -> Flow {
    let frame = Frame::open(c, 0x2b0);
    'b_2e8980: {
        c.cycles += 18;
        c.sq(16, frame.at(c, 0x260));
        c.sq(17, frame.at(c, 0x250));
        c.g[S0][0] = c.g[A0][0];
        c.sq(19, frame.at(c, 0x230));
        c.g[S1][0] = 0x36;
        c.sq(18, frame.at(c, 0x240));
        c.sd(frame.at(c, 0x220), c.g[RA][0]);
        c.swc1(27, frame.at(c, 0x2a8));
        c.swc1(26, frame.at(c, 0x2a0));
        c.swc1(25, frame.at(c, 0x298));
        c.swc1(24, frame.at(c, 0x290));
        c.swc1(23, frame.at(c, 0x288));
        c.swc1(22, frame.at(c, 0x280));
        c.swc1(21, frame.at(c, 0x278));
        c.swc1(20, frame.at(c, 0x270));
        if c.g[S1][0] == 0u64 {
            break 'b_2e8980;
        } else {
            break 'b_2e8980;
        }
    }
    // The update proper. `s0` holds the trail object, `s1` is 54 (vertices
    // per strip) and `s3` the vertex index the emit path will write.
    'b_2ea440: {
        'b_2e8b38: {
            c.cycles += 8;
            c.g[V0][0] = c.lw(p(g(c, S0), 0x8c));
            c.g[V0][0] = sx(p(g(c, V0), 0x35));
            c.div(0, 2, 17);
            c.g[V1][0] = c.hi[0];
            c.g[S3][0] = c.g[V1][0];
            callee::IS_ACTIVE.call(c, 0x002e_8998)?;
            if c.g[V0][0] != 0u64 {
                c.g[V0][0] = c.lw(p(g(c, S0), 0x94));
                break 'b_2e8b38;
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.lw(p(g(c, S0), 0x94));
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_2ea440;
                } else {
                    'b_2e8b30: {
                        'b_2e89f4: {
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A1][0] = 0x30;
                            if t {
                                break 'b_2e89f4;
                            } else {
                                'b_2e89c4: {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                    c.g[V0][0] = 0x36;
                                    if c.g[V0][0] == 0u64 {
                                        break 'b_2e89c4;
                                    } else {
                                        break 'b_2e89c4;
                                    }
                                }
                                c.cycles += 12;
                                c.g[A1][0] = 0x36;
                                c.g[V1][0] = sx(p(g(c, V1), 0x33));
                                c.g[A0][0] = 0xffffffffffffffff;
                                c.div(0, 3, 17);
                                c.g[V0][0] = c.hi[0];
                                c.sw(p(g(c, S0), 0x8c), c.g[V0][0]);
                                c.g[V0][0] = c.lw(p(g(c, S0), 0x90));
                                c.g[V0][0] = sx(g(c, V0).wrapping_sub(0x3));
                                c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[A0][0] == 0 {
                                    c.g[V0][0] = 0u64;
                                }
                                let t = true;
                                c.sw(p(g(c, S0), 0x90), c.g[V0][0]);
                                if t {
                                    break 'b_2e8b30;
                                } else {
                                    break 'b_2e89f4;
                                }
                            }
                        }
                        c.cycles += 31;
                        c.g[V0][0] = c.lw(p(g(c, S0), 0x10));
                        c.mult(0, 5, 19, 5);
                        c.g[A0][0] = c.lw(p(g(c, S0), 0x1c));
                        c.g[A2][0] = c.lw(p(g(c, S0), 0x18));
                        c.g[T1][0] = 0x30;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.g[A3][0] = sx(p(g(c, S0), 0x10));
                        c.g[T2][0] = 0x5;
                        c.g[V0][0] = sx(g(c, A1).wrapping_add(g(c, V0)));
                        c.g[A0][0] = sx(g(c, A1).wrapping_add(g(c, A0)));
                        c.lq(3, p(g(c, V0), 0x20));
                        c.g[A2][0] = sx(g(c, A1).wrapping_add(g(c, A2)));
                        c.g[T0][0] = c.g[A1][0];
                        c.sq(3, p(g(c, A0), 0x20));
                        c.g[V0][0] = c.lw(p(g(c, S0), 0x1c));
                        c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, A1)));
                        c.lq(3, p(g(c, V0), 0x20));
                        c.sq(3, p(g(c, A2), 0x20));
                        c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                        c.g[V1][0] = c.lw(p(g(c, S0), 0x20));
                        c.g[V0][0] = sx(g(c, A1).wrapping_add(g(c, V0)));
                        c.g[A2][0] = c.lw(p(g(c, S0), 0x24));
                        c.lq(4, p(g(c, V0), 0x20));
                        c.g[V1][0] = sx(g(c, A1).wrapping_add(g(c, V1)));
                        c.g[A2][0] = sx(g(c, A1).wrapping_add(g(c, A2)));
                        c.sq(4, p(g(c, V1), 0x20));
                        c.g[V0][0] = c.lw(p(g(c, S0), 0x20));
                        c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, A1)));
                        c.lq(3, p(g(c, V0), 0x20));
                        c.sq(3, p(g(c, A2), 0x20));
                        'l_2e8a70: loop {
                            c.cycles += 48;
                            c.g[A0][0] = c.lw(p(g(c, S0), 0x8c));
                            c.g[T2][0] = sx(g(c, T2).wrapping_sub(0x1));
                            c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                            c.mult(0, 4, 4, 9);
                            c.g[A1][0] = c.lw(g(c, A3));
                            c.g[V0][0] = sx(g(c, T0).wrapping_add(g(c, V0)));
                            c.lq(3, p(g(c, V0), 0x20));
                            c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, A1)));
                            c.sq(3, p(g(c, A0), 0x20));
                            c.g[V0][0] = c.lw(p(g(c, S0), 0x8c));
                            c.g[V1][0] = c.lw(g(c, A3));
                            c.mult(0, 2, 2, 9);
                            c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, V1)));
                            c.swc1(0, p(g(c, V0), 0x2c));
                            c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                            c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                            c.mult(0, 3, 3, 9);
                            c.g[A0][0] = c.lw(g(c, A3));
                            c.g[V0][0] = sx(g(c, T0).wrapping_add(g(c, V0)));
                            c.sw(p(g(c, V0), 0x1c), 0u64);
                            c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A0)));
                            c.sw(p(g(c, V1), 0x1c), 0u64);
                            c.g[A0][0] = c.lw(p(g(c, S0), 0x8c));
                            c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                            c.mult(0, 4, 4, 9);
                            c.g[V1][0] = c.lw(g(c, A3));
                            c.g[V0][0] = sx(g(c, T0).wrapping_add(g(c, V0)));
                            c.g[A1][0] = c.lw(p(g(c, V0), 0x10));
                            c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                            c.sw(p(g(c, A0), 0x10), c.g[A1][0]);
                            c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                            c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                            c.mult(0, 3, 3, 9);
                            c.g[A0][0] = c.lw(g(c, A3));
                            c.g[V0][0] = sx(g(c, T0).wrapping_add(g(c, V0)));
                            c.g[A1][0] = c.lw(p(g(c, V0), 0x14));
                            c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A0)));
                            c.sw(p(g(c, V1), 0x14), c.g[A1][0]);
                            c.g[A0][0] = c.lw(p(g(c, S0), 0x8c));
                            c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                            c.mult(0, 4, 4, 9);
                            c.g[A1][0] = c.lw(g(c, A3));
                            c.g[V0][0] = sx(g(c, T0).wrapping_add(g(c, V0)));
                            c.g[A3][0] = sx(p(g(c, A3), 0x4));
                            c.g[V1][0] = c.lw(p(g(c, V0), 0x18));
                            c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, A1)));
                            let t = (c.g[T2][0] as i64) >= 0;
                            c.sw(p(g(c, A0), 0x18), c.g[V1][0]);
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002e8a70);
                                }
                                continue 'l_2e8a70;
                            } else {
                                break 'b_2e8b30;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.sw(p(g(c, S0), 0x94), 0u64);
                    if t {
                        break 'b_2ea440;
                    } else {
                        break 'b_2e8b38;
                    }
                }
            }
        }
        // `cBoardTrailFX_IsActive` said yes: pick up the fade it stores at
        // `self+0x9c`; a fresh trail (state 0) also advances the ring head.
        'b_2e8b78: {
            c.cycles += 2;
            if c.g[V0][0] != 0u64 {
                c.lwc1(1, p(g(c, S0), 0x9c));
                break 'b_2e8b78;
            } else {
                'b_2e8b50: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(p(g(c, S0), 0x8c));
                    c.g[V1][0] = 0x36;
                    if c.g[V1][0] == 0u64 {
                        break 'b_2e8b50;
                    } else {
                        break 'b_2e8b50;
                    }
                }
                c.cycles += 10;
                c.g[A1][0] = 0x36;
                c.g[V0][0] = sx(p(g(c, V0), 0x3));
                c.div(0, 2, 17);
                c.g[V1][0] = c.hi[0];
                c.g[A0][0] = sx(p(g(c, V1), 0x35));
                c.sw(p(g(c, S0), 0x8c), c.g[V1][0]);
                c.div(0, 4, 17);
                c.g[V0][0] = c.hi[0];
                c.g[S3][0] = c.g[V0][0];
                c.lwc1(1, p(g(c, S0), 0x9c));
                break 'b_2e8b78;
            }
        }
        // Rebuild the current vertex from the rider's board frame (`rider+0x370`
        // / `+0x460`) and the animation entry
        // `*(*(rider+0x780)+0x34) + (*(rider+0x8a4) << 6)`; the
        // dot products and clamps below blend its colour/width against the gp_6
        // float constants (f20-f27 keep the intermediate results).
        'b_2e8f1c: {
            'b_2e8e3c: {
                'b_2e8c7c: {
                    c.cycles += 51;
                    c.g[S2][0] = sx(frame.at(c, 0x40));
                    c.lwc1(2, gp_6(c, 0x149c));
                    c.g[S1][0] = sx(frame.at(c, 0x60));
                    c.lwc1(0, gp_6(c, 0x1498));
                    c.mul_s(1, 1, 2);
                    c.g[A2][0] = c.lw(g(c, S0));
                    c.g[AT][0] = 0x42af0000;
                    c.mtc1(1, 2);
                    c.g[AT][0] = 0x418c0000;
                    c.mtc1(1, 3);
                    c.sub_s(0, 0, 1);
                    c.swc1(0, p(g(c, S0), 0xa0));
                    c.g[V0][0] = c.lw(p(g(c, A2), 0x780));
                    c.g[V1][0] = c.lw(p(g(c, A2), 0x8a4));
                    c.g[A0][0] = c.lw(p(g(c, V0), 0x34));
                    c.g[V1][0] = sx(g(c, V1) << 6);
                    c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                    c.lq(2, p(g(c, A0), 0x30));
                    c.sq(2, frame.at(c, 0x40));
                    c.g[A1][0] = c.lw(p(g(c, A2), 0x780));
                    c.g[V1][0] = c.lw(p(g(c, A2), 0x8a4));
                    c.g[A0][0] = c.lw(p(g(c, A1), 0x34));
                    c.g[V1][0] = sx(g(c, V1) << 6);
                    c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                    c.lq(2, p(g(c, A0), 0x20));
                    c.sq(2, frame.at(c, 0x50));
                    c.g[A1][0] = c.lw(p(g(c, A2), 0x780));
                    c.g[V1][0] = c.lw(p(g(c, A2), 0x8a4));
                    c.g[A0][0] = c.lw(p(g(c, A1), 0x34));
                    c.g[V1][0] = sx(g(c, V1) << 6);
                    c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                    c.lq(2, g(c, A0));
                    c.sq(2, frame.at(c, 0x60));
                    c.mfc1(3, 2);
                    c.lqc2(4, frame.at(c, 0x60));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x80));
                    c.lq(2, frame.at(c, 0x80));
                    c.sq(2, frame.at(c, 0x70));
                    c.mfc1(3, 3);
                    c.lqc2(4, frame.at(c, 0x50));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x90));
                    c.lq(2, frame.at(c, 0x90));
                    c.g[A0][0] = c.lw(g(c, S0));
                    c.sq(2, frame.at(c, 0x80));
                    call_ptr(c, 0x0011fe98, 0x002e8c3c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_2e8c7c;
                    } else {
                        'b_2e8c68: {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw(g(c, S0));
                            c.g[V0][0] = c.lw(p(g(c, V1), 0xac4));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[V1][0];
                            if t {
                                break 'b_2e8c68;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(p(g(c, A0), 0xad0));
                                let t = c.g[V0][0] != 0u64;

                                if t {
                                    break 'b_2e8c68;
                                } else {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.lw(p(g(c, A0), 0xafc));
                                    c.g[A1][0] = (0u64 < c.g[V0][0]) as u64;
                                    break 'b_2e8c68;
                                }
                            }
                        }
                        c.cycles += 2;
                        if c.g[A1][0] == 0u64 {
                            c.g[A2][0] = c.lw(g(c, S0));
                            break 'b_2e8e3c;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(p(g(c, A0), 0xb00));
                            if c.g[V0][0] == 0u64 {
                                c.g[A2][0] = c.lw(g(c, S0));
                                break 'b_2e8e3c;
                            } else {
                                break 'b_2e8c7c;
                            }
                        }
                    }
                }
                'b_2e8d1c: {
                    c.cycles += 17;
                    c.lqc2(3, g(c, S2));
                    c.lqc2(4, frame.at(c, 0x70));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0xa0));
                    c.mtc1(0, 2);
                    c.lq(2, frame.at(c, 0xa0));
                    c.sq(2, frame.at(c, 0x90));
                    c.lqc2(3, g(c, S2));
                    c.lqc2(4, frame.at(c, 0x70));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0xb0));
                    c.lq(2, frame.at(c, 0xb0));
                    c.g[A0][0] = c.lw(g(c, S0));
                    c.sq(2, frame.at(c, 0xa0));
                    c.g[V1][0] = c.lw(p(g(c, A0), 0x330));
                    let t = c.g[V1][0] == 0u64;

                    if t {
                        break 'b_2e8d1c;
                    } else {
                        'b_2e8d00: {
                            c.cycles += 7;
                            c.g[A0][0] = c.lw(p(g(c, A0), 0x784));
                            c.g[A1][0] = 0x2;
                            call_ptr(c, 0x00312aa0, 0x002e8ccc)?;
                            c.g[V0][0] = sx(g(c, V0).wrapping_sub(0x17));
                            c.g[V0][0] = (c.g[V0][0] < 0x8u64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = c.lw(g(c, S0));
                            if t {
                                break 'b_2e8d00;
                            } else {
                                c.cycles += 5;
                                c.g[AT][0] = 0xffffffffbf800000;
                                c.mtc1(1, 2);
                                c.g[V0][0] = c.lw(p(g(c, V1), 0x320));
                                let t = c.g[V0][0] == 0u64;

                                if t {
                                    break 'b_2e8d1c;
                                } else {
                                    c.cycles += 4;
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 2);
                                    let t = true;

                                    if t {
                                        break 'b_2e8d1c;
                                    } else {
                                        break 'b_2e8d00;
                                    }
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 2);
                        c.g[V0][0] = c.lw(p(g(c, V1), 0x320));
                        let t = c.g[V0][0] == 0u64;

                        if t {
                            break 'b_2e8d1c;
                        } else {
                            c.cycles += 2;
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 2);
                            break 'b_2e8d1c;
                        }
                    }
                }
                'b_2e8dc8: {
                    'b_2e8d78: {
                        c.cycles += 4;
                        c.mtc1(0, 0);
                        c.c_lt_s(0, 2);
                        let t = !c.fcc();

                        if t {
                            break 'b_2e8d78;
                        } else {
                            c.cycles += 19;
                            c.g[AT][0] = 0x432f0000;
                            c.mtc1(1, 0);
                            c.g[AT][0] = 0x430c0000;
                            c.mtc1(1, 1);
                            c.mul_s(0, 2, 0);
                            c.min_s(0, 0, 1);
                            c.mfc1(3, 0);
                            c.lqc2(4, g(c, S1));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, frame.at(c, 0xc0));
                            c.lq(2, frame.at(c, 0xc0));
                            c.sq(2, frame.at(c, 0xb0));
                            c.lqc2(3, frame.at(c, 0x90));
                            c.lqc2(4, frame.at(c, 0xb0));
                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, frame.at(c, 0x90));
                            let t = true;

                            if t {
                                break 'b_2e8dc8;
                            } else {
                                break 'b_2e8d78;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.c_lt_s(2, 0);
                    let t = !c.fcc();

                    if t {
                        break 'b_2e8dc8;
                    } else {
                        c.cycles += 17;
                        c.g[AT][0] = 0x432f0000;
                        c.mtc1(1, 0);
                        c.g[AT][0] = 0xffffffffc30c0000;
                        c.mtc1(1, 1);
                        c.mul_s(0, 2, 0);
                        c.max_s(0, 0, 1);
                        c.mfc1(3, 0);
                        c.lqc2(4, g(c, S1));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, frame.at(c, 0xc0));
                        c.lq(2, frame.at(c, 0xc0));
                        c.sq(2, frame.at(c, 0xb0));
                        c.lqc2(3, frame.at(c, 0xa0));
                        c.lqc2(4, frame.at(c, 0xb0));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, frame.at(c, 0xa0));
                        break 'b_2e8dc8;
                    }
                }
                c.cycles += 29;
                c.lqc2(3, frame.at(c, 0xa0));
                c.lqc2(4, frame.at(c, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0xc0));
                c.lq(2, frame.at(c, 0xc0));
                c.sq(2, frame.at(c, 0xb0));
                c.sq(2, frame.at(c, 0));
                c.lqc2(3, frame.at(c, 0x90));
                c.lqc2(4, frame.at(c, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0xc0));
                c.lq(2, frame.at(c, 0xc0));
                c.sq(2, frame.at(c, 0xb0));
                c.sq(2, frame.at(c, 0x20));
                c.lqc2(3, frame.at(c, 0xa0));
                c.lqc2(4, frame.at(c, 0x80));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0xc0));
                c.lq(2, frame.at(c, 0xc0));
                c.sq(2, frame.at(c, 0xb0));
                c.sq(2, frame.at(c, 0x10));
                c.lqc2(3, frame.at(c, 0x90));
                c.lqc2(4, frame.at(c, 0x80));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0xc0));
                c.lq(2, frame.at(c, 0xc0));
                c.sq(2, frame.at(c, 0x30));
                let t = true;
                c.sq(2, frame.at(c, 0xb0));
                if t {
                    break 'b_2e8f1c;
                } else {
                    break 'b_2e8e3c;
                }
            }
            'b_2e8ecc: {
                c.cycles += 19;
                c.g[V0][0] = c.lw(p(g(c, A2), 0x780));
                c.g[V1][0] = c.lw(p(g(c, A2), 0x8b0));
                c.g[A0][0] = c.lw(p(g(c, V0), 0x34));
                c.g[V1][0] = sx(g(c, V1) << 6);
                c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                c.lq(2, p(g(c, A0), 0x30));
                c.sq(2, frame.at(c, 0x90));
                c.sq(2, frame.at(c, 0));
                c.g[A1][0] = c.lw(p(g(c, A2), 0x780));
                c.g[V1][0] = c.lw(p(g(c, A2), 0x8b8));
                c.g[A0][0] = c.lw(p(g(c, A1), 0x34));
                c.g[V1][0] = sx(g(c, V1) << 6);
                c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                c.lq(2, p(g(c, A0), 0x30));
                c.sq(2, frame.at(c, 0x10));
                c.sq(2, frame.at(c, 0x90));
                c.g[V1][0] = c.lw(p(g(c, A2), 0x150));
                let t = c.g[V1][0] == 0u64;

                if t {
                    break 'b_2e8ecc;
                } else {
                    c.cycles += 17;
                    c.g[V0][0] = c.lw(p(g(c, A2), 0x780));
                    c.g[V1][0] = c.lw(p(g(c, A2), 0x918));
                    c.g[A0][0] = c.lw(p(g(c, V0), 0x34));
                    c.g[V1][0] = sx(g(c, V1) << 6);
                    c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                    c.lq(2, p(g(c, A0), 0x30));
                    c.sq(2, frame.at(c, 0x90));
                    c.sq(2, frame.at(c, 0x20));
                    c.g[A1][0] = c.lw(p(g(c, A2), 0x780));
                    c.g[V1][0] = c.lw(p(g(c, A2), 0x8e8));
                    c.g[A0][0] = c.lw(p(g(c, A1), 0x34));
                    c.g[V1][0] = sx(g(c, V1) << 6);
                    c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                    c.lq(2, p(g(c, A0), 0x30));
                    c.sq(2, frame.at(c, 0x30));
                    let t = true;
                    c.sq(2, frame.at(c, 0x90));
                    if t {
                        break 'b_2e8f1c;
                    } else {
                        break 'b_2e8ecc;
                    }
                }
            }
            c.cycles += 20;
            c.lqc2(3, g(c, S2));
            c.lqc2(4, frame.at(c, 0x70));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0xa0));
            c.lq(2, frame.at(c, 0xa0));
            c.sq(2, frame.at(c, 0x90));
            c.lqc2(3, frame.at(c, 0x90));
            c.lqc2(4, frame.at(c, 0x80));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0xb0));
            c.lq(2, frame.at(c, 0xb0));
            c.sq(2, frame.at(c, 0xa0));
            c.sq(2, frame.at(c, 0x20));
            c.lqc2(3, frame.at(c, 0x90));
            c.lqc2(4, frame.at(c, 0x80));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0xb0));
            c.lq(2, frame.at(c, 0xb0));
            c.sq(2, frame.at(c, 0x30));
            c.sq(2, frame.at(c, 0xa0));
            break 'b_2e8f1c;
        }
        'b_2e8f8c: {
            'b_2e8f64: {
                c.cycles += 15;
                c.g[V1][0] = c.lw(g(c, S0));
                c.lwc1(0, gp_6(c, -0x39cc));
                c.lqc2(3, p(g(c, V1), 0x1e0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(2, 22);
                c.mtc1(2, 26);
                c.c_lt_s(26, 0);
                let t = !c.fcc();

                if t {
                    break 'b_2e8f64;
                } else {
                    c.cycles += 3;
                    c.lq(2, p(g(c, S0), 0xc0));
                    let t = true;
                    c.sq(2, frame.at(c, 0x90));
                    if t {
                        break 'b_2e8f8c;
                    } else {
                        break 'b_2e8f64;
                    }
                }
            }
            c.cycles += 10;
            c.mfc1(2, 26);
            c.qmtc2(2, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, p(g(c, V1), 0x1e0));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 4, 4, Src::Q);
            c.sqc2(4, frame.at(c, 0xb0));
            c.lq(2, frame.at(c, 0xb0));
            c.sq(2, frame.at(c, 0x90));
            c.sq(2, frame.at(c, 0xa0));
            break 'b_2e8f8c;
        }
        'b_2e90a4: {
            c.cycles += 59;
            c.g[V1][0] = c.lw(g(c, S0));
            c.lqc2(3, frame.at(c, 0x90));
            c.lqc2(5, p(g(c, V1), 0x370));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 0);
            c.mtc1(0, 1);
            c.mfc1(4, 0);
            c.lqc2(4, p(g(c, V1), 0x370));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, 0xd0));
            c.lq(2, frame.at(c, 0xd0));
            c.sq(2, frame.at(c, 0xc0));
            c.lqc2(3, frame.at(c, 0x90));
            c.lqc2(4, frame.at(c, 0xc0));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0xd0));
            c.lq(2, frame.at(c, 0xd0));
            c.sq(2, frame.at(c, 0xb0));
            c.lqc2(3, frame.at(c, 0xb0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().rsqrt(0, 3, 4, 0);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 3, Src::Q);
            c.sqc2(5, frame.at(c, 0xd0));
            c.lq(2, frame.at(c, 0xd0));
            c.g[V1][0] = c.lw(g(c, S0));
            c.sq(2, frame.at(c, 0xa0));
            c.sq(2, frame.at(c, 0x90));
            c.lqc2(4, frame.at(c, 0x90));
            c.lqc2(5, p(g(c, V1), 0x370));
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
            c.sqc2(6, frame.at(c, 0xb0));
            c.lq(2, frame.at(c, 0xb0));
            c.sq(2, frame.at(c, 0xa0));
            c.lqc2(3, frame.at(c, 0xa0));
            c.lqc2(5, p(g(c, S0), 0xd0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 0);
            c.c_lt_s(0, 1);
            if !c.fcc() {
                c.g[V0][0] = c.lw(g(c, S0));
                break 'b_2e90a4;
            } else {
                c.cycles += 11;
                c.g[AT][0] = 0xffffffffbf800000;
                c.mtc1(1, 0);
                c.mfc1(3, 0);
                c.lqc2(4, frame.at(c, 0xa0));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0xc0));
                c.lq(2, frame.at(c, 0xc0));
                c.sq(2, frame.at(c, 0xa0));
                c.sq(2, frame.at(c, 0xb0));
                c.g[V0][0] = c.lw(g(c, S0));
                break 'b_2e90a4;
            }
        }
        'b_2e93a0: {
            'b_2e9228: {
                c.cycles += 66;
                c.lqc2(3, frame.at(c, 0x10));
                c.lqc2(4, p(g(c, V0), 0x460));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0xc0));
                c.lq(3, frame.at(c, 0xc0));
                c.g[A0][0] = c.lw(g(c, S0));
                c.sq(3, frame.at(c, 0xb0));
                c.lqc2(3, frame.at(c, 0));
                c.lqc2(4, p(g(c, A0), 0x460));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0xd0));
                c.lq(2, frame.at(c, 0xd0));
                c.g[V1][0] = c.lw(g(c, S0));
                c.sq(2, frame.at(c, 0xc0));
                c.lqc2(3, frame.at(c, 0x30));
                c.lqc2(4, p(g(c, V1), 0x460));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0xe0));
                c.lq(2, frame.at(c, 0xe0));
                c.g[V1][0] = c.lw(g(c, S0));
                c.sq(2, frame.at(c, 0xd0));
                c.lqc2(3, frame.at(c, 0x20));
                c.lqc2(4, p(g(c, V1), 0x460));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0xf0));
                c.lq(2, frame.at(c, 0xf0));
                c.sq(2, frame.at(c, 0xe0));
                c.lqc2(3, frame.at(c, 0xb0));
                c.lqc2(5, frame.at(c, 0xa0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(3, 4);
                c.mtc1(3, 2);
                c.lqc2(3, frame.at(c, 0xc0));
                c.lqc2(5, frame.at(c, 0xa0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 3);
                c.c_lt_s(3, 2);
                c.lqc2(3, frame.at(c, 0xd0));
                c.lqc2(5, frame.at(c, 0xa0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(3, 4);
                c.mtc1(3, 4);
                c.lqc2(3, frame.at(c, 0xe0));
                c.lqc2(5, frame.at(c, 0xa0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 5);
                let t = !c.fcc();

                if t {
                    break 'b_2e9228;
                } else {
                    c.cycles += 3;
                    c.c_lt_s(4, 2);
                    let t = !c.fcc();

                    if t {
                        break 'b_2e9228;
                    } else {
                        c.cycles += 4;
                        c.c_lt_s(5, 2);
                        let t = !c.fcc();
                        c.lwc1(1, gp_6(c, 0xa38));
                        if t {
                            break 'b_2e9228;
                        } else {
                            c.cycles += 24;
                            c.g[V1][0] = c.lw(g(c, S0));
                            c.lqc2(3, p(g(c, V1), 0x370));
                            c.lqc2(5, frame.at(c, 0xb0));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.qmfc2(2, 4);
                            c.mtc1(2, 0);
                            c.sub_s(0, 0, 1);
                            c.mfc1(4, 0);
                            c.lqc2(4, p(g(c, V1), 0x370));
                            c.qmtc2(4, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, frame.at(c, 0x130));
                            c.lq(2, frame.at(c, 0x130));
                            c.sq(2, frame.at(c, 0x120));
                            c.lqc2(3, frame.at(c, 0x10));
                            c.lqc2(4, frame.at(c, 0x120));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, frame.at(c, 0x130));
                            let t = true;
                            c.lq(2, frame.at(c, 0x130));
                            if t {
                                break 'b_2e93a0;
                            } else {
                                break 'b_2e9228;
                            }
                        }
                    }
                }
            }
            'b_2e92b0: {
                c.cycles += 3;
                c.c_lt_s(2, 3);
                let t = !c.fcc();

                if t {
                    break 'b_2e92b0;
                } else {
                    c.cycles += 3;
                    c.c_lt_s(4, 3);
                    let t = !c.fcc();

                    if t {
                        break 'b_2e92b0;
                    } else {
                        c.cycles += 4;
                        c.c_lt_s(5, 3);
                        let t = !c.fcc();
                        c.lwc1(1, gp_6(c, 0xa38));
                        if t {
                            break 'b_2e92b0;
                        } else {
                            c.cycles += 24;
                            c.g[V1][0] = c.lw(g(c, S0));
                            c.lqc2(3, p(g(c, V1), 0x370));
                            c.lqc2(5, frame.at(c, 0xc0));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.qmfc2(2, 4);
                            c.mtc1(2, 0);
                            c.sub_s(0, 0, 1);
                            c.mfc1(4, 0);
                            c.lqc2(4, p(g(c, V1), 0x370));
                            c.qmtc2(4, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, frame.at(c, 0x130));
                            c.lq(2, frame.at(c, 0x130));
                            c.sq(2, frame.at(c, 0x120));
                            c.lqc2(3, frame.at(c, 0));
                            c.lqc2(4, frame.at(c, 0x120));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, frame.at(c, 0x130));
                            let t = true;
                            c.lq(2, frame.at(c, 0x130));
                            if t {
                                break 'b_2e93a0;
                            } else {
                                break 'b_2e92b0;
                            }
                        }
                    }
                }
            }
            'b_2e9344: {
                c.cycles += 4;
                c.c_lt_s(2, 4);
                if !c.fcc() {
                    c.g[V1][0] = c.lw(g(c, S0));
                    break 'b_2e9344;
                } else {
                    c.cycles += 4;
                    c.c_lt_s(3, 4);
                    if !c.fcc() {
                        c.g[V1][0] = c.lw(g(c, S0));
                        break 'b_2e9344;
                    } else {
                        'b_2e9340: {
                            c.cycles += 4;
                            c.c_lt_s(5, 4);
                            let t = !c.fcc();
                            c.lwc1(1, gp_6(c, 0xa38));
                            if t {
                                break 'b_2e9340;
                            } else {
                                c.cycles += 24;
                                c.g[V1][0] = c.lw(g(c, S0));
                                c.lqc2(3, p(g(c, V1), 0x370));
                                c.lqc2(5, frame.at(c, 0xd0));
                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                c.qmfc2(2, 4);
                                c.mtc1(2, 0);
                                c.sub_s(0, 0, 1);
                                c.mfc1(4, 0);
                                c.lqc2(4, p(g(c, V1), 0x370));
                                c.qmtc2(4, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, frame.at(c, 0x130));
                                c.lq(2, frame.at(c, 0x130));
                                c.sq(2, frame.at(c, 0x120));
                                c.lqc2(3, frame.at(c, 0x30));
                                c.lqc2(4, frame.at(c, 0x120));
                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, frame.at(c, 0x130));
                                let t = true;
                                c.lq(2, frame.at(c, 0x130));
                                if t {
                                    break 'b_2e93a0;
                                } else {
                                    break 'b_2e9340;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(g(c, S0));
                        break 'b_2e9344;
                    }
                }
            }
            c.cycles += 23;
            c.lwc1(1, gp_6(c, 0xa38));
            c.lqc2(3, p(g(c, V1), 0x370));
            c.lqc2(5, frame.at(c, 0xe0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 0);
            c.sub_s(0, 0, 1);
            c.mfc1(4, 0);
            c.lqc2(4, p(g(c, V1), 0x370));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, 0x130));
            c.lq(2, frame.at(c, 0x130));
            c.sq(2, frame.at(c, 0x120));
            c.lqc2(3, frame.at(c, 0x20));
            c.lqc2(4, frame.at(c, 0x120));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x130));
            c.lq(2, frame.at(c, 0x130));
            break 'b_2e93a0;
        }
        'b_2e95a8: {
            'b_2e9430: {
                c.cycles += 5;
                c.sq(2, frame.at(c, 0xf0));
                c.c_lt_s(2, 3);
                let t = !c.fcc();
                c.sq(2, frame.at(c, 0x110));
                if t {
                    break 'b_2e9430;
                } else {
                    c.cycles += 3;
                    c.c_lt_s(2, 4);
                    let t = !c.fcc();

                    if t {
                        break 'b_2e9430;
                    } else {
                        c.cycles += 4;
                        c.c_lt_s(2, 5);
                        let t = !c.fcc();
                        c.lwc1(1, gp_6(c, 0xa38));
                        if t {
                            break 'b_2e9430;
                        } else {
                            c.cycles += 24;
                            c.g[V1][0] = c.lw(g(c, S0));
                            c.lqc2(3, p(g(c, V1), 0x370));
                            c.lqc2(5, frame.at(c, 0xb0));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.qmfc2(2, 4);
                            c.mtc1(2, 0);
                            c.sub_s(0, 0, 1);
                            c.mfc1(4, 0);
                            c.lqc2(4, p(g(c, V1), 0x370));
                            c.qmtc2(4, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, frame.at(c, 0x130));
                            c.lq(2, frame.at(c, 0x130));
                            c.sq(2, frame.at(c, 0x120));
                            c.lqc2(3, frame.at(c, 0x10));
                            c.lqc2(4, frame.at(c, 0x120));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, frame.at(c, 0x130));
                            let t = true;
                            c.lq(2, frame.at(c, 0x130));
                            if t {
                                break 'b_2e95a8;
                            } else {
                                break 'b_2e9430;
                            }
                        }
                    }
                }
            }
            'b_2e94b8: {
                c.cycles += 3;
                c.c_lt_s(3, 2);
                let t = !c.fcc();

                if t {
                    break 'b_2e94b8;
                } else {
                    c.cycles += 3;
                    c.c_lt_s(3, 4);
                    let t = !c.fcc();

                    if t {
                        break 'b_2e94b8;
                    } else {
                        c.cycles += 4;
                        c.c_lt_s(3, 5);
                        let t = !c.fcc();
                        c.lwc1(1, gp_6(c, 0xa38));
                        if t {
                            break 'b_2e94b8;
                        } else {
                            c.cycles += 24;
                            c.g[V1][0] = c.lw(g(c, S0));
                            c.lqc2(3, p(g(c, V1), 0x370));
                            c.lqc2(5, frame.at(c, 0xc0));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.qmfc2(2, 4);
                            c.mtc1(2, 0);
                            c.sub_s(0, 0, 1);
                            c.mfc1(4, 0);
                            c.lqc2(4, p(g(c, V1), 0x370));
                            c.qmtc2(4, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, frame.at(c, 0x130));
                            c.lq(2, frame.at(c, 0x130));
                            c.sq(2, frame.at(c, 0x120));
                            c.lqc2(3, frame.at(c, 0));
                            c.lqc2(4, frame.at(c, 0x120));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, frame.at(c, 0x130));
                            let t = true;
                            c.lq(2, frame.at(c, 0x130));
                            if t {
                                break 'b_2e95a8;
                            } else {
                                break 'b_2e94b8;
                            }
                        }
                    }
                }
            }
            'b_2e954c: {
                c.cycles += 4;
                c.c_lt_s(4, 2);
                if !c.fcc() {
                    c.g[V1][0] = c.lw(g(c, S0));
                    break 'b_2e954c;
                } else {
                    c.cycles += 4;
                    c.c_lt_s(4, 3);
                    if !c.fcc() {
                        c.g[V1][0] = c.lw(g(c, S0));
                        break 'b_2e954c;
                    } else {
                        'b_2e9548: {
                            c.cycles += 4;
                            c.c_lt_s(4, 5);
                            let t = !c.fcc();
                            c.lwc1(1, gp_6(c, 0xa38));
                            if t {
                                break 'b_2e9548;
                            } else {
                                c.cycles += 24;
                                c.g[V1][0] = c.lw(g(c, S0));
                                c.lqc2(3, p(g(c, V1), 0x370));
                                c.lqc2(5, frame.at(c, 0xd0));
                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                c.qmfc2(2, 4);
                                c.mtc1(2, 0);
                                c.sub_s(0, 0, 1);
                                c.mfc1(4, 0);
                                c.lqc2(4, p(g(c, V1), 0x370));
                                c.qmtc2(4, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, frame.at(c, 0x130));
                                c.lq(2, frame.at(c, 0x130));
                                c.sq(2, frame.at(c, 0x120));
                                c.lqc2(3, frame.at(c, 0x30));
                                c.lqc2(4, frame.at(c, 0x120));
                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, frame.at(c, 0x130));
                                let t = true;
                                c.lq(2, frame.at(c, 0x130));
                                if t {
                                    break 'b_2e95a8;
                                } else {
                                    break 'b_2e9548;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(g(c, S0));
                        break 'b_2e954c;
                    }
                }
            }
            c.cycles += 23;
            c.lwc1(1, gp_6(c, 0xa38));
            c.lqc2(3, p(g(c, V1), 0x370));
            c.lqc2(5, frame.at(c, 0xe0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 0);
            c.sub_s(0, 0, 1);
            c.mfc1(4, 0);
            c.lqc2(4, p(g(c, V1), 0x370));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, 0x130));
            c.lq(2, frame.at(c, 0x130));
            c.sq(2, frame.at(c, 0x120));
            c.lqc2(3, frame.at(c, 0x20));
            c.lqc2(4, frame.at(c, 0x120));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x130));
            c.lq(2, frame.at(c, 0x130));
            break 'b_2e95a8;
        }
        'b_2e9794: {
            c.cycles += 58;
            c.sq(2, frame.at(c, 0x100));
            c.sq(2, frame.at(c, 0x110));
            c.g[V0][0] = c.lw(g(c, S0));
            c.lqc2(3, frame.at(c, 0xf0));
            c.lqc2(4, p(g(c, V0), 0x460));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x120));
            c.lq(3, frame.at(c, 0x120));
            c.g[A0][0] = c.lw(g(c, S0));
            c.sq(3, frame.at(c, 0x110));
            c.lqc2(3, frame.at(c, 0x100));
            c.lqc2(4, p(g(c, A0), 0x460));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x130));
            c.lq(2, frame.at(c, 0x130));
            c.g[A0][0] = c.lw(g(c, S0));
            c.sq(2, frame.at(c, 0x120));
            c.lqc2(3, frame.at(c, 0x110));
            c.lqc2(5, frame.at(c, 0x90));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 24);
            c.lqc2(3, frame.at(c, 0x120));
            c.lqc2(5, frame.at(c, 0x90));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 27);
            c.lqc2(3, frame.at(c, 0x110));
            c.lqc2(5, frame.at(c, 0xa0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 23);
            c.lqc2(3, frame.at(c, 0x120));
            c.lqc2(5, frame.at(c, 0xa0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 25);
            call_ptr(c, 0x0011fe98, 0x002e9684)?;
            c.g[V1][0] = 0x2;
            if c.g[V0][0] != c.g[V1][0] {
                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                break 'b_2e9794;
            } else {
                c.cycles += 65;
                c.g[AT][0] = 0x3f400000;
                c.mtc1(1, 1);
                c.mul_s(0, 23, 1);
                c.mfc1(2, 0);
                c.lqc2(4, frame.at(c, 0xa0));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x160));
                c.mul_s(1, 25, 1);
                c.lq(2, frame.at(c, 0x160));
                c.g[V1][0] = c.lw(g(c, S0));
                c.sq(2, frame.at(c, 0x150));
                c.lqc2(3, p(g(c, V1), 0x460));
                c.lqc2(4, frame.at(c, 0x150));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x160));
                c.lq(2, frame.at(c, 0x160));
                c.lwc1(0, gp_6(c, 0xa38));
                c.sq(2, frame.at(c, 0x140));
                c.g[V1][0] = c.lw(g(c, S0));
                c.mfc1(4, 0);
                c.lqc2(4, p(g(c, V1), 0x370));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x170));
                c.lq(2, frame.at(c, 0x170));
                c.sq(2, frame.at(c, 0x160));
                c.lqc2(3, frame.at(c, 0x140));
                c.lqc2(4, frame.at(c, 0x160));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x170));
                c.lq(2, frame.at(c, 0x170));
                c.sq(2, frame.at(c, 0xf0));
                c.sq(2, frame.at(c, 0x130));
                c.mfc1(3, 1);
                c.lqc2(4, frame.at(c, 0xa0));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x160));
                c.lq(2, frame.at(c, 0x160));
                c.sq(2, frame.at(c, 0x150));
                c.g[V1][0] = c.lw(g(c, S0));
                c.lqc2(3, p(g(c, V1), 0x460));
                c.lqc2(4, frame.at(c, 0x150));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x160));
                c.lq(2, frame.at(c, 0x160));
                c.lwc1(0, gp_6(c, 0xa38));
                c.sq(2, frame.at(c, 0x140));
                c.g[V1][0] = c.lw(g(c, S0));
                c.mfc1(4, 0);
                c.lqc2(4, p(g(c, V1), 0x370));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x170));
                c.lq(2, frame.at(c, 0x170));
                c.sq(2, frame.at(c, 0x160));
                c.lqc2(3, frame.at(c, 0x140));
                c.lqc2(4, frame.at(c, 0x160));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x170));
                c.lq(2, frame.at(c, 0x170));
                c.sq(2, frame.at(c, 0x100));
                c.sq(2, frame.at(c, 0x130));
                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                break 'b_2e9794;
            }
        }
        // Scale and clamp the blended value with the gp_6 float constants at
        // -0x39ac..-0x39cc.
        'b_2e98a0: {
            'b_2e9864: {
                c.cycles += 41;
                c.g[A1][0] = 0x30;
                c.g[V0][0] = c.lw(p(g(c, S0), 0x18));
                c.abs_s(21, 23);
                c.mult(0, 3, 3, 5);
                c.lq(4, frame.at(c, 0xf0));
                c.mtc1(0, 22);
                c.abs_s(20, 24);
                c.c_eq_s(21, 22);
                c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, V0)));
                c.sq(4, p(g(c, V1), 0x20));
                c.g[V0][0] = c.lw(p(g(c, S0), 0x8c));
                c.g[V1][0] = c.lw(p(g(c, S0), 0x10));
                c.mult(0, 2, 2, 5);
                c.lq(4, frame.at(c, 0xf0));
                c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, V1)));
                c.sq(4, p(g(c, V0), 0x20));
                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                c.g[V0][0] = c.lw(p(g(c, S0), 0x1c));
                c.mult(0, 3, 3, 5);
                c.lq(4, frame.at(c, 0xf0));
                c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, V0)));
                c.sq(4, p(g(c, V1), 0x20));
                c.g[V0][0] = c.lw(p(g(c, S0), 0x8c));
                c.g[V1][0] = c.lw(p(g(c, S0), 0x20));
                c.mult(0, 2, 2, 5);
                c.lq(4, frame.at(c, 0x100));
                c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, V1)));
                c.sq(4, p(g(c, V0), 0x20));
                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                c.mult(0, 3, 3, 5);
                c.lq(4, frame.at(c, 0x100));
                c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, V0)));
                c.sq(4, p(g(c, V1), 0x20));
                c.g[V0][0] = c.lw(p(g(c, S0), 0x8c));
                c.g[A0][0] = c.lw(p(g(c, S0), 0x24));
                c.mult(0, 2, 2, 5);
                c.lq(3, frame.at(c, 0x100));
                c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, A0)));
                let t = !c.fcc();
                c.sq(3, p(g(c, V0), 0x20));
                if t {
                    break 'b_2e9864;
                } else {
                    c.cycles += 4;
                    c.c_eq_s(20, 22);
                    let t = c.fcc();
                    c.mov_s(12, 20);
                    if t {
                        break 'b_2e98a0;
                    } else {
                        c.cycles += 4;
                        c.c_le_s(22, 20);
                        c.lwc1(12, gp_6(c, -0x39c8));
                        let t = !c.fcc();

                        if t {
                            break 'b_2e98a0;
                        } else {
                            c.cycles += 3;
                            c.lwc1(12, gp_6(c, -0x39c4));
                            let t = true;

                            if t {
                                break 'b_2e98a0;
                            } else {
                                break 'b_2e9864;
                            }
                        }
                    }
                }
            }
            c.cycles += 7;
            c.div_s(12, 20, 21);
            call_ptr(c, 0x0031c228, 0x002e9870)?;
            c.c_lt_s(21, 22);
            let t = !c.fcc();
            c.mov_s(12, 0);
            if t {
                break 'b_2e98a0;
            } else {
                'b_2e9898: {
                    c.cycles += 3;
                    c.c_lt_s(22, 20);
                    let t = !c.fcc();

                    if t {
                        break 'b_2e9898;
                    } else {
                        c.cycles += 3;
                        c.lwc1(0, gp_6(c, -0x39c0));
                        let t = true;
                        c.add_s(12, 12, 0);
                        if t {
                            break 'b_2e98a0;
                        } else {
                            break 'b_2e9898;
                        }
                    }
                }
                c.cycles += 2;
                c.lwc1(0, gp_6(c, -0x39bc));
                c.sub_s(12, 12, 0);
                break 'b_2e98a0;
            }
        }
        'b_2e9914: {
            'b_2e98e8: {
                c.cycles += 4;
                c.lwc1(0, gp_6(c, -0x39b8));
                c.c_lt_s(0, 12);
                let t = !c.fcc();

                if t {
                    break 'b_2e98e8;
                } else {
                    c.cycles += 12;
                    c.lwc1(0, gp_6(c, -0x39b4));
                    c.g[AT][0] = 0x40000000;
                    c.mtc1(1, 2);
                    c.mul_s(0, 12, 0);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 3);
                    c.mtc1(0, 1);
                    c.sub_s(0, 2, 0);
                    c.c_le_s(1, 0);
                    let t = !c.fcc();
                    c.mov_s(2, 1);
                    if t {
                        break 'b_2e9914;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.min_s(2, 0, 3);
                        if t {
                            break 'b_2e9914;
                        } else {
                            break 'b_2e98e8;
                        }
                    }
                }
            }
            'b_2e9910: {
                c.cycles += 9;
                c.lwc1(0, gp_6(c, -0x39b0));
                c.mtc1(0, 1);
                c.mul_s(0, 12, 0);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 2);
                c.c_le_s(1, 0);
                let t = !c.fcc();
                c.mov_s(3, 1);
                if t {
                    break 'b_2e9910;
                } else {
                    c.cycles += 1;
                    c.min_s(3, 0, 2);
                    break 'b_2e9910;
                }
            }
            c.cycles += 1;
            c.mov_s(2, 3);
            break 'b_2e9914;
        }
        'b_2e9934: {
            c.cycles += 8;
            c.lwc1(0, gp_6(c, -0x39ac));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 1);
            c.mul_s(0, 26, 0);
            c.c_le_s(0, 1);
            if c.fcc() {
                c.mul_s(2, 2, 0);
                break 'b_2e9934;
            } else {
                break 'b_2e9934;
            }
        }
        'b_2e9988: {
            'b_2e9984: {
                c.cycles += 18;
                c.g[V0][0] = c.lw(g(c, S0));
                c.c_lt_s(24, 27);
                c.lwc1(0, gp_6(c, 0xa38));
                c.mfc1(4, 0);
                c.lqc2(4, p(g(c, V0), 0x370));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x150));
                c.lq(3, frame.at(c, 0x150));
                c.g[A0][0] = c.lw(g(c, S0));
                c.sq(3, frame.at(c, 0x140));
                c.lqc2(3, p(g(c, A0), 0x460));
                c.lqc2(4, frame.at(c, 0x140));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x150));
                c.lq(2, frame.at(c, 0x150));
                let t = !c.fcc();
                c.sq(2, frame.at(c, 0x130));
                if t {
                    break 'b_2e9984;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.mul_s(0, 2, 24);
                    if t {
                        break 'b_2e9988;
                    } else {
                        break 'b_2e9984;
                    }
                }
            }
            c.cycles += 1;
            c.mul_s(0, 2, 27);
            break 'b_2e9988;
        }
        c.cycles += 242;
        c.mfc1(3, 0);
        c.lqc2(4, frame.at(c, 0x90));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x170));
        c.lq(2, frame.at(c, 0x170));
        c.sq(2, frame.at(c, 0x160));
        c.lqc2(3, frame.at(c, 0x130));
        c.lqc2(4, frame.at(c, 0x160));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x170));
        c.lq(2, frame.at(c, 0x170));
        c.sq(2, frame.at(c, 0x140));
        c.sq(2, frame.at(c, 0x150));
        c.mfc1(2, 23);
        c.lqc2(4, frame.at(c, 0xa0));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x170));
        c.mtc1(0, 20);
        c.lq(2, frame.at(c, 0x170));
        c.g[A2][0] = 0x30;
        c.g[AT][0] = 0x41200000;
        c.mtc1(1, 1);
        c.mult(0, 6, 19, 6);
        c.sq(2, frame.at(c, 0x160));
        c.g[S1][0] = sx(p(g(c, S0), 0x10));
        c.lqc2(3, frame.at(c, 0x140));
        c.lqc2(4, frame.at(c, 0x160));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x170));
        c.lq(2, frame.at(c, 0x170));
        c.sq(2, frame.at(c, 0x150));
        c.mfc1(3, 25);
        c.lqc2(4, frame.at(c, 0xa0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x180));
        c.lq(2, frame.at(c, 0x180));
        c.sq(2, frame.at(c, 0x170));
        c.lqc2(3, frame.at(c, 0x140));
        c.lqc2(4, frame.at(c, 0x170));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x180));
        c.lq(2, frame.at(c, 0x180));
        c.sq(2, frame.at(c, 0x160));
        c.lqc2(3, frame.at(c, 0x160));
        c.lqc2(4, frame.at(c, 0x150));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x190));
        c.lq(2, frame.at(c, 0x190));
        c.sq(2, frame.at(c, 0x180));
        c.lqc2(3, frame.at(c, 0x180));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, frame.at(c, 0x190));
        c.lq(2, frame.at(c, 0x190));
        c.sq(2, frame.at(c, 0x170));
        c.lwc1(0, gp_6(c, 0x1490));
        c.mfc1(3, 0);
        c.lqc2(4, frame.at(c, 0x170));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x190));
        c.lq(2, frame.at(c, 0x190));
        c.lwc1(0, gp_6(c, 0x1494));
        c.sq(2, frame.at(c, 0x180));
        c.mfc1(3, 0);
        c.lqc2(4, frame.at(c, 0x170));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x1b0));
        c.lq(2, frame.at(c, 0x1b0));
        c.g[V1][0] = c.lw(g(c, S0));
        c.sq(2, frame.at(c, 0x1a0));
        c.lwc1(0, p(g(c, V1), 0x454));
        c.min_s(0, 0, 20);
        c.abs_s(0, 0);
        c.add_s(0, 0, 1);
        c.mfc1(2, 0);
        c.lqc2(4, frame.at(c, 0x1a0));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x1b0));
        c.lq(3, frame.at(c, 0x1b0));
        c.lwc1(0, p(g(c, S0), 0xa0));
        c.sq(3, frame.at(c, 0x190));
        c.g[V0][0] = c.lw(g(c, S0));
        c.mfc1(4, 0);
        c.lqc2(4, p(g(c, V0), 0x370));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x1b0));
        c.lq(3, frame.at(c, 0x1b0));
        c.g[V0][0] = c.lw(g(c, S0));
        c.sq(3, frame.at(c, 0x1a0));
        c.lwc1(0, p(g(c, V0), 0x454));
        c.min_s(0, 0, 20);
        c.mfc1(4, 0);
        c.lqc2(4, p(g(c, V0), 0x370));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x1c0));
        c.lq(3, frame.at(c, 0x1c0));
        c.g[A0][0] = c.lw(p(g(c, S0), 0x18));
        c.sq(3, frame.at(c, 0x1b0));
        c.lqc2(3, frame.at(c, 0x150));
        c.lqc2(4, frame.at(c, 0x190));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.g[A0][0] = sx(g(c, A2).wrapping_add(g(c, A0)));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1e0));
        c.lqc2(3, frame.at(c, 0x1e0));
        c.lqc2(4, frame.at(c, 0x1b0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1d0));
        c.lqc2(3, frame.at(c, 0x1d0));
        c.lqc2(4, frame.at(c, 0x1b0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1c0));
        c.sq(2, p(g(c, A0), 0x20));
        c.g[A1][0] = c.lw(p(g(c, S0), 0x10));
        c.lwc1(0, p(g(c, S0), 0x4));
        c.mfc1(2, 0);
        c.lqc2(4, frame.at(c, 0x180));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.g[A1][0] = sx(g(c, A2).wrapping_add(g(c, A1)));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1e0));
        c.lqc2(3, frame.at(c, 0x150));
        c.lqc2(4, frame.at(c, 0x1e0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1d0));
        c.lqc2(3, frame.at(c, 0x1d0));
        c.lqc2(4, frame.at(c, 0x1a0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1c0));
        c.sq(2, p(g(c, A1), 0x20));
        c.g[V1][0] = c.lw(p(g(c, S0), 0x1c));
        c.lqc2(3, frame.at(c, 0x150));
        c.lqc2(4, frame.at(c, 0x1b0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1d0));
        c.lq(2, frame.at(c, 0x1d0));
        c.g[V1][0] = sx(g(c, A2).wrapping_add(g(c, V1)));
        c.sq(2, frame.at(c, 0x1c0));
        c.sq(2, p(g(c, V1), 0x20));
        c.g[A0][0] = c.lw(p(g(c, S0), 0x20));
        c.lqc2(3, frame.at(c, 0x160));
        c.lqc2(4, frame.at(c, 0x1b0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1d0));
        c.lq(2, frame.at(c, 0x1d0));
        c.g[A0][0] = sx(g(c, A2).wrapping_add(g(c, A0)));
        c.sq(2, frame.at(c, 0x1c0));
        c.sq(2, p(g(c, A0), 0x20));
        c.g[V1][0] = c.lw(p(g(c, S0), 0x14));
        c.lwc1(0, p(g(c, S0), 0x8));
        c.mfc1(2, 0);
        c.lqc2(4, frame.at(c, 0x180));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.g[V1][0] = sx(g(c, A2).wrapping_add(g(c, V1)));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1e0));
        c.lqc2(3, frame.at(c, 0x160));
        c.lqc2(4, frame.at(c, 0x1e0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1d0));
        c.lqc2(3, frame.at(c, 0x1d0));
        c.lqc2(4, frame.at(c, 0x1a0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1c0));
        c.sq(2, p(g(c, V1), 0x20));
        c.g[A0][0] = c.lw(p(g(c, S0), 0x24));
        c.lqc2(3, frame.at(c, 0x160));
        c.lqc2(4, frame.at(c, 0x190));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.g[A2][0] = sx(g(c, A2).wrapping_add(g(c, A0)));
        c.sq(2, frame.at(c, 0x1e0));
        c.lqc2(3, frame.at(c, 0x1e0));
        c.lqc2(4, frame.at(c, 0x1b0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1d0));
        c.lqc2(3, frame.at(c, 0x1d0));
        c.lqc2(4, frame.at(c, 0x1b0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1c0));
        c.sq(2, p(g(c, A2), 0x20));
        c.g[A0][0] = c.lw(g(c, S0));
        c.g[V1][0] = c.lw(p(g(c, A0), 0x6c0));
        c.g[A0][0] = sx(p(g(c, A0), 0x6c0));
        c.g[A1][0] = c.lh(p(g(c, V1), 0x38));
        c.g[V0][0] = c.lw(p(g(c, V1), 0x3c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, A1)));
        call_ptr(c, t, 0x002e9d08)?;
        c.g[A0][0] = 0xf0;
        c.g[V1][0] = 0x500000;
        c.mult(0, 2, 2, 4);
        c.g[V1][0] = sx(g(c, V1).wrapping_sub(0x5c68));
        c.g[AT][0] = 0x437f0000;
        c.mtc1(1, 1);
        c.g[T2][0] = 0u64;
        c.g[T0][0] = c.g[S1][0];
        c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, V1)));
        c.ldl(3, p(g(c, V0), 0x7));
        c.ldr(3, g(c, V0));
        c.ldl(4, p(g(c, V0), 0xf));
        c.ldr(4, p(g(c, V0), 0x8));
        c.sdl(3, frame.at(c, 0x1c7));
        c.sdr(3, frame.at(c, 0x1c0));
        c.sdl(4, frame.at(c, 0x1cf));
        c.sdr(4, frame.at(c, 0x1c8));
        // Shift the six strips down by one vertex (`strip[k]` takes the head
        // vertex of `strip[k+1]`), then store the seam value at +0x2c in all six.
        'l_2e9d50: loop {
            'b_2e9d6c: {
                c.cycles += 6;
                c.lwc1(0, frame.at(c, 0x1c0));
                c.mul_s(0, 0, 1);
                c.c_le_s(20, 0);
                let t = !c.fcc();
                c.mov_s(2, 20);
                if t {
                    break 'b_2e9d6c;
                } else {
                    c.cycles += 1;
                    c.min_s(2, 0, 1);
                    break 'b_2e9d6c;
                }
            }
            'b_2e9db4: {
                c.cycles += 17;
                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                c.g[T1][0] = 0x30;
                c.mult(0, 7, 19, 9);
                c.cvt_w_s(0, 2);
                c.mfc1(4, 0);
                c.mult(0, 3, 3, 9);
                c.g[A1][0] = c.lw(g(c, T0));
                c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, A1)));
                c.sw(p(g(c, V0), 0x1c), c.g[A0][0]);
                c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A1)));
                c.sw(p(g(c, V1), 0x1c), c.g[A0][0]);
                c.lwc1(0, frame.at(c, 0x1c4));
                c.mul_s(0, 0, 1);
                c.c_le_s(20, 0);
                let t = !c.fcc();
                c.mov_s(2, 20);
                if t {
                    break 'b_2e9db4;
                } else {
                    c.cycles += 1;
                    c.min_s(2, 0, 1);
                    break 'b_2e9db4;
                }
            }
            'b_2e9df4: {
                c.cycles += 15;
                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                c.cvt_w_s(0, 2);
                c.mfc1(5, 0);
                c.g[A0][0] = c.lw(g(c, T0));
                c.mult(0, 3, 3, 9);
                c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, A0)));
                c.sw(p(g(c, V0), 0x10), c.g[A1][0]);
                c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A0)));
                c.sw(p(g(c, V1), 0x10), c.g[A1][0]);
                c.lwc1(0, frame.at(c, 0x1c8));
                c.mul_s(0, 0, 1);
                c.c_le_s(20, 0);
                let t = !c.fcc();
                c.mov_s(2, 20);
                if t {
                    break 'b_2e9df4;
                } else {
                    c.cycles += 1;
                    c.min_s(2, 0, 1);
                    break 'b_2e9df4;
                }
            }
            'b_2e9e34: {
                c.cycles += 15;
                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                c.cvt_w_s(0, 2);
                c.mfc1(5, 0);
                c.g[A0][0] = c.lw(g(c, T0));
                c.mult(0, 3, 3, 9);
                c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, A0)));
                c.sw(p(g(c, V0), 0x14), c.g[A1][0]);
                c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A0)));
                c.sw(p(g(c, V1), 0x14), c.g[A1][0]);
                c.lwc1(0, frame.at(c, 0x1cc));
                c.mul_s(0, 0, 1);
                c.c_le_s(20, 0);
                let t = !c.fcc();
                c.mov_s(2, 20);
                if t {
                    break 'b_2e9e34;
                } else {
                    c.cycles += 1;
                    c.min_s(2, 0, 1);
                    break 'b_2e9e34;
                }
            }
            c.cycles += 13;
            c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
            c.cvt_w_s(0, 2);
            c.mfc1(6, 0);
            c.g[A1][0] = c.lw(g(c, T0));
            c.g[T2][0] = sx(p(g(c, T2), 0x1));
            c.mult(0, 3, 3, 9);
            c.g[T0][0] = sx(p(g(c, T0), 0x4));
            c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, A1)));
            c.g[A0][0] = ((c.g[T2][0] as i64) < 6i64) as u64;
            c.sw(p(g(c, V0), 0x18), c.g[A2][0]);
            c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A1)));
            let t = c.g[A0][0] != 0u64;
            c.sw(p(g(c, V1), 0x18), c.g[A2][0]);
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002e9d50);
                }
                continue 'l_2e9d50;
            } else {
                c.cycles += 6;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A2][0] = 0x30;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0x5;
                'l_2e9e80: loop {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(p(g(c, S0), 0x8c));
                    c.g[A1][0] = sx(g(c, A1).wrapping_sub(0x1));
                    c.g[V1][0] = c.lw(g(c, A0));
                    c.mult(0, 2, 2, 6);
                    c.g[A0][0] = sx(p(g(c, A0), 0x4));
                    c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, V1)));
                    let t = (c.g[A1][0] as i64) >= 0;
                    c.swc1(0, p(g(c, V0), 0x2c));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002e9e80);
                        }
                        continue 'l_2e9e80;
                    } else {
                        'b_2e9f3c: {
                            c.cycles += 39;
                            c.g[V0][0] = c.lw(p(g(c, S0), 0x8c));
                            c.g[T0][0] = 0x30;
                            c.mult(0, 9, 19, 8);
                            c.g[V1][0] = c.lw(p(g(c, S0), 0x10));
                            c.mult(0, 2, 2, 8);
                            c.mtc1(0, 1);
                            c.g[A2][0] = 0x1;
                            c.g[A0][0] = sx(g(c, T1).wrapping_add(g(c, V1)));
                            c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, V1)));
                            c.lqc2(3, p(g(c, V0), 0x20));
                            c.lqc2(4, p(g(c, A0), 0x20));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, frame.at(c, 0x1e0));
                            c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                            c.g[A0][0] = c.lw(p(g(c, S0), 0x14));
                            c.mult(0, 3, 3, 8);
                            c.lq(2, frame.at(c, 0x1e0));
                            c.g[A1][0] = sx(g(c, T1).wrapping_add(g(c, A0)));
                            c.sq(2, frame.at(c, 0x1d0));
                            c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A0)));
                            c.lqc2(3, p(g(c, V1), 0x20));
                            c.lqc2(4, p(g(c, A1), 0x20));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, frame.at(c, 0x1f0));
                            c.lq(2, frame.at(c, 0x1f0));
                            c.sq(2, frame.at(c, 0x1e0));
                            c.lqc2(3, frame.at(c, 0x1d0));
                            c.lqc2(5, frame.at(c, 0x90));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.qmfc2(3, 4);
                            c.mtc1(3, 0);
                            c.c_lt_s(0, 1);
                            if !c.fcc() {
                                c.g[A2][0] = 0u64;
                                break 'b_2e9f3c;
                            } else {
                                break 'b_2e9f3c;
                            }
                        }
                        'b_2e9f74: {
                            c.cycles += 13;
                            c.lqc2(3, frame.at(c, 0x1e0));
                            c.lqc2(5, frame.at(c, 0x90));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.qmfc2(2, 4);
                            c.mtc1(2, 0);
                            c.c_lt_s(0, 1);
                            let t = c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_2e9f74;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = 0u64;
                                break 'b_2e9f74;
                            }
                        }
                        'b_2ea138: {
                            'b_2ea124: {
                                'b_2ea100: {
                                    'b_2ea0cc: {
                                        c.cycles += 2;
                                        let t = c.g[A2][0] == 0u64;

                                        if t {
                                            break 'b_2ea0cc;
                                        } else {
                                            'b_2ea078: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.lwc1(0, gp_6(c, 0xa3c));
                                                if t {
                                                    break 'b_2ea078;
                                                } else {
                                                    c.cycles += 61;
                                                    c.g[A3][0] = c.lw(p(g(c, S0), 0x10));
                                                    c.g[A2][0] = c.lw(p(g(c, S0), 0x8c));
                                                    c.g[A1][0] = c.lw(p(g(c, S0), 0x18));
                                                    c.g[A0][0] = c.lw(p(g(c, S0), 0x1c));
                                                    c.mult(0, 6, 6, 8);
                                                    c.mfc1(3, 0);
                                                    c.lqc2(4, frame.at(c, 0x90));
                                                    c.qmtc2(3, 3);
                                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                    c.sqc2(5, frame.at(c, 0x210));
                                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x10));
                                                    c.lq(2, frame.at(c, 0x210));
                                                    c.g[A0][0] =
                                                        sx(g(c, A2).wrapping_add(g(c, A0)));
                                                    c.g[V1][0] =
                                                        sx(g(c, T1).wrapping_add(g(c, V1)));
                                                    c.g[A1][0] =
                                                        sx(g(c, A2).wrapping_add(g(c, A1)));
                                                    c.sq(2, frame.at(c, 0x200));
                                                    c.g[A2][0] =
                                                        sx(g(c, A2).wrapping_add(g(c, A3)));
                                                    c.lqc2(3, p(g(c, V1), 0x20));
                                                    c.lqc2(4, frame.at(c, 0x200));
                                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                    c.sqc2(5, frame.at(c, 0x210));
                                                    c.lq(2, frame.at(c, 0x210));
                                                    c.lwc1(0, gp_6(c, 0xa3c));
                                                    c.sq(2, frame.at(c, 0x1f0));
                                                    c.sq(2, p(g(c, A0), 0x20));
                                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                                    c.g[A0][0] = c.lw(p(g(c, S0), 0x1c));
                                                    c.mult(0, 3, 3, 8);
                                                    c.g[A0][0] =
                                                        sx(g(c, A0).wrapping_add(g(c, V1)));
                                                    c.lq(2, p(g(c, A0), 0x20));
                                                    c.sq(2, p(g(c, A1), 0x20));
                                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                                    c.g[A0][0] = c.lw(p(g(c, S0), 0x18));
                                                    c.mult(0, 3, 3, 8);
                                                    c.g[A0][0] =
                                                        sx(g(c, A0).wrapping_add(g(c, V1)));
                                                    c.lq(2, p(g(c, A0), 0x20));
                                                    c.sq(2, p(g(c, A2), 0x20));
                                                    c.g[A1][0] = c.lw(p(g(c, S0), 0x8c));
                                                    c.g[A3][0] = c.lw(p(g(c, S0), 0x14));
                                                    c.g[A2][0] = c.lw(p(g(c, S0), 0x24));
                                                    c.mult(0, 5, 5, 8);
                                                    c.g[A0][0] = c.lw(p(g(c, S0), 0x20));
                                                    c.mfc1(3, 0);
                                                    c.lqc2(4, frame.at(c, 0x90));
                                                    c.qmtc2(3, 3);
                                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                    c.sqc2(5, frame.at(c, 0x210));
                                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x14));
                                                    c.lq(2, frame.at(c, 0x210));
                                                    c.g[A0][0] =
                                                        sx(g(c, A1).wrapping_add(g(c, A0)));
                                                    c.g[V1][0] =
                                                        sx(g(c, T1).wrapping_add(g(c, V1)));
                                                    c.g[A2][0] =
                                                        sx(g(c, A1).wrapping_add(g(c, A2)));
                                                    c.sq(2, frame.at(c, 0x200));
                                                    c.g[A1][0] =
                                                        sx(g(c, A1).wrapping_add(g(c, A3)));
                                                    c.lqc2(3, p(g(c, V1), 0x20));
                                                    c.lqc2(4, frame.at(c, 0x200));
                                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                    c.sqc2(5, frame.at(c, 0x210));
                                                    c.lq(2, frame.at(c, 0x210));
                                                    let t = true;
                                                    c.sq(2, frame.at(c, 0x1f0));
                                                    if t {
                                                        break 'b_2ea100;
                                                    } else {
                                                        break 'b_2ea078;
                                                    }
                                                }
                                            }
                                            c.cycles += 21;
                                            c.g[A1][0] = c.lw(p(g(c, S0), 0x8c));
                                            c.g[V1][0] = c.lw(p(g(c, S0), 0x14));
                                            c.mult(0, 5, 5, 8);
                                            c.g[A0][0] = c.lw(p(g(c, S0), 0x1c));
                                            c.g[A3][0] = c.lw(p(g(c, S0), 0x10));
                                            c.g[A2][0] = c.lw(p(g(c, S0), 0x18));
                                            c.g[V1][0] = sx(g(c, A1).wrapping_add(g(c, V1)));
                                            c.g[A0][0] = sx(g(c, A1).wrapping_add(g(c, A0)));
                                            c.lq(2, p(g(c, V1), 0x20));
                                            c.g[A2][0] = sx(g(c, A1).wrapping_add(g(c, A2)));
                                            c.g[A1][0] = sx(g(c, A1).wrapping_add(g(c, A3)));
                                            c.sq(2, p(g(c, A0), 0x20));
                                            c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                            c.g[A0][0] = c.lw(p(g(c, S0), 0x1c));
                                            c.mult(0, 3, 3, 8);
                                            c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                                            c.lq(2, p(g(c, A0), 0x20));
                                            c.sq(2, p(g(c, A2), 0x20));
                                            c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                            let t = true;
                                            c.g[A0][0] = c.lw(p(g(c, S0), 0x18));
                                            if t {
                                                break 'b_2ea124;
                                            } else {
                                                break 'b_2ea0cc;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw(p(g(c, S0), 0x94));
                                        break 'b_2ea138;
                                    } else {
                                        c.cycles += 11;
                                        c.g[A1][0] = c.lw(p(g(c, S0), 0x8c));
                                        c.g[V1][0] = c.lw(p(g(c, S0), 0x10));
                                        c.mult(0, 5, 5, 8);
                                        c.g[A0][0] = c.lw(p(g(c, S0), 0x20));
                                        c.g[A3][0] = c.lw(p(g(c, S0), 0x14));
                                        c.g[A2][0] = c.lw(p(g(c, S0), 0x24));
                                        c.g[V1][0] = sx(g(c, A1).wrapping_add(g(c, V1)));
                                        c.g[A0][0] = sx(g(c, A1).wrapping_add(g(c, A0)));
                                        c.lq(2, p(g(c, V1), 0x20));
                                        c.g[A2][0] = sx(g(c, A1).wrapping_add(g(c, A2)));
                                        c.g[A1][0] = sx(g(c, A1).wrapping_add(g(c, A3)));
                                        break 'b_2ea100;
                                    }
                                }
                                c.cycles += 9;
                                c.sq(2, p(g(c, A0), 0x20));
                                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                c.g[A0][0] = c.lw(p(g(c, S0), 0x20));
                                c.mult(0, 3, 3, 8);
                                c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                                c.lq(2, p(g(c, A0), 0x20));
                                c.sq(2, p(g(c, A2), 0x20));
                                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                c.g[A0][0] = c.lw(p(g(c, S0), 0x24));
                                break 'b_2ea124;
                            }
                            c.cycles += 5;
                            c.mult(0, 3, 3, 8);
                            c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                            c.lq(2, p(g(c, A0), 0x20));
                            c.sq(2, p(g(c, A1), 0x20));
                            c.g[V0][0] = c.lw(p(g(c, S0), 0x94));
                            break 'b_2ea138;
                        }
                        // Post-sample bookkeeping: raise the sample count (`+0x90`) by three,
                        // clamped at 54, and mark the trail running (`+0x94` = 1).
                        'b_2ea28c: {
                            c.cycles += 2;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V1][0] = 0x36;
                            if t {
                                break 'b_2ea28c;
                            } else {
                                'b_2ea154: {
                                    c.cycles += 5;
                                    c.g[V0][0] = sx(p(g(c, S3), 0x35));
                                    c.div(0, 2, 3);
                                    c.g[A0][0] = 0x30;
                                    if c.g[V1][0] == 0u64 {
                                        break 'b_2ea154;
                                    } else {
                                        break 'b_2ea154;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A2][0] = c.g[S1][0];
                                c.g[T1][0] = 0x5;
                                c.g[A1][0] = c.hi[0];
                                c.mult(0, 8, 19, 4);
                                c.mult(0, 7, 5, 4);
                                'l_2ea168: loop {
                                    c.cycles += 32;
                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x14));
                                    c.g[T1][0] = sx(g(c, T1).wrapping_sub(0x1));
                                    c.g[A0][0] = c.lw(g(c, A2));
                                    c.g[V1][0] = sx(g(c, T0).wrapping_add(g(c, V1)));
                                    c.lq(2, p(g(c, V1), 0x20));
                                    c.g[A0][0] = sx(g(c, A3).wrapping_add(g(c, A0)));
                                    c.sq(2, p(g(c, A0), 0x20));
                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x14));
                                    c.g[A0][0] = c.lw(g(c, A2));
                                    c.g[V1][0] = sx(g(c, T0).wrapping_add(g(c, V1)));
                                    c.g[A1][0] = c.lw(p(g(c, V1), 0x10));
                                    c.g[A0][0] = sx(g(c, A3).wrapping_add(g(c, A0)));
                                    c.sw(p(g(c, A0), 0x10), c.g[A1][0]);
                                    c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                                    c.g[V1][0] = c.lw(g(c, A2));
                                    c.g[V0][0] = sx(g(c, T0).wrapping_add(g(c, V0)));
                                    c.g[A0][0] = c.lw(p(g(c, V0), 0x14));
                                    c.g[V1][0] = sx(g(c, A3).wrapping_add(g(c, V1)));
                                    c.sw(p(g(c, V1), 0x14), c.g[A0][0]);
                                    c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                                    c.g[V1][0] = c.lw(g(c, A2));
                                    c.g[V0][0] = sx(g(c, T0).wrapping_add(g(c, V0)));
                                    c.g[A0][0] = c.lw(p(g(c, V0), 0x18));
                                    c.g[V1][0] = sx(g(c, A3).wrapping_add(g(c, V1)));
                                    c.sw(p(g(c, V1), 0x18), c.g[A0][0]);
                                    c.g[V0][0] = c.lw(g(c, A2));
                                    c.g[A2][0] = sx(p(g(c, A2), 0x4));
                                    c.g[V1][0] = sx(g(c, T0).wrapping_add(g(c, V0)));
                                    c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, V0)));
                                    c.sw(p(g(c, V1), 0x1c), 0u64);
                                    let t = (c.g[T1][0] as i64) >= 0;
                                    c.sw(p(g(c, V0), 0x1c), 0u64);
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x002ea168);
                                        }
                                        continue 'l_2ea168;
                                    } else {
                                        'b_2ea284: {
                                            c.cycles += 30;
                                            c.g[A0][0] = 0x30;
                                            c.g[V0][0] = c.lw(p(g(c, S0), 0x10));
                                            c.mult(0, 4, 19, 4);
                                            c.g[A1][0] = c.lw(p(g(c, S0), 0x1c));
                                            c.g[A2][0] = c.lw(p(g(c, S0), 0x18));
                                            c.g[V0][0] = sx(g(c, A0).wrapping_add(g(c, V0)));
                                            c.g[A1][0] = sx(g(c, A0).wrapping_add(g(c, A1)));
                                            c.lq(3, p(g(c, V0), 0x20));
                                            c.g[A2][0] = sx(g(c, A0).wrapping_add(g(c, A2)));
                                            c.sq(3, p(g(c, A1), 0x20));
                                            c.g[V0][0] = c.lw(p(g(c, S0), 0x1c));
                                            c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, A0)));
                                            c.lq(3, p(g(c, V0), 0x20));
                                            c.sq(3, p(g(c, A2), 0x20));
                                            c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                                            c.g[V1][0] = c.lw(p(g(c, S0), 0x20));
                                            c.g[V0][0] = sx(g(c, A0).wrapping_add(g(c, V0)));
                                            c.g[A2][0] = c.lw(p(g(c, S0), 0x24));
                                            c.lq(5, p(g(c, V0), 0x20));
                                            c.g[V1][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                                            c.g[A2][0] = sx(g(c, A0).wrapping_add(g(c, A2)));
                                            c.sq(5, p(g(c, V1), 0x20));
                                            c.g[V0][0] = c.lw(p(g(c, S0), 0x20));
                                            c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, A0)));
                                            c.lq(3, p(g(c, V0), 0x20));
                                            c.sq(3, p(g(c, A2), 0x20));
                                            c.g[A0][0] = c.lw(p(g(c, S0), 0x90));
                                            c.g[V0][0] = ((c.g[A0][0] as i64) < 54i64) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = sx(p(g(c, A0), 0x1));
                                            if t {
                                                break 'b_2ea284;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V1][0] = ((c.g[V0][0] as i64) < 54i64) as u64;
                                                let t = c.g[V1][0] == 0u64;
                                                c.sw(p(g(c, S0), 0x90), c.g[V0][0]);
                                                if t {
                                                    break 'b_2ea284;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[V0][0] = sx(p(g(c, A0), 0x2));
                                                    c.g[V1][0] =
                                                        ((c.g[V0][0] as i64) < 54i64) as u64;
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.sw(p(g(c, S0), 0x90), c.g[V0][0]);
                                                    if t {
                                                        break 'b_2ea284;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.g[V0][0] = sx(p(g(c, A0), 0x3));
                                                        c.sw(p(g(c, S0), 0x90), c.g[V0][0]);
                                                        break 'b_2ea284;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] = 0x1;
                                        c.sw(p(g(c, S0), 0x94), c.g[V0][0]);
                                        break 'b_2ea28c;
                                    }
                                }
                            }
                        }
                        // Ask `cBoardTrailFX_ShouldEmit` whether this frame emits a sample;
                        // when it does, write the colour at `+0xd0`, blend the fades at
                        // `+0x04`/`+0x08` and shift the strips again.
                        'b_2ea428: {
                            c.cycles += 13;
                            c.lqc2(3, frame.at(c, 0x90));
                            c.lqc2(5, p(g(c, S0), 0xc0));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.qmfc2(2, 4);
                            c.mtc1(2, 12);
                            c.g[A0][0] = c.g[S0][0];
                            callee::SHOULD_EMIT.call(c, 0x002e_a2b8)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw(gp_6(c, 0xa0c));
                            if t {
                                break 'b_2ea428;
                            } else {
                                'b_2ea360: {
                                    c.cycles += 40;
                                    c.g[A2][0] = 0x10000;
                                    c.g[T2][0] = c.lw(p(g(c, S0), 0x8c));
                                    c.g[A2][0] = c.g[A2][0] | 0x8fcd;
                                    c.mult(0, 2, 2, 6);
                                    c.g[T0][0] = 0x36;
                                    c.g[V1][0] = sx(p(g(c, T2), 0x1));
                                    c.g[A0][0] = 0xe90000;
                                    c.div(0, 3, 8);
                                    c.g[A0][0] = c.g[A0][0] | 0x507c;
                                    c.g[A1][0] = 0x7f0000;
                                    c.lq(3, frame.at(c, 0xa0));
                                    c.g[A1][0] = c.g[A1][0] | 0xffff;
                                    c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, A0)));
                                    c.g[A3][0] = 0x3f800000;
                                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                                    c.sq(3, p(g(c, S0), 0xd0));
                                    c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                                    c.sw(gp_6(c, 0xa0c), c.g[V0][0]);
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 2);
                                    c.lwc1(1, gp_6(c, 0xa0c));
                                    c.lwc1(0, gp_6(c, 0x14bc));
                                    c.sub_s(1, 1, 2);
                                    c.mul_s(0, 0, 1);
                                    c.add_s(0, 0, 2);
                                    c.swc1(0, p(g(c, S0), 0x4));
                                    c.lwc1(1, gp_6(c, 0x14bc));
                                    c.g[T1][0] = c.hi[0];
                                    c.mult(0, 3, 2, 6);
                                    c.g[V0][0] = sx(g(c, V1).wrapping_add(g(c, A0)));
                                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                                    c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                                    c.sw(gp_6(c, 0xa0c), c.g[V0][0]);
                                    c.lwc1(0, gp_6(c, 0xa0c));
                                    c.sub_s(0, 0, 2);
                                    c.mul_s(1, 1, 0);
                                    c.add_s(1, 1, 2);
                                    c.swc1(1, p(g(c, S0), 0x8));
                                    if c.g[T0][0] == 0u64 {
                                        break 'b_2ea360;
                                    } else {
                                        break 'b_2ea360;
                                    }
                                }
                                'b_2ea378: {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x90));
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 54i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.sw(p(g(c, S0), 0x8c), c.g[T1][0]);
                                    if t {
                                        break 'b_2ea378;
                                    } else {
                                        c.cycles += 2;
                                        c.g[V0][0] = sx(p(g(c, V1), 0x1));
                                        c.sw(p(g(c, S0), 0x90), c.g[V0][0]);
                                        break 'b_2ea378;
                                    }
                                }
                                c.cycles += 4;
                                c.g[T0][0] = 0x30;
                                c.g[A2][0] = c.g[S1][0];
                                c.mult(0, 7, 10, 8);
                                c.g[T1][0] = 0x5;
                                'l_2ea388: loop {
                                    c.cycles += 38;
                                    c.g[A1][0] = c.lw(p(g(c, S0), 0x8c));
                                    c.g[T1][0] = sx(g(c, T1).wrapping_sub(0x1));
                                    c.g[A0][0] = c.lw(g(c, A2));
                                    c.mult(0, 5, 5, 8);
                                    c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, A0)));
                                    c.lq(3, p(g(c, V0), 0x20));
                                    c.g[A1][0] = sx(g(c, A1).wrapping_add(g(c, A0)));
                                    c.sq(3, p(g(c, A1), 0x20));
                                    c.g[A0][0] = c.lw(p(g(c, S0), 0x8c));
                                    c.g[V1][0] = c.lw(g(c, A2));
                                    c.mult(0, 4, 4, 8);
                                    c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, V1)));
                                    c.g[A1][0] = c.lw(p(g(c, V0), 0x1c));
                                    c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                                    c.sw(p(g(c, A0), 0x1c), c.g[A1][0]);
                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                    c.g[A0][0] = c.lw(g(c, A2));
                                    c.mult(0, 3, 3, 8);
                                    c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, A0)));
                                    c.g[A1][0] = c.lw(p(g(c, V0), 0x10));
                                    c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A0)));
                                    c.sw(p(g(c, V1), 0x10), c.g[A1][0]);
                                    c.g[A0][0] = c.lw(p(g(c, S0), 0x8c));
                                    c.g[V1][0] = c.lw(g(c, A2));
                                    c.mult(0, 4, 4, 8);
                                    c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, V1)));
                                    c.g[A1][0] = c.lw(p(g(c, V0), 0x14));
                                    c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                                    c.sw(p(g(c, A0), 0x14), c.g[A1][0]);
                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                    c.g[A1][0] = c.lw(g(c, A2));
                                    c.mult(0, 3, 3, 8);
                                    c.g[A2][0] = sx(p(g(c, A2), 0x4));
                                    c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, A1)));
                                    c.g[A0][0] = c.lw(p(g(c, V0), 0x18));
                                    c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A1)));
                                    let t = (c.g[T1][0] as i64) >= 0;
                                    c.sw(p(g(c, V1), 0x18), c.g[A0][0]);
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x002ea388);
                                        }
                                        continue 'l_2ea388;
                                    } else {
                                        c.cycles += 2;
                                        c.g[V0][0] = 0x2;
                                        c.sw(p(g(c, S0), 0x94), c.g[V0][0]);
                                        break 'b_2ea428;
                                    }
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[V0][0] = c.lw(p(g(c, S0), 0xc));
                        c.g[V1][0] = 0x1;
                        c.lq(4, frame.at(c, 0x90));
                        c.g[V1][0] = sx(g(c, V1).wrapping_sub(g(c, V0)));
                        c.sw(p(g(c, S0), 0xc), c.g[V1][0]);
                        c.sq(4, p(g(c, S0), 0xc0));
                        break 'b_2ea440;
                    }
                }
            }
        }
    }
    c.cycles += 15;
    c.lq(16, frame.at(c, 0x260));
    c.lq(17, frame.at(c, 0x250));
    c.lq(18, frame.at(c, 0x240));
    c.lq(19, frame.at(c, 0x230));
    c.g[RA][0] = c.ld(frame.at(c, 0x220));
    c.lwc1(27, frame.at(c, 0x2a8));
    c.lwc1(26, frame.at(c, 0x2a0));
    c.lwc1(25, frame.at(c, 0x298));
    c.lwc1(24, frame.at(c, 0x290));
    c.lwc1(23, frame.at(c, 0x288));
    c.lwc1(22, frame.at(c, 0x280));
    c.lwc1(21, frame.at(c, 0x278));
    c.lwc1(20, frame.at(c, 0x270));
    frame.close(c);
    abi::ret(c);
    Ok(())
}

/// `TransitionWipe_Draw` (0x002ebc40): draw the screen-wipe quad of a
/// transition effect.  `t` (f12) is the wipe progress, `self+0x14`/`self+0x18`
/// the wipe direction/size in pixels; the quad's screen rectangle is derived
/// from 40.0 and -480.0 and its texture coordinates from the globals at
/// `gp+0xa40..0xa54`, then submitted with the effect colour at `self+4`.
pub fn transition_wipe_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x30);
        let self_ = c.g[A0][0] as u32;
        let gp = c.g[GP][0] as u32;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1); // f1 = 1.0
        frame.save_ra(c, 0x20);
        c.sub_s(1, 1, 12); // f1 = 1.0 - t
        c.g[AT][0] = 0x4420_0000;
        c.mtc1(1, 3); // f3 = 40.0
        c.lwc1(2, self_.wrapping_add(0x14));
        c.cvt_s_w(2, 2); // f2 = (float)dx
        c.g[AT][0] = 0xffff_ffff_c3f0_0000;
        c.mtc1(1, 4); // f4 = -480.0
        c.lwc1(0, self_.wrapping_add(0x18));
        c.cvt_s_w(0, 0); // f0 = (float)dy
        c.mov_s(8, 3); // f8 = 40.0
        c.mtc1(0, 7); // f7 = (float)dy
        c.mul_s(2, 2, 1); // f2 = dx * k
        c.mul_s(0, 0, 1); // f0 = dy * k
        c.mul_s(5, 2, 3); // f5 = dx * k * 40.0
        c.c_lt_s(5, 7);
        let dx_left = !c.fcc();
        c.mul_s(6, 0, 4); // f6 = dy * k * -480.0
        if dx_left {
            c.cycles += 7;
            c.lwc1(0, gp.wrapping_add(0xa48));
            c.sub_s(8, 8, 5); // f8 = 40.0 - dx
            c.lwc1(1, gp.wrapping_sub(0x39a0));
            c.lwc1(7, gp.wrapping_add(0xa40));
            c.mul_s(0, 0, 1);
            c.mul_s(0, 0, 8);
            c.add_s(10, 7, 0); // u1 = base + step * f8
        } else {
            c.cycles += 10;
            c.lwc1(0, gp.wrapping_add(0xa48));
            c.add_s(8, 5, 8); // f8 = dx + 40.0
            c.lwc1(1, gp.wrapping_sub(0x39a4));
            c.lwc1(2, gp.wrapping_add(0xa40));
            c.mul_s(0, 0, 1);
            c.lwc1(10, gp.wrapping_add(0xa44));
            c.mul_s(0, 0, 5);
            c.mov_s(5, 7);
            c.add_s(7, 2, 0); // u0 = base + step * f5
        }
        c.cycles += 5;
        c.mtc1(0, 9); // f9 = 0.0
        c.c_lt_s(6, 9);
        let dy_top = !c.fcc();
        c.lwc1(0, gp.wrapping_add(0xa54));
        if dy_top {
            c.cycles += 9;
            c.lwc1(2, gp.wrapping_sub(0x3998));
            c.g[AT][0] = 0x43f0_0000;
            c.mtc1(1, 0); // f0 = 480.0
            c.lwc1(1, gp.wrapping_add(0xa54));
            c.sub_s(4, 0, 6); // f4 = 480.0 - dy
            c.lwc1(3, gp.wrapping_add(0xa4c));
            c.mul_s(1, 1, 2);
            c.mul_s(1, 1, 4);
            c.add_s(2, 3, 1); // v1 = base + step * f4
        } else {
            c.cycles += 11;
            c.lwc1(1, gp.wrapping_sub(0x399c));
            c.g[AT][0] = 0x43f0_0000;
            c.mtc1(1, 2); // f2 = 480.0
            c.mul_s(0, 0, 1);
            c.lwc1(3, gp.wrapping_add(0xa4c));
            c.add_s(4, 6, 2); // f4 = dy + 480.0
            c.lwc1(2, gp.wrapping_add(0xa50));
            c.mul_s(0, 0, 6);
            c.mov_s(6, 9); // dy = 0.0
            c.add_s(3, 3, 0); // v0 = base + step * f6
        }
        c.cycles += 18;
        c.sub_s(1, 10, 7); // uw = u1 - u0
        c.g[A2][0] = sx(self_.wrapping_add(0x4));
        c.sub_s(0, 2, 3); // vh = v1 - v0
        c.swc1(5, c.g[SP][0] as u32);
        c.mtc1(0, 12); // f12 = 0.0
        c.g[A0][0] = c.g[SP][0];
        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.g[RA][0] = sx(0x002e_bd7c);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x1c));
        call_ptr(c, 0x002e_b938, 0x002e_bd7c)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as RET;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_15 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::fepopup::tests::f_2;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::render::particle::tests::clear;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::vtable_2;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_4;
    pub(crate) use crate::game::util::menu::tests::wf_2 as wf;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::delta;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::f;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::gfx_fixture;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::gp_at;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::slot;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS_2;
    pub(crate) use crate::game::world::worldcache::tests::check_all as check_all_2;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn c_cut_scene_transition_man_start_out_matches() {
        check(
            ssx3_ee::s09::f_002ea780,
            c_cut_scene_transition_man_start_out,
            300,
            &|c, seed| {
                let mut r = Rng(0x100b ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let state = match seed % 8 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 4,
                    5 => 0x8000_0000, // sign-extends to a negative i64
                    6 => 0xffff_ffff,
                    _ => r.next_u32(),
                };
                c.bus.write32(DATA + 0x10, state);
                c.bus.write32(DATA + 0x14, r.edge_f32());
                c.bus.write32(DATA + 0x18, r.edge_f32());
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn c_cut_scene_transition_man_set_paused_matches() {
        check(
            ssx3_ee::s09::f_002ea8b8,
            c_cut_scene_transition_man_set_paused,
            300,
            &|c, seed| {
                let mut r = Rng(0x100c ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    ((r.next_u32() as u64) << 32) | 1
                };
                c.bus.write32(
                    DATA + 0x10,
                    match seed % 3 {
                        0 => 2,
                        1 => 3,
                        _ => r.next_u32(),
                    },
                );
                c.bus.write32(
                    DATA + 0x1c,
                    match seed % 4 {
                        0 => 0,
                        1 => r.next_u32() & 0x7fff_ffff,
                        2 => 0x8000_0000,
                        _ => r.next_u32(),
                    },
                );
            },
        );
    }

    #[test]
    fn rider_base_reset_power_up_trail_matches() {
        check(
            ssx3_ee::s09::f_002eadc0,
            rider_base_reset_power_up_trail,
            100,
            &|c, seed| {
                let mut r = Rng(0x100d ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0xc, r.next_u32());
                c.bus.write32(DATA + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn rider_base_render_power_up_trail_matches() {
        covered(
            ssx3_ee::s09::f_002eb198__generated,
            rider_base_render_power_up_trail,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2eb1_98, 4);
                gfx_fixture(c, &mut r);
                let this = DATA;
                fill(c, &mut r, this, 0x40);
                // 0/1/negative: nothing drawn; 2 or 3 sections (3 fills the strips).
                let n = match seed % 4 {
                    0 => [0, 1, (-3i32) as u32][(r.next_u32() % 3) as usize],
                    1 => 2,
                    _ => 3,
                };
                c.bus.write32(this + trail::COUNT, n);
                c.bus.write32(this + trail::FIRST, r.next_u32() % 16);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn rider_base_update_power_up_trail_matches() {
        check(
            ee::s09::f_002eadd0__generated,
            rider_base_update_power_up_trail,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ead, 0);
                let (s, rider, model, skel, nodes, buf) = (
                    DATA,
                    DATA + 0x2000,
                    DATA + 0x4000,
                    DATA + 0x4400,
                    DATA + 0x6000,
                    DATA + 0x4d30,
                );
                c.bus.write32(s, rider);
                c.bus.write32(s + 0xc, r.next_u32() % 5);
                c.bus.write32(s + 0x10, r.next_u32() % 3);
                for k in 0..4u32 {
                    c.bus.write32(s + 0xe0 + 4 * k, r.next_u32());
                    c.bus.write32(s + 0xf0 + 4 * k, r.next_u32());
                }
                c.bus
                    .write32(rider + 0x2ec, f([-1.0f32, 0.5, 5.0][(seed % 3) as usize]));
                c.bus.write32(rider + 0x77c, model);
                c.bus.write32(rider + 0x780, skel);
                c.bus.write32(rider + 0x8a4, r.next_u32() % 4);
                c.bus.write32(skel + 0x30, nodes);
                c.bus.write32(model + 0xde0, (seed % 2) as u32);
                for off in [0u32, 0x10, 0x20, 0x30, 0x50, 0x64, 0x104, 0x114, 0x124] {
                    c.bus.write32(buf + off, r.next_u32());
                }
                for i in 0..4u32 {
                    for k in 0..16u32 {
                        c.bus.write32(nodes + 0x40 * i + 4 * k, r.edge_f32());
                    }
                }
                c.bus
                    .write32(gp_at(0x1630), if seed % 5 == 0 { 1 } else { 0 });
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn c_board_trail_fx_is_active_matches() {
        check(
            ee::s09::f_002e87e8__generated,
            c_board_trail_fx_is_active,
            300,
            &|c, seed| {
                machine(c, seed, 0x2e87e8, 3);
                let fx = DATA;
                let owner = DATA + 0x100;
                let state = DATA + 0x400;
                let anim = DATA + 0x500;
                let seq = DATA + 0x600;
                c.bus.write32(fx, owner);
                c.bus.write32(owner + 0x77c, state);
                c.bus.write32(
                    state + 0xde0,
                    [0u32, 1, 2, 3, 4, 13, 14, 22][(seed % 8) as usize],
                );
                c.bus
                    .write32(state + 0x30, if seed % 3 == 0 { 1 } else { 0 });
                c.bus.write32(
                    owner + 0x438,
                    [0u32, 1, 2, 3, 4, 5, 13, 14, 0x30][(seed % 9) as usize],
                );
                c.bus.write32(owner + 0x784, anim);
                c.bus
                    .write32(anim + 8, if seed % 4 == 1 { 0x16 } else { 0x10 });
                c.bus.write32(anim + 0x50, seq);
                c.bus.write32(seq + 0x14, seq); // a one-element sequence list
                c.bus.write32(seq + 0xb0, if seed % 2 == 0 { 1 } else { 0 });
                c.bus.write32(seq + 0xb4, if seed % 5 == 2 { 1 } else { 0 });
                c.bus
                    .write32(owner + 0xac4, if seed % 3 == 1 { 1 } else { 0 });
                c.bus
                    .write32(owner + 0xad0, if seed % 2 == 0 { 1 } else { 0 });
                c.bus
                    .write32(owner + 0xafc, if seed % 7 == 3 { 0 } else { 1 });
                c.bus
                    .write32(owner + 0xb00, if seed % 4 == 0 { 0 } else { 1 });
                // The state switch's table (all entries are blocks of the
                // function itself: every state below 14 reaches one of them).
                let targets = [
                    0x002e_88f8u32,
                    0x002e_8900,
                    0x002e_8908,
                    0x002e_8910,
                    0x002e_8920,
                ];
                for i in 0..14u32 {
                    c.bus
                        .write32(0x0048_7b40 + 4 * i, targets[(i % 5) as usize]);
                }
                put_words(
                    c,
                    GAME_GP + 0x14a0,
                    &[
                        0x3f80_0000,
                        0x3f80_0000,
                        0x4000_0000,
                        0x3f00_0000,
                        0x3f40_0000,
                    ],
                );
                c.pc = 0x002e_87e8;
                c.g[A0][0] = sx(fx);
            },
        );
    }

    // ------------------------------------------------- visual fx and memory

    #[test]
    fn cut_scene_transition_man_force_off_matches() {
        check(
            ssx3_ee::s09::f_002ea820__generated,
            cut_scene_transition_man_force_off,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x2ea8_20, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x10, if seed % 2 == 0 { 0 } else { 1 });
                if seed % 4 == 3 {
                    w(c, GAME_GP + 0x2a90, DATA + 0x200);
                }
            },
        );
    }

    #[test]
    fn c_cut_scene_transition_man_ctor_matches() {
        covered(
            ee::s09::f_002ea670__generated,
            c_cut_scene_transition_man_ctor,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e_a670, 1);
                let obj = DATA + 0x1000;
                let world = DATA + 0x8000;
                c.bus.write32(GAME_GP + 0x2a90, world);
                let vt = vtable(c, &mut r, &[0x150]);
                c.bus.write32(world + 0x10d8, vt);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn c_cut_scene_transition_man_force_on_matches() {
        check(
            ee::s09::f_002ea860__generated,
            c_cut_scene_transition_man_force_on,
            SEEDS_4,
            &|c, seed| {
                let _ = common(c, seed, 0x2ea860, 1);
                let this = DATA + 0x100;
                let app = DATA + 0x200;
                clear(c, this, 0x100);
                clear(c, app, 0x1200);
                c.bus.write32(this + 0x10, (seed % 2) as u32);
                c.bus.write32(app + 0x10d8, DATA + 0x300);
                vtable_2(c, DATA + 0x300, &[(0x148, 0, FAKE)]);
                c.bus.write32(gp_at(0x2a90), app);
                c.g[A0][0] = sx(this);
            },
        );
    }

    pub(crate) fn u64v(r: &mut Rng) -> u64 {
        ((r.next_u32() as u64) << 32) | r.next_u32() as u64
    }

    #[test]
    fn transition_wipe_ctor_matches() {
        check(
            ee::s09::f_002ebba8__generated,
            transition_wipe_ctor,
            SEEDS_5,
            &|c, seed| {
                let mut r = base(c, seed, 0xebba, 0);
                let this = DATA;
                let arg = DATA + 0x1000;
                for i in 0..4 {
                    w(c, arg + 4 * i, r.next_u32());
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = arg as u64;
                c.g[A2][0] = u64v(&mut r);
                c.g[A3][0] = u64v(&mut r);
            },
        );
    }

    #[test]
    fn transition_effect_draw_fade_matches() {
        check(
            ee::s09::f_002ebb10__generated,
            transition_effect_draw_fade,
            SEEDS_5,
            &|c, seed| {
                let mut r = base(c, seed, 0xbb10, 2);
                let this = DATA;
                for i in 0..4 {
                    w(c, this + 4 + 4 * i, r.next_u32());
                }
                c.g[A0][0] = this as u64;
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn c_board_trail_fx_render_matches() {
        check(
            ee::s09::f_002ea480__generated,
            c_board_trail_fx_render,
            SEEDS_6,
            &|c, seed| {
                let mut r = base(c, seed, 0x2ea4, 0);
                w(c, GAME_GP + 0x1428, 0);
                w(c, GAME_GP + 0x142c, 0);
                let holder = DATA + 0x2_000;
                w(c, GAME_GP.wrapping_sub(0x854), holder);
                w(c, holder + 0xe84, DATA + 0x900);
                for i in 0..0x30u32 {
                    w(c, DATA + 0x900 + 4 * i, r.next_u32());
                }
                w(c, DATA + 0x90, seed % 4);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn ccutscene_transition_man_save_viewport_matches() {
        check(
            ee::s09::f_002eaa28__generated,
            ccutscene_transition_man_save_viewport,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e_aa28, 1);
                let this = DATA;
                c.g[A0][0] = sx(this);
                let world = DATA + 0x3000;
                let vt = DATA + 0x3400;
                c.bus.write32(GAME_GP + 0x2a90, world);
                c.bus.write32(world + 0x10d8, vt);
                c.bus.write16(vt + 0x150, 0);
                c.bus.write32(vt + 0x154, FAKE);
                for i in 0..8u32 {
                    c.bus.write32(this + 0x20 + i * 4, r.next_u32());
                }
                c.bus.write32(
                    this + 0x20,
                    if seed % 3 == 0 { 2 } else { r.next_u32() % 3 },
                );
                c.bus.write32(GAME_GP.wrapping_sub(0x39a8), r.edge_f32());
            },
        );
    }

    #[test]
    fn c_cut_scene_transition_man_start_in_matches() {
        check_all(
            ee::s09::f_002ea6b8__generated,
            c_cut_scene_transition_man_start_in,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ea6, 2);
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.f[12] = r.f32(2.0);
                c.bus.write32(this + 0x10, (seed % 5) as u32);
                c.bus.write32(this + 0x14, r.f32(1.0));
                c.bus.write32(this + 0x18, r.f32(2.0));
            },
        );
    }

    // -- 0x001b7650 cFEStateNetAccountMem_ShowPopup ---------------------

    // -- 0x002ebe20 TransitionEffect_ctorRandom2 ------------------------

    #[test]
    fn transition_effect_ctor_random2_matches() {
        check(
            ssx3_ee::s09::f_002ebe20__generated,
            transition_effect_ctor_random2,
            120,
            &|c, seed| {
                let mut r = Rng(0x2ebe20 ^ (seed as u64) << 3);
                c.g[GP][0] = GAME_GP as u64;
                c.g[SP][0] = crate::difftest::SP as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                let this = DATA + 0x1000;
                let v = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(v);
                for i in 0..4u32 {
                    wf(c, v + 4 * i, r.f32(1.0));
                }
                for i in 0..0x20u32 {
                    w(c, this + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// The trail object.
    pub(crate) const SELF: u32 = DATA + 0x1000;

    /// The rider the trail belongs to (`*(self)`).
    pub(crate) const RIDER_2: u32 = DATA + 0x4000;

    /// The zeroed BELibrary dummy (`gp_6-0x1f04`).
    pub(crate) const BE_3: u32 = DATA + 0x4800;

    /// The dummy world (`gp_6-0x848`) with its module and race manager.
    pub(crate) const WORLD_2: u32 = DATA + 0x5000;

    /// The rider's animation controller (`rider+0x784`).
    pub(crate) const ANIM: u32 = DATA + 0x6000;

    /// The rider's animation node (`rider+0x780`) and its entry table.
    pub(crate) const ANIM_NODE: u32 = DATA + 0x7000;

    pub(crate) const ANIM_TABLE: u32 = DATA + 0x20000;

    /// The rider's animation-base object (`rider+0x6c0`): a delta at +0x38
    /// and the virtual method at +0x3c, which the fixture points at [`FAKE`].
    pub(crate) const VSELF: u32 = DATA + 0x8000;

    /// The six 54-vertex strips (`self+0x10..+0x24`).
    pub(crate) const STRIPS: u32 = DATA + 0x10000;

    pub(crate) const STRIP_STRIDE: u32 = 54 * 0x30;

    /// Control words the fake callees read.
    pub(crate) const CTRL: u32 = DATA + 0x200;

    /// `difftest::check`, also requiring the caller-saved scratch state and
    /// LO/HI to match: the fake callees hash every register into their result
    /// and into RAM, and `div`/`mult` leave LO/HI a later read could see.
    pub(crate) fn check_strict(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..seed_count(seeds) {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(
                r.soft.is_empty(),
                "seed {seed} (scratch state): {:#?}",
                r.soft
            );
            assert!(r.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", r.lohi);
        }
    }

    /// `cBoardTrailFX_IsActive(self)`: bit 0 of the control word is the
    /// answer, the fade it stores at `self+0x9c` comes from the upper bits.
    pub(crate) fn fake_is_active(c: &mut Cpu) {
        let r = c.bus.read32(CTRL);
        w(c, SELF + 0x9c, r >> 8);
        c.g[V0][0] = (r & 1) as u64;
        abi::ret(c);
    }

    /// `func_0011FE98(rider)`: the rider sub-state. 0/1/2/12 reach every
    /// comparison the body makes on it.
    pub(crate) fn fake_sub_state(c: &mut Cpu) {
        let r = c.bus.read32(CTRL + 4);
        c.g[V0][0] = [0u64, 1, 2, 0xc][(r & 3) as usize];
        abi::ret(c);
    }

    /// `func_00312AA0(animbase, 2)`: the animation state. The body tests
    /// `(v0 - 0x17) < 8`, so 0x16/0x17/0x1e/0x1f cover both sides.
    pub(crate) fn fake_anim_state(c: &mut Cpu) {
        let r = c.bus.read32(CTRL + 8);
        c.g[V0][0] = [0x16u64, 0x17, 0x1e, 0x1f][(r & 3) as usize];
        abi::ret(c);
    }

    /// `func_0031C228(x)`: a float blend helper returning f0 (the body then
    /// does `mov.s f12, f0`).
    pub(crate) fn fake_blend(c: &mut Cpu) {
        let r = c.bus.read32(CTRL + 0xc);
        c.f[0] = r;
        abi::ret(c);
    }

    /// Installs the fake callees and puts the generated defaults back.
    pub(crate) struct Slots {
        is_active: fn(&mut Cpu),
        sub_state: fn(&mut Cpu),
        anim_state: fn(&mut Cpu),
        blend: fn(&mut Cpu),
    }

    impl Slots {
        fn install() -> Slots {
            unsafe {
                let saved = Slots {
                    is_active: ssx3_ee::s09::OVERRIDE_002E87E8,
                    sub_state: ssx3_ee::s00::OVERRIDE_0011FE98,
                    anim_state: ssx3_ee::s10::OVERRIDE_00312AA0,
                    blend: ssx3_ee::s10::OVERRIDE_0031C228,
                };
                ssx3_ee::s09::OVERRIDE_002E87E8 = fake_is_active;
                ssx3_ee::s00::OVERRIDE_0011FE98 = fake_sub_state;
                ssx3_ee::s10::OVERRIDE_00312AA0 = fake_anim_state;
                ssx3_ee::s10::OVERRIDE_0031C228 = fake_blend;
                saved
            }
        }
    }

    impl Drop for Slots {
        fn drop(&mut self) {
            unsafe {
                ssx3_ee::s09::OVERRIDE_002E87E8 = self.is_active;
                ssx3_ee::s00::OVERRIDE_0011FE98 = self.sub_state;
                ssx3_ee::s10::OVERRIDE_00312AA0 = self.anim_state;
                ssx3_ee::s10::OVERRIDE_0031C228 = self.blend;
            }
        }
    }

    /// The trail, its rider, the six strips and the globals the body and its
    /// callees read, all varied by `seed`.
    pub(crate) fn setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x002e_8938, 4);
        c.g[A0][0] = SELF as u64;

        // `gp_6`-relative globals.
        w(c, GAME_GP.wrapping_sub(0x1f04), BE_3);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD_2);
        w(c, WORLD_2 + 0x84, WORLD_2 + 0x100);
        w(c, WORLD_2 + 0x100 + 0xc, WORLD_2 + 0x200);
        w(c, WORLD_2 + 0x200 + 0x7c, 0);
        // The float constants the blend reads (and the emit cooldown rate).
        for off in [0xa38u32, 0xa3c, 0x1490, 0x1494, 0x1498, 0x149c, 0x14bc] {
            w(c, GAME_GP + off, r.edge_f32());
        }
        for off in [
            0x39acu32, 0x39b0, 0x39b4, 0x39b8, 0x39bc, 0x39c0, 0x39c4, 0x39c8, 0x39cc, 0x39d0,
        ] {
            w(c, GAME_GP.wrapping_sub(off), r.edge_f32());
        }
        // The pseudo-random word at `gp_6+0xa0c` that the emit path advances.
        w(c, GAME_GP + 0xa0c, r.next_u32());

        // The fake callees' answers.
        w(c, CTRL, r.next_u32());
        w(c, CTRL + 4, r.next_u32());
        w(c, CTRL + 8, r.next_u32());
        w(c, CTRL + 0xc, r.edge_f32());

        // The trail object.
        w(c, SELF + 0x00, RIDER_2);
        w(c, SELF + 0x04, r.edge_f32());
        w(c, SELF + 0x08, r.edge_f32());
        w(c, SELF + 0x0c, r.next_u32() & 1);
        for k in 0..6u32 {
            w(c, SELF + 0x10 + 4 * k, STRIPS + k * STRIP_STRIDE);
        }
        w(c, SELF + 0x8c, seed.wrapping_mul(7) % 54);
        w(c, SELF + 0x90, seed.wrapping_mul(13) % 54);
        // 0 = fresh trail, 1 = rewinding, >= 2 = live (both the live and the
        // "not active" clearing paths).
        w(c, SELF + 0x94, [0u32, 1, 2, 2, 3][(seed % 5) as usize]);
        w(c, SELF + 0x9c, r.edge_f32());
        w(c, SELF + 0xa0, r.edge_f32());
        for off in [0xb0u32, 0xc0, 0xd0] {
            put_vec(
                c,
                SELF + off,
                [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
            );
        }
        // 0 takes the long `ShouldEmit` path, 1 the cooldown shortcut.
        w(c, SELF + 0xe0, seed % 2);

        // The rider fields the body reads.
        w(c, RIDER_2 + 0x04, r.next_u32());
        w(c, RIDER_2 + 0x150, r.next_u32() % 4);
        w(c, RIDER_2 + 0x320, r.next_u32() & 1);
        w(c, RIDER_2 + 0x330, r.next_u32() % 3);
        w(c, RIDER_2 + 0x454, r.edge_f32());
        w(c, RIDER_2 + 0x6c0, VSELF);
        w(c, RIDER_2 + 0x780, ANIM_NODE);
        w(c, RIDER_2 + 0x784, ANIM);
        for off in [0x8a4u32, 0x8b0, 0x8b8, 0x8e8, 0x918] {
            w(c, RIDER_2 + off, r.next_u32() % 4);
        }
        // `boardTrailCanDraw`: all four combinations.
        w(c, RIDER_2 + 0xac4, seed & 1);
        w(c, RIDER_2 + 0xad0, (seed >> 1) & 1);
        w(c, RIDER_2 + 0xafc, (seed >> 2) & 1);
        w(c, RIDER_2 + 0xb00, (seed >> 3) & 1);
        w(c, ANIM_NODE + 0x34, ANIM_TABLE);
        // The rider's animation-base virtual method (the one `jalr`).
        c.bus.write16(VSELF + 0x38, 0);
        w(c, VSELF + 0x3c, FAKE);

        // The vectors the dot products consume.
        let q = |r: &mut Rng| [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()];
        put_vec(c, RIDER_2 + 0x370, q(&mut r));
        put_vec(c, RIDER_2 + 0x460, q(&mut r));
        put_vec(c, RIDER_2 + 0x1e0, q(&mut r));
        for k in 0..4u32 {
            let e = ANIM_TABLE + 0x40 * k;
            put_vec(c, e, q(&mut r));
            put_vec(c, e + 0x20, q(&mut r));
            put_vec(c, e + 0x30, q(&mut r));
        }

        // The strip vertices (the shift loops read them back).
        for i in 0..(6 * 54 * 0x30 / 4) {
            w(c, STRIPS + 4 * i, r.next_u32());
        }
    }

    #[test]
    fn c_board_trail_fx_update_matches() {
        let _slots = Slots::install();
        check_strict(
            ssx3_ee::s09::f_002e8938__generated,
            c_board_trail_fx_update,
            96,
            &setup,
        );
    }

    #[test]
    fn transition_wipe_draw_matches() {
        check_all_2(
            ee::s09::f_002ebc40__generated,
            transition_wipe_draw,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ebc40, 2);
                let self_ = DATA + 0x200;
                c.g[A0][0] = sx(self_);
                c.f[12] = r.edge_f32();
                w(c, self_ + 0x14, r.next_u32() % 3);
                w(c, self_ + 0x18, r.next_u32() % 3);
                w(c, self_ + 0x4, r.next_u32());
                for off in [0xa40u32, 0xa44, 0xa48, 0xa4c, 0xa50, 0xa54] {
                    f_2(c, crate::difftest::GAME_GP + off, r.edge_f32());
                }
                for off in [0x39a0u32, 0x39a4, 0x3998, 0x399c] {
                    f_2(c, crate::difftest::GAME_GP.wrapping_sub(off), r.edge_f32());
                }
            },
        );
    }
}
