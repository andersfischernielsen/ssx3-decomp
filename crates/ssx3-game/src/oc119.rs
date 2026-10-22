//! Phase 2 wave 42: cold-path menu, UI, script, audio and back-end helpers,
//! each rewritten as typed Rust with a differential test against the
//! generated body.
//!
//! The functions are small (a handful of calls, table walks and loops), so
//! the replacements keep the originals' register use, cycle charges and
//! stack frames exactly: `generated/ee` stays the authority for all three.
//! Indirect calls go through [`crate::call::call_ptr`] (the function table),
//! direct calls through [`Callee`]; both propagate host-thread unwinds.
//! Structured loops deliver events at their back edges with
//! [`event_due`] / [`deliver_event_here`].
//!
//! `ssx_LoadExecPS2` is ported but deliberately not installed: re-executing
//! the ELF runs the CDVD driver's initialisation, which retries its RPC bind
//! without bound on a machine with no CDVD server, so it has no differential
//! test. The two `CollSphereTree` collisions are installed, but their tests
//! only reach the "no collision" path (a real sphere tree is needed to hit).

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

#[rustfmt::skip]
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_7b70, berewardint_is_rival_reward_pending_for, "BERewardInt_IsRivalRewardPendingFor"),
    (0x0015_8af0, berewardint_has_points_tier, "BERewardInt_HasPointsTier"),
    (0x0014_d448, bolts_find_bolt_index, "Bolts_FindBoltIndex"),
    (0x0014_d908, bolts_is_descendant_of, "Bolts_IsDescendantOf"),
    (0x0027_71c8, scriptlist_is_playing_script, "ScriptList_IsPlayingScript"),
    (0x0039_cda0, uiscreen_cmd_send_event, "UIScreen_CmdSendEvent"),
    (0x003a_7b18, worldmemman_create_section_allocator, "WorldMemman_CreateSectionAllocator"),
    (0x001d_5330, memcardstatebase_ctor_2, "MemCardStateBase_ctor_2"),(0x0023_4910, gamestatepreraceeload_check_intro_script, "GameStatePreRaceLoad_CheckIntroScript"),
    (0x0035_8700, railman_on_section_unload, "RailMan_OnSectionUnload"),
    (0x0039_bad0, uipair_v04, "UIPair_v04"),
    (0x001f_7770, ovscreenbcfail_on_widget_created, "OVScreenBCFail_OnWidgetCreated"),
    (0x001a_2bc0, femodule_unload_anim_data, "FEModule_UnloadAnimData"),(0x0019_2d00, cfestatebraggingrights_ctor, "cFEStateBraggingRights_ctor"),
    (0x0039_f698, uistatemgr_close_all_popups, "UIStateMgr_CloseAllPopups"),
    (0x0039_fc48, uiobject_dtor, "UIObject_dtor"),
    (0x0019_6228, cfestateaudiooptions_shutdown, "cFEStateAudioOptions_Shutdown"),
    (0x0019_6e80, cfestaterequestline_shutdown, "cFEStateRequestLine_Shutdown"),
    (0x001a_5450, cfestatenetmessenger_clear_name_list, "cFEStateNetMessenger_ClearNameList"),
    (0x001b_1b10, cfestateonlinecharacterselect_show_character, "cFEStateOnlineCharacterSelect_ShowCharacter"),
    (0x001b_dd60, cfestatelobbymenu_prompt_room_password, "cFEStateLobbyMenu_PromptRoomPassword"),
    (0x001e_5148, ovscreenmessagecenter_delete_message, "OVScreenMessageCenter_DeleteMessage"),
    (0x0036_1eb8, intctable_init, "IntcTable_Init"),
    (0x0025_9288, tagfieldsetstringescaped, "TagFieldSetStringEscaped"),
    (0x0026_0f80, buddyman_create, "BuddyMan_Create"),
    (0x0029_80b0, ssxaudiosystem_stop_all_object_loops, "SSXAudioSystem_StopAllObjectLoops"),
    (0x0029_8d00, ssxaudiosystem_play_hit_boost_sound, "SSXAudioSystem_PlayHitBoostSound"),
    (0x002a_27d8, ssxaudiosystem_djsay20e6, "SSXAudioSystem_DJSay20E6"),(0x002b_0da0, speechsystem_shutdown, "SpeechSystem_Shutdown"),
    (0x002c_acc0, menusyspage_handle_input, "MenuSysPage_HandleInput"),
    (0x002c_bcc8, menusyspage_insert_item, "MenuSysPage_InsertItem"),
    (0x002c_d008, menusysactionitem_handle_input, "MenuSysActionItem_HandleInput"),
    (0x001c_b208, ckeyboardpopup_update, "cKeyboardPopup_Update"),
    (0x003a_d1a8, world_reset_lights, "World_ResetLights"),
    (0x0023_5a18, gamestatereset_load_state, "GameStateReset_LoadState"),
    (0x0023_5fd0, gamestaterestartrace_load_state, "GameStateRestartRace_LoadState"),
    (0x0023_6cd8, gamestatepreracemenu_resume_race, "GameStatePreRaceMenu_ResumeRace"),
    (0x002a_9df0, streambuffer_dtor, "StreamBuffer_dtor"),
    (0x0028_3320, scriptobj_pause, "ScriptObj_Pause"),
    (0x0028_33a8, scriptobj_resume, "ScriptObj_Resume"),
    (0x0025_4368, vd_create_texture_sink, "VD_CreateTextureSink"),
    (0x0014_5a98, benewplayerint_save_slot_defaults, "BENewPlayerInt_SaveSlotDefaults"),
    (0x0014_6e10, benewplayerint_set_char_record_field27c, "BENewPlayerInt_SetCharRecordField27C"),
    (0x0033_08c8, collspheretree_collide_sphere_tree, "CollSphereTree_CollideSphereTree"),
    (0x0033_0950, collspheretree_collide_aabb, "CollSphereTree_CollideAABB"),
    (0x002b_77c0, cworldsound_remove_emitter, "cWorldSound_RemoveEmitter"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR: Callee = Callee {
        addr: 0x001b_1760,
        f: ee::s03::cFEStateOnlineCharacterSelect_ShowOpponentChar,
    };
    pub const C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS: Callee = Callee {
        addr: 0x001b_1b98,
        f: ee::s03::cFEStateOnlineCharacterSelect_ShowCharStats,
    };
    pub const MESSAGE_CENTER_REMOVE_MESSAGE: Callee = Callee {
        addr: 0x001e_31b8,
        f: ee::s04::MessageCenter_RemoveMessage,
    };
    pub const MESSAGE_CENTER_COUNT_GROUP_MESSAGES: Callee = Callee {
        addr: 0x001e_2bb8,
        f: ee::s04::MessageCenter_CountGroupMessages,
    };
    pub const MESSAGE_CENTER_GET_MESSAGE_COUNT: Callee = Callee {
        addr: 0x001e_33e8,
        f: ee::s04::MessageCenter_GetMessageCount,
    };
    pub const OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST: Callee = Callee {
        addr: 0x001e_48b0,
        f: ee::s04::OVScreenMessageCenter_RefreshList,
    };
    pub const OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT: Callee = Callee {
        addr: 0x001e_4758,
        f: ee::s04::OVScreenMessageCenter_UpdateCountText,
    };
    pub const OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS: Callee = Callee {
        addr: 0x001e_4658,
        f: ee::s04::OVScreenMessageCenter_UpdateScrollArrows,
    };
    pub const INTC_TABLE_CLEAR_SLOTS: Callee = Callee {
        addr: 0x0036_1f60,
        f: ee::s12::IntcTable_ClearSlots,
    };
    pub const INTC_TABLE_INSTALL_VIF_VU_HANDLERS: Callee = Callee {
        addr: 0x0036_1e30,
        f: ee::s12::IntcTable_InstallVifVuHandlers,
    };
    pub const GAME_STATE_LOAD_STATE: Callee = Callee {
        addr: 0x0023_3b28,
        f: ee::s06::GameState_LoadState,
    };
    pub const VD_TEXTURE_SINK_CTOR: Callee = Callee {
        addr: 0x0025_4410,
        f: ee::s06::VD_TextureSink_ctor,
    };
    pub const BE_GET_RIDER_PROFILE: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::BE_GetRiderProfile,
    };
    pub const BE_NEW_PLAYER_INT_GET_SLOT: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::BENewPlayerInt_GetSlot,
    };
    pub const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::BELibrary_GetInstance,
    };
    pub const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::BELibrary_GetInterface,
    };
    pub const BE_ECON_INT_GET_TOTAL_EARNED: Callee = Callee {
        addr: 0x0015_09c0,
        f: ee::s01::BEEconInt_GetTotalEarned,
    };
    pub const BOLTS_FIND_BOLT_INDEX: Callee = Callee {
        addr: 0x0014_d448,
        f: ee::s01::Bolts_FindBoltIndex,
    };
    pub const SCRIPT_CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::ScriptCmdQueue_At,
    };
    pub const SCRIPT_LIST_IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::ScriptList_IsMovieEntry,
    };
    pub const SCRIPT_ENGINE_GET_PROCESS_IN_SEQ: Callee = Callee {
        addr: 0x0028_2bf0,
        f: ee::s07::ScriptEngine_GetProcessInSeq,
    };
    pub const SECTION_ALLOCATOR_CTOR: Callee = Callee {
        addr: 0x003a_77b0,
        f: ee::s13::SectionAllocator_ctor,
    };
    pub const UI_STATE_CTOR: Callee = Callee {
        addr: 0x0039_e2a0,
        f: ee::s13::UIState_ctor,
    };
    pub const MEMCARD_STATE_BASE_INSTALL_CALLBACKS: Callee = Callee {
        addr: 0x001d_5488,
        f: ee::s04::MemCardStateBase_InstallCallbacks,
    };
    pub const UI_STATE_MGR_FIND_STATE: Callee = Callee {
        addr: 0x0039_f9d8,
        f: ee::s13::UIStateMgr_FindState,
    };
    pub const UI_STATE_SHUTDOWN: Callee = Callee {
        addr: 0x0039_e4a0,
        f: ee::s13::UIState_Shutdown,
    };
    pub const SCRIPT_ENGINE_LIST_STOP: Callee = Callee {
        addr: 0x0027_90a0,
        f: ee::s07::ScriptEngine_ListStop,
    };
    pub const SCRIPT_ENGINE_LIST_START: Callee = Callee {
        addr: 0x0027_8f68,
        f: ee::s07::ScriptEngine_ListStart,
    };
    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: Callee = Callee {
        addr: 0x0028_b180,
        f: ee::s07::SSXAudioSystem_GetInstance,
    };
    pub const SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS: Callee = Callee {
        addr: 0x0028_71b0,
        f: ee::s07::SSXAudioSystem_ResetRaceSounds,
    };
    pub const SCRIPT_ENGINE_GET_SCRIPT_OBJ: Callee = Callee {
        addr: 0x0027_d320,
        f: ee::s07::ScriptEngine_GetScriptObj,
    };
    pub const SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS: Callee = Callee {
        addr: 0x0027_d790,
        f: ee::s07::ScriptObjCam_IsBoundToProcess,
    };
    pub const SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS: Callee = Callee {
        addr: 0x0027_d210,
        f: ee::s07::ScriptEngine_IsObjBoundToProcess,
    };
    pub const SCRIPT_OBJ_CAM_HAS_BINDING: Callee = Callee {
        addr: 0x0028_2ef0,
        f: ee::s07::ScriptObjCam_HasBinding,
    };
    pub const SPEECH_INSTANCE_DTOR: Callee = Callee {
        addr: 0x002a_fb38,
        f: ee::s08::SpeechInstance_dtor,
    };
    pub const SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON: Callee = Callee {
        addr: 0x0029_f0d8,
        f: ee::s08::SSXAudioSystem_IsDJSpeechOn,
    };
    pub const SPEECH_SYSTEM_SET_EVENT_CONTEXT: Callee = Callee {
        addr: 0x002b_1458,
        f: ee::s08::SpeechSystem_SetEventContext,
    };
    pub const SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK: Callee = Callee {
        addr: 0x0029_f198,
        f: ee::s08::SSXAudioSystem_GetLanguageMask,
    };
    pub const MENU_SYS_PAGE_VALIDATE_SELECTION: Callee = Callee {
        addr: 0x002c_aab0,
        f: ee::s09::MenuSysPage_ValidateSelection,
    };
    pub const MENU_SYS_PAGE_SELECT_NEXT: Callee = Callee {
        addr: 0x002c_ab08,
        f: ee::s09::MenuSysPage_SelectNext,
    };
    pub const MENU_SYS_PAGE_SELECT_PREV: Callee = Callee {
        addr: 0x002c_abe0,
        f: ee::s09::MenuSysPage_SelectPrev,
    };
    pub const MENU_SYS_ITEM_HANDLE_INPUT: Callee = Callee {
        addr: 0x002c_a2b0,
        f: ee::s09::MenuSysItem_HandleInput,
    };
    pub const MENU_SYS_ITEM_IS_DISABLED: Callee = Callee {
        addr: 0x002c_a3b0,
        f: ee::s09::MenuSysItem_IsDisabled,
    };
    pub const MENU_SYS_ITEM_GET_INPUT_CODE: Callee = Callee {
        addr: 0x002c_a3e8,
        f: ee::s09::MenuSysItem_GetInputCode,
    };
    pub const SCRIPT_BINDING_PAUSE_SEQ: Callee = Callee {
        addr: 0x0028_3038,
        f: ee::s07::ScriptBinding_PauseSeq,
    };
    pub const SCRIPT_BINDING_RESUME_SEQ: Callee = Callee {
        addr: 0x0028_3060,
        f: ee::s07::ScriptBinding_ResumeSeq,
    };
    pub const GAME_ANIM_MAP_UNREGISTER_BANK: Callee = Callee {
        addr: 0x0031_4fe8,
        f: ee::s10::GameAnimMap_UnregisterBank,
    };
    pub const ANIM_DATA_BASE_FREE_CHAN_CACHES: Callee = Callee {
        addr: 0x0031_1110,
        f: ee::s10::AnimDataBase_FreeChanCaches,
    };
    pub const ADF_DATA_SOURCE_DTOR: Callee = Callee {
        addr: 0x0031_12c8,
        f: ee::s10::AdfDataSource_dtor,
    };
    pub const C_KEYBOARD_POPUP_SET_KEY_ENABLED: Callee = Callee {
        addr: 0x001c_e3c8,
        f: ee::s04::cKeyboardPopup_SetKeyEnabled,
    };
    pub const RIDER_CLEAR_NEARBY_LIGHTS: Callee = Callee {
        addr: 0x0012_2658,
        f: ee::s00::Rider_ClearNearbyLights,
    };
    pub const SNDSTRM_CLOSE: Callee = Callee {
        addr: 0x003b_7838,
        f: ee::s13::SNDSTRM_Close,
    };
    pub const EA_MEM_FREE: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::EA_MemFree,
    };
    pub const BUILTIN_DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::__builtin_delete,
    };
    pub const NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID: Callee = Callee {
        addr: 0x001a_9150,
        f: ee::s03::NetFEState_OpenKeyboardTextId,
    };
    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    };
    pub const SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR: Callee = Callee {
        addr: 0x0028_7968,
        f: ee::s07::SSXAudioSystem_GetChannelGainPtr,
    };
    pub const BANK_MONITOR_ADD_TIMER: Callee = Callee {
        addr: 0x002a_dca0,
        f: ee::s08::BankMonitor_AddTimer,
    };
    pub const BANK_MONITOR_STOP_HANDLE: Callee = Callee {
        addr: 0x002a_d5f0,
        f: ee::s08::BankMonitor_StopHandle,
    };
    pub const UI_LIST_FIRST: Callee = Callee {
        addr: 0x0039_7840,
        f: ee::s13::UIList_First,
    };
    pub const UI_LIST_NODE_DTOR_BODY: Callee = Callee {
        addr: 0x003a_4868,
        f: ee::s13::UIListNode_dtorBody,
    };
    pub const C_GAME_COMM_RECV_GAME_PACKET: Callee = Callee {
        addr: 0x0025_8160,
        f: ee::s06::cGameComm_RecvGamePacket,
    };
    pub const C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS: Callee = Callee {
        addr: 0x0019_26f0,
        f: ee::s02::cFEStateBraggingRights_InitPlayerRows,
    };

    // Cross-shard calls the generator emitted as `call_indirect` (direct
    // `jal`s whose target lives in another shard).
    pub const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
    pub const MEM_MOVE: u32 = 0x0041_610c;
    pub const STR_COPY: u32 = 0x0041_66f4;
    pub const TAG_FIELD_SET_STRING: u32 = 0x003e_c180;
    pub const STR_N_COPY: u32 = 0x0041_6ce0;
    pub const SPHERE_TREE_COLLIDE_SPHERE_TREE: u32 = 0x0032_cb58;
    pub const SPHERE_TREE_COLLIDE_AABB: u32 = 0x0032_c928;
    pub const SPCH_SHUTDOWN: u32 = 0x003d_67f8;
    pub const SPCH_MAKE_ID: u32 = 0x003d_8008;
    pub const MEM_FREE: u32 = 0x0025_23a8;
    pub const STR_HASH: u32 = 0x0031_7670;
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off` (32-bit pointer arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gw(c: &Cpu, off: i32) -> u32 {
    abi::r32(c, GP).wrapping_add(off as u32)
}

/// `mult rd, rs, rt` (LO/HI of pipeline 0, `rd = lo`).
#[inline(always)]
fn mul32(c: &mut Cpu, rd: usize, rs: usize, rt: usize) {
    c.mult(0, rd, rs, rt);
}

/// `lw` (sign-extended) from a 32-bit address.
#[inline(always)]
fn lw_at(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}

/// `lh` (sign-extended).
#[inline(always)]
fn lh_at(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lh(c)
}

/// The two-instruction virtual-method read the originals use:
/// `a0 = lh(vt+off)`, `func = lw(vt+off+4)`. The original leaves `func` in
/// `vreg` (v0 or v1 depending on the site), which callers can see, so the
/// register is written here too. Returns `func` for the `call_ptr`.
#[inline(always)]
fn vslot(c: &mut Cpu, vt: u64, off: u32, vreg: usize) -> u32 {
    let base = (vt as u32).wrapping_add(off);
    c.g[A0][0] = Addr(base).lh(c);
    let func = Addr(base.wrapping_add(4)).lw(c);
    c.g[vreg][0] = func;
    func as u32
}
// ---------------------------------------------------------------------------
// Back-end rewards and bolt (grind-rail) lookups.

/// `int BERewardInt_IsRivalRewardPendingFor(a, b, _, peak)` (0x157b70): bit
/// 12 (peak 1) or bit 13 (peak 2) of the 64-bit flag word at
/// `0x4a6ca8 + 0xf88*b + 0x9b50*a + 0x278`; 0 for any other peak.
pub fn berewardint_is_rival_reward_pending_for(c: &mut Cpu) {
    let peak = abi::arg(c, 3);
    c.cycles += 3;
    c.g[V0][0] = 1;
    if peak == 1 {
        c.cycles += 13;
        c.g[V1][0] = 0xf88;
        let bit = rival_flag_bit(c, 20);
        c.g[V0][0] = bit;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let second = peak == 2;
    c.g[V1][0] = 0xf88;
    if second {
        c.cycles += 12;
        let bit = rival_flag_bit(c, 19);
        c.g[V0][0] = bit;
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// The shared tail: `((*(u64*)(0x4a6ca8 + 0xf88*b + 0x9b50*a + 0x278) << shift)
/// >> 32) & 1`, with the multiplications' LO/HI and scratch registers as the
/// original leaves them.
fn rival_flag_bit(c: &mut Cpu, shift: u32) -> u64 {
    c.g[A0][0] = 0x9b50;
    mul32(c, V1, A2, V1); // V1 = b * 0xf88
    mul32(c, A0, A1, A0); // A0 = a * 0x9b50
    c.g[V0][0] = 0x4a0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let word = lw_at(c, p(c.g[V1][0] as u32, 0x278));
    let shifted = (word << shift) as i64 >> 32;
    (shifted as u64) & 1
}

/// `int BERewardInt_HasPointsTier(index, a, b, medal)` (0x158af0): 1 when the
/// rider's total earned points reach `D_0045B048[medal]`, else -1.
pub fn berewardint_has_points_tier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x0015_8b14)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xb;
        callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x0015_8b20)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A0][0] = c.g[V0][0];
        callee::BE_ECON_INT_GET_TOTAL_EARNED.call(c, 0x0015_8b30)?;
        c.g[V1][0] = 0x460000;
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4fb8));
        c.g[A1][0] = 1;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = lw_at(c, c.g[S0][0] as u32);
        frame.restore(c, 16, 0x30);
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        frame.restore(c, 17, 0x20);
        if c.g[V0][0] == 0 {
            c.g[A0][0] = c.g[A1][0];
        }
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[A0][0];
        frame.close(c);
        Ok(())
    })
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
    let table = lw_at(c, p(slot, 0x4dc));
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
    let first = lw_at(c, p(slot, 0xa4));
    let count = lw_at(c, p(slot, 0x2c));
    c.g[A3][0] = first;
    c.g[V1][0] = count;
    mul32(c, A0, A0, A3);
    let base = lw_at(c, p(self_, 4));
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
            c.g[V1][0] = lw_at(c, p(self_, 4));
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
// Script lists.

