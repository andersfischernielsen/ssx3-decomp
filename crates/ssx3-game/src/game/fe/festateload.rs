//! `src/fe/festateload.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::Callee;
use crate::call::Flow;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::call::VSlot;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::dirtysock::tags2::arg;
pub(crate) use crate::game::fe::ovstatepause::p;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0018_6748, cfe_state_event_select_on_activate, "cFEStateEventSelect_OnActivate"),
    (0x0018_6778, cfe_state_event_select_shutdown, "cFEStateEventSelect_Shutdown"),
    (0x0018_6950, c_fe_state_event_select_on_widget_notify, "cFEStateEventSelect_OnWidgetNotify"),
    (0x0018_6978, cfe_state_event_select_handle_input, "cFEStateEventSelect_HandleInput"),
    (0x0018_6998, c_fe_state_event_select_post_input, "cFEStateEventSelect_PostInput"),
    (0x0018_69d8, cfe_state_event_select_update, "cFEStateEventSelect_Update"),
    (0x0018_6b18, cfe_state_mem_card_dtor, "cFEStateMemCard_dtor"),
];

// ---------------------------------------------------------------------------
// Callees (the generated crate's readable aliases, which dispatch through
// their override slot exactly as the generated `jal`/`call_indirect` does).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
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
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::C_WORLD_SOUND_UNLINK_EMITTERS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RIDER_METRIX_BANK_COMBO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_MANAGER_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EVENT_SELECT_ON_WIDGET_NOTIFY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_ALLOC_NAMED_ALIGNED;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_SEND_LOCK_CHAL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SHARED_LOGIN_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_VOIP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_SET_LISTENER;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_GET_BUFFER_SIZE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOAD_INDICATOR_CLEAR_BUSY;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_DELETE_ALL_PROCESSES;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_SPAWN_MISSIONS;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_READ;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_GET_WORLD_SECTION_INDEX;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_SHOW_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_DESTROY_MISSION_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_CREATE_COMPUTER_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_REWARD_INT_COUNT_SET_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::fe::ovstatepause::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_BEGIN_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_FORMAT_MONEY;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_ACCOUNT_NAME;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_PASSWORD;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_MOVE_DOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::EVENT_SELECT_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::ovstatepause::callee::VID_ENGINE_PLAYER_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::ovstatepause::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::ovstatepause::callee::MOUNTAIN_ROOM_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_UPDATE_2 as UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_ON_ACTIVATE_2 as UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::PLAY_FE_SOUND;
    pub(crate) use crate::game::fe::ovstatepause::callee::MEM_CARD_STATE_BASE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::ovstatepause::callee::OV_SCREEN_SESSION_DESTROY_INDICATORS;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_SHUTDOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_ASYNC_MANAGER_CANCEL;
    pub(crate) use crate::game::fe::ovstatepause::callee::SECTION_MAN_REQUEST_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRLEN;


    pub const SONG_INSTANCE_PAUSE: Callee = Callee { addr: 0x002b_2018, f: ee::s08::SongInstance_Pause };

    pub const SONG_INSTANCE_RESUME: Callee = Callee { addr: 0x002b_2070, f: ee::s08::SongInstance_Resume };

    pub const WORLD_TRIGGER_MANAGER_DTOR: Callee =
        Callee { addr: 0x002b_5988, f: ee::s08::WorldTriggerManager_dtor };

    pub const MENU_SYS_PAGE_VALIDATE_SELECTION: Callee =
        Callee { addr: 0x002c_aab0, f: ee::s09::MenuSysPage_ValidateSelection };

    pub const MENU_SYS_ITEM_ON_FOCUS: Callee =
        Callee { addr: 0x002c_a368, f: ee::s09::MenuSysItem_OnFocus };

    pub const WSCRIPT_PROCESS_JUMP_TO_TASK: Callee =
        Callee { addr: 0x0030_7ec0, f: ee::s10::WScriptProcess_JumpToTask };

    pub const WSCRIPT_PROCESS_GOTO_TASK: Callee =
        Callee { addr: 0x0030_7f58, f: ee::s10::WScriptProcess_GotoTask };

    pub const WSCRIPT_MAN_RESET_COMMAND_QUEUE: Callee =
        Callee { addr: 0x0030_c760, f: ee::s10::WScriptMan_ResetCommandQueue };

    pub const MODIFIER_BLOCK_FORWARD_V43: Callee =
        Callee { addr: 0x0035_3020, f: ee::s11::ModifierBlock_ForwardV43 };

    pub const MODIFIER_BLOCK_FORWARD_V42: Callee =
        Callee { addr: 0x0035_3098, f: ee::s11::ModifierBlock_ForwardV42 };

    pub const MODIFIER_BLOCK_HANDLE_EVENT: Callee =
        Callee { addr: 0x0035_2f40, f: ee::s11::ModifierBlock_HandleEvent };

    pub const WSCRIPT_CACHE_SET_DATA: Callee =
        Callee { addr: 0x003a_cc50, f: ee::s13::tWScriptCache_SetData };

    pub const CHAR_SELECT_UPDATE_RIDER_PREVIEW: Callee =
        Callee { addr: 0x0018_1ef0, f: ee::s02::cFEStateCharSelect_UpdateRiderPreview };

    pub const CHAR_SETUP_UPDATE_RIDER_PREVIEW: Callee =
        Callee { addr: 0x0018_2ec0, f: ee::s02::cFEStateCharSetup_UpdateRiderPreview };

    pub const BOLT_FILE_CACHE_INIT: Callee = Callee { addr: 0x0019_ce68, f: ee::s03::BoltFile_CacheInit };

    pub const RIDER_PREVIEW_APPLY_TRANSFORM: Callee =
        Callee { addr: 0x0019_f2d0, f: ee::s03::FEModule_RiderPreviewApplyTransform };

    pub const BE_LIBRARY_GET_INSTANCE: Callee =
        Callee { addr: 0x0014_dc80, f: ee::s01::BELibrary_GetInstance };

    pub const BE_LIBRARY_GET_SAVE_DATA_SIZE: Callee =
        Callee { addr: 0x0014_e048, f: ee::s01::BELibrary_GetSaveDataSize };

    pub const RIDER_METRIX_SET_SPIN_AMOUNT: Callee =
        Callee { addr: 0x0011_9898, f: ee::s00::RiderMetrix_SetSpinAmount };


    // Cross-shard calls the generator emitted as `call_indirect` (libc and
    // the frontend base states).
    pub const STRLEN_COPY: u32 = 0x0041_66f4;
 // strcpy (newlib)
    pub const WORLD_GET_OBJECT_SCRIPT_FUNC: u32 = 0x003a_6b78;

    pub const I_SIGNAL_SEMA: u32 = 0x0042_3dc0;

    pub const EA_MEM_FREE: u32 = 0x0031_7e98;

    pub const UI_STATE_V13: u32 = 0x0039_e510;

    pub const UI_BASE_ON_ACTIVATE: u32 = 0x0039_e4c0;

    pub const UI_BASE_ON_SHUTDOWN: u32 = 0x0039_e4a0;

    pub const EVENT_SELECT_ON_ACTIVATE: u32 = 0x0020_0ac0;

    pub const EVENT_SELECT_ON_SHUTDOWN: u32 = 0x0020_0af0;

    pub const EVENT_SELECT_ON_UPDATE: u32 = 0x0020_2828;

    pub const MEM_CARD_BASE_DTOR: u32 = 0x001d_5428;
}

