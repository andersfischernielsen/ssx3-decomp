//! `src/fe/ovstatepause.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::dirtysock::tagsunk::gp_5 as gp;
pub(crate) use crate::game::dirtysock::tagsunk::p_2;
pub(crate) use crate::game::dirtysock::tagsunk::sw;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw_2;
pub(crate) use crate::game::fe::fepopup::at as at_3;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_6;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_3;
pub(crate) use crate::game::fe::festateoptions::gpa;
pub(crate) use crate::game::fe::festaterewards::gp_4;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::ovstateprofile::ov_screen_set_two_visible;
pub(crate) use crate::game::fe::ovstateprofile::ov_screen_set_widget_visibility;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_3;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001f_aa78,
        ov_screen_be_options_on_widget_notify,
        "OVScreenBEOptions_OnWidgetNotify",
    ),
    (
        0x001f_aaf0,
        ov_screen_be_options_v22,
        "OVScreenBEOptions_v22",
    ),
    (
        0x001f_ab08,
        ov_screen_be_options_v21,
        "OVScreenBEOptions_v21",
    ),
    (
        0x001f_af10,
        ovscreen_in_game_error_update,
        "OVScreenInGameError_Update",
    ),
    (
        0x001f_aff8,
        ov_screen_in_game_error_show_cancel_game,
        "OVScreenInGameError_ShowCancelGame",
    ),
    (
        0x001f_b180,
        ov_screen_in_game_error_update_server_timeout_text,
        "OVScreenInGameError_UpdateServerTimeoutText",
    ),
    (
        0x001f_b218,
        ov_screen_in_game_error_update_opponent_disconnected_text,
        "OVScreenInGameError_UpdateOpponentDisconnectedText",
    ),
    (
        0x001f_b378,
        ovscreeningameerror_show_continue_button,
        "OVScreenInGameError_ShowContinueButton",
    ),
    (
        0x001f_b458,
        in_game_error_highlight_continue,
        "OVScreenInGameError_HighlightContinue",
    ),
    (
        0x001f_b588,
        in_game_error_highlight_cancel,
        "OVScreenInGameError_HighlightCancel",
    ),
    (
        0x001f_bcc8,
        ov_screen_race_pre_on_deactivate,
        "OVScreenRacePre_OnDeactivate",
    ),
    (
        0x001f_bce0,
        ov_screen_race_pre_update,
        "OVScreenRacePre_Update",
    ),
    (
        0x001f_d0d8,
        ov_screen_multi_freestyle_pre_on_activate,
        "OVScreenMultiFreestylePre_OnActivate",
    ),
    (
        0x001f_d118,
        ov_multi_freestyle_pre_on_deactivate,
        "OVScreenMultiFreestylePre_OnDeactivate",
    ),
    (
        0x001f_d150,
        ov_screen_multi_freestyle_pre_update,
        "OVScreenMultiFreestylePre_Update",
    ),
    (
        0x001f_d268,
        ov_screen_rival_pre_init,
        "OVScreenRivalPre_Init",
    ),
    (
        0x001f_db78,
        ov_screen_rival_pre_on_widget_notify,
        "OVScreenRivalPre_OnWidgetNotify",
    ),
    (
        0x001f_dbf0,
        ov_screen_save_replay_ctor,
        "OVScreenSaveReplay_ctor",
    ),
    (
        0x001f_df00,
        ov_screen_save_replay_v33,
        "OVScreenSaveReplay_v33",
    ),
    (
        0x001f_e5b0,
        ov_screen_save_replay_v38,
        "OVScreenSaveReplay_v38",
    ),
    (
        0x001f_e648,
        ov_screen_save_replay_v39,
        "OVScreenSaveReplay_v39",
    ),
    (
        0x001f_e8f0,
        ov_screen_save_replay_on_child_event,
        "OVScreenSaveReplay_OnChildEvent",
    ),
    (
        0x001f_e968,
        ov_screen_save_records_ctor,
        "OVScreenSaveRecords_ctor",
    ),
    (
        0x001f_ec60,
        ov_screen_save_records_v33,
        "OVScreenSaveRecords_v33",
    ),
    (
        0x001f_f278,
        ov_screen_save_records_v38,
        "OVScreenSaveRecords_v38",
    ),
    (
        0x001f_f3b0,
        ov_screen_save_records_on_widget_action,
        "OVScreenSaveRecords_OnWidgetAction",
    ),
    (
        0x001f_f6d0,
        ov_screen_reward_list_on_activate,
        "OVScreenRewardList_OnActivate",
    ),
    (
        0x001f_f700,
        ov_screen_reward_list_on_widget_action,
        "OVScreenRewardList_OnWidgetAction",
    ),
    (0x0023_d570, mc_manager_set_mode, "MCManager_SetMode"),
    (0x001f_b6b8, ov_screen_race_pre_init, "OVScreenRacePre_Init"),
    (
        0x001f_bbd8,
        ov_screen_race_pre_on_activate,
        "OVScreenRacePre_OnActivate",
    ),
    (0x0022_1df0, ov_screen_race_pre_dtor, "OVScreenRacePre_dtor"),
    (0x001f_d190, fe_get_event_name_text, "FE_GetEventNameText"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::be::beintmission::callee::AUDIO_START_PRERACE_MUSIC;

    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::BE_MESSAGE_FLAG_SET;

    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;

    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;

    pub(crate) use crate::game::fe::feasyncfile::callee::OV_STATE_OVERLAY_INIT;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;

    pub(crate) use crate::game::fe::fepopup::callee::C00227F80;

    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_OPPONENT_DISCONNECTED;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SERVER_TIMEOUT;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SHOW_CANCEL;

    pub(crate) use crate::game::main::gamemode::callee::STR_HASH as STR_HASH_2;

    pub(crate) use crate::game::main::ssxappdtor::callee::AUDIO_GET_INSTANCE;

    pub(crate) use crate::game::main::ssxappdtor::callee::REWARD_LIST_SHOW_PAGE;

    pub(crate) use crate::game::main::ssxappdtor::callee::UI_STATE_ON_ACTIVATE;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_ON_CHILD_EVENT;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_END_SAVE;

    pub const MPEG2_SHOW_BITS: Callee = Callee {
        addr: 0x003b_1258,
        f: ee::s13::MPEG2_Show_Bits,
    };

    pub const MPEG2_FLUSH_BUFFER: Callee = Callee {
        addr: 0x003b_1290,
        f: ee::s13::MPEG2_Flush_Buffer,
    };

    pub const AI_DESTROY_MISSION_RIDERS: Callee = Callee {
        addr: 0x0012_af68,
        f: ee::s00::AI_DestroyMissionRiders,
    };

    pub const AI_CREATE_COMPUTER_RIDERS: Callee = Callee {
        addr: 0x0012_9e20,
        f: ee::s00::AI_CreateComputerRiders,
    };

    pub const ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY: Callee = Callee {
        addr: 0x0031_1ae8,
        f: ee::s10::AnimController_GetLayerAnimEntry,
    };

    pub const BE_NEW_PLAYER_INT_GET_SLOT: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::BENewPlayerInt_GetSlot,
    };

    pub const BE_REWARD_INT_COUNT_SET_BITS: Callee = Callee {
        addr: 0x0015_7390,
        f: ee::s01::BERewardInt_CountSetBits,
    };

    pub const CHASE_CAMERA_ALGORITHM_CTOR: Callee = Callee {
        addr: 0x0016_2318,
        f: ee::s01::ChaseCameraAlgorithm_ctor,
    };

    pub const BUILTIN_DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::__builtin_delete,
    };

    pub const RIDER_METRIX_CLEAR_PEAK_A4: Callee = Callee {
        addr: 0x0011_76f8,
        f: ee::s00::RiderMetrix_ClearPeakA4,
    };

    pub const EA_MEM_SET: Callee = Callee {
        addr: 0x003e_6448,
        f: ee::s14::EA_MemSet,
    };

    pub const MC_MANAGER_GET: Callee = Callee {
        addr: 0x0022_7f80,
        f: ee::s05::MCManager_Get,
    };

    pub const MC_MANAGER_BEGIN_LOAD: Callee = Callee {
        addr: 0x0023_c8f0,
        f: ee::s06::MCManager_BeginLoad,
    };

    pub const UI_STATE_CTOR: Callee = Callee {
        addr: 0x0039_e2a0,
        f: ee::s13::UIState_ctor,
    };

    pub const FE_FORMAT_MONEY: Callee = Callee {
        addr: 0x0019_8af0,
        f: ee::s03::FE_FormatMoney,
    };

    pub const UI_TEXT_SET_STRING: Callee = Callee {
        addr: 0x003a_0c50,
        f: ee::s13::UIText_SetString,
    };

    pub const FE_MODULE_RIDER_PREVIEW_UNLOAD: Callee = Callee {
        addr: 0x0019_eba0,
        f: ee::s03::FEModule_RiderPreviewUnload,
    };

    pub const FE_MODULE_RIDER_PREVIEW_UPDATE: Callee = Callee {
        addr: 0x0019_f138,
        f: ee::s03::FEModule_RiderPreviewUpdate,
    };

    pub const CHECK_ACCOUNT_NAME: Callee = Callee {
        addr: 0x001a_c360,
        f: ee::s03::cFEUseExistingAccount_CheckAccountName,
    };

    pub const CHECK_PASSWORD: Callee = Callee {
        addr: 0x001a_c2b8,
        f: ee::s03::cFEUseExistingAccount_CheckPassword,
    };

    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    };

    pub const NET_FE_STATE_CTOR_2: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::NetFEState_ctor,
    };

    pub const KEYBOARD_POPUP_IS_KEY_ENABLED: Callee = Callee {
        addr: 0x001c_e560,
        f: ee::s04::cKeyboardPopup_IsKeyEnabled,
    };

    pub const KEYBOARD_POPUP_MOVE_DOWN: Callee = Callee {
        addr: 0x001c_b9b0,
        f: ee::s04::cKeyboardPopup_MoveDown,
    };

    pub const EVENT_SELECT_POST_INPUT: Callee = Callee {
        addr: 0x0020_2770,
        f: ee::s05::EventSelect_PostInput,
    };

    pub const UI_STATE_POST_INPUT: Callee = Callee {
        addr: 0x0039_e508,
        f: ee::s13::UIState_PostInput,
    };

    pub const STR_HASH: Callee = Callee {
        addr: 0x0031_7670,
        f: ee::s10::StrHash,
    };

    pub const UI_TEXTURE_SET_FIND: Callee = Callee {
        addr: 0x0039_8380,
        f: ee::s13::UITextureSet_Find,
    };

    pub const VID_ENGINE_PLAYER_RENDER: Callee = Callee {
        addr: 0x0025_3890,
        f: ee::s06::VidEnginePlayer_Render,
    };

    pub const UI_STATE_RENDER: Callee = Callee {
        addr: 0x0039_e6b8,
        f: ee::s13::UIState_Render,
    };

    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: Callee = Callee {
        addr: 0x0028_b180,
        f: ee::s07::SSXAudioSystem_GetInstance,
    };

    pub const MUSIC_PLAYER_RESUME: Callee = Callee {
        addr: 0x002b_3a98,
        f: ee::s08::MusicPlayer_Resume,
    };

    pub const MOUNTAIN_ROOM_POLL_IMAGE_LOAD: Callee = Callee {
        addr: 0x001d_31c0,
        f: ee::s04::cFEStateMountainRoom_PollImageLoad,
    };

    pub const UI_STATE_UPDATE_2: Callee = Callee {
        addr: 0x0039_e510,
        f: ee::s13::UIState_Update,
    };

    pub const C_FE_STATE_ON_ACTIVATE: Callee = Callee {
        addr: 0x0018_6518,
        f: ee::s02::cFEState_OnActivate,
    };

    pub const C_OV_STATE_OVERLAY_UPDATE: Callee = Callee {
        addr: 0x0020_e900,
        f: ee::s05::cOVStateOverlay_Update,
    };

    pub const UI_STATE_ON_ACTIVATE_2: Callee = Callee {
        addr: 0x0039_e4c0,
        f: ee::s13::UIState_OnActivate,
    };

    pub const PLAY_FE_SOUND: Callee = Callee {
        addr: 0x0029_4f78,
        f: ee::s08::SSXAudioSystem_PlayFESound,
    };

    pub const MEM_CARD_STATE_BASE_CTOR_2: Callee = Callee {
        addr: 0x001d_5330,
        f: ee::s04::MemCardStateBase_ctor_2,
    };

    pub const UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS: Callee = Callee {
        addr: 0x0039_f718,
        f: ee::s13::UIStateMgr_CloseTopStateAndPopups,
    };

    pub const UI_STATE_MGR_CLOSE_TOP_STATES: Callee = Callee {
        addr: 0x0039_f190,
        f: ee::s13::UIStateMgr_CloseTopStates,
    };

    pub const OV_SCREEN_SESSION_DESTROY_INDICATORS: Callee = Callee {
        addr: 0x0020_a088,
        f: ee::s05::OVScreenSession_DestroyIndicators,
    };

    pub const C_OV_STATE_OVERLAY_SHUTDOWN: Callee = Callee {
        addr: 0x0020_a430,
        f: ee::s05::cOVStateOverlay_Shutdown,
    };

    pub const FE_ASYNC_MANAGER_CANCEL: Callee = Callee {
        addr: 0x001a_38c0,
        f: ee::s03::FEAsyncManager_Cancel,
    };

    pub const SECTION_MAN_REQUEST_ACTIVATE: Callee = Callee {
        addr: 0x0022_d478,
        f: ee::s05::SectionMan_RequestActivate,
    };

    pub const STRCPY: Callee = Callee {
        addr: 0x0041_66f4,
        f: ee::s15::strcpy,
    };

    pub const STRLEN: Callee = Callee {
        addr: 0x0041_6810,
        f: ee::s15::strlen,
    };
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {

    pub(crate) use crate::game::fe::festaterewards::table_callee::GET_HASH_VALUE32;

    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_FIND_OBJECT;

    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {

    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_BEGIN_PLAYBACK;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_END_SAVE;
}

// ---------------------------------------------------------------------------
// Memory card manager.

/// `void MCManager_SetMode(MCManager *self, int mode)` (0x23d570): publish the
/// mode at +0xb8; when it is exactly 1, also store the mode at +0x108, 1.0f at
/// +0x110 and a global float (`gp-0x512c`) at +0x114.
pub fn mc_manager_set_mode(c: &mut Cpu) {
    c.cycles += 2;
    let this = Addr(abi::arg(c, 0));
    let mode = c.g[A1][0];
    this.off(0xb8).sw(c, mode);
    if mode != 0 {
        c.cycles += 3;
        c.g[V0][0] = 0x3c;
        if mode == 1 {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.f[0] = c.g[AT][0] as u32; // mtc1 $at, $f0 (1.0f)
            c.f[1] = Addr((c.g[GP][0] as u32).wrapping_sub(0x512c)).read_u32(c);
            this.off(0x108).sw(c, c.g[V0][0]);
            this.off(0x110).write_fpu(c, FpuF32(c.f[0]));
            this.off(0x114).write_fpu(c, FpuF32(c.f[1]));
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void OVScreenRewardList_OnActivate(self)` (0x1ff6d0): activate the base
/// screen state, then show the reward list's first page.
pub fn ov_screen_reward_list_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_ON_ACTIVATE, 0x001ff6e4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, callee::REWARD_LIST_SHOW_PAGE, 0x001ff6f0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenMultiFreestylePre_OnDeactivate(OVScreen* self)`
/// (0x1fd118): mark the freestyle screen inactive, then resume the
/// pre-race music.
pub fn ov_multi_freestyle_pre_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
        let none = c.g[V1][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            Addr((c.g[V1][0] as u32).wrapping_add(0x64)).sw(c, c.g[V0][0]);
        }
        c.cycles += 7;
        call_ptr(c, callee::AUDIO_GET_INSTANCE, 0x001f_d138)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::AUDIO_START_PRERACE_MUSIC, 0x001f_d140)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `lw` at `base + off` (sign-extended), the generators' most common load.
#[inline(always)]
pub(crate) fn lw(c: &mut Cpu, base: u32, off: u32) -> u64 {
    Addr(base.wrapping_add(off)).lw(c)
}

/// `a + off`.
#[inline(always)]
pub(crate) fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

// ---------------------------------------------------------------------------
// Overlay screens.

/// The shared tail of `OVScreenRacePre_Update` / `OVScreenMultiFreestylePre_Update`:
/// update the overlay base, then flag the loaded race "finished" once.
pub(crate) fn ov_screen_pre_update_common(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::C_OV_STATE_OVERLAY_UPDATE.call(c, ret)?;
        let g = crate::abi::r32(c, GP);
        let race = lw(c, g, 0xfffffdfc); // gp - 0x204
        c.g[A0][0] = race;
        let missing = race == 0;
        frame.restore_ra(c, 0);
        if !missing {
            c.cycles += 3;
            c.g[V0][0] = lw(c, race as u32, 0x64);
            let done = c.g[V0][0] != 0;
            c.g[V0][0] = u64::MAX;
            if !done {
                c.cycles += 4;
                c.g[V1][0] = 1;
                Addr(p(race as u32, 0x64)).sw(c, c.g[V0][0]);
                Addr(crate::abi::r32(c, GP).wrapping_sub(0x9f4)).sw(c, c.g[V1][0]);
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenRacePre_Update(Screen* s)` (0x1fbce0).
pub fn ov_screen_race_pre_update(c: &mut Cpu) {
    ov_screen_pre_update_common(c, 0x001f_bcf0)
}

/// `void OVScreenMultiFreestylePre_Update(Screen* s)` (0x1fd150).
pub fn ov_screen_multi_freestyle_pre_update(c: &mut Cpu) {
    ov_screen_pre_update_common(c, 0x001f_d160)
}

/// `void OVScreenMultiFreestylePre_OnActivate(Screen* s)` (0x1fd0d8):
/// activate the overlay, mark the loaded race with state 0x4b0, and play the
/// front-end "activate" sound (0xe).
pub fn ov_screen_multi_freestyle_pre_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::UI_STATE_ON_ACTIVATE_2.call(c, 0x001f_d0e8)?;
        let g = crate::abi::r32(c, GP);
        let race = lw(c, g, 0xfffffdfc); // gp - 0x204
        c.g[V1][0] = race;
        c.g[V0][0] = 0x4b0;
        if race != 0 {
            c.cycles += 1;
            Addr((race as u32).wrapping_add(0x64)).sw(c, c.g[V0][0]);
        }
        c.cycles += 8;
        callee::SSX_AUDIO_SYSTEM_GET_INSTANCE.call(c, 0x001f_d100)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xe;
        callee::PLAY_FE_SOUND.call(c, 0x001f_d10c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* OVScreenSaveReplay_ctor(void* self, int a1, int a2)` (0x1fdbf0):
/// memory-card base state (variant 1), class vtable, `+0x22c` cleared.
pub fn ov_screen_save_replay_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A2][0];
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 1;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MEM_CARD_STATE_BASE_CTOR_2.call(c, 0x001f_dc0c)?;
        let this = crate::abi::r32(c, S0);
        c.g[V1][0] = 0x47_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_add(0x2440));
        Addr(p(this, 0x22c)).sw(c, 0);
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* OVScreenSaveRecords_ctor(void* self, int a1, int a2)` (0x1fe968):
/// the same with variant 2 and the records vtable.
pub fn ov_screen_save_records_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A2][0];
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 2;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MEM_CARD_STATE_BASE_CTOR_2.call(c, 0x001f_e984)?;
        let this = crate::abi::r32(c, S0);
        c.g[V1][0] = 0x47_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_add(0x22b8));
        Addr(p(this, 0x22c)).sw(c, 0);
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The three "close the top state once" hooks: when `+0x1ac` is clear, close
/// the state at `*(self + 0x10) + 0x18`, then set `+0x1ac`.
pub(crate) fn close_top_state_once(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = lw(c, this, 0x1ac);
        let done = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !done {
            c.cycles += 4;
            c.g[A0][0] = lw(c, this, 0x10);
            c.g[A0][0] = sx(p(crate::abi::r32(c, A0), 0x18));
            callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS.call(c, ret)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        Addr(p(crate::abi::r32(c, S0), 0x1ac)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenSaveReplay_v33(Screen* s)` (0x1fdf00).
pub fn ov_screen_save_replay_v33(c: &mut Cpu) {
    close_top_state_once(c, 0x001f_df28)
}

/// `void OVScreenSaveRecords_v33(Screen* s)` (0x1fec60).
pub fn ov_screen_save_records_v33(c: &mut Cpu) {
    close_top_state_once(c, 0x001f_ec88)
}

/// `void OVScreenRewardList_OnWidgetAction(self, widget /*a1*/, action
/// /*a2*/)` (0x1ff700): when the "confirm" key is pressed, arm the reward
/// list and tell rider slot 0.
pub fn ov_screen_reward_list_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let no_widget = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        if !no_widget {
            c.cycles += 3;
            c.g[V0][0] = 5;
            let not_confirm = c.g[A2][0] != c.g[V0][0];
            c.g[RA][0] = c.ld(p_2(sp(c), 0));
            if not_confirm {
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 8;
            c.g[V0][0] = 1;
            sw(c, gp(c, -0x9f4), 0, c.g[V0][0]);
            call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x001f_f724)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0xd;
            call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x001f_f730)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_MESSAGE_FLAG_SET, 0x001f_f738)?;
        }
        // Both the no-widget path and the confirm path join here.
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `int OVScreenBEOptions_v22(void*, int, int id)` (0x1faaf0): id 4 (the
/// "back" widget of the back-end options overlay) is handled (0x100).
pub fn ov_screen_be_options_v22(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] ^= 0x4;
    c.g[V1][0] = 0x100;
    c.g[V0][0] = 0x101;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}

/// `int OVScreenBEOptions_v21(void*, int, int id)` (0x1fab08): 0x100 for the
/// "no id" case, as the other option screens.
pub fn ov_screen_be_options_v21(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x100;
    c.g[V0][0] = 0x101;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}

/// `void OVScreenRacePre_OnDeactivate(void)` (0x1fbcc8): reset the pending
/// count of the overlay (`gp-0x204 -> +0x64`) to -1, when it exists.
pub fn ov_screen_race_pre_on_deactivate(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr(gpa(c, -0x204)).lw(c);
    let absent = c.g[V1][0] == 0;
    c.g[V0][0] = u64::MAX;
    if absent {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[V0][0]);
    c.cycles += 2;
    abi::ret(c);
}

/// `int OVScreenRivalPre_OnWidgetNotify(void*, int, int id)` (0x1fdb78):
/// id 6 handled (0x101), everything else 0.
pub fn ov_screen_rival_pre_on_widget_notify(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] ^= 0x6;
    c.g[V0][0] = 0x101;
    if c.g[A2][0] != 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `OVScreenSaveReplay_OnChildEvent` (0x1FE8F0): on the "save done" child event
/// of the replay save screen, stop the replay save and clear the pending flag;
/// everything else goes to the memory card state's child-event handler.
pub fn ov_screen_save_replay_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x16;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        let other_key = c.g[S1][0] != c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        let mut tail = true;
        if !other_key {
            c.cycles += 4;
            c.g[V1][0] = lw_at(c, S0, 0x19c);
            c.g[V0][0] = 5;
            let not_five = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            if not_five {
                tail = false;
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
                c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lw(c);
                callee::REPLAY_END_SAVE.call(c, 0x001f_e938)?;
                sw_at(c, S0, 0x1c8, 0);
            }
        }
        if tail {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 9;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        callee::MEM_CARD_ON_CHILD_EVENT.call(c, 0x001f_e94c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int OVScreenBEOptions_OnWidgetNotify(self, menu, key, index)` (0x1faa78):
/// the pause-menu widget notification filter — 0x101 for the keys it does not
/// handle, 0x100 for "start"/"options" and for the pause title.
pub fn ov_screen_be_options_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x20));
        c.g[A0][0] = c.g[A1][0];
        c.sq(16, sp_2(c).wrapping_add(0x10));
        c.g[A1][0] = c.g[A3][0];
        c.g[V0][0] = 0x6;
        let title = c.g[A2][0] == c.g[V0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        if title {
            c.cycles += 8;
            c.g[A0][0] = sx(low(c, A0).wrapping_add(0x74));
            call_ptr(c, 0x0039_7870, 0x001f_aac4)?;
            c.g[A0][0] = 0x4a_0000;
            c.g[S0][0] = lw_r(c, V0, 0x38);
            c.g[A0][0] = sx(low(c, A0).wrapping_add(0x24e8));
            call_ptr(c, callee::STR_HASH_2, 0x001f_aad4)?;
            if c.g[S0][0] != c.g[V0][0] {
                c.g[V0][0] = 0x100;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0x101;
            }
        } else {
            c.cycles += 3;
            if c.g[A2][0] < 0x6 {
                c.g[V0][0] = 0x101;
            } else {
                c.cycles += 3;
                if !(c.g[A2][0] < 0xa) {
                    c.cycles += 1;
                    c.g[V0][0] = 0x101;
                } else if c.g[A2][0] < 0x8 {
                    c.cycles += 2;
                    c.g[V0][0] = 0x101;
                } else {
                    c.cycles += 2;
                    c.cycles += 2;
                    c.g[V0][0] = 0x100;
                }
            }
        }
        c.cycles += 4;
        c.lq(16, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `void OVScreenSaveReplay_v38(self, on)` (0x1fe5b0).
pub fn ov_screen_save_replay_v38(c: &mut Cpu) {
    guest(c, |c| ov_screen_set_two_visible(c, 0x001f_e5b0, 0x188))
}

/// `void OVScreenSaveRecords_v38(self, on)` (0x1ff278).
pub fn ov_screen_save_records_v38(c: &mut Cpu) {
    guest(c, |c| ov_screen_set_two_visible(c, 0x001f_f278, 0x188))
}

/// `void OVScreenInGameError_UpdateServerTimeoutText(self)` (0x1fb180): update
/// the timeout text and its object's countdown value.
pub fn ov_screen_in_game_error_update_server_timeout_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_2(c, S1, 0x40);
        if c.g[V0][0] != 0 {
            // The text object (S0) is looked up by hash; the hash of the
            // unloaded string is 0 on the test machine.
            c.cycles += 9;
            c.g[A0][0] = 0x470000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e8));
            call_ptr(c, table_callee::GET_HASH_VALUE32, 0x001f_b1ac)?;
            c.g[A0][0] = lw_2(c, S1, 0x40);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, table_callee::UI_SCREEN_FIND_OBJECT, 0x001f_b1b8)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0 {
                c.g[V0][0] = lw_2(c, S0, 0x8);
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw_2(c, S1, 0xa0);
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = lw_2(c, S0, 0x8);
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4a0));
                    call_ptr(c, table_callee::GET_HASH_VALUE32, 0x001f_b1dc)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(
                        c,
                        table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID,
                        0x001f_b1e8,
                    )?;
                    c.g[V0][0] = lw_2(c, S0, 0x8);
                }
            }
            c.cycles += 6;
            c.g[A1][0] = lw_2(c, S1, 0xa0);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
            c.g[V1][0] = lw_2(c, V0, 0x4c);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001f_b200)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenBEOptions_Update` (0x1fa9e0): update the BE options overlay and,
/// when the pause overlay is active, toggle the option thread's value.
pub fn ov_screen_be_options_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001f_a9f4);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::OVERLAY_UPDATE, 0x001f_a9f4)?;
        c.g[V1][0] = Addr(gp_2(c, -0x204)).lw(c);
        let no_overlay = c.g[V1][0] == 0;
        if !no_overlay {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r(c, V1), 0x68)).lw(c);
            let zero68 = c.g[V0][0] == 0;
            c.g[A0][0] = 0;
            let active;
            if zero68 {
                c.cycles += 1;
                c.g[A0][0] = 1;
                active = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r(c, V1), 0x64)).lw(c);
                active = c.g[V0][0] != 0;
                if !active {
                    c.cycles += 1;
                    c.g[A0][0] = 1;
                }
            }
            let _ = active;
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                c.cycles += 10;
                c.g[A0][0] = 0x0047_0000;
                c.g[RA][0] = sx(0x001f_aa30);
                c.g[A0][0] = sx(0x0047_0000u32.wrapping_sub(0x1fb0));
                call_ptr(c, callee::FE_STRING_HASH, 0x001f_aa30)?;
                c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                c.g[RA][0] = sx(0x001f_aa3c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, callee::UI_SCREEN_LOOKUP_VALUE, 0x001f_aa3c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                let is_ffff = c.g[A1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                if !is_ffff {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                    c.g[RA][0] = sx(0x001f_aa58);
                    c.g[A2][0] = 1;
                    call_ptr(c, callee::UI_SCREEN_LAUNCH_THREAD, 0x001f_aa58)?;
                }
                c.cycles += 5;
                c.g[A0][0] = Addr(p(r(c, S0), 0x10)).lw(c);
                c.g[A1][0] = 1;
                c.g[RA][0] = sx(0x001f_aa68);
                c.g[A0][0] = sx(r(c, A0).wrapping_add(0x18));
                call_ptr(c, callee::UI_SCREEN_SET_OBJECT_VALUE, 0x001f_aa68)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenInGameError_UpdateOpponentDisconnectedText(Screen* self)`
/// (0x1fb218): find the disconnected-text object on the screen, set its text
/// id to the opponent-disconnected string and show the continue button.
pub fn ov_screen_in_game_error_update_opponent_disconnected_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_3(c, p(c.g[S1][0] as u32, 0x40));
        let no_screen = c.g[V0][0] == 0;
        c.g[A0][0] = 0x0047_0000;
        if !no_screen {
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e8));
            call_ptr(c, 0x0031_7670, 0x001f_b240)?; // StrHash
            c.g[A0][0] = lw_3(c, p(c.g[S1][0] as u32, 0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_b24c)?; // UIScreen_FindObject
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0 {
                c.g[V0][0] = lw_3(c, 8);
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw_3(c, p(c.g[S1][0] as u32, 0xa0));
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = lw_3(c, p(c.g[S0][0] as u32, 8));
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = 0x0047_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x478));
                    call_ptr(c, 0x0031_7670, 0x001f_b270)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a_1000, 0x001f_b27c)?; // UIText_SetTextId
                    c.g[V0][0] = lw_3(c, p(c.g[S0][0] as u32, 8));
                }
            }
            c.cycles += 8;
            c.g[A1][0] = lw_3(c, p(c.g[S1][0] as u32, 0xa0));
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x48)).lh(c);
            c.g[V1][0] = lw_3(c, p(c.g[V0][0] as u32, 0x4c));
            c.g[V1][0] = sx(c.g[V1][0] as u32);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001f_b294)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x001f_b378, 0x001f_b2a0)?; // ShowContinueButton
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void OVScreenSaveReplay_Shutdown(self)` (0x1fde60): end the replay save if
/// active, pop the screen from the stack and begin playback.
pub fn ov_screen_save_replay_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        c.g[V0][0] = lw_4(c, S0, 0x1c8);
        let not_saving = c.g[V0][0] != c.g[V1][0];
        c.g[A1][0] = Addr(gp_3(c, -0x69c)).lw(c);
        if !not_saving {
            c.cycles += 6;
            c.g[V0][0] = Addr(gp_3(c, -0x848)).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
            c.g[A0][0] = lw_4(c, V1, 0x28);
            call_ptr(c, table::REPLAY_END_SAVE, 0x001f_de90)?;
            sw_2(c, S0, 0x1c8, 0);
            c.g[A1][0] = Addr(gp_3(c, -0x69c)).lw(c);
        }
        c.cycles += 26;
        c.g[V0][0] = 0x0053_66e8;
        c.g[A2][0] = 0x0044_28f0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(gp_3(c, -0x69c)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        Addr(gp_3(c, -0x6a0)).sw(c, c.g[V1][0]);
        call_ptr(c, 0x001d_58b8, 0x001f_ded4)?;
        c.g[V1][0] = Addr(gp_3(c, -0x848)).lw(c);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x84).lw(c);
        c.g[A0][0] = lw_4(c, V0, 0x28);
        call_ptr(c, table::REPLAY_BEGIN_PLAYBACK, 0x001f_def0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub fn ov_screen_save_replay_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        ov_screen_set_widget_visibility(c, c.g[A2][0])
    })
}

