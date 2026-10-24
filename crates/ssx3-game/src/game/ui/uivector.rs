//! `src/ui/uivector.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
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
pub(crate) use crate::game::ai::ai::virtual_call_slot;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_2;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::object::railmodifier::reset_vtable_and_delete;
pub(crate) use crate::game::object::railmodifier::DELETE;
pub(crate) use crate::game::sound::bankmonitor::add32;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::ui::uithing::ui_anim_update;
pub(crate) use crate::game::world::wscriptcache::DELETE_2;
pub(crate) use crate::game::wscript::wscriptman::p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_3d28, ui_vector_update_anim, "UIVector_UpdateAnim"),
    (0x003a_3d70, ui_vector_v20, "UIVector_v20"),
    (0x003a_3e38, ui_vector_v21, "UIVector_v21"),
    (0x003a_47a8, ui_visitor_dtor, "UIVisitor_dtor"),
    (0x003a_47f8, ui_list_node_dtor, "UIListNode_dtor"),
    (0x003a_4888, ui_list_dtor, "UIList_dtor"),
    (0x003a_49e8, ui_object_v04, "UIObject_v04"),
    (0x003a_4a08, ui_object_v05, "UIObject_v05"),
    (0x003a_4a28, ui_object_v06, "UIObject_v06"),
    (0x003a_4a48, ui_object_set_focus, "UIObject_SetFocus"),
    (0x003a_4a68, ui_object_v08, "UIObject_v08"),
    (0x003a_4a88, ui_object_set_visible, "UIObject_SetVisible"),
    (0x003a_4af8, ui_object_set_color, "UIObject_SetColor"),
    (0x003a_4c90, ui_object_get_vec3_at60, "UIObject_GetVec3At60"),
    (0x003a_4cb0, ui_object_set_vec3_at60, "UIObject_SetVec3At60"),
    (0x003a_4cd0, ui_input_visitor_dtor, "UIInputVisitor_dtor"),
    (0x003a_4d00, ui_input_visitor_visit, "UIInputVisitor_Visit"),
    (0x003a_4d40, ui_focus_visitor_dtor, "UIFocusVisitor_dtor"),
    (0x003a_4d70, ui_focus_visitor_visit, "UIFocusVisitor_Visit"),
    (0x003a_5068, c_ui_group_dtor, "cUIGroup_dtor"),
    (0x003a_51a0, c_ui_group_accept, "cUIGroup_Accept"),
    (0x003a_51d0, ui_group_v03, "cUIGroup_v03"),
    (0x003a_52a0, ui_menu_dtor, "UIMenu_dtor"),
    (0x003a_54b8, ui_pair_dtor, "UIPair_dtor"),
    (0x003a_5658, ui_list_box_dtor, "UIListBox_dtor"),
    (0x003a_5708, ui_list_box_v24, "UIListBox_v24"),
    (0x003a_5b48, ui_thread_dtor, "UIThread_dtor"),
    (0x003a_5da8, ui_text_dtor, "UIText_dtor"),
    (0x003a_5e68, ui_text_v23, "UIText_v23"),
    (0x003a_5e88, ui_text_v24, "UIText_v24"),
    (0x003a_5ea8, ui_text_v25, "UIText_v25"),
    (0x003a_5ec8, ui_text_v26, "UIText_v26"),
    (0x003a_5ee8, ui_text_v27, "UIText_v27"),
    (0x003a_62a0, c_ui_text_scroll_dtor, "cUITextScroll_dtor"),
    (0x003a_6378, c_ui_text_scroll_v23, "cUITextScroll_v23"),
    (0x003a_6398, c_ui_text_scroll_v24, "cUITextScroll_v24"),
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2 as BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2 as BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2 as SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2 as SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2 as SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE as UI_ANIM_ADVANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY as UI_ANIM_APPLY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2 as UI_OBJECT_DTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UNLOAD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamemode::callee::FE_ASYNC_MANAGER_SET_FILE_STATUS;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_REQUEST_INSTANT_REPLAY;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;
    pub(crate) use crate::game::main::gamemode::callee::RUN_LIST_SEEK;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_LIST_ON_SEQ_ADVANCE;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF as SPRINTF_2;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamemode::callee::STRCPY;
    pub(crate) use crate::game::main::gamemode::callee::STRLEN;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF as UI_OBJECT_SHOW_CLAMP_ALPHA_HALF_2;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES as CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR as UI_OBJECT_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
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

    pub const UI_PAIR_FREE_CHILDREN: Callee = Callee {
        addr: 0x0039_be70,
        f: ee::s13::f_0039be70,
    };

    pub const UI_OBJECT_DTOR: Callee = Callee {
        addr: 0x0039_fc48,
        f: ee::s13::f_0039fc48,
    };

    pub const UI_LIST_BOX_CLEAR: Callee = Callee {
        addr: 0x0039_a670,
        f: ee::s13::f_0039a670,
    };

    pub const UI_TEXT_FREE_STRINGS: Callee = Callee {
        addr: 0x003a_0b38,
        f: ee::s13::f_003a0b38,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_VEC_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BXFILE_EXISTS;
    pub(crate) use crate::game::wscript::wscriptman::table::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::wscript::wscriptman::table::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::wscript::wscriptman::table::MEM_FREE;
    pub(crate) use crate::game::wscript::wscriptman::table::SCE_MC_FLUSH;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32_2;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_BANK_REMOVE;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_MIDI_RESUME;
    pub(crate) use crate::game::wscript::wscriptman::table::STREAM_FILE_DESTROY;
    pub(crate) use crate::game::wscript::wscriptman::table::STR_HASH;
    pub(crate) use crate::game::wscript::wscriptman::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::wscript::wscriptman::table::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::table::WSCRIPT_PROCESS_LIST_POP_FRONT;
}

