//! `src/object/flagset.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::float;
use ssx3_ee_rt::Cpu;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::events::back_edge_unwinds;
use crate::call::sp;
pub(crate) use crate::game::intersect::worldsphtree::p;
pub(crate) use crate::game::intersect::worldsphtree::sp_2;
use crate::game::ai::rider::per_tick_60;
pub(crate) use crate::game::visualfx::boardwakefx::gp;
pub(crate) use crate::game::visualfx::boardwakefx::ldf_2 as ldf;
pub(crate) use crate::game::visualfx::boardwakefx::fr_2 as fr;
pub(crate) use crate::game::visualfx::boardwakefx::stf;
pub(crate) use crate::game::visualfx::boardwakefx::lw;
pub(crate) use crate::game::visualfx::boardwakefx::lh;
pub(crate) use crate::game::visualfx::boardwakefx::copy_render_state;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::gp as gp_2;
pub(crate) use crate::game::render::lightman::Oc49Mem;
pub(crate) use crate::game::render::lightman::addu;
pub(crate) use crate::game::intersect::worldsphtree::save_field_block;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_2;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw_2;
pub(crate) use crate::game::fe::fereal::gp as gp_3;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::wscript::wscriptman::frame_at;
pub(crate) use crate::game::wscript::wscriptman::sw;
pub(crate) use crate::game::util::menu::gp_5 as gp_4;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0034_b818, cflag_manager_group_update, "cFlagManager_GroupUpdate"),
    (0x0034_b9b0, flag_manager_group_render, "cFlagManager_GroupRender"),
    (0x0034_c378, cflag_manager_group_load, "cFlagManager_GroupLoad"),
    (0x0034_c428, c_flag_manager_ctor, "cFlagManager_ctor"),
    (0x0034_c4b8, c_flag_manager_dtor, "cFlagManager_dtor"),
    (0x0034_c548, cflag_manager_add_flag, "cFlagManager_AddFlag"),
    (0x0034_c898, c_flag_manager_load, "cFlagManager_Load"),
    (0x0034_cb80, fence_dtor, "Fence_dtor"),
    (0x0034_cde8, fence_kick_from_neighbour, "Fence_KickFromNeighbour"),
    (0x0034_d650, fence_v40, "Fence_v40"),
    (0x0034_d6f0, fence_add_impulse, "Fence_AddImpulse"),
    (0x0034_d778, fence_render, "Fence_Render"),
    (0x0034_d960, fence_save, "Fence_Save"),
    (0x0034_dd18, anim_object_build_matrices, "AnimObject_BuildMatrices"),
    (0x0034_e320, anim_object_get_node_intersect_matrix, "AnimObject_GetNodeIntersectMatrix"),
    (0x0034_e348, anim_object_v52, "AnimObject_v52"),
];

// ---------------------------------------------------------------------------
// Callees (address + the generated shard function, which dispatches through
// its override slot). Entries for calls that the generator emitted as
// `call_indirect` are kept for reference next to the `call_ptr` sites.

#[allow(dead_code)]
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WP_DATA_FIND_CELL_INDEX;
    pub(crate) use crate::game::intersect::worldsphtree::callee::INPUT_VM_READ_BUILTIN;
    pub(crate) use crate::game::intersect::worldsphtree::callee::INPUT_VM_READ_PAD_CHANNEL;
    pub(crate) use crate::game::intersect::worldsphtree::callee::FOCUS_MAN_COLLECT_OCTREE_NODE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::ANIM_OBJECT_GET_NODE_INTERSECT_MATRIX;
    pub(crate) use crate::game::intersect::worldsphtree::callee::COLLIDE_VOLUME_OBJECT_2 as COLLIDE_VOLUME_OBJECT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::RAIL_MODIFIER_FIND_CLOSEST_POINT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::FLEX_RAIL_FIND_CLOSEST_POINT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::RAIL_FIND_NEAREST_POINT;
    pub(crate) use crate::game::render::lightman::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_FLAG_MANAGER;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_AI_FRAME_COUNT;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_NASH;
    pub(crate) use crate::game::render::lightman::callee::LIGHT_CONTRIBUTION;
    pub(crate) use crate::game::render::lightman::callee::MATH_SIN_COS;
    pub(crate) use crate::game::render::lightman::callee::SPATIAL_HASH_UPDATE;
    pub(crate) use crate::game::render::lightman::callee::FLAG_MANAGER_GROUP_COMPUTE_VERTS;
    pub(crate) use crate::game::render::lightman::callee::INTERSECT_INFO_EVALUATE;
    pub(crate) use crate::game::render::lightman::callee::TYPED_PROCESS_SAVE;
    pub(crate) use crate::game::render::lightman::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::render::lightman::callee::MULTI_SPLINE_MODIFIER_DEBUG_DRAW_PATH;
    pub(crate) use crate::game::render::lightman::callee::EA_MEMSET;
    pub(crate) use crate::game::render::lightman::callee::DMA_WAIT_FROM_SCRATCHPAD;
    pub(crate) use crate::game::render::lightman::callee::DMA_WAIT_TO_SCRATCHPAD;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES as CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2 as KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2 as BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2 as BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2 as SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2 as SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2 as SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_FROM_ASCII_2 as USTR_FROM_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WIDGET_ALIGN;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WRAPPED_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::RIDER_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_MISSION_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_2 as WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_MAN_CLEAR;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_SET_EVENT_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::OPTIONS_SET_VIBRATION;
    pub(crate) use crate::game::bx::bxstring2::callee::STR_HASH;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_3;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_FIND_FIRST_SELECTABLE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::STRLEN;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_DEQUEUE_SECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_POP_FREE_NODE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_ADD_ITEM_STR_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_FIND_ITEM_BY_VALUE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_GET_POPUP as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_SHOW_POPUP as NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_CTOR_COPY;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_DTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_SPEECH_CONTEXT_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_SKIP_WHITESPACE;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::bx::bxstring2::callee::CHUNK_RESOLVE_RELOCATE_DMA;
    pub(crate) use crate::game::bx::bxstring2::callee::FLAG_MANAGER_GROUP_BUILD_MESH;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_CHARACTER as BE_GET_RIDER_CHARACTER_2;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::READ_PRIVATE_MESSAGE_SET_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_MANAGER;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_SCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_FLOAT_3 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_DESIRED_EYE;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_LOOK_AT;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_INIT_FROM_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::MEMSET;
    pub(crate) use crate::game::bx::bxstring2::callee::QSORT;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN3D_GROUP_BUILD_MATRICES;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAM_CONTROLLER_RESTORE_DEFAULT;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_SET_SKIP_DELAY;
    pub(crate) use crate::game::bx::bxstring2::callee::CUTSCENE_TRANSITION_MAN_FORCE_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::bx::bxstring2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_FREE;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_LEN;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_CPY;
    pub(crate) use crate::game::bx::bxstring2::callee::UITEXT_SET_WIDE_STRING;
}

/// The game's render-context singleton (`gp+0x2a90`, `D_004A5B80`): its
/// vtable (`+0x10d8`), the render-state stack pointer (`+0xe84`) and the
/// texture-id table (`+0xf50`).
#[allow(dead_code)]
pub(crate) mod ctx {
    pub(crate) use crate::game::visualfx::boardwakefx::ctx::VTABLE;
    pub(crate) use crate::game::visualfx::boardwakefx::ctx::TOP;
    pub(crate) use crate::game::visualfx::boardwakefx::ctx::TEX_IDS;
}

/// `sAmCtx` (`D_004A289C`) used by `Rider_RenderModel`, and the world
/// pointer (`gp-0x848`) whose `+0x84 -> +0x84 -> +0x14` is the active rider.
#[allow(dead_code)]
pub(crate) mod globals {
    pub(crate) use crate::game::visualfx::boardwakefx::globals::AM_CTX;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::WORLD;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::EMITTER_BLEND;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::IDENTITY_MAT;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::FLAG_STATE;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::BONE_MATRICES;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::BONE_MATRICES_SHADOW;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::FLAG_MANAGER;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::FLAG_TABLE;
}

/// `cFlagManager` fields.
pub(crate) mod flag_manager {

    /// Frame parity the group was last rebuilt for (`+0x90`).
    pub const PARITY: u32 = 0x90;

    /// Vertex animation blend (`+0x4c`).
    pub const BLEND: u32 = 0x4c;

    /// Per-phase amount (`+0x4`, 4 floats) and accumulator (`+0x78`).
    pub const PHASE: u32 = 0x4;

    pub const SPEED: u32 = 0x78;

    /// Scroll speeds (`+0x50` v, `+0x54` u) and offsets (`+0x88` v, `+0x8c` u).
    pub const SCROLL_V: u32 = 0x50;

    pub const SCROLL_U: u32 = 0x54;

    pub const V: u32 = 0x88;

    pub const U: u32 = 0x8c;
}


/// A float constant read relative to `gp` (game globals / `.lit4`).
pub(crate) fn gp_float(c: &mut Cpu, off: u32) -> FpuF32 {
    Addr((c.g[GP][0] as u32).wrapping_sub(off)).read_fpu(c)
}


// ---------------------------------------------------------------------------
// Fence.

