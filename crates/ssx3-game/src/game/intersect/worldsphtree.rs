//! `src/intersect/worldsphtree.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::float;
use ssx3_ee_rt::Cpu;
use crate::abi::addu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use crate::events;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use callee::*;
use gather_callee::*;
use ssx3_ee_rt::funcs::call_indirect;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw as lw_2;
pub(crate) use crate::game::be::beintplayer::lh;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::sp_2 as sp_3;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::lh_r;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_3;
pub(crate) use crate::game::intersect::riderspheretree::collision_direction;
pub(crate) use crate::game::intersect::riderspheretree::frame_restore_s0;
pub(crate) use crate::game::intersect::riderspheretree::coll_sphere_tree_common;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_2;
pub(crate) use crate::game::ai::computer::lh_2;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::fe::fepopup::lq;
pub(crate) use crate::game::fe::fepopup::sq;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0033_04e8, coll_sphere_tree_dtor, "CollSphereTree_dtor"),
    (0x0033_0540, coll_sphere_tree_overlaps_aabb, "CollSphereTree_OverlapsAABB"),
    (0x0033_06d8, coll_sphere_tree_get_diameter, "CollSphereTree_GetDiameter"),
    (0x0033_0710, coll_sphere_tree_transform, "CollSphereTree_Transform"),
    (0x0033_0788, coll_sphere_tree_collide_triangle, "CollSphereTree_CollideTriangle"),
    (0x0033_0828, coll_sphere_tree_collide_triangle2_sided, "CollSphereTree_CollideTriangle2Sided"),
    (0x0033_08c8, collspheretree_collide_sphere_tree, "CollSphereTree_CollideSphereTree"),
    (0x0033_0950, collspheretree_collide_aabb, "CollSphereTree_CollideAABB"),
    (0x0033_09d8, cell_gather_objects, "Cell_GatherObjects"),
    (0x0033_3ef8, overlap_list_collide_volume, "OverlapList_CollideVolume"),
    (0x0033_4458, overlap_list_collide_objects, "OverlapList_CollideObjects"),
    (0x0033_4800, nash_create_fwd_diff_cache, "Nash_CreateFwdDiffCache"),
    (0x0033_4850, nash_destroy_fwd_diff_cache, "Nash_DestroyFwdDiffCache"),
    (0x0033_5128, rail_find_nearest_point, "Rail_FindNearestPoint"),
    (0x0033_5960, cell_collide_instance, "Cell_CollideInstance"),
    (0x0033_5b90, cell_collide_object, "Cell_CollideObject"),
    (0x0033_b748, cell_gather_all_recursive, "Cell_GatherAllRecursive"),
    (0x0033_ccf8, cell_collide_volume_recursive, "Cell_CollideVolumeRecursive"),
    (0x0034_0a18, coll_sphere_get_diameter, "CollSphere_GetDiameter"),
    (0x0034_1548, boost_ctor_2, "Boost_ctor_2"),
    (0x0034_15d0, boost_update, "Boost_Update"),
    (0x0034_1770, boost_v48, "Boost_v48"),
    (0x0034_1a30, boost_handle_event, "Boost_HandleEvent"),
    (0x0034_1a50, boost_save, "Boost_Save"),
    (0x0034_1c80, anim_object_ctor_2, "AnimObject_ctor_2"),
    (0x0034_1e48, anim_object_step_loop, "AnimObject_StepLoop"),
    (0x0034_1ec0, anim_object_step_ping_pong, "AnimObject_StepPingPong"),
    (0x0034_1f38, anim_object_step_clamp, "AnimObject_StepClamp"),
    (0x0034_1fc8, anim_object_kill_restorable, "AnimObject_KillRestorable"),
    (0x0034_1fe8, anim_object_v36, "AnimObject_v36"),
    (0x0034_22e8, anim_teeter_ctor_2, "AnimTeeter_ctor_2"),
    (0x0034_24d0, anim_teeter_v53, "AnimTeeter_v53"),
    (0x0034_2768, boost_modifier_ctor, "BoostModifier_ctor"),
    (0x0034_2808, boost_modifier_ctor_2, "BoostModifier_ctor_2"),
    (0x0034_2880, boost_modifier_v02, "BoostModifier_v02"),
    (0x0034_29e0, boost_modifier_v08, "BoostModifier_v08"),
    (0x0034_2b80, boost_modifier_handle_event, "BoostModifier_HandleEvent"),
    (0x0034_2ba0, boost_modifier_v06, "BoostModifier_v06"),
    (0x0035_84b8, boost_modifier_base_save, "BoostModifier_BaseSave"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::be::beintmission::callee::WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::be::beintmission::callee::UITEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::be::beintmission::callee::MPEG2_ERROR;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_COUNT;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_NTH;
    pub(crate) use crate::game::be::beintmission::callee::REWARDS_FILE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_EMPTY_STUB;
    pub(crate) use crate::game::be::beintmission::callee::UBER_TRICK_REFRESH;
    pub(crate) use crate::game::be::beintmission::callee::MEMCARD_SCAN_STEP;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_ON_WIDGET_ACTION;
    pub(crate) use crate::game::be::beintmission::callee::VIEW_POPUP_SHOW_MESSAGE;
    pub(crate) use crate::game::be::beintmission::callee::BUY_ATTRIB_UPDATE_BARS;
    pub(crate) use crate::game::be::beintmission::callee::CAREER_STATS_SHOW_HIGHLIGHTS;
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
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_REMOVE_MESSAGE;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_GET_MESSAGE_COUNT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;
    pub(crate) use crate::game::main::gamestate::callee::GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamestate::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2 as UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_SCRIPT_OBJ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_HAS_BINDING;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_INSTANCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2 as UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3 as EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_SPHERE_TREE;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_AABB;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::STR_HASH;


    pub const COLLIDE_VOLUME_OBJECT: Callee = Callee { addr: 0x0033_4888, f: ee::s11::Collide_VolumeObject };

    pub const WORLD_PAINTER_LOOKUP_CELL: Callee =
        Callee { addr: 0x002c_0a10, f: ee::s08::WorldPainterQuery_LookupCell };

    pub const WORLD_SOUND_QUERY_TREE_NODE: Callee =
        Callee { addr: 0x002b_83f8, f: ee::s08::cWorldSound_QueryTreeNode };

    /// Cross-shard direct calls the generator emitted as `call_indirect`.
    pub const AI_FWD_DIFF_CACHE_GET: u32 = 0x0032_79d0;

    pub const AI_FWD_DIFF_CACHE_COLLIDE_ENTRY: u32 = 0x0032_b6e0;


    pub const WP_DATA_FIND_CELL_INDEX: Callee = Callee {
        addr: 0x002b_af90,
        f: ee::s08::WPData_FindCellIndex,
    };

    pub const INPUT_VM_READ_BUILTIN: Callee = Callee {
        addr: 0x0032_5430,
        f: ee::s10::InputVM_ReadBuiltin,
    };

    pub const INPUT_VM_READ_PAD_CHANNEL: Callee = Callee {
        addr: 0x0032_52f8,
        f: ee::s10::InputVM_ReadPadChannel,
    };

    pub const FOCUS_MAN_COLLECT_OCTREE_NODE: Callee = Callee {
        addr: 0x0014_4368,
        f: ee::s01::FocusMan_CollectOctreeNode,
    };

    pub const ANIM_OBJECT_GET_NODE_INTERSECT_MATRIX: Callee = Callee {
        addr: 0x0034_e320,
        f: ee::s11::AnimObject_GetNodeIntersectMatrix,
    };

    pub const COLLIDE_VOLUME_OBJECT_2: Callee = Callee {
        addr: 0x0033_4888,
        f: ee::s11::Collide_VolumeObject,
    };

    pub const RAIL_MODIFIER_FIND_CLOSEST_POINT: Callee = Callee {
        addr: 0x0035_c698,
        f: ee::s11::RailModifier_FindClosestPoint,
    };

    pub const FLEX_RAIL_FIND_CLOSEST_POINT: Callee = Callee {
        addr: 0x0034_8290,
        f: ee::s11::FlexRail_FindClosestPoint,
    };

    pub const RAIL_FIND_NEAREST_POINT: Callee = Callee {
        addr: 0x0033_5128,
        f: ee::s11::Rail_FindNearestPoint,
    };


    pub const SCRIPT_ENGINE_GET_INSTANCE: Callee =
        Callee { addr: 0x0027_d2e8, f: ee::s07::f_0027d2e8 };

    pub const SCRIPT_ENGINE_BINDING_HAS_PROCESS: Callee =
        Callee { addr: 0x0027_d2c8, f: ee::s07::f_0027d2c8 };

    pub const MENU_SYS_ITEM_CTOR: Callee = Callee { addr: 0x002c_a258, f: ee::s09::f_002ca258 };

    pub const MENU_SYS_ITEM_DRAW_ROW: Callee = Callee { addr: 0x002c_a988, f: ee::s09::f_002ca988 };

    pub const MEMCARD_SJIS_CHAR_TO_ASCII: Callee =
        Callee { addr: 0x002c_2430, f: ee::s08::f_002c2430 };

    pub const PS2_MEMCARD_SET_STATE: Callee = Callee { addr: 0x002c_48c0, f: ee::s08::f_002c48c0 };

    pub const WSCRIPT_PROCESS_LIST_CONTAINS: Callee =
        Callee { addr: 0x0030_b428, f: ee::s10::f_0030b428 };

    pub const WSCRIPT_PROCESS_LIST_REMOVE: Callee =
        Callee { addr: 0x0030_b2d0, f: ee::s10::f_0030b2d0 };

    pub const WSCRIPT_PROCESS_LIST_PUSH_FRONT: Callee =
        Callee { addr: 0x0030_b260, f: ee::s10::f_0030b260 };

    pub const POSITION_MODIFIER_CTOR: Callee = Callee { addr: 0x0035_6eb8, f: ee::s11::f_00356eb8 };

    pub const MOD_OBJ_SET_TRANSFORM_MODIFIER: Callee =
        Callee { addr: 0x0035_54b0, f: ee::s11::f_003554b0 };

    pub const SSX_AUDIO_SYSTEM_RESTART_PEAK_MUSIC: Callee =
        Callee { addr: 0x0028_ef90, f: ee::s07::f_0028ef90 };

    pub const SCRIPT_ENGINE_STOP_PROCESS_SEQ: Callee =
        Callee { addr: 0x0028_2338, f: ee::s07::f_00282338 };

    pub const SCRIPT_ENGINE_KILL_PROCESS: Callee =
        Callee { addr: 0x0028_2540, f: ee::s07::f_00282540 };

    pub const SCRIPT_ENGINE_GET_PROCESS: Callee =
        Callee { addr: 0x0028_2cb0, f: ee::s07::f_00282cb0 };

    pub const SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS: Callee =
        Callee { addr: 0x0028_2908, f: ee::s07::f_00282908 };

    pub const WORLD_OBJ_CTOR_2: Callee = Callee { addr: 0x0034_fb90, f: ee::s11::f_0034fb90 };

    pub const SONG_INSTANCE_GET_BEAT_MS: Callee =
        Callee { addr: 0x002b_2690, f: ee::s08::f_002b2690 };

    pub const SONG_INSTANCE_GET_POSITION_MS: Callee =
        Callee { addr: 0x002b_2600, f: ee::s08::f_002b2600 };

    pub const SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP2: Callee =
        Callee { addr: 0x002a_41c8, f: ee::s08::f_002a41c8 };

    pub const SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP3: Callee =
        Callee { addr: 0x002a_4238, f: ee::s08::f_002a4238 };


    /// `Cell_CollideInstance(ctx, instance)` (0x00335960).
    pub const COLLIDE_INSTANCE: Callee = Callee {
        addr: 0x0033_5960,
        f: ee::s11::Cell_CollideInstance,
    };

    /// `Cell_CollideObject(ctx, object)` (0x00335b90).
    pub const COLLIDE_OBJECT: Callee = Callee {
        addr: 0x0033_5b90,
        f: ee::s11::Cell_CollideObject,
    };

    /// `Cell_CollideVolumeSubtree(tree, ctx)` (0x00340fa0): collects a whole
    /// subtree that lies inside the volume.
    pub const COLLIDE_VOLUME_SUBTREE: Callee = Callee {
        addr: 0x0034_0fa0,
        f: ee::s11::Cell_CollideVolumeSubtree,
    };

    /// This function itself.
    pub const COLLIDE_VOLUME_RECURSIVE: Callee = Callee {
        addr: 0x0033_ccf8,
        f: ee::s11::Cell_CollideVolumeRecursive,
    };
}

/// Contact record (0x80 bytes) fields written here.
pub(crate) mod contact {

    pub const POS: u32 = 0x00;

    pub const NORMAL: u32 = 0x10;

    pub const ROW0: u32 = 0x20;

    pub const ROW1: u32 = 0x30;

    pub const DEPTH: u32 = 0x40;

    pub const KIND: u32 = 0x44;

    pub const ID: u32 = 0x4c;

    pub const OWNER: u32 = 0x54;

    pub const U: u32 = 0x6c;

    pub const V: u32 = 0x70;
}

// ---------------------------------------------------------------------------
// Cells.

/// `WorldObject` fields.
pub(crate) mod obj {

    pub const NEXT: u32 = 0x00;

    /// 0x20: static (own box at +0x60), 0x40: dynamic (box from the
    /// sub-object's virtual 0x168).
    pub const FLAGS: u32 = 0x08;

    pub const SUB: u32 = 0x0c;

    pub const BOX_MIN: u32 = 0x60;

    pub const BOX_MAX: u32 = 0x6c;

    pub const SHAPE: u32 = 0x88;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::ai::computer::table::DELETE;
    pub(crate) use crate::game::ai::computer::table::EA_MEM_FREE;
    pub(crate) use crate::game::ai::computer::table::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::ai::computer::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::ai::computer::table::WAIT_SEMA;
    pub(crate) use crate::game::ai::computer::table::WAKEUP_THREAD;
    pub(crate) use crate::game::ai::computer::table::SLEEP_THREAD;
    pub(crate) use crate::game::ai::computer::table::STR_HASH;
    pub(crate) use crate::game::ai::computer::table::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_MENU_RESET_SCROLL;
    pub(crate) use crate::game::fe::feasyncfile::table::PS2_MEM_CARD_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::table::PATH_STOP_TRACKS;
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

pub(crate) mod addr {
    pub(crate) use crate::game::bx::bxstring2::addr::CBE_GET_BE;
    pub(crate) use crate::game::bx::bxstring2::addr::CBE_GET_INTERFACE;
    pub(crate) use crate::game::bx::bxstring2::addr::BE_INTERFACE_LOOKUP;
    pub(crate) use crate::game::bx::bxstring2::addr::BANK_MONITOR_TIMER_RESET;
    pub(crate) use crate::game::bx::bxstring2::addr::SPEECH_BANK_FREE_CALLBACK;
    pub(crate) use crate::game::bx::bxstring2::addr::UI_ENGINE_TEARDOWN;
    pub(crate) use crate::game::bx::bxstring2::addr::SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::addr::FMODF;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_FREE;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_FREE_NOSIZE;
    pub(crate) use crate::game::bx::bxstring2::addr::MC_MANAGER_GET_INFO;
}

/// The other gathers this function calls (through their generated aliases).
pub(crate) mod gather_callee {
    use super::ee;
    use super::Callee;


    /// `Cell_GatherObjects(ctx, tree)` (0x003309d8): append the node's
    /// objects that overlap `ctx->nodes.box` to `ctx->nodes`.
    pub const GATHER_OBJECTS: Callee = Callee {
        addr: 0x0033_09d8,
        f: ee::s11::Cell_GatherObjects,
    };

    /// `Cell_GatherAllSubtree(tree, ctx)` (0x00340dc0): gather a whole
    /// subtree (objects, all-list and children).
    pub const GATHER_ALL_SUBTREE: Callee = Callee {
        addr: 0x0034_0dc0,
        f: ee::s11::Cell_GatherAllSubtree,
    };

    /// This function itself.
    pub const GATHER_ALL_RECURSIVE: Callee = Callee {
        addr: 0x0033_b748,
        f: ee::s11::Cell_GatherAllRecursive,
    };
}


/// `cvt.w.s`: the bits of the truncated float (same as the FPU op).
pub(crate) fn cvt_w_s(x: FpuF32) -> FpuF32 {
    FpuF32(float::to_fixed(x.0, 0))
}


// ---------------------------------------------------------------------------
// Anim objects: advance a parameter toward its loop bounds.

/// `void AnimObject_StepLoop(AnimObject *o)` (0x341e48): wrap the value at
/// `+0x1c` into `[+0x10, +0x14[, +0x18)`.
pub fn anim_object_step_loop(c: &mut Cpu) {
    let obj = Addr(abi::arg(c, 0));
    c.cycles += 6;
    c.f[0] = 0;
    c.f[1] = obj.off(0x10).read_fpu(c).0;
    let cc = FpuF32(c.f[0]).le(FpuF32(c.f[1]));
    abi::set_fcc(c, cc);
    c.f[0] = obj.off(0x1c).read_fpu(c).0;
    if !cc {
        c.cycles += 6;
        c.f[2] = obj.off(0x14).read_fpu(c).0;
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
        let cc2 = FpuF32(c.f[0]).lt(FpuF32(c.f[2]));
        abi::set_fcc(c, cc2);
        obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
        obj.off(0x28).write_fpu(c, FpuF32(c.f[0]));
        if cc2 {
            c.cycles += 4;
            c.f[1] = (FpuF32(c.f[2]) - FpuF32(c.f[0])).0;
            c.f[0] = obj.off(0x18).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) - FpuF32(c.f[1])).0;
            obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
        }
    } else {
        c.cycles += 6;
        c.f[2] = obj.off(0x18).read_fpu(c).0;
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
        let cc2 = FpuF32(c.f[2]).lt(FpuF32(c.f[0]));
        abi::set_fcc(c, cc2);
        obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
        obj.off(0x28).write_fpu(c, FpuF32(c.f[0]));
        if cc2 {
            c.cycles += 5;
            c.f[1] = (FpuF32(c.f[0]) - FpuF32(c.f[2])).0;
            c.f[0] = obj.off(0x14).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
            obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void AnimObject_StepPingPong(AnimObject *o)` (0x341ec0): advance `+0x1c`
/// within `[+0x10, +0x14]`, reversing the sign of the rate at `+0x10` at a
/// bound.
pub fn anim_object_step_ping_pong(c: &mut Cpu) {
    let obj = Addr(abi::arg(c, 0));
    c.cycles += 6;
    c.f[0] = 0;
    c.f[1] = obj.off(0x10).read_fpu(c).0;
    let cc = FpuF32(c.f[0]).le(FpuF32(c.f[1]));
    abi::set_fcc(c, cc);
    c.f[0] = obj.off(0x1c).read_fpu(c).0;
    if cc {
        c.cycles += 6;
        c.f[2] = obj.off(0x18).read_fpu(c).0;
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
        let cc2 = FpuF32(c.f[2]).lt(FpuF32(c.f[0]));
        abi::set_fcc(c, cc2);
        obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
        obj.off(0x28).write_fpu(c, FpuF32(c.f[0]));
        if !cc2 {
            c.cycles += 2;
            abi::ret(c);
            return;
        } else {
            c.cycles += 4;
            c.f[0] = (FpuF32(c.f[0]) - FpuF32(c.f[2])).0;
            c.f[1] = (-FpuF32(c.f[1])).0;
            c.f[0] = (FpuF32(c.f[2]) - FpuF32(c.f[0])).0;
        }
    } else {
        c.cycles += 6;
        c.f[2] = obj.off(0x14).read_fpu(c).0;
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
        let cc2 = FpuF32(c.f[0]).lt(FpuF32(c.f[2]));
        abi::set_fcc(c, cc2);
        obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
        obj.off(0x28).write_fpu(c, FpuF32(c.f[0]));
        if !cc2 {
            c.cycles += 2;
            abi::ret(c);
            return;
        } else {
            c.cycles += 3;
            c.f[0] = (FpuF32(c.f[2]) - FpuF32(c.f[0])).0;
            c.f[1] = (-FpuF32(c.f[1])).0;
            c.f[0] = (FpuF32(c.f[2]) + FpuF32(c.f[0])).0;
        }
    }
    c.cycles += 2;
    obj.off(0x10).write_fpu(c, FpuF32(c.f[1]));
    obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
    c.cycles += 2;
    abi::ret(c);
}


/// `void AnimObject_StepClamp(AnimObject *o)` (0x341f38): clamp `+0x1c` to the
/// bounds at `+0x10`/`+0x14` (or `+0x18`), flagging the hit in `+0x8`.
pub fn anim_object_step_clamp(c: &mut Cpu) {
    let obj = Addr(abi::arg(c, 0));
    c.cycles += 6;
    c.f[1] = 0;
    c.f[0] = obj.off(0x10).read_fpu(c).0;
    let cc = FpuF32(c.f[1]).le(FpuF32(c.f[0]));
    abi::set_fcc(c, cc);
    c.f[1] = obj.off(0x1c).read_fpu(c).0;
    if !cc {
        // min < 0 path (bounds +0x14)
        c.cycles += 5;
        c.f[0] = obj.off(0x14).read_fpu(c).0;
        let eq = FpuF32(c.f[1]).eq_s(FpuF32(c.f[0]));
        abi::set_fcc(c, eq);
        c.g[V0][0] = 0x1;
        if eq {
            c.cycles += 1;
            obj.off(8).sw(c, 1);
        }
        c.cycles += 6;
        c.f[0] = obj.off(0x1c).read_fpu(c).0;
        c.f[1] = obj.off(0x10).read_fpu(c).0;
        c.f[2] = obj.off(0x14).read_fpu(c).0;
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
        let clt = FpuF32(c.f[0]).lt(FpuF32(c.f[2]));
        abi::set_fcc(c, clt);
        obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
    } else {
        // min >= 0 path (bounds +0x18)
        c.cycles += 5;
        c.f[0] = obj.off(0x18).read_fpu(c).0;
        let eq = FpuF32(c.f[1]).eq_s(FpuF32(c.f[0]));
        abi::set_fcc(c, eq);
        c.g[V0][0] = 0x1;
        if eq {
            c.cycles += 1;
            obj.off(8).sw(c, 1);
        }
        c.cycles += 7;
        c.f[0] = obj.off(0x1c).read_fpu(c).0;
        c.f[1] = obj.off(0x10).read_fpu(c).0;
        c.f[2] = obj.off(0x18).read_fpu(c).0;
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
        let clt = FpuF32(c.f[2]).lt(FpuF32(c.f[0]));
        abi::set_fcc(c, clt);
        obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
    }
    c.cycles += 2;
    let t = !c.fcc();
    obj.off(0x28).write_fpu(c, FpuF32(c.f[0]));
    if !t {
        c.cycles += 1;
        obj.off(0x1c).write_fpu(c, FpuF32(c.f[2]));
    }
    c.cycles += 2;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Boost modifier.

/// `BoostModifier* BoostModifier_ctor(BoostModifier *m, void *desc, void *o)`
/// (0x342768): build a boost modifier from its descriptor.
pub fn boost_modifier_ctor(c: &mut Cpu) {
    c.cycles += 39;
    let this = Addr(abi::arg(c, 0));
    let desc = Addr(abi::arg(c, 1));
    let a2_in = c.g[A2][0];
    c.g[V1][0] = 0x490000;
    c.g[V0][0] = 0x1;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x898));
    this.off(0x40).sw(c, a2_in);
    this.off(4).sw(c, c.g[V1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    this.sw(c, c.g[V0][0]);
    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = desc.off(4).lh(c);
    c.g[AT][0] = 0x42c80000;
    c.f[3] = c.g[AT][0] as u32;
    this.off(0x30).sw(c, c.g[V1][0]);
    c.g[V1][0] = c.g[V1][0] ^ 0x1;
    c.f[1] = Addr(c.g[A2][0] as u32).off(0x10).read_fpu(c).0;
    c.f[1] = FpuF32::from_i32(c.f[1] as i32).0;
    c.f[0] = desc.off(8).read_fpu(c).0;
    Addr(c.g[SP][0] as u32).off(0xc).sw(c, 0);
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[1])).0;
    c.f[1] = cvt_w_s(FpuF32(c.f[0])).0;
    c.g[A2][0] = sx(c.f[1]);
    this.off(0x28).sw(c, c.g[A2][0]);
    if c.g[V1][0] != 0 {
        c.g[A2][0] = 0;
    }
    c.f[0] = desc.off(0x14).read_fpu(c).0;
    c.f[1] = desc.off(0x18).read_fpu(c).0;
    c.f[2] = desc.off(0x1c).read_fpu(c).0;
    let sp = c.g[SP][0] as u32;
    Addr(sp).write_fpu(c, FpuF32(c.f[0]));
    Addr(sp + 4).write_fpu(c, FpuF32(c.f[1]));
    Addr(sp + 8).write_fpu(c, FpuF32(c.f[2]));
    let quad = Addr(sp).read_vec4(c);
    abi::set_gpr128(c, V1, quad.to_bits());
    this.off(0x10).write_vec4(c, quad);
    c.f[0] = desc.off(0x10).read_fpu(c).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[3])).0;
    this.off(0x20).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = desc.off(0xc).read_fpu(c).0;
    this.off(0x2c).sw(c, c.g[A2][0]);
    this.off(0x24).write_fpu(c, FpuF32(c.f[1]));
    c.g[SP][0] = sx(sp.wrapping_add(0x10));
    abi::ret(c);
}


/// The identity-matrix rows (`.rodata`) copied into every contact record.
pub(crate) const IDENTITY_ROWS: u32 = 0x004f_f120;


// ---------------------------------------------------------------------------
// Small register helpers.

/// `lw` through the CPU load path (sign-extended).
#[inline(always)]
pub(crate) fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}


/// The address `reg + off` (32-bit address arithmetic).
#[inline(always)]
pub(crate) fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}


/// `lwc1`.
#[inline(always)]
pub(crate) fn fpu(c: &mut Cpu, a: u32) -> FpuF32 {
    Addr(a).read_fpu(c)
}


/// `swc1` of FPU register `f`.
#[inline(always)]
pub(crate) fn store_fpr(c: &mut Cpu, f: usize, a: u32) {
    let v = FpuF32(c.f[f]);
    Addr(a).write_fpu(c, v);
}


/// `sw` of a register.
#[inline(always)]
pub(crate) fn sw_reg(c: &mut Cpu, a: u32, reg: usize) {
    let v = c.g[reg][0];
    c.sw(a, v);
}


/// `p + off`.
#[inline(always)]
pub(crate) fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}


/// A gcc 2.9x virtual call `vt[slot](obj + delta)`: reads the slot (`lh`
/// delta, `lw` function), leaves the function in `func_reg` and the
/// adjusted `this` in a0, and calls through the function table.
#[inline]
pub(crate) fn virtual_call(c: &mut Cpu, obj: u64, vt: u64, slot: u32, func_reg: usize, ret: u32) -> Flow {
    let m = VSlot::read(c, vt, slot);
    c.g[func_reg][0] = m.func;
    c.g[A0][0] = m.this(obj);
    call_ptr(c, m.func as u32, ret)
}


/// One compare of an AABB rejection chain: `f1 = [f1_at]`, `f0 = [f0_at]`,
/// then `c.lt.s`/`c.le.s f0, f1` (or `f1, f0` when `rev`); the chain goes
/// on while the compare holds.
#[derive(Clone, Copy)]
pub(crate) struct Bound {
    f1_at: u32,
    f0_at: u32,
    rev: bool,
}


pub(crate) const fn b(f1_at: u32, f0_at: u32, rev: bool) -> Bound {
    Bound { f1_at, f0_at, rev }
}


/// The six-compare overlap chain the collision code inlines: `first`
/// cycles before the first compare, 5 before each next one, 6 before the
/// last. Leaves f0/f1 and the FCC bit as the original does; true when all
/// six compares hold.
pub(crate) fn overlap(c: &mut Cpu, le: bool, first: u64, tests: [Bound; 6]) -> bool {
    for (i, t) in tests.iter().enumerate() {
        c.cycles += match i {
            0 => first,
            5 => 6,
            _ => 5,
        };
        let f1 = fpu(c, t.f1_at);
        let f0 = fpu(c, t.f0_at);
        c.f[1] = f1.0;
        c.f[0] = f0.0;
        let (x, y) = if t.rev { (f1, f0) } else { (f0, f1) };
        let holds = if le { x.le(y) } else { x.lt(y) };
        abi::set_fcc(c, holds);
        if !holds {
            return false;
        }
    }
    true
}


/// The next free contact record of a collision query (`query+4` records,
/// `query+0xc` count pointer), re-read as the original does: leaves the
/// count pointer in `count_reg`, the array in `base_reg` and the record in
/// `rec_reg`.
pub(crate) fn next_contact(c: &mut Cpu, query: u32, count_reg: usize, base_reg: usize, rec_reg: usize) -> u32 {
    c.g[count_reg][0] = lw(c, p(query, 0xc));
    c.g[base_reg][0] = lw(c, p(query, 4));
    c.g[rec_reg][0] = lw(c, r32(c, count_reg));
    c.g[rec_reg][0] = sx(r32(c, rec_reg) << 7);
    c.g[rec_reg][0] = addu(c.g[rec_reg][0], c.g[base_reg][0]);
    r32(c, rec_reg)
}


/// `void Cell_GatherObjects(GatherQuery* q, Cell* cell)` (0x3309d8): walks
/// the cell's object list (`cell+0x20`) and appends every object whose box
/// touches the query box (`q+0xc`, `c.le.s`) to `q->out[(*q->count)++]`.
/// Static objects (flag 0x20) use their own box; dynamic ones (0x40) ask
/// their sub-object (virtual 0x160 "active", 0x168 "box").
pub fn cell_gather_objects(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = lw(c, at(c, A1, 0x20));
        c.g[S1][0] = c.g[A0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = lw(c, at(c, S0, obj::FLAGS));
            loop {
                c.cycles += 3;
                let flags = c.g[V1][0];
                c.g[V0][0] = flags & 0x40;
                // Some(extra cycles) when the object is taken.
                let take = if flags & 0x20 != 0 {
                    c.g[V0][0] = lw(c, at(c, S1, 0xc));
                    c.g[V1][0] = 0;
                    let (o, q) = (r32(c, S0), r32(c, V0));
                    let hit = overlap(c, true, 7, [
                        b(p(o, obj::BOX_MAX), q, false),
                        b(p(q, 0x10), p(o, obj::BOX_MIN), false),
                        b(p(q, 4), p(o, obj::BOX_MAX + 4), true),
                        b(p(q, 0x14), p(o, obj::BOX_MIN + 4), false),
                        b(p(q, 8), p(o, obj::BOX_MAX + 8), true),
                        b(p(q, 0x18), p(o, obj::BOX_MIN + 8), false),
                    ]);
                    if hit {
                        c.g[V1][0] = 1;
                    }
                    c.cycles += 2;
                    hit.then_some(2)
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        None
                    } else {
                        c.cycles += 8;
                        let sub = lw(c, at(c, S0, obj::SUB));
                        c.g[A1][0] = sub;
                        c.g[V1][0] = lw(c, p(sub as u32, 0xc));
                        let vt = c.g[V1][0];
                        virtual_call(c, sub, vt, 0x160, V0, 0x0033_0ab8)?;
                        if c.g[V0][0] == 0 {
                            None
                        } else {
                            c.cycles += 13;
                            let sub = lw(c, at(c, S0, obj::SUB));
                            c.g[A1][0] = sub;
                            c.g[V1][0] = lw(c, p(sub as u32, 0xc));
                            let vt = c.g[V1][0];
                            virtual_call(c, sub, vt, 0x168, V0, 0x0033_0ad8)?;
                            c.g[V1][0] = lw(c, at(c, S1, 0xc));
                            c.g[A0][0] = 0;
                            let (bx, q) = (r32(c, V0), r32(c, V1));
                            let hit = overlap(c, true, 0, [
                                b(p(bx, 0x10), q, false),
                                b(p(q, 0x10), bx, false),
                                b(p(q, 4), p(bx, 0x14), true),
                                b(p(q, 0x14), p(bx, 4), false),
                                b(p(q, 8), p(bx, 0x18), true),
                                b(p(q, 0x18), p(bx, 8), false),
                            ]);
                            if hit {
                                c.g[A0][0] = 1;
                            }
                            c.cycles += 2;
                            hit.then_some(1)
                        }
                    }
                };
                if let Some(extra) = take {
                    c.cycles += extra;
                    c.g[A1][0] = lw(c, at(c, S1, 4));
                    c.cycles += 8;
                    c.g[A0][0] = lw(c, at(c, S1, 0));
                    c.g[V1][0] = lw(c, r32(c, A1));
                    c.g[V0][0] = sx(r32(c, V1) << 2);
                    c.g[V0][0] = addu(c.g[V0][0], c.g[A0][0]);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                    sw_reg(c, r32(c, V0), S0);
                    sw_reg(c, r32(c, A1), V1);
                }
                c.g[S0][0] = lw(c, at(c, S0, obj::NEXT));
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[V1][0] = lw(c, at(c, S0, obj::FLAGS));
                if event_due(c) {
                    deliver_event_here(c, 0x0033_09f8);
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


/// `void Cell_CollideInstance(CollideQuery* q, Instance* inst)` (0x335960):
/// for a collidable instance (flags `+0xa` & 0x41 == 0x41) whose box
/// (`+0x158`/`+0x164`) strictly overlaps the volume's box (`q->volume`
/// +0x20 max / +0x30 min), looks up the instance's forward-difference
/// entry (`AIFwdDiffCache_Get(q->cache)`), collides the volume with it and
/// on a hit fills the next contact record.
pub fn cell_collide_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x70);
        c.g[V1][0] = 0x41;
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = c.lhu(at(c, S1, 0xa)) & 0x41;
        c.g[S0][0] = c.g[A0][0];
        if c.g[V0][0] == c.g[V1][0] {
            collide_instance_body(c, frame)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}


pub(crate) fn collide_instance_body(c: &mut Cpu, frame: Frame) -> Flow {
    c.g[V0][0] = lw(c, at(c, S0, 0));
    c.g[V1][0] = 0;
    let (q, i) = (r32(c, V0), r32(c, S1));
    let hit = overlap(c, false, 7, [
        b(p(i, 0x158), p(q, 0x20), true),
        b(p(q, 0x30), p(i, 0x164), true),
        b(p(q, 0x24), p(i, 0x15c), false),
        b(p(q, 0x34), p(i, 0x168), true),
        b(p(q, 0x28), p(i, 0x160), false),
        b(p(q, 0x38), p(i, 0x16c), true),
    ]);
    if hit {
        c.g[V1][0] = 1;
    }
    c.cycles += 2;
    c.g[A1][0] = c.g[S1][0];
    if !hit {
        return Ok(());
    }
    c.cycles += 14;
    // AIFwdDiffCache_Get(q->cache, inst, &entry)
    c.g[A0][0] = lw(c, at(c, S0, 0x14));
    c.g[A2][0] = sx(frame.at(c, 0x30));
    call_ptr(c, callee::AI_FWD_DIFF_CACHE_GET, 0x0033_5a20)?;
    // AIFwdDiffCache_CollideEntry(volume, inst, entry, &pos, &normal, &depth, &uv, q->hint)
    c.g[A0][0] = lw(c, at(c, S0, 0));
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = lw(c, frame.at(c, 0x30));
    c.g[A3][0] = c.g[SP][0];
    c.g[T3][0] = lw(c, at(c, S0, 0x10));
    c.g[T0][0] = sx(frame.at(c, 0x10));
    c.g[T1][0] = sx(frame.at(c, 0x34));
    c.g[T2][0] = sx(frame.at(c, 0x20));
    call_ptr(c, callee::AI_FWD_DIFF_CACHE_COLLIDE_ENTRY, 0x0033_5a44)?;
    let collided = c.g[V0][0] != 0;
    c.g[A0][0] = u64::MAX;
    if !collided {
        return Ok(());
    }
    c.cycles += 75;
    let s0 = r32(c, S0);
    let rec = next_contact(c, s0, V0, A2, V1);
    c.g[A3][0] = 2;
    c.g[A1][0] = sx(IDENTITY_ROWS);
    sw_reg(c, p(rec, contact::KIND), A3);
    sw_reg(c, p(rec, contact::ID), A0);
    c.sw(p(rec, 0x54), 0);
    c.sw(p(rec, 0x50), 0);
    c.sw(p(rec, 0x58), 0);
    sw_reg(c, p(rec, 0x5c), A0);
    sw_reg(c, p(rec, 0x60), A0);
    sw_reg(c, p(rec, 0x64), A0);
    c.lq(V0, IDENTITY_ROWS);
    c.sq(V0, p(rec, contact::ROW0));
    c.lq(A0, IDENTITY_ROWS);
    c.sw(p(rec, 0x70), 0);
    c.sq(A0, p(rec, contact::ROW1));
    c.sw(p(rec, 0x68), 0);
    c.sw(p(rec, 0x6c), 0);

    let rec = next_contact(c, s0, A0, A1, V0);
    c.lq(V1, frame.at(c, 0));
    c.sq(V1, p(rec, contact::POS));
    let rec = next_contact(c, s0, A0, A1, V0);
    c.lq(V1, frame.at(c, 0x10));
    c.sq(V1, p(rec, contact::NORMAL));
    c.g[A1][0] = c.lh(at(c, S1, 0x8));
    let rec = next_contact(c, s0, A0, V1, V0);
    sw_reg(c, p(rec, contact::ID), A1);
    let rec = next_contact(c, s0, V1, A0, V0);
    sw_reg(c, p(rec, contact::OWNER), S1);
    for (from, to) in [(0x34, contact::DEPTH), (0x20, contact::U), (0x24, contact::V)] {
        c.f[0] = fpu(c, frame.at(c, from)).0;
        let rec = next_contact(c, s0, V1, A0, V0);
        store_fpr(c, 0, p(rec, to));
    }
    // ++*q->count
    c.g[V1][0] = lw(c, at(c, S0, 0xc));
    c.g[V0][0] = lw(c, r32(c, V1));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    sw_reg(c, r32(c, V1), V0);
    Ok(())
}


/// `void Cell_CollideObject(CollideQuery* q, WorldObject* o)` (0x335b90):
/// for an object with a shape (`+0x88`) whose box strictly overlaps the
/// volume's (static: own box; dynamic: sub-object virtual 0x160/0x168),
/// `*q->count += Collide_VolumeObject(q->volume, o, &records[count],
/// q->max - count, 2)`.
pub fn cell_collide_object(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, at(c, S0, obj::SHAPE));
        c.g[S1][0] = c.g[A0][0];
        let extra = if c.g[V0][0] == 0 { 1 } else { collide_object_body(c)? };
        c.cycles += extra;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The tests and the collision; returns the cycles charged on the way out
/// before the epilogue (1 after a collision, else 0).
pub(crate) fn collide_object_body(c: &mut Cpu) -> Result<u64, Unwound> {
    c.cycles += 4;
    c.g[V1][0] = lw(c, at(c, S0, obj::FLAGS));
    let flags = c.g[V1][0];
    c.g[V0][0] = flags & 0x40;
    if flags & 0x20 != 0 {
        c.g[V0][0] = lw(c, at(c, S1, 0));
        c.g[V1][0] = 0;
        let (q, o) = (r32(c, V0), r32(c, S0));
        let hit = overlap(c, false, 7, [
            b(p(o, obj::BOX_MIN), p(q, 0x20), true),
            b(p(q, 0x30), p(o, obj::BOX_MAX), true),
            b(p(q, 0x24), p(o, obj::BOX_MIN + 4), false),
            b(p(q, 0x34), p(o, obj::BOX_MAX + 4), true),
            b(p(q, 0x28), p(o, obj::BOX_MIN + 8), false),
            b(p(q, 0x38), p(o, obj::BOX_MAX + 8), true),
        ]);
        if hit {
            c.g[V1][0] = 1;
        }
        c.cycles += 2;
        if !hit {
            return Ok(0);
        }
        c.cycles += 2;
    } else {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            return Ok(0);
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, at(c, S0, obj::SUB));
        if c.g[A1][0] == 0 {
            return Ok(0);
        }
        c.cycles += 7;
        let sub = c.g[A1][0];
        c.g[V1][0] = lw(c, p(sub as u32, 0xc));
        let vt = c.g[V1][0];
        virtual_call(c, sub, vt, 0x160, V0, 0x0033_5c7c)?;
        if c.g[V0][0] == 0 {
            return Ok(0);
        }
        c.cycles += 13;
        let sub = lw(c, at(c, S0, obj::SUB));
        c.g[A1][0] = sub;
        c.g[V1][0] = lw(c, p(sub as u32, 0xc));
        let vt = c.g[V1][0];
        virtual_call(c, sub, vt, 0x168, V0, 0x0033_5c9c)?;
        c.g[V1][0] = lw(c, at(c, S1, 0));
        c.g[A0][0] = 0;
        let (bx, q) = (r32(c, V0), r32(c, V1));
        let hit = overlap(c, false, 0, [
            b(bx, p(q, 0x20), true),
            b(p(q, 0x30), p(bx, 0x10), true),
            b(p(q, 0x24), p(bx, 4), false),
            b(p(q, 0x34), p(bx, 0x14), true),
            b(p(q, 0x28), p(bx, 8), false),
            b(p(q, 0x38), p(bx, 0x18), true),
        ]);
        if hit {
            c.g[A0][0] = 1;
        }
        c.cycles += 2;
        if !hit {
            return Ok(0);
        }
        c.cycles += 1;
    }
    c.g[V0][0] = lw(c, at(c, S1, 0xc));
    // Collide_VolumeObject(q->volume, o, &records[*count], q->max - *count, 2)
    c.cycles += 14;
    c.g[A1][0] = c.g[S0][0];
    c.g[V1][0] = lw(c, at(c, S1, 8));
    c.g[T0][0] = 2;
    c.g[A3][0] = lw(c, r32(c, V0));
    c.g[T1][0] = lw(c, at(c, S1, 4));
    c.g[A2][0] = sx(r32(c, A3) << 7);
    c.g[A0][0] = lw(c, at(c, S1, 0));
    c.g[A3][0] = sx(r32(c, V1).wrapping_sub(r32(c, A3)));
    c.g[A2][0] = addu(c.g[T1][0], c.g[A2][0]);
    callee::COLLIDE_VOLUME_OBJECT.call(c, 0x0033_5d54)?;
    c.g[A0][0] = lw(c, at(c, S1, 0xc));
    c.g[V1][0] = lw(c, r32(c, A0));
    c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
    sw_reg(c, r32(c, A0), V1);
    Ok(1)
}


// ---------------------------------------------------------------------------
// Overlap lists.

/// `int OverlapList_CollideVolume(OverlapList* l, Volume* v, Contact* out,
/// int max, Hint* hint)` (0x333ef8): collides the volume with every
/// collidable instance of the list (`+0x10c` count, `+0x110` pointers;
/// forward-difference cache of `l->world+0xa4`) and every object with a
/// shape (`+8` count, `+0xc` pointers; `Collide_VolumeObject`), filling
/// `out`; returns the number of contacts.
pub fn overlap_list_collide_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        frame.save(c, S4, 0x90);
        frame.save(c, S6, 0x70);
        frame.save(c, S7, 0x60);
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        frame.save(c, S5, 0x80);
        frame.save(c, FP, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = 0;
        c.g[S4][0] = c.g[A0][0];
        c.g[S6][0] = 0;
        c.g[S7][0] = c.g[A2][0];
        c.g[V0][0] = lw(c, at(c, S4, 0x10c));
        sw_reg(c, frame.at(c, 0x38), A3);
        sw_reg(c, frame.at(c, 0x3c), T0);
        // Instances.
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[V0][0] = 0x50_0000;
            c.g[S5][0] = u64::MAX;
            c.g[FP][0] = sx(IDENTITY_ROWS);
            c.g[S0][0] = c.g[S7][0];
            c.g[V0][0] = sx(r32(c, S6) << 2);
            loop {
                c.cycles += 7;
                c.g[A0][0] = 0x41;
                c.g[V0][0] = addu(c.g[S4][0], c.g[V0][0]);
                c.g[S1][0] = lw(c, at(c, V0, 0x110));
                c.g[V1][0] = c.lhu(at(c, S1, 0xa)) & 0x41;
                if c.g[V1][0] == c.g[A0][0] {
                    overlap_list_instance(c, frame)?;
                }
                c.g[V0][0] = lw(c, at(c, S4, 0x10c));
                c.cycles += 4;
                c.g[S6][0] = sx(r32(c, S6).wrapping_add(1));
                c.g[V0][0] = (c.g[S6][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, S6) << 2);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0033_3f60);
                }
            }
        }
        // Objects.
        c.cycles += 3;
        c.g[V0][0] = lw(c, at(c, S4, 8));
        c.g[S1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, S1) << 2);
            loop {
                c.cycles += 5;
                c.g[V0][0] = addu(c.g[S4][0], c.g[V0][0]);
                c.g[S0][0] = lw(c, at(c, V0, 0xc));
                c.g[V1][0] = lw(c, at(c, S0, obj::SHAPE));
                if c.g[V1][0] != 0 {
                    overlap_list_object(c, frame)?;
                } else {
                    c.g[V0][0] = lw(c, at(c, S4, 8));
                }
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, S1) << 2);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0033_40e0);
                }
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore(c, S7, 0x60);
        frame.restore(c, FP, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}


