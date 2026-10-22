//! `src/render/graphicsman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::sp;
use crate::call::Flow;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use ssx3_ee_rt::funcs::call_indirect;
pub(crate) use crate::game::visualfx::boardwakefx::p;
pub(crate) use crate::game::visualfx::boardwakefx::gp;
pub(crate) use crate::game::visualfx::boardwakefx::ldf_2 as ldf;
pub(crate) use crate::game::visualfx::boardwakefx::lw;
pub(crate) use crate::game::visualfx::boardwakefx::lh;
pub(crate) use crate::game::visualfx::boardwakefx::copy_render_state;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::fe::festateload::at as at_2;
pub(crate) use crate::game::object::railmodifier::DELETE;
pub(crate) use crate::game::object::railmodifier::reset_vtable_and_delete;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw as lw_2;
pub(crate) use crate::game::be::beintplayer::lh as lh_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::ofs;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::gpa;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::fe::festaterewards::r;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0036_9a78, ps2_graphics_man_match_blended_matrix, "PS2GraphicsMan_MatchBlendedMatrix"),
    (0x0036_9c28, ps2_graphics_man_wait_for_render_idle, "PS2GraphicsMan_WaitForRenderIdle"),
    (0x0036_9ff8, graphics_man_snapshot_texture_table, "GraphicsMan_SnapshotTextureTable"),
    (0x0036_a020, graphics_man_free_all_textures, "GraphicsMan_FreeAllTextures"),
    (0x0036_a1c0, deferred_delete_push, "DeferredDelete_Push"),
    (0x0036_a2c0, ps2_graphics_man_v94, "PS2GraphicsMan_v94"),
    (0x0036_aa60, ps2_graphics_man_emit_depth_fog_clut_upload, "PS2GraphicsMan_EmitDepthFogClutUpload"),
    (0x0036_aba0, ps2_graphics_man_reset_depth_fog_params, "PS2GraphicsMan_ResetDepthFogParams"),
    (0x0036_ce00, emitter_set_desc_curve_a, "Emitter_SetDescCurveA"),
    (0x0036_ce28, emitter_set_desc_curve_b, "Emitter_SetDescCurveB"),
    (0x0036_cef8, emitter_set_desc_path_coeffs, "Emitter_SetDescPathCoeffs"),
    (0x0036_d008, emitter_reset_desc_accel, "Emitter_ResetDescAccel"),
    (0x0036_d1f0, emitter_set_desc_path_coeffs_scaled, "Emitter_SetDescPathCoeffsScaled"),
    (0x0036_d3d8, emitter_set_desc_time, "Emitter_SetDescTime"),
    (0x0036_d3e8, emitter_advance_desc_time, "Emitter_AdvanceDescTime"),
    (0x0036_d500, emitter_compute_desc_bounds, "Emitter_ComputeDescBounds"),
    (0x0037_0758, emitter_dtor, "Emitter_dtor"),
    (0x0037_0788, emitter_update, "Emitter_Update"),
    (0x0037_0888, emitter_render_at, "Emitter_RenderAt"),
    (0x0037_08c0, emitter_render, "Emitter_Render"),
    (0x0037_0af8, emitter_load, "Emitter_Load"),
    (0x0037_0c08, dynamic_emitter_dtor, "DynamicEmitter_dtor"),
];

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

// ---------------------------------------------------------------------------
// Direct-call targets (the generated body's `c.pc = addr; Name(c)` calls).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    use super::*;
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
    pub(crate) use crate::game::worldpainter::quadtree::callee::GET_TRANSFORM_BOUNDS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::INTC_REMOVE_HANDLER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EMITTER_RENDER as EMITTER_RENDER_2;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_ON_VIF1_DMA_END;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_MAN_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STRLEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::QSORT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
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
    pub(crate) use crate::game::fe::festaterewards::callee::BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::fe::festaterewards::callee::VFX_PLANTS_GET_SWAY_MATRIX;
}

// ---------------------------------------------------------------------------
// Emitters.

/// Emitter (descriptor) fields used by `Emitter_Update`.
pub(crate) mod emitter {

    /// Set once the emitter has been (re)started / finished this frame.
    pub const STARTED: u32 = 0xc;

    /// The remaining life is at +0; then the progress accumulator and its
    /// limit, and the rate `dt` is scaled by.
    pub const PROGRESS: u32 = 0x184;

    pub const RATE: u32 = 0x188;

    pub const LIMIT: u32 = 0x180;
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

/// Cross-shard call targets (the generator emits `call_indirect` for them).
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
}


/// `void DeferredDelete_Push(void *obj)` (0x36a1c0): append `obj` to the
/// deferred-delete ring at 0x53ab40 indexed by the count at `gp + 0x2a98`,
/// linking it to the previous head.
pub fn deferred_delete_push(c: &mut Cpu) {
    c.cycles += 9;
    const RING: u32 = 0x54_0000 - 0x54c0;
    let count = Addr((c.g[GP][0] as u32).wrapping_add(0x2a98)).lw(c);
    c.g[A1][0] = count;
    c.g[V0][0] = 0x54_0000;
    c.g[V0][0] = sx(0x54_0000u32.wrapping_sub(0x54c0));
    c.g[A1][0] = sx((count as u32) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let slot = c.g[A1][0] as u32;
    c.g[V1][0] = Addr(slot).lw(c);
    Addr(abi::arg(c, 0)).sw(c, c.g[V1][0]);
    Addr(slot).sw(c, c.g[A0][0]);
    debug_assert_eq!(slot, RING.wrapping_add((count as u32).wrapping_shl(2)));
    abi::ret(c);
}


/// `void PS2GraphicsMan_ResetDepthFogParams(PS2GraphicsMan *man, uint index)`
/// (0x36aba0): fill the 0x20-byte depth-fog record for `index` from the
/// `gp + 0x123c` defaults, copying the 0x10-byte table entry at 0x547230 in
/// after the first four words.
pub fn ps2_graphics_man_reset_depth_fog_params(c: &mut Cpu) {
    c.cycles += 23;
    let gp = c.g[GP][0] as u32;
    let f1 = Addr(gp.wrapping_add(0x123c)).read_fpu(c);
    c.f[1] = f1.0;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 5);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c64));
    c.g[V1][0] = 0x50_0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let dst = c.g[A0][0] as u32;
    Addr(dst).write_fpu(c, f1);
    let f0 = Addr(gp.wrapping_add(0x1240)).read_fpu(c);
    c.f[0] = f0.0;
    Addr(dst).off(0x4).write_fpu(c, f0);
    c.g[V0][0] = Addr(gp.wrapping_add(0x1244)).lw(c);
    Addr(dst).off(0x8).sw(c, c.g[V0][0]);
    let f0 = Addr(gp.wrapping_add(0x1248)).read_fpu(c);
    c.f[0] = f0.0;
    Addr(dst).off(0xc).write_fpu(c, f0);
    c.g[A3][0] = sx(0x50_0000u32.wrapping_add(0x4720));
    let src = c.g[A3][0] as u32;
    c.ldl(2, src.wrapping_add(7));
    c.ldr(2, src);
    c.ldl(5, src.wrapping_add(0xf));
    c.ldr(5, src.wrapping_add(8));
    c.sdl(2, dst.wrapping_add(0x17));
    c.sdr(2, dst.wrapping_add(0x10));
    c.sdl(5, dst.wrapping_add(0x1f));
    c.sdr(5, dst.wrapping_add(0x18));
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Emitters

/// `void Emitter_SetDescTime(Emitter *e, float time)` (0x36d3d8): set the
/// current description time to `time * rate` (rate at +0x4c).
pub fn emitter_set_desc_time(c: &mut Cpu) {
    c.cycles += 4;
    let emitter = Addr(abi::arg(c, 0));
    let rate = emitter.off(0x4c).read_fpu(c);
    c.f[0] = rate.0;
    let result = abi::farg(c, 0) * rate;
    c.f[12] = result.0;
    emitter.off(0x8).write_fpu(c, result);
    abi::ret(c);
}


/// `void Emitter_AdvanceDescTime(Emitter *e, float dt)` (0x36d3e8): advance
/// the description time at +0x8 by `dt * rate` (rate at +0x4c).
pub fn emitter_advance_desc_time(c: &mut Cpu) {
    c.cycles += 6;
    let emitter = Addr(abi::arg(c, 0));
    let rate = emitter.off(0x4c).read_fpu(c);
    c.f[1] = rate.0;
    let current = emitter.off(0x8).read_fpu(c);
    c.f[0] = current.0;
    let delta = abi::farg(c, 0) * rate;
    c.f[12] = delta.0;
    let result = current + delta;
    c.f[0] = result.0;
    emitter.off(0x8).write_fpu(c, result);
    abi::ret(c);
}


/// `lh vt+off` (the vtable slot's `this` adjustment) and `lw vt+off+4`
/// (its entry point).
#[inline(always)]
pub(crate) fn vslot(c: &mut Cpu, vt: u32, off: u32) -> (u64, u64) {
    (lh(c, p(vt, off)), lw(c, p(vt, off + 4)))
}


// ---------------------------------------------------------------------------
// Emitter_Render (0x3708c0).

/// `void Emitter_Render(Emitter* e)` (0x3708c0): cull the emitter's billboard
/// (`e+0x160`/`e+0x170`) with the camera pair from slot 43; when visible, push
/// a render state with the emitter's flags, texture
/// `texIds[base + trunc(e+0x184)]`, draw `e+0x10` through slot 82 and pop the
/// state.
pub fn emitter_render(c: &mut Cpu) {
    c.cycles += 29;
    let frame = Frame::open(c, 0xb0);
    frame.save(c, S4, 0x60);
    frame.save(c, S2, 0x80);
    frame.save(c, S3, 0x70);
    frame.save(c, S5, 0x50);
    frame.save(c, S1, 0x90);
    let e = r32(c, A0);
    c.g[S5][0] = c.g[A0][0];
    frame.save_ra(c, 0x40);
    let context = lw(c, gp(c, 0x2a90)) as u32;
    c.g[S4][0] = sx(context);
    c.g[A0][0] = sx(frame.at(c, 0));
    c.g[S3][0] = sx(p(e, 0x160));
    frame.save(c, S0, 0xa0);
    c.g[S2][0] = sx(p(e, 0x170));
    guest(c, |c| {
        'body: {
            let vt = lw(c, p(context, ctx::VTABLE)) as u32;
            c.g[S0][0] = sx(vt);
            let (d93, f93) = vslot(c, vt, 0x2e8);
            c.g[S1][0] = d93;
            c.g[A1][0] = lh(c, p(vt, 0x158));
            let f43 = lw(c, p(vt, 0x15c));
            c.g[V0][0] = f43;
            c.g[S1][0] = sx(context.wrapping_add(d93 as u32));
            let f43 = f43 as u32;
            c.g[A1][0] = sx(context.wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, f43, 0x0037_0910)?;
            c.g[V0][0] = f93;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S2][0];
            let f93 = f93 as u32;
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, f93, 0x0037_0928)?;
            c.g[V1][0] = sx(1);
            let t = c.g[V0][0] == c.g[V1][0];
            frame.restore(c, S0, 0xa0);
            if t {
                break 'body;
            }
            c.cycles += 3;
            let visible = lw(c, p(e, 0xc));
            c.g[V0][0] = visible;
            let t = visible == 0;
            c.g[V0][0] = sx(0x45_0000);
            if t {
                break 'body;
            }
            c.cycles += 82;
            let kind = lw(c, p(e, 0x8));
            c.g[V1][0] = kind;
            c.g[V0][0] = sx(0x45_0000u32.wrapping_sub(0x4be0));
            let top = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(top);
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[A1][0] = sx(!0x3e0u32);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A2][0] = 0xffff_ffff_ffbf_ffff;
            let layer = lw(c, p(r32(c, V1), 0));
            c.g[T1][0] = layer;
            copy_render_state(c, 2, 3, A3, top, top);
            c.g[T5][0] = 0x40_0000;
            c.g[A3][0] = 0xffff_ffff_fe7f_ffff;
            c.g[T2][0] = 0xffff_ffff_ffcf_ffff;
            let top = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V1][0] = sx(top);
            c.g[T4][0] = sx(context);
            c.g[V0][0] = sx(p(top, 0x14));
            c.g[T3][0] = 0xffff_ffff_fff0_0fff;
            c.g[T0][0] = 0x1_4000;
            c.g[T1][0] = sx(r32(c, T1) << 2);
            c.g[V0][0] = lw(c, p(top, 0x1c));
            c.g[T6][0] = 0xffff_ffff_ffff_ff83;
            c.g[T1][0] = c.g[T1][0] & 0x7c;
            c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
            c.g[V0][0] = c.g[V0][0] | 0xe0;
            c.g[A1][0] = sx(p(e, 0x10));
            let v0 = c.g[V0][0];
            Addr(p(top, 0x1c)).sw(c, v0);
            // top++.
            let new_top = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V1][0] = sx(new_top);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(0x14));
            let pushed = c.g[V0][0];
            Addr(p(context, ctx::TOP)).sw(c, pushed);
            // flagsA: (flagsA & ~0x400000) | 0x400000.
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
            c.g[V0][0] = c.g[V0][0] | c.g[T5][0];
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V1][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V1][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
            c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[T6][0];
            c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
            // texture id and draw.
            let _u = ldf(c, 0, p(e, 0x184));
            c.g[V0][0] = lw(c, p(e, 0x4));
            c.cvt_w_s(1, 0);
            c.mfc1(V1, 1);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(a);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[T4][0] = sx(r32(c, T4).wrapping_add(r32(c, V0)));
            let tex = lw(c, p(r32(c, T4), ctx::TEX_IDS));
            c.g[V1][0] = tex;
            c.sh(p(a, 0x10), tex);
            let vt = lw(c, p(context, ctx::VTABLE)) as u32;
            c.g[V0][0] = sx(vt);
            c.g[A0][0] = lh(c, p(vt, 0x290));
            let f = lw(c, p(vt, 0x294));
            c.g[V1][0] = f;
            let f = f as u32;
            c.g[A0][0] = sx(context.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, f, 0x0037_0a78)?;
            let top = lw(c, p(context, ctx::TOP));
            c.g[V0][0] = top;
            c.g[V0][0] = sx((top as u32).wrapping_sub(0x14));
            let top = c.g[V0][0];
            Addr(p(context, ctx::TOP)).sw(c, top);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}