/// `void Fence_AddImpulse(Fence *f, float impulse)` (0x34d6f0): add an impulse
/// to the fence's sway, clamping to 1.0, and damp the velocity.
pub fn fence_add_impulse(c: &mut Cpu) {
    let fence = Addr(abi::arg(c, 0));
    c.cycles += 9;
    c.f[0] = fence.off(0x30).read_fpu(c).0;
    c.f[1] = FpuF32(c.f[12]).abs().0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[2] = 0x3f80_0000;
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
    let cc = FpuF32(c.f[2]).lt(FpuF32(c.f[0]));
    abi::set_fcc(c, cc);
    fence.off(0x30).write_fpu(c, FpuF32(c.f[0]));
    if cc {
        c.cycles += 1;
        fence.off(0x30).write_fpu(c, FpuF32(c.f[2]));
    }
    c.cycles += 5;
    c.f[1] = fence.off(0x34).read_fpu(c).0;
    c.f[0] = 0;
    let is_zero = FpuF32(c.f[1]).eq_s(FpuF32::ZERO);
    abi::set_fcc(c, is_zero);
    if is_zero {
        c.cycles += 4;
        let neg = FpuF32(c.f[12]).lt(FpuF32::ZERO);
        abi::set_fcc(c, neg);
        c.f[0] = fence.off(0x30).read_fpu(c).0;
        if neg {
            c.cycles += 3;
            c.f[1] = gp_float(c, 0x2ab4).0;
        } else {
            c.cycles += 2;
            c.f[1] = gp_float(c, 0x2aac).0;
        }
    } else {
        c.cycles += 4;
        let neg = FpuF32(c.f[1]).lt(FpuF32::ZERO);
        abi::set_fcc(c, neg);
        c.f[0] = fence.off(0x30).read_fpu(c).0;
        if neg {
            c.cycles += 3;
            c.f[1] = gp_float(c, 0x2ab0).0;
        } else {
            c.cycles += 2;
            c.f[1] = gp_float(c, 0x2aac).0;
        }
    }
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[1])).0;
    c.cycles += 2;
    fence.off(0x34).write_fpu(c, FpuF32(c.f[0]));
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Anim object node intersection matrix.

/// `void* AnimObject_GetNodeIntersectMatrix(AnimObject *o, int node)`
/// (0x34e320): the intersect matrix of `node`, or a default when there is no
/// node table.
pub fn anim_object_get_node_intersect_matrix(c: &mut Cpu) {
    c.cycles += 3;
    let node = abi::arg(c, 1);
    let table = Addr(abi::arg(c, 0)).off(0x48).lw(c);
    c.g[V1][0] = table;
    if table == 0 {
        c.g[V0][0] = 0x500000;
        c.cycles += 2;
        c.g[V0][0] = sx(0x500000u32.wrapping_sub(0xe60));
        abi::ret(c);
    } else {
        c.cycles += 5;
        c.g[V0][0] = 0xd0;
        let off = abi::mult(c, node, 0xd0);
        c.g[A0][0] = sx(off);
        let ptr = off.wrapping_add(table as u32);
        c.g[V0][0] = sx(ptr.wrapping_add(0x90));
        abi::ret(c);
    }
}


// ---------------------------------------------------------------------------
// Animation object: bone matrices.

