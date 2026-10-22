//! `src/object/flexbridgenode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use crate::events;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::Callee;
use crate::call::Flow;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use crate::call::sp;
use crate::events::back_edge_unwinds;
use crate::ps2f::FpuF32;
use crate::call::Unwound;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::intersect::worldsphtree::save_field_block;
pub(crate) use crate::game::wscript::wscriptman::p;
pub(crate) use crate::game::render::particle::this_with_delta;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::fe::feasyncfile::low_2;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::fepopup3::s32;
pub(crate) use crate::game::util::menu::gp_5 as gp;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0034_6d38, flex_bridge_init_segment_midpoints, "FlexBridge_InitSegmentMidpoints"),
    (0x0034_75a8, flex_bridge_update, "FlexBridge_Update"),
    (0x0034_7b80, flex_bridge_render, "FlexBridge_Render"),
    (0x0034_7d38, flex_bridge_save, "FlexBridge_Save"),
    (0x0034_7f90, flex_rail_dtor, "FlexRail_dtor"),
    (0x0034_8008, flex_rail_update, "FlexRail_Update"),
    (0x0034_8058, flex_rail_update_space_node, "FlexRail_UpdateSpaceNode"),
    (0x0034_8b40, flex_rail_save, "FlexRail_Save"),
];

// ---------------------------------------------------------------------------
// Intra-shard callees the generator called directly (its `jal` aliases).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_STATE_GOTO_NEXT_3 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_INSTALL_MEM_HOOKS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_LOGIN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_MESSAGES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_LIST_STOP_2 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_2 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SSX_AUDIO_SYSTEM_DJ_SAY_20E5;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_DEACTIVATE_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_UNLOAD_SECTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_BX_STRING_COPY_BEFORE_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ANIM_OBJECT_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RAIL_MODIFIER_LIST_PUSH_FRONT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INTC_TABLE_ADD_HANDLER;
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
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;
    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_STREAM_HANDLE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT;
    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::render::particle::callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT;
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::render::particle::callee::AVA_SPLINE_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE as TEX_MAN_LOAD_TEXTURE_2;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2 as UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_FINISH_BOARD_PRESS;
    pub(crate) use crate::game::render::particle::callee::ALGORITHM_LIST_DTOR;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::render::particle::callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::CROWD_MAN3D_GROUP_REMOVE_SECTION;
    pub(crate) use crate::game::render::particle::callee::WORLD_OBJ_SAVE as WORLD_OBJ_SAVE_2;
    pub(crate) use crate::game::render::particle::callee::FLEX_BRIDGE_SPRINGS_SAVE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INTERFACE;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET as UI_LIST_RESET_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_HIDE_ROOM_TABS;
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


/// `FlexBridge_Update(FlexBridge* bridge)` (0x3475a8): one physics step of the
/// bridge springs, then rebuild the rendered vertices.
pub fn flex_bridge_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
        call_ptr(c, 0x0035_3fc0, 0x0034_75c0)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0034_75d8, 0x0034_75c8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void FlexRail_Save(FlexRail* self, Stream* stream)` (0x348b40): 0x30 bytes
/// at `+0x20`.
pub fn flex_rail_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(c, callee::WORLD_OBJ_SAVE, 0x20, 0x30, 0x0034_8b5c, 0x0034_8b78)
    })
}


/// `void FlexRail_Update(FlexRail* self)` (0x348008): run the two per-node
/// virtual updates, then re-place the space node.
pub fn flex_rail_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(this, 0xc)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x178).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x17c).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0034_802c)?;
        c.g[V0][0] = Addr(p(this, 0xc)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x180).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x184).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0034_8040)?;
        c.g[A0][0] = c.g[S0][0];
        callee::FLEX_RAIL_UPDATE_SPACE_NODE.call(c, 0x0034_8048)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void FlexBridge_Save(FlexBridge* self, Stream* stream)` (0x347d38): save
