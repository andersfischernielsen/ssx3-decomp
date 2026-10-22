//! `src/intersect/riderspheretree.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::float;
use ssx3_ee_rt::Cpu;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Frame;
use crate::events;
use crate::abi::r32;
use crate::call::sp;
use ssx3_ee_rt::funcs::call_indirect;
pub(crate) use crate::game::ai::rider::r;
pub(crate) use crate::game::ai::rider::p;
pub(crate) use crate::game::ai::rider::at;
pub(crate) use crate::game::ai::rider::gp_2 as gp;
pub(crate) use crate::game::ai::rider::sp_2;
pub(crate) use crate::game::ai::rider::slt;
pub(crate) use crate::game::ai::rider::Ops;
pub(crate) use crate::game::ai::rider::epilogue;
pub(crate) use crate::game::ai::rider::prologue;
pub(crate) use crate::game::ai::rider::vcall;
pub(crate) use crate::game::ai::control::handplantcontrol::gp_2;
pub(crate) use crate::game::intersect::worldsphtree::sp_2 as sp_3;
pub(crate) use crate::game::ai::computer::gp_2 as gp_3;
pub(crate) use crate::game::ai::computer::lw;
pub(crate) use crate::game::ai::computer::fr_2 as fr;
pub(crate) use crate::game::ai::computer::setf_2 as setf;
pub(crate) use crate::game::ai::computer::ldf_2 as ldf;
pub(crate) use crate::game::ai::computer::lt_2 as lt;
#[cfg(test)]
pub(crate) use crate::game::render::bezierman::f;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p_3;
pub(crate) use crate::game::fe::festateoptions::gpa;
pub(crate) use crate::game::world::worldview::p_2 as p_4;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::at as at_3;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw;
pub(crate) use crate::game::fe::ovstates::at as at_4;
pub(crate) use crate::game::fe::ovstates::open;
pub(crate) use crate::game::object::flexrailnode::at_3 as at_5;
pub(crate) use crate::game::object::flexrailnode::r32v;
pub(crate) use crate::game::object::flexrailnode::spa;
pub(crate) use crate::game::render::graphicsman::RegMem;

/// Replacements in this unit: (address, function, name).
pub(crate) use crate::game::fe::ovstatemission2::ov_screen_set_widget_visibility;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0032_9970, rider_sphere_tree_dtor, "RiderSphereTree_dtor"),
    (0x0032_99c8, rider_sphere_tree_init, "RiderSphereTree_Init"),
    (0x0032_9a90, rider_sphere_tree_set_spheres, "RiderSphereTree_SetSpheres"),
    (0x0032_9dc8, rider_sphere_tree_get_bounds, "RiderSphereTree_GetBounds"),
    (0x0032_9f98, rider_sphere_tree_intersect, "RiderSphereTree_Intersect"),
    (0x0032_b6a8, vu0_triangle_intersect, "VU0_TriangleIntersect"),
    (0x0032_b6e0, ai_fwd_diff_cache_collide_entry, "AIFwdDiffCache_CollideEntry"),
    (0x0032_c508, sphere_tree_ctor, "SphereTree_ctor"),
    (0x0032_c540, sphere_tree_reset_transform, "SphereTree_ResetTransform"),
    (0x0032_c5a8, sphere_tree_get_half_leaf_radius, "SphereTree_GetHalfLeafRadius"),
    (0x0032_c898, sphere_tree_collide_sphere, "SphereTree_CollideSphere"),
    (0x0032_c928, sphere_tree_collide_aabb, "SphereTree_CollideAABB"),
    (0x0032_ca78, spheretree_collide_triangle, "SphereTree_CollideTriangle"),
    (0x0032_cb58, sphere_tree_collide_sphere_tree, "SphereTree_CollideSphereTree"),
    (0x0032_da40, shell_sort_desc_key_val, "ShellSortDesc_KeyVal"),
    (0x0032_e100, coll_segment_ctor, "CollSegment_ctor"),
    (0x0032_e288, coll_segment_overlaps_aabb, "CollSegment_OverlapsAABB"),
    (0x0032_e4b8, coll_segment_reset_transform, "CollSegment_ResetTransform"),
    (0x0032_e4d0, coll_segment_collide_triangle, "CollSegment_CollideTriangle"),
    (0x0032_e690, coll_segment_collide_aabb, "CollSegment_CollideAABB"),
    (0x0032_f650, coll_hull_ctor, "CollHull_ctor"),
    (0x0032_f708, coll_hull_dtor, "CollHull_dtor"),
    (0x0032_f8c0, coll_hull_get_diameter, "CollHull_GetDiameter"),
    (0x0032_f990, coll_hull_collide_triangle2_sided, "CollHull_CollideTriangle2Sided"),
    (0x0032_fa30, coll_hull_collide_sphere_tree, "CollHull_CollideSphereTree"),
    (0x0032_fb48, coll_sphere_ctor, "CollSphere_ctor"),
    (0x0032_fc08, coll_sphere_overlaps_aabb, "CollSphere_OverlapsAABB"),
    (0x0032_fd40, coll_sphere_transform, "CollSphere_Transform"),
    (0x0032_fd98, coll_sphere_reset_transform, "CollSphere_ResetTransform"),
    (0x0032_fdb0, coll_sphere_collide_sphere_tree, "CollSphere_CollideSphereTree"),
    (0x0032_fe40, coll_sphere_collide_aabb, "CollSphere_CollideAABB"),
    (0x0033_0250, coll_sphere_hits_triangle_edges, "CollSphere_HitsTriangleEdges"),
    (0x0033_0360, coll_sphere_plane_distance, "CollSphere_PlaneDistance"),
];

// ---------------------------------------------------------------------------
// Sphere tree (`sSphereBox_9DC8`, 0x20-byte nodes).

pub(crate) mod sphere_tree {

    pub const CENTER: u32 = 0x10;

    pub const RADIUS: u32 = 0x20;

    pub const COUNT: u32 = 0x2c;

    pub const CHILD_CENTER: u32 = 0x30;

    pub const CHILD_RADIUS: u32 = 0x40;

    pub const CHILD_SIZE: u32 = 0x20;

    /// Bit `i` selects child `i`; 0 means a leaf.
    pub const MASK: u32 = 0x28;

    /// Child `i`'s fields are read from the slot base `tree + i * 0x20`: the
    /// position quadword at `+0x30`, the radius one quadword further at
    /// `+0x40`.
    pub const KID_POS: u32 = 0x30;

    pub const KID_RADIUS: u32 = 0x40;

    pub const KID_STRIDE: u32 = 0x20;
}

/// A cache cell (0x40 bytes): two triangle normals (`+0`, `+0x10`; x =
/// the "not computed" sentinel `gp-0x2d94`, or the "degenerate" value
/// `gp-0x2d90`) and the cell's box (`+0x20` min, `+0x30` max).
pub(crate) mod cell {

    pub const NORMAL1: u32 = 0x10;

    pub const BOX_MIN: u32 = 0x20;

    pub const BOX_MAX: u32 = 0x30;

    pub const SIZE: u32 = 0x40;
}

/// The hint (`t3`): `{key, u16 row, u16 col, first_triangle, kind}`.
pub(crate) mod hint {

    pub const KEY: u32 = 0x0;

    pub const ROW: u32 = 0x4;

    pub const COL: u32 = 0x6;

    /// Non-zero: the cell's first triangle (else the second).
    pub const FIRST: u32 = 0x8;

    pub const KIND: u32 = 0xc;
}

/// The volume (`a0`): kind (`+0`), facing test (`+8`) with its direction
/// (`+0x10`), box (`+0x20` max, `+0x30` min... compared as the original),
/// vtable (`+0x50`: 0x20 triangle test, 0x40 UV refine).
pub(crate) mod volume {

    pub const KIND: u32 = 0x0;

    pub const FACING: u32 = 0x8;

    pub const DIR: u32 = 0x10;

    pub const VTABLE: u32 = 0x50;
}

/// Direct-call targets (dossier callees, found by address in `generated/ee`).
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::render::bezierman::callee::CULL_AGAINST_LIGHTS;
    pub(crate) use crate::game::render::bezierman::callee::PROJECT_RIDER;
    pub(crate) use crate::game::render::bezierman::callee::RIDER_GET_DIST_TO_FINISH;
    pub(crate) use crate::game::render::bezierman::callee::RIDER_SPHERE_TREE_CTOR;
    pub(crate) use crate::game::render::bezierman::callee::RIDER_SPHERE_TREE_GET_BOUNDS;
    pub(crate) use crate::game::render::bezierman::callee::SND_LOCK;
    pub(crate) use crate::game::render::bezierman::callee::SND_UNLOCK;
    pub(crate) use crate::game::render::bezierman::callee::SND_STREAM_POSITION;
    pub(crate) use crate::game::render::bezierman::callee::SND_SUB_TIME;
    pub(crate) use crate::game::render::bezierman::callee::SND_SUB_STATUS;
    pub(crate) use crate::game::render::bezierman::callee::SND_STREAM_INFO;
    pub(crate) use crate::game::render::bezierman::callee::SND_SUB_STATE;
    pub(crate) use crate::game::render::bezierman::callee::SND_STATUS;
    pub(crate) use crate::game::camera::camera::callee::MEASURE_ROW;
    pub(crate) use crate::game::camera::camera::callee::MEMCARD_SET_STATE;
    pub(crate) use crate::game::camera::camera::callee::TRANSITION_MAN_START;
    pub(crate) use crate::game::camera::camera::callee::WSCRIPT_DELETE_ALL;
    pub(crate) use crate::game::camera::camera::callee::SPHERE_TREE_RESET_TRANSFORM;
    pub(crate) use crate::game::camera::camera::callee::CAM_CONTROLLER_RESET;
    pub(crate) use crate::game::camera::camera::callee::MANUAL_CAM_ALGO_BASE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_FREE_TRIGGERS;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_RESET_HEADER;
    pub(crate) use crate::game::camera::camera::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::camera::camera::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::camera::camera::callee::BE_LIBRARY_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BEBAGT_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BE_OPTION_GET_MAX_HANDICAP;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_PROFILE_LIST;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_SLOTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_ABORT_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_EMAIL_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_LOGIN_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_12 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHALLENGE_POPUP_SET_SETTINGS_LOCKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_11 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_DEACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_RESET_RACE_RESULT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_IS_TRACK_GROUP0_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STORE_RACE_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_SKIP_WHITESPACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COLLECTED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COMPLETED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_GET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_4 as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_3 as NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID_2;
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
    pub(crate) use crate::game::be::bedbprofile::callee::BE_GET_EVENT_RECORD_INDEX;
    pub(crate) use crate::game::be::bedbprofile::callee::WORLD_TRIGGER_FIND_INSTANCE;
    pub(crate) use crate::game::be::bedbprofile::callee::WORLD_TRIGGER_FIND_FREE;
    pub(crate) use crate::game::be::bedbprofile::callee::SHP_NORMALIZE_BPP;
    pub(crate) use crate::game::be::bedbprofile::callee::SHP_CALC_IMAGE_SIZE;
    pub(crate) use crate::game::be::bedbprofile::callee::SHP_PALETTE_ENTRIES_FOR_BPP;
    pub(crate) use crate::game::be::bedbprofile::callee::CLOSEST_POINT_ON_SEGMENT;


    /// `__builtin_delete` (operator delete): frees through the heap manager.
    pub const DELETE: Callee = Callee { addr: 0x0031_7e50, f: ee::s10::f_00317e50 };

    /// `Dma_FromScratchpad` (0x371dd8): programs a scratchpad → RAM transfer.
    pub const DMA_FROM_SCRATCHPAD: Callee = Callee { addr: 0x0037_1dd8, f: ee::s12::f_00371dd8 };

    /// `ModObj_Update` (0x356198): the base object update.
    pub const MOD_OBJ_UPDATE: Callee = Callee { addr: 0x0035_6198, f: ee::s11::f_00356198 };
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
// Sphere tree.

/// `float SphereTree_GetHalfLeafRadius(SphereTree *tree)` (0x32c5a8): half the
/// radius read through `tree->leaves[?]`, scaled by the value at `+0x90`.
pub fn sphere_tree_get_half_leaf_radius(c: &mut Cpu) {
    c.cycles += 13;
    let tree = Addr(abi::arg(c, 0));
    let leaf = tree.off(0x98).lw(c);
    c.g[V1][0] = leaf;
    c.g[A1][0] = 0xc;
    c.f[2] = tree.off(0x90).read_fpu(c).0;
    let a = Addr(leaf as u32).off(0xc).lw(c);
    let b = Addr(leaf as u32).off(0x20).lw(c);
    let scaled = abi::mult(c, a as u32, 0xc);
    c.g[V0][0] = sx(scaled);
    c.g[A0][0] = b;
    c.g[AT][0] = 0x3f00_0000;
    c.f[1] = 0x3f00_0000; // 0.5, as `lui 0x3f00` + `mtc1 $at, $f1`
    let entry = sx(scaled.wrapping_add(b as u32));
    c.g[V0][0] = entry;
    c.f[0] = Addr(entry as u32).read_fpu(c).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[2])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[1])).0;
    abi::ret(c);
}


pub(crate) const ST_ENTRY: u32 = 0x0032_9f98;

/// The hit block (reached from the inner loop's back edge): resume entry.
pub(crate) const ST_HIT: u32 = 0x0032_a004;

/// Outer loop head (children of tree A against B's root): resume entry.
pub(crate) const ST_OUTER: u32 = 0x0032_a0b8;

/// Inner loop head (children of B against A's child): resume entry.
pub(crate) const ST_INNER: u32 = 0x0032_a130;

pub(crate) const ST_END: u32 = 0x0032_a1c0;


#[derive(Clone, Copy)]
pub(crate) enum SphereStage {
    Children,
    Outer,
    OuterReload,
    OuterNext,
    Inner,
    Hit,
    Miss,
    Done,
}


/// The inlined sphere-overlap test: `f1 = rb`, `f0 = ra`, `d = cb - ca`
/// (stored at `sp_2`, `sp_2+0x10`, `sp_2+0x20`), `f2 = rb + ra`, `f1 = f2^2`,
/// `f0 = |d|^2`, FCC = `f1 < f0`. True when the spheres are apart.
pub(crate) fn spheres_apart(c: &mut Cpu, b_center: u32, b_radius: u32, a_center: u32, a_radius: u32) -> bool {
    let s = sp_2(c);
    c.lwc1(1, b_radius);
    c.lwc1(0, a_radius);
    c.vsub(b_center, a_center);
    c.sqc2(5, p(s, 0x20));
    c.add_s(2, 1, 0);
    c.lq(V0, p(s, 0x20));
    c.sq(V0, s);
    c.mul_s(1, 2, 2);
    c.sq(V0, p(s, 0x10));
    c.lqc2(3, s);
    c.lqc2(5, s);
    c.vdot_regs(5);
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    c.c_lt_s(1, 0);
    c.fcc()
}


/// `bool RiderSphereTree_Intersect(SphereTree* a, SphereTree* b, Vec4*
/// push, Vec4* point)` (0x329f98): when the root spheres overlap, tests
/// every child sphere of `a` against `b`'s root and then against every
/// child of `b`; at the first overlapping pair writes the push-out vector
/// (`normal * penetration`) to `push` and the contact point (on `a`'s
/// child, `t2`) to `point` and returns true.
///
/// Dispatcher-shaped in the original: the loop heads and the hit block are
/// resume entries, all state lives in registers (t0-t4, f0/f2, the stack
/// scratch).
pub fn rider_sphere_tree_intersect(c: &mut Cpu) {
    use sphere_tree as st;
    let mut stage = match c.pc {
        ST_ENTRY => {
            c.cycles += 25;
            c.set_r(SP, sp_2(c).wrapping_sub(0x50));
            let (a, b) = (r(c, A0), r(c, A1));
            let apart = spheres_apart(
                c,
                p(b, st::CENTER),
                p(b, st::RADIUS),
                p(a, st::CENTER),
                p(a, st::RADIUS),
            );
            c.g[V0][0] = 0;
            if apart {
                c.cycles += 2;
                SphereStage::Done
            } else {
                SphereStage::Children
            }
        }
        ST_HIT => SphereStage::Hit,
        ST_OUTER => SphereStage::Outer,
        ST_INNER => SphereStage::Inner,
        pc => {
            c.jump_out(pc, ST_ENTRY, ST_END);
            return;
        }
    };
    loop {
        stage = match stage {
            SphereStage::Children => {
                c.cycles += 3;
                c.lw_r(V0, at(c, A0, st::COUNT));
                c.g[T4][0] = 0;
                if (c.g[V0][0] as i64) <= 0 {
                    SphereStage::Miss
                } else {
                    c.cycles += 2;
                    c.g[T3][0] = c.g[A0][0];
                    SphereStage::Outer
                }
            }
            // A's child t3 against B's root.
            SphereStage::Outer => {
                c.cycles += 24;
                let (b, child) = (r(c, A1), r(c, T3));
                let apart = spheres_apart(
                    c,
                    p(b, st::CENTER),
                    p(b, st::RADIUS),
                    p(child, st::CHILD_CENTER),
                    p(child, st::CHILD_RADIUS),
                );
                if apart {
                    c.lw_r(V0, at(c, A0, st::COUNT));
                    SphereStage::OuterNext
                } else {
                    c.cycles += 3;
                    c.lw_r(V0, at(c, A1, st::COUNT));
                    c.g[T1][0] = 0;
                    if (c.g[V0][0] as i64) <= 0 {
                        SphereStage::OuterReload
                    } else {
                        c.cycles += 3;
                        c.g[T2][0] = c.g[T3][0];
                        c.g[T0][0] = c.g[A1][0];
                        SphereStage::Inner
                    }
                }
            }
            // B's child t0 against A's child t2.
            SphereStage::Inner => {
                c.cycles += 24;
                let (bc, ac) = (r(c, T0), r(c, T2));
                let apart = spheres_apart(
                    c,
                    p(bc, st::CHILD_CENTER),
                    p(bc, st::CHILD_RADIUS),
                    p(ac, st::CHILD_CENTER),
                    p(ac, st::CHILD_RADIUS),
                );
                c.inc_r(T1, 1);
                if !apart {
                    if back_edge_unwinds(c, ST_HIT) {
                        return;
                    }
                    SphereStage::Hit
                } else {
                    c.cycles += 4;
                    c.lw_r(V0, at(c, A1, st::COUNT));
                    c.g[V0][0] = slt(c, T1, V0);
                    c.inc_r(T0, st::CHILD_SIZE);
                    if c.g[V0][0] != 0 {
                        if back_edge_unwinds(c, ST_INNER) {
                            return;
                        }
                        SphereStage::Inner
                    } else {
                        SphereStage::OuterReload
                    }
                }
            }
            SphereStage::OuterReload => {
                c.cycles += 1;
                c.lw_r(V0, at(c, A0, st::COUNT));
                SphereStage::OuterNext
            }
            SphereStage::OuterNext => {
                c.cycles += 4;
                c.inc_r(T4, 1);
                c.g[V0][0] = slt(c, T4, V0);
                c.inc_r(T3, st::CHILD_SIZE);
                if c.g[V0][0] != 0 {
                    if back_edge_unwinds(c, ST_OUTER) {
                        return;
                    }
                    SphereStage::Outer
                } else {
                    SphereStage::Miss
                }
            }
            SphereStage::Hit => {
                sphere_tree_hit(c);
                SphereStage::Done
            }
            SphereStage::Miss => {
                c.cycles += 1;
                c.g[V0][0] = 0;
                SphereStage::Done
            }
            SphereStage::Done => {
                c.cycles += 2;
                let ra = r(c, RA);
                c.set_r(SP, sp_2(c).wrapping_add(0x50));
                c.pc = ra;
                return;
            }
        };
    }
}