// ---------------------------------------------------------------------------
// FE overlay: rival pre-screen.

/// `void OVScreenRivalPre_Init(OVScreenRivalPre* s)` (0x1fd268): create the
/// overlay's two screens, play them, initialise the overlay base and start the
/// rival-entry speech.
pub fn ov_screen_rival_pre_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x0046_ff58);
        c.g[RA][0] = sx(0x001f_d28c);
        c.g[S0][0] = Addr(p(this, 0x10)).lw(c);
        call_ptr(c, callee::FE_STRING_HASH, 0x001f_d28c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001f_d2a0);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_d2a0)?;
        let none = c.g[V0][0] == 0;
        Addr(p(r(c, S1), 0x40)).sw(c, c.g[V0][0]);
        if !none {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001f_d2b8);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_d2b8)?;
        }
        'second: {
            c.cycles += 11;
            c.g[A0][0] = sx(0x0047_0000);
            c.g[S0][0] = Addr(p(r(c, S1), 0x10)).lw(c);
            c.g[RA][0] = sx(0x001f_d2c8);
            c.g[A0][0] = sx(0x0046_e378);
            call_ptr(c, callee::FE_STRING_HASH, 0x001f_d2c8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001f_d2dc);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0039_7cc0, 0x001f_d2dc)?;
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if none {
                break 'second;
            }
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001f_d2f0);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_d2f0)?;
        }
        c.cycles += 12;
        c.g[RA][0] = sx(0x001f_d2f8);
        c.g[A0][0] = c.g[S1][0];
        callee::OV_STATE_OVERLAY_INIT.call(c, 0x001f_d2f8)?;
        c.g[RA][0] = sx(0x001f_d300);
        call_ptr(c, 0x0028_b180, 0x001f_d300)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001f_d30c);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x0029_4f78, 0x001f_d30c)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `OVScreenInGameError_OnWidgetAction` (0x001fb0c0).