/// `out = a * b` with `mulScaledFS`'s layout: the four rows of `a` times the
/// broadcast columns of `b`, transposed twice through the two scratch quads
/// (`s1`, `s2`) and stored to `dst`.
pub(crate) fn mul_scaled(c: &mut Cpu, a: u32, b: u32, dst: u32, s1: u32, s2: u32) {
    c.lqc2(4, a);
    c.lqc2(5, p(a, 0x10));
    c.lqc2(6, p(a, 0x20));
    c.lqc2(7, p(a, 0x30));
    c.lqc2(8, b);
    c.lqc2(9, p(b, 0x10));
    c.lqc2(10, p(b, 0x20));
    c.lqc2(11, p(b, 0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.vu0().mula(0b1111, 4, Src::Bc(9, X));
    c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
    c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
    c.vu0().mula(0b1111, 4, Src::Bc(10, X));
    c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
    c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
    c.vu0().mula(0b1111, 4, Src::Bc(11, X));
    c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
    c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
    c.sqc2(12, s1);
    c.sqc2(13, p(s1, 0x10));
    c.sqc2(14, p(s1, 0x20));
    c.sqc2(15, p(s1, 0x30));
    c.lqc2(1, s1);
    c.lqc2(2, p(s1, 0x10));
    c.lqc2(3, p(s1, 0x20));
    c.lqc2(4, p(s1, 0x30));
    c.sqc2(1, s2);
    c.sqc2(2, p(s2, 0x10));
    c.sqc2(3, p(s2, 0x20));
    c.sqc2(4, p(s2, 0x30));
    c.lqc2(1, s2);
    c.lqc2(2, p(s2, 0x10));
    c.lqc2(3, p(s2, 0x20));
    c.lqc2(4, p(s2, 0x30));
    c.sqc2(1, dst);
    c.sqc2(2, p(dst, 0x10));
    c.sqc2(3, p(dst, 0x20));
    c.sqc2(4, p(dst, 0x30));
}


/// The scaled second operand of [`mul_scaled`]: the frame's scratch matrix
/// (`+0x50`, rows 0..2) from `mtx`, with row 3 replaced by the translation
/// row times the scale quad at `sp_2+0..0xc`. `fx`/`fy` are the FPU registers
/// the original keeps the x/y scale in (6 and 7, swapped per branch);
/// `r01`/`r2`/`r3` are the GPRs the original loads each row through (they are
/// left holding whole quads, so their upper halves must match too).
pub(crate) fn scaled_rows(
    c: &mut Cpu,
    mtx: u32,
    sp_2: u32,
    fx: usize,
    fy: usize,
    r01: usize,
    r2: usize,
    r3: usize,
) {
    c.lq(r01, mtx);
    c.sq(r01, p(sp_2, 0x90));
    c.sq(r01, p(sp_2, 0x50));
    c.lwc1(fx, p(sp_2, 0));
    c.lq(r01, p(mtx, 0x10));
    c.lwc1(fy, p(sp_2, 4));
    c.sq(r01, p(sp_2, 0x90));
    c.sq(r01, p(sp_2, 0x60));
    c.lwc1(5, p(sp_2, 8));
    c.lq(r2, p(mtx, 0x20));
    c.lwc1(4, p(sp_2, 0xc));
    c.sq(r2, p(sp_2, 0x90));
    c.sq(r2, p(sp_2, 0x70));
    c.lq(r3, p(mtx, 0x30));
    c.sq(r3, p(sp_2, 0xa0));
    c.lwc1(1, p(sp_2, 0xa0));
    c.lwc1(2, p(sp_2, 0xa4));
    c.lwc1(3, p(sp_2, 0xa8));
    c.mul_s(1, 1, fx);
    c.lwc1(0, p(sp_2, 0xac));
    c.mul_s(2, 2, fy);
    c.mul_s(3, 3, 5);
    c.mul_s(0, 0, 4);
    c.swc1(1, p(sp_2, 0x90));
    c.swc1(2, p(sp_2, 0x94));
    c.swc1(3, p(sp_2, 0x98));
    c.swc1(0, p(sp_2, 0x9c));
    c.lq(2, p(sp_2, 0x90));
    c.sq(2, p(sp_2, 0x80));
}


/// `void AnimObject_BuildMatrices(AnimObject* self, const sFsMtx* root,
/// sFsMtx* out)` (0x0034dd18): one matrix per skeleton bone, `out[i]`, from
/// the bone's parent (`out[parent]`, or `root` when it has none) and its
/// animation matrix (from `AnimObject_GetNodeIntersectMatrix` for animated
/// bones, the bone's own matrix otherwise), with the model scale applied to
/// the translation row.
pub fn anim_object_build_matrices(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x1b0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        frame.save(c, S7, 0x130);
        frame.save(c, S0, 0x1a0);
        frame.save(c, S1, 0x190);
        frame.save(c, S2, 0x180);
        frame.save(c, S3, 0x170);
        frame.save(c, S4, 0x160);
        frame.save(c, S5, 0x150);
        frame.save(c, S6, 0x140);
        frame.save(c, FP, 0x120);
        frame.save_ra(c, 0x110);
        c.sw(p(sp_2(c), 0xf0), c.g[A0][0]);
        let model = Addr(p(c.g[A0][0] as u32, 0x2c)).lw(c);
        c.g[V0][0] = model;
        c.sw(p(sp_2(c), 0xf4), c.g[A2][0]);
        c.sw(p(sp_2(c), 0xf8), c.g[A1][0]);
        let skel = Addr(p(c.g[V0][0] as u32, 0x80)).lw(c);
        c.g[V1][0] = skel;
        c.sw(p(sp_2(c), 0x100), 0);
        c.sw(p(sp_2(c), 0xfc), c.g[V1][0]);
        c.lwc1(0, p(c.g[V0][0] as u32, 0x84)); // model scale
        c.g[S3][0] = Addr(p(c.g[V1][0] as u32, 8)).lw(c); // bones
        c.swc1(0, p(sp_2(c), 0x8));
        c.swc1(1, p(sp_2(c), 0xc));
        c.swc1(0, p(sp_2(c), 0));
        c.swc1(0, p(sp_2(c), 0x4));
        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 4)).lw(c); // bone count
        let empty = c.g[V0][0] as i64 <= 0;
        c.g[S7][0] = 0;
        let spc = sp_2(c);
        if !empty {
            c.cycles += 7;
            c.g[S5][0] = c.g[A2][0];
            c.g[S2][0] = sx(p(spc, 0x10));
            c.g[S6][0] = c.g[S5][0];
            c.sw(p(spc, 0x104), c.g[S5][0]);
            c.g[S1][0] = sx(p(spc, 0xb0));
            c.g[S4][0] = sx(p(spc, 0x50));
            c.g[FP][0] = c.g[S6][0];
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(c.g[S3][0] as u32, 8)).lw(c); // animated
                let still = c.g[V0][0] == 0;
                c.g[A0][0] = u64::MAX;
                if still {
                    // Not animated: use the bone's own matrix.
                    c.cycles += 4;
                    c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 0)).lw(c); // parent
                    c.g[V1][0] = u64::MAX;
                    let root_bone = c.g[A1][0] == c.g[V1][0];
                    c.g[V0][0] = Addr(p(spc, 0xf4)).lw(c); // out
                    if root_bone {
                        // out[i] = mul(root, bone->mtx * scale)
                        c.cycles += 77;
                        let mtx = Addr(p(c.g[S3][0] as u32, 0xc)).lw(c) as u32;
                        scaled_rows(c, mtx, spc, 6, 7, 2, 3, 2);
                        c.g[V1][0] = Addr(p(spc, 0xf8)).lw(c); // root
                        let dst = Addr(p(spc, 0x104)).lw(c);
                        c.g[A0][0] = dst;
                        mul_scaled(
                            c,
                            c.g[V1][0] as u32,
                            p(spc, 0x50),
                            c.g[A0][0] as u32,
                            p(spc, 0xb0),
                            p(spc, 0x10),
                        );
                        c.g[V1][0] = Addr(p(spc, 0xfc)).lw(c);
                    } else {
                        // out[i] = mul(out[parent], bone->mtx * scale)
                        c.cycles += 78;
                        c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
                        let mtx = Addr(p(c.g[S3][0] as u32, 0xc)).lw(c) as u32;
                        c.g[A0][0] = sx(mtx);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        scaled_rows(c, mtx, spc, 7, 6, 2, 3, 2);
                        mul_scaled(
                            c,
                            c.g[A1][0] as u32,
                            p(spc, 0x50),
                            c.g[S5][0] as u32,
                            p(spc, 0xb0),
                            p(spc, 0x10),
                        );
                        c.g[V1][0] = Addr(p(spc, 0xfc)).lw(c);
                    }
                } else {
                    // Animated: the node's intersect matrix, k counts them.
                    c.cycles += 3;
                    c.g[S0][0] = Addr(p(c.g[S3][0] as u32, 0)).lw(c); // parent
                    let root_bone = c.g[S0][0] == c.g[A0][0];
                    c.g[A0][0] = Addr(p(spc, 0xf0)).lw(c); // self
                    if root_bone {
                        // out[i] = mul(root, anim_mtx * scale)
                        c.cycles += 79;
                        c.g[A1][0] = c.g[S7][0];
                        c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(1));
                        callee::ANIM_OBJECT_GET_NODE_INTERSECT_MATRIX.call(c, 0x0034_df10)?;
                        let mtx = c.g[V0][0] as u32;
                        scaled_rows(c, mtx, spc, 6, 7, 3, 4, 3);
                        c.g[V0][0] = Addr(p(spc, 0xf8)).lw(c); // root
                        mul_scaled(
                            c,
                            c.g[V0][0] as u32,
                            p(spc, 0x50),
                            c.g[FP][0] as u32,
                            p(spc, 0xb0),
                            p(spc, 0x10),
                        );
                        c.g[V1][0] = Addr(p(spc, 0xfc)).lw(c);
                    } else {
                        // out[i] = mul(out[parent], anim_mtx * scale)
                        c.cycles += 81;
                        c.g[A1][0] = c.g[S7][0];
                        c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(1));
                        callee::ANIM_OBJECT_GET_NODE_INTERSECT_MATRIX.call(c, 0x0034_ddcc)?;
                        let mtx = c.g[V0][0] as u32;
                        c.g[A1][0] = sx((c.g[S0][0] as u32) << 6);
                        c.g[A0][0] = Addr(p(spc, 0xf4)).lw(c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        scaled_rows(c, mtx, spc, 7, 6, 3, 4, 3);
                        mul_scaled(
                            c,
                            c.g[A1][0] as u32,
                            p(spc, 0x50),
                            c.g[S6][0] as u32,
                            p(spc, 0xb0),
                            p(spc, 0x10),
                        );
                        c.g[V1][0] = Addr(p(spc, 0xfc)).lw(c);
                    }
                }
                // Next bone: advance the bone, both outputs and the counter.
                c.cycles += 13;
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A0][0] = Addr(p(spc, 0x100)).lw(c);
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 4)).lw(c); // bone count
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = Addr(p(spc, 0x104)).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.sw(p(spc, 0x100), c.g[A0][0]);
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x10));
                let more = c.g[V0][0] != 0;
                c.sw(p(spc, 0x104), c.g[V1][0]);
                if more {
                    if events::event_due(c) {
                        events::deliver_event_here(c, 0x0034_dda8);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 12;
        frame.restore(c, 16, 0x1a0);
        frame.restore(c, 17, 0x190);
        frame.restore(c, 18, 0x180);
        frame.restore(c, 19, 0x170);
        frame.restore(c, 20, 0x160);
        frame.restore(c, 21, 0x150);
        frame.restore(c, 22, 0x140);
        frame.restore(c, 23, 0x130);
        frame.restore(c, FP, 0x120);
        frame.restore_ra(c, 0x110);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// cFlagManager_GroupRender (0x34b9b0).

/// `void func_0034B9B0(sFlagSetEntry* e)` (0x34b9b0): push a flag render
/// state (`D_00501420`) on the render context's stack, set its blend words
/// (`0xff000`/`0x14`, or `0x2c` when `e+3` is set), then for every model of
/// the group (`e+0x5c` count, `e+0x94` array) that is loaded
/// (`IsLoaded`: `+0x7d == 0xff`, or the two table lookups) cull the model's
/// box between `m+0x60` and `m+0x6c`; culled models are skipped, the others are
/// drawn either through the handler (`e+0x60`, slot 7) with the identity
/// matrix at `e+0x88`, or through the context's slot 96.
pub fn flag_manager_group_render(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x110);
    let manager = lw(c, gp(c, globals::FLAG_MANAGER));
    c.g[V0][0] = manager;
    frame.save(c, S4, 0xb0);
    frame.save(c, S0, 0xf0);
    let group = r32(c, A0);
    c.g[S4][0] = c.g[A0][0];
    frame.save(c, S1, 0xe0);
    frame.save(c, S2, 0xd0);
    frame.save(c, S3, 0xc0);
    frame.save(c, S5, 0xa0);
    frame.save(c, S6, 0x90);
    frame.save(c, S7, 0x80);
    frame.save(c, FP, 0x70);
    frame.save_ra(c, 0x60);
    let t = manager == 0;
    frame.save_fpr(c, 20, 0x100);
    if t {
        c.cycles += 13;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore(c, FP, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        return;
    }
    guest(c, |c| {
        c.cycles += 30;
        let context = lw(c, gp(c, 0x2a90)) as u32;
        c.g[S3][0] = sx(context);
        c.g[A0][0] = sx(0x50_0000);
        let top = lw(c, p(context, ctx::TOP)) as u32;
        c.g[V0][0] = sx(top);
        copy_render_state(c, V1, A1, A2, top, top);
        let top = lw(c, p(context, ctx::TOP)) as u32;
        c.g[V1][0] = sx(top);
        c.g[V0][0] = sx(p(top, 0x14));
        let pushed = c.g[V0][0];
        Addr(p(context, ctx::TOP)).sw(c, pushed);
        c.g[A3][0] = globals::FLAG_STATE as u64;
        copy_render_state(c, V0, A1, A2, globals::FLAG_STATE, top);
        let flag = Addr(p(group, 3)).lb(c);
        c.g[V0][0] = flag;
        let plain = flag == 0;
        c.g[A3][0] = sx(lw(c, gp(c, 0x2a90)) as u32);
        if plain {
            c.cycles += 6;
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(a);
            c.g[V1][0] = sx(!0x7cu32);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x14;
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
        } else {
            c.cycles += 16;
            c.g[V1][0] = 0xffff_ffff_fff0_0fff;
            c.g[A0][0] = sx(0xf_0000);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A1][0] = sx(a);
            c.g[A0][0] = c.g[A0][0] | 0xf000;
            c.g[A2][0] = sx(!0x7cu32);
            c.g[V0][0] = lw(c, p(a, 0));
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
            let v = c.g[V0][0];
            Addr(p(a, 0)).sw(c, v);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V1][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
            c.g[V0][0] = c.g[V0][0] | 0x2c;
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
        }
        // Flag group models.
        'models: {
            c.cycles += 3;
            let count = lw(c, p(group, 0x5c));
            c.g[V0][0] = count;
            let none = (count as i64) <= 0;
            c.g[S5][0] = 0;
            if none {
                break 'models;
            }
            c.cycles += 8;
            c.g[V0][0] = sx(0x50_0000);
            c.g[AT][0] = sx(0x3f80_0000);
            c.mtc1(1, 20); // f20 = 1.0
            c.g[FP][0] = sx(0x50_0000u32.wrapping_sub(0xe60));
            c.g[S7][0] = sx(frame.at(c, 0x10));
            c.g[S6][0] = sx(frame.at(c, 0x20));
            c.g[V0][0] = sx(r32(c, S5) << 2);
            let mut m = 0u32;
            loop {
                'loaded: {
                    'probe: {
                        c.cycles += 7;
                        c.g[V1][0] = lw(c, gp(c, globals::FLAG_TABLE));
                        c.g[V0][0] = sx(group.wrapping_add(r32(c, V0)));
                        c.g[A0][0] = 0xff;
                        m = lw(c, p(r32(c, V0), 0x94)) as u32;
                        c.g[S2][0] = sx(m);
                        let b = Addr(p(m, 0x7d)).lbu(c);
                        c.g[V0][0] = b;
                        let t = b != c.g[A0][0];
                        c.g[A1][0] = lw(c, p(r32(c, V1), 0));
                        if t {
                            break 'probe;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = 0x1;
                        break 'loaded;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = sx(r32(c, V0) << 3);
                    c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = lw(c, p(r32(c, V0), 0x24));
                    c.g[V1][0] = c.g[V1][0] ^ 0x6;
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = 0;
                        break 'loaded;
                    }
                    c.cycles += 7;
                    c.g[V0][0] = lh(c, p(m, 0x7e));
                    c.g[A0][0] = 0x18;
                    c.mult(0, V1, V0, A0);
                    c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
                    c.g[V1][0] = lw(c, p(r32(c, V0), 0x3f0));
                    c.g[V1][0] = c.g[V1][0] ^ 0x3;
                    c.g[V0][0] = (c.g[V1][0] < 0x1u64) as u64;
                    break 'loaded;
                }
                // Draw (or skip) this model.
                'draw: {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = lw(c, p(group, 0x5c));
                        break 'draw;
                    }
                    'culled: {
                        c.cycles += 32;
                        c.g[S1][0] = lw(c, p(context, ctx::VTABLE));
                        c.g[A0][0] = c.g[S6][0];
                        ldf(c, 0, p(m, 0x60));
                        c.g[S0][0] = lh(c, p(r32(c, S1), 0x2e8));
                        stf(c, 0, frame.at(c, 0));
                        c.g[S0][0] = sx(context.wrapping_add(r32(c, S0)));
                        ldf(c, 0, p(m, 0x64));
                        stf(c, 0, frame.at(c, 4));
                        ldf(c, 1, p(m, 0x68));
                        stf(c, 20, frame.at(c, 0xc));
                        stf(c, 1, frame.at(c, 8));
                        ldf(c, 0, p(m, 0x6c));
                        stf(c, 0, frame.at(c, 0x10));
                        ldf(c, 1, p(m, 0x70));
                        stf(c, 1, frame.at(c, 0x14));
                        ldf(c, 0, p(m, 0x74));
                        stf(c, 20, frame.at(c, 0x1c));
                        stf(c, 0, frame.at(c, 0x18));
                        // vt[43]: the camera matrix pair (hidden struct at sp+0x20).
                        let vt = lw(c, p(context, ctx::VTABLE)) as u32;
                        c.g[V0][0] = sx(vt);
                        c.g[A1][0] = lh(c, p(vt, 0x158));
                        let f43 = lw(c, p(vt, 0x15c));
                        c.g[V1][0] = f43;
                        let f43 = f43 as u32;
                        c.g[A1][0] = sx(context.wrapping_add(c.g[A1][0] as u32));
                        c.g[RA][0] = sx(0x0034_bba4);
                        call_ptr(c, f43, 0x0034_bba4)?;
                        // vt[93]: cull.
                        c.g[V0][0] = lw(c, p(r32(c, S1), 0x2ec));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[SP][0];
                        c.g[A2][0] = c.g[S7][0];
                        let f93 = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x0034_bbbc);
                        c.g[A3][0] = c.g[S6][0];
                        call_ptr(c, f93, 0x0034_bbbc)?;
                        c.g[V1][0] = sx(1);
                        let skip = c.g[V0][0] == c.g[V1][0];
                        c.g[A1][0] = sx(p(m, 0x10));
                        if skip {
                            break 'culled;
                        }
                        // Visible: draw with the identity matrix and the
                        // group's offset (`e+0x88`, `e+0x8c`).
                        'matrix: {
                            c.cycles += 14;
                            c.lqc2(1, r32(c, FP));
                            c.lqc2(2, p(r32(c, FP), 0x10));
                            c.lqc2(3, p(r32(c, FP), 0x20));
                            c.lqc2(4, p(r32(c, FP), 0x30));
                            c.sqc2(1, frame.at(c, 0));
                            c.sqc2(2, frame.at(c, 0x10));
                            c.sqc2(3, frame.at(c, 0x20));
                            c.sqc2(4, frame.at(c, 0x30));
                            ldf(c, 0, p(group, 0x88));
                            ldf(c, 1, p(group, 0x8c));
                            c.g[T0][0] = lw(c, p(group, 0x60));
                            stf(c, 0, frame.at(c, 0x30));
                            let no_handler = c.g[T0][0] == 0;
                            stf(c, 1, frame.at(c, 0x34));
                            if no_handler {
                                break 'matrix;
                            }
                            c.cycles += 9;
                            let handler = r32(c, T0);
                            c.g[V0][0] = lw(c, p(handler, 4));
                            c.g[A3][0] = c.g[SP][0];
                            c.g[A2][0] = lw(c, p(m, 0x94));
                            c.g[A0][0] = lh(c, p(r32(c, V0), 0x38));
                            let f = lw(c, p(r32(c, V0), 0x3c));
                            c.g[V1][0] = f;
                            let f = f as u32;
                            c.g[RA][0] = sx(0x0034_bc1c);
                            c.g[A0][0] = sx(handler.wrapping_add(r32(c, A0)));
                            call_ptr(c, f, 0x0034_bc1c)?;
                            c.g[V0][0] = lw(c, p(group, 0x5c));
                            break 'draw;
                        }
                        // No handler: draw through the context.
                        c.cycles += 9;
                        c.g[V1][0] = lw(c, p(context, ctx::VTABLE));
                        c.g[A1][0] = sx(m);
                        c.g[A2][0] = 0;
                        c.g[A3][0] = 0;
                        c.g[A0][0] = lh(c, p(r32(c, V1), 0x300));
                        c.g[T0][0] = 0;
                        let f = lw(c, p(r32(c, V1), 0x304));
                        c.g[V0][0] = f;
                        let f = f as u32;
                        c.g[RA][0] = sx(0x0034_bc48);
                        c.g[A0][0] = sx(context.wrapping_add(r32(c, A0)));
                        call_ptr(c, f, 0x0034_bc48)?;
                        break 'culled;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = lw(c, p(group, 0x5c));
                    break 'draw;
                }
                c.cycles += 4;
                c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
                c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, S5) << 2);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_bae8);
                    }
                    continue;
                }
                break 'models;
            }
        }
        c.cycles += 3;
        let top = lw(c, p(context, ctx::TOP));
        c.g[V0][0] = top;
        c.g[V0][0] = sx((top as u32).wrapping_sub(0x14));
        let top = c.g[V0][0];
        Addr(p(context, ctx::TOP)).sw(c, top);
        c.cycles += 13;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore(c, FP, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}