/// The hit: `n = d / |d|` (d at `sp_2`), `pen = (rb + ra) - |d|` (f2 - f1),
/// `*push = n * pen`, `*point = ca - n * (ra - pen)` with `a`'s child t2.
pub(crate) fn sphere_tree_hit(c: &mut Cpu) {
    c.cycles += 40;
    let s = sp_2(c);
    c.vlen2(s);
    c.vnorm();
    c.sqc2(5, p(s, 0x20));
    c.sqrt_s(1, 0);
    c.lq(V1, p(s, 0x20));
    c.sub_s(1, 2, 1);
    c.g[V0][0] = 1;
    c.sq(V1, p(s, 0x10));
    c.mfc1(A0, 1);
    c.vscale(p(s, 0x10), A0);
    let push = r(c, A2);
    c.vstore(5, p(s, 0x30), V1, &[push, p(s, 0x20)]);
    let child = r(c, T2);
    c.lwc1(0, p(child, sphere_tree::CHILD_RADIUS));
    c.sub_s(0, 0, 1);
    c.mfc1(A0, 0);
    c.vscale(p(s, 0x10), A0);
    c.vstore(5, p(s, 0x40), V1, &[p(s, 0x30)]);
    c.vsub(p(child, sphere_tree::CHILD_CENTER), p(s, 0x30));
    let point = r(c, A3);
    c.vstore(5, p(s, 0x40), V1, &[point, p(s, 0x20)]);
}


// ---------------------------------------------------------------------------
// Forward-difference cache.

pub(crate) const FD_ROW: u32 = 0x0032_bbf0;

pub(crate) const FD_CELL: u32 = 0x0032_bc28;


/// The triangle normal of `(a, b, d)` on VU0: `vf5 = normalize((a - b) x
/// (b - d))`, `vf10.w = 1 / |n|`, `v0 = vf10` (`qmfc2`).
pub(crate) fn triangle_normal_regs(c: &mut Cpu, a: u32, b: u32, d: u32) {
    c.lqc2(2, a);
    c.lqc2(3, b);
    c.lqc2(4, d);
    let v = c.vu0();
    v.sub(0b1111, 5, 2, Src::Vec(3));
    v.sub(0b1111, 6, 3, Src::Vec(4));
    v.opmula(5, 6);
    v.opmsub(7, 6, 5);
    v.sub(0b0001, 7, 7, Src::Vec(7));
    v.add(0b1000, 8, 0, Src::Bc(0, W));
    v.mul(0b1111, 9, 7, Src::Vec(7));
    v.adda(0b1000, 9, Src::Bc(9, Y));
    v.madda(0b1000, 8, Src::Bc(9, Z));
    v.madd(0b1000, 9, 8, Src::Bc(9, W));
    v.rsqrt(0, 3, 9, 0);
    v.waitq();
    v.mul(0b1111, 5, 7, Src::Q);
    v.add(0b1000, 10, 0, Src::Bc(0, W));
    v.mul(0b1111, 10, 10, Src::Q);
    c.qmfc2(V0, 10);
}


/// The box of the quad `(a, b, d, e)` into `cell+0x20` (min) / `+0x30`
/// (max).
pub(crate) fn quad_box(c: &mut Cpu, a: u32, b: u32, d: u32, e: u32, cell_at: u32) {
    c.lqc2(2, a);
    c.lqc2(3, b);
    c.lqc2(4, d);
    c.lqc2(5, e);
    let v = c.vu0();
    v.max(0b1111, 6, 2, Src::Vec(3));
    v.mini(0b1111, 7, 2, Src::Vec(3));
    v.max(0b1111, 8, 4, Src::Vec(5));
    v.mini(0b1111, 9, 4, Src::Vec(5));
    v.max(0b1111, 6, 6, Src::Vec(8));
    v.mini(0b1111, 7, 7, Src::Vec(9));
    c.sqc2(6, p(cell_at, cell::BOX_MAX));
    c.sqc2(7, p(cell_at, cell::BOX_MIN));
}


/// The volume's UV refine (virtual 0x40) for kind-2 volumes: `vt[0x40](
/// vol, key, &u, &v, a3', t0', t1')` with the saved t5-t7.
pub(crate) fn uv_refine(c: &mut Cpu, uv: u32, ret: u32) -> Flow {
    c.cycles += 11;
    c.lw_r(V0, at(c, S3, volume::VTABLE));
    c.g[T0][0] = c.g[T5][0];
    let s = sp_2(c);
    c.lw_r(A1, p(s, 0x30));
    c.g[T1][0] = c.g[T6][0];
    c.g[T2][0] = c.g[T7][0];
    c.set_r(A2, p(s, uv));
    c.set_r(A3, p(s, uv + 4));
    vcall(c, S3, V0, 0x40, V1, ret)
}


/// The volume's triangle test (virtual 0x20) with t4-t7 (`t4` only in the
/// grid scan) saved around it; v0 = hit.
pub(crate) fn triangle_test(c: &mut Cpu, save_t4: bool, ret: u32) -> Flow {
    let s = sp_2(c);
    if save_t4 {
        c.sq(T4, p(s, 0x50));
    }
    c.sq(T5, p(s, 0x60));
    c.sq(T6, p(s, 0x70));
    c.sq(T7, p(s, 0x80));
    vcall(c, S3, V1, 0x20, V0, ret)?;
    let s = sp_2(c);
    if save_t4 {
        c.lq(T4, p(s, 0x50));
    }
    c.lq(T5, p(s, 0x60));
    c.lq(T6, p(s, 0x70));
    c.lq(T7, p(s, 0x80));
    Ok(())
}


/// Facing test of a triangle normal at `n` against the volume direction:
/// when the volume asks for it (`+8`), `f{fr} = dot(n, dir)` and `c.le.s
/// fr, limit` (`limit` loaded from `gp + limit_at` first when given). True
/// when the triangle is to be tested; leaves v1 = the volume vtable then.
pub(crate) fn faces_volume(c: &mut Cpu, n: u32, fr: usize, limit: usize, limit_at: Option<i32>, cycles: u64) -> bool {
    c.cycles += 3;
    c.lw_r(V0, at(c, S3, volume::FACING));
    if c.g[V0][0] != 0 {
        c.cycles += cycles;
        if let Some(off) = limit_at {
            c.lwc1(limit, gp(c, off));
        }
        let dir = at(c, S3, volume::DIR);
        c.lqc2(3, n);
        c.lqc2(5, dir);
        c.vdot_regs(5);
        c.qmfc2(V0, 4);
        c.mtc1(V0, fr);
        c.c_le_s(fr, limit);
        if !c.fcc() {
            return false;
        }
        c.cycles += 1;
    }
    c.lw_r(V1, at(c, S3, volume::VTABLE));
    true
}


/// `bool AIFwdDiffCache_CollideEntry(Volume* vol, int key, Entry* e, a3,
/// t0, t1, float uv[2], Hint* hint)` (0x32b6e0): the forward-difference
/// grid of a cache entry (fine 3x3 cells of 0x15 vertices, or coarse 9x9
/// of 1) against a volume. The hint's cell (same key and kind) is tried
/// first; otherwise every cell whose box overlaps the volume's box has its
/// two triangle normals computed on first use (degenerate ones marked) and
/// its triangles, when facing, handed to the volume's triangle test. A hit
/// writes the cell's UV (refined by kind-2 volumes) and the hint, and
/// returns true.
pub fn ai_fwd_diff_cache_collide_entry(c: &mut Cpu) {
    const SAVED: &[(usize, u32)] = &[
        (S1, 0x110),
        (S3, 0xf0),
        (S6, 0xc0),
        (S0, 0x120),
        (S2, 0x100),
        (S4, 0xe0),
        (S5, 0xd0),
        (S7, 0xb0),
        (FP, 0xa0),
    ];
    const FSAVED: &[(usize, u32)] = &[
        (25, 0x158),
        (24, 0x150),
        (23, 0x148),
        (22, 0x140),
        (21, 0x138),
        (20, 0x130),
    ];
    guest(c, |c| {
        c.cycles += 27;
        prologue(c, 0x160, SAVED, 0x90, FSAVED);
        c.g[T5][0] = c.g[A3][0];
        c.g[T6][0] = c.g[T0][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[T7][0] = c.g[T1][0];
        c.g[S6][0] = c.g[T3][0];
        let s = sp_2(c);
        c.sw_r(p(s, 0x30), A1);
        c.lw_r(V0, at(c, S3, volume::KIND));
        c.sw_r(p(s, 0x34), T2);
        // The grid: sp_2+0x38 cells per side, sp_2+0x3c vertex rows per cell
        // row, s2 vertex step, s0 the cells, f22/f23 the UV scale/offset.
        if c.g[V0][0] == 0 {
            c.cycles += 9;
            c.g[A2][0] = 3;
            c.g[V0][0] = 0x15;
            c.g[S2][0] = 3;
            c.sw_r(p(s, 0x3c), V0);
            c.sw_r(p(s, 0x38), A2);
            c.lwc1(22, gp(c, -0x2dc0));
            c.lwc1(23, gp(c, -0x2dbc));
            c.set_r(S0, at(c, S1, 0x1a90));
        } else {
            c.cycles += 8;
            c.g[A2][0] = 1;
            c.g[V1][0] = 9;
            c.lwc1(22, gp(c, -0x2db8));
            c.g[S2][0] = 1;
            c.lwc1(23, gp(c, -0x2db4));
            c.set_r(S0, at(c, S1, 0x650));
            c.sw_r(p(s, 0x3c), A2);
            c.sw_r(p(s, 0x38), V1);
        }
        let hit = fd_try_hint(c)?;
        if !hit {
            fd_scan(c)?;
        }
        c.cycles += 18;
        epilogue(
            c,
            &[
                (S0, 0x120),
                (S1, 0x110),
                (S2, 0x100),
                (S3, 0xf0),
                (S4, 0xe0),
                (S5, 0xd0),
                (S6, 0xc0),
                (S7, 0xb0),
                (FP, 0xa0),
            ],
            &[
                (25, 0x158),
                (24, 0x150),
                (23, 0x148),
                (22, 0x140),
                (21, 0x138),
                (20, 0x130),
            ],
            0x90,
            0x160,
        );
        Ok(())
    })
}


/// The hint's cell first. True (v0 = 1) on a hit; otherwise the hint is
/// cleared (when it was this entry's) and v0 = cells per side.
pub(crate) fn fd_try_hint(c: &mut Cpu) -> Result<bool, Unwound> {
    let s = sp_2(c);
    c.cycles += 2;
    let no_hint = c.g[S6][0] == 0;
    c.lw_r(V1, p(s, 0x30));
    if no_hint {
        c.cycles += 1;
        c.lw_r(V0, p(s, 0x38));
        return Ok(false);
    }
    c.cycles += 3;
    c.lw_r(V0, at(c, S6, hint::KEY));
    let other_key = c.g[V0][0] != c.g[V1][0];
    c.lw_r(V0, p(s, 0x38));
    if other_key {
        return Ok(false);
    }
    c.cycles += 4;
    c.lw_r(V1, at(c, S6, hint::KIND));
    c.lw_r(V0, at(c, S3, volume::KIND));
    if c.g[V1][0] != c.g[V0][0] {
        c.lw_r(V0, p(s, 0x38));
        return Ok(false);
    }
    // The cell (t1) and its quad's vertices a3, a2, a0, a1.
    c.cycles += 2;
    let coarse = c.g[V1][0] != 0;
    c.g[V1][0] = c.lhu(at(c, S6, hint::ROW));
    if !coarse {
        c.cycles += 6;
        c.g[V0][0] = 0xc0;
        c.g[A0][0] = c.lhu(at(c, S6, hint::COL));
        c.mult(0, V0, V1, V0);
        c.set_r(A1, r(c, A0) << 6);
        c.inc_r(V0, 0x1a90);
    } else {
        c.cycles += 5;
        c.g[V0][0] = 0x240;
        c.g[A0][0] = c.lhu(at(c, S6, hint::COL));
        c.mult(0, V0, V1, V0);
        c.set_r(A1, r(c, A0) << 6);
        c.inc_r(V0, 0x650);
    }
    c.cycles += 19;
    c.set_r(V0, r(c, S1).wrapping_add(r(c, V0)));
    c.set_r(T1, r(c, V0).wrapping_add(r(c, A1)));
    c.mult(0, V0, V1, A2);
    c.lwc1(0, at(c, T1, cell::BOX_MIN));
    c.mult(0, V1, A0, S2);
    c.lwc1(1, gp(c, -0x2db0));
    c.g[A0][0] = 0xa0;
    c.set_r(A1, r(c, S2) << 4);
    c.mult(0, A2, S2, A0);
    c.c_eq_s(0, 1);
    c.mult(0, V0, V0, A0);
    c.set_r(V1, r(c, V1) << 4);
    c.inc_r(V0, 0x10);
    c.set_r(V0, r(c, S1).wrapping_add(r(c, V0)));
    c.set_r(A3, r(c, V0).wrapping_add(r(c, V1)));
    c.set_r(A2, r(c, A3).wrapping_add(r(c, A2)));
    c.set_r(A0, r(c, A3).wrapping_add(r(c, A1)));
    let no_box = c.fcc();
    c.set_r(A1, r(c, A2).wrapping_add(r(c, A1)));
    if no_box {
        c.cycles += 12;
        let (a3, a2, a0, a1, cl) = (r(c, A3), r(c, A2), r(c, A0), r(c, A1), r(c, T1));
        quad_box(c, a3, a2, a0, a1, cl);
    }
    // The first triangle when the hint says so, else the second.
    c.cycles += 3;
    c.lw_r(V0, at(c, S6, hint::FIRST));
    let cl = r(c, T1);
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.lwc1(0, cl);
        c.c_eq_s(0, 1);
        if c.fcc() {
            c.cycles += 27;
            let (a1, a2, a0) = (r(c, A1), r(c, A2), r(c, A0));
            triangle_normal_regs(c, a1, a2, a0);
            c.sqc2(5, cl);
            c.g[AT][0] = 0x4120_0000; // 10.0
            c.mtc1(AT, 0);
            c.mtc1(V0, 1);
            c.c_lt_s(0, 1);
            let tiny = c.fcc();
            let s = sp_2(c);
            c.sw_r(s, V0);
            if tiny {
                c.cycles += 2;
                c.lwc1(0, gp(c, -0x2dac));
                c.swc1(0, cl);
            }
            c.cycles += 1;
        }
        c.lwc1(1, cl);
        c.cycles += 5;
        c.lwc1(0, gp(c, -0x2da8));
        c.c_eq_s(1, 0);
        if c.fcc() {
            c.sw(r(c, S6), 0);
            fd_clear_hint(c);
            return Ok(false);
        }
        if !faces_volume(c, cl, 1, 0, Some(-0x2da4), 14) {
            c.sw(r(c, S6), 0);
            fd_clear_hint(c);
            return Ok(false);
        }
        c.cycles += 16;
        c.g[A3][0] = c.g[A0][0];
        c.g[T0][0] = c.g[T1][0];
        c.g[T2][0] = c.g[T6][0];
        c.g[T1][0] = c.g[T5][0];
        c.g[T3][0] = c.g[T7][0];
        triangle_test(c, false, 0x0032_b988)?;
        if c.g[V0][0] == 0 {
            fd_drop_hint(c);
            return Ok(false);
        }
        fd_hint_uv(c, 0x10, 0x0032_ba00, 15, 4)?;
        return Ok(true);
    }
    c.lwc1(0, p(cl, cell::NORMAL1));
    c.cycles += 4;
    c.c_eq_s(0, 1);
    if c.fcc() {
        c.cycles += 27;
        let (a0, a2, a3) = (r(c, A0), r(c, A2), r(c, A3));
        triangle_normal_regs(c, a0, a2, a3);
        c.sqc2(5, p(cl, cell::NORMAL1));
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(AT, 0);
        c.mtc1(V0, 1);
        c.c_lt_s(0, 1);
        let tiny = c.fcc();
        let s = sp_2(c);
        c.sw_r(s, V0);
        if tiny {
            c.cycles += 2;
            c.lwc1(0, gp(c, -0x2da0));
            c.swc1(0, p(cl, cell::NORMAL1));
        }
        c.cycles += 1;
    }
    c.lwc1(1, p(cl, cell::NORMAL1));
    c.cycles += 5;
    c.lwc1(0, gp(c, -0x2d9c));
    c.c_eq_s(1, 0);
    let degenerate = c.fcc();
    c.set_r(T0, p(cl, cell::NORMAL1));
    if degenerate {
        fd_drop_hint(c);
        return Ok(false);
    }
    if !faces_volume(c, p(cl, cell::NORMAL1), 1, 0, Some(-0x2d98), 14) {
        c.sw(r(c, S6), 0);
        fd_clear_hint(c);
        return Ok(false);
    }
    c.cycles += 15;
    c.g[A1][0] = c.g[A0][0];
    c.g[T1][0] = c.g[T5][0];
    c.g[T2][0] = c.g[T6][0];
    c.g[T3][0] = c.g[T7][0];
    triangle_test(c, false, 0x0032_bb20)?;
    if c.g[V0][0] == 0 {
        fd_drop_hint(c);
        return Ok(false);
    }
    fd_hint_uv(c, 0x18, 0x0032_bb98, 15, 3)?;
    Ok(true)
}


