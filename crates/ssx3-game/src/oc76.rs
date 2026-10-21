//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for 50 small front-end, script-engine, audio and menu
//! functions, each with a differential test against the generated body.
//!
//! The bulk of them are one- or two-call wrappers around a state machine:
//!
//! - **NetFE child events** (`cFEStateChallengePopup_OnChildEvent`,
//!   `cFEStateChallengeReceivedPopup_OnChildEvent`): kind `0x835` goes to the
//!   state's own v24 handler, anything else to `NetFEState_OnChildEvent`.
//! - **`v24` popups** (`cKeyboardPopup_v24`, `cFEStateNetHelpPopup_v24`): when
//!   the popup is open and has a widget, forward to widget slot 24.
//! - **`cFEStateNetAccountMem` stepping** (`UpdateState08` .. `UpdateState1B`):
//!   compare the account-memory mode (`*(*(gp+0x2e0))`) and advance it.
//! - **Script engine** (`ScriptEngine_TickProcess`, `..._PauseProcess`,
//!   `SeqListenerOnStart/OnCreate`, `ScriptBinding_Bind`,
//!   `ScriptCmdQueue_Count`, `ScriptEngine_SetLocatorTable`) and
//!   **`ScriptList_dtor`**: the sequence-process tables behind the game's
//!   scripting.
//! - **Replay** (`ReplayCache_ctor`, `WorldState_Save`,
//!   `ReplayStream_AllocBuffer`), **audio** (`SSXAudioSystem_PauseSfx`,
//!   `OnScriptRiderSpeech01`, `IsRaceType4or5`, `MusicPlayer_IsSongDataLoaded`)
//!   and **menu** (`MenuSysItem_IsSelectable`, `MenuSysPage_SetRowColor`,
//!   `MenuSysPage_OnDeactivate`, `MenuSysBackItem_ctor`,
//!   `MenuSysActionItem_ctor`).
//!
//! Every function keeps the generated code's cycle charging (block by block,
//! including the cost of the calls' surrounding instructions), its frame,
//! every register the original leaves behind (including the `v0`/`v1` results
//! and the scratch values), and its call protocol ([`crate::call`]: the same
//! `ra` per call, `call_ptr` for the `jalr`/cross-shard calls, and the unwind
//! check after each call).

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001a_f428,
        c_fe_state_challenge_popup_on_child_event,
        "cFEStateChallengePopup_OnChildEvent",
    ),
    (
        0x001b_05e0,
        c_fe_state_challenge_received_popup_on_child_event,
        "cFEStateChallengeReceivedPopup_OnChildEvent",
    ),
    (
        0x001b_3568,
        c_fe_state_chat_room_on_widget_notify,
        "cFEStateChatRoom_OnWidgetNotify",
    ),
    (
        0x001b_78f0,
        c_fe_state_net_account_mem_update_state08,
        "cFEStateNetAccountMem_UpdateState08",
    ),
    (
        0x001b_7a08,
        c_fe_state_net_account_mem_update_state0e,
        "cFEStateNetAccountMem_UpdateState0E",
    ),
    (
        0x001b_7a50,
        c_fe_state_net_account_mem_update_state10,
        "cFEStateNetAccountMem_UpdateState10",
    ),
    (
        0x001b_7c28,
        c_fe_state_net_account_mem_update_state1a,
        "cFEStateNetAccountMem_UpdateState1A",
    ),
    (
        0x001b_7c70,
        c_fe_state_net_account_mem_update_state1b,
        "cFEStateNetAccountMem_UpdateState1B",
    ),
    (0x001b_ee30, c_fe_state_login_ctor, "cFEStateLogin_ctor"),
    (
        0x001c_0418,
        c_fe_state_online_agreement_ctor,
        "cFEStateOnlineAgreement_ctor",
    ),
    (
        0x001c_0ce0,
        c_fe_state_online_agreement_on_widget_created,
        "cFEStateOnlineAgreement_OnWidgetCreated",
    ),
    (0x001c_b3d0, c_keyboard_popup_v24, "cKeyboardPopup_v24"),
    (
        0x001d_9e20,
        c_fe_state_net_help_popup_v24,
        "cFEStateNetHelpPopup_v24",
    ),
    (
        0x001e_4610,
        ov_screen_message_center_on_child_event,
        "OVScreenMessageCenter_OnChildEvent",
    ),
    (
        0x001f_7548,
        ov_screen_bc_start_pop_event_arg,
        "OVScreenBCStart_PopEventArg",
    ),
    (
        0x0020_d190,
        ov_screen_pda_template_close,
        "OVScreenPDATemplate_Close",
    ),
    (0x0020_eb08, hud_meter_dtor, "HUDMeter_dtor"),
    (
        0x0021_08f8,
        race_progress_reset_all,
        "RaceProgress_ResetAll",
    ),
    (0x0023_2ae0, start_screen_ctor, "StartScreen_ctor"),
    (
        0x0023_ca28,
        mc_manager_read_save_by_slot,
        "MCManager_ReadSaveBySlot",
    ),
    (0x0023_d618, mc_manager_update, "MCManager_Update"),
    (0x0025_f430, lobby_man_is_in_room, "LobbyMan_IsInRoom"),
    (
        0x0025_f858,
        lobby_man_reset_chal_state,
        "LobbyMan_ResetChalState",
    ),
    (0x0026_14e8, buddy_man_create_api, "BuddyMan_CreateApi"),
    (0x0026_36a8, net_start_connect, "Net_StartConnect"),
    (0x0026_d0a0, replay_cache_ctor, "ReplayCache_ctor"),
    (0x0026_d7d0, world_state_save, "WorldState_Save"),
    (
        0x0026_e908,
        replay_stream_alloc_buffer,
        "ReplayStream_AllocBuffer",
    ),
    (0x0027_5d90, script_list_dtor, "ScriptList_dtor"),
    (
        0x0027_8a10,
        script_engine_set_locator_table,
        "ScriptEngine_SetLocatorTable",
    ),
    (
        0x0028_2450,
        script_engine_tick_process,
        "ScriptEngine_TickProcess",
    ),
    (
        0x0028_26d8,
        script_engine_pause_process,
        "ScriptEngine_PauseProcess",
    ),
    (
        0x0028_29d0,
        script_engine_seq_listener_on_start,
        "ScriptEngine_SeqListenerOnStart",
    ),
    (
        0x0028_2b40,
        script_engine_seq_listener_on_create,
        "ScriptEngine_SeqListenerOnCreate",
    ),
    (0x0028_2fb8, script_binding_bind, "ScriptBinding_Bind"),
    (0x0028_3d28, script_cmd_queue_count, "ScriptCmdQueue_Count"),
    (
        0x0028_b830,
        snd_bank_is_loaded_with_name,
        "SndBank_IsLoadedWithName",
    ),
    (
        0x0029_ce28,
        ssx_audio_system_pause_sfx,
        "SSXAudioSystem_PauseSfx",
    ),
    (
        0x002a_1b88,
        ssx_audio_system_on_script_rider_speech01,
        "SSXAudioSystem_OnScriptRiderSpeech01",
    ),
    (
        0x002a_4248,
        ssx_audio_system_is_race_type4_or5,
        "SSXAudioSystem_IsRaceType4or5",
    ),
    (
        0x002b_4aa0,
        music_player_is_song_data_loaded,
        "MusicPlayer_IsSongDataLoaded",
    ),
    (0x002c_2688, ustr_cat, "UStr_Cat"),
    (
        0x002c_a320,
        menu_sys_item_is_selectable,
        "MenuSysItem_IsSelectable",
    ),
    (
        0x002c_b880,
        menu_sys_page_set_row_color,
        "MenuSysPage_SetRowColor",
    ),
    (
        0x002c_bfd0,
        menu_sys_page_on_deactivate,
        "MenuSysPage_OnDeactivate",
    ),
    (0x002c_cdf0, menu_sys_back_item_ctor, "MenuSysBackItem_ctor"),
    (
        0x002c_cf08,
        menu_sys_action_item_ctor,
        "MenuSysActionItem_ctor",
    ),
    (
        0x002d_1a30,
        wscript_man_try_run_obj_script3,
        "WScriptMan_TryRunObjScript3",
    ),
    (
        0x002e_4ce8,
        transition_effect_alloc_data,
        "TransitionEffect_AllocData",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    // Front-end state machine (shard 3).
    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::f_001a97b8,
    };
    pub const NET_FE_STATE_CTOR: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::f_001a8500,
    };
    pub const CHAT_ROOM_REFRESH_PLAYER_LIST: Callee = Callee {
        addr: 0x001b_2378,
        f: ee::s03::f_001b2378,
    };
    pub const NET_ACCOUNT_MEM_SET_STATE: Callee = Callee {
        addr: 0x001b_70d0,
        f: ee::s03::f_001b70d0,
    };
    pub const OV_MESSAGE_CENTER_REFRESH_LIST: Callee = Callee {
        addr: 0x001e_48b0,
        f: ee::s04::f_001e48b0,
    };
    pub const OV_MESSAGE_CENTER_UPDATE_COUNT_TEXT: Callee = Callee {
        addr: 0x001e_4758,
        f: ee::s04::f_001e4758,
    };

    // Memory card / app modules (shard 6).
    pub const APP_MODULE_CTOR: Callee = Callee {
        addr: 0x0023_1cd0,
        f: ee::s06::f_00231cd0,
    };
    pub const MC_MANAGER_UPDATE_STATES: Callee = Callee {
        addr: 0x0023_d7d8,
        f: ee::s06::f_0023d7d8,
    };
    pub const MC_MANAGER_UPDATE_MODE1: Callee = Callee {
        addr: 0x0023_d660,
        f: ee::s06::f_0023d660,
    };
    pub const NET_DISCONNECT_ALL: Callee = Callee {
        addr: 0x0025_5f50,
        f: ee::s06::f_00255f50,
    };

    // Replay / script engine (shard 7).
    pub const REPLAY_CACHE_RESET: Callee = Callee {
        addr: 0x0026_d168,
        f: ee::s07::f_0026d168,
    };
    pub const WORLD_STATE_WRITE: Callee = Callee {
        addr: 0x0026_d818,
        f: ee::s07::f_0026d818,
    };
    pub const SCRIPT_LIST_STOP: Callee = Callee {
        addr: 0x0027_6868,
        f: ee::s07::f_00276868,
    };
    pub const SCRIPT_ENGINE_GET_PROCESS: Callee = Callee {
        addr: 0x0028_2cb0,
        f: ee::s07::f_00282cb0,
    };
    pub const IN_SEQ_INSTANCE_UPDATE: Callee = Callee {
        addr: 0x0027_4a30,
        f: ee::s07::f_00274a30,
    };
    pub const IN_SEQ_INSTANCE_PAUSE: Callee = Callee {
        addr: 0x0027_49e8,
        f: ee::s07::f_002749e8,
    };
    pub const SCRIPT_ENGINE_ALLOC_PROCESS: Callee = Callee {
        addr: 0x0028_2c38,
        f: ee::s07::f_00282c38,
    };
    pub const IN_SEQ_INSTANCE_SET_USER_DATA: Callee = Callee {
        addr: 0x0027_4c10,
        f: ee::s07::f_00274c10,
    };
    pub const SCRIPT_ENGINE_GET_INSTANCE: Callee = Callee {
        addr: 0x0027_d2e8,
        f: ee::s07::f_0027d2e8,
    };
    pub const SCRIPT_ENGINE_ACTIVATE_BINDING: Callee = Callee {
        addr: 0x0027_d400,
        f: ee::s07::f_0027d400,
    };
    pub const SCRIPT_CMD_QUEUE_IS_FULL: Callee = Callee {
        addr: 0x0028_3c58,
        f: ee::s07::f_00283c58,
    };

    // Audio (shard 8).
    pub const BANK_MONITOR_PAUSE_ALL: Callee = Callee {
        addr: 0x002a_d2a8,
        f: ee::s08::f_002ad2a8,
    };
    pub const RIDER_SAY_20D3: Callee = Callee {
        addr: 0x002a_1138,
        f: ee::s08::f_002a1138,
    };
    pub const RIDER_SAY_20D4: Callee = Callee {
        addr: 0x002a_1280,
        f: ee::s08::f_002a1280,
    };
    pub const USTR_COPY: Callee = Callee {
        addr: 0x002c_2508,
        f: ee::s08::f_002c2508,
    };

    // Menu items (shard 9).
    pub const MENU_SYS_ITEM_IS_VISIBLE: Callee = Callee {
        addr: 0x002c_a378,
        f: ee::s09::f_002ca378,
    };
    pub const MENU_SYS_ITEM_IS_DISABLED: Callee = Callee {
        addr: 0x002c_a3b0,
        f: ee::s09::f_002ca3b0,
    };
    pub const MENU_SYS_ITEM_CTOR: Callee = Callee {
        addr: 0x002c_a258,
        f: ee::s09::f_002ca258,
    };
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    abi::r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// NetFEState child events.