/// `void cFlagManager_GroupUpdate(cFlagManager *self)` (0x0034b818): return
/// when the manager is not set up; otherwise advance the four phase
/// animators by `blend + wind*(1-blend)`, rebuild the group's vertices when
/// the frame parity matches, and scroll `u`/`v`.
pub fn cflag_manager_group_update(c: &mut Cpu) {
    use flag_manager::*;
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x280);
        c.g[V0][0] = c.ldw(gp_2(c, 0x1150));
        frame.save(c, S0, 0x270);
        frame.save_ra(c, 0x260);
        let no_manager = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        let mut tail_lq = true;
        'body: {
            if no_manager {
                break 'body;
            }
            c.cycles += 3;
            c.g[V0][0] = c.ldw(off(c.g[S0][0] as u32, 0x5c));
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x270);
                tail_lq = false;
                break 'body;
            }
            c.cycles += 3;
            c.g[V0][0] = c.ldw(off(c.g[S0][0] as u32, 0x60));
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x270);
                tail_lq = false;
                break 'body;
            }
            c.cycles += 14;
            callee::WORLD_GET_FLAG_MANAGER.call(c, 0x0034_b850)?;
            // t = blend + wind->rate * (1 - blend); advance the four phases.
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 1);
            c.g[A0][0] = sx(off(c.g[S0][0] as u32, PHASE));
            c.lwc1(0, off(c.g[S0][0] as u32, BLEND));
            c.g[V1][0] = sx(off(c.g[S0][0] as u32, SPEED));
            c.mov_s(3, 1);
            c.lwc1(2, off(c.g[V0][0] as u32, 0x10));
            c.sub_s(1, 1, 0);
            c.g[V0][0] = 3;
            c.mul_s(2, 2, 1);
            c.add_s(2, 0, 2);
            // Tick rate: the four phases advance per 60 Hz tick, so scale
            // the common step by 60/rate (nothing changes at 60).
            per_tick_60(c, 2);
            loop {
                c.cycles += 8;
                c.lwc1(0, c.g[A0][0] as u32);
                c.lwc1(1, c.g[V1][0] as u32);
                c.mul_s(0, 0, 2);
                c.add_s(0, 1, 0);
                c.c_le_s(3, 0);
                let no_wrap = !c.fcc();
                c.swc1(0, c.g[V1][0] as u32);
                if !no_wrap {
                    c.cycles += 2;
                    c.sub_s(0, 0, 3);
                    c.swc1(0, c.g[V1][0] as u32);
                }
                c.cycles += 4;
                c.g[A0][0] = sx(off(c.g[A0][0] as u32, 4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                let more = (c.g[V0][0] as i64) >= 0;
                c.g[V1][0] = sx(off(c.g[V1][0] as u32, 4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_b880);
                }
            }
            // Frame parity: rebuild the group's vertices when it matches.
            c.cycles += 10;
            callee::WORLD_GET_AI_FRAME_COUNT.call(c, 0x0034_b8c0)?;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
            c.g[A0][0] = c.ldw(off(c.g[S0][0] as u32, PARITY));
            c.g[V1][0] = addu(c.g[V0][0], c.g[V1][0]);
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            if c.g[V0][0] != c.g[A0][0] {
                c.lwc1(1, off(c.g[S0][0] as u32, SCROLL_V));
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x31;
                c.g[V1][0] = u64::MAX;
                loop {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    let again = c.g[V0][0] != c.g[V1][0];
                    c.g[A0][0] = c.g[S0][0];
                    if !again {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_b8e8);
                    }
                }
                c.cycles += 10;
                c.g[A1][0] = c.g[SP][0];
                callee::FLAG_MANAGER_GROUP_COMPUTE_VERTS.call(c, 0x0034_b90c)?;
                c.g[A2][0] = c.ldw(off(c.g[S0][0] as u32, 0x60));
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = c.ldw(off(c.g[A2][0] as u32, 4));
                c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x18));
                c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x1c));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = addu(c.g[A2][0], c.g[A0][0]);
                call_ptr(c, target, 0x0034_b928)?;
                c.lwc1(1, off(c.g[S0][0] as u32, SCROLL_V));
            }
            // Scroll the texture offsets.
            c.cycles += 5;
            c.lwc1(2, gp_2(c, -0x2b00));
            c.c_lt_s(2, 1);
            if c.fcc() {
                c.lwc1(0, off(c.g[S0][0] as u32, V));
            } else {
                c.cycles += 5;
                c.lwc1(0, off(c.g[S0][0] as u32, SCROLL_U));
                c.c_lt_s(2, 0);
                if !c.fcc() {
                    frame.restore(c, S0, 0x270);
                    tail_lq = false;
                    break 'body;
                }
                c.cycles += 1;
                c.lwc1(0, off(c.g[S0][0] as u32, V));
            }
            c.cycles += 7;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 2);
            // Tick rate: the scroll speeds are per 60 Hz tick (the test
            // above them keeps the authored values).
            per_tick_60(c, 1);
            c.add_s(0, 0, 1);
            c.c_lt_s(2, 0);
            let no_wrap = !c.fcc();
            c.swc1(0, off(c.g[S0][0] as u32, V));
            if !no_wrap {
                c.cycles += 2;
                c.sub_s(0, 0, 2);
                c.swc1(0, off(c.g[S0][0] as u32, V));
            }
            c.cycles += 7;
            c.lwc1(1, off(c.g[S0][0] as u32, U));
            c.lwc1(0, off(c.g[S0][0] as u32, SCROLL_U));
            per_tick_60(c, 0);
            c.add_s(0, 1, 0);
            c.c_lt_s(2, 0);
            let no_wrap = !c.fcc();
            c.swc1(0, off(c.g[S0][0] as u32, U));
            if !no_wrap {
                c.cycles += 2;
                c.sub_s(0, 0, 2);
                c.swc1(0, off(c.g[S0][0] as u32, U));
            }
        }
        if tail_lq {
            c.cycles += 1;
            frame.restore(c, S0, 0x270);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x260);
        frame.close(c);
        Ok(())
    })
}


