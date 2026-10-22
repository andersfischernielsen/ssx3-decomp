//! `src/object/movenode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::addu;
use crate::abi::r32;
use crate::ps2f::vu0_scratch;
use crate::ps2f::Vec4;
use ssx3_ee as ee;
use crate::call::sp;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Flow;
use crate::events::back_edge_unwinds;
use callee::*;
use crate::call::Unwound;
use crate::call;
use crate::events::deliver_event;
use ssx3_ee_rt::funcs::call_indirect;
pub(crate) use crate::game::sound::asyncsys::lw;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::object::debouncenode::lw32;
pub(crate) use crate::game::fe::festateload::at as at_2;
pub(crate) use crate::game::object::deadfadenode::ctor_with_vtable;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
pub(crate) use crate::game::render::particle::p as p_2;
pub(crate) use crate::game::bx::bxstring::gp;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::fe::debugfe::at as at_3;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_3;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::main::gamemode::low as low_2;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::ai::ridermetrix::swc1_r;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::movie::movieplayer::add32;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_4;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at as lw_at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::festaterewards::p as p_7;
pub(crate) use crate::game::fe::festaterewards::gp_4 as gp_2;
pub(crate) use crate::game::fe::festaterewards::r;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_4;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_2;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::ai::computer::sd;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_5420, mod_obj_v14, "ModObj_v14"),
    (0x0035_55a8, mod_obj_set_tex_flip_modifier, "ModObj_SetTexFlipModifier"),
    (0x0035_5600, mod_obj_set_boost_modifier, "ModObj_SetBoostModifier"),
    (0x0035_56a8, mod_obj_set_rail_modifier, "ModObj_SetRailModifier"),
    (0x0035_5770, mod_obj_v40, "ModObj_v40"),
    (0x0035_5858, mod_obj_v41, "ModObj_v41"),
    (0x0035_58b8, mod_obj_add_position_modifier, "ModObj_AddPositionModifier"),
    (0x0035_5978, mod_obj_add_parent_modifier, "ModObj_AddParentModifier"),
    (0x0035_59f8, mod_obj_add_magnet_modifier, "ModObj_AddMagnetModifier"),
    (0x0035_5a78, mod_obj_add_ava_spline_modifier, "ModObj_AddAvaSplineModifier"),
    (0x0035_5bf0, mod_obj_add_tex_flip_modifier, "ModObj_AddTexFlipModifier"),
    (0x0035_5c50, mod_obj_add_boost_modifier, "ModObj_AddBoostModifier"),
    (0x0035_5db8, mod_obj_add_roller_modifier, "ModObj_AddRollerModifier"),
    (0x0035_6078, mod_obj_v24, "ModObj_v24"),
    (0x0035_60c0, mod_obj_v28, "ModObj_v28"),
    (0x0035_6298, mod_obj_render, "ModObj_Render"),
    (0x0035_68b0, mod_obj_v50, "ModObj_v50"),
    (0x0035_69d0, mod_obj_v44, "ModObj_v44"),
    (0x0035_6a00, mod_obj_v45, "ModObj_v45"),
    (0x0035_6a70, mod_obj_v39, "ModObj_v39"),
    (0x0035_6ab8, mod_obj_v43, "ModObj_v43"),
    (0x0035_6ae0, mod_obj_v42, "ModObj_v42"),
    (0x0035_6b08, mod_obj_v36, "ModObj_v36"),
    (0x0035_6bf0, mod_obj_v20, "ModObj_v20"),
    (0x0035_6c48, mod_obj_v21, "ModObj_v21"),
    (0x0035_6d48, mod_obj_v23, "ModObj_v23"),
    (0x0035_6e60, object_ctor_2, "Object_ctor_2"),
    (0x0035_6eb8, position_modifier_ctor, "PositionModifier_ctor"),
    (0x0035_6ff0, position_modifier_update_transform, "PositionModifier_UpdateTransform"),
    (0x0035_7038, parent_modifier_ctor, "ParentModifier_ctor"),
    (0x0035_7090, parent_modifier_ctor_2, "ParentModifier_ctor_2"),
    (0x0035_7210, parent_modifier_v22, "ParentModifier_v22"),
    (0x0035_7278, parent_modifier_save, "ParentModifier_Save"),
    (0x0035_72e0, magnet_modifier_ctor, "MagnetModifier_ctor"),
    (0x0035_7358, magnet_modifier_ctor_2, "MagnetModifier_ctor_2"),
    (0x0035_73f8, magnet_modifier_update, "MagnetModifier_Update"),
    (0x0035_7528, magnet_modifier_update_transform, "MagnetModifier_UpdateTransform"),
    (0x0035_7660, magnet_modifier_v09, "MagnetModifier_v09"),
    (0x0035_76d0, magnet_modifier_save, "MagnetModifier_Save"),
    (0x0035_7750, ava_spline_modifier_ctor, "AvaSplineModifier_ctor"),
    (0x0035_7820, ava_spline_modifier_update, "AvaSplineModifier_Update"),
    (0x0035_7848, ava_spline_modifier_save, "AvaSplineModifier_Save"),
    (0x0035_7918, particle_ctor, "Particle_ctor"),
    (0x0035_7950, particle_update, "Particle_Update"),
    (0x0035_79c8, multi_particle_man_ctor, "MultiParticleMan_ctor"),
    (0x0035_7a00, multi_particle_man_dtor, "MultiParticleMan_dtor"),
    (0x0035_7a78, multi_particle_man_create_slot, "MultiParticleMan_CreateSlot"),
    (0x0035_7b38, multi_particle_man_clear, "MultiParticleMan_Clear"),
    (0x0035_7b90, multi_particle_man_on_section_unload, "MultiParticleMan_OnSectionUnload"),
    (0x0035_7d28, multiparticle_man_load, "MultiParticleMan_Load"),
    (0x0035_7dd8, multi_particle_ctor, "MultiParticle_ctor"),
    (0x0035_7e80, multi_particle_ctor_2, "MultiParticle_ctor_2"),
    (0x0035_7fa0, multi_particle_dtor, "MultiParticle_dtor"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    use super::*;
    pub(crate) use crate::game::object::debouncenode::callee::HAS_OBJECT_SCRIPT1;
    pub(crate) use crate::game::object::debouncenode::callee::HAS_OBJECT_SCRIPT3;
    pub(crate) use crate::game::object::debouncenode::callee::GET_OBJECT_SLOT;
    pub(crate) use crate::game::object::debouncenode::callee::GET_PLAYER_INSTANCE;
    pub(crate) use crate::game::object::debouncenode::callee::GET_HUMAN_RIDER;
    pub(crate) use crate::game::object::debouncenode::callee::GET_CHARACTER;
    pub(crate) use crate::game::object::debouncenode::callee::GET_TRACK_PEAK;
    pub(crate) use crate::game::object::debouncenode::callee::GET_TRANSFORM_BOUNDS as GET_TRANSFORM_BOUNDS_2;
    pub(crate) use crate::game::object::debouncenode::callee::EVAL_SEGMENT;
    pub(crate) use crate::game::intersect::riderspheretree::callee::DELETE;
    pub(crate) use crate::game::intersect::riderspheretree::callee::DMA_FROM_SCRATCHPAD;
    pub(crate) use crate::game::intersect::riderspheretree::callee::MOD_OBJ_UPDATE;
    pub(crate) use crate::game::ai::rider::callee::EMITTER_UPDATE_DESCRIPTOR;
    pub(crate) use crate::game::ai::rider::callee::EMITTER_ADVANCE_DESC_TIME;
    pub(crate) use crate::game::ai::rider::callee::PAD_QUEUE_ADVANCE_WRITE;
    pub(crate) use crate::game::ai::rider::callee::PAD_QUEUE_GET_PAD_STATUS;
    pub(crate) use crate::game::ai::rider::callee::PAD_QUEUE_GET_PAD_DATA;
    pub(crate) use crate::game::ai::rider::callee::INPUT_PAD_UPDATE;
    pub(crate) use crate::game::ai::rider::callee::COMM_SYSTEM_UPDATE;
    pub(crate) use crate::game::ai::rider::callee::USB_KEYBOARD_UPDATE;
    pub(crate) use crate::game::ai::rider::callee::MATH_SIN_COS_2 as MATH_SIN_COS;
    pub(crate) use crate::game::ai::rider::callee::EMITTER_RENDER;
    pub(crate) use crate::game::ai::rider::callee::VU0_SAVE_CONTEXT;
    pub(crate) use crate::game::ai::rider::callee::VU0_RESTORE_CONTEXT;
    pub(crate) use crate::game::ai::rider::callee::MODIFIER_BLOCK_GET_TRANSFORM_BOUNDS;
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
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_INSTANCE;
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
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::PS2_SOUND_MANAGER_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_SET_FILE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_STOP;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SPEECH_INSTANCE_STOP_STREAM;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_GET_STREAM_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_POST_EVENT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_FADE_OUT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SNDSTRM_GET_REQUEST_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_SET_STATE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::CONSOLE_CONFIG_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STRCPY;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MENU_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STATS_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::AI_VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_GET_SECTION_SCRIPT_FUNC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MOD_OBJ_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::INTC_REMOVE_HANDLER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EMITTER_RENDER as EMITTER_RENDER_2;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_ON_VIF1_DMA_END;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_MAN_CTOR;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2 as DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2 as RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_NEW_PLAYER_INT_GET_SLOT;
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
    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;
    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_STREAM_HANDLE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT;
    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::render::particle::callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT;
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::render::particle::callee::AVA_SPLINE_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2 as UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS as RIDER_METRIX_AWARD_POINTS_2;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST as RIDER_ADD_BOOST_2;
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
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_BINDING_HAS_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MENU_SYS_ITEM_CTOR as MENU_SYS_ITEM_CTOR_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MEMCARD_SJIS_CHAR_TO_ASCII;
    pub(crate) use crate::game::intersect::worldsphtree::callee::PS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_CONTAINS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_REMOVE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::POSITION_MODIFIER_CTOR as POSITION_MODIFIER_CTOR_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER as MOD_OBJ_SET_TRANSFORM_MODIFIER_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_RESTART_PEAK_MUSIC;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_STOP_PROCESS_SEQ;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_KILL_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR_2_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SONG_INSTANCE_GET_BEAT_MS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SONG_INSTANCE_GET_POSITION_MS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP3;
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
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE as CPS2_MEM_CARD_SET_STATE_2;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_3;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR as MULTI_PARTICLE_DTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT as BUDDY_MAN_DISCONNECT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_CTOR_5 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2 as STRCPY_3;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO as SCRIPT_DATA_GET_INFO_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_FIND_SOUND_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_ITEM_ON_FOCUS as MENU_SYS_ITEM_ON_FOCUS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_SET_SELECTION;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS as RIDER_METRIX_RESET_STATS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_ON_OPERATION_DONE;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_END_OPERATION_4 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_SET_MODE_2 as MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_RESET_LAYOUT;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_CONFIG;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_3;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::ROLLER_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER as MOD_OBJ_SET_TRANSFORM_MODIFIER_3;
    pub(crate) use crate::game::movie::movieplayer::callee::MULTI_SPLINE_MODIFIER_ADD_REF;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_3;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS as SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_CTOR_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_DTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::FONT_PARSE_GLYPHS;
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2;
    pub(crate) use crate::game::movie::movieplayer::callee::FILE_LOAD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES as FLEX_BRIDGE_ALLOC_SPRING_NODES_2;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_BE as CBE_GET_BE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_INTERFACE;
    pub(crate) use crate::game::movie::movieplayer::callee::C_OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::EVENT_SELECT_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FLAG_MANAGER_GROUP_INIT_KEY;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RESET_FOR_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2 as RIDER_ADD_BOOST_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_APPLY_SCRIPT_TRACK;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::QSORT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_FROM_ASCII_2 as USTR_FROM_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_TO_ASCII as USTR_TO_ASCII_2;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WIDGET_ALIGN;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WRAPPED_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::RIDER_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_MISSION_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED_3;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_2;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_MAN_CLEAR;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_SET_EVENT_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::OPTIONS_SET_VIBRATION;
    pub(crate) use crate::game::bx::bxstring2::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_FIND_FIRST_SELECTABLE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::STRLEN as STRLEN_2;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_DEQUEUE_SECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_POP_FREE_NODE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_ADD_ITEM_STR_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_FIND_ITEM_BY_VALUE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_SHOW_POPUP;
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
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_3;
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
    pub(crate) use crate::game::bx::bxstring2::callee::MEMSET as MEMSET_2;
    pub(crate) use crate::game::bx::bxstring2::callee::QSORT as QSORT_2;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN3D_GROUP_BUILD_MATRICES;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAM_CONTROLLER_RESTORE_DEFAULT;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_SET_SKIP_DELAY;
    pub(crate) use crate::game::bx::bxstring2::callee::CUTSCENE_TRANSITION_MAN_FORCE_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::bx::bxstring2::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_4;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_FREE as EA_MEM_FREE_3;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_LEN as USTR_LEN_2;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_CPY as USTR_CPY_2;
    pub(crate) use crate::game::bx::bxstring2::callee::UITEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_DJ_SPEECH_ON_2 as IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_ARCADE_SPEECH_ON;
    pub(crate) use crate::game::fe::festaterewards::callee::GET_LANGUAGE_MASK_2 as GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_STOP_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_POP_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_PUSH_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_START_SCRIPT;
    pub(crate) use crate::game::fe::festaterewards::callee::MOD_OBJ_RUN_SCRIPT_EVENT;
    pub(crate) use crate::game::fe::festaterewards::callee::REWARD_GALLERY_REGISTER_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SERVER_TIMEOUT;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_OPPONENT_DISCONNECTED;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SHOW_CANCEL;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_SET_BIT;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_GET_FILE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_SPEND_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_GET_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_COPY_TAG;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_SET_RAW as TAG_FIELD_SET_RAW_2;
    pub(crate) use crate::game::fe::festaterewards::callee::BRAGGING_RIGHTS_UPDATE_ROW_COLORS;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_IS_SHARING_CHAR_RECORD;
    pub(crate) use crate::game::fe::festaterewards::callee::BECHAR_GET_OVERALL_RATING;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_IS_FLAG_SET;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_SET_FLAG;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_POST;
    pub(crate) use crate::game::fe::festaterewards::callee::TEXFLIP_SET_FRAME;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_GET_RIDER_PROFILE_2 as BE_GET_RIDER_PROFILE_4;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::fe::festaterewards::callee::VFX_PLANTS_GET_SWAY_MATRIX;
    pub(crate) use crate::game::fe::fepopup::callee::C00113f88;
    pub(crate) use crate::game::fe::fepopup::callee::C00115ab0;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe78;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe98;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fec8;
    pub(crate) use crate::game::fe::fepopup::callee::C00125108;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a080;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a0e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0014bdb8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d448;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d988;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d998;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dc80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dd58;
    pub(crate) use crate::game::fe::fepopup::callee::C00150928;
    pub(crate) use crate::game::fe::fepopup::callee::C00150b48;
    pub(crate) use crate::game::fe::fepopup::callee::C00151c48;
    pub(crate) use crate::game::fe::fepopup::callee::C001535c0;
    pub(crate) use crate::game::fe::fepopup::callee::C001543e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00156988;
    pub(crate) use crate::game::fe::fepopup::callee::C00156a60;
    pub(crate) use crate::game::fe::fepopup::callee::C001577e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00157bf0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158910;
    pub(crate) use crate::game::fe::fepopup::callee::C001589b0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158a50;
    pub(crate) use crate::game::fe::fepopup::callee::C00158af0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158c80;
    pub(crate) use crate::game::fe::fepopup::callee::C00158d58;
    pub(crate) use crate::game::fe::fepopup::callee::C00159170;
    pub(crate) use crate::game::fe::fepopup::callee::C00159cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C0015a2e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c6b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c718;
    pub(crate) use crate::game::fe::fepopup::callee::C00176890;
    pub(crate) use crate::game::fe::fepopup::callee::C0017f500;
    pub(crate) use crate::game::fe::fepopup::callee::C0019e2b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0019f878;
    pub(crate) use crate::game::fe::fepopup::callee::C001bb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001c84a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001c8568;
    pub(crate) use crate::game::fe::fepopup::callee::C001c9b68;
    pub(crate) use crate::game::fe::fepopup::callee::C001cb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbc48;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbce8;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbfc0;
    pub(crate) use crate::game::fe::fepopup::callee::C001ce560;
    pub(crate) use crate::game::fe::fepopup::callee::C001d32e0;
    pub(crate) use crate::game::fe::fepopup::callee::C001e1a30;
    pub(crate) use crate::game::fe::fepopup::callee::C001e3388;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91f8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e9290;
    pub(crate) use crate::game::fe::fepopup::callee::C001f10f8;
    pub(crate) use crate::game::fe::fepopup::callee::C0020e900;
    pub(crate) use crate::game::fe::fepopup::callee::C00224c78;
    pub(crate) use crate::game::fe::fepopup::callee::C00224da0;
    pub(crate) use crate::game::fe::fepopup::callee::C00224df0;
    pub(crate) use crate::game::fe::fepopup::callee::C00225068;
    pub(crate) use crate::game::fe::fepopup::callee::C002259a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00227f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00231f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00238bf8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026d4d8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026db88;
    pub(crate) use crate::game::fe::fepopup::callee::C0026e448;
    pub(crate) use crate::game::fe::fepopup::callee::C0026f4a0;
    pub(crate) use crate::game::fe::fepopup::callee::C002702f8;
    pub(crate) use crate::game::fe::fepopup::callee::C002703f0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275498;
    pub(crate) use crate::game::fe::fepopup::callee::C002758c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275b98;
    pub(crate) use crate::game::fe::fepopup::callee::C00275cf8;
    pub(crate) use crate::game::fe::fepopup::callee::C00278308;
    pub(crate) use crate::game::fe::fepopup::callee::C00278e50;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f38;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f68;
    pub(crate) use crate::game::fe::fepopup::callee::C00281ed8;
    pub(crate) use crate::game::fe::fepopup::callee::C00283228;
    pub(crate) use crate::game::fe::fepopup::callee::C00283518;
    pub(crate) use crate::game::fe::fepopup::callee::C002837c8;
    pub(crate) use crate::game::fe::fepopup::callee::C0028b180;
    pub(crate) use crate::game::fe::fepopup::callee::C0028bb10;
    pub(crate) use crate::game::fe::fepopup::callee::C0028e8c0;
    pub(crate) use crate::game::fe::fepopup::callee::C002913d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003457c8;
    pub(crate) use crate::game::fe::fepopup::callee::C00358020;
    pub(crate) use crate::game::fe::fepopup::callee::C00362478;
    pub(crate) use crate::game::fe::fepopup::callee::C003625c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00362660;
    pub(crate) use crate::game::fe::fepopup::callee::C003626d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003627a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00364cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C00366ce0;
    pub(crate) use crate::game::fe::fepopup::callee::C00368170;
    pub(crate) use crate::game::fe::fepopup::callee::C003684f0;
    pub(crate) use crate::game::fe::fepopup::callee::C00368970;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f460;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f668;
    pub(crate) use crate::game::fe::fepopup::callee::C0039c870;
    pub(crate) use crate::game::fe::fepopup::callee::C0039ff78;
    pub(crate) use crate::game::fe::fepopup::callee::C003b1300;
    pub(crate) use crate::game::fe::fepopup::callee::C003b2360;

    pub const GET_TRANSFORM_BOUNDS: Callee =
        Callee { addr: 0x0035_2b88, f: ee::s11::ModifierBlock_GetTransformBounds };

    pub const SET_TRANSFORM_BOUNDS: Callee =
        Callee { addr: 0x0035_2bc0, f: ee::s11::ModifierBlock_SetTransformBounds };

    pub const GET_TRANSFORM_RADIUS: Callee =
        Callee { addr: 0x0035_2bf8, f: ee::s11::ModifierBlock_GetTransformRadius };

    pub const MODIFIER_BLOCK_RENDER: Callee =
        Callee { addr: 0x0035_2dd0, f: ee::s11::ModifierBlock_Render };
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
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STR_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_SCREEN_START_THREAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CLOSE_UNPINNED_POPUPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCASECMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
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
}