/// `UIVisitor` destroying destructor (0x3a47a8): the visitor vtable
/// `D_00494CE8`.
pub fn ui_visitor_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x003a_47cc, 0x0049_4ce8, 0)
    })
}

/// `UIInputVisitor` destroying destructor (0x3a4cd0).
pub fn ui_input_visitor_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x003a_4cf4, 0x0049_4ce8, 0)
    })
}

/// `UIFocusVisitor` destroying destructor (0x3a4d40).
pub fn ui_focus_visitor_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x003a_4d64, 0x0049_4ce8, 0)
    })
}

/// `cUIGroup::Accept(self, node)` (0x3a51a0): `node->v02(self)`.
pub fn c_ui_group_accept(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x003a_51c4, None))
}

/// `int UIFocusVisitor_Visit(UIList* list, UIObject* obj)` (0x3a4d70): call
/// `obj->vtable[0x38/4]` with the visitor state `list+4`, then return 0.
pub fn ui_focus_visitor_visit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
        c.g[A1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x38).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x3c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x003a_4d94)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    })
}

/// `void* UIInputVisitor_Visit(Visitor* unused, UIObject* obj /*a1*/)`
/// (0x3a4d00): call the object's accept method (virtual slot 0x1e) with
/// `obj->+4`; return the object (or null when it was not accepted).
pub fn ui_input_visitor_visit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[V1][0] = Addr(r32(c, S0)).off(8).lw(c);
        c.g[A1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x78).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x7c).lw(c);
        let func = r32(c, V0);
        c.g[RA][0] = sx(0x003a_4d28);
        c.g[A0][0] = add32(c.g[S0][0], c.g[A0][0]);
        c.pc = func;
        call_indirect(c, func);
        if c.pc != 0x003a_4d28 {
            return Err(crate::call::Unwound);
        }
        if c.g[V0][0] == 0 {
            c.g[S0][0] = 0;
        }
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        leave(c, 0x20);
        Ok(())
    })
}

/// `void UIVector_UpdateAnim(UIVector* v, ushort event)` (0x3a3d28): the same
/// for a vector widget.
pub fn ui_vector_update_anim(c: &mut Cpu) {
    guest(c, |c| ui_anim_update(c, 0x003a_3d50, 0x003a_3d5c))
}

/// The shared body of the three derived UI destructors: install `vtable` at
/// `+8`, run the class's own teardown (`clear`), then `UIObject_dtor`.
pub(crate) fn ui_derived_dtor(c: &mut Cpu, vtable: u32, clear: Callee, ret1: u32, ret2: u32) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(vtable);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(c.g[S0][0] as u32).off(8).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        clear.call(c, ret1)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        UI_OBJECT_DTOR.call(c, ret2)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIPair_dtor(this, flags)` (0x3a54b8).
pub fn ui_pair_dtor(c: &mut Cpu) {
    ui_derived_dtor(
        c,
        0x0049_4868,
        UI_PAIR_FREE_CHILDREN,
        0x003a_54e0,
        0x003a_54ec,
    )
}

/// `void UIListBox_dtor(this, flags)` (0x3a5658).
pub fn ui_list_box_dtor(c: &mut Cpu) {
    ui_derived_dtor(c, 0x0049_4798, UI_LIST_BOX_CLEAR, 0x003a_5680, 0x003a_568c)
}

/// `void UIText_dtor(this, flags)` (0x3a5da8).
pub fn ui_text_dtor(c: &mut Cpu) {
    ui_derived_dtor(
        c,
        0x0049_4588,
        UI_TEXT_FREE_STRINGS,
        0x003a_5dd0,
        0x003a_5ddc,
    )
}

/// `void UIListNode_dtor(UIListNode* self, int flags)` (0x3a47f8): install
/// the list-node vtable, run the body and maybe delete the node.
pub fn ui_list_node_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_4cc0; // D_00494CC0
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x003a_4820)?;
        c.g[S0][0] &= 1;
        let owned = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if owned {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x003a_4834)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIThread_dtor(UIThread* self, int flags)` (0x3a5b48): the thread
/// node shares the list-node body and vtable.
pub fn ui_thread_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_4cc0; // D_00494CC0
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x003a_5b70)?;
        c.g[S0][0] &= 1;
        let owned = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if owned {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x003a_5b84)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Field offsets.

/// The UI widget flag words: `UIObject` at `+0x14`, its subclasses at `+0x74`.
pub(crate) const UI_FLAGS_OBJECT: u32 = 0x14;

pub(crate) const UI_FLAGS_WIDGET: u32 = 0x74;

/// `sVec3` at `+0x60` of a UI object, `+0x1c` the colour, `+0x80` the point
/// array, `+0x74` the `{lo:17, count:15}` size field.
pub(crate) const UI_VEC3: u32 = 0x60;

pub(crate) const UI_COLOR: u32 = 0x1c;

pub(crate) const UI_POINTS: u32 = 0x80;

pub(crate) const UI_SIZE: u32 = 0x74;

/// The low 32 bits of a register (a pointer).
#[inline(always)]
pub(crate) fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

// ---------------------------------------------------------------------------
// UI widget flags.