/// `void AnimObject_v52(AnimObject *self)` (0x0034e348): move every flag
/// element's vertices to `self->time` (virtual slot 2 per element), then
/// mark the object dirty.
pub fn anim_object_v52(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.ldw(off(c.g[S0][0] as u32, 0x48));
        let has_items = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if has_items {
            c.cycles += 3;
            c.g[V0][0] = c.ldw(off(c.g[S0][0] as u32, 0x40));
            let no_items = (c.g[V0][0] as i64) <= 0;
            c.g[S1][0] = 0;
            if !no_items {
                c.cycles += 2;
                c.g[S2][0] = 0;
                c.g[A1][0] = c.ldw(off(c.g[S0][0] as u32, 0x48));
                loop {
                    c.cycles += 13;
                    c.g[S1][0] = sx(off(c.g[S1][0] as u32, 1));
                    c.lwc1(12, off(c.g[S0][0] as u32, 4));
                    c.g[A1][0] = addu(c.g[S2][0], c.g[A1][0]);
                    c.g[V0][0] = c.ldw(off(c.g[A1][0] as u32, 0x84));
                    c.g[S2][0] = sx(off(c.g[S2][0] as u32, 0xd0));
                    c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x10));
                    c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x14));
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = addu(c.g[A1][0], c.g[A0][0]);
                    call_ptr(c, target, 0x0034_e3a4)?;
                    c.g[V0][0] = c.ldw(off(c.g[S0][0] as u32, 0x40));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[A1][0] = c.ldw(off(c.g[S0][0] as u32, 0x48));
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_e380);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x20);
        c.stw(off(c.g[S0][0] as u32, 0x10), c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void Fence_Save(Fence* self, Stream* stream)` (0x34d960): 0x24 bytes at
/// `+0x1c`.
pub fn fence_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(c, callee::WORLD_OBJ_SAVE, 0x1c, 0x24, 0x0034_d97c, 0x0034_d998)
    })
}


/// `void cFlagManager_Load(cFlagManager* self, void* arg)` (0x34c898): load
/// the 15 flag groups that follow the header.
pub fn c_flag_manager_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let mut group = r32(c, A0).wrapping_add(0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(group);
        c.g[S1][0] = 0xe;
        c.g[S2][0] = c.g[A1][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = sx(group);
            c.g[RA][0] = sx(0x0034_c8c4);
            c.g[A1][0] = c.g[S2][0];
            callee::C_FLAG_MANAGER_GROUP_LOAD.call(c, 0x0034_c8c4)?;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            group = r32(c, S0).wrapping_add(0x188);
            c.g[S0][0] = sx(group);
            if !more {
                c.cycles += 6;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_c8b8);
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


// ---------------------------------------------------------------------------
// Fences and riders.

/// `void Fence_KickFromNeighbour(Fence* self, int on)` (0x34cde8): give the
/// fence an impulse and arm the kick timer for whichever side was hit.
pub fn fence_kick_from_neighbour(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0034_d6f0, 0x0034_ce04)?;
        if c.g[S0][0] == 0 {
            c.g[V0][0] = lw_2(c, p_2(c.g[S1][0], 0x3c));
            c.cycles += 2;
            let armed = c.g[V0][0] != 0;
            frame.restore(c, S0, 0x20);
            if !armed {
                c.cycles += 2;
                c.g[V0][0] = 4;
                Addr(p_2(c.g[S1][0], 0x3c)).sw(c, c.g[V0][0]);
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw_2(c, p_2(c.g[S1][0], 0x38));
            let armed = c.g[V0][0] != 0;
            frame.restore(c, S0, 0x20);
            if !armed {
                c.cycles += 3;
                c.g[V0][0] = 4;
                Addr(p_2(c.g[S1][0], 0x38)).sw(c, c.g[V0][0]);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void Fence_dtor(Fence* self, int flags)` (0x34cb80): install the deleting
/// vtable, release the handler object through the world's interface (slot
/// 77), then run the `WorldObj` destructor.
pub fn fence_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        open(c, 0x30);
        c.g[V0][0] = 0x490000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x600));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
        let none = c.g[V1][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !none {
            c.cycles += 7;
            c.g[A2][0] = c.lw(gp_3(c, 0x2a90));
            c.g[A1][0] = c.g[V1][0];
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x268));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x26c));
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0034_cbc8);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, target, 0x0034_cbc8)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_fbf0, 0x0034_cbd4)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Flag manager.

/// `cFlagManager* cFlagManager_ctor(cFlagManager* self)` (0x34c428): process
/// base, then fourteen flag groups and the blend constants.
pub fn c_flag_manager_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = sx(p(r32(c, S0), 0x20));
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xe;
        callee::GAME_PROCESS_CTOR.call(c, 0x0034_c450)?;
        c.g[S3][0] = u64::MAX;
        c.g[V0][0] = 0x0049_0000;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x480));
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V0][0]);
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
            callee::C_FLAG_MANAGER_GROUP_CTOR.call(c, 0x0034_c46c)?;
            let more = c.g[S2][0] != c.g[S3][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x188));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_c460);
            }
        }
        c.cycles += 16;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 1);
        c.g[V0][0] = c.g[S0][0];
        c.g[AT][0] = 0x3e80_0000;
        c.mtc1(1, 0);
        c.swc1(1, p(r32(c, S0), 0x14));
        c.swc1(0, p(r32(c, S0), 0x18));
        Addr(p(r32(c, S0), 0x1c)).sw(c, 0);
        Addr(p(r32(c, S0), 0x10)).sw(c, 0);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFlagManager_dtor(cFlagManager* self, int flags)` (0x34c4b8): destroy
/// the embedded flag set's elements backwards, then the game process.
pub fn c_flag_manager_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let flags = abi::arg64(c, 1);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x30);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x480));
        frame.save(c, S3, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x40);
        c.g[V1][0] = sx(p(self_, 0x20));
        frame.save(c, S2, 0x20);
        c.g[S3][0] = flags;
        frame.save_ra(c, 0);
        let no_elems = c.g[V1][0] == 0;
        c.sw(at(c, S1, 0xc), c.g[V0][0]);
        if !no_elems {
            c.cycles += 3;
            c.g[S0][0] = sx(p(r32(c, S1), 0x1718));
            let empty = c.g[V1][0] == c.g[S0][0];
            c.g[S2][0] = c.g[V1][0];
            if !empty {
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x188));
                loop {
                    c.cycles += 8;
                    c.g[A1][0] = 0;
                    c.g[V0][0] = c.lw(at(c, S0, 0x184));
                    c.g[A0][0] = c.lh(at(c, V0, 8));
                    c.g[V1][0] = c.lw(at(c, V0, 0xc));
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    let target = r32(c, V1);
                    call_ptr(c, target, 0x0034_c518)?;
                    let more = c.g[S2][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x188));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_c500);
                    }
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, callee::GAME_PROCESS_DTOR, 0x0034_c52c)?;
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
// Fences.

/// `void Fence_v40(Fence* self, Vec4* impulse)` (0x34d650): when the fence is
/// below full strength, add a scaled impulse to it and count the hit (up to
/// four in each of two counters).
pub fn fence_v40(c: &mut Cpu) {
    c.cycles += 11;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.sq(16, frame_at(c, 0x10));
    c.sd(frame_at(c, 0), c.g[RA][0]);
    c.g[S0][0] = c.g[A0][0];
    c.lwc1(0, p(c.g[S0][0] as u32, 0x30));
    c.c_le_s(1, 0);
    if c.fcc() {
        c.lq(16, frame_at(c, 0x10));
    } else {
        c.cycles += 17;
        c.lwc1(1, gp(c, -0x2ab8));
        c.lqc2(3, p(c.g[A1][0] as u32, 0x20));
        c.lqc2(5, p(c.g[A1][0] as u32, 0x10));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 12);
        c.lwc1(0, p(c.g[A1][0] as u32, 0x30));
        c.mul_s(12, 12, 1);
        c.mul_s(12, 12, 0);
        if call_ptr(c, 0x0034_d6f0, 0x0034_d6b4).is_err() { return; } // Fence_AddImpulse
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x38));
        if c.g[V0][0] != 0 {
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x3c));
        } else {
            c.cycles += 3;
            c.g[V0][0] = 4;
            sw(c, p(c.g[S0][0] as u32, 0x38), c.g[V0][0]);
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x3c));
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.lq(16, frame_at(c, 0x10));
        } else {
            c.cycles += 3;
            c.g[V0][0] = 4;
            sw(c, p(c.g[S0][0] as u32, 0x3c), c.g[V0][0]);
            c.lq(16, frame_at(c, 0x10));
        }
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld(frame_at(c, 0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t;
}


// ---------------------------------------------------------------------------
// `cFlagManager_GroupLoad` (0x34c378): let the stream create the group's mesh
// data, fill each group item from the stream, then build the mesh for the
// first item.
pub fn cflag_manager_group_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0x60;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0u64;
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw((c.g[S3][0] as u32));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x0034_c3b8)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x94));
        if !empty {
            c.cycles += 1;
            loop {
                c.cycles += 11;
                c.g[V1][0] = c.lw((c.g[S3][0] as u32));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, target, 0x0034_c3e0)?;
                c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_c3c8);
                }
            }
        }
        c.cycles += 2;
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = c.g[S1][0];
        if !none {
            c.cycles += 2;
            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x94));
            callee::FLAG_MANAGER_GROUP_BUILD_MESH.call(c, 0x0034_c408)?;
        }
        c.cycles += 7;
        c.lq(16, frame.at(c, 0x40));
        c.lq(17, frame.at(c, 0x30));
        c.lq(18, frame.at(c, 0x20));
        c.lq(19, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFlagManager_AddFlag(self, group, flag)` (0x34c548): add a flag to