/// `int ScriptList_IsPlayingScript(self, process, script_obj_id)` (0x2771c8):
/// 0 unless the list is in state 3 and its current command is not a movie,
/// then whether `process` is the one running the command's script.
pub fn scriptlist_is_playing_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 3;
        frame.save(c, 16, 0x30);
        frame.save(c, 18, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        let state = lw_at(c, p(r32(c, S0), 0xa4));
        c.g[V1][0] = state;
        let not_ready = state != c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        if not_ready {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 9;
            c.g[A0][0] = sx(p(r32(c, S0), 0xc));
            c.g[A1][0] = 0;
            callee::SCRIPT_CMD_QUEUE_AT.call(c, 0x0027_71fc)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            callee::SCRIPT_LIST_IS_MOVIE_ENTRY.call(c, 0x0027_720c)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 6;
                c.g[A1][0] = lw_at(c, p(r32(c, S1), 0xc));
                c.g[A0][0] = lw_at(c, gw(c, -0x84c));
                callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ.call(c, 0x0027_7220)?;
                c.g[V0][0] ^= c.g[S2][0];
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            }
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

// ---------------------------------------------------------------------------
// UI screens.

/// `void UIScreen_CmdSendEvent(self, msg, cmd)` (0x39cda0): dispatch a
/// command through the screen's controller (`self+0xd0`), falling back to the
/// controller's child when the first handler declines.
pub fn uiscreen_cmd_send_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = c.g[S0][0];
        let ctrl = lw_at(c, p(r32(c, S0), 0xd0));
        c.g[A3][0] = ctrl;
        c.g[A2][0] = lw_at(c, p(r32(c, S1), 4));
        c.g[V1][0] = lw_at(c, p(ctrl as u32, 8));
        let func = vslot(c, c.g[V1][0], 0x80, V0);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, 0x0039_cdd8)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, 16, 0x20);
        } else {
            c.cycles += 5;
            c.g[V0][0] = lw_at(c, p(r32(c, S0), 0xd0));
            c.g[V1][0] = lw_at(c, p(c.g[V0][0] as u32, 0x10));
            c.g[A3][0] = lw_at(c, c.g[V1][0] as u32);
            let none = c.g[A3][0] == 0;
            c.g[A1][0] = c.g[S0][0];
            if none {
                c.cycles += 1;
                frame.restore(c, 16, 0x20);
            } else {
                c.cycles += 6;
                c.g[V1][0] = lw_at(c, p(c.g[A3][0] as u32, 4));
                c.g[A2][0] = lw_at(c, p(r32(c, S1), 4));
                let func = vslot(c, c.g[V1][0], 0x18, V0);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x0039_ce0c)?;
                c.cycles += 1;
                frame.restore(c, 16, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldMemman_CreateSectionAllocator(self, index, section)`
/// (0x3a7b18): `self->allocators[index] = new SectionAllocator(
/// EA_MemAllocNamed(0x14, ...), &sections[section], index)`.
pub fn worldmemman_create_section_allocator(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x50);
        c.g[A3][0] = 0;
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 16, 0x40);
        frame.save(c, 19, 0x10);
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
        c.g[S3][0] = c.g[A2][0];
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3c));
        c.g[A1][0] = 0x490000;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4de8));
        c.g[A0][0] = 0x14;
        c.g[A2][0] = 0x20000000;
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x003a_7b60)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x14;
        mul32(c, V0, S3, A1);
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        callee::SECTION_ALLOCATOR_CTOR.call(c, 0x003a_7b78)?;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
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
        callee::UI_STATE_CTOR.call(c, 0x001d_5354)?;
        c.g[V1][0] = 0x470000;
        c.sw(p(r32(c, S0), 0x98), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3680));
        c.sw(p(r32(c, S0), 0x48), 0);
        c.sw(p(r32(c, S0), 8), c.g[V1][0]);
        c.g[A2][0] = 0x4a0000;
        c.sw(p(r32(c, S0), 0x4c), 0);
        c.g[V0][0] = 0x470000;
        c.sw(p(r32(c, S0), 0x54), 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3680));
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = Addr(p(c.g[A2][0] as u32, 0x1390)).lhu(c);
        c.sw(p(r32(c, S0), 8), c.g[V0][0]);
        c.sh(p(r32(c, S0), 0x58), c.g[V1][0]);
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

