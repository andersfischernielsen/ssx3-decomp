//! `src/visualfx/lensfx.cpp`: hand-written replacements (moved from the
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
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::VSlot;
use callee::*;
pub(crate) use crate::game::be::belibrary::char_bolts_pointer;
pub(crate) use crate::game::be::belibrary::BOLT_RECORDS;
pub(crate) use crate::game::be::belibrary::BOLT_RECORDS_OFF;
pub(crate) use crate::game::render::particle::p;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::render::particle::gp;
pub(crate) use crate::game::render::particle::ldf;
pub(crate) use crate::game::render::particle::fr;
pub(crate) use crate::game::render::particle::setf;
pub(crate) use crate::game::render::particle::fconst;
pub(crate) use crate::game::render::particle::lt;
pub(crate) use crate::game::render::particle::le;
pub(crate) use crate::game::render::particle::add32;
pub(crate) use crate::game::render::particle::shl32;
pub(crate) use crate::game::render::particle::srl32;
pub(crate) use crate::game::fe::ovstatehud::ld_d;
pub(crate) use crate::game::fe::ovstatehud::sd_d;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::gp_2;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_2;
pub(crate) use crate::game::fe::feasyncfile::lh;
pub(crate) use crate::game::fe::feasyncfile::gp_16 as gp_3;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_ad28, be_get_char_profile_data, "BE_GetCharProfileData"),
    (0x0014_bdb8, bolts_get_db, "Bolts_GetDB"),
    (0x0014_d998, bolts_get_char_bolts, "Bolts_GetCharBolts"),
    (0x002e_c9e0, c_bolt_on_fx_interface_ctor, "cBoltOnFXInterface_ctor"),
    (0x002e_ca68, c_bolt_on_fx_interface_ctor_2, "cBoltOnFXInterface_ctor_2"),
    (0x002e_caf0, c_bolt_on_fx_interface_clear_emitters, "cBoltOnFXInterface_ClearEmitters"),
    (0x002e_cb28, bolt_on_fx_interface_dtor, "cBoltOnFXInterface_dtor"),
    (0x002e_cbc0, c_bolt_on_fx_interface_find_or_create_emitter, "cBoltOnFXInterface_FindOrCreateEmitter"),
    (0x002e_cc28, c_bolt_on_fx_interface_create_emitter, "cBoltOnFXInterface_CreateEmitter"),
    (0x002e_cf78, c_bolton_fx_interface_update, "cBoltOnFXInterface_Update"),
    (0x002e_cfa0, world_painter_free_view_queries, "WorldPainter_FreeViewQueries"),
    (0x002e_d338, world_painter_lerp_view_colours, "WorldPainter_LerpViewColours"),
    (0x002e_d490, world_painter_update_view, "WorldPainter_UpdateView"),
    (0x002e_dfb8, world_painter_get_view_irradiance, "WorldPainter_GetViewIrradiance"),
    (0x002e_dfd0, irradiance_ref_resolve, "IrradianceRef_Resolve"),
    (0x002e_e268, world_painter_sun_flare_intensity, "WorldPainter_GetSunFlareIntensity"),
    (0x002e_e2f8, irradiance_get_override, "Irradiance_GetOverride"),
    (0x002e_e318, world_painter_get_irradiance0, "WorldPainter_GetIrradiance0"),
    (0x002e_e340, world_painter_get_irradiance1, "WorldPainter_GetIrradiance1"),
    (0x002e_e368, world_painter_get_irradiance2, "WorldPainter_GetIrradiance2"),
    (0x002e_e6f0, world_painter_get_weather30, "WorldPainter_GetWeather30"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::render::particle::callee::DYNAMIC_EMITTER_SPAWN;
    pub(crate) use crate::game::render::particle::callee::RANDOM;
    pub(crate) use crate::game::render::particle::callee::POINT_BOX_DISTANCE;
    pub(crate) use crate::game::render::particle::callee::IS_HULL_RESIDENT;
    pub(crate) use crate::game::render::particle::callee::REQUEST_HULL;
    pub(crate) use crate::game::render::particle::callee::STREAMER_UPDATE_VIEW_NODE;
    pub(crate) use crate::game::render::particle::callee::HUD_DRAW_PANEL_SEGMENT;
    pub(crate) use crate::game::render::particle::callee::HUD_GET_ELEM_COLOR;
    pub(crate) use crate::game::render::particle::callee::HUD_GET_ELEM_POS;
    pub(crate) use crate::game::render::particle::callee::HUD_GET_ELEM_SIZE;
    pub(crate) use crate::game::render::particle::callee::HUD_ALIGN_ELEM_POS;
    pub(crate) use crate::game::render::particle::callee::HUD_DRAW_SPRITE;
    pub(crate) use crate::game::render::particle::callee::FONT_DRAW_TEXT;
    pub(crate) use crate::game::render::particle::callee::SPATIAL_KEY_FROM_AABB;
    pub(crate) use crate::game::render::particle::callee::SPATIAL_HASH_REMOVE;
    pub(crate) use crate::game::render::particle::callee::SPATIAL_HASH_INSERT;
    pub(crate) use crate::game::render::particle::callee::SPATIAL_HASH_MOVE;
    pub(crate) use crate::game::render::particle::callee::DELETE;
    pub(crate) use crate::game::render::particle::callee::SPOT_LIGHT_VECTOR;
    pub(crate) use crate::game::render::particle::callee::POINT_LIGHT_VECTOR;
    pub(crate) use crate::game::render::particle::callee::ADD_DIRECTIONAL;
    pub(crate) use crate::game::render::particle::callee::SAMPLE_SURFACE_COLOURS;
    pub(crate) use crate::game::render::particle::callee::LERP_VIEW_COLOURS;
    pub(crate) use crate::game::render::particle::callee::BLEND_VIEW_COLOURS;
    pub(crate) use crate::game::render::particle::callee::GET_IRRADIANCE0;
    pub(crate) use crate::game::render::particle::callee::GET_IRRADIANCE1;
    pub(crate) use crate::game::render::particle::callee::GET_IRRADIANCE2;
    pub(crate) use crate::game::render::particle::callee::COMPUTE_COLOUR_FACTOR;
    pub(crate) use crate::game::render::particle::callee::GET_LIGHTING28;
    pub(crate) use crate::game::render::particle::callee::SECTION_GET_EVENT_WORLD_SECTION;
    pub(crate) use crate::game::render::particle::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::render::particle::callee::IRRADIANCE_SCALE;
    pub(crate) use crate::game::render::particle::callee::IRRADIANCE_SCALE_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::IRRADIANCE_ADD;
    pub(crate) use crate::game::be::beintnetwork::callee::MEM_MAN_ALLOC;
    pub(crate) use crate::game::be::beintnetwork::callee::C_BOLTON_FX_REFRESH_EMITTERS;
    pub(crate) use crate::game::be::beintnetwork::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::be::beintnetwork::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME0;
    pub(crate) use crate::game::be::beintnetwork::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME1;
    pub(crate) use crate::game::be::beintnetwork::callee::PATH_QUERY_EVENTS;
    pub(crate) use crate::game::be::beintnetwork::callee::SSX_AUDIO_SYSTEM_GET_APP;
    pub(crate) use crate::game::be::beintnetwork::callee::SSX_AUDIO_SYSTEM_GET_VIEW_COUNT;
    pub(crate) use crate::game::be::beintnetwork::callee::INPUT_VM_INIT;
    pub(crate) use crate::game::be::beintnetwork::callee::INPUT_VM_EXECUTE;
    pub(crate) use crate::game::be::beintnetwork::callee::INPUT_MAP_EVAL_FLOAT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EVENT_SELECT_HANDLE_INPUT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MC_MANAGER_GET_INSTANCE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_UPDATE_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_DUMMY_OBJ_READ_FIELDS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EA_MEM_COPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::TEX_MAN_SET_ALLOC_FLAGS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::GRAPHICS_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_STATE_MGR_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_NODE_UNLINK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_RESET_SECTIONS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STRLEN_AND_STRCPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::CBXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::LOBBY_MAN_REQUEST_STATS_PAGE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SSX_APP_BOOT_LOAD;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::USTR_FROM_ASCII;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_SET_TRACK_CONTROL_VALUE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_INFO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_NAME;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STREAM_BUFFER_GET_STATUS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MENU_SYS_GET_INPUT_CODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::IRRADIANCE_REF_GET_ENTRY as IRRADIANCE_REF_GET_ENTRY_2;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::C_WORLD_SOUND_UNLINK_EMITTERS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RIDER_METRIX_BANK_COMBO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::be::beintoption::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::be::beintoption::callee::INSEQ_GET_INSTANCE;
    pub(crate) use crate::game::be::beintoption::callee::INSEQ_INSTANCE_STOP;
    pub(crate) use crate::game::be::beintoption::callee::SCRIPT_ENGINE_CLEAR_PROCESS;
    pub(crate) use crate::game::be::beintoption::callee::STREAM_ARRAY_STOP;
    pub(crate) use crate::game::be::beintoption::callee::C_WORLD_SOUND_RELOCATE_EMITTERS;
    pub(crate) use crate::game::be::beintoption::callee::USTR_FROM_ASCII_2;
    pub(crate) use crate::game::be::beintoption::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME2;
    pub(crate) use crate::game::be::beintoption::callee::BE_OPTION_INT_GET_MAX_HANDICAP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_BLOCK_POOL_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2 as UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2_2 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_SPRINGS_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_BUILD_MESH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_UPDATE_VERTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_STACK_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME as SND_BANK_IS_LOADED_WITH_NAME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK as SND_LOAD_QUEUE_CANCEL_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD as SND_BANK_LOAD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
}

