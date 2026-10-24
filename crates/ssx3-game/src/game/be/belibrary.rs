//! `src/be/belibrary.cpp`: hand-written replacements (moved from the
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
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::animation::rideranimbase::sw;
pub(crate) use crate::game::be::beintmission::event_check;
pub(crate) use crate::game::be::beintmission::lw;
pub(crate) use crate::game::be::beintmission::BE_GET_RIDER_CHARACTER;
pub(crate) use crate::game::be::beintmission::BE_GET_RIDER_PROFILE;
pub(crate) use crate::game::be::beintmission::EA_MEM_ALLOC_NAMED;
pub(crate) use crate::game::be::beintplayer::mul32;
pub(crate) use crate::game::fe::feasyncfile::a;
pub(crate) use crate::game::fe::feasyncfile::addu;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_4;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::sub;
pub(crate) use crate::game::fe::fepopup::at as at_2;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lq;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_5;
pub(crate) use crate::game::fe::fepopup::sq;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::sound::bankmonitor::add32;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_2;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
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
    (
        0x0014_a130,
        be_find_rider_by_profile,
        "BE_FindRiderByProfile",
    ),
    (
        0x0014_ab20,
        be_score_int_get_high_score_table,
        "BEScoreInt_GetHighScoreTable",
    ),
    (
        0x0014_ab68,
        be_get_event_record_index,
        "BE_GetEventRecordIndex",
    ),
    (
        0x0014_abe0,
        be_bolt_int_get_instance,
        "BEBoltInt_GetInstance",
    ),
    (
        0x0014_ac30,
        beb_bolt_int_get_rider_character,
        "BEBoltInt_GetRiderCharacter",
    ),
    (
        0x0014_ac50,
        beb_bolt_int_get_rider_profile,
        "BEBoltInt_GetRiderProfile",
    ),
    (
        0x0014_ac70,
        beb_bolt_int_get_rider_profile_and_char,
        "BEBoltInt_GetRiderProfileAndChar",
    ),
    (
        0x0014_acb0,
        beb_bolt_int_set_bolt_states,
        "BEBoltInt_SetBoltStates",
    ),
    (
        0x0014_ad50,
        beb_bolt_int_get_rider_profile_data,
        "BEBoltInt_GetRiderProfileData",
    ),
    (
        0x0014_ae28,
        be_bolt_int_get_bolt_states,
        "BEBoltInt_GetBoltStates",
    ),
    (
        0x0014_ae60,
        beb_bolt_int_get_rider_bolt_states,
        "BEBoltInt_GetRiderBoltStates",
    ),
    (
        0x0014_aea8,
        be_bolt_int_save_equip_flags,
        "BEBoltInt_SaveEquipFlags",
    ),
    (
        0x0014_b478,
        be_bolt_int_sum_equipped_bolt_value,
        "BEBoltInt_SumEquippedBoltValue",
    ),
    (0x0014_b560, be_bolt_int_unlock_bolt, "BEBoltInt_UnlockBolt"),
    (
        0x0014_c2b0,
        bolts_index_bolts_by_char,
        "Bolts_IndexBoltsByChar",
    ),
    (
        0x0014_c320,
        bolts_index_table2_by_char,
        "Bolts_IndexTable2ByChar",
    ),
    (
        0x0014_c3c8,
        bolts_index_table3_by_char,
        "Bolts_IndexTable3ByChar",
    ),
    (
        0x0014_c488,
        bolts_index_table4_by_char,
        "Bolts_IndexTable4ByChar",
    ),
    (0x0014_c620, bolts_init_db, "Bolts_InitDB"),
    (0x0014_c658, bolts_destroy_db, "Bolts_DestroyDB"),
    (0x0014_d240, bolts_free_db, "Bolts_FreeDB"),
    (0x0014_d448, bolts_find_bolt_index, "Bolts_FindBoltIndex"),
    (0x0014_d558, bolts_find_first_child, "Bolts_FindFirstChild"),
    (0x0014_d908, bolts_is_descendant_of, "Bolts_IsDescendantOf"),
    (
        0x0014_d988,
        bolts_get_char_bolt_count,
        "Bolts_GetCharBoltCount",
    ),
    (0x0014_d9d0, bolts_get_char_bolts_2, "Bolts_GetCharBolts_2"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::fepopup::callee::C00113f88;
    pub(crate) use crate::game::fe::fepopup::callee::C00115ab0;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe78;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe98;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fec8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a080;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a0e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0014bdb8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d448;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d988;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d998;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dc80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dd58;
    pub(crate) use crate::game::fe::fepopup::callee::C00150b48;
    pub(crate) use crate::game::fe::fepopup::callee::C00151c48;
    pub(crate) use crate::game::fe::fepopup::callee::C001535c0;
    pub(crate) use crate::game::fe::fepopup::callee::C001543e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00156a60;
    pub(crate) use crate::game::fe::fepopup::callee::C001577e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00157bf0;
    pub(crate) use crate::game::fe::fepopup::callee::C001589b0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158a50;
    pub(crate) use crate::game::fe::fepopup::callee::C00158af0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158c80;
    pub(crate) use crate::game::fe::fepopup::callee::C00158d58;
    pub(crate) use crate::game::fe::fepopup::callee::C00159cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C0015a2e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c6b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c718;
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
    pub(crate) use crate::game::fe::fepopup::callee::C002758c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275b98;
    pub(crate) use crate::game::fe::fepopup::callee::C00275cf8;
    pub(crate) use crate::game::fe::fepopup::callee::C00278e50;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f38;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f68;
    pub(crate) use crate::game::fe::fepopup::callee::C00281ed8;
    pub(crate) use crate::game::fe::fepopup::callee::C002837c8;
    pub(crate) use crate::game::fe::fepopup::callee::C0028b180;
    pub(crate) use crate::game::fe::fepopup::callee::C0028bb10;
    pub(crate) use crate::game::fe::fepopup::callee::C0028e8c0;
    pub(crate) use crate::game::fe::fepopup::callee::C002913d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003457c8;
    pub(crate) use crate::game::fe::fepopup::callee::C003625c0;
    pub(crate) use crate::game::fe::fepopup::callee::C003626d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003627a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00364cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C00366ce0;
    pub(crate) use crate::game::fe::fepopup::callee::C003684f0;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f460;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f668;
    pub(crate) use crate::game::fe::fepopup::callee::C0039c870;
    pub(crate) use crate::game::fe::fepopup::callee::C0039ff78;
    pub(crate) use crate::game::fe::fepopup::callee::C003b1300;
    pub(crate) use crate::game::fe::fepopup::callee::C003b2360;
    pub(crate) use crate::game::fe::fepopup::callee::C00125108;
    pub(crate) use crate::game::fe::fepopup::callee::C00150928;
    pub(crate) use crate::game::fe::fepopup::callee::C00156988;
    pub(crate) use crate::game::fe::fepopup::callee::C00158910;
    pub(crate) use crate::game::fe::fepopup::callee::C00159170;
    pub(crate) use crate::game::fe::fepopup::callee::C00176890;
    pub(crate) use crate::game::fe::fepopup::callee::C00225068;
    pub(crate) use crate::game::fe::fepopup::callee::C00275498;
    pub(crate) use crate::game::fe::fepopup::callee::C00278308;
    pub(crate) use crate::game::fe::fepopup::callee::C00283228;
    pub(crate) use crate::game::fe::fepopup::callee::C00283518;
    pub(crate) use crate::game::fe::fepopup::callee::C00358020;
    pub(crate) use crate::game::fe::fepopup::callee::C00362478;
    pub(crate) use crate::game::fe::fepopup::callee::C00362660;
    pub(crate) use crate::game::fe::fepopup::callee::C00368170;
    pub(crate) use crate::game::fe::fepopup::callee::C00368970;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3 as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_GET_MESSAGE_COUNT;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_REMOVE_MESSAGE;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_SCRIPT_OBJ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_STOP_3 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_HAS_BINDING;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_INSTANCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_AABB;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_SPHERE_TREE;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::STR_HASH;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2 as UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2 as UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::ADF_DATA_SOURCE_SET_DATA;
    pub(crate) use crate::game::sound::bankmonitor::callee::AUDIO_RAND;
    pub(crate) use crate::game::sound::bankmonitor::callee::BANK_MONITOR_TIMER_KILL;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_GET_EVENT_RECORD_INDEX;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_MISSION_INT_GET_COLLECTED_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BOLTS_FREE_DB;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUDDY_MAN_CONNECT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::sound::bankmonitor::callee::CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::sound::bankmonitor::callee::EA_FILE_LOAD_DECOMPRESSED;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_ANIM_MAP_FIND_HASH;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::sound::bankmonitor::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMCARD_REQUEST_GET_INFO_NOW;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMSET_2 as MEMSET;
    pub(crate) use crate::game::sound::bankmonitor::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::sound::bankmonitor::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_FLOAT_2 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_VEC3;
    pub(crate) use crate::game::sound::bankmonitor::callee::PS2_MEMCARD_CREATE;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_LIST_ADD_CHOICE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SECTION_ALLOCATOR_DTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::sound::bankmonitor::callee::SNDSTRM_SET_PREBUFFER;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_MEMORY_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::SONG_INSTANCE_GET_REQUEST_STATUS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SPEECH_SYSTEM_STOP_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::STREAM_FILE_RELEASE_CHUNK;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;
    pub(crate) use crate::game::sound::bankmonitor::callee::TEX_MAN_FREE_TEXTURE;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_MENU_SET_NAV_BUTTONS;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::sound::bankmonitor::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::sound::bankmonitor::callee::WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_MAN_CALL_TASK_FUNC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_REMOVE;
}

// ---------------------------------------------------------------------------
// BEBoltInt_SumEquippedBoltValue (0x0014b478).

/// The character bolt database and the equipped-bolt flags table.
pub(crate) mod bolts {

