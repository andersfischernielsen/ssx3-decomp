//! `src/fe/ovstates.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use crate::tick;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::sp;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Flow;
use crate::events::back_edge_unwinds;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::fe::festaterewards2::init_state;
pub(crate) use crate::game::dirtysock::tags2::p_2 as p;
pub(crate) use crate::game::dirtysock::tags2::sw;
pub(crate) use crate::game::dirtysock::tags2::const_reg;
pub(crate) use crate::game::wscript::wscriptman::p as p_2;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_IGNORED;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_ACTIVATED;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p as p_3;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::be::beintplayer::lh;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::sound::soundsys::p as p_5;
pub(crate) use crate::game::sound::soundsys::gp_4 as gp;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0019_4778, c_fe_state_title_init, "cFEStateTitle_Init"),
    (0x0019_47f8, cfe_state_title_transition_in, "cFEStateTitle_TransitionIn"),
    (0x0019_48a8, cfestatetitle_update, "cFEStateTitle_Update"),
    (0x0019_4988, c_fe_state_title_on_widget_created, "cFEStateTitle_OnWidgetCreated"),
    (0x0019_49e0, c_fe_state_title_on_widget_action, "cFEStateTitle_OnWidgetAction"),
    (0x0019_4a48, cfe_state_title_on_widget_notify, "cFEStateTitle_OnWidgetNotify"),
    (0x0019_4a60, fe_state_main_menu_ctor, "cFEStateMainMenu_ctor"),
    (0x0019_4d18, cfe_state_main_menu_on_widget_created, "cFEStateMainMenu_OnWidgetCreated"),
    (0x0019_4dd0, cfestatemainmenu_update, "cFEStateMainMenu_Update"),
    (0x0019_52e8, fe_state_main_menu_on_widget_notify, "cFEStateMainMenu_OnWidgetNotify"),
    (0x0019_5498, fe_state_bonus_material_ctor, "cFEStateBonusMaterial_ctor"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_NAMES;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_COUNT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::PREVIEW_REWARD_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::festaterewards2::callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards2::callee::SECTION_MAN_FIND_LOGICAL_SECTION;
    pub(crate) use crate::game::fe::festaterewards2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::festaterewards2::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_PREV_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SNAP_TO_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_ADD_MOVIE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_POLL_LOADING;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::fe::festaterewards2::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::fe::festaterewards2::callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_MAN_FREE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_GET_POOL_BLOCK_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::DL_HEAP_USABLE_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_REPORT_POOL_STATS;
    pub(crate) use crate::game::fe::festaterewards2::callee::PS2_PAD_CREATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::ANIM_OBJECT_V36;
    pub(crate) use crate::game::fe::festaterewards2::callee::DYNAMIC_EMITTER_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::TWSCRIPT_CACHE_RELOCATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_UNLOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_ARRANGEMENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_READ_MODE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEMSET;
    pub(crate) use crate::game::fe::festaterewards2::callee::SPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LISTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LOBBY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_PEER_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SHARED_LOGIN_MAN_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_MAN_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2 as SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_FIELD_B0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_LIST_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE_RIDER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_VIEW_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_API_ADD_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_CREATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::APP_SOMETHING;
    pub(crate) use crate::game::wscript::wscriptman::callee::SET_FADE_GAIN;
    pub(crate) use crate::game::wscript::wscriptman::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND as CROWD_AUDIO_INSTANCE_STOP_SOUND_2;
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
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_3;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SET_USER_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_FILE_MOUNT_BIG;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_MEM_ALLOC_NAMED;
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::wscript::wscriptman::table::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_BANK_REMOVE;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_MIDI_RESUME;
    pub(crate) use crate::game::wscript::wscriptman::table::SCE_MC_FLUSH;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_VEC_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::MEM_FREE;
    pub(crate) use crate::game::wscript::wscriptman::table::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::table::WSCRIPT_PROCESS_LIST_POP_FRONT;
    pub(crate) use crate::game::wscript::wscriptman::table::STR_HASH;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32_2;
    pub(crate) use crate::game::wscript::wscriptman::table::BXFILE_EXISTS;
    pub(crate) use crate::game::wscript::wscriptman::table::STREAM_FILE_DESTROY;
    pub(crate) use crate::game::wscript::wscriptman::table::UI_SCREEN_FIND_OBJECT;
}