/// A `bitN: 1` setter: `w = *(self + off); w = (w & !bit) | (enable != 0) << n;
/// *(self + off) = w`, leaving the word in `v0` and the mask/value in `v1`/`a1`
/// as the original does.
pub(crate) fn set_ui_flag(c: &mut Cpu, off: u32, bit: u32) {
    c.cycles += 8;
    let ui = abi::arg(c, 0);
    c.g[V0][0] = c.lw(p(ui, off));
    c.g[A1][0] = (0u64 < c.g[A1][0]) as u64;
    c.g[V1][0] = sx(!(1u32 << bit));
    c.g[A1][0] = sx((c.g[A1][0] as u32) << bit);
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] |= c.g[A1][0];
    let ra = r(c, RA);
    c.sw(p(ui, off), c.g[V0][0]);
    c.pc = ra;
}

/// `UIObject_v04` (0x003a49e8): `bit1 = enable` of the flag word at `+0x14`.
pub fn ui_object_v04(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 1);
}

/// `UIObject_v05` (0x003a4a08): `bit2 = enable`.
pub fn ui_object_v05(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 2);
}

/// `UIObject_v06` (0x003a4a28): `bit3 = enable`.
pub fn ui_object_v06(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 3);
}

/// `UIObject_SetFocus` (0x003a4a48): `bit4 = enable` (the focused state).
pub fn ui_object_set_focus(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 4);
}

/// `UIObject_v08` (0x003a4a68): `bit5 = enable`.
pub fn ui_object_v08(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 5);
}

/// `UIObject_SetVisible` (0x003a4a88): `bit6 = enable` (the visible state).
pub fn ui_object_set_visible(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 6);
}

/// `UIListBox_v24` (0x003a5708): `bit5 = enable` of the widget flag word at
/// `+0x74`.
pub fn ui_list_box_v24(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 5);
}

/// `UIText_v23` (0x003a5e68): `bit2 = enable` of the widget flag word.
pub fn ui_text_v23(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 2);
}

/// `UIText_v24` (0x003a5e88): `bit3 = enable`.
pub fn ui_text_v24(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 3);
}

/// `UIText_v25` (0x003a5ea8): `bit4 = enable`.
pub fn ui_text_v25(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 4);
}

/// `UIText_v26` (0x003a5ec8): `bit5 = enable`.
pub fn ui_text_v26(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 5);
}

/// `UIText_v27` (0x003a5ee8): `bit6 = enable`.
pub fn ui_text_v27(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 6);
}

/// `cUITextScroll_v23` (0x003a6378): `bit3 = enable`.
pub fn c_ui_text_scroll_v23(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 3);
}

/// `cUITextScroll_v24` (0x003a6398): `bit4 = enable`.
pub fn c_ui_text_scroll_v24(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 4);
}

// ---------------------------------------------------------------------------
// UI vec3 accessors and colour.

/// `UIObject_SetColor` (0x003a4af8): `*(self + 0x1c) = *in` (16 bytes).
pub fn ui_object_set_color(c: &mut Cpu) {
    c.cycles += 10;
    let ui = abi::arg(c, 0);
    let color = abi::arg(c, 1);
    c.ldl(2, p(color, 7));
    c.ldr(2, color);
    c.ldl(3, p(color, 0xf));
    c.ldr(3, p(color, 8));
    c.sdl(2, p(ui, UI_COLOR + 7));
    c.sdr(2, p(ui, UI_COLOR));
    c.sdl(3, p(ui, UI_COLOR + 0xf));
    c.sdr(3, p(ui, UI_COLOR + 8));
    abi::ret(c);
}

/// `UIObject_GetVec3At60` (0x003a4c90): `*out = *(sVec3*)(self + 0x60)`.
pub fn ui_object_get_vec3_at60(c: &mut Cpu) {
    c.cycles += 8;
    let ui = abi::arg(c, 0);
    let out = abi::arg(c, 1);
    c.ldl(2, p(ui, UI_VEC3 + 7));
    c.ldr(2, p(ui, UI_VEC3));
    c.g[V1][0] = c.lw(p(ui, UI_VEC3 + 8));
    c.sdl(2, p(out, 7));
    c.sdr(2, out);
    c.sw(p(out, 8), c.g[V1][0]);
    abi::ret(c);
}

/// `UIObject_SetVec3At60` (0x003a4cb0): `*(sVec3*)(self + 0x60) = *in`.
pub fn ui_object_set_vec3_at60(c: &mut Cpu) {
    c.cycles += 8;
    let ui = abi::arg(c, 0);
    let v = abi::arg(c, 1);
    c.ldl(2, p(v, 7));
    c.ldr(2, v);
    c.g[V1][0] = c.lw(p(v, 8));
    c.sdl(2, p(ui, UI_VEC3 + 7));
    c.sdr(2, p(ui, UI_VEC3));
    c.sw(p(ui, UI_VEC3 + 8), c.g[V1][0]);
    abi::ret(c);
}