/// `void cUITextScroll_RefreshText(self)` (0x3a1360): re-resolve the text id
/// (`self+0xc0`) through the UI object chain and replace the wide string,
/// then relayout the widget.
pub fn cuitextscroll_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        let id = lw_at(c, p(self_, 0xc0));
        if id == 0 {
            frame.restore(c, 16, 0x10);
        } else {
            c.cycles += 6;
            c.g[V0][0] = lw_at(c, p(self_, 0x5c));
            c.g[V1][0] = lw_at(c, p(c.g[V0][0] as u32, 0xd0));
            c.g[A0][0] = lw_at(c, p(c.g[V1][0] as u32, 0x10));
            c.g[A2][0] = lw_at(c, p(c.g[A0][0] as u32, 0x10));
            if c.g[A2][0] == 0 {
                frame.restore(c, 16, 0x10);
            } else {
                c.cycles += 7;
                c.g[V1][0] = lw_at(c, p(c.g[A2][0] as u32, 4));
                let func = vslot(c, c.g[V1][0], 0x20, V0);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x003a_13a8)?;
                let text = c.g[V0][0];
                c.g[A1][0] = text;
                if text != 0 {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = text;
                    call_ptr(c, 0x003a_1cf0, 0x003a_13b8)?;
                    c.g[A1][0] = text;
                }
                c.cycles += 7;
                c.g[V0][0] = lw_at(c, p(r32(c, S0), 8));
                c.g[A1][0] = 1;
                let func = vslot(c, c.g[V0][0], 0x28, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x003a_13d0)?;
                frame.restore(c, 16, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
/// `void GameStatePreRaceLoad_CheckIntroScript(self)` (0x234910): start the
/// intro script when the engine is in state 2 and mark `self+0x14`.
pub fn gamestatepreraceeload_check_intro_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let engine = lw_at(c, gw(c, -0x84c));
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = engine;
        c.g[V0][0] = lw_at(c, p(engine as u32, 0x550));
        c.g[V0][0] ^= 2;
        let not_state = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if !not_state {
            c.cycles += 5;
            c.g[A0][0] = c.g[V1][0];
            c.g[A1][0] = 1;
            c.g[A2][0] = 1;
            c.g[A3][0] = 1;
            call_ptr(c, callee::SCRIPT_ENGINE_LIST_START.addr, 0x0023_4944)?;
        }
        c.cycles += 5;
        c.g[A0][0] = lw_at(c, gw(c, -0x84c));
        c.g[V0][0] = lw_at(c, p(c.g[A0][0] as u32, 0x550));
        c.g[V0][0] ^= 2;
        if c.g[V0][0] != 0 {
            c.sw(p(r32(c, S0), 0x14), 0);
        } else {
            c.cycles += 5;
            c.g[A1][0] = 0;
            c.g[A2][0] = 1;
            c.g[A3][0] = 1;
            call_ptr(c, callee::SCRIPT_ENGINE_LIST_START.addr, 0x0023_4968)?;
            c.sw(p(r32(c, S0), 0x14), 0);
        }
        c.cycles += 5;
        c.g[V1][0] = 1;
        c.g[V0][0] = lw_at(c, gw(c, -0x84c));
        c.g[V0][0] = lw_at(c, p(c.g[V0][0] as u32, 0x550));
        if c.g[V0][0] == c.g[V1][0] {
            c.sw(p(r32(c, S0), 0x14), c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RailMan_OnSectionUnload(kind)` (0x358700): for every rail slot
/// (`D_004A4040`, 128 entries) with a kind tag matching `kind`, re-enable the
/// rail.
pub fn railman_on_section_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = lw_at(c, gw(c, 0xf50));
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        let empty = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !empty {
            c.cycles += 4;
            c.g[S1][0] = 0xffff_ffff_ffff_0000;
            c.g[S0][0] = 0;
            c.g[S1][0] |= 0xffff;
            let mut m = lw_at(c, gw(c, 0xf50));
            loop {
                c.cycles += 5;
                c.g[A0][0] = m;
                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                let slot = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V0][0] = slot;
                c.g[V1][0] = lw_at(c, p(slot as u32, 0x18));
                let dead = c.g[V1][0] == c.g[S1][0];
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                if !dead {
                    c.cycles += 3;
                    c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x18)).lbu(c);
                    if c.g[V1][0] != c.g[S2][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x0035_8998, 0x0035_8758)?;
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    }
                } else {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < 128) as u64;
                c.g[A0][0] = lw_at(c, gw(c, 0xf50));
                if c.g[V0][0] == 0 {
                    break;
                }
                m = c.g[A0][0];
                if event_due(c) {
                    deliver_event_here(c, 0x0035_8730);
                }
            }
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

/// `void UIPair_v04(self, on)` (0x39bad0): store the pair's on flag
/// (`self+0x14`, bit 1) and notify the two child widgets through their
/// virtual `on` handlers.
pub fn uipair_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = 0xffff_ffff_ffff_fffd;
        c.g[V0][0] = (0u64 < c.g[S1][0]) as u64;
        c.g[V1][0] = lw_at(c, p(r32(c, S0), 0x14));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[A2][0] = lw_at(c, p(r32(c, S0), 0x78));
        c.g[V1][0] &= c.g[A0][0];
        c.g[V1][0] |= c.g[V0][0];
        c.sw(p(r32(c, S0), 0x14), c.g[V1][0]);
        c.g[V0][0] = lw_at(c, p(c.g[A2][0] as u32, 8));
        let func = vslot(c, c.g[V0][0], 0x20, V1);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, func, 0x0039_bb1c)?;
        c.g[A2][0] = lw_at(c, p(r32(c, S0), 0x7c));
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !none {
            c.cycles += 5;
            c.g[V0][0] = lw_at(c, p(c.g[A2][0] as u32, 8));
            let func = vslot(c, c.g[V0][0], 0x20, V1);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, func, 0x0039_bb3c)?;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenBCFail_OnWidgetCreated(self, widget)` (0x1f7770): pick the
/// failure screen's mode (0..2) from the created widget's name hash.
pub fn ovscreenbcfail_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x4a0000;
        frame.save(c, 17, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23e0));
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 16, 0x20);
        c.g[S0][0] = lw_at(c, p(r32(c, S1), 0x38));
        call_ptr(c, callee::STR_HASH, 0x001f_7794)?;
        let differs = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x4a0000;
        if !differs {
            c.cycles += 2;
            c.sw(p(r32(c, S1), 0x18), 0);
            c.cycles += 1;
            frame.restore(c, 16, 0x20);
        } else {
            c.cycles += 5;
            c.g[S0][0] = lw_at(c, p(r32(c, S1), 0x38));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23e8));
            call_ptr(c, callee::STR_HASH, 0x001f_77b0)?;
            if c.g[S0][0] != c.g[V0][0] {
                c.g[A0][0] = 0x4a0000;
                c.cycles += 5;
                c.g[S0][0] = lw_at(c, p(r32(c, S1), 0x38));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23f8));
                call_ptr(c, callee::STR_HASH, 0x001f_77cc)?;
                let differs = c.g[S0][0] != c.g[V0][0];
                frame.restore(c, 16, 0x20);
                if differs {
                    c.cycles += 4;
                    frame.restore(c, 17, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 1;
                c.g[V0][0] = 2;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
            }
            c.cycles += 1;
            c.sw(p(r32(c, S1), 0x18), c.g[V0][0]);
            c.cycles += 1;
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEModule_UnloadAnimData(self)` (0x1a2bc0): drop the first animation
/// bank: unregister the bank, free its channel caches and destroy the data
/// source.
pub fn femodule_unload_anim_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[S2][0] & 0xff;
        loop {
            c.cycles += 7;
            let banks = lw_at(c, gw(c, 0xd08));
            c.g[V0][0] = banks;
            c.g[S1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[S0][0] = lw_at(c, c.g[V0][0] as u32);
            let none = c.g[S0][0] == 0;
            c.g[A1][0] = c.g[S0][0];
            if !none {
                c.cycles += 9;
                c.g[A0][0] = lw_at(c, gw(c, 0xd8c));
                call_ptr(c, callee::GAME_ANIM_MAP_UNREGISTER_BANK.addr, 0x001a_2c04)?;
                c.g[A0][0] = lw_at(c, gw(c, 0xd08));
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.sw(c.g[V0][0] as u32, 0);
                call_ptr(c, callee::ANIM_DATA_BASE_FREE_CHAN_CACHES.addr, 0x001a_2c14)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 3;
                call_ptr(c, callee::ADF_DATA_SOURCE_DTOR.addr, 0x001a_2c20)?;
            }
            c.cycles += 2;
            let again = (c.g[S2][0] as i64) <= 0;
            c.g[V1][0] = c.g[S2][0] & 0xff;
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_2be0);
            }
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

/// `void OVScreenReadPrivateMessage_TransitionIn(self, has_sender)`
/// (0x210b58): when a sender is given, copy its message into the screen;
/// otherwise open the on-screen keyboard for a new message.
pub fn ovscreenreadprivatemessage_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let no_sender = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        if !no_sender {
            c.cycles += 7;
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
            c.g[A0][0] = lw_at(c, gw(c, -0x204));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0x41;
            call_ptr(c, callee::C_GAME_COMM_RECV_GAME_PACKET.addr, 0x0021_0b84)?;
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !empty {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0;
                call_ptr(c, 0x0021_0bd8, 0x0021_0b98)?;
                c.cycles += 5;
                c.g[V0][0] = 1;
                frame.restore(c, 16, 0x20);
                frame.restore(c, 17, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 8;
        c.g[A2][0] = 0x4a2560;
        c.g[A1][0] = 0;
        c.g[T0][0] = 0;
        c.g[A3][0] = c.g[A2][0];
        c.g[T1][0] = 0;
        c.g[T2][0] = 0x3d;
        call_ptr(c, 0x0021_0f40, 0x0021_0bc0)?;
        c.cycles += 1;
        frame.restore(c, 16, 0x20);
        c.cycles += 5;
        c.g[V0][0] = 1;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateBraggingRights_ctor(self, ...)` (0x192d00): construct the state,
/// set its vtable and clear the two player rows (`0x34` bytes each).
pub fn cfestatebraggingrights_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0;
        frame.save(c, 18, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save(c, 19, 0x10);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6d0));
        frame.save_ra(c, 0);
        c.g[S1][0] = 1;
        call_ptr(c, 0x001a_8500, 0x0019_2d2c)?;
        c.g[S3][0] = u64::MAX;
        c.g[V0][0] = 0x470000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5098));
        c.sw(p(r32(c, S2), 8), c.g[V0][0]);
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            call_ptr(
                c,
                callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS.addr,
                0x0019_2d4c,
            )?;
            let more = c.g[S1][0] != c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_2d40);
            }
        }
        c.cycles += 11;
        c.g[V1][0] = 0x14;
        c.g[V0][0] = c.g[S2][0];
        c.sw(p(r32(c, S2), 0xc), c.g[V1][0]);
        c.sw(p(r32(c, S2), 0x738), 0);
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIStateMgr_CloseAllPopups()` (0x39f698): walk the popup list and
/// clear the "popping" bits (7, 8, and the 3-bit popup kind when it is below
/// 6) of every entry; returns the last node.
pub fn uistatemgr_close_all_popups(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::UI_LIST_FIRST.call(c, 0x0039_f6a8)?;
        let node = c.g[V0][0];
        c.g[A0][0] = node;
        let mut v0 = 0u64;
        if node != 0 {
            c.cycles += 4;
            c.g[V0][0] = lw_at(c, p(node as u32, 0x1c));
            c.g[V0][0] &= 1;
            c.g[A2][0] = 0xffff_ffff_ffff_c0ff;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = 0xffff_ffff_ffff_ff7f;
                c.g[V1][0] = lw_at(c, p(node as u32, 0x1c));
                loop {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    c.g[V0][0] &= 0x3f;
                    c.g[V0][0] = (c.g[V0][0] < 6) as u64;
                    let small = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[V1][0] & c.g[A2][0];
                    if !small {
                        c.cycles += 4;
                        c.g[V0][0] |= 0x700;
                        c.g[V0][0] &= c.g[A1][0];
                        c.g[V0][0] |= 0x80;
                        c.sw(p(c.g[A0][0] as u32, 0x1c), c.g[V0][0]);
                    }
                    c.cycles += 5;
                    c.g[A0][0] = lw_at(c, p(c.g[A0][0] as u32, 4));
                    c.g[V0][0] = lw_at(c, p(c.g[A0][0] as u32, 0x1c));
                    c.g[V0][0] &= 1;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V1][0] = lw_at(c, p(c.g[A0][0] as u32, 0x1c));
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_f6d0);
                    }
                }
            }
            c.cycles += 1;
            v0 = c.g[A0][0];
        }
        c.g[V0][0] = v0;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIObject_dtor(self, flags)` (0x39fc48): destroy the child object, then