// ---------------------------------------------------------------------------
// World streamer.

/// Streamer view records: 0x50 bytes each at `wc + 0x250`.
pub(crate) mod view {
    pub(crate) use crate::game::main::game::view::STRIDE;
    pub(crate) use crate::game::main::game::view::EYE;
    pub(crate) use crate::game::main::game::view::BOX;
    pub(crate) use crate::game::main::game::view::LEFT_POINT;
    pub(crate) use crate::game::main::game::view::RIGHT_POINT;
    pub(crate) use crate::game::main::game::view::HULL_RANGE;
    pub(crate) use crate::game::main::game::view::HULL_DIST;
    pub(crate) use crate::game::main::game::view::NEAREST;
}


// ---------------------------------------------------------------- world painter

/// `ViewIrradiance *WorldPainter_GetViewIrradiance(int view)` (0x2edfb8):
/// `&TABLE[view]`, entries of 0xf0 bytes starting at 0x4fa3c0. The index
/// multiply also sets `LO`/`HI`.
pub fn world_painter_get_view_irradiance(c: &mut Cpu) {
    c.cycles += 6;
    const VIEW_SIZE: u32 = 0xf0;
    const TABLE: u32 = 0x0050_0000 - 0x5c40; // 0x004f_a3c0
    c.g[V1][0] = VIEW_SIZE as u64;
    c.g[V0][0] = 0x0050_0000;
    let offset = abi::mult(c, abi::arg(c, 0), VIEW_SIZE);
    c.g[A0][0] = sx(offset);
    c.g[V0][0] = sx(TABLE);
    c.g[V0][0] = sx(TABLE.wrapping_add(offset));
    abi::ret(c);
}


/// `BoltsBolt *Bolts_GetCharBolts(BoltsDB *db, int charIndex)` (0x14d998).
pub fn bolts_get_char_bolts(c: &mut Cpu) {
    char_bolts_pointer(c);
}


/// `void* BE_GetCharProfileData(int char, int char2)` (0x14ad28): index into
/// the character-profile table (`char2*0xf88 + char*0x9b50`).
pub fn be_get_char_profile_data(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V1][0] = 0xf88;
    c.g[A0][0] = 0x9b50;
    c.mult(0, V0, A2, V1);
    c.mult(0, A1, A1, A0);
    c.g[V1][0] = BOLT_RECORDS as u64;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(BOLT_RECORDS_OFF));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}


/// `void* Bolts_GetDB(void)` (0x14bdb8): the bolt database at `0x4a6750`.
pub fn bolts_get_db(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = BOLT_RECORDS as u64;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6750));
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// World painter views.

/// The painter views (0xf0 bytes each).
pub(crate) const WP_VIEWS: u32 = 0x004f_a370;

pub(crate) const WP_VIEW_SIZE: u32 = 0xf0;

/// Default colour pair copied to sp+0/sp+0x10 for the rider's surface.
pub(crate) const WP_DEFAULT_COLOURS: u32 = 0x004f_ab20;

pub(crate) const WP_BLEND_COLOURS: u32 = 0x004f_ab40;

/// Loop heads (event checks).
pub(crate) const WP_NOTIFY_LOOP: u32 = 0x002e_d4e8;

pub(crate) const WP_COPY2_LOOP: u32 = 0x002e_d9ac;

pub(crate) const WP_COPY0_LOOP: u32 = 0x002e_d9ec;

pub(crate) const WP_COPY1_LOOP: u32 = 0x002e_da18;

pub(crate) const WP_COPY_BACK_LOOP: u32 = 0x002e_dad0;


/// `void WorldPainter_UpdateView(int view, float x /*f12*/, float y, float
/// z)` (0x2ed490): tell the view's nine painters about the position
/// (virtual 0x10); for the five player views also: the event world
/// section (view 0), the view's brightness, the rider's surface colours
/// (sampled, or lerped/blended by the rider's state) and the irradiance
/// mix (a cross-fade from the previous one, or the lighting-weighted blend
/// of two irradiance sets) into the view record.
pub fn world_painter_update_view(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x330);
        c.g[V0][0] = WP_VIEW_SIZE as u64;
        frame.save(c, S3, 0x2d0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x2b0);
        c.mult(0, V0, S3, V0);
        c.g[S5][0] = 0x0050_0000;
        frame.save(c, S0, 0x300);
        c.g[V1][0] = WP_VIEWS as u64;
        frame.save(c, S1, 0x2f0);
        frame.save_fpr(c, 22, 0x320);
        c.g[S1][0] = 8;
        frame.save_fpr(c, 21, 0x318);
        c.mov_s(22, 12);
        frame.save_fpr(c, 20, 0x310);
        c.mov_s(21, 13);
        frame.save(c, S2, 0x2e0);
        c.mov_s(20, 14);
        frame.save(c, S4, 0x2c0);
        c.g[S0][0] = add32(c.g[V0][0], c.g[V1][0]);
        frame.save_ra(c, 0x2a0);
        c.g[A1][0] = c.lw(r32(c, S0));
        // The nine painters: virtual 0x10 (position).
        loop {
            c.cycles += 12;
            c.mov_s(12, 22);
            c.mov_s(13, 21);
            c.mov_s(14, 20);
            c.g[V1][0] = c.lw(at(c, A1, 4));
            c.g[S0][0] = add32(c.g[S0][0], 4);
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            let vt = r32(c, V1);
            c.g[A0][0] = c.lh(p(vt, 0x10));
            c.g[V0][0] = c.lw(p(vt, 0x14));
            c.g[A0][0] = add32(c.g[A1][0], c.g[A0][0]);
            let target = r32(c, V0);
            call_ptr(c, target, 0x002e_d510)?;
            if (c.g[S1][0] as i64) < 0 {
                break;
            }
            c.g[A1][0] = c.lw(r32(c, S0));
            if event_due(c) {
                deliver_event_here(c, WP_NOTIFY_LOOP);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = (c.g[S3][0] < 6) as u64;
        if c.g[V0][0] != 0 {
            wp_player_view(c, frame)?;
        }
        frame.restore(c, S0, 0x300);
        c.cycles += 11;
        frame.restore(c, S1, 0x2f0);
        frame.restore(c, S2, 0x2e0);
        frame.restore(c, S3, 0x2d0);
        frame.restore(c, S4, 0x2c0);
        frame.restore(c, S5, 0x2b0);
        frame.restore_ra(c, 0x2a0);
        frame.restore_fpr(c, 22, 0x320);
        frame.restore_fpr(c, 21, 0x318);
        frame.restore_fpr(c, 20, 0x310);
        frame.close(c);
        Ok(())
    })
}


