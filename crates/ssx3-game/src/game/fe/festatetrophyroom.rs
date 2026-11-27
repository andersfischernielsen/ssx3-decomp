//! `src/fe/festatetrophyroom.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::call::Unwound;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::script::scriptcontroller::vslot;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::dirtysock::tagsunk::lb_at;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_3;
pub(crate) use crate::game::fe::feasyncfile::gp_16 as gp;
pub(crate) use crate::game::fe::feasyncfile::lh;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::festatenethelppopup::fe_state_screen_init;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamestate::p_2 as p_3;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_4368,
        c_fe_state_trophy_room_init,
        "cFEStateTrophyRoom_Init",
    ),
    (
        0x001d_4660,
        cfe_state_trophy_room_on_widget_notify,
        "cFEStateTrophyRoom_OnWidgetNotify",
    ),
    (
        0x001d_4698,
        cfe_state_trophy_room_on_widget_action,
        "cFEStateTrophyRoom_OnWidgetAction",
    ),
    (
        0x001d_4760,
        fe_state_trophy_room_update,
        "cFEStateTrophyRoom_Update",
    ),
    (
        0x001d_4890,
        c_fe_state_trophy_room_show_trophy_info,
        "cFEStateTrophyRoom_ShowTrophyInfo",
    ),
    (
        0x001d_4b20,
        cfe_state_trophy_room_release_trophy_images,
        "cFEStateTrophyRoom_ReleaseTrophyImages",
    ),
    (
        0x001d_4bc8,
        cfe_state_rewards_room_ctor,
        "cFEStateRewardsRoom_ctor",
    ),
    (
        0x001d_4c60,
        c_fe_state_rewards_room_init,
        "cFEStateRewardsRoom_Init",
    ),
    (
        0x001d_4ea8,
        fe_state_rewards_room_on_widget_action,
        "cFEStateRewardsRoom_OnWidgetAction",
    ),
    (
        0x001d_4f68,
        cfe_state_rewards_room_on_widget_notify,
        "cFEStateRewardsRoom_OnWidgetNotify",
    ),
    (
        0x001d_4f90,
        cfe_rewards_room_set_count_text,
        "cFEStateRewardsRoom_SetCountText",
    ),
    (0x001d_5038, fe_state_relogin_ctor, "cFEStateRelogin_ctor"),
    (0x001d_5078, c_fe_state_relogin_init, "cFEStateRelogin_Init"),
    (
        0x001d_50e0,
        cfe_state_relogin_on_widget_created,
        "cFEStateRelogin_OnWidgetCreated",
    ),
    (
        0x001d_5240,
        fe_state_relogin_on_child_event,
        "cFEStateRelogin_OnChildEvent",
    ),
    (0x001d_5280, cfe_relogin_v33, "cFEStateRelogin_v33"),
    (
        0x001d_5330,
        memcardstatebase_ctor_2,
        "MemCardStateBase_ctor_2",
    ),
    (
        0x001d_53b0,
        mem_card_state_base_ctor,
        "MemCardStateBase_ctor",
    ),
    (
        0x001d_5428,
        mem_card_state_base_dtor,
        "MemCardStateBase_dtor",
    ),
    (
        0x001d_5460,
        mem_card_state_base_on_widget_notify,
        "MemCardStateBase_OnWidgetNotify",
    ),
    (
        0x001d_58b8,
        mem_card_state_base_shutdown,
        "MemCardStateBase_Shutdown",
    ),
    (
        0x001d_5488,
        mem_card_state_base_install_callbacks,
        "MemCardStateBase_InstallCallbacks",
    ),
    (0x001d_5df0, mem_card_ui_begin_load, "MemCardUI_BeginLoad"),
    (
        0x001d_5f40,
        mem_card_ui_apply_loaded_data,
        "MemCardUI_ApplyLoadedData",
    ),
    (
        0x001d_43d0,
        cfe_state_trophy_room_on_activate,
        "cFEStateTrophyRoom_OnActivate",
    ),
    (
        0x001d_43f0,
        c_fe_state_trophy_room_on_widget_created,
        "cFEStateTrophyRoom_OnWidgetCreated",
    ),
    (
        0x001d_47a0,
        cfe_state_trophy_room_show_trophy_image,
        "cFEStateTrophyRoom_ShowTrophyImage",
    ),
    (
        0x001d_4918,
        cfe_state_trophy_room_request_trophy_images,
        "cFEStateTrophyRoom_RequestTrophyImages",
    ),
    (
        0x001d_4a20,
        cfe_state_trophy_room_load_trophy_textures,
        "cFEStateTrophyRoom_LoadTrophyTextures",
    ),
    (
        0x001d_4cd8,
        cfe_state_rewards_room_on_widget_created,
        "cFEStateRewardsRoom_OnWidgetCreated",
    ),
    (0x001d_5138, c_fe_state_relogin_v31, "cFEStateRelogin_v31"),
    (
        0x001d_59a0,
        mem_card_ui_write_save_data,
        "MemCardUI_WriteSaveData",
    ),
    (
        0x001d_5c28,
        mem_card_ui_read_save_chunk,
        "MemCardUI_ReadSaveChunk",
    ),
    (
        0x001d_5d78,
        mem_card_ui_write_replay_save_block,
        "MemCardUI_WriteReplaySaveBlock",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_STATE_DTOR_2 as UI_STATE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_IMAGE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR_2;

    pub const FE_STATE_ON_ACTIVATE: Callee = Callee {
        addr: 0x0018_6518,
        f: ee::s02::cFEState_OnActivate,
    };

    pub const REQUEST_LINE_UPDATE_SONG_NAMES: Callee = Callee {
        addr: 0x0019_7bc8,
        f: ee::s02::cFEStateRequestLine_UpdateSongNames,
    };

    pub const REQUEST_LINE_UPDATE_SONG_ICONS: Callee = Callee {
        addr: 0x0019_7ca0,
        f: ee::s02::cFEStateRequestLine_UpdateSongIcons,
    };

    pub const REQUEST_LINE_UPDATE_SCROLL_ARROWS: Callee = Callee {
        addr: 0x0019_7db8,
        f: ee::s02::cFEStateRequestLine_UpdateScrollArrows,
    };

    pub const REQUEST_LINE_UPDATE_SONG_COUNT: Callee = Callee {
        addr: 0x0019_8118,
        f: ee::s02::cFEStateRequestLine_UpdateSongCount,
    };

    pub const NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT: Callee = Callee {
        addr: 0x001a_9668,
        f: ee::s03::NetFEState_OnDisconnectedPopupEvent,
    };

    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    };

    pub const NET_FE_STATE_ON_ACTIVATE: Callee = Callee {
        addr: 0x001a_87d0,
        f: ee::s03::NetFEState_OnActivate,
    };

    pub const CHAT_ROOM_UPDATE_PLAYER_QUALITY: Callee = Callee {
        addr: 0x001b_3bf0,
        f: ee::s03::cFEStateChatRoom_UpdatePlayerQuality,
    };

    pub const FE_STATE_LOGIN_CTOR: Callee = Callee {
        addr: 0x001b_ee30,
        f: ee::s03::cFEStateLogin_ctor,
    };

    pub const FE_STATE_LOGIN_ON_CHILD_EVENT: Callee = Callee {
        addr: 0x001b_f6c0,
        f: ee::s03::cFEStateLogin_OnChildEvent,
    };

    pub const NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT: Callee = Callee {
        addr: 0x001c_1b88,
        f: ee::s03::cFEStateNetQuickPlayPopup_OnOptionsPopupEvent,
    };

    pub const FE_STATE_STATS_REFRESH_RESULTS: Callee = Callee {
        addr: 0x001c_3310,
        f: ee::s03::cFEStateStats_RefreshResults,
    };

    pub const POPUP_CONFIRM_GET_TEXT_BOX_SIZE: Callee = Callee {
        addr: 0x001c_9210,
        f: ee::s03::cFEPopupConfirm_GetTextBoxSize,
    };

    pub const PREVIEW_REWARD_POLL_IMAGE_LOAD: Callee = Callee {
        addr: 0x001d_2198,
        f: ee::s04::cFEStatePreviewReward_PollImageLoad,
    };

    pub const MOUNTAIN_ROOM_UPDATE_EVENT_ICONS: Callee = Callee {
        addr: 0x001d_3340,
        f: ee::s04::cFEStateMountainRoom_UpdateEventIcons,
    };

    pub const TROPHY_ROOM_LOAD_TROPHY_TEXTURES: Callee = Callee {
        addr: 0x001d_4a20,
        f: ee::s04::cFEStateTrophyRoom_LoadTrophyTextures,
    };

    pub const SECTION_MAN_FIND_LOGICAL_SECTION: Callee = Callee {
        addr: 0x0022_e0e0,
        f: ee::s05::SectionMan_FindLogicalSection,
    };

    pub const APP_MODULE_TRY_EXIT: Callee = Callee {
        addr: 0x0023_1cf0,
        f: ee::s06::AppModule_TryExit,
    };

    pub const S_LOGIN_UPDATE: Callee = Callee {
        addr: 0x003f_8048,
        f: ee::s14::SLogin_Update,
    };

    pub const USB_KB_DEFAULT_CONNECT_CB: Callee = Callee {
        addr: 0x003f_6358,
        f: ee::s14::UsbKb_DefaultConnectCb,
    };

    pub const WORLD_STATE_STREAM_RESET: Callee = Callee {
        addr: 0x0026_d628,
        f: ee::s07::WorldStateStream_Reset,
    };

    pub const REPLAY_STREAM_CTOR: Callee = Callee {
        addr: 0x0026_e8e0,
        f: ee::s07::ReplayStream_ctor,
    };

    pub const REPLAY_FIND_NEXT_SNAPSHOT: Callee = Callee {
        addr: 0x0026_e468,
        f: ee::s07::Replay_FindNextSnapshot,
    };

    pub const REPLAY_FIND_PREV_SNAPSHOT: Callee = Callee {
        addr: 0x0026_e490,
        f: ee::s07::Replay_FindPrevSnapshot,
    };

    pub const REPLAY_SNAP_TO_SNAPSHOT: Callee = Callee {
        addr: 0x0026_e4b8,
        f: ee::s07::Replay_SnapToSnapshot,
    };

    pub const REPLAY_SET_STATE_2: Callee = Callee {
        addr: 0x0026_f4a0,
        f: ee::s07::Replay_SetState,
    };

    pub const SCRIPT_LIST_ADD_MOVIE: Callee = Callee {
        addr: 0x0027_6048,
        f: ee::s07::ScriptList_AddMovie,
    };

    pub const SCRIPT_LIST_POLL_LOADING: Callee = Callee {
        addr: 0x0027_7778,
        f: ee::s07::ScriptList_PollLoading,
    };

    pub const SCRIPT_CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::ScriptCmdQueue_At,
    };

    pub const SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS: Callee = Callee {
        addr: 0x0027_8cd8,
        f: ee::s07::ScriptEngine_UnloadWorldScripts,
    };

    pub const SCRIPT_ENGINE_LIST_STOP: Callee = Callee {
        addr: 0x0027_90a0,
        f: ee::s07::ScriptEngine_ListStop,
    };

    pub const CROWD_AUDIO_INSTANCE_STOP_SOUND: Callee = Callee {
        addr: 0x002a_6848,
        f: ee::s08::CrowdAudio_InstanceStopSound,
    };

    pub const SONG_INSTANCE_GET_DUCK_TO_LOOPS: Callee = Callee {
        addr: 0x002b_4868,
        f: ee::s08::SongInstance_GetDuckToLoops,
    };

    pub const WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO: Callee = Callee {
        addr: 0x002b_63d0,
        f: ee::s08::WorldTriggerManager_GetTriggerInfo,
    };

    pub const MEM_MAN_FREE: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::f_00317e98,
    };

    pub const MEM_GET_POOL_BLOCK_SIZE: Callee = Callee {
        addr: 0x0031_a130,
        f: ee::s10::MEM_GetPoolBlockSize,
    };

    pub const DL_HEAP_USABLE_SIZE: Callee = Callee {
        addr: 0x0031_fbb8,
        f: ee::s10::DLHeap_UsableSize,
    };

    pub const MEM_REPORT_POOL_STATS: Callee = Callee {
        addr: 0x0031_a200,
        f: ee::s10::MEM_ReportPoolStats,
    };

    pub const PS2_PAD_CREATE: Callee = Callee {
        addr: 0x0032_6d60,
        f: ee::s10::PS2Pad_Create,
    };

    pub const ANIM_OBJECT_V36: Callee = Callee {
        addr: 0x0034_1fe8,
        f: ee::s11::AnimObject_v36,
    };

    pub const DYNAMIC_EMITTER_CTOR: Callee = Callee {
        addr: 0x0037_0b60,
        f: ee::s12::DynamicEmitter_ctor,
    };

    pub const UI_STATE_CTOR: Callee = Callee {
        addr: 0x0039_e2a0,
        f: ee::s13::UIState_ctor,
    };

    pub const UI_STATE_UPDATE: Callee = Callee {
        addr: 0x0039_e510,
        f: ee::s13::UIState_Update,
    };

    pub const UI_TEXT_SET_STRING: Callee = Callee {
        addr: 0x003a_0c50,
        f: ee::s13::UIText_SetString,
    };

    pub const UI_TEXT_SCROLL_CLEAR_LINES: Callee = Callee {
        addr: 0x003a_18c0,
        f: ee::s13::cUITextScroll_ClearLines,
    };

    pub const UI_TEXT_SCROLL_SET_WIDE_STRING: Callee = Callee {
        addr: 0x003a_1d30,
        f: ee::s13::cUITextScroll_SetWideString,
    };

    pub const TWSCRIPT_CACHE_RELOCATE: Callee = Callee {
        addr: 0x003a_ccd8,
        f: ee::s13::tWScriptCache_Relocate,
    };

    pub const EA_MUTEX_LOCK: Callee = Callee {
        addr: 0x003e_5700,
        f: ee::s14::EAMutex_Lock,
    };

    pub const EA_MUTEX_UNLOCK: Callee = Callee {
        addr: 0x003e_5760,
        f: ee::s14::EAMutex_Unlock,
    };

    pub const SCE_USBKB_SET_ARRANGEMENT: Callee = Callee {
        addr: 0x0040_c7c0,
        f: ee::s15::sceUsbKbSetArrangement,
    };

    pub const SCE_USBKB_SET_READ_MODE: Callee = Callee {
        addr: 0x0040_d430,
        f: ee::s15::sceUsbKbSetReadMode,
    };

    pub const MEMSET: Callee = Callee {
        addr: 0x0041_6210,
        f: ee::s15::memset,
    };

    pub const SPRINTF: Callee = Callee {
        addr: 0x0041_7828,
        f: ee::s15::sprintf,
    };
}

/// Cross-shard call targets (the generator emits `call_indirect` for them).
pub(crate) mod table {
    pub(crate) use crate::game::ai::computer::table::STR_HASH;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::BENEW_PLAYER_INT_GET_CHAR_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_INTERFACE_FV;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_TEXT_FROM_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_STATE_CTOR;
}

/// `void MemCardStateBase_dtor(MemCardStateBase* self, int flags)` (0x1d5428):
/// install the two intermediate vtables, clear the global and run the state
/// destructor.
pub fn mem_card_state_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0047_0000;
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3680));
        c.g[V1][0] = 0x0047_0000;
        let a2 = c.g[A2][0];
        let v0 = c.g[V0][0];
        Addr(a2 as u32).off(8).sw(c, v0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        Addr((c.g[GP][0] as u32).wrapping_sub(0x10c8)).sw(c, 0);
        let v1 = c.g[V1][0];
        Addr(a2 as u32).off(8).sw(c, v1);
        callee::UI_STATE_DTOR.call(c, 0x001d_5454)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Helpers.

/// The low word of a register.
#[inline(always)]
pub(crate) fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// `lw`/`lh`/`sw` into a named register, and `lwc1`/`swc1`. Methods on the CPU
/// so that the address may read the registers (two-phase borrows).
pub(crate) trait Ops {
    /// `lw reg, [a]`, leaving the sign-extended value in the register.
    fn lw_r_2(&mut self, reg: usize, a: u32) -> u64;
    /// `lh reg, [a]`.
    fn lh_r(&mut self, reg: usize, a: u32) -> u64;
    /// `sw reg, [a]`.
    fn sw_r_2(&mut self, a: u32, reg: usize);
    /// `reg += n` (`addiu`, sign-extended result).
    fn inc_r_2(&mut self, reg: usize, n: u32);
    /// `lwc1 f{n}, [a]`.
    fn ld_f(&mut self, n: usize, a: u32);
    /// `swc1 f{n}, [a]`.
    fn st_f(&mut self, n: usize, a: u32);
}

impl Ops for Cpu {
    #[inline(always)]
    fn lw_r_2(&mut self, reg: usize, a: u32) -> u64 {
        let v = self.lw(a);
        self.g[reg][0] = v;
        v
    }
    #[inline(always)]
    fn lh_r(&mut self, reg: usize, a: u32) -> u64 {
        let v = self.lh(a);
        self.g[reg][0] = v;
        v
    }
    #[inline(always)]
    fn sw_r_2(&mut self, a: u32, reg: usize) {
        let v = self.g[reg][0];
        self.sw(a, v);
    }
    #[inline(always)]
    fn inc_r_2(&mut self, reg: usize, n: u32) {
        let v = r(self, reg).wrapping_add(n);
        self.g[reg][0] = sx(v);
    }
    #[inline(always)]
    fn ld_f(&mut self, n: usize, a: u32) {
        self.lwc1(n, a);
    }
    #[inline(always)]
    fn st_f(&mut self, n: usize, a: u32) {
        self.swc1(n, a);
    }
}

/// The `+0x8`/`+0xC` pair a front-end state constructor writes: the class
/// vtable and the state id.
#[inline(always)]
pub(crate) fn init_state(c: &mut Cpu, this: u64, vtable: u32, id: u32) {
    c.g[V0][0] = id as u64;
    c.g[V1][0] = sx(vtable);
    c.sw_r_2((this as u32).wrapping_add(0xc), V0);
    c.sw_r_2((this as u32).wrapping_add(0x8), V1);
    c.g[V0][0] = this;
}

/// `void cFEStateTrophyRoom_Update(cFEStateTrophyRoom* self)` (0x1D4760):
/// load the trophy textures while any are pending (count > 0), then run the
/// UI state update.
pub fn fe_state_trophy_room_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let pending = c.lw_r_2(V0, r(c, S0).wrapping_add(0x5c));
        if (pending as i64) > 0 {
            c.cycles += 2;
            callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES.call(c, 0x001d_4784)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::UI_STATE_UPDATE.call(c, 0x001d_478c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateRelogin* cFEStateRelogin_ctor(cFEStateRelogin*)` (0x1D5038): the
/// login-state constructor, vtable `0x468050` and state id 0x34.
pub fn fe_state_relogin_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::FE_STATE_LOGIN_CTOR.call(c, 0x001d_504c)?;
        init_state(c, this, 0x0046_8050, 0x34);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateRelogin_OnChildEvent(self, child, event)` (0x1D5240): the
/// disconnect popup (id 0x102) handles the event itself, every other child
/// goes to the login state.
pub fn fe_state_relogin_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x102;
        frame.save_ra(c, 0);
        let event = Addr(r(c, A1).wrapping_add(0x18)).lw(c);
        c.g[V1][0] = event;
        if event == c.g[V0][0] {
            c.cycles += 4;
            callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT.call(c, 0x001d_5260)?;
        } else {
            c.cycles += 3;
            callee::FE_STATE_LOGIN_ON_CHILD_EVENT.call(c, 0x001d_5270)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRelogin_OnWidgetCreated(cFEStateRelogin* s, UIWidget* w)`
/// (0x1d50e0): when the created widget is the retry button, give it the
/// "please wait" label.
pub fn cfe_state_relogin_on_widget_created(c: &mut Cpu) {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    c.g[A0][0] = sx(0x0046_7790);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A1][0];
    frame.save(c, S0, 0x20);
    c.g[S0][0] = Addr(p(c, S1, 0x38)).lw(c);
    if call_ptr(c, table::STR_HASH, 0x001d_5104).is_err() {
        return;
    }
    let same = c.g[S0][0] == c.g[V0][0];
    frame.restore(c, S0, 0x20);
    if same {
        c.cycles += 7;
        c.g[A0][0] = sx(0x0046_1cc0);
        if call_ptr(c, table::STR_HASH, 0x001d_5118).is_err() {
            return;
        }
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        if call_ptr(c, 0x003a_1000, 0x001d_5124).is_err() {
            return;
        }
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// The list-widget handlers that answer 0x100 (handled) for the scroll keys
/// 8 and 9 and 0x101 (ignored) otherwise, keeping the original's compare
/// chain (`key < 10`, then `key < 8`) and its cycle charges.
pub(crate) fn handle_list_key(c: &mut Cpu) {
    c.cycles += 3;
    let key = abi::arg64(c, 2);
    c.g[V1][0] = (key < 0xa) as u64;
    c.g[V0][0] = 0x101;
    if c.g[V1][0] != 0 {
        c.cycles += 4;
        c.g[A0][0] = (key < 0x8) as u64;
        c.g[V1][0] = 0x101;
        c.g[V0][0] = if key < 0x8 { 0x101 } else { 0x100 };
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int cFEStateRewardsRoom_OnWidgetNotify(State*, void*, int key)`
/// (0x001d4f68).
pub fn cfe_state_rewards_room_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// `int MemCardStateBase_OnWidgetNotify(State*, void*, int key)`
/// (0x001d5460).
pub fn mem_card_state_base_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// The handlers that accept the keys 6, 8 and 9 (0x100) and ignore the rest
/// (0x101). The original tests 6, then `< 6`, `< 10`, `< 8`, so the cycle
/// charge depends on where the key leaves the chain.
pub(crate) fn handle_key_6_8_9(c: &mut Cpu) {
    let key = abi::arg64(c, 2);
    c.cycles += 3;
    c.g[V0][0] = 6;
    let is6 = key == 6;
    c.g[V0][0] = (key < 6) as u64;
    let accept = if is6 {
        true
    } else {
        c.cycles += 2;
        let below6 = c.g[V0][0] != 0;
        c.g[V0][0] = (key < 0xa) as u64;
        if below6 {
            false
        } else {
            c.cycles += 2;
            let above9 = c.g[V0][0] == 0;
            c.g[V0][0] = (key < 8) as u64;
            if above9 {
                false
            } else {
                c.cycles += 2;
                c.g[V0][0] == 0
            }
        }
    };
    c.cycles += 2;
    c.g[V0][0] = if accept { 0x100 } else { 0x101 };
    abi::ret(c);
}

/// `int cFEStateTrophyRoom_OnWidgetNotify(State*, void*, int key)`
/// (0x001d4660).
pub fn cfe_state_trophy_room_on_widget_notify(c: &mut Cpu) {
    handle_key_6_8_9(c);
}

/// `cFEStateTrophyRoom_Init(cFEStateTrophyRoom* self)` (0x1d4368).
pub fn c_fe_state_trophy_room_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0046_76c8, [0x001d_438c, 0x001d_43a0, 0x001d_43b8])
    })
}

