//! Phase 2 wave 14 module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for small lifecycle, menu, replay, script and audio helpers,
//! each with a differential test against the generated body.
//!
//! Groups:
//!
//! - **UI / menu widgets** (`0x0039c2c8..0x003a4d70`): list counting, the two
//!   loader shims that chain `UIObject_Load`, the focus visitor's vtable
//!   forward and the script-cache constructor.
//! - **Front end states** (`0x0014ef70..0x001d97e0`): the character DB flag
//!   query, the lobby event hook, memory-card state transitions and the
//!   keyboard popup action, plus the two `UISound` constructors.
//! - **Movie / memory card / lobby managers** (`0x00232d18..0x0025b650`):
//!   teardown and refresh helpers.
//! - **Network singletons** (`0x00264cb8..0x00266728`): `cSetupMan`,
//!   `cSharedLoginMan`, `cVoipMan`, the tagged allocators and cache frees.
//! - **Replay** (`0x0026ef80..0x002742e0`): menu state, highlight slot clear,
//!   save-header emission and the block payload accessor.
//! - **Script / sound** (`0x002757f0..0x002a4290`): free-entry scans, bank
//!   unload, the music-event wrapper and the two BE mode queries.
//! - **Bank monitor** (`0x002ae100..0x002ae1b8`): shutdown and the track-slot
//!   scan.
//!
//! Replacements that call guest code follow [`crate::call`] (same frame,
//! same registers at every call, unwind check after it); loops keep the
//! original's back-edge event checks through [`crate::events`].

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0038_ae28, light_man_dtor, "LightMan_dtor"),
    (0x0039_79c0, ui_list_count, "UIList_Count"),
    (0x0039_c2c8, cui_progress_load, "cUIProgress_Load"),
    (0x0039_dfb0, cui_slider_load, "cUISlider_Load"),
    (0x003a_4d70, ui_focus_visitor_visit, "UIFocusVisitor_Visit"),
    (0x003a_c7c8, cw_script_cache_ctor, "cWScriptCache_ctor"),
    (
        0x0014_ef70,
        be_get_rider_char_db_flag14,
        "BE_GetRiderCharDBFlag14",
    ),
    (
        0x0017_d260,
        lobby_menu_lobby_event_hook,
        "LobbyMenu_LobbyEventHook",
    ),
    (
        0x0018_7148,
        cfe_state_mem_card_close_and_set_state6,
        "cFEStateMemCard_CloseAndSetState6",
    ),
    (0x001a_2e20, feui_sound_ctor, "FEUISound_ctor"),
    (0x001a_2f38, ovui_sound_ctor, "OVUISound_ctor"),
    (
        0x001b_0f28,
        char_to_list_index,
        "cFEStateOnlineCharacterSelect_CharToListIndex",
    ),
    (
        0x001c_b290,
        ckeyboard_popup_on_widget_action,
        "cKeyboardPopup_OnWidgetAction",
    ),
    (
        0x001d_2430,
        cfe_state_video_player_shutdown,
        "cFEStateVideoPlayer_Shutdown",
    ),
    (
        0x001d_5428,
        mem_card_state_base_dtor,
        "MemCardStateBase_dtor",
    ),
    (
        0x001d_97e0,
        mem_card_state_base_step_slot_dir,
        "MemCardStateBase_StepSlotDir",
    ),
    (0x0022_4f30, cluno_table_free_data, "cLunoTable_FreeData"),
    (
        0x0023_6f08,
        game_state_finish_cinematic_enter,
        "GameStateFinishCinematic_Enter",
    ),
    (0x0023_c860, mc_manager_destroy, "MCManager_Destroy"),
    (
        0x0023_cc20,
        mc_manager_card_set_state18,
        "MCManager_CardSetState18",
    ),
    (
        0x0023_fae0,
        mc_manager_refresh_slot,
        "MCManager_RefreshSlot",
    ),
    (
        0x0025_a6f8,
        lobby_man_destroy_peer_table,
        "LobbyMan_DestroyPeerTable",
    ),
    (0x0026_4cb8, csetup_man_send_hello, "cSetupMan_SendHello"),
    (0x0026_6728, tib_ext_mem_alloc, "TibExtMemAlloc"),
    (0x0026_6ca0, cvoip_man_create, "cVoipMan_Create"),
    (0x0026_6df8, cvoip_man_stop, "cVoipMan_Stop"),
    (0x0026_6f58, cvoip_man_set_volume, "cVoipMan_SetVolume"),
    (0x0026_d130, replay_cache_free, "ReplayCache_Free"),
    (
        0x0026_ef80,
        replay_restore_instant_replay_cameras,
        "Replay_RestoreInstantReplayCameras",
    ),
    (0x0027_06b8, replay_open_menu, "Replay_OpenMenu"),
    (
        0x0027_0de0,
        replay_cancel_highlight,
        "Replay_CancelHighlight",
    ),
    (
        0x0027_12c0,
        replay_save_write_header,
        "Replay_SaveWriteHeader",
    ),
    (
        0x0027_42e0,
        inseq_channel_get_block_payload,
        "InSeq_ChannelGetBlockPayload",
    ),
    (
        0x0027_57f0,
        script_anim_bank_man_find_free_entry,
        "ScriptAnimBankMan_FindFreeEntry",
    ),
    (
        0x0028_3760,
        script_sound_bank_man_find_free_entry,
        "ScriptSoundBankMan_FindFreeEntry",
    ),
    (
        0x0028_e888,
        ssx_audio_system_end_script_music,
        "SSXAudioSystem_EndScriptMusic",
    ),
    (
        0x0028_f730,
        ssx_audio_system_unload_loading_screen_bank,
        "SSXAudioSystem_UnloadLoadingScreenBank",
    ),
    (
        0x0029_b7e0,
        ssx_audio_system_on_bonus_trick,
        "SSXAudioSystem_OnBonusTrick",
    ),
    (
        0x002a_4040,
        ssx_audio_system_is_event_mode4,
        "SSXAudioSystem_IsEventMode4",
    ),
    (
        0x002a_4290,
        ssx_audio_system_is_race_type4,
        "SSXAudioSystem_IsRaceType4",
    ),
    (0x002a_e100, path_plat_shutdown, "PATH_PlatShutdown"),
    (
        0x002a_e1b8,
        path_plat_find_free_track_slot,
        "PATH_PlatFindFreeTrackSlot",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const BUILTIN_DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::f_00317e50,
    };
    pub const EA_MEM_ALLOC: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::f_00317d70,
    };
    pub const EA_MEM_FREE: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::f_00317e98,
    };
    pub const OPERATOR_NEW_NAMED: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::f_00317e30,
    };

    pub const UI_OBJECT_LOAD: Callee = Callee {
        addr: 0x0039_fe00,
        f: ee::s13::f_0039fe00,
    };
    pub const UI_SOUND_BASE_CTOR: Callee = Callee {
        addr: 0x0039_e288,
        f: ee::s13::f_0039e288,
    };
    pub const UI_STATE_MGR_CLOSE_TOP_STATES: Callee = Callee {
        addr: 0x0039_f190,
        f: ee::s13::f_0039f190,
    };
    pub const UI_STATE_DTOR: Callee = Callee {
        addr: 0x0039_e390,
        f: ee::s13::f_0039e390,
    };
    pub const SND_REMOVE_TICK_CALLBACK: Callee = Callee {
        addr: 0x003b_5948,
        f: ee::s13::f_003b5948,
    };
    pub const CW_SCRIPT_CACHE_ALLOC: Callee = Callee {
        addr: 0x003a_c800,
        f: ee::s13::f_003ac800,
    };

    pub const BE_GET_RIDER_CHARACTER: Callee = Callee {
        addr: 0x0014_a080,
        f: ee::s01::f_0014a080,
    };
    pub const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };
    pub const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };
    pub const GAME_CAMERA_SET_CONTROLLER: Callee = Callee {
        addr: 0x0015_dfd8,
        f: ee::s01::f_0015dfd8,
    };
    pub const LOBBY_MENU_ON_LOBBY_EVENT: Callee = Callee {
        addr: 0x0017_d6f0,
        f: ee::s02::f_0017d6f0,
    };
    pub const LOBBY_MAN_FORWARD_EVENT_TO_UI: Callee = Callee {
        addr: 0x0025_9230,
        f: ee::s06::f_00259230,
    };
    pub const CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS: Callee = Callee {
        addr: 0x0016_d1b8,
        f: ee::s02::f_0016d1b8,
    };
    pub const VID_ENGINE_PLAYER_DTOR: Callee = Callee {
        addr: 0x0025_3418,
        f: ee::s06::f_00253418,
    };
    pub const LUNO_TABLE_DTOR: Callee = Callee {
        addr: 0x0022_4d00,
        f: ee::s05::f_00224d00,
    };
    pub const SSX_AUDIO_GET_INSTANCE: Callee = Callee {
        addr: 0x0028_b180,
        f: ee::s07::f_0028b180,
    };
    pub const SSX_AUDIO_PLAY_CHARACTER_TUNE: Callee = Callee {
        addr: 0x0028_cdf8,
        f: ee::s07::f_0028cdf8,
    };
    pub const NET_GAME_UTIL_DESTROY: Callee = Callee {
        addr: 0x003e_6e70,
        f: ee::s14::f_003e6e70,
    };
    pub const CSETUP_MAN_SEND_PACKET: Callee = Callee {
        addr: 0x0026_4be8,
        f: ee::s07::f_00264be8,
    };
    pub const CVOIP_MAN_CTOR: Callee = Callee {
        addr: 0x0026_6d00,
        f: ee::s07::f_00266d00,
    };
    pub const VOIP_RPC_STOP: Callee = Callee {
        addr: 0x003f_6af0,
        f: ee::s14::f_003f6af0,
    };
    pub const VOIP_RPC_SET_PARAMS: Callee = Callee {
        addr: 0x003f_6c10,
        f: ee::s14::f_003f6c10,
    };

    pub const REPLAY_SET_STATE: Callee = Callee {
        addr: 0x0026_f4a0,
        f: ee::s07::f_0026f4a0,
    };
    pub const REPLAY_HIGHLIGHT_SLOT_CLEAR: Callee = Callee {
        addr: 0x0026_e800,
        f: ee::s07::f_0026e800,
    };
    pub const REPLAY_SAVE_EMIT: Callee = Callee {
        addr: 0x0027_1058,
        f: ee::s07::f_00271058,
    };
    pub const MC_MANAGER_REQUEST_CARD_INFO: Callee = Callee {
        addr: 0x0023_fb20,
        f: ee::s06::f_0023fb20,
    };
    pub const SSX_AUDIO_HANDLE_MUSIC_EVENT: Callee = Callee {
        addr: 0x0028_e8c0,
        f: ee::s07::f_0028e8c0,
    };
    pub const SSX_AUDIO_STOP_LOADING_SCREEN_LOOP: Callee = Callee {
        addr: 0x0028_fa98,
        f: ee::s07::f_0028fa98,
    };
    pub const SND_BANK_MAN_UNLOAD_BANK: Callee = Callee {
        addr: 0x0028_bce8,
        f: ee::s07::f_0028bce8,
    };
    pub const SSX_AUDIO_ARCADE_SAY: Callee = Callee {
        addr: 0x002a_3de0,
        f: ee::s08::f_002a3de0,
    };
    pub const EATIMER_REMOVE_CALLBACK: Callee = Callee {
        addr: 0x003e_58c8,
        f: ee::s14::f_003e58c8,
    };
}