/// Views 0..5. Returns with s0 about to be restored by the caller.
pub(crate) fn wp_player_view(c: &mut Cpu, frame: Frame) -> Flow {
    let _ = frame;
    // View 0 with event sections on: follow the rider's event section
    // (layer at gp+0x770/0xa7c) when it is a type-6 section.
    c.cycles += 3;
    c.g[V0][0] = c.lw(gp(c, 0xa6c));
    let events_on = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(gp(c, -0x848));
    let mut skip_v1 = false;
    if events_on {
        c.cycles += 2;
        let other_view = c.g[S3][0] != 0;
        c.g[V1][0] = WP_VIEW_SIZE as u64;
        if other_view {
            skip_v1 = true;
        } else {
            c.cycles += 16;
            c.g[V0][0] = c.lw(gp(c, -0x848));
            c.g[V1][0] = c.lw(at(c, V0, 0x84));
            c.g[A0][0] = c.lw(at(c, V1, 0x78));
            c.g[A1][0] = c.lw(at(c, A0, 0x1bc));
            call_ptr(c, callee::SECTION_GET_EVENT_WORLD_SECTION, 0x002e_d54c)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[V0][0] = c.lw(gp(c, -0x848));
            c.g[A1][0] = shl32(c.g[A2][0], 3);
            c.g[V1][0] = c.lw(at(c, V0, 0x84));
            c.g[A0][0] = c.lw(at(c, V1, 0x10));
            c.g[V0][0] = c.lw(r32(c, A0));
            c.g[V0][0] = add32(c.g[V0][0], c.g[A1][0]);
            c.g[V1][0] = c.lw(at(c, V0, 0x24));
            c.g[V1][0] ^= 6;
            let not_event = c.g[V1][0] != 0;
            c.g[V0][0] = c.lw(gp(c, -0x848));
            if !not_event {
                c.cycles += 3;
                c.g[V0][0] = c.lw(gp(c, 0xa7c));
                let same = c.g[A2][0] == c.g[V0][0];
                c.g[V0][0] = c.lw(gp(c, -0x848));
                if !same {
                    c.cycles += 3;
                    let v = c.g[A2][0];
                    c.sw(gp(c, 0x770), v);
                    c.sw(gp(c, 0xa7c), v);
                    c.g[V0][0] = c.lw(gp(c, -0x848));
                }
            }
        }
    }
    if !skip_v1 {
        c.cycles += 1;
        c.g[V1][0] = WP_VIEW_SIZE as u64;
    }
    // View brightness (+0x24) and the rider's ground tile (s2).
    c.cycles += 14;
    c.mult(0, V1, S3, V1);
    c.g[A2][0] = shl32(c.g[S3][0], 2);
    c.g[A0][0] = c.lw(at(c, V0, 0x84));
    c.g[A1][0] = WP_VIEWS as u64;
    let k = ldf(c, 1, gp(c, -0x3974));
    c.g[V0][0] = c.lw(at(c, A0, 0xc));
    c.g[V1][0] = add32(c.g[V1][0], c.g[A1][0]);
    c.g[V0][0] = add32(c.g[V0][0], c.g[A2][0]);
    let level = ldf(c, 0, at(c, V1, 0x24));
    c.g[S1][0] = c.lw(at(c, V0, 0x28));
    c.g[A0][0] = add32(c.g[V1][0], 0x24);
    let rider = r32(c, S1);
    c.g[V0][0] = c.lw(p(rider, 0x3fc));
    let boosted = c.g[V0][0] != 0;
    let scaled = level * k;
    setf(c, 1, scaled);
    if boosted {
        c.cycles += 3;
        let add = ldf(c, 0, gp(c, -0x3970));
        setf(c, 0, scaled + add);
    } else {
        c.cycles += 1;
        setf(c, 0, scaled);
    }
    c.cycles += 5;
    c.swc1(0, r32(c, A0));
    c.g[A2][0] = c.lw(p(rider, 0x430));
    c.g[V0][0] = !c.g[A2][0];
    let none = c.g[V0][0] == 0;
    c.g[A1][0] = c.lw(gp(c, 0x16c8));
    if none {
        c.cycles += 1;
        c.g[S2][0] = 0;
    } else {
        // tile = world->layers[id & 0xff]->tiles[id >> 8] (>> 8, * 4).
        c.cycles += 8;
        c.g[V0][0] = shl32(c.g[A2][0] & 0xff, 2);
        c.g[V1][0] = c.lw(r32(c, A1));
        c.g[A0][0] = c.lw(at(c, V1, 8));
        c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
        c.g[V0][0] = c.lw(r32(c, V0));
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.lw(at(c, V0, 0xc));
            c.cycles += 7;
            c.g[V1][0] = shl32(srl32(c.g[A2][0], 8), 2);
            c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
            c.g[A0][0] = c.lw(r32(c, V1));
            c.g[V0][0] = srl32(c.g[A0][0], 8);
            let found = c.g[V0][0] != 0;
            c.g[V0][0] = shl32(c.g[V0][0], 2);
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[V0][0];
        } else {
            c.cycles += 2;
            c.g[A0][0] = 0;
        }
        c.cycles += 2;
        let other_view = c.g[S3][0] != 0;
        c.g[S2][0] = c.g[A0][0];
        if !other_view {
            c.cycles += 4;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            let v = c.g[V0][0];
            c.sw(gp(c, 0x770), v);
            c.sw(gp(c, 0xa7c), v);
        }
    }
    wp_surface_colours(c)?;
    wp_irradiance(c)
}


/// The rider's surface colours into the view (`gp+0xa68`: blend pending).
/// Leaves v0 = 0xf0.
pub(crate) fn wp_surface_colours(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    c.g[A0][0] = c.g[S1][0];
    c.g[S4][0] = add32(c.g[S1][0], 0xaac);
    call_ptr(c, callee::RIDER_GET_MAIN_STATE, 0x002e_d664)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = 1;
    let state1 = c.g[A0][0] == c.g[V0][0];
    c.g[V0][0] = 0x0050_0000;
    let sp = sp(c);
    let pending;
    if state1 {
        wp_lerp_colours(c, sp)?;
        c.g[V0][0] = 1;
        pending = Some(());
    } else {
        // Tile surface type 6 with a type-3 entry: blend toward fixed
        // colours; rider state 4: pending; else sample and blend.
        c.cycles += 2;
        let has_tile = c.g[S2][0] != 0;
        c.g[V0][0] = c.lw(gp(c, 0x16c8));
        let mut special = false;
        if has_tile {
            c.cycles += 4;
            let tile = r32(c, S2);
            c.g[A1][0] = c.lh(p(tile, 0x156));
            c.g[A0][0] = c.lw(r32(c, V0));
            let neg = (c.g[A1][0] as i64) < 0;
            c.g[V0][0] = 0;
            if !neg {
                c.cycles += 7;
                c.g[V0][0] = c.lbu(p(tile, 0x155));
                c.g[V0][0] = shl32(c.g[V0][0], 3);
                c.g[V0][0] = add32(c.g[A0][0], c.g[V0][0]);
                c.g[V1][0] = c.lw(at(c, V0, 0x24));
                c.g[V1][0] ^= 6;
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 0x18;
                    c.mult(0, V1, A1, V0);
                    c.g[V0][0] = add32(c.g[V1][0], c.g[A0][0]);
                    c.g[V1][0] = c.lw(at(c, V0, 0x3f0));
                    c.g[V1][0] ^= 3;
                    c.g[V0][0] = (c.g[V1][0] < 1) as u64;
                }
            }
            c.cycles += 2;
            special = c.g[V0][0] != 0;
        }
        if !special {
            c.cycles += 19;
            c.g[A0][0] = WP_VIEW_SIZE as u64;
            let one = fconst(c, 0, 0x3f80_0000);
            c.mult(0, A0, S3, A0);
            c.g[V0][0] = 1;
            c.g[V1][0] = WP_VIEWS as u64;
            c.g[A2][0] = 0x0050_0000;
            let v = c.g[V0][0];
            c.sw(gp(c, 0xa68), v);
            c.g[A2][0] = WP_BLEND_COLOURS as u64;
            c.swc1(0, p(sp, 0xc));
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
            c.sw(sp, 0);
            c.swc1(0, p(sp, 4));
            c.g[A3][0] = 0;
            c.swc1(0, p(sp, 8));
            let _ = one;
            callee::BLEND_VIEW_COLOURS.call(c, 0x002e_d8c8)?;
            c.g[V0][0] = WP_VIEW_SIZE as u64;
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::RIDER_GET_MAIN_STATE, 0x002e_d8d8)?;
        c.g[V1][0] = 4;
        let state4 = c.g[V0][0] == c.g[V1][0];
        c.g[S0][0] = sx(p(sp, 0x10));
        if state4 {
            c.cycles += 1;
            c.g[V0][0] = 1;
            pending = Some(());
        } else {
            c.cycles += 15;
            let pos = r32(c, S4);
            ldf(c, 13, p(pos, 4));
            ldf(c, 12, pos);
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S0][0];
            callee::SAMPLE_SURFACE_COLOURS.call(c, 0x002e_d908)?;
            c.g[A0][0] = WP_VIEW_SIZE as u64;
            c.g[V0][0] = WP_VIEWS as u64;
            c.mult(0, A0, S3, A0);
            c.g[A3][0] = c.lw(gp(c, 0xa68));
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = add32(c.g[A0][0], c.g[V0][0]);
            callee::BLEND_VIEW_COLOURS.call(c, 0x002e_d928)?;
            c.sw(gp(c, 0xa68), 0);
            pending = None;
        }
    }
    if pending.is_some() {
        c.cycles += 2;
        let v = c.g[V0][0];
        c.sw(gp(c, 0xa68), v);
    }
    c.cycles += 1;
    c.g[V0][0] = WP_VIEW_SIZE as u64;
    Ok(())
}


/// Rider state 1: the default colour pair, then (on the rider's board
/// surface kinds 1 and 3) a lerp factor from the board's travel and the
/// sampled surface; `WorldPainter_LerpViewColours`.
pub(crate) fn wp_lerp_colours(c: &mut Cpu, sp: u32) -> Flow {
    c.cycles += 24;
    c.g[V1][0] = 0x0050_0000;
    c.g[T0][0] = WP_DEFAULT_COLOURS as u64;
    ld_d(c, A1, WP_DEFAULT_COLOURS);
    ld_d(c, A2, p(WP_DEFAULT_COLOURS, 8));
    sd_d(c, A1, sp);
    sd_d(c, A2, p(sp, 8));
    c.g[T0][0] = WP_BLEND_COLOURS as u64;
    ld_d(c, V0, WP_BLEND_COLOURS);
    ld_d(c, A2, p(WP_BLEND_COLOURS, 8));
    sd_d(c, V0, p(sp, 0x10));
    sd_d(c, A2, p(sp, 0x18));
    ldf(c, 1, gp(c, -0x396c));
    let rider = r32(c, S1);
    c.g[V0][0] = c.lw(p(rider, 0x788));
    c.g[V1][0] = c.lw(at(c, V0, 0xac));
    let kind1 = c.g[V1][0] == c.g[A0][0];
    c.g[A1][0] = 0;
    let lerp = if kind1 {
        true
    } else {
        c.cycles += 3;
        c.g[V0][0] = 3;
        c.g[V1][0] == c.g[V0][0]
    };
    if lerp {
        c.cycles += 1;
        c.g[A1][0] = 1;
    }
    c.cycles += 2;
    let on = c.g[A1][0] != 0;
    c.g[A2][0] = sx(p(sp, 0x10));
    if on {
        wp_lerp_factor(c, sp)?;
    }
    c.cycles += 9;
    c.g[A0][0] = WP_VIEW_SIZE as u64;
    c.g[V0][0] = WP_VIEWS as u64;
    c.mult(0, A0, S3, A0);
    c.mov_s(12, 1);
    c.g[A1][0] = c.g[SP][0];
    c.g[A0][0] = add32(c.g[A0][0], c.g[V0][0]);
    callee::LERP_VIEW_COLOURS.call(c, 0x002e_d828)
}