/// The frame clock object (`D_004A5B64`, `gp+0x2a74`); `+0x10` is the frame
/// rate as an integer.
pub(crate) const CLOCK: i32 = 0x2a74;


/// `void Emitter_Update(Emitter* e, float dt /*f12*/)` (0x370788): run the
/// emitter's countdown; when it has run out (a negative remaining time) the
/// descriptor is restarted, otherwise its clock is advanced and its remaining
/// life reduced, and the progress accumulator is advanced by `rate * dt`,
/// wrapping to 0 once it reaches the limit. Marks the emitter started.
///
/// State lives in the original's registers: f21 the constant 0, f22 `dt`.
pub fn emitter_update(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 22, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x28);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x20);
        c.mtc1(0, 21);
        c.lwc1(0, r32(c, S0));
        c.c_eq_s(0, 21);
        let dead = c.fcc();
        c.mov_s(22, 12);
        if !dead {
            c.cycles += 4;
            c.c_lt_s(0, 21);
            let expired = c.fcc();
            c.g[V0][0] = c.lw(gp(c, CLOCK));
            if expired {
                // Restart the descriptor: one frame's worth of time.
                c.cycles += 9;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.lwc1(12, at(c, V0, 0x10));
                c.cvt_s_w(12, 12);
                c.div_s(12, 0, 12);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x10));
                callee::EMITTER_UPDATE_DESCRIPTOR.call(c, 0x0037_07e8)?;
                c.lwc1(0, at(c, S0, emitter::RATE));
            } else {
                // Advance its clock, then its remaining life.
                c.cycles += 17;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 20);
                c.lwc1(12, at(c, V0, 0x10));
                c.cvt_s_w(12, 12);
                c.div_s(12, 20, 12);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x10));
                callee::EMITTER_ADVANCE_DESC_TIME.call(c, 0x0037_080c)?;
                c.g[V0][0] = c.lw(gp(c, CLOCK));
                c.lwc1(0, r32(c, S0));
                c.lwc1(1, at(c, V0, 0x10));
                c.cvt_s_w(1, 1);
                c.div_s(20, 20, 1);
                c.sub_s(0, 0, 20);
                c.c_lt_s(0, 21);
                let negative = c.fcc();
                c.swc1(0, r32(c, S0));
                if negative {
                    c.cycles += 1;
                    c.swc1(21, r32(c, S0));
                }
                c.cycles += 1;
                c.lwc1(0, at(c, S0, emitter::RATE));
            }
            // progress += rate * dt, back to 0 once it reaches the limit.
            c.cycles += 9;
            c.lwc1(1, at(c, S0, emitter::PROGRESS));
            c.mul_s(0, 0, 22);
            c.g[V1][0] = c.lw(at(c, S0, emitter::LIMIT));
            c.add_s(1, 1, 0);
            c.cvt_w_s(0, 1);
            c.mfc1(2, 0);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let running = c.g[V0][0] != 0;
            c.swc1(1, at(c, S0, emitter::PROGRESS));
            if !running {
                c.cycles += 1;
                c.sw(at(c, S0, emitter::PROGRESS), 0);
            }
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            c.sw(at(c, S0, emitter::STARTED), c.g[V0][0]);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x30);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `D_004FF120`: the constant vector (0, 0, 0, 1) an emitter's acceleration is
/// reset to.
pub(crate) const ZERO_ACCEL: u32 = 0x004f_f120;


// ---------------------------------------------------------------------------
// Render, UI and world accessors.

/// `void Emitter_ResetDescAccel(Emitter* e)` (0x36d008): copies the constant
/// (0, 0, 0, 1) vector `D_004FF120` into the descriptor's acceleration at
/// +0xe0.
pub fn emitter_reset_desc_accel(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0x50_0000;
    c.g[V0][0] = sx(ZERO_ACCEL);
    let src = c.g[V0][0] as u32;
    c.lq(3, src);
    let selfp = abi::arg(c, 0);
    c.sq(3, selfp.wrapping_add(0xe0));
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Emitter_SetDescCurveB.

/// `void Emitter_SetDescCurveB(Emitter* e, float a, float b)` (0x36ce28):
/// scale the descriptor curve by the emitter's rate (`+0x4c`) and store the
/// two ramps: `+0x14 = (b - a) * s`, `+0x1c = a * s - (b - a) * s`.
pub fn emitter_set_desc_curve_b(c: &mut Cpu) {
    c.cycles += 8;
    let this = abi::arg(c, 0);
    // f13 = b, f12 = a (the EABI float argument order).
    c.sub_s(13, 13, 12);
    c.lwc1(0, this.wrapping_add(0x4c));
    c.mul_s(12, 12, 0);
    c.mul_s(13, 13, 0);
    c.sub_s(12, 12, 13);
    c.swc1(13, this.wrapping_add(0x14));
    c.swc1(12, this.wrapping_add(0x1c));
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Emitter curve.

/// `void Emitter_SetDescCurveA(EmitterDesc *d, float a, float b, float c)`
/// (0x36ce00): store the third value, the `b - a` span, and `a - 1.5 * c` into
/// the curve descriptor at `+0x10`, `+0x20` and `+0x18`.
pub fn emitter_set_desc_curve_a(c: &mut Cpu) {
    c.cycles += 9;
    let base = c.g[A0][0] as u32;
    let a = FpuF32(c.f[12]);
    let b = FpuF32(c.f[13]);
    let c3 = FpuF32(c.f[14]);
    c.g[AT][0] = 0x3fc0_0000; // 1.5
    c.mtc1(AT, 0);
    c.f[13] = (b - a).0; // span into f13
    Addr(base.wrapping_add(0x10)).write_fpu(c, c3);
    c.f[0] = (c3 * FpuF32(c.f[0])).0; // 1.5 * c
    Addr(base.wrapping_add(0x20)).write_fpu(c, FpuF32(c.f[13]));
    c.f[12] = (a - FpuF32(c.f[0])).0; // a - 1.5 * c
    Addr(base.wrapping_add(0x18)).write_fpu(c, FpuF32(c.f[12]));
    abi::ret(c);
}


/// `void GraphicsMan_SnapshotTextureTable(GraphicsMan* self)` (0x369ff8):
/// call slot 0xf2 of the current texture table (`+0x10d8`).
pub fn graphics_man_snapshot_texture_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at_2(abi::arg(c, 0), 0x10d8)).lw(c);
        let table = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(at_2(table, 0x3c8)).lh(c);
        c.g[A1][0] = Addr(at_2(table, 0x3cc)).lw(c);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0036_a014)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void GraphicsMan_FreeAllTextures(GraphicsMan* self)` (0x36a020): call
/// slot 0xf1 of the texture table.
pub fn graphics_man_free_all_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at_2(abi::arg(c, 0), 0x10d8)).lw(c);
        let table = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(at_2(table, 0x3c0)).lh(c);
        c.g[A1][0] = Addr(at_2(table, 0x3c4)).lw(c);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0036_a03c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `Emitter` destroying destructor (0x370758): the particle emitter's base
/// vtable lives at `self + 0x18c`.
pub fn emitter_dtor(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x0037_077c, 0x0049_3160, 0x18c))
}


// ---------------------------------------------------------------------------
// Graphics.

/// `void Emitter_RenderAt(Emitter* e, sMPBox* box, sMPVec4* center)`
/// (0x370888): copy the box and centre into the descriptor's quad slots and
/// render.
pub fn emitter_render_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let self_ = abi::arg(c, 0);
        let bbox = abi::arg(c, 1);
        let center = abi::arg(c, 2);
        c.lq(3, bbox);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = sx(self_);
        c.sq(3, self_.wrapping_add(0x160));
        c.lq(2, bbox.wrapping_add(0x10));
        c.sq(2, self_.wrapping_add(0x170));
        c.lq(3, center);
        c.sq(3, self_.wrapping_add(0xc0));
        callee::EMITTER_RENDER_2.call(c, 0x0037_08b0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void DynamicEmitter_dtor(DynamicEmitter* e, int flags)` (0x370c08): set