/// the matching group, or to the first group with room.
pub fn cflag_manager_add_flag(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0034_c548 => {
                    c.cycles += 12;
                    let frame = Frame::open(c, 0x60);
                    frame.save(c, S4, 0x10);
                    frame.save(c, S0, 0x50);
                    frame.save(c, S1, 0x40);
                    frame.save(c, S2, 0x30);
                    frame.save(c, S3, 0x20);
                    frame.save_ra(c, 0);
                    let this = c.g[A0][0];
                    c.g[S4][0] = this;
                    c.g[S0][0] = sx((this as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A2][0];
                    c.g[S2][0] = 0;
                    c.g[S3][0] = c.g[A1][0];
                    pc = 0x0034_c578;
                }
                0x0034_c578 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5c)).lw(c);
                    let empty = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    pc = if empty { 0x0034_c598 } else { 0x0034_c584 };
                }
                0x0034_c584 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x0034_afe8, 0x0034_c590)?;
                    let matched = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    pc = if matched { 0x0034_c5bc } else { 0x0034_c598 };
                }
                0x0034_c598 => {
                    c.cycles += 4;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 15) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x188));
                    if !more {
                        pc = 0x0034_c5a8;
                    } else {
                        if crate::events::back_edge_unwinds(c, 0x0034_c578) {
                            return Ok(());
                        }
                        pc = 0x0034_c578;
                    }
                }
                0x0034_c5a8 => {
                    c.cycles += 2;
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = 0;
                    pc = 0x0034_c5b0;
                }
                0x0034_c5b0 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x5c)).lw(c);
                    let busy = c.g[V0][0] != 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    pc = if busy { 0x0034_c5d0 } else { 0x0034_c5bc };
                }
                0x0034_c5bc => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x0034_b038, 0x0034_c5c8)?;
                    c.lq(16, p(c.g[SP][0] as u32, 0x50));
                    pc = 0x0034_c5e0;
                }
                0x0034_c5d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 15) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x188));
                    if !more {
                        pc = 0x0034_c5dc;
                    } else {
                        if crate::events::back_edge_unwinds(c, 0x0034_c5b0) {
                            return Ok(());
                        }
                        pc = 0x0034_c5b0;
                    }
                }
                0x0034_c5dc => {
                    c.cycles += 1;
                    c.lq(16, p(c.g[SP][0] as u32, 0x50));
                    pc = 0x0034_c5e0;
                }
                0x0034_c5e0 => {
                    c.cycles += 7;
                    c.lq(17, p(c.g[SP][0] as u32, 0x40));
                    c.lq(18, p(c.g[SP][0] as u32, 0x30));
                    c.lq(19, p(c.g[SP][0] as u32, 0x20));
                    c.lq(20, p(c.g[SP][0] as u32, 0x10));
                    c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
                    let ret = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.pc = ret;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0034_c548, 0x0034_c5fc);
                    return Ok(());
                }
            }
        }
    })
}


// ---------------------------------------------------------------------------
// 0x0034d778 Fence_Render
// ---------------------------------------------------------------------------