/// `UIVector_v20` (0x003a3d70): `getBounds(out)` — copy the default 12-byte
/// extent to `out`, then grow it to the x/y extent of the `count = size>>17`
/// points at `+0x80` (1000 / -1000 sentinels). No data: return at once.
pub fn ui_vector_v20(c: &mut Cpu) {
    c.cycles += 3;
    let vec = abi::arg(c, 0);
    let out = abi::arg(c, 1);
    c.g[V0][0] = c.lw(p(vec, UI_POINTS));
    let no_points = c.g[V0][0] == 0;
    c.g[V0][0] = 0x0050_0000;
    if no_points {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    // `D_004FF0D8`, the 12-byte default extent.
    c.cycles += 17;
    let default_extent = 0x0050_0000u32.wrapping_sub(0xf28);
    c.g[AT][0] = 0x447a_0000;
    c.mtc1(1, 3); // f3 = 1000.0
    c.g[T0][0] = sx(default_extent);
    c.ldl(3, p(default_extent, 7));
    c.ldr(3, default_extent);
    c.g[A2][0] = c.lw(p(default_extent, 8));
    c.sdl(3, p(out, 7));
    c.sdr(3, out);
    c.sw(p(out, 8), c.g[A2][0]);
    c.g[AT][0] = sx(0xc47a_0000);
    c.mtc1(1, 0); // f0 = -1000.0
    c.g[V1][0] = 0;
    c.g[V0][0] = c.lw(p(vec, UI_SIZE));
    c.mov_s(4, 3); // f4 = 1000.0
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 17);
    c.mov_s(1, 0); // f1 = -1000.0
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[A0][0] = c.lw(p(vec, UI_POINTS));
        c.g[A2][0] = c.g[V0][0];
        loop {
            c.cycles += 5;
            c.lwc1(2, r(c, A0));
            c.c_lt_s(2, 3);
            if c.fcc() {
                c.mov_s(3, 2); // minX = x
            }
            c.cycles += 4;
            c.c_lt_s(0, 2);
            if c.fcc() {
                c.mov_s(0, 2); // maxX = x
            }
            c.cycles += 5;
            c.lwc1(2, p(r(c, A0), 4));
            c.c_lt_s(2, 4);
            if c.fcc() {
                c.mov_s(4, 2); // minY = y
            }
            c.cycles += 4;
            c.c_lt_s(1, 2);
            if c.fcc() {
                c.mov_s(1, 2); // maxY = y
            }
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[V0][0] = (c.g[V1][0] < c.g[A2][0]) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_3dc8);
            }
        }
    }
    c.cycles += 4;
    c.sub_s(0, 0, 3); // maxX - minX
    c.sub_s(1, 1, 4); // maxY - minY
    c.swc1(0, out);
    c.swc1(1, p(out, 4));
    c.cycles += 2;
    abi::ret(c);
}

/// `UIVector_v21` (0x003a3e38): when the vector holds exactly four points,
/// fan them out from point 0 and add the x/y offset pair to points 1/3 and
/// 2; then `*(self + 0x60) = *off`. The `{lo:17, count:15}` field at `+0x74`
/// must read 4.
pub fn ui_vector_v21(c: &mut Cpu) {
    c.cycles += 7;
    let vec = abi::arg(c, 0);
    let off = abi::arg(c, 1);
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = sx(0xfffe_0000);
    c.g[V1][0] = c.lw(p(vec, UI_SIZE));
    c.g[A0][0] = 0x8_0000;
    c.g[V1][0] &= c.g[V0][0];
    let not_four = c.g[V1][0] != c.g[A0][0];
    c.g[T0][0] = 0x0200_0000;
    if !not_four {
        c.cycles += 3;
        c.g[A3][0] = 0xc;
        c.g[T1][0] = 0x0100_0000;
        loop {
            c.cycles += 14;
            c.g[V0][0] = c.lw(p(vec, UI_POINTS));
            c.g[V1][0] = c.g[T0][0];
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[T1][0] as u32));
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(r(c, V0)));
            c.g[V1][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
            c.ldl(10, p(r(c, V0), 7));
            c.ldr(10, r(c, V0));
            c.g[T3][0] = c.lw(p(r(c, V0), 8));
            c.sdl(10, p(r(c, A0), 7));
            c.sdr(10, r(c, A0));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc));
            let more = c.g[V1][0] != 0;
            c.sw(p(r(c, A0), 8), c.g[T3][0]);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003a_3e60);
                }
                continue;
            }
            c.cycles += 20;
            c.g[V0][0] = c.lw(p(vec, UI_POINTS));
            c.lwc1(1, off);
            c.lwc1(0, p(r(c, V0), 0xc));
            c.add_s(0, 0, 1);
            c.swc1(0, p(r(c, V0), 0xc));
            c.g[V1][0] = c.lw(p(vec, UI_POINTS));
            c.lwc1(1, p(off, 4));
            c.lwc1(0, p(r(c, V1), 0x28));
            c.add_s(0, 0, 1);
            c.swc1(0, p(r(c, V1), 0x28));
            c.g[V0][0] = c.lw(p(vec, UI_POINTS));
            c.lwc1(1, off);
            c.lwc1(0, p(r(c, V0), 0x18));
            c.add_s(0, 0, 1);
            c.swc1(0, p(r(c, V0), 0x18));
            c.g[V1][0] = c.lw(p(vec, UI_POINTS));
            c.lwc1(1, p(off, 4));
            c.lwc1(0, p(r(c, V1), 0x1c));
            c.add_s(0, 0, 1);
            c.swc1(0, p(r(c, V1), 0x1c));
            break;
        }
    }
    c.cycles += 8;
    c.ldl(2, p(off, 7));
    c.ldr(2, off);
    c.g[V1][0] = c.lw(p(off, 8));
    c.sdl(2, p(vec, UI_VEC3 + 7));
    c.sdr(2, p(vec, UI_VEC3));
    c.sw(p(vec, UI_VEC3 + 8), c.g[V1][0]);
    abi::ret(c);
}