// ---------------------------------------------------------------------------
// Front-end states.

/// `cFEStateBonusMaterial* cFEStateBonusMaterial_ctor(cFEStateBonusMaterial*)`
/// (0x195498): the shared UI-state constructor, then vtable `0x46b918` at +0x8
/// and state id 5 at +0xC.
pub fn fe_state_bonus_material_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::UI_STATE_CTOR.call(c, 0x0019_54ac)?;
        init_state(c, this, 0x0046_b918, 5);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateMainMenu* cFEStateMainMenu_ctor(self)` (0x194a60): UI state
/// constructor, then the main-menu vtable/state and two cleared fields.
pub fn fe_state_main_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_CTOR_2, 0x0019_4a74)?;
        let this = r32(c, S0);
        c.g[V0][0] = 2;
        const_reg(c, V1, 0x47_0000, -0x4618);
        sw(c, this, 0x0c, c.g[V0][0]);
        sw(c, this, 0x08, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        sw(c, this, 0x48, 0);
        sw(c, this, 0x4c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void cFEStateTitle_OnWidgetCreated(cFEStateTitle* self, UIObject* item)`
/// (0x194988): when the widget is the title screen's, clear its user data.
pub fn c_fe_state_title_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x4a_17d0; // D_004A17D0
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let item = r32(c, A1);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = Addr(p_2(item, 0x38)).lw(c);
        call_ptr(c, table::STR_HASH, 0x0019_49ac)?;
        let other = c.g[S0][0] != c.g[V0][0];
        frame.restore(c, S0, 0x20);
        if !other {
            c.cycles += 6;
            Addr(p_2(item, 0x18)).sw(c, 0);
            c.g[A0][0] = sx(item);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            callee::UI_OBJECT_SET_USER_DATA.call(c, 0x0019_49c8)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int cFEStateTitle_OnWidgetNotify(State*, void*, int key)` (0x00194a48):
/// the title accepts every key but reports 0x101 only for the start widget.
pub fn cfe_state_title_on_widget_notify(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] ^= 0x6;
    c.g[V1][0] = 0x101;
    c.g[V0][0] = if c.g[A2][0] == 0 { 0x101 } else { 0x100 };
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Front-end widget notifications.

/// `int cFEStateMainMenu_OnWidgetNotify(cFEStateMainMenu* self, void* widget,
/// int id, void* data)` (0x1952E8): a `Back`/`Continue` widget reports whether
/// the selection it points at (`+0x48`) is null.
pub fn fe_state_main_menu_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 6;
    if c.g[A2][0] == c.g[V0][0] {
        c.g[A0][0] = c.lw(ea(c.g[A0][0], 0x48));
    } else {
        c.cycles += 3;
        c.g[V0][0] = 9;
        let is_nine = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = WIDGET_IGNORED;
        if is_nine {
            c.cycles += 2;
            c.g[V0][0] = WIDGET_ACTIVATED;
            abi::ret(c);
            return;
        }
        // (the original's `li t, 1; beqz` tail always returns here)
        c.cycles += 4;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[V1][0] = WIDGET_IGNORED;
    c.g[V0][0] = 1;
    if c.g[A0][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}


/// `cFEStateTitle_OnWidgetAction(cFEStateTitle* self, Widget* w, int type)`
/// (0x1949e0): on a widget action of type 5, let the first UI object handle
/// the message and push the resulting state.
pub fn c_fe_state_title_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 5;
        frame.save_ra(c, 0);
        let action = c.g[A2][0] != c.g[V0][0];
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        if !action {
            c.cycles += 11;
            c.g[V0][0] = lw(c, p_3(this as u32, 0x10));
            c.g[A1][0] = this;
            c.g[A2][0] = lw(c, p_3(c.g[V1][0] as u32, 0x18));
            c.g[A3][0] = lw(c, c.g[V0][0] as u32);
            c.g[V1][0] = lw(c, p_3(c.g[A3][0] as u32, 4));
            c.g[A0][0] = lh(c, p_3(c.g[V1][0] as u32, 0x20));
            c.g[V0][0] = lw(c, p_3(c.g[V1][0] as u32, 0x24));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_4a20)?;
            let handled = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[V0][0];
            if !handled {
                c.cycles += 3;
                c.g[A0][0] = lw(c, p_3(this as u32, 0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f400, 0x0019_4a34)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Front-end state changes.

/// `void cFEStateTitle_Init(cFEStateTitle* st)` (0x00194778): clear the
/// autosave counter, create the title screen from the screen engine named by
/// `st+0x10`, start it, then set the FE music screen.
pub fn c_fe_state_title_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = sx(0x004a_17c8); // "cFEStateTitle"
        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0);
        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x0031_7670, 0x0019_47a0)?; // StrHash
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0019_47b4)?; // UIEngine_CreateScreen
        c.sw((c.g[S0][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0019_47cc)?; // UIScreen_StartThread
        }
        c.cycles += 10;
        call_ptr(c, 0x0028_b180, 0x0019_47d4)?; // SSXAudioSystem_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0028_f140, 0x0019_47e0)?; // SSXAudioSystem_SetFEMusicScreen
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}


/// The pre-fade wait of the title screen: `D_004A17C0` (`gp-0x1930`) starts at
/// 300 (5 s at 60 Hz) in the game's data and is decremented once per update by
/// `cFEStateTitle_TransitionIn` only (Ghidra: the only references to
/// `0x4a17c0` are the load and the store in that function).
const TRANSIENT_WAIT_FRAMES: u32 = 300;


/// `bool cFEStateTitle_TransitionIn(self)` (0x1947f8): count the transition
/// in over a few frames, then fade in the title's two text widgets.
pub fn cfe_state_title_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V0][0] = Addr(p_4(r32(c, GP), 0xffff_e6d0)).lw(c);
        let mut fade = c.g[V0][0] == 0;
        if !fade {
            c.cycles += 3;
            // The countdown is a wall-clock wait (5 s at 60 Hz). The data
            // initializer is 300 frames; `tick::on_vblank` rewrites it to
            // 300 * rate / 60 ticks once, when it applies the rate (see
            // `tick::FRAME_COUNTS`), so the countdown is the original one.
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let still = c.g[V0][0] != 0;
            c.sw(p_4(r32(c, GP), 0xffff_e6d0), c.g[V0][0]);
            if still {
                // one more frame to wait
                c.cycles += 1;
                c.g[V0][0] = 0;
                frame.restore(c, S0, 0x10);
                c.cycles += 4;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            fade = true;
        }
        if fade {
            // create the screen and fade its title texts in
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2470));
            call_ptr(c, 0x0031_7670, 0x0019_4828)?;
            c.g[A0][0] = Addr(p_4(c.g[S0][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0019_4834)?;
            c.g[A2][0] = c.g[V0][0];
            let missing = c.g[A2][0] == 0;
            c.g[A1][0] = 0x1;
            if !missing {
                c.cycles += 5;
                c.g[V0][0] = Addr(p_4(c.g[A2][0] as u32, 8)).lw(c);
                c.g[A0][0] = Addr(p_4(c.g[V0][0] as u32, 0x38)).lh(c);
                c.g[V1][0] = Addr(p_4(c.g[V0][0] as u32, 0x3c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0019_4854)?;
            }
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x17d0));
            call_ptr(c, 0x0031_7670, 0x0019_4860)?;
            c.g[A0][0] = Addr(p_4(c.g[S0][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0019_486c)?;
            c.g[A2][0] = c.g[V0][0];
            let missing = c.g[A2][0] == 0;
            c.g[A1][0] = 0x1;
            if !missing {
                c.cycles += 5;
                c.g[V0][0] = Addr(p_4(c.g[A2][0] as u32, 8)).lw(c);
                c.g[A0][0] = Addr(p_4(c.g[V0][0] as u32, 0x48)).lh(c);
                c.g[V1][0] = Addr(p_4(c.g[V0][0] as u32, 0x4c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0019_488c)?;
            }
            c.cycles += 2;
            c.g[V0][0] = 0x1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `a + off`.
#[inline(always)]
pub(crate) fn at(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}


/// `addiu sp, sp, -n`.
#[inline(always)]
pub(crate) fn open(c: &mut Cpu, n: u32) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(n));
}


/// `addiu sp, sp, n; jr ra` (`ra` already reloaded by the caller).
#[inline(always)]
pub(crate) fn close(c: &mut Cpu, n: u32) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(n));
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// FE widget messages.

/// Hash of the string at `site` via the ELF hash, i.e. `GetHashValue32`.
pub(crate) fn hash_eq(c: &mut Cpu, item: u64, site: u32, ret: u32) -> Flow {
    c.g[S0][0] = Addr(item as u32).off(0x38).lw(c);
    c.g[A0][0] = sx(site);
    call_ptr(c, 0x0031_7670, ret)?;
    Ok(())
}


/// `void cFEStateMainMenu_OnWidgetCreated(cFEStateMainMenu* self,
/// FEItem* item)` (0x194d18): map the widget's hash id (+0x38) to a screen
/// action id (+0x18), comparing against five hash-keyed widgets.
pub fn cfe_state_main_menu_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        open(c, 0x30);
        c.g[A0][0] = 0x460000;
        c.sq(17, at(c.g[SP][0] as u32, 0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let item = c.g[A1][0];
        c.g[S1][0] = item;
        c.sq(16, at(c.g[SP][0] as u32, 0x20));
        c.g[RA][0] = sx(0x0019_4d3c);
        c.g[S0][0] = Addr(item as u32).off(0x38).lw(c);
        call_ptr(c, 0x0031_7670, 0x0019_4d3c)?;
        let same = c.g[S0][0] == c.g[V0][0];
        c.g[A0][0] = 0x460000;
        if same {
            c.cycles += 2;
            c.g[V0][0] = 0xb;
            c.cycles += 2;
            Addr(c.g[S1][0] as u32).off(0x18).sw(c, c.g[V0][0]);
            c.lq(16, at(c.g[SP][0] as u32, 0x20));
            c.cycles += 4;
            c.lq(17, at(c.g[SP][0] as u32, 0x10));
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            close(c, 0x30);
            return Ok(());
        }
        for (site, id, ret) in [
            (0x0046_0060u32, 0x9u64, 0x0019_4d58u32),
            (0x0046_0040, 0x16, 0x0019_4d74),
            (0x0046_0070, 0x5, 0x0019_4d90),
            (0x004a_17d8, 0x31, 0x0019_4dac),
        ] {
            c.cycles += 5;
            hash_eq(c, item, site, ret)?;
            if c.g[S0][0] == c.g[V0][0] {
                if id == 0x31 {
                    c.cycles += 1;
                    c.g[V0][0] = 0x31;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = id;
                }
                c.cycles += 2;
                Addr(c.g[S1][0] as u32).off(0x18).sw(c, c.g[V0][0]);
                c.lq(16, at(c.g[SP][0] as u32, 0x20));
                c.cycles += 4;
                c.lq(17, at(c.g[SP][0] as u32, 0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                close(c, 0x30);
                return Ok(());
            }
        }
        c.lq(16, at(c.g[SP][0] as u32, 0x20));
        c.cycles += 4;
        c.lq(17, at(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Front end: title / main menu.

/// The title screen's idle timeout before the attract mode: 30 s, the
/// original's `self->timer > 0x708` in frames.
const TITLE_IDLE_FRAMES: u32 = 1800;


/// `void cFEStateTitle_Update(self)` (0x1948a8): tick the idle timer and drop
/// back to the attract mode after 30 seconds; any active list item resets it.
///
/// The idle timeout is a wall-clock 30 s (`self->timer > 0x708` in the C++,
/// i.e. 1800 frames at 60 Hz), not a fixed number of updates: the limit is
/// `ticks_from_frames60(1800)`, so the counter `timer + 1` continuing means
/// `timer + 1 <= 1800` (the original `timer + 1 < 1801`; the two forms agree
/// for every int32, and at 60 the limit is exactly 1800).
pub fn cfestatetitle_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e510, 0x0019_48bc)?;
        c.g[V0][0] = Addr(p_5(r32(c, S0), 0x48)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        let idle = tick::ticks_from_frames60(c, TITLE_IDLE_FRAMES) as i64;
        c.g[V1][0] = ((c.g[V0][0] as i64) <= idle) as u64;
        let ticking = c.g[V1][0] != 0;
        Addr(p_5(r32(c, S0), 0x48)).sw(c, c.g[V0][0]);
        if !ticking {
            c.cycles += 5;
            c.g[V0][0] = Addr(gp(c, -0x1724)).lw(c);
            Addr(p_5(r32(c, S0), 0x48)).sw(c, 0);
            c.g[V0][0] = c.g[V0][0] | 4;
            Addr(gp(c, -0x1724)).sw(c, c.g[V0][0]);
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 7;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[A3][0] = 0;
        c.g[V1][0] = 0;
        c.g[T0][0] = 0x1c;
        c.g[T1][0] = sx(p_5(r32(c, V0), 0xb0));
        c.g[V0][0] = sx(r32(c, V1) << 2);
        loop {
            c.cycles += 7;
            c.g[A2][0] = sx(r32(c, V1).wrapping_add(1));
            c.g[V0][0] = sx(p_5(r32(c, T1), r32(c, V0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(r32(c, V1)).lw(c);
            c.g[A1][0] = Addr(r32(c, V1)).lw(c);
            if c.g[A1][0] == 0 {
                c.g[V1][0] = c.g[A2][0];
            } else {
                c.cycles += 1;
                c.g[A0][0] = 0;
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let out_of_range = c.g[V0][0] == 0;
                    c.mult(0, 10, 4, 8);
                    let mut found = 0u64;
                    if out_of_range {
                        c.cycles += 1;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = sx(p_5(r32(c, T2), r32(c, V1)));
                        found = Addr(p_5(r32(c, V0), 0x10)).lw(c);
                    }
                    c.cycles += 2;
                    if found == 0 {
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                        if event_due(c) {
                            deliver_event_here(c, 0x0019_4920);
                        }
                    } else {
                        c.cycles += 1;
                        c.g[A3][0] = 1;
                        break;
                    }
                }
                c.cycles += 2;
                if c.g[A3][0] != 0 {
                    Addr(p_5(r32(c, S0), 0x48)).sw(c, 0);
                }
                c.cycles += 1;
                c.g[V1][0] = c.g[A2][0];
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx(r32(c, V1) << 2);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_4900);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateMainMenu_Update(self)` (0x194dd0): select the two menu
/// columns and pick the first entry if both are valid.
pub fn cfestatemainmenu_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        call_ptr(c, 0x0039_e510, 0x0019_4df0)?;
        c.g[V0][0] = Addr(p_5(r32(c, S1), 0x10)).lw(c);
        c.g[S0][0] = Addr(p_5(r32(c, V0), 0xc)).lw(c);
        Addr(p_5(r32(c, S1), 0x55)).sb(c, 0);
        c.g[V0][0] = Addr(p_5(r32(c, S0), 8)).lw(c);
        loop {
            c.cycles += 12;
            c.g[A1][0] = c.g[S2][0] & 0xff;
            c.g[A0][0] = Addr(p_5(r32(c, V0), 0x160)).lh(c);
            c.g[A2][0] = Addr(p_5(r32(c, V0), 0x164)).lw(c);
            let select = r32(c, A2);
            c.g[A0][0] = sx(p_5(r32(c, S0), r32(c, A0)));
            call_ptr(c, select, 0x0019_4e14)?;
            c.g[V1][0] = Addr(p_5(r32(c, S0), 8)).lw(c);
            c.g[A0][0] = Addr(p_5(r32(c, V1), 0x168)).lh(c);
            c.g[V0][0] = Addr(p_5(r32(c, V1), 0x16c)).lw(c);
            let valid = r32(c, V0);
            c.g[A0][0] = sx(p_5(r32(c, S0), r32(c, A0)));
            call_ptr(c, valid, 0x0019_4e28)?;
            let not_valid = c.g[V0][0] == 0;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            if !not_valid {
                c.cycles += 3;
                c.g[V0][0] = Addr(p_5(r32(c, S1), 0x55)).lbu(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                Addr(p_5(r32(c, S1), 0x55)).sb(c, c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 2i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(p_5(r32(c, S0), 8)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0019_4e00);
            }
        }
        c.cycles += 9;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1398));
        call_ptr(c, 0x0031_7670, 0x0019_4e54)?;
        c.g[A0][0] = Addr(p_5(r32(c, S1), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0019_4e60)?;
        c.g[A0][0] = c.g[V0][0];
        let no_menu = c.g[A0][0] == 0;
        c.g[V0][0] = 2;
        if no_menu {
            c.cycles += 1;
            c.lq(S0, frame.at(c, 0x30));
            c.cycles += 1;
            c.lq(S1, frame.at(c, 0x20));
        } else {
            c.cycles += 3;
            c.g[V1][0] = Addr(p_5(r32(c, A0), 0x95)).lbu(c);
            let not_sel = c.g[V1][0] != c.g[V0][0];
            c.lq(S0, frame.at(c, 0x30));
            if not_sel {
                c.cycles += 1;
                c.lq(S1, frame.at(c, 0x20));
            } else {
                c.cycles += 4;
                c.g[V0][0] = Addr(p_5(r32(c, S1), 0x55)).lb(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                let few = c.g[V0][0] == 0;
                c.lq(S1, frame.at(c, 0x20));
                if !few {
                    c.cycles += 2;
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0039_b960, 0x0019_4e90)?;
                    c.cycles += 1;
                    c.lq(S0, frame.at(c, 0x30));
                    c.cycles += 1;
                    c.lq(S1, frame.at(c, 0x20));
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::put_vec;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fill;
    use ssx3_ee as ee;
    use crate::difftest::scramble;
    use crate::difftest::RA as RET;
    use crate::difftest::SP;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS_3;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS as SEEDS_4;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_2;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool_3;
    pub(crate) use crate::game::sound::soundsys::tests::load_2;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_4;
 // gp - 0x428c

    // -------------------------------------------------------------- front end

    #[test]
    fn fe_state_bonus_material_ctor_matches() {
        check(ssx3_ee::s02::f_00195498__generated, fe_state_bonus_material_ctor, SEEDS, &|c, seed| {
            machine(c, seed, 0x1954_98, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
        });
    }


    #[test]
    fn fe_state_main_menu_ctor_matches() {
        check(
            ssx3_ee::s02::f_00194a60__generated,
            fe_state_main_menu_ctor,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x194a60, 0);
                be_library(c);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }


    #[test]
    fn c_fe_state_title_on_widget_created_matches() {
        check(
            ee::s02::f_00194988__generated,
            c_fe_state_title_on_widget_created,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x1949_88, 0);
                alloc_ready(c);
                let item = DATA + 0x100;
                clear(c, item, 0x100);
                put_words(c, 0x004a_17d0, &[0x5449_544c, 0]);
                let hash = load(c, 0x0031_7670, ee::s10::f_00317670, &[0x004a_17d0]) as u32;
                c.bus
                    .write32(item + 0x38, if seed % 2 == 0 { hash } else { hash ^ 1 });
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = sx(item);
            },
        );
    }


    #[test]
    fn cfe_state_title_on_widget_notify_matches() {
        check(ssx3_ee::s02::f_00194a48__generated, cfe_state_title_on_widget_notify, SEEDS_3, &|c, seed| {
            let _ = machine(c, seed, 0x194a48, 0);
            c.g[A2][0] = (seed % 13) as u64;
        });
    }


    // ------------------------------------------------------- front end widgets

    #[test]
    fn fe_state_main_menu_on_widget_notify_matches() {
        check(ee::s02::f_001952e8__generated, fe_state_main_menu_on_widget_notify, SEEDS_4, &|c, seed| {
            machine(c, seed, 0x0019_52e8, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A2][0] = match seed % 4 {
                0 => 6,
                1 => 9,
                2 => 5,
                _ => 12,
            };
            w(c, DATA + 0x48, if seed % 2 == 0 { 0 } else { DATA + 0x100 });
        });
    }


    #[test]
    fn c_fe_state_title_on_widget_action_matches() {
        check(
            ssx3_ee::s02::f_001949e0__generated,
            c_fe_state_title_on_widget_action,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1949e0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    5
                } else {
                    1 + seed as u64 % 7
                };
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x118, r.next_u32());
            },
        );
    }


    #[test]
    fn c_fe_state_title_init_matches() {
        check(
            ee::s02::f_00194778__generated,
            c_fe_state_title_init,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1947, 3);
                w(
                    c,
                    DATA + 0x10,
                    if seed % 3 == 0 { 0 } else { DATA + 0x1000 },
                );
                fill(c, &mut r, DATA + 0x40, 4);
                c.g[A0][0] = sx(DATA);
            },
        );
    }


    #[test]
    fn cfe_state_title_transition_in_matches() {
        check(
            ee::s02::f_001947f8__generated,
            cfe_state_title_transition_in,
            6,
            &|c, seed| {
                let _ = base_2(c, seed, 0x0019_47f8, 3);
                c.g[A0][0] = OBJ as u64;
                w(c, GAME_GP.wrapping_sub(0x1930), match seed % 4 {
                    0 => 0,
                    1 => 1,
                    _ => 2 + (seed % 3),
                });
                w(c, OBJ + 0x40, if seed % 3 == 0 { 0 } else { OBJ2 });
                w(c, OBJ2 + 8, OBJ3);
                c.bus.write16(OBJ3 + 0x38, (seed as u16) & !3);
                w(c, OBJ3 + 0x3c, FAKE);
                c.bus.write16(OBJ3 + 0x48, (seed as u16) & !3);
                w(c, OBJ3 + 0x4c, FAKE + 0x10);
            },
        );
    }


    /// Scrambled registers and a game-like `gp` (no fake-call hooks: these
    /// tests only reach callees that are already ported).
    pub(crate) fn base_3(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x47_0133 ^ ((seed as u64) << 9) ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }


    /// Call a guest setup function and return `v0`.
    pub(crate) fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = RET as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, RET, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }


    /// `GetHashValue32` of the string at `site`.
    pub(crate) fn str_hash(c: &mut Cpu, site: u32) -> u64 {
        run(c, 0x0031_7670, ee::s10::f_00317670, &[site as u64])
    }


    /// The engine's size-class pool allocator (see `oc57`/`oc102`): short free
    /// lists behind `gp + 0x2a60..0x2a84`, enabled by the "pools ready" flag at
    /// `gp + 0xdd8`.
    pub(crate) fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base_3 = nodes + 0x100 * i as u32;
            for n in 0..4u32 {
                let node = base_3 + 0x40 * n;
                c.bus.write32(node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base_3);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }


    // ------------------------------------------------------------------- FE.

    #[test]
    fn cfe_state_main_menu_on_widget_created_matches() {
        check(ee::s02::f_00194d18__generated, cfe_state_main_menu_on_widget_created, 200, &|c, seed| {
            let mut r = base_3(c, seed, 0x19_4d18);
            let sites = [0x0046_0050u32, 0x0046_0060, 0x0046_0040, 0x0046_0070, 0x004a_17d8];
            // Distinct short strings at every hash key.
            for (i, site) in sites.iter().enumerate() {
                for k in 0..8u32 {
                    c.bus.write8(site + k, 0);
                }
                for k in 0..(i as u32 + 1) {
                    c.bus.write8(site + k, b'A' + i as u8 + k as u8);
                }
            }
            let pick = seed % 6;
            let id = if pick < 5 {
                str_hash(c, sites[pick as usize]) as u32
            } else {
                r.next_u32()
            };
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x1000);
            c.bus.write32(DATA + 0x1000 + 0x38, id);
        });
    }


    #[test]
    fn cfestatetitle_update_matches() {
        check_lohi(ee::s02::f_001948a8__generated, cfestatetitle_update, 12, &|c, seed| {
            let _ = base_4(c, seed, 0x1948a8, 2);
            let this = DATA;
            clear_2(c, this, 0x200);
            c.g[A0][0] = this as u64;
            w(
                c,
                this + 0x48,
                match seed % 3 {
                    0 => 0,
                    1 => 1800,
                    _ => 1801,
                },
            );
            w(c, GAME_GP.wrapping_sub(0x1724), 0);
            // two empty lists under the world
            let l0 = DATA + 0x1000;
            let l1 = DATA + 0x2000;
            clear_2(c, l0, 0x40);
            clear_2(c, l1, 0x40);
            w(c, WORLD + 0xb0, l0);
            w(c, WORLD + 0xb4, l1);
            w(c, l0, 0);
            w(c, l1, 0);
        });
    }


    #[test]
    fn cfestatemainmenu_update_matches() {
        check(
            ee::s02::f_00194dd0__generated,
            cfestatemainmenu_update,
            12,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x194dd0, 3);
                let this = DATA;
                clear_2(c, this, 0x400);
                c.g[A0][0] = this as u64;
                w(c, this + 0x10, DATA + 0x200);
                w(c, DATA + 0x200 + 0xc, DATA + 0x300);
                let vt = vtable(c, &mut r, &[0x160, 0x168]);
                w(c, DATA + 0x300 + 8, vt);
                w(c, this + 0x55, (seed % 3) as u32);
                w(c, this + 0x40, 0);
            },
        );
    }


    // ------------------------------------------------------- title screen timers

    /// A callee outside the translated code: answer null and return at once.
    /// The rate tests below only count updates (the widgets they look up are
    /// absent anyway), so the callee's results do not matter.
    fn no_callee(c: &mut Cpu, _target: u32) {
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
    }


    /// `cFEStateTitle_Update` at rate `hz` from a zero idle counter: true when
    /// the attract mode is entered within `n` updates.
    fn title_attracts_after(hz: u32, n: u32) -> bool {
        let mut c = crate::difftest::fresh();
        c.hooks.miss = no_callee;
        crate::tick::set_for_test(&mut c, hz);
        c.g[GP][0] = GAME_GP as u64;
        let this = DATA;
        clear_2(&mut c, this, 0x200);
        w(&mut c, this + 0x48, 0);
        let flag = GAME_GP.wrapping_sub(0x1724);
        w(&mut c, flag, 0);
        // the two (empty) world lists the idle check walks
        let l0 = DATA + 0x1000;
        let l1 = DATA + 0x2000;
        clear_2(&mut c, l0, 0x40);
        clear_2(&mut c, l1, 0x40);
        w(&mut c, WORLD + 0xb0, l0);
        w(&mut c, WORLD + 0xb4, l1);
        for i in 1..=n {
            c.g[A0][0] = this as u64;
            cfestatetitle_update(&mut c);
            assert_eq!(c.pc, RET, "update {i} did not return");
            if c.bus.read32(flag) & 4 != 0 {
                return true;
            }
        }
        false
    }


    /// `cFEStateTitle_TransitionIn` at rate `hz`, starting from the game's
    /// data initializer of `D_004A17C0` (300): true when the fade starts
    /// within `n` updates.
    fn title_fades_after(hz: u32, n: u32) -> bool {
        let mut c = crate::difftest::fresh();
        c.hooks.miss = no_callee;
        crate::tick::set_for_test(&mut c, hz);
        c.g[GP][0] = GAME_GP as u64;
        let this = DATA;
        clear_2(&mut c, this, 0x100);
        // no screen at +0x40: both widget lookups return null and are skipped
        w(&mut c, this + 0x40, 0);
        // The data initializer as `tick::on_vblank` writes it for this rate.
        let init = crate::tick::ticks_from_frames60(&mut c, TRANSIENT_WAIT_FRAMES);
        w(&mut c, GAME_GP.wrapping_sub(0x1930), init);
        for i in 1..=n {
            c.g[A0][0] = this as u64;
            cfe_state_title_transition_in(&mut c);
            assert_eq!(c.pc, RET, "transition {i} did not return");
            if c.g[V0][0] == 1 {
                return true;
            }
        }
        false
    }


    /// The title's 30 s idle timeout is a wall clock duration, not a count of
    /// updates: 1801 updates at 60 Hz (the original), 3601 at 120 Hz, 7201 at
    /// 240 Hz.
    #[test]
    fn cfestatetitle_idle_timeout_follows_the_rate() {
        assert!(!title_attracts_after(60, 1800));
        assert!(title_attracts_after(60, 1801));
        assert!(!title_attracts_after(120, 3600));
        assert!(title_attracts_after(120, 3601));
        assert!(!title_attracts_after(240, 7200));
        assert!(title_attracts_after(240, 7201));
    }


    /// The title's pre-fade wait (`D_004A17C0` = 300 frames = 5 s) is a wall
    /// clock duration too: `tick::on_vblank` rewrites its data initializer to
    /// 300 ticks at 60 Hz, 600 at 120 Hz, 1200 at 240 Hz (see
    /// `tick::FRAME_COUNTS`), and the countdown consumes exactly that many
    /// updates.
    #[test]
    fn cfe_state_title_transition_wait_is_the_patched_tick_count() {
        assert!(!title_fades_after(60, 299));
        assert!(title_fades_after(60, 300));
        assert!(!title_fades_after(120, 599));
        assert!(title_fades_after(120, 600));
        assert!(!title_fades_after(240, 1199));
        assert!(title_fades_after(240, 1200));
    }
}