/// the node body, and free the object when `flags & 1`.
pub fn uiobject_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x490000;
        frame.save(c, 16, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4bd8));
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(p(r32(c, S0), 8), c.g[V0][0]);
        c.g[A2][0] = lw_at(c, p(r32(c, S0), 0xc));
        let no_child = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_child {
            c.cycles += 6;
            c.g[V0][0] = lw_at(c, p(c.g[A2][0] as u32, 0x10));
            c.g[A1][0] = 3;
            let func = vslot(c, c.g[V0][0], 8, V1);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, func, 0x0039_fc8c)?;
        }
        c.cycles += 8;
        c.g[V0][0] = 0x490000;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4cc0));
        c.sw(p(r32(c, S0), 8), c.g[V0][0]);
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039_fca0)?;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::BUILTIN_DELETE.addr, 0x0039_fcb4)?;
        }
        frame.restore(c, 16, 0x20);
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
/// `void cFEStateAudioOptions_Shutdown(self)` (0x1962a0 area, 0x196228):
/// find the state's audio-options UI state and shut the screen down.
pub fn cfestateaudiooptions_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = lw_at(c, gw(c, -0x848));
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, p(c.g[V1][0] as u32, 0x84));
        let no_race = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        if !no_race {
            c.cycles += 11;
            c.g[S0][0] = lw_at(c, p(r32(c, S1), 0x10));
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18a8));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
            call_ptr(c, callee::STR_HASH, 0x0019_625c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_STATE_MGR_FIND_STATE.addr, 0x0019_6268)?;
            c.g[A3][0] = c.g[V0][0];
            let none = c.g[A3][0] == 0;
            c.g[A1][0] = 4;
            if !none {
                c.cycles += 6;
                c.g[V1][0] = lw_at(c, p(c.g[A3][0] as u32, 8));
                c.g[A2][0] = 0;
                let func = vslot(c, c.g[V1][0], 0xc0, V0);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x0019_628c)?;
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::UI_STATE_SHUTDOWN.addr, 0x0019_6294)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRequestLine_Shutdown(self)` (0x196e80): the request-line
/// twin of the audio-options shutdown (vtable slot at `+0xc0`).
pub fn cfestaterequestline_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = lw_at(c, gw(c, -0x848));
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, p(c.g[V1][0] as u32, 0x84));
        let no_race = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        if !no_race {
            c.cycles += 11;
            c.g[S0][0] = lw_at(c, p(r32(c, S1), 0x10));
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18a8));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
            call_ptr(c, callee::STR_HASH, 0x0019_6eb4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_STATE_MGR_FIND_STATE.addr, 0x0019_6ec0)?;
            c.g[A3][0] = c.g[V0][0];
            let none = c.g[A3][0] == 0;
            c.g[A1][0] = 4;
            if !none {
                c.cycles += 6;
                c.g[V1][0] = lw_at(c, p(c.g[A3][0] as u32, 8));
                c.g[A2][0] = 0;
                let func = vslot(c, c.g[V1][0], 0xc0, V0);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x0019_6ee4)?;
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::UI_STATE_SHUTDOWN.addr, 0x0019_6eec)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ssx_LoadExecPS2(path, args, env)` (0x31b008): flush the CD/DVD
/// streams, shut the command server down and re-execute the ELF.
///
/// Ported for completeness but **not installed and not tested**: the
/// re-execution path runs the driver's initialisation (`sceCdInit` retries
/// its RPC bind forever on a machine without the CDVD server), so no fixture
/// can bound it.
pub fn ssx_loadexecps2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        c.g[A3][0] = lw_at(c, gw(c, 0x2a74));
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        c.g[V0][0] = lw_at(c, p(c.g[A3][0] as u32, 0x5c));
        let func = vslot(c, c.g[V0][0], 0x10, V1);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, 0x0031_b040)?;
        c.g[A0][0] = 2;
        call_ptr(c, 0x0040_2450, 0x0031_b048)?;
        c.g[A0][0] = 0;
        call_ptr(c, 0x0040_1718, 0x0031_b050)?;
        c.g[A0][0] = 5;
        call_ptr(c, 0x0040_1718, 0x0031_b058)?;
        call_ptr(c, 0x0042_5fb8, 0x0031_b060)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, 0x0042_c6a0, 0x0031_b070)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateNetMessenger_ClearNameList(self)` (0x1a5450): drop the
/// queued player names from `self+0x6e8 .. +0x6ec`, one `lw` at a time.
///
/// Dispatcher-shaped in the original (`0x1a5470` is its loop head): it can
/// be re-entered there after an event unwind, so the port accepts every
/// resume entry and keeps its state in the registers.
pub fn cfestatenetmessenger_clear_name_list(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x001a_5450 => {
                    c.cycles += 7;
                    c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
                    c.sq(18, p(sp(c), 0x10));
                    c.sq(16, p(sp(c), 0x30));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(17, p(sp(c), 0x20));
                    c.sd(p(sp(c), 0), c.g[RA][0]);
                    pc = 0x001a_5484;
                }
                0x001a_5470 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
                    let done = c.g[A0][0] == 0;
                    c.sw(p(r32(c, S2), 0x6ec), c.g[V0][0]);
                    pc = if done { 0x001a_5484 } else { 0x001a_547c };
                }
                0x001a_547c => {
                    c.cycles += 2;
                    c.g[A1][0] = 3;
                    call_ptr(c, 0x0031_8260, 0x001a_5484)?;
                    pc = 0x001a_5484;
                }
                0x001a_5484 => {
                    c.cycles += 4;
                    c.g[V1][0] = lw_at(c, p(r32(c, S2), 0x6e8));
                    c.g[V0][0] = lw_at(c, p(r32(c, S2), 0x6ec));
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A0][0] = lw_at(c, (c.g[V0][0] as u32).wrapping_sub(4));
                        if event_due(c) && back_edge_unwinds(c, 0x001a_5470) {
                            return Ok(());
                        }
                        pc = 0x001a_5470;
                    } else {
                        pc = 0x001a_5494;
                    }
                }
                0x001a_5494 => {
                    c.cycles += 16;
                    c.g[S1][0] = lw_at(c, p(r32(c, S2), 0x6e8));
                    c.g[A2][0] = 0;
                    c.g[S0][0] = lw_at(c, p(r32(c, S2), 0x6ec));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, callee::MEM_MOVE, 0x001a_54ac)?;
                    c.g[V1][0] = lw_at(c, p(r32(c, S2), 0x6ec));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    frame_restore_at(c, 17, 0x20);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                    c.g[RA][0] = c.ld(sp(c));
                    c.sw(p(r32(c, S2), 0x6ec), c.g[V1][0]);
                    frame_restore_at(c, 16, 0x30);
                    frame_restore_at(c, 18, 0x10);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001a_5450, 0x001a_54d4);
                    return Ok(());
                }
            }
        }
    })
}

/// `lq reg, off(sp)` at the current `sp`.
#[inline(always)]
fn frame_restore_at(c: &mut Cpu, reg: usize, off: u32) {
    c.lq(reg, p(sp(c), off));
}

/// `void cFEStateOnlineCharacterSelect_ShowCharacter(self, char, opponent)`
/// (0x1b1b10): show the character rows and their stats, showing the opponent
/// char on the current slot when one is selected.
pub fn cfestateonlinecharacterselect_show_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        loop {
            c.cycles += 2;
            c.g[A3][0] = c.g[S0][0] ^ c.g[S2][0];
            let mut plus_one = true;
            if c.g[S0][0] == c.g[S2][0] {
                c.cycles += 3;
                c.g[A1][0] = c.g[S0][0];
                let no_opponent = c.g[S1][0] == 0;
                c.g[A0][0] = c.g[S3][0];
                if no_opponent {
                    plus_one = false;
                } else {
                    c.cycles += 3;
                    call_ptr(
                        c,
                        callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR.addr,
                        0x001b_1b54,
                    )?;
                    c.g[A3][0] = c.g[S0][0] ^ c.g[S2][0];
                }
            }
            if plus_one {
                c.cycles += 1;
                c.g[A1][0] = c.g[S0][0];
            }
            c.cycles += 8;
            c.g[A3][0] = (c.g[A3][0] < 1) as u64;
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            call_ptr(
                c,
                callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS.addr,
                0x001b_1b70,
            )?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 30) as u64;
            let again = c.g[V0][0] != 0;
            c.g[RA][0] = c.ld(sp(c));
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001b_1b38);
            }
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateLobbyMenu_PromptRoomPassword(self, a1, event)` (0x1bdd60):
/// when the child event is 0xf, open the room-password keyboard; otherwise
/// forward the event to the state base.
pub fn cfestatelobbymenu_prompt_room_password(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let not_id = c.g[A2][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if !not_id {
            c.cycles += 21;
            c.g[A2][0] = 0xf;
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001b_dd80)?;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50d0));
            call_ptr(c, callee::STR_HASH, 0x001b_dd8c)?;
            c.g[A3][0] = 0x4a0000;
            c.g[A2][0] = c.g[V0][0];
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1408));
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.g[T0][0] = 0;
            c.g[T1][0] = 8;
            c.g[T2][0] = 0x3f;
            c.g[RA][0] = sx(0x001b_ddb0);
            c.pc = callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID.addr;
            call_ptr(
                c,
                callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID.addr,
                0x001b_ddb0,
            )?;
            c.g[A0][0] = lw_at(c, p(r32(c, S0), 0x69c));
            c.g[A1][0] = 0x35;
            c.g[A2][0] = 0;
            c.g[A3][0] = 2;
            call_ptr(
                c,
                callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED.addr,
                0x001b_ddc4,
            )?;
            frame.restore(c, 16, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001b_ddd4)?;
        frame.restore(c, 16, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenMessageCenter_DeleteMessage(self, index)` (0x1e5148):
/// remove the message, fix up the group's seen counter and refresh the list,
/// count and scroll arrows.
pub fn ovscreenmessagecenter_delete_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 18, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, callee::MESSAGE_CENTER_REMOVE_MESSAGE.addr, 0x001e_5168)?;
        c.g[V0][0] = lw_at(c, p(r32(c, S2), 0x144));
        c.g[A1][0] = 1;
        c.g[A0][0] = 4;
        c.g[S1][0] = Addr(p(c.g[V0][0] as u32, 0x95)).lbu(c);
        call_ptr(
            c,
            callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES.addr,
            0x001e_517c,
        )?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(
            c,
            callee::MESSAGE_CENTER_GET_MESSAGE_COUNT.addr,
            0x001e_5184,
        )?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = lw_at(c, p(r32(c, S2), 0x144));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
        c.g[S0][0] = ((c.g[S1][0] as i64) < (c.g[S0][0] as i64)) as u64;
        if c.g[S0][0] == 0 {
            c.g[S1][0] = c.g[V0][0];
        }
        c.g[A0][0] = c.g[S2][0];
        c.sb(p(c.g[V1][0] as u32, 0x95), c.g[S1][0]);
        call_ptr(
            c,
            callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST.addr,
            0x001e_51a4,
        )?;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(
            c,
            callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT.addr,
            0x001e_51ac,
        )?;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(
            c,
            callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS.addr,
            0x001e_51b4,
        )?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void IntcTable_Init(table)` (0x361eb8): fill the 16 interrupt slots and
/// their enable masks with -1, then clear the slots and install the VIF/VU
/// handlers.
pub fn intctable_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0xf;
        frame.save(c, 16, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3c));
        loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_1ed8);
            }
        }
        c.cycles += 3;
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = 0xf;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x7c));
        loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_1f00);
            }
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::INTC_TABLE_CLEAR_SLOTS.addr, 0x0036_1f24)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(
            c,
            callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS.addr,
            0x0036_1f2c,
        )?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void TagFieldSetStringEscaped(self, tag, value, text)` (0x259288): when
/// `text` starts with a single quote, copy the rest through a stack buffer;
/// set the tag field.
pub fn tagfieldsetstringescaped(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x140);
        c.g[V0][0] = 0x27;
        frame.save(c, 16, 0x130);
        frame.save(c, 17, 0x120);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x110);
        c.g[S1][0] = c.g[A1][0];
        c.sd(p(sp(c), 0x100), c.g[RA][0]);
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lb(c);
        let quoted = c.g[V1][0] != c.g[V0][0];
        c.g[S2][0] = c.g[A2][0];
        if !quoted {
            c.cycles += 12;
            c.g[V0][0] = 0x60;
            c.g[A1][0] = c.g[A3][0];
            c.g[A0][0] = c.g[SP][0] | 1;
            c.sb(c.g[SP][0] as u32, c.g[V0][0]);
            call_ptr(c, callee::STR_COPY, 0x0025_92c8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0025_92dc)?;
            frame.restore(c, 16, 0x130);
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0025_92f4)?;
            frame.restore(c, 16, 0x130);
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x120);
        frame.restore(c, 18, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_Create()` (0x260f80): allocate the buddy manager (0xf0
/// bytes), initialise its fields and install it at `gp+0x238`.
pub fn buddyman_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let _frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0x480000;
        c.sd(p(sp(c), 0x20), c.g[RA][0]);
        c.g[A0][0] = 0xf0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe48));
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0;
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0026_0fa0)?;
        c.g[A1][0] = lw_at(c, gw(c, 0xda0));
        c.g[V1][0] = 0x10000;
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] |= 0x5180;
        c.sw(p(c.g[V0][0] as u32, 0x40), c.g[A0][0]);
        c.sw(p(c.g[V0][0] as u32, 0xa4), c.g[V1][0]);
        c.sw(p(c.g[V0][0] as u32, 0xec), c.g[A1][0]);
        c.sw(gw(c, 0x238), c.g[V0][0]);
        c.g[RA][0] = c.ld(p(sp(c), 0x20));
        c.sw(p(c.g[V0][0] as u32, 0xac), 0);
        c.sw(p(c.g[V0][0] as u32, 0xb0), 0);
        c.sw(p(c.g[V0][0] as u32, 0xb4), 0);
        c.sw(p(c.g[V0][0] as u32, 0xbc), 0);
        c.sw(p(c.g[V0][0] as u32, 0xc0), 0);
        c.sw(p(c.g[V0][0] as u32, 0xc4), 0);
        c.sw(p(c.g[V0][0] as u32, 0xc8), 0);
        c.sw(p(c.g[V0][0] as u32, 0xcc), 0);
        c.sw(p(c.g[V0][0] as u32, 0xd0), 0);
        c.sw(p(c.g[V0][0] as u32, 0xd8), 0);
        c.sw(p(c.g[V0][0] as u32, 0xdc), 0);
        c.sw(p(c.g[V0][0] as u32, 0xe0), 0);
        c.sw(p(c.g[V0][0] as u32, 0xe4), 0);
        c.sw(p(c.g[V0][0] as u32, 0xe8), 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `void SSXAudioSystem_StopAllObjectLoops(self)` (0x2980b0): stop the
/// looping handle of every active object slot (`0x30` bytes each, 0x5a0
/// bytes of slots).
pub fn ssxaudiosystem_stop_all_object_loops(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        frame.save(c, 16, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 18, 0x20);
        c.g[S0][0] = c.g[S1][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5a0));
        frame.save_ra(c, 0);
        c.g[S3][0] = 1;
        c.g[V0][0] = lw_at(c, p(r32(c, S0), 0x5a00));
        loop {
            c.cycles += 2;
            if c.g[V0][0] != c.g[S3][0] {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
            } else {
                c.cycles += 9;
                c.g[V0][0] = lw_at(c, p(r32(c, S1), 0x118));
                c.g[A2][0] = 1;
                c.g[A1][0] = lw_at(c, p(r32(c, S0), 0x5a24));
                c.g[A0][0] = lw_at(c, c.g[V0][0] as u32);
                c.mtc1(0, 12);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                callee::BANK_MONITOR_STOP_HANDLE.call(c, 0x0029_8104)?;
                c.sw(p(r32(c, S0), 0x5a00), 0);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = lw_at(c, p(r32(c, S0), 0x5a00));
            if event_due(c) {
                deliver_event_here(c, 0x0029_80e0);
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

/// `void SSXAudioSystem_PlayHitBoostSound(self, rider)` (0x298d00): when the
/// rider has a boost sound, add its timer to the bank monitor.
pub fn ssxaudiosystem_play_hit_boost_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, p(abi::r32(c, A1), 0x874));
        let none = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[V0][0] = lw_at(c, p(abi::r32(c, A1), 0x87c));
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        let silent = c.g[V1][0] == 0;
        c.g[S1][0] = 0x4a0000;
        if !silent {
            c.cycles += 17;
            c.g[V0][0] = lw_at(c, p(abi::r32(c, A0), 0x118));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3678));
            c.g[A1][0] = 9;
            c.g[S0][0] = lw_at(c, c.g[V0][0] as u32);
            c.g[A2][0] = 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
            call_ptr(
                c,
                callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR.addr,
                0x0029_8d48,
            )?;
            c.ldl(6, p(c.g[S1][0] as u32, 7));
            c.ldr(6, c.g[S1][0] as u32);
            c.g[A0][0] = c.g[S0][0];
            c.g[A3][0] = c.g[V0][0];
            c.g[A1][0] = 0xfa;
            c.g[T0][0] = 0;
            c.g[T1][0] = 0x72;
            c.g[T2][0] = 0x7f;
            c.g[T3][0] = 0;
            callee::BANK_MONITOR_ADD_TIMER.call(c, 0x0029_8d70)?;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_DJSay20E6(self)` (0x2a27d8): when DJ speech is on, set
/// the speech event context and play the DJ line through the speech system.
pub fn ssxaudiosystem_djsay20e6(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON.call(c, 0x002a_27ec)?;
        let off = c.g[V0][0] == 0;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
        if !off {
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20e6;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT.call(c, 0x002a_2814)?;
            if c.g[V0][0] == 0 {
                frame.restore(c, 16, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 12;
            c.g[A0][0] = c.g[S0][0];
            callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK.call(c, 0x002a_2824)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 1;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20e6;
            call_ptr(c, callee::SPCH_MAKE_ID, 0x002a_2838)?;
            c.g[V1][0] = lw_at(c, gw(c, 0x173c));
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, c.g[V1][0] as u32, 0x002a_284c)?;
        }
        c.cycles += 1;
        frame.restore(c, 16, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
/// `void ScriptEngine_v04(engine, obj, process)` (0x279e50): dispatch a
/// script object camera request by the object's opcode.
pub fn scriptengine_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        callee::SCRIPT_ENGINE_GET_SCRIPT_OBJ.call(c, 0x0027_9e74)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = lw_at(c, c.g[A0][0] as u32);
        c.g[V0][0] = 1;
        let not_one = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = 2;
        if !not_one {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            callee::SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS.call(c, 0x0027_9e90)?;
            frame.restore(c, 16, 0x30);
            c.cycles += 5;
            frame.restore(c, 17, 0x20);
            frame.restore(c, 18, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        let is_two = c.g[V1][0] == c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        if !is_two {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S0][0];
            callee::SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS.call(c, 0x0027_9eac)?;
            frame.restore(c, 16, 0x30);
            c.cycles += 5;
            frame.restore(c, 17, 0x20);
            frame.restore(c, 18, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A1][0] = c.g[S0][0];
        callee::SCRIPT_OBJ_CAM_HAS_BINDING.call(c, 0x0027_9ebc)?;
        frame.restore(c, 16, 0x30);
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SpeechSystem_Shutdown(self)` (0x2b0da0): destroy both speech
/// instances, shut the speech system down and free its buffer.
pub fn speechsystem_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = 0;
        frame.save(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        loop {
            c.cycles += 6;
            c.g[V0][0] = lw_at(c, p(r32(c, S2), 4));
            c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = lw_at(c, c.g[V0][0] as u32);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = 3;
                callee::SPEECH_INSTANCE_DTOR.call(c, 0x002b_0de0)?;
            }
            c.g[V0][0] = lw_at(c, p(r32(c, S2), 4));
            c.cycles += 5;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V1][0] = ((c.g[S1][0] as i64) < 2) as u64;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let more = c.g[V1][0] != 0;
            c.sw(c.g[V0][0] as u32, 0);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002b_0dc0);
            }
        }
        c.cycles += 11;
        call_ptr(c, callee::SPCH_SHUTDOWN, 0x002b_0e00)?;
        c.g[A0][0] = lw_at(c, p(r32(c, S2), 8));
        call_ptr(c, callee::MEM_FREE, 0x002b_0e08)?;
        c.sw(p(r32(c, S2), 8), 0);
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_HandleInput(page)` (0x2cacc0): let the selected item
/// handle its input, then move the selection for the "next"/"previous" codes
/// (3 and 4); returns 0 for the unhandled case.
pub fn menusyspage_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_PAGE_VALIDATE_SELECTION.call(c, 0x002c_acd4)?;
        let base = r32(c, S0);
        c.g[V0][0] = lw_at(c, p(base, 4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = lw_at(c, p(c.g[V0][0] as u32, 0xc));
        c.g[V1][0] = lw_at(c, p(c.g[A1][0] as u32, 0x10));
        let func = vslot(c, c.g[V1][0], 0x10, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, 0x002c_acf8)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 3;
        let next = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 4;
        let mut handled = true;
        if next {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::MENU_SYS_PAGE_SELECT_NEXT.call(c, 0x002c_ad20)?;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            let prev = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = c.g[V1][0];
            if prev {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                callee::MENU_SYS_PAGE_SELECT_PREV.call(c, 0x002c_ad30)?;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 2;
                frame.restore(c, 16, 0x10);
                handled = false;
            }
        }
        if handled {
            c.cycles += 1;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_InsertItem(page, item, index)` (0x2cbcc8): insert the