/// The hint cell's UV (`sp_2+uv`), refined for kind-2 volumes, written to
/// the caller's `uv`; v0 = 1.
pub(crate) fn fd_hint_uv(c: &mut Cpu, uv: u32, ret: u32, cycles: u64, tail: u64) -> Flow {
    c.cycles += cycles;
    let s = sp_2(c);
    c.g[V0][0] = c.lhu(at(c, S6, hint::COL));
    c.g[A1][0] = 2;
    c.g[V1][0] = c.lhu(at(c, S6, hint::ROW));
    c.mtc1(V0, 1);
    c.cvt_s_w(1, 1);
    c.lw_r(A0, at(c, S3, volume::KIND));
    c.mtc1(V1, 0);
    c.cvt_s_w(0, 0);
    c.mul_s(1, 1, 22);
    c.mul_s(0, 0, 22);
    c.add_s(1, 1, 23);
    c.add_s(0, 0, 23);
    c.swc1(1, p(s, uv + 4));
    let refine = c.g[A0][0] == c.g[A1][0];
    c.swc1(0, p(s, uv));
    if refine {
        uv_refine(c, uv, ret)?;
    }
    c.cycles += tail;
    let s = sp_2(c);
    c.lwc1(1, p(s, uv));
    c.g[V0][0] = 1;
    c.lwc1(0, p(s, uv + 4));
    c.cycles += 4;
    c.lw_r(V1, p(s, 0x34));
    let out = r(c, V1);
    c.swc1(1, out);
    c.swc1(0, p(out, 4));
    Ok(())
}


/// `hint->key = 0`, then [`fd_clear_hint`].
pub(crate) fn fd_drop_hint(c: &mut Cpu) {
    c.cycles += 1;
    c.sw(r(c, S6), 0);
    fd_clear_hint(c);
}


/// Clear the rest of the hint; v0 = cells per side.
pub(crate) fn fd_clear_hint(c: &mut Cpu) {
    c.cycles += 4;
    let h = r(c, S6);
    c.sh(p(h, hint::ROW), 0);
    c.sh(p(h, hint::COL), 0);
    c.sw(p(h, hint::FIRST), 0);
    c.sw(p(h, hint::KIND), 0);
    c.cycles += 1;
    let s = sp_2(c);
    c.lw_r(V0, p(s, 0x38));
}


/// Scan every cell (v0 = cells per side on entry); v0 = 1 on a hit, else 0.
pub(crate) fn fd_scan(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.set_r(S4, at(c, S1, 0x10));
    let empty = c.g[V0][0] == 0;
    c.g[FP][0] = 0;
    if empty {
        c.cycles += 1;
        c.g[V0][0] = 0;
        return Ok(());
    }
    c.cycles += 6;
    let s = sp_2(c);
    c.set_r(V1, r(c, S2) << 4);
    c.g[V0][0] = 0xa0;
    c.sw_r(p(s, 0x44), V1);
    c.mult(0, V1, S2, V0);
    c.sw_r(p(s, 0x40), V1);
    c.lw_r(V0, p(s, 0x40));
    loop {
        // Row fp: the quad corners s4, s1 (next row), s2, s5.
        c.cycles += 7;
        c.g[S7][0] = 0;
        c.lw_r(V1, p(s, 0x44));
        c.set_r(S1, r(c, S4).wrapping_add(r(c, V0)));
        c.lw_r(V0, p(s, 0x38));
        c.set_r(S2, r(c, S4).wrapping_add(r(c, V1)));
        let no_cells = c.g[V0][0] == 0;
        c.set_r(S5, r(c, S1).wrapping_add(r(c, V1)));
        if !no_cells {
            c.cycles += 7;
            c.lwc1(21, gp(c, -0x2d94));
            c.g[T4][0] = c.g[V1][0];
            c.g[AT][0] = 0x4120_0000; // 10.0
            c.mtc1(AT, 25);
            c.lwc1(20, gp(c, -0x2d90));
            c.lwc1(24, gp(c, -0x2d8c));
            loop {
                if fd_cell(c)? {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    return Ok(());
                }
                c.cycles += 8;
                c.inc_r(S7, 1);
                let step = r(c, T4);
                c.inc_r(S4, step);
                c.inc_r(S1, step);
                c.inc_r(S2, step);
                c.inc_r(S5, step);
                c.g[V0][0] = slt(c, S7, V1);
                let more = c.g[V0][0] != 0;
                c.inc_r(S0, cell::SIZE);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, FD_CELL);
                }
            }
        }
        c.cycles += 8;
        c.lw_r(V0, p(s, 0x3c));
        c.inc_r(FP, 1);
        c.set_r(V1, r(c, V0) << 4);
        c.set_r(S4, r(c, S4).wrapping_add(r(c, V1)));
        c.lw_r(V1, p(s, 0x38));
        c.g[V0][0] = slt(c, FP, V1);
        let more = c.g[V0][0] != 0;
        c.lw_r(V0, p(s, 0x40));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, FD_ROW);
        }
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
    Ok(())
}


/// One cell (s0) of the scan: box, overlap with the volume, both
/// triangles. True on a hit (UV and hint written); otherwise v1 = cells
/// per side.
pub(crate) fn fd_cell(c: &mut Cpu) -> Result<bool, Unwound> {
    let cl = r(c, S0);
    let vol = r(c, S3);
    c.cycles += 5;
    c.lwc1(0, p(cl, cell::BOX_MIN));
    c.c_eq_s(0, 21);
    if c.fcc() {
        c.cycles += 14;
        let (s4, s1, s2, s5) = (r(c, S4), r(c, S1), r(c, S2), r(c, S5));
        quad_box(c, s4, s1, s2, s5, cl);
        c.lwc1(1, p(vol, 0x20));
        c.lwc1(0, p(cl, cell::BOX_MIN));
    } else {
        c.lwc1(1, p(vol, 0x20));
    }
    // Box overlap: cell min < vol max and vol min < cell max on each axis.
    let mut overlap = false;
    c.cycles += 4;
    c.c_lt_s(0, 1);
    c.g[V0][0] = 0;
    if c.fcc() {
        let tests: [(u32, u32, bool, u64); 5] = [
            (0x30, cell::BOX_MAX, true, 5),
            (0x24, cell::BOX_MIN + 4, false, 5),
            (0x34, cell::BOX_MAX + 4, true, 5),
            (0x28, cell::BOX_MIN + 8, false, 5),
            (0x38, cell::BOX_MAX + 8, true, 6),
        ];
        overlap = true;
        for (i, &(voff, coff, rev, cyc)) in tests.iter().enumerate() {
            c.cycles += cyc;
            c.lwc1(1, p(vol, voff));
            c.lwc1(0, p(cl, coff));
            if rev {
                c.c_lt_s(1, 0);
            } else {
                c.c_lt_s(0, 1);
            }
            if !c.fcc() {
                overlap = false;
                break;
            }
            if i == 4 {
                c.g[V0][0] = 1;
            }
        }
    }
    c.cycles += 2;
    let s = sp_2(c);
    c.lw_r(V1, p(s, 0x38));
    if !overlap {
        return Ok(false);
    }
    // First triangle (s5, s1, s2).
    c.cycles += 4;
    c.lwc1(0, cl);
    c.c_eq_s(0, 21);
    if c.fcc() {
        c.cycles += 25;
        let (s5, s1, s2) = (r(c, S5), r(c, S1), r(c, S2));
        triangle_normal_regs(c, s5, s1, s2);
        c.sqc2(5, cl);
        c.mtc1(V0, 0);
        c.c_lt_s(25, 0);
        let tiny = c.fcc();
        c.sw_r(s, V0);
        if tiny {
            c.cycles += 1;
            c.swc1(20, cl);
        }
        c.cycles += 1;
        c.lwc1(0, cl);
    }
    c.cycles += 4;
    c.c_eq_s(0, 20);
    let mut tested = false;
    if !c.fcc() && faces_volume(c, cl, 0, 24, None, 13) {
        c.cycles += 20;
        c.g[T1][0] = c.g[T5][0];
        c.g[T2][0] = c.g[T6][0];
        c.g[T3][0] = c.g[T7][0];
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[T0][0] = c.g[S0][0];
        triangle_test(c, true, 0x0032_be00)?;
        tested = true;
        if c.g[V0][0] != 0 {
            fd_cell_hit(c, 0x20, 0x0032_be74, true)?;
            return Ok(true);
        }
    }
    if tested {
        c.cycles += 1;
    }
    c.lwc1(0, p(cl, cell::NORMAL1));
    // Second triangle (s2, s1, s4).
    c.cycles += 4;
    c.c_eq_s(0, 21);
    if c.fcc() {
        c.cycles += 25;
        let (s2, s1, s4) = (r(c, S2), r(c, S1), r(c, S4));
        triangle_normal_regs(c, s2, s1, s4);
        c.sqc2(5, p(cl, cell::NORMAL1));
        c.mtc1(V0, 0);
        c.c_lt_s(25, 0);
        let tiny = c.fcc();
        c.sw_r(s, V0);
        if tiny {
            c.cycles += 1;
            c.swc1(20, p(cl, cell::NORMAL1));
        }
        c.cycles += 1;
        c.lwc1(0, p(cl, cell::NORMAL1));
    } else {
        c.lwc1(0, p(cl, cell::NORMAL1));
    }
    c.cycles += 4;
    c.c_eq_s(0, 20);
    let degenerate = c.fcc();
    c.set_r(T0, p(cl, cell::NORMAL1));
    if degenerate {
        c.cycles += 1;
        c.lw_r(V1, p(s, 0x38));
        return Ok(false);
    }
    // Facing (the original reloads v1 = cells per side when not facing).
    c.cycles += 3;
    c.lw_r(V0, p(vol, volume::FACING));
    if c.g[V0][0] != 0 {
        c.cycles += 13;
        c.lqc2(3, p(cl, cell::NORMAL1));
        c.lqc2(5, p(vol, volume::DIR));
        c.vdot_regs(5);
        c.qmfc2(V0, 4);
        c.mtc1(V0, 0);
        c.c_le_s(0, 24);
        let facing = c.fcc();
        c.lw_r(V1, p(s, 0x38));
        if !facing {
            return Ok(false);
        }
        c.cycles += 1;
    }
    c.lw_r(V1, p(vol, volume::VTABLE));
    c.cycles += 19;
    c.g[T1][0] = c.g[T5][0];
    c.g[T2][0] = c.g[T6][0];
    c.g[T3][0] = c.g[T7][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S4][0];
    triangle_test(c, true, 0x0032_bfb4)?;
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        c.lw_r(V1, p(s, 0x38));
        return Ok(false);
    }
    fd_cell_hit(c, 0x28, 0x0032_c028, false)?;
    Ok(true)
}


/// A hit in the scan: the cell's UV at `sp_2+uv` (refined for kind-2
/// volumes) to the caller's `uv`, the hint (`first`: the cell's first
/// triangle) with the volume kind.
pub(crate) fn fd_cell_hit(c: &mut Cpu, uv: u32, ret: u32, first: bool) -> Flow {
    c.cycles += 13;
    let s = sp_2(c);
    c.mtc1(FP, 0);
    c.cvt_s_w(0, 0);
    c.lw_r(V0, at(c, S3, volume::KIND));
    c.mtc1(S7, 1);
    c.cvt_s_w(1, 1);
    c.g[V1][0] = 2;
    c.mul_s(0, 0, 22);
    c.mul_s(1, 1, 22);
    c.add_s(0, 0, 23);
    c.add_s(1, 1, 23);
    c.swc1(0, p(s, uv));
    let refine = c.g[V0][0] == c.g[V1][0];
    c.swc1(1, p(s, uv + 4));
    if refine {
        uv_refine(c, uv, ret)?;
    }
    c.cycles += 6;
    let s = sp_2(c);
    c.lwc1(0, p(s, uv));
    c.lwc1(1, p(s, uv + 4));
    c.lw_r(V0, p(s, 0x34));
    let out = r(c, V0);
    c.swc1(0, out);
    let has_hint = c.g[S6][0] != 0;
    c.swc1(1, p(out, 4));
    if has_hint {
        let h = r(c, S6);
        if first {
            c.cycles += 7;
            c.lw_r(V0, p(s, 0x30));
            c.g[V1][0] = 1;
            c.sh(p(h, hint::ROW), c.g[FP][0]);
            c.sw_r(h, V0);
            c.sh(p(h, hint::COL), c.g[S7][0]);
            c.sw_r(p(h, hint::FIRST), V1);
        } else {
            c.cycles += 5;
            c.lw_r(V1, p(s, 0x30));
            c.sh(p(h, hint::ROW), c.g[FP][0]);
            c.sw_r(h, V1);
            c.sh(p(h, hint::COL), c.g[S7][0]);
            c.sw(p(h, hint::FIRST), 0);
        }
        c.cycles += 2;
        c.lw_r(V0, at(c, S3, volume::KIND));
        c.sw_r(p(h, hint::KIND), V0);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Segment against a triangle (inlined VU0 dot/add/scale sequences).

/// `int CollSegment_CollideTriangle(sSegTree* self, sVec4* p,
/// void* c, sVec4* n, sVec4* hit, sVec4* outN, float* outT)` (0x0032e4d0):
/// the tree's segment `{origin +0x60, dir +0x70}` against the plane through
/// `p` with normal `n`. Fills the hit point and the hit normal and returns
/// the triangle test at 0x32b6a8; 0 when the segment is parallel, misses
/// behind the origin or goes past its end (`t` is still stored).
pub fn coll_segment_collide_triangle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x40);
        c.g[T4][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[A0][0] = c.g[A1][0];
        // denom = dot(dir, n) (the unit's VU0 dot sequence)
        let tree = c.g[T4][0] as u32;
        c.lqc2(3, p(tree, 0x70));
        c.lqc2(5, c.g[T0][0] as u32);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 2);
        c.lwc1(1, gp_2(c, -0x2d7c));
        c.abs_s(0, 2);
        c.g[A1][0] = c.g[A2][0];
        c.c_lt_s(0, 1);
        let parallel = c.fcc();
        c.g[A2][0] = c.g[A3][0];
        let mut miss = parallel;
        if !parallel {
            c.cycles += 21;
            // t = dot(p - origin, n) / denom
            c.lqc2(3, c.g[A0][0] as u32);
            c.lqc2(4, p(tree, 0x60));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, p(sp_3(c), 0x10));
            c.mtc1(0, 1);
            c.lq(2, p(sp_3(c), 0x10));
            c.sq(2, sp_3(c));
            c.lqc2(3, sp_3(c));
            c.lqc2(5, c.g[T0][0] as u32);
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 0);
            c.div_s(2, 0, 2);
            c.c_lt_s(2, 1);
            let behind = c.fcc();
            c.swc1(2, c.g[T3][0] as u32);
            if behind {
                miss = true;
            } else {
                c.cycles += 5;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.c_lt_s(0, 2);
                let past_end = !c.fcc();
                if past_end {
                    // On the segment: hit = origin + dir * t, outN = n.
                    c.cycles += 19;
                    c.mfc1(2, 2);
                    c.lqc2(4, p(tree, 0x70));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(sp_3(c), 0x20));
                    c.lwc1(12, gp_2(c, -0x2d78));
                    c.lq(2, p(sp_3(c), 0x20));
                    c.g[A3][0] = c.g[T1][0];
                    c.sq(2, p(sp_3(c), 0x10));
                    c.lqc2(3, p(tree, 0x60));
                    c.lqc2(4, p(sp_3(c), 0x10));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, p(sp_3(c), 0x20));
                    let hit = c.g[T1][0] as u32;
                    c.lq(3, p(sp_3(c), 0x20));
                    c.sq(3, hit);
                    c.sq(3, sp_3(c));
                    c.lq(2, c.g[T0][0] as u32);
                    c.sq(2, c.g[T2][0] as u32); // *outN = *n
                    crate::call::call_ptr(c, 0x0032_b6a8, 0x0032_e5dc)?;
                } else {
                    miss = true;
                }
            }
        }
        if miss {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}


/// `swc1 fN, a`.
#[inline(always)]
pub(crate) fn stf(c: &mut Cpu, n: usize, a: u32) {
    let v = fr(c, n);
    Addr(a).write_fpu(c, v);
}


/// The generated loop back-edge event check (`if cycles >= next_event`).
#[inline(always)]
pub(crate) fn event_check(c: &mut Cpu, head: u32) {
    if c.cycles >= c.next_event {
        c.pc = head;
        crate::events::deliver_event_here(c, head);
    }
}


/// `void RiderSphereTree_GetBounds(tree, Vec4* mn, Vec4* mx)` (0x329dc8):
/// `*mn`/`*mx` start as ±1e10 (w = 1). A leaf node (mask 0) yields its own
/// sphere: centre ∓/± radius with w = centre.w. Otherwise the box grows over
/// the children its mask selects, one float at a time. The VU0 difference and
/// sum go through `vf3`/`vf4` into `vf5`; every test uses the FPU condition
/// bit.
pub fn rider_sphere_tree_get_bounds(c: &mut Cpu) {
    use sphere_tree::*;
    c.cycles += 19;
    let frame = Frame::open(c, 0x40);
    let tree = r32(c, A0);
    let mn = r32(c, A1);
    let mx = r32(c, A2);
    let sp0 = sp(c);
    // mn = (1e10, 1e10, 1e10, 1.0)
    ldf(c, 0, gp_3(c, -0x2de8));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    stf(c, 0, p(sp0, 8));
    stf(c, 0, sp0);
    stf(c, 0, p(sp0, 4));
    stf(c, 1, p(sp0, 0xc));
    // mx = (-1e10, -1e10, -1e10, 1.0)
    ldf(c, 0, gp_3(c, -0x2de4));
    c.lq(3, sp0);
    stf(c, 0, p(sp0, 8));
    stf(c, 0, sp0);
    stf(c, 0, p(sp0, 4));
    c.sq(3, mn);
    c.lq(2, sp0);
    c.sq(2, mx);
    c.g[V1][0] = lw(c, p(tree, MASK));
    if c.g[V1][0] != 0 {
        c.g[V0][0] = lw(c, p(tree, COUNT));
    } else {
        c.cycles += 25;
        // mn = centre - (r, r, r, 0)
        ldf(c, 0, p(tree, RADIUS));
        c.sw(p(sp0, 0x1c), 0);
        stf(c, 0, p(sp0, 0x18));
        stf(c, 0, p(sp0, 0x10));
        stf(c, 0, p(sp0, 0x14));
        c.lqc2(3, p(tree, CENTER));
        c.lqc2(4, p(sp0, 0x10));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(sp0, 0x20));
        c.lq(2, p(sp0, 0x20));
        c.sw(p(sp0, 0x1c), 0);
        c.sq(2, mn);
        c.sq(2, sp0);
        // mx = centre + (r, r, r, 0)
        ldf(c, 0, p(tree, RADIUS));
        stf(c, 0, p(sp0, 0x18));
        stf(c, 0, p(sp0, 0x10));
        stf(c, 0, p(sp0, 0x14));
        c.lqc2(3, p(tree, CENTER));
        c.lqc2(4, p(sp0, 0x10));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(sp0, 0x20));
        c.lq(2, p(sp0, 0x20));
        c.sq(2, mx);
        c.sq(2, sp0); // also mirrored into the frame's low quadword
        // A leaf returns straight to the epilogue: the count test and the
        // child loop belong to the masked-node path only.
        c.cycles += 2;
        frame.close(c);
        return;
    }
    c.cycles += 2;
    c.g[T0][0] = 0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 4;
        c.mov_s(2, 1); // f2 = 1.0 (the children's w)
        c.g[T1][0] = 1;
        c.g[A3][0] = c.g[A0][0];
        loop {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(tree, MASK));
            c.g[V1][0] = sx((c.g[T1][0] as u32) << ((c.g[T0][0] as u32) & 31));
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            if c.g[V0][0] != 0 {
                accumulate_kid(c, r32(c, A3), mn, mx);
                c.cycles += 1;
                c.g[V0][0] = lw(c, p(tree, COUNT));
            } else {
                c.g[V0][0] = lw(c, p(tree, COUNT));
            }
            c.cycles += 4;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(KID_STRIDE));
            if !more {
                break;
            }
            event_check(c, 0x0032_9e90);
        }
    }
    c.cycles += 2;
    frame.close(c);
}