/// the vtable, free the emitter's data and `delete this` for bit 0.
pub fn dynamic_emitter_dtor(c: &mut Cpu) {
    c.cycles += 14;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0049_3118);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    c.sw(p_2(c, S1, 0x1f8), c.g[V0][0]);
    c.g[A1][0] = 1;
    if callee::DYNAMIC_EMITTER_FREE_DATA
        .call(c, 0x0037_0c34)
        .is_err()
    {
        return;
    }
    c.g[S0][0] &= 1;
    let keep = c.g[S0][0] != 0;
    frame.restore(c, S0, 0x20);
    if keep {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        if call_ptr(c, table::DELETE, 0x0037_0c48).is_err() {
            return;
        }
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `Emitter_Load(Emitter* self, void* stream)` (0x370af8): read the 12-byte
/// record (`f0`, `f18`, `c`) through the stream vtable and compute the
/// descriptor bounds.
pub fn emitter_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x30);
        let stream = c.g[A1][0];
        c.g[A3][0] = stream;
        frame.save(c, S0, 0x20);
        c.g[A1][0] = sp(c) as u64;
        frame.save_ra(c, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[A2][0] = 0xc;
        c.g[V1][0] = lw_2(c, stream as u32);
        c.g[A0][0] = lh_2(c, p(c.g[V1][0] as u32, 0x10));
        c.g[V0][0] = lw_2(c, p(c.g[V1][0] as u32, 0x14));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((stream as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0037_0b28)?;
        c.lwc1(1, sp(c));
        c.g[A0][0] = sx((this as u32).wrapping_add(0x10));
        c.lwc1(0, p(sp(c), 4));
        c.g[A1][0] = sx((this as u32).wrapping_add(0x160));
        c.swc1(1, this as u32);
        c.swc1(0, p(this as u32, 0x18));
        call_ptr(c, 0x0036_d500, 0x0037_0b44)?;
        c.g[V0][0] = lw_2(c, p(sp(c), 8));
        frame.restore_ra(c, 0x10);
        sw(c, p(this as u32, 0xc), c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}


/// `void Emitter_ComputeDescBounds(const Desc* d, Bounds* out)` (0x36d500):
/// derive the emitter descriptor's world-space bounding box (min at `out+0`,
/// max at `out+0x10`) and its extent from the descriptor's vectors at
/// `+0x60`..`+0xef`, folding in the reference scale/offset vectors at
/// `gp-0x2814`.. and the existing `out` values.
pub fn emitter_compute_desc_bounds(c: &mut Cpu) {
    'b_36d52c: {
        c.cycles += 10;
        c.lwc1(0, at(c, A0, 0x14));
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0xb0));
        c.lwc1(1, at(c, A0, 0x1c));
        c.add_s(2, 0, 0);
        c.add_s(0, 1, 0);
        c.add_s(1, 1, 2);
        c.c_lt_s(1, 0);
        let t = !c.fcc();
        c.mov_s(3, 1);
        if t {
            break 'b_36d52c;
        } else {
            c.cycles += 1;
            c.mov_s(3, 0);
            break 'b_36d52c;
        }
    }
    'b_36d5d0: {
        c.cycles += 41;
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, A0, 0xc0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x20));
        c.g[AT][0] = 0x40000000;
        c.mtc1(1, 2);
        c.lq(2, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x10));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, A0, 0xd0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x20));
        c.lq(2, at(c, SP, 0x20));
        c.sq(2, (c.g[SP][0] as u32));
        c.sq(2, at(c, A1, 0x10));
        c.sq(2, (c.g[A1][0] as u32));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xc0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x20));
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, SP, 0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x10));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, A0, 0xd0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.sq(2, (c.g[SP][0] as u32));
        c.lwc1(1, (c.g[SP][0] as u32));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, (c.g[A1][0] as u32));
            break 'b_36d5d0;
        } else {
            break 'b_36d5d0;
        }
    }
    'b_36d5e8: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36d5e8;
        } else {
            break 'b_36d5e8;
        }
    }
    'b_36d600: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36d600;
        } else {
            break 'b_36d600;
        }
    }
    'b_36d618: {
        c.cycles += 6;
        c.lwc1(1, (c.g[SP][0] as u32));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36d618;
        } else {
            break 'b_36d618;
        }
    }
    'b_36d630: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36d630;
        } else {
            break 'b_36d630;
        }
    }
    'b_36d648: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36d648;
        } else {
            break 'b_36d648;
        }
    }
    'b_36d6ac: {
        c.cycles += 25;
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, A0, 0xc0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x20));
        c.lq(2, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x10));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xd0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x20));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.sq(2, (c.g[SP][0] as u32));
        c.lwc1(1, (c.g[SP][0] as u32));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, (c.g[A1][0] as u32));
            break 'b_36d6ac;
        } else {
            break 'b_36d6ac;
        }
    }
    'b_36d6c4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36d6c4;
        } else {
            break 'b_36d6c4;
        }
    }
    'b_36d6dc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36d6dc;
        } else {
            break 'b_36d6dc;
        }
    }
    'b_36d6f4: {
        c.cycles += 6;
        c.lwc1(1, (c.g[SP][0] as u32));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36d6f4;
        } else {
            break 'b_36d6f4;
        }
    }
    'b_36d70c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36d70c;
        } else {
            break 'b_36d70c;
        }
    }
    'b_36d724: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36d724;
        } else {
            break 'b_36d724;
        }
    }
    'b_36d7a4: {
        c.cycles += 32;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xc0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x20));
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, SP, 0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x10));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xd0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x30));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.sq(2, (c.g[SP][0] as u32));
        c.lwc1(1, (c.g[SP][0] as u32));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, (c.g[A1][0] as u32));
            break 'b_36d7a4;
        } else {
            break 'b_36d7a4;
        }
    }
    'b_36d7bc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36d7bc;
        } else {
            break 'b_36d7bc;
        }
    }
    'b_36d7d4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36d7d4;
        } else {
            break 'b_36d7d4;
        }
    }
    'b_36d7ec: {
        c.cycles += 6;
        c.lwc1(1, (c.g[SP][0] as u32));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36d7ec;
        } else {
            break 'b_36d7ec;
        }
    }
    'b_36d804: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36d804;
        } else {
            break 'b_36d804;
        }
    }
    'b_36d81c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36d81c;
        } else {
            break 'b_36d81c;
        }
    }
    'b_36d8bc: {
        c.cycles += 40;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xc0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x30));
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, SP, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x20));
        c.lqc2(3, at(c, SP, 0x20));
        c.lqc2(4, at(c, A0, 0xd0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.lwc1(0, (c.g[A0][0] as u32));
        c.cvt_s_w(0, 0);
        c.sq(2, at(c, SP, 0x10));
        c.mfc1(3, 0);
        c.lqc2(4, at(c, A0, 0xe0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.sq(2, at(c, SP, 0x40));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.sq(2, (c.g[SP][0] as u32));
        c.lwc1(1, (c.g[SP][0] as u32));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, (c.g[A1][0] as u32));
            break 'b_36d8bc;
        } else {
            break 'b_36d8bc;
        }
    }
    'b_36d8d4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36d8d4;
        } else {
            break 'b_36d8d4;
        }
    }
    'b_36d8ec: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36d8ec;
        } else {
            break 'b_36d8ec;
        }
    }
    'b_36d904: {
        c.cycles += 6;
        c.lwc1(1, (c.g[SP][0] as u32));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36d904;
        } else {
            break 'b_36d904;
        }
    }
    'b_36d91c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36d91c;
        } else {
            break 'b_36d91c;
        }
    }
    'b_36d934: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36d934;
        } else {
            break 'b_36d934;
        }
    }
    'b_36d9d4: {
        c.cycles += 40;
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, A0, 0xc0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x20));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xd0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x30));
        c.lqc2(3, at(c, SP, 0x20));
        c.lqc2(4, at(c, SP, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.lwc1(0, (c.g[A0][0] as u32));
        c.cvt_s_w(0, 0);
        c.sq(2, at(c, SP, 0x10));
        c.mfc1(3, 0);
        c.lqc2(4, at(c, A0, 0xe0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.sq(2, at(c, SP, 0x40));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.sq(2, (c.g[SP][0] as u32));
        c.lwc1(1, (c.g[SP][0] as u32));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, (c.g[A1][0] as u32));
            break 'b_36d9d4;
        } else {
            break 'b_36d9d4;
        }
    }
    'b_36d9ec: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36d9ec;
        } else {
            break 'b_36d9ec;
        }
    }
    'b_36da04: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36da04;
        } else {
            break 'b_36da04;
        }
    }
    'b_36da1c: {
        c.cycles += 6;
        c.lwc1(1, (c.g[SP][0] as u32));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36da1c;
        } else {
            break 'b_36da1c;
        }
    }
    'b_36da34: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36da34;
        } else {
            break 'b_36da34;
        }
    }
    'b_36da4c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36da4c;
        } else {
            break 'b_36da4c;
        }
    }
    'b_36db08: {
        c.cycles += 47;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xc0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x30));
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, SP, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x20));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xd0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.sq(2, at(c, SP, 0x40));
        c.lqc2(3, at(c, SP, 0x20));
        c.lqc2(4, at(c, SP, 0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.lwc1(0, (c.g[A0][0] as u32));
        c.cvt_s_w(0, 0);
        c.sq(2, at(c, SP, 0x10));
        c.mfc1(3, 0);
        c.lqc2(4, at(c, A0, 0xe0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x60));
        c.lq(2, at(c, SP, 0x60));
        c.sq(2, at(c, SP, 0x50));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x50));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x60));
        c.lq(2, at(c, SP, 0x60));
        c.sq(2, (c.g[SP][0] as u32));
        c.lwc1(1, (c.g[SP][0] as u32));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, (c.g[A1][0] as u32));
            break 'b_36db08;
        } else {
            break 'b_36db08;
        }
    }
    'b_36db20: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36db20;
        } else {
            break 'b_36db20;
        }
    }
    'b_36db38: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36db38;
        } else {
            break 'b_36db38;
        }
    }
    'b_36db50: {
        c.cycles += 6;
        c.lwc1(1, (c.g[SP][0] as u32));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36db50;
        } else {
            break 'b_36db50;
        }
    }
    'b_36db68: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36db68;
        } else {
            break 'b_36db68;
        }
    }
    'b_36db80: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36db80;
        } else {
            break 'b_36db80;
        }
    }
    'b_36dc5c: {
        c.cycles += 55;
        c.lq(3, at(c, A1, 0x10));
        c.lq(2, (c.g[A1][0] as u32));
        c.sq(3, at(c, SP, 0x10));
        c.sq(2, (c.g[SP][0] as u32));
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, A0, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x60));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, A0, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x50));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, A0, 0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x30));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x80));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x60));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, A0, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x50));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, A0, 0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36dc5c;
        } else {
            break 'b_36dc5c;
        }
    }
    'b_36dc74: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36dc74;
        } else {
            break 'b_36dc74;
        }
    }
    'b_36dc8c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36dc8c;
        } else {
            break 'b_36dc8c;
        }
    }
    'b_36dca4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36dca4;
        } else {
            break 'b_36dca4;
        }
    }
    'b_36dcbc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36dcbc;
        } else {
            break 'b_36dcbc;
        }
    }
    'b_36dcd4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36dcd4;
        } else {
            break 'b_36dcd4;
        }
    }
    'b_36dd50: {
        c.cycles += 31;
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, A0, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x60));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x90));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x50));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, A0, 0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36dd50;
        } else {
            break 'b_36dd50;
        }
    }
    'b_36dd68: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36dd68;
        } else {
            break 'b_36dd68;
        }
    }
    'b_36dd80: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36dd80;
        } else {
            break 'b_36dd80;
        }
    }
    'b_36dd98: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36dd98;
        } else {
            break 'b_36dd98;
        }
    }
    'b_36ddb0: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36ddb0;
        } else {
            break 'b_36ddb0;
        }
    }
    'b_36ddc8: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36ddc8;
        } else {
            break 'b_36ddc8;
        }
    }
    'b_36de44: {
        c.cycles += 31;
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, A0, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x60));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, A0, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x50));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xa0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36de44;
        } else {
            break 'b_36de44;
        }
    }
    'b_36de5c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36de5c;
        } else {
            break 'b_36de5c;
        }
    }
    'b_36de74: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36de74;
        } else {
            break 'b_36de74;
        }
    }
    'b_36de8c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36de8c;
        } else {
            break 'b_36de8c;
        }
    }
    'b_36dea4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36dea4;
        } else {
            break 'b_36dea4;
        }
    }
    'b_36debc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36debc;
        } else {
            break 'b_36debc;
        }
    }
    'b_36df54: {
        c.cycles += 38;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x80));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x60));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x90));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x50));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, A0, 0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36df54;
        } else {
            break 'b_36df54;
        }
    }
    'b_36df6c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36df6c;
        } else {
            break 'b_36df6c;
        }
    }
    'b_36df84: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36df84;
        } else {
            break 'b_36df84;
        }
    }
    'b_36df9c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36df9c;
        } else {
            break 'b_36df9c;
        }
    }
    'b_36dfb4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36dfb4;
        } else {
            break 'b_36dfb4;
        }
    }
    'b_36dfcc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36dfcc;
        } else {
            break 'b_36dfcc;
        }
    }
    'b_36e064: {
        c.cycles += 38;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x80));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x60));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, A0, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x50));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xa0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36e064;
        } else {
            break 'b_36e064;
        }
    }
    'b_36e07c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36e07c;
        } else {
            break 'b_36e07c;
        }
    }
    'b_36e094: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36e094;
        } else {
            break 'b_36e094;
        }
    }
    'b_36e0ac: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36e0ac;
        } else {
            break 'b_36e0ac;
        }
    }
    'b_36e0c4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36e0c4;
        } else {
            break 'b_36e0c4;
        }
    }
    'b_36e0dc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36e0dc;
        } else {
            break 'b_36e0dc;
        }
    }
    'b_36e174: {
        c.cycles += 38;
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, A0, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x60));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x90));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x50));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xa0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36e174;
        } else {
            break 'b_36e174;
        }
    }
    'b_36e18c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36e18c;
        } else {
            break 'b_36e18c;
        }
    }
    'b_36e1a4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36e1a4;
        } else {
            break 'b_36e1a4;
        }
    }
    'b_36e1bc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36e1bc;
        } else {
            break 'b_36e1bc;
        }
    }
    'b_36e1d4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36e1d4;
        } else {
            break 'b_36e1d4;
        }
    }
    'b_36e1ec: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36e1ec;
        } else {
            break 'b_36e1ec;
        }
    }
    'b_36e2a0: {
        c.cycles += 45;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x80));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x60));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x90));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x50));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xa0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x90));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, SP, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x20));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36e2a0;
        } else {
            break 'b_36e2a0;
        }
    }
    'b_36e2b8: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36e2b8;
        } else {
            break 'b_36e2b8;
        }
    }
    'b_36e2d0: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36e2d0;
        } else {
            break 'b_36e2d0;
        }
    }
    'b_36e2e8: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36e2e8;
        } else {
            break 'b_36e2e8;
        }
    }
    'b_36e300: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36e300;
        } else {
            break 'b_36e300;
        }
    }
    'b_36e318: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36e318;
        } else {
            break 'b_36e318;
        }
    }
    'b_36e3e4: {
        c.cycles += 51;
        c.lqc2(3, at(c, A0, 0x60));
        c.lqc2(4, at(c, SP, 0x30));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x50));
        c.lwc1(1, gp(c, -0x2868));
        c.lq(2, at(c, SP, 0x50));
        c.lwc1(0, gp(c, -0x2864));
        c.min_s(1, 3, 1);
        c.sq(2, at(c, SP, 0x40));
        c.lqc2(3, at(c, A0, 0x60));
        c.lqc2(4, at(c, SP, 0x20));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x60));
        c.mul_s(0, 1, 0);
        c.lq(2, at(c, SP, 0x60));
        c.lwc1(2, gp(c, -0x2860));
        c.sq(2, at(c, SP, 0x50));
        c.mfc1(3, 3);
        c.lqc2(4, at(c, A0, 0x60));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.add_s(0, 0, 2);
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.mul_s(1, 1, 0);
        c.lqc2(3, (c.g[SP][0] as u32));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x70));
        c.mfc1(3, 1);
        c.lqc2(4, at(c, SP, 0x40));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x90));
        c.lqc2(3, at(c, SP, 0x70));
        c.lqc2(4, at(c, SP, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x60));
        c.lwc1(2, at(c, SP, 0x60));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, (c.g[A1][0] as u32));
            break 'b_36e3e4;
        } else {
            break 'b_36e3e4;
        }
    }
    'b_36e3fc: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x4));
            break 'b_36e3fc;
        } else {
            break 'b_36e3fc;
        }
    }
    'b_36e414: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x8));
            break 'b_36e414;
        } else {
            break 'b_36e414;
        }
    }
    'b_36e42c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x60));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x10));
            break 'b_36e42c;
        } else {
            break 'b_36e42c;
        }
    }
    'b_36e444: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x14));
            break 'b_36e444;
        } else {
            break 'b_36e444;
        }
    }
    'b_36e45c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x18));
            break 'b_36e45c;
        } else {
            break 'b_36e45c;
        }
    }
    'b_36e4dc: {
        c.cycles += 32;
        c.mfc1(3, 3);
        c.lqc2(4, at(c, A0, 0x60));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, (c.g[SP][0] as u32));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x70));
        c.mfc1(3, 1);
        c.lqc2(4, at(c, SP, 0x50));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x90));
        c.lqc2(3, at(c, SP, 0x70));
        c.lqc2(4, at(c, SP, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.sq(2, at(c, SP, 0x60));
        c.lwc1(2, at(c, SP, 0x60));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, (c.g[A1][0] as u32));
            break 'b_36e4dc;
        } else {
            break 'b_36e4dc;
        }
    }
    'b_36e4f4: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x4));
            break 'b_36e4f4;
        } else {
            break 'b_36e4f4;
        }
    }
    'b_36e50c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x8));
            break 'b_36e50c;
        } else {
            break 'b_36e50c;
        }
    }
    'b_36e524: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x60));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x10));
            break 'b_36e524;
        } else {
            break 'b_36e524;
        }
    }
    'b_36e53c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x14));
            break 'b_36e53c;
        } else {
            break 'b_36e53c;
        }
    }
    'b_36e554: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x18));
            break 'b_36e554;
        } else {
            break 'b_36e554;
        }
    }
    'b_36e5d4: {
        c.cycles += 32;
        c.mfc1(3, 3);
        c.lqc2(4, at(c, A0, 0x60));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x70));
        c.mfc1(3, 1);
        c.lqc2(4, at(c, SP, 0x40));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x90));
        c.lqc2(3, at(c, SP, 0x70));
        c.lqc2(4, at(c, SP, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.sq(2, at(c, SP, 0x60));
        c.lwc1(2, at(c, SP, 0x60));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, (c.g[A1][0] as u32));
            break 'b_36e5d4;
        } else {
            break 'b_36e5d4;
        }
    }
    'b_36e5ec: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x4));
            break 'b_36e5ec;
        } else {
            break 'b_36e5ec;
        }
    }
    'b_36e604: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x8));
            break 'b_36e604;
        } else {
            break 'b_36e604;
        }
    }
    'b_36e61c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x60));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x10));
            break 'b_36e61c;
        } else {
            break 'b_36e61c;
        }
    }
    'b_36e634: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x14));
            break 'b_36e634;
        } else {
            break 'b_36e634;
        }
    }
    'b_36e64c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x18));
            break 'b_36e64c;
        } else {
            break 'b_36e64c;
        }
    }
    'b_36e6cc: {
        c.cycles += 32;
        c.mfc1(3, 3);
        c.lqc2(4, at(c, A0, 0x60));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x70));
        c.mfc1(3, 1);
        c.lqc2(4, at(c, SP, 0x50));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x90));
        c.lqc2(3, at(c, SP, 0x70));
        c.lqc2(4, at(c, SP, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.sq(2, at(c, SP, 0x60));
        c.lwc1(1, at(c, SP, 0x60));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, (c.g[A1][0] as u32));
            break 'b_36e6cc;
        } else {
            break 'b_36e6cc;
        }
    }
    'b_36e6e4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36e6e4;
        } else {
            break 'b_36e6e4;
        }
    }
    'b_36e6fc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36e6fc;
        } else {
            break 'b_36e6fc;
        }
    }
    'b_36e714: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x60));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36e714;
        } else {
            break 'b_36e714;
        }
    }
    'b_36e72c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36e72c;
        } else {
            break 'b_36e72c;
        }
    }
    'b_36e744: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36e744;
        } else {
            break 'b_36e744;
        }
    }
    'b_36ebd4: {
        c.cycles += 38;
        c.lwc1(3, at(c, A0, 0x60));
        c.lwc1(1, at(c, SP, 0x20));
        c.lwc1(4, at(c, A0, 0x64));
        c.sub_s(1, 1, 3);
        c.lwc1(5, at(c, A0, 0x68));
        c.lwc1(2, at(c, SP, 0x34));
        c.lwc1(0, at(c, SP, 0x38));
        c.div_s(1, 3, 1);
        c.lwc1(7, at(c, SP, 0x30));
        c.lwc1(9, at(c, SP, 0x24));
        c.sub_s(2, 2, 4);
        c.lwc1(8, at(c, SP, 0x28));
        c.sub_s(0, 0, 5);
        c.sub_s(7, 7, 3);
        c.lwc1(10, gp(c, -0x285c));
        c.sub_s(9, 9, 4);
        c.lwc1(6, gp(c, -0x2858));
        c.sub_s(8, 8, 5);
        c.div_s(2, 4, 2);
        c.div_s(0, 5, 0);
        c.div_s(3, 3, 7);
        c.div_s(4, 4, 9);
        c.div_s(5, 5, 8);
        c.sub_s(1, 1, 10);
        c.mtc1(0, 7);
        c.sub_s(2, 2, 10);
        c.mul_s(8, 1, 6);
        c.sub_s(3, 3, 10);
        c.sub_s(4, 4, 10);
        c.sub_s(5, 5, 10);
        c.sub_s(0, 0, 10);
        c.c_lt_s(7, 8);
        c.mul_s(3, 3, 6);
        c.mul_s(9, 0, 6);
        c.mul_s(4, 4, 6);
        c.mul_s(7, 2, 6);
        let t = !c.fcc();
        c.mul_s(5, 5, 6);
        if t {
            break 'b_36ebd4;
        } else {
            'b_36e874: {
                c.cycles += 38;
                c.lwc1(0, gp(c, -0x2854));
                c.lwc1(1, gp(c, -0x2850));
                c.min_s(8, 8, 0);
                c.mfc1(2, 8);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.mul_s(1, 8, 1);
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.add_s(1, 1, 10);
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.mul_s(2, 8, 1);
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36e874;
                } else {
                    break 'b_36e874;
                }
            }
            'b_36e88c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36e88c;
                } else {
                    break 'b_36e88c;
                }
            }
            'b_36e8a4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36e8a4;
                } else {
                    break 'b_36e8a4;
                }
            }
            'b_36e8bc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36e8bc;
                } else {
                    break 'b_36e8bc;
                }
            }
            'b_36e8d4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36e8d4;
                } else {
                    break 'b_36e8d4;
                }
            }
            'b_36e8ec: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36e8ec;
                } else {
                    break 'b_36e8ec;
                }
            }
            'b_36e96c: {
                c.cycles += 32;
                c.mfc1(3, 8);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36e96c;
                } else {
                    break 'b_36e96c;
                }
            }
            'b_36e984: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36e984;
                } else {
                    break 'b_36e984;
                }
            }
            'b_36e99c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36e99c;
                } else {
                    break 'b_36e99c;
                }
            }
            'b_36e9b4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36e9b4;
                } else {
                    break 'b_36e9b4;
                }
            }
            'b_36e9cc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36e9cc;
                } else {
                    break 'b_36e9cc;
                }
            }
            'b_36e9e4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36e9e4;
                } else {
                    break 'b_36e9e4;
                }
            }
            'b_36ea64: {
                c.cycles += 32;
                c.mfc1(3, 8);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36ea64;
                } else {
                    break 'b_36ea64;
                }
            }
            'b_36ea7c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ea7c;
                } else {
                    break 'b_36ea7c;
                }
            }
            'b_36ea94: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36ea94;
                } else {
                    break 'b_36ea94;
                }
            }
            'b_36eaac: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36eaac;
                } else {
                    break 'b_36eaac;
                }
            }
            'b_36eac4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36eac4;
                } else {
                    break 'b_36eac4;
                }
            }
            'b_36eadc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36eadc;
                } else {
                    break 'b_36eadc;
                }
            }
            'b_36eb5c: {
                c.cycles += 32;
                c.mfc1(3, 8);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36eb5c;
                } else {
                    break 'b_36eb5c;
                }
            }
            'b_36eb74: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36eb74;
                } else {
                    break 'b_36eb74;
                }
            }
            'b_36eb8c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36eb8c;
                } else {
                    break 'b_36eb8c;
                }
            }
            'b_36eba4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36eba4;
                } else {
                    break 'b_36eba4;
                }
            }
            'b_36ebbc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36ebbc;
                } else {
                    break 'b_36ebbc;
                }
            }
            c.cycles += 6;
            c.lwc1(1, at(c, SP, 0x68));
            c.lwc1(0, at(c, A1, 0x18));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(c, A1, 0x18));
                break 'b_36ebd4;
            } else {
                break 'b_36ebd4;
            }
        }
    }
    'b_36efe4: {
        c.cycles += 4;
        c.mtc1(0, 0);
        c.c_lt_s(0, 3);
        let t = !c.fcc();

        if t {
            break 'b_36efe4;
        } else {
            'b_36ec80: {
                c.cycles += 39;
                c.lwc1(0, gp(c, -0x284c));
                c.lwc1(1, gp(c, -0x2848));
                c.min_s(3, 3, 0);
                c.lwc1(2, gp(c, -0x2844));
                c.mfc1(2, 3);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.mul_s(1, 3, 1);
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.add_s(1, 1, 2);
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.mul_s(2, 3, 1);
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36ec80;
                } else {
                    break 'b_36ec80;
                }
            }
            'b_36ec98: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ec98;
                } else {
                    break 'b_36ec98;
                }
            }
            'b_36ecb0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36ecb0;
                } else {
                    break 'b_36ecb0;
                }
            }
            'b_36ecc8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36ecc8;
                } else {
                    break 'b_36ecc8;
                }
            }
            'b_36ece0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36ece0;
                } else {
                    break 'b_36ece0;
                }
            }
            'b_36ecf8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36ecf8;
                } else {
                    break 'b_36ecf8;
                }
            }
            'b_36ed78: {
                c.cycles += 32;
                c.mfc1(3, 3);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36ed78;
                } else {
                    break 'b_36ed78;
                }
            }
            'b_36ed90: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ed90;
                } else {
                    break 'b_36ed90;
                }
            }
            'b_36eda8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36eda8;
                } else {
                    break 'b_36eda8;
                }
            }
            'b_36edc0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36edc0;
                } else {
                    break 'b_36edc0;
                }
            }
            'b_36edd8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36edd8;
                } else {
                    break 'b_36edd8;
                }
            }
            'b_36edf0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36edf0;
                } else {
                    break 'b_36edf0;
                }
            }
            'b_36ee70: {
                c.cycles += 32;
                c.mfc1(3, 3);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36ee70;
                } else {
                    break 'b_36ee70;
                }
            }
            'b_36ee88: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ee88;
                } else {
                    break 'b_36ee88;
                }
            }
            'b_36eea0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36eea0;
                } else {
                    break 'b_36eea0;
                }
            }
            'b_36eeb8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36eeb8;
                } else {
                    break 'b_36eeb8;
                }
            }
            'b_36eed0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36eed0;
                } else {
                    break 'b_36eed0;
                }
            }
            'b_36eee8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36eee8;
                } else {
                    break 'b_36eee8;
                }
            }
            'b_36ef68: {
                c.cycles += 32;
                c.mfc1(3, 3);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36ef68;
                } else {
                    break 'b_36ef68;
                }
            }
            'b_36ef80: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ef80;
                } else {
                    break 'b_36ef80;
                }
            }
            'b_36ef98: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36ef98;
                } else {
                    break 'b_36ef98;
                }
            }
            'b_36efb0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36efb0;
                } else {
                    break 'b_36efb0;
                }
            }
            'b_36efc8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36efc8;
                } else {
                    break 'b_36efc8;
                }
            }
            'b_36efe0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36efe0;
                } else {
                    break 'b_36efe0;
                }
            }
            c.cycles += 1;
            c.mtc1(0, 0);
            break 'b_36efe4;
        }
    }
    'b_36f3ec: {
        c.cycles += 3;
        c.c_lt_s(0, 4);
        let t = !c.fcc();

        if t {
            break 'b_36f3ec;
        } else {
            'b_36f08c: {
                c.cycles += 39;
                c.lwc1(0, gp(c, -0x2840));
                c.lwc1(1, gp(c, -0x283c));
                c.min_s(4, 4, 0);
                c.lwc1(2, gp(c, -0x2838));
                c.mfc1(2, 4);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.mul_s(1, 4, 1);
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.add_s(1, 1, 2);
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.mul_s(2, 4, 1);
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36f08c;
                } else {
                    break 'b_36f08c;
                }
            }
            'b_36f0a4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f0a4;
                } else {
                    break 'b_36f0a4;
                }
            }
            'b_36f0bc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f0bc;
                } else {
                    break 'b_36f0bc;
                }
            }
            'b_36f0d4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f0d4;
                } else {
                    break 'b_36f0d4;
                }
            }
            'b_36f0ec: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f0ec;
                } else {
                    break 'b_36f0ec;
                }
            }
            'b_36f104: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36f104;
                } else {
                    break 'b_36f104;
                }
            }
            'b_36f184: {
                c.cycles += 32;
                c.mfc1(3, 4);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36f184;
                } else {
                    break 'b_36f184;
                }
            }
            'b_36f19c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f19c;
                } else {
                    break 'b_36f19c;
                }
            }
            'b_36f1b4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f1b4;
                } else {
                    break 'b_36f1b4;
                }
            }
            'b_36f1cc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f1cc;
                } else {
                    break 'b_36f1cc;
                }
            }
            'b_36f1e4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f1e4;
                } else {
                    break 'b_36f1e4;
                }
            }
            'b_36f1fc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36f1fc;
                } else {
                    break 'b_36f1fc;
                }
            }
            'b_36f27c: {
                c.cycles += 32;
                c.mfc1(3, 4);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36f27c;
                } else {
                    break 'b_36f27c;
                }
            }
            'b_36f294: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f294;
                } else {
                    break 'b_36f294;
                }
            }
            'b_36f2ac: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f2ac;
                } else {
                    break 'b_36f2ac;
                }
            }
            'b_36f2c4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f2c4;
                } else {
                    break 'b_36f2c4;
                }
            }
            'b_36f2dc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f2dc;
                } else {
                    break 'b_36f2dc;
                }
            }
            'b_36f2f4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36f2f4;
                } else {
                    break 'b_36f2f4;
                }
            }
            'b_36f374: {
                c.cycles += 32;
                c.mfc1(3, 4);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36f374;
                } else {
                    break 'b_36f374;
                }
            }
            'b_36f38c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f38c;
                } else {
                    break 'b_36f38c;
                }
            }
            'b_36f3a4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f3a4;
                } else {
                    break 'b_36f3a4;
                }
            }
            'b_36f3bc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f3bc;
                } else {
                    break 'b_36f3bc;
                }
            }
            'b_36f3d4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f3d4;
                } else {
                    break 'b_36f3d4;
                }
            }
            c.cycles += 6;
            c.lwc1(1, at(c, SP, 0x68));
            c.lwc1(0, at(c, A1, 0x18));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(c, A1, 0x18));
                break 'b_36f3ec;
            } else {
                break 'b_36f3ec;
            }
        }
    }
    'b_36f7fc: {
        c.cycles += 4;
        c.mtc1(0, 0);
        c.c_lt_s(0, 7);
        let t = !c.fcc();

        if t {
            break 'b_36f7fc;
        } else {
            c.cycles += 17;
            c.lwc1(0, gp(c, -0x2834));
            c.lwc1(1, gp(c, -0x2830));
            c.min_s(7, 7, 0);
            c.lwc1(2, gp(c, -0x282c));
            c.mfc1(2, 7);
            c.lqc2(4, at(c, A0, 0x60));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(c, SP, 0x90));
            c.mul_s(1, 7, 1);
            c.lq(2, at(c, SP, 0x90));
            c.sq(2, at(c, SP, 0x80));
            c.add_s(1, 1, 2);
            c.lqc2(3, (c.g[SP][0] as u32));
            c.lqc2(4, at(c, SP, 0x80));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(c, SP, 0x90));
            break 'b_36f7fc;
        }
    }
    'b_36fc04: {
        c.cycles += 3;
        c.c_lt_s(0, 5);
        let t = !c.fcc();

        if t {
            break 'b_36fc04;
        } else {
            'b_36f8a4: {
                c.cycles += 39;
                c.lwc1(0, gp(c, -0x2828));
                c.lwc1(1, gp(c, -0x2824));
                c.min_s(5, 5, 0);
                c.lwc1(2, gp(c, -0x2820));
                c.mfc1(2, 5);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.mul_s(1, 5, 1);
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.add_s(1, 1, 2);
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.mul_s(2, 5, 1);
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36f8a4;
                } else {
                    break 'b_36f8a4;
                }
            }
            'b_36f8bc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f8bc;
                } else {
                    break 'b_36f8bc;
                }
            }
            'b_36f8d4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f8d4;
                } else {
                    break 'b_36f8d4;
                }
            }
            'b_36f8ec: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f8ec;
                } else {
                    break 'b_36f8ec;
                }
            }
            'b_36f904: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f904;
                } else {
                    break 'b_36f904;
                }
            }
            'b_36f91c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36f91c;
                } else {
                    break 'b_36f91c;
                }
            }
            'b_36f99c: {
                c.cycles += 32;
                c.mfc1(3, 5);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36f99c;
                } else {
                    break 'b_36f99c;
                }
            }
            'b_36f9b4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f9b4;
                } else {
                    break 'b_36f9b4;
                }
            }
            'b_36f9cc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f9cc;
                } else {
                    break 'b_36f9cc;
                }
            }
            'b_36f9e4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f9e4;
                } else {
                    break 'b_36f9e4;
                }
            }
            'b_36f9fc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f9fc;
                } else {
                    break 'b_36f9fc;
                }
            }
            'b_36fa14: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36fa14;
                } else {
                    break 'b_36fa14;
                }
            }
            'b_36fa94: {
                c.cycles += 32;
                c.mfc1(3, 5);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36fa94;
                } else {
                    break 'b_36fa94;
                }
            }
            'b_36faac: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36faac;
                } else {
                    break 'b_36faac;
                }
            }
            'b_36fac4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36fac4;
                } else {
                    break 'b_36fac4;
                }
            }
            'b_36fadc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36fadc;
                } else {
                    break 'b_36fadc;
                }
            }
            'b_36faf4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36faf4;
                } else {
                    break 'b_36faf4;
                }
            }
            'b_36fb0c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36fb0c;
                } else {
                    break 'b_36fb0c;
                }
            }
            'b_36fb8c: {
                c.cycles += 32;
                c.mfc1(3, 5);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36fb8c;
                } else {
                    break 'b_36fb8c;
                }
            }
            'b_36fba4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36fba4;
                } else {
                    break 'b_36fba4;
                }
            }
            'b_36fbbc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36fbbc;
                } else {
                    break 'b_36fbbc;
                }
            }
            'b_36fbd4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36fbd4;
                } else {
                    break 'b_36fbd4;
                }
            }
            'b_36fbec: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36fbec;
                } else {
                    break 'b_36fbec;
                }
            }
            c.cycles += 6;
            c.lwc1(1, at(c, SP, 0x68));
            c.lwc1(0, at(c, A1, 0x18));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(c, A1, 0x18));
                break 'b_36fc04;
            } else {
                break 'b_36fc04;
            }
        }
    }
    'b_370010: {
        c.cycles += 4;
        c.mtc1(0, 0);
        c.c_lt_s(0, 9);
        let t = !c.fcc();

        if t {
            break 'b_370010;
        } else {
            'b_36fcb0: {
                c.cycles += 39;
                c.lwc1(0, gp(c, -0x281c));
                c.lwc1(1, gp(c, -0x2818));
                c.min_s(9, 9, 0);
                c.lwc1(2, gp(c, -0x2814));
                c.mfc1(2, 9);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.mul_s(1, 9, 1);
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.add_s(1, 1, 2);
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.mul_s(2, 9, 1);
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36fcb0;
                } else {
                    break 'b_36fcb0;
                }
            }
            'b_36fcc8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36fcc8;
                } else {
                    break 'b_36fcc8;
                }
            }
            'b_36fce0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36fce0;
                } else {
                    break 'b_36fce0;
                }
            }
            'b_36fcf8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36fcf8;
                } else {
                    break 'b_36fcf8;
                }
            }
            'b_36fd10: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36fd10;
                } else {
                    break 'b_36fd10;
                }
            }
            'b_36fd28: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36fd28;
                } else {
                    break 'b_36fd28;
                }
            }
            'b_36fda8: {
                c.cycles += 32;
                c.mfc1(3, 9);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36fda8;
                } else {
                    break 'b_36fda8;
                }
            }
            'b_36fdc0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36fdc0;
                } else {
                    break 'b_36fdc0;
                }
            }
            'b_36fdd8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36fdd8;
                } else {
                    break 'b_36fdd8;
                }
            }
            'b_36fdf0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36fdf0;
                } else {
                    break 'b_36fdf0;
                }
            }
            'b_36fe08: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36fe08;
                } else {
                    break 'b_36fe08;
                }
            }
            'b_36fe20: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36fe20;
                } else {
                    break 'b_36fe20;
                }
            }
            'b_36fea0: {
                c.cycles += 32;
                c.mfc1(3, 9);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36fea0;
                } else {
                    break 'b_36fea0;
                }
            }
            'b_36feb8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36feb8;
                } else {
                    break 'b_36feb8;
                }
            }
            'b_36fed0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36fed0;
                } else {
                    break 'b_36fed0;
                }
            }
            'b_36fee8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36fee8;
                } else {
                    break 'b_36fee8;
                }
            }
            'b_36ff00: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36ff00;
                } else {
                    break 'b_36ff00;
                }
            }
            'b_36ff18: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36ff18;
                } else {
                    break 'b_36ff18;
                }
            }
            'b_36ff98: {
                c.cycles += 32;
                c.mfc1(3, 9);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36ff98;
                } else {
                    break 'b_36ff98;
                }
            }
            'b_36ffb0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ffb0;
                } else {
                    break 'b_36ffb0;
                }
            }
            'b_36ffc8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36ffc8;
                } else {
                    break 'b_36ffc8;
                }
            }
            'b_36ffe0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36ffe0;
                } else {
                    break 'b_36ffe0;
                }
            }
            'b_36fff8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36fff8;
                } else {
                    break 'b_36fff8;
                }
            }
            c.cycles += 6;
            c.lwc1(1, at(c, SP, 0x68));
            c.lwc1(0, at(c, A1, 0x18));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(c, A1, 0x18));
                break 'b_370010;
            } else {
                break 'b_370010;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xb0));
    c.pc = t;
    return;
}