/// The lerp factor (f1) from the board record (rider +0x788): travel
/// `f21 = +0x98 - +0xa0`, a soft limit `f20 = clamp(+0x98 / 2, 2,
/// +0x98)`; below it a ratio, beyond it the sampled surface colours.
pub(crate) fn wp_lerp_factor(c: &mut Cpu, sp: u32) -> Flow {
    c.cycles += 9;
    let rider = r32(c, S1);
    c.g[V0][0] = c.lw(p(rider, 0x788));
    let board = r32(c, V0);
    c.g[A0][0] = c.lw(p(board, 0x30));
    c.g[T0][0] = c.g[V0][0];
    let span = ldf(c, 2, p(board, 0x98));
    let pos = ldf(c, 0, p(board, 0xa0));
    c.g[A3][0] = c.g[A0][0];
    c.g[V1][0] = !c.g[A0][0];
    let no_surface = c.g[V1][0] == 0;
    let travel = span - pos;
    setf(c, 21, travel);
    if no_surface {
        return Ok(());
    }
    c.cycles += 11;
    let half = fconst(c, 1, 0x3f00_0000);
    let two = fconst(c, 0, 0x4000_0000);
    let limit = (span * half).max(two).min(span);
    setf(c, 20, limit);
    let beyond = lt(c, limit, travel);
    setf(c, 2, span - limit);
    if beyond {
        c.cycles += 9;
        let rest = fr(c, 2);
        let d = travel - limit;
        setf(c, 0, d);
        let cap = ldf(c, 3, gp(c, -0x3968));
        setf(c, 1, FpuF32::ZERO);
        let r = (rest - d).div(rest);
        setf(c, 0, r);
        let pos = le(c, FpuF32::ZERO, r);
        setf(c, 12, FpuF32::ZERO);
        if pos {
            c.cycles += 1;
            setf(c, 12, r.min(cap));
        }
        c.cycles += 2;
        c.mov_s(1, 12);
        return Ok(());
    }
    // The tile under the surface id (a3).
    c.cycles += 9;
    c.g[A1][0] = c.lw(gp(c, 0x16c8));
    c.g[V0][0] = shl32(c.g[A0][0] & 0xff, 2);
    c.g[V1][0] = c.lw(r32(c, A1));
    c.g[A0][0] = c.lw(at(c, V1, 8));
    c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
    c.g[V1][0] = c.lw(r32(c, V0));
    let none = c.g[V1][0] == 0;
    c.g[A0][0] = 0;
    if !none {
        c.cycles += 8;
        c.g[A0][0] = c.lw(at(c, V1, 0xc));
        c.g[V0][0] = shl32(srl32(c.g[A3][0], 8), 2);
        c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
        c.g[V1][0] = c.lw(r32(c, V0));
        c.g[V0][0] = srl32(c.g[V1][0], 8);
        let none = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        if !none {
            c.cycles += 1;
            c.g[A0][0] = shl32(c.g[V0][0], 2);
        }
    }
    c.cycles += 11;
    c.g[S0][0] = sx(p(sp, 0x10));
    let board = r32(c, T0);
    ldf(c, 13, p(board, 0x38));
    ldf(c, 12, p(board, 0x34));
    c.g[A2][0] = c.g[S0][0];
    c.g[A1][0] = c.g[SP][0];
    callee::SAMPLE_SURFACE_COLOURS.call(c, 0x002e_d7cc)?;
    setf(c, 2, FpuF32::ZERO);
    let limit = fr(c, 20);
    let positive = lt(c, FpuF32::ZERO, limit);
    c.g[A2][0] = c.g[S0][0];
    if positive {
        c.cycles += 7;
        let d = limit - fr(c, 21);
        setf(c, 0, d);
        let cap = ldf(c, 3, gp(c, -0x3964));
        let r = d.div(limit);
        setf(c, 0, r);
        let pos = le(c, FpuF32::ZERO, r);
        c.mov_s(1, 2);
        if pos {
            c.cycles += 2;
            setf(c, 1, r.min(cap));
        }
        return Ok(());
    }
    c.cycles += 2;
    fconst(c, 1, 0x3f80_0000);
    Ok(())
}


/// Copy `n` 0x20-byte blocks with two `lq`/`sq` pairs through `r0`/`r1`
/// (`src`/`dst` registers advanced, loop until `src == end_reg`), event
/// checks at `head`.
pub(crate) fn wp_copy_blocks(c: &mut Cpu, src: usize, dst: usize, end_reg: usize, r0: usize, r1: usize, head: u32) {
    loop {
        c.cycles += 7;
        let s = r32(c, src);
        let d = r32(c, dst);
        c.lq(r0, s);
        c.lq(r1, p(s, 0x10));
        c.sq(r0, d);
        c.sq(r1, p(d, 0x10));
        c.g[src][0] = sx(p(s, 0x20));
        let more = c.g[src][0] != c.g[end_reg][0];
        c.g[dst][0] = sx(p(r32(c, dst), 0x20));
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}


/// The view's irradiance (+0x50, 0xa0 bytes): a cross-fade (fade >
/// threshold) or the lighting-weighted mix of sets 0 and 1.
pub(crate) fn wp_irradiance(c: &mut Cpu) -> Flow {
    let sp = sp(c);
    c.cycles += 9;
    c.g[V1][0] = WP_VIEWS as u64;
    c.mult(0, A2, S3, V0);
    let threshold = ldf(c, 1, gp(c, -0x3960));
    c.g[V0][0] = add32(c.g[A2][0], c.g[V1][0]);
    let fade = ldf(c, 0, at(c, V0, 0x24));
    let fading = lt(c, threshold, fade);
    c.g[A0][0] = 1;
    if !fading {
        c.cycles += 1;
        c.g[A0][0] = 0;
    }
    c.cycles += 2;
    let fading = c.g[A0][0] != 0;
    c.g[S0][0] = add32(c.g[V1][0], 0x50);
    if fading {
        c.cycles += 19;
        c.g[S2][0] = sx(p(sp, 0xc0));
        c.g[S0][0] = add32(c.g[A2][0], c.g[S0][0]);
        c.g[S1][0] = sx(p(sp, 0x20));
        c.g[A0][0] = c.g[S3][0];
        callee::GET_IRRADIANCE2.call(c, 0x002e_d974)?;
        ldf(c, 12, gp(c, 0xa80));
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, callee::IRRADIANCE_SCALE, 0x002e_d984)?;
        let t = ldf(c, 0, gp(c, 0xa80));
        c.g[A0][0] = c.g[S0][0];
        let one = fconst(c, 12, 0x3f80_0000);
        setf(c, 12, one - t);
        call_ptr(c, callee::IRRADIANCE_SCALE_IN_PLACE, 0x002e_d99c)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, callee::IRRADIANCE_ADD, 0x002e_d9ac)?;
        wp_copy_blocks(c, S1, S0, S2, A0, A1, WP_COPY2_LOOP);
        c.cycles += 2;
        return Ok(());
    }
    // Sets 0 and 1 to sp+0 and sp+0xa0.
    c.cycles += 7;
    c.g[A0][0] = c.g[S3][0];
    callee::GET_IRRADIANCE0.call(c, 0x002e_d9d8)?;
    c.g[S4][0] = sx(p(sp, 0xa0));
    c.g[S5][0] = sx(p(sp, 0x160));
    c.g[S2][0] = sx(p(sp, 0x200));
    c.g[A0][0] = add32(c.g[V0][0], 0xa0);
    c.g[V1][0] = c.g[SP][0];
    wp_copy_blocks(c, V0, V1, A0, T0, A1, WP_COPY0_LOOP);
    c.cycles += 4;
    c.g[A0][0] = c.g[S3][0];
    callee::GET_IRRADIANCE1.call(c, 0x002e_da10)?;
    c.g[V1][0] = c.g[S4][0];
    c.g[A0][0] = add32(c.g[V0][0], 0xa0);
    wp_copy_blocks(c, V0, V1, A0, T0, A1, WP_COPY1_LOOP);
    // w = factor * lighting; mix = set0 * w + set1 * (1 - w); then the
    // cross-fade with the view's previous irradiance (+0x68).
    c.cycles += 39;
    c.g[S1][0] = WP_VIEW_SIZE as u64;
    c.g[S0][0] = 0x0050_0000;
    c.mult(0, S1, S3, S1);
    c.g[S0][0] = sx(0x004f_a3a8);
    let one = fconst(c, 21, 0x3f80_0000);
    c.g[A0][0] = add32(c.g[S1][0], c.g[S0][0]);
    callee::COMPUTE_COLOUR_FACTOR.call(c, 0x002e_da54)?;
    c.mov_s(20, 0);
    c.g[A0][0] = c.g[S3][0];
    callee::GET_LIGHTING28.call(c, 0x002e_da60)?;
    let w = fr(c, 20) * fr(c, 0);
    setf(c, 20, w);
    c.g[S0][0] = add32(c.g[S0][0], 0x18);
    c.g[A0][0] = c.g[SP][0];
    c.g[S1][0] = add32(c.g[S1][0], c.g[S0][0]);
    setf(c, 12, w);
    call_ptr(c, callee::IRRADIANCE_SCALE_IN_PLACE, 0x002e_da78)?;
    setf(c, 12, fr(c, 21) - fr(c, 20));
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S4][0];
    call_ptr(c, callee::IRRADIANCE_SCALE_IN_PLACE, 0x002e_da88)?;
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, callee::IRRADIANCE_ADD, 0x002e_da98)?;
    ldf(c, 12, gp(c, 0xa80));
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, callee::IRRADIANCE_SCALE_IN_PLACE, 0x002e_daa4)?;
    let t = ldf(c, 12, gp(c, 0xa80));
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    setf(c, 12, fr(c, 21) - t);
    let _ = one;
    call_ptr(c, callee::IRRADIANCE_SCALE_IN_PLACE, 0x002e_dab8)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, callee::IRRADIANCE_ADD, 0x002e_dac8)?;
    c.g[V0][0] = c.g[S2][0];
    c.g[A0][0] = c.g[S5][0];
    wp_copy_blocks(c, A0, S1, V0, T0, V1, WP_COPY_BACK_LOOP);
    c.cycles += 1;
    Ok(())
}