/// One collidable instance (s1) against the volume (s2); a hit writes the
/// record at s0 and advances s0/s3.
pub(crate) fn overlap_list_instance(c: &mut Cpu, frame: Frame) -> Flow {
    let (v, i) = (r32(c, S2), r32(c, S1));
    c.g[V0][0] = 0;
    let hit = overlap(c, false, 6, [
        b(p(v, 0x20), p(i, 0x158), false),
        b(p(v, 0x30), p(i, 0x164), true),
        b(p(v, 0x24), p(i, 0x15c), false),
        b(p(v, 0x34), p(i, 0x168), true),
        b(p(v, 0x28), p(i, 0x160), false),
        b(p(v, 0x38), p(i, 0x16c), true),
    ]);
    if hit {
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    c.g[A1][0] = c.g[S1][0];
    if hit {
        c.cycles += 15;
        c.g[V0][0] = lw(c, at(c, S4, 0));
        c.g[A2][0] = sx(frame.at(c, 0x30));
        c.g[A0][0] = lw(c, at(c, V0, 0xa4));
        call_ptr(c, callee::AI_FWD_DIFF_CACHE_GET, 0x0033_4014)?;
        c.g[A2][0] = lw(c, frame.at(c, 0x30));
        c.g[A0][0] = c.g[S2][0];
        c.g[T3][0] = lw(c, frame.at(c, 0x3c));
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = sx(frame.at(c, 0x10));
        c.g[T0][0] = sx(frame.at(c, 0x20));
        c.g[T1][0] = sx(frame.at(c, 0x34));
        c.g[T2][0] = c.g[SP][0];
        call_ptr(c, callee::AI_FWD_DIFF_CACHE_COLLIDE_ENTRY, 0x0033_4038)?;
        let collided = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if collided {
            c.cycles += 30;
            let rec = r32(c, S0);
            let sp = r32(c, SP);
            c.sw(p(rec, 0x54), 0);
            sw_reg(c, p(rec, contact::ID), S5);
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
            sw_reg(c, p(rec, contact::KIND), V0);
            c.sw(p(rec, 0x50), 0);
            c.sw(p(rec, 0x58), 0);
            sw_reg(c, p(rec, 0x5c), S5);
            sw_reg(c, p(rec, 0x60), S5);
            sw_reg(c, p(rec, 0x64), S5);
            let rows = r32(c, FP);
            c.lq(V0, rows);
            c.lq(V1, p(sp, 0x10));
            c.lq(A0, p(sp, 0x20));
            c.sq(V0, p(rec, contact::ROW0));
            c.lq(V0, rows);
            c.f[2] = fpu(c, p(sp, 0x34)).0;
            c.sw(p(rec, 0x6c), 0);
            c.sw(p(rec, 0x70), 0);
            c.sq(V1, p(rec, contact::POS));
            c.sq(V0, p(rec, contact::ROW1));
            c.sq(A0, p(rec, contact::NORMAL));
            c.sw(p(rec, 0x68), 0);
            c.f[1] = fpu(c, sp).0;
            c.g[V0][0] = c.lh(at(c, S1, 0x8));
            c.f[0] = fpu(c, p(sp, 4)).0;
            sw_reg(c, p(rec, contact::ID), V0);
            sw_reg(c, p(rec, contact::OWNER), S1);
            store_fpr(c, 2, p(rec, contact::DEPTH));
            store_fpr(c, 1, p(rec, contact::U));
            store_fpr(c, 0, p(rec, contact::V));
            c.g[S0][0] = sx(p(rec, 0x80));
        }
    }
    c.cycles += 1;
    Ok(())
}


/// One object with a shape (s0) against the volume (s2); ends with
/// `v0 = l->object_count` as every path of the original does.
pub(crate) fn overlap_list_object(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[V1][0] = lw(c, at(c, S0, obj::FLAGS));
    let flags = c.g[V1][0];
    c.g[V0][0] = flags & 0x40;
    let collide = if flags & 0x20 != 0 {
        c.g[V0][0] = 0;
        let (v, o) = (r32(c, S2), r32(c, S0));
        let hit = overlap(c, false, 6, [
            b(p(v, 0x20), p(o, obj::BOX_MIN), false),
            b(p(v, 0x30), p(o, obj::BOX_MAX), true),
            b(p(v, 0x24), p(o, obj::BOX_MIN + 4), false),
            b(p(v, 0x34), p(o, obj::BOX_MAX + 4), true),
            b(p(v, 0x28), p(o, obj::BOX_MIN + 8), false),
            b(p(v, 0x38), p(o, obj::BOX_MAX + 8), true),
        ]);
        if hit {
            c.g[V0][0] = 1;
        }
        c.cycles += 2;
        c.g[V0][0] = lw(c, frame.at(c, 0x38));
        if hit {
            c.cycles += 2;
        }
        hit
    } else {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = lw(c, at(c, S4, 8));
            return Ok(());
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, at(c, S0, obj::SUB));
        if c.g[A1][0] == 0 {
            c.g[V0][0] = lw(c, at(c, S4, 8));
            return Ok(());
        }
        c.cycles += 7;
        let sub = c.g[A1][0];
        c.g[V1][0] = lw(c, p(sub as u32, 0xc));
        let vt = c.g[V1][0];
        virtual_call(c, sub, vt, 0x160, V0, 0x0033_41bc)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = lw(c, at(c, S4, 8));
            return Ok(());
        }
        c.cycles += 12;
        let sub = lw(c, at(c, S0, obj::SUB));
        c.g[A1][0] = sub;
        c.g[V1][0] = lw(c, p(sub as u32, 0xc));
        let vt = c.g[V1][0];
        virtual_call(c, sub, vt, 0x168, V0, 0x0033_41dc)?;
        c.g[V1][0] = 0;
        let (v, bx) = (r32(c, S2), r32(c, V0));
        let hit = overlap(c, false, 0, [
            b(p(v, 0x20), bx, false),
            b(p(v, 0x30), p(bx, 0x10), true),
            b(p(v, 0x24), p(bx, 4), false),
            b(p(v, 0x34), p(bx, 0x14), true),
            b(p(v, 0x28), p(bx, 8), false),
            b(p(bx, 0x18), p(v, 0x38), false),
        ]);
        if hit {
            c.g[V1][0] = 1;
        }
        c.cycles += 2;
        c.g[V0][0] = lw(c, frame.at(c, 0x38));
        if hit {
            c.cycles += 1;
        }
        hit
    };
    if collide {
        // n += Collide_VolumeObject(v, o, &out[n], max - n, 2)
        c.g[A2][0] = sx(r32(c, S3) << 7);
        c.cycles += 7;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = addu(c.g[S7][0], c.g[A2][0]);
        c.g[A3][0] = sx(r32(c, V0).wrapping_sub(r32(c, S3)));
        c.g[A0][0] = c.g[S2][0];
        c.g[T0][0] = 2;
        callee::COLLIDE_VOLUME_OBJECT.call(c, 0x0033_4280)?;
        c.g[S3][0] = addu(c.g[S3][0], c.g[V0][0]);
    }
    c.cycles += 1;
    c.g[V0][0] = lw(c, at(c, S4, 8));
    Ok(())
}


/// The current stack pointer (the frame moves it, so read it afresh).
#[inline(always)]
pub(crate) fn sp_2(c: &Cpu) -> u32 {
    c.g[SP][0] as u32
}


// ---------------------------------------------------------------------------
// Overlap list: collide objects.

/// One inlined AABB comparison of the overlap query (`x`/`y` are the two
/// floats' addresses; `first_lt` picks `x < y` or `y < x`).
#[inline(always)]
pub(crate) fn box_pair(c: &mut Cpu, x: u32, y: u32, first_lt: bool) -> bool {
    c.lwc1(1, x);
    c.lwc1(0, y);
    if first_lt {
        c.c_lt_s(0, 1);
    } else {
        c.c_lt_s(1, 0);
    }
    c.fcc()
}


/// `int OverlapList_CollideObjects(sOverlapList* self, sQuery* q, char* buf,
/// int cap, int full)` (0x00334458): collide every object of the list whose
/// broad-phase box overlaps `q`'s box, writing up to `cap` contact records
/// (0x80 bytes each) to `buf`; returns how many were written. Objects with
/// flag 0x20 use their node box, objects with flag 0x40 ask their object
/// (`vt[44]` "should test", `vt[45]` box).
pub fn overlap_list_collide_objects(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x2;
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A1][0]; // q
        frame.save(c, S3, 0x50);
        c.g[S2][0] = 0; // written
        frame.save(c, S4, 0x40);
        c.g[S3][0] = 0; // i
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[A0][0]; // self
        frame.save(c, S6, 0x20);
        c.g[S5][0] = c.g[A3][0]; // cap
        frame.save(c, S7, 0x10);
        c.g[S6][0] = c.g[A2][0]; // buf
        frame.save(c, S0, 0x80);
        c.g[S7][0] = 0x1; // mode
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(c.g[S4][0] as u32, 8)).lw(c); // count
        let empty = c.g[V1][0] == 0;
        if c.g[T0][0] != 0 {
            c.g[S7][0] = c.g[V0][0];
        }
        if !empty {
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
            loop {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[S0][0] = Addr(p(c.g[V0][0] as u32, 0xc)).lw(c); // node
                c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c); // flags
                c.g[V0][0] = c.g[V1][0] & 0x20;
                let no_solid = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V1][0] & 0x40;
                let q = c.g[S1][0] as u32;
                let node = c.g[S0][0] as u32;
                let mut collide = false;
                let mut short = false;
                if !no_solid {
                    // The node's own box (+0x60..+0x74) against the query.
                    let mut overlap = 0u64;
                    c.cycles += 6;
                    if box_pair(c, p(q, 0x20), p(node, 0x60), true) {
                        c.cycles += 5;
                        if box_pair(c, p(q, 0x30), p(node, 0x6c), false) {
                            c.cycles += 5;
                            if box_pair(c, p(q, 0x24), p(node, 0x64), true) {
                                c.cycles += 5;
                                if box_pair(c, p(q, 0x34), p(node, 0x70), false) {
                                    c.cycles += 5;
                                    if box_pair(c, p(q, 0x28), p(node, 0x68), true) {
                                        c.cycles += 6;
                                        if box_pair(c, p(q, 0x38), p(node, 0x74), false) {
                                            overlap = 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.g[V0][0] = overlap;
                    c.cycles += 2;
                    let apart = c.g[V0][0] == 0;
                    c.g[A2][0] = sx((c.g[S2][0] as u32) << 7);
                    if !apart {
                        c.cycles += 2;
                        c.g[A3][0] = sx((c.g[S5][0] as u32).wrapping_sub((c.g[S2][0] as u32)));
                        collide = true;
                    }
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        // Neither flag: nothing to do this node.
                        c.g[V0][0] = Addr(p(c.g[S4][0] as u32, 8)).lw(c);
                        short = true;
                    } else {
                        // Ask the object whether it should be tested.
                        c.cycles += 8;
                        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0xc)).lw(c);
                        c.g[V1][0] = Addr(p(c.g[A1][0] as u32, 0xc)).lw(c);
                        c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x160)).lh(c);
                        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x164)).lw(c);
                        let target = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        crate::call::call_ptr(c, target, 0x0033_4578)?;
                        if c.g[V0][0] == 0 {
                            c.g[V0][0] = Addr(p(c.g[S4][0] as u32, 8)).lw(c);
                            short = true;
                        } else {
                            // Its box from vt[45], then the same comparisons.
                            c.cycles += 12;
                            c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0xc)).lw(c);
                            c.g[V1][0] = Addr(p(c.g[A1][0] as u32, 0xc)).lw(c);
                            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x168)).lh(c);
                            c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x16c)).lw(c);
                            let target = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            crate::call::call_ptr(c, target, 0x0033_4598)?;
                            let bx = c.g[V0][0] as u32;
                            let mut overlap = 0u64;
                            if box_pair(c, p(q, 0x20), bx, true) {
                                c.cycles += 5;
                                if box_pair(c, p(q, 0x30), p(bx, 0x10), false) {
                                    c.cycles += 5;
                                    if box_pair(c, p(q, 0x24), p(bx, 4), true) {
                                        c.cycles += 5;
                                        if box_pair(c, p(q, 0x34), p(bx, 0x14), false) {
                                            c.cycles += 5;
                                            if box_pair(c, p(q, 0x28), p(bx, 8), true) {
                                                c.cycles += 6;
                                                if box_pair(c, p(bx, 0x18), p(q, 0x38), true) {
                                                    overlap = 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            c.g[V1][0] = overlap;
                            c.cycles += 2;
                            let apart = c.g[V1][0] == 0;
                            c.g[A2][0] = sx((c.g[S2][0] as u32) << 7);
                            if !apart {
                                c.cycles += 1;
                                c.g[A3][0] =
                                    sx((c.g[S5][0] as u32).wrapping_sub((c.g[S2][0] as u32)));
                                collide = true;
                            }
                        }
                    }
                }
                if collide {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[T0][0] = c.g[S7][0];
                    callee::COLLIDE_VOLUME_OBJECT_2.call(c, 0x0033_4638)?;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                }
                if !short {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(c.g[S4][0] as u32, 8)).lw(c);
                }
                c.cycles += 4;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                if more {
                    if events::event_due(c) {
                        events::deliver_event_here(c, 0x0033_44b0);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 12;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, 16, 0x80);
        frame.restore(c, 17, 0x70);
        frame.restore(c, 18, 0x60);
        frame.restore(c, 19, 0x50);
        frame.restore(c, 20, 0x40);
        frame.restore(c, 21, 0x30);
        frame.restore(c, 22, 0x20);
        frame.restore(c, 23, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Overlap list: nearest rail.

/// `int OverlapList_FindNearestRail(sOverlapList* self, const sVec4* pos,
/// void* a2, int mask, float r)` (0x00334680): for every rail object of the
/// list whose type passes the mask, find the point closest to `pos` (rails
/// via `RailModifier_FindClosestPoint`/`FlexRail_FindClosestPoint`, plain
/// rails via `Rail_FindNearestPoint`) and keep the nearest contact; returns
/// the number of contacts found. The sphere `r` around `pos` is the query
/// volume (`box.min = pos - r`, `box.max = pos + r`).
pub fn overlap_list_find_nearest_rail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0xe0);
        c.mtc1(0, 0); // f0 = 0
        frame.save(c, S0, 0xd0);
        frame.save(c, S2, 0xb0);
        c.g[S0][0] = c.g[A1][0]; // pos
        frame.save(c, S3, 0xa0);
        c.g[S2][0] = 0; // i
        frame.save(c, S4, 0x90);
        c.g[S3][0] = c.g[A3][0]; // mask
        frame.save(c, S5, 0x80);
        c.g[S4][0] = c.g[A2][0]; // a2
        frame.save(c, S6, 0x70);
        c.g[S5][0] = c.g[A0][0]; // self
        frame.save(c, S7, 0x60);
        frame.save(c, S1, 0xc0);
        frame.save(c, FP, 0x50);
        frame.save_ra(c, 0x40);
        c.lq(2, c.g[S0][0] as u32);
        c.swc1(12, p(sp_2(c), 0x28));
        c.sq(2, p(sp_2(c), 0x10));
        c.swc1(0, p(sp_2(c), 0x2c));
        c.sw(p(sp_2(c), 0x30), 0); // cnt
        c.swc1(0, p(sp_2(c), 0x34)); // dist
        c.sq(2, sp_2(c));
        c.swc1(12, p(sp_2(c), 0x20));
        c.swc1(12, p(sp_2(c), 0x24));
        c.lqc2(3, sp_2(c));
        c.lqc2(4, p(sp_2(c), 0x20));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp_2(c));
        c.lqc2(3, p(sp_2(c), 0x10));
        c.lqc2(4, p(sp_2(c), 0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(sp_2(c), 0x10));
        c.g[V0][0] = Addr(p(c.g[S5][0] as u32, 0x210)).lw(c); // count
        let empty = c.g[V0][0] == 0;
        c.g[FP][0] = 0x2; // FP holds the "rail modifier" type
        if !empty {
            c.cycles += 3;
            c.g[S7][0] = 0x3; // "flex rail" type
            c.g[S6][0] = 0x1; // "rail" type
            c.g[S1][0] = sx(p(c.g[S5][0] as u32, 0x214)); // &items[0]
            loop {
                c.cycles += 4;
                c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0)).lw(c); // object
                c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 8)).lw(c); // type
                let is_modifier = c.g[V0][0] == c.g[FP][0];
                c.g[A1][0] = c.g[SP][0];
                let mut finished = false;
                if is_modifier {
                    c.cycles += 10;
                    c.g[A0][0] = Addr(p(c.g[A0][0] as u32, 0xc)).lw(c);
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = c.g[S3][0];
                    c.g[T1][0] = sx(p(sp_2(c), 0x30));
                    c.g[T2][0] = sx(p(sp_2(c), 0x34));
                    c.g[T3][0] = c.g[S4][0];
                    callee::RAIL_MODIFIER_FIND_CLOSEST_POINT.call(c, 0x0033_4750)?;
                    c.g[V0][0] = Addr(p(c.g[S5][0] as u32, 0x210)).lw(c);
                    finished = true;
                }
                if !finished {
                    c.cycles += 2;
                    let is_flex = c.g[V0][0] == c.g[S7][0];
                    c.g[A2][0] = c.g[SP][0];
                    if is_flex {
                        c.cycles += 9;
                        c.g[A0][0] = Addr(p(c.g[A0][0] as u32, 0xc)).lw(c);
                        c.g[A3][0] = c.g[S0][0];
                        c.g[T0][0] = c.g[S3][0];
                        c.g[T1][0] = sx(p(sp_2(c), 0x30));
                        c.g[T2][0] = sx(p(sp_2(c), 0x34));
                        c.g[T3][0] = c.g[S4][0];
                        callee::FLEX_RAIL_FIND_CLOSEST_POINT.call(c, 0x0033_477c)?;
                        c.g[V0][0] = Addr(p(c.g[S5][0] as u32, 0x210)).lw(c);
                        finished = true;
                    }
                }
                if !finished {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[S6][0] {
                        c.g[V0][0] = Addr(p(c.g[S5][0] as u32, 0x210)).lw(c);
                    } else {
                        // A plain rail: only when the object's flags pass.
                        c.cycles += 5;
                        c.g[V1][0] = Addr(p(c.g[A0][0] as u32, 0x68)).lw(c);
                        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x1c)).lw(c);
                        c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
                        let masked_out = c.g[V0][0] == 0;
                        c.g[A1][0] = c.g[SP][0];
                        if !masked_out {
                            c.cycles += 5;
                            c.g[A2][0] = c.g[S0][0];
                            c.g[A3][0] = sx(p(sp_2(c), 0x30));
                            c.g[T0][0] = sx(p(sp_2(c), 0x34));
                            c.g[T1][0] = c.g[S4][0];
                            callee::RAIL_FIND_NEAREST_POINT.call(c, 0x0033_47b4)?;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = Addr(p(c.g[S5][0] as u32, 0x210)).lw(c);
                    }
                }
                c.cycles += 4;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
                if more {
                    if events::event_due(c) {
                        events::deliver_event_here(c, 0x0033_4720);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 13;
        c.g[V0][0] = Addr(p(sp_2(c), 0x30)).lw(c); // contacts found
        frame.restore(c, 16, 0xd0);
        frame.restore(c, 17, 0xc0);
        frame.restore(c, 18, 0xb0);
        frame.restore(c, 19, 0xa0);
        frame.restore(c, 20, 0x90);
        frame.restore(c, 21, 0x80);
        frame.restore(c, 22, 0x70);
        frame.restore(c, 23, 0x60);
        frame.restore(c, FP, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// AnimObject_KillRestorable / MultiParticle_Update.

/// `void AnimObject_KillRestorable(AnimObject* o, int kill)` (0x341fc8): run
/// the object's kill script (`WorldObj_RunKillScript(o + 0x30)`) when `kill`
/// is non-zero.
pub fn anim_object_kill_restorable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let kill = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if kill {
            c.cycles += 2;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x30));
            callee::WORLD_OBJ_RUN_KILL_SCRIPT.call(c, 0x0034_1fdc)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Vtable forwarders: `obj->slot(a1, a2)` where the vtable pointer is the
// first word of the argument object.

/// Forward to slot `slot` of the vtable found at the first word of `a1`.
/// `a1_off` selects `a1 = self + off` (else `a1 = self`).
pub(crate) fn forward_to_arg_vtable(c: &mut Cpu, ret: u32, slot: u32, a2: u32, a1_off: Option<u32>) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x10);
    let target = c.g[A1][0];
    c.g[A3][0] = target;
    frame.save_ra(c, 0);
    c.g[A1][0] = match a1_off {
        Some(off) => sx(r32(c, A0).wrapping_add(off)),
        None => c.g[A0][0],
    };
    c.g[A2][0] = sx(a2);
    let vt = Addr(target as u32).lw(c);
    c.g[V1][0] = vt;
    let m = VSlot::read(c, vt, slot);
    c.g[V0][0] = m.func;
    c.g[A0][0] = m.this(target);
    call_ptr(c, m.func as u32, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}


/// `void BoostModifier_BaseSave(BoostModifier* mod, sWScriptStream* stream)`
/// (0x3584b8).
pub fn boost_modifier_base_save(c: &mut Cpu) {
    guest(c, |c| forward_to_arg_vtable(c, 0x0035_84e0, 8, 4, None))
}

/// `SphereTree_GetRadius` (0x32c590).
pub(crate) const SPHERE_TREE_GET_RADIUS: u32 = 0x0032_c590;

/// `AIFwdDiffCache_dtor` (0x327828).
pub(crate) const AI_FWD_DIFF_CACHE_DTOR: u32 = 0x0032_7828;


/// `float CollSphereTree_GetDiameter(tree)` (0x3306d8): twice the radius of
/// the root sphere, or -1 when the tree is empty.
pub fn coll_sphere_tree_get_diameter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 0);
        } else {
            c.cycles += 4;
            c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x60)).lw(c);
            call_ptr(c, SPHERE_TREE_GET_RADIUS, 0x0033_06f4)?;
            c.add_s(0, 0, 0);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void Nash_DestroyFwdDiffCache(this)` (0x334850): destroy the forward
/// difference cache at `+0xa4` (when there is one) and clear the field.
pub fn nash_destroy_fwd_diff_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[A0][0] = Addr((this as u32).wrapping_add(0xa4)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, AI_FWD_DIFF_CACHE_DTOR, 0x0033_4874)?;
        }
        c.sw((this as u32).wrapping_add(0xa4), 0);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// World object save family.

/// The shared save tail: run the type's `save` prelude, then call the stream's
/// serialiser slot (`vt[1]`) on `obj + off` for `size` bytes.
pub(crate) fn save_field_block(c: &mut Cpu, pre: Callee, off: u32, size: u32, r1: u32, r2: u32) -> Flow {
    c.cycles += 19;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    let obj = r32(c, A0);
    c.g[S1][0] = c.g[A0][0];
    pre.call(c, r1)?;
    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
    c.g[A1][0] = sx(obj.wrapping_add(off));
    c.g[A2][0] = size as u64;
    c.g[A0][0] = Addr(r32(c, V1)).off(8).lh(c);
    c.g[V0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, target, r2)?;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}


/// `void Boost_Save(Boost* self, Stream* stream)` (0x341a50): 0x30 bytes at
/// `+0x20`.
pub fn boost_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(c, callee::WORLD_OBJ_SAVE, 0x20, 0x30, 0x0034_1a6c, 0x0034_1a88)
    })
}


// ---------------------------------------------------------------------------
// Objects and world.

/// `void Nash_CreateFwdDiffCache(void* self)` (0x334800): create the rider's
/// forward-difference cache and initialise it with 0x28 entries.
pub fn nash_create_fwd_diff_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x48_e4f8; // D_0048E4F8, the allocation name
        frame.save(c, S0, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x10;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0033_4828)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0032_7810, 0x0033_4830)?;
        Addr(p(this, 0xa4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x28;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0032_7890, 0x0033_4840)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void CollSphereTree_dtor(CollSphereTree* t, int flags)` (0x3304e8): set