/// `int PS2GraphicsMan_WaitForRenderIdle(PS2GraphicsMan* self)` (0x369c28):
/// wait until the renderer reports idle, servicing timers and yielding the
/// EE thread between polls. Dispatcher-shaped: an event unwind leaves `pc`
/// at_3 the poll loop head and the trampoline re-enters there.
pub fn ps2_graphics_man_wait_for_render_idle(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        if pc == 0x0036_9c28 {
            c.cycles += 5;
            open(c, 0x20);
            let sp0 = spv(c);
            c.sq(16, at_3(sp0, 0x10));
            c.sd(sp0, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            pc = 0x0036_9c88;
        }
        loop {
            if pc == 0x0036_9c88 {
                c.cycles += 7;
                let this = c.g[S0][0];
                c.g[V1][0] = lw_at(c, this, 0x10d8);
                c.g[A0][0] = lh_at(c, c.g[V1][0], 0x90);
                c.g[V0][0] = lw_at(c, c.g[V1][0], 0x94);
                let func = c.g[V0][0] as u32;
                c.g[A0][0] = sx(at_3(this as u32, c.g[A0][0] as u32));
                call_ptr(c, func, 0x0036_9c9c)?;
                let idle = c.g[V0][0] == 0;
                let sp0 = spv(c);
                c.g[RA][0] = c.ld(sp0);
                if idle {
                    if event_due(c) && deliver_event(c, 0x0036_9c40) {
                        return Err(crate::call::Unwound);
                    }
                    pc = 0x0036_9c40;
                } else {
                    c.cycles += 3;
                    c.lq(16, at_3(sp0, 0x10));
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx(at_3(sp0, 0x20));
                    c.pc = ra;
                    return Ok(());
                }
            } else if pc == 0x0036_9c40 {
                c.cycles += 18;
                let g = gpa(c, 0x2a74);
                c.g[V0][0] = lw_at(c, g as u64, 0);
                c.g[A1][0] = lw_at(c, c.g[V0][0], 8);
                c.g[V1][0] = lw_at(c, c.g[A1][0], 8);
                c.g[A0][0] = lh_at(c, c.g[V1][0], 0x58);
                c.g[V0][0] = lw_at(c, c.g[V1][0], 0x5c);
                let func = c.g[V0][0] as u32;
                c.g[A0][0] = sx(at_3(c.g[A1][0] as u32, c.g[A0][0] as u32));
                call_ptr(c, func, 0x0036_9c5c)?;
                c.g[A0][0] = 0;
                call_ptr(c, callee::EA_TIMER_SERVICE, 0x0036_9c64)?;
                c.g[A0][0] = 0;
                call_ptr(c, callee::EA_THREAD_YIELD, 0x0036_9c6c)?;
                let g = gpa(c, 0x2a74);
                c.g[V0][0] = lw_at(c, g as u64, 0);
                c.g[A1][0] = lw_at(c, c.g[V0][0], 8);
                c.g[V1][0] = lw_at(c, c.g[A1][0], 8);
                c.g[A0][0] = lh_at(c, c.g[V1][0], 0x60);
                c.g[V0][0] = lw_at(c, c.g[V1][0], 0x64);
                let func = c.g[V0][0] as u32;
                c.g[A0][0] = sx(at_3(c.g[A1][0] as u32, c.g[A0][0] as u32));
                call_ptr(c, func, 0x0036_9c88)?;
                pc = 0x0036_9c88;
            } else if pc == 0x0036_9c3c {
                c.cycles += 1;
                pc = 0x0036_9c40;
            } else {
                c.jump_out(pc, 0x0036_9c28, 0x0036_9cb0);
                return Ok(());
            }
        }
    })
}