/// item into the page's item array at `index`, shifting the rest up, and
/// keep the item count and the selected index consistent.
pub fn menusyspage_insert_item(c: &mut Cpu) {
    c.cycles += 2;
    let item = c.g[A0][0];
    let page = c.g[A1][0];
    let mut index = c.g[A2][0];
    let given = (index as i64) >= 0;
    c.sw(page as u32, item);
    if !given {
        c.cycles += 1;
        index = lw_at(c, item as u32);
        c.g[A2][0] = index;
    }
    c.g[A2][0] = index;
    c.cycles += 5;
    let count = lw_at(c, item as u32);
    c.g[T0][0] = count;
    c.g[T1][0] = sx((item as u32).wrapping_add(0xc));
    c.g[V0][0] = ((index as i64) < (count as i64)) as u64;
    let fits = c.g[V0][0] != 0;
    c.g[T2][0] = sx((index as u32) << 2);
    if fits {
        c.cycles += 2;
        c.g[V0][0] = sx((count as u32) << 2);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
        loop {
            c.cycles += 7;
            c.g[V0][0] = lw_at(c, (c.g[A3][0] as u32).wrapping_sub(4));
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(1));
            c.g[V1][0] = ((index as i64) < (c.g[T0][0] as i64)) as u64;
            c.sw(c.g[A3][0] as u32, c.g[V0][0]);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(4));
            if c.g[V1][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_bcf0);
            }
        }
    }
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
    c.g[V1][0] = lw_at(c, p(item as u32, 0x124));
    if c.g[V1][0] == 0 {
        c.g[V0][0] = lw_at(c, item as u32);
    } else {
        c.cycles += 4;
        c.g[V1][0] = lw_at(c, p(item as u32, 4));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (index as i64)) as u64;
        let after = c.g[V0][0] != 0;
        c.g[V0][0] = lw_at(c, item as u32);
        if !after {
            c.cycles += 2;
            let empty = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if !empty {
                c.cycles += 1;
                c.sw(p(item as u32, 4), c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = lw_at(c, item as u32);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.sw(item as u32, c.g[V0][0]);
    abi::ret(c);
}

/// `void MenuSysActionItem_HandleInput(item)` (0x2cd008): the action item's
/// input: run the item input, then the action when it was triggered.
pub fn menusysactionitem_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_HANDLE_INPUT.call(c, 0x002c_d020)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::MENU_SYS_ITEM_IS_DISABLED.call(c, 0x002c_d02c)?;
        let disabled = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S1][0];
        let mut skip_one = false;
        if !disabled {
            c.cycles += 2;
            if c.g[S1][0] != 0 {
                frame.restore(c, 16, 0x20);
                skip_one = true;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                callee::MENU_SYS_ITEM_GET_INPUT_CODE.call(c, 0x002c_d044)?;
                c.g[V1][0] = 6;
                let other = c.g[V0][0] != c.g[V1][0];
                c.g[V0][0] = c.g[S1][0];
                if other {
                } else {
                    c.cycles += 10;
                    c.g[A3][0] = lw_at(c, c.g[S0][0] as u32);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = lw_at(c, p(c.g[A1][0] as u32, 0x1c));
                    c.g[V1][0] = lw_at(c, p(c.g[A3][0] as u32, 0x12c));
                    let func = vslot(c, c.g[V1][0], 0x18, V0);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, func, 0x002c_d070)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V0][0] = c.g[S1][0];
                }
            }
        }
        if !skip_one {
            c.cycles += 1;
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cKeyboardPopup_Update(self)` (0x1cb208): update the popup's base
/// state and its key-list widget.
pub fn ckeyboardpopup_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e510, 0x001c_b21c)?;
        c.g[V0][0] = lw_at(c, gw(c, -0x204));
        let v0;
        if c.g[V0][0] == 0 {
            v0 = lw_at(c, p(r32(c, S0), 0x440));
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw_at(c, p(c.g[V0][0] as u32, 0x64));
            if c.g[V0][0] != 0 {
                v0 = lw_at(c, p(r32(c, S0), 0x440));
            } else {
                c.cycles += 4;
                c.g[V0][0] = lw_at(c, gw(c, -0x848));
                c.g[V1][0] = lw_at(c, p(c.g[V0][0] as u32, 0x84));
                let no_race = c.g[V1][0] == 0;
                c.g[A1][0] = 1;
                if !no_race {
                    c.cycles += 3;
                    c.g[A0][0] = lw_at(c, p(r32(c, S0), 0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f190, 0x001c_b250)?;
                }
                c.cycles += 1;
                v0 = lw_at(c, p(r32(c, S0), 0x440));
            }
        }
        c.g[V0][0] = v0;
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = lw_at(c, p(r32(c, S0), 0x20));
            let none = c.g[A1][0] == 0;
            frame.restore(c, 16, 0x10);
            if !none {
                c.cycles += 6;
                c.g[V0][0] = lw_at(c, p(c.g[A1][0] as u32, 8));
                let func = vslot(c, c.g[V0][0], 0x60, V1);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x001c_b27c)?;
                frame.restore(c, 16, 0x10);
            }
        } else {
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void World_ResetLights()` (0x3ad1a8): clear every rider's nearby-light
/// list through the race manager.
pub fn world_reset_lights(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = lw_at(c, gw(c, -0x848));
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = lw_at(c, p(c.g[V0][0] as u32, 0x84));
        c.g[A0][0] = lw_at(c, p(c.g[V1][0] as u32, 0x90));
        call_ptr(c, 0x002f_5998, 0x003a_d1cc)?;
        c.g[V0][0] = lw_at(c, gw(c, -0x848));
        c.g[V1][0] = lw_at(c, p(c.g[V0][0] as u32, 0x84));
        c.g[S2][0] = lw_at(c, p(c.g[V1][0] as u32, 0xc));
        let none = c.g[S2][0] == 0;
        frame.restore(c, 16, 0x30);
        if !none {
            c.cycles += 3;
            c.g[V0][0] = lw_at(c, p(c.g[S2][0] as u32, 0x78));
            let no_riders = (c.g[V0][0] as i64) <= 0;
            c.g[S1][0] = 0;
            if !no_riders {
                c.cycles += 3;
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
                c.g[A0][0] = lw_at(c, c.g[S0][0] as u32);
                loop {
                    c.cycles += 7;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                    callee::RIDER_CLEAR_NEARBY_LIGHTS.call(c, 0x003a_d204)?;
                    c.g[V0][0] = lw_at(c, p(r32(c, S2), 0x78));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.cycles += 1;
                        frame.restore(c, 16, 0x30);
                        break;
                    }
                    c.g[A0][0] = lw_at(c, c.g[S0][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_d1f8);
                    }
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One of the three "copy my `+0x10/+0x14/+0x18` vector into the state" calls
/// of the game-state loaders: `vt[0x10](self + 0, self + dst, 4)` with the
/// vtable at `src->+0x10`.
fn load_state_vector(c: &mut Cpu, vt: u64, data: u32, dst: u32, ret: u32) -> Flow {
    let func = vslot(c, vt, 0x10, V0);
    c.g[A1][0] = sx(dst);
    c.g[A2][0] = 4;
    c.g[A0][0] = sx(data.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, func, ret)
}

/// `void GameStateReset_LoadState(self, data)` (0x235a18): load the state,
/// then copy the three transform vectors from the data object.
pub fn gamestatereset_load_state(c: &mut Cpu) {
    game_state_load_state(c, 0x0023_5a34, [0x0023_5a50, 0x0023_5a6c, 0x0023_5a88]);
}

/// `void GameStateRestartRace_LoadState(self, data)` (0x235fd0): the restart
/// twin of [`gamestatereset_load_state`].
pub fn gamestaterestartrace_load_state(c: &mut Cpu) {
    game_state_load_state(c, 0x0023_5fec, [0x0023_6008, 0x0023_6024, 0x0023_6040]);
}

fn game_state_load_state(c: &mut Cpu, load_ret: u32, vec_rets: [u32; 3]) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::GAME_STATE_LOAD_STATE.call(c, load_ret)?;
        for (i, ret) in vec_rets.into_iter().enumerate() {
            c.g[V1][0] = lw_at(c, c.g[S0][0] as u32);
            load_state_vector(
                c,
                c.g[V1][0],
                r32(c, S0),
                p(r32(c, S1), 0x10 + 4 * i as u32),
                ret,
            )?;
        }
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStatePreRaceMenu_ResumeRace()` (0x236cd8): stop the script
/// list, restart the race and reset the race sounds.
pub fn gamestatepreracemenu_resume_race(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = lw_at(c, gw(c, -0x84c));
        c.sd(p(sp(c), 0), c.g[RA][0]);
        c.g[A1][0] = 0;
        call_ptr(c, callee::SCRIPT_ENGINE_LIST_STOP.addr, 0x0023_6cec)?;
        callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x0023_6cf4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x0023_6d00)?;
        c.g[V1][0] = 0x530000;
        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x5c10)).lb(c);
        c.g[V0][0] ^= 4;
        let no_restart = c.g[V0][0] == 0;
        c.g[V1][0] = lw_at(c, gw(c, -0x488));
        if !no_restart {
            c.cycles += 11;
            c.g[A1][0] = 3;
            c.g[A2][0] = lw_at(c, p(c.g[V1][0] as u32, 0xc));
            c.g[V0][0] = lw_at(c, p(c.g[A2][0] as u32, 0xcc));
            let func = vslot(c, c.g[V0][0], 0x28, V1);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, func, 0x0023_6d30)?;
            c.g[A0][0] = lw_at(c, gw(c, -0x488));
            c.g[V1][0] = 1;
            c.g[V0][0] = lw_at(c, p(c.g[A0][0] as u32, 0xc));
            c.sw(p(c.g[V0][0] as u32, 0x14), c.g[V1][0]);
        }
        c.cycles += 7;
        callee::SSX_AUDIO_SYSTEM_GET_INSTANCE.call(c, 0x0023_6d48)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS.call(c, 0x0023_6d50)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void StreamBuffer_dtor(self, flags)` (0x2a9df0): close the stream, free
/// its buffer and the buffer object, then the object itself when
/// `flags & 1`.
pub fn streambuffer_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, c.g[S0][0] as u32);
        let no_stream = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_stream {
            c.cycles += 5;
            c.g[A0][0] = lw_at(c, p(r32(c, S0), 8));
            call_ptr(c, callee::SNDSTRM_CLOSE.addr, 0x002a_9e18)?;
            c.g[V1][0] = lw_at(c, p(r32(c, S0), 4));
            if c.g[V1][0] != 0 {
                c.g[A0][0] = lw_at(c, p(r32(c, S0), 0x74));
            } else {
                c.cycles += 3;
                c.g[A0][0] = lw_at(c, c.g[S0][0] as u32);
                if c.g[A0][0] != 0 {
                    c.cycles += 2;
                    callee::EA_MEM_FREE.call(c, 0x002a_9e38)?;
                    c.cycles += 1;
                }
                c.g[A0][0] = lw_at(c, p(r32(c, S0), 0x74));
            }
        } else {
            c.cycles += 1;
            c.g[A0][0] = lw_at(c, p(r32(c, S0), 0x74));
        }
        c.cycles += 2;
        let none = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !none {
            c.cycles += 3;
            call_ptr(c, callee::MEM_FREE, 0x002a_9e4c)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::BUILTIN_DELETE.addr, 0x002a_9e60)?;
        }
        frame.restore(c, 16, 0x20);
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptObj_Pause(self)` (0x283320): pause every running sequence of
/// the object's binding.
pub fn scriptobj_pause(c: &mut Cpu) {
    script_obj_pause_resume(
        c,
        ScriptLoop {
            get_count_ret: 0x0028_3350,
            each: callee::SCRIPT_BINDING_PAUSE_SEQ,
            each_ret: 0x0028_3380,
            seq_ret: 0x0028_3378,
            head: 0x0028_3360,
        },
    );
}

/// `void ScriptObj_Resume(self)` (0x2833a8): resume every paused sequence.
pub fn scriptobj_resume(c: &mut Cpu) {
    script_obj_pause_resume(
        c,
        ScriptLoop {
            get_count_ret: 0x0028_33d8,
            each: callee::SCRIPT_BINDING_RESUME_SEQ,
            each_ret: 0x0028_3408,
            seq_ret: 0x0028_3400,
            head: 0x0028_33e8,
        },
    );
}

/// The addresses that differ between the pause and resume variants.
struct ScriptLoop {
    get_count_ret: u32,
    each: Callee,
    each_ret: u32,
    seq_ret: u32,
    head: u32,
}

fn script_obj_pause_resume(c: &mut Cpu, l: ScriptLoop) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = lw_at(c, p(r32(c, S1), 0xc));
        let func = vslot(c, c.g[V1][0], 0x30, V0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, l.get_count_ret)?;
        c.g[S2][0] = c.g[V0][0];
        if (c.g[S2][0] as i64) <= 0 {
            frame.restore(c, 16, 0x30);
        } else {
            c.cycles += 1;
            c.g[V1][0] = lw_at(c, p(r32(c, S1), 0xc));
            loop {
                c.cycles += 11;
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                let func = vslot(c, c.g[V1][0], 0x38, V0);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, l.seq_ret)?;
                c.g[A0][0] = c.g[V0][0];
                l.each.call(c, l.each_ret)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V1][0] = lw_at(c, p(r32(c, S1), 0xc));
                if event_due(c) {
                    deliver_event_here(c, l.head);
                }
            }
            frame.restore(c, 16, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `vd_CreateTextureSink(sink, w, h, fmt)` (0x254368): allocate a
/// `VD_TextureSink` through the render-device hook and construct it.
pub fn vd_create_texture_sink(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x510000;
        frame.save(c, 16, 0x40);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        c.g[A0][0] = 0x4a0000;
        c.g[A1][0] = 0x54;
        c.g[T0][0] = lw_at(c, p(c.g[V0][0] as u32, 8));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ea0));
        c.g[V0][0] = lw_at(c, (c.g[V1][0] as u32).wrapping_sub(0x6bd0));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, c.g[V0][0] as u32, 0x0025_43b8)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S2][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::VD_TEXTURE_SINK_CTOR.addr, 0x0025_43d0)?;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BENewPlayerInt_SaveSlotDefaults()` (0x145a98): copy the twelve
/// character-record slots into the save slots, preserving the "new" flag.
pub fn benewplayerint_save_slot_defaults(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x530000;
        frame.save(c, 16, 0x40);
        c.g[V1][0] = 0x530000;
        frame.save(c, 17, 0x30);
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4fe0));
        frame.save(c, 18, 0x20);
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5b20));
        frame.save(c, 19, 0x10);
        c.g[S2][0] = 0xffff_ffff_ffff_fffd;
        frame.save_ra(c, 0);
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa8));
        loop {
            c.cycles += 14;
            c.g[V0][0] = lw_at(c, p(r32(c, S1), 0x10));
            c.g[A0][0] = c.g[S1][0];
            c.g[V1][0] = lw_at(c, p(r32(c, S0), 0x10));
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] &= c.g[S2][0];
            c.g[A2][0] = 0xc;
            c.g[V1][0] &= 2;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] |= c.g[V1][0];
            c.sw(p(r32(c, S1), 0x10), c.g[V0][0]);
            call_ptr(c, callee::STR_N_COPY, 0x0014_5af4)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S3][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1c));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_5ac8);
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