//
// `int OVScreenInGameError::OnWidgetAction(self, UIWidget* w, int action)`:
// map the dialog buttons onto the highlight/select handlers.

pub fn ov_screen_in_game_error_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fb16c: {
            'b_1fb168: {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[A0][0];
                let t = c.g[S1][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_1fb168;
                } else {
                    'b_1fb104: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[A2][0] == c.g[V0][0];
                        c.g[V0][0] = 0x5;
                        if t {
                            break 'b_1fb104;
                        } else {
                            c.cycles += 2;
                            let t = c.g[A2][0] != c.g[V0][0];
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1fb16c;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001fb0fc);
                                c.g[A2][0] = 0x5;
                                call_ptr(c, 0x001fb2b8, 0x001fb0fc)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_1fb16c;
                                } else {
                                    break 'b_1fb104;
                                }
                            }
                        }
                    }
                    'b_1fb13c: {
                        c.cycles += 6;
                        c.g[A0][0] = 0x470000;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[RA][0] = sx(0x001fb114);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x570));
                        call_ptr(c, 0x00317670, 0x001fb114)?;
                        let t = c.g[S0][0] != c.g[V0][0];
                        c.g[A0][0] = 0x470000;
                        if t {
                            break 'b_1fb13c;
                        } else {
                            c.cycles += 8;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001fb128);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, 0x001fb588, 0x001fb128)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001fb134);
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x001fb458, 0x001fb134)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1fb16c;
                            } else {
                                break 'b_1fb13c;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001fb148);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4b8));
                    call_ptr(c, 0x00317670, 0x001fb148)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_1fb16c;
                    } else {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x001fb15c);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x001fb458, 0x001fb15c)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x001fb168);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x001fb588, 0x001fb168)?;
                        break 'b_1fb168;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1fb16c;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `OVScreenInGameError_ShowCancelGame(this, on)` (0x1faff8): show or hide
/// the "cancel game" prompt (and its second, menu-driven variant).
pub fn ov_screen_in_game_error_show_cancel_game(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let this = abi::arg(c, 0);
        let _on = abi::arg(c, 1);
        let frame = Frame::open(c, 0x40);
        frame.save(c, 18, 0x10);
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save(c, 17, 0x20);
        c.g[A0][0] = 0x0047_0000;
        c.g[S1][0] = abi::arg64(c, 1);
        frame.save(c, 16, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x0047_0000u32.wrapping_sub(0x570));
        callee::STR_HASH.call(c, 0x001f_b020)?;
        c.g[A0][0] = Addr(this).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        callee::UI_SCREEN_FIND_OBJECT.call(c, 0x001f_b02c)?;
        let obj = abi::r32(c, V0);
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = Addr(obj).off(8).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001f_b048)?;

        c.g[V0][0] = Addr(obj).off(8).lw(c);
        c.g[A1][0] = c.g[S1][0] ^ 1;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x40).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x44).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001f_b060)?;

        let off = c.g[S1][0] == 0;
        c.g[A0][0] = 0x004a_0000;
        if !off {
            c.cycles += 15;
            c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x2470));
            callee::STR_HASH.call(c, 0x001f_b070)?;
            c.g[A0][0] = Addr(this).off(0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            callee::UI_SCREEN_FIND_OBJECT.call(c, 0x001f_b07c)?;
            let menu = abi::r32(c, V0);
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            c.g[V0][0] = Addr(menu).off(8).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x38).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x3c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx(menu.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001f_b098)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            callee::UI_MENU_SET_SELECTION_2.call(c, 0x001f_b0a4)?;
        }

        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenInGameError_ShowContinueButton(self, int visible)`
/// (0x1fb378): find the error screen's buttons and show/hide the continue
/// button.
pub fn ovscreeningameerror_show_continue_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[A0][0] = 0x470000;
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x4b8));
        call_ptr(c, 0x0031_7670, 0x001f_b3a0)?;
        c.g[A0][0] = Addr(p(r32(c, S2), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_b3ac)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 21;
            c.g[A0][0] = 0x470000;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x450));
            call_ptr(c, 0x0031_7670, 0x001f_b3c4)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003a_1000, 0x001f_b3d0)?;
            c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.g[A1][0] = c.g[S1][0] ^ 1;
            c.g[A0][0] = Addr(p(r32(c, V0), 0x40)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x44)).lw(c);
            let f = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, f, 0x001f_b3e8)?;
            c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
            let f = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, f, 0x001f_b400)?;
            c.g[V0][0] = 4;
            Addr(p(r32(c, S0), 0x18)).sw(c, c.g[V0][0]);
        }
        c.cycles += 8;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x2470));
        call_ptr(c, 0x0031_7670, 0x001f_b414)?;
        c.g[A0][0] = Addr(p(r32(c, S2), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_d860, 0x001f_b420)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.lq(S0, frame.at(c, 0x30));
        } else {
            c.cycles += 2;
            let hide = c.g[S1][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !hide {
                c.cycles += 2;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0039_b960, 0x001f_b43c)?;
            }
            c.cycles += 1;
            c.lq(S0, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenInGameError_Update(OVScreenInGameError* self)` (0x1faf10):
/// run the overlay's common update, then, while the online error is armed,
/// check the network timeout / disconnect / ping state and either show the
/// timeout or disconnect message, or hide the "ping" object; finally refresh
/// the cancel-game button and clear the armed flag.
pub fn ovscreen_in_game_error_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0020_e900, 0x001f_af24)?;

        c.g[V1][0] = Addr(gp_4(c, -0x204)).lw(c);
        if c.g[V1][0] == 0 {
            frame.restore(c, 16, 0x10);
            return game_error_tail(c, frame);
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r(c, S0)).off(0x9c).lw(c);
        if c.g[V0][0] == 0 {
            frame.restore(c, 16, 0x10);
            return game_error_tail(c, frame);
        }

        // a0 = (p->+0x138 <= 0 && p->+0x13c == 0): the ping timed out.
        c.cycles += 3;
        c.g[V0][0] = Addr(r(c, V1)).off(0x138).lw(c);
        let positive = (c.g[V0][0] as i64) > 0;
        c.g[A0][0] = 0;
        if !positive {
            c.cycles += 2;
            c.g[V0][0] = Addr(r(c, V1)).off(0x13c).lw(c);
            c.g[A0][0] = (c.g[V0][0] < 1) as u64;
        }

        'outer: {
            'reply: {
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.g[V0][0] = Addr(r(c, V1)).off(0x13c).lw(c);
                    break 'reply;
                }
                c.cycles += 5;
                c.sw(Addr(r(c, V1)).off(0x9c).0, 0);
                c.g[A0][0] = c.g[S0][0];
                callee::GAME_ERROR_SERVER_TIMEOUT.call(c, 0x001f_af64)?;
                c.g[A0][0] = c.g[S0][0];
                break 'outer;
            }

            c.cycles += 2;
            if c.g[V0][0] == 0 {
                frame.restore(c, 16, 0x10);
                return game_error_tail(c, frame);
            }

            // a0 = (p->+0x80 != 0 && p->+0xa0 == 0 && p->+0x24 == 0).
            'received: {
                c.cycles += 3;
                c.g[V0][0] = Addr(r(c, V1)).off(0x80).lw(c);
                let none = c.g[V0][0] == 0;
                c.g[A0][0] = 0;
                if none {
                    break 'received;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(r(c, V1)).off(0xa0).lw(c);
                let busy = c.g[V0][0] != 0;
                if busy {
                    break 'received;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(r(c, V1)).off(0x24).lw(c);
                c.g[A0][0] = (c.g[V0][0] < 1) as u64;
            }

            'message: {
                c.cycles += 2;
                let not_received = c.g[A0][0] == 0;
                c.g[A0][0] = 0x470000;
                if not_received {
                    break 'message;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                callee::GAME_ERROR_OPPONENT_DISCONNECTED.call(c, 0x001f_afa4)?;
                c.g[A0][0] = c.g[S0][0];
                break 'outer;
            }

            c.cycles += 12;
            c.g[A0][0] = sx(r(c, A0).wrapping_sub(0x4e8));
            call_ptr(c, 0x0031_7670, 0x001f_afb4)?;
            c.g[A0][0] = Addr(r(c, S0)).off(0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_afc0)?;
            c.g[V1][0] = Addr(r(c, V0)).off(0x8).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(r(c, V1)).off(0x48).lh(c);
            c.g[A2][0] = Addr(r(c, V1)).off(0x4c).lw(c);
            let target = r(c, A2);
            c.g[A0][0] = sx(p(r(c, V0), r(c, A0)));
            call_ptr(c, target, 0x001f_afd8)?;
            c.g[A0][0] = c.g[S0][0];
        }

        c.cycles += 4;
        c.g[A1][0] = 1;
        callee::GAME_ERROR_SHOW_CANCEL.call(c, 0x001f_afe4)?;
        c.sw(Addr(r(c, S0)).off(0x9c).0, 0);
        frame.restore(c, 16, 0x10);
        game_error_tail(c, frame)
    })
}