/// the world object, the bridge springs and the 0x30 bytes at +0x20.
pub fn flex_bridge_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0034_7d54);
        c.g[S1][0] = c.g[A1][0];
        callee::WORLD_OBJ_SAVE_2.call(c, 0x0034_7d54)?;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x50));
        c.g[RA][0] = sx(0x0034_7d60);
        c.g[A1][0] = c.g[S1][0];
        callee::FLEX_BRIDGE_SPRINGS_SAVE.call(c, 0x0034_7d60)?;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(0x20));
        c.g[A2][0] = 0x30;
        c.g[A0][0] = Addr(r32(c, V1)).off(0x8).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0034_7d7c);
        c.g[A0][0] = this_with_delta(c.g[S1][0], c.g[A0][0]);
        call_ptr(c, target, 0x0034_7d7c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `FlexRail_UpdateSpaceNode` (0x348058): refresh a rail's space node: call
/// its `vt[50]` update, get the world painter, and hand the rail's box to the
/// spatial hash (`SpatialHash_Insert(world, 2, id, box, &old)`).
pub fn flex_rail_update_space_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[S1][0] = low(c, S0).wrapping_add(0x30);
        c.lq(2, r32(c, S0).wrapping_add(0x40));
        c.lq(3, r32(c, S0).wrapping_add(0x30));
        c.sq(2, frame.at(c, 0x10));
        c.sq(3, frame.at(c, 0));
        c.g[V1][0] = lw_at(c, S0, 0xc);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x190)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x194)).lw(c);
        let update = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, update, 0x0034_8094)?;
        call_ptr(c, addr::WORLD_PAINTER_2D, 0x0034_809c)?;
        c.g[A2][0] = lw_at(c, S0, 0x54);
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        c.g[T0][0] = c.g[SP][0];
        call_ptr(c, addr::SPATIAL_INSERT, 0x0034_80b4)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}