/// `World_GetAvalancheMatrix` (0x2d1cf0), a direct-call target of
/// `AvaSplineModifier_Update`.
pub(crate) const WORLD_GET_AVALANCHE_MATRIX: Callee =
    Callee { addr: 0x002d_1cf0, f: ssx3_ee::s09::f_002d1cf0 };


// ---------------------------------------------------------------------------
// AvaSplineModifier.

/// `AvaSplineModifier *AvaSplineModifier_ctor(AvaSplineModifier *this,
/// void *arg)` (0x357750): set the vtable, store `arg` at +0x80, clear +0x30,
/// and copy the 0x40-byte block at `arg + 0x10` into `this + 0x40` with four
/// `lqc2`/`sqc2` pairs.
pub fn ava_spline_modifier_ctor(c: &mut Cpu) {
    c.cycles += 18;
    let this = abi::arg(c, 0);
    let arg = abi::arg(c, 1);
    c.g[V1][0] = 0x0049_0000;
    c.g[V0][0] = this as u64;
    let vtable = 0x0049_0000u32.wrapping_sub(0xcc8);
    c.g[V1][0] = sx(vtable);
    let src = arg.wrapping_add(0x10);
    c.g[A0][0] = sx(src);
    c.sw(this, vtable as u64);
    c.sw(this.wrapping_add(0x80), arg as u64);
    let dst = this.wrapping_add(0x40);
    c.g[V1][0] = sx(dst);
    c.sw(this.wrapping_add(0x30), 0);
    for i in 0..4 {
        c.lqc2(1 + i, src.wrapping_add(0x10 * i as u32));
    }
    for i in 0..4 {
        c.sqc2(1 + i, dst.wrapping_add(0x10 * i as u32));
    }
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// AvaSplineModifier.

/// `void AvaSplineModifier_Update(AvaSplineModifier *this)` (0x357820):
/// `World_GetAvalancheMatrix(this->matrix(+0x80), &this->+0x40)`.
pub fn ava_spline_modifier_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[A1][0] = this as u64;
        c.g[A0][0] = Addr(this).off(0x80).lw(c);
        c.g[A1][0] = sx(this.wrapping_add(0x40));
        WORLD_GET_AVALANCHE_MATRIX.call(c, 0x0035_7838)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// Calls the original makes through the function table (other shards).
pub(crate) const WORLD_GET_NASH: u32 = 0x002d_1be0;

pub(crate) const WORLD_IS_OCCLUDED: u32 = 0x002d_2058;

pub(crate) const SPATIAL_HASH_UPDATE: u32 = 0x0032_91e0;

pub(crate) const UV_SCROLL_GET_MATRIX: u32 = 0x0035_fc20;


/// `*(gp + GFX_MAN)`: the graphics manager; its camera vtable is at +0x10d8.
pub(crate) const GFX_MAN: u32 = 0x2a90;

/// `**(gp + LAYER_TABLE)`: the render layer switches (8 bytes per layer,
/// mode at +0x24; 0x18 bytes per pass from +0x3f0, mode at +0).
pub(crate) const LAYER_TABLE: u32 = 0x16c8;

/// The identity matrix `ModObj_Render` passes when an object has its own.
pub(crate) const IDENTITY_MATRIX: u32 = 0x004f_f1a0;

/// 1.0f.
pub(crate) const ONE: u32 = 0x3f80_0000;


// ---------------------------------------------------------------------------
// Modifier objects.

/// `void ModObj_v50(ModObj *this)` (0x3568b0, vtable slot 0x50): when the
/// object has a modifier block with an implementation, rebuild its box
/// around its position (method 0xc0, +0x30) with the block's radius, store
/// it as the block's bounds and move the object (`this+0x18`) in the
/// world's spatial hash from the old bounds to the new; then notify the
/// implementation (its method 0xc0).
pub fn mod_obj_v50(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0x80);
        let field = |off: u32| Addr((this as u32).wrapping_add(off));
        let block = field(0x1c).lw(c);
        c.g[V0][0] = block;
        let mut imp = 0;
        if block != 0 {
            c.cycles += 1;
            imp = Addr(block as u32).lw(c);
        }
        c.g[S1][0] = imp;
        c.cycles += 2;
        if imp != 0 {
            c.cycles += 57;
            // The old bounds (two quadwords) to sp+0x20.
            c.g[A0][0] = field(0x1c).lw(c);
            callee::GET_TRANSFORM_BOUNDS.call(c, 0x0035_68e4)?;
            let bounds = r32(c, V0);
            c.lq(V1, bounds);
            c.g[A0][0] = field(0x1c).lw(c);
            let old_min = frame.at(c, 0x20);
            c.sq(V1, old_min);
            c.lq(A1, bounds.wrapping_add(0x10));
            let old_max = frame.at(c, 0x30);
            c.sq(A1, old_max);
            // The radius vector (r, r, r, 0) at sp+0x40.
            callee::GET_TRANSFORM_RADIUS.call(c, 0x0035_68fc)?;
            let radius = c.f[0];
            let rad = Addr(frame.at(c, 0x40));
            rad.off(0xc).sw(c, 0);
            rad.off(8).write_u32(c, radius);
            rad.write_u32(c, radius);
            rad.off(4).write_u32(c, radius);
            // New min = position - radius (sp, sp+0x50).
            let vt = field(0xc).lw(c);
            c.g[V1][0] = vt;
            let position = VSlot::read(c, vt, 0xc0);
            c.g[A0][0] = position.this(this);
            c.g[V0][0] = position.func;
            call_ptr(c, position.func as u32, 0x0035_6920)?;
            let pos = r32(c, V0);
            offset_by_radius(c, frame, pos, false);
            let vt = field(0xc).lw(c);
            c.g[V1][0] = vt;
            for off in [0, 0x50] {
                let at = frame.at(c, off);
                c.sq(V0, at);
            }
            // New max = position + radius (sp+0x10, sp+0x50).
            let position = VSlot::read(c, vt, 0xc0);
            c.g[A0][0] = position.this(this);
            c.g[V0][0] = position.func;
            call_ptr(c, position.func as u32, 0x0035_6958)?;
            let pos = r32(c, V0);
            offset_by_radius(c, frame, pos, true);
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = field(0x1c).lw(c);
            for off in [0x10, 0x50] {
                let at = frame.at(c, off);
                c.sq(V0, at);
            }
            callee::SET_TRANSFORM_BOUNDS.call(c, 0x0035_6988)?;
            // SpatialHash_Update(World_GetNash(), 0, obj, new, old).
            let obj = field(0x18).lw(c);
            c.g[S0][0] = obj;
            call_ptr(c, WORLD_GET_NASH, 0x0035_6990)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = obj;
            c.g[A3][0] = c.g[SP][0];
            c.g[T0][0] = sx(old_min);
            c.g[A1][0] = 0;
            call_ptr(c, SPATIAL_HASH_UPDATE, 0x0035_69a8)?;
            let vt = Addr(imp as u32).lw(c);
            c.g[V0][0] = vt;
            let notify = VSlot::read(c, vt, 0xc0);
            c.g[A0][0] = notify.this(imp);
            c.g[V1][0] = notify.func;
            call_ptr(c, notify.func as u32, 0x0035_69bc)?;
        }
        frame.restore(c, S0, 0xa0);
        c.cycles += 4;
        frame.restore(c, S1, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}


/// `ModObj_v50`'s box corner: the position quadword at `pos + 0x30`
/// (through v1 and sp+0x60) plus or minus the radius vector at sp+0x40 in
/// VU0 (VF3 ± VF4 → VF5), stored to sp+0x70 and loaded into v0.
pub(crate) fn offset_by_radius(c: &mut Cpu, frame: Frame, pos: u32, add: bool) {
    c.lq(V1, pos.wrapping_add(0x30));
    let staged = frame.at(c, 0x60);
    c.sq(V1, staged);
    let p = Vec4::from_bits(c.bus.read128(staged));
    let rad_at = frame.at(c, 0x40);
    let r = Vec4::from_bits(c.bus.read128(rad_at));
    let corner = if add {
        Vec4 { x: p.x + r.x, y: p.y + r.y, z: p.z + r.z, w: p.w + r.w }
    } else {
        Vec4 { x: p.x - r.x, y: p.y - r.y, z: p.z - r.z, w: p.w - r.w }
    };
    vu0_scratch::set_vf(c, 3, 0b1111, p);
    vu0_scratch::set_vf(c, 4, 0b1111, r);
    vu0_scratch::set_vf(c, 5, 0b1111, corner);
    let out = frame.at(c, 0x70);
    c.bus.write128(out, corner.to_bits());
    c.lq(V0, out);
}


/// `block && *(block + off) != 0`, as `ModObj_Render` tests the modifier
/// block's optional parts (+2 cycles when there is a block).
pub(crate) fn block_has(c: &mut Cpu, block: u64, off: u32) -> bool {
    if block == 0 {
        return false;
    }
    c.cycles += 2;
    Addr((block as u32).wrapping_add(off)).lw(c) != 0
}


/// `void ModObj_Render(ModObj *this)` (0x356298). Draws the object unless:
/// its model's layer (+0x7d) is switched off (layer mode != 6, or the
/// pass (+0x7e) mode != 3), the model's flags (+8) lack bit 2, the object
/// is hidden (+0x28), the camera test (graphics manager method 0x2e8 on
/// the object's own bounds, method 0x168, or the model's box at +0x60)
/// answers 1, or (model box only) the world occludes it. First it renders
/// the modifier block (`ModifierBlock_Render`). The draw call is the
/// graphics manager's method 0x2f8 with the model, the object's matrix
/// (method 0xc0, or method 0xd8's with the identity matrix when 0xd0 says
/// it has one), the UV-scroll matrix, the block's +8 part (or the model's
/// +0x94), the draw flags (0x2000: the block implementation's method 0xd0
/// answered 7; 0x400: camera-tested; 0x20: the test answered neither 0 nor
/// 1; 2: UV scroll) and f12 = `this+0x24`.
pub fn mod_obj_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x110);
        let layers = Addr(r32(c, GP).wrapping_add(LAYER_TABLE)).lw(c);
        c.g[V0][0] = layers;
        frame.save(c, S4, 0xc0);
        frame.save(c, S0, 0x100);
        let this = c.g[A0][0];
        c.g[S4][0] = this;
        frame.save(c, S1, 0xf0);
        c.g[A0][0] = 0xff;
        frame.save(c, S2, 0xe0);
        frame.save(c, S3, 0xd0);
        frame.save(c, S5, 0xb0);
        frame.save(c, S6, 0xa0);
        frame.save(c, S7, 0x90);
        frame.save_ra(c, 0x80);
        let field = |off: u32| Addr((this as u32).wrapping_add(off));
        // Layer and pass switches.
        let model = field(0x18).lw(c);
        c.g[A1][0] = model;
        let table = Addr(layers as u32).lw(c);
        c.g[A2][0] = table;
        let layer = Addr(model as u32).off(0x7d).lbu(c);
        c.g[V1][0] = layer;
        let visible: u64 = if layer == 0xff {
            1
        } else {
            c.cycles += 7;
            let layer = Addr(model as u32).off(0x7d).lbu(c) as u32;
            let mode = Addr((layer << 3).wrapping_add(table as u32)).off(0x24).lw(c) ^ 6;
            c.g[V1][0] = mode;
            if mode != 0 {
                0
            } else {
                c.cycles += 7;
                let pass = Addr(model as u32).off(0x7e).lh(c);
                c.g[A0][0] = 0x18;
                let pass_off = abi::mult(c, pass as u32, 0x18);
                let mode = Addr(pass_off.wrapping_add(table as u32)).off(0x3f0).lw(c) ^ 3;
                c.g[V1][0] = mode;
                (mode == 0) as u64
            }
        };
        c.g[V0][0] = visible;
        c.cycles += 2;
        c.g[S6][0] = 0;
        // How many of s0/s1 the path already restored (the epilogue's entry).
        let restored: u8 = 'body: {
            if visible == 0 {
                break 'body 0;
            }
            c.cycles += 3;
            let block = field(0x1c).lw(c);
            c.g[A0][0] = block;
            let gfx = Addr(r32(c, GP).wrapping_add(GFX_MAN)).lw(c);
            c.g[S5][0] = gfx;
            if block == 0 {
                c.cycles += 1;
            } else {
                c.cycles += 6;
                callee::MODIFIER_BLOCK_RENDER.call(c, 0x0035_6334)?;
                let block = field(0x1c).lw(c);
                c.g[V0][0] = block;
                let imp = Addr(block as u32).lw(c);
                c.g[A1][0] = imp;
                if imp != 0 {
                    c.cycles += 8;
                    let vt = Addr(imp as u32).lw(c);
                    c.g[V1][0] = vt;
                    let kind = VSlot::read(c, vt, 0xd0);
                    c.g[A0][0] = kind.this(imp);
                    c.g[V0][0] = kind.func;
                    call_ptr(c, kind.func as u32, 0x0035_6358)?;
                    let kind = c.g[V0][0] ^ 7;
                    c.g[V0][0] = kind;
                    c.g[V1][0] = 0x2000;
                    if kind == 0 {
                        c.g[S6][0] = 0x2000;
                    }
                    c.cycles += 1;
                }
            }
            let model = field(0x18).lw(c);
            c.g[V1][0] = model;
            c.cycles += 4;
            let drawn = Addr(model as u32).off(8).lw(c) & 4;
            c.g[V0][0] = drawn;
            frame.restore(c, S0, 0x100);
            if drawn == 0 {
                break 'body 1;
            }
            c.cycles += 3;
            let hidden = field(0x28).lw(c);
            c.g[V0][0] = hidden;
            frame.restore(c, S1, 0xf0);
            if hidden != 0 {
                break 'body 2;
            }
            // The camera test, on the object's own bounds or the model's box.
            c.cycles += 8;
            let vt = field(0xc).lw(c);
            c.g[V1][0] = vt;
            let own = VSlot::read(c, vt, 0x160);
            c.g[A0][0] = own.this(this);
            c.g[V0][0] = own.func;
            call_ptr(c, own.func as u32, 0x0035_6398)?;
            let own_bounds = c.g[V0][0];
            c.g[S7][0] = own_bounds;
            if own_bounds == 0 {
                let model = field(0x18).lw(c);
                c.g[V0][0] = model;
                c.cycles += 32;
                let (min, max, view) = (frame.at(c, 0), frame.at(c, 0x10), frame.at(c, 0x40));
                c.g[S2][0] = sx(view);
                let cam_vt = Addr((gfx as u32).wrapping_add(0x10d8)).lw(c);
                c.g[S1][0] = cam_vt;
                c.g[S3][0] = sx(max);
                let m = Addr(model as u32);
                let corner = [
                    m.off(0x60).read_u32(c),
                    m.off(0x64).read_u32(c),
                    m.off(0x68).read_u32(c),
                    m.off(0x6c).read_u32(c),
                    m.off(0x70).read_u32(c),
                    m.off(0x74).read_u32(c),
                ];
                c.g[A0][0] = sx(view);
                let test_delta = Addr((cam_vt as u32).wrapping_add(0x2e8)).lh(c);
                c.g[S0][0] = addu(gfx, test_delta);
                c.g[AT][0] = ONE as u64;
                (c.f[0], c.f[1], c.f[2]) = (corner[5], corner[4], ONE);
                for (i, &w) in corner[..3].iter().enumerate() {
                    Addr(min).off(4 * i as u32).write_u32(c, w);
                    Addr(max).off(4 * i as u32).write_u32(c, corner[3 + i]);
                }
                Addr(min).off(0xc).write_u32(c, ONE);
                Addr(max).off(0xc).write_u32(c, ONE);
                let vt = Addr((gfx as u32).wrapping_add(0x10d8)).lw(c);
                c.g[V0][0] = vt;
                let query = VSlot::read(c, vt, 0x158);
                c.g[A1][0] = query.this(gfx);
                c.g[V1][0] = query.func;
                call_ptr(c, query.func as u32, 0x0035_6464)?;
                c.g[V0][0] = Addr((cam_vt as u32).wrapping_add(0x2ec)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[S2][0];
                c.g[A1][0] = c.g[SP][0];
            } else {
                c.cycles += 21;
                let vt = field(0xc).lw(c);
                c.g[V1][0] = vt;
                let get = VSlot::read(c, vt, 0x168);
                c.g[A0][0] = get.this(this);
                c.g[V0][0] = get.func;
                call_ptr(c, get.func as u32, 0x0035_63b8)?;
                let cam_vt = Addr((gfx as u32).wrapping_add(0x10d8)).lw(c);
                c.g[S1][0] = cam_vt;
                let bounds = c.g[V0][0];
                c.g[S2][0] = bounds;
                c.g[A0][0] = c.g[SP][0];
                c.g[S3][0] = sx((bounds as u32).wrapping_add(0x10));
                let test_delta = Addr((cam_vt as u32).wrapping_add(0x2e8)).lh(c);
                let query = VSlot::read(c, cam_vt, 0x158);
                c.g[S0][0] = addu(gfx, test_delta);
                c.g[A1][0] = query.this(gfx);
                c.g[V0][0] = query.func;
                call_ptr(c, query.func as u32, 0x0035_63e0)?;
                c.g[V0][0] = Addr((cam_vt as u32).wrapping_add(0x2ec)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[SP][0];
            }
            c.cycles += 6;
            let test = r32(c, V0);
            call_ptr(c, test, 0x0035_6480)?;
            let answer = c.g[V0][0];
            c.g[V1][0] = answer;
            c.g[V0][0] = 1;
            c.g[S6][0] |= 0x400;
            if answer == 1 {
                break 'body 0;
            }
            c.cycles += 3;
            let partly = c.g[S6][0] | 0x20;
            c.g[V0][0] = partly;
            if answer != 0 {
                c.g[S6][0] = partly;
            }
            if own_bounds == 0 {
                c.cycles += 5;
                let model = field(0x18).lw(c);
                c.g[A0][0] = sx((model as u32).wrapping_add(0x50));
                call_ptr(c, WORLD_IS_OCCLUDED, 0x0035_64a8)?;
                frame.restore(c, S0, 0x100);
                if c.g[V0][0] != 0 {
                    break 'body 1;
                }
            }
            // The object's matrix (s2) and its own extra matrix (s1).
            c.cycles += 8;
            let vt = field(0xc).lw(c);
            c.g[V1][0] = vt;
            c.g[S0][0] = 0;
            let has = VSlot::read(c, vt, 0xd0);
            c.g[A0][0] = has.this(this);
            c.g[V0][0] = has.func;
            call_ptr(c, has.func as u32, 0x0035_64c8)?;
            let has_own_matrix = c.g[V0][0] != 0;
            let vt = field(0xc).lw(c);
            c.g[V1][0] = vt;
            if has_own_matrix {
                c.cycles += 8;
                let get = VSlot::read(c, vt, 0xd8);
                c.g[A0][0] = get.this(this);
                c.g[V0][0] = get.func;
                call_ptr(c, get.func as u32, 0x0035_64e0)?;
                c.g[V1][0] = 0x50_0000;
                c.g[S1][0] = c.g[V0][0];
                c.g[S2][0] = sx(IDENTITY_MATRIX);
            } else {
                c.cycles += 6;
                c.g[S1][0] = 0;
                let get = VSlot::read(c, vt, 0xc0);
                c.g[A0][0] = get.this(this);
                c.g[V0][0] = get.func;
                call_ptr(c, get.func as u32, 0x0035_6504)?;
                c.g[S2][0] = c.g[V0][0];
            }
            // The UV-scroll matrix of the modifier block (+4) into s0.
            c.cycles += 3;
            let mut block = field(0x1c).lw(c);
            c.g[A0][0] = block;
            let has_uv = block_has(c, block, 4);
            c.cycles += 2;
            if has_uv {
                c.cycles += 2;
                let has_uv = block_has(c, block, 4);
                c.cycles += 2;
                c.g[S0][0] = 0;
                if has_uv {
                    c.cycles += 3;
                    let uv = Addr((block as u32).wrapping_add(4)).lw(c);
                    c.g[A0][0] = uv;
                    c.g[V0][0] = 0;
                    if uv != 0 {
                        c.cycles += 4;
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, UV_SCROLL_GET_MATRIX, 0x0035_6550)?;
                    } else {
                        c.cycles += 1;
                    }
                    c.g[S0][0] = c.g[V0][0];
                    c.cycles += 1;
                    block = field(0x1c).lw(c);
                    c.g[A0][0] = block;
                }
                c.cycles += 1;
                c.g[S6][0] |= 2;
            }
            // The block's +8 part (+0x24 into it), else the model's +0x94.
            c.cycles += 2;
            let has_extra = block_has(c, block, 8);
            c.cycles += 2;
            let model_extra = |c: &mut Cpu| {
                let model = field(0x18).lw(c);
                c.cycles += 1;
                Addr((model as u32).wrapping_add(0x94)).lw(c)
            };
            let extra = if !has_extra {
                model_extra(c)
            } else {
                c.cycles += 2;
                let has_extra = block_has(c, block, 8);
                c.cycles += 2;
                if !has_extra {
                    model_extra(c)
                } else {
                    c.cycles += 3;
                    let part = Addr((block as u32).wrapping_add(8)).lw(c);
                    c.g[A0][0] = part;
                    if part != 0 {
                        c.cycles += 2;
                        sx((part as u32).wrapping_add(0x24))
                    } else {
                        0
                    }
                }
            };
            c.g[A2][0] = extra;
            // draw(gfx, model, extra, matrix, own matrix, uv, flags; alpha).
            c.cycles += 11;
            let cam_vt = Addr((gfx as u32).wrapping_add(0x10d8)).lw(c);
            c.g[V0][0] = cam_vt;
            c.g[A3][0] = c.g[S2][0];
            let alpha = field(0x24).read_u32(c);
            c.f[12] = alpha;
            c.g[T0][0] = c.g[S1][0];
            let draw = VSlot::read(c, cam_vt, 0x2f8);
            c.g[T1][0] = c.g[S0][0];
            c.g[V1][0] = draw.func;
            c.g[T2][0] = c.g[S6][0];
            c.g[A1][0] = field(0x18).lw(c);
            c.g[A0][0] = draw.this(gfx);
            call_ptr(c, draw.func as u32, 0x0035_65d8)?;
            0
        };
        if restored < 1 {
            c.cycles += 1;
            frame.restore(c, S0, 0x100);
        }
        if restored < 2 {
            c.cycles += 1;
            frame.restore(c, S1, 0xf0);
        }
        c.cycles += 9;
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore(c, S7, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Objects.

/// `bool ModObj_v20(ModObj* o)` (0x356bf0): runs the object's slot-0xd0
/// method (an object type query) and reports whether it returned 2.
///
/// The reference calls it `VCall_6BF0(o, 26) == 2` through the slot table at
/// `*(o+0x1c) -> *slot -> vtable[26]`.
pub fn mod_obj_v20(c: &mut Cpu) {
    c.cycles += 5;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    let this = abi::arg64(c, 0);
    c.g[V1][0] = Addr(this as u32).off(0x1c).lw(c);
    c.g[V0][0] = 0;
    // (The "no object" branch goes straight to the epilogue: it skips the
    // tail increment and RA reload below.)
    let mut straight_out = false;
    if c.g[V1][0] != 0 {
        c.cycles += 3;
        c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
        let obj = c.g[A1][0];
        c.g[RA][0] = Addr(sp(c)).ld(c);
        if obj == 0 {
            straight_out = true;
        } else {
            c.cycles += 9;
            c.g[V1][0] = Addr(obj as u32).lw(c);
            let vt = c.g[V1][0];
            c.g[A0][0] = Addr(vt as u32).off(0xd0).lh(c);
            c.g[V0][0] = Addr(vt as u32).off(0xd4).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0035_6c24);
            c.g[A0][0] = abi::addu(obj, c.g[A0][0]);
            if call_ptr(c, target, 0x0035_6c24).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = 0x2;
            let same = c.g[A0][0] == c.g[V1][0];
            c.g[V0][0] = 1;
            if !same {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
    }
    if !straight_out {
        c.cycles += 1;
        c.g[RA][0] = Addr(sp(c)).ld(c);
    }
    c.cycles += 2;
    frame.close(c);
}


// ---------------------------------------------------------------------------
// Move nodes.

/// `int ModObj_v44(ModObj* self)` (0x3569d0): whether the modifier block at
/// `self + 0x1c` has transform bounds (`ModifierBlock_GetTransformBounds`
/// non-zero); 0 without a block.
pub fn mod_obj_v44(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x1c).lw(c);
        let no_block = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !no_block {
            c.cycles += 3;
            callee::GET_TRANSFORM_BOUNDS_2.call(c, 0x0035_69ec)?;
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* ModObj_v24(ModObj* self)` (0x356078): the first child at
/// `*(*(self + 0x1c))`; when it exists, dispatch `child + (short)
/// *(vtable + 0x90)` through `*(vtable + 0x94)` (the child's update), else
/// return `*(self + 0x18) + 0x10` (the node's own matrix).
pub fn mod_obj_v24(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[V1][0] = Addr(this).off(0x1c).lw(c);
        let no_head = c.g[V1][0] == 0;
        c.g[A1][0] = 0;
        if !no_head {
            c.cycles += 1;
            c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
        }
        c.cycles += 2;
        if c.g[A1][0] != 0 {
            let child = c.g[A1][0] as u32;
            c.g[V1][0] = Addr(child).lw(c);
            c.cycles += 4;
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x90).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x94).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0035_60b4)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(this).off(0x18).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Particle_Update.

/// `void Particle_Update(Particle* p)` (0x357950): the base object update;
/// afterwards, unless the modifier block is mid-transform (`+0x1c` set with a
/// non-zero `+0x10`), tell the object's vtable slot 1 (`+8`) that it is done
/// (`a1 = 3`).
pub fn particle_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MOD_OBJ_UPDATE.call(c, 0x0035_7964)?;
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x1c));
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 2;
        let null_self = c.g[S0][0] == 0;
        c.g[A1][0] = 0x3;
        if !null_self {
            c.cycles += 5;
            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0035_7998)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Modifier nodes.