/// `gp`-relative globals used here.
pub(crate) mod g {

    /// The music player's song-instance pointer (`D_004A52D4` is at +0x21e4).
    pub const WORLD_TRIGGER_MANAGER: i32 = 0x21e4;

    /// The save buffer and its end (`BESaveInt`).
    pub const SAVE_BUFFER: i32 = -0x1ecc;

    pub const SAVE_BUFFER_END: i32 = -0x1ec8;

    /// The `BEOptionInt` singleton head.
    pub const BE_OPTION_SINGLETON: i32 = -0x1ee4;

    /// The frontend MemCard base's back pointer.
    pub const FE_BASE_BACK: i32 = -0x1c38;

    /// The bolt-file cache singleton.
    pub const BOLT_FILE_CACHE: i32 = -0x1784;
}


// ---------------------------------------------------------------------------
// Forwards into another virtual method of `this`.

/// `void cFEStateEventSelect_HandleInput(this)` (0x00186978):
/// `EventSelect_HandleInput(this + 0x48)`.
pub fn cfe_state_event_select_handle_input(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48)),
        callee::EVENT_SELECT_HANDLE_INPUT,
        0x0018_6988,
    )
}


// ---------------------------------------------------------------------------
// Front-end state hooks.

/// `int cFEStateEventSelect_OnWidgetNotify(self, a1, notify)` (0x186950): run
/// the event-select notify on the state object at `self+0x48`, and report
/// `0x101` when it handled the notification, else 0.
pub fn c_fe_state_event_select_on_widget_notify(c: &mut Cpu) {
    event_select_notify(c, 0x48, 0x0018_6960)
}