    /// `BoltsDB`: per-character counts at `+0x2c`, bolt array index at
    /// `+0xa4`, the shared bolt array base at `+4`.
    pub const DB: u32 = 0x004a_6750;

    pub const ARRAY_BASE: u32 = 0x004a_7c30;

    /// `BoltsBolt` stride and the equipped flag word offset.
    pub const STRIDE: u32 = 0x38;

    pub const FLAGS: u32 = 4;

    pub const VALUE: u32 = 0xc;
}

// ---------------------------------------------------------------------------
// Bolt database.

/// `void Bolts_InitDB(BoltsDB *db)` (0x14c620): zero the eleven words of the
/// DB header (0x00..0x28); leaves `db` in v0, as the constructor stub does.
pub fn bolts_init_db(c: &mut Cpu) {
    c.cycles += 13;
    let db = Addr(c.g[A0][0] as u32);
    c.g[V0][0] = c.g[A0][0];
    for off in (0..0x2c).step_by(4) {
        db.off(off).sw(c, 0);
    }
    abi::ret(c);
}

/// `int Bolts_GetCharBoltCount(BoltsDB *db, int charIndex)` (0x14d988): read the
/// count at `db + charIndex*4 + 0x2c`.
pub fn bolts_get_char_bolt_count(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    let record = abi::arg(c, 0).wrapping_add(c.g[A1][0] as u32);
    c.g[A0][0] = sx(record);
    c.g[V0][0] = Addr(record).off(0x2c).lw(c);
    abi::ret(c);
}

/// The shared body of `Bolts_GetCharBolts`/`_2`: if the character's bolt count
/// is positive, resolve the bolt array at `record + 0xa4` through its stride
/// (0x38) and add the base read at `db + 4`; otherwise return 0.
pub(crate) fn char_bolts_pointer(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    let record = abi::arg(c, 0).wrapping_add(c.g[A1][0] as u32);
    c.g[V0][0] = sx(record);
    let count = Addr(record).off(0x2c).lw(c);
    c.g[V1][0] = c.g[V0][0];
    if (count as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[A1][0] = 0x38;
    c.g[V0][0] = Addr(record).off(0xa4).lw(c);
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x4).lw(c);
    let index = abi::mult(c, c.g[V0][0] as u32, 0x38);
    c.g[V0][0] = sx(index);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `BoltsBolt *Bolts_GetCharBolts_2(BoltsDB *db, int charIndex)` (0x14d9d0):
/// identical to `Bolts_GetCharBolts`.
pub fn bolts_get_char_bolts_2(c: &mut Cpu) {
    char_bolts_pointer(c);
}

/// The base of the per-bolt record table.
pub(crate) const BOLT_RECORDS: u32 = 0x004a_0000;

/// Offset of the per-bolt record table within its base.
pub(crate) const BOLT_RECORDS_OFF: u32 = 0x6ca8;

/// `void BEBoltInt_GetBoltStates(int a1, int a2, int a3)` (0x14ae28): read a
/// bolt word into `*a3`.
pub fn be_bolt_int_get_bolt_states(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V1][0] = 0x9b50;
    c.g[A0][0] = 0xf88;
    c.mult(0, V0, A1, V1);
    c.mult(0, A2, A2, A0);
    c.g[V1][0] = BOLT_RECORDS as u64;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(BOLT_RECORDS_OFF));
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = c.g[A0][0];
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x28c)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
    Addr(abi::arg(c, 3)).sw(c, c.g[A1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Singleton accessors: each keeps a lazily allocated singleton in a gp-relative
// slot (the address is the gp offset). They return the instance pointer.

/// `BEBoltInt *BEBoltInt_GetInstance(void)` (0x14abe0).
pub fn be_bolt_int_get_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1f28));
        let have = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !have {
            c.cycles += 11;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x10;
            c.g[A1][0] = sx(0x460000u32.wrapping_sub(0x5bf8));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, EA_MEM_ALLOC_NAMED, 0x0014_ac08)?;
            c.g[V1][0] = 0x460000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
            c.g[V1][0] = sx(0x460000u32.wrapping_sub(0x5178));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f28), c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1f28));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bolt interface: thin forwarding wrappers around the rider character/profile
// accessors.