/// `int ModObj_v45(MoveNode* self)` (0x356a00): the transform bounds of the
/// modifier block at +0x1c (0 when there is none).
pub fn mod_obj_v45(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(at(c, A0, 0x1c));
        let none = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !none {
            c.cycles += 2;
            callee::MODIFIER_BLOCK_GET_TRANSFORM_BOUNDS.call(c, 0x0035_6a1c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// ModObj_v23.

/// `int ModObj_v23(ModObj* self)` (0x356d48): 0 unless the object's block
/// pointer (`+0x1c`) is set, its first node is set and that node's vtable
/// slot 26 (`+0xd0`) reports mode 2; then the node's state word `+0x34`.
pub fn mod_obj_v23(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        let block = Addr(this).off(0x1c).lw(c);
        c.g[V1][0] = block;
        c.g[V0][0] = 0;
        let mut tail = true;
        if block != 0 {
            c.cycles += 3;
            let node = lw32(c, block as u32);
            c.g[A1][0] = sx(node);
            if node == 0 {
                let a = frame.at(c, 0x10);
                c.lq(S0, a);
                tail = false;
            } else {
                c.cycles += 8;
                let vt = lw32(c, node);
                c.g[V1][0] = sx(vt);
                c.g[A0][0] = Addr(vt).off(0xd0).lh(c);
                c.g[V0][0] = Addr(vt).off(0xd4).lw(c);
                let target = r32(c, V0);
                let delta = r32(c, A0);
                c.g[RA][0] = sx(0x0035_6d84);
                c.g[A0][0] = sx(node.wrapping_add(delta));
                call_ptr(c, target, 0x0035_6d84)?;
                c.g[V1][0] = 0x2;
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(this).off(0x1c).lw(c);
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    c.g[V0][0] = Addr(r32(c, V1)).off(0x34).lw(c);
                }
            }
        }
        if tail {
            c.cycles += 1;
            let a = frame.at(c, 0x10);
            c.lq(S0, a);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ModObj_v43(ModObj* self)` (0x356ab8): forward to the modifier
/// block's v43.
pub fn mod_obj_v43(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at_2(abi::arg(c, 0), 0x1c)).lw(c);
        let block = c.g[A0][0];
        frame.restore_ra(c, 0);
        if block != 0 {
            c.cycles += 3;
            callee::MODIFIER_BLOCK_FORWARD_V43.call(c, 0x0035_6ad4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}


/// `void ModObj_v42(ModObj* self)` (0x356ae0): forward to v42.
pub fn mod_obj_v42(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at_2(abi::arg(c, 0), 0x1c)).lw(c);
        let block = c.g[A0][0];
        frame.restore_ra(c, 0);
        if block != 0 {
            c.cycles += 3;
            callee::MODIFIER_BLOCK_FORWARD_V42.call(c, 0x0035_6afc)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}


/// `void ModObj_v36(ModObj* self)` (0x356b08): forward an event to the
/// modifier block's event handler.
pub fn mod_obj_v36(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at_2(abi::arg(c, 0), 0x1c)).lw(c);
        let block = c.g[A0][0];
        frame.restore_ra(c, 0);
        if block != 0 {
            c.cycles += 3;
            callee::MODIFIER_BLOCK_HANDLE_EVENT.call(c, 0x0035_6b24)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}


/// `void MultiParticleMan_ctor(MultiParticleMan* man)` (0x3579c8): publish the
/// singleton and clear its four lists.
pub fn multi_particle_man_ctor(c: &mut Cpu) {
    const HEAD: u32 = 0x0035_79d8;
    c.cycles += 4;
    let man = abi::arg(c, 0);
    c.g[V0][0] = sx(man);
    c.g[A1][0] = 3;
    c.sw(r32(c, GP).wrapping_add(0xf38), c.g[V0][0]);
    c.g[V1][0] = sx(man.wrapping_add(0xc));
    loop {
        c.cycles += 7;
        Addr(r32(c, V1)).sw(c, 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, HEAD);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void* Object_ctor_2(Object* self)` (0x356e60).
pub fn object_ctor_2(c: &mut Cpu) {
    guest(c, |c| ctor_with_vtable(c, 0x0035_6e74, callee::MOD_OBJ_CTOR, 0x0049_0e80))
}


/// `void* Particle_ctor(Particle* self)` (0x357918).
pub fn particle_ctor(c: &mut Cpu) {
    guest(c, |c| ctor_with_vtable(c, 0x0035_792c, callee::MOD_OBJ_CTOR, 0x0048_ee60))
}


/// `int ModObj_v14(self)` (0x355420): the move-node block's
/// "is-section-removed" query (vtable slot 0x40/0x44 of `*(self+0x1c)`);
/// 1 when there is no block or no node.
pub fn mod_obj_v14(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        c.g[V1][0] = Addr(this).off(0x1c).lw(c);
        let no_block = c.g[V1][0] == 0;
        c.g[V0][0] = 1;
        let mut extra = 0;
        if no_block {
            extra = 1;
        } else {
            c.cycles += 3;
            c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
            if c.g[A1][0] != 0 {
                c.cycles += 7;
                let node = c.g[A1][0] as u32;
                c.g[V1][0] = Addr(node).lw(c);
                let vt = c.g[V1][0] as u32;
                c.g[A0][0] = Addr(vt).off(0x40).lh(c);
                c.g[V0][0] = Addr(vt).off(0x44).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0035_5454);
                c.g[A0][0] = sx(node.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0035_5454)?;
            }
        }
        c.cycles += extra + 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int ModObj_v39(self)` (0x356a70): the move-node block's modifier-block
/// query (vtable slot 0xa8/0xac); 0 when there is no block or no node.
pub fn mod_obj_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        c.g[V1][0] = Addr(this).off(0x1c).lw(c);
        let no_block = c.g[V1][0] == 0;
        c.g[V0][0] = 0;
        let mut extra = 0;
        if no_block {
            extra = 1;
        } else {
            c.cycles += 3;
            c.g[T1][0] = Addr(c.g[V1][0] as u32).lw(c);
            if c.g[T1][0] != 0 {
                c.cycles += 7;
                let node = c.g[T1][0] as u32;
                c.g[V1][0] = Addr(node).lw(c);
                let vt = c.g[V1][0] as u32;
                c.g[A0][0] = Addr(vt).off(0xa8).lh(c);
                c.g[V0][0] = Addr(vt).off(0xac).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0035_6aa4);
                c.g[A0][0] = sx(node.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0035_6aa4)?;
            }
        }
        c.cycles += extra + 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ModObj_SetRailModifier(ModObj* self, RailModifier* item)` (0x3556a8):
/// create the modifier block when absent, then push `item` onto the rail
/// modifier list.
pub fn mod_obj_set_rail_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this).off(0x1c).lw(c);
        let present = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !present {
            c.cycles += 2;
            callee::MOD_OBJ_CREATE_MODIFIER_BLOCK.call(c, 0x0035_56d0)?;
        }
        c.cycles += 9;
        c.g[A0][0] = Addr(this).off(0x1c).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1c));
        callee::RAIL_MODIFIER_LIST_PUSH_FRONT.call(c, 0x0035_56e0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ModObj_SetTexFlipModifier(ModObj* self, TexFlipModifier* v)`
/// (0x3555a8): make sure the modifier block exists, clear the old tex-flip
/// modifier and install `v` in slot `+0x8`. Returns the block in v0.
pub fn mod_obj_set_tex_flip_modifier(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = Addr(p(c, S0, 0x1c)).lw(c);
    let have = c.g[A0][0] != 0;
    c.g[S1][0] = c.g[A1][0];
    if have {
        c.cycles += 3;
        if callee::MODIFIER_BLOCK_FREE_TEX_FLIP
            .call(c, 0x0035_55e0)
            .is_err()
        {
            return;
        }
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        if callee::MOD_OBJ_CREATE_MODIFIER_BLOCK
            .call(c, 0x0035_55d0)
            .is_err()
        {
            return;
        }
    }
    c.g[V0][0] = Addr(p(c, S0, 0x1c)).lw(c);
    c.cycles += 6;
    frame.restore(c, S0, 0x20);
    c.sw(abi::r32(c, V0).wrapping_add(8), c.g[S1][0]);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `void ModObj_SetBoostModifier(ModObj* self, BoostModifier* v)` (0x355600):
/// the same shape as the tex-flip setter, in slot `+0xc`.
pub fn mod_obj_set_boost_modifier(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = Addr(p(c, S0, 0x1c)).lw(c);
    let have = c.g[A0][0] != 0;
    c.g[S1][0] = c.g[A1][0];
    if have {
        c.cycles += 3;
        if callee::MODIFIER_BLOCK_FREE_BOOST
            .call(c, 0x0035_5638)
            .is_err()
        {
            return;
        }
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        if callee::MOD_OBJ_CREATE_MODIFIER_BLOCK
            .call(c, 0x0035_5628)
            .is_err()
        {
            return;
        }
    }
    c.g[V0][0] = Addr(p(c, S0, 0x1c)).lw(c);
    c.cycles += 6;
    frame.restore(c, S0, 0x20);
    c.sw(abi::r32(c, V0).wrapping_add(0xc), c.g[S1][0]);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `void MultiParticleMan_Clear(MultiParticle** list)` (0x357b38): destroy and
/// clear the first four particle slots.
pub fn multi_particle_man_clear(c: &mut Cpu) {
    c.cycles += 6;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = 3;
    loop {
        c.cycles += 5;
        c.g[V0][0] = Addr(p(c, S0, 0)).lw(c);
        c.g[A1][0] = 3;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
        let null = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[V0][0];
        if !null {
            c.cycles += 2;
            if callee::MULTI_PARTICLE_DTOR.call(c, 0x0035_7b6c).is_err() {
                return;
            }
        }
        c.cycles += 3;
        c.sw(p(c, S0, 0), 0);
        let again = (c.g[S1][0] as i64) >= 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
        if !again {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0035_7b50);
        }
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `void ModObj_AddAvaSplineModifier(ModObj* self)` (0x355a78): create the
/// Ava spline transform modifier once and install it.
pub fn mod_obj_add_ava_spline_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, 0x9d4)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let installed = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if !installed {
            c.cycles += 12;
            c.g[A1][0] = 0x0049_0000;
            c.g[A0][0] = 0x90;
            c.g[A1][0] = sx(0x0048_e928);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0035_5aa8)?;
            c.g[A1][0] = Addr(r32(c, S0)).off(0x18).lw(c);
            c.g[RA][0] = sx(0x0035_5ab4);
            c.g[A0][0] = c.g[V0][0];
            callee::AVA_SPLINE_MODIFIER_CTOR.call(c, 0x0035_5ab4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0035_5ac0);
            c.g[A1][0] = c.g[V0][0];
            callee::MOD_OBJ_SET_TRANSFORM_MODIFIER.call(c, 0x0035_5ac0)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `PositionModifier* PositionModifier_ctor(PositionModifier* self, const
/// Quad* pos)` (0x356eb8): vtable, copied home position, identity transform
/// and the initial matrix.
pub fn position_modifier_ctor(c: &mut Cpu) {
    c.cycles += 22;
    c.g[V0][0] = 0x0049_0000;
    c.g[V1][0] = 0x0050_0000;
    c.g[V0][0] = sx(0x0048_f5f0);
    c.g[V1][0] = sx(0x004f_f1a0);
    c.sw(r32(c, A0), c.g[V0][0]);
    c.g[A2][0] = sx(r32(c, A0).wrapping_add(0x50));
    c.lq(2, r32(c, A1));
    c.sq(2, p_2(r32(c, A0), 0x30));
    c.lqc2(1, r32(c, V1));
    c.lqc2(2, p_2(r32(c, V1), 0x10));
    c.lqc2(3, p_2(r32(c, V1), 0x20));
    c.lqc2(4, p_2(r32(c, V1), 0x30));
    c.sqc2(1, r32(c, A2));
    c.sqc2(2, p_2(r32(c, A2), 0x10));
    c.sqc2(3, p_2(r32(c, A2), 0x20));
    c.sqc2(4, p_2(r32(c, A2), 0x30));
    c.g[V0][0] = 1;
    c.lq(5, p_2(r32(c, A0), 0x30));
    c.sw(p_2(r32(c, A0), 0x44), c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    c.sq(5, p_2(r32(c, A0), 0x80));
    abi::ret(c);
}


/// `ParentModifier* ParentModifier_ctor(self, a1, a2, pos)` (0x357038): the
/// position modifier with a different vtable and two extra fields.
pub fn parent_modifier_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A3][0];
        c.g[RA][0] = sx(0x0035_7060);
        callee::POSITION_MODIFIER_CTOR.call(c, 0x0035_7060)?;
        c.g[V1][0] = 0x0049_0000;
        Addr(r32(c, S0)).off(0xa0).sw(c, c.g[S1][0]);
        c.g[V1][0] = sx(0x0048_f508);
        Addr(r32(c, S0)).off(0x90).sw(c, c.g[S2][0]);
        Addr(r32(c, S0)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Move-node modifiers (object/movenode.cpp).

/// `void ModObj_v41(Modifier* self, float secs)` (0x355858): `self->+0x20 =
/// (int)((float)gp->0x2a74->0x10 * secs)` — an integer field scaled by the
/// frame time and truncated.
pub fn mod_obj_v41(c: &mut Cpu) {
    c.cycles += 8;
    let gp = r32(c, GP);
    let cfg = Addr(gp.wrapping_add(0x2a74)).lw(c);
    c.g[V0][0] = cfg;
    let cfg = cfg as u32;
    c.lwc1(0, cfg.wrapping_add(0x10));
    c.cvt_s_w(0, 0);
    c.mul_s(0, 0, 12);
    c.cvt_w_s(1, 0);
    let this = r32(c, A0);
    c.swc1(1, this.wrapping_add(0x20));
    abi::ret(c);
}


/// `void PositionModifier_UpdateTransform(PositionModifier* self)`
/// (0x356ff0): publish the node's offset (`+0x30`, a quadword) at `+0x80`
/// and clear the pending-flag at `+0x44`.
pub fn position_modifier_update_transform(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.lq(2, this.wrapping_add(0x30));
    c.sw(this.wrapping_add(0x44), 0);
    c.sq(2, this.wrapping_add(0x80));
    abi::ret(c);
}


/// `void MagnetModifier_UpdateTransform(MagnetModifier* self)` (0x357528):
/// publish the magnet target (`+0x10`) at `+0x90` and clear the pending
/// flag at `+0x50`.
pub fn magnet_modifier_update_transform(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.lq(2, this.wrapping_add(0x10));
    c.sw(this.wrapping_add(0x50), 0);
    c.sq(2, this.wrapping_add(0x90));
    abi::ret(c);
}


/// `void ModObj_AddPositionModifier(ModObj* self, void* params)` (0x3558B8):
/// allocate and install a position modifier built from `params`.
pub fn mod_obj_add_position_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x490000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x90;
        c.g[A1][0] = sx(ea(c.g[A1][0], 0xffff_e8f0));
        call_ptr(c, addr::MEM_MAN_ALLOC, 0x0035_58e8)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        callee::POSITION_MODIFIER_CTOR_2.call(c, 0x0035_58f4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        callee::MOD_OBJ_SET_TRANSFORM_MODIFIER_2.call(c, 0x0035_5900)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `lw reg, off(base)` leaving the zero-extended value in `reg`.
#[inline(always)]
pub(crate) fn lw_into(c: &mut Cpu, reg: usize, base: u32, off: u32) {
    c.g[reg][0] = Addr(base.wrapping_add(off)).lw(c);
}


// ---------------------------------------------------------------------------
// Object modifiers (movenode.cpp).

/// `void cMoveNode_addSpline(cMoveNode* self, void* desc)` (0x355b30):
/// allocate a multi-spline modifier (0x58 bytes), construct it from `desc`
/// and the node's modifier block, and install it as the transform modifier.
pub fn mod_obj_add_multi_spline_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S0, 0x20);
        let desc = c.g[A1][0];
        let obj = c.g[A0][0];
        c.g[S1][0] = desc;
        c.g[S0][0] = obj;
        c.g[A1][0] = sx(0x0048_e9e8);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x58;
        call_ptr(c, 0x0031_7d70, 0x0035_5b60)?;
        lw_into(c, A2, r32_2(c, S0), 0x18);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0035_9f88, 0x0035_5b70)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0035_54b0, 0x0035_5b7c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void func_00355BF0(cMoveNode* self, void* desc)` (0x355bf0): allocate and
/// install a texture-flip modifier (0xe8 bytes).
pub fn mod_obj_add_tex_flip_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S0, 0x20);
        let desc = c.g[A1][0];
        let obj = c.g[A0][0];
        c.g[S1][0] = desc;
        c.g[S0][0] = obj;
        c.g[A1][0] = sx(0x0048_e958);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0xe8;
        call_ptr(c, 0x0031_7d70, 0x0035_5c20)?;
        lw_into(c, A2, r32_2(c, S0), 0x18);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0035_f0b8, 0x0035_5c30)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0035_55a8, 0x0035_5c3c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void func_00355C50(cMoveNode* self, void* desc)` (0x355c50): allocate and
/// install a boost modifier (0x50 bytes).
pub fn mod_obj_add_boost_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S0, 0x20);
        let desc = c.g[A1][0];
        let obj = c.g[A0][0];
        c.g[S1][0] = desc;
        c.g[S0][0] = obj;
        c.g[A1][0] = sx(0x0048_e968);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x50;
        call_ptr(c, 0x0031_7d70, 0x0035_5c80)?;
        lw_into(c, A2, r32_2(c, S0), 0x18);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0034_2768, 0x0035_5c90)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0035_5600, 0x0035_5c9c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void func_00357848(AvaSplineModifier* self, void* desc)` (0x357848): save
/// the modifier's curve data through two virtual methods of the two objects.
pub fn ava_spline_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let desc = c.g[A1][0];
        let self_ = c.g[A0][0];
        c.g[S0][0] = desc;
        frame.save_ra(c, 0);
        c.g[S1][0] = self_;
        c.g[V0][0] = Addr(desc as u32).lw(c);
        c.g[A1][0] = Addr(at_3(c, S1, 0x80)).lw(c);
        c.g[A0][0] = Addr(r32_2(c, V0)).off(0x28).lh(c);
        c.g[V1][0] = Addr(r32_2(c, V0)).off(0x2c).lw(c);
        let target = r32_2(c, V1);
        c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, target, 0x0035_7878)?;
        c.g[A3][0] = Addr(r32_2(c, S0)).lw(c);
        c.g[A1][0] = sx(at_3(c, S1, 0x10));
        c.g[A2][0] = 0x30;
        c.g[A0][0] = Addr(r32_2(c, A3)).off(0x8).lh(c);
        c.g[V0][0] = Addr(r32_2(c, A3)).off(0xc).lw(c);
        let target = r32_2(c, V0);
        c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, target, 0x0035_7894)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Parent (transform) modifiers and particle sections.

/// `void ParentModifier_v22(ParentModifier* self, void* obj)` (0x357210):
/// copy the current transform value into the object around a virtual call.
pub fn parent_modifier_v22(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(p_3(c.g[V0][0] as u32, 0xa0)).lw(c);
        let no_parent = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        let mut plus1 = true;
        if !no_parent {
            c.cycles += 3;
            c.g[A2][0] = Addr(p_3(c.g[A0][0] as u32, 0xc)).lw(c);
            if c.g[A2][0] == 0 {
                plus1 = false;
            } else {
                c.cycles += 9;
                c.g[V1][0] = Addr(p_3(c.g[V0][0] as u32, 0x90)).lw(c);
                c.g[S0][0] = Addr(p_3(c.g[S1][0] as u32, 0x5c)).lw(c);
                Addr(p_3(c.g[S1][0] as u32, 0x5c)).sw(c, c.g[V1][0]);
                c.g[V0][0] = Addr(p_3(c.g[A2][0] as u32, 0xc)).lw(c);
                c.g[A0][0] = Addr(p_3(c.g[V0][0] as u32, 0x150)).lh(c);
                c.g[V1][0] = Addr(p_3(c.g[V0][0] as u32, 0x154)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0035_725c)?;
                Addr(p_3(c.g[S1][0] as u32, 0x5c)).sw(c, c.g[S0][0]);
            }
        }
        if plus1 {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ParentModifier_Save(ParentModifier* self, Stream* stream)` (0x357278).
pub fn parent_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0035_7000, 0x0035_7294)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A1][0] = sx(p_3(c.g[S1][0] as u32, 0x90));
        c.g[A2][0] = 0x4;
        c.g[A0][0] = Addr(p_3(c.g[V0][0] as u32, 8)).lh(c);
        c.g[V1][0] = Addr(p_3(c.g[V0][0] as u32, 0xc)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_72b0)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A1][0] = Addr(p_3(c.g[S1][0] as u32, 0xa0)).lw(c);
        c.g[A0][0] = Addr(p_3(c.g[V0][0] as u32, 0x28)).lh(c);
        c.g[V1][0] = Addr(p_3(c.g[V0][0] as u32, 0x2c)).lw(c);
        let target2 = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target2, 0x0035_72c8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void MultiParticleMan_OnSectionUnload(MultiParticleMan* self, int arg)`
/// (0x357b90): unload the four section instance lists.
pub fn multi_particle_man_on_section_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0x3;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        loop {
            c.cycles += 5;
            c.g[A1][0] = c.g[S2][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            let skip = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !skip {
                c.cycles += 2;
                call_ptr(c, 0x0035_8140, 0x0035_7bd4)?;
            }
            c.cycles += 2;
            if (c.g[S1][0] as i64) >= 0 {
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0035_7bb8);
                }
                continue;
            }
            c.cycles += 6;
            frame.restore(c, S0, 0x30);
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}


/// `ModObj_v28(self)` 0x3560c0: forward to the held object's slot +0xb8,
/// falling back to the modifier's own slot +0xd8 when that returns 0.
pub fn mod_obj_v28(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        c.g[V1][0] = lw_at(c, this, 0x1c);
        c.g[A2][0] = 0;
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            c.g[A2][0] = Addr(r32(c, V1)).lw(c);
        }
        c.cycles += 2;
        let mut v1;
        if c.g[A2][0] == 0 {
            v1 = lw_at(c, this, 0xc);
            c.g[V1][0] = v1;
        } else {
            c.cycles += 7;
            let obj = r32(c, A2);
            v1 = lw_at(c, obj, 0);
            c.g[V1][0] = v1;
            let vt = v1 as u32;
            c.g[A0][0] = Addr(p_4(vt, 0xb8)).lh(c);
            c.g[V0][0] = lw_at(c, vt, 0xbc);
            let func = r32(c, V0);
            c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
            call_ptr(c, func, 0x0035_60fc)?;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                frame.restore(c, S0, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 1;
            v1 = lw_at(c, this, 0xc);
            c.g[V1][0] = v1;
        }
        c.cycles += 5;
        let vt = v1 as u32;
        c.g[A0][0] = Addr(p_4(vt, 0xd8)).lh(c);
        c.g[V0][0] = lw_at(c, vt, 0xdc);
        let func = r32(c, V0);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, func, 0x0035_6118)?;
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `MagnetModifier_v09(self, id)` (0x357660): return the cached value when the
/// magnet already tracks `id`; otherwise ask the rider world object whether the
/// id is valid and remember it.
pub fn magnet_modifier_v09(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p_5(c.g[S1][0] as u32, 0x44)).lw(c);
        let unset = (c.g[V0][0] as i64) < 0;
        c.g[S0][0] = c.g[A1][0];
        if !unset {
            c.cycles += 2;
            let different = c.g[V0][0] != c.g[S0][0];
            c.g[V0][0] = 0;
            if different {
                // cached miss
            } else {
                c.cycles += 2;
                c.g[V0][0] = Addr(p_5(c.g[S1][0] as u32, 0x54)).lw(c);
            }
        } else {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002d_1b08, 0x0035_7698)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[A0][0] = Addr(p_5(c.g[V1][0] as u32, 0x40)).lh(c);
            c.g[A1][0] = Addr(p_5(c.g[V1][0] as u32, 0x44)).lw(c);
            let target = c.g[A1][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0035_76ac)?;
            if c.g[V0][0] != 0 {
                Addr(p_5(c.g[S1][0] as u32, 0x44)).sw(c, c.g[S0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `MultiParticle* MultiParticleMan_CreateSlot(MultiParticleMan* m, int i,
/// void* a2, void* a3)` (0x357a78): create slot `i`'s particle instance on
/// first use.
pub fn multi_particle_man_create_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = abi::addu(c.g[A0][0], c.g[A1][0]);
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let occupied = c.g[V0][0] != 0;
        c.g[S2][0] = c.g[A2][0];
        if !occupied {
            c.cycles += 11;
            c.g[A1][0] = 0x0048_ea00;
            c.g[A0][0] = 0x50;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call::call_ptr(c, 0x0031_7d70, 0x0035_7abc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            call::call_ptr(c, 0x0035_7dd8, 0x0035_7acc)?;
            Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}


/// `ParentModifier_ctor_2` (0x357090): construct a parent modifier from its
/// stream: base position modifier, its vtable, the parent read and the parent
/// id stored at +0xa0.
pub fn parent_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, addr::POSITION_MODIFIER_CTOR_3, 0x0035_70ac)?;
        c.g[V0][0] = 0x49_0000;
        c.g[A1][0] = low(c, S1).wrapping_add(0x90);
        c.g[V0][0] = low(c, V0).wrapping_sub(0xaf8);
        c.g[A2][0] = 4;
        sw_at(c, S1, 0, c.g[V0][0]);
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x14)).lw(c);
        let read = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, read, 0x0035_70d4)?;
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x18)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x1c)).lw(c);
        let parent = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, parent, 0x0035_70e8)?;
        sw_at(c, S1, 0xa0, c.g[V0][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ModObj_AddRailModifier(self, desc)` (0x355e38): unless the object's
/// modifier block already has a rail modifier for `desc+4`, allocate a
/// 0x90-byte `RailModifier` and install it.
pub fn mod_obj_add_rail_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx(low_2(c, SP).wrapping_sub(0x30));
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[A0][0] = lw_r(c, S0, 0x1c);
        let none = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        let mut done = false;
        if !none {
            c.cycles += 4;
            c.g[A1][0] = lw_r(c, S1, 0x4);
            callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER.call(c, 0x0035_5e60)?;
            if c.g[V0][0] != 0 {
                c.lq(16, sp_2(c).wrapping_add(0x20));
                done = true;
            }
        }
        if !done {
            c.cycles += 14;
            c.g[A1][0] = 0x49_0000;
            c.g[A0][0] = 0x90;
            c.g[A1][0] = sx(low_2(c, A1).wrapping_sub(0x1648));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::C_MEM_MAN_ALLOC, 0x0035_5e80)?;
            c.g[A2][0] = lw_r(c, S0, 0x18);
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            callee::RAIL_MODIFIER_CTOR.call(c, 0x0035_5e90)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            callee::MOD_OBJ_SET_RAIL_MODIFIER.call(c, 0x0035_5e9c)?;
            c.lq(16, sp_2(c).wrapping_add(0x20));
        }
        c.cycles += 4;
        c.lq(17, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low_2(c, RA);
        c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}


/// `MagnetModifier* MagnetModifier_ctor(self, owner, mat, float a, float b)`
/// (0x3572e0): install the vtable, copy the 4x4 matrix into `+0x60` through
/// VU0, store the speed (b * 100/3.6) and the quad at `+0x10`.
pub fn magnet_modifier_ctor(c: &mut Cpu) {
    c.cycles += 30;
    c.lwc1(0, gpo(c, -0x2a84));
    c.g[V0][0] = 0x49_0000;
    c.g[V0][0] = sx(low_2(c, V0).wrapping_sub(0xbe0));
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    c.mul_s(13, 13, 0);
    c.g[SP][0] = sx(low_2(c, SP).wrapping_sub(0x10));
    sw_r(c, A0, 0, c.g[V0][0]);
    c.g[A3][0] = sx(low_2(c, A0).wrapping_add(0x60));
    sw_r(c, A0, 0xa0, c.g[A1][0]);
    swc1_r(c, 12, A0, 0x40);
    swc1_r(c, 13, A0, 0x48);
    sw_r(c, A0, 0x44, c.g[V1][0]);
    sw_r(c, A0, 0x4c, 0);
    sw_r(c, A0, 0x54, 0);
    c.lqc2(1, low_2(c, A2));
    c.lqc2(2, low_2(c, A2).wrapping_add(0x10));
    c.lqc2(3, low_2(c, A2).wrapping_add(0x20));
    c.lqc2(4, low_2(c, A2).wrapping_add(0x30));
    c.sqc2(1, low_2(c, A3));
    c.sqc2(2, low_2(c, A3).wrapping_add(0x10));
    c.sqc2(3, low_2(c, A3).wrapping_add(0x20));
    c.sqc2(4, low_2(c, A3).wrapping_add(0x30));
    c.lq(3, low_2(c, A0).wrapping_add(0x90));
    c.g[V0][0] = 0x1;
    sw_r(c, A0, 0x50, c.g[V0][0]);
    c.sq(3, low_2(c, A0).wrapping_add(0x10));
    c.g[V0][0] = c.g[A0][0];
    c.sq(3, sp_2(c));
    let t = low_2(c, RA);
    c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x10));
    c.pc = t;
}


/// `void MultiParticleMan_dtor(self, flags)` (0x357a00): destroy the four
/// owned particle groups, clear the global manager pointer and free the
/// object when `flags & 1`.
pub fn multi_particle_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(low_2(c, SP).wrapping_sub(0x50));
        c.sq(18, sp_2(c).wrapping_add(0x20));
        c.sq(16, sp_2(c).wrapping_add(0x40));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, sp_2(c).wrapping_add(0x30));
        c.g[S0][0] = c.g[S2][0];
        c.sq(19, sp_2(c).wrapping_add(0x10));
        c.g[S1][0] = 0x3;
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S3][0] = c.g[A1][0];
        loop {
            c.cycles += 3;
            c.g[A0][0] = Addr(low_2(c, S0)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = 0x3;
                callee::MULTI_PARTICLE_DTOR_2.call(c, 0x0035_7a3c)?;
            }
            c.g[S1][0] = sx(low_2(c, S1).wrapping_sub(1));
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(low_2(c, S0).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0035_7a28);
                }
                continue;
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[S3][0] & 0x1;
            let keep = c.g[V0][0] == 0;
            sw_r(c, GP, 0xf38, 0);
            if !keep {
                c.cycles += 2;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, callee::BUILTIN_DELETE_2.addr, 0x0035_7a5c)?;
            }
            c.cycles += 7;
            c.lq(16, sp_2(c).wrapping_add(0x40));
            c.lq(17, sp_2(c).wrapping_add(0x30));
            c.lq(18, sp_2(c).wrapping_add(0x20));
            c.lq(19, sp_2(c).wrapping_add(0x10));
            c.g[RA][0] = Addr(sp_2(c)).ld(c);
            let t = low_2(c, RA);
            c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x50));
            c.pc = t;
            break;
        }
        Ok(())
    })
}


/// `void ModObj_AddParentModifier(self, a1, a2, pos)` (0x00355978): allocate a
/// parent modifier, build it and install it as the transform modifier.
pub fn mod_obj_add_parent_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A3][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[A1][0] = sx(0x490000);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0xb0;
        c.g[A1][0] = sx(0x490000u32.wrapping_sub(0x1720));
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0;
        callee::MEM_ALLOC_NAMED.call(c, 0x0035_59b8)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = c.g[V0][0];
        callee::PARENT_MODIFIER_CTOR.call(c, 0x0035_59cc)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[V0][0];
        callee::MOD_OBJ_SET_TRANSFORM_MODIFIER_3.call(c, 0x0035_59d8)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ModObj_AddRollerModifier(self, a1, a2, a3)` (0x00355db8): allocate a
/// roller modifier, build it and install it as the transform modifier.
pub fn mod_obj_add_roller_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A3][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[A1][0] = sx(0x490000);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x2d0;
        c.g[A1][0] = sx(0x490000u32.wrapping_sub(0x16f8));
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0;
        callee::MEM_ALLOC_NAMED.call(c, 0x0035_5df8)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = c.g[V0][0];
        callee::ROLLER_MODIFIER_CTOR.call(c, 0x0035_5e0c)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[V0][0];
        callee::MOD_OBJ_SET_TRANSFORM_MODIFIER_3.call(c, 0x0035_5e18)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int ModObj_v21(self, a1)` (0x00356c48): when the object's multi-spline
/// modifier is in state 2, add a reference to it.
pub fn mod_obj_v21(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at(c, S0, 0x1c)).lw(c);
        let none = c.g[V1][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if none {
            c.cycles += 1;
            c.g[V0][0] = 0;
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            c.cycles += 4;
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A1][0] = Addr(at(c, V1, 0)).lw(c);
        let null = c.g[A1][0] == 0;
        c.g[V0][0] = 0;
        if null {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            c.cycles += 4;
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 8;
        c.g[V1][0] = Addr(at(c, A1, 0)).lw(c);
        c.g[A0][0] = Addr(at(c, V1, 0xd0)).lh(c);
        c.g[V0][0] = Addr(at(c, V1, 0xd4)).lw(c);
        c.g[A0][0] = add32(word(c, A1), word(c, A0));
        call_ptr(c, word(c, V0), 0x0035_6c88)?;
        c.g[V1][0] = 0x2;
        let not_state2 = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0;
        if not_state2 {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            c.cycles += 4;
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(at(c, S0, 0x1c)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = Addr(at(c, V0, 0)).lw(c);
        callee::MULTI_SPLINE_MODIFIER_ADD_REF.call(c, 0x0035_6ca4)?;
        c.lq(16, frame.at(c, 0x20));
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void MagnetModifier_Save(self, stream)` (0x003576d0): write the magnet's
/// three parameter blocks through the stream's virtual writer.
pub fn magnet_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(at(c, S0, 0)).lw(c);
        c.g[A1][0] = Addr(at(c, S1, 0xa0)).lw(c);
        c.g[A0][0] = Addr(at(c, V0, 0x28)).lh(c);
        c.g[V1][0] = Addr(at(c, V0, 0x2c)).lw(c);
        c.g[A0][0] = add32(word(c, S0), word(c, A0));
        call_ptr(c, word(c, V1), 0x0035_7700)?;
        c.g[A3][0] = Addr(at(c, S0, 0)).lw(c);
        c.g[A1][0] = sx(word(c, S1).wrapping_add(0x10));
        c.g[A2][0] = 0x50;
        c.g[A0][0] = Addr(at(c, A3, 0x8)).lh(c);
        c.g[V0][0] = Addr(at(c, A3, 0xc)).lw(c);
        c.g[A0][0] = add32(word(c, S0), word(c, A0));
        call_ptr(c, word(c, V0), 0x0035_771c)?;
        c.g[V1][0] = Addr(at(c, S0, 0)).lw(c);
        c.g[A1][0] = sx(word(c, S1).wrapping_add(0x60));
        c.g[A2][0] = 0x40;
        c.g[A0][0] = Addr(at(c, V1, 0x8)).lh(c);
        c.g[V0][0] = Addr(at(c, V1, 0xc)).lw(c);
        c.g[A0][0] = add32(word(c, S0), word(c, A0));
        call_ptr(c, word(c, V0), 0x0035_7738)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void MultiParticle_dtor(self, flags)` (0x00357fa0): free the particle
/// arrays, tell the owner, then free the object when asked to.
pub fn multi_particle_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = Addr(at(c, S0, 0x8)).lw(c);
        callee::BUILTIN_DELETE_3.call(c, 0x0035_7fc0)?;
        c.g[A0][0] = Addr(at(c, S0, 0xc)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[A2][0] = Addr(at(c, S0, 0x10)).lw(c);
        } else {
            c.cycles += 3;
            callee::MEM_FREE_2.call(c, 0x0035_7fd4)?;
            c.g[A2][0] = Addr(at(c, S0, 0x10)).lw(c);
        }
        c.cycles += 2;
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(at(c, A2, 0x18c)).lw(c);
            c.g[A0][0] = Addr(at(c, V0, 0x8)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0xc)).lw(c);
            c.g[A0][0] = add32(word(c, A2), word(c, A0));
            call_ptr(c, word(c, V1), 0x0035_7ff4)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            c.lq(16, frame.at(c, 0x20));
        } else {
            // `a0 = s0` still needs the object pointer: the original reloads
            // s0 only after `__builtin_delete` returns.
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::BUILTIN_DELETE_3.call(c, 0x0035_8008)?;
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ModObj_AddMagnetModifier(ModObj* o, float* v, void* target)`
/// (0x003559f8): allocate a 0xb0-byte magnet modifier, construct it with the
/// two float parameters and install it as the object's transform modifier.
pub fn mod_obj_add_magnet_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.g[A3][0] = 0;
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x2000_0000;
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[S1][0] = c.g[A1][0];
        c.swc1(21, sp(c).wrapping_add(0x38));
        c.g[S0][0] = c.g[A0][0];
        c.swc1(20, sp(c).wrapping_add(0x30));
        c.g[A1][0] = sx(0x0048_e918);
        c.sd(sp(c), c.g[RA][0]);
        c.mov_s(20, 12);
        c.mov_s(21, 13);
        c.g[A0][0] = 0xb0;
        call_ptr(c, 0x0031_7d70, 0x0035_5a38)?; // EA_MemAllocNamed
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0035_72e0, 0x0035_5a50)?; // MagnetModifier_ctor
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0035_54b0, 0x0035_5a5c)?; // ModObj_SetTransformModifier
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.lwc1(21, sp(c).wrapping_add(0x38));
        c.lwc1(20, sp(c).wrapping_add(0x30));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}