// ---------------------------------------------------------------------------
// IrradianceRef_Resolve.

/// `void IrradianceRef_Resolve(IrradianceRef* r)` (0x2edfd0): store the
/// manager's handle for `r` (`func_0038AC50(&D_005047F8, r)`) in `r->handle`
/// (+0) and the type tag 0x123400 in `r->tag` (+4).
pub fn irradiance_ref_resolve(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x50_0000;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x47f8));
        call_ptr(c, 0x0038_ac50, 0x002e_dff0)?;
        c.g[V1][0] = 0x12_0000;
        let handle = c.g[V0][0];
        let this = r32(c, S0);
        Addr(this).sw(c, handle);
        c.g[V1][0] = c.g[V1][0] | 0x3400;
        frame.restore_ra(c, 0);
        let tag = c.g[V1][0];
        Addr(this.wrapping_add(0x4)).sw(c, tag);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void WorldPainter_GetIrradiance0()` (0x2ee318): look up entry 0 of the
/// irradiance table by name.
pub fn world_painter_get_irradiance0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::WORLD_PAINTER_GET_IRRADIANCE_NAME0.call(c, 0x002e_e328)?;
        c.g[A0][0] = c.g[V0][0];
        callee::IRRADIANCE_REF_GET_ENTRY.call(c, 0x002e_e330)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void WorldPainter_GetIrradiance1()` (0x2ee340): look up entry 1 of the
/// irradiance table by name.
pub fn world_painter_get_irradiance1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::WORLD_PAINTER_GET_IRRADIANCE_NAME1.call(c, 0x002e_e350)?;
        c.g[A0][0] = c.g[V0][0];
        callee::IRRADIANCE_REF_GET_ENTRY.call(c, 0x002e_e358)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Bolt-on FX.

/// `void cBoltOnFXInterface_Update(cBoltOnFXInterface *self)` (0x2ecf78):
/// refresh the emitters when the interface is attached to a world.
pub fn c_bolton_fx_interface_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let selfp = Addr(abi::arg(c, 0));
        let attached = selfp.lw(c);
        c.g[V0][0] = attached;
        frame.save_ra(c, 0);
        if attached != 0 {
            c.cycles += 2;
            callee::C_BOLTON_FX_REFRESH_EMITTERS.call(c, 0x002e_cf90)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// World painter getters.

/// The lens-fx / render-state entry table `D_004FA370` (built as
/// `0x500000 - 0x5c90`) with stride 0xf0; the entry's object pointer is at
/// `+0x14` (lens fx) or `+0x1c` (render state).

/// `float WorldPainter_GetSunFlareIntensity(int i)` (0x2ee268): call virtual
/// slot 34 on the object of lens-fx entry `i` and return the float it
/// returns.
pub fn world_painter_sun_flare_intensity(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 18;
        c.g[V1][0] = 0xf0;
        c.mult(0, A0, A0, V1);
        c.g[V0][0] = 0x50_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5c90));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(at(c, A0, 0x14));
        c.g[A2][0] = c.lw(r32(c, V1));
        c.g[A1][0] = c.lw(at(c, A2, 0x4));
        c.g[A0][0] = c.lh(at(c, A1, 0x110));
        c.g[V0][0] = c.lw(at(c, A1, 0x114));
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002e_e2a0)?;
        c.lwc1(0, r32(c, V0));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// World painter view colours.

/// `void WorldPainter_LerpViewColours(WorldPainter* self, const Vec4* a,
/// const Vec4* b, float t)` (0x2ed338): `self->colour0/1 = old * (1 - t) +
/// new * t` (two 16-byte colour quads at self+0x28 and self+0x38).
pub fn world_painter_lerp_view_colours(c: &mut Cpu) {
    c.cycles += 86;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
    let this = abi::arg(c, 0);
    let a = abi::arg(c, 1);
    let b = abi::arg(c, 2);
    // s = 1 - t, kept in f1.
    c.lwc1(5, p(this, 0x28));
    c.sub_s(1, 1, 12);
    c.lwc1(4, p(this, 0x2c));
    c.lwc1(3, p(this, 0x30));
    c.lwc1(2, p(this, 0x34));
    c.lwc1(8, p(a, 0x0c));
    c.mul_s(5, 5, 1);
    c.lwc1(0, a);
    c.mul_s(4, 4, 1);
    c.lwc1(7, p(a, 0x04));
    c.mul_s(3, 3, 1);
    c.lwc1(6, p(a, 0x08));
    c.mul_s(2, 2, 1);
    c.mul_s(8, 8, 12);
    c.swc1(5, p(sp(c), 0x10));
    c.mul_s(0, 0, 12);
    c.swc1(4, p(sp(c), 0x14));
    c.mul_s(7, 7, 12);
    c.swc1(3, p(sp(c), 0x18));
    c.mul_s(6, 6, 12);
    c.swc1(2, p(sp(c), 0x1c));
    c.add_s(5, 5, 0);
    c.swc1(0, p(sp(c), 0x20));
    c.add_s(4, 4, 7);
    c.swc1(7, p(sp(c), 0x24));
    c.add_s(3, 3, 6);
    c.swc1(6, p(sp(c), 0x28));
    c.add_s(2, 2, 8);
    c.swc1(5, sp(c));
    c.swc1(4, p(sp(c), 0x04));
    c.swc1(3, p(sp(c), 0x08));
    c.swc1(2, p(sp(c), 0x0c));
    c.swc1(8, p(sp(c), 0x2c));
    c.ldl(2, p(sp(c), 0x07));
    c.ldr(2, sp(c));
    c.ldl(3, p(sp(c), 0x0f));
    c.ldr(3, p(sp(c), 0x08));
    c.sdl(2, p(this, 0x2f));
    c.sdr(2, p(this, 0x28));
    c.sdl(3, p(this, 0x37));
    c.sdr(3, p(this, 0x30));
    // Second colour quad.
    c.lwc1(4, p(this, 0x38));
    c.lwc1(5, p(this, 0x3c));
    c.lwc1(3, p(this, 0x40));
    c.mul_s(4, 4, 1);
    c.lwc1(2, p(this, 0x44));
    c.mul_s(5, 5, 1);
    c.lwc1(8, p(b, 0x0c));
    c.mul_s(3, 3, 1);
    c.lwc1(0, b);
    c.mul_s(2, 2, 1);
    c.lwc1(7, p(b, 0x04));
    c.mul_s(8, 8, 12);
    c.lwc1(6, p(b, 0x08));
    c.mul_s(0, 0, 12);
    c.mul_s(7, 7, 12);
    c.swc1(4, p(sp(c), 0x10));
    c.mul_s(6, 6, 12);
    c.swc1(5, p(sp(c), 0x14));
    c.swc1(3, p(sp(c), 0x18));
    c.add_s(4, 4, 0);
    c.swc1(2, p(sp(c), 0x1c));
    c.add_s(5, 5, 7);
    c.add_s(3, 3, 6);
    c.swc1(0, p(sp(c), 0x20));
    c.add_s(2, 2, 8);
    c.swc1(4, sp(c));
    c.swc1(5, p(sp(c), 0x04));
    c.swc1(3, p(sp(c), 0x08));
    c.swc1(2, p(sp(c), 0x0c));
    c.swc1(7, p(sp(c), 0x24));
    c.swc1(6, p(sp(c), 0x28));
    c.swc1(8, p(sp(c), 0x2c));
    c.ldl(2, p(sp(c), 0x07));
    c.ldr(2, sp(c));
    c.ldl(3, p(sp(c), 0x0f));
    c.ldr(3, p(sp(c), 0x08));
    c.sdl(2, p(this, 0x3f));
    c.sdr(2, p(this, 0x38));
    c.sdl(3, p(this, 0x47));
    c.sdr(3, p(this, 0x40));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
    c.pc = ra;
}


/// `void Irradiance_GetOverride(void)` (0x002ee2f8): look up the global lens
/// FX irradiance entry at `gp_2 + 0x24c0`.
pub fn irradiance_get_override(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = sx(gp_2(c, 0x24c0)),
        callee::IRRADIANCE_REF_GET_ENTRY_2,
        0x002e_e30c,
    )
}