/// `Character *BEBoltInt_GetRiderCharacter(BEBoltInt *self, int rider)`
/// (0x14ac30): `BE_GetRiderCharacter(rider)`.
pub fn beb_bolt_int_get_rider_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        BE_GET_RIDER_CHARACTER.call(c, 0x0014_ac40)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderProfile *BEBoltInt_GetRiderProfile(BEBoltInt *self, int rider)`
/// (0x14ac50): `BE_GetRiderProfile(rider)`.
pub fn beb_bolt_int_get_rider_profile(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        BE_GET_RIDER_PROFILE.call(c, 0x0014_ac60)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderProfile *BEBoltInt_GetRiderProfileAndChar(BEBoltInt *self, int rider,
/// Character **out)` (0x14ac70): store the rider's character in `*out`, then
/// return the rider's profile.
pub fn beb_bolt_int_get_rider_profile_and_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0014_ac90)?;
        c.g[A0][0] = c.g[S1][0];
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        BE_GET_RIDER_PROFILE.call(c, 0x0014_ac9c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const BOLT_GET_RIDER_PROFILE: Callee = Callee {
    addr: 0x0014_ac50,
    f: ssx3_ee::s01::f_0014ac50,
};

pub(crate) const BOLT_GET_RIDER_CHAR: Callee = Callee {
    addr: 0x0014_ac30,
    f: ssx3_ee::s01::f_0014ac30,
};

pub(crate) const GET_CHAR_PROFILE_DATA: Callee = Callee {
    addr: 0x0014_ad28,
    f: ssx3_ee::s01::f_0014ad28,
};

pub(crate) const BOLT_GET_BOLT_STATES: Callee = Callee {
    addr: 0x0014_ae28,
    f: ssx3_ee::s01::f_0014ae28,
};

pub(crate) const EA_MEM_COPY: u32 = 0x003e_6574;

/// `int BE_FindRiderByProfile(int profile)` (0x14a130): the index (0..5) of the
/// rider whose profile id matches, or -1.
pub fn be_find_rider_by_profile(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x530000;
    c.g[V0][0] = 0x530000;
    c.g[T0][0] = sx(0x530000u32.wrapping_add(0x5b20));
    c.g[A2][0] = sx(0x530000u32.wrapping_add(0x5b0));
    c.g[A1][0] = 0;
    c.g[A3][0] = 0x1c;
    loop {
        c.cycles += 7;
        c.g[V0][0] = lw(c, c.g[A2][0] as u32);
        c.mult(0, V1, V0, A3);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V1][0] = lw(c, (c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.g[V1][0] & 0x1;
        if c.g[V1][0] != c.g[A0][0] {
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 6) as u64;
            let t = c.g[V0][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
            if t {
                event_check(c, 0x0014_a148);
                continue;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                c.pc = c.g[RA][0] as u32;
                return;
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[A1][0];
            c.pc = c.g[RA][0] as u32;
            return;
        }
    }
}

/// `void BEBoltInt_GetRiderProfileData(BEBoltInt *self, int rider, void *out)`
/// (0x14ad50): look up the rider's profile and character and copy the character
/// profile data into `out`.
pub fn beb_bolt_int_get_rider_profile_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        BOLT_GET_RIDER_PROFILE.call(c, 0x0014_ad74)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        BOLT_GET_RIDER_CHAR.call(c, 0x0014_ad80)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        GET_CHAR_PROFILE_DATA.call(c, 0x0014_ad90)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BEBoltInt_GetRiderBoltStates(BEBoltInt *self, int rider, int *out)`
/// (0x14ae60): `BEBoltInt_GetBoltStates(self, profile, char, out)`.
pub fn beb_bolt_int_get_rider_bolt_states(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[SP][0];
        Callee {
            addr: 0x0014_ac70,
            f: ssx3_ee::s01::f_0014ac70,
        }
        .call(c, 0x0014_ae80)?;
        c.g[A2][0] = lw(c, c.g[SP][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        BOLT_GET_BOLT_STATES.call(c, 0x0014_ae94)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void BEBoltInt_GetRiderProfileData?` — `BEBoltInt_SetBoltStates(BEBoltInt
/// *self, int rider, void *states, ...)` (0x14acb0): `EA_MemCopy(profile*0xf88
/// + rider*0x9b50 + 0x4a6ca8 + 0x290, states, 0x834)`.
pub fn beb_bolt_int_set_bolt_states(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        c.g[V0][0] = 0xf88;
        c.g[T0][0] = 0x9b50;
        c.mult(0, V1, A2, V0);
        c.mult(0, A0, A1, T0);
        c.g[V0][0] = 0x4a0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A3][0];
        c.g[A2][0] = 0x834;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x290));
        call_ptr(c, EA_MEM_COPY, 0x0014_ace8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int* BEScoreInt_GetHighScoreTable(int a, int b)` (0x14ab20): the high
/// score table for the event record (`D_00535C18 + type*100`), null for the
/// record type 26.
pub fn be_score_int_get_high_score_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::BE_GET_EVENT_RECORD_INDEX.call(c, 0x0014_ab30)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x1a;
        let unknown = c.g[A0][0] == c.g[V0][0];
        c.g[V1][0] = 0x64;
        if !unknown {
            c.cycles += 5;
            c.g[V0][0] = 0x0053_0000;
            c.mult(0, V1, A0, V1);
            c.g[V0][0] = sx(0x0053_0000u32.wrapping_add(0x5c18));
            c.g[V0][0] = add32(c.g[V1][0], c.g[V0][0]);
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        leave(c, 0x10);
        Ok(())
    })
}

/// `void Bolts_DestroyDB(Bolts* self, u32 flags)` (0x14c658): free the bolt
/// database and, with bit 0 of `flags`, the object itself.
pub fn bolts_destroy_db(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::BOLTS_FREE_DB.call(c, 0x0014_c674)?;
        c.g[S0][0] &= 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            callee::BUILTIN_DELETE.call(c, 0x0014_c688)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[inline(always)]
pub(crate) fn lhu(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lhu(c)
}

#[inline(always)]
pub(crate) fn sh(c: &mut Cpu, a: u32, v: u64) {
    Addr(a).sh(c, v);
}

/// `BEBoltInt_SaveEquipFlags(int a1, int a2, int self)` (0x14aea8): for each
/// bolt entry (`{u16 pad; u16 flags; u16 kind; u16 pad}` at +0x290, `a1*0x9B50
/// + a2*0xF88` into `D_004A6CA8`), set bit 2 when bit 4 is set and clear it
/// otherwise.
pub fn be_bolt_int_save_equip_flags(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V0][0] = 0xf88;
    c.g[V1][0] = 0x9b50;
    c.mult(0, A2, A2, V0);
    c.mult(0, A1, A1, V1);
    c.g[V0][0] = 0x004a_0000;
    c.g[V0][0] = 0x004a_6ca8;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = lw(c, p(c.g[A1][0] as u32, 0x28c));
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[A1][0] = sx(p(c.g[A1][0] as u32, 0x290));
    if !none {
        c.cycles += 3;
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_fffb;
        loop {
            c.cycles += 4;
            c.g[V1][0] = lhu(c, p(c.g[A1][0] as u32, 2));
            c.g[V0][0] = c.g[V1][0] & 0x10;
            let set = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[V1][0] | 4;
            if !set {
                c.cycles += 1;
                c.g[V0][0] = c.g[V1][0] & c.g[A2][0];
            }
            c.cycles += 4;
            sh(c, p(c.g[A1][0] as u32, 2), c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            let more = c.g[A0][0] != 0;
            c.g[A1][0] = sx(p(c.g[A1][0] as u32, 4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_aee0);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void Bolts_IndexBoltsByChar(Bolts* self)` (0x14c2b0): count the bolts per
/// character group and remember the first index of each group.
pub fn bolts_index_bolts_by_char(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr(at(c, A0, 0x1c)).lw(c);
    c.g[T1][0] = u64::MAX;
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[A3][0] = 0;
    if !none {
        c.cycles += 4;
        c.g[T3][0] = sx(at(c, A0, 0x2c));
        c.g[T2][0] = sx(at(c, A0, 0xa4));
        c.g[T0][0] = 0;
        loop {
            c.cycles += 9;
            c.g[V0][0] = Addr(at(c, A0, 4)).lw(c);
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = Addr(at(c, V0, 0)).lb(c);
            c.g[A2][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V0][0] = Addr(at(c, V1, 0)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            let same = c.g[T1][0] == c.g[A1][0];
            Addr(at(c, V1, 0)).sw(c, c.g[V0][0]);
            if !same {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[T1][0] = c.g[A1][0];
                Addr(at(c, V0, 0)).sw(c, c.g[A3][0]);
            }
            c.cycles += 5;
            c.g[V0][0] = Addr(at(c, A0, 0x1c)).lw(c);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x38));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_c2d0);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `BE_GetEventRecordIndex` (0x14AB68): map a BE event id `b` (and the player
/// `a`) to a score-type index; 26 when there is no record.
pub fn be_get_event_record_index(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A0][0] as i64) < 17) as u64;
    let known_player = c.g[V0][0] != 0;
    c.g[A2][0] = low(c, A1).wrapping_sub(6);
    if !known_player {
        c.cycles += 2;
        c.g[V0][0] = 0x1a;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = (c.g[A2][0] < 3) as u64;
    let low_id = c.g[V0][0] != 0;
    c.g[V0][0] = low(c, A1).wrapping_sub(9);
    let mut table = false;
    if !low_id {
        c.cycles += 3;
        c.g[V0][0] = (c.g[V0][0] < 3) as u64;
        // `t` is `(b - 9) < 3` compared against zero: the table scan is the
        // arm where it is *not* a low id (b >= 12), the other one returns b-6.
        let high_id = c.g[V0][0] != 0;
        c.g[V0][0] = 0x46_0000;
        if !high_id {
            table = true;
        }
    }
    if !table {
        c.cycles += 2;
        c.g[V0][0] = c.g[A2][0];
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V1][0] = sx(r32(c, A0) << 4);
    c.g[V0][0] = low(c, V0).wrapping_sub(0x5d08);
    c.g[V1][0] = low(c, V1).wrapping_add(low(c, V0));
    c.g[A2][0] = c.g[V1][0];
    c.g[A3][0] = 0;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
        let hit = c.g[V0][0] == c.g[A1][0];
        c.g[A3][0] = low(c, A3).wrapping_add(1);
        if hit {
            c.cycles += 2;
            c.g[V0][0] = lw_at(c, V1, 0x4);
            abi::ret(c);
            return;
        }
        c.cycles += 4;
        c.g[A2][0] = low(c, A2).wrapping_add(8);
        c.g[V0][0] = ((c.g[A3][0] as i64) < 2) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = low(c, V1).wrapping_add(8);
        if !more {
            c.cycles += 2;
            c.g[V0][0] = 0x1a;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_abb8);
        }
    }
}

/// `lh` (sign-extended).
#[inline(always)]
pub(crate) fn lh_at(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lh(c)
}

/// `int Bolts_FindBoltIndex(self, group, key)` (0x14d448): the key's index in
/// the group's bolt list — a per-group short lookup table when present, else
/// a linear scan of `self->entries[first .. first+count]` (0x38-byte records,
/// key at +4); -1 when absent.
pub fn bolts_find_bolt_index(c: &mut Cpu) {
    c.cycles += 7;
    let self_ = abi::arg(c, 0);
    let group = abi::arg(c, 1);
    let key = c.g[A2][0];
    c.g[T0][0] = self_ as u64;
    c.g[A1][0] = sx(group << 2);
    let slot = p(self_, group << 2);
    c.g[V0][0] = sx(slot);
    c.g[T1][0] = key;
    let table = lw_at_2(c, p(slot, 0x4dc));
    c.g[A2][0] = table;
    if table != 0 {
        c.cycles += 3;
        c.g[V1][0] = sx(((key as u32) << 1).wrapping_add(table as u32));
        c.g[V0][0] = lh_at(c, c.g[V1][0] as u32);
        abi::ret(c);
        return;
    }
    c.cycles += 10;
    c.g[V1][0] = sx(slot);
    c.g[A0][0] = 0x38;
    let first = lw_at_2(c, p(slot, 0xa4));
    let count = lw_at_2(c, p(slot, 0x2c));
    c.g[A3][0] = first;
    c.g[V1][0] = count;
    mul32(c, A0, A0, A3);
    let base = lw_at_2(c, p(self_, 4));
    let end = first.wrapping_add(count);
    c.g[A1][0] = base;
    c.g[T0][0] = end;
    c.g[V0][0] = ((first as i64) < (end as i64)) as u64;
    c.g[V1][0] = sx((base as u32).wrapping_add(c.g[A0][0] as u32));
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    loop {
        c.cycles += 3;
        c.g[V0][0] = lh_at(c, p(c.g[V1][0] as u32, 4));
        if c.g[V0][0] == c.g[T1][0] {
            c.cycles += 2;
            c.g[V0][0] = c.g[A3][0];
            abi::ret(c);
            return;
        }
        c.cycles += 4;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x38));
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_d490);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    abi::ret(c);
}

/// `int Bolts_IsDescendantOf(self, entry, key)` (0x14d908): 1 when `entry` or
/// one of its ancestors (walking `entry->group` through
/// [`bolts_find_bolt_index`]) is the entry whose group tag is `key`.
pub fn bolts_is_descendant_of(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        let key = c.g[S1][0];
        let mut entry = c.g[A1][0];
        let mut result: u64;
        loop {
            c.cycles += 2;
            if entry == 0 {
                result = 0;
                break;
            }
            c.cycles += 3;
            let group = lh_at(c, (entry as u32).wrapping_add(6));
            c.g[A2][0] = group;
            result = 1;
            if group == key {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            if group != u64::MAX {
                entry = Addr(entry as u32).lb(c);
                c.g[A1][0] = entry;
            } else {
                c.cycles += 2;
                result = 0;
                break;
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = entry;
            c.g[A2][0] = group;
            callee::BOLTS_FIND_BOLT_INDEX.call(c, 0x0014_d950)?;
            c.g[A0][0] = c.g[V0][0];
            let found = (c.g[A0][0] as i64) >= 0;
            c.g[V0][0] = 0x38;
            if !found {
                c.cycles += 2;
                c.g[A1][0] = 0;
                entry = 0;
                if event_due(c) {
                    deliver_event_here(c, 0x0014_d920);
                }
                continue;
            }
            c.cycles += 4;
            c.g[V1][0] = lw_at_2(c, p(self_, 4));
            mul32(c, V0, V0, A0);
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            entry = c.g[A1][0];
            if event_due(c) {
                deliver_event_here(c, 0x0014_d920);
            }
        }
        c.g[V0][0] = result;
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bolt-on scene graph.

pub(crate) const BOLTS_COUNT_ENTRY: u32 = 0x0014_d4c8;

/// The two back edges of the child walk (event resume points).
pub(crate) const BOLTS_COUNT_MATCH: u32 = 0x0014_d510;

pub(crate) const BOLTS_COUNT_NEXT: u32 = 0x0014_d514;

/// `int Bolts_CountChildren(Bolts* self, int group, short key)` (0x14d4c8):
/// with a lookup table, the cached count; otherwise walk the group's entries
/// and count the ones whose key matches.
///
/// The original is a `match pc` dispatcher re-enterable at both loop heads.
pub fn bolts_count_children(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                BOLTS_COUNT_ENTRY => {
                    c.cycles += 6;
                    c.g[T0][0] = c.g[A0][0];
                    c.g[A1][0] = sx(r32(c, A1) << 2);
                    c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, A1)));
                    c.g[V0][0] = Addr(p(r32(c, V0), 0x11c)).lw(c);
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = sx(r32(c, A2) << 1);
                        pc = 0x0014_d548;
                    } else {
                        pc = 0x0014_d4e0;
                    }
                }
                0x0014_d4e0 => {
                    c.cycles += 11;
                    c.g[V1][0] = sx(r32(c, T0).wrapping_add(r32(c, A1)));
                    c.g[A0][0] = 0x38;
                    c.g[A3][0] = Addr(p(r32(c, V1), 0xa4)).lw(c);
                    c.g[V0][0] = c.g[V1][0];
                    c.g[V1][0] = Addr(p(r32(c, V0), 0x2c)).lw(c);
                    c.g[T1][0] = 0;
                    c.mult(0, A0, A3, A0);
                    c.g[V0][0] = Addr(p(r32(c, T0), 4)).lw(c);
                    c.g[A1][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    pc = 0x0014_d518;
                }
                0x0014_d50c => {
                    c.cycles += 1;
                    pc = BOLTS_COUNT_MATCH;
                }
                BOLTS_COUNT_MATCH => {
                    c.cycles += 1;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x38));
                    pc = BOLTS_COUNT_NEXT;
                }
                BOLTS_COUNT_NEXT => {
                    c.cycles += 1;
                    c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                    pc = 0x0014_d518;
                }
                0x0014_d518 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    pc = if c.g[V0][0] == 0 {
                        0x0014_d540
                    } else {
                        0x0014_d524
                    };
                }
                0x0014_d524 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r32(c, V1), 6)).lh(c);
                    let different = c.g[V0][0] != c.g[A2][0];
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    pc = if different { 0x0014_d538 } else { 0x0014_d530 };
                }
                0x0014_d530 => {
                    c.cycles += 2;
                    c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
                    if crate::events::back_edge_unwinds(c, BOLTS_COUNT_MATCH) {
                        return Ok(());
                    }
                    pc = BOLTS_COUNT_MATCH;
                }
                0x0014_d538 => {
                    c.cycles += 2;
                    let advance = c.g[V0][0] == 0;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x38));
                    if !advance {
                        pc = 0x0014_d540;
                    } else {
                        if crate::events::back_edge_unwinds(c, BOLTS_COUNT_NEXT) {
                            return Ok(());
                        }
                        pc = BOLTS_COUNT_NEXT;
                    }
                }
                0x0014_d540 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[T1][0];
                    abi::ret(c);
                    return Ok(());
                }
                0x0014_d548 => {
                    c.cycles += 3;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                    c.g[V0][0] = Addr(r32(c, V1)).lh(c);
                    abi::ret(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, BOLTS_COUNT_ENTRY, 0x0014_d554);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Board equipment.

/// `void BEBoltInt_RestoreAllEquipFlags(void)` (0x14af10): for the three board
/// slots and their ten equipment entries, restore the "owned" flag bit 0x10
/// on entries whose bit 4 was set, and clear it elsewhere.
pub fn be_bolt_int_restore_all_equip_flags(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x004a_0000;
    c.g[A0][0] = 0;
    c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
    c.g[T5][0] = 0x9b50;
    c.g[T3][0] = 0xf88;
    c.mult(0, 9, 4, 13);
    loop {
        c.cycles += 4;
        c.g[V1][0] = 0;
        c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        c.mult(0, 4, 3, 11);
        loop {
            c.cycles += 6;
            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T4][0] as u32));
            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = lw_2(c, p(c.g[V0][0] as u32, 0x28c));
            let empty = (c.g[V1][0] as i64) <= 0;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
            if !empty {
                c.cycles += 2;
                c.g[A2][0] = c.g[V1][0];
                c.g[T0][0] = 0xffff_ffff_ffff_ffef;
                loop {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 2)).lhu(c);
                    c.g[A1][0] = c.g[V0][0] | 0x10;
                    c.g[V1][0] = c.g[V0][0] & c.g[T0][0];
                    c.g[V0][0] = c.g[V0][0] & 0x4;
                    let clear = c.g[V0][0] == 0;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                    if clear {
                        c.cycles += 1;
                        c.sh(p(c.g[A0][0] as u32, 2), c.g[V1][0]);
                    } else {
                        c.cycles += 2;
                        c.sh(p(c.g[A0][0] as u32, 2), c.g[A1][0]);
                    }
                    c.cycles += 2;
                    let more = c.g[A2][0] != 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014_af58);
                        }
                        continue;
                    }
                    break;
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[A3][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 10) as u64;
            let more = c.g[V0][0] != 0;
            c.mult(0, 4, 3, 11);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0014_af38);
                }
                continue;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[T2][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
            let more_outer = c.g[V0][0] != 0;
            c.mult(0, 9, 4, 13);
            if more_outer {
                if event_due(c) {
                    deliver_event_here(c, 0x0014_af28);
                }
                continue;
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        }
    }
}

// ---------------------------------------------------------------------------
// Table and pool helpers.

/// `void Bolts_IndexTable2ByChar(self)` (0x14c320): clear the per-character
/// group counts and record the first entry of each group.
pub fn bolts_index_table2_by_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x30);
        c.g[A2][0] = 0x78;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = sx(p(r32(c, S0), 0x20c));
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003e_6448, 0x0014c34c)?;
        c.g[S2][0] = 0x1e;
        c.g[V0][0] = Addr(p(r32(c, S0), 0x20)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[T1][0] = sx(p(r32(c, S0), 0x284));
            c.g[T0][0] = 0;
            loop {
                c.cycles += 9;
                c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                c.g[V0][0] = sx(p(r32(c, T0), r32(c, V0)));
                c.g[A1][0] = Addr(r32(c, V0)).lb(c);
                c.g[A2][0] = sx(r32(c, A1) << 2);
                c.g[V1][0] = sx(p(r32(c, A0), r32(c, A2)));
                c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                let same = c.g[S2][0] == c.g[A1][0];
                Addr(r32(c, V1)).sw(c, c.g[V0][0]);
                if !same {
                    c.cycles += 3;
                    c.g[V0][0] = sx(p(r32(c, T1), r32(c, A2)));
                    c.g[S2][0] = c.g[A1][0];
                    Addr(r32(c, V0)).sw(c, c.g[A3][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r32(c, S0), 0x20)).lw(c);
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T0][0] = sx(p(r32(c, T0), 0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_c368);
                }
            }
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
// `Bolts_FindFirstChild` (0x14d558): find the bolt entry whose 16-bit key
// matches (`key == -1` or a null lookup table falls back to a linear scan)
// and return the entry's target pointer, or null.
pub fn bolts_find_first_child(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    let no_key = c.g[A2][0] == c.g[V0][0];
    c.g[T0][0] = c.g[A0][0];
    if !no_key {
        c.cycles += 5;
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x194));
        if c.g[A0][0] != 0 {
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
            c.cycles += 5;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[V0][0] = c.lh((c.g[V1][0] as u32));
            let negative = (c.g[V0][0] as i64) <= 0;
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            if negative {
                c.cycles += 2;
                c.g[V0][0] = 0u64;
                abi::ret(c);
                return;
            }
            c.cycles += 6;
            c.g[A1][0] = 0x38;
            c.g[V0][0] = c.lh((c.g[V1][0] as u32));
            c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
            c.mult(0, 2, 2, 5);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            abi::ret(c);
            return;
        }
    }
    // Linear scan over the group's entry range.
    c.cycles += 12;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A0][0] = 0x38;
    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
    c.g[V0][0] = c.g[V1][0];
    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
    c.mult(0, 4, 7, 4);
    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
    let empty = c.g[V0][0] == 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    if !empty {
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x6));
            let hit = c.g[V0][0] == c.g[A2][0];
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            if hit {
                c.cycles += 2;
                c.g[V0][0] = c.g[V1][0];
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x38));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_d5a8);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    abi::ret(c);
}

/// `void* Bolts_FindTable2EntriesByFlag(Bolts* self, int index, int kind,
/// int want, int* out)` (0x14db40): walk the `table2[index]` entries (`kind`
/// at +2, nonzero flag at +1) and return the first entry whose kind and flag
/// match, counting every match in `*out`.
pub fn bolts_find_table2_entries_by_flag(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    Addr(c.g[T0][0] as u32).sw(c, 0);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20c));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[T2][0] = Addr(c.g[V1][0] as u32).lw(c);
    let none = (c.g[T2][0] as i64) <= 0;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x284));
    if none {
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[V0][0] = 0;
        c.pc = ra;
        return;
    }
    c.cycles += 6;
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[T1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let empty = c.g[V0][0] == 0;
    c.g[T3][0] = 0;
    if empty {
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[V0][0] = c.g[T3][0];
        c.pc = ra;
        return;
    }
    c.cycles += 3;
    c.g[T4][0] = c.g[V1][0];
    c.mult(0, 10, 9, 2);
    loop {
        c.cycles += 5;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(2).lh(c);
        if c.g[V0][0] != c.g[A2][0] {
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                let ra = c.g[RA][0] as u32;
                c.g[V0][0] = c.g[T3][0];
                c.pc = ra;
                return;
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(1).lb(c);
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
            if c.g[V0][0] != c.g[A3][0] {
                c.cycles += 2;
                if c.g[T3][0] != 0 {
                    let ra = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[T3][0];
                    c.pc = ra;
                    return;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[T0][0] as u32).lw(c);
                if c.g[T3][0] == 0 {
                    c.g[T3][0] = c.g[V1][0];
                }
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                Addr(c.g[T0][0] as u32).sw(c, c.g[V0][0]);
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            }
        }
        c.cycles += 6;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
        c.g[V1][0] = Addr(c.g[T4][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0xc));
        if !more {
            let ra = c.g[RA][0] as u32;
            c.g[V0][0] = c.g[T3][0];
            c.pc = ra;
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_db88);
        }
    }
}

/// `lb` of `a + off`.
#[inline(always)]
pub(crate) fn lb(c: &mut Cpu, a: u32, off: u32) -> u64 {
    Addr(p(a, off)).lb(c)
}

// ---------------------------------------------------------------------------
// `Bolts_IndexTable3ByChar` (0x0014c3c8) / `Bolts_IndexTable4ByChar`
// (0x0014c488): clear a per-character index table and rebuild it from the
// character bytes, keeping the first occurrence of each character.

pub fn bolts_index_table3_by_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0;
        frame.save(c, 16, 0x40);
        c.g[A2][0] = 0x78;
        let this = c.g[A0][0] as u32;
        frame.save(c, 17, 0x30);
        c.g[S0][0] = sx(this);
        c.g[S1][0] = sx(this.wrapping_add(0x2fc));
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        c.g[A0][0] = sx(this.wrapping_add(0x2fc));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(this.wrapping_add(0x374));
        call_ptr(c, 0x003e_6448, 0x0014_c3fc)?;
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = sx(this.wrapping_add(0x374));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x78;
        call_ptr(c, 0x003e_6448, 0x0014_c410)?;
        c.g[V0][0] = lw_3(c, this, 0x24);
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !t {
            c.cycles += 3;
            c.g[T0][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[V1][0] = lw_3(c, this, 0xc);
            loop {
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = lb(c, c.g[V0][0] as u32, 0);
                c.g[A2][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = lw_3(c, c.g[V1][0] as u32, 0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                let same = c.g[S3][0] == c.g[A1][0];
                sw(c, c.g[V1][0] as u32, 0, c.g[V0][0]);
                if !same {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[S3][0] = c.g[A1][0];
                    sw(c, c.g[V0][0] as u32, 0, c.g[A3][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = lw_3(c, this, 0x24);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = lw_3(c, this, 0xc);
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0014_c428);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub fn bolts_index_table4_by_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0;
        frame.save(c, 16, 0x40);
        c.g[A2][0] = 0x78;
        let this = c.g[A0][0] as u32;
        frame.save(c, 17, 0x30);
        c.g[S0][0] = sx(this);
        c.g[S1][0] = sx(this.wrapping_add(0x3ec));
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        c.g[A0][0] = sx(this.wrapping_add(0x3ec));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(this.wrapping_add(0x464));
        call_ptr(c, 0x003e_6448, 0x0014_c4bc)?;
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = sx(this.wrapping_add(0x464));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x78;
        call_ptr(c, 0x003e_6448, 0x0014_c4d0)?;
        c.g[V0][0] = lw_3(c, this, 0x28);
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !t {
            c.cycles += 3;
            c.g[T0][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[V1][0] = lw_3(c, this, 0x10);
            loop {
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = lb(c, c.g[V0][0] as u32, 0);
                c.g[A2][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = lw_3(c, c.g[V1][0] as u32, 0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                let same = c.g[S3][0] == c.g[A1][0];
                sw(c, c.g[V1][0] as u32, 0, c.g[V0][0]);
                if !same {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[S3][0] = c.g[A1][0];
                    sw(c, c.g[V0][0] as u32, 0, c.g[A3][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = lw_3(c, this, 0x28);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = lw_3(c, this, 0x10);
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0014_c4e8);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BEBoltInt_SumEquippedBoltValue(cBEState*, int charIndex, int, int)`:
/// sum the `+0xc` values of the character's bolts whose equipped bit (0x10)
/// is set in the shared flags table.
pub fn be_bolt_int_sum_equipped_bolt_value(c: &mut Cpu) {
    guest(c, |c| {
        'b_14b538: {
            c.cycles += 28;
            c.g[SP][0] = sub(c.g[SP][0], 0x60);
            c.g[V0][0] = 0x9b50;
            c.sq(17, p(c.g[SP][0] as u32, 0x40));
            c.mult(0, 7, 5, 2);
            c.g[S1][0] = c.g[A2][0];
            c.sq(16, p(c.g[SP][0] as u32, 0x50));
            c.g[A2][0] = 0xf88;
            c.g[V1][0] = 0x4a_0000;
            c.mult(0, 6, 17, 6);
            c.g[V1][0] = sx(0x004a_6ca8u32.wrapping_add(0));
            c.g[S0][0] = 0x4a_0000;
            c.sq(20, p(c.g[SP][0] as u32, 0x10));
            c.g[S0][0] = sx(0x004a_6750u32.wrapping_add(0));
            c.sq(18, p(c.g[SP][0] as u32, 0x30));
            c.sq(19, p(c.g[SP][0] as u32, 0x20));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = addu(c.g[A2][0], c.g[V1][0]);
            c.sd(p(c.g[SP][0] as u32, 0), c.g[RA][0]);
            c.g[A0][0] = c.g[S0][0];
            c.g[S4][0] = addu(c.g[A3][0], c.g[A2][0]);
            call_ptr(c, 0x0014_d988, 0x0014_b4cc)?;
            c.g[S2][0] = 0;
            c.g[S3][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0014_d998, 0x0014_b4e0)?;
            let none = (c.g[S3][0] as i64) <= 0;
            c.g[V1][0] = c.g[V0][0];
            if none {
                break 'b_14b538;
            }
            c.cycles += 2;
            c.g[A1][0] = lw(c, p(c.g[S4][0] as u32, 0x288));
            c.g[A0][0] = c.g[S3][0];
            'l_14b4f0: loop {
                'b_14b518: {
                    'b_14b514: {
                        c.cycles += 6;
                        c.g[V0][0] = c.lh(p(c.g[V1][0] as u32, bolts::FLAGS));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                        c.g[V0][0] = addu(c.g[V0][0], c.g[A1][0]);
                        c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                        let negative = (c.g[V0][0] as i64) < 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        if !negative {
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                            c.g[V0][0] = addu(c.g[S4][0], c.g[V0][0]);
                            break 'b_14b518;
                        }
                        break 'b_14b514;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break 'b_14b518;
                }
                'b_14b530: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu(p(c.g[V0][0] as u32, 2));
                    c.g[V0][0] = c.g[V0][0] & 0x10;
                    let unequipped = c.g[V0][0] == 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                    if unequipped {
                        break 'b_14b530;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.lh(p(c.g[V1][0] as u32, bolts::VALUE));
                    c.g[S2][0] = addu(c.g[S2][0], c.g[V0][0]);
                    break 'b_14b530;
                }
                c.cycles += 2;
                let more = c.g[A0][0] != 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(bolts::STRIDE));
                if !more {
                    break 'b_14b538;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_b4f0);
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S2][0];
        c.lq(16, p(c.g[SP][0] as u32, 0x50));
        c.lq(17, p(c.g[SP][0] as u32, 0x40));
        c.lq(18, p(c.g[SP][0] as u32, 0x30));
        c.lq(19, p(c.g[SP][0] as u32, 0x20));
        c.lq(20, p(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        Ok(())
    });
}

/// `Bolts_LoadDB(self)` (0x14c6a0): load the bolts database (read the file,
/// release the old one, index the four tables by character), then set bit 4
/// of every bolt entry's flags.
pub fn bolts_load_db(c: &mut Cpu) {
    const FREE_DB: Callee = Callee {
        addr: 0x0014_d240,
        f: ee::s01::Bolts_FreeDB,
    };
    const READ_FILE: Callee = Callee {
        addr: 0x0014_be70,
        f: ee::s01::Bolts_ReadFile,
    };
    const INDEX1: Callee = Callee {
        addr: 0x0014_c2b0,
        f: ee::s01::Bolts_IndexBoltsByChar,
    };
    const INDEX2: Callee = Callee {
        addr: 0x0014_c320,
        f: ee::s01::Bolts_IndexTable2ByChar,
    };
    const INDEX3: Callee = Callee {
        addr: 0x0014_c3c8,
        f: ee::s01::Bolts_IndexTable3ByChar,
    };
    const INDEX4: Callee = Callee {
        addr: 0x0014_c488,
        f: ee::s01::Bolts_IndexTable4ByChar,
    };

    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        let this = abi::arg(c, 0);
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_4(c, p(this, 0x10));
        if c.g[V0][0] == 0 {
            c.cycles += 15;
            c.g[S0][0] = lw_4(c, p(this, 4));
            c.g[S0][0] = (c.g[S0][0] < 1) as u64;
            c.g[A0][0] = c.g[S1][0];
            FREE_DB.call(c, 0x0014_c6cc)?;
            c.g[A0][0] = c.g[S1][0];
            READ_FILE.call(c, 0x0014_c6d4)?;
            c.g[A0][0] = c.g[S1][0];
            INDEX1.call(c, 0x0014_c6dc)?;
            c.g[A0][0] = c.g[S1][0];
            INDEX2.call(c, 0x0014_c6e4)?;
            c.g[A0][0] = c.g[S1][0];
            INDEX3.call(c, 0x0014_c6ec)?;
            c.g[A0][0] = c.g[S1][0];
            INDEX4.call(c, 0x0014_c6f4)?;
            let skip = c.g[S0][0] != 0;
            if !skip {
                c.cycles += 7;
                c.g[V0][0] = 0x004a_0000;
                c.g[A0][0] = 0;
                c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                c.g[T4][0] = 0x9b50;
                c.g[T2][0] = 0xf88;
                c.mult(0, T0, A0, T4);
                'outer: loop {
                    c.cycles += 4;
                    c.g[V1][0] = 0;
                    c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                    c.mult(0, A0, V1, T2);
                    loop {
                        c.cycles += 6;
                        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = lw_4(c, p(c.g[V0][0] as u32, 0x28c));
                        let empty = (c.g[V1][0] as i64) <= 0;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                        if !empty {
                            c.cycles += 2;
                            c.g[A1][0] = c.g[V1][0];
                            c.g[A3][0] = 0xffff_ffff_ffff_ffef;
                            loop {
                                c.cycles += 4;
                                c.g[V1][0] = c.lhu(p(c.g[A0][0] as u32, 2));
                                c.g[V0][0] = c.g[V1][0] & 4;
                                let has4 = c.g[V0][0] != 0;
                                c.g[V0][0] = c.g[V1][0] | 0x10;
                                if !has4 {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.g[V1][0] & c.g[A3][0];
                                }
                                c.cycles += 4;
                                c.sh(p(c.g[A0][0] as u32, 2), c.g[V0][0]);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                                let more = c.g[A1][0] != 0;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                                if !more {
                                    break;
                                }
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014_c748);
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[V1][0] = c.g[A2][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 10) as u64;
                        let more = c.g[V0][0] != 0;
                        c.mult(0, A0, V1, T2);
                        if !more {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[T1][0];
                            c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
                            if c.g[V0][0] != 0 {
                                c.mult(0, T0, A0, T4);
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014_c718);
                                }
                                continue 'outer;
                            }
                            c.cycles += 1;
                            break 'outer;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0014_c728);
                        }
                    }
                }
            }
            c.lq(16, frame.at(c, 0x20));
        } else {
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *EA_MemSet(void*, int, int)`.
pub(crate) const EA_MEMSET: u32 = 0x003e_6448;

/// `void EA_MemFree(void*)`.
pub(crate) const EA_MEM_FREE: u32 = 0x0031_7e98;

// ---------------------------------------------------------------------------
// Bolts free-db.

/// The eleven 0x78-byte sub-blocks cleared by [`bolts_free_db`].
pub(crate) const BOLT_SUB_BLOCKS: [u32; 11] = [
    0x02c, 0x0a4, 0x11c, 0x194, 0x20c, 0x284, 0x2fc, 0x374, 0x3ec, 0x464, 0x4dc,
];

/// `void Bolts_FreeDB(BoltsDB *db)` (0x14d240): free the database's two heap
/// blocks and clear the eleven per-mode sub-blocks.
pub fn bolts_free_db(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);
        let db = r32(c, S0);

        c.g[A0][0] = Addr(db).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, EA_MEM_FREE, 0x0014_d264)?;
        }
        Addr(db).sw(c, 0);

        c.cycles += 56;
        // The first block's clear also covers the small header words.
        for off in [0x4u32, 0x8, 0xc, 0x10, 0x14, 0x1c, 0x20, 0x24, 0x28] {
            Addr(db).off(off).sw(c, 0);
        }
        c.g[A1][0] = 0;
        for (i, off) in BOLT_SUB_BLOCKS.iter().enumerate() {
            c.g[A2][0] = 0x78;
            c.g[A0][0] = sx(db.wrapping_add(*off));
            call_ptr(c, EA_MEMSET, 0x0014_d29c + 0x10 * i as u32)?;
        }

        c.g[A0][0] = Addr(db).off(0x18).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, EA_MEM_FREE, 0x0014_d350)?;
        }
        Addr(db).off(0x18).sw(c, 0);

        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Bolts_CollectByGroup` (0x0014d7e8): recursively collect the bolt nodes of
/// group `g` whose `parent` equals `key` into `out`, returning how many were
/// written.  The group's node range comes from `self+0xa4+4g` (first) and
/// `self+0x2c+4g` (count) over the 0x38-byte nodes at `*(self+4)`; a matching
/// node is also descended into when `depth != 0` and it is either not
/// `0x20`-flagged or `all` is set.
pub fn bolts_collect_by_group(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S7, 0x20);
        frame.save(c, S6, 0x30);
        c.g[S7][0] = c.g[A1][0]; // g
        frame.save(c, S0, 0x90);
        c.g[S6][0] = c.g[A0][0]; // self
        frame.save(c, S1, 0x80);
        c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
        frame.save(c, S3, 0x60);
        c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.save(c, S4, 0x50);
        c.g[A0][0] = 0x38; // node stride
        frame.save(c, S5, 0x40);
        c.g[V0][0] = c.g[V1][0];
        frame.save(c, FP, 0x10);
        c.g[S1][0] = c.g[A3][0]; // out
        frame.save_ra(c, 0);
        c.g[FP][0] = c.g[A2][0]; // key
        frame.save(c, S2, 0x70);
        c.g[S5][0] = c.g[T0][0]; // depth
        c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4)); // first
        c.g[S4][0] = c.g[T1][0]; // all
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c)); // count
        c.g[S3][0] = 0u64; // collected
        c.mult(0, A0, S2, A0); // first * 0x38
        c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4)); // nodes
        c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32)); // end
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if !empty {
            c.cycles += 2;
            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S2][0] as u32)); // remaining
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x8));
                if c.g[V0][0] != c.g[FP][0] {
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                } else {
                    c.cycles += 4;
                    c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                    let leaf = c.g[S5][0] == 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    let mut stop = false;
                    if !leaf {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                        c.g[V0][0] = c.g[V0][0] & 0x20;
                        if c.g[V0][0] == 0 {
                            c.g[A2][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                        } else {
                            c.cycles += 2;
                            if c.g[S4][0] == 0 {
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                                stop = true;
                            } else {
                                c.cycles += 1;
                                c.g[A2][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                            }
                        }
                    }
                    if !stop {
                        c.cycles += 9;
                        c.g[A3][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S6][0];
                        c.g[A1][0] = c.g[S7][0];
                        c.g[T0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                        c.g[T1][0] = c.g[S4][0];
                        call_ptr(c, 0x0014_d7e8, 0x0014_d8b8)?;
                        c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.cycles += 1;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                    } else {
                        // The child was skipped: the caller's loop continues.
                    }
                }
                c.cycles += 2;
                let more = c.g[S2][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_d868);
                }
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void func_0014B560(void* self, int rider, int chr, int key, int flag)`
/// (0x14b560): unlock bolt `key` of character `chr`: mark the entry unlocked,
/// unlock every bolt whose scale is the negative key, then either spend the
/// bolt's price or grant it for free.
pub fn be_bolt_int_unlock_bolt(c: &mut Cpu) {
    let frame = Frame::open(c, 0xb0);
    'b_14b650: {
        c.cycles += 44;
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S6, 0x40);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        sq(c, 30, at_2(c, SP, 0x20));
        sw_2(c, r32(c, SP), c.g[T0][0]);
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        frame.save(c, S7, 0x30);
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x0014b5a0);
        c.g[S6][0] = c.g[A1][0];
        if callee::C0014dc80.call(c, 0x0014b5a0).is_err() {
            return;
        }
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0014b5ac);
        c.g[A1][0] = 0xb;
        if callee::C0014dd58.call(c, 0x0014b5ac).is_err() {
            return;
        }
        c.g[V1][0] = 0xf88;
        c.g[A0][0] = 0x9b50;
        c.mult(0, V1, S4, V1);
        c.g[FP][0] = c.g[V0][0];
        c.mult(0, A0, S6, A0);
        c.g[V0][0] = 0x4a0000;
        c.g[V0][0] = sx(at_2(c, V0, 0x6ca8));
        c.g[A2][0] = 0x1;
        c.g[A1][0] = c.g[S5][0];
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[S2][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        c.g[RA][0] = sx(0x0014b5e0);
        c.g[A0][0] = c.g[S2][0];
        if callee::C00151c48.call(c, 0x0014b5e0).is_err() {
            return;
        }
        c.g[RA][0] = sx(0x0014b5e8);
        if callee::C0014bdb8.call(c, 0x0014b5e8).is_err() {
            return;
        }
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0014b5f4);
        c.g[A1][0] = c.g[S4][0];
        if callee::C0014d998.call(c, 0x0014b5f4).is_err() {
            return;
        }
        c.g[RA][0] = sx(0x0014b5fc);
        c.g[S0][0] = c.g[V0][0];
        if callee::C0014bdb8.call(c, 0x0014b5fc).is_err() {
            return;
        }
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0014b608);
        c.g[A1][0] = c.g[S4][0];
        if callee::C0014d988.call(c, 0x0014b608).is_err() {
            return;
        }
        let t = (c.g[V0][0] as i64) <= 0;

        if t {
            break 'b_14b650;
        } else {
            c.cycles += 4;
            c.g[S1][0] = c.g[V0][0];
            c.g[S7][0] = 0xa;
            c.g[S3][0] = sx(0u32.wrapping_sub(r32(c, S5)));
            'l_14b620: loop {
                'b_14b62c: {
                    c.cycles += 3;
                    c.g[V0][0] = lh(c, at_2(c, S0, 0xe));
                    if (c.g[V0][0] as i64) > 0 {
                        c.mult(0, V0, V0, S7);
                        break 'b_14b62c;
                    } else {
                        break 'b_14b62c;
                    }
                }
                'b_14b648: {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x1));
                        break 'b_14b648;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = lh(c, at_2(c, S0, 0x4));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x0014b644);
                        c.g[A2][0] = 0x1;
                        if callee::C00151c48.call(c, 0x0014b644).is_err() {
                            return;
                        }
                        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x1));
                        break 'b_14b648;
                    }
                }
                c.cycles += 2;
                let t = c.g[S1][0] != 0u64;
                c.g[S0][0] = sx(at_2(c, S0, 0x38));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014b620);
                    }
                    continue 'l_14b620;
                } else {
                    break 'b_14b650;
                }
            }
        }
    }
    'b_14b68c: {
        'b_14b67c: {
            c.cycles += 9;
            c.g[RA][0] = sx(0x0014b658);
            if callee::C0014bdb8.call(c, 0x0014b658).is_err() {
                return;
            }
            c.g[S0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S5][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0014b66c);
            c.g[A1][0] = c.g[S4][0];
            if callee::C0014d448.call(c, 0x0014b66c).is_err() {
                return;
            }
            let t = (c.g[V0][0] as i64) >= 0;
            c.g[A0][0] = c.g[V0][0];
            if t {
                break 'b_14b67c;
            } else {
                c.cycles += 2;
                let t = true;
                c.g[S0][0] = 0u64;
                if t {
                    break 'b_14b68c;
                } else {
                    break 'b_14b67c;
                }
            }
        }
        c.cycles += 4;
        c.g[V0][0] = 0x38;
        c.g[V1][0] = lw_5(c, at_2(c, S0, 0x4));
        c.mult(0, V0, A0, V0);
        c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        break 'b_14b68c;
    }
    'b_14b6d0: {
        'b_14b6c0: {
            c.cycles += 3;
            c.g[V0][0] = lw_5(c, r32(c, SP));
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[FP][0];
            if t {
                break 'b_14b6c0;
            } else {
                'b_14b6ac: {
                    c.cycles += 3;
                    c.g[A3][0] = lh(c, at_2(c, S0, 0xe));
                    let t = (c.g[A3][0] as i64) <= 0;
                    c.g[V0][0] = 0xa;
                    if t {
                        break 'b_14b6ac;
                    } else {
                        c.cycles += 2;
                        c.mult(0, A3, A3, V0);
                        c.g[A0][0] = c.g[FP][0];
                        break 'b_14b6ac;
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = c.g[S6][0];
                c.g[RA][0] = sx(0x0014b6b8);
                c.g[A2][0] = c.g[S4][0];
                if callee::C00150b48.call(c, 0x0014b6b8).is_err() {
                    return;
                }
                let t = true;
                frame.restore(c, S0, 0xa0);
                if t {
                    break 'b_14b6d0;
                } else {
                    break 'b_14b6c0;
                }
            }
        }
        c.cycles += 4;
        c.g[A1][0] = c.g[S6][0];
        c.g[RA][0] = sx(0x0014b6cc);
        c.g[A2][0] = c.g[S4][0];
        if callee::C00150928.call(c, 0x0014b6cc).is_err() {
            return;
        }
        frame.restore(c, S0, 0xa0);
        break 'b_14b6d0;
    }
    c.cycles += 11;
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    lq(c, 30, at_2(c, SP, 0x20));
    c.g[RA][0] = ld(c, at_2(c, SP, 0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_2(c, SP, 0xb0));
    c.pc = t;
    return;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::*;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::SP;
    pub(crate) use crate::game::be::beintmission::tests::instance_setup;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_4;
    pub(crate) use crate::game::bx::bxstring2::tests::gp_at;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE_3;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::RM as RM_2;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_5;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_2;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::main::debugmenu::tests::alloc_setup;
    pub(crate) use crate::game::main::debugmenu::tests::MEM_MUTEX;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_2;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_2;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn bolts_init_db_matches() {
        check(ssx3_ee::s01::f_0014c620, bolts_init_db, 50, &|c, seed| {
            let mut r = Rng(0x1000_0001 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for off in (0..0x2c).step_by(4) {
                c.bus.write32(DATA + off, r.next_u32());
            }
        });
    }

    #[test]
    fn bolts_get_char_bolt_count_matches() {
        check(
            ssx3_ee::s01::f_0014d988,
            bolts_get_char_bolt_count,
            100,
            &|c, seed| {
                let mut r = Rng(0x2000_0002 ^ seed as u64);
                scramble(c, &mut r);
                let idx = seed % 4;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = idx as u64;
                c.bus.write32(DATA + idx * 4 + 0x2c, r.next_u32());
            },
        );
    }

    #[test]
    fn bolts_get_char_bolts_2_matches() {
        check_lohi(
            ssx3_ee::s01::f_0014d9d0,
            bolts_get_char_bolts_2,
            300,
            &|c, seed| {
                let mut r = Rng(0x3000_0004 ^ seed as u64);
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
            },
        );
    }

    #[test]
    fn be_bolt_int_get_bolt_states_matches() {
        check_lohi(
            ssx3_ee::s01::f_0014ae28,
            be_bolt_int_get_bolt_states,
            200,
            &|c, seed| {
                let mut r = Rng(0x14ae ^ seed as u64);
                scramble(c, &mut r);
                let a1 = (r.next_u32() % 4) as u32;
                let a2 = (r.next_u32() % 4) as u32;
                c.g[A1][0] = a1 as u64;
                c.g[A2][0] = a2 as u64;
                c.g[A3][0] = (DATA + 0x800) as u64;
                let addr = BOLT_RECORDS
                    .wrapping_add(BOLT_RECORDS_OFF)
                    .wrapping_add(a1.wrapping_mul(0x9b50))
                    .wrapping_add(a2.wrapping_mul(0xf88))
                    .wrapping_add(0x28c);
                c.bus.write32(addr, r.next_u32());
            },
        );
    }

    #[test]
    fn be_bolt_int_get_instance_matches() {
        check(
            ssx3_ee::s01::f_0014abe0,
            be_bolt_int_get_instance,
            90,
            &|c, seed| {
                instance_setup(c, seed, 0x1f28, 0xb01);
            },
        );
    }

    #[test]
    fn beb_bolt_int_getters_match() {
        for (gen, hand, salt) in [
            (
                ssx3_ee::s01::f_0014ac30 as fn(&mut Cpu),
                beb_bolt_int_get_rider_character as fn(&mut Cpu),
                0xac30,
            ),
            (
                ssx3_ee::s01::f_0014ac50,
                beb_bolt_int_get_rider_profile,
                0xac50,
            ),
        ] {
            check(gen, hand, 60, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A1][0] = (seed % 12) as u64;
                let _ = &mut r;
            });
        }
    }

    #[test]
    fn beb_bolt_int_get_rider_profile_and_char_matches() {
        check(
            ssx3_ee::s01::f_0014ac70,
            beb_bolt_int_get_rider_profile_and_char,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0xac70, 0);
                c.g[A1][0] = (seed % 12) as u64;
                // Sometimes `out` aliases the input pointer.
                c.g[A2][0] = (if seed % 4 == 0 {
                    DATA + 0x300
                } else {
                    DATA + 0x400
                }) as u64;
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn be_find_rider_by_profile_matches() {
        check(
            ssx3_ee::s01::f_0014a130,
            be_find_rider_by_profile,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xa130, 0);
                // All six rider profile slots index entry 0 of the table.
                for i in 0..6u32 {
                    c.bus.write32(0x5305b0 + i * 4, 0);
                }
                fill(c, &mut r, 0x535b20, 0x200);
                let bit = c.bus.read32(0x535b30) & 1;
                c.g[A0][0] = if seed % 2 == 0 {
                    bit as u64
                } else {
                    (bit ^ 1) as u64
                };
            },
        );
    }

    #[test]
    fn beb_bolt_int_profile_state_matches() {
        for (gen, hand, salt, ptr_arg) in [
            (
                ssx3_ee::s01::f_0014ad50 as fn(&mut Cpu),
                beb_bolt_int_get_rider_profile_data as fn(&mut Cpu),
                0xad50,
                true,
            ),
            (
                ssx3_ee::s01::f_0014ae60,
                beb_bolt_int_get_rider_bolt_states,
                0xae60,
                true,
            ),
            (
                ssx3_ee::s01::f_0014acb0,
                beb_bolt_int_set_bolt_states,
                0xacb0,
                true,
            ),
        ] {
            check(gen, hand, 40, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A1][0] = (seed % 8) as u64;
                if ptr_arg {
                    c.g[A2][0] = (DATA + 0x2000) as u64;
                }
                c.g[A3][0] = (DATA + 0x3000) as u64;
                c.g[A0][0] = (DATA + 0x4000) as u64;
                fill(c, &mut r, DATA, 0x4000);
            });
        }
    }

    /// Call a generated helper as a setup step (arguments in a0..t1).
    pub(crate) fn load(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA,
            "setup call {addr:#x} did not return"
        );
        c.g[V0][0]
    }

    /// The main heap's arena over 256 KiB of test RAM, so constructors and
    /// destructors that allocate or free take their normal path.
    pub(crate) fn alloc_ready(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[GP][0] = GAME_GP as u64;
        load(
            c,
            0x0031_ff60,
            ssx3_ee::s10::f_0031ff60,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
        c.bus.write32(GAME_GP + 0xdd8, 1); // size-class pools ready
    }

    #[test]
    fn be_score_int_get_high_score_table_matches() {
        check_lohi(
            ee::s01::f_0014ab20__generated,
            be_score_int_get_high_score_table,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x14ab20, 0);
                let (a, b) = match seed % 4 {
                    0 => (0u64, 6u64),
                    1 => (0, 7),
                    2 => (16, 7),
                    _ => (0, 12),
                };
                c.g[A0][0] = a;
                c.g[A1][0] = b;
            },
        );
    }

    #[test]
    fn bolts_destroy_db_matches() {
        check(
            ee::s01::f_0014c658__generated,
            bolts_destroy_db,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x14c658, 0);
                alloc_ready(c);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn be_bolt_int_save_equip_flags_matches() {
        check(
            ssx3_ee::s01::f_0014aea8__generated,
            be_bolt_int_save_equip_flags,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x14aea8);
                c.g[A0][0] = 0;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                let n = seed % 5;
                c.bus.write32(0x004a_6ca8 + 0x28c, n);
                for i in 0..n {
                    let e = 0x004a_6ca8 + 0x290 + 4 * i;
                    c.bus.write16(e, r.next_u32() as u16);
                    c.bus.write16(
                        e + 2,
                        if seed % 2 == 0 {
                            (r.next_u32() | 0x10) as u16
                        } else {
                            (r.next_u32() & !0x10) as u16
                        },
                    );
                }
            },
        );
    }

    #[test]
    fn bolts_index_bolts_by_char_matches() {
        check(
            ee::s01::f_0014c2b0__generated,
            bolts_index_bolts_by_char,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x14c2_b0);
                let count = seed % 4;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x1c, count);
                c.bus.write32(DATA + 4, DATA + 0x1000);
                for i in 0..count {
                    c.bus
                        .write8(DATA + 0x1000 + i * 0x38, (r.next_u32() % 3) as u8);
                }
                fill(c, &mut r, DATA + 0x2c, 4);
                fill(c, &mut r, DATA + 0xa4, 4);
            },
        );
    }

    #[test]
    fn be_get_event_record_index_matches() {
        check(
            ee::s01::f_0014ab68__generated,
            be_get_event_record_index,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x14_ab68, 0);
                let a = (seed % 20) as u32;
                let b = [6u32, 7, 8, 9, 10, 11, 12, 3][(seed % 8) as usize];
                c.g[A0][0] = a as u64;
                c.g[A1][0] = b as u64;
                let row = DATA + 0x2200 + 0x20 * (a % 5);
                if a < 17 {
                    w(c, 0x0045_a2f8 + 16 * a, row);
                }
                w(c, row, b);
                w(c, row + 4, 3 + seed % 4);
                w(c, row + 8, 0xffff_ffff);
                w(c, row + 0xc, 0x1a);
                c.pc = 0x0014_ab68;
            },
        );
    }

    #[test]
    fn bolts_find_bolt_index_matches() {
        check(
            ee::s01::f_0014d448__generated,
            bolts_find_bolt_index,
            SEEDS_3,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x4d448, 0);
                let self_ = DATA + 0x100;
                let group = (seed % 4) as u32;
                let key = ((seed / 4) % 8) as u32;
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx(group);
                c.g[A2][0] = sx(key);
                if seed % 3 == 0 {
                    let tab = DATA + 0x3000;
                    c.bus.write32(self_ + 0x4dc + 4 * group, tab);
                    for k in 0..8u32 {
                        c.bus.write16(tab + 2 * k, (k as u32 + seed) as u16);
                    }
                } else {
                    c.bus.write32(self_ + 0x4dc + 4 * group, 0);
                    let first = ((seed / 3) % 4) as u32;
                    let count = ((seed / 12) % 4) as u32;
                    c.bus.write32(self_ + 0xa4 + 4 * group, first);
                    c.bus.write32(self_ + 0x2c + 4 * group, count);
                    let base_2 = DATA + 0x1000;
                    c.bus.write32(self_ + 4, base_2);
                    for i in 0..4u32 {
                        c.bus
                            .write16(base_2 + i * 0x38 + 4, ((i + seed) % 8) as u16);
                    }
                    let _ = &mut r;
                }
            },
        );
    }

    #[test]
    fn bolts_is_descendant_of_matches() {
        check(
            ee::s01::f_0014d908__generated,
            bolts_is_descendant_of,
            SEEDS_3,
            &|c, seed| {
                base_2(c, seed, 0x4d908, 0);
                let self_ = DATA + 0x100;
                let base_ = DATA + 0x1000;
                c.g[A0][0] = sx(self_);
                c.bus.write32(self_ + 4, base_);
                for g in 0..4u32 {
                    c.bus.write32(self_ + 0x4dc + 4 * g, 0);
                    c.bus.write32(self_ + 0xa4 + 4 * g, 0);
                    c.bus.write32(self_ + 0x2c + 4 * g, 4);
                }
                // Group 0: entries 0..3.
                //  e0: group tag 0, own key 1, parent key 7
                //  e1: group tag 0, own key 7, parent key -1
                //  e2: group tag 0, own key 7, parent key 5
                let keys = [(1u16, 7i16), (7, -1), (7, 5), (2, -1)];
                for (i, (own, parent)) in keys.iter().enumerate() {
                    let e = base_ + 0x38 * i as u32;
                    c.bus.write8(e, 0);
                    c.bus.write16(e + 4, *own);
                    c.bus.write16(e + 6, *parent as u16);
                }
                let start = (seed % 5) as u32;
                let key: u16 = match seed % 4 {
                    0 => 1,
                    1 => 7,
                    2 => 0xffff,
                    _ => 5,
                };
                let entry = if start >= 4 { 0 } else { base_ + 0x38 * start };
                c.g[A1][0] = sx(entry);
                c.g[A2][0] = sx(key as u32);
            },
        );
    }

    pub(crate) const SRC: u32 = DATA + 0x4000;

    // ---------------------------------------------------------------- tables

    #[test]
    fn bolts_index_table2_by_char_matches() {
        check(
            ee::s01::f_0014c320__generated,
            bolts_index_table2_by_char,
            6,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x14c320, 2);
                c.g[A0][0] = OBJ as u64;
                let n = (seed % 4) as u32;
                w(c, OBJ + 0x20, n);
                w(c, OBJ + 8, SRC);
                fill(c, &mut r, SRC, 4 * 4);
                for i in 0..n {
                    c.bus.write8(SRC + 0xc * i, ((i * 3) % 30) as u8);
                }
            },
        );
    }

    #[test]
    fn bolts_find_first_child_matches() {
        check(
            ee::s01::f_0014d558__generated,
            bolts_find_first_child,
            150,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x14d5);
                let g = (seed % 3) as u32;
                let key = [0xffff_ffffu64, 7, 0x41][(seed / 3 % 3) as usize];
                let variant = seed / 9 % 4;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = g as u64;
                c.g[A2][0] = key;
                c.bus.write32(
                    DATA + g * 4 + 0x194,
                    if variant == 0 { 0 } else { DATA + 0x1000 },
                );
                c.bus.write32(DATA + g * 4 + 0x11c, DATA + 0x1200);
                c.bus.write32(DATA + g * 4 + 0x2c, 3);
                c.bus.write32(DATA + g * 4 + 0xa4, 0);
                c.bus.write32(DATA + 4, DATA + 0x2000);
                c.bus.write16(
                    DATA + 0x1200 + (key as u32 & 0x7f) * 2,
                    if variant >= 2 { 1 } else { 0 },
                );
                c.bus.write16(DATA + 0x1000 + (key as u32 & 0x7f) * 2, 2);
                for i in 0..4u32 {
                    let e = DATA + 0x2000 + 0x38 * i;
                    for k in 0..0x38 / 4 {
                        c.bus.write32(e + 4 * k, r.next_u32());
                    }
                    c.bus.write16(e + 6, [0x41u16, 7, 9, 3][i as usize]);
                }
            },
        );
    }

    #[test]
    fn bolts_index_table3_by_char_matches() {
        check(
            ee::s01::f_0014c3c8__generated,
            bolts_index_table3_by_char,
            40,
            &|c, seed| {
                let mut r = Rng(0x0014_c3c8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let this = DATA;
                c.bus.write32(this + 0x24, (seed % 4) as u32);
                c.bus.write32(this + 0xc, DATA + 0x1000);
                for i in 0..4 {
                    c.bus.write32(DATA + 0x1000 + 8 * i, r.next_u32());
                    c.bus
                        .write8(DATA + 0x1000 + 8 * i, (r.next_u32() % 5) as u8);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn bolts_index_table4_by_char_matches() {
        check(
            ee::s01::f_0014c488__generated,
            bolts_index_table4_by_char,
            40,
            &|c, seed| {
                let mut r = Rng(0x0014_c488 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let this = DATA;
                c.bus.write32(this + 0x28, (seed % 4) as u32);
                c.bus.write32(this + 0x10, DATA + 0x1000);
                for i in 0..4 {
                    c.bus.write32(DATA + 0x1000 + 8 * i, r.next_u32());
                    c.bus
                        .write8(DATA + 0x1000 + 8 * i, (r.next_u32() % 5) as u8);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn sum_equipped_bolt_value_matches() {
        diff(
            ee::s01::f_0014b478__generated,
            be_bolt_int_sum_equipped_bolt_value,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14_b478, 0);
                globals(c);
                let idx = seed % 4;
                let count = (seed % 5) as u32;
                w(c, 0x004a_6750 + idx * 4 + 0x2c, count);
                w(c, 0x004a_6750 + 4, DATA);
                w(c, 0x004a_6750 + idx * 4 + 0xa4, 0);
                w(c, 0x004a_7eb8, DATA + 0x3000);
                for i in 0..count {
                    let bolt = DATA + i * bolts::STRIDE;
                    c.bus.write16(bolt + bolts::FLAGS, (seed as u16 % 5) as u16);
                    c.bus.write16(bolt + bolts::VALUE, r.next_u32() as u16);
                    c.bus.write16(
                        DATA + 0x3000 + 2 * (seed as u32 % 5),
                        (if seed % 3 == 0 { 0x8000 } else { 0x10 }) as u16,
                    );
                }
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = idx as u64;
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn bolts_free_db_matches() {
        check(
            ee::s01::f_0014d240__generated,
            bolts_free_db,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14_d240, 11);
                alloc_setup(c, &mut r);
                c.g[A0][0] = DATA as u64;
                // Cover both the free and the already-free path.
                w(c, DATA, if seed % 2 == 0 { 0 } else { ptr(&mut r) });
                w(c, DATA + 0x18, if seed % 3 == 0 { 0 } else { ptr(&mut r) });
            },
        );
    }

    pub(crate) const BOLT_SELF: u32 = DATA + 0x1_0000;

    #[test]
    fn be_bolt_int_unlock_bolt_matches() {
        check_all(
            ee::s01::f_0014b560__generated,
            be_bolt_int_unlock_bolt,
            20,
            &|c, seed| {
                let _ = base_5(c, seed, 0x14b5_60, 12);
                c.g[A0][0] = sx(BOLT_SELF);
                c.g[A1][0] = sx(seed % 3); // rider slot
                c.g[A2][0] = sx(seed % 4); // character
                c.g[A3][0] = sx((seed / 4) % 6); // bolt key
                c.g[T0][0] = sx(seed % 2); // free
            },
        );
    }
}