/// the vtable, delete the child array at `+0x68`, and `delete this` when bit
/// 0 of `flags` is set.
pub fn coll_sphere_tree_dtor(c: &mut Cpu) {
    c.cycles += 14;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0048_e5f0);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    c.sw(p_2(c, S1, 0x50), c.g[V0][0]);
    c.g[A0][0] = Addr(p_2(c, S1, 0x68)).lw(c);
    if call_ptr(c, table::DELETE, 0x0033_0514).is_err() {
        return;
    }
    c.g[S0][0] &= 1;
    let keep = c.g[S0][0] != 0;
    frame.restore(c, S0, 0x20);
    if keep {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        if call_ptr(c, table::DELETE, 0x0033_0528).is_err() {
            return;
        }
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


// ---------------------------------------------------------------------------
// Collision primitives (intersect/riderspheretree.cpp).

/// `float CollSphere_GetDiameter(CollSphere* sphere)` (0x340a18): twice the
/// radius at `+0x70`, or -1.0 in `f0` when the sphere is empty.
pub fn coll_sphere_get_diameter(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, A0)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.lwc1(0, r32(c, A0).wrapping_add(0x70));
    c.add_s(0, 0, 0);
    abi::ret(c);
}


/// `void Boost_HandleEvent(Boost* self, int event)` (0x341a30): on event 1
/// (start) remember the current boost level as the event's baseline.
pub fn boost_handle_event(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x1;
    if c.g[A1][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x38)).lw(c);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `float AnimTeeter_v53(AnimTeeter* self)` (0x3424d0): clamp the lean value
/// at `+0x4` to the [lower, upper] limits (`+0x10`, `+0x14`) only while the
/// teeter is heading outwards (`+0x30` is the angular velocity), returning 0
/// when it is already pinned.
pub fn anim_teeter_v53(c: &mut Cpu) {
    c.cycles += 6;
    let this = r32(c, A0);
    c.lwc1(1, this.wrapping_add(0x30)); // velocity
    c.lwc1(0, this.wrapping_add(0x10)); // upper limit
    c.c_le_s(1, 0); // velocity <= upper
    if !c.fcc() {
        // Below the upper limit: test the lower one.
        c.lwc1(0, this.wrapping_add(0x14));
    } else {
        c.cycles += 6;
        c.lwc1(0, this.wrapping_add(0x4)); // current lean
        c.mtc1(0, 1); // f1 = 0.0
        c.c_lt_s(0, 1); // lean < 0
        if !c.fcc() {
            c.lwc1(1, this.wrapping_add(0x30));
            c.cycles += 1;
            c.lwc1(0, this.wrapping_add(0x14));
        } else {
            // Heading back inwards: no clamp.
            c.cycles += 2;
            c.mov_s(0, 1);
            abi::ret(c);
            return;
        }
    }
    // f0 = lower limit, f1 = velocity (or the upper limit).
    c.cycles += 4;
    c.c_le_s(0, 1); // lower <= velocity
    let outwards = !c.fcc();
    c.lwc1(2, this.wrapping_add(0x4));
    if !outwards {
        c.cycles += 4;
        c.mtc1(0, 0); // f0 = 0.0
        c.c_lt_s(0, 2); // 0 < lean
        if c.fcc() {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 1;
    c.mov_s(0, 2);
    c.cycles += 2;
    abi::ret(c);
}


/// `void BoostModifier_HandleEvent(BoostModifier* self, int event)`
/// (0x342b80): on the boost-start event (0x258) remember the boost level as
/// the modifier's baseline.
pub fn boost_modifier_handle_event(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x258;
    if c.g[A1][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x28)).lw(c);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `Boost* Boost_ctor_2(Boost* self, void* a1, void* stream)` (0x341548): the
/// instance-node constructor plus the boost vtable and the stream's
/// transform setup.
pub fn boost_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        callee::WORLD_OBJ_CTOR_2.call(c, 0x0034_1564)?;
        c.g[V1][0] = 0x490000;
        c.g[A1][0] = sx(ea(c.g[S0][0], 0x20));
        c.g[V1][0] = sx(ea(c.g[V1][0], 0x14e0));
        c.g[A2][0] = 0x30;
        c.sw(ea(c.g[S0][0], 0xc), c.g[V1][0]);
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        c.g[A0][0] = c.lh(ea(c.g[V0][0], 0x10));
        c.g[V1][0] = c.lw(ea(c.g[V0][0], 0x14));
        let method = c.g[V1][0] as u32;
        c.g[A0][0] = sx(ea(c.g[S1][0], c.g[A0][0] as u32));
        call_ptr(c, method, 0x0034_158c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `CollSphereTree_Transform(CollSphereTree* self, SphereTree* tree)`
/// (0x330710): copy the tree's matrix block into the collision tree, apply
/// the transform and flag the tree dirty.
pub fn coll_sphere_tree_transform(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        let tree = c.g[A1][0];
        c.g[S1][0] = tree;
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(20, 12);
        c.g[A0][0] = lw_2(c, p(this as u32, 0x68));
        c.g[A1][0] = lw_2(c, p(this as u32, 0x60));
        call_ptr(c, 0x0032_7cc8, 0x0033_073c)?;
        c.g[V1][0] = lw_2(c, p(this as u32, 0x68));
        c.g[A1][0] = tree;
        c.mov_s(12, 20);
        sw(c, p(this as u32, 0x60), c.g[V1][0]);
        c.g[A0][0] = c.g[V1][0];
        call_ptr(c, 0x0032_c770, 0x0033_0754)?;
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x10);
        sw(c, p(this as u32, 4), c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}


/// `BoostModifier_v06(BoostModifier* self, void* stream)` (0x342ba0): save
/// the base modifier, then write `self+0x10` (0x30 bytes) and `self+0x40`
/// through two stream vtable slots.
pub fn boost_modifier_v06(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let stream = c.g[A1][0];
        c.g[S0][0] = stream;
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        call_ptr(c, 0x0035_84b8, 0x0034_2bbc)?;
        c.g[V0][0] = lw_2(c, stream as u32);
        c.g[A1][0] = sx((this as u32).wrapping_add(0x10));
        c.g[A2][0] = 0x30;
        c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 8));
        c.g[V1][0] = lw_2(c, p(c.g[V0][0] as u32, 0xc));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((stream as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_2bd8)?;
        c.g[V0][0] = lw_2(c, stream as u32);
        c.g[A1][0] = lw_2(c, p(this as u32, 0x40));
        c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 0x28));
        c.g[V1][0] = lw_2(c, p(c.g[V0][0] as u32, 0x2c));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((stream as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_2bf0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Rail_FindNearestPoint: the rail's bounding box test, then two VU0 passes
// (five control points, then 24 samples along the segment) tracking the
// closest point on the rail and its normal/tangent frame.

/// `int Rail_FindNearestPoint(Rail *rail, Vec4 *box, Vec4 *out, ...)`?
/// (0x335128). `a0` is the rail object (bounds at +0x6c..+0x80 and a 4x4
/// matrix at +0x10), `a1` the query box (min at +0, max at +0x10), `a2` a
/// vector; the result is written to `a3` (0x335828..0x335840) and the frame to
/// the caller's structures.
///
/// The port keeps the original's stack scratch: the VU0 code exchanges
/// vectors through `sp+0x00..0xc0`, and those stores are part of what a
/// caller (or an interrupt handler) can observe.
pub fn rail_find_nearest_point(c: &mut Cpu) {
    let mut pc = c.pc as u32;
    loop {
        match pc {
            // Bounding-box reject: six compares against the rail's bounds.
            0x0033_5128 => {
                c.cycles += 7;
                c.g[T2][0] = c.g[A0][0];
                let a1 = c.g[A1][0] as u32;
                let t2 = c.g[T2][0] as u32;
                c.lwc1(1, a1.wrapping_add(0x10));
                c.lwc1(0, t2.wrapping_add(0x6c));
                c.c_lt_s(1, 0);
                let out = c.fcc();
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                pc = if out { 0x0033_5954 } else { 0x0033_5144 };
            }
            0x0033_5144 => {
                c.cycles += 5;
                c.lwc1(1, c.g[A1][0] as u32);
                c.lwc1(0, (c.g[T2][0] as u32).wrapping_add(0x78));
                c.c_lt_s(0, 1);
                let out = c.fcc();
                pc = if out { 0x0033_5954 } else { 0x0033_5158 };
            }
            0x0033_5158 => {
                c.cycles += 5;
                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x14));
                c.lwc1(0, (c.g[T2][0] as u32).wrapping_add(0x70));
                c.c_lt_s(1, 0);
                let out = c.fcc();
                pc = if out { 0x0033_5954 } else { 0x0033_516c };
            }
            0x0033_516c => {
                c.cycles += 5;
                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.lwc1(0, (c.g[T2][0] as u32).wrapping_add(0x7c));
                c.c_lt_s(0, 1);
                let out = c.fcc();
                pc = if out { 0x0033_5954 } else { 0x0033_5180 };
            }
            0x0033_5180 => {
                c.cycles += 5;
                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x18));
                c.lwc1(0, (c.g[T2][0] as u32).wrapping_add(0x74));
                c.c_lt_s(1, 0);
                let out = c.fcc();
                pc = if out { 0x0033_5954 } else { 0x0033_5194 };
            }
            0x0033_5194 => {
                c.cycles += 6;
                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.lwc1(0, (c.g[T2][0] as u32).wrapping_add(0x80));
                c.c_lt_s(0, 1);
                let out = c.fcc();
                c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x10));
                pc = if out { 0x0033_5954 } else { 0x0033_51ac };
            }
            // Pass 1 setup: transform the rail origin, seed the running
            // minimum (f2) with 1.0 and the best point with the origin.
            0x0033_51ac => {
                c.cycles += 47;
                let sp = c.g[SP][0] as u32;
                let v0 = c.g[V0][0] as u32;
                c.mtc1(0, 10);
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 12);
                c.g[T3][0] = c.g[V0][0];
                c.swc1(10, sp.wrapping_add(0x48));
                c.mov_s(5, 10);
                c.swc1(12, sp.wrapping_add(0x4c));
                c.mov_s(4, 12);
                c.swc1(10, sp.wrapping_add(0x44));
                c.g[T5][0] = c.g[T3][0];
                c.swc1(10, sp.wrapping_add(0x40));
                c.g[T4][0] = 0;
                c.lqc2(8, sp.wrapping_add(0x40));
                c.lqc2(4, v0);
                c.lqc2(5, v0.wrapping_add(0x10));
                c.lqc2(6, v0.wrapping_add(0x20));
                c.lqc2(7, v0.wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0x50));
                c.g[AT][0] = 0x3e80_0000;
                c.mtc1(1, 6);
                c.lq(2, sp.wrapping_add(0x50));
                c.g[A1][0] = 1;
                c.g[T6][0] = sx(sp.wrapping_add(0x30));
                c.g[A0][0] = sx(sp.wrapping_add(0x60));
                c.sq(2, sp.wrapping_add(0x30));
                c.sq(2, sp);
                c.lqc2(3, c.g[A2][0] as u32);
                c.lqc2(4, sp);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x40));
                c.lq(2, sp.wrapping_add(0x40));
                c.sq(2, sp.wrapping_add(0x30));
                c.lqc2(3, sp.wrapping_add(0x30));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 3);
                pc = 0x0033_5268;
            }
            // Pass 1 body: distance from the query point to the rail origin,
            // clamped against the running minimum (f2), five times.
            0x0033_5268 => {
                c.cycles += 59;
                let sp = c.g[SP][0] as u32;
                let t5 = c.g[T5][0] as u32;
                c.mtc1(5, 0);
                c.cvt_s_w(0, 0);
                c.swc1(4, sp.wrapping_add(0x4c));
                c.mov_s(2, 5);
                c.mul_s(0, 0, 6);
                c.mul_s(1, 0, 0);
                c.swc1(0, sp.wrapping_add(0x48));
                c.mul_s(0, 0, 1);
                c.swc1(1, sp.wrapping_add(0x44));
                c.swc1(0, sp.wrapping_add(0x40));
                c.lqc2(8, sp.wrapping_add(0x40));
                c.lqc2(4, t5);
                c.lqc2(5, t5.wrapping_add(0x10));
                c.lqc2(6, t5.wrapping_add(0x20));
                c.lqc2(7, t5.wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0x50));
                c.lq(2, sp.wrapping_add(0x50));
                c.sq(2, sp.wrapping_add(0x30));
                c.sq(2, sp.wrapping_add(0x10));
                c.lqc2(3, sp.wrapping_add(0x10));
                c.lqc2(4, sp);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x40));
                c.lq(2, sp.wrapping_add(0x40));
                c.sq(2, sp.wrapping_add(0x30));
                c.sq(2, sp.wrapping_add(0x20));
                c.lqc2(3, c.g[A2][0] as u32);
                c.lqc2(4, sp);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x40));
                c.lq(2, sp.wrapping_add(0x40));
                c.sq(2, sp.wrapping_add(0x30));
                c.lqc2(3, sp.wrapping_add(0x30));
                c.lqc2(5, sp.wrapping_add(0x20));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(3, 4);
                c.mtc1(3, 1);
                c.lqc2(3, sp.wrapping_add(0x20));
                c.lqc2(5, sp.wrapping_add(0x20));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 0);
                c.div_s(1, 1, 0);
                c.c_le_s(5, 1);
                if c.fcc() {
                    c.min_s(2, 1, 4);
                }
                pc = 0x0033_5354;
            }
            // Pass 1 step: move the sample point, recompute the closest point.
            0x0033_5354 => {
                c.cycles += 34;
                let sp = c.g[SP][0] as u32;
                c.mfc1(3, 2);
                c.lqc2(4, sp.wrapping_add(0x20));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp.wrapping_add(0x50));
                c.lq(2, sp.wrapping_add(0x50));
                c.sq(2, sp.wrapping_add(0x40));
                c.lqc2(3, sp);
                c.lqc2(4, sp.wrapping_add(0x40));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x50));
                c.lq(2, sp.wrapping_add(0x50));
                c.sq(2, sp.wrapping_add(0x30));
                c.sq(2, sp.wrapping_add(0x20));
                c.lqc2(3, c.g[A2][0] as u32);
                c.lqc2(4, sp.wrapping_add(0x20));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x40));
                c.lq(2, sp.wrapping_add(0x40));
                c.sq(2, sp.wrapping_add(0x30));
                c.lqc2(3, sp.wrapping_add(0x30));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 0);
                c.c_lt_s(0, 3);
                let closer = c.fcc();
                c.lq(2, sp.wrapping_add(0x10));
                pc = if closer { 0x0033_53dc } else { 0x0033_53e4 };
            }
            0x0033_53dc => {
                c.cycles += 2;
                c.mov_s(3, 0);
                c.g[T4][0] = c.g[A1][0];
                pc = 0x0033_53e4;
            }
            // Pass 1 back edge (five samples).
            0x0033_53e4 => {
                c.cycles += 4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = ((c.g[A1][0] as i64) < 5i64) as u64;
                let more = c.g[V1][0] != 0;
                c.sq(2, c.g[SP][0] as u32);
                if more {
                    if crate::events::back_edge_unwinds(c, 0x0033_5268) {
                        return;
                    }
                    pc = 0x0033_5268;
                } else {
                    pc = 0x0033_53f4;
                }
            }
            // Pass 2 setup: pick the best sample's two value pairs out of the
            // stack scratch, rebuild the query frame.
            0x0033_53f4 => {
                c.cycles += 167;
                let sp = c.g[SP][0] as u32;
                let t3 = c.g[T3][0] as u32;
                let t4 = c.g[T4][0] as u32;
                let t6 = c.g[T6][0] as u32;
                c.g[V0][0] = 0x49_0000;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2d68));
                c.g[T7][0] = sx(0x49_0000u32.wrapping_sub(0x1aa0));
                let t7 = 0x48_e560u32;
                c.ldl(3, t7.wrapping_add(0x7));
                c.ldr(3, t7);
                c.ldl(5, t7.wrapping_add(0xf));
                c.ldr(5, t7.wrapping_add(0x8));
                c.g[T5][0] = c.lw(t7.wrapping_add(0x10));
                c.sdl(3, sp.wrapping_add(0x37));
                c.sdr(3, sp.wrapping_add(0x30));
                c.sdl(5, sp.wrapping_add(0x3f));
                c.sdr(5, sp.wrapping_add(0x38));
                c.sw(sp.wrapping_add(0x40), c.g[T5][0]);
                c.g[V1][0] = sx(t4 << 2);
                c.g[V0][0] = 0x49_0000;
                c.g[T7][0] = sx(0x49_0000u32.wrapping_sub(0x1a88));
                let t7b = 0x48_e578u32;
                c.ldl(5, t7b.wrapping_add(0x7));
                c.ldr(5, t7b);
                c.ldl(12, t7b.wrapping_add(0xf));
                c.ldr(12, t7b.wrapping_add(0x8));
                c.g[T5][0] = c.lw(t7b.wrapping_add(0x10));
                c.sdl(5, sp.wrapping_add(0x67));
                c.sdr(5, sp.wrapping_add(0x60));
                c.sdl(12, sp.wrapping_add(0x6f));
                c.sdr(12, sp.wrapping_add(0x68));
                c.sw(sp.wrapping_add(0x70), c.g[T5][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V1][0] = sx(t6.wrapping_add(c.g[V1][0] as u32));
                c.mov_s(15, 0);
                c.lwc1(10, c.g[V1][0] as u32);
                c.g[A1][0] = 0;
                c.lwc1(12, c.g[A0][0] as u32);
                c.mul_s(2, 10, 10);
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 4);
                c.sub_s(1, 12, 10);
                c.swc1(10, sp.wrapping_add(0x88));
                c.swc1(4, sp.wrapping_add(0x8c));
                c.mul_s(5, 12, 12);
                c.mul_s(3, 10, 2);
                c.swc1(2, sp.wrapping_add(0x84));
                c.mul_s(0, 1, 0);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2d64));
                c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x2d60));
                c.mov_s(14, 4);
                c.swc1(3, sp.wrapping_add(0x80));
                c.mul_s(1, 1, 2);
                c.add_s(7, 10, 0);
                c.lqc2(8, sp.wrapping_add(0x80));
                c.lqc2(4, t3);
                c.lqc2(5, t3.wrapping_add(0x10));
                c.lqc2(6, t3.wrapping_add(0x20));
                c.lqc2(7, t3.wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0x90));
                c.mov_s(13, 2);
                c.lq(2, sp.wrapping_add(0x90));
                c.add_s(6, 10, 1);
                c.g[A0][0] = c.g[T3][0];
                c.mul_s(2, 7, 7);
                c.sq(2, sp.wrapping_add(0x50));
                c.sq(2, sp);
                c.mul_s(3, 12, 5);
                c.lqc2(3, sp);
                c.lqc2(4, c.g[A2][0] as u32);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x80));
                c.mul_s(1, 6, 6);
                c.mul_s(0, 7, 2);
                c.lq(2, sp.wrapping_add(0x80));
                c.swc1(4, sp.wrapping_add(0x8c));
                c.mov_s(11, 6);
                c.sq(2, sp.wrapping_add(0x50));
                c.swc1(0, sp.wrapping_add(0x80));
                c.swc1(7, sp.wrapping_add(0x88));
                c.mul_s(0, 6, 1);
                c.swc1(2, sp.wrapping_add(0x84));
                c.lqc2(8, sp.wrapping_add(0x80));
                c.lqc2(4, t3);
                c.lqc2(5, t3.wrapping_add(0x10));
                c.lqc2(6, t3.wrapping_add(0x20));
                c.lqc2(7, t3.wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0x90));
                c.lq(2, sp.wrapping_add(0x90));
                c.sq(2, sp);
                c.sq(2, sp.wrapping_add(0x50));
                c.lqc2(3, sp);
                c.lqc2(4, c.g[A2][0] as u32);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x80));
                c.lq(2, sp.wrapping_add(0x80));
                c.sq(2, sp.wrapping_add(0x50));
                c.lqc2(3, sp.wrapping_add(0x50));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 9);
                c.swc1(1, sp.wrapping_add(0x84));
                c.swc1(0, sp.wrapping_add(0x80));
                c.swc1(4, sp.wrapping_add(0x8c));
                c.swc1(6, sp.wrapping_add(0x88));
                c.lqc2(8, sp.wrapping_add(0x80));
                c.lqc2(4, t3);
                c.lqc2(5, t3.wrapping_add(0x10));
                c.lqc2(6, t3.wrapping_add(0x20));
                c.lqc2(7, t3.wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0x90));
                c.lq(2, sp.wrapping_add(0x90));
                c.sq(2, sp.wrapping_add(0x50));
                c.sq(2, sp);
                c.lqc2(3, sp);
                c.lqc2(4, c.g[A2][0] as u32);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x80));
                c.lq(2, sp.wrapping_add(0x80));
                c.sq(2, sp.wrapping_add(0x50));
                c.lqc2(3, sp.wrapping_add(0x50));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 8);
                c.swc1(3, sp.wrapping_add(0x80));
                c.mov_s(1, 8);
                c.swc1(4, sp.wrapping_add(0x8c));
                c.swc1(12, sp.wrapping_add(0x88));
                c.swc1(5, sp.wrapping_add(0x84));
                c.lqc2(8, sp.wrapping_add(0x80));
                c.lqc2(4, t3);
                c.lqc2(5, t3.wrapping_add(0x10));
                c.lqc2(6, t3.wrapping_add(0x20));
                c.lqc2(7, t3.wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0x90));
                c.lq(2, sp.wrapping_add(0x90));
                c.sq(2, sp.wrapping_add(0x50));
                c.sq(2, sp);
                c.lqc2(3, sp);
                c.lqc2(4, c.g[A2][0] as u32);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x80));
                c.lq(2, sp.wrapping_add(0x80));
                c.sq(2, sp.wrapping_add(0x50));
                pc = 0x0033_5804;
            }
            // Pass 2 body: refine the closest point over 24 samples.
            0x0033_5690 => {
                c.cycles += 4;
                c.c_lt_s(8, 9);
                if !c.fcc() {
                    c.mov_s(12, 6);
                    pc = 0x0033_5754;
                } else {
                    pc = 0x0033_56a0;
                }
            }
            0x0033_56a0 => {
                c.cycles += 45;
                let sp = c.g[SP][0] as u32;
                c.mov_s(10, 7);
                c.mul_s(2, 12, 15);
                c.mov_s(7, 6);
                c.swc1(14, sp.wrapping_add(0x8c));
                c.mul_s(0, 7, 13);
                c.mov_s(9, 8);
                c.add_s(6, 0, 2);
                c.mul_s(1, 6, 6);
                c.swc1(6, sp.wrapping_add(0x88));
                c.mov_s(11, 6);
                c.mul_s(0, 6, 1);
                c.swc1(1, sp.wrapping_add(0x84));
                c.swc1(0, sp.wrapping_add(0x80));
                c.lqc2(8, sp.wrapping_add(0x80));
                c.lqc2(4, c.g[A0][0] as u32);
                c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0x90));
                c.lq(2, sp.wrapping_add(0x90));
                c.sq(2, sp.wrapping_add(0x50));
                c.sq(2, sp);
                c.lqc2(3, sp);
                c.lqc2(4, c.g[A2][0] as u32);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x80));
                c.lq(2, sp.wrapping_add(0x80));
                c.sq(2, sp.wrapping_add(0x50));
                c.lqc2(3, sp.wrapping_add(0x50));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 0);
                c.mov_s(8, 0);
                c.mov_s(1, 8);
                pc = 0x0033_5800;
            }
            0x0033_5754 => {
                c.cycles += 43;
                let sp = c.g[SP][0] as u32;
                c.mul_s(2, 10, 15);
                c.mov_s(6, 7);
                c.swc1(14, sp.wrapping_add(0x8c));
                c.mul_s(0, 6, 13);
                c.mov_s(8, 9);
                c.add_s(7, 0, 2);
                c.mul_s(1, 7, 7);
                c.swc1(7, sp.wrapping_add(0x88));
                c.mov_s(11, 7);
                c.mul_s(0, 7, 1);
                c.swc1(1, sp.wrapping_add(0x84));
                c.swc1(0, sp.wrapping_add(0x80));
                c.lqc2(8, sp.wrapping_add(0x80));
                c.lqc2(4, c.g[A0][0] as u32);
                c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0x90));
                c.lq(2, sp.wrapping_add(0x90));
                c.sq(2, sp.wrapping_add(0x50));
                c.sq(2, sp);
                c.lqc2(3, sp);
                c.lqc2(4, c.g[A2][0] as u32);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x80));
                c.lq(2, sp.wrapping_add(0x80));
                c.sq(2, sp.wrapping_add(0x50));
                c.lqc2(3, sp.wrapping_add(0x50));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 0);
                c.mov_s(9, 0);
                c.mov_s(1, 9);
                pc = 0x0033_5800;
            }
            0x0033_5800 => {
                c.cycles += 1;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                pc = 0x0033_5804;
            }
            0x0033_5804 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < 24i64) as u64;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
                    pc = 0x0033_5828;
                } else {
                    pc = 0x0033_5810;
                }
            }
            0x0033_5810 => {
                c.cycles += 5;
                c.sub_s(0, 12, 10);
                c.abs_s(0, 0);
                c.c_le_s(0, 16);
                let keep = !c.fcc();
                if keep {
                    if crate::events::back_edge_unwinds(c, 0x0033_5690) {
                        return;
                    }
                    pc = 0x0033_5690;
                } else {
                    pc = 0x0033_5824;
                }
            }
            0x0033_5824 => {
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
                pc = 0x0033_5828;
            }
            0x0033_5828 => {
                c.cycles += 2;
                let no_hit = c.g[V0][0] == 0;
                c.g[V0][0] = 1;
                pc = if no_hit { 0x0033_5840 } else { 0x0033_5830 };
            }
            0x0033_5830 => {
                c.cycles += 4;
                c.lwc1(0, c.g[T0][0] as u32);
                c.c_lt_s(1, 0);
                let keep = !c.fcc();
                pc = if keep { 0x0033_5954 } else { 0x0033_5840 };
            }
            // Success: write the result flag, the frame and the rail output.
            0x0033_5840 => {
                c.cycles += 69;
                let sp = c.g[SP][0] as u32;
                let t3 = c.g[T3][0] as u32;
                c.g[A1][0] = u64::MAX;
                c.sw(c.g[A3][0] as u32, c.g[V0][0]);
                c.g[V1][0] = 0x2;
                c.swc1(1, c.g[T0][0] as u32);
                c.g[A0][0] = 0x50_0000;
                c.sw((c.g[T1][0] as u32).wrapping_add(0x44), c.g[V1][0]);
                c.g[A0][0] = sx(0x50_0000u32.wrapping_sub(0xee0));
                c.sw((c.g[T1][0] as u32).wrapping_add(0x54), 0);
                c.add_s(2, 11, 11);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x50), 0);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x58), 0);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x5c), c.g[A1][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x60), c.g[A1][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x64), c.g[A1][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x4c), c.g[A1][0]);
                c.g[AT][0] = 0x4040_0000;
                c.mtc1(1, 0);
                c.lq(2, c.g[A0][0] as u32);
                c.lq(3, sp);
                c.mul_s(0, 11, 0);
                c.sq(2, (c.g[T1][0] as u32).wrapping_add(0x20));
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 1);
                c.lq(2, c.g[A0][0] as u32);
                c.mul_s(0, 11, 0);
                c.sq(3, c.g[T1][0] as u32);
                c.sq(2, (c.g[T1][0] as u32).wrapping_add(0x30));
                c.swc1(1, sp.wrapping_add(0xa8));
                c.swc1(2, sp.wrapping_add(0xa4));
                c.swc1(0, sp.wrapping_add(0xa0));
                c.sw((c.g[T1][0] as u32).wrapping_add(0x68), 0);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x6c), 0);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x70), 0);
                c.sw(sp.wrapping_add(0xac), 0);
                c.lqc2(8, sp.wrapping_add(0xa0));
                c.lqc2(4, t3);
                c.lqc2(5, t3.wrapping_add(0x10));
                c.lqc2(6, t3.wrapping_add(0x20));
                c.lqc2(7, t3.wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0xb0));
                c.lq(2, sp.wrapping_add(0xb0));
                c.sq(2, sp.wrapping_add(0x80));
                c.lqc2(3, sp.wrapping_add(0x80));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, sp.wrapping_add(0x90));
                c.lq(2, sp.wrapping_add(0x90));
                c.sq(2, sp.wrapping_add(0x50));
                c.sq(2, (c.g[T1][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = Addr((c.g[T2][0] as u32).wrapping_add(0x68)).lw(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lw(c);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x50), 0);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x5c), c.g[A1][0]);
                c.g[V0][0] = Addr((c.g[T2][0] as u32).wrapping_add(0x68)).lw(c);
                c.swc1(11, (c.g[T1][0] as u32).wrapping_add(0x68));
                c.sw((c.g[T1][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x64), c.g[A1][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x54), 0);
                pc = 0x0033_5954;
            }
            // Return.
            0x0033_5954 => {
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x0033_5128, 0x0033_595c);
                return;
            }
        }
    }
}