/// The `ld ra; addiu sp; jr ra` tail.
pub(crate) fn game_error_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// The two in-game error highlight handlers (0x1fb458 / 0x1fb588): look up
/// the two popup buttons and the message node; with `on` set the highlighted
/// button gets four 1.0 floats, otherwise both buttons are cleared and the
/// node's setter is called.
pub(crate) fn in_game_error_highlight(c: &mut Cpu, names: [u32; 3], rets: [u32; 10]) {
    guest(c, |c| {
        c.cycles += 30;
        let f = Frame::open(c, 0x60);
        f.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        f.save(c, S2, 0x30);
        c.g[A0][0] = 0x0047_0000;
        f.save(c, S1, 0x40);
        f.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        f.save_ra(c, 0x10);
        // Three name -> widget lookups on the screen.
        let hash_ret = [rets[0], rets[2], rets[4]];
        let find_ret = [rets[1], rets[3], rets[5]];
        for (i, name) in names.iter().enumerate() {
            c.g[RA][0] = sx(hash_ret[i]);
            c.g[A0][0] = sx(*name);
            call_ptr(c, 0x0031_7670, hash_ret[i])?;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
            c.g[RA][0] = sx(find_ret[i]);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, find_ret[i])?;
            match i {
                0 => c.g[S1][0] = c.g[V0][0],
                1 => c.g[S3][0] = c.g[V0][0],
                _ => c.g[S0][0] = c.g[V0][0],
            }
        }
        // The button: all four floats 1.0 when highlighted, only the first
        // (alpha) otherwise. The four words go through the stack.
        let on = c.g[S2][0] != 0;
        c.cycles += if on { 14 } else { 13 };
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(8)).lw(c);
        c.g[A1][0] = sx(f.at(c, 0));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
        c.swc1(0, f.at(c, if on { 0xc } else { 0 }));
        if on {
            c.swc1(0, f.at(c, 0));
            c.swc1(0, f.at(c, 4));
            c.swc1(0, f.at(c, 8));
        } else {
            c.sw(f.at(c, 4), 0);
            c.sw(f.at(c, 8), 0);
            c.sw(f.at(c, 0xc), 0);
        }
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
        let func = c.g[V0][0] as u32;
        let ret = if on { rets[6] } else { rets[7] };
        c.g[RA][0] = sx(ret);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, ret)?;
        c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(8)).lw(c);
        // Then the node's setter, and the node widget's own setter.
        c.cycles += 18;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
        let func = c.g[V1][0] as u32;
        c.g[RA][0] = sx(rets[8]);
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, rets[8])?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
        let func = c.g[V1][0] as u32;
        c.g[RA][0] = sx(rets[9]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, rets[9])?;
        f.restore(c, S0, 0x50);
        f.restore(c, S1, 0x40);
        f.restore(c, S2, 0x30);
        f.restore(c, S3, 0x20);
        f.restore_ra(c, 0x10);
        f.close(c);
        Ok(())
    })
}

/// `void OVScreenInGameError_HighlightContinue(OVScreen*, int on)` (0x1fb458).
pub fn in_game_error_highlight_continue(c: &mut Cpu) {
    in_game_error_highlight(
        c,
        [0x0046_fb48, 0x0046_fad8, 0x0046_fb00],
        [
            0x001f_b484,
            0x001f_b490,
            0x001f_b4a0,
            0x001f_b4ac,
            0x001f_b4bc,
            0x001f_b4c8,
            0x001f_b500,
            0x001f_b538,
            0x001f_b550,
            0x001f_b568,
        ],
    );
}

/// `void OVScreenInGameError_HighlightCancel(OVScreen*, int on)` (0x1fb588).
pub fn in_game_error_highlight_cancel(c: &mut Cpu) {
    in_game_error_highlight(
        c,
        [0x0046_fa90, 0x0046_fac8, 0x0046_fae8],
        [
            0x001f_b5b4,
            0x001f_b5c0,
            0x001f_b5d0,
            0x001f_b5dc,
            0x001f_b5ec,
            0x001f_b5f8,
            0x001f_b630,
            0x001f_b668,
            0x001f_b680,
            0x001f_b698,
        ],
    );
}