/// `cFEStateRelogin_Init(cFEStateRelogin* self)` (0x1d5078).
pub fn c_fe_state_relogin_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0046_7778, [0x001d_509c, 0x001d_50b0, 0x001d_50c8])
    })
}

/// `lhu off(reg)`.
#[inline]
pub(crate) fn lhu_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lhu(c)
}

/// `sh off(reg)`.
#[inline]
pub(crate) fn sh_r(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    let a = low(c, reg).wrapping_add(off);
    c.sh(a, v);
}

/// `func_001D53B0` (0x1d53b0): the memory-card state constructor. Runs the UI
/// state constructor (`0x39e2a0`), installs vtables `0x474e08`/`0x46c980`,
/// clears the callback/word fields at `+0x48`..`+0x98` and calls
/// `MemCardStateBase_InstallCallbacks`. Signature: `void ctor(this, a, b)`.
pub fn mem_card_state_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001d_53cc);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x001d_53cc)?;
        c.g[V1][0] = 0x47_0000;
        sw_r(c, S0, 0x98, 0);
        c.g[V1][0] = sx(low(c, V1).wrapping_add(0x4e08));
        sw_r(c, S0, 0x48, 0);
        sw_r(c, S0, 0x8, c.g[V1][0]);
        c.g[A2][0] = 0x4a_0000;
        sw_r(c, S0, 0x4c, 0);
        c.g[V0][0] = 0x47_0000;
        sw_r(c, S0, 0x54, 0);
        c.g[V0][0] = sx(low(c, V0).wrapping_sub(0x3680));
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = lhu_r(c, A2, 0x1390);
        sw_r(c, S0, 0x8, c.g[V0][0]);
        c.g[RA][0] = sx(0x001d_540c);
        sh_r(c, S0, 0x58, c.g[V1][0]);
        callee::MEMCARD_INSTALL_CALLBACKS.call(c, 0x001d_540c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRewardsRoom_Init(self)` (0x1d4c60): create the rewards-room
/// screen by name hash, play its first frame and tell the audio system the FE
/// state is 6.
pub fn c_fe_state_rewards_room_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x7738));
        c.g[S1][0] = lw_r(c, S0, 0x10);
        call_ptr(c, callee::STR_HASH, 0x001d_4c84)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME, 0x001d_4c98)?;
        let failed = c.g[V0][0] == 0;
        sw_r(c, S0, 0x40, c.g[V0][0]);
        if !failed {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, callee::C_UI_SCREEN_PLAY_FRAME, 0x001d_4cb0)?;
        }
        c.cycles += 10;
        call_ptr(c, callee::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x001d_4cb8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x6;
        call_ptr(c, callee::FUNC_0028F140, 0x001d_4cc4)?;
        c.lq(16, sp_2(c).wrapping_add(0x20));
        c.lq(17, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end states and screens.

/// `MemCardStateBase_ctor_2(self, index, a2, a3)` (0x1d5330): the second
/// constructor entry: install the vtable, clear the state fields, copy the
/// memory-card slot count and install the callbacks.
pub fn memcardstatebase_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        callee::UI_STATE_CTOR_2.call(c, 0x001d_5354)?;
        c.g[V1][0] = 0x470000;
        c.sw(p_3(r32(c, S0), 0x98), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3680));
        c.sw(p_3(r32(c, S0), 0x48), 0);
        c.sw(p_3(r32(c, S0), 8), c.g[V1][0]);
        c.g[A2][0] = 0x4a0000;
        c.sw(p_3(r32(c, S0), 0x4c), 0);
        c.g[V0][0] = 0x470000;
        c.sw(p_3(r32(c, S0), 0x54), 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3680));
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = Addr(p_3(c.g[A2][0] as u32, 0x1390)).lhu(c);
        c.sw(p_3(r32(c, S0), 8), c.g[V0][0]);
        c.sh(p_3(r32(c, S0), 0x58), c.g[V1][0]);
        callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS.call(c, 0x001d_5394)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 17, 0x20);
        frame.restore(c, 16, 0x30);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateTrophyRoom_ShowTrophyInfo(cFEStateTrophyRoom* self, int
/// which)` (0x1d4890): select the trophy list entry (`self+0x4c` -> list,
/// `+8` -> item) and refresh the event stat text.
pub fn c_fe_state_trophy_room_show_trophy_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x20));
        c.sq(17, at(sp0, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        let this = c.g[S0][0];
        c.g[A2][0] = lw_at(c, this, 0x4c);
        let list = c.g[A2][0];
        let empty = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !empty {
            c.cycles += 2;
            let which = c.g[S1][0] != 0;
            c.g[V0][0] = lw_at(c, list, 8);
            let item = c.g[V0][0];
            if which {
                c.cycles += 12;
                c.g[A1][0] = 1;
                c.g[A0][0] = lh_at(c, item, 0x48);
                c.g[V1][0] = lw_at(c, item, 0x4c);
                let func = c.g[V1][0] as u32;
                c.g[A0][0] = sx(at(list as u32, c.g[A0][0] as u32));
                call_ptr(c, func, 0x001d_48e8)?;
                c.g[T0][0] = lw_at(c, this, 0x58);
                c.g[T1][0] = sx(at(c.g[S1][0] as u32, 0xffff_ffff));
                c.g[A0][0] = lw_at(c, this, 0x4c);
                c.g[A1][0] = lb_at(c, this, 0x44);
                c.g[A2][0] = lw_at(c, this, 0x50);
                c.g[A3][0] = lw_at(c, this, 0x54);
                callee::UI_TEXT_SET_EVENT_STAT.call(c, 0x001d_4904)?;
                c.cycles += 1;
                c.lq(16, at(sp0, 0x20));
            } else {
                c.cycles += 7;
                c.g[A1][0] = 0;
                c.g[A0][0] = lh_at(c, item, 0x48);
                c.g[V1][0] = lw_at(c, item, 0x4c);
                let func = c.g[V1][0] as u32;
                c.g[A0][0] = sx(at(list as u32, c.g[A0][0] as u32));
                call_ptr(c, func, 0x001d_48cc)?;
                c.lq(16, at(sp0, 0x20));
            }
        } else {
            c.cycles += 1;
            c.lq(16, at(sp0, 0x20));
        }
        c.cycles += 4;
        c.lq(17, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `cFEStateRewardsRoom* cFEStateRewardsRoom_ctor(self, idx, char, a3)`
/// (0x1d4bc8): build the rewards-room state, remember the player's character
/// id and return the object.
pub fn cfe_state_rewards_room_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = sx((c.g[A2][0] as u32) << 24);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, table_callee::FE_STATE_CTOR, 0x001d_4bf0)?;
        c.g[V0][0] = 0x2e;
        c.g[V1][0] = 0x470000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6a18));
        Addr(c.g[S0][0] as u32).off(0xc).sw(c, c.g[V0][0]);
        Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S1][0];
        Addr(c.g[S0][0] as u32).off(0x44).sb(c, c.g[S1][0]);
        c.g[V0][0] = lw_2(c, S0, 0x10);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        call_ptr(c, table_callee::BE_GET_TEXT_FROM_ID, 0x001d_4c18)?;
        Addr(c.g[S0][0] as u32).off(0x4c).sw(c, c.g[S2][0]);
        Addr(c.g[S0][0] as u32).off(0x15).sb(c, c.g[V0][0]);
        call_ptr(c, table_callee::BE_GET, 0x001d_4c24)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, table_callee::BE_GET_INTERFACE_FV, 0x001d_4c30)?;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x44).lb(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, table_callee::BENEW_PLAYER_INT_GET_CHAR_ID, 0x001d_4c3c)?;
        Addr(c.g[S0][0] as u32).off(0x48).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRewardsRoom_SetCountText(self, index, text)` (0x1d4f90):
/// format the collected/total count into a buffer and set it on the label.
pub fn cfe_rewards_room_set_count_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S1, 0x140);
        frame.save(c, S3, 0x120);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S4, 0x110);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x150);
        frame.save(c, S2, 0x130);
        frame.save_ra(c, 0x100);
        c.g[S4][0] = c.g[A2][0];
        call_ptr(c, 0x0014_dc80, 0x001d4fbc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x001d4fc8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0015_7210, 0x001d4fd8)?;
        c.g[A1][0] = Addr(p_4(r32(c, S1), 0x44)).lb(c);
        c.g[S2][0] = c.g[V0][0];
        c.g[A2][0] = Addr(p_4(r32(c, S1), 0x48)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S3][0];
        call_ptr(c, 0x0015_7518, 0x001d4ff0)?;
        c.g[A1][0] = sx(0x004a_2020);
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, 0x0041_7828, 0x001d5008)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x003a_0c50, 0x001d5014)?;
        frame.restore(c, S0, 0x150);
        frame.restore(c, S1, 0x140);
        frame.restore(c, S2, 0x130);
        frame.restore(c, S3, 0x120);
        frame.restore(c, S4, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateTrophyRoom_ReleaseTrophyImages(self)` (0x1d4b20): for the
/// five trophy images, unlink the image object and free it.
pub fn cfe_state_trophy_room_release_trophy_images(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x60);
        c.cycles += 14;
        c.g[V1][0] = lw(c, gp(c, -0x848));
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = 0;
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx(at_3(c.g[S2][0], 0x74));
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, at_3(c.g[V1][0], 0x7c));
        c.g[S4][0] = sx(at_3(c.g[V0][0], 0x1a70));
        loop {
            {
                c.cycles += 3;
                c.g[A1][0] = lw(c, at_3(c.g[S0][0], 0xffff_ffec));
                let t = (c.g[A1][0] as i64) < 0;
                c.g[A2][0] = lw(c, gp(c, -0x854));
                if !t {
                    c.cycles += 5;
                    c.g[V0][0] = lw(c, at_3(c.g[A2][0], 0x10d8));
                    c.g[A0][0] = lh(c, at_3(c.g[V0][0], 0x190));
                    c.g[V1][0] = lw(c, at_3(c.g[V0][0], 0x194));
                    let target = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d_4b78);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001d_4b78)?;
                }
            }
            {
                c.cycles += 4;
                c.g[S1][0] = sx(0xffff_ffff);
                c.g[A2][0] = lw(c, c.g[S0][0] as u32);
                let t = (c.g[A2][0] as i64) < 0;
                Addr(at_3(c.g[S0][0], 0xffff_ffec)).sw(c, c.g[S1][0]);
                if !t {
                    c.cycles += 3;
                    c.g[A1][0] = lw(c, at_3(c.g[S2][0], 0x50));
                    c.g[RA][0] = sx(0x001d_4b94);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x0019_dc20, 0x001d_4b94)?;
                }
            }
            c.cycles += 5;
            Addr(c.g[S0][0] as u32).sw(c, c.g[S1][0]);
            c.g[S3][0] = sx(at_3(c.g[S3][0], 1));
            c.g[V0][0] = ((c.g[S3][0] as i64) < 5i64) as u64;
            let t = c.g[V0][0] != 0;
            c.g[S0][0] = sx(at_3(c.g[S0][0], 4));
            if !t {
                c.cycles += 8;
                frame.restore(c, S0, 0x50);
                frame.restore(c, S1, 0x40);
                frame.restore(c, S2, 0x30);
                frame.restore(c, S3, 0x20);
                frame.restore(c, S4, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x001d_4b58);
            }
        }
    })
}