/// Grow `mn`/`mx` by child `kid`'s sphere: `pos - r` against the minimum
/// (17 cycles, each further axis test 6) and `pos + r` against the maximum
/// (13, each further axis test 6). The radius quadword `(r, r, r, 1)` sits at
/// the frame's `sp + 0` throughout.
pub(crate) fn accumulate_kid(c: &mut Cpu, kid: u32, mn: u32, mx: u32) {
    use sphere_tree::*;
    let sp0 = sp(c);
    c.cycles += 17;
    ldf(c, 0, p(kid, KID_RADIUS));
    stf(c, 2, p(sp0, 0xc));
    stf(c, 0, p(sp0, 8));
    stf(c, 0, sp0);
    stf(c, 0, p(sp0, 4));
    c.lqc2(3, p(kid, KID_POS));
    c.lqc2(4, sp0);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp0, 0x20));
    c.lq(2, p(sp0, 0x20));
    ldf(c, 0, mn); // f0 = mn.x
    c.sq(2, p(sp0, 0x10));
    ldf(c, 1, p(sp0, 0x10)); // f1 = t.x
    if lt(c, 1, 0) {
        stf(c, 1, mn);
    }
    c.cycles += 6;
    ldf(c, 1, p(sp0, 0x14));
    ldf(c, 0, p(mn, 4));
    if lt(c, 1, 0) {
        stf(c, 1, p(mn, 4));
    }
    c.cycles += 6;
    ldf(c, 1, p(sp0, 0x18));
    ldf(c, 0, p(mn, 8));
    if lt(c, 1, 0) {
        stf(c, 1, p(mn, 8));
    }
    c.cycles += 13;
    c.lqc2(3, p(kid, KID_POS));
    c.lqc2(4, sp0);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp0, 0x30));
    c.lq(2, p(sp0, 0x30));
    ldf(c, 0, mx); // f0 = mx.x
    c.sq(2, p(sp0, 0x10));
    c.sq(2, p(sp0, 0x20));
    ldf(c, 1, p(sp0, 0x10)); // f1 = t.x
    if lt(c, 0, 1) {
        stf(c, 1, mx);
    }
    c.cycles += 6;
    ldf(c, 1, p(sp0, 0x14));
    ldf(c, 0, p(mx, 4));
    if lt(c, 0, 1) {
        stf(c, 1, p(mx, 4));
    }
    c.cycles += 6;
    ldf(c, 1, p(sp0, 0x18));
    ldf(c, 0, p(mx, 8));
    if lt(c, 0, 1) {
        stf(c, 1, p(mx, 8));
    }
}


// ---------------------------------------------------------------------------
// CollSegment_ctor.

/// `CollSegment* CollSegment_ctor(CollSegment* seg, const Vec4* p0,
/// const Vec4* p1, int state, float r)` (0x32e100): a capsule: the state and
/// vtable (`0x48e6a8`), the start point `p0` (raw and as the base), the
/// direction `p1 - p0`, the radius, the AABB of the two points and its
/// centre.
pub fn coll_segment_ctor(c: &mut Cpu) {
    c.cycles += 26;
    let seg = r32(c, A0);
    c.g[T0][0] = c.g[A0][0];
    c.g[V0][0] = sx(0x490000u32.wrapping_sub(0x1958));
    c.sw(seg, c.g[A3][0]); // state
    c.sw(seg.wrapping_add(0x50), c.g[V0][0]); // vtable
    let frame = Frame::open(c, 0x30);
    let sp = sp(c);
    c.sw(seg.wrapping_add(8), 0);
    c.sw(seg.wrapping_add(4), 0);
    let p0 = r32(c, A1);
    let p1 = r32(c, A2);
    c.lq(2, p0);
    c.sq(2, seg.wrapping_add(0x60)); // p0b
    c.sq(2, seg.wrapping_add(0x80)); // p0
    c.lqc2(3, p1);
    c.lqc2(4, p0);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp.wrapping_add(0x10));
    c.lq(2, sp.wrapping_add(0x10));
    c.swc1(12, seg.wrapping_add(0xa0)); // radius
    c.sq(2, seg.wrapping_add(0x70)); // dirb
    c.sq(2, seg.wrapping_add(0x90)); // dir
    c.sq(2, sp);

    // max = component-wise greater of p0 and p1, w = 1.
    c.lwc1(0, p1);
    c.lwc1(1, p0);
    c.c_lt_s(0, 1);
    let lt = c.fcc();
    c.mov_s(4, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(4, 1);
    }
    c.cycles += 6;
    c.lwc1(0, p1.wrapping_add(4));
    c.lwc1(1, p0.wrapping_add(4));
    c.c_lt_s(0, 1);
    let lt = c.fcc();
    c.mov_s(3, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(3, 1);
    }
    c.cycles += 6;
    c.lwc1(0, p1.wrapping_add(8));
    c.lwc1(1, p0.wrapping_add(8));
    c.c_lt_s(0, 1);
    let lt = c.fcc();
    c.mov_s(2, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(2, 1);
    }
    c.cycles += 14;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 5);
    c.swc1(4, sp);
    c.swc1(3, sp.wrapping_add(4));
    c.swc1(2, sp.wrapping_add(8));
    c.swc1(5, sp.wrapping_add(0xc));
    c.lq(2, sp);
    c.sq(2, seg.wrapping_add(0x20)); // max

    // min = component-wise lesser of p0 and p1, w = 1.
    c.lwc1(0, p1);
    c.lwc1(1, p0);
    c.c_lt_s(1, 0);
    let lt = c.fcc();
    c.mov_s(4, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(4, 1);
    }
    c.cycles += 6;
    c.lwc1(0, p1.wrapping_add(4));
    c.lwc1(1, p0.wrapping_add(4));
    c.c_lt_s(1, 0);
    let lt = c.fcc();
    c.mov_s(3, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(3, 1);
    }
    c.cycles += 6;
    c.lwc1(0, p1.wrapping_add(8));
    c.lwc1(1, p0.wrapping_add(8));
    c.c_lt_s(1, 0);
    let lt = c.fcc();
    c.mov_s(2, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(2, 1);
    }

    // center = 0.5 * (min + max).
    c.cycles += 27;
    c.swc1(4, sp);
    c.g[V0][0] = c.g[T0][0];
    c.swc1(3, sp.wrapping_add(4));
    c.swc1(2, sp.wrapping_add(8));
    c.swc1(5, sp.wrapping_add(0xc));
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(1, 0);
    c.lq(3, sp);
    c.sq(3, seg.wrapping_add(0x30)); // min
    c.lqc2(3, seg.wrapping_add(0x30));
    c.lqc2(4, seg.wrapping_add(0x20));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp.wrapping_add(0x20));
    c.lq(3, sp.wrapping_add(0x20));
    c.sq(3, sp.wrapping_add(0x10));
    c.mfc1(4, 0);
    c.qmtc2(4, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, sp.wrapping_add(0x10));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, sp.wrapping_add(0x20));
    c.lq(3, sp.wrapping_add(0x20));
    c.sq(3, seg.wrapping_add(0x40)); // center
    c.sq(3, sp);
    frame.close(c);
}


// ---------------------------------------------------------------------------
// CollHull_ctor.

/// `CollHull* CollHull_ctor(CollHull* hull, RiderSphereTree* tree, int flag)`
/// (0x32f650): the base at +0x50 (vtable `0x48e590`), the embedded sphere
/// tree node at +0x70, the tree at +0x60/+0x64, the state `flag == 0`, the
/// tree's bounds at +0x20/+0x30 and their centre at +0x40.
pub fn coll_hull_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0x490000;
        frame.save(c, S0, 0x60);
        c.g[V0][0] = sx(0x490000u32.wrapping_sub(0x1a70));
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        let hull = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.sw(hull.wrapping_add(8), 0);
        c.g[A0][0] = sx(hull.wrapping_add(0x70));
        c.sw(hull.wrapping_add(4), 0);
        c.g[S1][0] = c.g[A1][0];
        c.sw(hull.wrapping_add(0x50), c.g[V0][0]); // vtable
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, callee::RIDER_SPHERE_TREE_CTOR, 0x0032_f68c)?;
        c.sw(hull.wrapping_add(0x64), c.g[S1][0]); // tree2
        c.g[S2][0] = (c.g[S2][0] < 1) as u64;
        c.sw(hull.wrapping_add(0x60), c.g[S1][0]); // tree
        c.g[A0][0] = c.g[S1][0];
        c.sw(hull, c.g[S2][0]); // state
        c.g[A1][0] = sx(hull.wrapping_add(0x30));
        c.g[A2][0] = sx(hull.wrapping_add(0x20));
        call_ptr(c, callee::RIDER_SPHERE_TREE_GET_BOUNDS, 0x0032_f6ac)?;

        // center = 0.5 * (min + max).
        c.lqc2(3, hull.wrapping_add(0x30));
        c.lqc2(4, hull.wrapping_add(0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        let sp = sp(c);
        c.sqc2(5, sp.wrapping_add(0x20));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 0); // f0 = 0.5
        c.lq(3, sp.wrapping_add(0x20));
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x50);
        c.sq(3, sp.wrapping_add(0x10));
        c.mfc1(4, 0);
        c.lqc2(4, sp.wrapping_add(0x10));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp.wrapping_add(0x20));
        frame.restore(c, S2, 0x40);
        c.lq(3, sp.wrapping_add(0x20));
        frame.restore_ra(c, 0x30);
        c.sq(3, hull.wrapping_add(0x40)); // center
        frame.restore(c, S0, 0x60);
        c.sq(3, sp);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// VU0_TriangleIntersect.

/// `bool VU0_TriangleIntersect(const Vec4* a, const Vec4* b, const Vec4* c,
/// const Vec4* d)` (0x32b6a8): loads the four points into VF10..VF13 and runs
/// the microprogram at VU address 0x6e0 (`vcallmsr` through VI27); VI2 comes
/// back non-zero when the triangle intersects.
pub fn vu0_triangle_intersect(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V0][0] = 0x0;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6e0));
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
    c.lqc2(10, c.g[A0][0] as u32);
    c.lqc2(11, c.g[A1][0] as u32);
    c.lqc2(12, c.g[A2][0] as u32);
    c.lqc2(13, c.g[A3][0] as u32);
    c.ctc2(V0, 27);
    c.vu0().vnop();
    c.vu0().vnop();
    c.vcallmsr();
    c.cfc2(V0, 2);
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    c.pc = t;
}


// ---------------------------------------------------------------------------
// RiderSphereTree_dtor / CollHull_dtor.

/// `void RiderSphereTree_dtor(RiderSphereTree* t, int flags)` (0x329970): free
/// the object when `flags & 1` (its members are plain data).
pub fn rider_sphere_tree_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A1][0] & 0x1;
        let free = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if free {
            c.cycles += 2;
            callee::DELETE.call(c, 0x0032_9988)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void CollHull_dtor(CollHull* h, int flags)` (0x32f708): restore the base
/// vtable (`+0x50`), destroy the sphere tree at `+0x70`, then free the hull
/// when `flags & 1`.
pub fn coll_hull_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1a70));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let vtable = c.g[V0][0];
        let hull = r32(c, S1);
        Addr(hull.wrapping_add(0x50)).sw(c, vtable);
        c.g[A0][0] = sx(hull.wrapping_add(0x70));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0032_9970, 0x0032_f738)?;
        c.g[S0][0] = c.g[S0][0] & 0x1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0032_f74c)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SphereTree_SweepSphereAABB` (0x329590).
pub(crate) const SPHERE_TREE_SWEEP_AABB: u32 = 0x0032_9590;


// ---------------------------------------------------------------------------
// Sphere tree helpers.

/// `SphereTree* SphereTree_ctor(SphereTree* self)` (0x32c508): clear the
/// count at `+0x98` and reset the root transform; returns `self`.
pub fn sphere_tree_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 7;
        frame.save(c, S0, 0x10);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.sw((this as u32).wrapping_add(0x98), 0);
        c.g[A0][0] = c.g[S0][0];
        callee::SPHERE_TREE_RESET_TRANSFORM.call(c, 0x0032_c52c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `int CollSphere_CollideAABB(coll, a, b, box)` (0x32fe40): test the sphere
/// at `self + 0x60` (radius at `+0x70`) against the AABB in `box` (`+0x0`,
/// `+0x10`, `+0x40`); the callee's result is returned as a bool.
pub fn coll_sphere_collide_aabb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = c.g[A2][0];
        c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
        c.lwc1(12, (c.g[V0][0] as u32).wrapping_add(0x70));
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x60));
        c.g[T1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x40));
        call_ptr(c, SPHERE_TREE_SWEEP_AABB, 0x0032_fe68)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}


/// `base + off` (32-bit address arithmetic).
#[inline(always)]
pub(crate) fn p_2(base: u32, off: u32) -> u32 {
    base.wrapping_add(off)
}


/// `void RiderSphereTree_SetSpheres(RiderSphereTree* self, int n,
/// const Sphere* spheres, const float* radii, const Vec4* center, ...)`
/// (0x329a90): copy up to `n` child sphere bounds, then the root sphere.
pub fn rider_sphere_tree_set_spheres(c: &mut Cpu) {
    c.cycles += 3;
    let this = r32(c, A0);
    Addr(this).off(0x2c).sw(c, c.g[A1][0]);
    let none = (c.g[A1][0] as i64) <= 0;
    c.g[T1][0] = 0;
    if !none {
        c.cycles += 1;
        c.g[V1][0] = c.g[A0][0];
        loop {
            c.cycles += 11;
            c.lq(2, r32(c, A2));
            c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
            c.sq(2, p_2(r32(c, V1), 0x30));
            c.lwc1(0, r32(c, A3));
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
            c.swc1(0, p_2(r32(c, V1), 0x40));
            c.g[V0][0] = Addr(this).off(0x2c).lw(c);
            c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_9aa0);
            }
        }
    }
    c.cycles += 4;
    c.lq(2, r32(c, T0));
    c.swc1(12, p_2(this, 0x20));
    c.sq(2, p_2(this, 0x10));
    abi::ret(c);
}


/// `void SphereTree_ResetTransform(SphereTree* self)` (0x32c540): copy the 8
/// unit sphere nodes into the tree and install the identity transform and a
/// 1.0 radius.
pub fn sphere_tree_reset_transform(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.g[V0][0] = 0x500000;
    c.g[V1][0] = c.g[A0][0];
    c.g[A2][0] = sx(0x004f_f640);
    c.g[A1][0] = 7;
    loop {
        c.cycles += 7;
        c.lq(2, r32(c, A2));
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
        c.sq(2, r32(c, V1));
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x10));
        if !more {
            c.cycles += 8;
            c.g[V0][0] = 0x500000;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[V0][0] = sx(0x004f_f130);
            c.lq(3, r32(c, V0));
            c.swc1(0, p_2(this, 0x90));
            c.sq(3, p_2(this, 0x80));
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_c550);
        }
    }
}


// ---------------------------------------------------------------------------
// Collision.