/// `int Fence_Render(Fence* self)` (0x34d778).
///
/// Returns 1 when the fence's board object is not in the "hidden" state; 0
/// when the rider's board model is not the active type or the visual state
/// mismatches; otherwise builds the render object for the fence's locator and
/// returns the result of the draw call. `v1` carries the last visual-state word
/// on the early-out paths.
pub fn fence_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xd0);
        c.g[V0][0] = c.lw(gp_4(c, 0x16c8));
        frame.save(c, 21, 0x70);
        frame.save(c, 16, 0xc0);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, 17, 0xb0);
        c.g[A0][0] = 0xff;
        frame.save(c, 18, 0xa0);
        frame.save(c, 19, 0x90);
        frame.save(c, 20, 0x80);
        frame.save_ra(c, 0x60);

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
            c.g[S4][0] = c.lw(gp_4(c, 0x2a90));
            if t {
                break 'body;
            }
            c.cycles += 37;
            c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
            c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.lw(p(c.g[S4][0] as u32, 0x10d8));
            c.g[A0][0] = c.g[S2][0];
            c.lwc1(0, p(c.g[V0][0] as u32, 0x60));
            c.g[S0][0] = c.lh(p(c.g[S1][0] as u32, 0x2e8));
            c.swc1(0, c.g[SP][0] as u32);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 2);
            c.g[S0][0] = sx(c.g[S4][0] as u32 + c.g[S0][0] as u32);
            c.lwc1(0, p(c.g[V0][0] as u32, 0x64));
            c.swc1(0, p(c.g[SP][0] as u32, 4));
            c.lwc1(1, p(c.g[V0][0] as u32, 0x68));
            c.swc1(2, p(c.g[SP][0] as u32, 0xc));
            c.swc1(1, p(c.g[SP][0] as u32, 8));
            c.lwc1(0, p(c.g[V0][0] as u32, 0x6c));
            c.swc1(0, p(c.g[SP][0] as u32, 0x10));
            c.lwc1(1, p(c.g[V0][0] as u32, 0x70));
            c.swc1(1, p(c.g[SP][0] as u32, 0x14));
            c.lwc1(0, p(c.g[V0][0] as u32, 0x74));
            c.swc1(2, p(c.g[SP][0] as u32, 0x1c));
            c.swc1(0, p(c.g[SP][0] as u32, 0x18));
            c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0x10d8));
            c.g[A1][0] = c.lh(p(c.g[V0][0] as u32, 0x158));
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x15c));
            let t = c.g[V1][0] as u32;
            c.g[A1][0] = sx(c.g[S4][0] as u32 + c.g[A1][0] as u32);
            call_ptr(c, t, 0x0034_d868)?;
            c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x2ec));
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S2][0];
            let t = c.g[V0][0] as u32;
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, t, 0x0034_d880)?;
            c.g[V1][0] = 1;
            let t = c.g[V0][0] == c.g[V1][0];
            c.g[A1][0] = 0xffff_ffff_ffff_ff83;
            if t {
                break 'body;
            }
            'inner: {
              'no_anim: {
                c.cycles += 22;
                c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                c.ldl(4, p(c.g[V0][0] as u32, 7));
                c.ldr(4, c.g[V0][0] as u32);
                c.ldl(6, p(c.g[V0][0] as u32, 0xf));
                c.ldr(6, p(c.g[V0][0] as u32, 8));
                c.g[A3][0] = c.lw(p(c.g[V0][0] as u32, 0x10));
                c.sdl(4, p(c.g[V0][0] as u32, 0x1b));
                c.sdr(4, p(c.g[V0][0] as u32, 0x14));
                c.sdl(6, p(c.g[V0][0] as u32, 0x23));
                c.sdr(6, p(c.g[V0][0] as u32, 0x1c));
                c.sw(p(c.g[V0][0] as u32, 0x24), c.g[A3][0]);
                c.g[A0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                c.g[V1][0] = sx(c.g[A0][0] as u32 + 0x14);
                c.sw(p(c.g[S4][0] as u32, 0xe84), c.g[V1][0]);
                c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 0x18));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | 0x14;
                c.sw(p(c.g[A0][0] as u32, 0x18), c.g[V0][0]);
                c.g[A0][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
                c.g[T0][0] = c.lw(p(c.g[S5][0] as u32, 0x40));
                let t = c.g[T0][0] == 0;
                c.g[A1][0] = sx(c.g[A0][0] as u32 + 0x10);
                if t {
                    break 'no_anim;
                }
                c.cycles += 9;
                c.g[V0][0] = c.lw(p(c.g[T0][0] as u32, 4));
                c.g[A3][0] = 0;
                c.g[A2][0] = c.lw(p(c.g[A0][0] as u32, 0x94));
                c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x38));
                c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x3c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx(c.g[T0][0] as u32 + c.g[A0][0] as u32);
                call_ptr(c, t, 0x0034_d900)?;
                let t = true;
                c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                if t {
                    break 'inner;
                }
              }
                c.cycles += 10;
                c.g[V1][0] = c.lw(p(c.g[S4][0] as u32, 0x10d8));
                c.g[A1][0] = c.g[A0][0];
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x300));
                c.g[T0][0] = 0;
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x304));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx(c.g[S4][0] as u32 + c.g[A0][0] as u32);
                call_ptr(c, t, 0x0034_d92c)?;
                c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                break 'inner;
            }
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
            c.sw(p(c.g[S4][0] as u32, 0xe84), c.g[V0][0]);
        }

        c.cycles += 9;
        frame.restore(c, 16, 0xc0);
        frame.restore(c, 17, 0xb0);
        frame.restore(c, 18, 0xa0);
        frame.restore(c, 19, 0x90);
        frame.restore(c, 20, 0x80);
        frame.restore(c, 21, 0x70);
        frame.restore_ra(c, 0x60);
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
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::put_vec;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    use ssx3_ee as ee;
    use crate::difftest::RA as RET;
    use crate::difftest::fakes::ptr;
    use crate::difftest::elf_image;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::fakes;
    pub(crate) use crate::game::intersect::worldsphtree::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::intersect::worldsphtree::tests::f;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS_3 as SEEDS_2;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::CTX;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::TOP;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::TEX;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::BASE;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::uf;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::ctx_at;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::put_ctx;
    pub(crate) use crate::game::input::inputmap::tests::put_f32;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::intersect::worldsphtree::tests::fake_stream;
    pub(crate) use crate::game::intersect::worldsphtree::tests::save_test;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_4;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;
    pub(crate) use crate::game::fe::feasyncfile::tests::w16;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::wscript::wscriptman::tests::w as w_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_3;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE_3;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_5;
    pub(crate) use crate::game::util::menu::tests::w as w_5;
    pub(crate) use crate::game::util::menu::tests::wf_2 as wf;


    #[test]
    fn fence_add_impulse_matches() {
        check(ssx3_ee::s11::f_0034d6f0, fence_add_impulse, 300, &|c, seed| {
            let mut r = Rng(0x1015 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[GP][0] = GAME_GP as u64;
            c.f[12] = r.edge_f32();
            c.bus.write32(DATA + 0x30, r.edge_f32());
            c.bus.write32(DATA + 0x34, r.edge_f32());
            c.bus.write32(GAME_GP - 0x2ab4, r.edge_f32());
            c.bus.write32(GAME_GP - 0x2ab0, r.edge_f32());
            c.bus.write32(GAME_GP - 0x2aac, r.edge_f32());
        });
    }


    #[test]
    fn anim_object_get_node_intersect_matrix_matches() {
        check_lohi(ssx3_ee::s11::f_0034e320, anim_object_get_node_intersect_matrix, 200, &|c, seed| {
            let mut r = Rng(0x1016 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 8) as u64;
            c.bus.write32(DATA + 0x48, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
            c.bus.write32(DATA + 0x100 + 0x90, r.next_u32());
        });
    }

    

    #[test]
    fn anim_object_build_matrices_matches() {
        const SELF: u32 = DATA + 0x1000;
        const MODEL: u32 = DATA + 0x1100;
        const SKEL: u32 = DATA + 0x1200;
        const BONES: u32 = DATA + 0x1400;
        const OUT: u32 = DATA + 0x2000;
        const ROOT: u32 = DATA + 0x3000;
        const NODES: u32 = DATA + 0x4000;
        const OWN: u32 = DATA + 0x6000;
        check(
            ssx3_ee::s11::f_0034dd18__generated,
            anim_object_build_matrices,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x4005, 0);
                let n = 1 + seed % 4;
                c.bus.write32(SELF + 0x2c, MODEL);
                c.bus.write32(SELF + 0x48, NODES);
                c.bus.write32(MODEL + 0x80, SKEL);
                c.bus.write32(MODEL + 0x84, f(1.0));
                c.bus.write32(SKEL + 4, n);
                c.bus.write32(SKEL + 8, BONES);
                for i in 0..n {
                    let b = BONES + 0x10 * i;
                    let parent = if i == 0 || seed % 3 == 0 {
                        0xffff_ffff
                    } else {
                        i - 1
                    };
                    c.bus.write32(b, parent);
                    c.bus.write32(b + 8, (seed + i) % 2);
                    c.bus.write32(b + 0xc, OWN + 0x40 * i);
                    for k in 0..16 {
                        c.bus.write32(OWN + 0x40 * i + 4 * k, r.f32(2.0));
                    }
                }
                for k in 0..n {
                    let m = NODES + 0x90 + 0xd0 * k;
                    for j in 0..16 {
                        c.bus.write32(m + 4 * j, r.f32(1.0));
                    }
                }
                for k in 0..16 {
                    c.bus.write32(ROOT + 4 * k, r.f32(1.0));
                }
                for k in 0..(16 * n) {
                    c.bus.write32(OUT + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(ROOT);
                c.g[A2][0] = sx(OUT);
            },
        );
    }


    #[test]
    fn flag_manager_group_render_matches() {
        check(ee::s11::f_0034b9b0__generated, flag_manager_group_render, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0xb9b0, 5);
            put_ctx(c, &mut r, &[0x158, 0x2e8, 0x300]);
            c.bus.write32(GAME_GP.wrapping_add(globals::FLAG_MANAGER as u32), (seed % 8 != 0) as u32);
            for i in 0..5 {
                c.bus.write32(globals::FLAG_STATE + 4 * i, r.next_u32());
            }
            for i in 0..4u32 {
                let mut m = [0u32; 4];
                m[i as usize] = f(1.0);
                put_vec(c, globals::IDENTITY_MAT + 16 * i, m);
            }
            // The model-loaded table: `base + idx*8 + 0x24` and
            // `base + field*0x18 + 0x3f0`.
            let table = BASE + 0x3_0000;
            let mbase = BASE + 0x4_0000;
            c.bus.write32(GAME_GP.wrapping_add(globals::FLAG_TABLE as u32), table);
            c.bus.write32(table, mbase);
            for idx in 0..4u32 {
                c.bus.write32(mbase + 8 * idx + 0x24, if idx == 1 { 0 } else { 6 });
            }
            for idx in 0..4u32 {
                c.bus.write32(mbase + 0x3f0 + 0x18 * idx, if idx < 2 { 3 } else { 0 });
            }
            let group = BASE;
            let count = seed % 3;
            c.bus.write32(group + 0x5c, count);
            c.bus.write8(group + 3, (seed % 4 == 1) as u8);
            c.bus.write32(group + 0x88, uf(&mut r, -10.0, 10.0));
            c.bus.write32(group + 0x8c, uf(&mut r, -10.0, 10.0));
            if seed % 5 == 0 {
                c.bus.write32(group + 0x60, 0);
            } else {
                let h = BASE + 0x5_0000;
                let vt = vtable(c, &mut r, &[0x38]);
                c.bus.write32(h + 4, vt);
                c.bus.write32(group + 0x60, h);
            }
            for i in 0..count {
                let m = BASE + 0x1_0000 + 0x100 * i;
                c.bus.write32(group + 0x94 + 4 * i, m);
                let mode = (seed + i) % 4;
                let loaded_byte = match mode {
                    0 => 0xff,
                    1 => 1,
                    _ => 2,
                };
                c.bus.write8(m + 0x7d, loaded_byte);
                c.bus.write16(m + 0x7e, if mode == 3 { 2 } else { 1 });
                for k in 0..6 {
                    c.bus.write32(m + 0x60 + 4 * k, uf(&mut r, -5.0, 5.0));
                }
                c.bus.write32(m + 0x94, r.next_u32());
            }
            for k in 0..8u32 {
                c.bus.write32(LOG_RESULTS + 4 * k, if seed % 2 == 0 { 0 } else { 1 });
            }
            c.g[A0][0] = sx(group);
        });
    }


    #[test]
    fn cflag_manager_group_update_matches() {
        covered(
            ssx3_ee::s11::f_0034b818__generated,
            cflag_manager_group_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34b818, 1);
                let selfp = DATA;
                let world = DATA + 0x4000;
                let w84 = DATA + 0x4100;
                let mesh = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_add(0x1150), 1);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, w84);
                c.bus.write32(w84 + 0x70, DATA + 0x4200);
                c.bus.write32(w84 + 0xc, DATA + 0x4300);
                let frame = r.next_u32() % 8;
                c.bus.write32(DATA + 0x4308, frame);
                c.g[A0][0] = selfp as u64;
                c.bus
                    .write32(selfp + 0x5c, if seed % 7 == 0 { 0 } else { 1 });
                c.bus
                    .write32(selfp + 0x60, if seed % 5 == 0 { 0 } else { mesh });
                c.bus.write32(
                    selfp + 0x90,
                    if seed % 2 == 0 {
                        frame & 1
                    } else {
                        (frame + 1) & 1
                    },
                );
                put_f32(c, selfp + 0x4c, r.edge_f32());
                put_f32(c, DATA + 0x4200 + 0x10, r.edge_f32());
                for i in 0..4 {
                    put_f32(c, selfp + 4 + 4 * i, r.edge_f32());
                    put_f32(c, selfp + 0x78 + 4 * i, r.edge_f32());
                }
                put_f32(c, selfp + 0x50, if seed % 4 == 0 { 0 } else { r.f32(1.0) });
                put_f32(c, selfp + 0x54, if seed % 4 == 1 { 0 } else { r.f32(1.0) });
                put_f32(c, selfp + 0x88, r.f32(1.0));
                put_f32(c, selfp + 0x8c, r.f32(1.0));
                put_f32(c, GAME_GP.wrapping_sub(0x2b00), 0x3a83_126f); // 0.001f
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(mesh + 4, vt);
            },
        );
    }


    /// The flag's wave phases (`+0x78..`) and texture scroll (`+0x88`,
    /// `+0x8c`) advance per 60 Hz tick: one update at 60 Hz and `rate/60`
    /// updates at 120 and 240 Hz end at the same values.
    #[test]
    fn cflag_manager_group_update_follows_the_rate() {
        fn after(hz: u32) -> Vec<f32> {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            let _ = machine(c, 1, 0x34b819, 0);
            crate::tick::set_for_test(c, hz);
            let selfp = DATA;
            let world = DATA + 0x4000;
            let w84 = DATA + 0x4100;
            c.bus.write32(GAME_GP.wrapping_add(0x1150), 1);
            c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
            c.bus.write32(world + 0x84, w84);
            c.bus.write32(w84 + 0x70, DATA + 0x4200);
            c.bus.write32(w84 + 0xc, DATA + 0x4300);
            c.bus.write32(DATA + 0x4308, 2);
            c.bus.write32(selfp + 0x5c, 1);
            c.bus.write32(selfp + 0x60, DATA + 0x2000);
            // Parity never matches the (fixed) frame count: no rebuild.
            c.bus.write32(selfp + 0x90, 1);
            put_f32(c, selfp + 0x4c, f(0.25));
            put_f32(c, DATA + 0x4200 + 0x10, f(0.5));
            for i in 0..4 {
                put_f32(c, selfp + 4 + 4 * i, f(0.01 * (i + 1) as f32));
                put_f32(c, selfp + 0x78 + 4 * i, f(0.1));
            }
            put_f32(c, selfp + 0x50, f(0.02));
            put_f32(c, selfp + 0x54, f(0.03));
            put_f32(c, selfp + 0x88, f(0.2));
            put_f32(c, selfp + 0x8c, f(0.3));
            put_f32(c, GAME_GP.wrapping_sub(0x2b00), 0x3a83_126f); // 0.001f
            for _ in 0..hz / 60 {
                c.g[A0][0] = selfp as u64;
                cflag_manager_group_update(c);
                assert_eq!(c.pc, crate::difftest::RA);
            }
            let mut out: Vec<f32> =
                (0..4).map(|i| f32::from_bits(c.bus.read32(selfp + 0x78 + 4 * i))).collect();
            out.push(f32::from_bits(c.bus.read32(selfp + 0x88)));
            out.push(f32::from_bits(c.bus.read32(selfp + 0x8c)));
            out
        }
        let base = after(60);
        assert!((base[0] - 0.1).abs() > 1e-4 && (base[4] - 0.2).abs() > 1e-4);
        for hz in [120, 240] {
            for (a, b) in base.iter().zip(after(hz)) {
                assert!((a - b).abs() < 1e-5, "{hz} Hz: {b} vs {a} at 60");
            }
        }
    }


    #[test]
    fn anim_object_v52_matches() {
        covered(
            ssx3_ee::s11::f_0034e348__generated,
            anim_object_v52,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34e348, 3);
                let selfp = DATA;
                c.g[A0][0] = selfp as u64;
                let items = DATA + 0x1000;
                let count = seed % 4;
                c.bus.write32(selfp + 0x40, count);
                c.bus
                    .write32(selfp + 0x48, if count == 0 { 0 } else { items });
                put_f32(c, selfp + 4, r.edge_f32());
                c.bus.write32(selfp + 0x10, r.next_u32());
                let vt = vtable(c, &mut r, &[0x10]);
                for i in 0..3 {
                    c.bus.write32(items + 0xd0 * i + 0x84, vt);
                }
            },
        );
    }


    #[test]
    fn fence_save_matches() {
        save_test(ee::s11::f_0034d960__generated, fence_save, 0x34d9_60, 0x1c);
    }


    #[test]
    fn c_flag_manager_load_matches() {
        check(
            ee::s11::f_0034c898__generated,
            c_flag_manager_load,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34c8_98, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x2000);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }


    #[test]
    fn fence_kick_from_neighbour_matches() {
        check(ee::s11::f_0034cde8__generated, fence_kick_from_neighbour, SEEDS_5, &|c, seed| {
            let _ = base(c, seed, 0x34cd_e8);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            w(c, s + 0x38, if seed % 3 == 0 { 0 } else { 1 });
            w(c, s + 0x3c, if seed % 4 == 0 { 0 } else { 1 });
            c.g[A0][0] = sx(s);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }


    #[test]
    fn fence_dtor_matches() {
        check(ee::s11::f_0034cb80__generated, fence_dtor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x34_cb80, 1);
            c.g[A0][0] = sx(DATA);
            // Even flags: `WorldObj_dtor` passes them on to a process
            // destructor that would otherwise free the test object.
            c.g[A1][0] = [0u64, 2, 4][seed as usize % 3];
            c.bus.write32(DATA + 0x40, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
            // The world interface (gp_3+0x2a90), its table at +0x10d8 and slot 77.
            let vt = DATA + 0x300;
            c.bus.write16(vt + 0x268, 0);
            c.bus.write32(vt + 0x26c, FAKE);
            c.bus.write32(DATA + 0x400 + 0x10d8, vt);
            c.bus.write32(GAME_GP + 0x2a90, DATA + 0x400);
            c.bus.write32(DATA + 0x18, DATA + 0x500);
            let _ = r.next_u32();
        });
    }


    #[test]
    fn c_flag_manager_ctor_matches() {
        check(ee::s11::f_0034c428__generated, c_flag_manager_ctor, 40, &|c, seed| {
            let _ = base_2(c, seed, 0x34_c428);
            globals(c);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(seed);
        });
    }


    #[test]
    fn c_flag_manager_dtor_matches() {
        check(
            ee::s11::f_0034c4b8__generated,
            c_flag_manager_dtor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x34c4, 0);
                env(c);
                let this = DATA + 0x400;
                let vt = DATA + 0x2000;
                slot(c, vt, 8, EMPTY);
                for i in 0..15u32 {
                    w32(c, this + 0x20 + 0x188 * i + 0x184, vt);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }


    // ----------------------------------------------------------------- fence

    #[test]
    fn fence_v40_matches() {
        check(ssx3_ee::s11::f_0034d650__generated, fence_v40, 12, &|c, seed| {
            let mut r = base_3(c, seed, 0x0034_d650);
            let fence = DATA + 0x100;
            let impulse = DATA + 0x2000;
            clear_2(c, fence, 0x100);
            for k in 0..16 {
                w_3(c, impulse + 4 * k, r.edge_f32());
            }
            w_3(c, fence + 0x30, if seed % 3 == 0 { 1.5f32.to_bits() } else { 0.25f32.to_bits() });
            w_3(c, fence + 0x38, if seed % 2 == 0 { 0 } else { 4 });
            w_3(c, fence + 0x3c, if seed % 2 == 0 { 0 } else { 4 });
            let cst = gp(c, -0x2ab8);
            c.bus.write32(cst, r.edge_f32());
            c.g[A0][0] = sx(fence);
            c.g[A1][0] = sx(impulse);
        });
    }


    #[test]
    fn flag_manager_group_load_matches() {
        check(
            ee::s11::f_0034c378__generated,
            cflag_manager_group_load,
            40,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x34c3);
                let stream = DATA + 0x400;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(stream);
                c.bus.write32(stream, DATA + 0x500);
                c.bus.write16(DATA + 0x500 + 0x10, 0);
                c.bus.write32(DATA + 0x500 + 0x14, FAKE);
                c.bus.write16(DATA + 0x500 + 0x18, 0);
                c.bus.write32(DATA + 0x500 + 0x1c, FAKE + 0x10);
                c.bus.write32(DATA + 0x5c, (seed % 2) as u32);
                fill(c, &mut r, DATA + 0x94, 8);
            },
        );
    }


    // ---------------------------------------------------------------- engine

        #[test]
    fn cflag_manager_add_flag_matches() {
        check(ee::s11::f_0034c548__generated, cflag_manager_add_flag, 6, &|c, seed| {
            let _ = base_5(c, seed, 0x0034_c548, 2);
            c.pc = 0x0034_c548;
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = (seed % 7) as u64;
            let used = (seed % 4) as u32;
            for i in 0..15u32 {
                let slot = OBJ + 0x20 + i * 0x188;
                w_4(c, slot + 0x5c, if i < used { 1 } else { 0 });
                w_4(c, slot + 4, (seed % 4) as u32);
            }
        });
    }


    // -- 0x0031bcb0 Quat_Slerp ------------------------------------------

        // -- 0x0034d778 Fence_Render ---------------------------------------

    #[test]
    fn fence_render_matches() {
        covered(ssx3_ee::s11::f_0034d778__generated, fence_render, 60, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x34d778, 4);
            let world = DATA + 0x8_0000;
            let fence = DATA + 0x1000;
            let board = DATA + 0x2000;
            let manager = DATA + 0x3000;
            let obj = DATA + 0x4000;
            let anim = DATA + 0x5000;
            let draw = DATA + 0x6000;
            w_5(c, GAME_GP.wrapping_add(0x16c8), manager);
            w_5(c, GAME_GP.wrapping_add(0x2a90), world);
            w_5(c, fence + 0x18, board);
            w_5(c, fence + 0x40, anim);
            c.g[A0][0] = sx(fence);
            c.bus.write8(board + 0x7d, if seed % 3 == 0 { 0xff } else { 1 });
            c.bus.write16(board + 0x7e, (r.next_u32() & 3) as u16);
            let table = DATA + 0x7000;
            w_5(c, manager, table);
            for i in 0..4u32 {
                w_5(c, table + 8 * i + 0x24, 6);
                w_5(c, table + 0x18 * i + 0x3f0, 3);
            }
            w_5(c, anim + 4, obj);
            w_5(c, anim + 0x38, 0);
            w_5(c, anim + 0x3c, FAKE);
            w_5(c, anim + 0x94, 0);
            w_5(c, world + 0xe84, draw);
            w_5(c, world + 0x10d8, obj);
            w_5(c, obj + 0x2e8, 0);
            w_5(c, obj + 0x158, 0);
            w_5(c, obj + 0x15c, FAKE + 0x20);
            w_5(c, obj + 0x2ec, FAKE + 0x30);
            w_5(c, obj + 0x300, 0);
            w_5(c, obj + 0x304, FAKE + 0x40);
            for off in [0x60u32, 0x64, 0x68, 0x6c, 0x70, 0x74] {
                wf(c, board + off, r.f32(10.0));
            }
            for i in 0..0x30u32 {
                c.bus.write8(draw + i, i as u8 | 1);
            }
            w_5(c, draw + 0x18, r.next_u32());
        });
    }
}