/// `AvaSplineModifier* AvaSplineModifier_ctor_2(AvaSplineModifier* self,
/// RenderInstance* inst)` (0x357798): install the vtable, take the instance's
/// id and transform through its vtable, and snapshot the avalanche matrix.
pub fn ava_spline_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        open(c, 0x40);
        let sp0 = spv(c);
        c.g[V0][0] = 0x490000;
        c.sq(17, at_4(sp0, 0x20));
        c.g[V0][0] = sx(at_4(c.g[V0][0] as u32, 0xffff_f338));
        c.sq(16, at_4(sp0, 0x30));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, at_4(sp0, 0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd(sp0, c.g[RA][0]);
        let this = c.g[S1][0];
        c.g[S2][0] = sx(at_4(this as u32, 0x10));
        c.sw(this as u32, c.g[V0][0]);
        let inst = c.g[S0][0];
        c.g[V1][0] = lw_at_2(c, inst, 0);
        c.g[A0][0] = lh_at(c, c.g[V1][0], 0x18);
        c.g[V0][0] = lw_at_2(c, c.g[V1][0], 0x1c);
        let func = c.g[V0][0] as u32;
        c.g[A0][0] = sx(at_4(inst as u32, c.g[A0][0] as u32));
        call_ptr(c, func, 0x0035_77d8)?;
        c.sw(at_4(this as u32, 0x80), c.g[V0][0]);
        c.g[A2][0] = 0x30;
        c.g[A1][0] = c.g[S2][0];
        c.g[V0][0] = lw_at_2(c, inst, 0);
        c.g[A0][0] = lh_at(c, c.g[V0][0], 0x10);
        c.g[V1][0] = lw_at_2(c, c.g[V0][0], 0x14);
        let func = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at_4(inst as u32, c.g[A0][0] as u32));
        call_ptr(c, func, 0x0035_77f8)?;
        c.g[A0][0] = lw_at_2(c, this, 0x80);
        c.g[A1][0] = sx(at_4(this as u32, 0x40));
        callee::AVALANCHE_GET_OBJECT_MATRIX.call(c, 0x0035_7804)?;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, at_4(sp0, 0x30));
        c.lq(17, at_4(sp0, 0x20));
        c.lq(18, at_4(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_4(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}


/// `void* MagnetModifier_ctor_2(self, stream)` (0x357358): install the
/// modifier's vtable and deserialize its two parameter blocks.
pub fn magnet_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x0048_f420;
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(this.wrapping_add(0x10));
        Addr(this).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x18).lh(c);
        c.g[V0][0] = lw_3(c, V1, 0x1c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_7398)?;
        sw(c, S1, 0xa0, c.g[V0][0]);
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = 0x50;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x10).lh(c);
        c.g[V1][0] = lw_3(c, V0, 0x14);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_73b8)?;
        c.g[A3][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A1][0] = sx(this.wrapping_add(0x60));
        c.g[A2][0] = 0x40;
        c.g[A0][0] = Addr(c.g[A3][0] as u32).off(0x10).lh(c);
        c.g[V0][0] = lw_3(c, A3, 0x14);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_73d4)?;
        c.g[V1][0] = 1;
        c.g[V0][0] = c.g[S1][0];
        sw(c, S1, 0x50, c.g[V1][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `sMultiPart* MultiParticle_ctor(mem, src, owner)` (0x357dd8): allocate the
/// particle data, copy it in, set up the emitter and copy the descriptor's
/// vectors.
pub fn multi_particle_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x40);
        c.g[A3][0] = 0;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A1][0];
        Addr(r32(c, S0)).sw(c, c.g[A2][0]);
        c.g[A1][0] = sx(0x0048_ea10);
        Addr(p_6(r32(c, S0), 4)).sw(c, 0);
        c.g[A0][0] = 0xd8;
        c.g[A2][0] = 0x2000_0000;
        call_ptr(c, 0x0031_7d70, 0x00357e10)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        Addr(p_6(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0xd8;
        call_ptr(c, 0x003e_6574, 0x00357e24)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0035_8020, 0x00357e2c)?;
        c.g[A1][0] = Addr(p_6(r32(c, S0), 0x10)).lw(c);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        c.lq(3, p_6(r32(c, A1), 0x160));
        frame.restore_ra(c, 0x10);
        c.sq(3, p_6(r32(c, S0), 0x20));
        c.lq(4, p_6(r32(c, A1), 0x170));
        c.sq(4, p_6(r32(c, S0), 0x30));
        c.lwc1(2, p_6(r32(c, A1), 0xc8));
        c.lwc1(0, p_6(r32(c, A1), 0xc0));
        c.lwc1(1, p_6(r32(c, A1), 0xc4));
        c.swc1(0, sp(c));
        c.swc1(1, p_6(sp(c), 4));
        c.swc1(2, p_6(sp(c), 8));
        Addr(p_6(sp(c), 0xc)).sw(c, 0);
        c.lq(3, sp(c));
        c.sq(3, p_6(r32(c, S0), 0x40));
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `MultiParticleMan_Load` (0x357d28): clear the manager, then for four stream
// entries read a `(delta, fn)` pair; when the entry reports "present",
// allocate and construct the particle there, otherwise store null.
pub fn multiparticle_man_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[V0][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0x3;
        frame.save_ra(c, 0x10);
        c.g[S3][0] = 0x490000;
        callee::MULTIPARTICLE_MAN_CLEAR.call(c, 0x0035_7d58)?;
        loop {
            c.cycles += 14;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32));
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = 0x4;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, target, 0x0035_7d78)?;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32));
            c.g[A0][0] = 0x50;
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1600));
            c.g[A2][0] = 0x20000000;
            let absent = c.g[V0][0] == 0;
            c.g[A3][0] = 0u64;
            if absent {
                c.cycles += 1;
                c.sw((c.g[S0][0] as u32), 0u64);
            } else {
                c.cycles += 7;
                call_ptr(c, 0x00317d70, 0x0035_7d98)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S1][0];
                callee::MULTIPARTICLE_CTOR.call(c, 0x0035_7da4)?;
                c.sw((c.g[S0][0] as u32), c.g[V0][0]);
            }
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_7d58);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Modifier objects.