/// `void CollSphere_Transform(CollSphere* s, const Matrix* m, float scale)`
/// (0x32fd40): transform the sphere centre at `+0x80` by `m` with the VU0
/// macro pipeline, store it at `+0x60`, set `+0x4 = 1` and scale the radius
/// (`+0x90` into `+0x70`).
pub fn coll_sphere_transform(c: &mut Cpu) {
    c.cycles += 21;
    let frame = Frame::open(c, 0x20);
    c.g[V1][0] = 1;
    c.lqc2(8, p_3(c, A0, 0x80));
    c.lqc2(4, abi::r32(c, A1));
    c.lqc2(5, p_3(c, A1, 0x10));
    c.lqc2(6, p_3(c, A1, 0x20));
    c.lqc2(7, p_3(c, A1, 0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, frame.at(c, 0x10));
    c.lwc1(0, p_3(c, A0, 0x90));
    c.lq(2, frame.at(c, 0x10));
    c.mul_s(12, 12, 0);
    c.sw(p_3(c, A0, 4), c.g[V1][0]);
    c.sq(2, p_3(c, A0, 0x60));
    c.sq(2, frame.at(c, 0));
    c.swc1(12, p_3(c, A0, 0x70));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(abi::r32(c, SP).wrapping_add(0x20));
    c.pc = ra;
}


/// `void CollSegment_ResetTransform(CollSegment* self)` (0x32e4b8): copy the
/// transformed endpoints (`+0x80`, `+0x90`) back over the local frame
/// (`+0x60`, `+0x70`) and clear the flags at `+0x4`.
pub fn coll_segment_reset_transform(c: &mut Cpu) {
    c.cycles += 6;
    let this = r32(c, A0);
    c.lq(2, this.wrapping_add(0x80));
    c.lq(3, this.wrapping_add(0x90));
    c.sq(2, this.wrapping_add(0x60));
    c.sq(3, this.wrapping_add(0x70));
    c.sw(this.wrapping_add(0x4), 0);
    abi::ret(c);
}


/// `int CollSegment_CollideAABB(CollSegment* self, float* bmin, float* bmax,
/// CollHit* hits)` (0x32e690): slab test of the segment (`origin` at `+0x60`,
/// `dir` at `+0x70`) against the box `[bmin, bmax]`, writing up to two hit
/// records (0x80 bytes each: normal quad at `+0x10`, position at `+0x0`,
/// distance at `+0x40`, flags/indices around `+0x44..+0x70`).
pub fn coll_segment_collide_aabb(c: &mut Cpu) {
    // Prologue: constants and loop state.
    c.cycles += 16;
    let this = r32(c, A0);
    c.g[T1][0] = c.g[A0][0]; // the segment
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 4); // f4 = -1.0 (tmin)
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(1, 5); // f5 = 2.0 (tmax)
    let sp = r32(c, SP).wrapping_sub(0x30);
    c.g[SP][0] = sx(sp);
    c.lwc1(8, gpa(c, -0x2d74)); // f8 = epsilon
    c.g[T0][0] = sx(this.wrapping_add(0x70)); // pd = &dir
    c.mtc1(0, 7); // f7 = 0.0
    c.g[A0][0] = sx(this.wrapping_add(0x60)); // po = &origin
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 6); // f6 = 1.0
    c.g[T2][0] = u64::MAX; // fmin = -1
    c.g[T3][0] = u64::MAX; // fmax = -1
    c.g[V1][0] = 0;
    'outer: loop {
        'axis: {
            c.cycles += 6;
            c.lwc1(3, r32(c, T0));
            c.abs_s(0, 3);
            c.c_lt_s(8, 0); // epsilon < |d|
            if !c.fcc() {
                // Parallel to this axis: skip it.
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                break 'axis;
            }
            {
                c.cycles += 11;
                let po = r32(c, A0);
                c.lwc1(0, po);
                c.lwc1(1, r32(c, A1));
                c.lwc1(2, r32(c, A2));
                c.sub_s(1, 1, 0);
                c.sub_s(2, 2, 0);
                c.div_s(1, 1, 3);
                c.div_s(2, 2, 3);
                c.c_lt_s(2, 1);
                let ordered = !c.fcc();
                c.g[V0][0] = sx(r32(c, V1) << 1); // face = i * 2
                if !ordered {
                    c.cycles += 4;
                    c.mov_s(0, 1);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                    c.mov_s(1, 2);
                    c.mov_s(2, 0);
                }
            }
            c.cycles += 3;
            c.c_lt_s(4, 1); // tmin < t1
            if c.fcc() {
                c.cycles += 2;
                c.mov_s(4, 1);
                c.g[T2][0] = c.g[V0][0];
            }
            c.cycles += 3;
            c.c_lt_s(2, 5); // t2 < tmax
            if c.fcc() {
                c.cycles += 2;
                c.mov_s(5, 2);
                c.g[T3][0] = c.g[V0][0] ^ 0x1;
            }
            c.cycles += 4;
            c.c_lt_s(5, 4); // tmax < tmin
            let empty = c.fcc();
            c.g[V0][0] = 0;
            if empty {
                break 'outer; // miss
            }
            c.cycles += 3;
            c.c_lt_s(5, 7); // tmax < 0
            if c.fcc() {
                break 'outer; // behind
            }
            c.cycles += 4;
            c.c_lt_s(6, 4); // 1 < tmin
            let behind = c.fcc();
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            if behind {
                break 'outer; // past the end
            }
        }
        c.cycles += 6;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x4));
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x4));
        c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x4));
        c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
        let next_axis = c.g[V0][0] != 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x4));
        if next_axis {
            if event_due(c) {
                deliver_event_here(c, 0x0032_e6d0);
            }
            continue 'outer;
        }
        // All three axes passed: build the hit records.
        // Lazy initialisation of the six box normals.
        'init: {
            c.cycles += 3;
            c.g[V0][0] = Addr(gpa(c, 0x2888)).lw(c);
            let ready = c.g[V0][0] != 0;
            c.g[V1][0] = 0x500000;
            if ready {
                break 'init;
            }
            c.cycles += 31;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 0); // f0 = -1.0
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1); // f1 = 1.0
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xda0)); // &normals[0]
            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0xda0));
            c.g[A0][0] = 0x1;
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x48));
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x58));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x5c), 0);
            c.sw(gpa(c, 0x2888), c.g[A0][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x14), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x20), 0);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x24));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x28), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x2c), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x30), 0);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x34));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x38), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x3c), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x40), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x44), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x50), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x54), 0);
        }
        // Entry hit (tmin in [0, 1]).
        'enter: {
            c.cycles += 5;
            c.mtc1(0, 1); // f1 = 0.0
            c.c_le_s(1, 4); // 0 <= tmin
            let miss = !c.fcc();
            c.g[T0][0] = 0;
            if miss {
                break 'enter;
            }
            c.cycles += 41;
            c.g[V0][0] = 0x2;
            c.g[V1][0] = u64::MAX;
            c.sw((c.g[A3][0] as u32).wrapping_add(0x44), c.g[V0][0]);
            c.g[A0][0] = 0x500000;
            c.g[V0][0] = 0x500000;
            c.sw((c.g[A3][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
            c.sw((c.g[A3][0] as u32).wrapping_add(0x54), 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xda0));
            c.sw((c.g[A3][0] as u32).wrapping_add(0x50), 0);
            c.g[A1][0] = sx((c.g[T2][0] as u32) << 4);
            c.sw((c.g[A3][0] as u32).wrapping_add(0x58), 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xee0));
            c.sw((c.g[A3][0] as u32).wrapping_add(0x5c), c.g[V1][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[A3][0] as u32).wrapping_add(0x60), c.g[V1][0]);
            c.g[T0][0] = 0x1;
            c.sw((c.g[A3][0] as u32).wrapping_add(0x64), c.g[V1][0]);
            c.lq(2, c.g[A0][0] as u32);
            c.sq(2, (c.g[A3][0] as u32).wrapping_add(0x20));
            c.lq(3, c.g[A0][0] as u32);
            c.swc1(1, (c.g[A3][0] as u32).wrapping_add(0x68));
            c.sq(3, (c.g[A3][0] as u32).wrapping_add(0x30));
            c.swc1(1, (c.g[A3][0] as u32).wrapping_add(0x6c));
            c.swc1(1, (c.g[A3][0] as u32).wrapping_add(0x70));
            c.lq(2, c.g[A1][0] as u32);
            c.sq(2, (c.g[A3][0] as u32).wrapping_add(0x10));
            c.mfc1(3, 4); // v1 = tmin bits
            c.lqc2(4, (c.g[T1][0] as u32).wrapping_add(0x70));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp.wrapping_add(0x20));
            c.lq(2, sp.wrapping_add(0x20));
            c.sq(2, sp.wrapping_add(0x10));
            c.lqc2(3, (c.g[T1][0] as u32).wrapping_add(0x60));
            c.lqc2(4, sp.wrapping_add(0x10));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp.wrapping_add(0x20));
            c.lq(2, sp.wrapping_add(0x20));
            c.swc1(4, (c.g[A3][0] as u32).wrapping_add(0x40));
            c.sq(2, c.g[A3][0] as u32);
            c.sq(2, sp);
        }
        // Exit hit (tmax < 1).
        'leave: {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0); // f0 = 1.0
            c.c_lt_s(5, 0); // tmax < 1
            let exit_hit = c.fcc();
            c.g[V1][0] = sx((c.g[T0][0] as u32) << 7);
            if !exit_hit {
                break 'leave;
            }
            c.cycles += 42;
            c.g[V0][0] = 0x2;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            c.g[A0][0] = u64::MAX;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x44), c.g[V0][0]);
            c.g[A1][0] = 0x500000;
            c.g[V0][0] = 0x500000;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4c), c.g[A0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x54), 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xda0));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x50), 0);
            c.g[A2][0] = sx((c.g[T3][0] as u32) << 4);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x58), 0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xee0));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x5c), c.g[A0][0]);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x60), c.g[A0][0]);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[A0][0]);
            c.lq(2, c.g[A1][0] as u32);
            c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x20));
            c.lq(4, c.g[A1][0] as u32);
            c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x70));
            c.sq(4, (c.g[V1][0] as u32).wrapping_add(0x30));
            c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x68));
            c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x6c));
            c.lq(2, c.g[A2][0] as u32);
            c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.mfc1(4, 5); // a0 = tmax bits
            c.lqc2(4, (c.g[T1][0] as u32).wrapping_add(0x70));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp.wrapping_add(0x20));
            c.lq(2, sp.wrapping_add(0x20));
            c.sq(2, sp.wrapping_add(0x10));
            c.lqc2(3, (c.g[T1][0] as u32).wrapping_add(0x60));
            c.lqc2(4, sp.wrapping_add(0x10));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp.wrapping_add(0x20));
            c.lq(2, sp.wrapping_add(0x20));
            c.sq(2, sp);
            c.swc1(5, (c.g[V1][0] as u32).wrapping_add(0x40));
            c.sq(2, c.g[V1][0] as u32);
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[T0][0];
        break 'outer;
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp.wrapping_add(0x30));
    abi::ret(c);
}


/// `float CollHull_GetDiameter(CollHull* hull)` (0x0032f8c0): -1 unless the
/// hull has a shape, else twice its bounding sphere's radius.
pub fn coll_hull_get_diameter(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(abi::arg(c, 0)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
        c.mtc1(1, 0);
    } else {
        c.cycles += 4;
        c.g[V0][0] = c.lw(abi::arg(c, 0).wrapping_add(0x60)); // the sphere
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x20));
        c.add_s(0, 0, 0);
    }
    abi::ret(c);
}


/// `void CollSphere_ResetTransform(CollSphere* s)` (0x0032fd98): the
/// transform at +0x80/+0x90 becomes the current one at +0x60/+0x70.
pub fn coll_sphere_reset_transform(c: &mut Cpu) {
    c.cycles += 6;
    let s = abi::arg(c, 0);
    c.lq(2, p_4(s, 0x80));
    c.lwc1(0, p_4(s, 0x90));
    c.sq(2, p_4(s, 0x60));
    c.swc1(0, p_4(s, 0x70));
    c.sw(p_4(s, 0x4), 0);
    abi::ret(c);
}


/// `void RiderSphereTree_Init(RiderSphereTree* self, int type, int count,
/// int* ids, float* radii, int user, int radius)` (0x3299C8): fill `count`
/// (id, radius) entries at `+0x40`/`+0x44`, 0x20 bytes apart.
pub fn rider_sphere_tree_init(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.sw(ea(c.g[A0][0], 0x2c), c.g[A2][0]);
    c.g[V0][0] |= 0xffff;
    c.g[T2][0] = 0;
    let empty = (c.g[A2][0] as i64) <= 0;
    c.sw(ea(c.g[A0][0], 0x28), c.g[V0][0]);
    if !empty {
        c.cycles += 2;
        c.g[V1][0] = c.g[A0][0];
        loop {
            c.cycles += 11;
            c.g[V0][0] = c.lw(c.g[A3][0] as u32);
            c.g[T2][0] = sx(ea(c.g[T2][0], 1));
            c.g[A3][0] = sx(ea(c.g[A3][0], 4));
            c.sw(ea(c.g[V1][0], 0x44), c.g[V0][0]);
            c.lwc1(0, c.g[T0][0] as u32);
            c.g[T0][0] = sx(ea(c.g[T0][0], 4));
            c.swc1(0, ea(c.g[V1][0], 0x40));
            c.g[V0][0] = c.lw(ea(c.g[A0][0], 0x2c));
            c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(ea(c.g[V1][0], 0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_99e8);
            }
        }
    }
    c.cycles += 4;
    c.sw(ea(c.g[A0][0], 0x24), c.g[T1][0]);
    c.sw(ea(c.g[A0][0], 0), c.g[A1][0]);
    c.swc1(12, ea(c.g[A0][0], 0x20));
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Collision.

/// `int CollHull_CollideSphereTree(CollHull* self, CollSphere* sphere,
/// Hit* hit)` (0x32fa30): collide the sphere with the hull's sphere tree and,
/// on a hit, store the hit direction length and normalise the direction.
pub fn coll_hull_collide_sphere_tree(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A0][0];
        frame.save(c, S0, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A2][0] = sx(p(r32(c, S0), 0x10));
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = Addr(p(r32(c, V0), 0x60)).lw(c);
        call_ptr(c, 0x0032_7f18, 0x0032_fa58)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 19;
            c.lqc2(3, p(r32(c, S0), 0x10));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 0);
            c.swc1(0, p(r32(c, S0), 0x40));
            c.g[V0][0] = 1;
            c.mfc1(3, 0);
            c.qmtc2(3, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, p(r32(c, S0), 0x10));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 4, Src::Q);
            c.sqc2(5, p(r32(c, S0), 0x10));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int CollSphere_PlaneDistance(CollSphere* self, Vec4* plane, Vec4* p,
/// float* out)` (0x330360): the signed distance of the sphere to the plane,
/// written to `out`, and whether its magnitude is inside the radius.
pub fn coll_sphere_plane_distance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = c.g[S0][0];
        callee::VEC3_NEG_DOT.call(c, 0x0033_038c)?;
        c.lqc2(3, r32(c, S0));
        c.lqc2(5, p(r32(c, S1), 0x60));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 1);
        c.add_s(1, 1, 0);
        c.g[V0][0] = 1;
        c.abs_s(2, 1);
        c.swc1(1, r32(c, S2));
        c.lwc1(0, p(r32(c, S1), 0x70));
        c.c_lt_s(2, 0);
        if !c.fcc() {
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
// Sphere tree collision.

/// `int SphereTree_CollideSphere(self, const Sphere* s, float f, void* a2,
/// void* a3)` (0x32c898): refresh the tree's node offset, then test a sphere
/// against the node at_2 `self + 0x80`.
pub fn sphere_tree_collide_sphere(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        let self_ = abi::arg(c, 0);
        let a1 = abi::arg64(c, 1);
        let a2 = abi::arg64(c, 2);
        let a3 = abi::arg64(c, 3);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = sx(self_);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = a2;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = a3;
        frame.save_fpr(c, 20, 0x50);
        c.g[S3][0] = a1;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(at_2(c, S0, 0x98));
        c.g[V0][0] = c.lw(at_2(c, A0, 8));
        let no_offset = c.g[V0][0] == 0;
        c.mov_s(20, 12);
        if !no_offset {
            c.cycles += 4;
            call_ptr(c, callee::SPHERE_TREE_COMPUTE_OFFSET, 0x0032_c8dc)?;
            c.g[V1][0] = c.lw(at_2(c, S0, 0x98));
            c.sw(at_2(c, V1, 0x28), c.g[V0][0]);
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S3][0];
        c.mov_s(12, 20);
        c.g[T1][0] = c.g[S1][0];
        c.g[T2][0] = c.g[S2][0];
        c.g[A2][0] = sx(p(r32(c, A0), 0x80));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, callee::SPHERE_TREE_COLLIDE_SPHERE_NODE, 0x0032_c908)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}


/// `int SphereTree_CollideAABB(self, const Box* b, void* a2, void* a3,
/// void* a4)` (0x32c928): refresh the tree's node offset, then test an AABB
/// against the node at_2 `self + 0x80`.
pub fn sphere_tree_collide_aabb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        let self_ = abi::arg(c, 0);
        let a1 = abi::arg64(c, 1);
        let a2 = abi::arg64(c, 2);
        let a3 = abi::arg64(c, 3);
        let a4 = abi::arg64(c, 4);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx(self_);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = a2;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = a3;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = a4;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(at_2(c, S0, 0x98));
        c.g[V0][0] = c.lw(at_2(c, A0, 8));
        let no_offset = c.g[V0][0] == 0;
        c.g[S4][0] = a1;
        if !no_offset {
            c.cycles += 4;
            call_ptr(c, callee::SPHERE_TREE_COMPUTE_OFFSET, 0x0032_c96c)?;
            c.g[V1][0] = c.lw(at_2(c, S0, 0x98));
            c.sw(at_2(c, V1, 0x28), c.g[V0][0]);
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[T2][0] = c.g[S2][0];
        c.g[T3][0] = c.g[S3][0];
        c.g[A3][0] = sx(p(r32(c, A0), 0x80));
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        call_ptr(c, callee::SPHERE_TREE_COLLIDE_AABB_NODE, 0x0032_c998)?;
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


/// `int CollSphere_CollideSphereTree(self, SphereTree* tree, Hit* hit)`
/// (0x32fdb0): test the collider's sphere against the tree and, on a hit,
/// normalise the hit normal and return its length.
pub fn coll_sphere_collide_sphere_tree(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        let tree = abi::arg64(c, 1);
        let hit = abi::arg64(c, 2);
        c.g[V0][0] = c.g[A0][0];
        frame.save(c, S0, 0x10);
        c.g[A0][0] = tree;
        frame.save_ra(c, 0);
        c.g[S0][0] = hit;
        c.g[A1][0] = sx(p(self_, 0x60));
        c.g[A2][0] = sx(p(r32(c, S0), 0x10));
        c.lwc1(12, at_2(c, V0, 0x70));
        c.g[A3][0] = c.g[S0][0];
        call_ptr(c, callee::SPHERE_TREE_COLLIDE_SPHERE, 0x0032_fddc)?;
        let miss = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if !miss {
            c.cycles += 19;
            c.lqc2(3, at_2(c, S0, 0x10));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 0);
            c.swc1(0, at_2(c, S0, 0x40));
            c.g[V0][0] = 1;
            c.mfc1(3, 0);
            c.qmtc2(3, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, at_2(c, S0, 0x10));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 4, Src::Q);
            c.sqc2(5, at_2(c, S0, 0x10));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


pub fn ov_screen_save_records_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        ov_screen_set_widget_visibility(c, c.g[A2][0])
    })
}


// ---------------------------------------------------------------------------
// Collision queries (VU0: the original's COP2 ops are issued in order).

/// The length/normalise sequence shared by the sphere/hull triangle tests:
/// `v` at_3 `sp` is the collision point, `out_len` (s0) receives |v| and
/// `out_dir` (s1) the normalised v.
pub(crate) fn collision_direction(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 22;
    c.lqc2(3, c.g[SP][0] as u32);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(2, 22);
    c.mtc1(2, 0);
    c.swc1(0, c.g[S0][0] as u32);
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, c.g[SP][0] as u32);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(2, c.g[S1][0] as u32);
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    frame_restore_s0(c);
    Ok(())
}


#[inline(always)]
pub(crate) fn frame_restore_s0(c: &mut Cpu) {
    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
}


pub(crate) fn coll_sphere_tree_common(c: &mut Cpu, collide: u32, ret: u32) -> crate::call::Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    c.g[S0][0] = c.g[T3][0];
    frame.save_ra(c, 0x30);
    c.g[S1][0] = c.g[T2][0];
    c.g[T2][0] = c.g[T1][0];
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x60).lw(c);
    c.g[T1][0] = c.g[SP][0];
    call_ptr(c, collide, ret)?;
    c.g[A0][0] = c.g[V0][0];
    if c.g[A0][0] != 0 {
        collision_direction(c)?;
    } else {
        frame_restore_s0(c);
    }
    c.cycles += 5;
    c.g[V0][0] = c.g[A0][0];
    frame.restore(c, S1, 0x40);
    c.g[RA][0] = Addr(c.g[SP][0] as u32).off(0x30).ld(c);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = c.g[RA][0] as u32;
    Ok(())
}