/// `void cFEStateRelogin_v33(self, item)` (0x1d5280): widget event 0xf/0x14
/// tears down the relogin UI, marks the current UI state (0x80) and enters the
/// "relogin failed" state; every other event goes to the base handler.
pub fn cfe_relogin_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let msg = c.g[A2][0];
        let big = if msg == 0xf {
            true
        } else {
            c.cycles += 3;
            msg == 0x14
        };
        if !big {
            c.cycles += 3;
            c.g[A0][0] = this;
            call_ptr(c, 0x001a_97b8, 0x001d_5320)?;
        } else {
            c.cycles += 29;
            c.g[A0][0] = this;
            call_ptr(c, 0x001a_97b8, 0x001d_52ac)?;
            c.g[A0][0] = Addr(p_5(c.g[S0][0] as u32, 0x10)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_f698, 0x001d_52b8)?;
            c.g[A0][0] = Addr(p_5(c.g[S0][0] as u32, 0x10)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_fa88, 0x001d_52c4)?;
            c.g[V1][0] = Addr(p_5(c.g[V0][0] as u32, 0x1c)).lw(c);
            c.g[V1][0] = c.g[V1][0] | 0x80;
            c.sw(p_5(c.g[V0][0] as u32, 0x1c), c.g[V1][0]);
            call_ptr(c, 0x0025_91b8, 0x001d_52d4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0026_36f0, 0x001d_52e0)?;
            c.g[A1][0] = 0x460000;
            c.g[A3][0] = 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xbf0));
            c.g[A0][0] = 0x91c;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x001d_52f8)?;
            c.g[A1][0] = Addr(p_5(c.g[S0][0] as u32, 0x10)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x001b_3f78, 0x001d_5304)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x001a_8a40, 0x001d_5310)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateRewardsRoom_OnWidgetAction` (0x001d4ea8).
//
// `int cFEStateRewardsRoom::OnWidgetAction(self, widget, action)`: route the
// rewards-room buttons.

pub fn fe_state_rewards_room_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1d4f5c: {
            'b_1d4f58: {
                c.cycles += 6;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.g[V1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                let t = c.g[V1][0] == 0u64;
                c.g[S0][0] = c.g[A0][0];
                if t {
                    break 'b_1d4f58;
                } else {
                    'b_1d4f24: {
                        'b_1d4ee8: {
                            c.cycles += 3;
                            c.g[V0][0] = 0x5;
                            let t = c.g[A2][0] == c.g[V0][0];
                            c.g[V0][0] = (c.g[A2][0] < 0x6u64) as u64;
                            if t {
                                break 'b_1d4ee8;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != 0u64 {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                    break 'b_1d4f5c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x6;
                                    if c.g[A2][0] == c.g[V0][0] {
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                        break 'b_1d4f24;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        if t {
                                            break 'b_1d4f5c;
                                        } else {
                                            break 'b_1d4ee8;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 15;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                        c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = vslot(c, c.g[V1][0], 0x20, V0);
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001d4f0c);
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001d4f0c)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001d4f1c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039f400, 0x001d4f1c)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_1d4f5c;
                        } else {
                            break 'b_1d4f24;
                        }
                    }
                    c.cycles += 10;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = vslot(c, c.g[V1][0], 0x28, V0);
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001d4f44);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001d4f44)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        break 'b_1d4f58;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = sx(0x001d4f58);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039f400, 0x001d4f58)?;
                        break 'b_1d4f58;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1d4f5c;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `cFEStateTrophyRoom_OnWidgetAction(this, item, msg)` (0x1d4698): show a
/// trophy's image and info, or switch the trophy room into the item's view.
pub fn cfe_state_trophy_room_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let this = abi::arg(c, 0);
        let item = abi::arg(c, 1);
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = abi::arg64(c, 1);
        frame.save_ra(c, 0);
        let no_item = c.g[S0][0] == 0;
        c.g[S1][0] = abi::arg64(c, 0);

        // The original's exits differ: 'b_1d4748 adds one cycle before the
        // epilogue, 'b_1d474c does not.
        let mut extra = true;
        if no_item {
            // straight to 'b_1d4748
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let is_one = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = 6;
            if is_one {
                // case 1: set the trophy and refresh the panel ('b_1d4748).
                c.cycles += 8;
                c.g[V0][0] = Addr(item).off(0x18).lw(c);
                c.g[A0][0] = c.g[S1][0];
                Addr(this).off(0x9c).sw(c, c.g[V0][0]);
                c.g[A1][0] = Addr(item).off(0x18).lw(c);
                callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_IMAGE.call(c, 0x001d_473c)?;
                c.g[A1][0] = Addr(item).off(0x18).lw(c);
                c.g[A0][0] = c.g[S1][0];
                callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_INFO.call(c, 0x001d_4748)?;
            } else {
                c.cycles += 2;
                if c.g[A2][0] != c.g[V0][0] {
                    extra = false;
                } else {
                    // case 6: ask the room's menu to show the item.
                    c.cycles += 3;
                    c.g[A2][0] = Addr(this).off(0x48).lw(c);
                    let none = c.g[A2][0] == 0;
                    c.g[A1][0] = 0;
                    if !none {
                        c.cycles += 5;
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
                        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x001d_46e8)?;
                    }
                    c.cycles += 11;
                    c.g[V0][0] = Addr(this).off(0x10).lw(c);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = Addr(this).off(0x54).lw(c);
                    c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[V1][0] = Addr(c.g[A3][0] as u32).off(4).lw(c);
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x28).lh(c);
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x2c).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001d_470c)?;
                    let no_target = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[V0][0];
                    if no_target {
                        // 'b_1d4748
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = Addr(this).off(0x10).lw(c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        callee::UI_STATE_MGR_SWITCH_STATE.call(c, 0x001d_4720)?;
                        extra = false;
                    }
                }
            }
        }

        if extra {
            c.cycles += 1;
        }
        frame.restore(c, 16, 0x20);
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MemCardStateBase_Shutdown (0x00001d58b8).

/// `MemCardStateBase_Shutdown`: shut the memory-card state base down: close the dialogs and free the state.
pub fn mem_card_state_base_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        'b_1d591c: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            call_ptr(c, 0x0014dc80, 0x001d58d0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x5;
            call_ptr(c, 0x0014dd58, 0x001d58dc)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1bc));
            c.g[V0][0] = 0x1;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0x2;
            if t {
                break 'b_1d591c;
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x228));
                let t = c.g[V1][0] != c.g[V0][0];

                if t {
                    break 'b_1d591c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b4));
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_1d591c;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x00152728, 0x001d5910)?;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1d4), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1d0), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x228), 0u64);
                        break 'b_1d591c;
                    }
                }
            }
        }
        'b_1d5958: {
            c.cycles += 8;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00152700, 0x001d5924)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            call_ptr(c, 0x00227f80, 0x001d592c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
            if c.g[V0][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x228));
                break 'b_1d5958;
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002410a0, 0x001d5944)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0023d5e8, 0x001d594c)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0023c860, 0x001d5954)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x228));
                break 'b_1d5958;
            }
        }
        'b_1d5980: {
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            let t = c.g[V1][0] != c.g[V0][0];

            if t {
                break 'b_1d5980;
            } else {
                'b_1d5978: {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1d4));
                    let t = c.g[A0][0] == 0u64;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x228), 0u64);
                    if t {
                        break 'b_1d5978;
                    } else {
                        c.cycles += 2;
                        call_ptr(c, 0x00317e98, 0x001d5978)?;
                        break 'b_1d5978;
                    }
                }
                c.cycles += 2;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x1d4), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x1d0), 0u64);
                break 'b_1d5980;
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020a430, 0x001d5988)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    });
}

/// The inlined `Cfg_1D5F40()` copy: the 0x288-byte option block at 0x535610
/// goes to `dst` (here the stack). The source is a constant 8-aligned address,
/// so the original's unaligned `ldl/ldr` loop is never taken; this is its
/// doubleword loop (20 rounds of 32 bytes, `v0` the end pointer) with the four
/// scratch registers `lr` in the original's allocation. The 8-byte tail is
/// `oc763_cfg_tail`.
pub(crate) fn oc763_cfg_copy(c: &mut Cpu, src: usize, dst: usize, lr: [usize; 4], head: u32) {
    c.cycles += 1;
    c.g[V0][0] = sx((c.g[src][0] as u32).wrapping_add(0x280));
    loop {
        c.cycles += 11;
        let s = c.g[src][0] as u32;
        let d = c.g[dst][0] as u32;
        for (i, &reg) in lr.iter().enumerate() {
            c.g[reg][0] = c.ld(s.wrapping_add(8 * i as u32));
        }
        for (i, &reg) in lr.iter().enumerate() {
            let v = c.g[reg][0];
            c.sd(d.wrapping_add(8 * i as u32), v);
        }
        c.g[src][0] = sx(s.wrapping_add(0x20));
        let more = c.g[src][0] != c.g[V0][0];
        c.g[dst][0] = sx(d.wrapping_add(0x20));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The last 8 bytes of the option block copy (`ldl/ldr` into `reg`, then
/// `sdl/sdr`).
pub(crate) fn oc763_cfg_tail(c: &mut Cpu, src: usize, dst: usize, reg: usize) {
    let s = c.g[src][0] as u32;
    c.ldl(reg, s.wrapping_add(7));
    c.ldr(reg, s);
    let d = c.g[dst][0] as u32;
    c.sdl(reg, d.wrapping_add(7));
    c.sdr(reg, d);
}

/// `cFEMemCard_readDone` (0x1d5f40, `MemCardUI_ApplyLoadedData`): the memory
/// card read finished. With the save interface (`getInterface(getBE(), 5)`)
/// it hands the read buffer to `loadSaveGame` (type 2, then pushes the loaded
/// sound options through the audio object: output mode, three volumes, DJ
/// speech and arcade SFX, each re-reading the option block at 0x535610 into a
/// stack copy), or to `loadSaveProfile` (type 0, with the player index);
/// type 1 loads nothing. Returns 0 when the load fails; otherwise, if the
/// buffer state (`+0x228`) is 1, frees the start buffer and clears the
/// read/start buffer and state, and returns 1.
pub fn mem_card_ui_apply_loaded_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x2b0);
        frame.save(c, 16, 0x2a0);
        frame.save_ra(c, 0x290);
        call_ptr(c, 0x0014_dc80, 0x001d_5f54)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 5;
        call_ptr(c, 0x0014_dd58, 0x001d_5f60)?;
        let inst_slot = (c.g[GP][0] as u32).wrapping_sub(0x10c8);
        c.g[A1][0] = c.lw(inst_slot);
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1bc));
        let ty = c.g[V1][0];
        c.g[V0][0] = ((ty as i64) < 2) as u64;
        'epi: {
            'tail: {
                if ty == 1 {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(inst_slot);
                    break 'tail;
                }
                c.cycles += 2;
                let ge2 = c.g[V0][0] == 0;
                c.g[V0][0] = 2;
                if !ge2 {
                    c.cycles += 2;
                    if ty != 0 {
                        c.cycles += 2;
                        c.g[A0][0] = c.lw(inst_slot);
                        break 'tail;
                    }
                    // type 0: loadSaveProfile(iface, readBuffer, playerIdx)
                    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc));
                    c.cycles += 4;
                    c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1d0));
                    call_ptr(c, 0x0015_2de0, 0x001d_6464)?;
                    let ok = c.g[V0][0] != 0;
                    c.g[A0][0] = c.lw(inst_slot);
                    if ok {
                        break 'tail;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    break 'epi;
                }
                c.cycles += 2;
                if ty != c.g[V0][0] {
                    c.g[A0][0] = c.lw(inst_slot);
                    break 'tail;
                }
                // type 2: loadSaveGame(iface, readBuffer)
                c.cycles += 4;
                c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1d0));
                call_ptr(c, 0x0015_2bb0, 0x001d_5fa0)?;
                let failed = c.g[V0][0] == 0;
                c.g[V0][0] = 0;
                if failed {
                    break 'epi;
                }
                c.cycles += 4;
                call_ptr(c, 0x0028_b180, 0x001d_5fb0)?;
                if c.g[V0][0] == 0 {
                    c.g[A0][0] = c.lw(inst_slot);
                    break 'tail;
                }
                // The audio object exists: apply the loaded sound options.
                c.cycles += 11;
                call_ptr(c, 0x0014_dc80, 0x001d_5fc0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 4;
                call_ptr(c, 0x0014_dd58, 0x001d_5fcc)?;
                c.g[V1][0] = 0x53_0000;
                c.g[S0][0] = c.g[V1][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                c.g[V0][0] = c.g[V1][0] & 7;
                c.g[A0][0] = c.g[SP][0];
                oc763_cfg_copy(c, V1, A0, [T1, A1, A2, A3], 0x001d_6040);
                // output mode: bits 12..13
                c.cycles += 15;
                oc763_cfg_tail(c, V1, A0, 8);
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[A0][0] = sx((c.g[A0][0] as u32) >> 12);
                c.g[A0][0] &= 3;
                let apply = Callee {
                    addr: 0x001d_64c0,
                    f: ee::s04::MemCardUI_ApplySoundMode,
                };
                apply.call(c, 0x001d_608c)?;
                call_ptr(c, 0x0028_b180, 0x001d_6094)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5610));
                c.g[V0][0] = c.g[A1][0] & 7;
                c.g[V1][0] = c.g[SP][0];
                oc763_cfg_copy(c, A1, V1, [A2, A3, T0, T1], 0x001d_6104);
                // sfx volume: bits 0..3
                c.cycles += 14;
                oc763_cfg_tail(c, A1, V1, 2);
                c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                c.g[A1][0] &= 0xf;
                call_ptr(c, 0x0028_7410, 0x001d_614c)?;
                call_ptr(c, 0x0028_b180, 0x001d_6154)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5610));
                c.g[V0][0] = c.g[A1][0] & 7;
                c.g[V1][0] = c.g[SP][0];
                oc763_cfg_copy(c, A1, V1, [T1, A2, A3, T0], 0x001d_61c4);
                // speech volume: bits 8..11
                c.cycles += 15;
                oc763_cfg_tail(c, A1, V1, 9);
                c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 8);
                c.g[A1][0] &= 0xf;
                call_ptr(c, 0x0028_7488, 0x001d_6210)?;
                call_ptr(c, 0x0028_b180, 0x001d_6218)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5610));
                c.g[V0][0] = c.g[A1][0] & 7;
                c.g[V1][0] = c.g[SP][0];
                oc763_cfg_copy(c, A1, V1, [T0, T1, A2, A3], 0x001d_6288);
                // music volume: high nibble of byte 0
                c.cycles += 14;
                oc763_cfg_tail(c, A1, V1, 8);
                c.g[A1][0] = c.lbu(c.g[SP][0] as u32);
                c.g[A1][0] >>= 4;
                call_ptr(c, 0x0028_7520, 0x001d_62d0)?;
                call_ptr(c, 0x0028_b180, 0x001d_62d8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5610));
                c.g[V0][0] = c.g[A1][0] & 7;
                c.g[V1][0] = c.g[SP][0];
                oc763_cfg_copy(c, A1, V1, [A3, T0, T1, A2], 0x001d_6348);
                // DJ speech flag: bit 17
                c.cycles += 15;
                oc763_cfg_tail(c, A1, V1, 7);
                c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 17) as u32);
                c.g[A1][0] &= 1;
                call_ptr(c, 0x0028_7558, 0x001d_6394)?;
                call_ptr(c, 0x0028_b180, 0x001d_639c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5610));
                c.g[V0][0] = c.g[V1][0] & 7;
                c.g[A1][0] = c.g[SP][0];
                oc763_cfg_copy(c, V1, A1, [A2, A3, T0, T1], 0x001d_640c);
                // arcade SFX flag: bit 16
                c.cycles += 9;
                oc763_cfg_tail(c, V1, A1, 2);
                c.g[A1][0] = c.lh((c.g[SP][0] as u32).wrapping_add(2));
                c.g[A1][0] &= 1;
                call_ptr(c, 0x0028_75d0, 0x001d_6454)?;
                c.g[A0][0] = c.lw(inst_slot);
            }
            // The load worked: release a finished read buffer.
            c.cycles += 4;
            c.g[V1][0] = 1;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x228));
            let busy = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 1;
            if busy {
                break 'epi;
            }
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1d4));
            let none = c.g[A0][0] == 0;
            c.g[V0][0] = c.lw(inst_slot);
            if !none {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x001d_649c)?;
                c.g[V0][0] = c.lw(inst_slot);
            }
            c.cycles += 4;
            let inst = c.g[V0][0] as u32;
            c.sw(inst.wrapping_add(0x228), 0);
            c.sw(inst.wrapping_add(0x1d0), 0);
            c.sw(inst.wrapping_add(0x1d4), 0);
            c.g[V0][0] = 1;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x2a0);
        frame.restore_ra(c, 0x290);
        frame.close(c);
        Ok(())
    })
}

/// `cFEMemCard::InstallCallbacks(self, mode)` (0x1d5488): the memory-card
/// front-end state registers itself as the memory-card overlay manager's user.
///
/// Clears the state's flags, fetches the MC manager (`cSSXApp_getMCManager`),
/// stores the 40 `cFEMemCard_callback*` function pointers into the manager's
/// callback table (`fp + 0x00 .. 0xa8`), starts the manager (`ManagerGainState`),
/// sizes the read/write sets by `mode` (2: save game, 1: 0x80000 bytes,
/// else: save profile) and builds the save file name with `sprintf`.
/// The straight-line table setup is kept in the generated order (every
/// scratch register the code leaves behind is part of the call logs).
#[allow(unreachable_code)]
pub fn mem_card_state_base_install_callbacks(c: &mut Cpu) {
    guest(c, |c| {
        'b_1d583c: {
            'b_1d57d4: {
                c.cycles += 197;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x6b0));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x630));
                c.g[S7][0] = c.g[A0][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x600), c.g[A1][0]);
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x6a0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x690));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x680));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x670));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x660));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x650));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x640));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x620));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x610), c.g[RA][0]);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x10c8), c.g[S7][0]);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1c0), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x214), 0u64);
                call_ptr(c, 0x00227f80, 0x001d54d0)?;
                c.g[T3][0] = 0x1e0000;
                c.g[FP][0] = c.g[V0][0];
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x22b8));
                c.g[T9][0] = 0x1d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x68), c.g[T3][0]);
                c.g[T8][0] = 0x1d0000;
                c.g[T3][0] = 0x1e0000;
                c.g[S6][0] = 0x1d0000;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x2288));
                c.g[S5][0] = 0x1d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x74), c.g[T3][0]);
                c.g[S4][0] = 0x1d0000;
                c.g[T3][0] = 0x1e0000;
                c.g[S3][0] = 0x1d0000;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x2248));
                c.g[S2][0] = 0x1d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x84), c.g[T3][0]);
                c.g[A0][0] = 0x1d0000;
                c.g[T3][0] = 0x1d0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5f38));
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x68e8));
                c.g[T7][0] = 0x1d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x44), c.g[T3][0]);
                c.g[T6][0] = 0x1e0000;
                c.g[T3][0] = 0x1d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7010));
                c.g[A0][0] = 0x1e0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x40), c.g[T3][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2298));
                c.g[T3][0] = 0x1d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x70), c.g[A0][0]);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x5c28));
                c.g[A0][0] = 0x1e0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x1c), c.g[T3][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x22c8));
                c.g[T3][0] = 0x1e0000;
                c.g[T5][0] = 0x1d0000;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x2258));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x78), c.g[A0][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x88), c.g[T3][0]);
                c.g[V0][0] = 0x1d0000;
                c.g[T3][0] = 0x1d0000;
                c.g[V1][0] = 0x1d0000;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7318));
                c.g[A1][0] = 0x1d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x48), c.g[T3][0]);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x59a0));
                c.g[T3][0] = 0x1d0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5df0));
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7420));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f40));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x50), c.g[T3][0]);
                c.g[T2][0] = 0x1e0000;
                c.g[T3][0] = 0x1d0000;
                c.g[T4][0] = 0x1e0000;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7548));
                c.g[S0][0] = 0x1e0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x4c), c.g[T3][0]);
                c.g[S1][0] = 0x1e0000;
                c.g[T3][0] = 0x1d0000;
                c.g[A0][0] = 0x1d0000;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7688));
                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x22d8));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x54), c.g[T3][0]);
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x22a8));
                c.g[T3][0] = 0x1d0000;
                c.sw(c.g[FP][0] as u32, c.g[V0][0]);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x72a8));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x38), c.g[T3][0]);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2278));
                c.g[T3][0] = c.g[T9][0];
                c.sw((c.g[FP][0] as u32).wrapping_add(0x20), c.g[A1][0]);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x67c8));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2268));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x28), c.g[T3][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d78));
                c.g[T3][0] = c.g[T8][0];
                c.g[T1][0] = 0x1e0000;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x6ad0));
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x7ea0));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x24), c.g[T3][0]);
                c.g[T0][0] = 0x1e0000;
                c.g[T3][0] = c.g[S6][0];
                c.sw((c.g[FP][0] as u32).wrapping_add(0x64), c.g[T4][0]);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x6d88));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x6c), c.g[T2][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x3c), c.g[T3][0]);
                c.g[A3][0] = 0x1e0000;
                c.g[T3][0] = c.g[S5][0];
                c.sw((c.g[FP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x77e8));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x7c), c.g[S0][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x58), c.g[T3][0]);
                c.g[A2][0] = 0x1e0000;
                c.g[T3][0] = c.g[S4][0];
                c.sw((c.g[FP][0] as u32).wrapping_add(0x80), c.g[S1][0]);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x6c28));
                c.g[A1][0] = 0x1d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x10), c.g[T3][0]);
                c.g[V1][0] = 0x1d0000;
                c.g[T3][0] = c.g[S3][0];
                c.g[V0][0] = 0x1d0000;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x66b0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7c68));
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), c.g[T3][0]);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7c58));
                c.g[T3][0] = c.g[S2][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7ca8));
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7998));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7dc0));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x60), c.g[T3][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7b18));
                c.g[T3][0] = c.g[T7][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7d70));
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7ed0));
                c.g[T2][0] = 0x19;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x30), c.g[T3][0]);
                c.g[A0][0] = c.g[FP][0];
                c.g[T3][0] = c.g[T6][0];
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7fe0));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x34), c.g[T3][0]);
                c.g[T3][0] = c.g[T5][0];
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7190));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x8c), c.g[T3][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x90), c.g[T1][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x94), c.g[T0][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x98), c.g[A3][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x9c), c.g[A2][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x5c), c.g[A1][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0xa8), c.g[V0][0]);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x19c), c.g[T2][0]);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x600));
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1a8), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1bc), c.g[V0][0]);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1ac), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1b0), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1b4), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1b8), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1c4), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1cc), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1d8), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1dc), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1e0), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1e4), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1e8), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1ec), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1f0), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1f4), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1f8), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1fc), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x200), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x204), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x208), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x20c), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x210), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x218), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x21c), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1a0), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1a4), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x220), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x224), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x228), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1d4), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1d0), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1c8), 0u64);
                c.sh((c.g[S7][0] as u32).wrapping_add(0x9c), 0u64);
                call_ptr(c, 0x0023d5a8, 0x001d5778)?;
                call_ptr(c, 0x0014dc80, 0x001d5780)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x5;
                call_ptr(c, 0x0014dd58, 0x001d578c)?;
                c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x1bc));
                c.g[V0][0] = 0x2;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_1d57d4;
                } else {
                    c.cycles += 14;
                    call_ptr(c, 0x00152948, 0x001d57a4)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0023cc58, 0x001d57b4)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[FP][0];
                    call_ptr(c, 0x0023caa0, 0x001d57c0)?;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x0023c8f0, 0x001d57cc)?;
                    let t = true;
                    c.g[A0][0] = c.g[FP][0];
                    if t {
                        break 'b_1d583c;
                    } else {
                        break 'b_1d57d4;
                    }
                }
            }
            'b_1d5808: {
                c.cycles += 2;
                let t = c.g[V1][0] != c.g[V0][0];

                if t {
                    break 'b_1d5808;
                } else {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = 0x80000;
                    call_ptr(c, 0x0023cc58, 0x001d57e8)?;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = 0x80000;
                    call_ptr(c, 0x0023caa0, 0x001d57f4)?;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0023c8f0, 0x001d5800)?;
                    let t = true;
                    c.g[A0][0] = c.g[FP][0];
                    if t {
                        break 'b_1d583c;
                    } else {
                        break 'b_1d5808;
                    }
                }
            }
            c.cycles += 13;
            call_ptr(c, 0x00152ba8, 0x001d5810)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[FP][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0023cc58, 0x001d5820)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[FP][0];
            call_ptr(c, 0x0023caa0, 0x001d582c)?;
            c.g[A0][0] = c.g[FP][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0023c8f0, 0x001d5838)?;
            c.g[A0][0] = c.g[FP][0];
            break 'b_1d583c;
        }
        c.cycles += 30;
        c.g[A2][0] = c.g[SP][0];
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xb4));
        c.g[A3][0] = 0x100;
        call_ptr(c, 0x00241d40, 0x001d584c)?;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x400));
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14c8));
        call_ptr(c, 0x002c2540, 0x001d5860)?;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
        c.g[A1][0] = 0x100;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00241aa0, 0x001d5870)?;
        c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x11c));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[SP][0];
        call_ptr(c, 0x002c26d0, 0x001d5884)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x6a0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x690));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x680));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x670));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x660));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x650));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x640));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x630));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x620));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x610));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x6b0));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `void MemCardUI_BeginLoad(void)` (0x1d5df0; reference `cFEMemCard_readStart`).
///
/// Starts the memory-card read of the front-end save screen. Marks the
/// `cFEMemCard` instance (`gp-0x10c8`) as waiting (`+0x1b0 = 1`, `+0x1b4 = 0`),
/// points the card manager's file cursor (`+0xf8`) at its slot (`+0x1c4`) and
/// then reads by load type (`+0x1bc`): 0 profile (size
/// `getSaveProfileSize`), 1 replay (buffer `getReplayBuffer`, size
/// `cBE_getDataSize`, `+0x228 = 2`, remembers the type at `+0x200`), 2 save
/// game (size from `0x152948`); each reads via `cMCOverlayManager_readFile`.
/// Other types do nothing more.
pub fn mem_card_ui_begin_load(c: &mut Cpu) {
    guest(c, |c| {
        'b_1d5f24: {
            'b_1d5f20: {
                'b_1d5f1c: {
                    'b_1d5eec: {
                        'b_1d5ea4: {
                            c.cycles += 23;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                            c.g[RA][0] = sx(0x001d5e0c);
                            call_ptr(c, 0x0014dc80, 0x001d5e0c)?;
                            c.g[A1][0] = 0x5;
                            c.g[RA][0] = sx(0x001d5e18);
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, 0x0014dd58, 0x001d5e18)?;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[RA][0] = sx(0x001d5e24);
                            c.g[S0][0] = c.g[V0][0];
                            call_ptr(c, 0x00227f80, 0x001d5e24)?;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                            c.g[S2][0] = c.g[V0][0];
                            c.g[V1][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c4));
                            c.sw((c.g[A1][0] as u32).wrapping_add(0x1b0), c.g[V1][0]);
                            c.sw((c.g[A1][0] as u32).wrapping_add(0x1b4), 0u64);
                            c.sw((c.g[S2][0] as u32).wrapping_add(0xf8), c.g[V0][0]);
                            c.g[S1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1bc));
                            let t = c.g[S1][0] == c.g[V1][0];
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_1d5ea4;
                            } else {
                                'b_1d5e64: {
                                    c.cycles += 2;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = 0x2;
                                        break 'b_1d5e64;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == 0u64;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_1d5eec;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_1d5f24;
                                            } else {
                                                break 'b_1d5e64;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S1][0] != c.g[V0][0];
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_1d5f20;
                                } else {
                                    c.cycles += 14;
                                    c.g[RA][0] = sx(0x001d5e74);
                                    call_ptr(c, 0x00152948, 0x001d5e74)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                                    c.g[RA][0] = sx(0x001d5e84);
                                    c.g[A1][0] = c.g[S0][0];
                                    call_ptr(c, 0x001d9948, 0x001d5e84)?;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A3][0] = c.g[S0][0];
                                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d0));
                                    c.g[RA][0] = sx(0x001d5e9c);
                                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c4));
                                    call_ptr(c, 0x0023ca28, 0x001d5e9c)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_1d5f20;
                                    } else {
                                        break 'b_1d5ea4;
                                    }
                                }
                            }
                        }
                        c.cycles += 18;
                        c.g[V0][0] = 0x2;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001d5eb4);
                        c.sw((c.g[A1][0] as u32).wrapping_add(0x228), c.g[V0][0]);
                        call_ptr(c, 0x00152688, 0x001d5eb4)?;
                        c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1d4), c.g[V0][0]);
                        c.g[RA][0] = sx(0x001d5ec4);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1d0), c.g[V0][0]);
                        call_ptr(c, 0x0014dc80, 0x001d5ec4)?;
                        c.g[RA][0] = sx(0x001d5ecc);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014e048, 0x001d5ecc)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1d0));
                        c.g[A3][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c4));
                        c.g[RA][0] = sx(0x001d5ee0);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0023ca28, 0x001d5ee0)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                        let t = true;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x200), c.g[S1][0]);
                        if t {
                            break 'b_1d5f1c;
                        } else {
                            break 'b_1d5eec;
                        }
                    }
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x001d5ef4);
                    call_ptr(c, 0x00152ba8, 0x001d5ef4)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                    c.g[RA][0] = sx(0x001d5f04);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x001d9948, 0x001d5f04)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d0));
                    c.g[RA][0] = sx(0x001d5f1c);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c4));
                    call_ptr(c, 0x0023ca28, 0x001d5f1c)?;
                    break 'b_1d5f1c;
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_1d5f20;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1d5f24;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    })
}

/// A widget's virtual call `widget->vtbl[off]` with this-adjust (`lh off`, `lw off + 4`),
/// as the compiler emits it: v0 = vtable, a0 = widget + delta, v1 = function.
/// The caller sets a1.. beforehand; `x` is the register holding the widget.
pub(crate) fn oc943_vcall(c: &mut Cpu, x: usize, off: u32, ret: u32) -> crate::call::Flow {
    let widget = c.g[x][0] as u32;
    c.g[V0][0] = Addr(widget).off(8).lw(c);
    let vt = c.g[V0][0] as u32;
    c.g[A0][0] = Addr(vt).off(off).lh(c);
    c.g[V1][0] = Addr(vt).off(off + 4).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[RA][0] = sx(ret);
    c.g[A0][0] = sx(widget.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `void cFEStateTrophyRoom_OnWidgetCreated(self, cWidget* w)` (0x001d43f0): binds the trophy room's widgets by name hash (trophy image, a hidden button, the title text from the peak's string id, the six challenge names and their boxes) and shows or hides each by the number of event entries of the peak.
/// Reproduces every register, cycle and call of the generated body; `pc != entry`
/// (a resume after an unwound callee) runs the generated body.
pub fn c_fe_state_trophy_room_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x001d43f0 {
        return ee::s04::f_001d43f0__generated(c);
    }
    guest(c, |c| {
        'b_1d463c: {
            'b_1d4448: {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S1][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S0][0] = c.g[A1][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = 0x4a0000;
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1fe8));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = sx(0x001d4428);
                c.g[S3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                call_ptr(c, 0x00317670, 0x001d4428)?;
                let t = c.g[S3][0] != c.g[V0][0];
                c.g[A0][0] = 0x4a0000;
                if t {
                    break 'b_1d4448;
                } else {
                    c.cycles += 6;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[S0][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001d4440);
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x9c));
                    Callee {
                        addr: 0x001d47a0,
                        f: ee::s04::cFEStateTrophyRoom_ShowTrophyImage,
                    }
                    .call(c, 0x001d4440)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        break 'b_1d463c;
                    } else {
                        break 'b_1d4448;
                    }
                }
            }
            'b_1d447c: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001d4450);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ff0));
                call_ptr(c, 0x00317670, 0x001d4450)?;
                if c.g[S3][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x460000;
                    break 'b_1d447c;
                } else {
                    c.cycles += 9;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[S0][0]);
                    c.g[A1][0] = 0u64;
                    oc943_vcall(c, S0, 0x48, 0x001d4474)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        break 'b_1d463c;
                    } else {
                        break 'b_1d447c;
                    }
                }
            }
            'b_1d4508: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001d4484);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e60));
                call_ptr(c, 0x00317670, 0x001d4484)?;
                let t = c.g[S3][0] != c.g[V0][0];
                c.g[V1][0] = 0x440000;
                if t {
                    break 'b_1d4508;
                } else {
                    'b_1d44f8: {
                        'b_1d44ec: {
                            'b_1d44e4: {
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
                                c.g[V0][0] = 0x1;
                                let t = c.g[A0][0] == c.g[V0][0];
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_1d44e4;
                                } else {
                                    'b_1d44dc: {
                                        'b_1d44b8: {
                                            c.cycles += 3;
                                            c.g[V0][0] = ((c.g[A0][0] as i64) < 2i64) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_1d44b8;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A0][0] == 0u64;
                                                c.g[A0][0] = 0x460000;
                                                if t {
                                                    break 'b_1d44dc;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;

                                                    if t {
                                                        break 'b_1d44f8;
                                                    } else {
                                                        break 'b_1d44b8;
                                                    }
                                                }
                                            }
                                        }
                                        'b_1d44d0: {
                                            c.cycles += 2;
                                            let t = c.g[A0][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x3;
                                            if t {
                                                break 'b_1d44d0;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A0][0] != c.g[V0][0];
                                                c.g[A0][0] = 0x460000;
                                                if t {
                                                    break 'b_1d44f8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x76d8));
                                                    if t {
                                                        break 'b_1d44ec;
                                                    } else {
                                                        break 'b_1d44d0;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x460000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x76f0));
                                        if t {
                                            break 'b_1d44ec;
                                        } else {
                                            break 'b_1d44dc;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7708));
                                    if t {
                                        break 'b_1d44ec;
                                    } else {
                                        break 'b_1d44e4;
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.g[A0][0] = 0x460000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7720));
                            break 'b_1d44ec;
                        }
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x001d44f4);
                        call_ptr(c, 0x00317670, 0x001d44f4)?;
                        c.g[A1][0] = c.g[V0][0];
                        break 'b_1d44f8;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d4500);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a1000, 0x001d4500)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        break 'b_1d463c;
                    } else {
                        break 'b_1d4508;
                    }
                }
            }
            c.cycles += 4;
            c.g[V0][0] = 0x440000;
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1618));
            c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1600));
            c.g[S2][0] = 0u64;
            'l_1d4518: loop {
                'b_1d462c: {
                    'b_1d4628: {
                        'b_1d45d0: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d4520);
                            c.g[A0][0] = c.lw(c.g[S4][0] as u32);
                            call_ptr(c, 0x00317670, 0x001d4520)?;
                            let t = c.g[S3][0] != c.g[V0][0];

                            if t {
                                break 'b_1d45d0;
                            } else {
                                'b_1d4578: {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                                    c.g[RA][0] = sx(0x001d4534);
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
                                    Callee {
                                        addr: 0x001ce6f0,
                                        f: ee::s04::cFEStateTrophyRoom_CountEventEntries,
                                    }
                                    .call(c, 0x001d4534)?;
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S2][0]);
                                    if t {
                                        break 'b_1d4578;
                                    } else {
                                        c.cycles += 14;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001d4558);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001d4558)?;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0x1;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001d4570);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001d4570)?;
                                        let t = true;
                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_1d4628;
                                        } else {
                                            break 'b_1d4578;
                                        }
                                    }
                                }
                                'b_1d45b8: {
                                    c.cycles += 2;
                                    if c.g[S2][0] != 0u64 {
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                                        break 'b_1d45b8;
                                    } else {
                                        c.cycles += 14;
                                        c.g[RA][0] = sx(0x001d4588);
                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                        call_ptr(c, 0x0014dc80, 0x001d4588)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001d4594);
                                        c.g[A1][0] = 0xd;
                                        call_ptr(c, 0x0014dd58, 0x001d4594)?;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001d45a4);
                                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
                                        call_ptr(c, 0x00156a90, 0x001d45a4)?;
                                        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                        c.g[RA][0] = sx(0x001d45b0);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x003a0c50, 0x001d45b0)?;
                                        let t = true;
                                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_1d462c;
                                        } else {
                                            break 'b_1d45b8;
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
                                c.g[RA][0] = sx(0x001d45c8);
                                c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                Callee {
                                    addr: 0x001ce758,
                                    f: ee::s04::cFEStateTrophyRoom_SetEventName,
                                }
                                .call(c, 0x001d45c8)?;
                                let t = true;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_1d4628;
                                } else {
                                    break 'b_1d45d0;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001d45d8);
                        c.g[A0][0] = c.lw(c.g[S5][0] as u32);
                        call_ptr(c, 0x00317670, 0x001d45d8)?;
                        if c.g[S3][0] != c.g[V0][0] {
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                            break 'b_1d4628;
                        } else {
                            'b_1d4624: {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                                c.g[RA][0] = sx(0x001d45ec);
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
                                Callee {
                                    addr: 0x001ce6f0,
                                    f: ee::s04::cFEStateTrophyRoom_CountEventEntries,
                                }
                                .call(c, 0x001d45ec)?;
                                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_1d4624;
                                } else {
                                    c.cycles += 11;
                                    oc943_vcall(c, S0, 0x48, 0x001d460c)?;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0x1;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001d4624);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001d4624)?;
                                    break 'b_1d4624;
                                }
                            }
                            c.cycles += 1;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                            break 'b_1d4628;
                        }
                    }
                    c.cycles += 1;
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                    break 'b_1d462c;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 6i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x001d4518);
                    }
                    continue 'l_1d4518;
                } else {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    break 'b_1d463c;
                }
            }
        }
        c.cycles += 8;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `void MemCardUI_WriteSaveData(void)` (0x001d59a0): starts the save of the current memory-card operation: settings write, replay write (replay data size, file name and product code formatted into a title) or profile write, then, if no status popup is up and autosave is off, raises the "saving" popup with the overlay manager's message.
/// Reproduces every register, cycle and call of the generated body; `pc != entry`
/// (a resume after an unwound callee) runs the generated body.
pub fn mem_card_ui_write_save_data(c: &mut Cpu) {
    if c.pc != 0x001d59a0 {
        return ee::s04::f_001d59a0__generated(c);
    }
    guest(c, |c| {
        'b_1d5b68: {
            'b_1d59f8: {
                c.cycles += 18;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x7a0));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x730));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x790));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x780));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x770));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x760));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x750));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x740));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x720), c.g[RA][0]);
                c.g[RA][0] = sx(0x001d59d0);
                call_ptr(c, 0x00227f80, 0x001d59d0)?;
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                c.g[S6][0] = c.g[V0][0];
                c.g[V0][0] = 0x2;
                c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1bc));
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_1d59f8;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d59f0);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x002412a0, 0x001d59f0)?;
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                    if t {
                        break 'b_1d5b68;
                    } else {
                        break 'b_1d59f8;
                    }
                }
            }
            'b_1d5ac0: {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t {
                    break 'b_1d5ac0;
                } else {
                    c.cycles += 48;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[RA][0] = sx(0x001d5a10);
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x1c8), c.g[A1][0]);
                    call_ptr(c, 0x00270e68, 0x001d5a10)?;
                    c.g[RA][0] = sx(0x001d5a18);
                    call_ptr(c, 0x0014dc80, 0x001d5a18)?;
                    c.g[RA][0] = sx(0x001d5a20);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014e048, 0x001d5a20)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                    c.g[RA][0] = sx(0x001d5a30);
                    c.g[A1][0] = c.g[S3][0];
                    Callee {
                        addr: 0x001d9948,
                        f: ee::s04::MemCardStateBase_AllocReadBuffer,
                    }
                    .call(c, 0x001d5a30)?;
                    c.g[RA][0] = sx(0x001d5a38);
                    call_ptr(c, 0x0014dc80, 0x001d5a38)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001d5a48);
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d4));
                    call_ptr(c, 0x0014e050, 0x001d5a48)?;
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 0x41;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x001d5a5c);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x00241d40, 0x001d5a5c)?;
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x320));
                    c.g[A1][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x77a8));
                    c.g[RA][0] = sx(0x001d5a70);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x002c2540, 0x001d5a70)?;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
                    c.g[A1][0] = 0x100;
                    c.g[RA][0] = sx(0x001d5a80);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00241aa0, 0x001d5a80)?;
                    c.g[T0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xf8));
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A3][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001d5aa0);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x002c26d0, 0x001d5aa0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A3][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x001d5ab8);
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d4));
                    call_ptr(c, 0x00241000, 0x001d5ab8)?;
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                    if t {
                        break 'b_1d5b68;
                    } else {
                        break 'b_1d5ac0;
                    }
                }
            }
            c.cycles += 42;
            c.g[RA][0] = sx(0x001d5ac8);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            call_ptr(c, 0x00227f80, 0x001d5ac8)?;
            c.g[RA][0] = sx(0x001d5ad0);
            c.g[S3][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x001d5ad0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001d5adc);
            c.g[A1][0] = 0x5;
            call_ptr(c, 0x0014dd58, 0x001d5adc)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001d5aec);
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1cc));
            call_ptr(c, 0x00152950, 0x001d5aec)?;
            c.g[RA][0] = sx(0x001d5af4);
            c.g[S5][0] = c.g[V0][0];
            call_ptr(c, 0x00152ba8, 0x001d5af4)?;
            c.g[A2][0] = c.g[SP][0];
            c.g[A3][0] = 0x41;
            c.g[S4][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            c.g[RA][0] = sx(0x001d5b0c);
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x00241d40, 0x001d5b0c)?;
            c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x520));
            c.g[A1][0] = 0x460000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x77a8));
            c.g[RA][0] = sx(0x001d5b20);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x002c2540, 0x001d5b20)?;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
            c.g[A1][0] = 0x100;
            c.g[RA][0] = sx(0x001d5b30);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00241aa0, 0x001d5b30)?;
            c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xf8));
            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A3][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x001d5b50);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
            call_ptr(c, 0x002c26d0, 0x001d5b50)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S5][0];
            c.g[RA][0] = sx(0x001d5b64);
            c.g[A3][0] = c.g[S4][0];
            call_ptr(c, 0x00241000, 0x001d5b64)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
            break 'b_1d5b68;
        }
        'b_1d5c04: {
            'b_1d5c00: {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x210));
                let t = c.g[V0][0] != 0u64;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x790));
                if t {
                    break 'b_1d5c00;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x208));
                    let t = c.g[V0][0] != 0u64;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x780));
                    if t {
                        break 'b_1d5c04;
                    } else {
                        'b_1d5bfc: {
                            c.cycles += 29;
                            c.g[V0][0] = 0x16;
                            c.g[A1][0] = 0x460000;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x19c), c.g[V0][0]);
                            c.g[A3][0] = 0u64;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1d60));
                            c.g[A0][0] = 0x640;
                            c.g[RA][0] = sx(0x001d5ba0);
                            c.g[A2][0] = 0x100;
                            call_ptr(c, 0x00317e30, 0x001d5ba0)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S6][0];
                            c.g[A1][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001d5bb4);
                            c.g[A2][0] = 0x320;
                            call_ptr(c, 0x00241e18, 0x001d5bb4)?;
                            c.g[T3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A2][0] = 0x1;
                            c.g[A3][0] = 0x1;
                            c.g[V1][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x8));
                            c.g[T0][0] = 0u64;
                            c.g[T1][0] = 0u64;
                            c.g[T2][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x120));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001d5be4);
                            c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001d5be4)?;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                            c.g[V0][0] = 0x1;
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x210), c.g[V0][0]);
                            if t {
                                break 'b_1d5bfc;
                            } else {
                                c.cycles += 2;
                                c.g[RA][0] = sx(0x001d5bfc);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00317e98, 0x001d5bfc)?;
                                break 'b_1d5bfc;
                            }
                        }
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x790));
                        break 'b_1d5c00;
                    }
                }
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x780));
            break 'b_1d5c04;
        }
        c.cycles += 8;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x770));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x760));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x750));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x740));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x730));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x720));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x7a0));
        c.pc = t;
        return Ok(());
    });
}

/// `r32` plus an offset: the address `off(reg)`.
#[inline(always)]
pub(crate) fn oc945_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// `lw off(reg)`.
#[inline(always)]
pub(crate) fn oc945_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = oc945_at(c, reg, off);
    c.lw(a)
}

/// `lh off(reg)`.
#[inline(always)]
pub(crate) fn oc945_lh(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = oc945_at(c, reg, off);
    c.lh(a)
}

/// `lb off(reg)`.
#[inline(always)]
pub(crate) fn oc945_lb(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = oc945_at(c, reg, off);
    c.lb(a)
}

/// `lw -off(gp)` (the world and BE pointers the game keeps at gp).
#[inline(always)]
pub(crate) fn oc945_lw_gp(c: &mut Cpu, off: u32) -> u64 {
    let a = r32(c, GP).wrapping_sub(off);
    c.lw(a)
}

/// `lw off(obj); call` through the object's vtable: `v0 = obj->vt`,
/// `a0 = sx(obj + vt->delta)`, `v1 = vt->fn`, call `v1` returning to `ret`.
pub(crate) fn oc945_vcall(c: &mut Cpu, obj_reg: usize, ret: u32) -> Flow {
    c.g[V0][0] = oc945_lw(c, obj_reg, 8);
    c.g[A0][0] = oc945_lh(c, V0, 0x48);
    c.g[V1][0] = oc945_lw(c, V0, 0x4c);
    let target = r32(c, V1);
    c.g[A0][0] = sx(r32(c, obj_reg).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret)
}

/// `cFEStateTrophyRoom::OnActivate` (0x1d43d0): forwards to the UI state's
/// activate through the table.
pub fn cfe_state_trophy_room_on_activate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0039_e4c0), 0x001d_43e0)
}

/// `cFEStateTrophyRoom::ShowTrophyImage(self, index)` (0x1d47a0): the trophy
/// slot for `index` (negative: the image of the last trophy), then the item's
/// virtual method, and the slot's image index written at `item+0x78`.
pub fn cfe_state_trophy_room_show_trophy_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let negative = (c.g[A1][0] as i64) < 0;
        frame.save_ra(c, 0);
        // A negative index, or no slot found, takes the "no slot" exit.
        let has_slot = if negative {
            false
        } else {
            trophy_slot(c)?;
            c.cycles += 2;
            (c.g[S1][0] as i64) >= 0
        };
        if has_slot {
            trophy_item_slot(c, frame)?;
        } else {
            trophy_item_no_slot(c, frame)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The slot search of [`cfe_state_trophy_room_show_trophy_image`]: with an
/// index, `QueryEventReward` gives the slot; without one, the first trophy
/// name is hashed and looked up. Leaves the slot in s1 (-1 if none).
pub(crate) fn trophy_slot(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    let has_index = c.g[A1][0] != 0;
    c.g[S1][0] = u64::MAX;
    if has_index {
        c.cycles += 8;
        c.g[T0][0] = sx(r32(c, A1).wrapping_sub(1));
        c.g[A0][0] = oc945_lb(c, S0, 0x44);
        c.g[A1][0] = oc945_lw(c, S0, 0x50);
        c.g[A2][0] = oc945_lw(c, S0, 0x54);
        c.g[A3][0] = oc945_lw(c, S0, 0x58);
        Callee {
            addr: 0x001c_ed90,
            f: ee::s04::f_001ced90,
        }
        .call(c, 0x001d_4814)?;
        let same = c.g[V0][0] == c.g[S1][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        if !same {
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
            c.g[S1][0] = oc945_lw(c, V0, 0x60);
        }
        return Ok(());
    }
    c.cycles += 13;
    call_ptr(c, 0x0014_dc80, 0x001d_47c8)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xd;
    call_ptr(c, 0x0014_dd58, 0x001d_47d4)?;
    c.g[A1][0] = oc945_lb(c, S0, 0x44);
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = oc945_lw(c, S0, 0x50);
    c.g[A3][0] = oc945_lw(c, S0, 0x54);
    c.g[T0][0] = oc945_lw(c, S0, 0x58);
    call_ptr(c, 0x0015_7bf0, 0x001d_47ec)?;
    if c.g[V0][0] != 0 {
        c.g[S1][0] = oc945_lw(c, S0, 0x60);
    } else {
        c.cycles += 2;
    }
    Ok(())
}

/// The slot is known (`s1 >= 0`): the item's method is called with `a1 = 1`,
/// then the slot and a zero are stored in the item.
pub(crate) fn trophy_item_slot(c: &mut Cpu, frame: Frame) -> Flow {
    c.g[A2][0] = oc945_lw(c, S0, 0x48);
    c.cycles += 10;
    c.g[A1][0] = 1;
    oc945_vcall(c, A2, 0x001d_486c)?;
    c.g[V0][0] = oc945_lw(c, S0, 0x48);
    let a = oc945_at(c, V0, 0x78);
    c.sw(a, c.g[S1][0]);
    let a = oc945_at(c, V0, 0x7c);
    c.sw(a, 0);
    frame.restore(c, S0, 0x20);
    Ok(())
}

/// No slot (`s1 < 0`): the item's method is called with `a1 = 0`.
pub(crate) fn trophy_item_no_slot(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 9;
    c.g[A2][0] = oc945_lw(c, S0, 0x48);
    c.g[A1][0] = 0;
    oc945_vcall(c, A2, 0x001d_484c)?;
    frame.restore(c, S0, 0x20);
    Ok(())
}

/// `cFEStateTrophyRoom::RequestTrophyImages(self)` (0x1d4918): hash the five
/// trophy kinds into the list at `self+0x74`, one entry per tile, through the
/// world's texture table. Each entry asks for its image.
pub fn cfe_state_trophy_room_request_trophy_images(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0);
        c.g[S5][0] = sx(oc945_at(c, S0, 0x74));
        call_ptr(c, 0x0014_dc80, 0x001d_4948)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x001d_4954)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[A1][0] = oc945_lw(c, S0, 0x54);
        c.g[V0][0] = oc945_lw_gp(c, 0x848);
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = oc945_lw(c, S0, 0x58);
        c.g[V1][0] = oc945_lw(c, V0, 0x7c);
        c.g[S3][0] = sx(r32(c, V1).wrapping_add(0x1a70));
        call_ptr(c, 0x0015_6a90, 0x001d_4974)?;
        c.g[A2][0] = oc945_lw(c, V0, 4);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = oc945_lw(c, S0, 0x50);
        c.g[A3][0] = 0xa;
        c.g[T0][0] = 1;
        c.g[T1][0] = 0;
        call_ptr(c, 0x0019_da10, 0x001d_4990)?;
        c.g[V1][0] = 1;
        let a = oc945_at(c, S0, 0x74);
        c.sw(a, c.g[V0][0]);
        let a = oc945_at(c, S0, 0x5c);
        c.sw(a, c.g[V1][0]);
        c.g[A1][0] = oc945_lw(c, S0, 0x58);
        loop {
            // One tile per pass: the tile's record, then the next tile.
            c.cycles += 12;
            c.g[A2][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[S1][0] = sx(r32(c, S2).wrapping_add(1));
            call_ptr(c, 0x0015_6ae0, 0x001d_49b0)?;
            c.g[S2][0] = c.g[S1][0];
            c.g[V1][0] = oc945_lw(c, V0, 4);
            c.g[A0][0] = c.g[S3][0];
            c.g[A3][0] = 0xa;
            c.g[T0][0] = 1;
            c.g[T1][0] = 0;
            let empty = c.g[V1][0] == 0;
            c.g[A2][0] = c.g[V1][0];
            if !empty {
                c.cycles += 8;
                c.g[A1][0] = oc945_lw(c, S0, 0x50);
                call_ptr(c, 0x0019_da10, 0x001d_49d8)?;
                c.g[V1][0] = sx(r32(c, S1) << 2);
                c.g[V1][0] = sx(r32(c, S5).wrapping_add(r32(c, V1)));
                let a = r32(c, V1);
                c.sw(a, c.g[V0][0]);
                c.g[V0][0] = oc945_lw(c, S0, 0x5c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                let a = oc945_at(c, S0, 0x5c);
                c.sw(a, c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 4) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[A1][0] = oc945_lw(c, S0, 0x58);
            if event_due(c) {
                deliver_event_here(c, 0x001d_49a0);
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
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

/// `cFEStateTrophyRoom::LoadTrophyTextures(self)` (0x1d4a20): for each of the
/// five list entries with an index, load its texture (the world's texture
/// table) and show the trophy image for it.
pub fn cfe_state_trophy_room_load_trophy_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        c.g[V1][0] = oc945_lw_gp(c, 0x848);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = sx(oc945_at(c, S0, 0x74));
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        c.g[S3][0] = 4;
        frame.save(c, S5, 0x10);
        c.g[S4][0] = u64::MAX;
        frame.save_ra(c, 0);
        c.g[S5][0] = 0x004a_0000;
        c.g[V0][0] = oc945_lw(c, V1, 0x7c);
        c.g[S2][0] = sx(r32(c, V0).wrapping_add(0x1a70));
        loop {
            c.cycles += 5;
            c.g[V0][0] = oc945_lw(c, S1, 0);
            c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
            c.g[A0][0] = c.g[S2][0];
            let negative = (c.g[V0][0] as i64) < 0;
            c.g[A2][0] = c.g[V0][0];
            if !negative {
                c.cycles += 4;
                c.g[A1][0] = oc945_lw(c, S0, 0x50);
                call_ptr(c, 0x0019_e238, 0x001d_4a7c)?;
                let no_entry = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S2][0];
                if !no_entry {
                    c.cycles += 27;
                    c.g[A1][0] = oc945_lw(c, S0, 0x50);
                    c.g[A2][0] = oc945_lw(c, S1, 0);
                    call_ptr(c, 0x0019_e2b0, 0x001d_4a90)?;
                    c.g[T2][0] = oc945_lw_gp(c, 0x854);
                    c.g[A2][0] = sx(r32(c, S5).wrapping_add(0x1ff8));
                    c.g[A1][0] = oc945_lw(c, V0, 0x14);
                    c.g[A3][0] = 0;
                    c.g[V1][0] = oc945_lw(c, T2, 0x10d8);
                    c.g[T0][0] = 1;
                    c.g[A1][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
                    c.g[T1][0] = u64::MAX;
                    c.g[A0][0] = oc945_lh(c, V1, 0x170);
                    c.g[V0][0] = oc945_lw(c, V1, 0x174);
                    let target = r32(c, V0);
                    c.g[A0][0] = sx(r32(c, T2).wrapping_add(r32(c, A0)));
                    call_ptr(c, target, 0x001d_4ac0)?;
                    let a = r32(c, S1).wrapping_sub(0x14);
                    c.sw(a, c.g[V0][0]);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = oc945_lw(c, S1, 0);
                    c.g[A1][0] = oc945_lw(c, S0, 0x50);
                    call_ptr(c, 0x0019_dc20, 0x001d_4ad4)?;
                    let a = r32(c, S1);
                    c.sw(a, c.g[S4][0]);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] = oc945_lw(c, S0, 0x5c);
                    c.g[A1][0] = oc945_lw(c, S0, 0x9c);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                    let a = oc945_at(c, S0, 0x5c);
                    c.sw(a, c.g[V0][0]);
                    Callee {
                        addr: 0x001d_47a0,
                        f: ee::s04::f_001d47a0,
                    }
                    .call(c, 0x001d_4af0)?;
                }
            }
            c.cycles += 2;
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_4a60);
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
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

// ---------------------------------------------------------------------------
// cFEStateRewardsRoom::OnWidgetCreated (0x1d4cd8, 460 bytes)
// ---------------------------------------------------------------------------

/// Where the rewards chain leaves to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum RewardsExit {
    /// `cycles += 1; lq s0`, then the epilogue.
    Epilogue8c,
    /// The epilogue (the restores were done on the way).
    Epilogue90,
}

/// One string of the rewards chain: `a0 = sx(a0 + off)` (the table base is
/// already in `a0`), hash it, and report whether it is the widget's `s0`.
pub(crate) fn rewards_hash(c: &mut Cpu, off: u32, ret: u32) -> Result<bool, Unwound> {
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(off));
    call_ptr(c, 0x0031_7670, ret)?;
    Ok(c.g[S0][0] == c.g[V0][0])
}

/// The count-text call of a matched rewards entry (`a1` = the count).
pub(crate) fn rewards_set_count(c: &mut Cpu, count: u64, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A1][0] = count;
    Callee {
        addr: 0x001d_4f90,
        f: ee::s04::f_001d4f90,
    }
    .call(c, ret)
}

/// The rewards chain of [`cfe_state_rewards_room_on_widget_created`]: each
/// string of the table (hashed in turn, the hash compared with the widget's
/// name in `s0`) decides which entry of the room the widget is.
pub(crate) fn rewards_chain(c: &mut Cpu, frame: Frame) -> Result<RewardsExit, Unwound> {
    // Entries that store their kind at `self+0x18`.
    const KINDS: [(u32, u32, u64, u64); 6] = [
        (0x2000, 0x001d_4d04, 5, 0x004a_0000),
        (0x2008, 0x001d_4d20, 6, 0x004a_0000),
        (0x2010, 0x001d_4d3c, 3, 0x004a_0000),
        (0x2018, 0x001d_4d58, 7, 0x0046_0000),
        (0x7748, 0x001d_4d74, 4, 0x0046_0000),
        (0x7758, 0x001d_4d90, 8, 0x0046_0000),
    ];
    for (i, &(off, ret, kind, base)) in KINDS.iter().enumerate() {
        if i > 0 {
            c.cycles += 4;
        }
        let hit = rewards_hash(c, off, ret)?;
        c.g[A0][0] = base;
        if hit {
            c.cycles += 3;
            c.g[V0][0] = kind;
            let a = oc945_at(c, S1, 0x18);
            c.sw(a, c.g[V0][0]);
            return Ok(RewardsExit::Epilogue8c);
        }
    }
    // Entries that set the count text; the base of their strings is 0x460000.
    const COUNTS: [(u32, u32, u64, u32); 5] = [
        (0x74a8, 0x001d_4dac, 5, 0x001d_4dc4),
        (0x74b8, 0x001d_4dd4, 3, 0x001d_4dec),
        (0x74c8, 0x001d_4dfc, 7, 0x001d_4e14),
        (0x74d8, 0x001d_4e24, 4, 0x001d_4e3c),
        (0x74e8, 0x001d_4e4c, 8, 0x001d_4e64),
    ];
    for &(off, ret, count, text_ret) in COUNTS.iter() {
        c.cycles += 4;
        let hit = rewards_hash(c, off, ret)?;
        c.g[A0][0] = 0x0046_0000;
        if hit {
            c.cycles += 6;
            rewards_set_count(c, count, text_ret)?;
            frame.restore(c, S0, 0x30);
            return Ok(RewardsExit::Epilogue90);
        }
    }
    // The last string: the restore comes before the test.
    c.cycles += 4;
    let hit = rewards_hash(c, 0x7768, 0x001d_4e74)?;
    frame.restore(c, S0, 0x30);
    if !hit {
        return Ok(RewardsExit::Epilogue90);
    }
    c.cycles += 4;
    rewards_set_count(c, 6, 0x001d_4e8c)?;
    Ok(RewardsExit::Epilogue8c)
}

/// `cFEStateRewardsRoom::OnWidgetCreated(self, widget)` (0x1d4cd8): hash the
/// widget's name (`widget+0x38`) against the rewards strings and store the
/// matching kind at `self+0x18` (or set the count text of that entry).
pub fn cfe_state_rewards_room_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x004a_0000;
        c.g[S0][0] = oc945_lw(c, S1, 0x38);
        let exit = rewards_chain(c, frame)?;
        if exit == RewardsExit::Epilogue8c {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `gp - back`: the address of a global the generated code reaches through gp.
pub(crate) fn oc946_gp(c: &Cpu, back: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(back)
}

/// `cFEStateRelogin` net-event codes the dispatcher tests (the reference's
/// `kNetEvent_LobbyConnect`, `kNetEvent_Authentication` and the two numbered
/// events it handles).
pub(crate) const RELOGIN_LOBBY_CONNECT: u64 = 0xc9;

pub(crate) const RELOGIN_AUTHENTICATION: u64 = 0xd4;

pub(crate) const RELOGIN_ONLINE_MENU: u64 = 0x101;

pub(crate) const RELOGIN_RANKED_OPTIONS: u64 = 0x111;

/// `cFEStateRelogin_v31(self, msg)` (0x1d5138): the net-event handler of the
/// relogin state. Lobby connect and authentication forward to LobbyMan, 0x101
/// builds the online main menu, 0x111 starts a ranked game with the network
/// options; any other event goes to the parent handler `func_001BF228`.
pub fn c_fe_state_relogin_v31(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = RELOGIN_AUTHENTICATION;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let msg = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        if relogin_handled(c, msg)? {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            // Default: the parent state's handler with the same message.
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001b_f228, 0x001d_5228)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The cases of [`c_fe_state_relogin_v31`]: true when the message was handled
/// (the function returns 0), false when it goes to the parent handler. Leaves
/// `v0` as the generated dispatch leaves it.
pub(crate) fn relogin_handled(c: &mut Cpu, msg: u64) -> Result<bool, Unwound> {
    if msg == RELOGIN_AUTHENTICATION {
        c.cycles += 5;
        c.g[A0][0] = Addr(oc946_gp(c, 0xc8)).lw(c);
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x80)).lw(c);
        call_ptr(c, 0x0025_cd50, 0x001d_51a4)?;
        c.g[V0][0] = 0;
        return Ok(true);
    }
    c.cycles += 3;
    let below_213 = (msg as i64) < 213;
    c.g[V0][0] = RELOGIN_LOBBY_CONNECT;
    if !below_213 {
        c.cycles += 3;
        c.g[V0][0] = RELOGIN_ONLINE_MENU;
        let online_menu = msg == RELOGIN_ONLINE_MENU;
        c.g[V0][0] = RELOGIN_RANKED_OPTIONS;
        if online_menu {
            c.cycles += 14;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x6d0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd8));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x001d_51c4)?;
            c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x001b_fbb8, 0x001d_51d0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x001a_8a40, 0x001d_51dc)?;
            c.g[V0][0] = 0;
            return Ok(true);
        }
        c.cycles += 2;
        let ranked_options = msg == RELOGIN_RANKED_OPTIONS;
        c.g[V0][0] = Addr(oc946_gp(c, 0x848)).lw(c);
        if ranked_options {
            c.cycles += 15;
            c.g[A1][0] = 7;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x78)).lw(c);
            call_ptr(c, 0x0014_dd58, 0x001d_51f0)?;
            c.g[T0][0] = 0x530000;
            c.g[A0][0] = Addr(oc946_gp(c, 0xc8)).lw(c);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x4b30));
            c.g[A2][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x8)).lw(c);
            c.g[T1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x3dc));
            c.g[A1][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[A3][0] = (0u64 < c.g[A2][0]) as u64;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x330));
            c.g[A2][0] = (c.g[A2][0] < 0x1u64) as u64;
            call_ptr(c, 0x0025_fd50, 0x001d_5218)?;
            c.g[V0][0] = 0;
            return Ok(true);
        }
        c.cycles += 2;
        return Ok(false);
    }
    // below 213: lobby connect, or the parent handler
    c.cycles += 2;
    if msg == RELOGIN_LOBBY_CONNECT {
        c.cycles += 4;
        c.g[A0][0] = Addr(oc946_gp(c, 0xc8)).lw(c);
        call_ptr(c, 0x0025_c610, 0x001d_5190)?;
        c.g[V0][0] = 0;
        return Ok(true);
    }
    c.cycles += 2;
    Ok(false)
}

/// `bool cFEMemCard_callbackReadUpdate(size)` (`MemCardUI_ReadSaveChunk`,
/// 0x1d5c28): reads one replay save chunk from the memory card. Only for the
/// replay card type: checks the BE data's CRC (a mismatch releases the replay
/// buffer and fails), then copies `size` bytes (at most 0x4000) into the
/// overlay manager's read. Returns true unless the CRC failed.
pub fn mem_card_ui_read_save_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = Addr(oc946_gp(c, 0x10c8)).lw(c);
        frame.save(c, S2, 0x30);
        c.g[S2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1bc)).lw(c);
        c.g[S1][0] = c.g[A0][0];
        if c.g[S2][0] == c.g[V0][0] {
            if read_chunk_bulk(c)? {
                read_chunk_size(c)?;
                c.cycles += 1;
                c.g[V0][0] = 1;
            } else {
                c.g[V0][0] = 0;
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

/// The `mLoadBEData` part of [`mem_card_ui_read_save_chunk`]. False when the
/// CRC of the BE data did not match (the function then returns false).
pub(crate) fn read_chunk_bulk(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x200)).lw(c);
    if c.g[V0][0] == 0 {
        return Ok(true);
    }
    c.cycles += 29;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x200), 0u64);
    c.g[S3][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1d0)).lw(c);
    call_ptr(c, 0x0014_dc80, 0x001d_5c6c)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_e048, 0x001d_5c74)?;
    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1d0)).lw(c);
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    c.g[A2][0] = 0xfbea;
    call_ptr(c, 0x003e_62d0, 0x001d_5c84)?;
    c.g[S0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_dc80, 0x001d_5c8c)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_e048, 0x001d_5c94)?;
    c.g[A0][0] = Addr(oc946_gp(c, 0x10c8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1d0)).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x1d0), c.g[V1][0]);
    // The stored CRC: an unaligned 4-byte copy of the buffer tail to the stack.
    c.sw(sp(c), 0u64);
    c.lwl(2, (c.g[V1][0] as u32).wrapping_add(0x3));
    c.lwr(2, c.g[V1][0] as u32);
    c.swl(2, sp(c).wrapping_add(0x3));
    c.swr(2, sp(c));
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1d0)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x1d0), c.g[V0][0]);
    c.g[V1][0] = Addr(sp(c)).lw(c);
    if c.g[S0][0] != c.g[V1][0] {
        c.cycles += 13;
        call_ptr(c, 0x0014_dc80, 0x001d_5cdc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 5;
        call_ptr(c, 0x0014_dd58, 0x001d_5ce8)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0015_2728, 0x001d_5cf0)?;
        c.g[V1][0] = Addr(oc946_gp(c, 0x10c8)).lw(c);
        c.g[V0][0] = 0;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x228), 0u64);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), 0u64);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1d0), 0u64);
        return Ok(false);
    }
    c.cycles += 5;
    c.sw(oc946_gp(c, 0x1738), c.g[S2][0]);
    call_ptr(c, 0x0014_dc80, 0x001d_5d10)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S3][0];
    call_ptr(c, 0x0014_e0c0, 0x001d_5d1c)?;
    Ok(true)
}

/// The size part of [`mem_card_ui_read_save_chunk`]: clamps `size` to 0x4000,
/// reads that many bytes through the overlay manager and advances the buffer.
pub(crate) fn read_chunk_size(c: &mut Cpu) -> Result<(), Unwound> {
    c.cycles += 2;
    let positive = (c.g[S1][0] as i64) > 0;
    c.g[V1][0] = ((c.g[S1][0] as i64) < 16385) as u64;
    if !positive {
        return Ok(());
    }
    c.cycles += 13;
    c.g[A0][0] = Addr(oc946_gp(c, 0x848)).lw(c);
    c.g[V0][0] = 0x4000;
    if c.g[V1][0] == 0 {
        c.g[S1][0] = c.g[V0][0];
    }
    call_ptr(c, 0x0022_7f80, 0x001d_5d34)?;
    c.g[V1][0] = Addr(oc946_gp(c, 0x10c8)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1d0)).lw(c);
    call_ptr(c, 0x0024_1200, 0x001d_5d48)?;
    c.g[V1][0] = Addr(oc946_gp(c, 0x10c8)).lw(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1d0)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x1d0), c.g[V0][0]);
    Ok(())
}

/// `bool cFEMemCard_callbackWriteUpdate(key)` (`MemCardUI_WriteReplaySaveBlock`,
/// 0x1d5d78): for the replay card type, asks the replay for its save data and
/// writes it to the overlay manager. Returns true when the data was written.
pub fn mem_card_ui_write_replay_save_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr(oc946_gp(c, 0x10c8)).lw(c);
        frame.save(c, S0, 0x20);
        c.g[A1][0] = 0x1;
        frame.save_ra(c, 0x10);
        c.sw(frame.at(c, 0x4), c.g[A0][0]);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1bc)).lw(c);
        let replay = c.g[V1][0] == c.g[A1][0];
        c.g[V0][0] = 0;
        if replay {
            c.cycles += 11;
            c.g[A0][0] = Addr(oc946_gp(c, 0x848)).lw(c);
            call_ptr(c, 0x0022_7f80, 0x001d_5da4)?;
            c.g[V1][0] = Addr(oc946_gp(c, 0x848)).lw(c);
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = c.g[SP][0] | 0x4;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
            call_ptr(c, 0x0027_10a8, 0x001d_5dc0)?;
            c.g[A2][0] = Addr(frame.at(c, 0x4)).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = Addr(sp(c)).lw(c);
                call_ptr(c, 0x0024_1138, 0x001d_5dd4)?;
                c.g[V0][0] = 1;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_paths;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::bx::bxstring::tests::heap_init as heap_init_3;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::alloc_machine;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::zero;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4_2 as OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_block;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::run as run_2;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::tests::oc770_install_fakes;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::tests::oc784_install_fakes;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS_2;
    use ssx3_ee as ee;

    #[test]
    fn mem_card_state_base_dtor_matches() {
        check(
            ee::s04::f_001d5428__generated,
            mem_card_state_base_dtor,
            60,
            &|c, seed| {
                let _r = alloc_machine(c, seed, 0x1d54_28);
                let obj = heap_block(c, 0x300);
                zero(c, obj, 0x300);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx((seed & 1) as u32);
            },
        );
    }

    pub(crate) const SEEDS: u32 = 100;

    /// A 32-bit word of test data.
    pub(crate) fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// The BELibrary singleton (`gp - 0x1f04`) and a zeroed dummy object it
    /// points at: without it a library lookup tries to load from disc forever.
    pub(crate) fn belib(c: &mut Cpu) {
        w(c, GAME_GP - 0x1f04, DATA + 0x8000);
    }

    #[test]
    fn fe_state_trophy_room_update_matches() {
        check(
            ssx3_ee::s04::f_001d4760__generated,
            fe_state_trophy_room_update,
            40,
            &|c, seed| {
                machine(c, seed, 0x1d47_60, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                let pending = if seed % 2 == 0 { 0 } else { 2 };
                w(c, DATA + 0x5c, pending);
            },
        );
    }

    #[test]
    fn fe_state_relogin_ctor_matches() {
        check(
            ssx3_ee::s04::f_001d5038__generated,
            fe_state_relogin_ctor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1d50_38, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn fe_state_relogin_on_child_event_matches() {
        check(
            ssx3_ee::s04::f_001d5240__generated,
            fe_state_relogin_on_child_event,
            60,
            &|c, seed| {
                machine(c, seed, 0x1d52_40, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                let event = if seed % 2 == 0 { 0x102 } else { 0x100 };
                w(c, DATA + 0x300 + 0x18, event);
            },
        );
    }

    #[test]
    fn cfe_state_relogin_on_widget_created_matches() {
        check(
            ssx3_ee::s04::f_001d50e0__generated,
            cfe_state_relogin_on_widget_created,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d50e0, 3);
                let this = DATA + 0x400;
                let widget = DATA + 0x500;
                let target = run_2(c, 0x0031_7670, ssx3_ee::s10::StrHash, &[0x0046_7790]);
                c.bus.write32(
                    widget + 0x38,
                    if seed % 2 == 0 {
                        target as u32
                    } else {
                        r.next_u32()
                    },
                );
                let vt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(widget + 8, vt);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = widget as u64;
            },
        );
    }

    #[test]
    fn cfe_state_trophy_room_on_widget_notify_matches() {
        check(
            ssx3_ee::s04::f_001d4660__generated,
            cfe_state_trophy_room_on_widget_notify,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d4660, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_trophy_room_init_matches() {
        check(
            ssx3_ee::s04::f_001d4368__generated,
            c_fe_state_trophy_room_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x1d4368);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn c_fe_state_relogin_init_matches() {
        check(
            ssx3_ee::s04::f_001d5078__generated,
            c_fe_state_relogin_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x1d5078);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn mem_card_state_base_ctor_matches() {
        check(
            ee::s04::f_001d53b0__generated,
            mem_card_state_base_ctor,
            16,
            &|c, seed| {
                machine(c, seed, 0x1d53b0, 8);
                be_library(c);
                heap_init_2(c);
                world(c);
                c.g[A0][0] = (DATA + 0x2_0000) as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = (DATA + 0xc00) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_rewards_room_init_matches() {
        check(
            ee::s04::f_001d4c60__generated,
            c_fe_state_rewards_room_init,
            SEEDS_3,
            &|c, seed| {
                let _r = machine(c, seed, 0x1d4c60, 4);
                be_library(c);
                heap_init_3(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x10, DATA + 0x1000);
                w(c, DATA + 0x1000, 0);
            },
        );
    }

    #[test]
    fn memcardstatebase_ctor_2_matches() {
        check(
            ee::s04::f_001d5330__generated,
            memcardstatebase_ctor_2,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x5330, 0);
                c.bus.write32(GAME_GP - 0x47c, DATA + 0x6000);
                let s = DATA + 0x100;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x700);
                c.g[A3][0] = (seed / 4) as u64;
                c.bus.write32(s + 0x1bc, if seed % 3 == 0 { 2 } else { 0 });
                c.bus.write32(0x004a_1390, (seed * 3) as u32);
            },
        );
    }

    #[test]
    fn cfe_state_rewards_room_ctor_matches() {
        check(
            ee::s04::f_001d4bc8__generated,
            cfe_state_rewards_room_ctor,
            40,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1d4bc8, 3);
                let obj = DATA + 0x1000;
                let engine = DATA + 0x2000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (seed % 8) as u64;
                c.g[A3][0] = r.next_u32() as u64;
                w(c, obj + 0x10, engine);
                w(c, engine, DATA + 0x3000);
                w(
                    c,
                    0x0053_08a8,
                    if seed % 2 == 0 { 0 } else { DATA + 0x4000 },
                );
                w(c, GAME_GP.wrapping_sub(0x1efc), 0);
            },
        );
    }

    #[test]
    fn cfe_rewards_room_set_count_text_matches() {
        check(
            ee::s04::f_001d4f90__generated,
            cfe_rewards_room_set_count_text,
            6,
            &|c, seed| {
                let _ = base_3(c, seed, 0x1d4f90, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = OBJ2 as u64;
                w(c, OBJ + 0x44, seed % 10);
                w(c, OBJ + 0x48, seed % 5);
                w(c, 0x004a_2020, 0);
            },
        );
    }

    // ----------------------------------------------------- trophy room images

    #[test]
    fn cfe_state_trophy_room_release_trophy_images_matches() {
        check(
            ee::s04::f_001d4b20__generated,
            cfe_state_trophy_room_release_trophy_images,
            SEEDS_4,
            &|c, seed| {
                let _ = base_4(c, seed, 0x1d4b_20, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                // The five image handles: -1 or a live image id.
                for i in 0..6u32 {
                    w(
                        c,
                        obj + 0x74 + 4 * i,
                        if (seed as u32 + i) % 3 == 0 {
                            0xffff_ffff
                        } else {
                            i + 1
                        },
                    );
                }
                w(c, obj + 0x50, obj + 0x2000);
                // The world's rider array (empty) and the interface vtable the
                // first call goes through.
                w(c, W, 0);
                w(c, IFACE + 0x10d8, obj + 0x2100);
                slot_2(c, obj + 0x2100, 0x190, 0);
            },
        );
    }

    #[test]
    fn cfe_relogin_v33_matches() {
        check(
            ee::s04::f_001d5280__generated,
            cfe_relogin_v33,
            8,
            &|c, seed| {
                let _ = base_5(c, seed, 0x001d_5280, 6);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = match seed % 4 {
                    0 => 0xf,
                    1 => 0x14,
                    _ => (seed % 32) as u64,
                };
                w(c, OBJ + 0x10, OBJ3);
                w(c, OBJ3 + 0x18, OBJ4);
                w(c, OBJ3 + 0x40, OBJ4);
            },
        );
    }

    #[test]
    fn fe_state_rewards_room_on_widget_action_matches() {
        strict(
            ee::s04::f_001d4ea8__generated,
            fe_state_rewards_room_on_widget_action,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d4e, 6);
                let this = DATA;
                let widget = DATA + 0x1000;
                c.bus.write32(this + 0x10, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 4, DATA + 0x4000);
                c.bus.write32(DATA + 0x4000 + 4, DATA + 0x5000);
                c.bus.write16(DATA + 0x5000 + 0x20, 0);
                c.bus.write32(DATA + 0x5000 + 0x24, FAKE);
                c.bus.write16(DATA + 0x5000 + 0x28, 0);
                c.bus.write32(DATA + 0x5000 + 0x2c, FAKE);
                c.bus.write32(widget + 0x18, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(widget) };
                c.g[A2][0] = [0u64, 1, 5, 6, 7][(seed % 5) as usize];
            },
        );
    }

    #[test]
    fn cfe_state_trophy_room_on_widget_action_matches() {
        check(
            ee::s04::f_001d4698__generated,
            cfe_state_trophy_room_on_widget_action,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d46, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                let item = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { item as u64 };
                c.g[A2][0] = match seed % 3 {
                    0 => 1,
                    1 => 6,
                    _ => 2,
                };
                c.bus.write32(item + 0x18, seed % 5);
                let panel = DATA + 0x2400;
                c.bus.write32(this + 0x48, panel);
                let vt = vtable(c, &mut r, &[0x48]);
                c.bus.write32(panel + 8, vt);
                c.bus.write32(this + 0x10, DATA + 0x2800);
                c.bus.write32(DATA + 0x2800, DATA + 0x2c00);
                c.bus.write32(DATA + 0x2c00 + 4, vt);
                c.bus.write32(this + 0x54, seed % 3);
            },
        );
    }

    #[test]
    fn mem_card_state_base_shutdown_matches() {
        diff(
            ee::s04::f_001d58b8__generated,
            mem_card_state_base_shutdown,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x1d_58b8);
                c.g[A0][0] = sx(DATA);
                w(c, DATA + 0x1bc, (seed % 4) as u32);
                w(c, DATA + 0x1c0, r.next_u32());
                w(c, DATA + 0x20, r.next_u32());
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fill;

    use crate::difftest::Rng;
    use std::sync::atomic::AtomicUsize;

    // Fixture objects in DATA+0x200000 and up, clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return.
    pub(crate) const OC763_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC763_MAGIC: u32 = 0x4f43_3736 + 3;

    pub(crate) const OC763_INST: u32 = DATA + 0x21_0000;

    pub(crate) const OC763_AUDIO: u32 = DATA + 0x22_0000;

    pub(crate) const OC763_P1: u32 = DATA + 0x23_0000;

    pub(crate) const OC763_P2: u32 = DATA + 0x24_0000;

    pub(crate) const OC763_P3: u32 = DATA + 0x25_0000;

    pub(crate) const OC763_P4: u32 = DATA + 0x26_0000;

    pub(crate) const OC763_FORCE: u32 = DATA + 0x30_0000;

    pub(crate) const OC763_CFG: u32 = 0x0053_5610;

    pub(crate) fn oc763_mail(addr: u32) -> u32 {
        OC763_FORCE + (addr & 0xf_fffc)
    }

    /// The gated fake callee: logs the call, returns the mailbox result
    /// (1 = 0, else value + 3; empty = the fake's pointer).
    pub(crate) fn oc763_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc763_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = if mail == 1 { 0 } else { (mail - 3) as u64 };
        }
    }

    macro_rules! oc763_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC763_CTRL) != OC763_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc763_fake_call(c, $addr);
                }
            )*
            fn oc763_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc763_fakes!(
        (
            oc763_fake_14dc80,
            OC763_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc763_fake_14dd58,
            OC763_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc763_fake_152bb0,
            OC763_PREV_152BB0,
            ee::s01::OVERRIDE_00152BB0,
            ee::s01::f_00152bb0__generated,
            0x0015_2bb0
        ),
        (
            oc763_fake_152de0,
            OC763_PREV_152DE0,
            ee::s01::OVERRIDE_00152DE0,
            ee::s01::f_00152de0__generated,
            0x0015_2de0
        ),
        (
            oc763_fake_287410,
            OC763_PREV_287410,
            ee::s07::OVERRIDE_00287410,
            ee::s07::f_00287410__generated,
            0x0028_7410
        ),
        (
            oc763_fake_287488,
            OC763_PREV_287488,
            ee::s07::OVERRIDE_00287488,
            ee::s07::f_00287488__generated,
            0x0028_7488
        ),
        (
            oc763_fake_287520,
            OC763_PREV_287520,
            ee::s07::OVERRIDE_00287520,
            ee::s07::f_00287520__generated,
            0x0028_7520
        ),
        (
            oc763_fake_287558,
            OC763_PREV_287558,
            ee::s07::OVERRIDE_00287558,
            ee::s07::f_00287558__generated,
            0x0028_7558
        ),
        (
            oc763_fake_2875d0,
            OC763_PREV_2875D0,
            ee::s07::OVERRIDE_002875D0,
            ee::s07::f_002875d0__generated,
            0x0028_75d0
        ),
        (
            oc763_fake_28b180,
            OC763_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc763_fake_317e98,
            OC763_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
    );

    /// `cFEMemCard_readDone` against the generated body: every memory card
    /// type (0, 1, 2, an unknown one), failing and succeeding loads, with and
    /// without the audio object (whose option copies run the real
    /// `MemCardUI_ApplySoundMode` down its short path), a finished read buffer
    /// (state 1, with and without a start buffer) or not. The callees are gated
    /// fakes, so the call log and argument registers are compared too, and some
    /// calls unwind.
    #[test]
    fn mem_card_ui_apply_loaded_data_matches() {
        oc763_install_fakes();
        covered(
            ee::s04::f_001d5f40__generated,
            mem_card_ui_apply_loaded_data,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d_5f40, 20);
                c.bus.write32(OC763_CTRL, OC763_MAGIC);
                let ty = [2u32, 2, 2, 0, 0, 1, 3, 0xffff_ffff, 2, 0, 2, 5][(seed % 12) as usize];
                fill(c, &mut r, OC763_INST, 0x90);
                c.bus.write32(GAME_GP.wrapping_sub(0x10c8), OC763_INST);
                c.bus.write32(OC763_INST + 0x1bc, ty);
                c.bus.write32(OC763_INST + 0x1d0, DATA + 0x2000);
                c.bus.write32(
                    OC763_INST + 0x1d4,
                    if (seed / 3) % 2 == 0 {
                        0
                    } else {
                        DATA + 0x3000
                    },
                );
                c.bus.write32(
                    OC763_INST + 0x228,
                    [0u32, 1, 1, 2][((seed / 2) % 4) as usize],
                );
                // Forced results: the loads fail now and then; the audio
                // object is missing now and then.
                c.bus.write32(
                    oc763_mail(0x0015_2bb0),
                    if seed % 5 == 4 { 1 } else { 0x77 + 3 },
                );
                c.bus.write32(
                    oc763_mail(0x0015_2de0),
                    if seed % 7 == 6 { 1 } else { 0x55 + 3 },
                );
                c.bus.write32(
                    oc763_mail(0x0028_b180),
                    if seed % 6 == 5 { 1 } else { OC763_AUDIO + 3 },
                );
                // `MemCardUI_ApplySoundMode`: audio -> +0x118 -> object ->
                // vtable owner +0x1d8 -> entry (+4) whose method is a fake.
                c.bus.write32(OC763_AUDIO + 0x118, OC763_P1);
                c.bus.write32(OC763_P1, OC763_P2);
                c.bus.write32(OC763_P2 + 0x1d8, OC763_P3);
                c.bus.write32(OC763_P3 + 4, OC763_P4);
                c.bus.write16(OC763_P4 + 0x10, 0);
                c.bus.write32(OC763_P4 + 0x14, fakes::FAKE + 0x60);
                fill(c, &mut r, OC763_CFG, 0xa4);
                c.pc = 0x001d_5f40;
            },
        );
    }

    /// Magic word at `OC770_CTRL`: while a machine holds it, the fakes of this
    /// module's callees are live (otherwise they run the body they replaced).
    pub(crate) const OC770_CTRL: u32 = DATA + 0x30_0000;

    pub(crate) const OC770_MAGIC: u32 = 0x4f43_3770;

    /// Fake-callee machine for this module's fixtures: scrambled registers,
    /// the miss hooks, a game-like gp, the fakes live, and a fake that
    /// unwinds at a random call among the first `calls`.
    pub(crate) fn oc770_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc770_install_fakes();
        let r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC770_CTRL, OC770_MAGIC);
        r
    }

    /// `MemCardStateBase_InstallCallbacks`: the three sizing modes, fake
    /// callees, and unwinds at every call.
    #[test]
    fn mem_card_state_base_install_callbacks_matches() {
        const THIS: u32 = DATA + 0x2_0000;
        check(
            ee::s04::f_001d5488__generated,
            mem_card_state_base_install_callbacks,
            300,
            &|c, seed| {
                let mut r = oc770_machine(c, seed, 0x7704, 19);
                c.pc = 0x001d_5488;
                c.g[A0][0] = THIS as u64;
                c.g[A1][0] = (seed % 4) as u64;
                for off in (0x1a0..0x230).step_by(4) {
                    let w = r.next_u32();
                    c.bus.write32(THIS + off, w);
                }
            },
        );
    }

    /// The word that arms oc784's callee fakes in a machine's RAM.
    pub(crate) const OC784_CTRL: u32 = DATA + 0x30_0000;

    pub(crate) const OC784_MAGIC: u32 = 0x0784_f4ce;

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc784_arm(c: &mut Cpu) {
        c.bus.write32(OC784_CTRL, OC784_MAGIC);
    }

    pub(crate) const OC784_OBJ: u32 = DATA + 0x1_0000;

    pub(crate) const OC784_APP: u32 = DATA + 0x1_8000;

    /// `MemCardUI_BeginLoad`: the load type (0, 1, 2 and others) and the
    /// unwinds of its up to eight calls.
    #[test]
    fn mem_card_ui_begin_load_matches() {
        oc784_install_fakes();
        fakes::covered(
            ee::s04::f_001d5df0__generated,
            mem_card_ui_begin_load,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x784a, 7);
                oc784_arm(c);
                c.bus.write32(GAME_GP - 0x848, OC784_APP);
                c.bus.write32(GAME_GP - 0x10c8, OC784_OBJ);
                let mode = [0u32, 1, 2, 3, 0xffff_ffff][(seed % 5) as usize];
                c.bus.write32(OC784_OBJ + 0x1bc, mode);
                c.bus.write32(OC784_OBJ + 0x1c4, r.next_u32());
                c.bus.write32(OC784_OBJ + 0x1d0, r.next_u32());
            },
        );
    }

    use crate::difftest::elf_image;

    use std::ptr::addr_of_mut;

    /// The fakes' gate word and its magic (each machine of a diff gets its own).
    pub(crate) const OC943_CTRL: u32 = DATA + 0x2e_0000;

    pub(crate) const OC943_MAGIC: u32 = 0x4f43_3934;

    /// Forced results: entry `i` is `{flag, value}`; the fake sign-extends the value into v0.
    pub(crate) const OC943_MBOX: u32 = DATA + 0x2e_0100;

    /// Result table of the `GetHashValue32` fake, indexed by `a0 & 0xffffc`.
    pub(crate) const OC943_HASH: u32 = DATA + 0x60_0000;

    /// The widgets' shared vtable.
    pub(crate) const OC943_VT: u32 = DATA + 0x1_0000;

    macro_rules! oc943_fakes {
        ($install:ident; $(($idx:expr, $fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC943_CTRL) != OC943_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc943_fake_call(c, $idx, $addr);
                }
            )*
            pub(crate) fn $install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    /// A logged fake call, then the forced result of mailbox `idx` (if set) or, for
    /// `GetHashValue32`, the hash table entry of the string address.
    pub(crate) fn oc943_fake_call(c: &mut Cpu, idx: u32, addr: u32) {
        let a0 = c.g[A0][0] as u32;
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        let mb = OC943_MBOX + 8 * idx;
        if c.bus.read32(mb) != 0 {
            c.g[V0][0] = sx(c.bus.read32(mb + 4));
        } else if addr == 0x0031_7670 {
            let h = c.bus.read32(OC943_HASH + (a0 & 0xf_fffc));
            if h != 0 {
                c.g[V0][0] = h as u64;
            }
        }
    }

    /// Force the result of fake `idx` (index in the `oc943_fakes!` table).
    pub(crate) fn oc943_force(c: &mut Cpu, idx: u32, v: u32) {
        c.bus.write32(OC943_MBOX + 8 * idx, 1);
        c.bus.write32(OC943_MBOX + 8 * idx + 4, v);
    }

    pub(crate) const OC943_GET_APP: u32 = 11;

    pub(crate) const OC943_NEW: u32 = 21;

    oc943_fakes!(
        oc943_install_fakes;
        (0, oc943_fake_0014dc80, OC943_PREV_0014DC80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated, 0x0014_dc80),
        (1, oc943_fake_0014dd58, OC943_PREV_0014DD58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated, 0x0014_dd58),
        (2, oc943_fake_0014e048, OC943_PREV_0014E048, ee::s01::OVERRIDE_0014E048, ee::s01::f_0014e048__generated, 0x0014_e048),
        (3, oc943_fake_0014e050, OC943_PREV_0014E050, ee::s01::OVERRIDE_0014E050, ee::s01::f_0014e050__generated, 0x0014_e050),
        (4, oc943_fake_00152950, OC943_PREV_00152950, ee::s01::OVERRIDE_00152950, ee::s01::f_00152950__generated, 0x0015_2950),
        (5, oc943_fake_00152ba8, OC943_PREV_00152BA8, ee::s01::OVERRIDE_00152BA8, ee::s01::f_00152ba8__generated, 0x0015_2ba8),
        (6, oc943_fake_00156a90, OC943_PREV_00156A90, ee::s01::OVERRIDE_00156A90, ee::s01::f_00156a90__generated, 0x0015_6a90),
        (7, oc943_fake_00156ae0, OC943_PREV_00156AE0, ee::s01::OVERRIDE_00156AE0, ee::s01::f_00156ae0__generated, 0x0015_6ae0),
        (8, oc943_fake_001d47a0, OC943_PREV_001D47A0, ee::s04::OVERRIDE_001D47A0, ee::s04::f_001d47a0__generated, 0x001d_47a0),
        (9, oc943_fake_001d9948, OC943_PREV_001D9948, ee::s04::OVERRIDE_001D9948, ee::s04::f_001d9948__generated, 0x001d_9948),
        (10, oc943_fake_001da110, OC943_PREV_001DA110, ee::s04::OVERRIDE_001DA110, ee::s04::f_001da110__generated, 0x001d_a110),
        (11, oc943_fake_00227f80, OC943_PREV_00227F80, ee::s05::OVERRIDE_00227F80, ee::s05::f_00227f80__generated, 0x0022_7f80),
        (12, oc943_fake_00241000, OC943_PREV_00241000, ee::s06::OVERRIDE_00241000, ee::s06::f_00241000__generated, 0x0024_1000),
        (13, oc943_fake_002412a0, OC943_PREV_002412A0, ee::s06::OVERRIDE_002412A0, ee::s06::f_002412a0__generated, 0x0024_12a0),
        (14, oc943_fake_00241aa0, OC943_PREV_00241AA0, ee::s06::OVERRIDE_00241AA0, ee::s06::f_00241aa0__generated, 0x0024_1aa0),
        (15, oc943_fake_00241d40, OC943_PREV_00241D40, ee::s06::OVERRIDE_00241D40, ee::s06::f_00241d40__generated, 0x0024_1d40),
        (16, oc943_fake_00241e18, OC943_PREV_00241E18, ee::s06::OVERRIDE_00241E18, ee::s06::f_00241e18__generated, 0x0024_1e18),
        (17, oc943_fake_00270e68, OC943_PREV_00270E68, ee::s07::OVERRIDE_00270E68, ee::s07::f_00270e68__generated, 0x0027_0e68),
        (18, oc943_fake_002c2540, OC943_PREV_002C2540, ee::s08::OVERRIDE_002C2540, ee::s08::f_002c2540__generated, 0x002c_2540),
        (19, oc943_fake_002c26d0, OC943_PREV_002C26D0, ee::s08::OVERRIDE_002C26D0, ee::s08::f_002c26d0__generated, 0x002c_26d0),
        (20, oc943_fake_00317670, OC943_PREV_00317670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x0031_7670),
        (21, oc943_fake_00317e30, OC943_PREV_00317E30, ee::s10::OVERRIDE_00317E30, ee::s10::f_00317e30__generated, 0x0031_7e30),
        (22, oc943_fake_00317e98, OC943_PREV_00317E98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated, 0x0031_7e98),
        (23, oc943_fake_00397cc0, OC943_PREV_00397CC0, ee::s13::OVERRIDE_00397CC0, ee::s13::f_00397cc0__generated, 0x0039_7cc0),
        (24, oc943_fake_00398380, OC943_PREV_00398380, ee::s13::OVERRIDE_00398380, ee::s13::f_00398380__generated, 0x0039_8380),
        (25, oc943_fake_0039c870, OC943_PREV_0039C870, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated, 0x0039_c870),
        (26, oc943_fake_003a0c50, OC943_PREV_003A0C50, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated, 0x003a_0c50),
        (27, oc943_fake_003a1000, OC943_PREV_003A1000, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated, 0x003a_1000),
        (28, oc943_fake_00144c60, OC943_PREV_00144C60, ee::s01::OVERRIDE_00144C60, ee::s01::f_00144c60__generated, 0x0014_4c60),
        (29, oc943_fake_001509c0, OC943_PREV_001509C0, ee::s01::OVERRIDE_001509C0, ee::s01::f_001509c0__generated, 0x0015_09c0),
        (30, oc943_fake_00153390, OC943_PREV_00153390, ee::s01::OVERRIDE_00153390, ee::s01::f_00153390__generated, 0x0015_3390),
        (31, oc943_fake_00153688, OC943_PREV_00153688, ee::s01::OVERRIDE_00153688, ee::s01::f_00153688__generated, 0x0015_3688),
        (32, oc943_fake_00154368, OC943_PREV_00154368, ee::s01::OVERRIDE_00154368, ee::s01::f_00154368__generated, 0x0015_4368),
        (33, oc943_fake_001544d0, OC943_PREV_001544D0, ee::s01::OVERRIDE_001544D0, ee::s01::f_001544d0__generated, 0x0015_44d0),
        (34, oc943_fake_00158960, OC943_PREV_00158960, ee::s01::OVERRIDE_00158960, ee::s01::f_00158960__generated, 0x0015_8960),
        (35, oc943_fake_00198af0, OC943_PREV_00198AF0, ee::s03::OVERRIDE_00198AF0, ee::s03::f_00198af0__generated, 0x0019_8af0),
        (36, oc943_fake_002c27c0, OC943_PREV_002C27C0, ee::s08::OVERRIDE_002C27C0, ee::s08::f_002c27c0__generated, 0x002c_27c0),
        (37, oc943_fake_003a0e90, OC943_PREV_003A0E90, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated, 0x003a_0e90),
        (38, oc943_fake_00413af8, OC943_PREV_00413AF8, ee::s15::OVERRIDE_00413AF8, ee::s15::f_00413af8__generated, 0x0041_3af8),
        (39, oc943_fake_00417828, OC943_PREV_00417828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated, 0x0041_7828),
        (40, oc943_fake_001589b0, OC943_PREV_001589B0, ee::s01::OVERRIDE_001589B0, ee::s01::f_001589b0__generated, 0x0015_89b0),
        (41, oc943_fake_00158a50, OC943_PREV_00158A50, ee::s01::OVERRIDE_00158A50, ee::s01::f_00158a50__generated, 0x0015_8a50),
        (42, oc943_fake_00158910, OC943_PREV_00158910, ee::s01::OVERRIDE_00158910, ee::s01::f_00158910__generated, 0x0015_8910),
        (43, oc943_fake_00158af0, OC943_PREV_00158AF0, ee::s01::OVERRIDE_00158AF0, ee::s01::f_00158af0__generated, 0x0015_8af0),
    );

    /// Gate open, the widgets' vtable (`+0x40`, `+0x48` and `+0x120` slots are fake calls with
    /// a this-adjust of 0, 0x10 or 0x20).
    pub(crate) fn oc943_open(c: &mut Cpu, seed: u32) {
        c.bus.write32(OC943_CTRL, OC943_MAGIC);
        for (k, off) in [0x40u32, 0x48, 0x120].into_iter().enumerate() {
            c.bus.write16(OC943_VT + off, 0x10 * (seed % 3) as u16);
            c.bus.write32(OC943_VT + off + 4, FAKE + 0x10 * k as u32);
        }
    }

    /// A widget at `at` with the shared vtable.
    pub(crate) fn oc943_widget(c: &mut Cpu, at: u32) {
        c.bus.write32(at + 8, OC943_VT);
    }

    /// Hash table value of the string at `addr`.
    pub(crate) fn oc943_set_hash(c: &mut Cpu, addr: u32, h: u32) {
        c.bus.write32(OC943_HASH + (addr & 0xf_fffc), h);
    }

    /// `OnWidgetCreated(self, w)`: the widget's name hash picks the trophy image, the hidden
    /// button, the title text by peak, or one of the six challenge names / boxes (shown or
    /// hidden by the peak's entry count; entry 0 reads the peak name from the reward table).
    #[test]
    fn c_fe_state_trophy_room_on_widget_created_matches() {
        oc943_install_fakes();
        fakes::covered(
            ee::s04::f_001d43f0__generated,
            c_fe_state_trophy_room_on_widget_created,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d43f0, 14);
                elf_image(c);
                oc943_open(c, seed);
                let this = DATA + 0x1000;
                let w = DATA + 0x3000;
                oc943_widget(c, w);
                // Strings of the loop tables (the tables are in the image: rewrite
                // them to known pointers) and their hashes.
                for i in 0..6u32 {
                    c.bus.write32(0x0044_1600 + 4 * i, 0x004a_3000 + 0x10 * i);
                    c.bus.write32(0x0044_1618 + 4 * i, 0x004a_3100 + 0x10 * i);
                    oc943_set_hash(c, 0x004a_3000 + 0x10 * i, 0x4000 + i);
                    oc943_set_hash(c, 0x004a_3100 + 0x10 * i, 0x5000 + i);
                }
                oc943_set_hash(c, 0x004a_1fe8, 0x1111);
                oc943_set_hash(c, 0x004a_1ff0, 0x2222);
                oc943_set_hash(c, 0x0046_6e60, 0x3333);
                let kind = seed % 20;
                let h = match kind {
                    0 => 0x1111,
                    1 => 0x2222,
                    2..=4 => 0x3333,
                    5..=10 => 0x4000 + (kind - 5),
                    11..=16 => 0x5000 + (kind - 11),
                    _ => 0x7777,
                };
                c.bus.write32(w + 0x38, h);
                c.bus.write32(this + 0x54, (seed / 7) % 3);
                c.bus.write32(this + 0x58, (seed / 20) % 5);
                c.bus.write32(this + 0x9c, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(w);
                c.pc = 0x001d_43f0;
            },
        );
    }

    /// `WriteSaveData()`: settings write (mode 2), replay write (mode 1: data size, file name,
    /// product code, formatted title) or profile write (other modes); then the "saving"
    /// popup unless a status popup is up or autosave is on, freeing the message buffer when
    /// it was allocated.
    #[test]
    fn mem_card_ui_write_save_data_matches() {
        oc943_install_fakes();
        fakes::covered(
            ee::s04::f_001d59a0__generated,
            mem_card_ui_write_save_data,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d59a0, 24);
                oc943_open(c, seed);
                let inst = DATA + 0x1000;
                let app = DATA + 0x4000;
                let world = DATA + 0x5000;
                c.bus.write32(GAME_GP.wrapping_sub(0x10c8), inst);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                fill(c, &mut r, world + 0x80, 8);
                c.bus.write32(world + 0x84, DATA + 0x6000);
                fill(c, &mut r, DATA + 0x6000 + 0x20, 4);
                c.bus
                    .write32(inst + 0x1bc, [2, 1, 0, 3][(seed % 4) as usize]);
                c.bus.write32(inst + 0x1d4, r.next_u32());
                c.bus.write32(inst + 0x1cc, r.next_u32());
                c.bus.write32(inst + 0x208, (seed / 4) % 2);
                c.bus.write32(inst + 0x210, (seed / 8) % 2);
                oc943_widget(c, inst);
                c.bus.write32(app + 0xf8, r.next_u32());
                oc943_force(c, OC943_GET_APP, app);
                oc943_force(
                    c,
                    OC943_NEW,
                    if (seed / 16) % 2 == 0 {
                        0
                    } else {
                        DATA + 0x7000
                    },
                );
                c.pc = 0x001d_59a0;
            },
        );
    }

    /// The gate word of this module's fakes (in the machine's own RAM), its
    /// magic, and the forced results (one word per fake call: 0 keeps the
    /// fake's own value, otherwise the result is `word - 1`).
    pub(crate) const OC945_GATE: u32 = DATA + 0x1f_f000;

    pub(crate) const OC945_MAGIC: u32 = 0x0945_0a11;

    pub(crate) const OC945_RESULTS: u32 = OC945_GATE + 4;

    pub(crate) const OC945_CALLS: u32 = 16;

    /// Gated fake: logs the call like `fakes::fake`, then returns the forced
    /// result of this call index; with the gate off, runs the body it replaced.
    pub(crate) fn oc945_gated(c: &mut Cpu, target: u32, prev: &AtomicUsize, generated: GuestFn) {
        if c.bus.read32(OC945_GATE) != OC945_MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        let call = c.bus.read32(fakes::LOG_COUNT) - 1;
        if call < OC945_CALLS && c.pc == ra {
            let word = c.bus.read32(OC945_RESULTS + 4 * call);
            if word != 0 {
                c.g[V0][0] = (word - 1) as u64;
            }
        }
    }

    /// Fakes for every callee of this module's replacements.
    macro_rules! oc945_fakes {
        ($( $name:ident => $target:expr, $slot:path, $generated:path; )*) => {
            $(
                pub(crate) fn $name(c: &mut Cpu) {
                    oc945_gated(c, $target, fakes::prev_cell($name), $generated)
                }
            )*

            /// Hooks the fakes into the override slots of their callees (once per process).
            pub(crate) fn oc945_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc945_fakes! {
        oc945_fake_14dc80 => 0x0014_dc80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        oc945_fake_14dd58 => 0x0014_dd58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        oc945_fake_157bf0 => 0x0015_7bf0, ee::s01::OVERRIDE_00157BF0, ee::s01::f_00157bf0__generated;
        oc945_fake_147488 => 0x0014_7488, ee::s01::OVERRIDE_00147488, ee::s01::f_00147488__generated;
        oc945_fake_156a90 => 0x0015_6a90, ee::s01::OVERRIDE_00156A90, ee::s01::f_00156a90__generated;
        oc945_fake_156ae0 => 0x0015_6ae0, ee::s01::OVERRIDE_00156AE0, ee::s01::f_00156ae0__generated;
        oc945_fake_39f400 => 0x0039_f400, ee::s13::OVERRIDE_0039F400, ee::s13::f_0039f400__generated;
        oc945_fake_3a1000 => 0x003a_1000, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated;
        oc945_fake_39e2a0 => 0x0039_e2a0, ee::s13::OVERRIDE_0039E2A0, ee::s13::f_0039e2a0__generated;
        oc945_fake_39e4c0 => 0x0039_e4c0, ee::s13::OVERRIDE_0039E4C0, ee::s13::f_0039e4c0__generated;
        oc945_fake_317670 => 0x0031_7670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated;
        oc945_fake_19da10 => 0x0019_da10, ee::s03::OVERRIDE_0019DA10, ee::s03::f_0019da10__generated;
        oc945_fake_19dc20 => 0x0019_dc20, ee::s03::OVERRIDE_0019DC20, ee::s03::f_0019dc20__generated;
        oc945_fake_19e238 => 0x0019_e238, ee::s03::OVERRIDE_0019E238, ee::s03::f_0019e238__generated;
        oc945_fake_19e2b0 => 0x0019_e2b0, ee::s03::OVERRIDE_0019E2B0, ee::s03::f_0019e2b0__generated;
        oc945_fake_1a1cd0 => 0x001a_1cd0, ee::s03::OVERRIDE_001A1CD0, ee::s03::f_001a1cd0__generated;
        oc945_fake_1d3ec8 => 0x001d_3ec8, ee::s04::OVERRIDE_001D3EC8, ee::s04::f_001d3ec8__generated;
        oc945_fake_1d3f80 => 0x001d_3f80, ee::s04::OVERRIDE_001D3F80, ee::s04::f_001d3f80__generated;
        oc945_fake_1d47a0 => 0x001d_47a0, ee::s04::OVERRIDE_001D47A0, ee::s04::f_001d47a0__generated;
        oc945_fake_1d4918 => 0x001d_4918, ee::s04::OVERRIDE_001D4918, ee::s04::f_001d4918__generated;
        oc945_fake_1d4f90 => 0x001d_4f90, ee::s04::OVERRIDE_001D4F90, ee::s04::f_001d4f90__generated;
    }

    /// The machine for one seed: scrambled registers, the gate set, no forced
    /// results (every fake returns its own value).
    pub(crate) fn oc945_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        oc945_install_fakes();
        let r = fakes::machine(c, seed, salt, 0);
        c.bus.write32(OC945_GATE, OC945_MAGIC);
        for k in 0..OC945_CALLS {
            c.bus.write32(OC945_RESULTS + 4 * k, 0);
        }
        r
    }

    /// Forces the result of the `k`-th fake call to `value`.
    pub(crate) fn oc945_force(c: &mut Cpu, k: u32, value: u32) {
        c.bus.write32(OC945_RESULTS + 4 * k, value.wrapping_add(1));
    }

    /// Forces the results of a few random calls to zero.
    pub(crate) fn oc945_zeros(c: &mut Cpu, r: &mut Rng) {
        for k in 0..OC945_CALLS {
            if r.next_u32() % 4 == 0 {
                oc945_force(c, k, 0);
            }
        }
    }

    #[test]
    fn cfe_state_trophy_room_on_activate_matches() {
        check(
            ee::s04::f_001d43d0__generated,
            cfe_state_trophy_room_on_activate,
            200,
            &|c, seed| {
                let _ = oc945_machine(c, seed, 0x43d0);
                c.g[A0][0] = sx(DATA + 0xa_e000);
                c.pc = 0x001d_43d0;
            },
        );
    }

    #[test]
    fn cfe_state_trophy_room_show_trophy_image_matches() {
        check(
            ee::s04::f_001d47a0__generated,
            cfe_state_trophy_room_show_trophy_image,
            300,
            &|c, seed| {
                let mut r = oc945_machine(c, seed, 0x47a0);
                let obj = DATA + 0xb_0000;
                let item = DATA + 0xb_1000;
                let vt = DATA + 0xb_2000;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = [u64::MAX, 0, 3, 1][(seed % 4) as usize];
                c.pc = 0x001d_47a0;
                c.bus.write8(obj + 0x44, r.next_u32() as u8);
                c.bus.write32(obj + 0x48, item);
                c.bus.write32(item + 8, vt);
                c.bus.write16(vt + 0x48, 8);
                c.bus.write32(vt + 0x4c, 0x0019_dc20);
                c.bus.write32(obj + 0x50, r.next_u32());
                c.bus.write32(obj + 0x54, r.next_u32());
                c.bus.write32(obj + 0x58, r.next_u32());
                c.bus.write32(obj + 0x60, r.next_u32() % 4);
                oc945_zeros(c, &mut r);
                // QueryEventReward (call 0) returns a small slot index.
                oc945_force(c, 0, r.next_u32() % 4);
            },
        );
    }

    #[test]
    fn cfe_state_trophy_room_request_trophy_images_matches() {
        check(
            ee::s04::f_001d4918__generated,
            cfe_state_trophy_room_request_trophy_images,
            300,
            &|c, seed| {
                let mut r = oc945_machine(c, seed, 0x4918);
                let obj = DATA + 0xb_3000;
                let world = DATA + 0xb_4000;
                let rec = DATA + 0xb_5000;
                c.g[A0][0] = sx(obj);
                c.pc = 0x001d_4918;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x7c, (seed % 4) * 0x10);
                c.bus.write32(obj + 0x54, r.next_u32());
                c.bus.write32(obj + 0x58, r.next_u32() % 6);
                c.bus.write32(obj + 0x50, r.next_u32());
                c.bus
                    .write32(rec + 4, if seed % 3 == 0 { 0 } else { DATA + 0xb_6000 });
                // The record pointer comes back from the table lookups (not
                // zeroed: those results are dereferenced).
                oc945_force(c, 2, rec);
                for k in (4..16).step_by(2) {
                    oc945_force(c, k, rec);
                }
            },
        );
    }

    #[test]
    fn cfe_state_trophy_room_load_trophy_textures_matches() {
        check(
            ee::s04::f_001d4a20__generated,
            cfe_state_trophy_room_load_trophy_textures,
            300,
            &|c, seed| {
                let mut r = oc945_machine(c, seed, 0x4a20);
                let obj = DATA + 0xb_7000;
                let world = DATA + 0xb_8000;
                let world2 = DATA + 0xb_9000;
                let table = DATA + 0xb_a000;
                c.g[A0][0] = sx(obj);
                c.pc = 0x001d_4a20;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x7c, (seed % 3) * 0x10);
                c.bus.write32(GAME_GP - 0x854, world2);
                c.bus.write32(world2 + 0x10d8, table);
                c.bus.write16(table + 0x170, 0x10);
                c.bus.write32(table + 0x174, 0x0019_dc20);
                for i in 0..5 {
                    let entry = if (seed >> i) & 1 == 0 { i } else { u32::MAX };
                    c.bus.write32(obj + 0x74 + 4 * i, entry);
                }
                c.bus.write32(obj + 0x5c, 2);
                c.bus.write32(obj + 0x50, r.next_u32());
                c.bus.write32(obj + 0x9c, r.next_u32());
                oc945_zeros(c, &mut r);
            },
        );
    }

    #[test]
    fn cfe_state_rewards_room_on_widget_created_matches() {
        check(
            ee::s04::f_001d4cd8__generated,
            cfe_state_rewards_room_on_widget_created,
            300,
            &|c, seed| {
                let mut r = oc945_machine(c, seed, 0x4cd8);
                let obj = DATA + 0xb_d000;
                let widget = DATA + 0xb_c000;
                let hash = 0x5a5a_0000 + seed;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(widget);
                c.pc = 0x001d_4cd8;
                c.bus.write32(widget + 0x38, hash);
                oc945_zeros(c, &mut r);
                // The hash of the matching string (call `m`) is `hash`.
                let m = seed % 13;
                if m < 12 {
                    oc945_force(c, m, hash);
                }
            },
        );
    }

    /// Gate of the callee fakes: they answer only while this word holds
    /// `OC946_MAGIC` in the machine's RAM (otherwise the generated body runs).
    pub(crate) const OC946_CTRL: u32 = DATA + 0x0060_0000;

    pub(crate) const OC946_MAGIC: u32 = 0x0946_c0de;

    /// Forced results of the fakes that must return a chosen value (word `k`
    /// at `OC946_MBX + 4k`: 0 getDataSize, 1 getSaveData, 2 USTR_length).
    pub(crate) const OC946_MBX: u32 = DATA + 0x0060_1000;

    /// Fixture objects (self, LobbyMan, SSXApp, the card, the buffers), each
    /// within 0x4000 of this base.
    pub(crate) const OC946_OBJ: u32 = DATA + 0x0060_2000;

    /// Scrambled machine with the callee gate open.
    pub(crate) fn oc946_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = fakes::machine(c, seed, salt, 3);
        c.bus.write32(OC946_CTRL, OC946_MAGIC);
        r
    }

    /// A callee fake: logs the call (`fake_call`) and returns the generated
    /// body when the gate is closed. `$mbx` names a forced-result word.
    macro_rules! oc946_fake {
        ($name:ident, $target:expr, $gen:path) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(OC946_CTRL) != OC946_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                fakes::fake_call(c, $target);
            }
        };
        ($name:ident, $target:expr, $gen:path, mbx $slot:expr) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(OC946_CTRL) != OC946_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                fakes::fake_call(c, $target);
                c.g[V0][0] = c.bus.read32(OC946_MBX + 4 * $slot) as u64;
            }
        };
    }

    oc946_fake!(
        oc946_fake_25c610,
        0x0025_c610,
        ee::s06::f_0025c610__generated
    );

    oc946_fake!(
        oc946_fake_25cd50,
        0x0025_cd50,
        ee::s06::f_0025cd50__generated
    );

    oc946_fake!(
        oc946_fake_25fd50,
        0x0025_fd50,
        ee::s06::f_0025fd50__generated
    );

    oc946_fake!(
        oc946_fake_1a8a40,
        0x001a_8a40,
        ee::s03::f_001a8a40__generated
    );

    oc946_fake!(
        oc946_fake_1bfbb8,
        0x001b_fbb8,
        ee::s03::f_001bfbb8__generated
    );

    oc946_fake!(
        oc946_fake_1bf228,
        0x001b_f228,
        ee::s03::f_001bf228__generated
    );

    oc946_fake!(
        oc946_fake_317d70,
        0x0031_7d70,
        ee::s10::f_00317d70__generated
    );

    oc946_fake!(
        oc946_fake_14dd58,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated
    );

    oc946_fake!(
        oc946_fake_227f80,
        0x0022_7f80,
        ee::s05::f_00227f80__generated
    );

    oc946_fake!(
        oc946_fake_14dc80,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated
    );

    oc946_fake!(
        oc946_fake_14e048,
        0x0014_e048,
        ee::s01::f_0014e048__generated,
        mbx 0
    );

    oc946_fake!(
        oc946_fake_152728,
        0x0015_2728,
        ee::s01::f_00152728__generated
    );

    oc946_fake!(
        oc946_fake_14e0c0,
        0x0014_e0c0,
        ee::s01::f_0014e0c0__generated
    );

    oc946_fake!(
        oc946_fake_241200,
        0x0024_1200,
        ee::s06::f_00241200__generated
    );

    oc946_fake!(
        oc946_fake_2710a8,
        0x0027_10a8,
        ee::s07::f_002710a8__generated,
        mbx 1
    );

    oc946_fake!(
        oc946_fake_241138,
        0x0024_1138,
        ee::s06::f_00241138__generated
    );

    oc946_fake!(oc946_fake_3dbca0, 0x003d_bca0, ee::s14::f_003dbca0__generated, mbx 2);

    oc946_fake!(
        oc946_fake_317e30,
        0x0031_7e30,
        ee::s10::f_00317e30__generated
    );

    oc946_fake!(
        oc946_fake_2c2688,
        0x002c_2688,
        ee::s08::f_002c2688__generated
    );

    oc946_fake!(
        oc946_fake_3a0d00,
        0x003a_0d00,
        ee::s13::f_003a0d00__generated
    );

    oc946_fake!(
        oc946_fake_317e98,
        0x0031_7e98,
        ee::s10::f_00317e98__generated
    );

    oc946_fake!(
        oc946_fake_2caa58,
        0x002c_aa58,
        ee::s09::f_002caa58__generated
    );

    oc946_fake!(
        oc946_fake_2ce418,
        0x002c_e418,
        ee::s09::f_002ce418__generated
    );

    oc946_fake!(
        oc946_fake_2ccf98,
        0x002c_cf98,
        ee::s09::f_002ccf98__generated
    );

    oc946_fake!(
        oc946_fake_2ccdf0,
        0x002c_cdf0,
        ee::s09::f_002ccdf0__generated
    );

    oc946_fake!(
        oc946_fake_2cbcc8,
        0x002c_bcc8,
        ee::s09::f_002cbcc8__generated
    );

    oc946_fake!(
        oc946_fake_416958,
        0x0041_6958,
        ee::s15::f_00416958__generated
    );

    /// Hooks every callee fake once per process.
    pub(crate) fn oc946_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025C610),
            oc946_fake_25c610,
            fakes::prev_cell(oc946_fake_25c610),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025CD50),
            oc946_fake_25cd50,
            fakes::prev_cell(oc946_fake_25cd50),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025FD50),
            oc946_fake_25fd50,
            fakes::prev_cell(oc946_fake_25fd50),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A8A40),
            oc946_fake_1a8a40,
            fakes::prev_cell(oc946_fake_1a8a40),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001BFBB8),
            oc946_fake_1bfbb8,
            fakes::prev_cell(oc946_fake_1bfbb8),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001BF228),
            oc946_fake_1bf228,
            fakes::prev_cell(oc946_fake_1bf228),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc946_fake_317d70,
            fakes::prev_cell(oc946_fake_317d70),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc946_fake_14dd58,
            fakes::prev_cell(oc946_fake_14dd58),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00227F80),
            oc946_fake_227f80,
            fakes::prev_cell(oc946_fake_227f80),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc946_fake_14dc80,
            fakes::prev_cell(oc946_fake_14dc80),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014E048),
            oc946_fake_14e048,
            fakes::prev_cell(oc946_fake_14e048),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00152728),
            oc946_fake_152728,
            fakes::prev_cell(oc946_fake_152728),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014E0C0),
            oc946_fake_14e0c0,
            fakes::prev_cell(oc946_fake_14e0c0),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00241200),
            oc946_fake_241200,
            fakes::prev_cell(oc946_fake_241200),
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002710A8),
            oc946_fake_2710a8,
            fakes::prev_cell(oc946_fake_2710a8),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00241138),
            oc946_fake_241138,
            fakes::prev_cell(oc946_fake_241138),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DBCA0),
            oc946_fake_3dbca0,
            fakes::prev_cell(oc946_fake_3dbca0),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc946_fake_317e30,
            fakes::prev_cell(oc946_fake_317e30),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2688),
            oc946_fake_2c2688,
            fakes::prev_cell(oc946_fake_2c2688),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0D00),
            oc946_fake_3a0d00,
            fakes::prev_cell(oc946_fake_3a0d00),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc946_fake_317e98,
            fakes::prev_cell(oc946_fake_317e98),
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CAA58),
            oc946_fake_2caa58,
            fakes::prev_cell(oc946_fake_2caa58),
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CE418),
            oc946_fake_2ce418,
            fakes::prev_cell(oc946_fake_2ce418),
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CCF98),
            oc946_fake_2ccf98,
            fakes::prev_cell(oc946_fake_2ccf98),
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CCDF0),
            oc946_fake_2ccdf0,
            fakes::prev_cell(oc946_fake_2ccdf0),
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CBCC8),
            oc946_fake_2cbcc8,
            fakes::prev_cell(oc946_fake_2cbcc8),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00416958),
            oc946_fake_416958,
            fakes::prev_cell(oc946_fake_416958),
        );
    }

    #[test]
    fn c_fe_state_relogin_v31_matches() {
        oc946_install_fakes();
        check(
            ee::s04::f_001d5138__generated,
            c_fe_state_relogin_v31,
            200,
            &|c, seed| {
                let mut r = oc946_setup(c, seed, 0x5138);
                let msgs: [u64; 8] = [0xc9, 0xd4, 0x101, 0x111, 0x7, 0xd5, 0x200, 0xcb];
                c.g[A0][0] = sx(OC946_OBJ);
                c.g[A1][0] = msgs[(seed % 8) as usize];
                let lobby_slot = oc946_gp(c, 0xc8);
                let ssx_slot = oc946_gp(c, 0x848);
                c.bus.write32(lobby_slot, OC946_OBJ + 0x400);
                c.bus.write32(ssx_slot, OC946_OBJ + 0x800);
                c.bus.write32(OC946_OBJ + 0x800 + 0x78, OC946_OBJ + 0xc00);
                c.bus.write32(OC946_OBJ + 0x400 + 0x80, r.next_u32());
                c.bus.write32(OC946_OBJ + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_card_ui_read_save_chunk_matches() {
        oc946_install_fakes();
        check(
            ee::s04::f_001d5c28__generated,
            mem_card_ui_read_save_chunk,
            200,
            &|c, seed| {
                let mut r = oc946_setup(c, seed, 0x5c28);
                let sizes: [u64; 6] = [0, u64::MAX, 5, 0x4000, 0x5000, 0x10];
                c.g[A0][0] = sizes[(seed % 6) as usize];
                let card_slot = oc946_gp(c, 0x10c8);
                let ssx_slot = oc946_gp(c, 0x848);
                let card = OC946_OBJ + 0x3000;
                c.bus.write32(card_slot, card);
                c.bus.write32(ssx_slot, OC946_OBJ + 0x800);
                c.bus
                    .write32(card + 0x1bc, if seed % 4 == 0 { 2 } else { 1 });
                c.bus
                    .write32(card + 0x200, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(card + 0x1d0, OC946_OBJ + 0x3400);
                c.bus.write32(OC946_MBX, 8 + seed % 56);
                for i in 0..16 {
                    c.bus.write32(OC946_OBJ + 0x3400 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn mem_card_ui_write_replay_save_block_matches() {
        oc946_install_fakes();
        check(
            ee::s04::f_001d5d78__generated,
            mem_card_ui_write_replay_save_block,
            200,
            &|c, seed| {
                let mut r = oc946_setup(c, seed, 0x5d78);
                c.g[A0][0] = r.next_u32() as u64;
                let card_slot = oc946_gp(c, 0x10c8);
                let ssx_slot = oc946_gp(c, 0x848);
                let card = OC946_OBJ + 0x3000;
                c.bus.write32(card_slot, card);
                c.bus.write32(ssx_slot, OC946_OBJ + 0x800);
                c.bus
                    .write32(card + 0x1bc, if seed % 4 == 0 { 2 } else { 1 });
                c.bus.write32(OC946_OBJ + 0x800 + 0x84, OC946_OBJ + 0x3800);
                c.bus.write32(OC946_OBJ + 0x3800 + 0x28, OC946_OBJ + 0x3c00);
                c.bus
                    .write32(OC946_MBX + 4, if seed % 3 == 0 { 0 } else { 1 });
            },
        );
    }

    // -----------------------------------------------------------------------
    // Kept tests for main's three save-flow drivers (`_r2`): the branch's
    // path-pin and unwind tests, pointed at main's bodies (all pass, so
    // main's ports match generated on these fixtures).

    use crate::game::fe::fememcardcallbacks::tests::memcardui_install_fakes as probe_install_fakes;
    use crate::game::fe::fememcardcallbacks::tests::memcardui_read_save_chunk_setup as probe_read_setup;
    use crate::game::fe::fememcardcallbacks::tests::memcardui_write_save_data_setup as probe_write_setup;

    #[test]
    fn mem_card_ui_write_save_data_r2_matches() {
        check_paths(
            ee::s04::f_001d59a0__generated,
            mem_card_ui_write_save_data,
            64,
            &[
                43, 45, 47, 49, 51, 53, 91, 103, 104, 114, 127, 129, 135, 145, 147, 151, 153, 156,
                157, 163, 165, 181, 195, 196, 206, 211, 225,
            ],
            &|c, seed| {
                probe_install_fakes();
                probe_write_setup(c, seed, u32::MAX);
            },
        );
    }

    #[test]
    fn mem_card_ui_write_save_data_r2_unwind_matches() {
        probe_install_fakes();
        crate::difftest::fakes::covered(
            ee::s04::f_001d59a0__generated,
            mem_card_ui_write_save_data,
            64,
            &|c, seed| probe_write_setup(c, seed, 0),
        );
        // Both sides return early with the frame still open: seed 0 (mode 2)
        // and seed 7 (profile arm) both unwind at the first fake call.
        for seed in [0u32, 7] {
            let mut a = crate::difftest::fresh();
            let mut b = crate::difftest::fresh();
            probe_write_setup(&mut a, seed, 0);
            probe_write_setup(&mut b, seed, 0);
            ee::s04::f_001d59a0__generated(&mut a);
            mem_card_ui_write_save_data(&mut b);
            for (side, c) in [("generated", &a), ("hand", &b)] {
                assert_eq!(c.pc, 0xdead_0000, "{side} seed {seed}");
                assert_eq!(
                    c.g[SP][0],
                    (crate::difftest::SP - 0x7a0) as u64,
                    "{side} seed {seed}"
                );
            }
        }
    }

    #[test]
    fn mem_card_ui_read_save_chunk_r2_matches() {
        check_paths(
            ee::s04::f_001d5c28__generated,
            mem_card_ui_read_save_chunk,
            64,
            &[
                18, 24, 46, 54, 104, 105, 107, 118, 120, 122, 126, 133, 135, 201, 207, 209, 217,
                220,
            ],
            &|c, seed| {
                probe_install_fakes();
                probe_read_setup(c, seed, u32::MAX);
            },
        );
    }

    #[test]
    fn mem_card_ui_read_save_chunk_r2_unwind_matches() {
        probe_install_fakes();
        crate::difftest::fakes::covered(
            ee::s04::f_001d5c28__generated,
            mem_card_ui_read_save_chunk,
            64,
            &|c, seed| probe_read_setup(c, seed, 0),
        );
        // Both sides return early with the frame still open: seeds 0 and 1
        // run mode 1 with a pending read, unwinding at the first fake call.
        for seed in [0u32, 1] {
            let mut a = crate::difftest::fresh();
            let mut b = crate::difftest::fresh();
            probe_read_setup(&mut a, seed, 0);
            probe_read_setup(&mut b, seed, 0);
            ee::s04::f_001d5c28__generated(&mut a);
            mem_card_ui_read_save_chunk(&mut b);
            for (side, c) in [("generated", &a), ("hand", &b)] {
                assert_eq!(c.pc, 0xdead_0000, "{side} seed {seed}");
                assert_eq!(
                    c.g[SP][0],
                    (crate::difftest::SP - 0x60) as u64,
                    "{side} seed {seed}"
                );
            }
        }
    }

    #[test]
    fn mem_card_ui_write_replay_save_block_r2_matches() {
        use crate::game::fe::fememcardcallbacks::tests::memcardui_fe_world as probe_fe_world;
        use crate::game::fe::fememcardcallbacks::tests::MEMCARDUI_APP as PROBE_APP;
        use crate::game::fe::fememcardcallbacks::tests::MEMCARDUI_FE as PROBE_FE;
        use crate::game::fe::fememcardcallbacks::tests::MEMCARDUI_GATE as PROBE_GATE;
        use crate::game::fe::fememcardcallbacks::tests::MEMCARDUI_MAGIC as PROBE_MAGIC;
        use crate::game::fe::fememcardcallbacks::tests::MEMCARDUI_WORLD as PROBE_WORLD;
        use crate::game::fe::fememcardcallbacks::tests::MEMCARD_UI_SAVE_OP as PROBE_SAVE_OP;
        use crate::game::fe::fememcardcallbacks::tests::R_MC_MANAGER_GET as PROBE_MGR;
        use crate::game::fe::fememcardcallbacks::tests::R_REPLAY_WRITE_SAVE_BLOCK as PROBE_BLOCK;
        check_paths(
            ee::s04::f_001d5d78__generated,
            mem_card_ui_write_replay_save_block,
            24,
            &[13, 32, 39, 40, 41, 42],
            &|c, seed| {
                probe_install_fakes();
                let mut r = machine(c, seed, 0x1d5d_78, 0);
                let mode = [1u64, 0, 2][seed as usize % 3];
                probe_fe_world(c, &mut r, PROBE_FE, PROBE_WORLD);
                c.bus.write32(PROBE_FE + PROBE_SAVE_OP, mode as u32);
                c.bus.write32(PROBE_GATE, PROBE_MAGIC);
                c.bus.write32(PROBE_MGR, PROBE_APP);
                c.bus
                    .write32(PROBE_BLOCK, if seed / 4 % 2 == 0 { 0 } else { 0x77 });
            },
        );
    }
}