/// The shared tail of the two animation-node constructors: construct the
/// embedded base object at_2 `self + base`, install the class v-table, run the
/// stream's node constructor for the base and set the object's flag bit.
pub(crate) fn anim_node_ctor(
    c: &mut Cpu,
    base: u32,
    class_vt: u32,
    vtable_arg: u32,
    flags_off: u32,
    ret_ctor: u32,
    ret_arg: u32,
) -> Flow {
    c.cycles += 27;
    let frame = Frame::open(c, 0x30);
    let self_ = c.g[A0][0] as u32;
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = sx(self_);
    c.g[S1][0] = c.g[A2][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = sx(self_.wrapping_add(base));
    call_ptr(c, 0x0034_dac8, ret_ctor)?;

    c.g[V1][0] = sx(class_vt);
    c.g[A1][0] = sx(self_);
    c.g[A2][0] = sx(vtable_arg);
    Addr(self_.wrapping_add(base).wrapping_add(0x20)).sw(c, c.g[V1][0]);
    c.g[V0][0] = Addr(at_2(c, S1, 0)).lw(c);
    c.g[A0][0] = Addr(at_2(c, V0, 0x10)).lh(c);
    c.g[A3][0] = Addr(at_2(c, V0, 0x14)).lw(c);
    let t = c.g[A3][0] as u32;
    c.g[A0][0] = sx(at_2(c, S1, 0).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret_arg)?;

    c.g[V1][0] = Addr(at_2(c, S0, flags_off)).lhu(c);
    c.g[V0][0] = c.g[S0][0];
    frame.restore(c, S1, 0x10);
    c.g[V1][0] = c.g[V1][0] | 1;
    frame.restore_ra(c, 0);
    Addr(at_2(c, S0, flags_off)).sh(c, c.g[V1][0]);
    frame.restore(c, S0, 0x20);
    frame.close(c);
    Ok(())
}


/// `void* AnimObject_ctor_2(AnimObject* self, void* arg, Stream* stream)`
/// (0x341c80): construct the base node at_2 +0x1c, install the class v-table,
/// let the stream build the node and set flag bit 0.
pub fn anim_object_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        anim_node_ctor(c, 0x1c, 0x490b_10, 0x1c, 0x42, 0x0034_1ca0, 0x0034_1cc8)
    })
}


/// `void* AnimTeeter_ctor_2(AnimTeeter* self, void* arg, Stream* stream)`
/// (0x3422e8): same for the teeter object at_2 +0x30.
pub fn anim_teeter_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        anim_node_ctor(c, 0x30, 0x4908_f8, 0x30, 0x56, 0x0034_2308, 0x0034_2330)
    })
}


/// `void Cell_CollideVolumeRecursive(void* unused, CollideCtx* q,
/// CollideCtx* volume_q, const float* box, TreeNode* tree, const Cell* cell)`
/// (0x0033ccf8): collide the objects of one octree node against the volume,
/// then, when the node's cell is not at the deepest level (11), classify the
/// eight child cells against the query box and either collect a child's whole
/// subtree into the volume query (`volume_q`) when the child is fully inside,
/// or recurse into it when it is only partly inside. Children outside the box
/// are skipped.
///
/// A tree node is `{ TreeNode* child[8]; void** nodes; void** objects; }`
/// (`nodes` at +0x20, whose entries go to `Cell_CollideObject`, and `objects`
/// at +0x24, whose entries go to `Cell_CollideInstance`), the cell is
/// `{ int level, x, y, z; }` and the box is `{ float min[4]; float max[4]; }`.
/// The child cell is built in this frame at `sp+0` (`level-1`, `2x`, `2y`,
/// `2z`) and adjusted one octant at a time.
pub fn cell_collide_volume_recursive(c: &mut Cpu) {
    guest(c, |c| {
        'b_33cd5c: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S1][0] = c.g[T1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S2][0] = c.g[A2][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S3][0] = c.g[A3][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S4][0] = c.g[A1][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S5][0] = c.g[T0][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.g[S0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x24));
            let t = c.g[S0][0] == 0u64;
            c.g[S6][0] = c.g[A0][0];
            if t {
                break 'b_33cd5c;
            } else {
                c.cycles += 1;
                c.g[A1][0] = c.g[S0][0];
                'l_33cd48: loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S4][0];
                    COLLIDE_INSTANCE.call(c, 0x0033cd50)?;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32));
                    let t = c.g[S0][0] != 0u64;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0033cd48);
                        }
                        continue 'l_33cd48;
                    } else {
                        break 'b_33cd5c;
                    }
                }
            }
        }
        'b_33cd88: {
            c.cycles += 3;
            c.g[S0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x20));
            if c.g[S0][0] == 0u64 {
                c.g[A1][0] = c.lw((c.g[S1][0] as u32));
                break 'b_33cd88;
            } else {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                'l_33cd70: loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S4][0];
                    COLLIDE_OBJECT.call(c, 0x0033cd78)?;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32));
                    let t = c.g[S0][0] != 0u64;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0033cd70);
                        }
                        continue 'l_33cd70;
                    } else {
                        c.cycles += 1;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
                        break 'b_33cd88;
                    }
                }
            }
        }
        // Classify the eight child cells against the query box; then visit
        // each child in z-order: collect it (`Cell_CollideVolumeSubtree`),
        // recurse into it, or skip it when it misses the box.
        'b_33dbbc: {
            'b_33dbb8: {
                c.cycles += 3;
                c.g[V0][0] = 0xb;
                let t = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e));
                if t {
                    break 'b_33dbb8;
                } else {
                    'b_33ce18: {
                        c.cycles += 28;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 23);
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2cf8));
                        c.mtc1(2, 4);
                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(4, 0);
                        c.cvt_s_w(0, 0);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.lwc1(3, (c.g[S3][0] as u32));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.sub_s(0, 0, 2);
                        c.add_s(1, 1, 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                        c.sw((c.g[SP][0] as u32), c.g[A1][0]);
                        c.g[FP][0] = 0u64;
                        c.mul_s(15, 0, 4);
                        c.mul_s(16, 1, 4);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                        c.c_lt_s(3, 16);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                        if t {
                            break 'b_33ce18;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(15, 0);
                            if c.fcc() {
                                c.g[FP][0] = 0x1;
                                break 'b_33ce18;
                            } else {
                                break 'b_33ce18;
                            }
                        }
                    }
                    'b_33ce78: {
                        c.cycles += 18;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2cf4));
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, (c.g[S3][0] as u32));
                        c.add_s(1, 1, 3);
                        c.mov_s(13, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(14, 1, 4);
                        c.c_lt_s(2, 14);
                        let t = !c.fcc();
                        c.mul_s(12, 0, 4);
                        if t {
                            break 'b_33ce78;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(12, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33ce78;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                break 'b_33ce78;
                            }
                        }
                    }
                    'b_33ce8c: {
                        c.cycles += 2;
                        let t = c.g[FP][0] != 0u64;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_33ce8c;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                break 'b_33dbbc;
                            } else {
                                break 'b_33ce8c;
                            }
                        }
                    }
                    'b_33cee8: {
                        c.cycles += 17;
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2cf0));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.g[A0][0] = c.g[V1][0];
                        c.mtc1(3, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, (c.g[S3][0] as u32).wrapping_add(0x4));
                        c.add_s(1, 1, 3);
                        c.mov_s(6, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(11, 1, 4);
                        c.c_lt_s(2, 11);
                        let t = !c.fcc();
                        c.mul_s(10, 0, 4);
                        if t {
                            break 'b_33cee8;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(10, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33cee8;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                                break 'b_33cee8;
                            }
                        }
                    }
                    'b_33cf3c: {
                        c.cycles += 15;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2cec));
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                        c.add_s(1, 1, 2);
                        c.sub_s(0, 0, 2);
                        c.mul_s(9, 1, 4);
                        c.c_lt_s(6, 9);
                        let t = !c.fcc();
                        c.mul_s(8, 0, 4);
                        if t {
                            break 'b_33cf3c;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(8, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33cf3c;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                break 'b_33cf3c;
                            }
                        }
                    }
                    'b_33cf54: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        if t {
                            break 'b_33cf54;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                break 'b_33dbbc;
                            } else {
                                break 'b_33cf54;
                            }
                        }
                    }
                    'b_33cfb0: {
                        c.cycles += 17;
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2ce8));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.g[A0][0] = c.g[V1][0];
                        c.mtc1(3, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, (c.g[S3][0] as u32).wrapping_add(0x8));
                        c.add_s(1, 1, 3);
                        c.mov_s(5, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(7, 1, 4);
                        c.c_lt_s(2, 7);
                        let t = !c.fcc();
                        c.mul_s(3, 0, 4);
                        if t {
                            break 'b_33cfb0;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(3, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33cfb0;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                                break 'b_33cfb0;
                            }
                        }
                    }
                    'b_33d004: {
                        c.cycles += 15;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2ce4));
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                        c.add_s(1, 1, 2);
                        c.sub_s(0, 0, 2);
                        c.mul_s(1, 1, 4);
                        c.c_lt_s(5, 1);
                        let t = !c.fcc();
                        c.mul_s(2, 0, 4);
                        if t {
                            break 'b_33d004;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(2, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33d004;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                                break 'b_33d004;
                            }
                        }
                    }
                    'b_33d018: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_33d018;
                        } else {
                            c.cycles += 2;
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                break 'b_33dbbc;
                            } else {
                                break 'b_33d018;
                            }
                        }
                    }
                    'b_33d03c: {
                        c.cycles += 4;
                        c.c_lt_s(13, 15);
                        let t = !c.fcc();
                        c.g[S7][0] = 0u64;
                        if t {
                            break 'b_33d03c;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(16, 0);
                            if c.fcc() {
                                c.g[S7][0] = 0x1;
                                break 'b_33d03c;
                            } else {
                                break 'b_33d03c;
                            }
                        }
                    }
                    'b_33d064: {
                        c.cycles += 4;
                        c.c_lt_s(13, 12);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                        if t {
                            break 'b_33d064;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(14, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33d064;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                                break 'b_33d064;
                            }
                        }
                    }
                    'b_33d08c: {
                        c.cycles += 4;
                        c.c_lt_s(6, 10);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        if t {
                            break 'b_33d08c;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(11, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33d08c;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                                break 'b_33d08c;
                            }
                        }
                    }
                    'b_33d0b4: {
                        c.cycles += 4;
                        c.c_lt_s(6, 8);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                        if t {
                            break 'b_33d0b4;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(9, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33d0b4;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                                break 'b_33d0b4;
                            }
                        }
                    }
                    'b_33d0dc: {
                        c.cycles += 4;
                        c.c_lt_s(5, 3);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        if t {
                            break 'b_33d0dc;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(7, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33d0dc;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                                break 'b_33d0dc;
                            }
                        }
                    }
                    'b_33d104: {
                        c.cycles += 4;
                        c.c_lt_s(5, 2);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
                        if t {
                            break 'b_33d104;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(1, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33d104;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                                break 'b_33d104;
                            }
                        }
                    }
                    'b_33d254: {
                        c.cycles += 3;
                        c.g[S1][0] = c.lw((c.g[S5][0] as u32));
                        let t = c.g[S1][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        if t {
                            break 'b_33d254;
                        } else {
                            'b_33d250: {
                                c.cycles += 2;
                                let t = c.g[FP][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_33d250;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_33d250;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_33d254;
                                        } else {
                                            'b_33d238: {
                                                'b_33d234: {
                                                    c.cycles += 2;
                                                    let t = c.g[S7][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                    if t {
                                                        break 'b_33d234;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x30));
                                                        if t {
                                                            break 'b_33d234;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[T0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_33d238;
                                                            } else {
                                                                'b_33d168: {
                                                                    c.cycles += 3;
                                                                    c.g[S0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_33d168;
                                                                    } else {
                                                                        c.cycles += 1;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d150: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_INSTANCE.call(
                                                                                c, 0x0033d158,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d150);
                                                                                }
                                                                                continue 'l_33d150;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                                break 'b_33d168;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d190: {
                                                                    c.cycles += 2;
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        break 'b_33d190;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d178: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_OBJECT.call(
                                                                                c, 0x0033d180,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d178);
                                                                                }
                                                                                continue 'l_33d178;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[S1][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33d190;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d1a4: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d1a4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d1a0)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d1a4;
                                                                    }
                                                                }
                                                                'b_33d1b8: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d1b8;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d1b4)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d1b8;
                                                                    }
                                                                }
                                                                'b_33d1cc: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d1cc;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d1c8)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d1cc;
                                                                    }
                                                                }
                                                                'b_33d1e0: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d1e0;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d1dc)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d1e0;
                                                                    }
                                                                }
                                                                'b_33d1f4: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d1f4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d1f0)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d1f4;
                                                                    }
                                                                }
                                                                'b_33d208: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d208;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d204)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d208;
                                                                    }
                                                                }
                                                                'b_33d21c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d21c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d218)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d21c;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33d254;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    COLLIDE_VOLUME_SUBTREE
                                                                        .call(c, 0x0033d22c)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    if t {
                                                                        break 'b_33d254;
                                                                    } else {
                                                                        break 'b_33d234;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[T0][0] = c.g[S1][0];
                                                break 'b_33d238;
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[A3][0] = c.g[S3][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            COLLIDE_VOLUME_RECURSIVE.call(c, 0x0033d250)?;
                                            break 'b_33d250;
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                            break 'b_33d254;
                        }
                    }
                    'b_33d3ac: {
                        'b_33d3a8: {
                            c.cycles += 4;
                            c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S1][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33d3a8;
                            } else {
                                c.cycles += 2;
                                let t = c.g[FP][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_33d3a8;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_33d3a8;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_33d3ac;
                                        } else {
                                            'b_33d390: {
                                                'b_33d38c: {
                                                    c.cycles += 2;
                                                    let t = c.g[S7][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                    if t {
                                                        break 'b_33d38c;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x34));
                                                        if t {
                                                            break 'b_33d38c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[T0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_33d390;
                                                            } else {
                                                                'b_33d2c0: {
                                                                    c.cycles += 3;
                                                                    c.g[S0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_33d2c0;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d2a8: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_INSTANCE.call(
                                                                                c, 0x0033d2b0,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d2a8);
                                                                                }
                                                                                continue 'l_33d2a8;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                                break 'b_33d2c0;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d2e8: {
                                                                    c.cycles += 2;
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        break 'b_33d2e8;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d2d0: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_OBJECT.call(
                                                                                c, 0x0033d2d8,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d2d0);
                                                                                }
                                                                                continue 'l_33d2d0;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[S1][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33d2e8;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d2fc: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d2fc;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d2f8)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d2fc;
                                                                    }
                                                                }
                                                                'b_33d310: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d310;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d30c)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d310;
                                                                    }
                                                                }
                                                                'b_33d324: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d324;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d320)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d324;
                                                                    }
                                                                }
                                                                'b_33d338: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d338;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d334)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d338;
                                                                    }
                                                                }
                                                                'b_33d34c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d34c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d348)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d34c;
                                                                    }
                                                                }
                                                                'b_33d360: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d360;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d35c)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d360;
                                                                    }
                                                                }
                                                                'b_33d374: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d374;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d370)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d374;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                if t {
                                                                    break 'b_33d3ac;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    COLLIDE_VOLUME_SUBTREE
                                                                        .call(c, 0x0033d384)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    if t {
                                                                        break 'b_33d3ac;
                                                                    } else {
                                                                        break 'b_33d38c;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[T0][0] = c.g[S1][0];
                                                break 'b_33d390;
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[A3][0] = c.g[S3][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            COLLIDE_VOLUME_RECURSIVE.call(c, 0x0033d3a8)?;
                                            break 'b_33d3a8;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        break 'b_33d3ac;
                    }
                    'b_33d504: {
                        'b_33d500: {
                            c.cycles += 4;
                            c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S1][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            if t {
                                break 'b_33d500;
                            } else {
                                c.cycles += 2;
                                let t = c.g[FP][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                if t {
                                    break 'b_33d500;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_33d500;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_33d504;
                                        } else {
                                            'b_33d4e8: {
                                                'b_33d4e4: {
                                                    c.cycles += 2;
                                                    let t = c.g[S7][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_33d4e4;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x34));
                                                        if t {
                                                            break 'b_33d4e4;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[T0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_33d4e8;
                                                            } else {
                                                                'b_33d418: {
                                                                    c.cycles += 3;
                                                                    c.g[S0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_33d418;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d400: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_INSTANCE.call(
                                                                                c, 0x0033d408,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d400);
                                                                                }
                                                                                continue 'l_33d400;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                                break 'b_33d418;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d440: {
                                                                    c.cycles += 2;
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        break 'b_33d440;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d428: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_OBJECT.call(
                                                                                c, 0x0033d430,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d428);
                                                                                }
                                                                                continue 'l_33d428;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[S1][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33d440;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d454: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d454;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d450)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d454;
                                                                    }
                                                                }
                                                                'b_33d468: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d468;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d464)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d468;
                                                                    }
                                                                }
                                                                'b_33d47c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d47c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d478)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d47c;
                                                                    }
                                                                }
                                                                'b_33d490: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d490;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d48c)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d490;
                                                                    }
                                                                }
                                                                'b_33d4a4: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d4a4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d4a0)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d4a4;
                                                                    }
                                                                }
                                                                'b_33d4b8: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d4b8;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d4b4)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d4b8;
                                                                    }
                                                                }
                                                                'b_33d4cc: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d4cc;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d4c8)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d4cc;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33d504;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    COLLIDE_VOLUME_SUBTREE
                                                                        .call(c, 0x0033d4dc)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    if t {
                                                                        break 'b_33d504;
                                                                    } else {
                                                                        break 'b_33d4e4;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[T0][0] = c.g[S1][0];
                                                break 'b_33d4e8;
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[A3][0] = c.g[S3][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            COLLIDE_VOLUME_RECURSIVE.call(c, 0x0033d500)?;
                                            break 'b_33d500;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_33d504;
                    }
                    'b_33d65c: {
                        'b_33d658: {
                            c.cycles += 4;
                            c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S1][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33d658;
                            } else {
                                c.cycles += 2;
                                let t = c.g[FP][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                if t {
                                    break 'b_33d658;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_33d658;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_33d65c;
                                        } else {
                                            'b_33d640: {
                                                'b_33d63c: {
                                                    c.cycles += 2;
                                                    let t = c.g[S7][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_33d63c;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x30));
                                                        if t {
                                                            break 'b_33d63c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[T0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_33d640;
                                                            } else {
                                                                'b_33d570: {
                                                                    c.cycles += 3;
                                                                    c.g[S0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_33d570;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d558: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_INSTANCE.call(
                                                                                c, 0x0033d560,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d558);
                                                                                }
                                                                                continue 'l_33d558;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                                break 'b_33d570;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d598: {
                                                                    c.cycles += 2;
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        break 'b_33d598;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d580: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_OBJECT.call(
                                                                                c, 0x0033d588,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d580);
                                                                                }
                                                                                continue 'l_33d580;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[S1][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33d598;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d5ac: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d5ac;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d5a8)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d5ac;
                                                                    }
                                                                }
                                                                'b_33d5c0: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d5c0;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d5bc)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d5c0;
                                                                    }
                                                                }
                                                                'b_33d5d4: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d5d4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d5d0)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d5d4;
                                                                    }
                                                                }
                                                                'b_33d5e8: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d5e8;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d5e4)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d5e8;
                                                                    }
                                                                }
                                                                'b_33d5fc: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d5fc;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d5f8)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d5fc;
                                                                    }
                                                                }
                                                                'b_33d610: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d610;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d60c)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d610;
                                                                    }
                                                                }
                                                                'b_33d624: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d624;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d620)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d624;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                if t {
                                                                    break 'b_33d65c;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    COLLIDE_VOLUME_SUBTREE
                                                                        .call(c, 0x0033d634)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    if t {
                                                                        break 'b_33d65c;
                                                                    } else {
                                                                        break 'b_33d63c;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[T0][0] = c.g[S1][0];
                                                break 'b_33d640;
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[A3][0] = c.g[S3][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            COLLIDE_VOLUME_RECURSIVE.call(c, 0x0033d658)?;
                                            break 'b_33d658;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        break 'b_33d65c;
                    }
                    'b_33d7b4: {
                        'b_33d7b0: {
                            c.cycles += 4;
                            c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x18));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S1][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            if t {
                                break 'b_33d7b0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                if t {
                                    break 'b_33d7b0;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_33d7b0;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                        if t {
                                            break 'b_33d7b0;
                                        } else {
                                            'b_33d798: {
                                                'b_33d794: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_33d794;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x30));
                                                        if t {
                                                            break 'b_33d794;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[T0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_33d798;
                                                            } else {
                                                                'b_33d6c8: {
                                                                    c.cycles += 3;
                                                                    c.g[S0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_33d6c8;
                                                                    } else {
                                                                        c.cycles += 1;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d6b0: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_INSTANCE.call(
                                                                                c, 0x0033d6b8,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d6b0);
                                                                                }
                                                                                continue 'l_33d6b0;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                                break 'b_33d6c8;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d6f0: {
                                                                    c.cycles += 2;
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        break 'b_33d6f0;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d6d8: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_OBJECT.call(
                                                                                c, 0x0033d6e0,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d6d8);
                                                                                }
                                                                                continue 'l_33d6d8;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[S1][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33d6f0;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d704: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d704;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d700)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d704;
                                                                    }
                                                                }
                                                                'b_33d718: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d718;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d714)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d718;
                                                                    }
                                                                }
                                                                'b_33d72c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d72c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d728)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d72c;
                                                                    }
                                                                }
                                                                'b_33d740: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d740;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d73c)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d740;
                                                                    }
                                                                }
                                                                'b_33d754: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d754;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d750)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d754;
                                                                    }
                                                                }
                                                                'b_33d768: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d768;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d764)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d768;
                                                                    }
                                                                }
                                                                'b_33d77c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d77c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d778)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d77c;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33d7b4;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    COLLIDE_VOLUME_SUBTREE
                                                                        .call(c, 0x0033d78c)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    if t {
                                                                        break 'b_33d7b4;
                                                                    } else {
                                                                        break 'b_33d794;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[T0][0] = c.g[S1][0];
                                                break 'b_33d798;
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[A3][0] = c.g[S3][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            COLLIDE_VOLUME_RECURSIVE.call(c, 0x0033d7b0)?;
                                            break 'b_33d7b0;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_33d7b4;
                    }
                    'b_33d90c: {
                        'b_33d908: {
                            c.cycles += 4;
                            c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S1][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33d908;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                if t {
                                    break 'b_33d908;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_33d908;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                        if t {
                                            break 'b_33d908;
                                        } else {
                                            'b_33d8f0: {
                                                'b_33d8ec: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_33d8ec;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x34));
                                                        if t {
                                                            break 'b_33d8ec;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[T0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_33d8f0;
                                                            } else {
                                                                'b_33d820: {
                                                                    c.cycles += 3;
                                                                    c.g[S0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_33d820;
                                                                    } else {
                                                                        c.cycles += 1;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d808: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_INSTANCE.call(
                                                                                c, 0x0033d810,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d808);
                                                                                }
                                                                                continue 'l_33d808;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                                break 'b_33d820;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d848: {
                                                                    c.cycles += 2;
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        break 'b_33d848;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d830: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_OBJECT.call(
                                                                                c, 0x0033d838,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d830);
                                                                                }
                                                                                continue 'l_33d830;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[S1][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33d848;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d85c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d85c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d858)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d85c;
                                                                    }
                                                                }
                                                                'b_33d870: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d870;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d86c)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d870;
                                                                    }
                                                                }
                                                                'b_33d884: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d884;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d880)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d884;
                                                                    }
                                                                }
                                                                'b_33d898: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d898;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d894)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d898;
                                                                    }
                                                                }
                                                                'b_33d8ac: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d8ac;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d8a8)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33d8ac;
                                                                    }
                                                                }
                                                                'b_33d8c0: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d8c0;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d8bc)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33d8c0;
                                                                    }
                                                                }
                                                                'b_33d8d4: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d8d4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d8d0)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33d8d4;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                if t {
                                                                    break 'b_33d90c;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    COLLIDE_VOLUME_SUBTREE
                                                                        .call(c, 0x0033d8e4)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    if t {
                                                                        break 'b_33d90c;
                                                                    } else {
                                                                        break 'b_33d8ec;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[T0][0] = c.g[S1][0];
                                                break 'b_33d8f0;
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[A3][0] = c.g[S3][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            COLLIDE_VOLUME_RECURSIVE.call(c, 0x0033d908)?;
                                            break 'b_33d908;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        break 'b_33d90c;
                    }
                    'b_33da64: {
                        'b_33da60: {
                            c.cycles += 4;
                            c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S1][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            if t {
                                break 'b_33da60;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_33da60;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_33da60;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                        if t {
                                            break 'b_33da60;
                                        } else {
                                            'b_33da48: {
                                                'b_33da44: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                    if t {
                                                        break 'b_33da44;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x34));
                                                        if t {
                                                            break 'b_33da44;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[T0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_33da48;
                                                            } else {
                                                                'b_33d978: {
                                                                    c.cycles += 3;
                                                                    c.g[S0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_33d978;
                                                                    } else {
                                                                        c.cycles += 1;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d960: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_INSTANCE.call(
                                                                                c, 0x0033d968,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d960);
                                                                                }
                                                                                continue 'l_33d960;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                                break 'b_33d978;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d9a0: {
                                                                    c.cycles += 2;
                                                                    if c.g[S0][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        break 'b_33d9a0;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        'l_33d988: loop {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            COLLIDE_OBJECT.call(
                                                                                c, 0x0033d990,
                                                                            )?;
                                                                            c.g[S0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[S0][0] != 0u64;
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033d988);
                                                                                }
                                                                                continue 'l_33d988;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[S1][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33d9a0;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33d9b4: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d9b4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d9b0)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33d9b4;
                                                                    }
                                                                }
                                                                'b_33d9c8: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d9c8;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d9c4)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33d9c8;
                                                                    }
                                                                }
                                                                'b_33d9dc: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d9dc;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d9d8)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33d9dc;
                                                                    }
                                                                }
                                                                'b_33d9f0: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d9f0;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033d9ec)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33d9f0;
                                                                    }
                                                                }
                                                                'b_33da04: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33da04;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033da00)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33da04;
                                                                    }
                                                                }
                                                                'b_33da18: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33da18;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033da14)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33da18;
                                                                    }
                                                                }
                                                                'b_33da2c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33da2c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                        COLLIDE_VOLUME_SUBTREE
                                                                            .call(c, 0x0033da28)?;
                                                                        c.g[A0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33da2c;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33da64;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    COLLIDE_VOLUME_SUBTREE
                                                                        .call(c, 0x0033da3c)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    if t {
                                                                        break 'b_33da64;
                                                                    } else {
                                                                        break 'b_33da44;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[T0][0] = c.g[S1][0];
                                                break 'b_33da48;
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[A3][0] = c.g[S3][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            COLLIDE_VOLUME_RECURSIVE.call(c, 0x0033da60)?;
                                            break 'b_33da60;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_33da64;
                    }
                    c.cycles += 4;
                    c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[S1][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    if t {
                        break 'b_33dbb8;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        if t {
                            break 'b_33dbb8;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                            if t {
                                break 'b_33dbb8;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                if t {
                                    break 'b_33dbb8;
                                } else {
                                    'b_33dba0: {
                                        'b_33db9c: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                            if t {
                                                break 'b_33db9c;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_33db9c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A0][0] = c.g[S6][0];
                                                    if t {
                                                        break 'b_33dba0;
                                                    } else {
                                                        'b_33dad0: {
                                                            c.cycles += 3;
                                                            c.g[S0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x24));
                                                            if c.g[S0][0] == 0u64 {
                                                                c.g[S0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x20));
                                                                break 'b_33dad0;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                'l_33dab8: loop {
                                                                    c.cycles += 5;
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    COLLIDE_INSTANCE
                                                                        .call(c, 0x0033dac0)?;
                                                                    c.g[S0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    let t = c.g[S0][0] != 0u64;
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x0033dab8,
                                                                            );
                                                                        }
                                                                        continue 'l_33dab8;
                                                                    } else {
                                                                        c.cycles += 1;
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_33dad0;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        'b_33daf8: {
                                                            c.cycles += 2;
                                                            if c.g[S0][0] == 0u64 {
                                                                c.g[A0][0] =
                                                                    c.lw((c.g[S1][0] as u32));
                                                                break 'b_33daf8;
                                                            } else {
                                                                c.cycles += 2;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                'l_33dae0: loop {
                                                                    c.cycles += 5;
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    COLLIDE_OBJECT
                                                                        .call(c, 0x0033dae8)?;
                                                                    c.g[S0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    let t = c.g[S0][0] != 0u64;
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x0033dae0,
                                                                            );
                                                                        }
                                                                        continue 'l_33dae0;
                                                                    } else {
                                                                        c.cycles += 1;
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        break 'b_33daf8;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        'b_33db0c: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                break 'b_33db0c;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                COLLIDE_VOLUME_SUBTREE
                                                                    .call(c, 0x0033db08)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                break 'b_33db0c;
                                                            }
                                                        }
                                                        'b_33db20: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                break 'b_33db20;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                COLLIDE_VOLUME_SUBTREE
                                                                    .call(c, 0x0033db1c)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                break 'b_33db20;
                                                            }
                                                        }
                                                        'b_33db34: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                break 'b_33db34;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                COLLIDE_VOLUME_SUBTREE
                                                                    .call(c, 0x0033db30)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                break 'b_33db34;
                                                            }
                                                        }
                                                        'b_33db48: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x10));
                                                                break 'b_33db48;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                COLLIDE_VOLUME_SUBTREE
                                                                    .call(c, 0x0033db44)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x10));
                                                                break 'b_33db48;
                                                            }
                                                        }
                                                        'b_33db5c: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x14));
                                                                break 'b_33db5c;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                COLLIDE_VOLUME_SUBTREE
                                                                    .call(c, 0x0033db58)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x14));
                                                                break 'b_33db5c;
                                                            }
                                                        }
                                                        'b_33db70: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x18));
                                                                break 'b_33db70;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                COLLIDE_VOLUME_SUBTREE
                                                                    .call(c, 0x0033db6c)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x18));
                                                                break 'b_33db70;
                                                            }
                                                        }
                                                        'b_33db84: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x1c));
                                                                break 'b_33db84;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                COLLIDE_VOLUME_SUBTREE
                                                                    .call(c, 0x0033db80)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x1c));
                                                                break 'b_33db84;
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        let t = c.g[A0][0] == 0u64;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        if t {
                                                            break 'b_33dbbc;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[A1][0] = c.g[S2][0];
                                                            COLLIDE_VOLUME_SUBTREE
                                                                .call(c, 0x0033db94)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xd0),
                                                            );
                                                            if t {
                                                                break 'b_33dbbc;
                                                            } else {
                                                                break 'b_33db9c;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A0][0] = c.g[S6][0];
                                        break 'b_33dba0;
                                    }
                                    c.cycles += 6;
                                    c.g[A1][0] = c.g[S4][0];
                                    c.g[A2][0] = c.g[S2][0];
                                    c.g[A3][0] = c.g[S3][0];
                                    c.g[T0][0] = c.g[S1][0];
                                    c.g[T1][0] = c.g[SP][0];
                                    COLLIDE_VOLUME_RECURSIVE.call(c, 0x0033dbb8)?;
                                    break 'b_33dbb8;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
            break 'b_33dbbc;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
        abi::ret(c);
        Ok(())
    })
}