/// `if (EventSelect_OnWidgetNotify(self + off, a1, notify)) return 0x101;`
pub(crate) fn event_select_notify(c: &mut Cpu, off: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(arg(c, 0).wrapping_add(off));
        call_ptr(c, callee::EVENT_SELECT_ON_WIDGET_NOTIFY.addr, ret)?;
        c.g[V1][0] = 0x101;
        frame.restore_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.g[V1][0] = 0;
        }
        c.g[V0][0] = c.g[V1][0];
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
pub(crate) fn at(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}


/// A `gp`-relative global address.
#[inline(always)]
pub(crate) fn glo(c: &Cpu, off: i32) -> u32 {
    abi::r32(c, GP).wrapping_add(off as u32)
}


/// `void cFEStateEventSelect_OnActivate(cFEStateEventSelect* self)`
/// (0x186748): the base state's activate, then the event-select widget's.
pub fn cfe_state_event_select_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_BASE_ON_ACTIVATE, 0x0018_675c)?;
        c.g[A0][0] = sx(at(c.g[S0][0] as u32, 0x48));
        call_ptr(c, callee::EVENT_SELECT_ON_ACTIVATE, 0x0018_6764)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void cFEStateEventSelect_Shutdown(cFEStateEventSelect* self)`
/// (0x186778): the base state's shutdown, then the widget's.
pub fn cfe_state_event_select_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_BASE_ON_SHUTDOWN, 0x0018_678c)?;
        c.g[A0][0] = sx(at(c.g[S0][0] as u32, 0x48));
        call_ptr(c, callee::EVENT_SELECT_ON_SHUTDOWN, 0x0018_6794)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void cFEStateEventSelect_Update(cFEStateEventSelect* self)` (0x1869d8):