/// `Irradiance *WorldPainter_GetIrradiance2(void)` (0x2ee368): resolve the
/// painter's irradiance name to a reference, then look the entry up.
pub fn world_painter_get_irradiance2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::WORLD_PAINTER_GET_IRRADIANCE_NAME2.call(c, 0x002e_e378)?;
        c.g[A0][0] = c.g[V0][0];
        callee::IRRADIANCE_REF_GET_ENTRY.call(c, 0x002e_e380)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cBoltOnFXInterface_ClearEmitters(cBoltOnFXInterface* fx)`
/// (0x2ecaf0): invalidate the 15 emitter slots (`id = -1`).
pub fn c_bolt_on_fx_interface_clear_emitters(c: &mut Cpu) {
    const HEAD: u32 = 0x002e_cb00;
    c.cycles += 4;
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0xb8));
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = 0xe;
    loop {
        c.cycles += 7;
        Addr(r32(c, A0)).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0xc));
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


// ---------------------------------------------------------------------------
// WorldPainter / lens effects.

/// `float WorldPainter_GetWeather30(int i)` (0x2ee6f0): the lens-effect slot
/// `i` (`D_004FA370 + i * 0xf0`) asks its object's virtual weather accessor
/// (vtable slot 0x180/0x184) for the current weather blend.
pub fn world_painter_get_weather30(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = 0xf0; // sizeof(sLensFxSlot)
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1); // a0 = i * 0xf0
        c.g[V0][0] = 0x0050_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x0050_0000u32.wrapping_sub(0x5c90)); // &D_004FA370
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let slot = c.g[A0][0] as u32;
        c.g[V1][0] = Addr(slot).off(0x20).lw(c); // s->p20
        let obj = Addr(c.g[V1][0] as u32).lw(c); // *s->p20
        c.g[A2][0] = obj;
        let vt = Addr((obj as u32).wrapping_add(4)).lw(c); // obj->vt
        c.g[A1][0] = vt;
        c.g[A0][0] = Addr((vt as u32).wrapping_add(0x180)).lh(c); // delta
        c.g[V0][0] = Addr((vt as u32).wrapping_add(0x184)).lw(c); // fn
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x002e_e728);
        c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002e_e728)?;
        c.lwc1(0, c.g[V0][0] as u32); // f0 = weather blend
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int cBoltOnFXInterface_FindOrCreateEmitter(cBoltOnFXInterface* self,
/// int a1, int a2)` (0x2ecbc0): find the first of the 15 `{id, a, b}` slots
/// matching `(a2, a1)` (id != -1) and return its index, else create one.
pub fn c_bolt_on_fx_interface_find_or_create_emitter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[T0][0] = 0xffff_ffff_ffff_ffff;
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        let mut i = c.g[A3][0];
        let mut slot = c.g[V1][0];
        loop {
            let mut found = false;
            c.cycles += 3;
            c.g[V0][0] = lw(c, (slot as u32).wrapping_sub(8));
            if c.g[V0][0] == c.g[T0][0] {
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw(c, (slot as u32).wrapping_sub(4));
                if c.g[V0][0] != c.g[A2][0] {
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, slot as u32);
                    if c.g[V0][0] != c.g[A1][0] {
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                    } else {
                        c.cycles += 2;
                        found = true;
                    }
                }
            }
            i = c.g[A3][0];
            slot = c.g[V1][0];
            if found {
                c.g[V0][0] = i;
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((i as i64) < 15) as u64;
            let more = c.g[V0][0] != 0;
            slot = sx((slot as u32).wrapping_add(0xc));
            c.g[V1][0] = slot;
            if !more {
                c.cycles += 2;
                call_ptr(c, 0x002e_cc28, 0x002e_cc18)?;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002e_cbd8);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Bolt-on effects.

/// Shared body of the two `cBoltOnFXInterface` constructors: publish the
/// vtable, store the two owner words, invalidate the 15 emitter handles and
/// clear the emitters. `swapped` is true for the variant that takes its
/// primary argument in `a2`; `rets` are the original's three return addresses.
pub fn c_bolt_on_fx_interface_ctor_common(
    c: &mut Cpu,
    swapped: bool,
    rets: [u32; 3],
    head: u32,
) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S0, 0x40);
        c.g[V0][0] = sx(0x0048_7ee8);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[V1][0] = 0x1;
        frame.save(c, S3, 0x10);
        c.g[S1][0] = sx(p(r32(c, S0), 0x10));
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xe;
        Addr(p(r32(c, S0), 0xc4)).sw(c, c.g[V0][0]);
        c.g[S3][0] = u64::MAX;
        Addr(r32(c, S0)).sw(c, c.g[V1][0]);
        if swapped {
            Addr(p(r32(c, S0), 8)).sw(c, c.g[A1][0]);
            Addr(p(r32(c, S0), 0xc)).sw(c, c.g[A2][0]);
            Addr(p(r32(c, S0), 4)).sw(c, 0);
        } else {
            Addr(p(r32(c, S0), 4)).sw(c, c.g[A1][0]);
            Addr(p(r32(c, S0), 0xc)).sw(c, c.g[A2][0]);
            Addr(p(r32(c, S0), 8)).sw(c, 0);
        }
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(rets[0]);
            c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
            callee::BOLT_ON_FX_INIT_HANDLE.call(c, rets[0])?;
            let more = c.g[S2][0] != c.g[S3][0];
            c.g[S1][0] = sx(p(r32(c, S1), 0xc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
        c.cycles += 10;
        c.g[RA][0] = sx(rets[2]);
        c.g[A0][0] = c.g[S0][0];
        callee::BOLT_ON_FX_CLEAR_EMITTERS.call(c, rets[2])?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* cBoltOnFXInterface_ctor(cBoltOnFXInterface* self, void* a1, void* a2)`
/// (0x2ec9e0).
pub fn c_bolt_on_fx_interface_ctor(c: &mut Cpu) {
    c_bolt_on_fx_interface_ctor_common(
        c,
        false,
        [0x002e_ca34, 0x002e_cab0, 0x002e_ca44],
        0x002e_ca28,
    );
}


/// `void* cBoltOnFXInterface_ctor_2(cBoltOnFXInterface* self, void* a1,
/// void* a2)` (0x2eca68): the variant that stores `a1` at +0x8.
pub fn c_bolt_on_fx_interface_ctor_2(c: &mut Cpu) {
    c_bolt_on_fx_interface_ctor_common(
        c,
        true,
        [0x002e_cabc, 0x002e_cab0, 0x002e_cacc],
        0x002e_cab0,
    );
}


// ---------------------------------------------------------------------------
// Lens FX emitters.

/// `int cBoltOnFXInterface_CreateEmitter(LensFX* self, void* x, void* y)`
/// (0x2ecc28): take the next free emitter slot (of 15) and start an attached
/// dynamic emitter for it.
pub fn c_bolt_on_fx_interface_create_emitter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = u64::MAX;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = sx(p(r32(c, A0), 0x10));
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A2][0];
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            if c.g[V0][0] != c.g[V1][0] {
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 15i64) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0xc));
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002e_cc58);
                }
                continue;
            }
            c.cycles += 9;
            c.g[A0][0] = Addr(p(r32(c, A0), 0xc)).lw(c);
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            callee::ATTACHED_DYNAMIC_EMITTER_CREATE.call(c, 0x002e_cc74)?;
            Addr(r32(c, S0)).sw(c, c.g[V0][0]);
            Addr(p(r32(c, S0), 8)).sw(c, c.g[S2][0]);
            c.g[V0][0] = c.g[S1][0];
            Addr(p(r32(c, S0), 4)).sw(c, c.g[S3][0]);
            break;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cBoltOnFXInterface_dtor` (0x2ecb28): destroy the interface's fifteen
/// attached emitter slots and optionally free the object.
pub fn bolt_on_fx_interface_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S2, 0x30);
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_add(0x7ee8));
        frame.save(c, S0, 0x50);
        let this = r(c, A0);
        c.g[S2][0] = sx(this);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx(this.wrapping_add(0x10));
        frame.save(c, S3, 0x20);
        c.g[S1][0] = 0xe;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A1][0];
        Addr(p(this, 0xc4)).sw(c, c.g[V0][0]);
        loop {
            c.cycles += 3;
            c.g[A1][0] = Addr(r(c, S0)).lw(c);
            if c.g[A1][0] != c.g[S3][0] {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002e_cb7c);
                c.g[A0][0] = Addr(p(r(c, S2), 0xc)).lw(c);
                call_ptr(c, callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER, 0x002e_cb7c)?;
                Addr(r(c, S0)).sw(c, c.g[S3][0]);
            }
            c.cycles += 2;
            c.g[S1][0] = sx(r(c, S1).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r(c, S0).wrapping_add(0xc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002e_cb68);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S4][0] & 1;
        let free_self = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x50);
        if free_self {
            c.cycles += 3;
            c.g[RA][0] = sx(0x002e_cba0);
            c.g[A0][0] = c.g[S2][0];
            callee::BUILTIN_DELETE_2.call(c, 0x002e_cba0)?;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void WorldPainter_FreeViewQueries()` (0x2ecfa0): release the 8 view