// ---------------------------------------------------------------------------
// Light manager / UI lists.

/// `LightMan::~LightMan()` (0x38ae28): install the light manager vtable, clear
/// the global pointer and (when bit 0 of `flags` is set) delete the object.
pub fn light_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[V0][0] = 0x0049_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x36d8));
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let vtable = c.g[V0][0];
        Addr(this).sw(c, vtable);
        c.g[A1][0] &= 1;
        let delete = c.g[A1][0] != 0;
        Addr((c.g[GP][0] as u32).wrapping_add(0x2a9c)).sw(c, 0);
        if delete {
            c.cycles += 2;
            callee::BUILTIN_DELETE.call(c, 0x0038_ae50)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int UIList_Count(UIList* list)` (0x3979c0): walk `node->prev` from the
/// head until a node points at itself.
pub fn ui_list_count(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
    c.g[A0][0] = 0;
    if c.g[V0][0] != c.g[V1][0] {
        loop {
            c.cycles += 7;
            c.g[V1][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
            let more = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_79d0);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `void cUIProgress_Load(cUIProgress* self)` (0x39c2c8): load the base menu
/// object, then set the bar's min (10) and max (100).
pub fn cui_progress_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_LOAD.call(c, 0x0039_c2dc)?;
        c.g[V1][0] = 0xa;
        c.g[V0][0] = 0x64;
        let s0 = c.g[S0][0];
        let v0 = c.g[V0][0];
        let v1 = c.g[V1][0];
        Addr(s0 as u32).off(0x84).sw(c, v0);
        Addr(s0 as u32).off(0x80).sw(c, v1);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUISlider_Load(cUISlider* self)` (0x39dfb0): same as the progress
/// bar, with the slider's range fields at `+0x78`/`+0x80`.
pub fn cui_slider_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_LOAD.call(c, 0x0039_dfc4)?;
        c.g[V1][0] = 0xa;
        c.g[V0][0] = 0x64;
        let s0 = c.g[S0][0];
        let v0 = c.g[V0][0];
        let v1 = c.g[V1][0];
        Addr(s0 as u32).off(0x80).sw(c, v0);
        Addr(s0 as u32).off(0x78).sw(c, v1);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int UIFocusVisitor_Visit(UIList* list, UIObject* obj)` (0x3a4d70): call
/// `obj->vtable[0x38/4]` with the visitor state `list+4`, then return 0.
pub fn ui_focus_visitor_visit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
        c.g[A1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x38).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x3c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x003a_4d94)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    })
}

/// `void* cWScriptCache_ctor(cWScriptCache* self, int count)` (0x3ac7c8):
/// stash the world, zero the list and allocate `count` script entries.
pub fn cw_script_cache_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let world = c.g[A1][0];
        Addr(this).sw(c, world);
        Addr(this).off(4).sw(c, 0);
        c.g[A1][0] = 1;
        callee::CW_SCRIPT_CACHE_ALLOC.call(c, 0x003a_c7e8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front end.

/// `bool BE_GetRiderCharDBFlag14(int riderIndex)` (0x14ef70): whether byte 0x44
/// of the rider's character-DB record is non-zero.
pub fn be_get_rider_char_db_flag14(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        callee::BE_GET_RIDER_CHARACTER.call(c, 0x0014_ef80)?;
        c.g[A0][0] = 0x88;
        c.g[V1][0] = 0x0053_0000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x970));
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x44).lw(c);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_LobbyEventHook(void* ev)` (0x17d260): forward the event to
/// the live lobby menu, then to the lobby manager's UI.
pub fn lobby_menu_lobby_event_hook(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x1da0)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_menu = c.g[A0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if !no_menu {
            c.cycles += 2;
            callee::LOBBY_MENU_ON_LOBBY_EVENT.call(c, 0x0017_d280)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::LOBBY_MAN_FORWARD_EVENT_TO_UI.call(c, 0x0017_d288)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMemCard_CloseAndSetState6(void)` (0x187148): call vtable slot
/// 0x148 of the current front-end state, then move it to state 6.
pub fn cfe_state_mem_card_close_and_set_state6(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1c38)).lw(c);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(8).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x148).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x14c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0018_7168)?;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1c38)).lw(c);
        c.g[V0][0] = 6;
        frame.restore_ra(c, 0);
        let a0 = c.g[A0][0];
        let v0 = c.g[V0][0];
        Addr(a0 as u32).off(0x1c0).sw(c, v0);
        frame.close(c);
        Ok(())
    })
}

/// `void* FEUISound_ctor(FEUISound* self)` (0x1a2e20): base UI-sound
/// constructor plus the derived vtable.
pub fn feui_sound_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_SOUND_BASE_CTOR.call(c, 0x001a_2e34)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6cb8));
        let v1 = c.g[V1][0];
        let s0 = c.g[S0][0];
        Addr(s0 as u32).off(4).sw(c, 0);
        Addr(s0 as u32).sw(c, v1);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* OVUISound_ctor(OVUISound* self)` (0x1a2f38): the overlay variant of
/// the UI-sound constructor.
pub fn ovui_sound_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_SOUND_BASE_CTOR.call(c, 0x001a_2f4c)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6ce8));
        let v1 = c.g[V1][0];
        let s0 = c.g[S0][0];
        Addr(s0 as u32).off(4).sw(c, 0);
        Addr(s0 as u32).sw(c, v1);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateOnlineCharacterSelect_CharToListIndex(int character)` (0x1b0f28):
/// index of `character` in the ten-entry character list, or -1.
pub fn char_to_list_index(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x0044_0000;
    c.g[V1][0] = 0;
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf68));
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lb(c);
        if c.g[A0][0] != c.g[V0][0] {
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 10i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            if !more {
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                abi::ret(c);
                return;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001b_0f38);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[V1][0];
            abi::ret(c);
            return;
        }
    }
}

/// `void cKeyboardPopup_OnWidgetAction(cKeyboardPopup* self, int, unsigned act)`
/// (0x1cb290): a widget action of 5 or 6 closes the popup's top menu states.
pub fn ckeyboard_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = (c.g[A2][0] < 7u64) as u64;
        let below_seven = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if below_seven {
            c.cycles += 3;
            c.g[V0][0] = (c.g[A2][0] < 5u64) as u64;
            let below_five = c.g[V0][0] != 0;
            if below_five {
                frame.restore_ra(c, 0);
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 4;
            c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x10).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            callee::UI_STATE_MGR_CLOSE_TOP_STATES.call(c, 0x001c_b2bc)?;
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateVideoPlayer_Shutdown(cFEStateVideoPlayer* self)` (0x1d2430):
/// stop and release the video player at `+0x248`.
pub fn cfe_state_video_player_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x248).lw(c);
        if c.g[A0][0] == 0 {
            let s0 = c.g[S0][0];
            Addr(s0 as u32).off(0x248).sw(c, 0);
        } else {
            c.cycles += 3;
            c.g[A1][0] = 3;
            callee::VID_ENGINE_PLAYER_DTOR.call(c, 0x001d_2454)?;
            let s0 = c.g[S0][0];
            Addr(s0 as u32).off(0x248).sw(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
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

/// `int MemCardStateBase_StepSlotDir(MemCardStateBase* self, void* list, int idx,
/// int back)` (0x1d97e0): step `idx` by ±1 and report whether it is still in
/// the list (1) or out of range (-1).
pub fn mem_card_state_base_step_slot_dir(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 1;
    c.g[V1][0] = u64::MAX;
    if c.g[A3][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let negative = (c.g[A2][0] as i64) < 0;
    c.g[V0][0] = u64::MAX;
    if negative {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V1][0] = Addr(r32(c, A1).wrapping_add(0x424)).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
    let too_big = c.g[V1][0] != 0;
    c.g[V0][0] = 1;
    if too_big {
        c.cycles += 1;
        c.g[V0][0] = u64::MAX;
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Luno / movie / memory card managers.

/// `void cLunoTable_FreeData(cLunoTableRef* ref)` (0x224f30): destroy and clear
/// the table the reference holds.
pub fn cluno_table_free_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        if c.g[A0][0] == 0 {
            let s0 = c.g[S0][0];
            Addr(s0 as u32).sw(c, 0);
        } else {
            c.cycles += 3;
            c.g[A1][0] = 3;
            callee::LUNO_TABLE_DTOR.call(c, 0x0022_4f54)?;
            let s0 = c.g[S0][0];
            Addr(s0 as u32).sw(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateFinishCinematic_Enter()` (0x236f08): hand the audio system to
/// the character-tune player and flag the race manager's cinematic end.
pub fn game_state_finish_cinematic_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SSX_AUDIO_GET_INSTANCE.call(c, 0x0023_6f18)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SSX_AUDIO_PLAY_CHARACTER_TUNE.call(c, 0x0023_6f20)?;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[A1][0] = 1;
        frame.restore_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x84).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        let v1 = c.g[V1][0];
        let a1 = c.g[A1][0];
        Addr(v1 as u32).off(0x8c).sw(c, a1);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_Destroy(void)` (0x23c860): run the manager's vtable
/// destructor and clear the global.
pub fn mc_manager_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x47c)).lw(c);
        let absent = c.g[A2][0] == 0;
        frame.save_ra(c, 0);
        if !absent {
            c.cycles += 6;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x748).lw(c);
            c.g[A1][0] = 3;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x10).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x14).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_c888)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        Addr((c.g[GP][0] as u32).wrapping_sub(0x47c)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_CardSetState18(MCManager* self)` (0x23cc20): if a card
/// object is present, call its vtable slot 0x158.
pub fn mc_manager_card_set_state18(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x118).lw(c);
        let no_card = c.g[V0][0] == 0;
        frame.restore_ra(c, 0);
        if !no_card {
            c.cycles += 7;
            c.g[A1][0] = Addr(abi::arg(c, 0)).off(0x434).lw(c);
            c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x158).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x15c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_cc4c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_RefreshSlot(MCManager* self)` (0x23fae0): reset the slot
/// cache when the current slot changed and request its card info.
pub fn mc_manager_refresh_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(abi::arg(c, 0)).off(0x428).lw(c);
        c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x444).lw(c);
        if c.g[V0][0] != c.g[V1][0] {
            c.cycles += 3;
            let a0 = abi::arg(c, 0);
            let v1 = c.g[V1][0];
            Addr(a0).off(0x444).sw(c, v1);
            Addr(a0).off(0x43c).sw(c, 0);
            Addr(a0).off(0x440).sw(c, 0);
        }
        c.cycles += 5;
        c.g[A1][0] = Addr(abi::arg(c, 0)).off(0x428).lw(c);
        callee::MC_MANAGER_REQUEST_CARD_INFO.call(c, 0x0023_fb0c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_DestroyPeerTable(LobbyMan* self)` (0x25a6f8): destroy the
/// network peer table at `+0xf4` and clear the pointer.
pub fn lobby_man_destroy_peer_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xf4).lw(c);
        if c.g[A0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 4;
            callee::NET_GAME_UTIL_DESTROY.call(c, 0x0025_a71c)?;
            let s0 = c.g[S0][0];
            Addr(s0 as u32).off(0xf4).sw(c, 0);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Network singletons.

/// `void cSetupMan_SendHello(cSetupMan* self)` (0x264cb8): post the 0x14-byte
/// hello message (type 16, lobby state at `+0xb8`).
pub fn csetup_man_send_hello(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
        // The 0x14-byte message lives at the frame base; `ra` is saved at
        // +0x20 (the argument registers must not clobber it).
        let msg = frame.at(c, 0);
        frame.save_ra(c, 0x20);
        c.g[A3][0] = 0x10;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x14;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xb8).lw(c);
        let a3 = c.g[A3][0];
        let v1 = c.g[V1][0];
        Addr(msg).sw(c, a3);
        Addr(msg).off(4).sw(c, v1);
        Addr(msg).off(8).sw(c, 0);
        callee::CSETUP_MAN_SEND_PACKET.call(c, 0x0026_4ce4)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void* TibExtMemAlloc(int size, int kind)` (0x266728): tagged allocation of
/// `size + 0x400` bytes (0x23000000 tag for kind 0x20).
pub fn tib_ext_mem_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x400));
        c.g[V1][0] = c.g[A2][0] ^ 0x20;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x2000_0000;
        c.g[A2][0] = 0x2300_0000;
        c.g[A1][0] = 0x0048_0000;
        frame.save_ra(c, 0);
        if c.g[V1][0] != 0 {
            c.g[A2][0] = c.g[V0][0];
        }
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1080));
        c.g[A3][0] = 0;
        callee::OPERATOR_NEW_NAMED.call(c, 0x0026_6754)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cVoipMan_Create(void)` (0x266ca0): allocate the eight-byte VoIP
/// manager and construct it.
pub fn cvoip_man_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x0048_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10a0));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A0][0] = 8;
        callee::EA_MEM_ALLOC.call(c, 0x0026_6cc0)?;
        c.g[A0][0] = c.g[V0][0];
        callee::CVOIP_MAN_CTOR.call(c, 0x0026_6cc8)?;
        frame.restore_ra(c, 0);
        let v0 = c.g[V0][0];
        Addr((c.g[GP][0] as u32).wrapping_add(0x304)).sw(c, v0);
        frame.close(c);
        Ok(())
    })
}

/// `void cVoipMan_Stop(cVoipMan* self)` (0x266df8): stop the VoIP RPC session
/// and clear the stored handle.
pub fn cvoip_man_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 4;
            callee::VOIP_RPC_STOP.call(c, 0x0026_6e1c)?;
            let s0 = c.g[S0][0];
            Addr(s0 as u32).sw(c, 0);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cVoipMan_SetVolume(cVoipMan* self, int level)` (0x266f58): push the