/// `void Cell_GatherAllRecursive(void* unused, GatherCtx* q,
/// GatherCtx* collect_q, const float* box, TreeNode* tree, const Cell* cell)`
/// (0x0033b748): gather one octree node of the world sphere tree, then (unless
/// the cell is at the deepest level, 11) classify its eight child cells
/// against `box`. A child wholly inside the box has its whole subtree
/// gathered into `collect_q` (`Cell_GatherAllSubtree`, inlined by the
/// compiler for every octant), a partly-inside child is recursed into, and a
/// child outside the box is skipped. `q` gathers the objects of the nodes on
/// the descent path.
///
/// A tree node is `{ TreeNode* child[8]; void** nodes; void** objects;
/// void** all; }` (`nodes` at +0x20, `objects` at +0x24, `all` at +0x28); a
/// gather context is three `{ void** list; int* count; int pad; float* box; }`
/// sub-structs at +0 (`nodes`), +0x10 (`objects`) and +0x20 (`all`).
pub fn cell_gather_all_recursive(c: &mut Cpu) {
    guest(c, |c| {
        'b_33b85c: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S3][0] = c.g[T0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S4][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.g[S5][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[S0][0] = c.g[T1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S1][0] = c.g[A2][0];
            c.g[S2][0] = c.g[A3][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[A0][0] = c.g[S4][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
            c.g[A1][0] = c.g[S3][0];
            GATHER_OBJECTS.call(c, 0x0033b798)?;
            c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x24));
            let t = c.g[A2][0] == 0u64;
            c.g[A3][0] = sx((c.g[S4][0] as u32).wrapping_add(0x10));
            if t {
                break 'b_33b85c;
            } else {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                'l_33b7a8: loop {
                    'b_33b828: {
                        c.cycles += 6;
                        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x164));
                        c.lwc1(0, (c.g[V0][0] as u32));
                        c.c_le_s(0, 1);
                        let t = !c.fcc();
                        c.g[V1][0] = 0u64;
                        if t {
                            break 'b_33b828;
                        } else {
                            c.cycles += 5;
                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                            c.c_le_s(0, 1);
                            let t = !c.fcc();

                            if t {
                                break 'b_33b828;
                            } else {
                                c.cycles += 5;
                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                c.c_le_s(1, 0);
                                let t = !c.fcc();

                                if t {
                                    break 'b_33b828;
                                } else {
                                    c.cycles += 5;
                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                    c.c_le_s(0, 1);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_33b828;
                                    } else {
                                        c.cycles += 5;
                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                        c.c_le_s(1, 0);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_33b828;
                                        } else {
                                            c.cycles += 6;
                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                            c.c_le_s(0, 1);
                                            if c.fcc() {
                                                c.g[V1][0] = 0x1;
                                                break 'b_33b828;
                                            } else {
                                                break 'b_33b828;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    'b_33b854: {
                        c.cycles += 2;
                        if c.g[V1][0] == 0u64 {
                            c.g[A2][0] = c.lw((c.g[A2][0] as u32));
                            break 'b_33b854;
                        } else {
                            c.cycles += 9;
                            c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.lw((c.g[A3][0] as u32));
                            c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                            c.sw((c.g[V0][0] as u32), c.g[A2][0]);
                            c.sw((c.g[A1][0] as u32), c.g[V1][0]);
                            c.g[A2][0] = c.lw((c.g[A2][0] as u32));
                            break 'b_33b854;
                        }
                    }
                    c.cycles += 2;
                    if c.g[A2][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                        if event_due(c) {
                            deliver_event_here(c, 0x0033b7a8);
                        }
                        continue 'l_33b7a8;
                    } else {
                        break 'b_33b85c;
                    }
                }
            }
        }
        'b_33b898: {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x28));
            let t = c.g[A2][0] == 0u64;
            c.g[A3][0] = sx((c.g[S4][0] as u32).wrapping_add(0x20));
            if t {
                break 'b_33b898;
            } else {
                c.cycles += 2;
                c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                'l_33b870: loop {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw((c.g[A3][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[V0][0] as u32), c.g[A2][0]);
                    c.sw((c.g[A1][0] as u32), c.g[V1][0]);
                    c.g[A2][0] = c.lw((c.g[A2][0] as u32));
                    if c.g[A2][0] != 0u64 {
                        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                        if event_due(c) {
                            deliver_event_here(c, 0x0033b870);
                        }
                        continue 'l_33b870;
                    } else {
                        break 'b_33b898;
                    }
                }
            }
        }
        // Classify the eight child cells against the query box; then visit
        // each child in z-order: gather it (`Cell_GatherAllSubtree`), recurse
        // into it, or skip it when it misses the box.
        'b_33ccc8: {
            'b_33ccc4: {
                c.cycles += 4;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                c.g[V0][0] = 0xb;
                let t = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e));
                if t {
                    break 'b_33ccc4;
                } else {
                    'b_33b92c: {
                        c.cycles += 28;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 23);
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2d10));
                        c.mtc1(2, 4);
                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(4, 0);
                        c.cvt_s_w(0, 0);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.lwc1(3, (c.g[S2][0] as u32));
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.sub_s(0, 0, 2);
                        c.add_s(1, 1, 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                        c.sw((c.g[SP][0] as u32), c.g[A1][0]);
                        c.g[S7][0] = 0u64;
                        c.mul_s(15, 0, 4);
                        c.mul_s(16, 1, 4);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                        c.c_lt_s(3, 16);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                        if t {
                            break 'b_33b92c;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(15, 0);
                            if c.fcc() {
                                c.g[S7][0] = 0x1;
                                break 'b_33b92c;
                            } else {
                                break 'b_33b92c;
                            }
                        }
                    }
                    'b_33b98c: {
                        c.cycles += 18;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2d0c));
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, (c.g[S2][0] as u32));
                        c.add_s(1, 1, 3);
                        c.mov_s(13, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(14, 1, 4);
                        c.c_lt_s(2, 14);
                        let t = !c.fcc();
                        c.mul_s(12, 0, 4);
                        if t {
                            break 'b_33b98c;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(12, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33b98c;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                break 'b_33b98c;
                            }
                        }
                    }
                    'b_33b9a0: {
                        c.cycles += 2;
                        let t = c.g[S7][0] != 0u64;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_33b9a0;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                            let t = c.g[V0][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                            if t {
                                break 'b_33ccc8;
                            } else {
                                break 'b_33b9a0;
                            }
                        }
                    }
                    'b_33b9fc: {
                        c.cycles += 17;
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2d08));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.g[A0][0] = c.g[V1][0];
                        c.mtc1(3, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x4));
                        c.add_s(1, 1, 3);
                        c.mov_s(6, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(11, 1, 4);
                        c.c_lt_s(2, 11);
                        let t = !c.fcc();
                        c.mul_s(10, 0, 4);
                        if t {
                            break 'b_33b9fc;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(10, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33b9fc;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                                break 'b_33b9fc;
                            }
                        }
                    }
                    'b_33ba4c: {
                        c.cycles += 15;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2d04));
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.g[FP][0] = 0u64;
                        c.add_s(1, 1, 2);
                        c.sub_s(0, 0, 2);
                        c.mul_s(9, 1, 4);
                        c.c_lt_s(6, 9);
                        let t = !c.fcc();
                        c.mul_s(8, 0, 4);
                        if t {
                            break 'b_33ba4c;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(8, 0);
                            if c.fcc() {
                                c.g[FP][0] = 0x1;
                                break 'b_33ba4c;
                            } else {
                                break 'b_33ba4c;
                            }
                        }
                    }
                    'b_33ba60: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        if t {
                            break 'b_33ba60;
                        } else {
                            c.cycles += 2;
                            let t = c.g[FP][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                            if t {
                                break 'b_33ccc8;
                            } else {
                                break 'b_33ba60;
                            }
                        }
                    }
                    'b_33babc: {
                        c.cycles += 17;
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2d00));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.g[A0][0] = c.g[V1][0];
                        c.mtc1(3, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x8));
                        c.add_s(1, 1, 3);
                        c.mov_s(5, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(7, 1, 4);
                        c.c_lt_s(2, 7);
                        let t = !c.fcc();
                        c.mul_s(3, 0, 4);
                        if t {
                            break 'b_33babc;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(3, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33babc;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                break 'b_33babc;
                            }
                        }
                    }
                    'b_33bb10: {
                        c.cycles += 15;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2cfc));
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
                        c.add_s(1, 1, 2);
                        c.sub_s(0, 0, 2);
                        c.mul_s(1, 1, 4);
                        c.c_lt_s(5, 1);
                        let t = !c.fcc();
                        c.mul_s(2, 0, 4);
                        if t {
                            break 'b_33bb10;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(2, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33bb10;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                                break 'b_33bb10;
                            }
                        }
                    }
                    'b_33bb24: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                        if t {
                            break 'b_33bb24;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                            if t {
                                break 'b_33ccc8;
                            } else {
                                break 'b_33bb24;
                            }
                        }
                    }
                    'b_33bb48: {
                        c.cycles += 4;
                        c.c_lt_s(13, 15);
                        let t = !c.fcc();
                        c.g[S6][0] = 0u64;
                        if t {
                            break 'b_33bb48;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(16, 0);
                            if c.fcc() {
                                c.g[S6][0] = 0x1;
                                break 'b_33bb48;
                            } else {
                                break 'b_33bb48;
                            }
                        }
                    }
                    'b_33bb70: {
                        c.cycles += 4;
                        c.c_lt_s(13, 12);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                        if t {
                            break 'b_33bb70;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(14, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33bb70;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                                break 'b_33bb70;
                            }
                        }
                    }
                    'b_33bb98: {
                        c.cycles += 4;
                        c.c_lt_s(6, 10);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                        if t {
                            break 'b_33bb98;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(11, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33bb98;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                                break 'b_33bb98;
                            }
                        }
                    }
                    'b_33bbc0: {
                        c.cycles += 4;
                        c.c_lt_s(6, 8);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        if t {
                            break 'b_33bbc0;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(9, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33bbc0;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                                break 'b_33bbc0;
                            }
                        }
                    }
                    'b_33bbe8: {
                        c.cycles += 4;
                        c.c_lt_s(5, 3);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                        if t {
                            break 'b_33bbe8;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(7, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33bbe8;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                                break 'b_33bbe8;
                            }
                        }
                    }
                    'b_33bc10: {
                        c.cycles += 4;
                        c.c_lt_s(5, 2);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        if t {
                            break 'b_33bc10;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(1, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33bc10;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                                break 'b_33bc10;
                            }
                        }
                    }
                    'b_33be20: {
                        c.cycles += 3;
                        c.g[S0][0] = c.lw((c.g[S3][0] as u32));
                        let t = c.g[S0][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        if t {
                            break 'b_33be20;
                        } else {
                            'b_33be1c: {
                                c.cycles += 2;
                                let t = c.g[S7][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_33be1c;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                    if t {
                                        break 'b_33be1c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_33be20;
                                        } else {
                                            'b_33be00: {
                                                c.cycles += 2;
                                                let t = c.g[S6][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                                if t {
                                                    break 'b_33be00;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_33be00;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_33be00;
                                                        } else {
                                                            'b_33bd1c: {
                                                                c.cycles += 6;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                GATHER_OBJECTS
                                                                    .call(c, 0x0033bc54)?;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[T0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                let t = c.g[A2][0] == 0u64;
                                                                c.g[A3][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x20));
                                                                if t {
                                                                    break 'b_33bd1c;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    'l_33bc68: loop {
                                                                        'b_33bce8: {
                                                                            c.cycles += 6;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x164,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.c_le_s(0, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_33bce8;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                                                c.c_le_s(0, 1);
                                                                                let t = !c.fcc();

                                                                                if t {
                                                                                    break 'b_33bce8;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                                                                    c.c_le_s(1, 0);
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_33bce8;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            5;
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                                                                        c.c_le_s(
                                                                                            0, 1,
                                                                                        );
                                                                                        let t = !c
                                                                                            .fcc();

                                                                                        if t {
                                                                                            break 'b_33bce8;
                                                                                        } else {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                                                                            c.c_le_s(1, 0);
                                                                                            let t = !c.fcc();

                                                                                            if t {
                                                                                                break 'b_33bce8;
                                                                                            } else {
                                                                                                c.cycles += 6;
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                                                                                c.c_le_s(0, 1);
                                                                                                if c.fcc() {
c.g[V1][0] = 0x1;
break 'b_33bce8;
} else {
break 'b_33bce8;
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_33bd14: {
                                                                            c.cycles += 2;
                                                                            if c.g[V1][0] == 0u64 {
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33bd14;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[T0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V1][0] = c.lw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                                                c.sw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                    c.g[A2][0],
                                                                                );
                                                                                c.sw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33bd14;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033bc68,
                                                                                );
                                                                            }
                                                                            continue 'l_33bc68;
                                                                        } else {
                                                                            break 'b_33bd1c;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33bd5c: {
                                                                c.cycles += 3;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                if c.g[A2][0] == 0u64 {
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    break 'b_33bd5c;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    'l_33bd30: loop {
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[A3][0] as u32)
                                                                            );
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A1][0] as u32)
                                                                            );
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [A3][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033bd30,
                                                                                );
                                                                            }
                                                                            continue 'l_33bd30;
                                                                        } else {
                                                                            c.cycles += 1;
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            break 'b_33bd5c;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33bd70: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33bd70;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bd6c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33bd70;
                                                                }
                                                            }
                                                            'b_33bd84: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33bd84;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bd80)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33bd84;
                                                                }
                                                            }
                                                            'b_33bd98: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33bd98;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bd94)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33bd98;
                                                                }
                                                            }
                                                            'b_33bdac: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33bdac;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bda8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33bdac;
                                                                }
                                                            }
                                                            'b_33bdc0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33bdc0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bdbc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33bdc0;
                                                                }
                                                            }
                                                            'b_33bdd4: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33bdd4;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bdd0)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33bdd4;
                                                                }
                                                            }
                                                            'b_33bde8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33bde8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bde4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33bde8;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if t {
                                                                break 'b_33be20;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A1][0] = c.g[S1][0];
                                                                GATHER_ALL_SUBTREE
                                                                    .call(c, 0x0033bdf8)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33be20;
                                                                } else {
                                                                    break 'b_33be00;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            GATHER_ALL_RECURSIVE.call(c, 0x0033be1c)?;
                                            break 'b_33be1c;
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                            break 'b_33be20;
                        }
                    }
                    'b_33c038: {
                        'b_33c034: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33c034;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S7][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_33c034;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_33c034;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_33c038;
                                        } else {
                                            'b_33c018: {
                                                c.cycles += 2;
                                                let t = c.g[S6][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                                if t {
                                                    break 'b_33c018;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    if t {
                                                        break 'b_33c018;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_33c018;
                                                        } else {
                                                            'b_33bf34: {
                                                                c.cycles += 6;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                GATHER_OBJECTS
                                                                    .call(c, 0x0033be68)?;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[T0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                let t = c.g[A2][0] == 0u64;
                                                                c.g[A3][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x20));
                                                                if t {
                                                                    break 'b_33bf34;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    'l_33be80: loop {
                                                                        'b_33bf00: {
                                                                            c.cycles += 6;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x164,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.c_le_s(0, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_33bf00;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                                                c.c_le_s(0, 1);
                                                                                let t = !c.fcc();

                                                                                if t {
                                                                                    break 'b_33bf00;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                                                                    c.c_le_s(1, 0);
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_33bf00;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            5;
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                                                                        c.c_le_s(
                                                                                            0, 1,
                                                                                        );
                                                                                        let t = !c
                                                                                            .fcc();

                                                                                        if t {
                                                                                            break 'b_33bf00;
                                                                                        } else {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                                                                            c.c_le_s(1, 0);
                                                                                            let t = !c.fcc();

                                                                                            if t {
                                                                                                break 'b_33bf00;
                                                                                            } else {
                                                                                                c.cycles += 6;
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                                                                                c.c_le_s(0, 1);
                                                                                                if c.fcc() {
c.g[V1][0] = 0x1;
break 'b_33bf00;
} else {
break 'b_33bf00;
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_33bf2c: {
                                                                            c.cycles += 2;
                                                                            if c.g[V1][0] == 0u64 {
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33bf2c;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[T0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V1][0] = c.lw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                                                c.sw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                    c.g[A2][0],
                                                                                );
                                                                                c.sw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33bf2c;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033be80,
                                                                                );
                                                                            }
                                                                            continue 'l_33be80;
                                                                        } else {
                                                                            break 'b_33bf34;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33bf74: {
                                                                c.cycles += 3;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                if c.g[A2][0] == 0u64 {
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    break 'b_33bf74;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    'l_33bf48: loop {
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[A3][0] as u32)
                                                                            );
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A1][0] as u32)
                                                                            );
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [A3][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033bf48,
                                                                                );
                                                                            }
                                                                            continue 'l_33bf48;
                                                                        } else {
                                                                            c.cycles += 1;
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            break 'b_33bf74;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33bf88: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33bf88;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bf84)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33bf88;
                                                                }
                                                            }
                                                            'b_33bf9c: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33bf9c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bf98)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33bf9c;
                                                                }
                                                            }
                                                            'b_33bfb0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33bfb0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bfac)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33bfb0;
                                                                }
                                                            }
                                                            'b_33bfc4: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33bfc4;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bfc0)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33bfc4;
                                                                }
                                                            }
                                                            'b_33bfd8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33bfd8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bfd4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33bfd8;
                                                                }
                                                            }
                                                            'b_33bfec: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33bfec;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bfe8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33bfec;
                                                                }
                                                            }
                                                            'b_33c000: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c000;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033bffc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c000;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x8));
                                                            if t {
                                                                break 'b_33c038;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A1][0] = c.g[S1][0];
                                                                GATHER_ALL_SUBTREE
                                                                    .call(c, 0x0033c010)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                if t {
                                                                    break 'b_33c038;
                                                                } else {
                                                                    break 'b_33c018;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            GATHER_ALL_RECURSIVE.call(c, 0x0033c034)?;
                                            break 'b_33c034;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        break 'b_33c038;
                    }
                    'b_33c250: {
                        'b_33c24c: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            if t {
                                break 'b_33c24c;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S7][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                if t {
                                    break 'b_33c250;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[FP][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_33c24c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_33c250;
                                        } else {
                                            'b_33c230: {
                                                c.cycles += 2;
                                                let t = c.g[S6][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                if t {
                                                    break 'b_33c230;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    if t {
                                                        break 'b_33c230;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_33c230;
                                                        } else {
                                                            'b_33c14c: {
                                                                c.cycles += 6;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                GATHER_OBJECTS
                                                                    .call(c, 0x0033c080)?;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[T0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                let t = c.g[A2][0] == 0u64;
                                                                c.g[A3][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x20));
                                                                if t {
                                                                    break 'b_33c14c;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    'l_33c098: loop {
                                                                        'b_33c118: {
                                                                            c.cycles += 6;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x164,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.c_le_s(0, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_33c118;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                                                c.c_le_s(0, 1);
                                                                                let t = !c.fcc();

                                                                                if t {
                                                                                    break 'b_33c118;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                                                                    c.c_le_s(1, 0);
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_33c118;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            5;
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                                                                        c.c_le_s(
                                                                                            0, 1,
                                                                                        );
                                                                                        let t = !c
                                                                                            .fcc();

                                                                                        if t {
                                                                                            break 'b_33c118;
                                                                                        } else {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                                                                            c.c_le_s(1, 0);
                                                                                            let t = !c.fcc();

                                                                                            if t {
                                                                                                break 'b_33c118;
                                                                                            } else {
                                                                                                c.cycles += 6;
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                                                                                c.c_le_s(0, 1);
                                                                                                if c.fcc() {
c.g[V1][0] = 0x1;
break 'b_33c118;
} else {
break 'b_33c118;
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_33c144: {
                                                                            c.cycles += 2;
                                                                            if c.g[V1][0] == 0u64 {
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c144;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[T0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V1][0] = c.lw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                                                c.sw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                    c.g[A2][0],
                                                                                );
                                                                                c.sw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c144;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c098,
                                                                                );
                                                                            }
                                                                            continue 'l_33c098;
                                                                        } else {
                                                                            break 'b_33c14c;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c18c: {
                                                                c.cycles += 3;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                if c.g[A2][0] == 0u64 {
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    break 'b_33c18c;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    'l_33c160: loop {
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[A3][0] as u32)
                                                                            );
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A1][0] as u32)
                                                                            );
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [A3][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c160,
                                                                                );
                                                                            }
                                                                            continue 'l_33c160;
                                                                        } else {
                                                                            c.cycles += 1;
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            break 'b_33c18c;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c1a0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33c1a0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c19c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33c1a0;
                                                                }
                                                            }
                                                            'b_33c1b4: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33c1b4;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c1b0)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33c1b4;
                                                                }
                                                            }
                                                            'b_33c1c8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33c1c8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c1c4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33c1c8;
                                                                }
                                                            }
                                                            'b_33c1dc: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33c1dc;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c1d8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33c1dc;
                                                                }
                                                            }
                                                            'b_33c1f0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33c1f0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c1ec)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33c1f0;
                                                                }
                                                            }
                                                            'b_33c204: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33c204;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c200)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33c204;
                                                                }
                                                            }
                                                            'b_33c218: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c218;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c214)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c218;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if t {
                                                                break 'b_33c250;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A1][0] = c.g[S1][0];
                                                                GATHER_ALL_SUBTREE
                                                                    .call(c, 0x0033c228)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33c250;
                                                                } else {
                                                                    break 'b_33c230;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            GATHER_ALL_RECURSIVE.call(c, 0x0033c24c)?;
                                            break 'b_33c24c;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_33c250;
                    }
                    'b_33c468: {
                        'b_33c464: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33c464;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S7][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                if t {
                                    break 'b_33c468;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[FP][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                    if t {
                                        break 'b_33c464;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_33c468;
                                        } else {
                                            'b_33c448: {
                                                c.cycles += 2;
                                                let t = c.g[S6][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                if t {
                                                    break 'b_33c448;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_33c448;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_33c448;
                                                        } else {
                                                            'b_33c364: {
                                                                c.cycles += 6;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                GATHER_OBJECTS
                                                                    .call(c, 0x0033c298)?;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[T0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                let t = c.g[A2][0] == 0u64;
                                                                c.g[A3][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x20));
                                                                if t {
                                                                    break 'b_33c364;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    'l_33c2b0: loop {
                                                                        'b_33c330: {
                                                                            c.cycles += 6;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x164,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.c_le_s(0, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_33c330;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                                                c.c_le_s(0, 1);
                                                                                let t = !c.fcc();

                                                                                if t {
                                                                                    break 'b_33c330;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                                                                    c.c_le_s(1, 0);
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_33c330;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            5;
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                                                                        c.c_le_s(
                                                                                            0, 1,
                                                                                        );
                                                                                        let t = !c
                                                                                            .fcc();

                                                                                        if t {
                                                                                            break 'b_33c330;
                                                                                        } else {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                                                                            c.c_le_s(1, 0);
                                                                                            let t = !c.fcc();

                                                                                            if t {
                                                                                                break 'b_33c330;
                                                                                            } else {
                                                                                                c.cycles += 6;
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                                                                                c.c_le_s(0, 1);
                                                                                                if c.fcc() {
c.g[V1][0] = 0x1;
break 'b_33c330;
} else {
break 'b_33c330;
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_33c35c: {
                                                                            c.cycles += 2;
                                                                            if c.g[V1][0] == 0u64 {
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c35c;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[T0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V1][0] = c.lw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                                                c.sw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                    c.g[A2][0],
                                                                                );
                                                                                c.sw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c35c;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c2b0,
                                                                                );
                                                                            }
                                                                            continue 'l_33c2b0;
                                                                        } else {
                                                                            break 'b_33c364;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c3a4: {
                                                                c.cycles += 3;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                if c.g[A2][0] == 0u64 {
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    break 'b_33c3a4;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    'l_33c378: loop {
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[A3][0] as u32)
                                                                            );
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A1][0] as u32)
                                                                            );
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [A3][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c378,
                                                                                );
                                                                            }
                                                                            continue 'l_33c378;
                                                                        } else {
                                                                            c.cycles += 1;
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            break 'b_33c3a4;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c3b8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33c3b8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c3b4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33c3b8;
                                                                }
                                                            }
                                                            'b_33c3cc: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33c3cc;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c3c8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33c3cc;
                                                                }
                                                            }
                                                            'b_33c3e0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33c3e0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c3dc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33c3e0;
                                                                }
                                                            }
                                                            'b_33c3f4: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33c3f4;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c3f0)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33c3f4;
                                                                }
                                                            }
                                                            'b_33c408: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33c408;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c404)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33c408;
                                                                }
                                                            }
                                                            'b_33c41c: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33c41c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c418)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33c41c;
                                                                }
                                                            }
                                                            'b_33c430: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c430;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c42c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c430;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x4));
                                                            if t {
                                                                break 'b_33c468;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A1][0] = c.g[S1][0];
                                                                GATHER_ALL_SUBTREE
                                                                    .call(c, 0x0033c440)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                if t {
                                                                    break 'b_33c468;
                                                                } else {
                                                                    break 'b_33c448;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            GATHER_ALL_RECURSIVE.call(c, 0x0033c464)?;
                                            break 'b_33c464;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        break 'b_33c468;
                    }
                    'b_33c680: {
                        'b_33c67c: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            if t {
                                break 'b_33c67c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                if t {
                                    break 'b_33c680;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[FP][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                    if t {
                                        break 'b_33c67c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_33c67c;
                                        } else {
                                            'b_33c660: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                if t {
                                                    break 'b_33c660;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_33c660;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_33c660;
                                                        } else {
                                                            'b_33c57c: {
                                                                c.cycles += 6;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                GATHER_OBJECTS
                                                                    .call(c, 0x0033c4b4)?;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[T0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                let t = c.g[A2][0] == 0u64;
                                                                c.g[A3][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x20));
                                                                if t {
                                                                    break 'b_33c57c;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    'l_33c4c8: loop {
                                                                        'b_33c548: {
                                                                            c.cycles += 6;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x164,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.c_le_s(0, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_33c548;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                                                c.c_le_s(0, 1);
                                                                                let t = !c.fcc();

                                                                                if t {
                                                                                    break 'b_33c548;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                                                                    c.c_le_s(1, 0);
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_33c548;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            5;
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                                                                        c.c_le_s(
                                                                                            0, 1,
                                                                                        );
                                                                                        let t = !c
                                                                                            .fcc();

                                                                                        if t {
                                                                                            break 'b_33c548;
                                                                                        } else {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                                                                            c.c_le_s(1, 0);
                                                                                            let t = !c.fcc();

                                                                                            if t {
                                                                                                break 'b_33c548;
                                                                                            } else {
                                                                                                c.cycles += 6;
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                                                                                c.c_le_s(0, 1);
                                                                                                if c.fcc() {
c.g[V1][0] = 0x1;
break 'b_33c548;
} else {
break 'b_33c548;
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_33c574: {
                                                                            c.cycles += 2;
                                                                            if c.g[V1][0] == 0u64 {
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c574;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[T0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V1][0] = c.lw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                                                c.sw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                    c.g[A2][0],
                                                                                );
                                                                                c.sw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c574;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c4c8,
                                                                                );
                                                                            }
                                                                            continue 'l_33c4c8;
                                                                        } else {
                                                                            break 'b_33c57c;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c5bc: {
                                                                c.cycles += 3;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                if c.g[A2][0] == 0u64 {
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    break 'b_33c5bc;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    'l_33c590: loop {
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[A3][0] as u32)
                                                                            );
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A1][0] as u32)
                                                                            );
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [A3][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c590,
                                                                                );
                                                                            }
                                                                            continue 'l_33c590;
                                                                        } else {
                                                                            c.cycles += 1;
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            break 'b_33c5bc;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c5d0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33c5d0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c5cc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33c5d0;
                                                                }
                                                            }
                                                            'b_33c5e4: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33c5e4;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c5e0)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33c5e4;
                                                                }
                                                            }
                                                            'b_33c5f8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33c5f8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c5f4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33c5f8;
                                                                }
                                                            }
                                                            'b_33c60c: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33c60c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c608)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33c60c;
                                                                }
                                                            }
                                                            'b_33c620: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33c620;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c61c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33c620;
                                                                }
                                                            }
                                                            'b_33c634: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33c634;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c630)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33c634;
                                                                }
                                                            }
                                                            'b_33c648: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c648;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c644)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c648;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if t {
                                                                break 'b_33c680;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A1][0] = c.g[S1][0];
                                                                GATHER_ALL_SUBTREE
                                                                    .call(c, 0x0033c658)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33c680;
                                                                } else {
                                                                    break 'b_33c660;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            GATHER_ALL_RECURSIVE.call(c, 0x0033c67c)?;
                                            break 'b_33c67c;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_33c680;
                    }
                    'b_33c898: {
                        'b_33c894: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33c894;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_33c898;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[FP][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_33c894;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_33c894;
                                        } else {
                                            'b_33c878: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                if t {
                                                    break 'b_33c878;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    if t {
                                                        break 'b_33c878;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_33c878;
                                                        } else {
                                                            'b_33c794: {
                                                                c.cycles += 6;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                GATHER_OBJECTS
                                                                    .call(c, 0x0033c6cc)?;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[T0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                let t = c.g[A2][0] == 0u64;
                                                                c.g[A3][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x20));
                                                                if t {
                                                                    break 'b_33c794;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    'l_33c6e0: loop {
                                                                        'b_33c760: {
                                                                            c.cycles += 6;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x164,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.c_le_s(0, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_33c760;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                                                c.c_le_s(0, 1);
                                                                                let t = !c.fcc();

                                                                                if t {
                                                                                    break 'b_33c760;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                                                                    c.c_le_s(1, 0);
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_33c760;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            5;
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                                                                        c.c_le_s(
                                                                                            0, 1,
                                                                                        );
                                                                                        let t = !c
                                                                                            .fcc();

                                                                                        if t {
                                                                                            break 'b_33c760;
                                                                                        } else {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                                                                            c.c_le_s(1, 0);
                                                                                            let t = !c.fcc();

                                                                                            if t {
                                                                                                break 'b_33c760;
                                                                                            } else {
                                                                                                c.cycles += 6;
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                                                                                c.c_le_s(0, 1);
                                                                                                if c.fcc() {
c.g[V1][0] = 0x1;
break 'b_33c760;
} else {
break 'b_33c760;
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_33c78c: {
                                                                            c.cycles += 2;
                                                                            if c.g[V1][0] == 0u64 {
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c78c;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[T0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V1][0] = c.lw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                                                c.sw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                    c.g[A2][0],
                                                                                );
                                                                                c.sw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c78c;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c6e0,
                                                                                );
                                                                            }
                                                                            continue 'l_33c6e0;
                                                                        } else {
                                                                            break 'b_33c794;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c7d4: {
                                                                c.cycles += 3;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                if c.g[A2][0] == 0u64 {
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    break 'b_33c7d4;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    'l_33c7a8: loop {
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[A3][0] as u32)
                                                                            );
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A1][0] as u32)
                                                                            );
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [A3][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c7a8,
                                                                                );
                                                                            }
                                                                            continue 'l_33c7a8;
                                                                        } else {
                                                                            c.cycles += 1;
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            break 'b_33c7d4;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c7e8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33c7e8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c7e4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33c7e8;
                                                                }
                                                            }
                                                            'b_33c7fc: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33c7fc;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c7f8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33c7fc;
                                                                }
                                                            }
                                                            'b_33c810: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33c810;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c80c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33c810;
                                                                }
                                                            }
                                                            'b_33c824: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33c824;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c820)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33c824;
                                                                }
                                                            }
                                                            'b_33c838: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33c838;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c834)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33c838;
                                                                }
                                                            }
                                                            'b_33c84c: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33c84c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c848)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33c84c;
                                                                }
                                                            }
                                                            'b_33c860: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c860;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c85c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33c860;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x8));
                                                            if t {
                                                                break 'b_33c898;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A1][0] = c.g[S1][0];
                                                                GATHER_ALL_SUBTREE
                                                                    .call(c, 0x0033c870)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                if t {
                                                                    break 'b_33c898;
                                                                } else {
                                                                    break 'b_33c878;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            GATHER_ALL_RECURSIVE.call(c, 0x0033c894)?;
                                            break 'b_33c894;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        break 'b_33c898;
                    }
                    'b_33cab0: {
                        'b_33caac: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            if t {
                                break 'b_33caac;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_33caac;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_33caac;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_33caac;
                                        } else {
                                            'b_33ca90: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                                if t {
                                                    break 'b_33ca90;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    if t {
                                                        break 'b_33ca90;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_33ca90;
                                                        } else {
                                                            'b_33c9ac: {
                                                                c.cycles += 6;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                GATHER_OBJECTS
                                                                    .call(c, 0x0033c8e4)?;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[T0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                let t = c.g[A2][0] == 0u64;
                                                                c.g[A3][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x20));
                                                                if t {
                                                                    break 'b_33c9ac;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    'l_33c8f8: loop {
                                                                        'b_33c978: {
                                                                            c.cycles += 6;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x164,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.c_le_s(0, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_33c978;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                                                c.c_le_s(0, 1);
                                                                                let t = !c.fcc();

                                                                                if t {
                                                                                    break 'b_33c978;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                                                                    c.c_le_s(1, 0);
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_33c978;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            5;
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                                                                        c.c_le_s(
                                                                                            0, 1,
                                                                                        );
                                                                                        let t = !c
                                                                                            .fcc();

                                                                                        if t {
                                                                                            break 'b_33c978;
                                                                                        } else {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                                                                            c.c_le_s(1, 0);
                                                                                            let t = !c.fcc();

                                                                                            if t {
                                                                                                break 'b_33c978;
                                                                                            } else {
                                                                                                c.cycles += 6;
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                                                                                c.c_le_s(0, 1);
                                                                                                if c.fcc() {
c.g[V1][0] = 0x1;
break 'b_33c978;
} else {
break 'b_33c978;
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_33c9a4: {
                                                                            c.cycles += 2;
                                                                            if c.g[V1][0] == 0u64 {
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c9a4;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[T0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V1][0] = c.lw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                                                c.sw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                    c.g[A2][0],
                                                                                );
                                                                                c.sw(
                                                                                    (c.g[A1][0]
                                                                                        as u32),
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[A2][0] = c.lw(
                                                                                    (c.g[A2][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_33c9a4;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c8f8,
                                                                                );
                                                                            }
                                                                            continue 'l_33c8f8;
                                                                        } else {
                                                                            break 'b_33c9ac;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33c9ec: {
                                                                c.cycles += 3;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                if c.g[A2][0] == 0u64 {
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    break 'b_33c9ec;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    'l_33c9c0: loop {
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[A3][0] as u32)
                                                                            );
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A1][0] as u32)
                                                                            );
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        if c.g[A2][0] != 0u64 {
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [A3][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0033c9c0,
                                                                                );
                                                                            }
                                                                            continue 'l_33c9c0;
                                                                        } else {
                                                                            c.cycles += 1;
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            break 'b_33c9ec;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_33ca00: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33ca00;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033c9fc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_33ca00;
                                                                }
                                                            }
                                                            'b_33ca14: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33ca14;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033ca10)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_33ca14;
                                                                }
                                                            }
                                                            'b_33ca28: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33ca28;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033ca24)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33ca28;
                                                                }
                                                            }
                                                            'b_33ca3c: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33ca3c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033ca38)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_33ca3c;
                                                                }
                                                            }
                                                            'b_33ca50: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33ca50;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033ca4c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_33ca50;
                                                                }
                                                            }
                                                            'b_33ca64: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33ca64;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033ca60)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_33ca64;
                                                                }
                                                            }
                                                            'b_33ca78: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33ca78;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    GATHER_ALL_SUBTREE
                                                                        .call(c, 0x0033ca74)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33ca78;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if t {
                                                                break 'b_33cab0;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A1][0] = c.g[S1][0];
                                                                GATHER_ALL_SUBTREE
                                                                    .call(c, 0x0033ca88)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33cab0;
                                                                } else {
                                                                    break 'b_33ca90;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[T1][0] = c.g[SP][0];
                                            GATHER_ALL_RECURSIVE.call(c, 0x0033caac)?;
                                            break 'b_33caac;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_33cab0;
                    }
                    c.cycles += 4;
                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[S0][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    if t {
                        break 'b_33ccc4;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        if t {
                            break 'b_33ccc4;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                            if t {
                                break 'b_33ccc4;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_33ccc4;
                                } else {
                                    'b_33cca8: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                        if t {
                                            break 'b_33cca8;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                            if t {
                                                break 'b_33cca8;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = c.g[S1][0];
                                                if t {
                                                    break 'b_33cca8;
                                                } else {
                                                    'b_33cbc4: {
                                                        c.cycles += 6;
                                                        c.g[A1][0] = c.g[S0][0];
                                                        GATHER_OBJECTS.call(c, 0x0033cafc)?;
                                                        c.g[A2][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x24));
                                                        c.g[T0][0] =
                                                            sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                        let t = c.g[A2][0] == 0u64;
                                                        c.g[A3][0] =
                                                            sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x20));
                                                        if t {
                                                            break 'b_33cbc4;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.g[V0][0] = c.lw((c.g[T0][0] as u32)
                                                                .wrapping_add(0xc));
                                                            'l_33cb10: loop {
                                                                'b_33cb90: {
                                                                    c.cycles += 6;
                                                                    c.lwc1(
                                                                        1,
                                                                        (c.g[A2][0] as u32)
                                                                            .wrapping_add(0x164),
                                                                    );
                                                                    c.lwc1(0, (c.g[V0][0] as u32));
                                                                    c.c_le_s(0, 1);
                                                                    let t = !c.fcc();
                                                                    c.g[V1][0] = 0u64;
                                                                    if t {
                                                                        break 'b_33cb90;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.lwc1(
                                                                            1,
                                                                            (c.g[V0][0] as u32)
                                                                                .wrapping_add(0x10),
                                                                        );
                                                                        c.lwc1(
                                                                            0,
                                                                            (c.g[A2][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x158,
                                                                                ),
                                                                        );
                                                                        c.c_le_s(0, 1);
                                                                        let t = !c.fcc();

                                                                        if t {
                                                                            break 'b_33cb90;
                                                                        } else {
                                                                            c.cycles += 5;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[V0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x4,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x168,
                                                                                    ),
                                                                            );
                                                                            c.c_le_s(1, 0);
                                                                            let t = !c.fcc();

                                                                            if t {
                                                                                break 'b_33cb90;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x15c));
                                                                                c.c_le_s(0, 1);
                                                                                let t = !c.fcc();

                                                                                if t {
                                                                                    break 'b_33cb90;
                                                                                } else {
                                                                                    c.cycles += 5;
                                                                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x16c));
                                                                                    c.c_le_s(1, 0);
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_33cb90;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            6;
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x160));
                                                                                        c.c_le_s(
                                                                                            0, 1,
                                                                                        );
                                                                                        if c.fcc() {
                                                                                            c.g[V1][0] = 0x1;
                                                                                            break 'b_33cb90;
                                                                                        } else {
                                                                                            break 'b_33cb90;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33cbbc: {
                                                                    c.cycles += 2;
                                                                    if c.g[V1][0] == 0u64 {
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        break 'b_33cbbc;
                                                                    } else {
                                                                        c.cycles += 9;
                                                                        c.g[A1][0] = c
                                                                            .lw((c.g[T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[T0][0] as u32)
                                                                            );
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A1][0] as u32)
                                                                            );
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[A1][0] as u32),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A2][0] =
                                                                            c.lw(
                                                                                (c.g[A2][0] as u32)
                                                                            );
                                                                        break 'b_33cbbc;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                if c.g[A2][0] != 0u64 {
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c, 0x0033cb10,
                                                                        );
                                                                    }
                                                                    continue 'l_33cb10;
                                                                } else {
                                                                    break 'b_33cbc4;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_33cc04: {
                                                        c.cycles += 3;
                                                        c.g[A2][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x28));
                                                        if c.g[A2][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                                            break 'b_33cc04;
                                                        } else {
                                                            c.cycles += 2;
                                                            c.g[A1][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x4));
                                                            'l_33cbd8: loop {
                                                                c.cycles += 10;
                                                                c.g[A0][0] =
                                                                    c.lw((c.g[A3][0] as u32));
                                                                c.g[V1][0] =
                                                                    c.lw((c.g[A1][0] as u32));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V1][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A0][0] as u32),
                                                                    ));
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                c.sw(
                                                                    (c.g[V0][0] as u32),
                                                                    c.g[A2][0],
                                                                );
                                                                c.sw(
                                                                    (c.g[A1][0] as u32),
                                                                    c.g[V1][0],
                                                                );
                                                                c.g[A2][0] =
                                                                    c.lw((c.g[A2][0] as u32));
                                                                if c.g[A2][0] != 0u64 {
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c, 0x0033cbd8,
                                                                        );
                                                                    }
                                                                    continue 'l_33cbd8;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    break 'b_33cc04;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_33cc18: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            break 'b_33cc18;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            GATHER_ALL_SUBTREE
                                                                .call(c, 0x0033cc14)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            break 'b_33cc18;
                                                        }
                                                    }
                                                    'b_33cc2c: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            break 'b_33cc2c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            GATHER_ALL_SUBTREE
                                                                .call(c, 0x0033cc28)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            break 'b_33cc2c;
                                                        }
                                                    }
                                                    'b_33cc40: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xc));
                                                            break 'b_33cc40;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            GATHER_ALL_SUBTREE
                                                                .call(c, 0x0033cc3c)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xc));
                                                            break 'b_33cc40;
                                                        }
                                                    }
                                                    'b_33cc54: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            break 'b_33cc54;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            GATHER_ALL_SUBTREE
                                                                .call(c, 0x0033cc50)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            break 'b_33cc54;
                                                        }
                                                    }
                                                    'b_33cc68: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x14));
                                                            break 'b_33cc68;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            GATHER_ALL_SUBTREE
                                                                .call(c, 0x0033cc64)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x14));
                                                            break 'b_33cc68;
                                                        }
                                                    }
                                                    'b_33cc7c: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            break 'b_33cc7c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            GATHER_ALL_SUBTREE
                                                                .call(c, 0x0033cc78)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            break 'b_33cc7c;
                                                        }
                                                    }
                                                    'b_33cc90: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1c));
                                                            break 'b_33cc90;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            GATHER_ALL_SUBTREE
                                                                .call(c, 0x0033cc8c)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1c));
                                                            break 'b_33cc90;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = c.g[A0][0] == 0u64;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                    );
                                                    if t {
                                                        break 'b_33ccc8;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[A1][0] = c.g[S1][0];
                                                        GATHER_ALL_SUBTREE.call(c, 0x0033cca0)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        if t {
                                                            break 'b_33ccc8;
                                                        } else {
                                                            break 'b_33cca8;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[A0][0] = c.g[S5][0];
                                    c.g[A1][0] = c.g[S4][0];
                                    c.g[A2][0] = c.g[S1][0];
                                    c.g[A3][0] = c.g[S2][0];
                                    c.g[T0][0] = c.g[S0][0];
                                    c.g[T1][0] = c.g[SP][0];
                                    GATHER_ALL_RECURSIVE.call(c, 0x0033ccc4)?;
                                    break 'b_33ccc4;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
            break 'b_33ccc8;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
        abi::ret(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Object constructors / destructors and small managers.

/// `void* BoostModifier_ctor_2(self, stream)` (0x342808): install the vtable,
/// clear the header word and read two 4-word blocks through the stream's
/// reader table.
pub fn boost_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.g[V1][0] = 0x49_0000;
        c.sq(17, sp_3(c).wrapping_add(0x10));
        c.g[V1][0] = sx(low(c, V1).wrapping_add(0x898));
        c.sq(16, sp_3(c).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sd(sp_3(c), c.g[RA][0]);
        c.g[V0][0] = 0x1;
        sw_r(c, S1, 0x4, c.g[V1][0]);
        c.g[S0][0] = c.g[A1][0];
        sw_r(c, S1, 0, c.g[V0][0]);
        c.g[A1][0] = sx(low(c, S1).wrapping_add(0x10));
        c.g[A2][0] = 0x30;
        c.g[V0][0] = Addr(low(c, S0)).lw(c);
        c.g[A0][0] = lh_r(c, V0, 0x10);
        c.g[A3][0] = lw_r(c, V0, 0x14);
        let target = low(c, A3);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0034_2850)?;
        c.g[V1][0] = Addr(low(c, S0)).lw(c);
        c.g[A0][0] = lh_r(c, V1, 0x18);
        c.g[V0][0] = lw_r(c, V1, 0x1c);
        let target = low(c, V0);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0034_2864)?;
        sw_r(c, S1, 0x40, c.g[V0][0]);
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, sp_3(c).wrapping_add(0x20));
        c.lq(17, sp_3(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_3(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}


/// `int CollSphereTree_CollideSphereTree(self, other, hit)` (0x3308c8): test
/// the tree against another tree; on a hit normalise the contact direction
/// into `hit->dir` and store its length, returning 1.
pub fn collspheretree_collide_sphere_tree(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A3][0] = c.g[S0][0];
        c.g[A0][0] = lw_at(c, p(abi::r32(c, A0), 0x60));
        call_ptr(c, callee::SPHERE_TREE_COLLIDE_SPHERE_TREE, 0x0033_08e8)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 19;
            collide_normalise(c, c.g[S0][0] as u32);
            c.g[V0][0] = 1;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int CollSphereTree_CollideAABB(self, aabb, hit)` (0x330950): the AABB
/// twin of [`collspheretree_collide_sphere_tree`].
pub fn collspheretree_collide_aabb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A3][0];
        c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[T0][0] = c.g[S0][0];
        c.g[A0][0] = lw_at(c, p(abi::r32(c, A0), 0x60));
        call_ptr(c, callee::SPHERE_TREE_COLLIDE_AABB, 0x0033_0970)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 19;
            collide_normalise(c, c.g[S0][0] as u32);
            c.g[V0][0] = 1;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The VU0 tail shared by the two `CollSphereTree` collisions: the length of
/// `hit->dir` (`hit+0x10`) into `hit+0x40` and the normalised direction back
/// into `hit+0x10`, with the original's COP2 macro sequence.
pub(crate) fn collide_normalise(c: &mut Cpu, hit: u32) {
    c.lqc2(3, p(hit, 0x10));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(2, 22);
    c.mtc1(2, 0);
    c.swc1(0, p(hit, 0x40));
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, p(hit, 0x10));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 4, Src::Q);
    c.sqc2(5, p(hit, 0x10));
}


/// `int CollSphereTree_CollideTriangle(self, ..., outLen, outDir)` (0x330788):
/// collide the tree with a triangle and compute the contact direction.
pub fn coll_sphere_tree_collide_triangle(c: &mut Cpu) {
    guest(c, |c| coll_sphere_tree_common(c, table::COLL_SPHERE_TREE_TRIANGLE, 0x0033_07b0))
}


/// `int CollSphereTree_CollideTriangle2Sided(...)` (0x330828).
pub fn coll_sphere_tree_collide_triangle2_sided(c: &mut Cpu) {
    guest(c, |c| coll_sphere_tree_common(c, table::COLL_HULL_TRIANGLE, 0x0033_0850))
}


// ---------------------------------------------------------------------------
// Modifiers.

/// The shared body of the boost modifiers: scale the object's velocity by
/// `1 + gain` and by the frame scale, then hand it back.
pub(crate) fn boost_scale(c: &mut Cpu, gain_off: u32, entry: u32) -> Flow {
    c.cycles += 41;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0x30);
    c.g[S1][0] = c.g[A0][0];
    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
    c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lh(c);
    c.g[V0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
    let get = r32(c, V0);
    c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
    call_ptr(c, get, entry + 0x2c)?;
    c.lwc1(0, p(r32(c, S1), gain_off));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.add_s(0, 0, 1);
    c.mfc1(3, 0);
    c.lqc2(4, r32(c, V0));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0x20));
    c.lq(2, p(sp(c), 0x20));
    c.sq(2, p(sp(c), 0x10));
    call_ptr(c, 0x002d_1c70, entry + 0x5c)?;
    c.mfc1(2, 0);
    c.lqc2(4, p(sp(c), 0x10));
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0x20));
    c.g[A1][0] = c.g[SP][0];
    c.lq(2, p(sp(c), 0x20));
    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
    c.sq(2, sp(c));
    c.g[A0][0] = Addr(p(r32(c, V1), 0x20)).lh(c);
    c.g[V0][0] = Addr(p(r32(c, V1), 0x24)).lw(c);
    let set = r32(c, V0);
    c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
    call_ptr(c, set, entry + 0x90)?;
    frame.restore(c, S0, 0x50);
    frame.restore(c, S1, 0x40);
    frame.restore_ra(c, 0x30);
    frame.close(c);
    Ok(())
}


/// `void Boost_v48(self, obj)` (0x341770): boost the object's velocity using
/// the effect's `+0x34` gain.
pub fn boost_v48(c: &mut Cpu) {
    guest(c, |c| boost_scale(c, 0x34, 0x0034_1770))
}


/// `void BoostModifier_v08(self, obj)` (0x3429e0): same with `+0x24`.
pub fn boost_modifier_v08(c: &mut Cpu) {
    guest(c, |c| boost_scale(c, 0x24, 0x0034_29e0))
}


/// `cWorldSphTree* func_003303F0(cWorldSphTree* self, SphereNode* owner, int flag)`
/// (0x3303f0): construct a collision sphere tree over `owner`: set its vtable
/// and counters, copy the owner's sphere, derive the plus/minus boxes from the
/// radius, and allocate and initialise the node storage.
pub fn coll_sphere_tree_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, 17, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1a10));
        frame.save(c, 16, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.sw(p(c.g[S1][0] as u32, 0x50), c.g[V0][0]); // vtable
        c.sw(p(c.g[S1][0] as u32, 8), 0);
        let no_flag = c.g[A2][0] == 0;
        c.sw(p(c.g[S1][0] as u32, 4), 0);
        if no_flag {
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        } else {
            c.sw(c.g[S1][0] as u32, 0);
        }
        c.cycles += 47;
        c.sw(p(c.g[S1][0] as u32, 0x64), c.g[A1][0]);
        c.g[A0][0] = c.g[A1][0];
        c.sw(p(c.g[S1][0] as u32, 0x60), c.g[A1][0]);
        c.lq(2, p(c.g[A1][0] as u32, 0x80)); // the owner's sphere
        c.sq(2, c.g[SP][0] as u32);
        call_ptr(c, 0x0032_c590, 0x0033_0444)?; // SphereTree_GetRadius(owner)
        c.lwc1(3, c.g[SP][0] as u32); // cx
        c.g[A1][0] = 0x49_0000;
        c.lwc1(2, p(c.g[SP][0] as u32, 4)); // cy
        c.g[A2][0] = 0;
        c.lwc1(1, p(c.g[SP][0] as u32, 8)); // cz
        c.add_s(4, 3, 0); // cx + r
        c.add_s(5, 2, 0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 7); // f7 = 1.0
        c.add_s(6, 1, 0);
        c.lq(2, c.g[SP][0] as u32);
        c.swc1(4, p(c.g[SP][0] as u32, 0x10));
        c.sub_s(1, 1, 0);
        c.swc1(5, p(c.g[SP][0] as u32, 0x14));
        c.sub_s(3, 3, 0);
        c.swc1(6, p(c.g[SP][0] as u32, 0x18));
        c.sub_s(2, 2, 0);
        c.swc1(7, p(c.g[SP][0] as u32, 0x1c));
        c.g[A3][0] = 0;
        c.sq(2, p(c.g[S1][0] as u32, 0x40)); // the sphere itself
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1ab0)); // D_0048E550
        c.g[A0][0] = 0x140;
        c.lq(3, p(c.g[SP][0] as u32, 0x10));
        c.swc1(3, p(c.g[SP][0] as u32, 0x10));
        c.swc1(2, p(c.g[SP][0] as u32, 0x14));
        c.swc1(1, p(c.g[SP][0] as u32, 0x18));
        c.swc1(7, p(c.g[SP][0] as u32, 0x1c));
        c.sq(3, p(c.g[S1][0] as u32, 0x20)); // plus box
        c.lq(2, p(c.g[SP][0] as u32, 0x10));
        c.sq(2, p(c.g[S1][0] as u32, 0x30)); // minus box
        call_ptr(c, 0x0031_7d70, 0x0033_04c0)?; // EA_MemAllocNamed(0x140, tag, 0, 0)
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0032_c508, 0x0033_04cc)?; // SphereTree_ctor(nodes)
        c.sw(p(c.g[S1][0] as u32, 0x68), c.g[S0][0]);
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, p(c.g[SP][0] as u32, 0x40));
        c.lq(17, p(c.g[SP][0] as u32, 0x30));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x20));
        frame.close(c);
        Ok(())
    })
}


/// `void AnimObject_v36(ClampFloat* self, int msg, float v)` (0x341fe8):
/// the clamp object's message handler: 0x64 sets the frame count from the
/// frame rate, 0x65/0x68 set the value/min, 0x66 the speed, 0x67 the max;
/// 0x69 clears the first word; anything else forwards to the base object at
/// `+0x30`.
pub fn anim_object_v36(c: &mut Cpu) {
    'b_342144: {
        'b_342018: {
            c.cycles += 4;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
            c.g[V0][0] = 0x64;
            let t = c.g[A1][0] != c.g[V0][0];
            c.std(SP, 0, c.g[RA][0]);
            if t {
                break 'b_342018;
            } else {
                c.cycles += 8;
                c.g[V0][0] = c.ldw_2(GP, 0x2a74);
                c.lwc1(0, c.pa(V0, 0x10));
                c.cvt_s_w(0, 0);
                c.mul_s(0, 12, 0);
                c.cvt_w_s(1, 0);
                c.swc1(1, c.pa(A0, 0xc));
                let t = true;
                c.g[RA][0] = c.ldd(SP, 0);
                if t {
                    break 'b_342144;
                } else {
                    break 'b_342018;
                }
            }
        }
        'b_342140: {
            'b_342128: {
                'b_342118: {
                    'b_342050: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x65;
                        if c.g[A1][0] != c.g[V0][0] {
                            c.g[V0][0] = 0x66;
                            break 'b_342050;
                        } else {
                            c.cycles += 7;
                            c.lwc1(0, c.pa(GP, -0x2b7c));
                            c.lwc1(1, c.pa(A0, 0x14));
                            c.mul_s(12, 12, 0);
                            c.c_le_s(1, 12);
                            let t = !c.fcc();
                            c.g[RA][0] = c.ldd(SP, 0);
                            if t {
                                break 'b_342144;
                            } else {
                                c.cycles += 4;
                                c.lwc1(0, c.pa(A0, 0x18));
                                c.c_le_s(12, 0);
                                let t = true;

                                if t {
                                    break 'b_342118;
                                } else {
                                    break 'b_342050;
                                }
                            }
                        }
                    }
                    'b_342078: {
                        c.cycles += 2;
                        let t = c.g[A1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x67;
                        if t {
                            break 'b_342078;
                        } else {
                            c.cycles += 8;
                            c.g[V0][0] = c.ldw_2(GP, 0x2a74);
                            c.lwc1(1, c.pa(GP, -0x2b78));
                            c.lwc1(0, c.pa(V0, 0x10));
                            c.cvt_s_w(0, 0);
                            c.mul_s(12, 12, 1);
                            c.div_s(0, 12, 0);
                            let t = true;
                            c.swc1(0, c.pa(A0, 0x10));
                            if t {
                                break 'b_342140;
                            } else {
                                break 'b_342078;
                            }
                        }
                    }
                    'b_3420c8: {
                        c.cycles += 2;
                        let t = c.g[A1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x68;
                        if t {
                            break 'b_3420c8;
                        } else {
                            'b_3420a4: {
                                c.cycles += 9;
                                c.g[V0][0] = c.ldw_2(A0, 0x48);
                                c.lwc1(0, c.pa(GP, -0x2b74));
                                c.g[V1][0] = c.ldw_2(V0, 0x80);
                                c.mul_s(12, 12, 0);
                                c.lwc1(0, c.pa(V1, 0x14));
                                c.c_lt_s(0, 12);
                                if c.fcc() {
                                    c.mov_s(12, 0);
                                    break 'b_3420a4;
                                } else {
                                    break 'b_3420a4;
                                }
                            }
                            'b_3420b8: {
                                c.cycles += 5;
                                c.mtc1(0, 0);
                                c.c_lt_s(12, 0);
                                if c.fcc() {
                                    c.mov_s(12, 0);
                                    break 'b_3420b8;
                                } else {
                                    break 'b_3420b8;
                                }
                            }
                            c.cycles += 4;
                            c.lwc1(0, c.pa(A0, 0x1c));
                            c.c_lt_s(12, 0);
                            let t = true;
                            c.swc1(12, c.pa(A0, 0x18));
                            if t {
                                break 'b_342118;
                            } else {
                                break 'b_3420c8;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x69;
                    if t {
                        break 'b_342128;
                    } else {
                        'b_3420f4: {
                            c.cycles += 9;
                            c.g[V0][0] = c.ldw_2(A0, 0x48);
                            c.lwc1(0, c.pa(GP, -0x2b70));
                            c.g[V1][0] = c.ldw_2(V0, 0x80);
                            c.mul_s(12, 12, 0);
                            c.lwc1(0, c.pa(V1, 0x14));
                            c.c_lt_s(0, 12);
                            if c.fcc() {
                                c.mov_s(12, 0);
                                break 'b_3420f4;
                            } else {
                                break 'b_3420f4;
                            }
                        }
                        'b_342108: {
                            c.cycles += 5;
                            c.mtc1(0, 0);
                            c.c_lt_s(12, 0);
                            if c.fcc() {
                                c.mov_s(12, 0);
                                break 'b_342108;
                            } else {
                                break 'b_342108;
                            }
                        }
                        c.cycles += 4;
                        c.lwc1(0, c.pa(A0, 0x1c));
                        c.swc1(12, c.pa(A0, 0x14));
                        c.c_lt_s(0, 12);
                        break 'b_342118;
                    }
                }
                c.cycles += 2;
                if c.fcc() {
                    c.swc1(12, c.pa(A0, 0x1c));
                    break 'b_342140;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[RA][0] = c.ldd(SP, 0);
                    if t {
                        break 'b_342144;
                    } else {
                        break 'b_342128;
                    }
                }
            }
            'b_342138: {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[V0][0];

                if t {
                    break 'b_342138;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.sth_2(A0, 0, 0u64);
                    if t {
                        break 'b_342140;
                    } else {
                        break 'b_342138;
                    }
                }
            }
            c.cycles += 2;
            c.g[RA][0] = sx(0x00342140);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
            c.pc = 0x00356b08;
            call_indirect(c, 0x00356b08);
            if c.pc != 0x00342140 {
                return;
            }
            break 'b_342140;
        }
        c.cycles += 1;
        c.g[RA][0] = c.ldd(SP, 0);
        break 'b_342144;
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
    return;
}


/// `void* AnimTeeter_ctor(Teeter* self, int a1, void* obj, const Def* def)`
/// (0x3421a0): construct a teeter animation: the base flag-set at `+0x30`,
/// then the rates from the definition (`+4..+0x24`) and the object (`obj +
/// 0x80`), scaled by the frame rate where the original does.
pub fn anim_teeter_ctor(c: &mut Cpu) {
    'b_3421fc: {
        c.cycles += 20;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[T0][0] = 0u64;
        c.sq(16, c.pa(SP, 0x30));
        c.sq(17, c.pa(SP, 0x20));
        c.g[S0][0] = c.g[A2][0];
        c.sq(18, c.pa(SP, 0x10));
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A3][0];
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x30));
        c.g[A3][0] = c.g[S0][0];
        c.std(SP, 0, c.g[RA][0]);
        c.g[RA][0] = sx(0x003421d4);
        c.g[A2][0] = 0x4;
        c.pc = 0x0034d9b0;
        call_indirect(c, 0x0034d9b0);
        if c.pc != 0x003421d4 {
            return;
        }
        c.g[V0][0] = 0x490000;
        c.g[A0][0] = c.ldw_2(S1, 0x78);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8f8));
        c.mtc1(0, 2);
        c.stw_2(S1, 0x50, c.g[V0][0]);
        let t = c.g[A0][0] == 0u64;
        c.g[S0][0] = c.ldw_2(S0, 0x80);
        if t {
            break 'b_3421fc;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003421f8);
            c.pc = 0x00351508;
            call_indirect(c, 0x00351508);
            if c.pc != 0x003421f8 {
                return;
            }
            c.mov_s(2, 0);
            break 'b_3421fc;
        }
    }
    'b_342224: {
        'b_342220: {
            c.cycles += 6;
            c.lwc1(1, c.pa(S2, 0x20));
            c.mtc1(0, 0);
            c.c_le_s(0, 1);
            let t = !c.fcc();
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
            if t {
                break 'b_342220;
            } else {
                c.cycles += 3;
                c.lwc1(0, c.pa(GP, -0x2b6c));
                let t = true;
                c.mul_s(0, 1, 0);
                if t {
                    break 'b_342224;
                } else {
                    break 'b_342220;
                }
            }
        }
        c.cycles += 1;
        c.mov_s(0, 2);
        break 'b_342224;
    }
    'b_342250: {
        'b_342248: {
            c.cycles += 7;
            c.swc1(0, c.pa(V0, 0));
            c.mtc1(0, 0);
            c.lwc1(1, c.pa(S2, 0x24));
            c.c_lt_s(1, 0);
            let t = !c.fcc();
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
            if t {
                break 'b_342248;
            } else {
                c.cycles += 2;
                let t = true;
                c.lwc1(0, c.pa(S0, 0x14));
                if t {
                    break 'b_342250;
                } else {
                    break 'b_342248;
                }
            }
        }
        c.cycles += 2;
        c.lwc1(0, c.pa(GP, -0x2b68));
        c.mul_s(0, 1, 0);
        break 'b_342250;
    }
    c.cycles += 37;
    c.swc1(0, c.pa(V0, 0));
    c.g[V1][0] = c.ldw_2(GP, 0x2a74);
    c.g[V0][0] = c.g[S1][0];
    c.lwc1(0, c.pa(S2, 0x8));
    c.lwc1(2, c.pa(GP, -0x2b64));
    c.swc1(0, c.pa(S1, 0xc));
    c.lwc1(3, c.pa(GP, -0x2b60));
    c.lwc1(1, c.pa(S2, 0x4));
    c.g[A0][0] = c.ldhu_2(S1, 0x56);
    c.mul_s(1, 1, 2);
    c.lq(16, c.pa(SP, 0x30));
    c.g[A0][0] = c.g[A0][0] | 0x1;
    c.g[RA][0] = c.ldd(SP, 0);
    c.swc1(1, c.pa(S1, 0x8));
    c.lwc1(0, c.pa(S2, 0xc));
    c.lwc1(1, c.pa(V1, 0x10));
    c.cvt_s_w(1, 1);
    c.mul_s(0, 0, 3);
    c.div_s(0, 0, 1);
    c.swc1(0, c.pa(S1, 0x18));
    c.g[V1][0] = c.ldw_2(S2, 0x1c);
    c.stw_2(S1, 0x2c, c.g[V1][0]);
    c.lwc1(0, c.pa(S2, 0x14));
    c.mul_s(0, 0, 3);
    c.swc1(0, c.pa(S1, 0x24));
    c.lwc1(1, c.pa(S2, 0x18));
    c.swc1(1, c.pa(S1, 0x1c));
    c.lwc1(0, c.pa(S2, 0x10));
    c.sth_2(S1, 0x56, c.g[A0][0]);
    c.swc1(0, c.pa(S1, 0x20));
    c.stw_2(S1, 0, 0u64);
    c.stw_2(S1, 0x4, 0u64);
    c.stw_2(S1, 0x28, 0u64);
    c.lq(17, c.pa(SP, 0x20));
    c.lq(18, c.pa(SP, 0x10));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
    return;
}