/// queries of every painter (the table is at_2 0x9fa370).
pub fn world_painter_free_view_queries(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x60);
        'end: {
            c.cycles += 9;
            c.g[V0][0] = lw(c, gp_3(c, 0xa64));
            frame.save(c, S0, 0x50);
            frame.save(c, S1, 0x40);
            frame.save(c, S2, 0x30);
            frame.save(c, S3, 0x20);
            frame.save(c, S4, 0x10);
            let t = c.g[V0][0] == 0;
            frame.save_ra(c, 0);
            if t {
                break 'end;
            }
            c.cycles += 5;
            c.g[V0][0] = sx(0x0050_0000);
            c.g[V1][0] = 0;
            c.g[S4][0] = sx(0x004f_a370);
            c.g[S3][0] = 0xf0;
            c.mult(0, 2, 3, 19);
            loop {
                c.cycles += 4;
                c.g[S2][0] = sx(at_2(c.g[V1][0], 1));
                c.g[S1][0] = 8;
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                loop {
                    {
                        c.cycles += 3;
                        c.g[A2][0] = lw(c, c.g[S0][0] as u32);
                        let t = c.g[A2][0] == 0;
                        c.g[A1][0] = 3;
                        if !t {
                            c.cycles += 5;
                            c.g[V0][0] = lw(c, at_2(c.g[A2][0], 4));
                            c.g[A0][0] = lh(c, at_2(c.g[V0][0], 8));
                            c.g[V1][0] = lw(c, at_2(c.g[V0][0], 0xc));
                            let target = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x002e_d008);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, target, 0x002e_d008)?;
                        }
                    }
                    c.cycles += 3;
                    c.g[S1][0] = sx(at_2(c.g[S1][0], 0xffff_ffff));
                    let t = (c.g[S1][0] as i64) >= 0;
                    c.g[S0][0] = sx(at_2(c.g[S0][0], 4));
                    if !t {
                        c.cycles += 4;
                        c.g[V1][0] = c.g[S2][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 8i64) as u64;
                        let t = c.g[V0][0] != 0;
                        c.mult(0, 2, 3, 19);
                        if !t {
                            break 'end;
                        }
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x002e_cfd8);
                        }
                        break;
                    }
                    if crate::events::event_due(c) {
                        crate::events::deliver_event_here(c, 0x002e_cfe8);
                    }
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        Addr(gp_3(c, 0xa64)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::SP as TEST_SP;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::put_vec;
    use crate::difftest::SP;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::RA as TEST_RA;
    use ssx3_ee_rt::funcs::GuestFn;
    pub(crate) use crate::game::bx::appman::tests::SEEDS;
    pub(crate) use crate::game::bx::appman::tests::rng;
    pub(crate) use crate::game::render::particle::tests::f;
    pub(crate) use crate::game::render::particle::tests::uf;
    pub(crate) use crate::game::render::particle::tests::put_vec3;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::SEEDS_2;
    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::bx::bxstring2::tests::game_globals;
    pub(crate) use crate::game::bx::bxstring2::tests::game;
    pub(crate) use crate::game::sound::bankmonitor::tests::difftest_gp;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::bx::bxstring::tests::heap_init_4 as heap_init;
    pub(crate) use crate::game::bx::bxstring::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator;
    pub(crate) use crate::game::input::inputmap::tests::alloc;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH as SCRATCH_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_3;


    #[test]
    fn world_painter_get_view_irradiance_matches() {
        check(s09::f_002edfb8__generated, world_painter_get_view_irradiance, SEEDS, &|c, seed| {
            let mut r = rng(seed, 3);
            scramble(c, &mut r);
            c.g[A0][0] = (match seed % 4 {
                0 => 0,
                1 => 1,
                2 => u32::MAX,
                _ => r.next_u32(),
            }) as u64;
        });
    }


    #[test]
    fn bolts_get_char_bolts_matches() {
        check_lohi(ssx3_ee::s01::f_0014d998, bolts_get_char_bolts, 300, &|c, seed| {
            let mut r = Rng(0x3000_0003 ^ seed as u64);
            scramble(c, &mut r);
            let idx = seed % 4;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = idx as u64;
            let record = DATA + idx * 4;
            let count = if seed % 2 == 0 {
                r.next_u32() | 0x8000_0000
            } else {
                (r.next_u32() & 0x7fff_ffff) | 1
            };
            c.bus.write32(record + 0x2c, count);
            c.bus.write32(record + 0xa4, r.next_u32());
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }


    #[test]
    fn be_get_char_profile_data_matches() {
        check_lohi(ssx3_ee::s01::f_0014ad28, be_get_char_profile_data, 200, &|c, seed| {
            let mut r = Rng(0x14ad ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = (r.next_u32() % 16) as u64;
            c.g[A2][0] = (r.next_u32() % 16) as u64;
        });
    }


    #[test]
    fn bolts_get_db_matches() {
        check(ssx3_ee::s01::f_0014bdb8, bolts_get_db, 32, &|c, seed| {
            let mut r = Rng(0x14bd ^ seed as u64);
            scramble(c, &mut r);
        });
    }


    #[test]
    fn world_painter_update_view_matches() {
        check(ee::s09::f_002ed490__generated, world_painter_update_view, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2ed4, 12);
            let view = seed % 7;
            // The nine painters of every view: virtual 0x10 is a fake.
            let vt = DATA + 0x500;
            c.bus.write16(vt + 0x10, 0);
            c.bus.write32(vt + 0x14, FAKE);
            for v in 0..6u32 {
                let rec = WP_VIEWS + WP_VIEW_SIZE * v;
                for i in 0..9u32 {
                    let painter = DATA + 0x600 + 0x10 * i;
                    c.bus.write32(painter + 4, vt);
                    c.bus.write32(rec + 4 * i, painter);
                }
                c.bus.write32(rec + 0x24, f(uf(&mut r, 0.0, 1.0)));
                fill(c, &mut r, rec + 0x50, 0x28);
            }
            c.f[12] = f(uf(&mut r, -100.0, 100.0));
            c.f[13] = f(uf(&mut r, -100.0, 100.0));
            c.f[14] = f(uf(&mut r, -100.0, 100.0));
            // World, sections, riders.
            let (world, x, y, z, sections, riders, rider, board) = (
                DATA + 0x1_0000,
                DATA + 0x1_0100,
                DATA + 0x1_0200,
                DATA + 0x1_0300,
                DATA + 0x1_0400,
                DATA + 0x1_0500,
                DATA + 0x1_1000,
                DATA + 0x1_2000,
            );
            c.bus.write32(gp_at(0xa6c), seed / 7 % 2);
            c.bus.write32(gp_at(0xa74), 0);
            c.bus.write32(gp_at(0xa7c), r.next_u32() % 3);
            c.bus.write32(gp_at(-0x848), world);
            c.bus.write32(world + 0x84, x);
            c.bus.write32(x + 0x78, y);
            c.bus.write32(y + 0x1bc, r.next_u32() % 4);
            c.bus.write32(x + 0x10, z);
            c.bus.write32(z, sections);
            for i in 0..16 {
                c.bus.write32(sections + 8 * i + 0x24, if r.next_u32() % 2 == 0 { 6 } else { 2 });
            }
            c.bus.write32(x + 0xc, riders);
            for v in 0..6u32 {
                c.bus.write32(riders + 4 * v + 0x28, rider);
            }
            for off in [-0x3974, -0x3970, -0x396c, -0x3968, -0x3964, -0x3960] {
                c.bus.write32(gp_at(off), f(uf(&mut r, 0.0, 1.0)));
            }
            c.bus.write32(gp_at(0xa80), f(uf(&mut r, 0.0, 1.0)));
            c.bus.write32(rider + 0x3fc, r.next_u32() % 2);
            let surface = if r.next_u32() % 4 == 0 { u32::MAX } else { (r.next_u32() % 2) << 8 | r.next_u32() % 2 };
            c.bus.write32(rider + 0x430, surface);
            c.bus.write32(rider + 0x788, board);
            c.bus.write32(rider + 0x77c, DATA + 0x1_1800);
            c.bus.write32(DATA + 0x1_1800 + 0xde0, [1u32, 4, 2][(seed / 2 % 3) as usize]);
            put_vec3(c, &mut r, rider + 0xaac, -50.0, 50.0);
            c.bus.write32(board + 0xac, [1u32, 3, 2][(seed / 3 % 3) as usize]);
            c.bus.write32(board + 0x30, if seed % 5 == 0 { u32::MAX } else { surface & 0x1ff });
            let span = uf(&mut r, 0.0, 12.0);
            c.bus.write32(board + 0x98, f(span));
            c.bus.write32(board + 0xa0, f(uf(&mut r, -2.0, span)));
            c.bus.write32(board + 0x34, f(uf(&mut r, -50.0, 50.0)));
            c.bus.write32(board + 0x38, f(uf(&mut r, -50.0, 50.0)));
            // Painter layers (gp+0x16c8): layer table, tiles, tile types.
            let (w2, layer_list, table, layer, tiles, tile) = (
                DATA + 0x1_3000,
                DATA + 0x1_3100,
                DATA + 0x1_3800,
                DATA + 0x1_3900,
                DATA + 0x1_3a00,
                DATA + 0x1_4000,
            );
            c.bus.write32(gp_at(0x16c8), w2);
            c.bus.write32(w2, layer_list);
            c.bus.write32(layer_list + 8, table);
            for i in 0..2 {
                c.bus.write32(table + 4 * i, if r.next_u32() % 4 == 0 { 0 } else { layer });
                c.bus.write32(tiles + 4 * i, if r.next_u32() % 4 == 0 { 0 } else { (tile >> 2) << 8 });
            }
            c.bus.write32(layer + 0xc, tiles);
            c.bus.write16(tile + 0x156, if r.next_u32() % 4 == 0 { 0xffff } else { (r.next_u32() % 3) as u16 });
            c.bus.write8(tile + 0x155, (r.next_u32() % 3) as u8);
            for i in 0..3 {
                c.bus.write32(layer_list + 8 * i + 0x24, if r.next_u32() % 2 == 0 { 6 } else { 1 });
                c.bus.write32(layer_list + 0x18 * i + 0x3f0, if r.next_u32() % 2 == 0 { 3 } else { 0 });
            }
            c.g[A0][0] = view as u64;
        });
    }


    #[test]
    fn irradiance_ref_resolve_matches() {
        check(ssx3_ee::s09::f_002edfd0__generated, irradiance_ref_resolve, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x2edf_d0, 0);
            // The manager's entry count at D_005047F8+4 is <= 0, so the lookup
            // callee returns 0 without walking its table.
            c.bus.write32(0x0050_47f8 + 4, 0);
            c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
        });
    }


    /// `WorldPainter_GetIrradianceNameN` returns the static name buffer while
    /// its cache word is set, which keeps its vtable path out of the test.
    pub(crate) fn irradiance_fixture(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_add(0xa74), 1);
    }


    #[test]
    fn world_painter_get_irradiance0_matches() {
        check(ee::s09::f_002ee318__generated, world_painter_get_irradiance0, SEEDS_2, &|c, seed| {
            let mut r = Rng(0x002e_e318 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            irradiance_fixture(c);
        });
    }


    #[test]
    fn world_painter_get_irradiance1_matches() {
        check(ee::s09::f_002ee340__generated, world_painter_get_irradiance1, SEEDS_2, &|c, seed| {
            let mut r = Rng(0x002e_e340 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            irradiance_fixture(c);
        });
    }


    // --------------------------------------------------------------- bolt-on FX

    #[test]
    fn c_bolton_fx_interface_update_matches() {
        check(ee::s09::f_002ecf78__generated, c_bolton_fx_interface_update, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0xcf78, 0);
            let selfp = DATA + 0x1000;
            c.g[A0][0] = selfp as u64;
            // The refresh walks the interface fields; a detached interface
            // (all zero past `+0`) keeps it on its short path.
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            // `BEBoltInt_GetInstance` returns this cached object, so the
            // refresh does not allocate.
            c.bus.write32(GAME_GP.wrapping_sub(0x1f28), DATA + 0x200);
            c.bus.write32(selfp, if seed % 3 == 0 { 0 } else { 1 });
            let _ = r.next_u32();
        });
    }


    #[test]
    fn world_painter_sun_flare_intensity_matches() {
        covered(ssx3_ee::s09::f_002ee268__generated, world_painter_sun_flare_intensity, 120, &|c, seed| {
            let mut r = machine(c, seed, 47, 1);
            let i = r.next_u32() % 3;
            let slot = 0x004f_a370 + i * 0xf0;
            let cell = DATA + 0x400 + i * 0x100;
            let obj = DATA + 0x800 + i * 0x100;
            let vt = vtable(c, &mut r, &[0x110]);
            c.bus.write32(obj, vt);
            c.bus.write32(cell, obj);
            c.bus.write32(slot + 0x14, cell);
            c.g[A0][0] = sx(i);
        });
    }


    #[test]
    fn world_painter_lerp_view_colours_matches() {
        check(ee::s09::f_002ed338__generated, world_painter_lerp_view_colours, 120, &|c, seed| {
            let mut r = Rng(0x2f_ed33 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            c.g[A2][0] = sx(DATA + 0x140);
            for i in 0..8 {
                c.bus.write32(DATA + 0x28 + i * 4, r.edge_f32());
                c.bus.write32(DATA + 0x100 + i * 4, r.edge_f32());
                c.bus.write32(DATA + 0x140 + i * 4, r.edge_f32());
            }
            c.f[12] = r.edge_f32();
        });
    }


    #[test]
    fn irradiance_get_override_matches() {
        check(ee::s09::f_002ee2f8__generated, irradiance_get_override, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2e_e2f8, 0);
            let entry = GAME_GP.wrapping_add(0x24c0);
            match seed % 3 {
                0 => {}
                1 => c.bus.write32(entry + 4, 0x0012_3400),
                _ => {
                    c.bus.write32(entry + 4, 0x0012_3400);
                    c.bus.write8(entry, 1);
                    c.bus.write32(entry, 0x0100_0001);
                }
            }
            c.bus.write32(entry + 8, r.next_u32() % 0x100);
        });
    }


    #[test]
    fn world_painter_get_irradiance2_matches() {
        check(ee::s09::f_002ee368__generated, world_painter_get_irradiance2, 80, &|c, seed| {
            let _r = game(c, seed, 0x2ee368, 2);
            // A non-null painter singleton makes the name lookup return the
            // static name at `gp + 0x24c0`; the reference lookup then resolves
            // that name.
            c.bus.write32(difftest_gp() + 0xa74, SCRATCH);
            c.g[A0][0] = (seed * 4) as u64;
        });
    }


    #[test]
    fn c_bolt_on_fx_interface_clear_emitters_matches() {
        check(ee::s09::f_002ecaf0__generated, c_bolt_on_fx_interface_clear_emitters, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2eca_f0, 0);
            let fx = DATA + 0x100;
            c.g[A0][0] = sx(fx);
            fill(c, &mut r, fx, 0x100);
        });
    }


    // -----------------------------------------------------------------------
    // 0x2ee6f0 WorldPainter_GetWeather30.

    #[test]
    fn world_painter_get_weather30_matches() {
        check(ee::s09::f_002ee6f0__generated, world_painter_get_weather30, 32, &|c, seed| {
            let mut r = machine(c, seed, 0x2ee6_f0, 1);
            let i = seed % 8;
            c.g[A0][0] = i as u64;
            let slot = 0x004f_a370 + 0xf0 * i;
            let obj = DATA + 0x200;
            let vt = vtable(c, &mut r, &[0x180]);
            c.bus.write32(slot + 0x20, DATA + 0x100);
            c.bus.write32(DATA + 0x100, obj);
            c.bus.write32(obj + 4, vt);
        });
    }


    

    // ------------------------------------------------------------- FX / emitters

    #[test]
    fn c_bolt_on_fx_interface_find_or_create_emitter_matches() {
        check(
            ssx3_ee::s09::f_002ecbc0__generated,
            c_bolt_on_fx_interface_find_or_create_emitter,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x2ecbc0);
                c.g[A0][0] = DATA as u64;
                let a1 = r.next_u32();
                let a2 = r.next_u32();
                c.g[A1][0] = (if seed % 2 == 0 { a1 } else { r.next_u32() }) as u64;
                c.g[A2][0] = a2 as u64;
                // Slot 0 always matches (id = a2, kind = a1); the others do not,
                // so both the found and the create paths are visited.
                c.bus
                    .write32(DATA + 0x10, if seed % 3 == 0 { 0xffff_ffff } else { 5 });
                c.bus.write32(DATA + 0x14, a2);
                c.bus.write32(DATA + 0x18, a1);
                for i in 1..15u32 {
                    c.bus.write32(DATA + 0x18 + 0xc * i - 8, 0xffff_ffff);
                }
            },
        );
    }


    // ------------------------------------------------------------------
    // Bolt-on effects.

    #[test]
    fn c_bolt_on_fx_interface_ctor_matches() {
        check(ssx3_ee::s09::f_002ec9e0__generated, c_bolt_on_fx_interface_ctor, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x2ec9e0, 0);
            be_library(c);
            world(c);
            let obj = DATA + 0x100;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn c_bolt_on_fx_interface_ctor_2_matches() {
        check(ssx3_ee::s09::f_002eca68__generated, c_bolt_on_fx_interface_ctor_2, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x2eca68, 0);
            be_library(c);
            world(c);
            let obj = DATA + 0x100;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn c_bolt_on_fx_interface_create_emitter_matches() {
        check(ee::s09::f_002ecc28__generated, c_bolt_on_fx_interface_create_emitter, 90, &|c, seed| {
            let _ = base(c, seed, 0x2e_cc28);
            globals(c);
            pool(c);
            heap_init(c);
            let mut free_first = seed % 3 == 0;
            for i in 0..15u32 {
                let id = if free_first && i == 0 { u64::MAX } else { 0 };
                if i == 3 {
                    free_first = seed % 5 == 0;
                }
                w(c, DATA + 0x10 + 0xc * i, id as u32);
            }
            w(c, DATA + 0xc, DATA + 0x8000);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x2000);
            c.g[A2][0] = sx(DATA + 0x3000);
        });
    }


    #[test]
    fn bolt_on_fx_interface_dtor_matches() {
        check(
            ee::s09::f_002ecb28__generated,
            bolt_on_fx_interface_dtor,
            SEEDS_4,
            &alloc_base_setup(|c, seed, _r| {
                let free_self = seed & 1;
                let this = if free_self != 0 {
                    alloc(c, 0x200)
                } else {
                    DATA
                };
                w(c, this + 0xc, DATA + 0x800);
                for i in 0..15u32 {
                    w(
                        c,
                        this + 0x10 + 0xc * i,
                        if (seed >> (i % 8)) & 1 == 0 {
                            0xffff_ffff
                        } else {
                            0x1
                        },
                    );
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = free_self as u64;
            }),
        );
    }


    // ------------------------------------------------------- world painter

    #[test]
    fn world_painter_free_view_queries_matches() {
        check(
            ee::s09::f_002ecfa0__generated,
            world_painter_free_view_queries,
            SEEDS_5,
            &|c, seed| {
                let _ = base_3(c, seed, 0x2ecf_a0, 4);
                w(c, GAME_GP + 0xa64, if seed % 3 == 0 { 0 } else { 1 });
                // The view-query table: 8 painters x (1 + 8) entries.
                for i in 0..0x60u32 {
                    w(c, 0x009f_a370 + 4 * i, 0);
                }
                if seed % 2 == 1 {
                    let obj = SCRATCH_2 + 0x1000;
                    w(c, 0x009f_a370, obj);
                    w(c, obj + 4, obj + 0x1100);
                    slot_2(c, obj + 0x1100, 8, 0);
                    slot_2(c, obj + 0x1100, 0xc, 1);
                }
            },
        );
    }
}