/// `int CollHull_CollideTriangle2Sided(...)` (0x32f990): the hull-variant
/// collision with the contact direction.
pub fn coll_hull_collide_triangle2_sided(c: &mut Cpu) {
    guest(c, |c| coll_sphere_tree_common(c, table::COLL_HULL_TRIANGLE2, 0x0032_f9b8))
}


/// `void SphereTree_CollideSphereTree(self, other, ..., a2, a3)` (0x32cb58):
/// refresh both trees' cached bounds and descend.
pub fn sphere_tree_collide_sphere_tree(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A3][0];
        c.g[A0][0] = lw_2(c, S0, 0x98);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        let skip = c.g[V0][0] == 0;
        if !skip {
            c.cycles += 4;
            call_ptr(c, table::COLL_SPHERE_CALC, 0x0032_cb94)?;
            c.g[V1][0] = lw_2(c, S0, 0x98);
            Addr(c.g[V1][0] as u32).off(0x28).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[A0][0] = lw_2(c, S1, 0x98);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        if c.g[V0][0] == 0 {
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.cycles += 5;
            call_ptr(c, table::COLL_SPHERE_CALC, 0x0032_cbb4)?;
            c.g[V1][0] = lw_2(c, S1, 0x98);
            Addr(c.g[V1][0] as u32).off(0x28).sw(c, c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 14;
        c.g[A1][0] = c.g[S1][0];
        c.g[T1][0] = c.g[S2][0];
        c.g[T2][0] = c.g[S3][0];
        c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x80));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        callee::SPHERE_TREE_COLLIDE_TREE_NODE.call(c, 0x0032_cbdc)?;
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
// Collision objects.

/// `CollSphere* CollSphere_ctor(CollSphere* self, const Vec3* pos, int flags,
/// float radius)` (0x32fb48): set the vtable and `state`, copy the centre and
/// radius, and build the `pos ± r` AABBs (`w = 1`).
pub fn coll_sphere_ctor(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x490000;
    c.g[A3][0] = c.g[A0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x19b0));
    open(c, 0x10);
    Addr(c.g[A3][0] as u32).off(0x50).sw(c, c.g[V0][0]);
    Addr(c.g[A3][0] as u32).off(8).sw(c, 0);
    let no_flag = c.g[A2][0] == 0;
    Addr(c.g[A3][0] as u32).off(4).sw(c, 0);
    if no_flag {
        c.cycles += 2;
        c.g[V0][0] = 1;
        Addr(c.g[A3][0] as u32).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 2;
        Addr(c.g[A3][0] as u32).sw(c, 0);
    }
    c.cycles += 35;
    c.lq(3, c.g[A1][0] as u32);
    c.g[V0][0] = c.g[A3][0];
    c.swc1(12, at_4(c.g[A3][0] as u32, 0x70));
    c.sq(3, at_4(c.g[A3][0] as u32, 0x60));
    c.sq(3, at_4(c.g[A3][0] as u32, 0x80));
    c.swc1(12, at_4(c.g[A3][0] as u32, 0x90));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    c.lwc1(1, at_4(c.g[A1][0] as u32, 4));
    c.lwc1(2, at_4(c.g[A1][0] as u32, 8));
    c.lwc1(0, c.g[A1][0] as u32);
    c.add_s(1, 1, 12);
    c.add_s(2, 2, 12);
    c.swc1(3, at_4(c.g[SP][0] as u32, 0xc));
    c.add_s(0, 0, 12);
    c.swc1(1, at_4(c.g[SP][0] as u32, 4));
    c.swc1(2, at_4(c.g[SP][0] as u32, 8));
    c.swc1(0, c.g[SP][0] as u32);
    c.lq(3, c.g[SP][0] as u32);
    c.sq(3, at_4(c.g[A3][0] as u32, 0x20));
    c.lwc1(1, at_4(c.g[A1][0] as u32, 4));
    c.lwc1(2, at_4(c.g[A1][0] as u32, 8));
    c.lwc1(0, c.g[A1][0] as u32);
    c.sub_s(1, 1, 12);
    c.sub_s(2, 2, 12);
    c.sub_s(0, 0, 12);
    c.swc1(1, at_4(c.g[SP][0] as u32, 4));
    c.swc1(2, at_4(c.g[SP][0] as u32, 8));
    c.swc1(0, c.g[SP][0] as u32);
    c.lq(3, c.g[SP][0] as u32);
    c.sq(3, at_4(c.g[A3][0] as u32, 0x30));
    c.lq(4, c.g[A1][0] as u32);
    c.sq(4, at_4(c.g[A3][0] as u32, 0x40));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}


// ---------------------------------------------------------------------------
// Sphere tree.