// ---------------------------------------------------------------------------
// 0x00330540 CollSphereTree_OverlapsAABB
// ---------------------------------------------------------------------------

/// `int CollSphereTree_OverlapsAABB(CollSphereTree* self, const Vec3* lo,
/// const Vec3* hi)` (0x330540).
///
/// Builds the tree's world box (centre at `self->[0x60] + 0x80` grown by the
/// three scales the object reports) and tests the two corners against it, one
/// axis at a time; returns 1 when the box overlaps.
pub fn coll_sphere_tree_overlaps_aabb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x90);
        frame.save(c, 16, 0x70);
        frame.save(c, 17, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x50);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 21, 0x88);
        c.g[S2][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x80);
        frame.save_ra(c, 0x40);
        let obj = c.g[S0][0] as u32;
        let lo = c.g[S1][0] as u32;
        let hi = c.g[S2][0] as u32;
        c.g[RA][0] = sx(0x0033_0570);
        c.g[A0][0] = c.lw(p(obj, 0x60));
        call_ptr(c, 0x0032_c590, 0x0033_0570)?;
        c.g[A0][0] = c.lw(p(obj, 0x60));
        c.g[RA][0] = sx(0x0033_057c);
        c.mov_s(21, 0);
        call_ptr(c, 0x0032_c590, 0x0033_057c)?;
        c.g[A0][0] = c.lw(p(obj, 0x60));
        c.g[RA][0] = sx(0x0033_0588);
        c.mov_s(20, 0);
        call_ptr(c, 0x0032_c590, 0x0033_0588)?;
        c.swc1(21, c.g[SP][0] as u32);
        c.swc1(20, p(c.g[SP][0] as u32, 4));
        c.swc1(0, p(c.g[SP][0] as u32, 8));
        c.sw(p(c.g[SP][0] as u32, 0xc), 0);

        // centre + extent
        c.g[V0][0] = c.lw(p(obj, 0x60));
        c.lqc2(3, p(c.g[V0][0] as u32, 0x80));
        c.lqc2(4, c.g[SP][0] as u32);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(c.g[SP][0] as u32, 0x20));
        c.lq(3, p(c.g[SP][0] as u32, 0x20));
        c.g[V0][0] = c.lw(p(obj, 0x60));
        c.sq(3, p(c.g[SP][0] as u32, 0x10));
        // centre - extent
        c.lqc2(3, p(c.g[V0][0] as u32, 0x80));
        c.lqc2(4, c.g[SP][0] as u32);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(c.g[SP][0] as u32, 0x30));

        'ret: {
            c.lwc1(1, lo);
            c.lwc1(2, p(c.g[SP][0] as u32, 0x10));
            c.lq(2, p(c.g[SP][0] as u32, 0x30));
            c.c_lt_s(2, 1);
            let no_lo = !c.fcc();
            c.sq(2, p(c.g[SP][0] as u32, 0x20));
            if no_lo {
                // X low edge not separated.
            } else {
                c.cycles += 5;
                c.lwc1(0, p(c.g[SP][0] as u32, 0x20));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.g[V0][0] = 0;
                    break 'ret;
                }
            }
            'x_hi: {
                c.cycles += 5;
                c.lwc1(1, hi);
                c.c_lt_s(1, 2);
                let no_hi = !c.fcc();
                c.lwc1(0, p(c.g[SP][0] as u32, 0x20));
                if no_hi {
                    break 'x_hi;
                }
                c.cycles += 4;
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.g[V0][0] = 0;
                    break 'ret;
                }
            }
            'y_lo: {
                c.cycles += 6;
                c.lwc1(1, p(lo, 4));
                c.lwc1(2, p(c.g[SP][0] as u32, 0x14));
                c.c_lt_s(2, 1);
                let no_lo = !c.fcc();
                c.lwc1(0, p(c.g[SP][0] as u32, 0x24));
                if no_lo {
                    break 'y_lo;
                }
                c.cycles += 4;
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.g[V0][0] = 0;
                    break 'ret;
                }
            }
            'y_hi: {
                c.cycles += 5;
                c.lwc1(1, p(hi, 4));
                c.c_lt_s(1, 2);
                let no_hi = !c.fcc();
                c.lwc1(0, p(c.g[SP][0] as u32, 0x24));
                if no_hi {
                    break 'y_hi;
                }
                c.cycles += 4;
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.g[V0][0] = 0;
                    break 'ret;
                }
            }
            'z_lo: {
                c.cycles += 6;
                c.lwc1(1, p(lo, 8));
                c.lwc1(2, p(c.g[SP][0] as u32, 0x18));
                c.c_lt_s(2, 1);
                let no_lo = !c.fcc();
                c.lwc1(0, p(c.g[SP][0] as u32, 0x28));
                if no_lo {
                    break 'z_lo;
                }
                c.cycles += 4;
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.g[V0][0] = 0;
                    break 'ret;
                }
            }
            'z_hi: {
                c.cycles += 5;
                c.lwc1(1, p(hi, 8));
                c.c_lt_s(1, 2);
                let no_hi = !c.fcc();
                c.lwc1(0, p(c.g[SP][0] as u32, 0x28));
                if no_hi {
                    break 'z_hi;
                }
                c.cycles += 4;
                c.c_lt_s(1, 0);
                let fail = c.fcc();
                c.g[V0][0] = 0;
                if fail {
                    break 'ret;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }

        c.cycles += 8;
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}