// ---------------------------------------------------------------------------
// `Emitter_SetDescPathCoeffs` (0x36cef8): build the emitter's three descriptor
// vectors from the (b, c) pair — `a - 3*(b + c)`, `2*b`, `2*c` — with the
// original's COP2 macro operations.
pub fn emitter_set_desc_path_coeffs(c: &mut Cpu) {
    c.cycles += 43;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.g[AT][0] = 0x40400000;
    c.mtc1(1, 1);
    c.lqc2(3, c.g[A2][0] as u32);
    c.lqc2(4, c.g[A3][0] as u32);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[AT][0] = 0x40000000;
    c.mtc1(1, 0);
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.mfc1(3, 1);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lqc2(3, c.g[A1][0] as u32);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0xb0));
    c.sq(2, (c.g[SP][0] as u32));
    c.mfc1(3, 0);
    c.lqc2(4, c.g[A2][0] as u32);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0xc0));
    c.sq(2, (c.g[SP][0] as u32));
    c.mfc1(3, 0);
    c.lqc2(4, c.g[A3][0] as u32);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0xd0));
    c.sq(2, (c.g[SP][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
}


// ---------------------------------------------------------------------------
// Plant sway matrices.

/// `void PS2GraphicsMan_v94(PS2GraphicsMan* self, float* mtx)` (0x36a2c0):
/// multiply the object's four sway matrices by the caller's matrix and write
/// them back (VU0, in place).
pub fn ps2_graphics_man_v94(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save_ra(c, 0x90);
        c.g[V0][0] = Addr(r(c, A0)).off(0xa4).lw(c);
        let none: bool = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if none {
            c.cycles += 4;
            frame.restore(c, 16, 0xa0);
            frame.restore_ra(c, 0x90);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 49;
        c.lq(2, p(r(c, S0), 0x30));
        c.g[A0][0] = sx(p(r(c, SP), 0x40));
        c.sq(2, p(r(c, SP), 0x40));
        callee::VFX_PLANTS_GET_SWAY_MATRIX.call(c, 0x0036_a2e8)?;

        c.g[V1][0] = sx(p(r(c, SP), 0x50));
        c.lqc2(4, r(c, S0));
        c.lqc2(5, p(r(c, S0), 0x10));
        c.lqc2(6, p(r(c, S0), 0x20));
        c.lqc2(7, p(r(c, S0), 0x30));
        c.lqc2(8, r(c, V0));
        c.lqc2(9, p(r(c, V0), 0x10));
        c.lqc2(10, p(r(c, V0), 0x20));
        c.lqc2(11, p(r(c, V0), 0x30));
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
        c.sqc2(12, r(c, V1));
        c.sqc2(13, p(r(c, V1), 0x10));
        c.sqc2(14, p(r(c, V1), 0x20));
        c.sqc2(15, p(r(c, V1), 0x30));
        c.lqc2(1, r(c, V1));
        c.lqc2(2, p(r(c, V1), 0x10));
        c.lqc2(3, p(r(c, V1), 0x20));
        c.lqc2(4, p(r(c, V1), 0x30));
        c.sqc2(1, r(c, SP));
        c.sqc2(2, p(r(c, SP), 0x10));
        c.sqc2(3, p(r(c, SP), 0x20));
        c.sqc2(4, p(r(c, SP), 0x30));
        c.lqc2(1, r(c, SP));
        c.lqc2(2, p(r(c, SP), 0x10));
        c.lqc2(3, p(r(c, SP), 0x20));
        c.lqc2(4, p(r(c, SP), 0x30));
        c.sqc2(1, r(c, S0));
        c.sqc2(2, p(r(c, S0), 0x10));
        c.sqc2(3, p(r(c, S0), 0x20));
        c.sqc2(4, p(r(c, S0), 0x30));

        c.cycles += 4;
        frame.restore(c, 16, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Memory accessors: the original's `lw rt, off(base)` and friends.

/// Register-relative guest memory, for the accessors below: the base is read
/// from the register file each time, as the compiled code does.
pub(crate) trait RegMem {
    fn pa(&self, base: usize, off: i32) -> u32;
    fn ldw_2(&mut self, base: usize, off: i32) -> u64;
    fn ldwu(&mut self, base: usize, off: i32) -> u64;
    fn ldh_2(&mut self, base: usize, off: i32) -> u64;
    fn ldhu_2(&mut self, base: usize, off: i32) -> u64;
    fn ldb(&mut self, base: usize, off: i32) -> u64;
    fn ldbu(&mut self, base: usize, off: i32) -> u64;
    fn ldd(&mut self, base: usize, off: i32) -> u64;
    fn stw_2(&mut self, base: usize, off: i32, v: u64);
    fn sth_2(&mut self, base: usize, off: i32, v: u64);
    fn stb(&mut self, base: usize, off: i32, v: u64);
    fn std(&mut self, base: usize, off: i32, v: u64);
}


impl RegMem for Cpu {
    #[inline(always)]
    fn pa(&self, base: usize, off: i32) -> u32 {
        r32(self, base).wrapping_add(off as u32)
    }
    #[inline(always)]
    fn ldw_2(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lw(self)
    }
    #[inline(always)]
    fn ldwu(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lwu(self)
    }
    #[inline(always)]
    fn ldh_2(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lh(self)
    }
    #[inline(always)]
    fn ldhu_2(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lhu(self)
    }
    #[inline(always)]
    fn ldb(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lb(self)
    }
    #[inline(always)]
    fn ldbu(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lbu(self)
    }
    #[inline(always)]
    fn ldd(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).ld(self)
    }
    #[inline(always)]
    fn stw_2(&mut self, base: usize, off: i32, v: u64) {
        let a = self.pa(base, off);
        Addr(a).sw(self, v)
    }
    #[inline(always)]
    fn sth_2(&mut self, base: usize, off: i32, v: u64) {
        let a = self.pa(base, off);
        Addr(a).sh(self, v)
    }
    #[inline(always)]
    fn stb(&mut self, base: usize, off: i32, v: u64) {
        let a = self.pa(base, off);
        Addr(a).sb(self, v)
    }
    #[inline(always)]
    fn std(&mut self, base: usize, off: i32, v: u64) {
        let a = self.pa(base, off);
        Addr(a).sd(self, v)
    }
}


// ---------------------------------------------------------------------------
// Functions.

/// `void Emitter_SetDescPathCoeffsScaled(sGmFrustum* self, const Vec4* a,
/// const Vec4* b, const Vec4* c, const Vec4* d)` (0x36d1f0): store each
/// direction divided by `self->scale` (`+0x4c`) at `+0x80`, `+0x90`, `+0xa0`,
/// and `a/scale - 1.5*(b+c+d)/scale` at `+0x70` (VU0 multiplies by the
/// reciprocal of the scale).
pub fn emitter_set_desc_path_coeffs_scaled(c: &mut Cpu) {
    c.cycles += 74;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.lwc1(0, c.pa(A0, 0x4c));
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, c.pa(A2, 0));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, c.pa(SP, 0x10));
    c.g[AT][0] = 0x3fc00000;
    c.mtc1(1, 1);
    c.lq(2, c.pa(SP, 0x10));
    c.lwc1(0, c.pa(A0, 0x4c));
    c.sq(2, c.pa(A0, 0x80));
    c.sq(2, c.pa(SP, 0));
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, c.pa(A3, 0));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, c.pa(SP, 0x10));
    c.lq(2, c.pa(SP, 0x10));
    c.lwc1(0, c.pa(A0, 0x4c));
    c.sq(2, c.pa(A0, 0x90));
    c.sq(2, c.pa(SP, 0));
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, c.pa(T0, 0));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, c.pa(SP, 0x10));
    c.lq(2, c.pa(SP, 0x10));
    c.lwc1(0, c.pa(A0, 0x4c));
    c.sq(2, c.pa(A0, 0xa0));
    c.sq(2, c.pa(SP, 0));
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, c.pa(A1, 0));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, c.pa(SP, 0x20));
    c.lq(2, c.pa(SP, 0x20));
    c.sq(2, c.pa(SP, 0x10));
    c.lqc2(3, c.pa(A0, 0x80));
    c.lqc2(4, c.pa(A0, 0x90));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, c.pa(SP, 0x50));
    c.lq(2, c.pa(SP, 0x50));
    c.sq(2, c.pa(SP, 0x40));
    c.lqc2(3, c.pa(SP, 0x40));
    c.lqc2(4, c.pa(A0, 0xa0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, c.pa(SP, 0x50));
    c.lq(2, c.pa(SP, 0x50));
    c.sq(2, c.pa(SP, 0x30));
    c.mfc1(3, 1);
    c.lqc2(4, c.pa(SP, 0x30));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, c.pa(SP, 0x50));
    c.lq(2, c.pa(SP, 0x50));
    c.sq(2, c.pa(SP, 0x20));
    c.lqc2(3, c.pa(SP, 0x10));
    c.lqc2(4, c.pa(SP, 0x20));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, c.pa(SP, 0x50));
    c.lq(2, c.pa(SP, 0x50));
    c.sq(2, c.pa(A0, 0x70));
    c.sq(2, c.pa(SP, 0));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = ra;
    return;
}