/// `void ModObj_v40(ModObj* self, void* a1, void* a2, int a3)` (0x355770):
/// forward the "run script event" call to the holder object at `self+0x1c`
/// (its slot 9/10 pair), then, if the object's script timer at `+0x20` has
/// expired, restart it from half the global tick delta and run the event.
pub fn mod_obj_v40(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r(c, S2)).off(0x1c).lw(c);
        let no_holder = r(c, V1) == 0;
        c.g[S5][0] = c.g[A3][0];

        'outer: {
            if !no_holder {
                c.cycles += 3;
                c.g[A3][0] = Addr(r(c, V1)).lw(c);
                if c.g[A3][0] != 0 {
                    // obj->v10(a1, a2)
                    c.cycles += 18;
                    c.g[V1][0] = Addr(r(c, A3)).lw(c);
                    c.g[A0][0] = Addr(r(c, V1)).off(0x50).lh(c);
                    c.g[V0][0] = Addr(r(c, V1)).off(0x54).lw(c);
                    let target = r(c, V0);
                    c.g[A0][0] = sx(p_7(r(c, A3), r(c, A0)));
                    call_ptr(c, target, 0x0035_57c8)?;

                    c.g[V1][0] = Addr(r(c, S2)).off(0x1c).lw(c);
                    c.g[A0][0] = c.g[S5][0];
                    c.g[S0][0] = Addr(r(c, V1)).lw(c);
                    c.g[S1][0] = Addr(r(c, S0)).lw(c);
                    c.g[V0][0] = Addr(r(c, S1)).off(0x48).lh(c);
                    c.g[S0][0] = sx(p_7(r(c, S0), r(c, V0)));
                    call_ptr(c, 0x002d_1b30, 0x0035_57e4)?;
                    c.g[V1][0] = Addr(r(c, S1)).off(0x4c).lw(c);
                    c.g[A0][0] = c.g[S0][0];
                    let target = r(c, V1);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, target, 0x0035_57f4)?;

                    let done = r(c, V0) == 0;
                    frame.restore(c, 16, 0x60);
                    if done {
                        break 'outer;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(r(c, S2)).off(0x20).lw(c);
                } else {
                    c.g[V0][0] = Addr(r(c, S2)).off(0x20).lw(c);
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = Addr(r(c, S2)).off(0x20).lw(c);
            }

            c.cycles += 2;
            let expired = (c.g[V0][0] as i64) > 0;
            frame.restore(c, 16, 0x60);
            if expired {
                break 'outer;
            }
            c.cycles += 12;
            c.g[V0][0] = Addr(gp_2(c, 0x2a74)).lw(c);
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[V1][0] = Addr(r(c, V0)).off(0x10).lw(c);
            c.g[A3][0] = c.g[S5][0];
            c.g[V0][0] = sx(r(c, V1) >> 31);
            c.g[V1][0] = sx(p_7(r(c, V1), r(c, V0)));
            c.g[V1][0] = sx((r(c, V1) as i32 >> 1) as u32);
            c.sw(Addr(r(c, S2)).off(0x20).0, c.g[V1][0]);
            callee::MOD_OBJ_RUN_SCRIPT_EVENT.call(c, 0x0035_5834)?;
            frame.restore(c, 16, 0x60);
        }

        c.cycles += 8;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void MagnetModifier_Update(MagnetMod* self)` (0x3573f8): advance the
/// magnet modifier's pull: move the target by the per-frame step, then add
/// the clamped displacement to the current position (and mark the modifier
/// when the target was reached).
pub fn magnet_modifier_update(c: &mut Cpu) {
    'b_35751c: {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(16, c.pa(SP, 0x40));
        c.std(SP, 0x30, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.ldw_2(S0, 0x44);
        if (c.g[V0][0] as i64) < 0 {
            c.lq(16, c.pa(SP, 0x40));
            break 'b_35751c;
        } else {
            'b_357514: {
                'b_357490: {
                    c.cycles += 29;
                    c.g[RA][0] = sx(0x0035741c);
                    c.pc = 0x002d1c70;
                    call_indirect(c, 0x002d1c70);
                    if c.pc != 0x0035741c {
                        return;
                    }
                    c.lwc1(1, c.pa(S0, 0x4c));
                    c.g[A0][0] = c.ldw_2(S0, 0x44);
                    c.add_s(1, 1, 0);
                    c.g[RA][0] = sx(0x00357430);
                    c.swc1(1, c.pa(S0, 0x4c));
                    c.pc = 0x002d1b58;
                    call_indirect(c, 0x002d1b58);
                    if c.pc != 0x00357430 {
                        return;
                    }
                    c.lqc2(3, c.pa(V0, 0));
                    c.lqc2(4, c.pa(S0, 0x10));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, c.pa(SP, 0x10));
                    c.g[AT][0] = 0x42480000;
                    c.mtc1(1, 0);
                    c.lq(2, c.pa(SP, 0x10));
                    c.sq(2, c.pa(SP, 0));
                    c.lqc2(3, c.pa(SP, 0));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 2);
                    c.c_lt_s(2, 0);
                    let t = !c.fcc();
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_357490;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.stw_2(S0, 0x54, c.g[V0][0]);
                        if t {
                            break 'b_357514;
                        } else {
                            break 'b_357490;
                        }
                    }
                }
                'b_357500: {
                    c.cycles += 25;
                    c.lwc1(0, c.pa(S0, 0x48));
                    c.lwc1(1, c.pa(S0, 0x4c));
                    c.div_s(0, 0, 2);
                    c.mul_s(0, 0, 1);
                    c.mfc1(3, 0);
                    c.lqc2(4, c.pa(SP, 0));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, c.pa(SP, 0x20));
                    c.lq(2, c.pa(SP, 0x20));
                    c.sq(2, c.pa(SP, 0x10));
                    c.lqc2(3, c.pa(SP, 0x10));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.c_lt_s(2, 0);
                    let t = !c.fcc();
                    c.lq(3, c.pa(SP, 0));
                    if t {
                        break 'b_357500;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        c.stw_2(S0, 0x54, c.g[V0][0]);
                        c.sq(3, c.pa(SP, 0x10));
                        break 'b_357500;
                    }
                }
                c.cycles += 5;
                c.lqc2(3, c.pa(S0, 0x10));
                c.lqc2(4, c.pa(SP, 0x10));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, c.pa(S0, 0x10));
                c.g[V0][0] = 0x1;
                break 'b_357514;
            }
            c.cycles += 2;
            c.stw_2(S0, 0x50, c.g[V0][0]);
            c.lq(16, c.pa(SP, 0x40));
            break 'b_35751c;
        }
    }
    c.cycles += 3;
    c.g[RA][0] = c.ldd(SP, 0x30);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = ra;
    return;
}


/// `void* func_00357E80(void* mem, void* stream)` (0x357e80): construct a
/// multi-particle at `mem` from a stream: read the id and the 0xd8-byte
/// emitter data into a fresh allocation, set up the particle, load the
/// descriptor from the stream and read the position, boxes and points.
pub fn multi_particle_ctor_2(c: &mut Cpu) {
    let frame = Frame::open(c, 0x40);
    'b_357f80: {
        c.cycles += 58;
        c.g[A2][0] = 0x4;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[S0][0] = sx(at(c, S2, 0x20));
        c.g[V1][0] = lw_4(c, r32(c, S1));
        c.g[A0][0] = lh(c, at(c, V1, 0x10));
        c.g[V0][0] = lw_4(c, at(c, V1, 0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00357ebc);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        if call_ptr(c, t, 0x00357ebc).is_err() {
            return;
        }
        c.g[V1][0] = lw_4(c, r32(c, S1));
        c.g[A1][0] = sx(at(c, S2, 0x4));
        c.g[A2][0] = 0x4;
        c.g[A0][0] = lh(c, at(c, V1, 0x10));
        c.g[V0][0] = lw_4(c, at(c, V1, 0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00357ed8);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        if call_ptr(c, t, 0x00357ed8).is_err() {
            return;
        }
        c.g[A1][0] = 0x490000;
        c.g[A3][0] = 0u64;
        c.g[A0][0] = 0xd8;
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x15f0));
        c.g[RA][0] = sx(0x00357ef0);
        c.g[A2][0] = 0x20000000;
        if call_ptr(c, 0x00317d70, 0x00357ef0).is_err() {
            return;
        }
        sw_2(c, at(c, S2, 0x8), c.g[V0][0]);
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0xd8;
        c.g[V0][0] = lw_4(c, r32(c, S1));
        c.g[A0][0] = lh(c, at(c, V0, 0x10));
        c.g[V1][0] = lw_4(c, at(c, V0, 0x14));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00357f10);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        if call_ptr(c, t, 0x00357f10).is_err() {
            return;
        }
        c.g[RA][0] = sx(0x00357f18);
        c.g[A0][0] = c.g[S2][0];
        if callee::C00358020.call(c, 0x00357f18).is_err() {
            return;
        }
        c.g[A0][0] = lw_4(c, at(c, S2, 0x10));
        c.g[RA][0] = sx(0x00357f24);
        c.g[A1][0] = c.g[S1][0];
        if call_ptr(c, 0x00370af8, 0x00357f24).is_err() {
            return;
        }
        c.g[V1][0] = lw_4(c, r32(c, S1));
        c.g[A1][0] = sx(at(c, S2, 0x40));
        c.g[A2][0] = 0x10;
        c.g[A0][0] = lh(c, at(c, V1, 0x10));
        c.g[V0][0] = lw_4(c, at(c, V1, 0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00357f40);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        if call_ptr(c, t, 0x00357f40).is_err() {
            return;
        }
        c.g[V1][0] = lw_4(c, r32(c, S1));
        c.g[A2][0] = 0x20;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = lh(c, at(c, V1, 0x10));
        c.g[V0][0] = lw_4(c, at(c, V1, 0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00357f5c);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        if call_ptr(c, t, 0x00357f5c).is_err() {
            return;
        }
        c.g[A2][0] = lw_4(c, at(c, S2, 0x4));
        let t = (c.g[A2][0] as i64) <= 0;
        c.g[A2][0] = sx(r32(c, A2) << 2);
        if t {
            break 'b_357f80;
        } else {
            c.cycles += 6;
            c.g[V0][0] = lw_4(c, r32(c, S1));
            c.g[A1][0] = lw_4(c, at(c, S2, 0xc));
            c.g[A0][0] = lh(c, at(c, V0, 0x10));
            c.g[V1][0] = lw_4(c, at(c, V0, 0x14));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00357f80);
            c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x00357f80).is_err() {
                return;
            }
            break 'b_357f80;
        }
    }
    c.cycles += 7;
    c.g[V0][0] = c.g[S2][0];
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x40));
    c.pc = t;
    return;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::fakes::*;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::check_lohi;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::GAME_GP;
    use crate::difftest::put_words;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes;
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
    use crate::difftest::events::driven;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::fakes::ptr;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::SP as STACK;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::seed_count;
    use crate::difftest::compare;
    pub(crate) use crate::game::render::particle::tests::SEEDS;
    pub(crate) use crate::game::render::particle::tests::vt;
    pub(crate) use crate::game::render::particle::tests::slot as slot_2;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::worldpainter::quadtree::tests::put_vtable;
    pub(crate) use crate::game::object::deadfadenode::tests::ctor_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear as clear_2;
    pub(crate) use crate::game::bx::bxstring::tests::step;
    pub(crate) use crate::game::bx::bxstring::tests::alloc_ready;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::MEM_HEAP;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA_SIZE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::run;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_init;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::alloc;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_3;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::clear as clear_3;
    pub(crate) use crate::game::render::particle::tests::base;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::be::bedbreward::tests::call_setup;
    pub(crate) use crate::game::be::bedbreward::tests::alloc_ready as alloc_ready_2;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festateoptions::tests::rng;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::be::beintplayer::tests::fillb;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::bx::bxstring2::tests::pool_allocator_3 as pool_allocator;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;
    pub(crate) use crate::game::fe::ovstatelodge::tests::VT;
    pub(crate) use crate::game::fe::debugfe::tests::w as w_2;
    pub(crate) use crate::game::fe::ovstatelodge::tests::slot as slot_3;
    pub(crate) use crate::game::fe::debugfe::tests::pool_allocator as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_LIBRARY;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_2 as slot_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::ovtemplatedialog::tests::ZERO_STUB;
    pub(crate) use crate::game::fe::ovtemplatedialog::tests::ONE_STUB;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals_2;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::camera::cameracontroller::tests::pool as pool_2;
    pub(crate) use crate::game::ai::ai::tests::slot as slot_5;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_5;
    pub(crate) use crate::game::ai::rider::tests::hh;
    pub(crate) use crate::game::ai::rider::tests::slot_2 as slot_6;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_6;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::movie::movieplayer::tests::RET_ZERO;
    pub(crate) use crate::game::movie::movieplayer::tests::RET_ONE;
    pub(crate) use crate::game::movie::movieplayer::tests::wh;
    pub(crate) use crate::game::movie::movieplayer::tests::ret_slot;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2;
    pub(crate) use crate::game::movie::movieplayer::tests::heap_init_2;
    pub(crate) use crate::game::movie::movieplayer::tests::pool_allocator as pool_allocator_3;
    pub(crate) use crate::game::fe::fememcard2::tests::alloc as alloc_2;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_2;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_4;
    pub(crate) use crate::game::be::belibrary::tests::SRC;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_5;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE_3;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::run as run_3;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_6;
    pub(crate) use crate::game::bx::bxstring2::tests::alloc_base as alloc_base_2;
    pub(crate) use crate::game::bx::bxstring2::tests::slot_2 as slot_8;
    pub(crate) use crate::game::bx::bxstring2::tests::f;
    pub(crate) use crate::game::fe::festaterewards::tests::BE_7 as BE_4;
    pub(crate) use crate::game::fe::festaterewards::tests::HOOKS;
    pub(crate) use crate::game::fe::festaterewards::tests::KEEP_F0;
    pub(crate) use crate::game::fe::festaterewards::tests::hook;
    pub(crate) use crate::game::fe::festaterewards::tests::force;
    pub(crate) use crate::game::be::beintstat::tests::call;
    pub(crate) use crate::game::be::beintstat::tests::pool_allocator as pool_allocator_4;
    pub(crate) use crate::game::be::beintstat::tests::allocator;
    pub(crate) use crate::game::fe::festaterewards::tests::strict;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_7;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_2;
    pub(crate) use crate::game::fe::fepopup::tests::AUX as AUX_2;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_8;


    #[test]
    fn ava_spline_modifier_ctor_matches() {
        check(ssx3_ee::s11::f_00357750, ava_spline_modifier_ctor, 100, &|c, seed| {
            let mut r = Rng(0x3577_5000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let arg = DATA + 0x200;
            c.g[A1][0] = arg as u64;
            for i in 0..0x40 / 4 {
                c.bus.write32(arg + 0x10 + 4 * i, r.next_u32());
            }
        });
    }


    #[test]
    fn ava_spline_modifier_update_matches() {
        check(ssx3_ee::s11::f_00357820, ava_spline_modifier_update, 60, &|c, seed| {
            let mut r = Rng(0x3578_2000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x80, DATA + 0x100);
            for i in 0..0x40 / 4 {
                c.bus.write32(DATA + 0x40 + 4 * i, r.edge_f32());
            }
        });
    }


    /// A real guest function that returns 0 (`UIState_TransitionIn`), for
    /// vtable slots whose 0 answer a fake cannot give.
    pub(crate) const RETURN_ZERO: u32 = 0x003a_5ab0;


    /// Vtable slot `off` = `{delta, func}`.
    pub(crate) fn slot(c: &mut Cpu, vt: u32, off: u32, delta: u32, func: u32) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, func);
    }


    /// A random `this` adjustment.
    pub(crate) fn delta(r: &mut Rng) -> u32 {
        (r.next_u32() as i16 as i32 & !3) as u32
    }


    // ------------------------------------------------------ modifier objects

    /// `ModObj_v50`: block / implementation present or not; the radius
    /// slot is the real `ModifierBlock_GetTransformRadius` on an object
    /// without implementation (radius 0), and the position (fakes 1, 2)
    /// equals both corners of the old bounds (fake 0), so the real
    /// `SpatialHash_Update` finds the keys unchanged.
    #[test]
    fn mod_obj_v50_matches() {
        covered(ssx3_ee::s11::f_003568b0__generated, mod_obj_v50, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3568_b000, 5);
            let this = DATA;
            let (block, imp, imp_vt, this_vt) = (DATA + 0x100, DATA + 0x200, DATA + 0x300, DATA + 0x400);
            let (app, game, old, pos) = (DATA + 0x600, DATA + 0x700, DATA + 0x1000, DATA + 0x1100);
            fill(c, &mut r, this, 0x10);
            c.g[A0][0] = sx(this);
            c.bus.write32(this + 0xc, this_vt);
            c.bus.write32(this + 0x18, DATA + 0x500);
            c.bus.write32(this + 0x1c, if seed % 6 == 0 { 0 } else { block });
            c.bus.write32(block, if seed % 6 == 1 { 0 } else { imp });
            c.bus.write32(imp, imp_vt);
            c.bus.write32(imp + 0x10, 0);
            slot(c, imp_vt, 0x60, delta(&mut r), FAKE);
            slot(c, imp_vt, 0x68, delta(&mut r), FAKE + 0x10);
            slot(c, imp_vt, 0x70, 0x10, callee::GET_TRANSFORM_RADIUS.addr);
            slot(c, imp_vt, 0xc0, delta(&mut r), FAKE + 0x20);
            slot(c, this_vt, 0xc0, delta(&mut r), FAKE + 0x30);
            // World_GetNash: *(gp - 0x848) -> +0x84 -> +0x20.
            c.bus.write32(GAME_GP - 0x848, app);
            c.bus.write32(app + 0x84, game);
            c.bus.write32(game + 0x20, r.next_u32());
            let p = [r.f32(1000.0), r.f32(1000.0), r.f32(1000.0), ONE];
            put_vec(c, old, p);
            put_vec(c, old + 0x10, p);
            put_vec(c, pos + 0x30, p);
            c.bus.write32(LOG_RESULTS, old);
            c.bus.write32(LOG_RESULTS + 4, pos);
            c.bus.write32(LOG_RESULTS + 8, pos);
        });
    }


    /// `ModObj_Render`: random layer / pass switches, flags and hidden
    /// word; modifier block parts present or not; the object's and the
    /// camera's methods fakes or the real `return 0`; fake answers 1, 7 or
    /// a pointer; `World_IsOccluded` with no occluder or one whose VU0 test
    /// leaves VI1 as set here (0 or 1: occluded); UV scroll through the
    /// real `UVScrollModifier_GetMatrix`.
    #[test]
    fn mod_obj_render_matches() {
        covered(ssx3_ee::s11::f_00356298__generated, mod_obj_render, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3562_9800, 8);
            let this = DATA;
            let (model, this_vt) = (DATA + 0x1000, DATA + 0x2000);
            let (block, imp, imp_vt, uv) = (DATA + 0x3000, DATA + 0x3100, DATA + 0x3180, DATA + 0x3200);
            let (gfx, cam_vt, layers, table) = (DATA + 0x4000, DATA + 0x4800, DATA + 0x5000, DATA + 0x6000);
            let (app, game, occl, occluder) = (DATA + 0x7000, DATA + 0x7100, DATA + 0x7200, DATA + 0x7400);
            fill(c, &mut r, this, 0x10);
            fill(c, &mut r, model, 0x40);
            c.g[A0][0] = sx(this);
            // Layer and pass switches.
            c.bus.write32(GAME_GP + LAYER_TABLE, layers);
            c.bus.write32(layers, table);
            let layer = if r.next_u32() % 3 == 0 { 0xff } else { r.next_u32() % 4 };
            c.bus.write8(model + 0x7d, layer as u8);
            c.bus.write16(model + 0x7e, (r.next_u32() % 4) as u16);
            for i in 0..4 {
                let layer_mode = if r.next_u32() % 4 == 0 { r.next_u32() } else { 6 };
                c.bus.write32(table + i * 8 + 0x24, layer_mode);
                let pass_mode = if r.next_u32() % 4 == 0 { r.next_u32() } else { 3 };
                c.bus.write32(table + i * 0x18 + 0x3f0, pass_mode);
            }
            let flags = r.next_u32();
            c.bus.write32(model + 8, if r.next_u32() % 6 == 0 { flags & !4 } else { flags | 4 });
            let hidden = if r.next_u32() % 6 == 0 { r.next_u32() | 1 } else { 0 };
            c.bus.write32(this + 0x28, hidden);
            // The modifier block: implementation, UV scroll, +8 part.
            c.bus.write32(this + 0xc, this_vt);
            c.bus.write32(this + 0x18, model);
            c.bus.write32(this + 0x1c, if r.next_u32() % 4 == 0 { 0 } else { block });
            c.bus.write32(block + 0x10, 0);
            c.bus.write32(block, if r.next_u32() % 3 == 0 { 0 } else { imp });
            c.bus.write32(block + 4, if r.next_u32() % 3 == 0 { 0 } else { uv });
            let part = r.next_u32();
            c.bus.write32(block + 8, if r.next_u32() % 3 == 0 { 0 } else { part });
            c.bus.write32(imp, imp_vt);
            slot(c, imp_vt, 0x58, delta(&mut r), FAKE);
            slot(c, imp_vt, 0xd0, delta(&mut r), FAKE + 0x10);
            fill(c, &mut r, uv, 0x10);
            if r.next_u32() % 2 == 0 {
                c.bus.write32(uv + 0x10, 0);
            }
            // The object's methods.
            let zero_or = |r: &mut Rng, fake: u32| if r.next_u32() % 2 == 0 { RETURN_ZERO } else { fake };
            let (d, f) = (delta(&mut r), zero_or(&mut r, FAKE + 0x20));
            slot(c, this_vt, 0x160, d, f);
            slot(c, this_vt, 0x168, delta(&mut r), FAKE + 0x30);
            let (d, f) = (delta(&mut r), zero_or(&mut r, FAKE + 0x40));
            slot(c, this_vt, 0xd0, d, f);
            slot(c, this_vt, 0xd8, delta(&mut r), FAKE + 0x50);
            slot(c, this_vt, 0xc0, delta(&mut r), FAKE + 0x60);
            // The graphics manager's camera methods.
            c.bus.write32(GAME_GP + GFX_MAN, gfx);
            c.bus.write32(gfx + 0x10d8, cam_vt);
            slot(c, cam_vt, 0x158, delta(&mut r), FAKE + 0x70);
            let (d, f) = (delta(&mut r), zero_or(&mut r, FAKE + 0x80));
            slot(c, cam_vt, 0x2e8, d, f);
            slot(c, cam_vt, 0x2f8, delta(&mut r), FAKE + 0x90);
            // World_IsOccluded: *(gp - 0x848) -> +0x84 -> +0x8c occluders.
            c.bus.write32(GAME_GP - 0x848, app);
            c.bus.write32(app + 0x84, game);
            c.bus.write32(game + 0x8c, occl);
            c.bus.write32(occl + 0x10c, r.next_u32() % 2);
            c.bus.write32(occl + 0x110, 1);
            c.bus.write32(occl + 8, occluder);
            fill(c, &mut r, occluder + 0x50, 0x14);
            c.bus.plat.write_ctrl(1, r.next_u32() % 2);
            // Fake answers: a pointer, 1 (hidden) or 7 (UV-scroll block).
            for n in 0..8 {
                c.bus.write32(LOG_RESULTS + 4 * n, [0, 1, 7][(r.next_u32() % 3) as usize]);
            }
        });
    }


    #[test]
    fn mod_obj_v20_matches() {
        covered(ee::s11::f_00356bf0__generated, mod_obj_v20, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x35_6bf0, 1);
            let block = DATA;
            let h = DATA + 0x3100;
            let obj = DATA + 0x3200;
            slot_2(c, 0xd0, FAKE);
            c.bus.write32(obj, vt());
            if seed % 4 == 0 {
                c.bus.write32(block + 0x1c, 0);
            } else {
                c.bus.write32(block + 0x1c, h);
                if seed % 5 == 0 {
                    c.bus.write32(h, 0);
                } else {
                    c.bus.write32(h, obj);
                    // Force a "type 2" answer for half of the calls.
                    c.bus.write32(
                        crate::difftest::fakes::LOG_RESULTS,
                        if seed % 2 == 0 { 2 } else { 0 },
                    );
                }
            }
            c.g[A0][0] = sx(block);
        });
    }


    #[test]
    fn mod_obj_v44_matches() {
        covered(ee::s11::f_003569d0__generated, mod_obj_v44, 200, &|c, seed| {
            let _r = machine(c, seed, 0x3569, 1);
            let this = DATA;
            c.g[A0][0] = sx(this);
            let block = DATA + 0x100;
            c.bus.write32(this + 0x1c, if seed % 3 == 0 { 0 } else { block });
            if seed % 3 == 1 {
                c.bus.write32(block, 0);
            } else if seed % 3 == 2 {
                let obj = DATA + 0x200;
                let vt = DATA + 0x300;
                c.bus.write32(block, obj);
                c.bus.write32(obj, vt);
                c.bus.write16(vt + 0x60, 0);
                c.bus.write32(vt + 0x64, FAKE);
            }
        });
    }


    #[test]
    fn mod_obj_v24_matches() {
        covered(ee::s11::f_00356078__generated, mod_obj_v24, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3560, 1);
            let this = DATA;
            c.g[A0][0] = sx(this);
            let head = DATA + 0x100;
            c.bus.write32(this + 0x18, r.next_u32());
            match seed % 3 {
                0 => c.bus.write32(this + 0x1c, 0),
                1 => {
                    c.bus.write32(this + 0x1c, head);
                    c.bus.write32(head, 0);
                }
                _ => {
                    let child = DATA + 0x200;
                    let vt = DATA + 0x300;
                    c.bus.write32(this + 0x1c, head);
                    c.bus.write32(head, child);
                    c.bus.write32(child, vt);
                    c.bus.write16(vt + 0x90, (r.next_u32() % 128) as u16);
                    c.bus.write32(vt + 0x94, FAKE);
                }
            }
        });
    }


    #[test]
    fn particle_update_matches() {
        check(ssx3_ee::s11::f_00357950__generated, particle_update, 150, &|c, seed| {
            machine(c, seed, 0x3579_50, 3);
            // Every virtual call is a fake: the first two results are forced
            // to 0, which takes the base update's short paths.
            c.bus.write32(LOG_RESULTS, 0);
            c.bus.write32(LOG_RESULTS + 4, 0);
            let obj = DATA;
            c.g[A0][0] = sx(obj);
            c.bus.write32(obj + 0xc, DATA + 0x200);
            for off in [0x08u32, 0x78, 0xa0, 0xb8] {
                c.bus.write16(DATA + 0x200 + off, 0);
                c.bus.write32(DATA + 0x200 + off + 4, FAKE);
            }
            c.bus.write32(obj + 0x20, 0);
            match seed % 3 {
                0 => c.bus.write32(obj + 0x1c, 0),
                1 => {
                    c.bus.write32(obj + 0x1c, DATA + 0x400);
                    c.bus.write32(DATA + 0x410, 1);
                }
                _ => {
                    c.bus.write32(obj + 0x1c, DATA + 0x400);
                    c.bus.write32(DATA + 0x410, 0);
                }
            }
        });
    }


    #[test]
    fn mod_obj_v45_matches() {
        covered(ssx3_ee::s11::f_00356a00__generated, mod_obj_v45, 150, &|c, seed| {
            let mut r = machine(c, seed, 73, 1);
            let movenode = DATA + 0x100;
            if seed % 3 == 0 {
                c.bus.write32(movenode + 0x1c, 0);
            } else {
                let node = DATA + 0x400;
                let vt = vtable(c, &mut r, &[0x60]);
                c.bus.write32(node, vt);
                c.bus.write32(movenode + 0x1c, node);
            }
            c.g[A0][0] = sx(movenode);
        });
    }


    // --------------------------------------------------------------- ModObj_v23

    #[test]
    fn mod_obj_v23_matches() {
        fakes::covered(ee::s11::f_00356d48__generated, mod_obj_v23, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x0e50_00c3, 1);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            let block = if seed % 3 == 0 { 0 } else { DATA + 0x2000 };
            c.bus.write32(this + 0x1c, block);
            let node = if seed % 4 == 0 { 0 } else { DATA + 0x3000 };
            c.bus.write32(DATA + 0x2000, node);
            let vt = DATA + 0x4000;
            c.bus.write32(DATA + 0x3000, vt);
            c.bus.write16(vt + 0xd0, 0x10);
            c.bus.write32(vt + 0xd4, FAKE);
            c.bus.write32(DATA + 0x3034, r.next_u32());
            // Half the calls report mode 2, the rest some other mode.
            let v = if seed % 2 == 0 { 2 } else { 0 };
            c.bus.write32(LOG_RESULTS, v);
        });
    }


    #[test]
    fn mod_obj_v43_matches() {
        check(s11::f_00356ab8__generated, mod_obj_v43, 120, &|c, seed| {
            machine(c, seed, 0x356ab8, 1);
            clear(c, OBJ, 0x40);
            clear(c, CHILD, 0x100);
            c.g[A0][0] = sx(OBJ);
            let block = if seed % 3 == 0 { 0 } else { CHILD };
            c.bus.write32(OBJ + 0x1c, block);
        });
    }


    #[test]
    fn mod_obj_v42_matches() {
        check(s11::f_00356ae0__generated, mod_obj_v42, 120, &|c, seed| {
            machine(c, seed, 0x356ae0, 1);
            clear(c, OBJ, 0x40);
            clear(c, CHILD, 0x100);
            c.g[A0][0] = sx(OBJ);
            let block = if seed % 3 == 0 { 0 } else { CHILD };
            c.bus.write32(OBJ + 0x1c, block);
        });
    }


    #[test]
    fn mod_obj_v36_matches() {
        check(s11::f_00356b08__generated, mod_obj_v36, 120, &|c, seed| {
            machine(c, seed, 0x356b08, 1);
            c.g[A1][0] = 0;
            clear(c, OBJ, 0x40);
            clear(c, CHILD, 0x100);
            c.g[A0][0] = sx(OBJ);
            let block = if seed % 3 == 0 { 0 } else { CHILD };
            c.bus.write32(OBJ + 0x1c, block);
        });
    }


    #[test]
    fn object_ctor_2_matches() {
        check(ee::s11::f_00356e60__generated, object_ctor_2, 80, &|c, seed| {
            ctor_setup(c, seed, 0x356e_60, 0x10, DATA + 0x300);
        });
    }


    #[test]
    fn particle_ctor_matches() {
        check(ee::s11::f_00357918__generated, particle_ctor, 80, &|c, seed| {
            ctor_setup(c, seed, 0x3579_18, 0x10, DATA + 0x300);
        });
    }


    #[test]
    fn multi_particle_man_ctor_matches() {
        check(ee::s11::f_003579c8__generated, multi_particle_man_ctor, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x3579_c8, 0);
            c.g[A0][0] = sx(DATA + 0x100);
            c.bus.write32(GAME_GP + 0xf38, r.next_u32());
            fill(c, &mut r, DATA + 0x100, 0x10);
        });
    }


    // -----------------------------------------------------------------------
    // 0x355420 ModObj_v14 / 0x356a70 ModObj_v39.

    #[test]
    fn mod_obj_v14_matches() {
        check(ee::s11::f_00355420__generated, mod_obj_v14, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x3554_20, 1);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            match seed % 3 {
                0 => c.bus.write32(this + 0x1c, 0),
                1 => c.bus.write32(this + 0x1c, DATA + 0x200), // block, *block == 0
                _ => {
                    let node = DATA + 0x300;
                    c.bus.write32(this + 0x1c, DATA + 0x200);
                    c.bus.write32(DATA + 0x200, node);
                    let vt = vtable(c, &mut r, &[0x40]);
                    c.bus.write32(node, vt);
                }
            }
        });
    }


    #[test]
    fn mod_obj_v39_matches() {
        check(ee::s11::f_00356a70__generated, mod_obj_v39, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x356a_70, 1);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            match seed % 3 {
                0 => c.bus.write32(this + 0x1c, 0),
                1 => c.bus.write32(this + 0x1c, DATA + 0x200),
                _ => {
                    let node = DATA + 0x300;
                    c.bus.write32(this + 0x1c, DATA + 0x200);
                    c.bus.write32(DATA + 0x200, node);
                    let vt = vtable(c, &mut r, &[0xa8]);
                    c.bus.write32(node, vt);
                }
            }
        });
    }


    #[test]
    fn mod_obj_set_rail_modifier_matches() {
        check(
            ee::s11::f_003556a8__generated,
            mod_obj_set_rail_modifier,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x3556_a8, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_2(c, this, 0x60);
                c.bus.write32(this + 0x1c, if seed % 2 == 0 { 0 } else { DATA + 0x800 });
                clear_2(c, DATA + 0x800, 0x60);
                clear_2(c, DATA + 0x900, 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x900);
            },
        );
    }


    /// The modifier block of a `ModObj`: `+0x1c` is the block, whose slots
    /// `+8` (tex flip) and `+0xc` (boost) are the installed modifiers.
    pub(crate) fn mod_obj(c: &mut Cpu, seed: u32) -> u32 {
        let this = DATA;
        let block = DATA + 0x200;
        match seed % 3 {
            0 => c.bus.write32(this + 0x1c, 0),
            1 => {
                c.bus.write32(this + 0x1c, block);
                c.bus.write32(block + 8, 0);
                c.bus.write32(block + 0xc, 0);
            }
            _ => {
                // A live tex-flip modifier to free, and a boost modifier.
                let flip = alloc(c, 0x20);
                let boost = alloc(c, 0x20);
                c.bus.write32(this + 0x1c, block);
                c.bus.write32(block + 8, flip);
                c.bus.write32(block + 0xc, boost);
            }
        }
        this
    }


    #[test]
    fn mod_obj_set_tex_flip_modifier_matches() {
        check(
            ssx3_ee::s11::f_003555a8__generated,
            mod_obj_set_tex_flip_modifier,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x555a8, 0);
                heap_init(c);
                let this = mod_obj(c, seed);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
            },
        );
    }


    #[test]
    fn mod_obj_set_boost_modifier_matches() {
        check(
            ssx3_ee::s11::f_00355600__generated,
            mod_obj_set_boost_modifier,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x55600, 0);
                heap_init(c);
                let this = mod_obj(c, seed);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
            },
        );
    }


    #[test]
    fn multi_particle_man_clear_matches() {
        check(
            ssx3_ee::s11::f_00357b38__generated,
            multi_particle_man_clear,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x57b38, 0);
                heap_init(c);
                let list = DATA;
                for i in 0..4u32 {
                    let p = if (seed >> i) & 1 != 0 {
                        alloc(c, 0x80)
                    } else {
                        0
                    };
                    c.bus.write32(list + 4 * i, p);
                }
                c.g[A0][0] = list as u64;
            },
        );
    }


    #[test]
    fn mod_obj_add_ava_spline_modifier_matches() {
        check(ee::s11::f_00355a78__generated, mod_obj_add_ava_spline_modifier, 20, &|c, seed| {
            let _ = common(c, seed, 0x355a78, 3);
            alloc_ready_2(c);
            let this = DATA + 0x100;
            clear_3(c, this, 0x200);
            c.bus.write32(this + 0x18, DATA + 0x2000);
            c.bus.write32(gp_at(0x9d4), if seed % 3 == 0 { 1 } else { 0 });
            c.g[A0][0] = sx(this);
        });
    }


    #[test]
    fn position_modifier_ctor_matches() {
        check(ee::s11::f_00356eb8__generated, position_modifier_ctor, SEEDS_3, &|c, seed| {
            let mut r = base(c, seed, 0x356eb8);
            let this = DATA + 0x100;
            let pos = DATA + 0x200;
            clear_3(c, this, 0x100);
            put_words(c, pos, &[r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()]);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(pos);
        });
    }


    #[test]
    fn parent_modifier_ctor_matches() {
        check(ee::s11::f_00357038__generated, parent_modifier_ctor, SEEDS_3, &|c, seed| {
            let mut r = base(c, seed, 0x357038);
            let this = DATA + 0x100;
            let pos = DATA + 0x200;
            clear_3(c, this, 0x100);
            put_words(c, pos, &[r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()]);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = sx(pos);
        });
    }


    // ------------------------------------------------------------------
    // Move-node modifiers.

    #[test]
    fn mod_obj_v41_matches() {
        check(ee::s11::f_00355858__generated, mod_obj_v41, 200, &|c, seed| {
            let mut r = setup(c, seed, 4);
            gpf(c, 0x2a74, AUX);
            put(c, AUX + 0x10, r.next_u32() % 1000);
            c.g[A0][0] = (AUX + 0x200) as u64;
            c.f[12] = r.edge_f32();
        });
    }


    #[test]
    fn position_modifier_update_transform_matches() {
        check(ee::s11::f_00356ff0__generated, position_modifier_update_transform, 200, &|c, seed| {
            let mut r = setup(c, seed, 5);
            c.g[A0][0] = AUX as u64;
            fillb(c, &mut r, AUX + 0x30, 0x20);
            put(c, AUX + 0x44, r.next_u32());
            fillb(c, &mut r, AUX + 0x80, 0x10);
        });
    }


    #[test]
    fn magnet_modifier_update_transform_matches() {
        check(ee::s11::f_00357528__generated, magnet_modifier_update_transform, 200, &|c, seed| {
            let mut r = setup(c, seed, 6);
            c.g[A0][0] = AUX as u64;
            fillb(c, &mut r, AUX + 0x10, 0x10);
            put(c, AUX + 0x50, r.next_u32());
            fillb(c, &mut r, AUX + 0x90, 0x10);
        });
    }


    // ------------------------------------------------------- objects and audio

    #[test]
    fn mod_obj_add_position_modifier_matches() {
        check(ee::s11::f_003558b8__generated, mod_obj_add_position_modifier, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x0035_58b8, 3);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            pool_allocator(c);
            // keep the modifier constructor on its short path
            for i in 0..0x20u32 {
                w(c, DATA + 0x200 + 4 * i, r.next_u32() & 1);
            }
        });
    }

    pub(crate) const OBJ4: u32 = DATA + 0x4000;


    // ------------------------------------------------------------- objects

    #[test]
    fn mod_obj_add_tex_flip_modifier_matches() {
        check(
            ssx3_ee::s11::f_00355bf0__generated,
            mod_obj_add_tex_flip_modifier,
            40,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x355bf0, 0);
                pool_allocator_2(c);
                fill(c, &mut r, OBJ2, 0x40);
                w_2(c, OBJ2 + 0x4, r.next_u32());
                w_2(c, OBJ2 + 0x8, r.next_u32());
                w_2(c, OBJ2 + 0xc, r.next_u32());
                w_2(c, OBJ2 + 0x10, 1 + (seed % 8));
                w_2(c, OBJ2 + 0x14, r.next_u32());
                w_2(c, OBJ_2 + 0x18, OBJ3);
                w_2(c, OBJ_2 + 0x1c, if seed % 2 == 0 { 0 } else { OBJ4 });
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = OBJ2 as u64;
            },
        );
    }


    #[test]
    fn mod_obj_add_boost_modifier_matches() {
        check(
            ssx3_ee::s11::f_00355c50__generated,
            mod_obj_add_boost_modifier,
            40,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x355c50, 0);
                pool_allocator_2(c);
                fill(c, &mut r, OBJ2, 0x40);
                w_2(c, OBJ_2 + 0x18, OBJ3);
                w_2(c, OBJ_2 + 0x1c, if seed % 2 == 0 { 0 } else { OBJ4 });
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = OBJ2 as u64;
            },
        );
    }


    #[test]
    fn ava_spline_modifier_save_matches() {
        check(
            ssx3_ee::s11::f_00357848__generated,
            ava_spline_modifier_save,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x357848, 0);
                w_2(c, OBJ_2, VT);
                slot_3(c, VT, 0x28, 0);
                slot_3(c, VT, 0x8, 0);
                w_2(c, OBJ_2 + 0x80, seed);
                w_2(c, OBJ2 + 0x80, seed.wrapping_mul(3));
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = OBJ2 as u64;
            },
        );
    }


    #[test]
    fn parent_modifier_v22_matches() {
        check(ee::s11::f_00357210__generated, parent_modifier_v22, 12, &|c, seed| {
            let mut r = machine(c, seed, 0x0035_7210, 1);
            be_library(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x3000) as u64;
            c.bus.write32(DATA + 0x90, r.next_u32());
            c.bus.write32(DATA + 0x3000 + 0x5c, r.next_u32());
            match seed % 3 {
                0 => c.bus.write32(DATA + 0xa0, 0),
                1 => {
                    c.bus.write32(DATA + 0xa0, DATA + 0x1000);
                    c.bus.write32(DATA + 0x1000 + 0xc, 0);
                }
                _ => {
                    c.bus.write32(DATA + 0xa0, DATA + 0x1000);
                    c.bus.write32(DATA + 0x1000 + 0xc, DATA + 0x2000);
                    c.bus.write32(DATA + 0x2000 + 0xc, DATA + 0x2100);
                    c.bus.write16(DATA + 0x2100 + 0x150, 0);
                    c.bus.write32(DATA + 0x2100 + 0x154, FAKE);
                }
            }
        });
    }


    #[test]
    fn parent_modifier_save_matches() {
        check(ee::s11::f_00357278__generated, parent_modifier_save, 8, &|c, seed| {
            let mut r = machine(c, seed, 0x0035_7278, 2);
            be_library(c);
            world(c);
            pool(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x1000) as u64;
            c.bus.write32(DATA + 0xa0, DATA + 0x3000);
            c.bus.write32(DATA + 0x1000, DATA + 0x2000);
            c.bus.write16(DATA + 0x2000 + 8, 0);
            c.bus.write32(DATA + 0x2000 + 0xc, FAKE);
            c.bus.write16(DATA + 0x2000 + 0x28, 0);
            c.bus.write32(DATA + 0x2000 + 0x2c, FAKE);
            c.bus.write32(DATA + 0x90, r.next_u32());
        });
    }


    #[test]
    fn multi_particle_man_on_section_unload_matches() {
        check(
            ee::s11::f_00357b90__generated,
            multi_particle_man_on_section_unload,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_7b90, 2);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                for i in 0..4u32 {
                    let p_3 = DATA + 0x1000 * (i + 1);
                    let v = if (seed >> i) & 1 != 0 { p_3 } else { 0 };
                    c.bus.write32(DATA + 4 * i, v);
                    c.bus.write32(p_3 + 4, 0);
                }
            },
        );
    }


    #[test]
    fn mod_obj_v28_matches() {
        check(ee::s11::f_003560c0__generated, mod_obj_v28, 30, &|c, seed| {
            fakes::machine(c, seed, 0x3560c0, 0);
            globals(c);
            c.g[A0][0] = OBJ_3 as u64;
            if seed % 3 == 0 {
                w_3(c, OBJ_3 + 0x1c, 0);
            } else {
                w_3(c, OBJ_3 + 0x1c, OBJ2);
                w_3(c, OBJ2, OBJ3);
                w_3(c, OBJ3, VT);
                slot_4(
                    c,
                    VT,
                    0xb8,
                    0,
                    if seed % 2 == 0 { ONE_STUB } else { ZERO_STUB },
                );
            }
            w_3(c, OBJ_3 + 0xc, OBJ4);
            w_3(c, OBJ4, VT + 0x100);
            slot_4(c, VT + 0x100, 0xd8, 0, fakes::FAKE);
        });
    }


    #[test]
    fn magnet_modifier_v09_matches() {
        check(ee::s11::f_00357660__generated, magnet_modifier_v09, 8, &|c, seed| {
            let mut r = machine(c, seed, 0x0035_7660, 2);
            globals_2(c);
            c.g[A0][0] = OBJ as u64;
            let id = (seed % 6) as u64;
            c.g[A1][0] = id;
            w(
                c,
                OBJ + 0x44,
                if seed % 3 == 0 {
                    0xffff_ffff
                } else {
                    (seed % 6) as u32
                },
            );
            w(c, OBJ + 0x54, r.next_u32());
            // A rider for every id: world -> +0x84 -> +0xc -> +0x28 + id*4.
            let race = WORLD + 0x2000;
            let rider = DATA + 0x3000;
            w(c, race + 0x28 + 4 * (id as u32), rider);
            let vt = vtable(c, &mut r, &[0x40]);
            w(c, rider + 0x6c0, vt);
        });
    }


    #[test]
    fn multi_particle_man_create_slot_matches() {
        check(
            ssx3_ee::s11::f_00357a78__generated,
            multi_particle_man_create_slot,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x357a78, 0);
                pool_2(c);
                let man = DATA + 0x1000;
                c.g[A0][0] = man as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = 0;
                // The ctor's first field (its 2nd argument) is an element
                // count: keep it tiny or the emitter free-list walk runs away.
                c.g[A3][0] = (seed % 2) as u64;
                for i in 0..8u32 {
                    w32(c, man + 4 * i, if (seed + i) % 3 == 0 { 0 } else { DATA + 0x3000 + i * 0x100 });
                }
                let _ = &mut r;
            },
        );
    }


    #[test]
    fn parent_modifier_ctor_2_matches() {
        check(ee::s11::f_00357090__generated, parent_modifier_ctor_2, SEEDS_5, &|c, seed| {
            machine(c, seed, 0x35_7090, 3);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (DATA + 0x2200) as u64;
            w(c, DATA + 0x2200, DATA + 0x2400);
            slot_6(c, DATA + 0x2400, 0x10, 0, FAKE);
            slot_6(c, DATA + 0x2400, 0x18, 0, FAKE);
            c.pc = 0x0035_7090;
        });
    }


        #[test]
    fn magnet_modifier_ctor_matches() {
        check(ee::s11::f_003572e0__generated, magnet_modifier_ctor, SEEDS_6, &|c, seed| {
            let mut r = machine(c, seed, 0x3572e0, 0);
            be_library_2(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            c.g[A2][0] = (DATA + 0x1000) as u64;
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
            for i in 0..16u32 {
                w(c, DATA + 0x1000 + 4 * i, r.next_u32());
            }
        });
    }


    #[test]
    fn multi_particle_man_dtor_matches() {
        check(ee::s11::f_00357a00__generated, multi_particle_man_dtor, SEEDS_6, &|c, seed| {
            machine(c, seed, 0x357a00, 1);
            be_library_2(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 2) as u64;
            for i in 0..4u32 {
                let p = DATA + 0x1000 + 0x100 * i;
                let present = (seed >> i) & 1 != 0;
                w(c, obj + 4 * i, if present { p } else { 0 });
                if present {
                    w(c, p, 0);
                }
            }
        });
    }

    pub(crate) const RET_TWO: u32 = 0x0014_5854;


    // -------------------------------------------------------------- modifiers

    #[test]
    fn mod_obj_add_parent_modifier_matches() {
        check(
            ee::s11::f_00355978__generated,
            mod_obj_add_parent_modifier,
            8,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x3559_78, 6);
                let self_ = DATA;
                let vtable = DATA + 0x1200;
                w(c, self_ + 0xc, 0);
                w(c, self_, vtable);
                ret_slot(c, vtable, 0x188, RET_ZERO);
                ret_slot(c, vtable, 0x180, RET_ZERO);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x1400) as u64;
                c.g[A3][0] = (DATA + 0x1800) as u64;
                let _ = r.next_u32();
            },
        );
    }


    #[test]
    fn mod_obj_add_roller_modifier_matches() {
        check(
            ee::s11::f_00355db8__generated,
            mod_obj_add_roller_modifier,
            8,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x355d_b8, 6);
                let self_ = DATA;
                let vtable = DATA + 0x1200;
                w(c, self_ + 0xc, 0);
                w(c, self_, vtable);
                ret_slot(c, vtable, 0x188, RET_ZERO);
                ret_slot(c, vtable, 0x180, RET_ZERO);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x1400) as u64;
                c.g[A3][0] = (DATA + 0x1800) as u64;
                let _ = r.next_u32();
            },
        );
    }


    #[test]
    fn mod_obj_v21_matches() {
        check(ee::s11::f_00356c48__generated, mod_obj_v21, 16, &|c, seed| {
            let _ = base_2(c, seed, 0x356c_48, 4);
            let self_ = DATA;
            let obj = DATA + 0x1000;
            let inner = DATA + 0x1100;
            let vtable = DATA + 0x1200;
            match seed % 4 {
                0 => {
                    w(c, self_ + 0x1c, 0);
                }
                1 => {
                    w(c, self_ + 0x1c, obj);
                    w(c, obj, 0);
                }
                2 => {
                    w(c, self_ + 0x1c, obj);
                    w(c, obj, inner);
                    w(c, inner, vtable);
                    ret_slot(c, vtable, 0xd0, RET_ONE);
                }
                _ => {
                    w(c, self_ + 0x1c, obj);
                    w(c, obj, inner);
                    w(c, inner, vtable);
                    ret_slot(c, vtable, 0xd0, RET_TWO);
                }
            }
            c.g[A0][0] = self_ as u64;
            c.g[A1][0] = (DATA + 0x1800) as u64;
        });
    }


    #[test]
    fn magnet_modifier_save_matches() {
        check(ee::s11::f_003576d0__generated, magnet_modifier_save, 12, &|c, seed| {
            let _ = base_2(c, seed, 0x3576_d0, 4);
            let self_ = DATA;
            let stream = DATA + 0x1000;
            let vtable = DATA + 0x1200;
            let nested = DATA + 0x1400;
            w(c, self_ + 0xa0, 0);
            w(c, stream, vtable);
            w(c, nested, vtable);
            ret_slot(c, vtable, 0x28, RET_ZERO);
            ret_slot(c, vtable, 0x8, RET_ZERO);
            ret_slot(c, vtable, 0xc, RET_ZERO);
            let _ = nested;
            c.g[A0][0] = self_ as u64;
            c.g[A1][0] = stream as u64;
        });
    }


    #[test]
    fn multi_particle_dtor_matches() {
        check(ee::s11::f_00357fa0__generated, multi_particle_dtor, 12, &|c, seed| {
            let _ = alloc_base(c, seed, 0x357f_a0, 4);
            let self_ = DATA;
            let a8 = if seed % 3 == 0 { 0 } else { alloc_2(c, 0x40) };
            let ac = if seed % 2 == 0 { 0 } else { alloc_2(c, 0x40) };
            w(c, self_ + 0x8, a8);
            w(c, self_ + 0xc, ac);
            let owner = if seed % 4 == 0 { 0 } else { DATA + 0x1000 };
            w(c, self_ + 0x10, owner);
            if owner != 0 {
                w(c, owner + 0x18c, DATA + 0x1200);
                ret_slot(c, DATA + 0x1200, 0x8, RET_ZERO);
            }
            c.g[A0][0] = self_ as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }


    #[test]
    fn mod_obj_add_magnet_modifier_matches() {
        check(
            ee::s11::f_003559f8__generated,
            mod_obj_add_magnet_modifier,
            60,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x3559, 3);
                let o = DATA;
                w(c, o + 0x18, seed);
                fill(c, &mut r, DATA + 0x200, 0x40);
                fill(c, &mut r, DATA + 0x300, 0x40);
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                c.g[A0][0] = sx(o);
                c.g[A1][0] = sx(DATA + 0x200);
                c.g[A2][0] = sx(DATA + 0x300);
            },
        );
    }


    #[test]
    fn magnet_modifier_ctor_2_matches() {
        check(ssx3_ee::s11::f_00357358__generated, magnet_modifier_ctor_2, 10, &|c, seed| {
            let mut r = base_4(c, seed, 0x357358, 2);
            let this = DATA + 0x1000;
            let stream = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = stream as u64;
            w_4(c, stream, DATA + 0x3000);
            for o in [0x10u32, 0x18] {
                c.bus.write16(DATA + 0x3000 + o, 0);
                w_4(c, DATA + 0x3000 + o + 4, 0);
            }
            w_4(c, this + 0x50, r.next_u32());
        });
    }

    pub(crate) const MEM: u32 = DATA + 0x6000;


        #[test]
    fn multi_particle_ctor_matches() {
        check(
            ee::s11::f_00357dd8__generated,
            multi_particle_ctor,
            8,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x357dd8, 4);
                c.g[A0][0] = MEM as u64;
                c.g[A1][0] = SRC as u64;
                c.g[A2][0] = (seed % 8) as u64;
                fill(c, &mut r, SRC, 0x40);
                w(c, 0x0048_ea10, 0);
            },
        );
    }


    #[test]
    fn multiparticle_man_load_matches() {
        check(
            ee::s11::f_00357d28__generated,
            multiparticle_man_load,
            60,
            &|c, seed| {
                let _r = alloc_base_2(c, seed, 0x357d);
                let entries = DATA + 0x500;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                c.bus.write32(DATA + 0x400, entries);
                // the stream entry writes its 4-byte "present" flag at sp+0
                c.bus.write32(crate::difftest::SP - 0x60, 0);
                for i in 0..4 {
                    let e = entries + 8 * i;
                    c.bus.write16(e + 0x10, 0);
                    c.bus.write32(e + 0x14, FAKE + 0x10 * i);
                }
            },
        );
    }


    // --------------------------------------------------------- memory card

        // --------------------------------------------------------- modifier obj

    #[test]
    fn mod_obj_v40_matches() {
        strict(ee::s11::f_00355770__generated, mod_obj_v40, 90, &|c, seed| {
            let mut r = base_7(c, seed, 0x3557, 6);
            allocator(c);
            let holder = DATA + 0x400;
            let obj = DATA + 0x500;
            let vt = DATA + 0x800;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = r.next_u32() as u64;
            w(c, DATA + 0x1c, if seed % 6 == 0 { 0 } else { holder });
            w(c, holder, if seed % 6 == 1 { 0 } else { obj });
            w(c, obj, vt);
            w(c, vt + 0x50, 0);
            w(c, vt + 0x54, FAKE);
            w(c, vt + 0x38, 0);
            w(c, vt + 0x3c, FAKE + 0x20);
            w(c, vt + 0x48, 0);
            w(c, vt + 0x4c, FAKE + 0x10);
            // The holder's slot-9 result selects the "done" path.
            force(
                c,
                0,
                FAKE + 0x10,
                if seed % 4 == 0 { 0 } else { 1 },
                KEEP_F0,
            );
            // The script timer at +0x20 and the global tick object.
            w(c, DATA + 0x20, if seed % 3 == 0 { 0 } else { 1 });
            let tick = DATA + 0x1000;
            w(c, GAME_GP + 0x2a74, tick);
            w(c, tick + 0x10, 0x20);
        });
    }


        #[test]
    fn magnet_modifier_update_matches() {
        same(
            ssx3_ee::s11::f_003573f8__generated,
            magnet_modifier_update,
            200,
            &|c, seed| {
                let mut r = Rng(0x3573f8 ^ seed as u64);
                scramble(c, &mut r);
                let s = DATA + 0x100;
                c.g[A0][0] = s as u64;
                c.bus.write32(s + 0x44, (seed % 8) as u32);
                c.bus.write32(s + 0x48, r.edge_f32());
                c.bus.write32(s + 0x4c, r.edge_f32());
                c.bus.write32(s + 0x50, (seed % 2) as u32);
                c.bus.write32(s + 0x54, (seed % 2) as u32);
                put_vec(
                    c,
                    s + 0x10,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
            },
        );
    }


    pub(crate) const MPART: u32 = DATA + 0x1_0000;

    pub(crate) const MPART_STREAM: u32 = DATA + 0x1_1000;


    #[test]
    fn multi_particle_ctor_2_matches() {
        check_all(
            ee::s11::f_00357e80__generated,
            multi_particle_ctor_2,
            20,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x357e_80, 10);
                c.g[A0][0] = sx(MPART);
                c.g[A1][0] = sx(MPART_STREAM);
                let vt6 = vtable(c, &mut r, &[0x8, 0xc]);
                c.bus.write32(MPART_STREAM, vt6);
                c.bus.write32(MPART_STREAM + 0x4, (seed % 3) as u32);
                let _ = &mut r;
            },
        );
    }
}