/// `void BoostModifier_v02(void* mod)` (0x342880): the boost modifier's
/// update: step the boost meter for the current rider (gain while boosting,
/// decay otherwise), and drive the effect's fade and sound.
pub fn boost_modifier_v02(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x00342880 => {
                c.cycles += 12;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x60));
                sq(c, 17, at(c, SP, 0x30));
                sq(c, 16, at(c, SP, 0x40));
                c.g[S1][0] = c.g[A0][0];
                sq(c, 18, at(c, SP, 0x20));
                sq(c, 19, at(c, SP, 0x10));
                sd(c, r32(c, SP), c.g[RA][0]);
                c.swc1(20, at(c, SP, 0x50));
                sw_2(c, r32(c, S1), 0u64);
                c.g[V0][0] = lw(c, at(c, S1, 0x30));

                if c.g[V0][0] != 0u64 {
                    c.g[V0][0] = lw(c, at(c, S1, 0x2c));
                    pc = 0x003428bc;
                    continue;
                }
                pc = 0x003428b0;
                continue;
            }
            0x003428b0 => {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                sw_2(c, r32(c, S1), c.g[V0][0]);
                c.g[V0][0] = lw(c, at(c, S1, 0x2c));
                pc = 0x003428bc;
                continue;
            }
            0x003428bc => {
                c.cycles += 2;
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                if t {
                    pc = 0x003428d4;
                    continue;
                }
                pc = 0x003428c4;
                continue;
            }
            0x003428c4 => {
                c.cycles += 2;
                let t = (c.g[V0][0] as i64) <= 0;
                sw_2(c, at(c, S1, 0x2c), c.g[V0][0]);
                if t {
                    pc = 0x003428d4;
                    continue;
                }
                pc = 0x003428cc;
                continue;
            }
            0x003428cc => {
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                sw_2(c, r32(c, S1), c.g[V0][0]);
                pc = 0x003428d4;
                continue;
            }
            0x003428d4 => {
                c.cycles += 9;
                c.g[V0][0] = lw(c, at(c, S1, 0x4));
                c.g[A0][0] = lh_2(c, at(c, V0, 0x48));
                c.g[V1][0] = lw(c, at(c, V0, 0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x003428e8);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x003428e8).is_err() {
                return;
                }
                c.g[A0][0] = lw(c, at(c, S1, 0x30));
                c.g[V0][0] = 0x1;
                let t = c.g[A0][0] == c.g[V0][0];

                if t {
                    pc = 0x00342904;
                    continue;
                }
                pc = 0x003428f8;
                continue;
            }
            0x003428f8 => {
                c.cycles += 3;
                c.g[V0][0] = lw(c, at(c, S1, 0x2c));

                if (c.g[V0][0] as i64) > 0 {
                    c.g[V0][0] = lw(c, at(c, S1, 0x4));
                    pc = 0x003429ac;
                    continue;
                }
                pc = 0x00342904;
                continue;
            }
            0x00342904 => {
                c.cycles += 4;
                c.mtc1(0, 20);
                c.g[S2][0] = 0u64;
                let t = true;
                c.g[S3][0] = 0x1;
                if t {
                    pc = 0x00342994;
                    continue;
                }
                pc = 0x00342914;
                continue;
            }
            0x00342914 => {
                c.cycles += 1;
                pc = 0x00342918;
                continue;
            }
            0x00342918 => {
                c.cycles += 11;
                c.g[RA][0] = sx(0x00342920);
                c.g[A0][0] = c.g[S2][0];
                if call_ptr(c, 0x002d1b08, 0x00342920).is_err() {
                return;
}
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = lw(c, at(c, S1, 0x40));
                c.g[V1][0] = lw(c, r32(c, S0));
                c.g[A0][0] = lh_2(c, at(c, V1, 0x18));
                c.g[V0][0] = lw(c, at(c, V1, 0x1c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0034293c);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x0034293c).is_err() {
                return;
                }

                if c.g[V0][0] == 0u64 {
                    c.g[S2][0] = sx(at(c, S2, 0x1));
                    pc = 0x00342994;
                    continue;
                }
                pc = 0x00342944;
                continue;
            }
            0x00342944 => {
                c.cycles += 5;
                c.lwc1(0, at(c, S1, 0x24));
                c.c_lt_s(0, 20);
                let t = !c.fcc();
                sw_2(c, r32(c, S1), c.g[S3][0]);
                if t {
                    pc = 0x00342978;
                    continue;
                }
                pc = 0x00342958;
                continue;
            }
            0x00342958 => {
                c.cycles += 8;
                c.g[V0][0] = lw(c, at(c, S1, 0x4));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = lh_2(c, at(c, V0, 0x40));
                c.g[V1][0] = lw(c, at(c, V0, 0x44));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00342970);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00342970).is_err() {
                return;
                }
                let t = true;
                c.g[S2][0] = sx(at(c, S2, 0x1));
                if t {
                    pc = 0x00342994;
                    continue;
                }
                pc = 0x00342978;
                continue;
            }
            0x00342978 => {
                c.cycles += 7;
                c.g[V0][0] = lw(c, at(c, S1, 0x4));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = lh_2(c, at(c, V0, 0x38));
                c.g[V1][0] = lw(c, at(c, V0, 0x3c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00342990);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00342990).is_err() {
                return;
                }
                c.g[S2][0] = sx(at(c, S2, 0x1));
                pc = 0x00342994;
                continue;
            }
            0x00342994 => {
                c.cycles += 5;
                c.g[RA][0] = sx(0x0034299c);
                if call_ptr(c, 0x002d1af0, 0x0034299c).is_err() {
                return;
}
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;

                if t {
                    if back_edge_unwinds(c, 0x00342918) {
                        return;
                    }
                    pc = 0x00342918;
                    continue;
                }
                pc = 0x003429a8;
                continue;
            }
            0x003429a8 => {
                c.cycles += 1;
                c.g[V0][0] = lw(c, at(c, S1, 0x4));
                pc = 0x003429ac;
                continue;
            }
            0x003429ac => {
                c.cycles += 12;
                c.g[A0][0] = lh_2(c, at(c, V0, 0x50));
                c.g[V1][0] = lw(c, at(c, V0, 0x54));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x003429bc);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x003429bc).is_err() {
                return;
                }
                lq(c, 16, at(c, SP, 0x40));
                lq(c, 17, at(c, SP, 0x30));
                lq(c, 18, at(c, SP, 0x20));
                lq(c, 19, at(c, SP, 0x10));
                c.g[RA][0] = ld(c, r32(c, SP));
                c.lwc1(20, at(c, SP, 0x50));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at(c, SP, 0x60));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x00342880, 0x003429dc);
                return;
            }
        }
    }
}