/// the base state's update, then the event-select widget's.
pub fn cfe_state_event_select_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_V13, 0x0018_69ec)?;
        c.g[A0][0] = sx(at(c.g[S0][0] as u32, 0x48));
        call_ptr(c, callee::EVENT_SELECT_ON_UPDATE, 0x0018_69f4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void cFEStateMemCard_dtor(cFEStateMemCard* self)` (0x186b18): install the
/// base vtable at `+0x8`, clear the frontend's current-state back pointer
/// (`gp-0x1c38`) and run the base destructor.
pub fn cfe_state_mem_card_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0047_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(0x0047_0000u32.wrapping_sub(0x4a90));
        let obj = c.g[V1][0] as u32;
        let vtable = c.g[V0][0];
        Addr(at(obj, 8)).sw(c, vtable);
        Addr(glo(c, g::FE_BASE_BACK)).sw(c, 0);
        call_ptr(c, callee::MEM_CARD_BASE_DTOR, 0x0018_6b38)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void cFEStateEventSelect_PostInput(State* s, int a1)` (0x186998):
/// forward the input to the event-select panel, then to the UI state.
pub fn c_fe_state_event_select_post_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(crate::abi::r32(c, S0), 0x48));
        callee::EVENT_SELECT_POST_INPUT.call(c, 0x0018_69b8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::UI_STATE_POST_INPUT.call(c, 0x0018_69c4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateEventSelect_Init(self)` (0x186658): create and enter the
/// event-select screen.
pub fn cfe_state_event_select_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14b0));
        c.g[S0][0] = Addr(p(this as u32, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0018_667c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_6690)?;
        let missing = c.g[V0][0] == 0;
        c.sw(p(c.g[S1][0] as u32, 0x40), c.g[V0][0]);
        if !missing {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_66a8)?;
        }
        // (The generated code also carries a second, unreachable screen
        // creation block here: both branches of the creation above leave the
        // block, so it never runs.)
        c.cycles += 10;
        c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0x40)).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x48));
        c.sw(p(r32(c, GP), 0xffff_f4a4), 0);
        call_ptr(c, 0x0020_0700, 0x0018_66f4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
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
    use crate::difftest::put_words;
    pub(crate) use crate::game::fe::ovstatepause::tests::BE_LIBRARY;
    pub(crate) use crate::game::fe::ovstatepause::tests::WORLD;
    pub(crate) use crate::game::fe::ovstatepause::tests::globals_2;
    pub(crate) use crate::game::fe::fememcard::tests::vtable_at;


    #[test]
    fn cfe_state_event_select_handle_input_matches() {
        check(ee::s02::f_00186978__generated, cfe_state_event_select_handle_input, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x18_6978, 0);
            c.g[A0][0] = ptr(&mut r) as u64;
        });
    }


    // ------------------------------------------------------------- front end

    pub(crate) fn notify_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = sx(DATA + 0x4000);
        c.g[A1][0] = r.next_u32() as u64;
        c.g[A2][0] = match seed % 4 {
            0 => 6,
            1 => 9,
            2 => 8,
            _ => r.next_u32() as u64,
        };
    }


    #[test]
    fn c_fe_state_event_select_on_widget_notify_matches() {
        check(ssx3_ee::s02::f_00186950__generated, c_fe_state_event_select_on_widget_notify, 150, &|c, seed| {
            notify_setup(c, seed, 0x17)
        });
    }


    /// Where test objects live (all within [`DATA`]).
    pub(crate) const OBJ: u32 = DATA;


    /// The behaviour library singleton (a zeroed dummy, so
    /// `BELibrary_GetInstance` neither constructs one nor reads the disc) and
    /// a world whose race manager holds one rider (rider loops in the sound
    /// and UI code would otherwise walk a garbage count and run away).
    pub(crate) fn globals(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6_c000);
        let (world, ail, arr) = (DATA + 0x6_8000, DATA + 0x6_8100, DATA + 0x6_8200);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, ail);
        c.bus.write32(ail + 0xc, arr);
        c.bus.write32(arr + 8, 20);
        c.bus.write32(arr + 0x28, DATA + 0x6_8400);
        c.bus.write32(arr + 0x7c, 1);
    }


    /// Zero `n` words from the object page, so no callee reads a stale value.
    pub(crate) fn clear(c: &mut Cpu, base: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(base + 4 * i, 0);
        }
    }


    #[test]
    fn cfe_state_event_select_on_activate_matches() {
        check(s02::f_00186748__generated, cfe_state_event_select_on_activate, 120, &|c, seed| {
            machine(c, seed, 0x186748, 2);
            globals(c);
            clear(c, OBJ, 0x120);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }


    #[test]
    fn cfe_state_event_select_shutdown_matches() {
        check(s02::f_00186778__generated, cfe_state_event_select_shutdown, 120, &|c, seed| {
            machine(c, seed, 0x186778, 2);
            globals(c);
            clear(c, OBJ, 0x120);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }


    #[test]
    fn cfe_state_event_select_update_matches() {
        check(s02::f_001869d8__generated, cfe_state_event_select_update, 120, &|c, seed| {
            machine(c, seed, 0x1869d8, 2);
            globals(c);
            clear(c, OBJ, 0x120);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }


    #[test]
    fn cfe_state_mem_card_dtor_matches() {
        check(s02::f_00186b18__generated, cfe_state_mem_card_dtor, 120, &|c, seed| {
            machine(c, seed, 0x186b18, 1);
            globals(c);
            clear(c, OBJ, 0x100);
            c.g[A0][0] = sx(OBJ);
            c.g[A1][0] = sx(seed & 1);
            let _ = seed;
        });
    }


    #[test]
    fn c_fe_state_event_select_post_input_matches() {
        check(
            ssx3_ee::s02::f_00186998__generated,
            c_fe_state_event_select_post_input,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11, 1);
                globals_2(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3000);
                // The event panel's vtable (slot +0x88) resolves to the fake.
                vtable_at(c, &mut r, DATA + 0x3000, 0x88);
                // Mode 3 keeps `EventSelect_PostInput` on its short path.
                c.bus.write32(GAME_GP - 0xb54, 3);
            },
        );
    }
}