/// `void PS2GraphicsMan_EmitDepthFogClutUpload(void*, int x, int y, char***
/// pp)` (0x36aa60): append the 0xB0-byte depth/fog CLUT-upload packet (six
/// GIF registers writing one CLUT at (x, y)) to the stream at `*pp`, then
/// advance the cursor past it. Leaf.
pub fn ps2_graphics_man_emit_depth_fog_clut_upload(c: &mut Cpu) {
    c.cycles += 79;
    c.g[A0][0] = c.ldw_2(A3, 0);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.sq(16, c.pa(SP, 0));
    c.por(3, 0, 0);
    c.g[V1][0] = 0x10000000;
    c.g[V1][0] = c.g[V1][0] | 0x6;
    c.sq(3, c.pa(A0, 0));
    c.g[V0][0] = 0x8000;
    c.g[V0][0] = c.g[V0][0] << 33;
    c.g[A2][0] = c.g[A2][0] << 32;
    c.g[T0][0] = 0x10000000;
    c.g[T0][0] = c.g[T0][0] << 32;
    c.g[T0][0] = c.g[T0][0] | 0x4;
    c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
    c.g[T7][0] = 0xe;
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.g[A1][0] = c.g[A1][0] << 32;
    c.g[T1][0] = 0x50;
    c.g[T2][0] = 0x51;
    c.std(V0, 0x10, c.g[T0][0]);
    c.g[T3][0] = 0x10;
    c.g[T3][0] = c.g[T3][0] << 32;
    c.g[T3][0] = c.g[T3][0] | 0x10;
    c.g[V0][0] = 0x30000000;
    c.g[V0][0] = c.g[V0][0] | 0x40;
    c.g[T4][0] = 0x52;
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
    c.g[T5][0] = 0x53;
    c.por(4, 0, 0);
    c.g[A0][0] = 0x8000000;
    c.g[A0][0] = c.g[A0][0] << 32;
    c.g[A0][0] = c.g[A0][0] | 0x40;
    c.std(V1, 0x18, c.g[T7][0]);
    c.por(8, 0, 0);
    c.g[T0][0] = 0x10000000;
    c.g[T0][0] = c.g[T0][0] | 0x2;
    c.g[T6][0] = 0x10000000;
    c.g[T6][0] = c.g[T6][0] << 32;
    c.g[T6][0] = c.g[T6][0] | 0x1;
    c.g[S0][0] = 0x3f;
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.std(V0, 0x20, c.g[A2][0]);
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.std(V1, 0x28, c.g[T1][0]);
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.std(V0, 0x30, 0u64);
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.std(V1, 0x38, c.g[T2][0]);
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.std(V0, 0x40, c.g[T3][0]);
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.std(V1, 0x48, c.g[T4][0]);
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.std(V0, 0x50, 0u64);
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.std(V1, 0x58, c.g[T5][0]);
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.sq(4, c.pa(V0, 0x60));
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.std(V1, 0x70, c.g[A1][0]);
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.std(V0, 0x78, 0u64);
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.sq(8, c.pa(V1, 0x80));
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.std(V0, 0x90, c.g[T6][0]);
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.std(V1, 0x98, c.g[T7][0]);
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.std(V0, 0xa0, 0u64);
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.std(V1, 0xa8, c.g[S0][0]);
    c.lq(16, c.pa(SP, 0));
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
    c.stw_2(A3, 0, c.g[V0][0]);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
    return;
}