/// A `NetFEState` child event handler: an event whose kind (`event+0x18`) is
/// `0x835` is delivered to the state's own virtual handler (slot 0x108 of the
/// vtable at `self+8`, called with `self` adjusted by the slot's delta), every
/// other kind to `NetFEState_OnChildEvent`. `ret_virtual`/`ret_other` are the
/// original's return addresses (its 0x10 frame holds only `ra`).
fn child_event_dispatch(c: &mut Cpu, ret_virtual: u32, ret_other: u32) -> Flow {
    c.cycles += 6;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V0][0] = 0x835;
    let event = abi::r32(c, A1);
    c.g[V1][0] = Addr(event.wrapping_add(0x18)).lw(c);
    let own_handler = c.g[V1][0] == 0x835;
    c.g[A3][0] = c.g[A0][0];
    if own_handler {
        c.cycles += 7;
        let this = abi::r32(c, A3);
        c.g[V0][0] = Addr(this.wrapping_add(8)).lw(c);
        let vtable = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vtable.wrapping_add(0x108)).lh(c);
        c.g[V1][0] = Addr(vtable.wrapping_add(0x10c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(this.wrapping_add(abi::r32(c, A0)));
        call_ptr(c, target, ret_virtual)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        return Ok(());
    }
    c.cycles += 3;
    c.g[A0][0] = c.g[A3][0];
    callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, ret_other)?;
    frame.restore_ra(c, 0);
    c.cycles += 2;
    frame.close(c);
    Ok(())
}

/// `void cFEStateChallengePopup_OnChildEvent(self, event, arg)` (0x1af428):
/// the challenge popup's child-event handler; a `0x835` event is the popup's
/// own (see [`child_event_dispatch`]).
pub fn c_fe_state_challenge_popup_on_child_event(c: &mut Cpu) {
    guest(c, |c| child_event_dispatch(c, 0x001a_f454, 0x001a_f464))
}

/// `void cFEStateChallengeReceivedPopup_OnChildEvent(self, event, arg)`
/// (0x1b05e0): the "challenge received" popup's child-event handler, the same
/// shape as [`c_fe_state_challenge_popup_on_child_event`].
pub fn c_fe_state_challenge_received_popup_on_child_event(c: &mut Cpu) {
    guest(c, |c| child_event_dispatch(c, 0x001b_060c, 0x001b_061c))
}