/// `func_00347F90` (0x347f90): `FlexRail_dtor`. Installs vtable `0x4900d0`,
/// frees the two arrays at `+0x50` (`0x317e98`) and destroys the object at
/// `+0x58` (`vtable[8]`, flag 3) before running the world object destructor.
pub fn flex_rail_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(low_2(c, V0).wrapping_add(0xd0));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        sw_r(c, S0, 0xc, c.g[V0][0]);
        c.g[A0][0] = lw_r(c, S0, 0x50);
        let no_array = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_array {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0034_7fc4);
            call_ptr(c, 0x0031_7e98, 0x0034_7fc4)?;
        }
        c.cycles += 3;
        c.g[A2][0] = lw_r(c, S0, 0x58);
        let no_obj = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !no_obj {
            c.cycles += 5;
            c.g[V0][0] = Addr(low_2(c, A2)).lw(c);
            c.g[A0][0] = lh_r(c, V0, 0x8);
            c.g[V1][0] = lw_r(c, V0, 0xc);
            let target = low_2(c, V1);
            c.g[RA][0] = sx(0x0034_7fe4);
            c.g[A0][0] = sx(low_2(c, A2).wrapping_add(low_2(c, A0)));
            call_ptr(c, target, 0x0034_7fe4)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0034_7ff0);
        c.g[A1][0] = c.g[S1][0];
        callee::WORLD_OBJ_DTOR.call(c, 0x0034_7ff0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void func_00346D38(FlexBridge* self)` (0x346d38): place the midpoint of
/// every bridge segment (`pts[i]` to `pts[count + i]`) into `segs[i].pos`, then
/// recompute the bridge's start/end offsets from the first segment.
pub fn flex_bridge_init_segment_midpoints(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A1][0] = c.g[A0][0];
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    let this = c.g[A1][0] as u32;
    c.g[T1][0] = c.lw(p(this, 0x50)); // count
    c.g[A2][0] = 0;
    let empty = s32(c.g[T1][0]) <= 0;
    c.g[A3][0] = c.lw(p(this, 0x74)); // segs
    if !empty {
        c.cycles += 4;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 0); // f0 = 0.5
        c.g[T0][0] = sx((c.g[T1][0] as u32) << 4);
        'loop_segments: loop {
            c.cycles += 32;
            c.g[V0][0] = c.lw(p(this, 0x7c)); // pts
            c.g[A0][0] = sx((c.g[A2][0] as u32) << 4);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.lqc2(3, c.g[V0][0] as u32); // pts[count + i]
            c.lqc2(4, c.g[V1][0] as u32); // pts[i]
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, p(c.g[SP][0] as u32, 0x30));
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x10));
            c.lq(2, p(c.g[SP][0] as u32, 0x30));
            c.sq(2, p(c.g[SP][0] as u32, 0x20));
            c.mfc1(3, 0); // v1 = 0.5
            c.lqc2(4, p(c.g[SP][0] as u32, 0x20));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, p(c.g[SP][0] as u32, 0x30));
            c.g[V1][0] = c.lw(p(this, 0x7c));
            c.lq(2, p(c.g[SP][0] as u32, 0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sq(2, p(c.g[SP][0] as u32, 0x10));
            c.lqc2(3, c.g[A0][0] as u32);
            c.lqc2(4, p(c.g[SP][0] as u32, 0x10));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, p(c.g[SP][0] as u32, 0x30));
            c.lq(2, p(c.g[SP][0] as u32, 0x30));
            c.sq(2, p(c.g[SP][0] as u32, 0));
            c.sq(2, c.g[A3][0] as u32); // segs[i].pos
            c.g[V0][0] = c.lw(p(this, 0x50));
            c.g[V0][0] = ((s32(c.g[A2][0]) < s32(c.g[V0][0])) as u64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x50));
            if !more {
                break 'loop_segments;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_6d60);
            }
        }
    }
    c.cycles += 21;
    c.g[V0][0] = c.lw(p(this, 0x7c));
    c.g[A0][0] = sx((c.g[T1][0] as u32) << 4);
    c.g[A3][0] = c.lw(p(this, 0x74));
    c.lqc2(3, c.g[V0][0] as u32);
    c.lqc2(4, c.g[A3][0] as u32);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(c.g[SP][0] as u32, 0x10));
    c.lq(3, p(c.g[SP][0] as u32, 0x10));
    c.g[V0][0] = c.lw(p(this, 0x7c));
    c.sq(3, p(this, 0x30)); // start
    c.sq(3, p(c.g[SP][0] as u32, 0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.lqc2(3, c.g[A0][0] as u32);
    c.lqc2(4, c.g[A3][0] as u32);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(c.g[SP][0] as u32, 0x10));
    c.lq(2, p(c.g[SP][0] as u32, 0x10));
    c.sq(2, p(this, 0x20)); // end
    c.sq(2, p(c.g[SP][0] as u32, 0));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
}


// ---------------------------------------------------------------------------
// 0x00347b80 FlexBridge_Render
// ---------------------------------------------------------------------------

/// `int FlexBridge_Render(FlexBridge* self)` (0x347b80).
///
/// Renders the flexing bridge segment: pushes the draw state, asks the
/// renderer's dispatch table for the segment transform, builds the two vertex
/// arrays and submits them, then restores the state. Returns 0.
pub fn flex_bridge_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = c.lw(gp(c, 0x16c8));
        frame.save(c, 21, 0x50);
        frame.save(c, 16, 0xa0);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, 17, 0x90);
        c.g[A0][0] = 0xff;
        frame.save(c, 18, 0x80);
        frame.save(c, 19, 0x70);
        frame.save(c, 20, 0x60);
        frame.save_ra(c, 0x40);

        'ret: {
            c.g[A1][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
            c.g[V1][0] = c.lbu(p(c.g[A1][0] as u32, 0x7d));
            let t = c.g[V1][0] == c.g[A0][0];
            c.g[V0][0] = 1;
            if t {
                break 'ret;
            }
            c.cycles += 7;
            c.g[V0][0] = c.lbu(p(c.g[A1][0] as u32, 0x7d));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.g[V0][0] = sx(c.g[A2][0] as u32 + c.g[V0][0] as u32);
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x24));
            c.g[V1][0] = c.g[V1][0] ^ 6;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = 0;
                break 'ret;
            }
            c.cycles += 7;
            c.g[V0][0] = c.lh(p(c.g[A1][0] as u32, 0x7e));
            c.g[A0][0] = 0x18;
            c.mult(0, 3, 2, 4);
            c.g[V0][0] = sx(c.g[V1][0] as u32 + c.g[A2][0] as u32);
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x3f0));
            c.g[V1][0] = c.g[V1][0] ^ 3;
            c.g[V0][0] = (c.g[V1][0] < 1) as u64;
        }

        'body: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0;
            c.g[S4][0] = c.lw(gp(c, 0x2a90));
            if t {
                break 'body;
            }
            c.cycles += 19;
            c.g[A0][0] = c.g[SP][0];
            c.g[S2][0] = sx(c.g[S5][0] as u32 + 0x90);
            c.g[S3][0] = sx(c.g[S5][0] as u32 + 0xa0);
            c.g[S0][0] = c.lw(p(c.g[S4][0] as u32, 0x10d8));
            c.g[S1][0] = c.lh(p(c.g[S0][0] as u32, 0x2e8));
            c.g[A1][0] = c.lh(p(c.g[S0][0] as u32, 0x158));
            c.g[V0][0] = c.lw(p(c.g[S0][0] as u32, 0x15c));
            c.g[S1][0] = sx(c.g[S4][0] as u32 + c.g[S1][0] as u32);
            let t = c.g[V0][0] as u32;
            c.g[A1][0] = sx(c.g[S4][0] as u32 + c.g[A1][0] as u32);
            call_ptr(c, t, 0x0034_7c28)?;
            c.g[V0][0] = c.lw(p(c.g[S0][0] as u32, 0x2ec));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            let t = c.g[V0][0] as u32;
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, t, 0x0034_7c40)?;
            c.g[V1][0] = 1;
            let t = c.g[V0][0] == c.g[V1][0];
            c.lq(16, p(c.g[SP][0] as u32, 0xa0));
            if t {
                break 'body;
            }
            'inner: {
              'no_anim: {
                c.cycles += 28;
                c.g[V1][0] = c.lw(p(c.g[S5][0] as u32, 0xc));
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0xc0));
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0xc4));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx(c.g[S5][0] as u32 + c.g[A0][0] as u32);
                call_ptr(c, t, 0x0034_7c60)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0xffff_ffff_ffff_ff83;
                c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                c.ldl(4, p(c.g[V0][0] as u32, 7));
                c.ldr(4, c.g[V0][0] as u32);
                c.ldl(7, p(c.g[V0][0] as u32, 0xf));
                c.ldr(7, p(c.g[V0][0] as u32, 8));
                c.g[T0][0] = c.lw(p(c.g[V0][0] as u32, 0x10));
                c.sdl(4, p(c.g[V0][0] as u32, 0x1b));
                c.sdr(4, p(c.g[V0][0] as u32, 0x14));
                c.sdl(7, p(c.g[V0][0] as u32, 0x23));
                c.sdr(7, p(c.g[V0][0] as u32, 0x1c));
                c.sw(p(c.g[V0][0] as u32, 0x24), c.g[T0][0]);
                c.g[A0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                c.g[V1][0] = sx(c.g[A0][0] as u32 + 0x14);
                c.sw(p(c.g[S4][0] as u32, 0xe84), c.g[V1][0]);
                c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 0x18));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | 0x14;
                c.sw(p(c.g[A0][0] as u32, 0x18), c.g[V0][0]);
                c.g[T1][0] = c.lw(p(c.g[S5][0] as u32, 0x78));
                let t = c.g[T1][0] == 0;
                c.g[A1][0] = c.g[A2][0];
                if t {
                    break 'no_anim;
                }
                c.cycles += 10;
                c.g[V1][0] = c.lw(p(c.g[T1][0] as u32, 4));
                c.g[T0][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
                c.g[A3][0] = 0;
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x38));
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x3c));
                c.g[A2][0] = c.lw(p(c.g[T0][0] as u32, 0x94));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx(c.g[T1][0] as u32 + c.g[A0][0] as u32);
                call_ptr(c, t, 0x0034_7cdc)?;
                let t = true;
                c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                if t {
                    break 'inner;
                }
              }
                c.cycles += 10;
                c.g[V1][0] = c.lw(p(c.g[S4][0] as u32, 0x10d8));
                c.g[A2][0] = 0;
                c.g[A1][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
                c.g[A3][0] = 0;
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x300));
                c.g[T0][0] = 0;
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x304));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx(c.g[S4][0] as u32 + c.g[A0][0] as u32);
                call_ptr(c, t, 0x0034_7d08)?;
                c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                break 'inner;
            }
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
            c.sw(p(c.g[S4][0] as u32, 0xe84), c.g[V0][0]);
            break 'body;
        }
        c.cycles += 1;
        c.lq(16, p(c.g[SP][0] as u32, 0xa0));

        c.cycles += 8;
        frame.restore(c, 17, 0x90);
        frame.restore(c, 18, 0x80);
        frame.restore(c, 19, 0x70);
        frame.restore(c, 20, 0x60);
        frame.restore(c, 21, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use ssx3_ee as ee;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::compare;
    use crate::difftest::put_vec;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::intersect::worldsphtree::tests::fake_stream;
    pub(crate) use crate::game::intersect::worldsphtree::tests::save_test;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_4;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::vtable_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_5;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    pub(crate) use crate::game::ai::rider::tests::hh;
    pub(crate) use crate::game::ai::rider::tests::fb;
    pub(crate) use crate::game::dirtysock::tags2::tests::world_2 as world;
    pub(crate) use crate::game::ai::rider::tests::slot_2 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot_2;
    pub(crate) use crate::game::bx::memblockman::tests::fake_slot;
    pub(crate) use crate::game::be::beintplayer::tests::strict;
    pub(crate) use crate::game::be::beintplayer::tests::globals_5 as globals;
    pub(crate) use crate::game::be::beintplayer::tests::base_10 as base;


    #[test]
    fn flex_bridge_update_matches() {
        check(ee::s11::f_003475a8__generated, flex_bridge_update, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3475a8, 0);
            c.g[A0][0] = DATA as u64;
            // The frame-clock singleton the step length comes from.
            c.bus.write32(GAME_GP + 0x2a74, DATA + 0x300);
            c.bus.write32(DATA + 0x310, 0x42f0_0000); // 120.0f
            // A bridge with no nodes: both physics passes return immediately.
            c.bus.write32(DATA + 0x50, 0);
            c.bus.write32(DATA + 0x74, r.next_u32());
        });
    }


    #[test]
    fn flex_rail_save_matches() {
        save_test(ee::s11::f_00348b40__generated, flex_rail_save, 0x348b_40, 0x20);
    }


    #[test]
    fn flex_rail_update_matches() {
        check(ee::s11::f_00348008__generated, flex_rail_update, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x3480_08, 0);
            alloc_ready(c);
            let this = DATA + 0x100;
            let vt = DATA + 0x800;
            clear(c, this, 0x40);
            clear(c, vt, 0x200);
            c.bus.write32(this + 0xc, vt);
            c.bus
                .write32(vt + 0x178, (r.next_u32() as i16 as i32 & !3) as u32);
            c.bus.write32(vt + 0x17c, FAKE);
            c.bus
                .write32(vt + 0x180, (r.next_u32() as i16 as i32 & !3) as u32);
            c.bus.write32(vt + 0x184, FAKE + 0x10);
            c.g[A0][0] = sx(this);
        });
    }


    #[test]
    fn flex_bridge_save_matches() {
        check(ee::s11::f_00347d38__generated, flex_bridge_save, SEEDS_4, &|c, seed| {
            let mut r = common(c, seed, 0x347d38, 3);
            let this = DATA + 0x100;
            let stream = DATA + 0x200;
            let vt = DATA + 0x300;
            clear(c, this, 0x200);
            clear(c, stream, 0x100);
            clear(c, vt, 0x200);
            c.bus.write32(stream, vt);
            vtable_2(c, vt, &[(0x8, 0, FAKE)]);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(stream);
            let _ = &mut r;
        });
    }


    #[test]
    fn flex_rail_update_space_node_matches() {
        check(ee::s11::f_00348058__generated, flex_rail_update_space_node, SEEDS_5, &|c, seed| {
            machine(c, seed, 0x34_8058, 1);
            world(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            w(c, DATA + 0x2000 + 0xc, DATA + 0x2200);
            slot(c, DATA + 0x2200, 0x190, 0, FAKE);
            for (i, v) in [1.0f32, 2.0, 3.0, 0.0].iter().enumerate() {
                w(c, DATA + 0x2000 + 0x30 + 4 * i as u32, fb(*v));
            }
            for (i, v) in [-1.0f32, -2.0, -3.0, 0.0].iter().enumerate() {
                w(c, DATA + 0x2000 + 0x40 + 4 * i as u32, fb(*v));
            }
            w(c, DATA + 0x2000 + 0x54, 1 + seed % 3);
            c.pc = 0x0034_8058;
        });
    }


    #[test]
    fn flex_rail_dtor_matches() {
        check(ee::s11::f_00347f90__generated, flex_rail_dtor, SEEDS_6, &|c, seed| {
            machine(c, seed, 0x347f90, 4);
            be_library(c);
            world(c);
            heap_init(c);
            let this = DATA;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = 0;
            w(c, this + 0x50, 0); // no array free
            if seed % 2 == 0 {
                w(c, this + 0x58, 0);
            } else {
                let obj = DATA + 0x400;
                w(c, this + 0x58, obj);
                fake_slot(c, obj, DATA + 0x2000, 8);
            }
        });
    }


    #[test]
    fn flex_bridge_init_segment_midpoints_matches() {
        strict(ee::s11::f_00346d38__generated, flex_bridge_init_segment_midpoints, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x6d38, 0);
            let this = DATA + 0x1000;
            let pts = DATA + 0x4000;
            let segs = DATA + 0x6000;
            let count = (seed % 4) as u32;
            w(c, this + 0x50, count);
            w(c, this + 0x7c, pts);
            w(c, this + 0x74, segs);
            for k in 0..(2 * count).max(1) {
                let v = [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()];
                put_vec(c, pts + 0x10 * k, v);
            }
            c.g[A0][0] = sx(this);
        });
    }


    // -- 0x00347b80 FlexBridge_Render ----------------------------------

    #[test]
    fn flex_bridge_render_matches() {
        covered(ssx3_ee::s11::f_00347b80__generated, flex_bridge_render, 60, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x347b80, 4);
            let world = DATA + 0x8_0000;
            let bridge = DATA + 0x1000;
            let board = DATA + 0x2000;
            let manager = DATA + 0x3000;
            let obj = DATA + 0x4000;
            let draw = DATA + 0x6000;
            w(c, GAME_GP.wrapping_add(0x16c8), manager);
            w(c, GAME_GP.wrapping_add(0x2a90), world);
            w(c, bridge + 0x18, board);
            w(c, bridge + 0xc, board);
            w(c, bridge + 0x78, 0);
            c.g[A0][0] = sx(bridge);
            c.bus.write8(board + 0x7d, if seed % 3 == 0 { 0xff } else { 1 });
            c.bus.write16(board + 0x7e, (r.next_u32() & 3) as u16);
            let table = DATA + 0x7000;
            w(c, manager, table);
            for i in 0..4u32 {
                w(c, table + 8 * i + 0x24, 6);
                w(c, table + 0x18 * i + 0x3f0, 3);
            }
            w(c, world + 0xe84, draw);
            w(c, world + 0x10d8, obj);
            w(c, obj + 0x2e8, 0);
            w(c, obj + 0x158, 0);
            w(c, obj + 0x15c, FAKE + 0x20);
            w(c, obj + 0x2ec, FAKE + 0x30);
            w(c, obj + 0x300, 0);
            w(c, obj + 0x304, FAKE + 0x40);
            w(c, board + 0xc0, 0);
            w(c, board + 0xc4, FAKE + 0x50);
            for i in 0..0x30u32 {
                c.bus.write8(draw + i, i as u8 | 1);
            }
            w(c, draw + 0x18, r.next_u32());
        });
    }
}