/// `void func_001FF3B0(void* self, void* msg, int type)` (0x1ff3b0): the save
/// records screen's widget action: type 5 stores the selected slot from the
/// message and starts the "save" movie sequence, type 6 cancels the movie
/// when the screen is in the right state, and type 7 does the same as 5 once
/// the memory card is ready, then starts the sequence.
pub fn ov_screen_save_records_on_widget_action(c: &mut Cpu) {
    let frame = Frame::open(c, 0x50);
    'b_1ff578: {
        'b_1ff574: {
            c.cycles += 9;
            frame.save(c, S0, 0x40);
            frame.save(c, S2, 0x20);
            c.g[S0][0] = c.g[A0][0];
            frame.save(c, S1, 0x30);
            c.g[S2][0] = c.g[A1][0];
            frame.save(c, S3, 0x10);
            let t = c.g[S2][0] == 0u64;
            sd(c, r32(c, SP), c.g[RA][0]);
            if t {
                break 'b_1ff574;
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw_6(c, at_3(c, S0, 0x1a8));
                if c.g[V0][0] != 0u64 {
                    frame.restore(c, S0, 0x40);
                    break 'b_1ff578;
                } else {
                    'b_1ff4d4: {
                        'b_1ff46c: {
                            c.cycles += 3;
                            c.g[S1][0] = 0x6;
                            let t = c.g[A2][0] == c.g[S1][0];
                            c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                            if t {
                                break 'b_1ff46c;
                            } else {
                                'b_1ff418: {
                                    'b_1ff404: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x5;
                                        if t {
                                            break 'b_1ff404;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[A2][0] == c.g[V0][0] {
                                                c.g[V0][0] = lw_6(c, at_3(c, S0, 0x1c0));
                                                break 'b_1ff418;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                frame.restore(c, S0, 0x40);
                                                if t {
                                                    break 'b_1ff578;
                                                } else {
                                                    break 'b_1ff404;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x7;
                                    if c.g[A2][0] == c.g[V0][0] {
                                        c.g[V0][0] = lw_6(c, at_3(c, S0, 0x1c0));
                                        break 'b_1ff4d4;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        frame.restore(c, S0, 0x40);
                                        if t {
                                            break 'b_1ff578;
                                        } else {
                                            break 'b_1ff418;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[V0][0] == c.g[S1][0];
                                c.g[S3][0] = 0x1;
                                if t {
                                    break 'b_1ff574;
                                } else {
                                    c.cycles += 7;
                                    c.g[RA][0] = sx(0x001ff428);
                                    c.g[A0][0] = lw_6(c, r32(c, GP).wrapping_sub(0x848));
                                    if callee::C00227F80.call(c, 0x001ff428).is_err() {
                                        return;
                                    }
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[S1][0] = lw_6(c, at_3(c, S0, 0x214));
                                    c.g[V0][0] = 0x3;
                                    if c.g[S1][0] != c.g[V0][0] {
                                        frame.restore(c, S0, 0x40);
                                        break 'b_1ff578;
                                    } else {
                                        c.cycles += 12;
                                        c.g[V0][0] = lw_6(c, at_3(c, S2, 0x18));
                                        c.g[A0][0] = c.g[V1][0];
                                        c.g[A1][0] = 0x2;
                                        c.g[A2][0] = 0xffffffffffffffff;
                                        sw_3(c, at_3(c, S0, 0x1c4), c.g[V0][0]);
                                        sw_3(c, at_3(c, V1, 0xf8), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x001ff45c);
                                        sw_3(c, at_3(c, S0, 0x1a8), c.g[S3][0]);
                                        if call_ptr(c, 0x0023cae8, 0x001ff45c).is_err() {
                                            return;
                                        }
                                        sw_3(c, at_3(c, S0, 0x1c0), c.g[S1][0]);
                                        sw_3(c, at_3(c, S0, 0x1a8), c.g[S3][0]);
                                        let t = true;
                                        sw_3(c, at_3(c, S0, 0x1dc), c.g[S3][0]);
                                        if t {
                                            break 'b_1ff574;
                                        } else {
                                            break 'b_1ff46c;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x001ff474);
                        c.g[A0][0] = lw_6(c, r32(c, GP).wrapping_sub(0x848));
                        if callee::C00227F80.call(c, 0x001ff474).is_err() {
                            return;
                        }
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = 0x1;
                        c.g[V0][0] = lw_6(c, at_3(c, S0, 0x1a8));
                        if c.g[V0][0] == c.g[A0][0] {
                            frame.restore(c, S0, 0x40);
                            break 'b_1ff578;
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = lw_6(c, at_3(c, S0, 0x1c0));
                            if c.g[V1][0] == c.g[S1][0] {
                                frame.restore(c, S0, 0x40);
                                break 'b_1ff578;
                            } else {
                                'b_1ff4cc: {
                                    'b_1ff4b4: {
                                        'b_1ff4b0: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == 0u64;
                                            sw_3(c, at_3(c, S0, 0x1e4), c.g[A0][0]);
                                            if t {
                                                break 'b_1ff4b0;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x2;
                                                if c.g[V1][0] == c.g[V0][0] {
                                                    c.g[V0][0] = lw_6(c, at_3(c, A2, 0x748));
                                                    break 'b_1ff4b4;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] != c.g[A0][0];
                                                    c.g[V0][0] = 0x6;
                                                    if t {
                                                        break 'b_1ff4cc;
                                                    } else {
                                                        break 'b_1ff4b0;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = lw_6(c, at_3(c, A2, 0x748));
                                        break 'b_1ff4b4;
                                    }
                                    c.cycles += 6;
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = lh(c, at_3(c, V0, 0x8));
                                    c.g[V1][0] = lw_6(c, at_3(c, V0, 0xc));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001ff4c8);
                                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x001ff4c8).is_err() {
                                        return;
                                    }
                                    c.g[V0][0] = 0x6;
                                    break 'b_1ff4cc;
                                }
                                c.cycles += 2;
                                let t = true;
                                sw_3(c, at_3(c, S0, 0x1c0), c.g[V0][0]);
                                if t {
                                    break 'b_1ff574;
                                } else {
                                    break 'b_1ff4d4;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[V0][0] == c.g[S1][0];
                    c.g[S3][0] = 0x1;
                    if t {
                        break 'b_1ff574;
                    } else {
                        c.cycles += 12;
                        c.g[RA][0] = sx(0x001ff4e4);
                        c.g[A0][0] = lw_6(c, r32(c, GP).wrapping_sub(0x848));
                        if callee::C00227F80.call(c, 0x001ff4e4).is_err() {
                            return;
                        }
                        c.g[S1][0] = c.g[V0][0];
                        c.g[A2][0] = lw_6(c, at_3(c, S1, 0x434));
                        c.g[A1][0] = lw_6(c, at_3(c, S1, 0x428));
                        c.g[V1][0] = lw_6(c, r32(c, A2));
                        c.g[A0][0] = lh(c, at_3(c, V1, 0x1b0));
                        c.g[V0][0] = lw_6(c, at_3(c, V1, 0x1b4));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001ff504);
                        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x001ff504).is_err() {
                            return;
                        }
                        if c.g[V0][0] == 0u64 {
                            frame.restore(c, S0, 0x40);
                            break 'b_1ff578;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = lw_6(c, at_3(c, S2, 0x18));
                            sw_3(c, at_3(c, S0, 0x1c4), c.g[V0][0]);
                            sw_3(c, at_3(c, S1, 0xf8), c.g[V0][0]);
                            c.g[S2][0] = lw_6(c, at_3(c, S0, 0x1c0));
                            if c.g[S2][0] != c.g[S3][0] {
                                frame.restore(c, S0, 0x40);
                                break 'b_1ff578;
                            } else {
                                c.cycles += 9;
                                c.g[A2][0] = lw_6(c, at_3(c, S1, 0x434));
                                c.g[A1][0] = lw_6(c, at_3(c, S0, 0x1c4));
                                c.g[V1][0] = lw_6(c, r32(c, A2));
                                c.g[A0][0] = lh(c, at_3(c, V1, 0x218));
                                c.g[V0][0] = lw_6(c, at_3(c, V1, 0x21c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001ff540);
                                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                if call_ptr(c, t, 0x001ff540).is_err() {
                                    return;
                                }
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x4;
                                if t {
                                    break 'b_1ff574;
                                } else {
                                    c.cycles += 11;
                                    sw_3(c, at_3(c, S0, 0x1a8), c.g[S2][0]);
                                    c.g[A0][0] = c.g[S1][0];
                                    sw_3(c, at_3(c, S0, 0x1c0), c.g[V0][0]);
                                    c.g[RA][0] = sx(0x001ff55c);
                                    c.g[A1][0] = 0u64;
                                    if call_ptr(c, 0x0023d570, 0x001ff55c).is_err() {
                                        return;
                                    }
                                    c.g[V0][0] = lw_6(c, at_3(c, S1, 0x748));
                                    c.g[A1][0] = 0x18;
                                    c.g[A0][0] = lh(c, at_3(c, V0, 0x8));
                                    c.g[V1][0] = lw_6(c, at_3(c, V0, 0xc));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001ff574);
                                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x001ff574).is_err() {
                                        return;
                                    }
                                    break 'b_1ff574;
                                }
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x40);
        break 'b_1ff578;
    }
    c.cycles += 6;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_3(c, SP, 0x50));
    c.pc = t;
    return;
}

/// `void OVScreenRacePre_Init(OVStateRacePre* state)` (0x1fb6b8, HEAT pre-event
/// screen `onCreateScreen`; `a0` is kept in `s5`): creates and starts the two
/// UI screens (the first one is stored at `state+0x40`), runs
/// `OVScreenOverlayMenu_Init`, fetches the BE interfaces (new-player, char
/// display, race, score), writes the track name / event-name line, shows the
/// rider-count objects (0x46fbd8/e8/f8 by `*(player_cfg)`, hiding 0x4a2148 or
/// setting its text for the game mode at `0x535c11`), sets the caption by
/// event kind, fills the six rider rows (names, with the player name for the
/// human riders in mode 2), the best-time text, plays FE sound 0xe and the
/// race-intro announcer line.
///
/// The body keeps the generated block structure, registers and cycle
/// charges (register leftovers are visible to the fake callees' call log);
/// every call goes through the function table with its return address.
pub fn ov_screen_race_pre_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fb724: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2f0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x290));
            c.g[S5][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2c0));
            c.g[A0][0] = 0x470000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x2b0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x438));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x2a0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x280));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x270));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x260));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x250), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
            c.g[S0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001fb6f8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fb70c)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S5][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1fb724;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001fb724)?;
                break 'b_1fb724;
            }
        }
        // Second screen (0x46e378 = the overlay-less status screen): create and start its thread.
        'b_1fb760: {
            c.cycles += 11;
            c.g[A0][0] = 0x470000;
            c.g[S0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c88));
            call_ptr(c, 0x00317670, 0x001fb734)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fb748)?;
            let t = c.g[V0][0] == 0u64;

            if t {
                break 'b_1fb760;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001fb760)?;
                break 'b_1fb760;
            }
        }
        // OVScreenOverlayMenu_Init, the BE interfaces, the track-name text, the
        // objective-mode object visibility and caption.
        'b_1fb990: {
            'b_1fb930: {
                c.cycles += 89;
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x0020d1d8, 0x001fb768)?;
                call_ptr(c, 0x0014dc80, 0x001fb770)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x001fb77c)?;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x230), c.g[V0][0]);
                call_ptr(c, 0x0014dc80, 0x001fb784)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x0014dd58, 0x001fb790)?;
                c.g[FP][0] = c.g[V0][0];
                call_ptr(c, 0x0014dc80, 0x001fb798)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x001fb7a4)?;
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dc80, 0x001fb7ac)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x8;
                call_ptr(c, 0x0014dd58, 0x001fb7b8)?;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x234), c.g[V0][0]);
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
                c.g[S6][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                c.g[S4][0] = c.lw(c.g[S6][0] as u32);
                call_ptr(c, 0x00146e98, 0x001fb7d4)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
                c.g[A1][0] = 0x1;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x238), c.g[V0][0]);
                call_ptr(c, 0x00146e98, 0x001fb7e4)?;
                c.g[A0][0] = 0x470000;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x23c), c.g[V0][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c78));
                call_ptr(c, 0x00317670, 0x001fb7f4)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fb800)?;
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S3][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00144bf0, 0x001fb810)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x002c2540, 0x001fb81c)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1ab8));
                c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x00317670, 0x001fb83c)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x001fb84c)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A1][0] = 0x4a0000;
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2140));
                call_ptr(c, 0x002c27c0, 0x001fb868)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0e90, 0x001fb874)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.g[S5][0];
                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x428));
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x0039e8b8, 0x001fb888)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.g[S5][0];
                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x418));
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x0039e8b8, 0x001fb89c)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.g[S5][0];
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x408));
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0039e8b8, 0x001fb8b0)?;
                c.g[V1][0] = 0x530000;
                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x49));
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = 0x4a0000;
                if t {
                    break 'b_1fb930;
                } else {
                    'b_1fb920: {
                        'b_1fb91c: {
                            'b_1fb918: {
                                'b_1fb90c: {
                                    'b_1fb8fc: {
                                        'b_1fb8f8: {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x2;
                                            let t = c.g[S4][0] == c.g[V0][0];
                                            c.g[V0][0] = ((c.g[S4][0] as i64) < 3i64) as u64;
                                            if t {
                                                break 'b_1fb8f8;
                                            } else {
                                                'b_1fb8e8: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V0][0] = 0x3;
                                                    if t {
                                                        break 'b_1fb8e8;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[S4][0] == c.g[V0][0];
                                                        c.g[A1][0] = 0x4a0000;
                                                        if t {
                                                            break 'b_1fb90c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A0][0] = c.g[S5][0];
                                                            if t {
                                                                break 'b_1fb920;
                                                            } else {
                                                                break 'b_1fb8e8;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = (c.g[S4][0] as i64) < 0;
                                                c.g[A1][0] = c.g[S3][0];
                                                if t {
                                                    break 'b_1fb918;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;

                                                    if t {
                                                        break 'b_1fb8fc;
                                                    } else {
                                                        break 'b_1fb8f8;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A1][0] = c.g[S2][0];
                                        break 'b_1fb8fc;
                                    }
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S5][0];
                                    call_ptr(c, 0x0039e948, 0x001fb904)?;
                                    let t = true;
                                    c.g[A1][0] = 0x4a0000;
                                    if t {
                                        break 'b_1fb91c;
                                    } else {
                                        break 'b_1fb90c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A0][0] = c.g[S5][0];
                                call_ptr(c, 0x0039e948, 0x001fb918)?;
                                break 'b_1fb918;
                            }
                            c.cycles += 1;
                            c.g[A1][0] = 0x4a0000;
                            break 'b_1fb91c;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S5][0];
                        break 'b_1fb920;
                    }
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2148));
                    call_ptr(c, 0x0039e8b8, 0x001fb928)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1fb990;
                    } else {
                        break 'b_1fb930;
                    }
                }
            }
            'b_1fb978: {
                c.cycles += 10;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2148));
                call_ptr(c, 0x00317670, 0x001fb938)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fb944)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x49));
                c.g[V0][0] = 0x2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x470000;
                    break 'b_1fb978;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3f8));
                    call_ptr(c, 0x00317670, 0x001fb964)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001fb970)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1fb990;
                    } else {
                        break 'b_1fb978;
                    }
                }
            }
            c.cycles += 6;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3e0));
            call_ptr(c, 0x00317670, 0x001fb980)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a1000, 0x001fb98c)?;
            c.g[A0][0] = 0x470000;
            break 'b_1fb990;
        }
        // Caption text id by event kind (s4 = 1 or 2: special strings).
        'b_1fba04: {
            'b_1fb9ec: {
                'b_1fb9d0: {
                    'b_1fb9c0: {
                        c.cycles += 9;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3c8));
                        call_ptr(c, 0x00317670, 0x001fb998)?;
                        c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fb9a4)?;
                        c.g[S3][0] = c.g[V0][0];
                        c.g[V0][0] = 0x1;
                        let t = c.g[S4][0] != c.g[V0][0];
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_1fb9c0;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = 0x470000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3b8));
                            if t {
                                break 'b_1fb9d0;
                            } else {
                                break 'b_1fb9c0;
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x470000;
                        break 'b_1fb9ec;
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3a0));
                        break 'b_1fb9d0;
                    }
                }
                c.cycles += 7;
                call_ptr(c, 0x00317670, 0x001fb9d8)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x001fb9e4)?;
                let t = true;
                c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
                if t {
                    break 'b_1fba04;
                } else {
                    break 'b_1fb9ec;
                }
            }
            c.cycles += 6;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x388));
            call_ptr(c, 0x00317670, 0x001fb9f4)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a1000, 0x001fba00)?;
            c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
            break 'b_1fba04;
        }
        c.cycles += 13;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x18));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x248), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x244), c.g[S6][0]);
        c.g[S2][0] = 0u64;
        c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
        c.g[S7][0] = 0x4a0000;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x240), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x248));
        // Six rider rows (s2 = row): character display names, with the player name
        // for the human riders in multi-player mode (+0x49 of 0x535bc8 == 2).
        'l_1fba38: loop {
            'b_1fbb3c: {
                'b_1fbb24: {
                    c.cycles += 22;
                    c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S6][0] = c.g[A2][0];
                    c.g[S1][0] = c.lw(c.g[V1][0] as u32);
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x244));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2170));
                    call_ptr(c, 0x00417828, 0x001fba68)?;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001fba70)?;
                    c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fba7c)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x240));
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        break 'b_1fbb24;
                    } else {
                        'b_1fbb04: {
                            c.cycles += 6;
                            c.g[A0][0] = 0x530000;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                            c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                            c.g[A0][0] = 0x2;
                            if c.g[V1][0] != c.g[A0][0] {
                                c.g[A1][0] = c.g[S0][0];
                                break 'b_1fbb04;
                            } else {
                                'b_1fbabc: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x238));
                                    let t = c.g[S2][0] == c.g[V0][0];
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x23c));
                                    if t {
                                        break 'b_1fbabc;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S2][0] != c.g[V1][0];
                                        c.g[A1][0] = c.g[S0][0];
                                        if t {
                                            break 'b_1fbb04;
                                        } else {
                                            break 'b_1fbabc;
                                        }
                                    }
                                }
                                c.cycles += 18;
                                c.g[A2][0] = c.g[S1][0];
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A0][0] = c.g[FP][0];
                                call_ptr(c, 0x0014eec8, 0x001fbacc)?;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = c.g[S2][0];
                                call_ptr(c, 0x00146f88, 0x001fbadc)?;
                                c.g[A0][0] = c.g[S4][0];
                                c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x24f0));
                                c.g[A2][0] = c.g[S0][0];
                                c.g[A3][0] = c.g[V0][0];
                                call_ptr(c, 0x00417828, 0x001fbaf0)?;
                                c.g[A0][0] = c.g[S3][0];
                                c.g[A1][0] = c.g[S4][0];
                                call_ptr(c, 0x003a0c50, 0x001fbafc)?;
                                let t = true;
                                c.g[S2][0] = c.g[S6][0];
                                if t {
                                    break 'b_1fbb3c;
                                } else {
                                    break 'b_1fbb04;
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = c.g[FP][0];
                        c.g[A2][0] = c.g[S1][0];
                        call_ptr(c, 0x0014eec8, 0x001fbb10)?;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a0c50, 0x001fbb1c)?;
                        let t = true;
                        c.g[S2][0] = c.g[S6][0];
                        if t {
                            break 'b_1fbb3c;
                        } else {
                            break 'b_1fbb24;
                        }
                    }
                }
                c.cycles += 6;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001fbb38)?;
                c.g[S2][0] = c.g[S6][0];
                break 'b_1fbb3c;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 6i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x248));
                if event_due(c) {
                    deliver_event_here(c, 0x001fba38);
                }
                continue 'l_1fba38;
            } else {
                // High score text, FE sound 0xe and the race-intro announcer line.
                c.cycles += 36;
                c.g[A0][0] = 0x4a0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24f8));
                call_ptr(c, 0x00317670, 0x001fbb54)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fbb60)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x234));
                c.g[S3][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00155130, 0x001fbb70)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0020a1a8, 0x001fbb78)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a0c50, 0x001fbb84)?;
                call_ptr(c, 0x0028b180, 0x001fbb8c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0xe;
                call_ptr(c, 0x00294f78, 0x001fbb98)?;
                call_ptr(c, 0x0028b180, 0x001fbba0)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x002a31c0, 0x001fbba8)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x2b0));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x2a0));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x290));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x280));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x270));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x260));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x250));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2f0));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `void cOVState_HEAT_PREEVENT_onGainFocus(cOVState* self)` (0x1fbbd8, `OVScreenRacePre_OnActivate`).