/// `void Boost_Update(void* boost)` (0x3415d0): per-frame boost update: read
/// the rider, advance the boost timers and meter, and spawn/refresh the
/// boost effect.
///
/// Tick rate: the window `+0x3c` counts 60 Hz frames
/// ([`crate::tick::frame60_due`] keyed by its address); everything else runs
/// per tick.
pub fn boost_update(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003415d0 => {
                c.cycles += 16;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x70));
                sq(c, 16, at(c, SP, 0x50));
                sq(c, 17, at(c, SP, 0x40));
                c.g[S0][0] = 0x1;
                sq(c, 18, at(c, SP, 0x30));
                c.g[S1][0] = c.g[A0][0];
                sq(c, 19, at(c, SP, 0x20));
                c.g[S2][0] = c.g[S0][0];
                sq(c, 20, at(c, SP, 0x10));
                sd(c, r32(c, SP), c.g[RA][0]);
                c.swc1(20, at(c, SP, 0x60));
                c.g[V0][0] = lw(c, at(c, S1, 0x40));
                c.g[V1][0] = lw(c, at(c, S1, 0x3c));
                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                let t = (c.g[V1][0] as i64) <= 0;
                if c.g[V0][0] == 0 {
                    c.g[S2][0] = 0u64;
                }
                if t {
                    pc = 0x00341620;
                    continue;
                }
                pc = 0x00341610;
                continue;
            }
            0x00341610 => {
                c.cycles += 4;
                // The boost window `+0x3c` (set from the zone's data, a
                // count of 60 Hz frames, by Boost_HandleEvent) goes down
                // once per 60 Hz frame of its own: above 60 Hz the ticks
                // between frames keep it (always taken down at 60).
                let window = at(c, S1, 0x3c);
                let left = r32(c, V1);
                c.g[V0][0] = if crate::tick::frame60_due(c, window, left) {
                    sx(left.wrapping_sub(0x1))
                } else {
                    c.g[V1][0]
                };
                let now = c.g[V0][0] as u32;
                crate::tick::frame60_ran(c, window, now);
                c.g[V1][0] = ((c.g[V0][0] as i64) < 1i64) as u64;
                sw_2(c, at(c, S1, 0x3c), c.g[V0][0]);
                if c.g[V1][0] == 0 {
                    c.g[S2][0] = c.g[S0][0];
                }
                pc = 0x00341620;
                continue;
            }
            0x00341620 => {
                c.cycles += 8;
                c.g[V0][0] = lw(c, at(c, S1, 0xc));
                c.g[A0][0] = lh_2(c, at(c, V0, 0x188));
                c.g[V1][0] = lw(c, at(c, V0, 0x18c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00341634);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00341634).is_err() {
                return;
                }
                c.g[V0][0] = lw(c, at(c, S1, 0x40));
                let t = c.g[V0][0] == c.g[S0][0];

                if t {
                    pc = 0x0034164c;
                    continue;
                }
                pc = 0x00341640;
                continue;
            }
            0x00341640 => {
                c.cycles += 3;
                c.g[V0][0] = lw(c, at(c, S1, 0x3c));

                if (c.g[V0][0] as i64) > 0 {
                    c.g[V0][0] = lw(c, at(c, S1, 0xc));
                    pc = 0x00341718;
                    continue;
                }
                pc = 0x0034164c;
                continue;
            }
            0x0034164c => {
                c.cycles += 4;
                c.mtc1(0, 20);
                c.g[S3][0] = 0u64;
                let t = true;
                c.g[S4][0] = 0x2;
                if t {
                    pc = 0x00341700;
                    continue;
                }
                pc = 0x0034165c;
                continue;
            }
            0x0034165c => {
                c.cycles += 1;
                pc = 0x00341660;
                continue;
            }
            0x00341660 => {
                c.cycles += 6;
                c.g[RA][0] = sx(0x00341668);
                c.g[A0][0] = c.g[S3][0];
                if call_ptr(c, 0x002d1b08, 0x00341668).is_err() {
                return;
}
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = lw(c, at(c, S1, 0x40));
                let t = c.g[V0][0] != c.g[S4][0];
                c.g[V1][0] = lw(c, r32(c, S0));
                if t {
                    pc = 0x00341694;
                    continue;
                }
                pc = 0x00341678;
                continue;
            }
            0x00341678 => {
                c.cycles += 6;
                c.g[A0][0] = lh_2(c, at(c, V1, 0x48));
                c.g[V0][0] = lw(c, at(c, V1, 0x4c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00341688);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00341688).is_err() {
                return;
                }

                if c.g[V0][0] != 0u64 {
                    c.g[S3][0] = sx(at(c, S3, 0x1));
                    pc = 0x00341700;
                    continue;
                }
                pc = 0x00341690;
                continue;
            }
            0x00341690 => {
                c.cycles += 1;
                c.g[V1][0] = lw(c, r32(c, S0));
                pc = 0x00341694;
                continue;
            }
            0x00341694 => {
                c.cycles += 7;
                c.g[A1][0] = lw(c, at(c, S1, 0x18));
                c.g[A0][0] = lh_2(c, at(c, V1, 0x18));
                c.g[V0][0] = lw(c, at(c, V1, 0x1c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003416a8);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x003416a8).is_err() {
                return;
                }

                if c.g[V0][0] == 0u64 {
                    c.g[S3][0] = sx(at(c, S3, 0x1));
                    pc = 0x00341700;
                    continue;
                }
                pc = 0x003416b0;
                continue;
            }
            0x003416b0 => {
                c.cycles += 5;
                c.lwc1(0, at(c, S1, 0x34));
                c.c_lt_s(0, 20);
                let t = !c.fcc();
                c.g[S2][0] = 0x1;
                if t {
                    pc = 0x003416e4;
                    continue;
                }
                pc = 0x003416c4;
                continue;
            }
            0x003416c4 => {
                c.cycles += 8;
                c.g[V0][0] = lw(c, at(c, S1, 0xc));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = lh_2(c, at(c, V0, 0x180));
                c.g[V1][0] = lw(c, at(c, V0, 0x184));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x003416dc);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x003416dc).is_err() {
                return;
                }
                let t = true;
                c.g[S3][0] = sx(at(c, S3, 0x1));
                if t {
                    pc = 0x00341700;
                    continue;
                }
                pc = 0x003416e4;
                continue;
            }
            0x003416e4 => {
                c.cycles += 7;
                c.g[V0][0] = lw(c, at(c, S1, 0xc));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = lh_2(c, at(c, V0, 0x178));
                c.g[V1][0] = lw(c, at(c, V0, 0x17c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x003416fc);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x003416fc).is_err() {
                return;
                }
                c.g[S3][0] = sx(at(c, S3, 0x1));
                pc = 0x00341700;
                continue;
            }
            0x00341700 => {
                c.cycles += 5;
                c.g[RA][0] = sx(0x00341708);
                if call_ptr(c, 0x002d1af0, 0x00341708).is_err() {
                return;
}
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;

                if t {
                    if back_edge_unwinds(c, 0x00341660) {
                        return;
                    }
                    pc = 0x00341660;
                    continue;
                }
                pc = 0x00341714;
                continue;
            }
            0x00341714 => {
                c.cycles += 1;
                c.g[V0][0] = lw(c, at(c, S1, 0xc));
                pc = 0x00341718;
                continue;
            }
            0x00341718 => {
                c.cycles += 6;
                c.g[A0][0] = lh_2(c, at(c, V0, 0x190));
                c.g[V1][0] = lw(c, at(c, V0, 0x194));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00341728);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00341728).is_err() {
                return;
                }
                let t = c.g[S2][0] != 0u64;
                lq(c, 16, at(c, SP, 0x50));
                if t {
                    pc = 0x0034174c;
                    continue;
                }
                pc = 0x00341730;
                continue;
            }
            0x00341730 => {
                c.cycles += 7;
                c.g[V0][0] = lw(c, at(c, S1, 0xc));
                c.g[A1][0] = 0x1;
                c.g[A0][0] = lh_2(c, at(c, V0, 0x110));
                c.g[V1][0] = lw(c, at(c, V0, 0x114));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00341748);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00341748).is_err() {
                return;
                }
                lq(c, 16, at(c, SP, 0x50));
                pc = 0x0034174c;
                continue;
            }
            0x0034174c => {
                c.cycles += 8;
                lq(c, 17, at(c, SP, 0x40));
                lq(c, 18, at(c, SP, 0x30));
                lq(c, 19, at(c, SP, 0x20));
                lq(c, 20, at(c, SP, 0x10));
                c.g[RA][0] = ld(c, r32(c, SP));
                c.lwc1(20, at(c, SP, 0x60));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at(c, SP, 0x70));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003415d0, 0x0034176c);
                return;
            }
        }
    }
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
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fakes::covered;
    use crate::difftest::put_vec;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::fakes::*;
    use crate::difftest::SP as STACK;
    use crate::difftest::fakes;
    use crate::difftest::compare;
    pub(crate) use crate::game::be::beintmission::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::worldpainter::quadtree::tests::put_vtable;
    pub(crate) use crate::game::camera::trigger::cameratriggerlist::tests::small_pointer;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_5;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::MEM_HEAP;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA_SIZE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::run;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_init;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::alloc;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festateoptions::tests::rng;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::bx::bxstring2::tests::fbits;
    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS as SEEDS_6;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup as setup_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_4 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_4 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_7 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_8;
    pub(crate) use crate::game::main::gamemode::tests::h;
    pub(crate) use crate::game::main::gamemode::tests::be_library;
    pub(crate) use crate::game::main::gamemode::tests::slot;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_9;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_3;
    pub(crate) use crate::game::intersect::riderspheretree::tests::coll_args;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD;
    pub(crate) use crate::game::ai::ai::tests::wf;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_4;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::ai::ai::tests::generic;
    pub(crate) use crate::game::ai::ai::tests::globals_8 as globals_3;
    pub(crate) use crate::game::util::menu::tests::wf_2;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_2;
    pub(crate) use crate::game::fe::fepopup::tests::AUX as AUX_2;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_5;


    /// Setup for the three `AnimObject_Step*` functions: random edge floats on
    /// the parameter, bounds and rate fields.
    pub(crate) fn anim_step_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = Rng(salt ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        for off in [0x10u32, 0x14, 0x18, 0x1c] {
            c.bus.write32(DATA + off, r.edge_f32());
        }
        c.bus.write32(DATA + 8, r.next_u32());
    }


    #[test]
    fn anim_object_step_loop_matches() {
        check(ssx3_ee::s11::f_00341e48, anim_object_step_loop, 300, &|c, seed| {
            anim_step_setup(c, seed, 0x341e48);
        });
    }


    #[test]
    fn anim_object_step_ping_pong_matches() {
        check(ssx3_ee::s11::f_00341ec0, anim_object_step_ping_pong, 300, &|c, seed| {
            anim_step_setup(c, seed, 0x341ec0);
        });
    }


    #[test]
    fn anim_object_step_clamp_matches() {
        check(ssx3_ee::s11::f_00341f38, anim_object_step_clamp, 300, &|c, seed| {
            anim_step_setup(c, seed, 0x341f38);
        });
    }


    #[test]
    fn boost_modifier_ctor_matches() {
        check(ssx3_ee::s11::f_00342768, boost_modifier_ctor, 200, &|c, seed| {
            let mut r = Rng(0x1010 ^ seed as u64);
            scramble(c, &mut r);
            let desc = DATA + 0x100;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = desc as u64;
            c.g[A2][0] = (DATA as u64 + 0x200) | ((r.next_u32() as u64) << 32);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP + 0x2a74, DATA + 0x300);
            c.bus.write32(DATA + 0x300 + 0x10, r.next_u32());
            c.bus.write16(desc + 4, (r.next_u32() & 0xfffe) as u16 | (seed as u16 & 1));
            for off in [8u32, 0xc, 0x10, 0x14, 0x18, 0x1c] {
                c.bus.write32(desc + off, r.edge_f32());
            }
        });
    }


    pub(crate) const SEEDS: u32 = 250;


    pub(crate) fn f(x: f32) -> u32 {
        x.to_bits()
    }


    /// A float in [lo, hi).
    pub(crate) fn uf(r: &mut Rng, lo: f32, hi: f32) -> f32 {
        lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)
    }


    /// Sub-object vtable: 0x130/0x138 (`Collide_VolumeObject` delegate),
    /// 0x160 (active), 0x168 (box); volume vtable: 0x20 (triangle test);
    /// painter vtable: 0x210, 0x220, 0x228.
    pub(crate) const VT_SUB: u32 = DATA + 0x3_0000;

    pub(crate) const VT_VOL: u32 = DATA + 0x3_0400;

    pub(crate) const VT_PAINTER: u32 = DATA + 0x3_0600;

    /// The box the fake 0x168 returns.
    pub(crate) const BOX2: u32 = DATA + 0x3_0800;


    pub(crate) fn put_vtables(c: &mut Cpu, r: &mut Rng) {
        for (i, off) in [0x130u32, 0x138, 0x160, 0x168].into_iter().enumerate() {
            c.bus.write16(VT_SUB + off, 0);
            c.bus.write32(VT_SUB + off + 4, FAKE + 0x10 * i as u32);
        }
        c.bus.write16(VT_VOL + 0x20, 0);
        c.bus.write32(VT_VOL + 0x24, FAKE + 0x40);
        for (i, off) in [0x210u32, 0x220, 0x228].into_iter().enumerate() {
            c.bus.write16(VT_PAINTER + off, 0);
            c.bus.write32(VT_PAINTER + off + 4, FAKE + 0x50 + 0x10 * i as u32);
        }
        // BOX2: min at +0, max at +0x10.
        for k in 0..3 {
            let lo = uf(r, -5.0, 5.0);
            c.bus.write32(BOX2 + 4 * k, f(lo));
            c.bus.write32(BOX2 + 0x10 + 4 * k, f(lo + uf(r, 0.0, 3.0)));
        }
    }


    /// `fakes::fake_call`, then a result that depends on the method.
    pub(crate) fn fake_varied(c: &mut Cpu, target: u32) {
        fake_call(c, target);
        let h = c.g[V1][0];
        match target.wrapping_sub(FAKE) {
            0x00 => {}                                              // delegate (non-null)
            0x10 => c.g[V0][0] = h & 3,                             // contacts added
            0x20 => c.g[V0][0] = ((h >> 8) & 3 != 0) as u64,        // active
            0x30 => c.g[V0][0] = BOX2 as u64,                       // box
            0x40 | 0x60 => c.g[V0][0] = ((h >> 4) & 3 != 0) as u64, // hit / entered
            _ => {}
        }
    }


    pub(crate) fn setup_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.hooks.miss = fake_varied;
        put_vtables(c, &mut r);
        r
    }


    /// A box at `at`: min at `+min`, max at `+max`, random in ±5.
    pub(crate) fn put_box(c: &mut Cpu, r: &mut Rng, at: u32, min: u32, max: u32) {
        for k in 0..3 {
            let lo = uf(r, -5.0, 5.0);
            c.bus.write32(at + min + 4 * k, f(lo));
            c.bus.write32(at + max + 4 * k, f(lo + uf(r, 0.0, 3.0)));
        }
    }


    /// A collision volume at `vol` (box max +0x20, min +0x30 around the
    /// origin, so the zero cells of the cache entry overlap it).
    pub(crate) fn put_volume(c: &mut Cpu, r: &mut Rng, vol: u32) {
        c.bus.write32(vol, 1);
        c.bus.write32(vol + 8, 0);
        for k in 0..3 {
            c.bus.write32(vol + 0x20 + 4 * k, f(uf(r, 1.0, 3.0)));
            c.bus.write32(vol + 0x30 + 4 * k, f(-uf(r, 1.0, 3.0)));
        }
        c.bus.write32(vol + 0x50, VT_VOL);
    }


    pub(crate) const KEY: u32 = 7;


    /// A forward-difference cache at `cache` whose one bucket holds an entry
    /// for `KEY`: cell 0 has a non-sentinel normal, so `CollideEntry`
    /// calls the volume's triangle test once.
    pub(crate) fn put_cache(c: &mut Cpu, cache: u32) {
        let bucket = cache + 0x80;
        let entry = DATA + 0x1_0000;
        c.bus.write32(cache + 4, entry);
        c.bus.write32(cache + 8, bucket);
        c.bus.write32(cache + 0xc, 1);
        c.bus.write32(bucket, entry);
        c.bus.write32(entry, KEY);
        c.bus.write32(entry + 4, entry);
        c.bus.write32(entry + 0x650, f(1.0));
        // CollideEntry's normal sentinel (cells with it are recomputed).
        c.bus.write32(GAME_GP.wrapping_sub(0x2d94), f(-99.0));
    }


    /// A collidable instance at `inst`.
    pub(crate) fn put_instance(c: &mut Cpu, r: &mut Rng, inst: u32, seed: u32) {
        let flags = if seed % 7 == 0 { 0x01 } else { 0x41 | (r.next_u32() & 0xff00) };
        c.bus.write16(inst + 0xa, flags as u16);
        c.bus.write16(inst + 8, r.next_u32() as u16);
        c.bus.write32(inst + 0x150, KEY);
        put_box(c, r, inst, 0x158, 0x164);
    }


    /// A world object at `o` (flags, sub-object with `VT_SUB`, shape).
    pub(crate) fn put_object(c: &mut Cpu, r: &mut Rng, o: u32) {
        let flags = [0x20u32, 0x40, 0x60, 0, 0x21][(r.next_u32() % 5) as usize];
        c.bus.write32(o + 8, flags);
        let sub = o + 0x90;
        c.bus.write32(o + 0xc, if r.next_u32() % 6 == 0 { 0 } else { sub });
        c.bus.write32(sub + 0xc, VT_SUB);
        // Shape record whose type (0) makes Collide_VolumeObject return 0
        // when it has no delegate.
        c.bus.write32(o + 0x88, if r.next_u32() % 6 == 0 { 0 } else { o + 0xc0 });
        put_box(c, r, o, 0x60, 0x6c);
    }


    #[test]
    fn cell_gather_objects_matches() {
        check(ee::s11::f_003309d8__generated, cell_gather_objects, SEEDS, &|c, seed| {
            let mut r = setup_machine(c, seed, 0x3309, 4);
            let (cell, q, qbox, out, count) = (DATA, DATA + 0x1000, DATA + 0x1200, DATA + 0x2000, DATA + 0x1100);
            c.bus.write32(q, out);
            c.bus.write32(q + 4, count);
            c.bus.write32(q + 0xc, qbox);
            c.bus.write32(count, r.next_u32() % 4);
            put_box(c, &mut r, qbox, 0, 0x10);
            let mut next = 0;
            for i in (0..seed % 4).rev() {
                let o = DATA + 0x4000 + 0x100 * i;
                put_object(c, &mut r, o);
                c.bus.write32(o, next);
                next = o;
            }
            c.bus.write32(cell + 0x20, next);
            c.g[A0][0] = sx(q);
            c.g[A1][0] = sx(cell);
        });
    }


    #[test]
    fn cell_collide_instance_matches() {
        check(ee::s11::f_00335960__generated, cell_collide_instance, SEEDS, &|c, seed| {
            let mut r = setup_machine(c, seed, 0x5960, 3);
            let (q, inst, vol, cache, count) = (DATA, DATA + 0x400, DATA + 0x800, DATA + 0xc00, DATA + 0xd00);
            put_volume(c, &mut r, vol);
            put_cache(c, cache);
            put_instance(c, &mut r, inst, seed);
            c.bus.write32(q, vol);
            c.bus.write32(q + 4, DATA + 0x2_0000);
            c.bus.write32(q + 0xc, count);
            c.bus.write32(q + 0x10, 0);
            c.bus.write32(q + 0x14, cache);
            c.bus.write32(count, r.next_u32() % 4);
            c.g[A0][0] = sx(q);
            c.g[A1][0] = sx(inst);
        });
    }


    #[test]
    fn cell_collide_object_matches() {
        check(ee::s11::f_00335b90__generated, cell_collide_object, SEEDS, &|c, seed| {
            let mut r = setup_machine(c, seed, 0x5b90, 4);
            let (q, o, vol, count) = (DATA, DATA + 0x400, DATA + 0x800, DATA + 0xd00);
            put_volume(c, &mut r, vol);
            put_object(c, &mut r, o);
            c.bus.write32(q, vol);
            c.bus.write32(q + 4, DATA + 0x2_0000);
            c.bus.write32(q + 8, 8 + r.next_u32() % 8);
            c.bus.write32(q + 0xc, count);
            c.bus.write32(count, r.next_u32() % 4);
            c.g[A0][0] = sx(q);
            c.g[A1][0] = sx(o);
        });
    }


    #[test]
    fn overlap_list_collide_volume_matches() {
        check(ee::s11::f_00333ef8__generated, overlap_list_collide_volume, SEEDS, &|c, seed| {
            let mut r = setup_machine(c, seed, 0x3ef8, 6);
            let (list, world, vol, cache) = (DATA, DATA + 0x200, DATA + 0x800, DATA + 0xc00);
            put_volume(c, &mut r, vol);
            put_cache(c, cache);
            c.bus.write32(list, world);
            c.bus.write32(world + 0xa4, cache);
            let instances = seed % 3;
            c.bus.write32(list + 0x10c, instances);
            for i in 0..instances {
                let inst = DATA + 0x4000 + 0x200 * i;
                put_instance(c, &mut r, inst, seed + i);
                c.bus.write32(list + 0x110 + 4 * i, inst);
            }
            let objects = (seed / 3) % 3;
            c.bus.write32(list + 8, objects);
            for i in 0..objects {
                let o = DATA + 0x5000 + 0x100 * i;
                put_object(c, &mut r, o);
                c.bus.write32(list + 0xc + 4 * i, o);
            }
            c.g[A0][0] = sx(list);
            c.g[A1][0] = sx(vol);
            c.g[A2][0] = sx(DATA + 0x2_0000);
            c.g[A3][0] = sx(r.next_u32() % 8);
            c.g[T0][0] = 0;
        });
    }


    pub(crate) const SEEDS_2: u32 = 200;


    pub(crate) fn put_v(c: &mut Cpu, at: u32, v: [f32; 4]) {
        for (k, x) in v.iter().enumerate() {
            c.bus.write32(at + 4 * k as u32, f(*x));
        }
    }


    #[test]
    fn overlap_list_collide_objects_matches() {
        const LIST: u32 = DATA + 0x1000;
        const Q: u32 = DATA + 0x1200;
        const BUF: u32 = DATA + 0x1400;
        const OBJ: u32 = DATA + 0x1c00;
        const VT: u32 = DATA + 0x1800;
        const BOX: u32 = DATA + 0x1a00;
        const NODES: u32 = DATA + 0x2000;
        check(
            ssx3_ee::s11::f_00334458__generated,
            overlap_list_collide_objects,
            SEEDS_2,
            &|c, seed| {
                let _r = machine(c, seed, 0x4006, 0);
                // The object's methods: "should test" (0x160), box (0x168), and
                // the collider's own first query (0x130, answered "no").
                c.hooks.miss = |c: &mut Cpu, target: u32| {
                    fake_call(c, target);
                    match target.wrapping_sub(FAKE) {
                        0x20 => c.g[V0][0] = BOX as u64,
                        0x30 => c.g[V0][0] = 0,
                        _ => {}
                    }
                };
                c.bus.write16(VT + 0x130, 0);
                c.bus.write32(VT + 0x134, FAKE + 0x30);
                c.bus.write16(VT + 0x160, 0);
                // Every third seed has the object answer "do not test it".
                c.bus
                    .write32(VT + 0x164, if seed % 3 == 0 { FAKE + 0x30 } else { FAKE });
                c.bus.write16(VT + 0x168, 0);
                c.bus.write32(VT + 0x16c, FAKE + 0x20);
                c.bus.write32(OBJ + 0xc, VT);
                // The query box and the object box that overlaps it.
                for k in 0..3 {
                    c.bus.write32(Q + 0x20 + 4 * k, f(1.0));
                    c.bus.write32(Q + 0x30 + 4 * k, f(-1.0));
                    c.bus.write32(BOX + 4 * k, f(0.0));
                    c.bus.write32(BOX + 0x10 + 4 * k, f(0.0));
                }
                let count = seed % 5;
                c.bus.write32(LIST + 8, count);
                for i in 0..count {
                    let node = NODES + 0x100 * i;
                    let flags = match i % 4 {
                        0 => 0x20, // the node's own box, overlapping
                        1 => 0x20, // the node's own box, apart
                        2 => 0x40, // ask the object
                        _ => 0,
                    };
                    c.bus.write32(node + 8, flags);
                    c.bus
                        .write32(node + 0xc, if flags == 0x40 { OBJ } else { 0 });
                    for k in 0..3 {
                        let min = if i % 4 == 1 { 2.0 } else { 0.0 };
                        c.bus.write32(node + 0x60 + 4 * k, f(min));
                        c.bus.write32(node + 0x6c + 4 * k, f(0.0));
                    }
                    c.bus.write32(LIST + 0xc + 4 * i, node);
                }
                c.g[A0][0] = sx(LIST);
                c.g[A1][0] = sx(Q);
                c.g[A2][0] = sx(BUF);
                c.g[A3][0] = sx(8);
                c.g[T0][0] = if seed % 2 == 0 { 1 } else { 0 };
            },
        );
    }


    /// Not listed in `OVERRIDES`: the rail finders it calls
    /// (`RailModifier_FindClosestPoint`, `FlexRail_FindClosestPoint`,
    /// `Rail_FindNearestPoint`) are dispatchers that need real rail data, so
    /// the test only reaches the loop and the masked-out rail object. The
    /// port stays here for the call paths to be tested on real data.
    #[test]
    fn overlap_list_find_nearest_rail_matches() {
        const LIST: u32 = DATA + 0x1000;
        const POS: u32 = DATA + 0x2000;
        const POS2: u32 = DATA + 0x2100;
        const NODES: u32 = DATA + 0x4000;
        check(
            ssx3_ee::s11::f_00334680,
            overlap_list_find_nearest_rail,
            SEEDS_2,
            &|c, seed| {
                let _r = machine(c, seed, 0x4007, 0);
                let count = if seed % 5 == 0 { 0 } else { 1 + seed % 3 };
                c.bus.write32(LIST + 0x210, count);
                for i in 0..count {
                    let node = NODES + 0x100 * i;
                    // Plain rails (type 1) whose flags fail the mask, and
                    // objects of no rail type at all.
                    c.bus.write32(node + 8, if seed % 3 == 0 { 1 } else { 0 });
                    c.bus.write32(node + 0x68, node + 0x90);
                    c.bus.write32(node + 0x90 + 0x1c, 0);
                    c.bus.write32(LIST + 0x214 + 4 * i, node);
                }
                put_v(c, POS, [4.0, 3.0, 2.0, 0.0]);
                c.g[A0][0] = sx(LIST);
                c.g[A1][0] = sx(POS);
                c.g[A2][0] = sx(POS2);
                c.g[A3][0] = sx(0xffff_ffff);
                c.f[12] = f(1.0);
            },
        );
    }


    #[test]
    fn anim_object_kill_restorable_matches() {
        crate::difftest::check(ee::s11::f_00341fc8__generated, anim_object_kill_restorable, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x341f_c8, 2);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 + (r.next_u32() & 0x10) as u64 };
            // The kill-script handle `WorldObj_RunKillScript` follows
            // (`o + 0x30 + 0x18`).
            c.bus.write32(DATA + 0x48, 0);
        });
    }


    // -------------------------------------------------- vtable forwarders

    /// The wrappers whose vtable comes from the argument object.
    pub(crate) fn forward_setup(c: &mut Cpu, seed: u32, salt: u64, slot: u32) {
        let mut r = machine(c, seed, salt, 0);
        let obj = DATA + 0x100;
        let arg = DATA + 0x300;
        let vt = put_vtable(c, &mut r, DATA + 0x800, &[slot]);
        c.bus.write32(arg, vt);
        c.g[A0][0] = sx(obj);
        c.g[A1][0] = sx(arg);
        c.g[A2][0] = sx(r.next_u32());
        c.g[A3][0] = sx(r.next_u32());
        fill(c, &mut r, obj, 8);
    }


    #[test]
    fn boost_modifier_base_save_matches() {
        check(ee::s11::f_003584b8__generated, boost_modifier_base_save, 120, &|c, seed| {
            forward_setup(c, seed, 0x3584_b8, 8);
        });
    }


    #[test]
    fn coll_sphere_tree_get_diameter_matches() {
        check(ssx3_ee::s11::f_003306d8__generated, coll_sphere_tree_get_diameter, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x3306_d8, 0);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, (seed % 3 != 0) as u32);
            c.bus.write32(DATA + 0x60, DATA + 0x200);
            c.bus.write32(DATA + 0x200 + 0x98, DATA + 0x300);
            c.bus.write32(DATA + 0x300 + 0x20, DATA + 0x400);
            c.bus.write32(DATA + 0x200 + 0x90, r.f32(100.0));
            c.bus.write32(DATA + 0x400, r.f32(100.0));
        });
    }


    #[test]
    fn nash_destroy_fwd_diff_cache_matches() {
        check(ssx3_ee::s11::f_00334850__generated, nash_destroy_fwd_diff_cache, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x3348_50, 0);
            c.g[A0][0] = DATA as u64;
            let p = if seed % 3 == 0 { 0 } else { small_pointer(seed) };
            c.bus.write32(DATA + 0xa4, p);
            c.bus.write32(DATA + 0x10, r.next_u32());
        });
    }


    /// A stream whose first word is a vtable with slot `off` pointing at the
    /// fake callee.
    pub(crate) fn fake_stream(c: &mut Cpu, r: &mut Rng, stream: u32, off: u32) {
        clear(c, stream, 0x40);
        let vt = vtable(c, r, &[off]);
        c.bus.write32(stream, vt);
    }


    // ------------------------------------------------------------------
    // World object save family.

    /// A save-function test: a dummy object, a stream whose vtable slot 8 is a
    /// fake serialiser, and the type's `save` prelude reading the stream.
    pub(crate) fn save_test(generated: GuestFn, hand: GuestFn, salt: u64, off: u32) {
        check(generated, hand, SEEDS_4, &move |c, seed| {
            let mut r = machine(c, seed, salt, 0);
            let this = DATA + 0x100;
            let stream = DATA + 0x400;
            clear(c, this, 0x300);
            fake_stream(c, &mut r, stream, 8);
            c.bus.write32(this + off, if seed % 3 == 0 { 0 } else { DATA + 0x800 });
            clear(c, DATA + 0x800, 0x40);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(stream);
        });
    }


    #[test]
    fn boost_save_matches() {
        save_test(ee::s11::f_00341a50__generated, boost_save, 0x341a_50, 0x20);
    }


    // ------------------------------------------------------------------
    // Objects and world.

    #[test]
    fn nash_create_fwd_diff_cache_matches() {
        check(
            ee::s11::f_00334800__generated,
            nash_create_fwd_diff_cache,
            SEEDS_5,
            &|c, seed| {
                let _ = machine(c, seed, 0x3348_00, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x120);
                c.g[A0][0] = sx(this);
            },
        );
    }


    #[test]
    fn coll_sphere_tree_dtor_matches() {
        check(
            ssx3_ee::s11::f_003304e8__generated,
            coll_sphere_tree_dtor,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x3304e8, 0);
                heap_init(c);
                let this = alloc(c, 0x100);
                let child = if seed % 3 == 0 { 0 } else { alloc(c, 0x40) };
                c.bus.write32(this + 0x68, child);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }


    // ------------------------------------------------------------------
    // Collision.

    #[test]
    fn coll_sphere_get_diameter_matches() {
        check(ee::s11::f_00340a18__generated, coll_sphere_get_diameter, 200, &|c, seed| {
            let mut r = setup(c, seed, 41);
            c.g[A0][0] = AUX as u64;
            put(c, AUX, if seed % 3 == 0 { 0 } else { r.next_u32() });
            put(c, AUX + 0x70, r.f32(10.0));
            c.f[0] = r.f32(1.0);
            c.f[1] = r.f32(1.0);
        });
    }


    #[test]
    fn boost_handle_event_matches() {
        check(ee::s11::f_00341a30__generated, boost_handle_event, 200, &|c, seed| {
            let mut r = setup(c, seed, 42);
            c.g[A0][0] = AUX as u64;
            c.g[A1][0] = [1u64, 0, 1, 2][(seed % 4) as usize];
            put(c, AUX + 0x38, r.next_u32());
            put(c, AUX + 0x3c, r.next_u32());
        });
    }


    #[test]
    fn anim_teeter_v53_matches() {
        // (lean +0x4, upper +0x10, lower +0x14, velocity +0x30)
        // Four distinct paths: the early `0.0` (heading inwards), the
        // `0 < lean` clamp, the outwards bypass and the inner `0.0`.
        const CASES: [[f32; 4]; 6] = [
            [1.0, 2.0, -2.0, 1.0],   // velocity <= upper, lean >= 0, 0 < lean -> 0
            [-1.0, 2.0, -2.0, 1.0],  // velocity <= upper, lean < 0 -> 0 (heading in)
            [1.0, 0.5, -2.0, 1.0],   // velocity > upper, lower <= velocity, 0 < lean -> 0
            [-1.0, 0.5, 2.0, 1.0],   // velocity > upper, lower > velocity -> lean
            [-1.0, 2.0, -2.0, -5.0], // velocity <= upper, lean < 0 -> 0
            [0.0, 2.0, -2.0, 1.0],   // velocity <= upper, lean == 0 -> lean
        ];
        check(ee::s11::f_003424d0__generated, anim_teeter_v53, 200, &|c, seed| {
            let mut r = setup(c, seed, 43);
            c.g[A0][0] = AUX as u64;
            let k = CASES[(seed % 6) as usize];
            put(c, AUX + 4, if seed % 7 == 0 { r.f32(2.0) } else { fbits(k[0]) });
            put(c, AUX + 0x10, fbits(k[1]));
            put(c, AUX + 0x14, fbits(k[2]));
            put(c, AUX + 0x30, fbits(k[3]));
        });
    }


    #[test]
    fn boost_modifier_handle_event_matches() {
        check(ee::s11::f_00342b80__generated, boost_modifier_handle_event, 200, &|c, seed| {
            let mut r = setup(c, seed, 44);
            c.g[A0][0] = AUX as u64;
            c.g[A1][0] = [0x258u64, 0, 0x258, 0x100][(seed % 4) as usize];
            put(c, AUX + 0x28, r.next_u32());
            put(c, AUX + 0x2c, r.next_u32());
        });
    }


    #[test]
    fn boost_ctor_2_matches() {
        check(ee::s11::f_00341548__generated, boost_ctor_2, SEEDS_6, &|c, seed| {
            let mut r = machine(c, seed, 0x0034_1548, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.g[A2][0] = (DATA + 0x300) as u64;
            let vt = vtable(c, &mut r, &[0x10]);
            w(c, DATA + 0x300, vt);
        });
    }


    // ------------------------------------------------------ transform/boost/sphere

    #[test]
    fn coll_sphere_tree_transform_matches() {
        check(
            ssx3_ee::s11::f_00330710__generated,
            coll_sphere_tree_transform,
            100,
            &|c, seed| {
                let mut r = setup_2(c, seed, 0x330710);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.bus.write32(DATA + 0x60, DATA + 0x300);
                c.bus.write32(DATA + 0x68, DATA + 0x400);
                for i in 0..0x200u32 / 4 {
                    c.bus.write32(DATA + 0x300 + 4 * i, r.next_u32());
                    c.bus.write32(DATA + 0x400 + 4 * i, r.next_u32());
                }
                c.f[12] = r.edge_f32();
            },
        );
    }


    #[test]
    fn boost_modifier_v06_matches() {
        check(
            ssx3_ee::s11::f_00342ba0__generated,
            boost_modifier_v06,
            80,
            &|c, seed| {
                let mut r = setup_2(c, seed, 0x342ba0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                let vt = vtable(c, &mut r, &[8, 0x28]);
                c.bus.write32(DATA + 0x800, vt);
                c.bus.write32(DATA + 0x40, r.next_u32());
            },
        );
    }


    #[test]
    fn rail_find_nearest_point_matches() {
        check(
            ssx3_ee::s11::f_00335128__generated,
            rail_find_nearest_point,
            40,
            &|c, seed| {
                let mut r = Rng(0x3351_28a7 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0033_5128;
                c.g[GP][0] = GAME_GP as u64;
                let rail = DATA;
                let query = DATA + 0x400;
                let vec = DATA + 0x500;
                let out = DATA + 0x800;
                c.g[A0][0] = rail as u64;
                c.g[A1][0] = query as u64;
                c.g[A2][0] = vec as u64;
                c.g[A3][0] = (DATA + 0x600) as u64;
                c.g[T0][0] = (DATA + 0x700) as u64;
                c.g[T1][0] = out as u64;
                // Rail bounds: min 1.0, max 3.0. The query box is either
                // inside them (main path) or below the minimum (reject).
                for off in [0x6c, 0x70, 0x74] {
                    c.bus.write32(rail + off, 0x3f80_0000);
                }
                for off in [0x78, 0x7c, 0x80] {
                    c.bus.write32(rail + off, 0x4040_0000);
                }
                let below = seed % 4 == 0;
                for off in [0x0, 0x4, 0x8] {
                    c.bus.write32(query + off, 0);
                }
                let max = if below { 0x3f00_0000 } else { 0x4000_0000 };
                for off in [0x10, 0x14, 0x18] {
                    c.bus.write32(query + off, max);
                }
                // The rail's 4x4 matrix and the query vector.
                for i in 0..4 {
                    put_vec(
                        c,
                        rail + 0x10 + i * 0x10,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.next_u32()],
                    );
                }
                put_vec(c, vec, [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.next_u32()]);
                // The result word read at 0x335804 / 0x335824, and the float
                // at t0 compared on the way to the success block.
                c.bus.write32(DATA + 0x600, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0x700, r.edge_f32());
                // The gp-relative constants the pass-2 setup scales by.
                c.bus.write32(GAME_GP - 0x2d68, r.edge_f32());
                c.bus.write32(GAME_GP - 0x2d64, r.edge_f32());
                c.bus.write32(GAME_GP - 0x2d60, r.edge_f32());
                // The rail pointer read in the success block.
                c.bus.write32(rail + 0x68, DATA + 0x900);
                c.bus.write32(DATA + 0x928, r.next_u32());
            },
        );
    }


    #[test]
    fn anim_object_ctor_2_matches() {
        check(ee::s11::f_00341c80__generated, anim_object_ctor_2, 60, &|c, seed| {
            let mut r = alloc_base(c, seed, 0x341c_80);
            let obj = DATA + 0x1000;
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = 0;
            c.bus.write32(STACK_TOP - 0xd0, 0);
        });
    }


    #[test]
    fn anim_teeter_ctor_2_matches() {
        check(ee::s11::f_003422e8__generated, anim_teeter_ctor_2, 60, &|c, seed| {
            let mut r = alloc_base(c, seed, 0x3422_e8);
            let obj = DATA + 0x1000;
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = 0;
            c.bus.write32(STACK_TOP - 0xd0, 0);
        });
    }


    pub(crate) const SEEDS_7: u32 = 200;


    pub(crate) fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }


    #[test]
    fn cell_collide_volume_recursive_matches() {
        check(
            ee::s11::f_0033ccf8__generated,
            cell_collide_volume_recursive,
            SEEDS_7,
            &|c, seed| {
                let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ 0x33cc);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                // The BELibrary singleton and the world singleton (callees).
                let belib = DATA + 0x3_0000;
                let world = DATA + 0x3_1000;
                c.bus.write32(gp_at(-0x1f04), belib);
                c.bus.write32(gp_at(-0x848), world);
                c.bus.write32(world + 0x84, world + 0x100);
                c.bus.write32(world + 0x100 + 0x7c, 0);
                // The 0.2f constants the classification multiplies with.
                for k in 0..16 {
                    c.bus.write32(gp_at(-0x2d10 + 4 * k), f(0.2));
                }

                let root = DATA + 0x1000;
                let cell = DATA + 0x2000;
                let pos = DATA + 0x2100;
                // One instance and one object on the root so both list loops run
                // (each callee returns from its first validity test).
                c.bus.write32(root + 0x24, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, 0);
                c.bus.write16(DATA + 0x3000 + 0xa, 0);
                c.bus.write32(root + 0x20, DATA + 0x3100);
                c.bus.write32(DATA + 0x3100, 0);
                c.bus.write32(DATA + 0x3100 + 0x88, 0);
                // All eight children present; the first two also have one leaf
                // grandchild (reached by `Cell_CollideVolumeSubtree`).
                for i in 0..8u32 {
                    let node = DATA + 0x1100 + 0x100 * i;
                    c.bus.write32(root + 4 * i, node);
                    if i < 2 {
                        c.bus.write32(node, DATA + 0x1500 + 0x100 * i);
                    }
                }
                let level = seed % 12;
                let (x, y, z) = (r.next_u32() % 4, r.next_u32() % 4, r.next_u32() % 4);
                c.bus.write32(cell, level);
                c.bus.write32(cell + 4, x);
                c.bus.write32(cell + 8, y);
                c.bus.write32(cell + 0xc, z);
                // The query box: containing (collect the subtree), disjoint (skip
                // the child) or straddling the first octant (recurse).
                let s = f32::from_bits((level + 0x7e) << 23);
                for k in 0..3u32 {
                    let coord = [x, y, z][k as usize] as f32;
                    let lo = (2.0 * coord - 0.2) * s;
                    let hi = (2.0 * coord + 1.2) * s;
                    let (min, max) = match seed % 3 {
                        0 => (-1.0e6f32, 1.0e6f32),
                        1 => (1.0e6f32, 1.0e6f32 + 1.0),
                        _ => (0.5 * (lo + hi), 1.0e6f32),
                    };
                    c.bus.write32(pos + 4 * k, f(min));
                    c.bus.write32(pos + 0x10 + 4 * k, f(max));
                }
                c.g[A0][0] = sx(DATA + 0x4000);
                c.g[A1][0] = sx(DATA + 0x3200);
                c.g[A2][0] = sx(DATA + 0x3300);
                c.g[A3][0] = sx(pos);
                c.g[T0][0] = sx(root);
                c.g[T1][0] = sx(cell);
            },
        );
    }


    #[test]
    fn cell_gather_all_recursive_matches() {
        check(
            ee::s11::f_0033b748__generated,
            cell_gather_all_recursive,
            SEEDS_7,
            &|c, seed| {
                let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ 0xb748);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let belib = DATA + 0x3_0000;
                let world = DATA + 0x3_1000;
                c.bus.write32(gp_at(-0x1f04), belib);
                c.bus.write32(gp_at(-0x848), world);
                c.bus.write32(world + 0x84, world + 0x100);
                c.bus.write32(world + 0x100 + 0x7c, 0);
                for k in 0..16 {
                    c.bus.write32(gp_at(-0x2d10 + 4 * k), f(0.2));
                }

                let root = DATA + 0x1000;
                let cell = DATA + 0x2000;
                let box_at = DATA + 0x2100;
                let gbox = DATA + 0x2200;
                let (q, cq) = (DATA + 0x5000, DATA + 0x5200);
                // The two gather contexts: list/count/box triples at +0
                // (nodes), +0x10 (objects) and +0x20 (all).
                for (qi, ctx) in [q, cq].into_iter().enumerate() {
                    for k in 0..3u32 {
                        let sub = ctx + 0x10 * k;
                        c.bus
                            .write32(sub, DATA + 0x5_4000 + 0x400 * qi as u32 + 0x100 * k);
                        c.bus
                            .write32(sub + 4, DATA + 0x5_c000 + 0x400 * qi as u32 + 0x100 * k);
                        c.bus.write32(sub + 0xc, gbox);
                    }
                }
                // The gather box contains the root's object.
                for k in 0..3 {
                    c.bus.write32(gbox + 4 * k, f(-1.0e6));
                    c.bus.write32(gbox + 0x10 + 4 * k, f(1.0e6));
                }
                c.bus.write32(root + 0x20, 0);
                c.bus.write32(root + 0x24, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, 0);
                for k in 0..3 {
                    c.bus.write32(DATA + 0x3000 + 0x158 + 4 * k, f(-1.0));
                    c.bus.write32(DATA + 0x3000 + 0x164 + 4 * k, f(1.0));
                }
                c.bus.write32(root + 0x28, DATA + 0x3100);
                c.bus.write32(DATA + 0x3100, 0);
                // All eight children present; the first two have a leaf
                // grandchild for the inlined `Cell_GatherAllSubtree`.
                for i in 0..8u32 {
                    let node = DATA + 0x1100 + 0x100 * i;
                    c.bus.write32(root + 4 * i, node);
                    if i < 2 {
                        c.bus.write32(node, DATA + 0x1500 + 0x100 * i);
                    }
                }
                let level = seed % 12;
                let (x, y, z) = (r.next_u32() % 4, r.next_u32() % 4, r.next_u32() % 4);
                c.bus.write32(cell, level);
                c.bus.write32(cell + 4, x);
                c.bus.write32(cell + 8, y);
                c.bus.write32(cell + 0xc, z);
                // Containing / disjoint / straddling query box.
                let s = f32::from_bits((level + 0x7e) << 23);
                for k in 0..3u32 {
                    let coord = [x, y, z][k as usize] as f32;
                    let lo = (2.0 * coord - 0.2) * s;
                    let hi = (2.0 * coord + 1.2) * s;
                    let (min, max) = match seed % 3 {
                        0 => (-1.0e6f32, 1.0e6f32),
                        1 => (1.0e6f32, 1.0e6f32 + 1.0),
                        _ => (0.5 * (lo + hi), 1.0e6f32),
                    };
                    c.bus.write32(box_at + 4 * k, f(min));
                    c.bus.write32(box_at + 0x10 + 4 * k, f(max));
                }
                c.g[A0][0] = sx(DATA + 0x4000);
                c.g[A1][0] = sx(q);
                c.g[A2][0] = sx(cq);
                c.g[A3][0] = sx(box_at);
                c.g[T0][0] = sx(root);
                c.g[T1][0] = sx(cell);
            },
        );
    }


    #[test]
    fn boost_modifier_ctor_2_matches() {
        check(ee::s11::f_00342808__generated, boost_modifier_ctor_2, SEEDS_8, &|c, seed| {
            let mut r = machine(c, seed, 0x342808, 2);
            be_library(c);
            let obj = DATA;
            let stream = DATA + 0x800;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = stream as u64;
            let vt = DATA + 0x3000;
            w(c, stream, vt);
            slot(c, vt, 0x10, 0x10, FAKE);
            slot(c, vt, 0x18, 0x18, FAKE + 0x40);
            w(c, obj + 0x40, r.next_u32());
        });
    }


    #[test]
    fn collspheretree_collide_matches() {
        for (gen, hand) in [
            (
                ee::s11::f_003308c8__generated as fn(&mut Cpu),
                collspheretree_collide_sphere_tree as fn(&mut Cpu),
            ),
            (ee::s11::f_00330950__generated, collspheretree_collide_aabb),
        ] {
            check(gen, hand, SEEDS_9, &|c, seed| {
                base_2(c, seed, 0x3308c8, 0);
                let s = DATA + 0x100;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = 0;
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] = sx(DATA + 0x200);
                // No collision tree: the deep tree walk needs a real sphere
                // tree, so the zero path is what the fixture can bound.
                c.bus.write32(s + 0x60, 0);
            });
        }
    }


    #[test]
    fn coll_sphere_tree_collide_triangle_matches() {
        check(ssx3_ee::s11::f_00330788__generated, coll_sphere_tree_collide_triangle, 20, &|c, seed| {
            coll_args(c, seed, 0x330788, DATA + 0x3000, ssx3_ee::s11::f_00330788__generated);
        });
    }


    #[test]
    fn coll_sphere_tree_collide_triangle2_sided_matches() {
        check(
            ssx3_ee::s11::f_00330828__generated,
            coll_sphere_tree_collide_triangle2_sided,
            20,
            &|c, seed| {
                coll_args(c, seed, 0x330828, DATA + 0x3000, ssx3_ee::s11::f_00330828__generated);
            },
        );
    }


    #[test]
    fn boost_v48_matches() {
        check(
            ee::s11::f_00341770__generated,
            boost_v48,
            8,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x341770, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                let vt4 = vtable(c, &mut r, &[0x10, 0x14, 0x20, 0x24]);
                w(c, OBJ2, vt4);
                wf(c, OBJ + 0x34, 0.5);
                w(c, OBJ + 0x60, 0);
            },
        );
    }


    #[test]
    fn boost_modifier_v08_matches() {
        check(
            ee::s11::f_003429e0__generated,
            boost_modifier_v08,
            8,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x3429e0, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                let vt5 = vtable(c, &mut r, &[0x10, 0x14, 0x20, 0x24]);
                w(c, OBJ2, vt5);
                wf(c, OBJ + 0x24, 0.25);
                w(c, OBJ + 0x60, 0);
            },
        );
    }


    #[test]
    fn anim_object_v36_matches() {
        same(
            ssx3_ee::s11::f_00341fe8__generated,
            anim_object_v36,
            200,
            &|c, seed| {
                let mut r = Rng(0x341fe8 ^ seed as u64);
                generic(c, seed, 0x20);
                let s = DATA + 0x100;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = match seed % 7 {
                    0 => 0x64,
                    1 => 0x65,
                    2 => 0x66,
                    3 => 0x67,
                    4 => 0x68,
                    5 => 0x69,
                    _ => 0x6a,
                } as u64;
                c.f[12] = r.edge_f32();
                for off in [0u32, 4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x48] {
                    c.bus.write32(s + off, r.edge_f32());
                }
                // self->+0x48 -> object -> +0x80 -> limiter (+0x14)
                let base = DATA + 0x300;
                let o = DATA + 0x400;
                c.bus.write32(s + 0x48, base);
                c.bus.write32(base + 0x80, o);
                c.bus.write32(o + 0x14, r.f32(5.0));
            },
        );
    }


    // -- 0x00330540 CollSphereTree_OverlapsAABB -------------------------

    #[test]
    fn coll_sphere_tree_overlaps_aabb_matches() {
        check(
            ssx3_ee::s11::f_00330540__generated,
            coll_sphere_tree_overlaps_aabb,
            150,
            &|c, seed| {
                let mut r = Rng(0x330540 ^ (seed as u64) << 8);
                c.g[GP][0] = GAME_GP as u64;
                c.g[SP][0] = crate::difftest::SP as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                let this = DATA + 0x1000;
                let lo = DATA + 0x2000;
                let hi = DATA + 0x2100;
                let inner = DATA + 0x3000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(lo);
                c.g[A2][0] = sx(hi);
                w(c, this + 0x60, inner);
                for i in 0..4u32 {
                    wf_2(c, inner + 0x80 + 4 * i, r.f32(10.0));
                }
                let mut g = |c: &mut Cpu, at: u32| {
                    let v = [r.f32(4.0), r.f32(4.0), r.f32(4.0)];
                    for i in 0..3 {
                        wf_2(c, at + 4 * i as u32, v[i]);
                    }
                };
                g(c, lo);
                g(c, hi);
            },
        );
    }


    pub(crate) const BOOST_MOD: u32 = DATA + 0x1_0000;


    #[test]
    fn boost_modifier_v02_matches() {
        check_all(ee::s11::f_00342880__generated, boost_modifier_v02, 20, &|c, seed| {
            let mut r = base_5(c, seed, 0x3428_80, 14);
            c.pc = 0x0034_2880;
            c.g[A0][0] = sx(BOOST_MOD);
            c.bus.write32(BOOST_MOD + 0x0, (seed % 4) as u32);
            c.bus.write32(BOOST_MOD + 0x4, r.edge_f32());
            c.bus.write32(BOOST_MOD + 0x8, r.f32(1.0));
            c.bus.write32(BOOST_MOD + 0x10, (seed % 3) as u32);
            c.bus.write32(BOOST_MOD + 0x1c, r.f32(2.0));
            c.bus.write32(RMAN + 0x7c, 1);
        });
    }


    pub(crate) const BOOST: u32 = DATA + 0x1_0000;


    /// The boost window (`+0x3c`, 30 frames) lasts 0.5 s at any rate: the
    /// zone (mode 0) stays active without testing riders until it runs out.
    #[test]
    fn boost_update_window_follows_rate() {
        for (hz, ticks) in [(60, 30), (120, 59), (240, 117)] {
            let mut c = crate::difftest::fresh();
            c.hooks.miss = crate::difftest::fakes::fake_call;
            c.bus.write32(crate::difftest::fakes::LOG_UNWIND_AT, u32::MAX);
            crate::tick::set_for_test(&mut c, hz);
            // Its vtable (+0xc) is empty: the effect calls are fakes.
            c.bus.write32(BOOST + 0xc, BOOST + 0x1000);
            c.bus.write32(BOOST + 0x40, 0);
            c.bus.write32(BOOST + 0x3c, 30);
            let mut n = 0;
            while c.bus.read32(BOOST + 0x3c) != 0 {
                n += 1;
                assert!(n < 1000);
                c.g[GP][0] = GAME_GP as u64;
                c.g[SP][0] = crate::difftest::SP as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                c.g[A0][0] = sx(BOOST);
                c.pc = 0x0034_15d0;
                boost_update(&mut c);
                assert_eq!(c.pc, crate::difftest::RA);
            }
            assert_eq!(n, ticks, "{hz} Hz");
        }
    }


    #[test]
    fn boost_update_matches() {
        check_all(ee::s11::f_003415d0__generated, boost_update, 20, &|c, seed| {
            let mut r = base_5(c, seed, 0x3415_d0, 14);
            c.pc = 0x0034_15d0;
            c.g[A0][0] = sx(BOOST);
            c.bus.write32(BOOST + 0x0, (seed % 4) as u32);
            c.bus.write32(BOOST + 0x4, r.edge_f32());
            c.bus.write32(BOOST + 0x8, r.f32(1.0));
            c.bus.write32(BOOST + 0xc, r.edge_f32());
            c.bus.write32(BOOST + 0x10, (seed % 3) as u32);
            c.bus.write32(BOOST + 0x1c, r.f32(2.0));
            c.bus.write32(RMAN + 0x7c, 1);
        });
    }
}