/// `int cFEStateChatRoom_OnWidgetNotify(self, sender, event, value)`
/// (0x1b3568): the chat room's widget notification. Only event 4 (a list
/// selection) counts: from the player list (`+0x6fc`) it stores the selected
/// value at `+0x704` and rebuilds the list; from the buddy list (`+0x700`) it
/// stores it at `+0x708`. Returns the handled code 0x101.
pub fn c_fe_state_chat_room_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 4;
        let selection = c.g[A2][0] == 4;
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, A0);
        let sender = c.g[A1][0];
        let value = c.g[A3][0];
        if selection {
            c.cycles += 3;
            c.g[V0][0] = Addr(self_.wrapping_add(0x6fc)).lw(c);
            if sender == c.g[V0][0] {
                c.cycles += 5;
                Addr(self_.wrapping_add(0x704)).sw(c, value);
                c.g[A1][0] = 0;
                callee::CHAT_ROOM_REFRESH_PLAYER_LIST.call(c, 0x001b_3590)?;
                c.g[V0][0] = 0x101;
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.g[V0][0] = Addr(self_.wrapping_add(0x700)).lw(c);
            c.cycles += 2;
            if sender == c.g[V0][0] {
                Addr(self_.wrapping_add(0x708)).sw(c, value);
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 0x101;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cFEStateNetAccountMem state stepping.

/// The account-memory hand-off shared by the five `UpdateState..` wrappers:
/// the state machine's mode is `*(*(gp+0x2e0))`; mode `from_a` advances it to
/// `to_a` (returning to `ret_a`), else mode `from_b` advances it to `to_b`
/// (returning to `ret_b`). Nothing happens for any other mode.
fn net_account_mem_step(
    c: &mut Cpu,
    from_a: u64,
    to_a: u64,
    ret_a: u32,
    from_b: u64,
    to_b: u64,
    ret_b: u32,
) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    c.g[V1][0] = Addr(gp(c, 0x2e0)).lw(c);
    frame.save_ra(c, 0);
    c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
    let mode = c.g[V1][0];
    c.g[V0][0] = mode ^ from_a;
    let first = c.g[V0][0] == 0;
    c.g[V0][0] = mode ^ from_b;
    if first {
        c.cycles += 4;
        c.g[A1][0] = to_a;
        callee::NET_ACCOUNT_MEM_SET_STATE.call(c, ret_a)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        return Ok(());
    }
    let second = c.g[V0][0] == 0;
    c.cycles += 2;
    frame.restore_ra(c, 0);
    if second {
        c.cycles += 3;
        c.g[A1][0] = to_b;
        callee::NET_ACCOUNT_MEM_SET_STATE.call(c, ret_b)?;
        frame.restore_ra(c, 0);
    }
    c.cycles += 2;
    frame.close(c);
    Ok(())
}

/// `void cFEStateNetAccountMem_UpdateState08(self)` (0x1b78f0): mode 8 →
/// state 0xa, mode 9 → state 9.
pub fn c_fe_state_net_account_mem_update_state08(c: &mut Cpu) {
    guest(c, |c| {
        net_account_mem_step(c, 8, 0xa, 0x001b_7914, 9, 9, 0x001b_792c)
    })
}

/// `void cFEStateNetAccountMem_UpdateState0E(self)` (0x1b7a08): mode 0x15 →
/// state 0x10, mode 0x16 → state 0xf.
pub fn c_fe_state_net_account_mem_update_state0e(c: &mut Cpu) {
    guest(c, |c| {
        net_account_mem_step(c, 0x15, 0x10, 0x001b_7a2c, 0x16, 0xf, 0x001b_7a44)
    })
}

/// `void cFEStateNetAccountMem_UpdateState10(self)` (0x1b7a50): mode 0xc →
/// state 0x12, mode 0xd → state 0x11.
pub fn c_fe_state_net_account_mem_update_state10(c: &mut Cpu) {
    guest(c, |c| {
        net_account_mem_step(c, 0xc, 0x12, 0x001b_7a74, 0xd, 0x11, 0x001b_7a8c)
    })
}

/// `void cFEStateNetAccountMem_UpdateState1A(self)` (0x1b7c28): mode 0x10 →
/// state 0x1b, mode 0x11 → state 0x1c.
pub fn c_fe_state_net_account_mem_update_state1a(c: &mut Cpu) {
    guest(c, |c| {
        net_account_mem_step(c, 0x10, 0x1b, 0x001b_7c4c, 0x11, 0x1c, 0x001b_7c64)
    })
}

/// `void cFEStateNetAccountMem_UpdateState1B(self)` (0x1b7c70): mode 0xc →
/// state 0x1d, mode 0xd → state 0x1c.
pub fn c_fe_state_net_account_mem_update_state1b(c: &mut Cpu) {
    guest(c, |c| {
        net_account_mem_step(c, 0xc, 0x1d, 0x001b_7c94, 0xd, 0x1c, 0x001b_7cac)
    })
}

// ---------------------------------------------------------------------------
// cFEStateLogin / cFEStateOnlineAgreement constructors.

/// A `NetFEState`-derived constructor: run `NetFEState_ctor`, install the
/// derived vtable at `+8` and the class id at `+0xc`, and clear two fields.
/// `zero_a`/`zero_b` are the fields the derived class clears.
fn net_fe_state_ctor(
    c: &mut Cpu,
    class_id: u32,
    vtable: u32,
    zero_a: u32,
    zero_b: u32,
    ret: u32,
) -> Flow {
    c.cycles += 18;
    let frame = Frame::open(c, 0x20);
    c.g[A2][0] = 0;
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    callee::NET_FE_STATE_CTOR.call(c, ret)?;
    let self_ = abi::r32(c, S0);
    c.g[V0][0] = sx(class_id);
    c.g[V1][0] = 0x47_0000;
    Addr(self_.wrapping_add(0xc)).sw(c, c.g[V0][0]);
    c.g[V1][0] = sx(vtable);
    Addr(self_.wrapping_add(8)).sw(c, c.g[V1][0]);
    c.g[V0][0] = c.g[S0][0];
    Addr(self_.wrapping_add(zero_a)).sw(c, 0);
    Addr(self_.wrapping_add(zero_b)).sw(c, 0);
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFEStateLogin* cFEStateLogin_ctor(self, arg)` (0x1bee30): the login
/// state's constructor (class id 0x33, vtable 0x468388; clears `+0x6d0` and
/// `+0x6a8`).
pub fn c_fe_state_login_ctor(c: &mut Cpu) {
    guest(c, |c| {
        net_fe_state_ctor(c, 0x33, 0x0046_8388, 0x6d0, 0x6a8, 0x001b_ee48)
    })
}

/// `cFEStateOnlineAgreement* cFEStateOnlineAgreement_ctor(self, arg)`
/// (0x1c0418): the online-agreement state's constructor (class id 0x43,
/// vtable 0x46ac38; clears `+0x6d0` and `+0x6d8`).
pub fn c_fe_state_online_agreement_ctor(c: &mut Cpu) {
    guest(c, |c| {
        net_fe_state_ctor(c, 0x43, 0x0046_ac38, 0x6d0, 0x6d8, 0x001c_0430)
    })
}

/// `void cFEStateOnlineAgreement_OnWidgetCreated(self, item)` (0x1c0ce0):
/// if the item's hash (`+0x38`) is the hash of the agreement text the item
/// gets the "needs update" flag (`+0x14 |= 1`).
pub fn c_fe_state_online_agreement_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x46_0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = sx(0x46_0000u32.wrapping_add(0x19b8));
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        let item = abi::r32(c, S1);
        c.g[S0][0] = Addr(item.wrapping_add(0x38)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001c_0d04)?;
        let differs = c.g[S0][0] != c.g[V0][0];
        frame.restore(c, S0, 0x20);
        if !differs {
            c.cycles += 3;
            c.g[V0][0] = Addr(item.wrapping_add(0x14)).lw(c);
            c.g[V0][0] |= 1;
            Addr(item.wrapping_add(0x14)).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// The `v24` widget-forwarding popups.

/// `cKeyboardPopup_v24`: the keyboard popup's virtual method 24 wrapper: when
/// the popup is open (`+0x438`) and owns a widget (`+0x20`), forward to the
/// widget's own slot 24 (vtable at `widget+8`, delta at `+0xc0`, method at
/// `+0xc4`); otherwise return 0.
pub fn c_keyboard_popup_v24(c: &mut Cpu) {
    guest(c, |c| widget_v24(c, 0x438, 0x001c_b404))
}

/// `cFEStateNetHelpPopup_v24`: the net-help popup's virtual method 24
/// wrapper, with its "popup open" flag at `+0x50`.
pub fn c_fe_state_net_help_popup_v24(c: &mut Cpu) {
    guest(c, |c| widget_v24(c, 0x50, 0x001d_9e54))
}

/// See [`c_keyboard_popup_v24`]: `flag_off` is the "popup is open" field.
fn widget_v24(c: &mut Cpu, flag_off: u32, ret: u32) -> Flow {
    c.cycles += 5;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    let self_ = abi::r32(c, A0);
    c.g[V0][0] = Addr(self_.wrapping_add(flag_off)).lw(c);
    let closed = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if closed {
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        return Ok(());
    }
    c.cycles += 3;
    c.g[A3][0] = Addr(self_.wrapping_add(0x20)).lw(c);
    let no_widget = c.g[A3][0] == 0;
    frame.restore_ra(c, 0);
    if no_widget {
        c.cycles += 2;
        frame.close(c);
        return Ok(());
    }
    c.cycles += 7;
    let widget = abi::r32(c, A3);
    c.g[V1][0] = Addr(widget.wrapping_add(8)).lw(c);
    let vtable = c.g[V1][0] as u32;
    c.g[A0][0] = Addr(vtable.wrapping_add(0xc0)).lh(c);
    c.g[V0][0] = Addr(vtable.wrapping_add(0xc4)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx(widget.wrapping_add(abi::r32(c, A0)));
    call_ptr(c, target, ret)?;
    frame.restore_ra(c, 0);
    c.cycles += 2;
    frame.close(c);
    Ok(())
}

/// `void cFEStatePreviewReward_RequestImage(self, id)` (0x1d2150): ask the
/// front end's image loader (`world+0x7c` tagged with 0x1a70) for `self+0x64`
/// through the six-argument request call, and keep the handle at `+0x6c`.
pub fn c_fe_state_preview_reward_request_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x10);
        c.g[A2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A3][0] = 9;
        c.g[T0][0] = 1;
        let world = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(world.wrapping_add(0x7c)).lw(c);
        c.g[T1][0] = 0;
        let self_ = abi::r32(c, S0);
        c.g[A1][0] = Addr(self_.wrapping_add(0x64)).lw(c);
        let owner = (c.g[A0][0] as u32).wrapping_add(0x1a70);
        c.g[A0][0] = sx(owner);
        call_ptr(c, 0x0019_da10, 0x001d_2184)?;
        Addr(self_.wrapping_add(0x6c)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenMessageCenter_OnChildEvent(self, sender, event, arg)`
/// (0x1e4610): on child event 0x14 the message center rebuilds its list and
/// its counter and scrolls the list (`self+0x10` is the list widget).
pub fn ov_screen_message_center_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x14;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let other = c.g[A2][0] != 0x14;
        c.g[S0][0] = c.g[A0][0];
        let self_ = abi::r32(c, S0);
        if other {
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 8;
        callee::OV_MESSAGE_CENTER_REFRESH_LIST.call(c, 0x001e_4630)?;
        c.g[A0][0] = c.g[S0][0];
        callee::OV_MESSAGE_CENTER_UPDATE_COUNT_TEXT.call(c, 0x001e_4638)?;
        c.g[A0][0] = Addr(self_.wrapping_add(0x10)).lw(c);
        c.g[A1][0] = 1;
        let list = (c.g[A0][0] as u32).wrapping_add(0x18);
        c.g[A0][0] = sx(list);
        call_ptr(c, 0x0039_f190, 0x001e_4648)?;
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenBCStart_PopEventArg(self)` (0x1f7548): pop the pending
/// broadcast-start event argument (the global `gp+0xce8` holds it; `+0x1c4`
/// is the queue) into `+0x9c`, falling back to the global's `+0x2a0` when the
/// queue is empty.
pub fn ov_screen_bc_start_pop_event_arg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, 0xce8)).lw(c);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let queue = (c.g[V0][0] as u32).wrapping_add(0x1c4);
        c.g[A0][0] = sx(queue);
        call_ptr(c, 0x0030_c820, 0x001f_7564)?;
        let self_ = abi::r32(c, S0);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(gp(c, 0xce8)).lw(c);
            c.cycles += 2;
            let fallback = c.g[V0][0] as u32;
            c.g[V1][0] = Addr(fallback.wrapping_add(0x2a0)).lw(c);
            Addr(self_.wrapping_add(0x9c)).sw(c, c.g[V1][0]);
        } else {
            c.cycles += 3;
            let value = c.g[V0][0] as u32;
            c.g[V0][0] = Addr(value).lw(c);
            Addr(self_.wrapping_add(0x9c)).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenPDATemplate_Close()` (0x20d190): unlink and delete the PDA
/// template (`gp-0x9e4`), then clear the global.
pub fn ov_screen_pda_template_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp(c, -0x9e4)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 5;
            call_ptr(c, 0x0039_7740, 0x0020_d1a8)?;
            c.g[A2][0] = Addr(gp(c, -0x9e4)).lw(c);
            let gone = c.g[A2][0] == 0;
            c.g[A1][0] = 3;
            if !gone {
                c.cycles += 5;
                let template = abi::r32(c, A2);
                c.g[V0][0] = Addr(template.wrapping_add(8)).lw(c);
                let vtable = c.g[V0][0] as u32;
                c.g[A0][0] = Addr(vtable.wrapping_add(8)).lh(c);
                c.g[V1][0] = Addr(vtable.wrapping_add(0xc)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx(template.wrapping_add(abi::r32(c, A0)));
                call_ptr(c, target, 0x0020_d1c8)?;
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        Addr(gp(c, -0x9e4)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void HUDMeter_dtor(self, flags)` (0x20eb08): drop the HUD meter
/// (`+0x38` gets its base vtable), decrement the meter count (`gp-0x984`) —
/// clearing the "meters loaded" flag (`gp-0x988`) on the last one — and free
/// `self` when `flags & 1`.
pub fn hud_meter_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr(gp(c, -0x984)).lw(c);
        c.g[V0][0] = 0x47_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x47_0000u32.wrapping_add(0x4230));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let self_ = abi::r32(c, A0);
        Addr(self_.wrapping_add(0x38)).sw(c, c.g[V0][0]);
        let more = c.g[V1][0] != 0;
        Addr(gp(c, -0x984)).sw(c, c.g[V1][0]);
        if !more {
            c.cycles += 1;
            Addr(gp(c, -0x988)).sw(c, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[A1][0] & 1;
        let keep = c.g[V0][0] == 0;
        frame.restore_ra(c, 0);
        if !keep {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e50, 0x0020_eb44)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Race progress reset.

/// `void RaceProgress_ResetAll()` (0x2108f8): clear the six race-progress
/// records at 0x4c8bc8 (0x18 bytes each: an "in use" word then five zero
/// words) and set the "progress initialised" flag (`gp-0x990`).
pub fn race_progress_reset_all(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x4d_0000;
    c.g[A1][0] = 1;
    c.g[V0][0] = sx(0x4d_0000u32.wrapping_sub(0x7438));
    c.g[V1][0] = c.g[V0][0];
    let first = c.g[V1][0] as u32;
    c.g[A0][0] = sx(first.wrapping_add(0x90));
    let one = c.g[A1][0];
    Addr(first).sw(c, one);
    let mut record = first;
    loop {
        c.cycles += 9;
        Addr(record.wrapping_add(4)).sw(c, 0);
        Addr(record.wrapping_add(8)).sw(c, 0);
        Addr(record.wrapping_add(0xc)).sw(c, 0);
        Addr(record.wrapping_add(0x10)).sw(c, 0);
        Addr(record.wrapping_add(0x14)).sw(c, 0);
        record = record.wrapping_add(0x18);
        c.g[V1][0] = sx(record);
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        Addr(record).sw(c, one);
        if event_due(c) {
            deliver_event_here(c, 0x0021_0910);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = 1;
    Addr(gp(c, -0x990)).sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// StartScreen + memory card.

/// `StartScreen* StartScreen_Load(self, arg)` (0x232ae0): the start screen's
/// constructor: `AppModule_ctor`, then the module's id/vtable and a cleared
/// field.
pub fn start_screen_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::APP_MODULE_CTOR.call(c, 0x0023_2afc)?;
        let self_ = abi::r32(c, S0);
        c.g[V1][0] = 0x48_0000;
        Addr(self_.wrapping_add(0x18)).sw(c, c.g[S1][0]);
        c.g[V1][0] = sx(0x48_0000u32.wrapping_sub(0x2830));
        c.g[V0][0] = c.g[S0][0];
        Addr(self_).sw(c, c.g[V1][0]);
        Addr(self_.wrapping_add(0xc)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_ReadSaveBySlot(self, slot, size, dest, ret)` (0x23ca28):
/// ask the memory card manager's v24 read-through (`self+0x434` is the card
/// object, slot 0xe8 of its vtable) for the save in `slot` into `dest`, and
/// remember `dest` at `+0xe8`.
pub fn mc_manager_read_save_by_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let self_ = abi::r32(c, S1);
        c.g[T0][0] = Addr(self_.wrapping_add(0x434)).lw(c);
        let card = abi::r32(c, T0);
        c.g[V0][0] = Addr(card).lw(c);
        let vtable = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vtable.wrapping_add(0xe8)).lh(c);
        c.g[V1][0] = Addr(vtable.wrapping_add(0xec)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(card.wrapping_add(abi::r32(c, A0)));
        call_ptr(c, target, 0x0023_ca58)?;
        Addr(self_.wrapping_add(0xe8)).sw(c, c.g[S0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_Update(self)` (0x23d618): the memory card manager's
/// per-frame update; `+0xb8` selects the update step (0: the state machine,
/// 1: mode 1, anything else: nothing).
pub fn mc_manager_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, A0);
        c.g[V1][0] = Addr(self_.wrapping_add(0xb8)).lw(c);
        let step0 = c.g[V1][0] == 0;
        c.g[V0][0] = 1;
        if step0 {
            c.cycles += 4;
            callee::MC_MANAGER_UPDATE_STATES.call(c, 0x0023_d644)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        let step1 = c.g[V1][0] == c.g[V0][0];
        c.cycles += 2;
        frame.restore_ra(c, 0);
        if !step1 {
            // The unknown-step block charges 2 cycles and then breaks out of
            // the outer label, which lands on the epilogue's own charge: it
            // pays both.
            c.cycles += 4;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        callee::MC_MANAGER_UPDATE_MODE1.call(c, 0x0023_d654)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Lobby.

/// `bool LobbyMan_IsInRoom(self)` (0x25f430): true when the lobby manager is
/// active (`*(self)`) and its room name (`+0x54`) contains "room".
pub fn lobby_man_is_in_room(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, A0);
        c.g[V0][0] = Addr(self_).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.g[A0][0] = Addr(self_.wrapping_add(0x54)).lw(c);
        c.cycles += 5;
        c.g[A1][0] = 0x726f_0000;
        c.g[A1][0] |= 0x6f6d;
        call_ptr(c, 0x003e_8a70, 0x0025_f458)?;
        let missing = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !missing {
            c.cycles += 2;
            c.g[V0][0] = Addr(c.g[V0][0] as u32).lb(c);
            c.g[V1][0] = (0i64 < c.g[V0][0] as i64) as u64;
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[V1][0];
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_ResetChalState(self)` (0x25f858): tell the lobby service
/// about the challenge ("chal") with the state block at 0x4a2f40 and mark
/// the manager as waiting (`*(self) = 1`).
pub fn lobby_man_reset_chal_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0x4a_0000;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0x6368_0000;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = sx(0x4a_0000u32.wrapping_add(0x2f40));
        c.g[A1][0] |= 0x616c;
        let self_ = abi::r32(c, S0);
        c.g[A0][0] = Addr(self_.wrapping_add(0x54)).lw(c);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, 0x003e_8b10, 0x0025_f888)?;
        c.g[V1][0] = 1;
        Addr(self_).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_CreateApi(self, lobby)` (0x2614e8): create the buddy API
/// object (`0x480e58` tag) for the lobby handle and register `self` as its
/// callback data with `0x268f40`.
pub fn buddy_man_create_api(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = sx(0x48_0000u32.wrapping_add(0xe58));
        call_ptr(c, 0x003f_0710, 0x0026_1508)?;
        let self_ = abi::r32(c, S0);
        c.g[A1][0] = 0x27_0000;
        c.g[A2][0] = c.g[S0][0];
        Addr(self_.wrapping_add(0xc8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx(0x27_0000u32.wrapping_sub(0x70c0));
        call_ptr(c, 0x003f_17c8, 0x0026_1520)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Net_StartConnect(self)` (0x2636a8): start the net connection on the
/// comm system (`gp-0x238`), then arm the retry timer: `+0x4` gets the
/// connect timeout (`gp-0x1e4`, or `gp-0x1e0` while the comm system is idle)
/// and `*(self) = 1`.
pub fn net_start_connect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(gp(c, -0x238)).lw(c);
        call_ptr(c, 0x0025_5f50, 0x0026_36c0)?;
        c.g[V0][0] = Addr(gp(c, -0x238)).lw(c);
        let comm = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(comm.wrapping_add(0x2c)).lw(c);
        let busy = c.g[V1][0] != 0;
        c.lwc1(0, gp(c, -0x1e4));
        if !busy {
            c.cycles += 1;
            c.lwc1(0, gp(c, -0x1e0));
        }
        c.cycles += 7;
        c.g[V0][0] = 1;
        let self_ = abi::r32(c, S0);
        c.swc1(0, self_.wrapping_add(4));
        Addr(self_).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ReplayCache* ReplayCache_ctor(self)` (0x26d0a0): allocate the 32 KiB
/// replay buffer, store it at `+0xc` and reset the cache.
pub fn replay_cache_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x48_0000;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx(0x48_0000u32.wrapping_add(0x15a0));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x8000;
        call_ptr(c, 0x0031_7e30, 0x0026_d0c8)?;
        let self_ = abi::r32(c, S0);
        Addr(self_.wrapping_add(0xc)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        callee::REPLAY_CACHE_RESET.call(c, 0x0026_d0d4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldState_Save(self, out, src)` (0x26d7d0): save the world state
/// into `out` (an unaligned 4-byte copy of `*src` at `out+0x1c`, the world
/// state proper, then the replay frame counter from `self+0x3c8` at
/// `out+0x30`).
pub fn world_state_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let out = abi::r32(c, S0);
        let src = abi::r32(c, A2);
        c.lwl(2, src.wrapping_add(3));
        c.lwr(2, src);
        c.swl(2, out.wrapping_add(0x1f));
        c.swr(2, out.wrapping_add(0x1c));
        c.g[S1][0] = c.g[A0][0];
        callee::WORLD_STATE_WRITE.call(c, 0x0026_d7fc)?;
        let self_ = abi::r32(c, S1);
        c.g[V0][0] = Addr(self_.wrapping_add(0x3c8)).lw(c);
        frame.restore(c, S1, 0x10);
        Addr(out.wrapping_add(0x30)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ReplayStream_AllocBuffer(self, size)` (0x26e908): allocate the
/// replay stream's buffer and point `+0xc`, `+0x4` and `+0x8` at it, with the
/// write cursor (`+0x10`) cleared.
pub fn replay_stream_alloc_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(0x48_0000u32.wrapping_add(0x1730));
        call_ptr(c, 0x0031_7e30, 0x0026_e930)?;
        let self_ = abi::r32(c, S0);
        Addr(self_.wrapping_add(0x10)).sw(c, 0);
        Addr(self_.wrapping_add(0xc)).sw(c, c.g[V0][0]);
        Addr(self_.wrapping_add(4)).sw(c, c.g[V0][0]);
        Addr(self_.wrapping_add(8)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine.

/// `ScriptList_dtor(self, flags)` (0x275d90): stop the script list, clear
/// its head and free it when `flags & 1`.
pub fn script_list_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        callee::SCRIPT_LIST_STOP.call(c, 0x0027_5db0)?;
        let self_ = abi::r32(c, S1);
        c.g[S0][0] &= 1;
        let keep = c.g[S0][0] == 0;
        Addr(self_).sw(c, 0);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0027_5dc4)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_SetLocatorTable(self, key, value)` (0x278a10): store
/// `value` in the locator table slot the script manager's locator list gives
/// for `key` (`self + index*4 + 0x558`, index 50 when the key is absent).
pub fn script_engine_set_locator_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        let world = c.g[V1][0] as u32;
        c.g[V0][0] = Addr(world.wrapping_add(0x84)).lw(c);
        let manager = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(manager.wrapping_add(0x78)).lw(c);
        call_ptr(c, 0x0022_e0e0, 0x0027_8a38)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        let self_ = abi::r32(c, S0);
        let slot = self_.wrapping_add(c.g[V0][0] as u32);
        Addr(slot.wrapping_add(0x558)).sw(c, c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptEngine_TickProcess(self, index)` (0x282450): tick the sequence
/// process in slot `index` of `self` when it is running (state 3 or 4);
/// returns 1 when it was ticked.
pub fn script_engine_tick_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SCRIPT_ENGINE_GET_PROCESS.call(c, 0x0028_2460)?;
        c.g[A0][0] = c.g[V0][0];
        let missing = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if missing {
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        let process = abi::r32(c, A0);
        c.cycles += 5;
        c.g[V0][0] = Addr(process.wrapping_add(4)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(3));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        let idle = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if idle {
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(process).lw(c);
        callee::IN_SEQ_INSTANCE_UPDATE.call(c, 0x0028_2488)?;
        c.g[V0][0] = 1;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_PauseProcess(self, index)` (0x2826d8): pause the
/// sequence process in slot `index` (`self + index*0x24 + 0x2c0`) and mark it
/// stopped (`+0x4 = 5`) when the process has pending work (`process+0x2c`).
pub fn script_engine_pause_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[V0][0] = 0x24;
        let frame = Frame::open(c, 0x20);
        c.mult(0, A1, A1, V0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c0));
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        let slot = abi::r32(c, S0);
        c.g[A0][0] = Addr(slot).lw(c);
        callee::IN_SEQ_INSTANCE_PAUSE.call(c, 0x0028_26fc)?;
        c.g[V0][0] = Addr(slot).lw(c);
        let process = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(process.wrapping_add(0x2c)).lw(c);
        let pending = (c.g[V1][0] as i64) <= 0;
        c.g[V0][0] = 5;
        if !pending {
            c.cycles += 1;
            Addr(slot.wrapping_add(4)).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_SeqListenerOnStart(self, instance)` (0x2829d0): the
/// sequence's start callback: call the listener slot (`self+0x2a8`, entry 10)
/// with the instance's ids and mark the instance started (`+0x4 = 4`).
pub fn script_engine_seq_listener_on_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, A0);
        let instance = c.g[V0][0] as u32;
        c.g[S0][0] = Addr(instance.wrapping_add(0xc)).lw(c);
        let state = abi::r32(c, S0);
        c.g[V1][0] = Addr(self_.wrapping_add(0x2a8)).lw(c);
        c.g[A2][0] = Addr(state.wrapping_add(8)).lw(c);
        let vtable = c.g[V1][0] as u32;
        c.g[A3][0] = Addr(vtable.wrapping_add(0x50)).lh(c);
        c.g[T0][0] = Addr(vtable.wrapping_add(0x54)).lw(c);
        let target = c.g[T0][0] as u32;
        c.g[A0][0] = sx(self_.wrapping_add(abi::r32(c, A3)));
        c.g[A3][0] = Addr(state.wrapping_add(0xc)).lw(c);
        call_ptr(c, target, 0x0028_2a00)?;
        c.g[V0][0] = 4;
        Addr(state.wrapping_add(4)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_SeqListenerOnCreate(self, obj)` (0x282b40): allocate a
/// script thread for `obj`, mark it "stop requested" (`+0x4 = 2`, handle
/// `+0x8 = -1`), store `obj` at `+0` and hand the thread to the instance.
pub fn script_engine_seq_listener_on_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        callee::SCRIPT_ENGINE_ALLOC_PROCESS.call(c, 0x0028_2b58)?;
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = 2;
        let thread = abi::r32(c, V0);
        Addr(thread.wrapping_add(8)).sw(c, c.g[A0][0]);
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        Addr(thread.wrapping_add(4)).sw(c, c.g[V1][0]);
        Addr(thread).sw(c, c.g[S0][0]);
        callee::IN_SEQ_INSTANCE_SET_USER_DATA.call(c, 0x0028_2b78)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptBinding_Bind(self, binding)` (0x282fb8): remember `binding`
/// at `self+8` and activate it with the instance's current key.
pub fn script_binding_bind(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, S0);
        let binding = abi::r32(c, S1);
        Addr(self_.wrapping_add(8)).sw(c, c.g[S1][0]);
        callee::SCRIPT_ENGINE_GET_INSTANCE.call(c, 0x0028_2fd8)?;
        c.g[V1][0] = Addr(binding.wrapping_add(8)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        let state = c.g[V1][0] as u32;
        c.g[A2][0] = Addr(state).lw(c);
        callee::SCRIPT_ENGINE_ACTIVATE_BINDING.call(c, 0x0028_2fec)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptCmdQueue_Count(self)` (0x283d28): the number of queued script
/// commands (0x8c = write cursor, 0x90 = read cursor, wrapped at 5), or 5
/// when the queue is full.
pub fn script_cmd_queue_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_CMD_QUEUE_IS_FULL.call(c, 0x0028_3d3c)?;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 5;
        } else {
            let self_ = abi::r32(c, S0);
            c.cycles += 7;
            c.g[V1][0] = Addr(self_.wrapping_add(0x8c)).lw(c);
            c.g[A0][0] = u64::MAX;
            c.g[V0][0] = Addr(self_.wrapping_add(0x90)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(5));
            if c.g[A0][0] == 0 {
                c.g[V0][0] = c.g[V1][0];
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SndBank_IsLoadedWithName(self, name)` (0x28b830): true when the bank
/// is in state 1 and its name (`self+0x20`) matches `name`.
pub fn snd_bank_is_loaded_with_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 1;
        let self_ = c.g[V1][0] as u32;
        c.g[S0][0] = Addr(self_).lw(c);
        let not_loaded = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = c.g[A1][0];
        if not_loaded {
            c.cycles += 1;
            c.g[V0][0] = 0;
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A1][0] = sx(self_.wrapping_add(0x20));
        call_ptr(c, 0x0041_65a8, 0x0028_b858)?;
        if c.g[V0][0] != 0 {
            c.g[S0][0] = 0;
        }
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_PauseSfx(self)` (0x29ce28): the first time it is
/// called, flag the audio system paused (`+0x5fb0`, and the bank monitor's
/// `+0x26c`) and pause every bank monitor.
pub fn ssx_audio_system_pause_sfx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let self_ = c.g[A1][0] as u32;
        c.g[V0][0] = Addr(self_.wrapping_add(0x5fb0)).lw(c);
        let already = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !already {
            c.cycles += 10;
            c.g[A0][0] = 1;
            c.g[V1][0] = Addr(self_.wrapping_add(0x118)).lw(c);
            Addr(self_.wrapping_add(0x5fb0)).sw(c, c.g[A0][0]);
            let monitor = c.g[V1][0] as u32;
            c.g[V0][0] = Addr(monitor).lw(c);
            Addr((c.g[V0][0] as u32).wrapping_add(0x26c)).sw(c, c.g[A0][0]);
            c.g[V1][0] = Addr(self_.wrapping_add(0x118)).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let bank = (c.g[A0][0] as u32).wrapping_add(0x1d8);
            c.g[A0][0] = sx(bank);
            callee::BANK_MONITOR_PAUSE_ALL.call(c, 0x0029_ce64)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_OnScriptRiderSpeech01(self, type, a, b)` (0x2a1b88):
/// dispatch a scripted rider speech request: type 0 to `..._RiderSay20D3`,
/// type 1 to `..._RiderSay20D4`, anything else is not handled.
pub fn ssx_audio_system_on_script_rider_speech01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let not_zero = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if !not_zero {
            c.cycles += 5;
            c.g[A1][0] = c.g[A2][0];
            c.g[A2][0] = c.g[A3][0];
            callee::RIDER_SAY_20D3.call(c, 0x002a_1ba0)?;
            c.g[V0][0] = 1;
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = 1;
        let is_one = c.g[A1][0] == c.g[V0][0];
        c.g[V0][0] = 0;
        if !is_one {
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[A1][0] = c.g[A2][0];
        c.g[A2][0] = c.g[A3][0];
        callee::RIDER_SAY_20D4.call(c, 0x002a_1bc0)?;
        c.g[V0][0] = 1;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_IsRaceType4or5(self)` (0x2a4248): true when the current
/// race type (`D_00535C12[0]`, a byte in the game's mode block) is 4 or 5.
/// Both calls are the BE library interface accesses the original makes first.
pub fn ssx_audio_system_is_race_type4_or5(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0014_dc80, 0x002a_4258)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x002a_4264)?;
        c.g[V1][0] = 0x53_0000;
        c.g[V0][0] = 4;
        let mode = (c.g[V1][0] as u32).wrapping_add(0x5c12);
        c.g[V1][0] = Addr(mode).lb(c);
        let type4 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 5;
        if !type4 {
            c.cycles += 2;
            let type5 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0;
            if !type5 {
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 1;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MusicPlayer_IsSongDataLoaded(self, force)` (0x2b4aa0): true when the
/// song's data has been loaded (`self+0x5438` is the async file, whose result
/// `0x3df980` returns: 1 means loaded).
pub fn music_player_is_song_data_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let forced = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if forced {
            c.cycles += 2;
            c.g[V0][0] = 1;
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        let self_ = abi::r32(c, A0);
        c.cycles += 3;
        c.g[A0][0] = Addr(self_.wrapping_add(0x5438)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[V0][0] = 0;
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 6;
        call_ptr(c, 0x003d_f980, 0x002b_4ac8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = 1;
        let loaded = c.g[A0][0] == c.g[V1][0];
        c.g[V0][0] = 1;
        if !loaded {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UStr_Cat(dst, src)` (0x2c2688): append the string `src` at the end of
/// the string `dst` (16-bit characters, walked by `lhu` in 2-byte steps).
pub fn ustr_cat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(abi::r32(c, A0)).lhu(c);
        let empty = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !empty {
            c.cycles += 2;
            c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(2));
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(abi::r32(c, A0)).lhu(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(2));
                if event_due(c) {
                    deliver_event_here(c, 0x002c_26a0);
                }
            }
        }
        c.cycles += 5;
        callee::USTR_COPY.call(c, 0x002c_26c4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool MenuSysItem_IsSelectable(self)` (0x2ca320): visible
/// (`menu_item_is_visible`) and not disabled (`menu_item_is_disabled`).
pub fn menu_sys_item_is_selectable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        callee::MENU_SYS_ITEM_IS_VISIBLE.call(c, 0x002c_a33c)?;
        let hidden = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !hidden {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::MENU_SYS_ITEM_IS_DISABLED.call(c, 0x002c_a34c)?;
            c.g[S1][0] = (c.g[V0][0] < 1) as u64;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_SetRowColor(self, row, on)` (0x2cb880): forward to the
/// row widget's colour method (`self+0x12c` is the vtable; entry 8's delta at
/// `+0x40`, its method at `+0x44`), passing the "on" or "off" colour global.
pub fn menu_sys_page_set_row_color(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, A0);
        c.g[V1][0] = Addr(self_.wrapping_add(0x12c)).lw(c);
        let vtable = c.g[V1][0] as u32;
        c.g[V0][0] = Addr(vtable.wrapping_add(0x40)).lh(c);
        c.g[A3][0] = sx(vtable.wrapping_add(0x44));
        let on = c.g[A2][0] != 0;
        c.g[A0][0] = sx(self_.wrapping_add(abi::r32(c, V0)));
        if on {
            c.cycles += 3;
            c.g[V0][0] = 0x4d_0000;
            c.g[A2][0] = sx(0x4d_0000u32.wrapping_add(0x5380));
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x4d_0000;
            c.g[A2][0] = sx(0x4d_0000u32.wrapping_add(0x5390));
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(abi::r32(c, A3)).lw(c);
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x002c_b8bc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_OnDeactivate(self)` (0x2cbfd0): deactivate the selected
/// row (`self->items[self->selected]`, its vtable's entry 7) and re-arm the
/// wrap-around flag (`+0x124 = 0`).
pub fn menu_sys_page_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(self_.wrapping_add(4)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        let slot = c.g[V0][0] as u32;
        c.g[V0][0] = sx(self_.wrapping_add(slot));
        let item_slot = c.g[V0][0] as u32;
        c.g[A1][0] = Addr(item_slot.wrapping_add(0xc)).lw(c);
        let item = c.g[A1][0] as u32;
        c.g[V1][0] = Addr(item.wrapping_add(0x10)).lw(c);
        let vtable = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(vtable.wrapping_add(0x38)).lh(c);
        c.g[V0][0] = Addr(vtable.wrapping_add(0x3c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(item.wrapping_add(abi::r32(c, A0)));
        call_ptr(c, target, 0x002c_c004)?;
        Addr(self_.wrapping_add(0x124)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysBackItem* MenuSysBackItem_ctor(self, text)` (0x2ccdf0): a menu
/// item (id -1) that keeps `text` at `+0x14` and uses the back-item vtable.
pub fn menu_sys_back_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = u64::MAX;
        callee::MENU_SYS_ITEM_CTOR.call(c, 0x002c_ce10)?;
        let self_ = abi::r32(c, S0);
        c.g[V1][0] = 0x48_0000;
        Addr(self_.wrapping_add(0x14)).sw(c, c.g[S1][0]);
        c.g[V1][0] = sx(0x48_0000u32.wrapping_add(0x6d90));
        c.g[V0][0] = c.g[S0][0];
        Addr(self_.wrapping_add(0x10)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysActionItem* MenuSysActionItem_ctor(self)` (0x2ccf08): a menu item
/// (id -1) that uses the action-item vtable and starts with no action, label
/// or child page.
pub fn menu_sys_action_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = u64::MAX;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_CTOR.call(c, 0x002c_cf20)?;
        let self_ = abi::r32(c, S0);
        c.g[V1][0] = 0x48_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(0x48_0000u32.wrapping_add(0x6d40));
        Addr(self_.wrapping_add(0x14)).sw(c, 0);
        Addr(self_.wrapping_add(0x10)).sw(c, c.g[V1][0]);
        Addr(self_.wrapping_add(0x1c)).sw(c, 0);
        Addr(self_.wrapping_add(0x20)).sw(c, 0);
        Addr(self_.wrapping_add(0x18)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int WScriptMan_TryRunObjScript3(id)` (0x2d1a30): when the script manager
/// (`gp+0xce8`) knows an object script for `id`, run it; returns 1 when it
/// ran.
pub fn wscript_man_try_run_obj_script3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(gp(c, 0xce8)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0030_a2e8, 0x002d_1a4c)?;
        let found = c.g[V0][0] != 0;
        c.g[A0][0] = Addr(gp(c, 0xce8)).lw(c);
        if !found {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0030_a548, 0x002d_1a64)?;
        c.g[V0][0] = 1;
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* TransitionEffect_AllocData()` (0x2e4ce8): take the first free slot
/// of the four transition-effect wake pools (0x538850), marking it used; when
/// all four are busy return the fallback pool (`gp+0xa2c`).
pub fn transition_effect_alloc_data(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x54_0000;
    c.g[V0][0] = 0x54_0000;
    c.g[A1][0] = sx(0x54_0000u32.wrapping_sub(0x7d40));
    c.g[V1][0] = sx(0x54_0000u32.wrapping_sub(0x77b0));
    c.g[A0][0] = 0;
    c.g[A2][0] = 1;
    let mut slot = c.g[V1][0] as u32;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(slot).lw(c);
        let busy = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        if busy {
            c.cycles += 4;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x64));
            c.g[V0][0] = ((c.g[A0][0] as i64) < 4) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(slot.wrapping_add(4));
            slot = c.g[V1][0] as u32;
            if !more {
                c.cycles += 2;
                c.g[V0][0] = Addr(gp(c, 0xa2c)).lw(c);
                abi::ret(c);
                return;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002e_4d00);
            }
            continue;
        }
        c.cycles += 3;
        Addr(slot).sw(c, c.g[A2][0]);
        c.g[V0][0] = c.g[A1][0];
        abi::ret(c);
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    /// The main heap's arena header (initialised by [`heap_init`]).
    const MEM_HEAP: u32 = 0x004f_f250;

    fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
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

    /// `gp + 0xdd8` says the size-class pools are ready, and the pool free
    /// lists are populated: on a fresh machine's zeroed globals the pool
    /// allocator forwards to the heap, which forwards back, forever.
    fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes;
            for n in 0..16u32 {
                let node = base + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }

    /// A dlmalloc arena over 256 KiB of test RAM (allocations above 0x200
    /// bytes, and the frees of them, come from here).
    fn heap_init(c: &mut Cpu) {
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x40000) as u64, 0x40000],
        );
    }

    /// A machine for functions that call into game code: fake callees for the
    /// misses, the pool free lists, a real heap arena, and (for some seeds) an
    /// unwind at one of the function's first `calls` faked calls.
    fn call_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        pool_allocator(c);
        heap_init(c);
        c.bus.write32(fakes::LOG_COUNT, 0);
        c.bus.write32(fakes::LOG_NEXT, fakes::LOG_START);
        let unwind = if calls > 0 && r.next_u32() % 3 == 0 {
            r.next_u32() % calls
        } else {
            u32::MAX
        };
        c.bus.write32(fakes::LOG_UNWIND_AT, unwind);
        r
    }

    /// A `{delta, fn}` vtable entry at `vt + off`.
    fn slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, target: u32) {
        c.bus
            .write16(vt + off, (r.next_u32() as i16 as i32 & !3) as u16);
        c.bus.write32(vt + off + 4, target);
    }

    fn put_str8(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
    }

    fn put_str16(c: &mut Cpu, at: u32, s: &[u16]) {
        for (i, h) in s.iter().enumerate() {
            c.bus.write16(at + 2 * i as u32, *h);
        }
    }

    /// A dummy world at `DATA+0x1000` (the `gp-0x848` global), its race
    /// manager at `+0x84 -> +0x8c -> +0x4`, so callees that walk it stop, and
    /// the BE library singleton (a zeroed object at `DATA+0x6000`): without it
    /// `BELibrary_GetInterface` builds one through `EA_MemAllocNamed` and the
    /// file layer spins in `EAFile_AllocRequest`.
    fn dummy_world(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
        c.bus.write32(DATA + 0x1084, DATA + 0x2000);
        c.bus.write32(DATA + 0x208c, DATA + 0x3000);
        c.bus.write32(DATA + 0x3004, DATA + 0x3400);
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
    }

    // ----------------------------------------------------------------- child events

    #[test]
    fn c_fe_state_challenge_popup_on_child_event_matches() {
        check(
            ee::s03::f_001af428__generated,
            c_fe_state_challenge_popup_on_child_event,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1af4_28, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(if seed % 2 == 0 { 0 } else { 0xe });
                let kind = if seed % 3 == 0 {
                    0x835
                } else {
                    0x900 + seed % 7
                };
                c.bus.write32(DATA + 0x418, kind);
                let vt = DATA + 0x800;
                c.bus.write32(this + 8, vt);
                slot(c, &mut r, vt, 0x108, FAKE + 0x10);
                // Not the keyboard popup (a different child of the state).
                c.bus.write32(this + 0x69c, 0xdead_beef);
            },
        );
    }

    #[test]
    fn c_fe_state_challenge_received_popup_on_child_event_matches() {
        check(
            ee::s03::f_001b05e0__generated,
            c_fe_state_challenge_received_popup_on_child_event,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b05_e0, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(if seed % 2 == 0 { 0 } else { 0xe });
                let kind = if seed % 3 == 0 {
                    0x835
                } else {
                    0x8ff + seed % 5
                };
                c.bus.write32(DATA + 0x418, kind);
                let vt = DATA + 0x800;
                c.bus.write32(this + 8, vt);
                slot(c, &mut r, vt, 0x108, FAKE + 0x10);
                c.bus.write32(this + 0x69c, 0xdead_beef);
            },
        );
    }

    #[test]
    fn c_fe_state_chat_room_on_widget_notify_matches() {
        check(
            ee::s03::f_001b3568__generated,
            c_fe_state_chat_room_on_widget_notify,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b35_68, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let sender = match seed % 4 {
                    0 | 1 => DATA + 0x200,
                    2 => DATA + 0x300,
                    _ => DATA + 0x500,
                };
                c.g[A1][0] = sx(sender);
                c.g[A2][0] = sx(if seed % 3 == 0 {
                    4
                } else {
                    r.next_u32() & 0xff
                });
                c.g[A3][0] = r.next_u32() as u64;
                c.bus.write32(this + 0x6fc, DATA + 0x200);
                c.bus.write32(this + 0x700, DATA + 0x300);
            },
        );
    }

    // -------------------------------------------------- net account mem stepping

    fn net_account_mem_machine(c: &mut Cpu, seed: u32, salt: u64, from_a: u32, from_b: u32) {
        let mut r = call_machine(c, seed, salt, 1);
        c.bus.write32(GAME_GP + 0x2e0, DATA + 0x1000);
        let mode = match seed % 4 {
            0 => from_a,
            1 => from_b,
            _ => r.next_u32() & 0x1f,
        };
        c.bus.write32(DATA + 0x1000, mode);
        c.g[A0][0] = sx(DATA + 0x100);
    }

    #[test]
    fn c_fe_state_net_account_mem_update_state08_matches() {
        check(
            ee::s03::f_001b78f0__generated,
            c_fe_state_net_account_mem_update_state08,
            120,
            &|c, seed| {
                net_account_mem_machine(c, seed, 0x1b78_f0, 8, 9);
            },
        );
    }

    #[test]
    fn c_fe_state_net_account_mem_update_state0e_matches() {
        check(
            ee::s03::f_001b7a08__generated,
            c_fe_state_net_account_mem_update_state0e,
            120,
            &|c, seed| {
                net_account_mem_machine(c, seed, 0x1b7a_08, 0x15, 0x16);
            },
        );
    }

    #[test]
    fn c_fe_state_net_account_mem_update_state10_matches() {
        check(
            ee::s03::f_001b7a50__generated,
            c_fe_state_net_account_mem_update_state10,
            120,
            &|c, seed| {
                net_account_mem_machine(c, seed, 0x1b7a_50, 0xc, 0xd);
            },
        );
    }

    #[test]
    fn c_fe_state_net_account_mem_update_state1a_matches() {
        check(
            ee::s03::f_001b7c28__generated,
            c_fe_state_net_account_mem_update_state1a,
            120,
            &|c, seed| {
                net_account_mem_machine(c, seed, 0x1b7c_28, 0x10, 0x11);
            },
        );
    }

    #[test]
    fn c_fe_state_net_account_mem_update_state1b_matches() {
        check(
            ee::s03::f_001b7c70__generated,
            c_fe_state_net_account_mem_update_state1b,
            120,
            &|c, seed| {
                net_account_mem_machine(c, seed, 0x1b7c_70, 0xc, 0xd);
            },
        );
    }

    // ------------------------------------------------------------ state ctors

    #[test]
    fn c_fe_state_login_ctor_matches() {
        check(
            ee::s03::f_001bee30__generated,
            c_fe_state_login_ctor,
            120,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x1bee_30, 3);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() & 0xffff);
                c.bus.write32(this + 0x10, DATA + 0x200);
            },
        );
    }

    #[test]
    fn c_fe_state_online_agreement_ctor_matches() {
        check(
            ee::s03::f_001c0418__generated,
            c_fe_state_online_agreement_ctor,
            120,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x1c04_18, 3);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() & 0xffff);
                c.bus.write32(this + 0x10, DATA + 0x200);
            },
        );
    }

    #[test]
    fn c_fe_state_online_agreement_on_widget_created_matches() {
        check(
            ee::s03::f_001c0ce0__generated,
            c_fe_state_online_agreement_on_widget_created,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c0c_e0, 1);
                let item = DATA + 0x100;
                c.g[A0][0] = sx(item);
                c.g[A1][0] = sx(item);
                // The hash of the (zeroed) agreement text, or something else.
                c.bus
                    .write32(item + 0x38, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(item + 0x14, r.next_u32());
            },
        );
    }

    // ------------------------------------------------------------ v24 popups

    fn popup_v24_machine(c: &mut Cpu, seed: u32, salt: u64, flag_off: u32) {
        let mut r = machine(c, seed, salt, 2);
        let this = DATA + 0x100;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(r.next_u32() & 0xffff);
        c.g[A2][0] = sx(r.next_u32() & 0xffff);
        c.bus
            .write32(this + flag_off, if seed % 3 == 0 { 0 } else { 1 });
        let widget = if seed % 4 == 1 { 0 } else { DATA + 0x200 };
        c.bus.write32(this + 0x20, widget);
        if widget != 0 {
            let vt = DATA + 0x800;
            c.bus.write32(widget + 8, vt);
            slot(c, &mut r, vt, 0xc0, FAKE + 0x10);
        }
    }

    #[test]
    fn c_keyboard_popup_v24_matches() {
        check(
            ee::s04::f_001cb3d0__generated,
            c_keyboard_popup_v24,
            120,
            &|c, seed| {
                popup_v24_machine(c, seed, 0x1cb3_d0, 0x438);
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_v24_matches() {
        check(
            ee::s04::f_001d9e20__generated,
            c_fe_state_net_help_popup_v24,
            120,
            &|c, seed| {
                popup_v24_machine(c, seed, 0x1d9e_20, 0x50);
            },
        );
    }

    

    #[test]
    fn ov_screen_message_center_on_child_event_matches() {
        check(
            ee::s04::f_001e4610__generated,
            ov_screen_message_center_on_child_event,
            24,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x1e46_10, 3);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() & 0xffff);
                c.g[A2][0] = sx(if seed % 2 == 0 {
                    0x14
                } else {
                    r.next_u32() & 0xff
                });
                dummy_world(c);
                c.bus.write32(this + 0x10, DATA + 0x400);
            },
        );
    }

    #[test]
    fn ov_screen_bc_start_pop_event_arg_matches() {
        check(
            ee::s04::f_001f7548__generated,
            ov_screen_bc_start_pop_event_arg,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1f75_48, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x2a0, r.next_u32());
                let head = r.next_u32();
                c.bus.write32(DATA + 0x11c4, head);
                c.bus.write32(
                    DATA + 0x11c8,
                    if seed % 3 == 0 { head } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn ov_screen_pda_template_close_matches() {
        check(
            ee::s05::f_0020d190__generated,
            ov_screen_pda_template_close,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x20d1_90, 2);
                let template = if seed % 3 == 0 { 0 } else { DATA + 0x100 };
                c.bus.write32(GAME_GP - 0x9e4, template);
                if template != 0 {
                    let vt = DATA + 0x800;
                    c.bus.write32(template + 8, vt);
                    slot(c, &mut r, vt, 8, FAKE + 0x10);
                }
            },
        );
    }

    #[test]
    fn hud_meter_dtor_matches() {
        check(ee::s05::f_0020eb08__generated, hud_meter_dtor, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x20eb_08, 1);
            let flags = seed % 4;
            // The free branch runs on a null object: freeing test data would
            // hand a bogus pointer to the heap.
            let this = if flags & 1 != 0 { 0 } else { DATA + 0x100 };
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(flags);
            c.bus.write32(GAME_GP - 0x984, 1 + r.next_u32() % 2);
            c.bus.write32(GAME_GP - 0x988, 0x1234_5678);
        });
    }

    #[test]
    fn race_progress_reset_all_matches() {
        check(
            ee::s05::f_002108f8__generated,
            race_progress_reset_all,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x2108_f8, 0);
            },
        );
    }

    #[test]
    fn start_screen_ctor_matches() {
        check(ee::s06::f_00232ae0__generated, start_screen_ctor, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x232a_e0, 1);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(r.next_u32() & 0xffff);
            c.bus.write32(GAME_GP - 0x51b8, r.next_u32());
        });
    }

    #[test]
    fn mc_manager_read_save_by_slot_matches() {
        check(
            ee::s06::f_0023ca28__generated,
            mc_manager_read_save_by_slot,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23ca_28, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() & 7);
                c.g[A2][0] = sx(r.next_u32() & 0xffff);
                c.g[A3][0] = sx(DATA + 0x300);
                let card = DATA + 0x400;
                c.bus.write32(this + 0x434, card);
                c.bus.write32(card, DATA + 0x800);
                slot(c, &mut r, DATA + 0x800, 0xe8, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn mc_manager_update_matches() {
        check(ee::s06::f_0023d618__generated, mc_manager_update, 32, &|c, seed| {
            let mut r = call_machine(c, seed, 0x23d6_18, 1);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            let step = match seed % 3 {
                0 => 0,
                1 => 1,
                _ => 2 + r.next_u32() % 0x1e,
            };
            c.bus.write32(this + 0xb8, step);
        });
    }

    #[test]
    fn lobby_man_is_in_room_matches() {
        check(
            ee::s06::f_0025f430__generated,
            lobby_man_is_in_room,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x25f4_30, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(this, if seed % 4 == 0 { 0 } else { 1 });
                let room = DATA + 0x400;
                c.bus.write32(this + 0x54, room);
                // The room's name (`room + 0x1c0`) is empty or not.
                c.bus
                    .write8(room + 0x1c0, if seed % 3 == 0 { 0u8 } else { b'r' });
                c.bus.write32(room + 0x2cc, r.next_u32());
            },
        );
    }

    #[test]
    fn lobby_man_reset_chal_state_matches() {
        check(
            ee::s06::f_0025f858__generated,
            lobby_man_reset_chal_state,
            40,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x25f8_58, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(this, r.next_u32());
                let room = DATA + 0x400;
                c.bus.write32(this + 0x54, room);
                put_str8(c, room + 0x1c0, b"chal\0");
            },
        );
    }

    #[test]
    fn buddy_man_create_api_matches() {
        check(ee::s06::f_002614e8__generated, buddy_man_create_api, 60, &|c, seed| {
            let mut r = call_machine(c, seed, 0x2614_e8, 2);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(DATA + 0x200);
            c.g[A2][0] = sx(r.next_u32() & 0xffff);
            c.bus.write32(DATA + 0x200, r.next_u32());
            c.bus.write32(this + 0xc8, 0);
        });
    }

    #[test]
    fn net_start_connect_matches() {
        check(ee::s07::f_002636a8__generated, net_start_connect, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x2636_a8, 1);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            c.bus.write32(GAME_GP - 0x238, DATA + 0x1000);
            c.bus.write32(DATA + 0x1000 + 0x2c, seed as u32 % 2);
            c.bus.write32(GAME_GP - 0x1e4, r.next_u32());
            c.bus.write32(GAME_GP - 0x1e0, r.next_u32());
        });
    }

    #[test]
    fn replay_cache_ctor_matches() {
        check(ee::s07::f_0026d0a0__generated, replay_cache_ctor, 60, &|c, seed| {
            let mut r = call_machine(c, seed, 0x26d0_a0, 1);
            c.g[A0][0] = sx(DATA + 0x100);
            let _ = r.next_u32();
        });
    }

    #[test]
    fn world_state_save_matches() {
        check(ee::s07::f_0026d7d0__generated, world_state_save, 40, &|c, seed| {
            let mut r = call_machine(c, seed, 0x26d7_d0, 2);
            dummy_world(c);
            let this = DATA + 0x100;
            let out = DATA + 0x200;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(out);
            c.g[A2][0] = sx(DATA + 0x300 + (seed as u32 & 3));
            c.bus.write32(DATA + 0x300, r.next_u32());
            c.bus.write32(DATA + 0x304, r.next_u32());
            c.bus.write32(this + 0x3c8, r.next_u32());
            c.bus.write32(out + 0x1c, 0xdead_beef);
        });
    }

    #[test]
    fn replay_stream_alloc_buffer_matches() {
        check(
            ee::s07::f_0026e908__generated,
            replay_stream_alloc_buffer,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x26e9_08, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(0x40 + (seed as u32 % 16) * 4);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn script_list_dtor_matches() {
        check(ee::s07::f_00275d90__generated, script_list_dtor, 60, &|c, seed| {
            let mut r = call_machine(c, seed, 0x275d_90, 2);
            let flags = seed % 2;
            // The free branch runs on a null list (a bogus pointer would be
            // handed to the heap).
            let this = if flags != 0 { 0 } else { DATA + 0x100 };
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(flags);
            c.bus.write32(this + 0x10, r.next_u32());
        });
    }

    #[test]
    fn script_engine_set_locator_table_matches() {
        check(
            ee::s07::f_00278a10__generated,
            script_engine_set_locator_table,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x278a_10, 1);
                let this = DATA + 0x100;
                let key = seed as u32 & 0xff;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(key);
                c.g[A2][0] = sx(r.next_u32());
                c.bus.write32(GAME_GP - 0x848, DATA + 0x1000);
                c.bus.write32(DATA + 0x1084, DATA + 0x2000);
                c.bus.write32(DATA + 0x2078, DATA + 0x3000);
                if seed % 2 == 0 {
                    // The locator table the lookup scans (0x44216c + i*0x10).
                    c.bus.write32(0x0044_216c + 0x20, key);
                }
            },
        );
    }

    #[test]
    fn script_engine_tick_process_matches() {
        check(
            ee::s07::f_00282450__generated,
            script_engine_tick_process,
            32,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x2824_50, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = 0;
                let slot = this + 0x2c0;
                c.bus
                    .write32(slot + 0x20, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(
                    slot + 4,
                    match seed % 4 {
                        0 => 3,
                        1 => 4,
                        2 => 2,
                        _ => r.next_u32() & 0xff,
                    },
                );
                c.bus.write32(slot, DATA + 0x400);
            },
        );
    }

    #[test]
    fn script_engine_pause_process_matches() {
        check(
            ee::s07::f_002826d8__generated,
            script_engine_pause_process,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2826_d8, 1);
                let this = DATA + 0x100;
                let index = seed % 3;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(index);
                let slot = this + 0x2c0 + 0x24 * index;
                c.bus.write32(slot, DATA + 0x400);
                c.bus.write32(slot + 4, r.next_u32());
                c.bus
                    .write32(DATA + 0x400 + 0x2c, if seed % 2 == 0 { 1 } else { 0 });
            },
        );
    }

    #[test]
    fn script_engine_seq_listener_on_start_matches() {
        check(
            ee::s07::f_002829d0__generated,
            script_engine_seq_listener_on_start,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2829_d0, 1);
                let this = DATA + 0x100;
                let obj = DATA + 0x200;
                let state = DATA + 0x400;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(obj);
                c.bus.write32(obj + 0xc, state);
                c.bus.write32(state + 8, r.next_u32());
                c.bus.write32(state + 0xc, r.next_u32());
                c.bus.write32(this + 0x2a8, DATA + 0x800);
                slot(c, &mut r, DATA + 0x800, 0x50, FAKE + 0x20);
            },
        );
    }

    #[test]
    fn script_engine_seq_listener_on_create_matches() {
        check(
            ee::s07::f_00282b40__generated,
            script_engine_seq_listener_on_create,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x282b_40, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x200);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn script_binding_bind_matches() {
        check(ee::s07::f_00282fb8__generated, script_binding_bind, 40, &|c, seed| {
            let mut r = call_machine(c, seed, 0x282f_b8, 1);
            let this = DATA + 0x100;
            let binding = DATA + 0x200;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(binding);
            c.bus.write32(binding + 8, DATA + 0x300);
            c.bus.write32(DATA + 0x300, r.next_u32());
        });
    }

    #[test]
    fn script_cmd_queue_count_matches() {
        check(
            ee::s07::f_00283d28__generated,
            script_cmd_queue_count,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x283d_28, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus
                    .write32(this + 0x94, if seed % 4 == 0 { 1 } else { 0 });
                c.bus.write32(this + 0x8c, r.next_u32() % 6);
                c.bus.write32(this + 0x90, r.next_u32() % 6);
            },
        );
    }

    #[test]
    fn snd_bank_is_loaded_with_name_matches() {
        check(
            ee::s07::f_0028b830__generated,
            snd_bank_is_loaded_with_name,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28b8_30, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x300);
                c.bus.write32(
                    this,
                    match seed % 4 {
                        0 => 0,
                        1 => 2,
                        _ => 1,
                    },
                );
                let name = if seed % 2 == 0 {
                    &b"bank_a\0"[..]
                } else {
                    &b"bank_b\0"[..]
                };
                put_str8(c, DATA + 0x300, name);
                let local = if seed % 4 == 1 {
                    &b"bank_a\0"[..]
                } else {
                    &b"other\0"[..]
                };
                put_str8(c, this + 0x20, local);
                c.bus.write32(this + 0x24, r.next_u32());
            },
        );
    }

    #[test]
    fn ssx_audio_system_pause_sfx_matches() {
        check(
            ee::s08::f_0029ce28__generated,
            ssx_audio_system_pause_sfx,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x29ce_28, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let monitor = DATA + 0x2000;
                c.bus.write32(this + 0x118, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, monitor);
                c.bus
                    .write32(this + 0x5fb0, if seed % 3 == 0 { 1 } else { 0 });
                c.bus.write32(monitor + 0x26c, r.next_u32());
                c.bus.write32(monitor + 0x1d8, r.next_u32());
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_script_rider_speech01_matches() {
        check(
            ee::s08::f_002a1b88__generated,
            ssx_audio_system_on_script_rider_speech01,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x2a1b_88, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(match seed % 4 {
                    0 => 0,
                    1 => 1,
                    _ => 2 + seed % 0x10,
                });
                c.g[A2][0] = sx(r.next_u32() & 0xffff);
                c.g[A3][0] = sx(r.next_u32() & 0xffff);
                c.bus.write32(this + 0x118, DATA + 0x1000);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_race_type4_or5_matches() {
        check(
            ee::s08::f_002a4248__generated,
            ssx_audio_system_is_race_type4_or5,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a42_48, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(seed % 2);
                // The BE library singleton (a zeroed dummy) and its interface
                // slot, whose method is a fake.
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x600);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                let iface = DATA + 0x400;
                c.bus.write32(0x0053_08a8, iface);
                c.bus.write32(iface + 4, DATA + 0x800);
                slot(c, &mut r, DATA + 0x800, 8, FAKE + 0x10);
                c.bus.write8(
                    0x0053_5c12,
                    match seed % 3 {
                        0 => 4,
                        1 => 5,
                        _ => 7,
                    },
                );
            },
        );
    }

    #[test]
    fn music_player_is_song_data_loaded_matches() {
        check(
            ee::s08::f_002b4aa0__generated,
            music_player_is_song_data_loaded,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x2b4a_a0, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(seed % 3);
                c.bus
                    .write32(this + 0x5438, if seed % 4 == 1 { 0 } else { DATA + 0x200 });
                c.bus.write32(DATA + 0x200, r.next_u32());
                c.bus.write32(DATA + 0x200 + 0x1c, seed as u32 % 3);
            },
        );
    }

    #[test]
    fn ustr_cat_matches() {
        check(ee::s08::f_002c2688__generated, ustr_cat, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x2c26_88, 1);
            let dst = DATA + 0x200;
            let src = DATA + 0x300;
            c.g[A0][0] = sx(dst);
            c.g[A1][0] = sx(src);
            let empty = seed % 5 == 0;
            let empty_str = [0u16];
            let filled_str = [0x61, 0x62 + (seed as u16 % 3), 0];
            put_str16(
                c,
                dst,
                if empty {
                    &empty_str[..]
                } else {
                    &filled_str[..]
                },
            );
            put_str16(c, src, &[0x63 + (seed as u16 % 3), 0]);
            let _ = r.next_u32();
        });
    }

    #[test]
    fn menu_sys_item_is_selectable_matches() {
        check(
            ee::s09::f_002ca320__generated,
            menu_sys_item_is_selectable,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ca3_20, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(
                    this + 4,
                    match seed % 4 {
                        0 => 0,
                        1 => 1,
                        2 => 8,
                        _ => 9,
                    },
                );
                c.bus.write32(this + 0x14, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_page_set_row_color_matches() {
        check(
            ee::s09::f_002cb880__generated,
            menu_sys_page_set_row_color,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cb8_80, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() % 8);
                c.g[A2][0] = sx(if seed % 2 == 0 { 0 } else { 1 + seed % 3 });
                let vt = DATA + 0x400;
                c.bus.write32(this + 0x12c, vt);
                slot(c, &mut r, vt, 0x40, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn menu_sys_page_on_deactivate_matches() {
        check(
            ee::s09::f_002cbfd0__generated,
            menu_sys_page_on_deactivate,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cbf_d0, 1);
                let this = DATA + 0x100;
                let index = seed % 4;
                let item = DATA + 0x400;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 4, index);
                c.bus.write32(this + 0xc + 4 * index, item);
                c.bus.write32(this + 0x124, r.next_u32());
                c.bus.write32(item + 0x10, DATA + 0x800);
                slot(c, &mut r, DATA + 0x800, 0x38, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn menu_sys_back_item_ctor_matches() {
        check(
            ee::s09::f_002ccdf0__generated,
            menu_sys_back_item_ctor,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ccd_f0, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(if seed % 2 == 0 { DATA + 0x200 } else { 0 });
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn menu_sys_action_item_ctor_matches() {
        check(
            ee::s09::f_002ccf08__generated,
            menu_sys_action_item_ctor,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ccf_08, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn wscript_man_try_run_obj_script3_matches() {
        check(
            ee::s09::f_002d1a30__generated,
            wscript_man_try_run_obj_script3,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x2d1a_30, 2);
                c.g[A0][0] = sx(seed as u32 & 0xf);
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x28c, DATA + 0x2000);
                c.bus.write32(DATA + 0x1000 + 0x290, r.next_u32());
            },
        );
    }

    #[test]
    fn transition_effect_alloc_data_matches() {
        check(
            ee::s09::f_002e4ce8__generated,
            transition_effect_alloc_data,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e4c_e8, 0);
                let busy = seed % 5;
                for i in 0..4u32 {
                    let value = if busy == 1 {
                        1
                    } else if busy >= 2 && i < busy {
                        1
                    } else {
                        0
                    };
                    c.bus.write32(0x0053_8850 + 4 * i, value);
                }
                c.bus.write32(GAME_GP + 0xa2c, r.next_u32());
            },
        );
    }
}