/// `void* cUIGroup_v03(cUIGroup* self, void* fn)` (0x3a51d0): let the functor
/// claim the group; when it declines, offer it to the group's list.
pub fn ui_group_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = lw(c, c.g[S1][0] as u32);
        c.g[A0][0] = Addr(p_2(c.g[V1][0], 0x10)).lh(c);
        c.g[V0][0] = lw(c, p_2(c.g[V1][0], 0x14));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x003a_5200)?;
        let claimed = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S0][0];
        if claimed {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0039_7a68, 0x003a_5214)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// UI construction / teardown.

/// `void UIList_dtor(self, flags)` (0x3a4888): deleting destructor of a UI
/// list — delete the entries, reset the two embedded list nodes and free the
/// object when `flags & 1`.
pub fn ui_list_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.g[V0][0] = 0x49_0000;
        c.sq(18, sp_2(c).wrapping_add(0x10));
        c.g[V0][0] = sx(low(c, V0).wrapping_add(0x4c98));
        c.g[S2][0] = c.g[A0][0];
        c.sq(16, sp_2(c).wrapping_add(0x30));
        c.sq(17, sp_2(c).wrapping_add(0x20));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        sw_r(c, S2, 0x18, c.g[V0][0]);
        callee::UI_LIST_DELETE_ALL.call(c, 0x003a_48b4)?;
        c.g[S0][0] = 0x49_0000;
        c.g[A0][0] = sx(low(c, S2).wrapping_add(0xc));
        c.g[S0][0] = sx(low(c, S0).wrapping_add(0x4cc0));
        sw_r(c, S2, 0x14, c.g[S0][0]);
        callee::UI_LIST_NODE_DTOR_BODY_2.call(c, 0x003a_48c8)?;
        sw_r(c, S2, 0x8, c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_NODE_DTOR_BODY_2.call(c, 0x003a_48d4)?;
        c.g[S1][0] = c.g[S1][0] & 0x1;
        let keep = c.g[S1][0] == 0;
        c.lq(16, sp_2(c).wrapping_add(0x30));
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, callee::BUILTIN_DELETE.addr, 0x003a_48e8)?;
            c.lq(16, sp_2(c).wrapping_add(0x30));
        }
        c.cycles += 5;
        c.lq(17, sp_2(c).wrapping_add(0x20));
        c.lq(18, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI groups / menus.

/// `void cUIGroup_dtor(cUIGroup* self, int flags)` (0x3a5068): republish the
/// vtables, empty the child list, destroy the two embedded list nodes and run
/// the base object destructor.
pub fn c_ui_group_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x30);
        c.g[V1][0] = 0x0049_0000;
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x49e8));
        frame.save(c, S3, 0x10);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x4c98));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(p(r32(c, S1), 0x74));
        c.g[S3][0] = c.g[A1][0];
        Addr(p(r32(c, S1), 8)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S1), 0x8c)).sw(c, c.g[V1][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_DELETE_ALL.call(c, 0x003a_50ac)?;
        c.g[S0][0] = 0x0049_0000;
        c.g[A0][0] = sx(p(r32(c, S1), 0x80));
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x4cc0));
        Addr(p(r32(c, S1), 0x88)).sw(c, c.g[S0][0]);
        callee::UI_LIST_NODE_DTOR_BODY_3.call(c, 0x003a_50c0)?;
        Addr(p(r32(c, S1), 0x7c)).sw(c, c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_NODE_DTOR_BODY_3.call(c, 0x003a_50cc)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        callee::UI_OBJECT_DTOR_2.call(c, 0x003a_50d8)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIMenu_dtor(UIMenu* self, int flags)` (0x3a52a0): the same teardown
/// as [`c_ui_group_dtor`] with the menu vtables.
pub fn ui_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x30);
        c.g[V1][0] = 0x0049_0000;
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x49e8));
        frame.save(c, S3, 0x10);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x4c98));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(p(r32(c, S1), 0x74));
        c.g[S3][0] = c.g[A1][0];
        Addr(p(r32(c, S1), 8)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S1), 0x8c)).sw(c, c.g[V1][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_DELETE_ALL.call(c, 0x003a_52e4)?;
        c.g[S0][0] = 0x0049_0000;
        c.g[A0][0] = sx(p(r32(c, S1), 0x80));
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x4cc0));
        Addr(p(r32(c, S1), 0x88)).sw(c, c.g[S0][0]);
        callee::UI_LIST_NODE_DTOR_BODY_3.call(c, 0x003a_52f8)?;
        Addr(p(r32(c, S1), 0x7c)).sw(c, c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_NODE_DTOR_BODY_3.call(c, 0x003a_5304)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        callee::UI_OBJECT_DTOR_2.call(c, 0x003a_5310)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUITextScroll_dtor(cUITextScroll* self, int flags)` (0x3a62a0):
/// reset the vtable slots, drop the scroll lines and both list nodes, then
/// run the UI object destructor.
pub fn c_ui_text_scroll_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let flags = abi::arg64(c, 1);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x30);
        c.g[V0][0] = sx(p(r32(c, V0), 0x4278));
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = sx(p(self_, 0x8c));
        frame.save_ra(c, 0);
        c.g[S3][0] = flags;
        c.sw(at(c, S1, 8), c.g[V0][0]);
        call_ptr(c, callee::CUI_TEXT_SCROLL_CLEAR_LINES, 0x003a_62d4)?;
        c.g[V0][0] = 0x0049_0000;
        c.g[A0][0] = c.g[S2][0];
        c.g[V0][0] = sx(p(r32(c, V0), 0x4c98));
        c.sw(at(c, S1, 0xa4), c.g[V0][0]);
        call_ptr(c, callee::UI_LIST_DELETE_ALL_2, 0x003a_62e8)?;
        c.g[S0][0] = 0x0049_0000;
        c.g[A0][0] = sx(p(r32(c, S1), 0x98));
        c.g[S0][0] = sx(p(r32(c, S0), 0x4cc0));
        c.sw(at(c, S1, 0xa0), c.g[S0][0]);
        call_ptr(c, callee::UI_LIST_NODE_DTOR_BODY_4, 0x003a_62fc)?;
        c.sw(at(c, S1, 0x94), c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, callee::UI_LIST_NODE_DTOR_BODY_4, 0x003a_6308)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, callee::UI_OBJECT_DTOR_3, 0x003a_6314)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
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
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    pub(crate) use crate::game::ai::ai::tests::be_library;
    pub(crate) use crate::game::ai::ai::tests::virtual_setup;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4;
    pub(crate) use crate::game::bx::bxstring::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_6;
    pub(crate) use crate::game::fe::fememcard::tests::put_slot;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::h;
    pub(crate) use crate::game::main::gamemode::tests::slot;
    pub(crate) use crate::game::main::gamemode::tests::w as w_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::object::debouncenode::tests::vtable_2;
    pub(crate) use crate::game::object::railmodifier::tests::destructor_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_2;
    pub(crate) use crate::game::ui::uithing::tests::ui_anim_setup;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_3;
    use ssx3_ee as ee;

    #[test]
    fn ui_visitor_dtor_matches() {
        check(
            ee::s13::f_003a47a8__generated,
            ui_visitor_dtor,
            SEEDS,
            &|c, seed| destructor_setup(c, seed, 0x3a47_a8),
        );
    }

    #[test]
    fn ui_input_visitor_dtor_matches() {
        check(
            ee::s13::f_003a4cd0__generated,
            ui_input_visitor_dtor,
            SEEDS,
            &|c, seed| destructor_setup(c, seed, 0x3a4c_d0),
        );
    }

    #[test]
    fn ui_focus_visitor_dtor_matches() {
        check(
            ee::s13::f_003a4d40__generated,
            ui_focus_visitor_dtor,
            SEEDS,
            &|c, seed| destructor_setup(c, seed, 0x3a4d_40),
        );
    }

    #[test]
    fn c_ui_group_accept_matches() {
        check(
            ee::s13::f_003a51a0__generated,
            c_ui_group_accept,
            SEEDS,
            &|c, seed| {
                virtual_setup(c, seed, 0x3a51_a0, 0x10);
            },
        );
    }

    #[test]
    fn ui_focus_visitor_visit_matches() {
        check(
            ee::s13::f_003a4d70__generated,
            ui_focus_visitor_visit,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a4d_70, 0);
                let list = DATA + 0x400;
                let obj = DATA + 0x600;
                fill(c, &mut r, list, 0x20);
                fill(c, &mut r, obj, 0x20);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(obj);
                c.bus.write32(obj + 8, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 0x38);
            },
        );
    }

    #[test]
    fn ui_input_visitor_visit_matches() {
        check(
            ee::s13::f_003a4d00__generated,
            ui_input_visitor_visit,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x3a4d00, 0);
                let obj = DATA + 0x200;
                let vt = DATA + 0x600;
                // A real, harmless method: the script engine singleton (0 when the
                // global is clear), so both the accept and reject results occur.
                c.bus.write16(vt + 0x78, 0);
                c.bus.write32(vt + 0x7c, 0x0027_d2e8);
                c.g[A0][0] = (DATA + 0x300) as u64;
                c.g[A1][0] = obj as u64;
                c.bus.write32(DATA + 0x304, (seed % 4) as u32);
                c.bus.write32(obj + 8, vt);
                c.bus.write32(
                    GAME_GP + 0x3ec,
                    if seed % 2 == 0 { 0 } else { DATA + 0x500 },
                );
            },
        );
    }

    #[test]
    fn ui_vector_update_anim_matches() {
        check(
            ee::s13::f_003a3d28__generated,
            ui_vector_update_anim,
            40,
            &|c, seed| {
                ui_anim_setup(c, seed, 0x3a_3d28);
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x3a54b8 UIPair_dtor / 0x3a5658 UIListBox_dtor / 0x3a5da8 UIText_dtor.

    pub(crate) fn dtor_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let this = DATA + 0x100;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = ((seed as u64) & 0xfe) << 1; // flags bit 0 clear: no delete
                                                  // A zeroed UI object with empty child lists.
        for off in [0x78u32, 0x7c, 0xc, 0xac, 0xb4] {
            c.bus.write32(this + off, 0);
        }
        c.bus.write32(this + 0x9c, 0);
        c.bus.write8(this + 0x318, 0);
        let owner = DATA + 0x800;
        c.bus.write32(this + 0x10, owner);
        c.bus.write32(owner + 0x28, 0);
        let _ = r.next_u32();
    }

    #[test]
    fn ui_pair_dtor_matches() {
        check(
            ee::s13::f_003a54b8__generated,
            ui_pair_dtor,
            8,
            &|c, seed| dtor_setup(c, seed, 0x3a54_b8),
        );
    }

    #[test]
    fn ui_list_box_dtor_matches() {
        check(
            ee::s13::f_003a5658__generated,
            ui_list_box_dtor,
            8,
            &|c, seed| dtor_setup(c, seed, 0x3a56_58),
        );
    }

    #[test]
    fn ui_text_dtor_matches() {
        check(
            ee::s13::f_003a5da8__generated,
            ui_text_dtor,
            8,
            &|c, seed| dtor_setup(c, seed, 0x3a5d_a8),
        );
    }

    #[test]
    fn ui_list_node_dtor_matches() {
        check(
            ee::s13::f_003a47f8__generated,
            ui_list_node_dtor,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x3a47_f8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn ui_thread_dtor_matches() {
        check(
            ee::s13::f_003a5b48__generated,
            ui_thread_dtor,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x3a5b_48, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    /// A fixture with `gp` set to the game's value and scrambled registers.
    pub(crate) fn start(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x2545_f491_4f6c_dd1d ^ (seed as u64) << 4 ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    // -------------------------------------------------------------- UI flags

    /// One shared test body for the `bitN = enable` setters.
    pub(crate) fn check_flag_setter(hand: GuestFn, generated: GuestFn, off: u32) {
        check(generated, hand, 200, &|c, seed| {
            let mut r = start(c, seed, off as u64);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed % 3 == 0 {
                0
            } else {
                r.next_u32() as u64 | (r.next_u32() as u64) << 32
            };
            c.bus.write32(DATA + off, r.next_u32());
        });
    }

    #[test]
    fn ui_object_v04_matches() {
        check_flag_setter(
            ui_object_v04,
            ssx3_ee::s13::f_003a49e8__generated,
            UI_FLAGS_OBJECT,
        );
    }

    #[test]
    fn ui_object_v05_matches() {
        check_flag_setter(
            ui_object_v05,
            ssx3_ee::s13::f_003a4a08__generated,
            UI_FLAGS_OBJECT,
        );
    }

    #[test]
    fn ui_object_v06_matches() {
        check_flag_setter(
            ui_object_v06,
            ssx3_ee::s13::f_003a4a28__generated,
            UI_FLAGS_OBJECT,
        );
    }

    #[test]
    fn ui_object_set_focus_matches() {
        check_flag_setter(
            ui_object_set_focus,
            ssx3_ee::s13::f_003a4a48__generated,
            UI_FLAGS_OBJECT,
        );
    }

    #[test]
    fn ui_object_v08_matches() {
        check_flag_setter(
            ui_object_v08,
            ssx3_ee::s13::f_003a4a68__generated,
            UI_FLAGS_OBJECT,
        );
    }

    #[test]
    fn ui_object_set_visible_matches() {
        check_flag_setter(
            ui_object_set_visible,
            ssx3_ee::s13::f_003a4a88__generated,
            UI_FLAGS_OBJECT,
        );
    }

    #[test]
    fn ui_list_box_v24_matches() {
        check_flag_setter(
            ui_list_box_v24,
            ssx3_ee::s13::f_003a5708__generated,
            UI_FLAGS_WIDGET,
        );
    }

    #[test]
    fn ui_text_v23_matches() {
        check_flag_setter(
            ui_text_v23,
            ssx3_ee::s13::f_003a5e68__generated,
            UI_FLAGS_WIDGET,
        );
    }

    #[test]
    fn ui_text_v24_matches() {
        check_flag_setter(
            ui_text_v24,
            ssx3_ee::s13::f_003a5e88__generated,
            UI_FLAGS_WIDGET,
        );
    }

    #[test]
    fn ui_text_v25_matches() {
        check_flag_setter(
            ui_text_v25,
            ssx3_ee::s13::f_003a5ea8__generated,
            UI_FLAGS_WIDGET,
        );
    }

    #[test]
    fn ui_text_v26_matches() {
        check_flag_setter(
            ui_text_v26,
            ssx3_ee::s13::f_003a5ec8__generated,
            UI_FLAGS_WIDGET,
        );
    }

    #[test]
    fn ui_text_v27_matches() {
        check_flag_setter(
            ui_text_v27,
            ssx3_ee::s13::f_003a5ee8__generated,
            UI_FLAGS_WIDGET,
        );
    }

    #[test]
    fn c_ui_text_scroll_v23_matches() {
        check_flag_setter(
            c_ui_text_scroll_v23,
            ssx3_ee::s13::f_003a6378__generated,
            UI_FLAGS_WIDGET,
        );
    }

    #[test]
    fn c_ui_text_scroll_v24_matches() {
        check_flag_setter(
            c_ui_text_scroll_v24,
            ssx3_ee::s13::f_003a6398__generated,
            UI_FLAGS_WIDGET,
        );
    }

    // ------------------------------------------------------- UI vec3 copies

    #[test]
    fn ui_object_set_color_matches() {
        check(
            ssx3_ee::s13::f_003a4af8__generated,
            ui_object_set_color,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x1c);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                for i in 0..4 {
                    c.bus.write32(DATA + 0x100 + 4 * i, r.next_u32());
                }
                // The unaligned pairs also read-modify-write the neighbouring
                // words of the 16-byte destination at DATA + 0x41c.
                for i in 0..6 {
                    c.bus.write32(DATA + 0x418 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ui_object_get_vec3_at60_matches() {
        check(
            ssx3_ee::s13::f_003a4c90__generated,
            ui_object_get_vec3_at60,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x60);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                for i in 0..3 {
                    c.bus.write32(DATA + 0x400 + 0x60 + 4 * i, r.next_u32());
                }
                // The 12-byte destination at DATA + 0x100, including the
                // neighbouring words its unaligned pairs read-modify-write.
                for i in 0..6 {
                    c.bus.write32(DATA + 0x0f8 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ui_object_set_vec3_at60_matches() {
        check(
            ssx3_ee::s13::f_003a4cb0__generated,
            ui_object_set_vec3_at60,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x60);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                for i in 0..3 {
                    c.bus.write32(DATA + 0x100 + 4 * i, r.next_u32());
                }
                for i in 0..6 {
                    c.bus.write32(DATA + 0x458 + 4 * i, r.next_u32());
                }
            },
        );
    }

    // --------------------------------------------------------- UI vectors

    #[test]
    fn ui_vector_v20_matches() {
        check(
            ssx3_ee::s13::f_003a3d70__generated,
            ui_vector_v20,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x3d70);
                let vec = DATA + 0x400;
                c.g[A0][0] = vec as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                let n = match seed % 4 {
                    0 => 0, // no data pointer at all
                    1 => 0, // count 0: only the default extent
                    _ => 1 + seed % 3,
                };
                c.bus
                    .write32(vec + 0x80, if seed % 4 == 0 { 0 } else { DATA + 0x800 });
                c.bus
                    .write32(vec + 0x74, (n << 17) | (r.next_u32() & 0x1_ffff));
                for i in 0..n {
                    c.bus.write32(DATA + 0x800 + 0xc * i, r.edge_f32());
                    c.bus.write32(DATA + 0x804 + 0xc * i, r.edge_f32());
                }
                // The default extent constant.
                for i in 0..3 {
                    c.bus.write32(0x004f_f0d8 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ui_vector_v21_matches() {
        check(
            ssx3_ee::s13::f_003a3e38__generated,
            ui_vector_v21,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x3e38);
                let vec = DATA + 0x400;
                c.g[A0][0] = vec as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.bus.write32(
                    vec + 0x74,
                    if seed % 2 == 0 {
                        0x8_0000
                    } else {
                        r.next_u32() & 0xfffe_0000
                    },
                );
                c.bus.write32(vec + 0x80, DATA + 0x800);
                for i in 0..4 * 3 {
                    c.bus.write32(DATA + 0x800 + 4 * i, r.edge_f32());
                }
                for i in 0..3 {
                    c.bus.write32(DATA + 0x300 + 4 * i, r.edge_f32());
                }
                // The 12-byte copy into the vec3 at +0x60.
                for i in 0..4 {
                    c.bus.write32(DATA + 0x45c + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ui_group_v03_matches() {
        check(
            ee::s13::f_003a51d0__generated,
            ui_group_v03,
            SEEDS_4,
            &|c, seed| {
                let mut r = base(c, seed, 0x3a51_d0);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                let vt = DATA + 0x580;
                vtable_2(c, vt, &[(0x10, FAKE + 0x40)]);
                let holder = DATA + 0x300;
                clear(c, holder, 8);
                w(c, holder, vt as u32);
                let list = s + 0x74;
                let sent = DATA + 0x500;
                w(c, sent, sent as u32);
                w(c, sent + 4, sent as u32);
                w(c, list + 4, sent as u32);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(holder);
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ui_list_dtor_matches() {
        check(
            ee::s13::f_003a4888__generated,
            ui_list_dtor,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x3a4888, 4);
                be_library_2(c);
                let list = DATA;
                c.g[A0][0] = list as u64;
                c.g[A1][0] = (seed % 2) as u64;
                let s = DATA + 0x2000;
                if seed % 3 == 0 {
                    w_2(c, list + 4, s);
                    w_2(c, s + 4, s);
                    w_2(c, s, s);
                } else {
                    // head -> a -> terminator (self-linked at +4)
                    let a = DATA + 0x2100;
                    w_2(c, list + 4, a);
                    w_2(c, a + 4, s);
                    w_2(c, a, 0);
                    w_2(c, s + 4, s);
                    w_2(c, s, 0);
                    let vt = DATA + 0x3000;
                    w_2(c, a + 8, vt);
                    slot(c, vt, 8, 0x20, FAKE);
                }
            },
        );
    }

    #[test]
    fn c_ui_group_dtor_matches() {
        check(
            ee::s13::f_003a5068__generated,
            c_ui_group_dtor,
            150,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x3a_5068);
                globals(c);
                pool(c);
                // Empty child list and zeroed embedded list nodes.
                w_3(c, DATA + 0x74, 0);
                w_3(c, DATA + 0x7c, 0);
                w_3(c, DATA + 0x80, 0);
                w_3(c, DATA + 0x88, 0);
                w_3(c, DATA + 0x8c, 0);
                c.g[A0][0] = sx(DATA);
                // Even flags: the base_2 destructor must not free test data.
                c.g[A1][0] = [0u64, 2][seed as usize % 2];
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn ui_menu_dtor_matches() {
        check(
            ee::s13::f_003a52a0__generated,
            ui_menu_dtor,
            150,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x3a_52a0);
                globals(c);
                pool(c);
                w_3(c, DATA + 0x74, 0);
                w_3(c, DATA + 0x7c, 0);
                w_3(c, DATA + 0x80, 0);
                w_3(c, DATA + 0x88, 0);
                w_3(c, DATA + 0x8c, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = [0u64, 2][seed as usize % 2];
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn c_ui_text_scroll_dtor_matches() {
        check(
            ee::s13::f_003a62a0__generated,
            c_ui_text_scroll_dtor,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x3a62, 0);
                env(c);
                let this = DATA + 0x400;
                let sentinel = DATA + 0x1000;
                w32(c, sentinel + 4, sentinel);
                w32(c, this + 0x8c, sentinel);
                w32(c, this + 0x8c + 4, sentinel);
                w32(c, this + 0x98, sentinel);
                w32(c, this + 0x98 + 4, sentinel);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }
}