/// `int SphereTree_CollideTriangle(self, a1, a2, a3, t0, t1, t2)` (0x32ca78):
/// refresh the tree's cached bound, collide the triangle and report whether
/// the closest hit is in front.
pub fn spheretree_collide_triangle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[T0][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[T1][0];
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[T2][0];
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S6, 0x20);
        c.g[S5][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S6][0] = c.g[A3][0];
        c.g[A0][0] = Addr(p(r32(c, S0), 0x98)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, A0), 8)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, 0x0032_df28, 0x0032_cacc)?;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x98)).lw(c);
            Addr(p(r32(c, V1), 0x28)).sw(c, c.g[V0][0]);
        }
        c.cycles += 21;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0032_db40, 0x0032_cae0)?;
        Addr(p(r32(c, S0), 0x94)).sw(c, 0);
        c.g[A1][0] = c.g[S4][0];
        c.sw(r32(c, SP), c.g[S2][0]);
        c.g[A2][0] = c.g[S5][0];
        c.sw(p(r32(c, SP), 8), c.g[S3][0]);
        c.g[A3][0] = c.g[S6][0];
        c.g[T0][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[T1][0] = sx(p(r32(c, S0), 0x80));
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        call_ptr(c, 0x0032_d470, 0x0032_cb10)?;
        c.lwc1(1, p(r32(c, S0), 0x94));
        c.mtc1(0, 0);
        c.c_lt_s(1, 0);
        let hit = c.fcc();
        c.g[V0][0] = 1;
        if !hit {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void func_0032DA40(int n, int* keys, int* vals)` (0x32da40): shell sort
/// (descending on `keys`, `vals` carried along) using the gap table at
/// `D_0044AF98`, starting with the first gap below 2.
pub fn shell_sort_desc_key_val(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x45_0000;
    c.g[T5][0] = c.g[A0][0]; // n
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5068)); // D_0044AF98
    c.g[T2][0] = c.g[A1][0]; // keys
    c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 4));
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T5][0] as i64)) as u64;
    let done = c.g[V0][0] == 0;
    c.g[V1][0] = 0x1;
    if !done {
        // k = 1; while (D[k] < n) k++
        c.cycles += 2;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        loop {
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T5][0] as i64)) as u64;
            let small = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if !small {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_da68);
            }
        }
    }
    c.cycles += 5;
    c.g[V0][0] = 0x45_0000;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5068));
    c.g[T6][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_sub(4));
    loop {
        // gap = D[--k]; the insertion pass only runs for gap < count
        c.cycles += 4;
        c.g[T0][0] = c.lw(c.g[T6][0] as u32);
        c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[T5][0] as i64)) as u64;
        let no_pass = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[T0][0];
        if !no_pass {
            loop {
                c.cycles += 8;
                c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[T3][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[T0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                c.g[T4][0] = c.lw(c.g[V1][0] as u32); // vals[i]
                c.g[T1][0] = c.lw(c.g[V0][0] as u32); // key = keys[i]
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = sx((c.g[A3][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    let ordered = c.g[V1][0] == 0;
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    if ordered {
                        break;
                    }
                    c.cycles += 9;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[T0][0] as u32));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]); // keys[j+gap] = keys[j]
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    let more = (c.g[A3][0] as i64) >= 0;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]); // vals[j+gap] = vals[j]
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_dac8);
                    }
                }
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[A1][0] = c.g[T3][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[T5][0] as i64)) as u64;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.sw(c.g[V1][0] as u32, c.g[T1][0]); // keys[j+gap] = key
                let next_i = c.g[A0][0] != 0;
                c.sw(c.g[V0][0] as u32, c.g[T4][0]); // vals[j+gap] = val
                if !next_i {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_daa8);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[T0][0] as i64) < 2) as u64;
        let next_gap = c.g[V0][0] == 0;
        c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_sub(4));
        if !next_gap {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_da98);
        }
    }
}


// ---------------------------------------------------------------------------
// Collision.

/// `bool CollSegment_OverlapsAABB(Segment* seg, Vec3* boxMin, Vec3* boxMax)`
/// (0x32e288): the segment's centre (`+0x60`) plus its extent (`+0x70`) is its
/// AABB; the test rejects the pair on each axis as the original does.
pub fn coll_segment_overlaps_aabb(c: &mut Cpu) {
    c.cycles += 12;
    let seg = r32v(c, A0);
    let lo = r32v(c, A1);
    let hi = r32v(c, A2);
    c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0x20));
    c.lqc2(3, at_5(seg, 0x60));
    c.lqc2(4, at_5(seg, 0x70));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, spa(c, 0x10));
    // First: the segment min against the box min.
    c.lwc1(2, at_5(seg, 0x60));
    c.lwc1(1, lo);
    c.lq(2, spa(c, 0x10));
    c.c_lt_s(2, 1);
    let mut separated = false;
    let skips = !c.fcc();
    c.sq(2, spa(c, 0));
    if !skips {
        c.cycles += 5;
        c.lwc1(0, spa(c, 0));
        c.c_lt_s(0, 1);
        separated = c.fcc();
    }
    // Then the box max against the segment min/max.
    if !separated {
        c.cycles += 5;
        c.lwc1(1, hi);
        c.c_lt_s(1, 2);
        let skips = !c.fcc();
        c.lwc1(0, spa(c, 0));
        if !skips {
            c.cycles += 4;
            c.c_lt_s(1, 0);
            separated = c.fcc();
        }
    }
    // y.
    if !separated {
        c.cycles += 6;
        c.lwc1(2, at_5(seg, 0x64));
        c.lwc1(1, at_5(lo, 4));
        c.c_lt_s(2, 1);
        let skips = !c.fcc();
        c.lwc1(0, spa(c, 4));
        if !skips {
            c.cycles += 4;
            c.c_lt_s(0, 1);
            separated = c.fcc();
        }
    }
    if !separated {
        c.cycles += 5;
        c.lwc1(1, at_5(hi, 4));
        c.c_lt_s(1, 2);
        let skips = !c.fcc();
        c.lwc1(0, spa(c, 4));
        if !skips {
            c.cycles += 4;
            c.c_lt_s(1, 0);
            separated = c.fcc();
        }
    }
    // z.
    if !separated {
        c.cycles += 6;
        c.lwc1(2, at_5(seg, 0x68));
        c.lwc1(1, at_5(lo, 8));
        c.c_lt_s(2, 1);
        let skips = !c.fcc();
        c.lwc1(0, spa(c, 8));
        if !skips {
            c.cycles += 4;
            c.c_lt_s(0, 1);
            separated = c.fcc();
        }
    }
    if !separated {
        c.cycles += 5;
        c.lwc1(1, at_5(hi, 8));
        c.c_lt_s(1, 2);
        let skips = !c.fcc();
        c.lwc1(0, spa(c, 8));
        if !skips {
            c.cycles += 4;
            c.c_lt_s(1, 0);
            separated = c.fcc();
        }
    }
    if separated {
        c.g[V0][0] = 0;
    } else {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0x20));
    abi::ret(c);
}


/// `bool CollSphere_HitsTriangleEdges(Sphere*, Vec3 a, Vec3 b, Vec3 c)`
/// (0x330250): for each of the three triangle edges, clamp the sphere centre
/// onto the edge and compare the distance with the sphere radius.
pub fn coll_sphere_hits_triangle_edges(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let sphere = r32v(c, A0);
        c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0xc0));
        c.sq(17, spa(c, 0xa0));
        c.sq(18, spa(c, 0x90));
        c.g[S1][0] = c.g[A0][0];
        c.sq(19, spa(c, 0x80));
        c.sq(20, spa(c, 0x70));
        c.sq(16, spa(c, 0xb0));
        c.sd(spa(c, 0x60), c.g[RA][0]);
        // The three vertices are passed by value; spill them to the frame.
        c.lq(2, r32v(c, A1));
        c.g[A1][0] = 0;
        c.sq(2, spa(c, 0x10));
        c.lq(2, r32v(c, A2));
        c.sq(2, spa(c, 0x20));
        c.lq(3, r32v(c, A3));
        c.sq(3, spa(c, 0x30));
        c.g[S2][0] = sx(spa(c, 0x10));
        c.g[S3][0] = 3;
        c.g[S4][0] = sx(spa(c, 0x40));
        // s0 walks the edges (0, 1, 2), a1 the start vertex, s3 the vertex
        // count for the div below.
        c.g[S0][0] = 1;
        loop {
            c.cycles += 2;
            c.cycles += 32;
            c.div(0, 16, 19);
            c.g[A1][0] = sx(r32v(c, A1) << 4);
            c.g[A1][0] = sx(r32v(c, S2).wrapping_add(r32v(c, A1)));
            c.g[A0][0] = c.g[S4][0];
            c.g[A3][0] = sx(at_5(sphere, 0x60));
            c.g[A2][0] = c.hi[0];
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 4);
            c.g[A2][0] = sx(r32v(c, S2).wrapping_add(r32v(c, A2)));
            callee::CLOSEST_POINT_ON_SEGMENT.call(c, 0x0033_02cc)?;
            // d = closest - centre; |d|^2 against r^2 (VU0 dot product).
            c.lq(2, spa(c, 0x40));
            c.sq(2, spa(c, 0));
            c.lqc2(3, at_5(sphere, 0x60));
            c.lqc2(4, spa(c, 0));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, spa(c, 0x50));
            c.lq(2, spa(c, 0x50));
            c.lwc1(1, at_5(sphere, 0x70));
            c.sq(2, spa(c, 0x40));
            c.lqc2(3, spa(c, 0x40));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 0);
            c.c_lt_s(0, 1);
            let hit = c.fcc();
            c.g[V0][0] = 1;
            if hit {
                break;
            }
            c.cycles += 4;
            let i = c.g[S0][0];
            c.g[A1][0] = i;
            c.g[V0][0] = ((i as i64) < 3i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((i as u32).wrapping_add(1));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0033_02a0);
                }
                continue;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
            break;
        }
        c.cycles += 8;
        c.lq(16, spa(c, 0xb0));
        c.lq(17, spa(c, 0xa0));
        c.lq(18, spa(c, 0x90));
        c.lq(19, spa(c, 0x80));
        c.lq(20, spa(c, 0x70));
        c.g[RA][0] = c.ld(spa(c, 0x60));
        c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0xc0));
        abi::ret(c);
        Ok(())
    })
}


/// `bool CollSphere_OverlapsAABB(const CollSphere* s, const Vec4* mn,
/// const Vec4* mx)` (0x32fc08): the sphere's capsule (`+0x60` centre,
/// `+0x70` radius) grown by the radius on every axis is tested against the
/// box; true when they overlap.
pub fn coll_sphere_overlaps_aabb(c: &mut Cpu) {
    'b_32fd38: {
        'b_32fc78: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.stw_2(SP, 0xc, 0u64);
            c.lwc1(0, c.pa(A0, 0x70));
            c.swc1(0, c.pa(SP, 0x8));
            c.swc1(0, c.pa(SP, 0));
            c.swc1(0, c.pa(SP, 0x4));
            c.lqc2(3, c.pa(A0, 0x60));
            c.lqc2(4, c.pa(SP, 0));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, c.pa(SP, 0x20));
            c.lq(2, c.pa(SP, 0x20));
            c.sq(2, c.pa(SP, 0x10));
            c.lqc2(3, c.pa(A0, 0x60));
            c.lqc2(4, c.pa(SP, 0));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, c.pa(SP, 0x30));
            c.lwc1(1, c.pa(A1, 0));
            c.lwc1(2, c.pa(SP, 0x10));
            c.lq(2, c.pa(SP, 0x30));
            c.c_lt_s(2, 1);
            let t = !c.fcc();
            c.sq(2, c.pa(SP, 0x20));
            if t {
                break 'b_32fc78;
            } else {
                c.cycles += 5;
                c.lwc1(0, c.pa(SP, 0x20));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.g[V0][0] = 0u64;
                    break 'b_32fd38;
                } else {
                    break 'b_32fc78;
                }
            }
        }
        'b_32fc9c: {
            c.cycles += 5;
            c.lwc1(1, c.pa(A2, 0));
            c.c_lt_s(1, 2);
            let t = !c.fcc();
            c.lwc1(0, c.pa(SP, 0x20));
            if t {
                break 'b_32fc9c;
            } else {
                c.cycles += 4;
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.g[V0][0] = 0u64;
                    break 'b_32fd38;
                } else {
                    break 'b_32fc9c;
                }
            }
        }
        'b_32fcc4: {
            c.cycles += 6;
            c.lwc1(1, c.pa(A1, 0x4));
            c.lwc1(2, c.pa(SP, 0x14));
            c.c_lt_s(2, 1);
            let t = !c.fcc();
            c.lwc1(0, c.pa(SP, 0x24));
            if t {
                break 'b_32fcc4;
            } else {
                c.cycles += 4;
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.g[V0][0] = 0u64;
                    break 'b_32fd38;
                } else {
                    break 'b_32fcc4;
                }
            }
        }
        'b_32fce8: {
            c.cycles += 5;
            c.lwc1(1, c.pa(A2, 0x4));
            c.c_lt_s(1, 2);
            let t = !c.fcc();
            c.lwc1(0, c.pa(SP, 0x24));
            if t {
                break 'b_32fce8;
            } else {
                c.cycles += 4;
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.g[V0][0] = 0u64;
                    break 'b_32fd38;
                } else {
                    break 'b_32fce8;
                }
            }
        }
        'b_32fd10: {
            c.cycles += 6;
            c.lwc1(1, c.pa(A1, 0x8));
            c.lwc1(2, c.pa(SP, 0x18));
            c.c_lt_s(2, 1);
            let t = !c.fcc();
            c.lwc1(0, c.pa(SP, 0x28));
            if t {
                break 'b_32fd10;
            } else {
                c.cycles += 4;
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.g[V0][0] = 0u64;
                    break 'b_32fd38;
                } else {
                    break 'b_32fd10;
                }
            }
        }
        'b_32fd34: {
            c.cycles += 5;
            c.lwc1(1, c.pa(A2, 0x8));
            c.c_lt_s(1, 2);
            let t = !c.fcc();
            c.lwc1(0, c.pa(SP, 0x28));
            if t {
                break 'b_32fd34;
            } else {
                c.cycles += 4;
                c.c_lt_s(1, 0);
                let t = c.fcc();
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_32fd38;
                } else {
                    break 'b_32fd34;
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 0x1;
        break 'b_32fd38;
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
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
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::SP as STACK;
    use crate::difftest::fakes;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::ptr;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use ssx3_ee as ee;
    use crate::difftest::RA as RET;
    use crate::difftest::SP;
    use crate::difftest::compare;
    use crate::difftest::seed_count;
    pub(crate) use crate::game::ai::rider::tests::SEEDS;
    pub(crate) use crate::game::ai::rider::tests::f;
    pub(crate) use crate::game::ai::rider::tests::uf_2 as uf;
    pub(crate) use crate::game::ai::rider::tests::put_v;
    pub(crate) use crate::game::ai::rider::tests::rand_v;
    pub(crate) use crate::game::ai::rider::tests::gp_f32;
    pub(crate) use crate::game::ai::rider::tests::VT_SUB;
    pub(crate) use crate::game::ai::rider::tests::VT_OWNER;
    pub(crate) use crate::game::ai::rider::tests::VT_VOL;
    pub(crate) use crate::game::ai::rider::tests::VT_HULL;
    pub(crate) use crate::game::ai::rider::tests::OWNER;
    pub(crate) use crate::game::ai::rider::tests::SURFS;
    pub(crate) use crate::game::ai::rider::tests::MATS;
    pub(crate) use crate::game::ai::rider::tests::INSTS;
    pub(crate) use crate::game::ai::rider::tests::RIDER_2 as RIDER;
    pub(crate) use crate::game::ai::rider::tests::fake_varied;
    pub(crate) use crate::game::ai::rider::tests::put_slot;
    pub(crate) use crate::game::ai::rider::tests::setup_machine;
    pub(crate) use crate::game::ai::rider::tests::put_sphere_tree;
    pub(crate) use crate::game::intersect::worldsphtree::tests::SEEDS_2;
    pub(crate) use crate::game::ai::computer::tests::literals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festateoptions::tests::rng;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::bx::bxstring2::tests::fbits;
    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::be::beintplayer::tests::fillb;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS_4;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_2;
    pub(crate) use crate::game::fe::ovstates::tests::base_3;
    pub(crate) use crate::game::sound::soundsys::tests::wf;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool_3;
    pub(crate) use crate::game::sound::soundsys::tests::load_2 as load;
    pub(crate) use crate::game::sound::soundsys::tests::alloc_ready;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_4;
    pub(crate) use crate::game::be::beintplayer::tests::SEEDS_7 as SEEDS_6;
    pub(crate) use crate::game::be::beintplayer::tests::strict;
    pub(crate) use crate::game::be::beintplayer::tests::globals_5 as globals_3;
    pub(crate) use crate::game::be::beintplayer::tests::base_10 as base_5;
    pub(crate) use crate::game::object::flexrailnode::tests::base_3 as base_6;
    pub(crate) use crate::game::object::flexrailnode::tests::check_all;
    pub(crate) use crate::game::object::flexrailnode::tests::floats;
    pub(crate) use crate::game::render::graphicsman::tests::same;


    #[test]
    fn sphere_tree_get_half_leaf_radius_matches() {
        check_lohi(ssx3_ee::s10::f_0032c5a8, sphere_tree_get_half_leaf_radius, 200, &|c, seed| {
            let mut r = Rng(0x100f ^ seed as u64);
            scramble(c, &mut r);
            let leaf = DATA + 0x200;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x98, leaf);
            let idx = seed % 8;
            c.bus.write32(leaf + 0xc, idx);
            c.bus.write32(leaf + 0x20, DATA);
            c.bus.write32(DATA + 0x90, r.edge_f32());
            c.bus.write32(DATA + idx * 0xc, r.edge_f32());
        });
    }


    pub(crate) fn sphere_setup(c: &mut Cpu, r: &mut Rng, na: u32, nb: u32, spread: f32) {
        let (a, b) = (DATA + 0x1000, DATA + 0x2000);
        let ca = rand_v(r, -2.0, 2.0);
        let off = rand_v(r, -spread, spread);
        let cb = [ca[0] + off[0], ca[1] + off[1], ca[2] + off[2], 0.0];
        let (ra, rb) = (uf(r, 1.0, 3.0), uf(r, 1.0, 3.0));
        put_sphere_tree(c, r, a, ca, ra, na);
        put_sphere_tree(c, r, b, cb, rb, nb);
        c.g[A0][0] = sx(a);
        c.g[A1][0] = sx(b);
        c.g[A2][0] = sx(DATA + 0x3000);
        c.g[A3][0] = sx(DATA + 0x3010);
        c.pc = ST_ENTRY;
    }


    driven!(gen_sphere_tree, ee::s10::f_00329f98__generated);

    driven!(hand_sphere_tree, rider_sphere_tree_intersect);


    #[test]
    fn rider_sphere_tree_intersect_matches() {
        check(ee::s10::f_00329f98__generated, rider_sphere_tree_intersect, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x9f98, 0);
            let spread = if seed % 2 == 0 { 1.0 } else { 5.0 };
            sphere_setup(c, &mut r, seed % 5, (seed / 5) % 5, spread);
        });
        // Events at the loop heads and the hit block, with re-entry.
        check_events(gen_sphere_tree, hand_sphere_tree, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x9f99, 0);
            sphere_setup(c, &mut r, 2 + seed % 4, 1 + (seed / 4) % 4, 0.5);
            schedule(c, &mut r, seed, ST_ENTRY)
        });
    }


    // -----------------------------------------------------------------
    // Forward-difference cache.

    pub(crate) const SENTINEL: f32 = -99.0;

    pub(crate) const DEGENERATE: f32 = -98.0;


    /// An entry at `e`: 10x10 vertices from +0x10 (0x10 apart, rows 0xa0),
    /// coarse cells at +0x650, fine cells at +0x1a90; cells either
    /// uncomputed (sentinel) or with precomputed normals and boxes.
    pub(crate) fn put_entry(c: &mut Cpu, r: &mut Rng, e: u32) {
        for i in 0..100u32 {
            let (row, col) = ((i / 10) as f32, (i % 10) as f32);
            let y = if r.next_u32() % 8 == 0 { 0.0 } else { uf(r, -1.0, 1.0) };
            put_v(c, e + 0x10 + 0x10 * i, [col - 4.5, y, row - 4.5, 1.0]);
        }
        for k in 0..(81 + 9) {
            let cl = e + 0x650 + 0x40 * k;
            let fresh = r.next_u32() % 3 != 0;
            if fresh {
                c.bus.write32(cl, f(SENTINEL));
                c.bus.write32(cl + 0x10, f(SENTINEL));
                c.bus.write32(cl + 0x20, f(SENTINEL));
            } else {
                let n0 = if r.next_u32() % 4 == 0 {
                    DEGENERATE
                } else {
                    uf(r, -1.0, 1.0)
                };
                let n1 = if r.next_u32() % 4 == 0 {
                    DEGENERATE
                } else {
                    uf(r, -1.0, 1.0)
                };
                put_v(c, cl, [n0, uf(r, 0.0, 1.0), uf(r, -1.0, 1.0), 0.0]);
                put_v(c, cl + 0x10, [n1, uf(r, 0.0, 1.0), uf(r, -1.0, 1.0), 0.0]);
                let lo = rand_v(r, -5.0, 5.0);
                put_v(c, cl + 0x20, lo);
                put_v(
                    c,
                    cl + 0x30,
                    [
                        lo[0] + uf(r, 0.0, 2.0),
                        lo[1] + uf(r, 0.0, 2.0),
                        lo[2] + uf(r, 0.0, 2.0),
                        0.0,
                    ],
                );
            }
        }
    }


    #[test]
    fn ai_fwd_diff_cache_collide_entry_matches() {
        covered(
            ee::s10::f_0032b6e0__generated,
            ai_fwd_diff_cache_collide_entry,
            SEEDS,
            &|c, seed| {
                let mut r = setup_machine(c, seed, 0xb6e0, 6);
                let (vol, entry, hint, uv) = (DATA + 0x100, DATA + 0x4000, DATA + 0x200, DATA + 0x300);
                put_entry(c, &mut r, entry);
                for (off, x) in [(-0x2dc0, 0.1f32), (-0x2dbc, 0.05), (-0x2db8, 0.3), (-0x2db4, 0.15)] {
                    gp_f32(c, off, x);
                }
                for off in [-0x2db0, -0x2d94] {
                    gp_f32(c, off, SENTINEL);
                }
                for off in [-0x2dac, -0x2da8, -0x2da0, -0x2d9c, -0x2d90] {
                    gp_f32(c, off, DEGENERATE);
                }
                for off in [-0x2da4, -0x2d98, -0x2d8c] {
                    gp_f32(c, off, uf(&mut r, -0.5, 0.5));
                }
                let kind = seed % 3;
                c.bus.write32(vol, kind);
                c.bus.write32(vol + 8, (r.next_u32() % 2 == 0) as u32);
                put_v(c, vol + 0x10, [uf(&mut r, -0.5, 0.5), 1.0, uf(&mut r, -0.5, 0.5), 0.0]);
                let lo = rand_v(&mut r, -5.0, 3.0);
                let ext = uf(&mut r, 0.5, 4.0);
                put_v(c, vol + 0x20, [lo[0] + ext, lo[1] + ext, lo[2] + ext, 0.0]);
                put_v(c, vol + 0x30, lo);
                c.bus.write32(vol + 0x50, VT_VOL);
                let key = 7;
                let side = if kind == 0 { 3 } else { 9 };
                c.bus.write32(hint, if seed % 4 == 0 { key + 1 } else { key });
                c.bus.write16(hint + 4, (r.next_u32() % side) as u16);
                c.bus.write16(hint + 6, (r.next_u32() % side) as u16);
                c.bus.write32(hint + 8, r.next_u32() % 2);
                c.bus
                    .write32(hint + 0xc, if seed % 5 == 0 { (kind + 1) % 3 } else { kind });
                c.g[A0][0] = sx(vol);
                c.g[A1][0] = sx(key);
                c.g[A2][0] = sx(entry);
                c.g[T2][0] = sx(uv);
                c.g[T3][0] = if seed % 7 == 0 { 0 } else { sx(hint) };
            },
        );
    }


    #[test]
    fn coll_segment_collide_triangle_matches() {
        const TREE: u32 = DATA + 0x2000;
        const N: u32 = DATA + 0x100;
        const P: u32 = DATA + 0x110;
        const B: u32 = DATA + 0x120;
        const CC: u32 = DATA + 0x130;
        const HIT: u32 = DATA + 0x140;
        const OUT_N: u32 = DATA + 0x150;
        const OUT_T: u32 = DATA + 0x160;
        check(
            ssx3_ee::s11::f_0032e4d0__generated,
            coll_segment_collide_triangle,
            SEEDS_2,
            &|c, seed| {
                let _r = machine(c, seed, 0x4003, 0);
                // The unit's epsilon and tolerance constants.
                c.bus.write32(GAME_GP.wrapping_sub(0x2d7c), 0x2edb_e6ff);
                c.bus.write32(GAME_GP.wrapping_sub(0x2d78), 0x38d1_b717);
                let (dx, nx) = match seed % 20 {
                    0 => (0.0, 0.0),      // the segment is parallel to the plane
                    1..=6 => (-1.0, 1.0), // the hit is behind the origin
                    7..=11 => (2.0, 1.0), // the hit is past the end
                    _ => (0.5, 1.0),      // on the segment
                };
                put_v(c, TREE + 0x60, [0.0, 0.0, 0.0, 0.0]); // origin
                put_v(c, TREE + 0x70, [1.0, 0.0, 0.0, 0.0]); // dir
                put_v(c, N, [nx, 0.0, 0.0, 0.0]);
                put_v(c, P, [dx, 0.0, 0.0, 0.0]);
                put_v(c, B, [0.25, 0.5, 0.75, 1.0]);
                put_v(c, CC, [-0.25, -0.5, -0.75, 1.0]);
                put_v(c, HIT, [0.0; 4]);
                put_v(c, OUT_N, [0.0; 4]);
                c.bus.write32(OUT_T, 0);
                c.g[A0][0] = sx(TREE);
                c.g[A1][0] = sx(P);
                c.g[A2][0] = sx(B);
                c.g[A3][0] = sx(CC);
                c.g[T0][0] = sx(N);
                c.g[T1][0] = sx(HIT);
                c.g[T2][0] = sx(OUT_N);
                c.g[T3][0] = sx(OUT_T);
            },
        );
    }


    #[test]
    fn rider_sphere_tree_get_bounds_matches() {
        check(ssx3_ee::s10::f_00329dc8__generated, rider_sphere_tree_get_bounds, 300, &|c, seed| {
            let mut r = Rng(0x51f3_9a2b ^ seed as u64);
            scramble(c, &mut r);
            literals(c);
            c.g[GP][0] = GAME_GP as u64;
            let tree = DATA;
            c.g[A0][0] = tree as u64;
            c.g[A1][0] = (tree + 0x100) as u64;
            c.g[A2][0] = (tree + 0x110) as u64;
            // A leaf every fourth seed; otherwise a mask that selects some (or
            // none) of the children, with 0..2 of them counted.
            let leaf = seed % 4 == 0;
            let mask = if leaf { 0 } else { (seed / 4) % 4 + 1 };
            let count = (seed / 16) % 3;
            c.bus.write32(tree + 0x28, mask);
            c.bus.write32(tree + 0x2c, count);
            put_vec(c, tree + 0x10, [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.edge_f32()]);
            c.bus.write32(tree + 0x20, r.f32(50.0));
            for i in 0..2 {
                let kid = tree + 0x30 + i * 0x20;
                put_vec(c, kid, [r.f32(200.0), r.f32(200.0), r.f32(200.0), r.edge_f32()]);
                c.bus.write32(kid + 0x10, r.f32(30.0));
            }
        });
    }


    // CollSegment_ctor -----------------------------------------------------

    pub(crate) const SEGMENT: u32 = DATA;

    pub(crate) const SEG_P0: u32 = DATA + 0x100;

    pub(crate) const SEG_P1: u32 = DATA + 0x200;


    #[test]
    fn coll_segment_ctor_matches() {
        check(
            ssx3_ee::s11::f_0032e100__generated,
            coll_segment_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x0032_e100 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = SEGMENT as u64;
                c.g[A1][0] = SEG_P0 as u64;
                c.g[A2][0] = SEG_P1 as u64;
                c.g[A3][0] = (seed % 3) as u64;
                put_vec(
                    c,
                    SEG_P0,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_vec(
                    c,
                    SEG_P1,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                c.f[12] = r.f32(3.0);
                c.g[GP][0] = GAME_GP as u64;
            },
        );
    }


    // CollHull_ctor --------------------------------------------------------

    pub(crate) const HULL: u32 = DATA;

    pub(crate) const HULL_TREE: u32 = DATA + 0x1000;


    #[test]
    fn coll_hull_ctor_matches() {
        check(ssx3_ee::s11::f_0032f650__generated, coll_hull_ctor, 200, &|c, seed| {
            let mut r = Rng(0x0032_f650 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = HULL as u64;
            c.g[A1][0] = HULL_TREE as u64;
            c.g[A2][0] = (seed % 2) as u64;
            // An empty sphere tree: the bounds come from the empty-box
            // globals and there is no per-sphere loop.
            c.bus.write32(HULL_TREE + 0x28, 0);
            c.bus.write32(GAME_GP - 0x2de8, f(1.5));
            c.bus.write32(GAME_GP - 0x2de4, f(-2.5));
        });
    }


    #[test]
    fn rider_sphere_tree_dtor_matches() {
        check(ssx3_ee::s10::f_00329970__generated, rider_sphere_tree_dtor, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x3299_70, 0);
            // Freeing a live object would walk the (uninitialised) heap manager,
            // so the delete branch is driven with a null object and the keep
            // branch with a real one: both reach the same code.
            if seed % 2 == 1 {
                c.g[A0][0] = 0;
                c.g[A1][0] = 1;
            } else {
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
                c.g[A1][0] = sx(r.next_u32() & !1);
            }
        });
    }


    #[test]
    fn coll_hull_dtor_matches() {
        check(ssx3_ee::s11::f_0032f708__generated, coll_hull_dtor, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x32f7_08, 0);
            let flags = (r.next_u32() & !1) | (seed % 2) as u32;
            c.g[A1][0] = sx(flags);
            c.g[A0][0] = if flags & 1 != 0 {
                0 // delete(null): a live hull would go to the heap manager
            } else {
                sx(DATA + (r.next_u32() & 0x1ff0))
            };
        });
    }


    #[test]
    fn vu0_triangle_intersect_matches() {
        check(ssx3_ee::s10::f_0032b6a8__generated, vu0_triangle_intersect, 100, &|c, seed| {
            let mut r = Rng(0x32b6_a801 ^ seed as u64);
            scramble(c, &mut r);
            for k in 0..4 {
                let at = DATA + 0x100 + 0x10 * k as u32;
                c.g[A0 + k][0] = sx(at);
                put_vec(c, at, [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()]);
            }
        });
    }


    // -----------------------------------------------------------------------
    // Sphere tree helpers.

    #[test]
    fn sphere_tree_ctor_matches() {
        check(ssx3_ee::s10::f_0032c508__generated, sphere_tree_ctor, 32, &|c, seed| {
            let mut r = machine(c, seed, 0x32c5_08, 0);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x98, r.next_u32());
            for i in 0..8u32 {
                c.bus.write32(DATA + i * 16, r.next_u32());
            }
        });
    }


    #[test]
    fn coll_sphere_collide_aabb_matches() {
        check(ssx3_ee::s11::f_0032fe40__generated, coll_sphere_collide_aabb, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x32fe_40, 0);
            let this = DATA;
            let box_ = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (DATA + 0x2000) as u64;
            c.g[A2][0] = (DATA + 0x3000) as u64;
            c.g[A3][0] = box_ as u64;
            for i in 0..0x60u32 / 4 {
                c.bus.write32(this + i * 4, r.next_u32());
                c.bus.write32(box_ + i * 4, r.edge_f32());
            }
        });
    }


    #[test]
    fn rider_sphere_tree_set_spheres_matches() {
        check(
            ee::s10::f_00329a90__generated,
            rider_sphere_tree_set_spheres,
            SEEDS_3,
            &|c, seed| {
                let mut r = base(c, seed, 0x329a_90);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                let spheres = DATA + 0x1000;
                let radii = DATA + 0x2000;
                for i in 0..6u32 {
                    for k in 0..4 {
                        c.bus.write32(spheres + 0x10 * i + 4 * k, r.next_u32());
                        c.bus.write32(radii + 4 * (4 * i + k), r.edge_f32());
                    }
                }
                let center = DATA + 0x3000;
                for k in 0..4 {
                    c.bus.write32(center + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = [-1i64 as u64, 0, 1, 3, 6][(seed % 5) as usize];
                c.g[A2][0] = sx(spheres);
                c.g[A3][0] = sx(radii);
                c.g[T0][0] = sx(center);
                c.f[12] = r.edge_f32();
            },
        );
    }


    #[test]
    fn sphere_tree_reset_transform_matches() {
        check(
            ee::s10::f_0032c540__generated,
            sphere_tree_reset_transform,
            SEEDS_3,
            &|c, seed| {
                let mut r = base(c, seed, 0x32c5_40);
                let this = DATA + 0x100;
                clear(c, this, 0x100);
                let table = 0x004f_f640u32;
                for i in 0..8u32 {
                    for k in 0..4 {
                        c.bus.write32(table + 0x10 * i + 4 * k, r.next_u32());
                    }
                }
                for k in 0..4 {
                    c.bus.write32(0x004f_f130 + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(this);
            },
        );
    }


    // ---------------------------------------------------------- collision

    #[test]
    fn coll_sphere_transform_matches() {
        check(
            ssx3_ee::s11::f_0032fd40__generated,
            coll_sphere_transform,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0xfd40, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                fill(c, &mut r, DATA, 0x40);
                for i in 0..16 {
                    c.bus.write32(DATA + 0x100 + 4 * i, r.f32(4.0));
                }
                c.f[12] = r.edge_f32();
            },
        );
    }


    #[test]
    fn coll_segment_reset_transform_matches() {
        check(ee::s11::f_0032e4b8__generated, coll_segment_reset_transform, 200, &|c, seed| {
            let mut r = setup(c, seed, 47);
            c.g[A0][0] = AUX as u64;
            fillb(c, &mut r, AUX + 0x60, 0x40);
            put(c, AUX + 4, r.next_u32());
        });
    }


    #[test]
    fn coll_segment_collide_aabb_matches() {
        check(ee::s11::f_0032e690__generated, coll_segment_collide_aabb, 250, &|c, seed| {
            let mut r = setup(c, seed, 48);
            gpf(c, 0x2888, if seed % 2 == 0 { 0 } else { 1 });
            gpf(c, -0x2d74, fbits(1e-6));
            let a = |x: f32| fbits(x);
            let (o, d, lo, hi): ([u32; 3], [u32; 3], [u32; 3], [u32; 3]) = match seed % 8 {
                // Through the box: an entry and an exit hit.
                0 => ([0, 0, 0], [a(1.0); 3], [a(-0.5); 3], [a(0.5); 3]),
                // Origin inside, exit ahead only: one hit.
                1 => ([0, 0, 0], [a(1.0); 3], [a(-2.0); 3], [a(0.5); 3]),
                // Box behind the origin: miss (tmax < 0).
                2 => ([0, 0, 0], [a(1.0); 3], [a(-1.0); 3], [a(-0.5); 3]),
                // Box beyond the segment: miss (1 < tmin).
                3 => ([0, 0, 0], [a(1.0); 3], [a(2.0); 3], [a(3.0); 3]),
                // Parallel to x.
                4 => ([0, 0, 0], [0, a(1.0), a(1.0)], [a(-0.5); 3], [a(0.5); 3]),
                // Box to the side: miss (tmax < tmin).
                5 => (
                    [0, 0, 0],
                    [a(1.0); 3],
                    [a(5.0), a(-1.0), a(-1.0)],
                    [a(6.0), a(1.0), a(1.0)],
                ),
                // Entering from the far corner.
                6 => ([a(1.0); 3], [a(-1.0); 3], [a(0.2); 3], [a(0.8); 3]),
                // Arbitrary.
                _ => {
                    let mut v = [0u32; 3];
                    for slot in v.iter_mut() {
                        *slot = r.f32(2.0);
                    }
                    let mut mn = [0u32; 3];
                    let mut mx = [0u32; 3];
                    for k in 0..3usize {
                        let x = f32::from_bits(r.f32(2.0));
                        let y = f32::from_bits(r.f32(2.0));
                        mn[k] = fbits(x.min(y));
                        mx[k] = fbits(x.max(y));
                    }
                    (v, [r.f32(2.0), r.f32(2.0), r.f32(2.0)], mn, mx)
                }
            };
            c.g[A0][0] = AUX as u64;
            for k in 0..3usize {
                let off = 4 * k as u32;
                put(c, AUX + 0x60 + off, o[k]);
                put(c, AUX + 0x70 + off, d[k]);
                put(c, AUX + 0x200 + off, lo[k]);
                put(c, AUX + 0x210 + off, hi[k]);
            }
            c.g[A1][0] = (AUX + 0x200) as u64;
            c.g[A2][0] = (AUX + 0x210) as u64;
            c.g[A3][0] = (AUX + 0x400) as u64;
            fillb(c, &mut r, AUX + 0x400, 0x120);
        });
    }


    // ----------------------------------------------------------------- front end

    #[test]
    fn coll_hull_get_diameter_matches() {
        check(ssx3_ee::s11::f_0032f8c0__generated, coll_hull_get_diameter, SEEDS_4, &|c, seed| {
            let mut r = machine(c, seed, 0x32f8c0, 0);
            c.g[A0][0] = sx(DATA);
            if seed % 4 == 0 {
                word(c, DATA, 0);
            } else {
                word(c, DATA, DATA + 0x100);
                word(c, DATA + 0x160, DATA + 0x200);
                word(c, DATA + 0x220, r.edge_f32());
            }
        });
    }


    #[test]
    fn coll_sphere_reset_transform_matches() {
        check(ssx3_ee::s11::f_0032fd98__generated, coll_sphere_reset_transform, SEEDS_4, &|c, seed| {
            let mut r = machine(c, seed, 0x32fd98, 0);
            c.g[A0][0] = sx(DATA);
            for i in 0..5 {
                word(c, DATA + 0x80 + 4 * i, r.next_u32());
            }
            word(c, DATA + 4, r.next_u32());
            word(c, DATA + 0x60, r.next_u32());
        });
    }


    #[test]
    fn rider_sphere_tree_init_matches() {
        check(ee::s10::f_003299c8__generated, rider_sphere_tree_init, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x0032_99c8, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = (seed % 5) as u64;
            c.g[A3][0] = (DATA + 0x400) as u64;
            c.g[T0][0] = (DATA + 0x500) as u64;
            c.g[T1][0] = (seed % 9) as u64;
            c.f[12] = r.edge_f32();
            for i in 0..5u32 {
                w(c, DATA + 0x400 + 4 * i, r.next_u32());
                w(c, DATA + 0x500 + 4 * i, r.edge_f32());
            }
        });
    }


        #[test]
    fn coll_hull_collide_sphere_tree_matches() {
        check(ee::s11::f_0032fa30__generated, coll_hull_collide_sphere_tree, 80, &|c, seed| {
            let _ = base(c, seed, 0x32_fa30);
            globals(c);
            // A zeroed tree node: the collide call finds nothing.
            const NODE: u32 = DATA + 0x2000;
            w(c, NODE + 0x98, NODE + 0x100);
            w(c, NODE + 0x100 + 0x20, 0);
            w(c, DATA + 0x60, NODE);
            w(c, DATA + 0x80, NODE + 0x100);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x3000);
            c.g[A2][0] = sx(DATA + 0x4000);
        });
    }


    #[test]
    fn coll_sphere_plane_distance_matches() {
        check(ee::s11::f_00330360__generated, coll_sphere_plane_distance, 150, &|c, seed| {
            let mut r = base(c, seed, 0x33_0360);
            globals(c);
            const PLANE: u32 = DATA + 0x100;
            const PN: u32 = DATA + 0x200;
            const OUT: u32 = DATA + 0x300;
            for i in 0..4u32 {
                c.bus.write32(PLANE + 4 * i, r.edge_f32());
                c.bus.write32(PN + 4 * i, r.edge_f32());
                c.bus.write32(DATA + 0x60 + 4 * i, r.edge_f32());
            }
            c.bus.write32(DATA + 0x70, r.f32(0.01));
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(PLANE);
            c.g[A2][0] = sx(PN);
            c.g[A3][0] = sx(OUT);
        });
    }


        #[test]
    fn sphere_tree_collide_sphere_matches() {
        check(
            ee::s10::f_0032c898__generated,
            sphere_tree_collide_sphere,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32c8, 0);
                env(c);
                let this = DATA + 0x400;
                let tree = DATA + 0x1000;
                w32(c, this + 0x98, tree);
                w32(c, tree + 8, (seed % 3 != 0) as u32);
                for k in 0..4u32 {
                    w32(c, DATA + 0x2000 + 4 * k, r.edge_f32());
                    w32(c, DATA + 0x2100 + 4 * k, r.edge_f32());
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x2100);
                c.g[A3][0] = sx(DATA + 0x2200);
                c.f[12] = r.f32(8.0);
            },
        );
    }


    #[test]
    fn sphere_tree_collide_aabb_matches() {
        check(
            ee::s10::f_0032c928__generated,
            sphere_tree_collide_aabb,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32c9, 0);
                env(c);
                let this = DATA + 0x400;
                let tree = DATA + 0x1000;
                w32(c, this + 0x98, tree);
                w32(c, tree + 8, (seed % 3 != 0) as u32);
                for k in 0..8u32 {
                    w32(c, DATA + 0x2000 + 4 * k, r.edge_f32());
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x2100);
                c.g[A3][0] = sx(DATA + 0x2200);
                c.g[T0][0] = sx(DATA + 0x2300);
            },
        );
    }


    #[test]
    fn coll_sphere_collide_sphere_tree_matches() {
        check(
            ee::s11::f_0032fdb0__generated,
            coll_sphere_collide_sphere_tree,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32fd, 0);
                env(c);
                let this = DATA + 0x400;
                let tree = DATA + 0x1000;
                let hit = DATA + 0x2000;
                for k in 0..4u32 {
                    w32(c, this + 0x60 + 4 * k, r.edge_f32());
                    w32(c, hit + 0x10 + 4 * k, r.f32(2.0));
                }
                w32(c, this + 0x70, r.f32(4.0));
                w32(c, tree + 0x98, DATA + 0x3000);
                w32(c, DATA + 0x3000 + 8, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(tree);
                c.g[A2][0] = sx(hit);
            },
        );
    }


    // ------------------------------------------------------------------
    // Collision.

    pub(crate) fn coll_args(c: &mut Cpu, seed: u32, salt: u64, addr: u32, f: GuestFn) {
        let _r = base_2(c, seed, salt, 1);
        let this = DATA + 0x1000;
        c.g[A0][0] = this as u64;
        c.g[A1][0] = (DATA + 0x2000) as u64;
        c.g[A2][0] = (DATA + 0x2100) as u64;
        c.g[A3][0] = (DATA + 0x2200) as u64;
        c.g[T0][0] = (DATA + 0x2300) as u64;
        c.g[T1][0] = (DATA + 0x2400) as u64;
        c.g[T2][0] = (DATA + 0x5000) as u64;
        c.g[T3][0] = (DATA + 0x6000) as u64;
        w_2(c, this + 0x60, addr);
        put_vec(c, DATA + 0x2400, [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits(), 0]);
        w_2(c, addr, 0);
        let _ = f;
    }


    #[test]
    fn coll_hull_collide_triangle2_sided_matches() {
        check(ssx3_ee::s11::f_0032f990__generated, coll_hull_collide_triangle2_sided, 20, &|c, seed| {
            coll_args(c, seed, 0x32f990, DATA + 0x3000, ssx3_ee::s11::f_0032f990__generated);
        });
    }


    #[test]
    fn sphere_tree_collide_sphere_tree_matches() {
        check(ssx3_ee::s10::f_0032cb58__generated, sphere_tree_collide_sphere_tree, 20, &|c, seed| {
            let _r = base_2(c, seed, 0x32cb58, 2);
            let a = DATA + 0x1000;
            let b = DATA + 0x2000;
            c.g[A0][0] = a as u64;
            c.g[A1][0] = b as u64;
            c.g[A2][0] = (DATA + 0x3000) as u64;
            c.g[A3][0] = (DATA + 0x4000) as u64;
            w_2(c, a + 0x98, DATA + 0x5000);
            w_2(c, b + 0x98, DATA + 0x6000);
            w_2(c, DATA + 0x5000 + 8, (seed & 1) as u32);
            w_2(c, DATA + 0x6000 + 8, ((seed >> 1) & 1) as u32);
            w_2(c, a + 0x80, 0);
        });
    }


    // ------------------------------------------------------------ Collision.

    #[test]
    fn coll_sphere_ctor_matches() {
        check(ee::s11::f_0032fb48__generated, coll_sphere_ctor, 200, &|c, seed| {
            let mut r = base_3(c, seed, 0x32_fb48);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x1000);
            c.g[A2][0] = sx((seed % 3) as u32);
            c.f[12] = r.edge_f32();
            for i in 0..4u32 {
                c.bus.write32(DATA + 0x1000 + i * 4, r.edge_f32());
            }
        });
    }


    #[test]
    fn spheretree_collide_triangle_matches() {
        check(
            ee::s10::f_0032ca78__generated,
            spheretree_collide_triangle,
            8,
            &|c, seed| {
                let _ = base_4(c, seed, 0x32ca78, 2);
                let this = DATA;
                clear_2(c, this, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (DATA + 0x300) as u64;
                c.g[A3][0] = 0;
                c.g[T0][0] = (seed % 2) as u64;
                c.g[T1][0] = (seed % 3) as u64;
                c.g[T2][0] = (seed % 5) as u64;
                w(c, this + 0x98, if seed % 3 == 0 { 0 } else { DATA + 0x500 });
                clear_2(c, DATA + 0x500, 0x100);
                wf(c, this + 0x94, 0.0);
            },
        );
    }


    #[test]
    fn shell_sort_desc_key_val_matches() {
        strict(ee::s11::f_0032da40__generated, shell_sort_desc_key_val, SEEDS_6, &|c, seed| {
            let mut r = base_5(c, seed, 0xda40, 0);
            let keys = DATA + 0x1000;
            let vals = DATA + 0x2000;
            let n = (seed % 7) as u32;
            // the gap table the sort walks down
            for (i, g) in [1u32, 4, 13, 40, 121, 364].iter().enumerate() {
                w(c, 0x0044_af98 + 4 * i as u32, *g);
            }
            for i in 0..n {
                let k = if i > 0 && seed % 3 == 0 { i - 1 } else { r.next_u32() % 100 };
                w(c, keys + 4 * i, k);
                w(c, vals + 4 * i, r.next_u32());
            }
            c.g[A0][0] = n as u64;
            c.g[A1][0] = sx(keys);
            c.g[A2][0] = sx(vals);
        });
    }


    pub(crate) fn put3(c: &mut Cpu, addr: u32, v: [u32; 3]) {
        for (i, x) in v.into_iter().enumerate() {
            c.bus.write32(addr + 4 * i as u32, x);
        }
    }


    #[test]
    fn coll_segment_overlaps_aabb_matches() {
        check_all(
            ee::s11::f_0032e288__generated,
            coll_segment_overlaps_aabb,
            300,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x32_e288);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x120);
                match seed % 3 {
                    0 => {
                        // Overlapping: centre 10, half extents 1, box [0, 20].
                        put3(
                            c,
                            DATA + 0x60,
                            [10.0f32.to_bits(), 10.0f32.to_bits(), 10.0f32.to_bits()],
                        );
                        put3(
                            c,
                            DATA + 0x70,
                            [1.0f32.to_bits(), 1.0f32.to_bits(), 1.0f32.to_bits()],
                        );
                        put3(c, DATA + 0x100, [0; 3]);
                        put3(c, DATA + 0x120, [20.0f32.to_bits(); 3]);
                    }
                    1 => {
                        // Separated on the first axis.
                        put3(c, DATA + 0x60, [10.0f32.to_bits(); 3]);
                        put3(c, DATA + 0x70, [1.0f32.to_bits(); 3]);
                        put3(c, DATA + 0x100, [100.0f32.to_bits(); 3]);
                        put3(c, DATA + 0x120, [200.0f32.to_bits(); 3]);
                    }
                    _ => {
                        floats(c, &mut r, DATA + 0x60, 8);
                        floats(c, &mut r, DATA + 0x100, 3);
                        floats(c, &mut r, DATA + 0x120, 3);
                    }
                }
                // The VU0 sum is stored to the frame first.
                floats(c, &mut r, DATA + 0x80, 8);
            },
        );
    }


        #[test]
    fn coll_sphere_hits_triangle_edges_matches() {
        check_all(
            ee::s11::f_00330250__generated,
            coll_sphere_hits_triangle_edges,
            200,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x33_0250);
                c.g[A0][0] = sx(DATA); // sphere: centre +0x60, radius +0x70
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x120);
                c.g[A3][0] = sx(DATA + 0x140);
                match seed % 3 {
                    0 => {
                        // Tiny radius: every edge is tested.
                        put3(c, DATA + 0x60, [0; 3]);
                        c.bus.write32(DATA + 0x70, 0.001f32.to_bits());
                    }
                    1 => {
                        // Huge radius: the first edge already hits.
                        put3(c, DATA + 0x60, [0; 3]);
                        c.bus.write32(DATA + 0x70, 1.0e6f32.to_bits());
                    }
                    _ => {
                        floats(c, &mut r, DATA + 0x60, 4);
                        c.bus.write32(DATA + 0x70, r.edge_f32());
                    }
                }
                for base_off in [0x100u32, 0x120, 0x140] {
                    match seed % 3 {
                        0 => put3(
                            c,
                            DATA + base_off,
                            [
                                (base_off as f32).to_bits(),
                                2.0f32.to_bits(),
                                3.0f32.to_bits(),
                            ],
                        ),
                        _ => floats(c, &mut r, DATA + base_off, 4),
                    }
                }
            },
        );
    }


    #[test]
    fn coll_sphere_overlaps_aabb_matches() {
        same(
            ssx3_ee::s11::f_0032fc08__generated,
            coll_sphere_overlaps_aabb,
            200,
            &|c, seed| {
                let mut r = Rng(0x32fc08 ^ seed as u64);
                scramble(c, &mut r);
                let s = DATA + 0x100;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (DATA + 0x240) as u64;
                put_vec(
                    c,
                    s + 0x60,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                c.bus.write32(s + 0x70, r.edge_f32());
                for i in 0..3 {
                    c.bus.write32(DATA + 0x200 + 4 * i, r.edge_f32());
                    c.bus.write32(DATA + 0x240 + 4 * i, r.edge_f32());
                }
            },
        );
    }
}