/// level to the RPC layer when the session is live and remember it.
pub fn cvoip_man_set_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[V0][0] = Addr(abi::arg(c, 0)).lw(c);
        let silent = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[A2][0];
        if !silent {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            callee::VOIP_RPC_SET_PARAMS.call(c, 0x0026_6f7c)?;
        }
        c.cycles += 5;
        let s0 = c.g[S0][0];
        Addr((c.g[GP][0] as u32).wrapping_add(0x308)).sw(c, s0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay.

/// `void ReplayCache_Free(ReplayCache* self)` (0x26d130): release the buffer at
/// `+0xc` and clear it.
pub fn replay_cache_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
        if c.g[A0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 4;
            callee::EA_MEM_FREE.call(c, 0x0026_d154)?;
            let s0 = c.g[S0][0];
            Addr(s0 as u32).off(0xc).sw(c, 0);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_RestoreInstantReplayCameras(void)` (0x26ef80): re-arm the
/// in-game camera triggers and restore the replay camera controller.
pub fn replay_restore_instant_replay_cameras(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x004c_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5830));
        callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS.call(c, 0x0026_ef94)?;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[A1][0] = 2;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x84).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
        callee::GAME_CAMERA_SET_CONTROLLER.call(c, 0x0026_efac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_OpenMenu(ReplayCache* self)` (0x2706b8): open the replay menu
/// unless a dialog is active or it is already open.
pub fn replay_open_menu(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x61c).lw(c);
        let busy = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if busy {
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V1][0] = Addr(abi::arg(c, 0)).lw(c);
        c.g[V0][0] = 9;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A1][0] = 9;
        callee::REPLAY_SET_STATE.call(c, 0x0027_06e4)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_CancelHighlight(sRcSlots* self, int slot)` (0x270de0): clear the
/// highlight flag of slot `i` when the replay is idle.
pub fn replay_cancel_highlight(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        let active = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !active {
            c.cycles += 6;
            c.g[A0][0] = 0xb4;
            c.mult(0, A0, A1, A0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x494));
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            callee::REPLAY_HIGHLIGHT_SLOT_CLEAR.call(c, 0x0027_0e0c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_SaveWriteHeader(ReplayCache* self)` (0x2712c0): emit the 0x28
/// byte save header and bump the record counter.
pub fn replay_save_write_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0x28;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x640).lw(c);
        callee::REPLAY_SAVE_EMIT.call(c, 0x0027_12dc)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x638).lw(c);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let v0 = c.g[V0][0];
        let s0 = c.g[S0][0];
        Addr(s0 as u32).off(0x638).sw(c, v0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void* InSeq_ChannelGetBlockPayload(InSeqChannel* self, int block)`
/// (0x2742e0): payload of block `block` (`base + table[block] + 8`) when the
/// block is large enough, else 0.
pub fn inseq_channel_get_block_payload(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x14).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A2][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = Addr(c.g[A2][0] as u32).lhu(c);
    c.g[V0][0] = (c.g[V0][0] < 9u64) as u64;
    let small = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if small {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(8));
        abi::ret(c);
    }
}

// ---------------------------------------------------------------------------
// Script tables.

/// `void* ScriptAnimBankMan_FindFreeEntry(ScriptAnimBank* self)` (0x2757f0):
/// first entry (stride 0x14) whose data pointer is null, or 0.
pub fn script_anim_bank_man_find_free_entry(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    let empty = (c.g[A1][0] as i64) <= 0;
    c.g[V1][0] = 0;
    if empty {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(8).lw(c);
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        let occupied = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        if !occupied {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0027_5800);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void* ScriptSoundBankMan_FindFreeEntry(ScriptSoundBank* self)` (0x283760):
/// first entry (stride 0xc) whose data pointer is null, or 0.
pub fn script_sound_bank_man_find_free_entry(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    let empty = (c.g[A1][0] as i64) <= 0;
    c.g[V1][0] = 0;
    if empty {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(8).lw(c);
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        let occupied = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        if !occupied {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0028_3770);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Sound.

/// `void SSXAudioSystem_EndScriptMusic(SSXAudioSystem* self)` (0x28e888):
/// post the script-music stop event and clear the handle.
pub fn ssx_audio_system_end_script_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x6254).lw(c);
        let silent = c.g[V0][0] == 0;
        c.g[A1][0] = 0x13;
        if !silent {
            c.cycles += 3;
            c.g[A2][0] = 1;
            callee::SSX_AUDIO_HANDLE_MUSIC_EVENT.call(c, 0x0028_e8ac)?;
            let s0 = c.g[S0][0];
            Addr(s0 as u32).off(0x6254).sw(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_UnloadLoadingScreenBank(SSXAudioSystem* self)`
/// (0x28f730): fade out the loading-screen loop and unload its bank.
pub fn ssx_audio_system_unload_loading_screen_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.mtc1(0, 12);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SSX_AUDIO_STOP_LOADING_SCREEN_LOOP.call(c, 0x0028_f748)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        callee::SND_BANK_MAN_UNLOAD_BANK.call(c, 0x0028_f758)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_OnBonusTrick(SSXAudioSystem* audio, Rider* r)`
/// (0x29b7e0): say the bonus-trick line when the rider's trick display is
/// active.
pub fn ssx_audio_system_on_bonus_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(abi::arg(c, 1)).off(0x874).lw(c);
        let inactive = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !inactive {
            c.cycles += 2;
            c.g[V0][0] = Addr(abi::arg(c, 1)).off(0x87c).lw(c);
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        let quiet = c.g[V1][0] == 0;
        frame.restore_ra(c, 0);
        if !quiet {
            c.cycles += 3;
            c.g[A2][0] = 8;
            callee::SSX_AUDIO_ARCADE_SAY.call(c, 0x0029_b80c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_IsEventMode4(void)` (0x2a4040): whether the BE event
/// mode byte is 4.
pub fn ssx_audio_system_is_event_mode4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x002a_4050)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x002a_405c)?;
        c.g[V1][0] = 0x0053_0000;
        frame.restore_ra(c, 0);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c10)).lb(c);
        c.g[V0][0] ^= 0x4;
        c.g[V0][0] = (c.g[V0][0] < 1u64) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_IsRaceType4(void)` (0x2a4290): whether the BE race type
/// byte is 4.
pub fn ssx_audio_system_is_race_type4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x002a_42a0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x002a_42ac)?;
        c.g[V1][0] = 0x0053_0000;
        frame.restore_ra(c, 0);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c12)).lb(c);
        c.g[V0][0] ^= 0x4;
        c.g[V0][0] = (c.g[V0][0] < 1u64) as u64;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bank monitor.

/// `void PATH_PlatShutdown(void)` (0x2ae100): remove the platform tick
/// callback, stop the timer and clear both globals.
pub fn path_plat_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x6a8)).lw(c);
        let absent = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !absent {
            c.cycles += 7;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x6ac)).lw(c);
            callee::SND_REMOVE_TICK_CALLBACK.call(c, 0x002a_e118)?;
            c.g[A0][0] = 0x002b_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fe0));
            callee::EATIMER_REMOVE_CALLBACK.call(c, 0x002a_e124)?;
            Addr((c.g[GP][0] as u32).wrapping_add(0x6a8)).sw(c, 0);
            Addr((c.g[GP][0] as u32).wrapping_add(0x6ac)).sw(c, 0);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_PlatFindFreeTrackSlot(void)` (0x2ae1b8): index of the first free
/// of 24 track slots, or -1.
pub fn path_plat_find_free_track_slot(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x004d_0000;
    c.g[V1][0] = 0;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e98));
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = c.g[V1][0];
            abi::ret(c);
            return;
        }
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 24i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        if !more {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002a_e1c8);
        }
    }
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP, RA as SENTINEL_RA};

    /// The engine's size-class pools (`MEM_PoolAlloc`), copied from `oc68`:
    /// allocations up to 0x200 bytes come from the per-class free lists hung
    /// off `gp + 0x2a60..`, which only work when the "pools ready" flag at
    /// `gp + 0xdd8` is set (otherwise the pool allocator recurses forever).
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

    /// The main heap's dlmalloc arena over 256 KiB of test RAM.
    const MEM_HEAP: u32 = 0x004f_f250;

    fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, SENTINEL_RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    fn heap_init(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x40000) as u64, 0x40000],
        );
    }

    /// A real block of `size` bytes from the main heap.
    fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        run(
            c,
            0x0032_0058,
            ee::s10::MEM_HeapMemalign,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    fn alloc_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        pool_allocator(c);
        heap_init(c);
        r
    }

    /// Write `{delta, fake}` at `at + off` (a gcc vtable slot).
    fn put_slot(c: &mut Cpu, r: &mut Rng, at: u32, off: u32) {
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(at + off, delta as u16);
        c.bus.write32(at + off + 4, FAKE);
    }

    fn zero(c: &mut Cpu, at: u32, bytes: u32) {
        for i in (0..bytes).step_by(4) {
            c.bus.write32(at + i, 0);
        }
    }

    // ------------------------------------------------------ light / UI lists

    #[test]
    fn light_man_dtor_matches() {
        check(ee::s12::f_0038ae28__generated, light_man_dtor, 60, &|c, seed| {
            let mut r = alloc_machine(c, seed, 0x38ae_28);
            let obj = heap_block(c, 0x140);
            zero(c, obj, 0x140);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx((seed & 3) as u32);
            c.bus.write32(GAME_GP + 0x2a9c, r.next_u32());
        });
    }

    #[test]
    fn ui_list_count_matches() {
        check(ee::s13::f_003979c0__generated, ui_list_count, 80, &|c, seed| {
            let _ = machine(c, seed, 0x3979_c0, 0);
            let list = DATA + 0x100;
            let head = DATA + 0x200;
            c.g[A0][0] = sx(list);
            c.bus.write32(list + 4, head);
            let n = seed % 5;
            for i in 0..5u32 {
                let node = head + 0x20 * i;
                let next = if i + 1 < n { node + 0x20 } else { node };
                c.bus.write32(node + 4, next);
            }
        });
    }

    #[test]
    fn cui_progress_load_matches() {
        check(ee::s13::f_0039c2c8__generated, cui_progress_load, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x39c2_c8, 0);
            let obj = DATA + 0x400;
            let desc = DATA + 0x600;
            fill(c, &mut r, obj, 0x40);
            fill(c, &mut r, desc, 0x20);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx(desc);
            c.bus.write32(obj + 8, DATA + 0x700);
            put_slot(c, &mut r, DATA + 0x700, 0x48);
        });
    }

    #[test]
    fn cui_slider_load_matches() {
        check(ee::s13::f_0039dfb0__generated, cui_slider_load, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x39df_b0, 0);
            let obj = DATA + 0x400;
            let desc = DATA + 0x600;
            fill(c, &mut r, obj, 0x40);
            fill(c, &mut r, desc, 0x20);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx(desc);
            c.bus.write32(obj + 8, DATA + 0x700);
            put_slot(c, &mut r, DATA + 0x700, 0x48);
        });
    }

    #[test]
    fn ui_focus_visitor_visit_matches() {
        check(
            ee::s13::f_003a4d70__generated,
            ui_focus_visitor_visit,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a4d_70, 0);
                let list = DATA + 0x400;
                let obj = DATA + 0x600;
                fill(c, &mut r, list, 0x20);
                fill(c, &mut r, obj, 0x20);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(obj);
                c.bus.write32(obj + 8, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 0x38);
            },
        );
    }

    #[test]
    fn cw_script_cache_ctor_matches() {
        check(ee::s13::f_003ac7c8__generated, cw_script_cache_ctor, 60, &|c, seed| {
            let mut r = alloc_machine(c, seed, 0x3ac7_c8);
            let this = DATA + 0x400;
            fill(c, &mut r, this, 8);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(seed % 3);
        });
    }

    // ------------------------------------------------------------ front end

    #[test]
    fn be_get_rider_char_db_flag14_matches() {
        check(
            ee::s01::f_0014ef70__generated,
            be_get_rider_char_db_flag14,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14ef_70, 0);
                let idx = seed % 4;
                let ch = idx as u32;
                c.g[A1][0] = sx(idx);
                c.bus.write32(0x0053_05b0 + 4 * idx, ch);
                c.bus
                    .write8(0x0053_05b20 + ch + 0x11, (r.next_u32() & 3) as u8);
                let flag = if seed % 3 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(0x0053_0970 + ch * 0x88 + 0x44, flag);
            },
        );
    }

    #[test]
    fn lobby_menu_lobby_event_hook_matches() {
        check(
            ee::s02::f_0017d260__generated,
            lobby_menu_lobby_event_hook,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x17d2_60, 0);
                let ev = DATA + 0x600;
                fill(c, &mut r, ev, 0x20);
                c.bus.write32(ev, 0);
                c.g[A1][0] = sx(ev);
                let menu = if seed % 2 == 0 { 0 } else { DATA + 0x400 };
                zero(c, DATA + 0x400, 0x40);
                c.bus.write32(GAME_GP + 0x1da0, menu);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), 0);
            },
        );
    }

    #[test]
    fn cfe_state_mem_card_close_and_set_state6_matches() {
        check(
            ee::s02::f_00187148__generated,
            cfe_state_mem_card_close_and_set_state6,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1871_48, 0);
                let state = DATA + 0x400;
                fill(c, &mut r, state, 0x40);
                c.bus.write32(GAME_GP.wrapping_sub(0x1c38), state);
                c.bus.write32(state + 8, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 0x148);
            },
        );
    }

    #[test]
    fn feui_sound_ctor_matches() {
        check(ee::s03::f_001a2e20__generated, feui_sound_ctor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1a2e_20, 0);
            let obj = DATA + 0x400;
            fill(c, &mut r, obj, 0x10);
            c.g[A0][0] = sx(obj);
        });
    }

    #[test]
    fn ovui_sound_ctor_matches() {
        check(ee::s03::f_001a2f38__generated, ovui_sound_ctor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1a2f_38, 0);
            let obj = DATA + 0x400;
            fill(c, &mut r, obj, 0x10);
            c.g[A0][0] = sx(obj);
        });
    }

    #[test]
    fn char_to_list_index_matches() {
        check(ee::s03::f_001b0f28__generated, char_to_list_index, 80, &|c, seed| {
            let _ = machine(c, seed, 0x1b0f_28, 0);
            c.g[A0][0] = sx(seed % 12);
            for i in 0..10u32 {
                let v = if seed % 3 == 0 {
                    (i + 5) as u8
                } else {
                    (i % 12) as u8
                };
                c.bus.write8(0x0044_0f68 + i, v);
            }
        });
    }

    #[test]
    fn ckeyboard_popup_on_widget_action_matches() {
        check(
            ee::s04::f_001cb290__generated,
            ckeyboard_popup_on_widget_action,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1cb2_90, 0);
                let popup = DATA + 0x400;
                fill(c, &mut r, popup, 0x40);
                zero(c, DATA + 0x600, 0x40);
                c.g[A0][0] = sx(popup);
                c.g[A2][0] = sx((seed % 9) as u32);
                c.bus.write32(popup + 0x10, DATA + 0x600);
                // The state-manager list at +0x18 is empty (`head` of the sentinel).
                c.bus.write32(DATA + 0x600 + 0x18 + 4, 0);
            },
        );
    }

    #[test]
    fn cfe_state_video_player_shutdown_matches() {
        check(
            ee::s04::f_001d2430__generated,
            cfe_state_video_player_shutdown,
            60,
            &|c, seed| {
                let _r = alloc_machine(c, seed, 0x1d24_30);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x300);
                c.g[A0][0] = sx(self_);
                if seed % 2 != 0 {
                    let player = heap_block(c, 0x200);
                    zero(c, player, 0x200);
                    c.bus.write32(self_ + 0x248, player);
                } else {
                    c.bus.write32(self_ + 0x248, 0);
                }
            },
        );
    }

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

    #[test]
    fn mem_card_state_base_step_slot_dir_matches() {
        check(
            ee::s04::f_001d97e0__generated,
            mem_card_state_base_step_slot_dir,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d97_e0, 0);
                let list = DATA + 0x400;
                zero(c, list, 0x40);
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx((seed % 6) as u32);
                c.g[A3][0] = sx((seed & 1) as u32);
                c.bus.write32(list + 0x424, (seed % 6) + 1);
            },
        );
    }

    // ------------------------------------------------ luno / movie / cards

    #[test]
    fn cluno_table_free_data_matches() {
        check(
            ee::s05::f_00224f30__generated,
            cluno_table_free_data,
            60,
            &|c, seed| {
                let _r = alloc_machine(c, seed, 0x224f_30);
                let r = DATA + 0x400;
                zero(c, r, 0x40);
                c.g[A0][0] = sx(r);
                if seed % 2 != 0 {
                    let table = heap_block(c, 0x300);
                    zero(c, table, 0x300);
                    c.bus.write32(r, table);
                } else {
                    c.bus.write32(r, 0);
                }
            },
        );
    }

    #[test]
    fn game_state_finish_cinematic_enter_matches() {
        check(
            ee::s06::f_00236f08__generated,
            game_state_finish_cinematic_enter,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x236f_08, 0);
                zero(c, DATA + 0x400, 0x40);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x410),
                    if seed % 2 == 0 { 0 } else { DATA + 0x400 },
                );
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
                zero(c, DATA + 0x1000, 0x400);
                c.bus.write32(DATA + 0x1000 + 0x84, DATA + 0x1100);
                c.bus.write32(DATA + 0x1100 + 0xc, DATA + 0x1200);
            },
        );
    }

    #[test]
    fn mc_manager_destroy_matches() {
        check(ee::s06::f_0023c860__generated, mc_manager_destroy, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x23c8_60, 0);
            let obj = DATA + 0x400;
            zero(c, obj, 0x800);
            c.bus.write32(
                GAME_GP.wrapping_sub(0x47c),
                if seed % 2 == 0 { 0 } else { obj },
            );
            c.bus.write32(obj + 0x748, DATA + 0x700);
            put_slot(c, &mut r, DATA + 0x700, 0x10);
        });
    }

    #[test]
    fn mc_manager_card_set_state18_matches() {
        check(
            ee::s06::f_0023cc20__generated,
            mc_manager_card_set_state18,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23cc_20, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x500);
                c.g[A0][0] = sx(self_);
                c.bus
                    .write32(self_ + 0x118, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(self_ + 0x434, DATA + 0x600);
                c.bus.write32(DATA + 0x600, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 0x158);
            },
        );
    }

    #[test]
    fn mc_manager_refresh_slot_matches() {
        check(
            ee::s06::f_0023fae0__generated,
            mc_manager_refresh_slot,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23fa_e0, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x500);
                c.g[A0][0] = sx(self_);
                c.bus.write32(self_ + 0x428, seed % 4);
                let cached = if seed % 2 == 0 {
                    seed % 4
                } else {
                    (seed + 1) % 4
                };
                c.bus.write32(self_ + 0x444, cached);
                c.bus.write32(self_ + 0x434, DATA + 0x600);
                c.bus.write32(DATA + 0x600, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 0x28);
            },
        );
    }

    // ----------------------------------------------------------- networking

    #[test]
    fn lobby_man_destroy_peer_table_matches() {
        check(
            ee::s06::f_0025a6f8__generated,
            lobby_man_destroy_peer_table,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x25a6_f8, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x200);
                zero(c, DATA + 0x600, 0x100);
                c.g[A0][0] = sx(self_);
                let table = if seed % 2 == 0 { 0 } else { DATA + 0x600 };
                c.bus.write32(self_ + 0xf4, table);
            },
        );
    }

    #[test]
    fn csetup_man_send_hello_matches() {
        check(
            ee::s07::f_00264cb8__generated,
            csetup_man_send_hello,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x264c_b8, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x200);
                zero(c, DATA + 0x600, 0x100);
                c.g[A0][0] = sx(self_);
                c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x600);
                c.bus.write32(DATA + 0x600 + 0xb8, r.next_u32());
            },
        );
    }

    #[test]
    fn tib_ext_mem_alloc_matches() {
        check(ee::s07::f_00266728__generated, tib_ext_mem_alloc, 60, &|c, seed| {
            let mut r = alloc_machine(c, seed, 0x2667_28);
            c.g[A1][0] = sx(seed % 0x100);
            c.g[A2][0] = sx(if seed % 2 == 0 {
                0x20
            } else {
                r.next_u32() & 0xff
            });
        });
    }

    #[test]
    fn cvoip_man_create_matches() {
        check(ee::s07::f_00266ca0__generated, cvoip_man_create, 40, &|c, seed| {
            let _ = alloc_machine(c, seed, 0x266c_a0);
        });
    }

    #[test]
    fn cvoip_man_stop_matches() {
        check(ee::s07::f_00266df8__generated, cvoip_man_stop, 60, &|c, seed| {
            let _ = machine(c, seed, 0x266d_f8, 0);
            let self_ = DATA + 0x400;
            zero(c, self_, 0x100);
            c.g[A0][0] = sx(self_);
            c.bus.write32(self_, if seed % 2 == 0 { 0 } else { 1 });
        });
    }

    #[test]
    fn cvoip_man_set_volume_matches() {
        check(ee::s07::f_00266f58__generated, cvoip_man_set_volume, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x266f_58, 0);
            let self_ = DATA + 0x400;
            zero(c, self_, 0x100);
            c.g[A0][0] = sx(self_);
            c.bus.write32(self_, if seed % 2 == 0 { 0 } else { 1 });
            c.g[A1][0] = sx(seed % 100);
            c.g[A2][0] = sx(r.next_u32() % 100);
        });
    }

    // --------------------------------------------------------------- replay

    #[test]
    fn replay_cache_free_matches() {
        check(ee::s07::f_0026d130__generated, replay_cache_free, 60, &|c, seed| {
            let _r = alloc_machine(c, seed, 0x26d1_30);
            let self_ = DATA + 0x400;
            zero(c, self_, 0x100);
            c.g[A0][0] = sx(self_);
            if seed % 2 != 0 {
                let buf = heap_block(c, 0x400);
                zero(c, buf, 0x400);
                c.bus.write32(self_ + 0xc, buf);
            } else {
                c.bus.write32(self_ + 0xc, 0);
            }
        });
    }

    #[test]
    fn replay_restore_instant_replay_cameras_matches() {
        check(
            ee::s07::f_0026ef80__generated,
            replay_restore_instant_replay_cameras,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x26ef_80, 0);
                // No in-game camera triggers: CameraTriggerMan_UpdateTriggers exits.
                c.bus.write32(0x004c_5830 + 4, 0);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
                zero(c, DATA + 0x1000, 0x400);
                c.bus.write32(DATA + 0x1000 + 0x84, DATA + 0x1100);
                c.bus.write32(DATA + 0x1100 + 0x84, DATA + 0x1200);
                c.bus.write32(DATA + 0x1200 + 4, DATA + 0x1300);
                zero(c, DATA + 0x1300, 0x100);
            },
        );
    }

    #[test]
    fn replay_open_menu_matches() {
        check(ee::s07::f_002706b8__generated, replay_open_menu, 60, &|c, seed| {
            let _ = machine(c, seed, 0x2706_b8, 0);
            let self_ = DATA + 0x400;
            zero(c, self_, 0x700);
            c.g[A0][0] = sx(self_);
            c.bus
                .write32(self_ + 0x61c, if seed % 3 == 0 { 0 } else { 1 });
            let state = if seed % 2 == 0 { 9 } else { seed % 9 };
            c.bus.write32(self_, state);
        });
    }

    #[test]
    fn replay_cancel_highlight_matches() {
        check(
            ee::s07::f_00270de0__generated,
            replay_cancel_highlight,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x270d_e0, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x800);
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx((seed % 4) as u32);
                c.bus.write32(self_, if seed % 2 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn replay_save_write_header_matches() {
        check(
            ee::s07::f_002712c0__generated,
            replay_save_write_header,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2712_c0, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x700);
                fill(c, &mut r, DATA + 0x600, 0x10);
                c.g[A0][0] = sx(self_);
                c.bus.write32(self_ + 0x638, r.next_u32());
                c.bus
                    .write32(self_ + 0x640, if seed % 2 == 0 { 0 } else { DATA + 0x600 });
            },
        );
    }

    #[test]
    fn inseq_channel_get_block_payload_matches() {
        check(
            ee::s07::f_002742e0__generated,
            inseq_channel_get_block_payload,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2742_e0, 0);
                let chan = DATA + 0x400;
                zero(c, chan, 0x40);
                c.g[A0][0] = sx(chan);
                c.g[A1][0] = sx((seed % 4) as u32);
                c.bus.write32(chan + 0x14, DATA + 0x600);
                c.bus.write32(chan + 4, DATA + 0x700);
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0x600 + 4 * i, i * 0x20);
                }
                let hit = (seed % 4) as u32;
                let size = if seed % 2 == 0 {
                    (seed % 9) as u16
                } else {
                    9 + (seed % 8) as u16
                };
                c.bus.write16(DATA + 0x700 + hit * 0x20, size);
            },
        );
    }

    // --------------------------------------------------------------- script

    #[test]
    fn script_anim_bank_man_find_free_entry_matches() {
        check(
            ee::s07::f_002757f0__generated,
            script_anim_bank_man_find_free_entry,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2757_f0, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x40);
                zero(c, DATA + 0x600, 0x100);
                c.g[A0][0] = sx(self_);
                let count = seed % 4;
                c.bus.write32(self_ + 4, count);
                c.bus.write32(self_ + 8, DATA + 0x600);
                for i in 0..4u32 {
                    let occupied = (i + seed) % 2 == 0;
                    c.bus.write32(DATA + 0x600 + i * 0x14 + 8, occupied as u32);
                }
            },
        );
    }

    #[test]
    fn script_sound_bank_man_find_free_entry_matches() {
        check(
            ee::s07::f_00283760__generated,
            script_sound_bank_man_find_free_entry,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2837_60, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x40);
                zero(c, DATA + 0x600, 0x100);
                c.g[A0][0] = sx(self_);
                let count = seed % 4;
                c.bus.write32(self_ + 4, count);
                c.bus.write32(self_ + 8, DATA + 0x600);
                for i in 0..4u32 {
                    let occupied = (i + seed) % 2 == 0;
                    c.bus.write32(DATA + 0x600 + i * 0xc + 8, occupied as u32);
                }
            },
        );
    }

    // ---------------------------------------------------------------- sound

    #[test]
    fn ssx_audio_system_end_script_music_matches() {
        check(
            ee::s07::f_0028e888__generated,
            ssx_audio_system_end_script_music,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x28e8_88, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x6300);
                c.g[A0][0] = sx(self_);
                c.bus
                    .write32(self_ + 0x6254, if seed % 2 == 0 { 0 } else { 1 });
                // HandleMusicEvent exits early when +0x627c is clear.
                c.bus.write32(self_ + 0x627c, 0);
            },
        );
    }

    #[test]
    fn ssx_audio_system_unload_loading_screen_bank_matches() {
        check(
            ee::s07::f_0028f730__generated,
            ssx_audio_system_unload_loading_screen_bank,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x28f7_30, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x6300);
                c.g[A0][0] = sx(self_);
                // Negative handle: StopLoadingScreenLoop returns at once.
                c.bus.write32(self_ + 0x62a8, 0xffff_ffff);
                c.bus.write32(self_ + 0x118, DATA + 0x600);
                c.bus.write32(DATA + 0x600, DATA + 0x700);
                zero(c, DATA + 0x700, 0x100);
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_bonus_trick_matches() {
        check(
            ee::s08::f_0029b7e0__generated,
            ssx_audio_system_on_bonus_trick,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x29b7_e0, 0);
                let audio = DATA + 0x400;
                let rider = DATA + 0x600;
                zero(c, audio, 0x100);
                zero(c, rider, 0x900);
                c.g[A0][0] = sx(audio);
                c.g[A1][0] = sx(rider);
                c.bus
                    .write32(rider + 0x874, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(rider + 0x87c, if seed % 2 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_event_mode4_matches() {
        check(
            ee::s08::f_002a4040__generated,
            ssx_audio_system_is_event_mode4,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a40_40, 0);
                let idx = seed % 4;
                zero(c, DATA + 0x400, 0x400);
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x400);
                c.bus.write32(GAME_GP.wrapping_sub(0x1efc), idx);
                c.bus.write32(0x0053_08a8 + 4 * idx, DATA + 0x600);
                c.bus.write32(DATA + 0x600 + 4, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 8);
                c.bus.write8(0x0053_5c10, (seed % 7) as u8);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_race_type4_matches() {
        check(
            ee::s08::f_002a4290__generated,
            ssx_audio_system_is_race_type4,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a42_90, 0);
                let idx = seed % 4;
                zero(c, DATA + 0x400, 0x400);
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x400);
                c.bus.write32(GAME_GP.wrapping_sub(0x1efc), idx);
                c.bus.write32(0x0053_08a8 + 4 * idx, DATA + 0x600);
                c.bus.write32(DATA + 0x600 + 4, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 8);
                c.bus.write8(0x0053_5c12, (seed % 7) as u8);
            },
        );
    }

    // ----------------------------------------------------------- bank monitor

    #[test]
    fn path_plat_shutdown_matches() {
        check(ee::s08::f_002ae100__generated, path_plat_shutdown, 60, &|c, seed| {
            let _ = machine(c, seed, 0x2ae1_00, 0);
            let present = if seed % 2 == 0 { 0 } else { 1 };
            c.bus.write32(GAME_GP.wrapping_add(0x6a8), present);
            c.bus.write32(GAME_GP.wrapping_add(0x6ac), present);
        });
    }

    #[test]
    fn path_plat_find_free_track_slot_matches() {
        check(
            ee::s08::f_002ae1b8__generated,
            path_plat_find_free_track_slot,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2ae1_b8, 0);
                let used = seed % 26;
                for i in 0..24u32 {
                    c.bus.write32(0x004d_3e98 + 4 * i, (i < used) as u32);
                }
            },
        );
    }
}