///
/// The pre-race overlay gains focus: runs the base `cUIState::onGainFocus`,
/// then shows the "RIDERPCT_D" rider name text (`sprintf` + `triggerTextFade`)
/// for human rider 1, and for rider 2 as well when the game path
/// (`newRace.cGamePath`, byte 0x535c11) is 2 (split screen). Finally sets the
/// game-comm overlay timeout (`+0x64`) to 1200 when a `cGameComm` exists.
pub fn ov_screen_race_pre_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fbca0: {
            c.cycles += 38;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            call_ptr(c, 0x0039e4c0, 0x001fbbf8)?;
            call_ptr(c, 0x0014dc80, 0x001fbc00)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x001fbc0c)?;
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x001fbc14)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x001fbc20)?;
            c.g[V1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S1][0];
            c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2170));
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x00146e98, 0x001fbc34)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x00417828, 0x001fbc44)?;
            c.g[V1][0] = 0x470000;
            c.g[A0][0] = c.g[S0][0];
            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1b18));
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S2][0];
            call_ptr(c, 0x0020a6f0, 0x001fbc5c)?;
            c.g[V1][0] = 0x530000;
            c.g[A0][0] = 0x2;
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
            let t = c.g[V0][0] != c.g[A0][0];
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            if t {
                break 'b_1fbca0;
            } else {
                c.cycles += 12;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x00146e98, 0x001fbc7c)?;
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00417828, 0x001fbc8c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x0020a6f0, 0x001fbc9c)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                break 'b_1fbca0;
            }
        }
        'b_1fbcac: {
            c.cycles += 2;
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = 0x4b0;
            if t {
                break 'b_1fbcac;
            } else {
                c.cycles += 1;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[V0][0]);
                break 'b_1fbcac;
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenRacePre_dtor(this)` (0x221df0): sets the vtable word at +8 to the
/// UIState vtable (0x474e08), then calls `UIState_dtor(this, a1)`.
pub fn ov_screen_race_pre_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x470000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4e08));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0022_1e0c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FE_GetEventNameText(kind)` (0x001fd190): the pause-screen text of an event kind 6..=11
/// (jump table): hashes a name through the world's string table. The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn fe_get_event_name_text(c: &mut Cpu) {
    if c.pc != 0x001fd190 {
        return ee::s04::f_001fd190__generated(c);
    }
    guest(c, |c| {
        'b_1fd258: {
            'b_1fd250: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[V0][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_1fd250;
                } else {
                    'b_1fd224: {
                        'b_1fd218: {
                            'b_1fd208: {
                                'b_1fd1f8: {
                                    'b_1fd1e8: {
                                        'b_1fd1d8: {
                                            c.cycles += 7;
                                            c.g[V0][0] = 0x470000;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc0));
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                            let t = c.g[A0][0] as u32;
                                            match t {
                                                0x001fd1c8 => {
                                                    c.cycles += 4;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    c.g[A0][0] = 0x470000;
                                                    let t = true;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_sub(0x130));
                                                    if t {
                                                        break 'b_1fd224;
                                                    } else {
                                                        break 'b_1fd1d8;
                                                    }
                                                }
                                                0x001fd1d8 => {
                                                    break 'b_1fd1d8;
                                                }
                                                0x001fd1e8 => {
                                                    break 'b_1fd1e8;
                                                }
                                                0x001fd1f8 => {
                                                    break 'b_1fd1f8;
                                                }
                                                0x001fd208 => {
                                                    break 'b_1fd208;
                                                }
                                                0x001fd218 => {
                                                    break 'b_1fd218;
                                                }
                                                _ => {
                                                    c.jump_out(t, 0x001fd190, 0x001fd268);
                                                    return Ok(());
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[A0][0] = 0x470000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x120));
                                        if t {
                                            break 'b_1fd224;
                                        } else {
                                            break 'b_1fd1e8;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[A0][0] = 0x470000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x110));
                                    if t {
                                        break 'b_1fd224;
                                    } else {
                                        break 'b_1fd1f8;
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A0][0] = 0x470000;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xf8));
                                if t {
                                    break 'b_1fd224;
                                } else {
                                    break 'b_1fd208;
                                }
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A0][0] = 0x470000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xe8));
                            if t {
                                break 'b_1fd224;
                            } else {
                                break 'b_1fd218;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xd8));
                        break 'b_1fd224;
                    }
                    c.cycles += 11;
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001fd238);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x001fd238)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fd248);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x001fd248)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_1fd258;
                    } else {
                        break 'b_1fd250;
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1fd258;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::fe::uitemplatemap::tests::oc797_install_fakes;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_2 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear;

    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_2 as globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_8;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::festatebonusmaterial::tests::str_hash;
    pub(crate) use crate::game::fe::festatecharequipdetail::tests::world;

    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_6;

    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot;
    pub(crate) use crate::game::fe::festaterewards::tests::strict;

    pub(crate) use crate::game::fe::ovstatemission::tests::str_hash as str_hash_2;
    pub(crate) use crate::game::fe::ovstateprofile::tests::hash_object;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_3;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base_14 as base_7;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::check3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::LEAF;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_2;

    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_5;

    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;

    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn mc_manager_set_mode_matches() {
        check(
            ssx3_ee::s06::f_0023d570,
            mc_manager_set_mode,
            300,
            &|c, seed| {
                let mut r = Rng(0x23d5_7000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = match seed % 5 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => ((r.next_u32() as u64) << 32) | (r.next_u32() as u64 | 2),
                    _ => 0x1_0000_0000,
                };
                c.bus.write32(GAME_GP - 0x512c, r.edge_f32());
                c.bus.write32(DATA + 0x108, r.next_u32());
                c.bus.write32(DATA + 0x110, r.next_u32());
                c.bus.write32(DATA + 0x114, r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_reward_list_on_activate_matches() {
        check(
            ee::s05::f_001ff6d0__generated,
            ov_screen_reward_list_on_activate,
            20,
            &|c, seed| {
                machine(c, seed, 0x1ff6d0, 6);
                globals(c);
                clear(c, DATA, 0x400);
                c.g[A0][0] = DATA as u64;
                let _ = seed;
            },
        );
    }

    #[test]
    fn ov_multi_freestyle_pre_on_deactivate_matches() {
        check(
            ssx3_ee::s04::f_001fd118__generated,
            ov_multi_freestyle_pre_on_deactivate,
            SEEDS,
            &|c, seed| {
                let _r = machine(c, seed, 0xfd118, 1);
                be_library(c);
                world(c);
                c.bus.write32(GAME_GP + 0x410, DATA + 0x2000);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x204),
                    if seed % 2 == 0 { DATA + 0x2400 } else { 0 },
                );
                c.g[A0][0] = sx(DATA + 0x300);
            },
        );
    }

    /// A non-null, zeroed stand-in for the BE-library singleton at
    /// `gp - 0x1f04` (its lazy constructor would otherwise load from disc).
    pub(crate) const BE_LIBRARY: u32 = DATA + 0x1_f000;

    /// A small zeroed world (`world -> +0x84 -> +0xc`, rider count `+0x7c`).
    pub(crate) const WORLD: u32 = DATA + 0x2_0000;

    pub(crate) fn globals_2(c: &mut Cpu) {
        c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
        c.bus.write32(GAME_GP - 0x848, WORLD);
        let game = WORLD + 0x1000;
        c.bus.write32(WORLD + 0x84, game);
        let race = WORLD + 0x2000;
        c.bus.write32(game + 0xc, race);
        c.bus.write32(race + 0x7c, 0);
    }

    // ------------------------------------------------------ overlay screens

    pub(crate) fn ov_pre_update_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            // Zeroed BE library: no disc load on first use.
            c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
            c.g[A0][0] = sx(DATA);
            match seed % 3 {
                0 => c.bus.write32(GAME_GP - 0x204, 0),
                1 => {
                    c.bus.write32(GAME_GP - 0x204, DATA + 0x3000);
                    c.bus.write32(DATA + 0x3064, 1);
                }
                _ => {
                    c.bus.write32(GAME_GP - 0x204, DATA + 0x3000);
                    c.bus.write32(DATA + 0x3064, 0);
                }
            }
        });
    }

    #[test]
    fn ov_screen_race_pre_update_matches() {
        ov_pre_update_test(
            ssx3_ee::s04::f_001fbce0__generated,
            ov_screen_race_pre_update,
            0x28,
        );
    }

    #[test]
    fn ov_screen_multi_freestyle_pre_update_matches() {
        ov_pre_update_test(
            ssx3_ee::s04::f_001fd150__generated,
            ov_screen_multi_freestyle_pre_update,
            0x29,
        );
    }

    #[test]
    fn ov_screen_multi_freestyle_pre_on_activate_matches() {
        check(
            ssx3_ee::s04::f_001fd0d8__generated,
            ov_screen_multi_freestyle_pre_on_activate,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x2a, 1);
                globals_2(c);
                c.g[A0][0] = sx(DATA);
                if seed % 2 == 0 {
                    c.bus.write32(GAME_GP - 0x204, 0);
                } else {
                    c.bus.write32(GAME_GP - 0x204, DATA + 0x3000);
                }
            },
        );
    }

    pub(crate) fn save_screen_ctor_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 30, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            // Zeroed BE library: no disc load on first use.
            c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x3000);
            c.g[A2][0] = (seed % 5) as u64;
        });
    }

    #[test]
    fn ov_screen_save_replay_ctor_matches() {
        save_screen_ctor_test(
            ssx3_ee::s05::f_001fdbf0__generated,
            ov_screen_save_replay_ctor,
            0x2b,
        );
    }

    #[test]
    fn ov_screen_save_records_ctor_matches() {
        save_screen_ctor_test(
            ssx3_ee::s05::f_001fe968__generated,
            ov_screen_save_records_ctor,
            0x2c,
        );
    }

    pub(crate) fn close_top_state_once_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            c.g[A0][0] = sx(DATA);
            c.bus
                .write32(DATA + 0x1ac, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(DATA + 0x10, DATA + 0x3000);
        });
    }

    #[test]
    fn ov_screen_save_replay_v33_matches() {
        close_top_state_once_test(
            ssx3_ee::s05::f_001fdf00__generated,
            ov_screen_save_replay_v33,
            0x2d,
        );
    }

    #[test]
    fn ov_screen_save_records_v33_matches() {
        close_top_state_once_test(
            ssx3_ee::s05::f_001fec60__generated,
            ov_screen_save_records_v33,
            0x2e,
        );
    }

    #[test]
    fn ov_screen_reward_list_on_widget_action_matches() {
        check(
            ssx3_ee::s05::f_001ff700__generated,
            ov_screen_reward_list_on_widget_action,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1ff700, 0);
                be_library_2(c);
                c.g[A0][0] = sx(DATA + 0x100);
                // No widget, or a widget with an action other than "confirm":
                // both return without touching the BE library.
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x1400) };
                c.g[A2][0] = if seed % 4 == 1 { 5 } else { 0 };
            },
        );
    }

    #[test]
    fn ov_screen_be_options_v22_matches() {
        check(
            ee::s04::f_001faaf0__generated,
            ov_screen_be_options_v22,
            200,
            &|c, seed| {
                setup(c, seed, 33);
                c.g[A2][0] = [4u64, 0, 9][(seed % 3) as usize];
            },
        );
    }

    #[test]
    fn ov_screen_be_options_v21_matches() {
        check(
            ee::s04::f_001fab08__generated,
            ov_screen_be_options_v21,
            200,
            &|c, seed| {
                setup(c, seed, 34);
                c.g[A2][0] = if seed % 2 == 0 { 0 } else { seed as u64 };
            },
        );
    }

    #[test]
    fn ov_screen_race_pre_on_deactivate_matches() {
        check(
            ee::s04::f_001fbcc8__generated,
            ov_screen_race_pre_on_deactivate,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 35);
                if seed % 3 == 0 {
                    gpf(c, -0x204, 0);
                } else {
                    gpf(c, -0x204, AUX);
                    put(c, AUX + 0x64, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ov_screen_rival_pre_on_widget_notify_matches() {
        check(
            ee::s05::f_001fdb78__generated,
            ov_screen_rival_pre_on_widget_notify,
            200,
            &|c, seed| {
                setup(c, seed, 36);
                c.g[A2][0] = [6u64, 0, 9][(seed % 3) as usize];
            },
        );
    }

    #[test]
    fn ov_screen_save_replay_on_child_event_matches() {
        check(
            ee::s05::f_001fe8f0__generated,
            ov_screen_save_replay_on_child_event,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x1f_e8f0, 3);
                world_2(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x2200) as u64;
                c.g[A2][0] = if seed % 2 == 0 { 0x16 } else { 0x11 };
                w(c, DATA + 0x2000 + 0x19c, if seed % 3 == 0 { 5 } else { 4 });
                c.pc = 0x001f_e8f0;
            },
        );
    }

    #[test]
    fn ov_screen_be_options_on_widget_notify_matches() {
        check(
            ee::s04::f_001faa78__generated,
            ov_screen_be_options_on_widget_notify,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1faa78, 2);
                be_library_3(c);
                let hash = str_hash(c, 0x004a_24e8);
                let menu = DATA;
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = menu as u64;
                c.g[A2][0] = (seed % 12) as u64;
                c.g[A3][0] = (seed % 4) as u64;
                let s = DATA + 0x2000;
                let a = DATA + 0x2100;
                w(c, menu + 0x74 + 4, a);
                w(c, a + 4, s);
                w(c, a, 0);
                w(c, s + 4, s);
                w(c, s, 0);
                w(c, a + 0x38, if seed % 3 == 0 { hash } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn ov_screen_save_replay_v38_matches() {
        check(
            ee::s05::f_001fe5b0__generated,
            ov_screen_save_replay_v38,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1fe5b0, 4);
                let obj = DATA + 0x1000;
                let screen = DATA + 0x2000;
                let node = DATA + 0x3000;
                let vt = DATA + 0x4_0000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, obj + 0x40, screen);
                hash_object(c, screen, if seed % 5 == 0 { 0 } else { node });
                w(c, node + 0x8, vt);
                slot(c, vt, 0x48, 6, FAKE);
                w(c, obj + 0x220, r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_save_records_v38_matches() {
        check(
            ee::s05::f_001ff278__generated,
            ov_screen_save_records_v38,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1ff278, 4);
                let obj = DATA + 0x1000;
                let screen = DATA + 0x2000;
                let node = DATA + 0x3000;
                let vt = DATA + 0x4_0000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, obj + 0x40, screen);
                hash_object(c, screen, if seed % 5 == 0 { 0 } else { node });
                w(c, node + 0x8, vt);
                slot(c, vt, 0x48, 6, FAKE);
                w(c, obj + 0x220, r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_in_game_error_update_server_timeout_text_matches() {
        check(
            ee::s04::f_001fb180__generated,
            ov_screen_in_game_error_update_server_timeout_text,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1fb180, 4);
                let obj = DATA + 0x1000;
                let screen = DATA + 0x2000;
                let node = DATA + 0x3000;
                let vt = DATA + 0x4_0000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x40, if seed % 4 == 0 { 0 } else { screen });
                w(c, obj + 0xa0, (seed % 7) as u32);
                hash_object(c, screen, if seed % 5 == 0 { 0 } else { node });
                w(c, node + 0x8, vt);
                slot(c, vt, 0x48, 6, FAKE);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn ov_screen_in_game_error_update_opponent_disconnected_text_matches() {
        check(
            ssx3_ee::s04::f_001fb218__generated,
            ov_screen_in_game_error_update_opponent_disconnected_text,
            12,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x001f_b218);
                let self_obj = DATA + 0x100;
                let screen = DATA + 0x2000;
                let objects = DATA + 0x2100;
                let obj = DATA + 0x2200;
                let vtable = DATA + 0x3000;
                clear_2(c, self_obj, 0x100);
                clear_2(c, screen, 0x80);
                clear_2(c, objects, 0x20);
                clear_2(c, obj, 0x100);
                clear_2(c, vtable, 0x80);
                w(c, self_obj + 0xa0, r.next_u32());
                if seed % 4 != 0 {
                    w(c, self_obj + 0x40, screen);
                    w(c, screen + 0x3c, 1);
                    w(c, screen + 0x38, objects);
                    w(c, objects, obj);
                    let hv2 = str_hash_2(c, 0x0046_fb18) as u32;
                    w(c, obj + 0x38, hv2);
                    w(c, obj + 8, vtable);
                    c.bus.write16(vtable + 0x48, 0);
                    w(c, vtable + 0x4c, crate::difftest::fakes::FAKE);
                    w(c, crate::difftest::fakes::LOG_RESULTS, 1);
                }
                c.g[A0][0] = sx(self_obj);
            },
        );
    }

    #[test]
    fn ov_screen_save_replay_v39_matches() {
        check(
            ssx3_ee::s05::f_001fe648__generated,
            ov_screen_save_replay_v39,
            20,
            &|c, seed| {
                let _r = base_3(c, seed, 0x1fe648, 2);
                let this = DATA + 0x1000;
                let screen = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A2][0] = (seed & 1) as u64;
                w_2(c, this + 0x40, screen);
                hash_object(c, screen, DATA + 0x3000);
                w_2(c, DATA + 0x3000 + 8, DATA + 0x3100);
                c.bus.write16(DATA + 0x3148, 0x48);
                w_2(c, DATA + 0x314c, 0);
            },
        );
    }

    #[test]
    fn ov_screen_rival_pre_init_matches() {
        check(
            ee::s05::f_001fd268__generated,
            ov_screen_rival_pre_init,
            40,
            &|c, seed| {
                alloc_base(c, seed, 0x1fd2, 5);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x10, DATA + 0x400);
                w(c, DATA + 0x400, DATA + 0x500);
            },
        );
    }

    #[test]
    fn ov_screen_in_game_error_show_cancel_game_matches() {
        check(
            ee::s04::f_001faff8__generated,
            ov_screen_in_game_error_show_cancel_game,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1faf, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
                let screen = DATA + 0x2000;
                let obj = DATA + 0x2400;
                let menu = DATA + 0x2800;
                c.bus.write32(this + 0x40, screen);
                let vt = vtable(c, &mut r, &[0x40, 0x48]);
                c.bus.write32(screen + 0x40, obj);
                c.bus.write32(screen + 0x48, menu);
                c.bus.write32(obj + 8, vt);
                c.bus.write32(menu + 8, vt);
            },
        );
    }

    #[test]
    fn ovscreeningameerror_show_continue_button_matches() {
        check(
            ee::s04::f_001fb378__generated,
            ovscreeningameerror_show_continue_button,
            8,
            &|c, seed| {
                let _ = base_5(c, seed, 0x1fb378, 2);
                let this = DATA;
                clear_3(c, this, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, this + 0x40, 0);
            },
        );
    }

    #[test]
    fn ovscreen_in_game_error_update_matches() {
        strict(
            ee::s04::f_001faf10__generated,
            ovscreen_in_game_error_update,
            120,
            &|c, seed| {
                let _ = base_6(c, seed, 0x1faf, 6);
                c.g[A0][0] = DATA as u64;
                let err = DATA + 0x300;
                w(
                    c,
                    GAME_GP.wrapping_sub(0x204),
                    if seed % 8 == 0 { 0 } else { err },
                );
                w(c, DATA + 0x9c, if seed % 7 == 0 { 0 } else { 1 });
                w(
                    c,
                    err + 0x138,
                    match seed % 3 {
                        0 => 0,
                        1 => 5,
                        _ => 0xffff_ffff,
                    },
                );
                w(c, err + 0x13c, if seed % 4 == 0 { 0 } else { 1 });
                w(c, err + 0x80, if seed % 5 == 0 { 0 } else { 1 });
                w(c, err + 0xa0, if seed % 6 == 0 { 0 } else { 1 });
                w(c, err + 0x24, if seed % 9 == 0 { 0 } else { 1 });
                // The screen and the "ping" object found by hash.
                w(c, DATA + 0x40, DATA + 0x400);
                let obj = DATA + 0x500;
                let vt = DATA + 0x1000;
                w(c, obj + 8, vt);
                w(c, vt + 0x48, 0);
                w(c, vt + 0x4c, FAKE);
            },
        );
    }

    /// A screen whose child list matches the three names the highlight
    /// handlers look up (the hashes are computed by the game's own hash
    /// function, so the lookups really find the widgets).
    pub(crate) fn highlight_screen(c: &mut Cpu, names: [u32; 3]) {
        let obj = DATA;
        let screen = DATA + 0x100;
        let count = DATA + 0x200;
        let list = DATA + 0x300;
        c.bus.write32(obj + 0x40, screen);
        c.bus.write32(screen + 0x38, count);
        c.bus.write32(count, 3);
        c.bus.write32(screen + 0x3c, list);
        for (i, name) in names.iter().enumerate() {
            c.g[A0][0] = sx(*name);
            ssx3_ee::s10::f_00317670(c);
            let hash = c.g[V0][0] as u32;
            let w = DATA + 0x1000 + 0x80 * i as u32;
            c.bus.write32(list + 4 * i as u32, w);
            c.bus.write32(w + 0x38, hash);
            c.bus.write32(w + 8, w + 0x40);
            for slot in [0x48u32, 0x58] {
                c.bus.write16(w + 0x40 + slot, 0);
                c.bus.write32(w + 0x40 + slot + 4, LEAF);
            }
            // The clear path also uses the third widget itself as a table.
            c.bus.write32(w + 0x4c, LEAF);
        }
    }

    #[test]
    fn in_game_error_highlight_continue_matches() {
        check3(
            ee::s04::f_001fb458__generated,
            in_game_error_highlight_continue,
            40,
            &|c, seed| {
                let _ = base_7(c, seed, 0x1fb458);
                highlight_screen(c, [0x0046_fb48, 0x0046_fad8, 0x0046_fb00]);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn in_game_error_highlight_cancel_matches() {
        check3(
            ee::s04::f_001fb588__generated,
            in_game_error_highlight_cancel,
            40,
            &|c, seed| {
                let _ = base_7(c, seed, 0x1fb588);
                highlight_screen(c, [0x0046_fa90, 0x0046_fac8, 0x0046_fae8]);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    pub(crate) const SAVE_REC: u32 = DATA + 0x1_0000;

    pub(crate) const SAVE_REC_MSG: u32 = DATA + 0x1_1000;

    pub(crate) const SAVE_MC: u32 = DATA + 0x1_2000;

    #[test]
    fn ov_screen_save_records_on_widget_action_matches() {
        check_all(
            ee::s05::f_001ff3b0__generated,
            ov_screen_save_records_on_widget_action,
            24,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x1ff3_b0, 12);
                c.g[A0][0] = sx(SAVE_REC);
                c.g[A1][0] = sx(if seed % 5 == 0 { 0 } else { SAVE_REC_MSG });
                c.g[A2][0] = sx([5u32, 6, 7, 0, 4][seed as usize % 5]);
                c.bus.write32(SAVE_REC_MSG + 0x18, r.next_u32() % 4);
                c.bus.write32(SAVE_REC + 0x1a8, ((seed / 5) % 3) as u32);
                c.bus.write32(SAVE_REC + 0x1c0, ((seed / 7) % 8) as u32);
                c.bus.write32(SAVE_REC + 0x214, ((seed / 11) % 4) as u32);
                c.bus.write32(SAVE_REC + 0x1c4, r.next_u32() % 4);
                // The movie manager: the object at_3 +0x434 carries the vtable the
                // card-ready check goes through; +0x428 is its state.
                c.bus.write32(SAVE_MC + 0x428, ((seed / 3) % 3) as u32);
                c.bus.write32(SAVE_MC + 0x434, DATA + 0x1_3000);
                c.bus.write32(SAVE_MC + 0x748, 0);
                c.bus.write32(DATA + 0x1_3000 + 8, 0);
                let _ = &mut r;
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fill;
    use crate::difftest::RA as DT_RA;
    pub(crate) use crate::game::fe::fepopupmisc::tests::oc797_arm;

    use std::sync::atomic::AtomicUsize;

    // Fixture objects, all in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    pub(crate) const OC766_BASE: u32 = DATA + 0x20_0000;

    pub(crate) const OC766_CTRL: u32 = OC766_BASE;

    pub(crate) const OC766_MAGIC: u32 = 0x4f43_3736;

    /// Results of the successive `UIEngine_CreateScreen` calls (0 = null).
    pub(crate) const OC766_CREATE: u32 = OC766_BASE + 0x100;

    /// `BENewPlayerInt_GetHumanRider(0)` and `(1)`.
    pub(crate) const OC766_RIDERS: u32 = OC766_BASE + 0x140;

    pub(crate) const OC766_STATE: u32 = OC766_BASE + 0x2000;

    pub(crate) const OC766_WORLD: u32 = OC766_BASE + 0x3000;

    pub(crate) const OC766_CFG: u32 = OC766_BASE + 0x3400;

    pub(crate) const OC766_W84: u32 = OC766_BASE + 0x3800;

    pub(crate) const OC766_W84P: u32 = OC766_BASE + 0x3900;

    pub(crate) const OC766_OVOBJ: u32 = OC766_BASE + 0x3a00;

    pub(crate) const OC766_IFACE: u32 = OC766_BASE + 0x3c00;

    pub(crate) const OC766_IFVT: u32 = OC766_BASE + 0x3d00;

    /// The game-mode byte (`0x535c11`).
    pub(crate) const OC766_MODE: u32 = 0x0053_5c11;

    /// The mailbox word of the fake for `addr`.
    pub(crate) fn oc766_mail(addr: u32) -> u32 {
        OC766_BASE + 0x10_0000 + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    pub(crate) fn oc766_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc766_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc766_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc766_mail(addr), 1);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns
    /// the forced result of its mailbox (0 = the fake's pointer), the next
    /// word of the `CreateScreen` list or the human rider asked for.
    pub(crate) fn oc766_fake_call(c: &mut Cpu, addr: u32) {
        let a1 = c.g[A1][0] as u32;
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc766_mail(addr));
        let create = if addr == 0x0039_7cc0 {
            let k = c.bus.read32(OC766_CREATE + 0x20);
            c.bus.write32(OC766_CREATE + 0x20, k + 1);
            Some(c.bus.read32(OC766_CREATE + 4 * (k & 3)))
        } else {
            None
        };
        let rider = if addr == 0x0014_6e98 {
            Some(c.bus.read32(OC766_RIDERS + 4 * (a1 & 1)))
        } else {
            None
        };
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        if let Some(v) = rider {
            c.g[V0][0] = sx(v);
        } else if let Some(v) = create {
            if v == 0 {
                c.g[V0][0] = 0;
            }
        } else if mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc766_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC766_CTRL) != OC766_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc766_fake_call(c, $addr);
                }
            )*
            fn oc766_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc766_fakes!(
        (
            oc766_fake_317670,
            OC766_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc766_fake_397cc0,
            OC766_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc766_fake_39c870,
            OC766_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc766_fake_14dc80,
            OC766_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc766_fake_14dd58,
            OC766_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc766_fake_146e98,
            OC766_PREV_146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x0014_6e98
        ),
        (
            oc766_fake_39d860,
            OC766_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc766_fake_144bf0,
            OC766_PREV_144BF0,
            ee::s01::OVERRIDE_00144BF0,
            ee::s01::f_00144bf0__generated,
            0x0014_4bf0
        ),
        (
            oc766_fake_2c2540,
            OC766_PREV_2C2540,
            ee::s08::OVERRIDE_002C2540,
            ee::s08::f_002c2540__generated,
            0x002c_2540
        ),
        (
            oc766_fake_2c27c0,
            OC766_PREV_2C27C0,
            ee::s08::OVERRIDE_002C27C0,
            ee::s08::f_002c27c0__generated,
            0x002c_27c0
        ),
        (
            oc766_fake_3a0e90,
            OC766_PREV_3A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a_0e90
        ),
        (
            oc766_fake_39e8b8,
            OC766_PREV_39E8B8,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039_e8b8
        ),
        (
            oc766_fake_39e948,
            OC766_PREV_39E948,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039_e948
        ),
        (
            oc766_fake_3a1000,
            OC766_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc766_fake_417828,
            OC766_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc766_fake_14eec8,
            OC766_PREV_14EEC8,
            ee::s01::OVERRIDE_0014EEC8,
            ee::s01::f_0014eec8__generated,
            0x0014_eec8
        ),
        (
            oc766_fake_146f88,
            OC766_PREV_146F88,
            ee::s01::OVERRIDE_00146F88,
            ee::s01::f_00146f88__generated,
            0x0014_6f88
        ),
        (
            oc766_fake_155130,
            OC766_PREV_155130,
            ee::s01::OVERRIDE_00155130,
            ee::s01::f_00155130__generated,
            0x0015_5130
        ),
        (
            oc766_fake_28b180,
            OC766_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc766_fake_3a0c70,
            OC766_PREV_3A0C70,
            ee::s13::OVERRIDE_003A0C70,
            ee::s13::f_003a0c70__generated,
            0x003a_0c70
        ),
        (
            oc766_fake_2ea820,
            OC766_PREV_2EA820,
            ee::s09::OVERRIDE_002EA820,
            ee::s09::f_002ea820__generated,
            0x002e_a820
        ),
        (
            oc766_fake_2ea860,
            OC766_PREV_2EA860,
            ee::s09::OVERRIDE_002EA860,
            ee::s09::f_002ea860__generated,
            0x002e_a860
        ),
        (
            oc766_fake_39f9d8,
            OC766_PREV_39F9D8,
            ee::s13::OVERRIDE_0039F9D8,
            ee::s13::f_0039f9d8__generated,
            0x0039_f9d8
        ),
        (
            oc766_fake_413af8,
            OC766_PREV_413AF8,
            ee::s15::OVERRIDE_00413AF8,
            ee::s15::f_00413af8__generated,
            0x0041_3af8
        ),
        (
            oc766_fake_287920,
            OC766_PREV_287920,
            ee::s07::OVERRIDE_00287920,
            ee::s07::f_00287920__generated,
            0x0028_7920
        ),
        (
            oc766_fake_29f128,
            OC766_PREV_29F128,
            ee::s08::OVERRIDE_0029F128,
            ee::s08::f_0029f128__generated,
            0x0029_f128
        ),
        (
            oc766_fake_2a4040,
            OC766_PREV_2A4040,
            ee::s08::OVERRIDE_002A4040,
            ee::s08::f_002a4040__generated,
            0x002a_4040
        ),
        (
            oc766_fake_295028,
            OC766_PREV_295028,
            ee::s08::OVERRIDE_00295028,
            ee::s08::f_00295028__generated,
            0x0029_5028
        ),
        (
            oc766_fake_2b1458,
            OC766_PREV_2B1458,
            ee::s08::OVERRIDE_002B1458,
            ee::s08::f_002b1458__generated,
            0x002b_1458
        ),
        (
            oc766_fake_3d8008,
            OC766_PREV_3D8008,
            ee::s14::OVERRIDE_003D8008,
            ee::s14::f_003d8008__generated,
            0x003d_8008
        ),
    );

    /// `OVScreenRacePre_Init`: the two screens created (or null), the heat /
    /// event kind (-1 to 5) picking the shown rider-count object and the caption,
    /// the game mode byte (0, 1, 2: the +0x4a2148 text), 0 to 7 riders in the six
    /// rows (human riders get "name (player)" in mode 2), and the announcer
    /// line's gates; the interface vtable call, the real `OVScreenOverlayMenu_Init`
    /// (`cOVStateOverlay_Init`, 5 rows), `UIText_SetString`, `FE_FormatMinSec`,
    /// `PlayFESound` and `PASayRaceIntro` run on faked callees.
    #[test]
    fn ov_screen_race_pre_init_matches() {
        oc766_install_fakes();
        covered(
            ee::s04::f_001fb6b8__generated,
            ov_screen_race_pre_init,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1fb6b8, 40);
                c.pc = 0x001f_b6b8;
                c.bus.write32(OC766_CTRL, OC766_MAGIC);
                c.g[A0][0] = sx(OC766_STATE);
                c.bus.write32(OC766_STATE + 0x10, r.next_u32());
                // The world.
                c.bus.write32(crate::difftest::GAME_GP - 0x848, OC766_WORLD);
                c.bus.write32(OC766_WORLD + 0xc0, OC766_CFG);
                c.bus.write32(OC766_WORLD + 0x84, OC766_W84);
                c.bus.write32(OC766_WORLD + 0x8c, OC766_IFACE);
                // Heat / event kind and the rider rows.
                let kind = [-1i32, 0, 1, 2, 3, 4][(seed % 6) as usize];
                c.bus.write32(OC766_CFG, kind as u32);
                fill(c, &mut r, OC766_CFG + 4, 0x20);
                // Game mode byte.
                c.bus.write8(OC766_MODE, ((seed / 6) % 3) as u8);
                // Rider count.
                c.bus.write32(OC766_W84 + 0xc, OC766_W84P);
                c.bus.write32(OC766_W84P + 0x78, (seed / 18) % 8);
                // cOVStateOverlay_Init: the cut-scene transition state.
                c.bus.write32(OC766_W84 + 0x64, OC766_OVOBJ);
                c.bus
                    .write32(OC766_OVOBJ + 0x10, [0, 1, 3, 2][((seed / 5) % 4) as usize]);
                // Interface object: vtable slot (+0x20 delta, +0x24 target).
                c.bus.write32(OC766_IFACE + 4, OC766_IFVT);
                c.bus
                    .write16(OC766_IFVT + 0x20, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(OC766_IFVT + 0x24, fakes::FAKE + 0x40);
                // The two screens: null or not.
                for k in 0..4 {
                    let v = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        0x8000_0000 + 0x40 * k
                    };
                    c.bus.write32(OC766_CREATE + 4 * k, v);
                }
                // Human riders.
                for k in 0..2 {
                    let v = if r.next_u32() % 5 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 7
                    };
                    c.bus.write32(OC766_RIDERS + 4 * k, v);
                }
                // Announcer gates: speech on, not event mode 4.
                if (seed / 3) % 3 == 0 {
                    oc766_force_zero(c, 0x0029_f128);
                } else {
                    oc766_force(c, 0x0029_f128, 1);
                }
                if (seed / 9) % 3 == 0 {
                    oc766_force(c, 0x002a_4040, 1);
                } else {
                    oc766_force_zero(c, 0x002a_4040);
                }
                if (seed / 27) % 2 == 0 {
                    oc766_force_zero(c, 0x0029_5028);
                }
                if (seed / 54) % 2 == 0 {
                    oc766_force_zero(c, 0x002b_1458);
                }
            },
        );
    }

    /// The word that arms oc778's callee fakes in a machine's RAM.
    pub(crate) const OC778_CTRL: u32 = DATA + 0x30_0000;

    pub(crate) const OC778_MAGIC: u32 = 0x0778_f4ce;

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    pub(crate) fn oc778_mail(addr: u32) -> u32 {
        DATA + 0x30_1000 + (addr & 0xffc)
    }

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc778_arm(c: &mut Cpu) {
        c.bus.write32(OC778_CTRL, OC778_MAGIC);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    pub(crate) fn oc778_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc778_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                2 => a0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc778_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC778_CTRL) != OC778_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc778_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc778_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc778_fakes!(
        (
            oc778_fk_146e98,
            OC778_PREV_00146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x0014_6e98
        ),
        (
            oc778_fk_14dc80,
            OC778_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc778_fk_14dd58,
            OC778_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc778_fk_14de28,
            OC778_PREV_0014DE28,
            ee::s01::OVERRIDE_0014DE28,
            ee::s01::f_0014de28__generated,
            0x0014_de28
        ),
        (
            oc778_fk_14de68,
            OC778_PREV_0014DE68,
            ee::s01::OVERRIDE_0014DE68,
            ee::s01::f_0014de68__generated,
            0x0014_de68
        ),
        (
            oc778_fk_14eca0,
            OC778_PREV_0014ECA0,
            ee::s01::OVERRIDE_0014ECA0,
            ee::s01::f_0014eca0__generated,
            0x0014_eca0
        ),
        (
            oc778_fk_155a50,
            OC778_PREV_00155A50,
            ee::s01::OVERRIDE_00155A50,
            ee::s01::f_00155a50__generated,
            0x0015_5a50
        ),
        (
            oc778_fk_155e58,
            OC778_PREV_00155E58,
            ee::s01::OVERRIDE_00155E58,
            ee::s01::f_00155e58__generated,
            0x0015_5e58
        ),
        (
            oc778_fk_113b10,
            OC778_PREV_00113B10,
            ee::s00::OVERRIDE_00113B10,
            ee::s00::f_00113b10__generated,
            0x0011_3b10
        ),
        (
            oc778_fk_128958,
            OC778_PREV_00128958,
            ee::s00::OVERRIDE_00128958,
            ee::s00::f_00128958__generated,
            0x0012_8958
        ),
        (
            oc778_fk_1289f0,
            OC778_PREV_001289F0,
            ee::s00::OVERRIDE_001289F0,
            ee::s00::f_001289f0__generated,
            0x0012_89f0
        ),
        (
            oc778_fk_128a10,
            OC778_PREV_00128A10,
            ee::s00::OVERRIDE_00128A10,
            ee::s00::f_00128a10__generated,
            0x0012_8a10
        ),
        (
            oc778_fk_128a48,
            OC778_PREV_00128A48,
            ee::s00::OVERRIDE_00128A48,
            ee::s00::f_00128a48__generated,
            0x0012_8a48
        ),
        (
            oc778_fk_129160,
            OC778_PREV_00129160,
            ee::s00::OVERRIDE_00129160,
            ee::s00::f_00129160__generated,
            0x0012_9160
        ),
        (
            oc778_fk_194738,
            OC778_PREV_00194738,
            ee::s02::OVERRIDE_00194738,
            ee::s02::f_00194738__generated,
            0x0019_4738
        ),
        (
            oc778_fk_1d5038,
            OC778_PREV_001D5038,
            ee::s04::OVERRIDE_001D5038,
            ee::s04::f_001d5038__generated,
            0x001d_5038
        ),
        (
            oc778_fk_1f3700,
            OC778_PREV_001F3700,
            ee::s04::OVERRIDE_001F3700,
            ee::s04::f_001f3700__generated,
            0x001f_3700
        ),
        (
            oc778_fk_20a6f0,
            OC778_PREV_0020A6F0,
            ee::s05::OVERRIDE_0020A6F0,
            ee::s05::f_0020a6f0__generated,
            0x0020_a6f0
        ),
        (
            oc778_fk_25b6d8,
            OC778_PREV_0025B6D8,
            ee::s06::OVERRIDE_0025B6D8,
            ee::s06::f_0025b6d8__generated,
            0x0025_b6d8
        ),
        (
            oc778_fk_266df8,
            OC778_PREV_00266DF8,
            ee::s07::OVERRIDE_00266DF8,
            ee::s07::f_00266df8__generated,
            0x0026_6df8
        ),
        (
            oc778_fk_266e88,
            OC778_PREV_00266E88,
            ee::s07::OVERRIDE_00266E88,
            ee::s07::f_00266e88__generated,
            0x0026_6e88
        ),
        (
            oc778_fk_26f7b8,
            OC778_PREV_0026F7B8,
            ee::s07::OVERRIDE_0026F7B8,
            ee::s07::f_0026f7b8__generated,
            0x0026_f7b8
        ),
        (
            oc778_fk_2790a0,
            OC778_PREV_002790A0,
            ee::s07::OVERRIDE_002790A0,
            ee::s07::f_002790a0__generated,
            0x0027_90a0
        ),
        (
            oc778_fk_2a4cf8,
            OC778_PREV_002A4CF8,
            ee::s08::OVERRIDE_002A4CF8,
            ee::s08::f_002a4cf8__generated,
            0x002a_4cf8
        ),
        (
            oc778_fk_2a77f8,
            OC778_PREV_002A77F8,
            ee::s08::OVERRIDE_002A77F8,
            ee::s08::f_002a77f8__generated,
            0x002a_77f8
        ),
        (
            oc778_fk_2a7890,
            OC778_PREV_002A7890,
            ee::s08::OVERRIDE_002A7890,
            ee::s08::f_002a7890__generated,
            0x002a_7890
        ),
        (
            oc778_fk_2a7bf8,
            OC778_PREV_002A7BF8,
            ee::s08::OVERRIDE_002A7BF8,
            ee::s08::f_002a7bf8__generated,
            0x002a_7bf8
        ),
        (
            oc778_fk_308c60,
            OC778_PREV_00308C60,
            ee::s10::OVERRIDE_00308C60,
            ee::s10::f_00308c60__generated,
            0x0030_8c60
        ),
        (
            oc778_fk_308f38,
            OC778_PREV_00308F38,
            ee::s10::OVERRIDE_00308F38,
            ee::s10::f_00308f38__generated,
            0x0030_8f38
        ),
        (
            oc778_fk_317d70,
            OC778_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc778_fk_317e30,
            OC778_PREV_00317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc778_fk_38d638,
            OC778_PREV_0038D638,
            ee::s12::OVERRIDE_0038D638,
            ee::s12::f_0038d638__generated,
            0x0038_d638
        ),
        (
            oc778_fk_39e2a0,
            OC778_PREV_0039E2A0,
            ee::s13::OVERRIDE_0039E2A0,
            ee::s13::f_0039e2a0__generated,
            0x0039_e2a0
        ),
        (
            oc778_fk_39e4c0,
            OC778_PREV_0039E4C0,
            ee::s13::OVERRIDE_0039E4C0,
            ee::s13::f_0039e4c0__generated,
            0x0039_e4c0
        ),
        (
            oc778_fk_39f238,
            OC778_PREV_0039F238,
            ee::s13::OVERRIDE_0039F238,
            ee::s13::f_0039f238__generated,
            0x0039_f238
        ),
        (
            oc778_fk_39f400,
            OC778_PREV_0039F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc778_fk_3e6448,
            OC778_PREV_003E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
        (
            oc778_fk_417828,
            OC778_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
    );

    /// `OVScreenRacePre_OnActivate`: the game path byte (2 = second rider
    /// text), the game-comm pointer (null or set) and the unwinds of its calls.
    #[test]
    fn ov_screen_race_pre_on_activate_matches() {
        oc778_install_fakes();
        fakes::covered(
            ee::s04::f_001fbbd8__generated,
            ov_screen_race_pre_on_activate,
            240,
            &|c, seed| {
                machine(c, seed, 0x778a, 10);
                oc778_arm(c);
                c.g[A0][0] = sx(DATA + 0x1_0000);
                c.bus.write8(0x0053_5c11, [2u8, 0, 1][(seed % 3) as usize]);
                let comm = if (seed / 3) % 2 == 0 {
                    0
                } else {
                    DATA + 0x2_0000
                };
                c.bus.write32(GAME_GP - 0x204, comm);
            },
        );
    }

    /// `OVScreenRacePre_dtor`: vtable store and the `UIState_dtor` call.
    #[test]
    fn ov_screen_race_pre_dtor_matches() {
        oc797_install_fakes();
        fakes::covered(
            ee::s05::f_00221df0__generated,
            ov_screen_race_pre_dtor,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x797d, 1);
                oc797_arm(c);
                c.g[A0][0] = ptr(&mut r) as u64;
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }

    pub(crate) use crate::game::fe::fereal::tests::oc802_install_fakes;
    pub(crate) use crate::game::fe::fereal::tests::OC802_CTRL;

    pub(crate) use crate::game::fe::fereal::tests::OC802_MAGIC;

    /// `FE_GetEventNameText`: kinds 5..=12 (6..=11 through the table), the
    /// string-table lookup through the world's object.
    #[test]
    fn fe_get_event_name_text_matches() {
        oc802_install_fakes();
        fakes::covered(
            ee::s04::f_001fd190__generated,
            fe_get_event_name_text,
            80,
            &|c, seed| {
                let _ = fakes::machine(c, seed, 0x802b, 4);
                c.bus.write32(OC802_CTRL, OC802_MAGIC);
                let table = [
                    0x001f_d1c8u32,
                    0x001f_d1d8,
                    0x001f_d1e8,
                    0x001f_d1f8,
                    0x001f_d208,
                    0x001f_d218,
                ];
                for (i, t) in table.into_iter().enumerate() {
                    c.bus.write32(0x0046_ff40 + 4 * i as u32, t);
                }
                if seed % 11 == 10 {
                    c.bus
                        .write32(0x0046_ff40 + 4 * (seed / 11 % 6), 0x0012_3450);
                }
                let world = DATA + 0x25_0000;
                let obj = DATA + 0x26_0000;
                let inner = DATA + 0x27_0000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x8c, obj);
                c.bus.write32(obj + 4, inner);
                c.bus.write16(inner + 0x20, (seed * 8) as u16);
                c.bus.write32(inner + 0x24, fakes::FAKE);
                c.g[A0][0] = (5 + seed % 8) as u64;
                c.pc = 0x001f_d190;
            },
        );
    }
}
