//! `src/fe/festatetrophyroom.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
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
}