/// `int PS2GraphicsMan_MatchBlendedMatrix(Ent* a, Ent* b, int tol0, int tol1,
/// int tol2)` (0x369a78): do two render entries share a blended matrix?
/// Same type and key id, then type-specific equality shortcuts, then either
/// the same key ids with coordinates within the tolerances, or a swapped
/// (x/z, y) match.
pub fn ps2_graphics_man_match_blended_matrix(c: &mut Cpu) {
    'b_369c1c: {
        c.cycles += 4;
        c.g[T3][0] = c.ldw_2(A0, 0);
        c.g[V0][0] = c.ldw_2(A1, 0);
        let t = c.g[T3][0] != c.g[V0][0];
        c.g[V0][0] = 0u64;
        if t {
            break 'b_369c1c;
        } else {
            'b_369c18: {
                c.cycles += 7;
                c.g[V0][0] = c.ldw_2(A0, 0x4);
                c.g[V1][0] = c.ldw_2(A1, 0x4);
                c.g[T1][0] = c.g[V0][0];
                c.g[A0][0] = c.ldhu_2(V0, 0x2);
                c.g[V0][0] = c.ldhu_2(V1, 0x2);
                let t = c.g[A0][0] != c.g[V0][0];
                c.g[T2][0] = c.g[V1][0];
                if t {
                    break 'b_369c18;
                } else {
                    'b_369ba0: {
                        'b_369b20: {
                            'b_369ae4: {
                                'b_369b1c: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x2;
                                    let t = c.g[T3][0] == c.g[V0][0];
                                    c.g[V0][0] = ((c.g[T3][0] as i64) < 3i64) as u64;
                                    if t {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.ldh_2(T1, 0);
                                        c.g[V0][0] = c.ldh_2(T2, 0);
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[V1][0] = c.ldhu_2(T1, 0x6);
                                        if t {
                                            break 'b_369b20;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.ldhu_2(T2, 0x6);
                                            if c.g[V1][0] != c.g[V0][0] {
                                                c.g[V1][0] = c.ldhu_2(T1, 0xa);
                                                break 'b_369ba0;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V1][0] = c.ldh_2(T1, 0x4);
                                                let t = true;
                                                c.g[V0][0] = c.ldh_2(T2, 0x4);
                                                if t {
                                                    break 'b_369ae4;
                                                } else {
                                                    break 'b_369b1c;
                                                }
                                            }
                                        }
                                    } else {
                                        'b_369adc: {
                                            'b_369ac8: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x1;
                                                if t {
                                                    break 'b_369ac8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[T3][0] == c.g[V0][0];
                                                    c.g[V0][0] = 0u64;
                                                    if t {
                                                        break 'b_369adc;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_369c1c;
                                                        } else {
                                                            break 'b_369ac8;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x3;
                                            let t = c.g[T3][0] == c.g[V0][0];
                                            c.g[V0][0] = 0u64;
                                            if t {
                                                break 'b_369b1c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;

                                                if t {
                                                    break 'b_369c1c;
                                                } else {
                                                    break 'b_369adc;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        c.g[V1][0] = c.ldh_2(T1, 0);
                                        c.g[V0][0] = c.ldh_2(T2, 0);
                                        break 'b_369ae4;
                                    }
                                }
                                c.cycles += 1;
                                c.g[V1][0] = c.ldhu_2(T1, 0x6);
                                break 'b_369b20;
                            }
                            c.cycles += 2;
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[V1][0] = c.ldhu_2(T1, 0x6);
                                break 'b_369b20;
                            } else {
                                c.cycles += 2;
                                let ra = c.g[RA][0] as u32;
                                c.g[V0][0] = 0x1;
                                c.pc = ra;
                                return;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.ldhu_2(T2, 0x6);
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V1][0] = c.ldhu_2(T1, 0xa);
                        if t {
                            break 'b_369ba0;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.ldhu_2(T2, 0xa);
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[V0][0] = c.ldhu_2(T2, 0x6);
                                break 'b_369ba0;
                            } else {
                                'b_369b4c: {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.ldh_2(T1, 0);
                                    c.g[V1][0] = c.ldh_2(T2, 0);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                                    if (c.g[V0][0] as i64) < 0 {
                                        c.g[V0][0] = sx(0u32.wrapping_sub((c.g[V0][0] as u32)));
                                        break 'b_369b4c;
                                    } else {
                                        break 'b_369b4c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_369c1c;
                                } else {
                                    'b_369b6c: {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.ldh_2(T1, 0x4);
                                        c.g[V1][0] = c.ldh_2(T2, 0x4);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_sub((c.g[V1][0] as u32)));
                                        if (c.g[V0][0] as i64) < 0 {
                                            c.g[V0][0] = sx(0u32.wrapping_sub((c.g[V0][0] as u32)));
                                            break 'b_369b6c;
                                        } else {
                                            break 'b_369b6c;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_369c1c;
                                    } else {
                                        'b_369b8c: {
                                            c.cycles += 5;
                                            c.g[V0][0] = c.ldh_2(T1, 0x8);
                                            c.g[V1][0] = c.ldh_2(T2, 0x8);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_sub((c.g[V1][0] as u32)));
                                            if (c.g[V0][0] as i64) < 0 {
                                                c.g[V0][0] =
                                                    sx(0u32.wrapping_sub((c.g[V0][0] as u32)));
                                                break 'b_369b8c;
                                            } else {
                                                break 'b_369b8c;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_369c1c;
                                        } else {
                                            c.cycles += 2;
                                            let ra = c.g[RA][0] as u32;
                                            c.g[V0][0] = 0x1;
                                            c.pc = ra;
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_369c1c;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.ldhu_2(T1, 0x6);
                        c.g[V0][0] = c.ldhu_2(T2, 0xa);
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_369c1c;
                        } else {
                            'b_369bcc: {
                                c.cycles += 5;
                                c.g[V0][0] = c.ldh_2(T1, 0);
                                c.g[V1][0] = c.ldh_2(T2, 0);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                                if (c.g[V0][0] as i64) < 0 {
                                    c.g[V0][0] = sx(0u32.wrapping_sub((c.g[V0][0] as u32)));
                                    break 'b_369bcc;
                                } else {
                                    break 'b_369bcc;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_369c1c;
                            } else {
                                'b_369bec: {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.ldh_2(T1, 0x8);
                                    c.g[V1][0] = c.ldh_2(T2, 0x4);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                                    if (c.g[V0][0] as i64) < 0 {
                                        c.g[V0][0] = sx(0u32.wrapping_sub((c.g[V0][0] as u32)));
                                        break 'b_369bec;
                                    } else {
                                        break 'b_369bec;
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_369c1c;
                                } else {
                                    'b_369c0c: {
                                        c.cycles += 5;
                                        c.g[V1][0] = c.ldh_2(T1, 0x4);
                                        c.g[A0][0] = c.ldh_2(T2, 0x8);
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_sub((c.g[A0][0] as u32)));
                                        if (c.g[V1][0] as i64) < 0 {
                                            c.g[V1][0] = sx(0u32.wrapping_sub((c.g[V1][0] as u32)));
                                            break 'b_369c0c;
                                        } else {
                                            break 'b_369c0c;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V1][0] = ((c.g[T0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    let t = c.g[V1][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_369c1c;
                                    } else {
                                        break 'b_369c18;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_369c1c;
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
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::put_vec;
    use crate::difftest::fakes::covered;
    use crate::difftest::put_words;
    use ssx3_ee as ee;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fakes;
    use crate::difftest::fakes::ptr;
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
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use ssx3_ee_rt::cpu::reg::*;
    use ssx3_ee_rt::cpu::sx;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::seed_count;
    use crate::difftest::compare;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS_3 as SEEDS;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::CTX;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::TOP;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::TEX;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::BASE;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::f;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::ctx_at;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::put_ctx;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::be::beintoption::tests::VT;
    pub(crate) use crate::game::be::beintoption::tests::slot;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::ai::ai::tests::be_library;
    pub(crate) use crate::game::object::railmodifier::tests::destructor_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::MEM_HEAP;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA_SIZE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::run;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_init;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::alloc;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::world_2 as world;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::fe_render_fixture;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::gp_at;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base;
    pub(crate) use crate::game::fe::festaterewards::tests::BE_7 as BE_2;
    pub(crate) use crate::game::fe::festaterewards::tests::W as W_2;
    pub(crate) use crate::game::fe::festaterewards::tests::HOOKS;
    pub(crate) use crate::game::fe::festaterewards::tests::KEEP_F0;
    pub(crate) use crate::game::fe::festaterewards::tests::w;
    pub(crate) use crate::game::fe::festaterewards::tests::hook;
    pub(crate) use crate::game::fe::festaterewards::tests::strict;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_2;


    #[test]
    fn deferred_delete_push_matches() {
        check(ssx3_ee::s12::f_0036a1c0, deferred_delete_push, 200, &|c, seed| {
            let mut r = Rng(0x36a1_c000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            let count = r.next_u32() % 8;
            c.bus.write32(GAME_GP + 0x2a98, count);
            c.bus.write32(0x53ab40 + count * 4, r.next_u32());
        });
    }


    #[test]
    fn ps2_graphics_man_reset_depth_fog_params_matches() {
        check(ssx3_ee::s12::f_0036aba0, ps2_graphics_man_reset_depth_fog_params, 200, &|c, seed| {
            let mut r = Rng(0x36ab_a000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.bus.write32(GAME_GP + 0x123c, r.edge_f32());
            c.bus.write32(GAME_GP + 0x1240, r.edge_f32());
            c.bus.write32(GAME_GP + 0x1244, r.next_u32());
            c.bus.write32(GAME_GP + 0x1248, r.edge_f32());
        });
    }


    #[test]
    fn emitter_set_desc_time_matches() {
        check(ssx3_ee::s12::f_0036d3d8, emitter_set_desc_time, 200, &|c, seed| {
            let mut r = Rng(0x36d3_d800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.f[12] = r.edge_f32();
            c.bus.write32(DATA + 0x4c, r.edge_f32());
        });
    }


    #[test]
    fn emitter_advance_desc_time_matches() {
        check(ssx3_ee::s12::f_0036d3e8, emitter_advance_desc_time, 200, &|c, seed| {
            let mut r = Rng(0x36d3_e800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.f[12] = r.edge_f32();
            c.bus.write32(DATA + 0x4c, r.edge_f32());
            c.bus.write32(DATA + 0x8, r.edge_f32());
        });
    }


    #[test]
    fn emitter_render_matches() {
        check(ee::s12::f_003708c0__generated, emitter_render, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x08c0, 3);
            put_ctx(c, &mut r, &[0x158, 0x2e8, 0x290]);
            let e = BASE;
            c.bus.write32(e + 0xc, (seed % 6 != 0) as u32);
            c.bus.write32(e + 4, 0);
            c.bus.write32(e + 8, r.next_u32() % 8);
            c.bus.write32(e + 0x184, if seed % 5 == 0 { f(1.5) } else { 0 });
            for i in 0..3u32 {
                c.bus.write32(TEX + 4 * i, r.next_u32() % 0x1000);
            }
            for off in [0x1e8u32, 0x1ec, 0x1f4] {
                c.bus.write32(e + off, r.next_u32());
            }
            for i in 0..5 {
                c.bus.write32(e + 0x1e4 + 4 * i, r.next_u32());
            }
            for i in 0..5 {
                c.bus.write32(globals::EMITTER_BLEND + 4 * i, r.next_u32() % 32);
            }
            // The cull call is the second fake; forcing 1 means "culled".
            for k in 0..8u32 {
                c.bus.write32(LOG_RESULTS + 4 * k, if seed % 2 == 0 { 0 } else { 1 });
            }
            c.g[A0][0] = sx(e);
        });
    }


    #[test]
    fn emitter_update_matches() {
        check(ssx3_ee::s12::f_00370788__generated, emitter_update, 200, &|c, seed| {
            let mut r = machine(c, seed, 79, 0);
            let e = DATA;
            c.g[A0][0] = sx(e);
            c.f[12] = if seed % 2 == 0 { r.edge_f32() } else { r.f32(1.0) };
            // Remaining life: 0 (idle), negative (restart) or positive.
            let remain = match seed % 3 {
                0 => 0,
                1 => 0xbf80_0000 | (r.next_u32() & 0x003f_ffff),
                _ => r.f32(120.0),
            };
            c.bus.write32(e, remain);
            // The descriptor at e+0x10: all zero, so both callees take their
            // short paths (progress 0, limit 0, rate 0).
            for off in [
                0x10u32, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c,
                0x40, 0x44, 0x48, 0x4c, 0x50, 0x54, 0x58, 0x5c,
            ] {
                c.bus.write32(e + off, 0);
            }
            // Progress, its limit and the rate `dt` scales.
            let limit = (r.next_u32() % 6) as i32;
            c.bus.write32(e + 0x180, limit as u32);
            c.bus.write32(e + 0x184, if seed % 5 == 0 { 0 } else { r.edge_f32() });
            c.bus.write32(e + 0x188, if seed % 4 == 0 { 0 } else { r.edge_f32() });
            c.bus.write32(e + 0xc, r.next_u32());
            // The frame clock object.
            c.bus.write32(GAME_GP + 0x2a74, DATA + 0x300);
            c.bus.write32(DATA + 0x310, if seed % 7 == 0 { 1 } else { 30 + r.next_u32() % 60 });
        });
    }


    #[test]
    fn emitter_reset_desc_accel_matches() {
        check(ee::s12::f_0036d008__generated, emitter_reset_desc_accel, SEEDS_2, &|c, seed| {
            let mut r = rng(0x0036_d008, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            put_vec(c, 0x004f_f120, [0, 0, 0, 0x3f80_0000]);
            c.bus.write32(DATA + 0xe0, r.next_u32());
        });
    }


    #[test]
    fn emitter_set_desc_curve_b_matches() {
        crate::difftest::check(ee::s12::f_0036ce28__generated, emitter_set_desc_curve_b, 200, &|c, seed| {
            let mut r = Rng(0x36ce_2801 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
            c.bus.write32(DATA + 0x4c, r.edge_f32());
        });
    }


    // -------------------------------------------------------- emitter / UI text

    #[test]
    fn emitter_set_desc_curve_a_matches() {
        check(ee::s12::f_0036ce00__generated, emitter_set_desc_curve_a, 200, &|c, seed| {
            let mut r = Rng(0x36ce_0000 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            c.g[A0][0] = SCRATCH as u64;
            for f in 12..15 {
                c.f[f] = r.edge_f32();
            }
        });
    }


    #[test]
    fn graphics_man_snapshot_texture_table_matches() {
        covered(s12::f_00369ff8__generated, graphics_man_snapshot_texture_table, 120, &|c, seed| {
            machine(c, seed, 0x369ff8, 1);
            clear(c, OBJ, 0x40);
            clear(c, VT, 0x40);
            slot(c, VT, 0x3c8);
            c.bus.write32(OBJ + 0x10d8, VT);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }


    #[test]
    fn graphics_man_free_all_textures_matches() {
        covered(s12::f_0036a020__generated, graphics_man_free_all_textures, 120, &|c, seed| {
            machine(c, seed, 0x36a020, 1);
            clear(c, OBJ, 0x40);
            clear(c, VT, 0x40);
            slot(c, VT, 0x3c0);
            c.bus.write32(OBJ + 0x10d8, VT);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }


    #[test]
    fn emitter_dtor_matches() {
        check(ee::s12::f_00370758__generated, emitter_dtor, SEEDS_3, &|c, seed| {
            destructor_setup(c, seed, 0x3707_58)
        });
    }


    // --------------------------------------------------------- graphics

    #[test]
    fn emitter_render_at_matches() {
        check(ee::s12::f_00370888__generated, emitter_render_at, 80, &|c, seed| {
            let mut r = machine(c, seed, 0x3708_88, 0);
            let desc = DATA + 0x2000;
            c.g[A0][0] = sx(desc);
            c.g[A1][0] = sx(DATA + 0x400);
            c.g[A2][0] = sx(DATA + 0x440);
            crate::difftest::put_vec(c, DATA + 0x400, [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()]);
            crate::difftest::put_vec(c, DATA + 0x410, [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()]);
            crate::difftest::put_vec(c, DATA + 0x440, [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()]);
            fill(c, &mut r, desc, 0x200);
            // Nothing pending on the descriptor: Emitter_Render returns early.
            c.bus.write32(desc + 0xc, 0);
        });
    }


    #[test]
    fn dynamic_emitter_dtor_matches() {
        check(
            ssx3_ee::s12::f_00370c08__generated,
            dynamic_emitter_dtor,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x370c08, 0);
                heap_init(c);
                let this = alloc(c, 0x400);
                let a = if seed % 3 == 0 { alloc(c, 0x40) } else { 0 };
                let b = if seed % 5 == 0 { alloc(c, 0x40) } else { 0 };
                c.bus.write32(this + 0x1a0, a);
                c.bus.write32(this + 0x1a4, b);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }


    #[test]
    fn emitter_load_matches() {
        check(ssx3_ee::s12::f_00370af8__generated, emitter_load, 80, &|c, seed| {
            let mut r = setup(c, seed, 0x370af8);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            // stream->v02: vtable slot 0x10/0x14 (a fake method that just logs).
            let vt = vtable(c, &mut r, &[0x10]);
            c.bus.write32(DATA + 0x800, vt);
        });
    }


    /// A pointer to the first descriptor byte is `a0`, the box being widened
    /// is `a1`; every float the pass reads (scalars at `+0x14`/`+0x1c`, the
    /// vectors at `+0x60`..`+0xef`, the box, the `gp-0x2868` reference
    /// vectors) is random, and the seeds reach both sides of the clamps.
    #[test]
    fn emitter_compute_desc_bounds_matches() {
        check(
            ssx3_ee::s12::f_0036d500__generated,
            emitter_compute_desc_bounds,
            300,
            &|c, seed| {
                let mut r = Rng(0x0036_d500 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                for i in 0..0x40u32 {
                    let v = if i % 3 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(100.0)
                    };
                    c.bus.write32(DATA + 4 * i, v);
                }
                for i in 0..8u32 {
                    c.bus.write32(DATA + 0x200 + 4 * i, r.f32(100.0));
                }
                let base = (GAME_GP as i32 - 0x2868) as u32;
                for i in 0..24u32 {
                    c.bus.write32(base + 4 * i, r.edge_f32());
                }
            },
        );
    }


    #[test]
    fn ps2_graphics_man_wait_for_render_idle_matches() {
        check(
            ee::s12::f_00369c28__generated,
            ps2_graphics_man_wait_for_render_idle,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x369c28, 4);
                c.pc = 0x0036_9c28;
                let o = fe_render_fixture(c, &mut r);
                c.g[A0][0] = sx(DATA);
                // Busy: the fake answers nonzero and the poll returns at_3 once.
                // Idle: a real zero returner sends the body through the flush
                // block, whose first virtual call unwinds the thread so the
                // poll loop stops after one iteration.
                if seed % 2 == 0 {
                    c.bus.write32(o + 0x94, 0x002d_1bd8);
                    c.bus.write32(GAME_GP + 0x16c8, 0);
                    c.bus.write32(crate::difftest::fakes::LOG_UNWIND_AT, 0);
                } else {
                    c.bus.write32(o + 0x94, FAKE);
                    c.bus.write32(LOG_RESULTS, 0x11);
                }
                c.bus.write32(GAME_GP + 0x2a74, 0);
            },
        );
    }


    #[test]
    fn emitter_set_desc_path_coeffs_matches() {
        check(
            ee::s12::f_0036cef8__generated,
            emitter_set_desc_path_coeffs,
            100,
            &|c, seed| {
                let mut r = base(c, seed, 0x36ce);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] = sx(DATA + 0x300);
                for k in 0..4 {
                    c.bus.write32(DATA + 0x100 + 4 * k, r.next_u32());
                    c.bus.write32(DATA + 0x200 + 4 * k, r.next_u32());
                    c.bus.write32(DATA + 0x300 + 4 * k, r.next_u32());
                }
            },
        );
    }


    // --------------------------------------------------------- memory card

        #[test]
    fn ps2_graphics_man_v94_matches() {
        strict(ee::s12::f_0036a2c0__generated, ps2_graphics_man_v94, 60, &|c, seed| {
            let mut r = base_2(c, seed, 0x36a2, 0);
            let mtx = DATA + 0x800;
            let obj = DATA + 0xa00;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = mtx as u64;
            w(c, DATA + 0xa4, if seed % 4 == 0 { 0 } else { obj });
            for off in (0..0x40u32).step_by(4) {
                w(c, mtx + off, r.edge_f32());
                w(c, obj + off, r.edge_f32());
            }
        });
    }


    /// Compare every observable: hard registers, caller-saved temporaries,
    /// LO/HI, cycles, pc and memory.
    pub(crate) fn same(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed} hard: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} soft: {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} lohi: {:#?}", r.lohi);
        }
    }


    #[test]
    fn emitter_set_desc_path_coeffs_scaled_matches() {
        same(
            ssx3_ee::s12::f_0036d1f0__generated,
            emitter_set_desc_path_coeffs_scaled,
            200,
            &|c, seed| {
                let mut r = Rng(0x36d1f0 ^ seed as u64);
                scramble(c, &mut r);
                let self_at = DATA + 0x100;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (DATA + 0x240) as u64;
                c.g[A3][0] = (DATA + 0x280) as u64;
                c.g[T0][0] = (DATA + 0x2c0) as u64;
                let scale = if seed % 7 == 0 { 0 } else { r.edge_f32() };
                c.bus.write32(self_at + 0x4c, scale);
                for base in [0x200u32, 0x240, 0x280, 0x2c0, 0x80, 0x90, 0xa0] {
                    put_vec(
                        c,
                        DATA + base,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    );
                }
            },
        );
    }


    #[test]
    fn ps2_graphics_man_emit_depth_fog_clut_upload_matches() {
        same(
            ssx3_ee::s12::f_0036aa60__generated,
            ps2_graphics_man_emit_depth_fog_clut_upload,
            200,
            &|c, seed| {
                let mut r = Rng(0x36aa60 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (r.next_u32() % 4096) as u64;
                c.g[A2][0] = (r.next_u32() % 4096) as u64;
                c.g[A3][0] = (DATA + 0x400) as u64;
                let dst = DATA + 0x1000;
                c.bus.write32(DATA + 0x400, dst);
                for i in 0..0x30 {
                    c.bus.write8(dst + i, r.next_u32() as u8);
                }
            },
        );
    }


    #[test]
    fn ps2_graphics_man_match_blended_matrix_matches() {
        same(
            ssx3_ee::s12::f_00369a78__generated,
            ps2_graphics_man_match_blended_matrix,
            200,
            &|c, seed| {
                let mut r = Rng(0x369a78 ^ seed as u64);
                scramble(c, &mut r);
                let a = DATA + 0x100;
                let b = DATA + 0x140;
                let ka = DATA + 0x200;
                let kb = DATA + 0x240;
                c.g[A0][0] = a as u64;
                c.g[A1][0] = b as u64;
                c.g[A2][0] = (r.next_u32() % 4) as u64;
                c.g[A3][0] = (r.next_u32() % 4) as u64;
                c.g[T0][0] = (r.next_u32() % 4) as u64;
                let ty = (seed % 5) as u32;
                c.bus.write32(a, ty);
                c.bus
                    .write32(b, if seed % 4 == 0 { (ty + 1) % 5 } else { ty });
                c.bus.write32(a + 4, ka);
                c.bus.write32(b + 4, kb);
                // Key: id u16 at +2, x/y/z s16 at +0/+4/+8, idA/idB u16 at +6/+0xa.
                for k in [ka, kb] {
                    c.bus.write16(k, (r.next_u32() % 9) as u16);
                    c.bus.write16(k + 2, 1 + (r.next_u32() % 3) as u16);
                    c.bus.write16(k + 4, (r.next_u32() % 9) as u16);
                    c.bus.write16(k + 6, (r.next_u32() % 3) as u16);
                    c.bus.write16(k + 8, (r.next_u32() % 9) as u16);
                    c.bus.write16(k + 0xa, (r.next_u32() % 3) as u16);
                }
                if seed % 2 == 0 {
                    for off in [0u32, 2, 4, 8] {
                        let v = c.bus.read16(ka + off);
                        c.bus.write16(kb + off, v);
                    }
                }
            },
        );
    }
}