/// `void BENewPlayerInt_SetCharRecordField27C(player, value)` (0x146e10):
/// store `value` in the rider's character record at `+0x27c`.
pub fn benewplayerint_set_char_record_field27c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 18, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::BE_GET_RIDER_PROFILE.addr, 0x0014_6e34)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, callee::BE_NEW_PLAYER_INT_GET_SLOT.addr, 0x0014_6e40)?;
        c.g[V1][0] = 0x1c;
        c.g[A1][0] = 0x530000;
        mul32(c, V0, V0, V1);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4fe0));
        c.g[V1][0] = 0x9b50;
        c.g[A2][0] = 0xf88;
        mul32(c, S1, S1, V1);
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ca8));
        frame.restore(c, 16, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.restore_ra(c, 0);
        c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x11)).lb(c);
        mul32(c, V1, V1, A2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        frame_restore_at(c, 17, 0x20);
        c.sw(p(c.g[V1][0] as u32, 0x27c), c.g[S2][0]);
        frame_restore_at(c, 18, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `int CollSphereTree_CollideSphereTree(self, other, hit)` (0x3308c8): test
/// the tree against another tree; on a hit normalise the contact direction
/// into `hit->dir` and store its length, returning 1.
pub fn collspheretree_collide_sphere_tree(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A3][0] = c.g[S0][0];
        c.g[A0][0] = lw_at(c, p(abi::r32(c, A0), 0x60));
        call_ptr(c, callee::SPHERE_TREE_COLLIDE_SPHERE_TREE, 0x0033_08e8)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 19;
            collide_normalise(c, c.g[S0][0] as u32);
            c.g[V0][0] = 1;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int CollSphereTree_CollideAABB(self, aabb, hit)` (0x330950): the AABB
/// twin of [`collspheretree_collide_sphere_tree`].
pub fn collspheretree_collide_aabb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A3][0];
        c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[T0][0] = c.g[S0][0];
        c.g[A0][0] = lw_at(c, p(abi::r32(c, A0), 0x60));
        call_ptr(c, callee::SPHERE_TREE_COLLIDE_AABB, 0x0033_0970)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 19;
            collide_normalise(c, c.g[S0][0] as u32);
            c.g[V0][0] = 1;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The VU0 tail shared by the two `CollSphereTree` collisions: the length of
/// `hit->dir` (`hit+0x10`) into `hit+0x40` and the normalised direction back
/// into `hit+0x10`, with the original's COP2 macro sequence.
fn collide_normalise(c: &mut Cpu, hit: u32) {
    c.lqc2(3, p(hit, 0x10));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(2, 22);
    c.mtc1(2, 0);
    c.swc1(0, p(hit, 0x40));
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, p(hit, 0x10));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 4, Src::Q);
    c.sqc2(5, p(hit, 0x10));
}

/// `void cWorldSound_RemoveEmitter(self, id)` (0x2b77c0): clear the id from
/// both emitter tables (0x120 bytes each).
pub fn cworldsound_remove_emitter(c: &mut Cpu) {
    c.cycles += 6;
    c.g[T4][0] = c.g[A0][0];
    c.g[T2][0] = 0x120;
    c.g[A0][0] = 0;
    c.g[T1][0] = 0x120;
    mul32(c, V0, A0, T2);
    loop {
        c.cycles += 5;
        c.g[A2][0] = 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
        c.g[V0][0] = lw_at(c, c.g[V1][0] as u32);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        if !empty {
            c.cycles += 7;
            mul32(c, V0, A0, T1);
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
            c.g[V0][0] = c.g[V1][0];
            c.g[A3][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            loop {
                c.cycles += 4;
                c.g[V0][0] = lw_at(c, c.g[V1][0] as u32);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                let other = c.g[V0][0] != c.g[A1][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
                if !other {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, 0);
                }
                c.cycles += 4;
                c.g[V0][0] = lw_at(c, c.g[A3][0] as u32);
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_7808);
                }
            }
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[T0][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < 2) as u64;
        let more = c.g[V0][0] != 0;
        mul32(c, V0, A0, T2);
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002b_77d8);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::*;
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 100;
    /// `UIListNode_IsSentinel`: returns 0 for a node whose `+4` and `+0` are
    /// not the node itself. Used as a "returns 0" stub behind a vtable slot.
    const RET_ZERO: u32 = 0x0039_7718;
    /// The BE library table and index global.
    const BE_LIBRARY: u32 = 0x0053_08a8;
    const BE_LIBRARY_INDEX_GP: u32 = 0x1efc;

    /// Scrambled machine with the two singletons every game path expects: a
    /// non-null BELibrary (so it never builds itself) and a dummy world.
    fn base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0xa000);
        c.bus.write32(GAME_GP - 0x848, DATA + 0x9000);
        c.bus.write32(DATA + 0x9000 + 0x84, DATA + 0x9400);
        r.next_u32();
        r
    }

    /// A `{ short delta; void* fn; }` vtable slot with a chosen delta.
    fn slot(c: &mut Cpu, vt: u32, off: u32, delta: u16, func: u32) {
        c.bus.write16(vt + off, delta);
        c.bus.write32(vt + off + 4, func);
    }

    /// `str_hash(text)` computed on a scratch copy (the hasher overwrites the
    /// string it hashes with the result).
    fn hash_text(c: &mut Cpu, bytes: &[u8]) -> u32 {
        let tmp = DATA + 0x3f00;
        for (i, b) in bytes.iter().enumerate() {
            c.bus.write8(tmp + i as u32, *b);
        }
        c.bus.write8(tmp + bytes.len() as u32, 0);
        c.g[A0][0] = sx(tmp);
        c.g[RA][0] = sx(0x0012_3450);
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(c);
        c.g[V0][0] as u32
    }

    // -----------------------------------------------------------------------
    // Rewards, bolts.

    #[test]
    fn berewardint_is_rival_reward_pending_for_matches() {
        check(
            ee::s01::f_00157b70__generated,
            berewardint_is_rival_reward_pending_for,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x7b70, 0);
                let a = (r.next_u32() % 4) as u64;
                let b = (r.next_u32() % 4) as u64;
                c.g[A1][0] = a;
                c.g[A2][0] = b;
                c.g[A3][0] = (seed % 4) as u64;
                let off = (b as u32).wrapping_mul(0xf88) + (a as u32).wrapping_mul(0x9b50);
                let addr = 0x004a_6ca8u32.wrapping_add(off).wrapping_add(0x278);
                c.bus
                    .write64(addr, (r.next_u32() as u64) << 12 | (seed as u64));
            },
        );
    }

    #[test]
    fn berewardint_has_points_tier_matches() {
        covered(
            ee::s01::f_00158af0__generated,
            berewardint_has_points_tier,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x8af0, 3);
                let index = (seed % 4) as u32;
                c.bus.write32(GAME_GP - BE_LIBRARY_INDEX_GP, index);
                let lib = ptr(&mut r);
                c.bus.write32(BE_LIBRARY + 4 * index, lib);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(lib + 4, vt);
                let a = ((seed / 4) % 3) as u64;
                let b = ((seed / 12) % 3) as u64;
                let medal = ((seed / 36) % 4) as u32;
                c.g[A1][0] = a;
                c.g[A2][0] = b;
                c.g[A3][0] = medal as u64;
                let off = (b as u32).wrapping_mul(0xf88) + (a as u32).wrapping_mul(0x9b50);
                c.bus.write32(0x004a_6ca8 + off + 0xac8, r.next_u32());
                c.bus.write32(
                    0x0045_b048 + 4 * medal,
                    if seed % 2 == 0 { 0 } else { 0xffff_ffff },
                );
            },
        );
    }

    #[test]
    fn bolts_find_bolt_index_matches() {
        check(
            ee::s01::f_0014d448__generated,
            bolts_find_bolt_index,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x4d448, 0);
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
                    let base = DATA + 0x1000;
                    c.bus.write32(self_ + 4, base);
                    for i in 0..4u32 {
                        c.bus.write16(base + i * 0x38 + 4, ((i + seed) % 8) as u16);
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
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x4d908, 0);
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

    // -----------------------------------------------------------------------
    // Script lists.

    #[test]
    fn scriptlist_is_playing_script_matches() {
        check(
            ee::s07::f_002771c8__generated,
            scriptlist_is_playing_script,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x71c8, 0);
                let engine = DATA + 0x8800;
                let s0 = DATA + 0x100;
                c.g[A0][0] = sx(s0);
                c.g[A1][0] = (seed % 3) as u64;
                c.bus.write32(GAME_GP - 0x84c, engine);
                c.bus.write32(s0 + 0xa4, if seed % 4 == 0 { 0 } else { 3 });
                c.bus.write32(s0 + 0xc + 0x8c, 0);
                c.bus.write32(s0 + 0xc, if seed % 2 == 0 { 5 } else { 40 });
                c.bus.write32(s0 + 0xc + 0xc, (seed / 4 % 3) as u32);
                for id in 0..3u32 {
                    let p = DATA + 0x8900 + 0x40 * id;
                    c.bus.write32(engine + 0x2c0 + 0x24 * id + 0x20, p);
                    c.bus.write32(p, id * 7 + 1);
                }
            },
        );
    }

    // -----------------------------------------------------------------------
    // UI screens and states.

    #[test]
    fn uiscreen_cmd_send_event_matches() {
        covered(
            ee::s13::f_0039cda0__generated,
            uiscreen_cmd_send_event,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0xcda0, 2);
                let s0 = DATA + 0x100;
                let s1 = DATA + 0x180;
                let ctrl = DATA + 0x200;
                c.g[A0][0] = sx(s0);
                c.g[A2][0] = sx(s1);
                c.bus.write32(s1 + 4, 0x99);
                c.bus.write32(s0 + 0xd0, ctrl);
                c.bus.write32(ctrl, 0);
                let vt1 = DATA + 0x400;
                slot(c, vt1, 0x80, 0, if seed % 2 == 0 { FAKE } else { RET_ZERO });
                c.bus.write32(ctrl + 8, vt1);
                let holder = DATA + 0x300;
                c.bus.write32(ctrl + 0x10, holder);
                let obj2 = DATA + 0x340;
                c.bus.write32(holder, if seed % 3 == 0 { 0 } else { obj2 });
                let vt2 = DATA + 0x440;
                slot(c, vt2, 0x18, 0, FAKE);
                c.bus.write32(obj2 + 4, vt2);
            },
        );
    }

    #[test]
    fn worldmemman_create_section_allocator_matches() {
        check(
            ee::s13::f_003a7b18__generated,
            worldmemman_create_section_allocator,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x7b18, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = (seed % 6) as u64;
                c.g[A2][0] = ((seed / 6) % 3) as u64;
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
    fn gamestatepreraceeload_check_intro_script_matches() {
        check(
            ee::s06::f_00234910__generated,
            gamestatepreraceeload_check_intro_script,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x4910, 0);
                let engine = DATA + 0x800;
                c.bus.write32(GAME_GP - 0x84c, engine);
                c.bus.write32(engine + 0x550, (seed % 3) as u32);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn railman_on_section_unload_matches() {
        check(
            ee::s11::f_00358700__generated,
            railman_on_section_unload,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x8700, 0);
                let m = DATA + 0x800;
                c.g[A0][0] = (seed % 3) as u64;
                c.bus
                    .write32(GAME_GP + 0xf50, if seed % 7 == 0 { 0 } else { m });
                for i in 0..0x80u32 {
                    let w = match (i + seed) % 8 {
                        0 => 0xffff_ffff,
                        1 => seed % 3,
                        _ => 0x100 + i,
                    };
                    c.bus.write32(m + 0x18 + 4 * i, w);
                }
            },
        );
    }

    #[test]
    fn uipair_v04_matches() {
        check(ee::s13::f_0039bad0__generated, uipair_v04, SEEDS, &|c, seed| {
            base(c, seed, 0xbad0, 0);
            let s = DATA + 0x100;
            c.g[A0][0] = sx(s);
            c.g[A1][0] = (seed % 3) as u64;
            c.bus.write32(s + 0x14, r_u32(seed, 1));
            let a = DATA + 0x200;
            let b = DATA + 0x240;
            c.bus.write32(s + 0x78, a);
            c.bus.write32(s + 0x7c, if seed % 3 == 0 { 0 } else { b });
            let vta = DATA + 0x400;
            slot(c, vta, 0x20, 0, FAKE);
            c.bus.write32(a + 8, vta);
            let vtb = DATA + 0x440;
            slot(c, vtb, 0x20, 0, FAKE + 0x10);
            c.bus.write32(b + 8, vtb);
        });
    }

    fn r_u32(seed: u32, salt: u32) -> u32 {
        let mut x = seed.wrapping_mul(0x9e37_79b9) ^ salt;
        x ^= x >> 13;
        x.wrapping_mul(0x85eb_ca6b)
    }

    #[test]
    fn ovscreenbcfail_on_widget_created_matches() {
        check(
            ee::s04::f_001f7770__generated,
            ovscreenbcfail_on_widget_created,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x7770, 0);
                // Short strings at the three name addresses the original hashes.
                for (addr, text) in [
                    (0x004a_23e0u32, b'A'),
                    (0x004a_23e8, b'B'),
                    (0x004a_23f8, b'C'),
                ] {
                    c.bus.write8(addr, text);
                    c.bus.write8(addr + 1, 0);
                }
                let h0 = hash_text(c, b"A");
                let h1 = hash_text(c, b"B");
                let h2 = hash_text(c, b"C");
                let widget = DATA + 0x100;
                c.g[A1][0] = sx(widget);
                let h = match seed % 4 {
                    0 => h0,
                    1 => h1,
                    2 => h2,
                    _ => 0xdead_beef,
                };
                c.bus.write32(widget + 0x38, h);
            },
        );
    }

    #[test]
    fn femodule_unload_anim_data_matches() {
        check(
            ee::s03::f_001a2bc0__generated,
            femodule_unload_anim_data,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x2bc0, 0);
                let map = DATA + 0x800;
                let banks = DATA + 0x900;
                c.bus.write32(GAME_GP + 0xd8c, map);
                c.bus.write32(GAME_GP + 0xd08, banks);
                let e = DATA + 0xa00;
                c.bus.write32(banks, if seed % 3 == 0 { 0 } else { e });
            },
        );
    }

        #[test]
    fn cfestatebraggingrights_ctor_matches() {
        check(
            ee::s02::f_00192d00__generated,
            cfestatebraggingrights_ctor,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x2d00, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn uistatemgr_close_all_popups_matches() {
        check(
            ee::s13::f_0039f698__generated,
            uistatemgr_close_all_popups,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0xf698, 0);
                let list = DATA + 0x800;
                let n1 = DATA + 0x840;
                let n2 = DATA + 0x880;
                c.g[A0][0] = sx(list);
                c.bus.write32(list + 4, if seed % 5 == 0 { 0 } else { n1 });
                c.bus.write32(n1 + 4, if seed % 2 == 0 { 0 } else { n2 });
                c.bus.write32(n1 + 0x1c, ((seed % 8) << 8) | 1);
                c.bus.write32(n2 + 4, n2);
                // Even: the walk stops after this node.
                c.bus.write32(n2 + 0x1c, (r_u32(seed, 7) << 1) & !1);
            },
        );
    }

    #[test]
    fn uiobject_dtor_matches() {
        covered(ee::s13::f_0039fc48__generated, uiobject_dtor, SEEDS, &|c, seed| {
            base(c, seed, 0xfc48, 1);
            let s = DATA + 0x100;
            let child = DATA + 0x200;
            c.g[A0][0] = sx(s);
            c.g[A1][0] = (seed & 1) as u64;
            c.bus
                .write32(s + 0xc, if seed % 3 == 0 { 0 } else { child });
            let vt = DATA + 0x300;
            slot(c, vt, 8, 0, FAKE);
            c.bus.write32(child + 0x10, vt);
        });
    }

    #[test]
    fn cfestateaudiooptions_shutdown_matches() {
        check(
            ee::s02::f_00196228__generated,
            cfestateaudiooptions_shutdown,
            SEEDS,
            &|c, seed| {
                shutdown_fixture(c, seed, 0x6228);
            },
        );
        check(
            ee::s02::f_00196e80__generated,
            cfestaterequestline_shutdown,
            SEEDS,
            &|c, seed| {
                shutdown_fixture(c, seed, 0x6e80);
            },
        );
    }

    fn shutdown_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        base(c, seed, salt, 0);
        let s1 = DATA + 0x100;
        let obj = DATA + 0x200;
        c.g[A0][0] = sx(s1);
        c.bus.write32(s1 + 0x10, obj);
        if seed % 4 == 1 {
            c.bus.write32(DATA + 0x9000 + 0x84, 0);
        }
        let node = DATA + 0x300;
        c.bus.write32(obj + 0x18 + 4, 0);
        let list2 = obj + 0x18 + 0x1c;
        if seed % 3 != 0 {
            c.bus.write32(list2 + 4, node);
            c.bus.write32(node, 0);
            c.bus.write32(node + 4, node + 8);
            c.bus.write32(node + 0xc, 0);
            let vt = DATA + 0x340;
            slot(c, vt, 0xc0, 0, FAKE);
            c.bus.write32(node + 8, vt);
        }
        c.bus.write32(s1 + 0x24 + 4, 0);
        c.bus.write32(0x004a_18a8, 0);
    }

    // -----------------------------------------------------------------------
    // Net/online front-end states.

    #[test]
    fn cfestatenetmessenger_clear_name_list_matches() {
        for (gen, hand) in [(
            ee::s03::f_001a5450__generated as fn(&mut Cpu),
            cfestatenetmessenger_clear_name_list as fn(&mut Cpu),
        )] {
            check(gen, hand, SEEDS, &|c, seed| {
                base(c, seed, 0xa5450, 0);
                let s = DATA + 0x100;
                c.g[A0][0] = sx(s);
                c.pc = 0x001a_5450;
                let lo = s + 0x600;
                let (head, tail) = match seed % 4 {
                    0 => (lo, lo),
                    1 => (lo, lo + 8),
                    2 => (lo, lo + 0xc),
                    _ => (lo, lo + 0x10),
                };
                c.bus.write32(s + 0x6e8, head);
                c.bus.write32(s + 0x6ec, tail);
                for k in 0..8u32 {
                    let at = (tail as i64 - 4 * (k as i64 + 1)) as u32;
                    c.bus.write32(at, 0);
                }
            });
        }
    }

    #[test]
    fn cfestateonlinecharacterselect_show_character_matches() {
        check(
            ee::s03::f_001b1b10__generated,
            cfestateonlinecharacterselect_show_character,
            24,
            &|c, seed| {
                base(c, seed, 0xb1b10, 0);
                let s = DATA + 0x100;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = ((seed / 3) % 2) as u64;
            },
        );
    }

    #[test]
    fn cfestatelobbymenu_prompt_room_password_matches() {
        check(
            ee::s03::f_001bdd60__generated,
            cfestatelobbymenu_prompt_room_password,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0xbdd60, 0);
                let s = DATA + 0x100;
                c.g[A0][0] = sx(s);
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (seed % 4) as u64
                };
                let popup = DATA + 0x200;
                c.bus.write32(s + 0x69c, popup);
                c.bus.write32(0x004a_1408, 0x1234);
                c.bus.write32(0x004a_50d0, 0x5678);
            },
        );
    }

    // -----------------------------------------------------------------------
    // Message center, interrupt table, tags.

    #[test]
    fn ovscreenmessagecenter_delete_message_matches() {
        check(
            ee::s04::f_001e5148__generated,
            ovscreenmessagecenter_delete_message,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0xe5148, 0);
                let s = DATA + 0x100;
                let mc = DATA + 0x800;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed % 6) as u64;
                c.bus.write32(s + 0x144, mc);
                c.bus.write8(mc + 0x95, (seed % 5) as u8);
                c.bus.write32(GAME_GP - 0x5f8, DATA + 0x900);
            },
        );
    }

    #[test]
    fn intctable_init_matches() {
        check(ee::s12::f_00361eb8__generated, intctable_init, SEEDS, &|c, seed| {
            base(c, seed, 0x61eb8, 0);
            c.g[A0][0] = sx(DATA + 0x100);
        });
    }

    #[test]
    fn tagfieldsetstringescaped_matches() {
        check(
            ee::s06::f_00259288__generated,
            tagfieldsetstringescaped,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x59288, 0);
                let s = DATA + 0x100;
                let text = DATA + 0x800;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = r_u32(seed, 3) as u64;
                c.g[A3][0] = sx(text);
                c.bus.write8(text, if seed % 2 == 0 { 0x27 } else { b'x' });
                for i in 1..16u32 {
                    c.bus
                        .write8(text + i, b'a' + (seed.wrapping_add(i) % 26) as u8);
                }
                c.bus.write8(text + 16, 0);
                c.bus.write32(0x004a_50d0, 0);
            },
        );
    }

    // -----------------------------------------------------------------------
    // Audio.

    #[test]
    fn buddyman_create_matches() {
        check(ee::s06::f_00260f80__generated, buddyman_create, SEEDS, &|c, seed| {
            base(c, seed, 0x60f80, 0);
            c.bus.write32(GAME_GP + 0xda0, seed);
        });
    }

    #[test]
    fn ssxaudiosystem_stop_all_object_loops_matches() {
        check(
            ee::s08::f_002980b0__generated,
            ssxaudiosystem_stop_all_object_loops,
            40,
            &|c, seed| {
                base(c, seed, 0x980b0, 0);
                let s = DATA + 0x100;
                let chans = DATA + 0x800;
                let gain = DATA + 0x840;
                c.g[A0][0] = sx(s);
                c.bus.write32(s + 0x118, chans);
                c.bus.write32(chans, gain);
                c.bus.write32(s + 0x5a24, seed);
                for i in 0..48u32 {
                    c.bus.write32(
                        s + 0x5a00 + 0x30 * i,
                        if (i + seed) % 4 == 0 { 1 } else { 0 },
                    );
                }
            },
        );
    }

    #[test]
    fn ssxaudiosystem_play_hit_boost_sound_matches() {
        check(
            ee::s08::f_00298d00__generated,
            ssxaudiosystem_play_hit_boost_sound,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x98d00, 0);
                let audio = DATA + 0x100;
                let rider = DATA + 0x200;
                let chans = DATA + 0x800;
                c.g[A0][0] = sx(audio);
                c.g[A1][0] = sx(rider);
                c.bus.write32(audio + 0x118, chans);
                c.bus.write32(chans, DATA + 0x840);
                c.bus
                    .write32(rider + 0x874, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(rider + 0x87c, if seed % 2 == 0 { 0 } else { seed % 5 });
            },
        );
    }

    #[test]
    fn ssxaudiosystem_djsay20e6_matches() {
        check(
            ee::s08::f_002a27d8__generated,
            ssxaudiosystem_djsay20e6,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0xa27d8, 0);
                let audio = DATA + 0x100;
                c.g[A0][0] = sx(audio);
                c.bus.write32(GAME_GP + 0x173c, FAKE);
                c.bus.write32(audio + 0x5560, 0);
                c.bus.write32(audio + 0x118, DATA + 0x800);
            },
        );
    }

    #[test]
    fn speechsystem_shutdown_matches() {
        check(
            ee::s08::f_002b0da0__generated,
            speechsystem_shutdown,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0xb0da0, 0);
                let s = DATA + 0x100;
                let insts = DATA + 0x800;
                c.g[A0][0] = sx(s);
                c.bus.write32(s + 4, insts);
                c.bus
                    .write32(insts, if seed % 2 == 0 { 0 } else { DATA + 0x900 });
                c.bus
                    .write32(insts + 4, if seed % 3 == 0 { 0 } else { DATA + 0x940 });
                c.bus
                    .write32(s + 8, if seed % 4 == 0 { 0 } else { DATA + 0xa00 });
            },
        );
    }

    // -----------------------------------------------------------------------
    // Menu system.

        #[test]
    fn menusyspage_handle_input_matches() {
        covered(
            ee::s09::f_002cacc0__generated,
            menusyspage_handle_input,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0xcacc0, 1);
                let page = DATA + 0x100;
                let item = DATA + 0x200;
                c.g[A0][0] = sx(page);
                c.bus.write32(page + 4, (seed % 3) as u32);
                c.bus.write32(page + 0xc + 4 * (seed % 3), item);
                let vt = DATA + 0x300;
                slot(c, vt, 0x10, 0, FAKE);
                c.bus.write32(item + 0x10, vt);
                // The fake returns a forced code: 3 (next), 4 (prev) or other.
                let code = match seed % 3 {
                    0 => 3,
                    1 => 4,
                    _ => 0x11,
                };
                for n in 0..8u32 {
                    c.bus.write32(LOG_RESULTS + 4 * n, code);
                }
            },
        );
    }

    #[test]
    fn menusyspage_insert_item_matches() {
        check(
            ee::s09::f_002cbcc8__generated,
            menusyspage_insert_item,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0xbcc8, 0);
                let item = DATA + 0x100;
                let page = DATA + 0x200;
                c.g[A0][0] = sx(item);
                c.g[A1][0] = sx(page);
                let count = 3u32;
                c.bus.write32(item, count);
                c.bus.write32(item + 4, (seed % 4) as u32);
                c.bus
                    .write32(item + 0x124, if seed % 2 == 0 { 0 } else { 1 });
                for i in 0..5u32 {
                    c.bus.write32(item + 0xc + 4 * i, 0x1000 + i);
                }
                c.g[A2][0] = match seed % 5 {
                    0 => sx(0xffff_ffff),
                    1 => sx(count),
                    _ => ((seed / 5) % 4) as u64,
                };
            },
        );
    }

    #[test]
    fn menusysactionitem_handle_input_matches() {
        check(
            ee::s09::f_002cd008__generated,
            menusysactionitem_handle_input,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0xcd008, 0);
                let item = DATA + 0x100;
                let parent = DATA + 0x200;
                let input = DATA + 0x300;
                c.g[A0][0] = sx(item);
                c.bus.write32(item, parent);
                c.bus.write32(item + 4, if seed % 2 == 0 { 0 } else { 8 });
                c.bus.write32(item + 0x1c, 0x55);
                c.bus.write32(parent + 0x124, input);
                c.bus
                    .write32(input + 0x48, [0u32, 1, 2, 6, 7][(seed / 2 % 5) as usize]);
                c.bus.write32(parent + 0x12c, input);
                slot(c, input, 0x18, 0, FAKE);
            },
        );
    }

    #[test]
    fn ckeyboardpopup_update_matches() {
        covered(
            ee::s04::f_001cb208__generated,
            ckeyboardpopup_update,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0xcb208, 1);
                let s = DATA + 0x100;
                c.g[A0][0] = sx(s);
                c.bus.write32(s + 0x440, if seed % 3 == 0 { 0 } else { 1 });
                let widget = DATA + 0x300;
                c.bus
                    .write32(s + 0x20, if seed % 4 == 0 { 0 } else { widget });
                let vt = DATA + 0x400;
                slot(c, vt, 0x60, 0, FAKE);
                c.bus.write32(widget + 8, vt);
                match seed % 4 {
                    0 => c.bus.write32(GAME_GP - 0x204, 0),
                    1 => {
                        let input = DATA + 0x500;
                        c.bus.write32(GAME_GP - 0x204, input);
                        c.bus.write32(input + 0x64, 1);
                    }
                    _ => {
                        let input = DATA + 0x500;
                        c.bus.write32(GAME_GP - 0x204, input);
                        c.bus.write32(input + 0x64, 0);
                        if seed % 8 == 2 {
                            c.bus.write32(DATA + 0x9000 + 0x84, 0);
                        }
                    }
                }
                c.bus.write32(s + 0x10, 0x10);
            },
        );
    }

    // -----------------------------------------------------------------------
    // World and game states.

    #[test]
    fn world_reset_lights_matches() {
        check(
            ee::s13::f_003ad1a8__generated,
            world_reset_lights,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0xd1a8, 0);
                let race = DATA + 0x9400;
                let lights = DATA + 0x800;
                let arr = DATA + 0x880;
                c.bus.write32(race + 0x90, lights);
                c.bus
                    .write32(race + 0xc, if seed % 5 == 0 { 0 } else { DATA + 0x900 });
                c.bus.write32(DATA + 0x900 + 0x78, (seed % 4) as u32);
                c.bus.write32(DATA + 0x900 + 0x28, arr);
                for i in 0..4u32 {
                    c.bus.write32(arr + 4 * i, DATA + 0x1000 + 0x40 * i);
                }
            },
        );
    }

    #[test]
    fn game_state_load_state_matches() {
        for (gen, hand) in [
            (
                ee::s06::f_00235a18__generated as fn(&mut Cpu),
                gamestatereset_load_state as fn(&mut Cpu),
            ),
            (ee::s06::f_00235fd0__generated, gamestaterestartrace_load_state),
        ] {
            covered(gen, hand, SEEDS, &|c, seed| {
                base(c, seed, 0x35a18, 4);
                let state = DATA + 0x100;
                let data = DATA + 0x200;
                c.g[A0][0] = sx(state);
                c.g[A1][0] = sx(data);
                let vtobj = DATA + 0x300;
                c.bus.write32(data, vtobj);
                slot(c, vtobj, 0x10, 0, FAKE);
            });
        }
    }

    #[test]
    fn gamestatepreracemenu_resume_race_matches() {
        covered(
            ee::s06::f_00236cd8__generated,
            gamestatepreracemenu_resume_race,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x36cd8, 3);
                let engine = DATA + 0x800;
                let obj = DATA + 0x300;
                let mgr = DATA + 0x500;
                let vtobj = DATA + 0x600;
                c.bus.write32(GAME_GP - 0x84c, engine);
                c.bus.write32(GAME_GP - 0x488, obj);
                c.bus.write32(obj + 0xc, mgr);
                c.bus.write32(mgr + 0xcc, vtobj);
                slot(c, vtobj, 0x28, 0, FAKE);
                c.bus.write8(0x0053_5c10, if seed % 2 == 0 { 4 } else { 0 });
            },
        );
    }

    #[test]
    fn streambuffer_dtor_matches() {
        check(ee::s08::f_002a9df0__generated, streambuffer_dtor, SEEDS, &|c, seed| {
            base(c, seed, 0xa9df0, 0);
            let s = DATA + 0x100;
            c.g[A0][0] = sx(s);
            c.g[A1][0] = (seed & 1) as u64;
            c.bus.write32(s, if seed % 5 == 0 { 0 } else { 1 });
            c.bus.write32(s + 4, 1);
            c.bus.write32(s + 8, 0);
            c.bus.write32(s + 0x74, 0);
        });
    }

    #[test]
    fn scriptobj_pause_resume_matches() {
        for (gen, hand) in [
            (
                ee::s07::f_00283320__generated as fn(&mut Cpu),
                scriptobj_pause as fn(&mut Cpu),
            ),
            (ee::s07::f_002833a8__generated, scriptobj_resume),
        ] {
            check(gen, hand, SEEDS, &|c, seed| {
                base(c, seed, 0x83320, 0);
                let obj = DATA + 0x100;
                let binding = DATA + 0x200;
                c.g[A0][0] = sx(obj);
                c.bus.write32(obj + 0xc, binding);
                slot(c, binding, 0x30, 0, FAKE);
                slot(c, binding, 0x38, 0, FAKE + 0x10);
                c.bus.write32(LOG_RESULTS, 1 + seed % 3);
            });
        }
    }

    #[test]
    fn vd_create_texture_sink_matches() {
        covered(
            ee::s06::f_00254368__generated,
            vd_create_texture_sink,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x54368, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = (seed % 0x40) as u64;
                c.g[A2][0] = ((seed / 0x40) % 0x40) as u64;
                c.g[A3][0] = (seed % 4) as u64;
                c.bus.write32(0x0050_9430, FAKE);
                c.bus.write32(0x0050_9438, 0x1234);
            },
        );
    }

    #[test]
    fn benewplayerint_save_slot_defaults_matches() {
        check(
            ee::s01::f_00145a98__generated,
            benewplayerint_save_slot_defaults,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x45a98, 0);
                for i in 0..12u32 {
                    let src = 0x0053_4fe0 + 0x1c * i;
                    let dst = 0x0053_5b20 + 0x1c * i;
                    c.bus.write32(src + 0x10, r_u32(seed, i));
                    c.bus.write32(dst + 0x10, r_u32(seed, i + 32));
                    for k in 0..7u32 {
                        c.bus.write32(src + 4 * k, r_u32(seed, i * 8 + k));
                        c.bus.write32(dst + 4 * k, r_u32(seed, i * 8 + k + 64));
                    }
                }
            },
        );
    }

    #[test]
    fn benewplayerint_set_char_record_field27c_matches() {
        check(
            ee::s01::f_00146e10__generated,
            benewplayerint_set_char_record_field27c,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x46e10, 0);
                let player = (seed % 4) as u32;
                let profile = ((seed / 4) % 4) as u32;
                let slot = ((seed / 16) % 4) as u32;
                c.g[A1][0] = player as u64;
                c.g[A2][0] = r_u32(seed, 9) as u64;
                c.bus.write32(0x0053_05b0 + 4 * player, profile);
                let rec = 0x0053_5b20 + 0x1c * profile;
                c.bus
                    .write32(rec + 0xc, if seed % 2 == 0 { 0xffff_ffff } else { 0 });
                c.bus.write32(rec + 0x10, seed & 1);
                let slot_base = 0x0053_4fe0 + 0x1c * slot;
                for k in 0..7u32 {
                    c.bus.write32(slot_base + 4 * k, r_u32(seed, k));
                }
                c.bus.write8(slot_base + 0x11, (seed % 8) as u8);
                c.bus.write32(0x004a_6ca8 + 0x27c, 0);
            },
        );
    }

    #[test]
    fn collspheretree_collide_matches() {
        for (gen, hand) in [
            (
                ee::s11::f_003308c8__generated as fn(&mut Cpu),
                collspheretree_collide_sphere_tree as fn(&mut Cpu),
            ),
            (ee::s11::f_00330950__generated, collspheretree_collide_aabb),
        ] {
            check(gen, hand, SEEDS, &|c, seed| {
                base(c, seed, 0x3308c8, 0);
                let s = DATA + 0x100;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = 0;
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] = sx(DATA + 0x200);
                // No collision tree: the deep tree walk needs a real sphere
                // tree, so the zero path is what the fixture can bound.
                c.bus.write32(s + 0x60, 0);
            });
        }
    }

    #[test]
    fn cworldsound_remove_emitter_matches() {
        check(
            ee::s08::f_002b77c0__generated,
            cworldsound_remove_emitter,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0xb77c0, 0);
                let s = DATA + 0x1000;
                c.g[A0][0] = sx(s);
                let id = (seed % 4) as u64;
                c.g[A1][0] = id;
                for t in 0..2u32 {
                    let base_ = s + 0x120 * t;
                    let count = ((seed / (1 + t)) % 4) as u32;
                    c.bus.write32(base_, count);
                    for j in 0..4u32 {
                        c.bus.write32(
                            base_ + 4 + 4 * j,
                            if (j + t) % 3 == 0 {
                                seed % 4
                            } else {
                                0x100 + j
                            },
                        );
                    }
                }
            },
        );
    }
}
