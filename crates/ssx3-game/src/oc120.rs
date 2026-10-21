//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path UI / front-end / script / audio helpers, each
//! with a differential test against the generated body.
//!
//! These are mostly small wrappers: constructors and destructors that publish
//! a vtable and forward to their base class, "command" handlers that look an
//! object up by hash and apply one change, and table initialisers. The vtable
//! calls issue the original's loads and register writes in the original order
//! and dispatch through [`crate::call::call_ptr`], so a test can point a slot at
//! a fake callee and compare the whole register dance. Loops that check events
//! use [`crate::events`].

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0027_7778, script_list_poll_loading, "ScriptList_PollLoading"),
    (0x0039_cca8, ui_screen_cmd_play_anim, "UIScreen_CmdPlayAnim"),
    (0x0039_d8c0, c_ui_scroll_dtor, "cUIScroll_dtor"),(0x0031_8350, c_bx_string_ctor_cstr_n, "cBXString_ctor_cstr_n"),
    (0x0031_8540, c_bx_string_concat_copy, "cBXString_ConcatCopy"),
    (0x002e_c9e0, c_bolt_on_fx_interface_ctor, "cBoltOnFXInterface_ctor"),
    (0x002e_ca68, c_bolt_on_fx_interface_ctor_2, "cBoltOnFXInterface_ctor_2"),
    (0x003a_7a20, world_memman_ctor, "WorldMemman_ctor"),
    (0x003a_81a8, c_world_cache_create_section_table, "cWorldCache_CreateSectionTable"),
    (0x003e_d070, tag_field_get_token, "TagFieldGetToken"),
    (0x0039_8910, c_ui_group_update_anim, "cUIGroup_UpdateAnim"),
    (0x0035_3278, modifier_block_remove_dead_particles, "ModifierBlock_RemoveDeadParticles"),(0x0023_3c50, game_state_finish_enter, "GameStateFinish_Enter"),(0x0022_4c78, luno_vm_register_read_ptr, "LunoVMRegister_ReadPtr"),
    (0x0022_6768, c_luno_table_entry_dtor, "cLunoTableEntry_dtor"),
    (0x002d_7dd8, avalanche_release_instance, "Avalanche_ReleaseInstance"),
    (0x0039_bb50, ui_pair_set_visible, "UIPair_SetVisible"),
    (0x0039_c240, c_ui_progress_dtor, "cUIProgress_dtor"),
    (0x001f_7210, ov_screen_enter_lodge_on_activate, "OVScreenEnterLodge_OnActivate"),
    (0x0018_71a8, c_fe_state_mem_card_restart_scan, "cFEStateMemCard_RestartScan"),
    (0x0030_ec18, rider_model_show_part_variant, "RiderModel_ShowPartVariant"),
    (0x0015_ce10, manual_cam_controller_dtor, "ManualCamController_dtor"),
    (0x0031_4fe8, game_anim_map_unregister_bank, "GameAnimMap_UnregisterBank"),
    (0x0016_c800, camera_trigger_man_ctor, "CameraTriggerMan_ctor"),
    (0x001a_3c50, fe_async_manager_free_all, "FEAsyncManager_FreeAll"),
    (0x001b_66b8, net_config_auth_popup_handle_input, "NetConfigAuthPopup_HandleInput"),
    (0x001b_8960, c_fe_state_create_step1_init, "cFEStateCreateStep1_Init"),
    (0x001b_a358, c_fe_state_create_step1_detect_changes, "cFEStateCreateStep1_DetectChanges"),
    (0x001b_c6a0, c_fe_state_create_duplicate_on_child_event, "cFEStateCreateDuplicate_OnChildEvent"),(0x001c_0a80, c_fe_state_online_agreement_update, "cFEStateOnlineAgreement_Update"),
    (0x001e_0470, c_fe_popup_select_multiplayer_mode_on_widget_notify, "cFEPopupSelectMultiplayerMode_OnWidgetNotify"),
    (0x001e_14c0, message_center_trigger_groups, "MessageCenter_TriggerGroups"),
    (0x0025_fa48, lobby_man_chal_request_host, "LobbyMan_ChalRequestHost"),
    (0x0025_fad0, lobby_man_chal_request_join, "LobbyMan_ChalRequestJoin"),
    (0x0026_35a8, buddy_man_delete_message_at, "BuddyMan_DeleteMessageAt"),
    (0x002a_7c68, ini_parser_find_section_prefix, "IniParser_FindSectionPrefix"),
    (0x0027_cdd0, script_engine_obj_table_ctor, "ScriptEngine_ObjTableCtor"),
    (0x0027_d850, script_obj_cam_activate, "ScriptObjCam_Activate"),
    (0x0028_05b8, script_audio_obj_ctor, "ScriptAudioObj_ctor"),
    (0x002a_f580, path_plat_set_track_level, "PATH_PlatSetTrackLevel"),
    (0x0027_4240, in_seq_channel_set_listener, "InSeq_ChannelSetListener"),
    (0x002c_6360, c_ps2_mem_card_close, "cPS2MemCard_Close"),
    (0x0016_0ae8, matrix_chase_camera_algorithm_reset_for_rider, "MatrixChaseCameraAlgorithm_ResetForRider"),
    (0x0016_8940, look_back_chase_camera_algorithm_reset_for_rider, "LookBackChaseCameraAlgorithm_ResetForRider"),
    (0x0016_8c50, mid_right_chase_camera_algorithm_reset_for_rider, "MidRightChaseCameraAlgorithm_ResetForRider"),
];

/// Direct and cross-shard callees, by guest address.
mod callee {
    use super::Callee;
    use ssx3_ee as ee;

    pub const SCRIPT_CMD_QUEUE_AT: Callee = Callee { addr: 0x0028_3d70, f: ee::s07::f_00283d70 };
    pub const SCRIPT_LIST_IS_MOVIE_ENTRY: Callee =
        Callee { addr: 0x0027_7dd8, f: ee::s07::f_00277dd8 };
    pub const SCRIPT_ENGINE_GET_PROCESS_STATE: Callee =
        Callee { addr: 0x0028_2798, f: ee::s07::f_00282798 };

    pub const UI_SCREEN_FIND_OBJECT: Callee = Callee { addr: 0x0039_d860, f: ee::s13::f_0039d860 };
    pub const UI_ANIM_SET_CREATE_ANIM: Callee =
        Callee { addr: 0x0039_7140, f: ee::s13::f_00397140 };
    pub const UI_OBJECT_SET_ANIM: Callee = Callee { addr: 0x0039_fcc8, f: ee::s13::f_0039fcc8 };

    pub const BX_STRING_ALLOC_BUFFER: Callee =
        Callee { addr: 0x0031_8040, f: ee::s10::f_00318040 };

    pub const BOLT_ON_FX_INIT_HANDLE: Callee =
        Callee { addr: 0x002e_c9d0, f: ee::s09::f_002ec9d0 };
    pub const BOLT_ON_FX_CLEAR_EMITTERS: Callee =
        Callee { addr: 0x002e_caf0, f: ee::s09::f_002ecaf0 };

    pub const WORLD_BLOCK_POOL_INIT: Callee = Callee { addr: 0x003a_7598, f: ee::s13::f_003a7598 };
    pub const WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR: Callee =
        Callee { addr: 0x003a_7b18, f: ee::s13::f_003a7b18 };
    pub const SECTION_TABLE_CTOR: Callee = Callee { addr: 0x003a_7c88, f: ee::s13::f_003a7c88 };

    pub const UI_LIST_FIRST: Callee = Callee { addr: 0x0039_7840, f: ee::s13::f_00397840 };
    pub const UI_LIST_NODE_IS_SENTINEL: Callee =
        Callee { addr: 0x0039_7718, f: ee::s13::f_00397718 };
    pub const UI_OBJECT_UPDATE_ANIM: Callee = Callee { addr: 0x003a_0000, f: ee::s13::f_003a0000 };

    pub const WORLD_OBJ_CTOR_2: Callee = Callee { addr: 0x0034_fb90, f: ee::s11::f_0034fb90 };
    pub const FLEX_BRIDGE_SPRINGS_LOAD: Callee =
        Callee { addr: 0x0035_3e80, f: ee::s11::f_00353e80 };
    pub const FLEX_BRIDGE_BUILD_MESH: Callee =
        Callee { addr: 0x0034_66a0, f: ee::s11::f_003466a0 };
    pub const FLEX_BRIDGE_UPDATE_VERTS: Callee =
        Callee { addr: 0x0034_75d8, f: ee::s11::f_003475d8 };
    pub const INSTANCE_GET_NODE_MATRIX: Callee =
        Callee { addr: 0x0034_fed8, f: ee::s11::f_0034fed8 };

    pub const GAME_STATE_FINISH_CLEAR_FLAG40: Callee =
        Callee { addr: 0x0024_4880, f: ee::s06::f_00244880 };

    pub const LUNO_VM_REGISTER_DTOR: Callee = Callee { addr: 0x0022_5b90, f: ee::s05::f_00225b90 };

    pub const UI_OBJECT_DTOR: Callee = Callee { addr: 0x0039_fc48, f: ee::s13::f_0039fc48 };

    pub const CAM_CONTROLLER_DTOR: Callee = Callee { addr: 0x0015_cc10, f: ee::s01::f_0015cc10 };

    pub const GAME_ANIM_MAP_SET_HANDLE: Callee =
        Callee { addr: 0x0031_4ef0, f: ee::s10::f_00314ef0 };

    pub const CAMERA_TRIGGER_LIST_CTOR: Callee =
        Callee { addr: 0x0016_bee8, f: ee::s02::f_0016bee8 };
    pub const ACTIVE_CAMERA_TRIGGER_LIST_CTOR: Callee =
        Callee { addr: 0x0016_c5d0, f: ee::s02::f_0016c5d0 };
    pub const CAMERA_TRIGGER_STACK_CTOR: Callee =
        Callee { addr: 0x0016_d210, f: ee::s02::f_0016d210 };

    pub const NET_FE_STATE_INIT: Callee = Callee { addr: 0x001a_8768, f: ee::s03::f_001a8768 };

    pub const FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER: Callee =
        Callee { addr: 0x001b_c818, f: ee::s03::f_001bc818 };
    pub const FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT: Callee =
        Callee { addr: 0x001b_c728, f: ee::s03::f_001bc728 };
    pub const FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT: Callee =
        Callee { addr: 0x001b_c990, f: ee::s03::f_001bc990 };
    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee =
        Callee { addr: 0x001a_97b8, f: ee::s03::f_001a97b8 };

    pub const FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB: Callee =
        Callee { addr: 0x001b_df60, f: ee::s03::f_001bdf60 };
    pub const FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM: Callee =
        Callee { addr: 0x001b_e3a8, f: ee::s03::f_001be3a8 };
    pub const FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST: Callee =
        Callee { addr: 0x001b_e510, f: ee::s03::f_001be510 };
    pub const NET_FE_STATE_UPDATE: Callee = Callee { addr: 0x001a_8770, f: ee::s03::f_001a8770 };

    pub const MESSAGE_CENTER_IS_FLAG_SET: Callee =
        Callee { addr: 0x001e_3a30, f: ee::s04::f_001e3a30 };
    pub const MESSAGE_CENTER_EMPTY_TRIGGER: Callee =
        Callee { addr: 0x001e_1548, f: ee::s04::f_001e1548 };

    pub const LOBBY_MAN_SEND_REQUEST: Callee = Callee { addr: 0x0025_b608, f: ee::s06::f_0025b608 };

    pub const INI_PARSER_NEXT_SECTION: Callee = Callee { addr: 0x002a_7b08, f: ee::s08::f_002a7b08 };

    pub const SCRIPT_OBJ_ACTIVATE: Callee = Callee { addr: 0x0028_3430, f: ee::s07::f_00283430 };
    pub const SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT: Callee =
        Callee { addr: 0x0027_d970, f: ee::s07::f_0027d970 };

    pub const SCRIPT_BINDING_CTOR: Callee = Callee { addr: 0x0028_2f60, f: ee::s07::f_00282f60 };
    pub const SCRIPT_OBJ_CTOR: Callee = Callee { addr: 0x0028_3200, f: ee::s07::f_00283200 };

    pub const PS2_MEM_CARD_CLEAR_DIR_CACHE: Callee =
        Callee { addr: 0x002c_5278, f: ee::s08::f_002c5278 };

    pub const CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER: Callee =
        Callee { addr: 0x0016_6c60, f: ee::s02::f_00166c60 };
    pub const CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER: Callee =
        Callee { addr: 0x0016_6550, f: ee::s02::f_00166550 };
}

// ---------------------------------------------------------------------------
// Small helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Script list.

/// `void ScriptList_PollLoading(ScriptList* self)` (0x277778): while the list
/// waits for its movie/process command (state 1), start the command at the
/// queue head; the state becomes 2 when the command is a movie entry or its
/// process reports state 2.
pub fn script_list_poll_loading(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0x1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S0), 0xa4)).lw(c);
        if c.g[V0][0] != c.g[V1][0] {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 9;
            c.g[A0][0] = sx(p(r32(c, S0), 0xc));
            c.g[A1][0] = 0;
            callee::SCRIPT_CMD_QUEUE_AT.call(c, 0x0027_77a8)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            callee::SCRIPT_LIST_IS_MOVIE_ENTRY.call(c, 0x0027_77b8)?;
            let movie = c.g[V0][0] != 0;
            c.g[V0][0] = 0x2;
            if movie {
                c.cycles += 1;
                Addr(p(r32(c, S0), 0xa4)).sw(c, c.g[V0][0]);
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 7;
                c.g[A1][0] = Addr(p(r32(c, S1), 0xc)).lw(c);
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                callee::SCRIPT_ENGINE_GET_PROCESS_STATE.call(c, 0x0027_77cc)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = 0x2;
                if c.g[V1][0] == c.g[V0][0] {
                    Addr(p(r32(c, S0), 0xa4)).sw(c, c.g[V1][0]);
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                } else {
                    // The process is still pending: leave the list alone.
                    c.cycles += 2;
                    frame.restore(c, S0, 0x20);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI commands.

/// `void UIScreen_CmdPlayAnim(UIScreen* self, int, UIMsg* msg)` (0x39cca8):
/// play the animation named by `msg[+4]` on the object named by `msg[+8]`.
pub fn ui_screen_cmd_play_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x0039_ccd4);
        c.g[A1][0] = Addr(p(r32(c, S0), 8)).lw(c);
        callee::UI_SCREEN_FIND_OBJECT.call(c, 0x0039_ccd4)?;
        c.g[S2][0] = c.g[V0][0];
        if c.g[S2][0] == 0 {
            frame.restore(c, S0, 0x40);
        } else {
            c.cycles += 7;
            c.g[V0][0] = Addr(p(r32(c, S1), 0xd0)).lw(c);
            c.g[A1][0] = Addr(p(r32(c, S0), 4)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lw(c);
            c.g[RA][0] = sx(0x0039_ccf4);
            c.g[A0][0] = sx(p(r32(c, A0), 0x50));
            callee::UI_ANIM_SET_CREATE_ANIM.call(c, 0x0039_ccf4)?;
            let missing = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S2][0];
            if !missing {
                c.cycles += 5;
                c.g[A2][0] = Addr(p(r32(c, S0), 0xc)).lbu(c);
                c.g[A3][0] = Addr(p(r32(c, S3), 8)).lhu(c);
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0039_cd10);
                c.g[T0][0] = 0;
                callee::UI_OBJECT_SET_ANIM.call(c, 0x0039_cd10)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIScroll_dtor(cUIScroll* self, int flags)` (0x39d8c0): release the
/// two child objects and run the base destructor.
pub fn c_ui_scroll_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0049_44c8);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x74)).lw(c);
        let first_null = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !first_null {
            c.cycles += 6;
            c.g[A1][0] = 0x3;
            delete_owner(c, 8, 0x0039_d904)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x78)).lw(c);
        let second_null = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !second_null {
            c.cycles += 5;
            delete_owner(c, 8, 0x0039_d924)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0039_d930);
        c.g[A1][0] = c.g[S1][0];
        callee::UI_OBJECT_DTOR.call(c, 0x0039_d930)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `sDeleteTarget` sequence of the UI destructors: `v0 = lw(a2+slot)`,
/// `delta = lh(v0+8)`, `fn = lw(v0+0xc)`, `a0 = a2 + delta`, `jalr fn`. `a1`
/// (the delete kind, 3) must already be set.
fn delete_owner(c: &mut Cpu, slot: u32, ret: u32) -> Flow {
    c.g[V0][0] = Addr(p(r32(c, A2), slot)).lw(c);
    c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
    c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
    let target = r32(c, V1);
    c.g[RA][0] = sx(ret);
    c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
    call_ptr(c, target, ret)
}

/// `void cUISlider_dtor(cUISlider* self, int flags)` (0x39df28): same shape as
/// [`c_ui_scroll_dtor`] with the slider's child slots at +0x88 and +0x8c.
pub fn c_ui_slider_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0049_4348);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x88)).lw(c);
        let first_null = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !first_null {
            c.cycles += 6;
            delete_owner(c, 8, 0x0039_df6c)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x8c)).lw(c);
        let second_null = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !second_null {
            c.cycles += 5;
            delete_owner(c, 8, 0x0039_df8c)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0039_df98);
        c.g[A1][0] = c.g[S1][0];
        callee::UI_OBJECT_DTOR.call(c, 0x0039_df98)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BX strings.

/// `cBXString* cBXString_ctor_cstr_n(cBXString* self, const char* str, int n)`
/// (0x318350): copy at most `n` bytes of `str` into a new BX string.
pub fn c_bx_string_ctor_cstr_n(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        Addr(r32(c, S2)).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x0031_8380);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0041_6810, 0x0031_8380)?;
        c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0031_8394);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0041_6810, 0x0031_8394)?;
            c.g[S0][0] = c.g[V0][0];
        }
        c.cycles += 2;
        let empty = c.g[S0][0] == 0;
        c.g[A0][0] = c.g[S2][0];
        if !empty {
            c.cycles += 6;
            c.g[RA][0] = sx(0x0031_83a8);
            c.g[A1][0] = c.g[S0][0];
            callee::BX_STRING_ALLOC_BUFFER.call(c, 0x0031_83a8)?;
            c.g[A0][0] = Addr(r32(c, S2)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0031_83b8);
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0041_605c, 0x0031_83b8)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString* cBXString_ConcatCopy(cBXString* self, int len1, const char* s1,
/// int len2, const char* s2)` (0x318540): a new string of the two pieces (the
/// generated call order puts the lengths in `a1`/`a3`).
pub fn c_bx_string_concat_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S2, 0x30);
        c.g[A1][0] = sx(p(r32(c, S1), r32(c, S3)));
        frame.save(c, S4, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        let empty = c.g[A1][0] == 0;
        c.g[S4][0] = c.g[T0][0];
        if !empty {
            c.cycles += 11;
            c.g[RA][0] = sx(0x0031_8580);
            callee::BX_STRING_ALLOC_BUFFER.call(c, 0x0031_8580)?;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x0031_8590);
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0041_605c, 0x0031_8590)?;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x0031_85a4);
            c.g[A0][0] = sx(p(r32(c, A0), r32(c, S1)));
            call_ptr(c, 0x0041_605c, 0x0031_85a4)?;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
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
// World memory manager and cache.

/// `WorldMemman* WorldMemman_ctor(WorldMemman* self)` (0x3a7a20): initialise
/// the three block pools and clear the 64-entry block table.
pub fn world_memman_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0x45_0000;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(0x0044_c1e0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = Addr(p(r32(c, S0), 4)).lw(c);
        c.g[RA][0] = sx(0x003a_7a48);
        c.g[A2][0] = Addr(p(r32(c, S0), 8)).lw(c);
        callee::WORLD_BLOCK_POOL_INIT.call(c, 0x003a_7a48)?;
        c.g[A1][0] = Addr(p(r32(c, S0), 0x10)).lw(c);
        c.g[A0][0] = sx(p(r32(c, S1), 0x14));
        c.g[RA][0] = sx(0x003a_7a58);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
        callee::WORLD_BLOCK_POOL_INIT.call(c, 0x003a_7a58)?;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x20)).lw(c);
        c.g[A0][0] = sx(p(r32(c, S1), 0x28));
        c.g[RA][0] = sx(0x003a_7a68);
        c.g[A1][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
        callee::WORLD_BLOCK_POOL_INIT.call(c, 0x003a_7a68)?;
        c.g[V1][0] = 0x3f;
        c.g[V0][0] = sx(p(r32(c, S1), 0x138));
        loop {
            c.cycles += 7;
            Addr(r32(c, V0)).sw(c, 0);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_7a70);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cWorldCache_CreateSectionTable(cWorldCache* self, int sections,
/// void* section)` (0x3a81a8): create the section allocator, allocate the
/// section table and construct it.
pub fn c_world_cache_create_section_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x003a_81cc);
        c.g[A0][0] = Addr(p(r32(c, S2), 0xc)).lw(c);
        callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR.call(c, 0x003a_81cc)?;
        c.g[V0][0] = Addr(p(r32(c, S2), 8)).lw(c);
        c.g[S1][0] = sx(r32(c, S0) << 2);
        c.g[A1][0] = 0x49_0000;
        c.g[A3][0] = 0;
        c.g[S1][0] = sx(p(r32(c, S1), r32(c, V0)));
        c.g[A1][0] = sx(0x0049_4e60);
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x003a_81f0);
        c.g[A0][0] = 0xe8;
        call_ptr(c, 0x0031_7d70, 0x003a_81f0)?;
        c.g[V1][0] = 0x58;
        c.g[A2][0] = Addr(r32(c, S2)).lw(c);
        c.mult(0, 16, 16, 3);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(p(r32(c, A2), 0x14)).lw(c);
        c.g[A1][0] = sx(p(r32(c, A1), r32(c, S0)));
        c.g[RA][0] = sx(0x003a_8210);
        c.g[A1][0] = sx(p(r32(c, A1), 0x20));
        callee::SECTION_TABLE_CTOR.call(c, 0x003a_8210)?;
        Addr(r32(c, S1)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
// ---------------------------------------------------------------------------
// Tag fields.

/// `int TagFieldGetToken(const char* data, int defval)` (0x3ed070): read up to
/// four characters into a FourCC (starting from `0x20202020`), or `defval`
/// when the text is empty or the token overflows; a token ends at a control
/// character or a character >= 127.
pub fn tag_field_get_token(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A2][0] = c.g[A0][0];
    let null = c.g[A2][0] == 0;
    c.g[V0][0] = c.g[A1][0];
    if null {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    let mut data = r32(c, A2);
    c.g[V1][0] = Addr(data).lb(c);
    let ch = c.g[V1][0];
    c.g[T0][0] = ((ch as i64) < 33) as u64;
    let too_small = c.g[T0][0] != 0;
    c.g[A3][0] = Addr(data).lbu(c);
    if too_small {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V1][0] = ((ch as i64) < 127) as u64;
    let too_big = c.g[V1][0] == 0;
    c.g[A0][0] = 0x2020_0000;
    if too_big {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[A0][0] | 0x2020;
    c.cycles += 2;
    c.g[A1][0] = 0x20ff_0000;
    c.cycles += 2;
    c.g[A1][0] = c.g[A1][0] | 0xfffe;
    c.g[V0][0] = sx(r32(c, A3) << 24);
    loop {
        c.cycles += 6;
        c.g[A0][0] = sx(r32(c, A0) << 8);
        c.g[V0][0] = sx(((r32(c, V0) as i32) >> 24) as u32);
        c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
        c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        let stop = c.g[V1][0] != 0;
        c.g[A2][0] = sx(data.wrapping_add(1));
        if stop {
            break;
        }
        c.cycles += 4;
        data = r32(c, A2);
        c.g[V1][0] = Addr(data).lb(c);
        let ch = c.g[V1][0];
        c.g[V0][0] = ((ch as i64) < 33) as u64;
        let too_small = c.g[V0][0] != 0;
        c.g[A3][0] = Addr(data).lbu(c);
        if too_small {
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = ((ch as i64) < 127) as u64;
        let too_big = c.g[V0][0] != 0;
        c.g[V0][0] = sx(r32(c, A3) << 24);
        if too_big {
            if event_due(c) {
                deliver_event_here(c, 0x003e_d0b0);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI groups.

const UI_GROUP_UPDATE_ENTRY: u32 = 0x0039_8910;
const UI_GROUP_UPDATE_END: u32 = 0x0039_8994;
/// Resumed after an event in the node walk (the animation applies loop).
const UI_GROUP_UPDATE_NODE: u32 = 0x0039_8948;

/// `void cUIGroup_UpdateAnim(cUIGroup* self, u16 ev)` (0x398910): apply the
/// event to every live node of the group's list, then update the group's own
/// animation.
pub fn c_ui_group_update_anim(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                UI_GROUP_UPDATE_ENTRY => {
                    c.cycles += 12;
                    let frame = Frame::open(c, 0x40);
                    frame.save(c, S2, 0x10);
                    frame.save(c, S1, 0x20);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S0, 0x30);
                    c.g[S1][0] = c.g[A1][0] & 0xffff;
                    frame.save_ra(c, 0);
                    c.g[RA][0] = sx(0x0039_8934);
                    c.g[A0][0] = sx(p(r32(c, S2), 0x74));
                    callee::UI_LIST_FIRST.call(c, 0x0039_8934)?;
                    c.g[S0][0] = c.g[V0][0];
                    let empty = c.g[S0][0] == 0;
                    c.g[A0][0] = c.g[S2][0];
                    if empty {
                        pc = 0x0039_8974;
                        continue;
                    }
                    pc = 0x0039_8940;
                }
                0x0039_8940 => {
                    // The generated code's always-taken branch to the check.
                    c.cycles += 2;
                    pc = 0x0039_8960;
                }
                UI_GROUP_UPDATE_NODE => {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = Addr(p(r32(c, V0), 0x98)).lh(c);
                    c.g[V1][0] = Addr(p(r32(c, V0), 0x9c)).lw(c);
                    let target = r32(c, V1);
                    c.g[RA][0] = sx(0x0039_895c);
                    c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
                    call_ptr(c, target, 0x0039_895c)?;
                    c.g[S0][0] = Addr(p(r32(c, S0), 4)).lw(c);
                    pc = 0x0039_8960;
                }
                0x0039_8960 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0039_8968);
                    c.g[A0][0] = c.g[S0][0];
                    callee::UI_LIST_NODE_IS_SENTINEL.call(c, 0x0039_8968)?;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                        if event_due(c) {
                            c.pc = UI_GROUP_UPDATE_NODE;
                            if c.event() {
                                return Err(crate::call::Unwound);
                            }
                        }
                        pc = UI_GROUP_UPDATE_NODE;
                    } else {
                        pc = 0x0039_8970;
                    }
                }
                0x0039_8970 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S2][0];
                    pc = 0x0039_8974;
                }
                0x0039_8974 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x0039_897c);
                    c.g[A1][0] = c.g[S1][0];
                    callee::UI_OBJECT_UPDATE_ANIM.call(c, 0x0039_897c)?;
                    frame_restore_group(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, UI_GROUP_UPDATE_ENTRY, UI_GROUP_UPDATE_END);
                    return Ok(());
                }
            }
        }
    })
}

/// The shared epilogue of [`c_ui_group_update_anim`].
fn frame_restore_group(c: &mut Cpu) {
    let frame = Frame { size: 0x40 };
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Modifier blocks.

/// `void ModifierBlock_RemoveDeadParticles(ModifierBlock* self)` (0x353278):
/// walk the block's particle links; a live one (its virtual `state()` is 3)
/// is deleted and unlinked.
pub fn modifier_block_remove_dead_particles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(p(r32(c, A0), 0x10)).lw(c);
        let null = c.g[A0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if !null {
            c.cycles += 2;
            c.g[S2][0] = 0x3;
            c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
            loop {
                c.cycles += 7;
                c.g[S1][0] = Addr(r32(c, S0)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lh(c);
                c.g[V0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
                let target = r32(c, V0);
                c.g[RA][0] = sx(0x0035_32b4);
                c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
                call_ptr(c, target, 0x0035_32b4)?;
                if c.g[V0][0] != c.g[S2][0] {
                    c.g[S0][0] = c.g[S1][0];
                } else {
                    c.cycles += 2;
                    let dead = c.g[S0][0] == 0;
                    c.g[A1][0] = 0x3;
                    if !dead {
                        c.cycles += 5;
                        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                        c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
                        c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
                        let target = r32(c, V1);
                        c.g[RA][0] = sx(0x0035_32d8);
                        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
                        call_ptr(c, target, 0x0035_32d8)?;
                    }
                    c.cycles += 1;
                    c.g[S0][0] = c.g[S1][0];
                }
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0035_32a0);
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
// Flex bridges.

/// `FlexBridge* FlexBridge_ctor_2(FlexBridge* self, u32, void* stream)`
/// (0x3464e0): the instance node, the bridge's springs, mesh and vertices.
pub fn flex_bridge_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0034_6504);
        c.g[S2][0] = sx(p(r32(c, S0), 0x20));
        callee::WORLD_OBJ_CTOR_2.call(c, 0x0034_6504)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = sx(0x0049_0270);
        c.g[A0][0] = sx(p(r32(c, S0), 0x50));
        c.g[RA][0] = sx(0x0034_651c);
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V1][0]);
        callee::FLEX_BRIDGE_SPRINGS_LOAD.call(c, 0x0034_651c)?;
        c.g[RA][0] = sx(0x0034_6524);
        c.g[A0][0] = c.g[S0][0];
        callee::FLEX_BRIDGE_BUILD_MESH.call(c, 0x0034_6524)?;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = 0x30;
        c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0034_6540);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, target, 0x0034_6540)?;
        c.g[RA][0] = sx(0x0034_6548);
        c.g[A0][0] = c.g[S0][0];
        callee::FLEX_BRIDGE_UPDATE_VERTS.call(c, 0x0034_6548)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Spring rails.

/// `void SpringRail_InitPosFromInstance(SpringRail* node, int bone)` (0x34a028):
/// resolve the node's model and bone, then copy the world matrix translation
/// into the node's position.
pub fn spring_rail_init_pos_from_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x50);
        c.g[RA][0] = sx(0x0034_a03c);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x002d_1bd8, 0x0034_a03c)?;
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[A1][0] = Addr(p(r32(c, S0), 0x50)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 8)).lw(c);
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(p(r32(c, V0), r32(c, A0)));
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        let empty = c.g[V1][0] == 0;
        c.g[A0][0] = 0;
        if !empty {
            c.cycles += 8;
            c.g[A0][0] = Addr(p(r32(c, V1), 0x1c)).lw(c);
            c.g[V0][0] = sx(r32(c, A1) >> 8);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(p(r32(c, V0), r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = sx(r32(c, V1) >> 8);
            let empty2 = c.g[V0][0] == 0;
            c.g[A0][0] = 0;
            if !empty2 {
                c.cycles += 1;
                c.g[A0][0] = sx(r32(c, V0) << 2);
            }
        }
        c.cycles += 10;
        c.g[A1][0] = Addr(p(r32(c, S0), 0x54)).lw(c);
        c.g[RA][0] = sx(0x0034_a090);
        c.g[A2][0] = c.g[SP][0];
        callee::INSTANCE_GET_NODE_MATRIX.call(c, 0x0034_a090)?;
        c.lq(2, p(r32(c, SP), 0x30));
        c.g[RA][0] = c.ld(p(r32(c, SP), 0x50));
        c.sq(2, r32(c, S0));
        c.lq(16, p(r32(c, SP), 0x60));
        c.sq(2, p(r32(c, SP), 0x40));
        let target = r32(c, RA);
        c.g[SP][0] = sx(p(r32(c, SP), 0x70));
        c.pc = target;
        Ok(())
    })
}
// ---------------------------------------------------------------------------
// Game state.

/// `void GameStateFinish_Enter(GameStateFinish* self)` (0x233c50): stop the
/// replay recorder, close the UI states, stop the script list, mark the
/// loading indicator busy and flag the finished race.
pub fn game_state_finish_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, -0x488)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x0023_3c6c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x28)).lw(c);
        call_ptr(c, 0x0026_fa50, 0x0023_3c6c)?;
        c.g[V0][0] = Addr(gp(c, -0x488)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lw(c);
        c.g[RA][0] = sx(0x0023_3c7c);
        c.g[A0][0] = sx(p(r32(c, A0), 0x18));
        call_ptr(c, 0x0039_f840, 0x0023_3c7c)?;
        c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
        c.g[RA][0] = sx(0x0023_3c88);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0027_90a0, 0x0023_3c88)?;
        c.g[V0][0] = 0xb4;
        Addr(p(r32(c, S0), 0x10)).sw(c, 0);
        Addr(p(r32(c, S0), 0x14)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x4;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x84)).lw(c);
        c.g[RA][0] = sx(0x0023_3ca8);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x94)).lw(c);
        call_ptr(c, 0x001f_36c0, 0x0023_3ca8)?;
        c.g[S0][0] = Addr(gp(c, -0x204)).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 5;
            c.g[RA][0] = sx(0x0023_3cbc);
            c.g[A0][0] = c.g[S0][0];
            callee::GAME_STATE_FINISH_CLEAR_FLAG40.call(c, 0x0023_3cbc)?;
            c.g[V0][0] = 0x1;
            Addr(p(r32(c, S0), 0x90)).sw(c, c.g[V0][0]);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateProgressiveVerifyPS2_OnWidgetAction(cFEState* self, void* item,
/// int msg, u8* payload)` (0x2461e0): on the "verify" widget action, allocate
/// a memory-card state and switch the UI to it.
pub fn c_fe_state_progressive_verify_ps2_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let item_null = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        if item_null {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x5;
            let wrong_msg = c.g[A2][0] != c.g[V0][0];
            frame.restore(c, S0, 0x20);
            if !wrong_msg {
                c.cycles += 6;
                c.g[A0][0] = 0x4a_0000;
                c.g[S0][0] = Addr(p(r32(c, A1), 0x38)).lw(c);
                c.g[RA][0] = sx(0x0024_6214);
                c.g[A0][0] = sx(0x004a_2ac8);
                call_ptr(c, 0x0031_7670, 0x0024_6214)?;
                if c.g[S0][0] == c.g[V0][0] {
                    c.cycles += 13;
                    c.g[A1][0] = 0x48_0000;
                    c.g[A0][0] = 0x248;
                    c.g[A1][0] = sx(0x0047_b978);
                    c.g[A2][0] = 0;
                    c.g[RA][0] = sx(0x0024_6234);
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7d70, 0x0024_6234)?;
                    c.g[A1][0] = Addr(p(r32(c, S1), 0x10)).lw(c);
                    c.g[RA][0] = sx(0x0024_6240);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0018_6a08, 0x0024_6240)?;
                    c.g[A0][0] = Addr(p(r32(c, S1), 0x10)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0024_6250);
                    c.g[A0][0] = sx(p(r32(c, A0), 0x18));
                    call_ptr(c, 0x0039_f400, 0x0024_6250)?;
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Luno VM.

/// `void LunoVMRegister_ReadPtr(LunoVMRegister* self, void* stream, int* out)`
/// (0x224c78): read a 32-bit base and a 32-bit offset from the stream and
/// publish `base + offset` at `out[0]` and the offset at `out[1]`.
pub fn luno_vm_register_read_ptr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[S2][0] = c.g[A0][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x4;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0022_4cb4);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, target, 0x0022_4cb4)?;
        c.g[V1][0] = Addr(r32(c, SP)).lw(c);
        c.g[A1][0] = c.g[SP][0] | 0x4;
        c.g[A2][0] = 0x4;
        c.g[S0][0] = sx(p(r32(c, S0), r32(c, V1)));
        Addr(r32(c, S2)).sw(c, c.g[S0][0]);
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x14)).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x0022_4cdc);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, target, 0x0022_4cdc)?;
        c.g[V0][0] = Addr(p(r32(c, SP), 4)).lw(c);
        frame.restore(c, S0, 0x40);
        Addr(p(r32(c, S2), 4)).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cLunoTableEntry_dtor(cLunoTableEntry* self, int flags)` (0x226768):
/// release the owned register value, destroy the two embedded registers and
/// free the entry when `flags & 1`.
pub fn c_luno_table_entry_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0047_9890);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 0x1c)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x18)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !null {
            c.cycles += 6;
            c.g[V0][0] = Addr(p(r32(c, A2), 0x1c)).lw(c);
            c.g[A1][0] = 0x3;
            c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0022_67ac);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, target, 0x0022_67ac)?;
        }
        c.cycles += 9;
        c.g[A0][0] = sx(p(r32(c, S0), 0xc));
        c.g[RA][0] = sx(0x0022_67b8);
        c.g[A1][0] = 0x2;
        callee::LUNO_VM_REGISTER_DTOR.call(c, 0x0022_67b8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0022_67c4);
        c.g[A1][0] = 0x2;
        callee::LUNO_VM_REGISTER_DTOR.call(c, 0x0022_67c4)?;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0022_67d8);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0022_67d8)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Avalanche.

/// `void Avalanche_ReleaseInstance(Avalanche* self, int force)` (0x2d7dd8):
/// walk to the live emitter (or use the first one when forced), release its
/// sub-emitter and reset the avalanche's colours.
pub fn avalanche_release_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p(r32(c, S0), 0x2e0)).lw(c);
        if c.g[A0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            // Pick the emitter: the first when forced, else the first live one.
            let mut release = true;
            c.cycles += 2;
            if c.g[A1][0] != 0 {
                c.g[A0][0] = Addr(p(r32(c, A0), 0xf8)).lw(c);
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(p(r32(c, A0), 0xf0)).lhu(c);
                c.g[V0][0] = 0x2;
                if c.g[V1][0] == c.g[V0][0] {
                    release = false;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = Addr(p(r32(c, A0), 0xf8)).lw(c);
                }
            }
            if !release {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x2e0)).lw(c);
                } else {
                    c.cycles += 3;
                    c.g[A2][0] = Addr(p(r32(c, A0), 0xc)).lw(c);
                    let no_sub = c.g[A2][0] == 0;
                    c.g[A1][0] = 0x3;
                    if !no_sub {
                        c.cycles += 5;
                        c.g[V0][0] = Addr(p(r32(c, A2), 0xc)).lw(c);
                        c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
                        c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
                        let target = r32(c, V1);
                        c.g[RA][0] = sx(0x002d_7e38);
                        c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                        call_ptr(c, target, 0x002d_7e38)?;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x2e0)).lw(c);
                }
                c.cycles += 5;
                c.g[A0][0] = sx(p(r32(c, S0), 0xd0));
                Addr(p(r32(c, V0), 0x114)).sw(c, 0);
                c.g[RA][0] = sx(0x002d_7e4c);
                Addr(p(r32(c, S0), 0x2e0)).sw(c, 0);
                call_ptr(c, 0x0037_1600, 0x002d_7e4c)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
// ---------------------------------------------------------------------------
// UI objects.

/// `void UIPair_SetVisible(UIPair* self, int on)` (0x39bb50): set the pair's
/// "visible" bit and forward the visibility to both children.
pub fn ui_pair_set_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = (0u64 < c.g[S1][0]) as u64;
        c.g[A0][0] = 0xffff_ffff_ffff_ffbf;
        c.g[V1][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 6);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x78)).lw(c);
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        let null = c.g[A2][0] == 0;
        Addr(p(r32(c, S0), 0x14)).sw(c, c.g[V1][0]);
        if !null {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0039_bba0);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, target, 0x0039_bba0)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !null {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0039_bbc0);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, target, 0x0039_bbc0)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIProgress_dtor(cUIProgress* self, int flags)` (0x39c240): release
/// the two child objects and run the base destructor.
pub fn c_ui_progress_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0049_4408);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x74)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !null {
            c.cycles += 6;
            c.g[A1][0] = 0x3;
            delete_owner(c, 8, 0x0039_c284)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x78)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !null {
            c.cycles += 5;
            delete_owner(c, 8, 0x0039_c2a4)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0039_c2b0);
        c.g[A1][0] = c.g[S1][0];
        callee::UI_OBJECT_DTOR.call(c, 0x0039_c2b0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenEnterLodge_OnActivate(OVScreenEnterLodge* self, int)` (0x1f7210):
/// clear the "disabled" flag of the lodge button, set the "enabled" flag of
/// the leave button and activate the state.
pub fn ov_screen_enter_lodge_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x4a_0000;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x001f_7234);
        c.g[A0][0] = sx(0x004a_23e0);
        call_ptr(c, 0x0031_7670, 0x001f_7234)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        c.g[RA][0] = sx(0x001f_7240);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_7240)?;
        c.g[V1][0] = c.g[V0][0];
        if c.g[V1][0] != 0 {
            Addr(p(r32(c, V1), 0x18)).sw(c, 0);
        }
        c.cycles += 9;
        c.g[A0][0] = 0x4a_0000;
        c.g[RA][0] = sx(0x001f_7258);
        c.g[A0][0] = sx(0x004a_23e8);
        call_ptr(c, 0x0031_7670, 0x001f_7258)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        c.g[RA][0] = sx(0x001f_7264);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_7264)?;
        c.g[V1][0] = c.g[V0][0];
        let null = c.g[V1][0] == 0;
        c.g[V0][0] = 0x1;
        if !null {
            c.cycles += 1;
            Addr(p(r32(c, V1), 0x18)).sw(c, c.g[V0][0]);
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001f_7280);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e4c0, 0x001f_7280)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMemCard_RestartScan(cFEStateMemCard* self)` (0x1871a8):
/// rewind the memory-card manager, close its slots, start the card state's
/// own scan and switch the UI state.
pub fn c_fe_state_mem_card_restart_scan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0018_71c0);
        call_ptr(c, 0x0022_7f80, 0x0018_71c0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x0018_71d0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0023_fb18, 0x0018_71d0)?;
        c.g[RA][0] = sx(0x0018_71d8);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0023_fae0, 0x0018_71d8)?;
        c.g[V0][0] = Addr(p(r32(c, S0), 0x748)).lw(c);
        c.g[A1][0] = 0x36;
        c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x0018_71f0);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, target, 0x0018_71f0)?;
        c.g[A2][0] = Addr(gp(c, -0x1c38)).lw(c);
        c.g[V0][0] = u64::MAX;
        Addr(gp(c, -0x1c34)).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        Addr(p(r32(c, A2), 0x22c)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x2;
        c.g[RA][0] = sx(0x0018_7210);
        Addr(p(r32(c, A2), 0x234)).sw(c, 0);
        call_ptr(c, 0x0023_c8f0, 0x0018_7210)?;
        c.g[V1][0] = Addr(gp(c, -0x1c38)).lw(c);
        c.g[V0][0] = 0x2;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(p(r32(c, V1), 0x1bc)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderModel_ShowPartVariant(RiderModel* self, int part, void* value)`
/// (0x30ec18): store `value` in the part's variant slot, OR the part's mask
/// bit and mark the three variant words.
pub fn rider_model_show_part_variant(c: &mut Cpu) {
    c.cycles += 6;
    c.g[T0][0] = c.g[A0][0];
    c.g[V1][0] = c.g[A1][0];
    c.g[V0][0] = Addr(p(r32(c, T0), 0x14)).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[A1][0] = Addr(p(r32(c, T0), 0x1c)).lw(c);
    c.g[V0][0] = sx(r32(c, V1) << 2);
    if c.g[A1][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[V0][0] = sx(p(r32(c, V0), r32(c, A1)));
    c.g[A1][0] = Addr(r32(c, V0)).lw(c);
    c.g[V0][0] = 0x58;
    if (c.g[A1][0] as i64) < 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 11;
    c.g[V1][0] = Addr(p(r32(c, T0), 0xc)).lw(c);
    c.mult(0, 2, 5, 2);
    c.g[A3][0] = 0x3;
    c.g[A1][0] = 0x1;
    c.g[V1][0] = sx(p(r32(c, V1), r32(c, V0)));
    Addr(p(r32(c, V1), 0x50)).sw(c, c.g[A2][0]);
    c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).ld(c);
    c.g[A2][0] = sx(p(r32(c, V1), 0x30));
    c.g[V0][0] = Addr(p(r32(c, T0), 0x150)).ld(c);
    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
    Addr(p(r32(c, T0), 0x150)).sd(c, c.g[V0][0]);
    loop {
        c.cycles += 7;
        Addr(r32(c, A2)).sw(c, c.g[A1][0]);
        c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
        let more = (c.g[A3][0] as i64) >= 0;
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(8));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0030_ec78);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}
// ---------------------------------------------------------------------------
// Camera controllers.

/// `void ManualCamController_dtor(ManualCamController* self, int flags)`
/// (0x15ce10): release the two camera controllers and run the base
/// destructor.
pub fn manual_cam_controller_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x46_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0045_b8d8);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 0x14)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x18)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !null {
            c.cycles += 6;
            c.g[A1][0] = 0x3;
            delete_owner(c, 0x10, 0x0015_ce54)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !null {
            c.cycles += 5;
            delete_owner(c, 0x10, 0x0015_ce74)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0015_ce80);
        c.g[A1][0] = c.g[S1][0];
        callee::CAM_CONTROLLER_DTOR.call(c, 0x0015_ce80)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Animation maps.

/// `void GameAnimMap_UnregisterBank(GameAnimMap* self, AnimBank* bank)`
/// (0x314fe8): clear every handle the bank registered, then forget the bank.
pub fn game_anim_map_unregister_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r32(c, S1), 4)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 2)).lh(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = c.g[A0][0];
        if !empty {
            c.cycles += 2;
            c.g[S2][0] = 0;
            c.g[V0][0] = Addr(p(r32(c, S1), 8)).lw(c);
            loop {
                c.cycles += 12;
                c.g[A0][0] = c.g[S3][0];
                c.g[A2][0] = Addr(gp(c, 0x2860)).lw(c);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V0][0] = sx(p(r32(c, S2), r32(c, V0)));
                c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                c.g[RA][0] = sx(0x0031_503c);
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x14));
                callee::GAME_ANIM_MAP_SET_HANDLE.call(c, 0x0031_503c)?;
                c.g[V1][0] = Addr(p(r32(c, S1), 4)).lw(c);
                c.g[V0][0] = Addr(p(r32(c, V1), 2)).lh(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(p(r32(c, S1), 8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0031_5020);
                }
            }
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

// ---------------------------------------------------------------------------
// Camera triggers.

/// `CameraTriggerMan* CameraTriggerMan_ctor(CameraTriggerMan* self)`
/// (0x16c800): construct the trigger list, the two active-trigger lists and
/// the two trigger stacks.
pub fn camera_trigger_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0016_c828);
        c.g[S0][0] = sx(p(r32(c, S2), 0x14));
        callee::CAMERA_TRIGGER_LIST_CTOR.call(c, 0x0016_c828)?;
        c.g[S3][0] = u64::MAX;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0016_c83c);
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR.call(c, 0x0016_c83c)?;
            let more = c.g[S1][0] != c.g[S3][0];
            c.g[S0][0] = sx(p(r32(c, S0), 4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0016_c830);
            }
        }
        c.cycles += 3;
        c.g[S0][0] = sx(p(r32(c, S2), 0x1c));
        c.g[S1][0] = 0x1;
        c.g[S3][0] = u64::MAX;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0016_c85c);
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            callee::CAMERA_TRIGGER_STACK_CTOR.call(c, 0x0016_c85c)?;
            let more = c.g[S1][0] != c.g[S3][0];
            c.g[S0][0] = sx(p(r32(c, S0), 0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0016_c850);
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end async manager.

/// `void FEAsyncManager_FreeAll(FEAsyncManager* self)` (0x1a3c50): release the
/// five file slots.
pub fn fe_async_manager_free_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(p(r32(c, A0), 0x110));
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0x4;
        frame.save_ra(c, 0);
        c.g[S2][0] = u64::MAX;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0xffff_fff0)).lw(c);
            let no_data = c.g[V0][0] == 0;
            c.g[A2][0] = Addr(gp(c, -0x854)).lw(c);
            if !no_data {
                c.cycles += 9;
                c.g[A1][0] = Addr(r32(c, S0)).lw(c);
                c.g[V0][0] = Addr(p(r32(c, A2), 0x10d8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V0), 0x190)).lh(c);
                c.g[V1][0] = Addr(p(r32(c, V0), 0x194)).lw(c);
                let target = r32(c, V1);
                c.g[RA][0] = sx(0x001a_3c94);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, target, 0x001a_3c94)?;
                c.g[A0][0] = Addr(p(r32(c, S0), 0xffff_fff0)).lw(c);
                if c.g[A0][0] != 0 {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001a_3ca8);
                    call_ptr(c, 0x0031_7e98, 0x001a_3ca8)?;
                }
                Addr(p(r32(c, S0), 0xffff_fff0)).sw(c, 0);
                c.cycles += 2;
                Addr(p(r32(c, S0), 8)).sw(c, 0);
                Addr(r32(c, S0)).sw(c, c.g[S2][0]);
            }
            c.cycles += 3;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(p(r32(c, S0), 0x11c));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_3c70);
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
// Network configuration popup.

/// `int NetConfigAuthPopup_HandleInput(NetConfigAuthPopup* self, void* obj)`
/// (0x1b66b8): when the popup waits for input (state 2), start the transfer
/// as soon as the object reports ready and advance to state 4; always
/// returns 1.
pub fn net_config_auth_popup_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x2;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = Addr(p(r32(c, S1), 0x6e4)).lw(c);
        let waiting = c.g[V1][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A1][0];
        let mut plus_one = true;
        if waiting {
            c.cycles += 7;
            c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x28)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x2c)).lw(c);
            let target = r32(c, V0);
            c.g[RA][0] = sx(0x001b_66f0);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, target, 0x001b_66f0)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(p(r32(c, S1), 0x6d0)).lw(c);
            } else {
                c.cycles += 7;
                c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V1), 0x30)).lh(c);
                c.g[V0][0] = Addr(p(r32(c, V1), 0x34)).lw(c);
                let target = r32(c, V0);
                c.g[RA][0] = sx(0x001b_670c);
                c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
                call_ptr(c, target, 0x001b_670c)?;
                if c.g[V0][0] == 0 {
                    plus_one = false;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(r32(c, S1), 0x6d0)).lw(c);
                }
            }
            if plus_one {
                c.cycles += 4;
                c.g[V1][0] = 0x1;
                c.g[A0][0] = 0x4;
                Addr(p(r32(c, V0), 0x14)).sw(c, c.g[V1][0]);
                Addr(p(r32(c, S1), 0x6e4)).sw(c, c.g[A0][0]);
            }
        }
        if plus_one {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        c.g[V0][0] = 0x1;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
// ---------------------------------------------------------------------------
// Front-end network states.

/// `void cFEStateCreateStep1_Init(cFEStateCreateStep1* self)` (0x1b8960):
/// pick the create-screen step by the "net mode" word, then look the screen up
/// and start it.
pub fn c_fe_state_create_step1_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S1), 0x6ec)).lw(c);
        let first = c.g[V0][0] == 0;
        c.g[V0][0] = Addr(gp(c, -0xc8)).lw(c);
        if first {
            c.cycles += 1;
            c.g[V0][0] = sx(p(r32(c, V0), 0x2b0));
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx(p(r32(c, V0), 0x260));
        }
        c.cycles += 12;
        Addr(p(r32(c, S1), 0x6d0)).sw(c, c.g[V0][0]);
        c.g[A0][0] = 0x46_0000;
        c.g[S0][0] = Addr(p(r32(c, S1), 0x10)).lw(c);
        c.g[RA][0] = sx(0x001b_89a0);
        c.g[A0][0] = sx(0x0046_4740);
        call_ptr(c, 0x0031_7670, 0x001b_89a0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001b_89b4);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001b_89b4)?;
        let missing = c.g[V0][0] == 0;
        Addr(p(r32(c, S1), 0x40)).sw(c, c.g[V0][0]);
        if !missing {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001b_89cc);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001b_89cc)?;
        }
        c.cycles += 7;
        c.g[RA][0] = sx(0x001b_89d4);
        c.g[A0][0] = c.g[S1][0];
        callee::NET_FE_STATE_INIT.call(c, 0x001b_89d4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCreateStep1_DetectChanges(cFEStateCreateStep1* self)`
/// (0x1ba358): compare the two account names and ids; set the matching
/// changed-flags.
pub fn c_fe_state_create_step1_detect_changes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x6d0)).lw(c);
        c.g[A1][0] = Addr(p(r32(c, S0), 0x6d4)).lw(c);
        c.g[RA][0] = sx(0x001b_a378);
        c.g[A0][0] = Addr(p(r32(c, V0), 4)).lw(c);
        call_ptr(c, 0x0041_65a8, 0x001b_a378)?;
        let same = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        if !same {
            c.cycles += 2;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x6d0)).lw(c);
            Addr(p(r32(c, V1), 0x44)).sw(c, c.g[V0][0]);
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(p(r32(c, S0), 0x6d0)).lw(c);
        c.g[A1][0] = Addr(p(r32(c, S0), 0x6dc)).lw(c);
        c.g[RA][0] = sx(0x001b_a398);
        c.g[A0][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
        call_ptr(c, 0x0041_65a8, 0x001b_a398)?;
        let same = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        if !same {
            c.cycles += 2;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x6d0)).lw(c);
            Addr(p(r32(c, V1), 0x4c)).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[A1][0] = Addr(p(r32(c, S0), 0x6d0)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, S0), 0x6e0)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, A1), 0x38)).lw(c);
        let differs = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0x1;
        if !differs {
            c.cycles += 4;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x6e4)).lw(c);
            c.g[V0][0] = Addr(p(r32(c, A1), 0x3c)).lw(c);
            if c.g[V0][0] != c.g[V1][0] {
                c.g[V0][0] = 0x1;
                c.cycles += 1;
                Addr(p(r32(c, A1), 0x48)).sw(c, c.g[V0][0]);
            }
        } else {
            c.cycles += 1;
            Addr(p(r32(c, A1), 0x48)).sw(c, c.g[V0][0]);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCreateDuplicate_OnChildEvent(cFEStateCreateDuplicate* self,
/// ChildEvent* ev, int, int)` (0x1bc6a0): dispatch the child's event word to
/// the account, error, created-callbacks.
pub fn c_fe_state_create_duplicate_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x81f;
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r32(c, A1), 0x18)).lw(c);
        let is_account = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[V1][0] < 0x820) as u64;
        if is_account {
            c.cycles += 4;
            c.g[RA][0] = sx(0x001b_c6ec);
            callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER.call(c, 0x001b_c6ec)?;
        } else {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = 0x82c;
                let is_created = c.g[V1][0] == c.g[V0][0];
                if is_created {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001b_c70c);
                    callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT.call(c, 0x001b_c70c)?;
                } else {
                    c.cycles += 2;
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001b_c71c);
                    callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001b_c71c)?;
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x80e;
                let is_error = c.g[V1][0] == c.g[V0][0];
                if is_error {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001b_c6fc);
                    callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT.call(c, 0x001b_c6fc)?;
                } else {
                    c.cycles += 2;
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001b_c71c);
                    callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001b_c71c)?;
                }
            }
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        let target = r32(c, RA);
        c.g[SP][0] = sx(p(r32(c, SP), 0x10));
        c.pc = target;
        Ok(())
    })
}

/// `int cFEStateLobbyMenu_OnWidgetNotify(cFEState* self, void* obj, int msg,
/// int)` (0x1bdc18): apply lobby-room updates for the two child kinds and
/// always returns 0x101.
pub fn c_fe_state_lobby_menu_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S1), 0x6d4)).lw(c);
        if c.g[S0][0] != c.g[V0][0] {
            c.g[V0][0] = Addr(p(r32(c, S1), 0x6d0)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x5;
            let wrong = c.g[A2][0] != c.g[V0][0];
            c.g[V0][0] = 0x101;
            if !wrong {
                c.cycles += 7;
                c.g[RA][0] = sx(0x001b_dc50);
                c.g[A1][0] = Addr(p(r32(c, S0), 0xa0)).lw(c);
                callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB.call(c, 0x001b_dc50)?;
                c.g[A1][0] = Addr(p(r32(c, S0), 0xa0)).lw(c);
                c.g[RA][0] = sx(0x001b_dc5c);
                c.g[A0][0] = c.g[S1][0];
                callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM.call(c, 0x001b_dc5c)?;
                c.g[V0][0] = 0x101;
            }
        }
        c.cycles += 2;
        let other = c.g[S0][0] != c.g[V0][0];
        c.g[V0][0] = 0x101;
        if !other {
            c.cycles += 3;
            c.g[V0][0] = 0x4;
            let wrong = c.g[A2][0] != c.g[V0][0];
            c.g[V0][0] = 0x101;
            if !wrong {
                c.cycles += 5;
                Addr(p(r32(c, S1), 0x778)).sw(c, c.g[A3][0]);
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001b_dc88);
                c.g[A1][0] = Addr(p(r32(c, A0), 0x774)).lw(c);
                callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST.call(c, 0x001b_dc88)?;
                c.g[V0][0] = 0x101;
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateOnlineAgreement_Update(cFEStateOnlineAgreement* self)`
/// (0x1c0a80): while the agreement has queued ids, look the text object up
/// and set the next id; then update the base state.
pub fn c_fe_state_online_agreement_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x6d8)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, S0), 0x6d4)).lw(c);
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x6d0)).lw(c);
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = 0x46_0000;
            if !none {
                c.cycles += 8;
                c.g[RA][0] = sx(0x001c_0ab8);
                c.g[A0][0] = sx(0x0046_5710);
                call_ptr(c, 0x0031_7670, 0x001c_0ab8)?;
                c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
                c.g[RA][0] = sx(0x001c_0ac4);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001c_0ac4)?;
                c.g[A0][0] = c.g[V0][0];
                if c.g[A0][0] != 0 {
                    c.cycles += 8;
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x6d8)).lw(c);
                    c.g[A1][0] = Addr(p(r32(c, S0), 0x6d0)).lw(c);
                    c.g[V1][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                    c.g[V1][0] = sx(p(r32(c, V1), r32(c, A1)));
                    Addr(p(r32(c, S0), 0x6d8)).sw(c, c.g[V0][0]);
                    c.g[RA][0] = sx(0x001c_0af0);
                    c.g[A1][0] = Addr(r32(c, V1)).lw(c);
                    call_ptr(c, 0x003a_1f18, 0x001c_0af0)?;
                }
            }
        }
        c.cycles += 6;
        c.g[RA][0] = sx(0x001c_0af8);
        c.g[A0][0] = c.g[S0][0];
        callee::NET_FE_STATE_UPDATE.call(c, 0x001c_0af8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEPopupSelectMultiplayerMode_OnWidgetNotify(Popup* self, void* menu,
/// int key, int idx)` (0x1e0470): move the menu selection by `idx` for the
/// arrow keys; always returns 0x101.
pub fn c_fe_popup_select_multiplayer_mode_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x3;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        let down = c.g[A2][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        let mut skip_e4 = false;
        if down {
            select_next(c)?;
            c.cycles += 1;
        } else if c.g[A2][0] >= 0x4 {
            c.cycles += 3;
            if c.g[A2][0] < 0x8 {
                c.cycles += 3;
                c.cycles += 1;
            } else {
                c.cycles += 3;
                c.cycles += 2;
            }
        } else {
            c.cycles += 3;
            c.cycles += 2;
            c.g[V0][0] = 0x101;
            if c.g[A2][0] == 0x2 {
                select_prev(c)?;
                select_next(c)?;
                c.cycles += 1;
            } else {
                c.cycles += 2;
                skip_e4 = true;
            }
        }
        if !skip_e4 {
            c.cycles += 1;
        }
        c.g[V0][0] = 0x101;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `setSelectedByIndex(menu, idx - 1)` block of
/// [`c_fe_popup_select_multiplayer_mode_on_widget_notify`].
fn select_prev(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A1][0] = sx(r32(c, S0).wrapping_sub(1));
    c.g[A0][0] = c.g[S1][0];
    c.g[RA][0] = sx(0x001e_04d0);
    c.g[A1][0] = c.g[A1][0] & 0xff;
    call_ptr(c, 0x0039_b960, 0x001e_04d0)
}

/// The `setSelectedByIndex(menu, idx + 1)` block of
/// [`c_fe_popup_select_multiplayer_mode_on_widget_notify`].
fn select_next(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A1][0] = sx(r32(c, S0).wrapping_add(1));
    c.g[A0][0] = c.g[S1][0];
    c.g[RA][0] = sx(0x001e_04e0);
    c.g[A1][0] = c.g[A1][0] & 0xff;
    call_ptr(c, 0x0039_b960, 0x001e_04e0)
}

/// `void MessageCenter_TriggerGroups(MessageCenter* self, int type)`
/// (0x1e14c0): empty every trigger the group table has for `type`.
pub fn message_center_trigger_groups(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[V0][0] = 0x44_0000;
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = sx(0x0044_1630);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = 0x5;
        loop {
            c.cycles += 4;
            c.g[RA][0] = sx(0x001e_14f8);
            c.g[A0][0] = c.g[S0][0];
            callee::MESSAGE_CENTER_IS_FLAG_SET.call(c, 0x001e_14f8)?;
            let mut skip = false;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                if c.g[V0][0] != c.g[S2][0] {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        skip = true;
                    }
                }
                if !skip {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001e_151c);
                    c.g[A0][0] = c.g[S0][0];
                    callee::MESSAGE_CENTER_EMPTY_TRIGGER.call(c, 0x001e_151c)?;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 61) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x14));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001e_14f0);
            }
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
/// `void LobbyMan_ChalRequestJoin(LobbyMan* self, const char* name)`
/// (0x25fad0): build the challenge tag record and send a join request.
pub fn lobby_man_chal_request_join(c: &mut Cpu) {
    guest(c, |c| lobby_chal_request(c, 0x004a_3250, 0x18, [0x0025_fafc, 0x0025_fb18, 0x0025_fb48]));
}

/// `void LobbyMan_ChalRequestHost(LobbyMan* self, const char* name)`
/// (0x25fa48): build the challenge tag record and send a host request.
pub fn lobby_man_chal_request_host(c: &mut Cpu) {
    guest(c, |c| lobby_chal_request(c, 0x004a_3240, 0x15, [0x0025_fa74, 0x0025_fa90, 0x0025_fac0]));
}

/// Shared body of the two challenge requests: a 0x100-byte tag buffer with
/// the name and the "chal" field, then `LobbyMan_SendRequest(self, kind,
/// 0x6368616c, buf, 20.0f)`.
fn lobby_chal_request(c: &mut Cpu, tag: u32, kind: u64, rets: [u32; 3]) -> Flow {
    c.cycles += 34;
    let frame = Frame::open(c, 0x120);
    c.g[A3][0] = c.g[A1][0];
    frame.save(c, S0, 0x110);
    c.g[A2][0] = 0x4a_0000;
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0x100);
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = sx(0x004a_3108);
    c.sb(r32(c, SP), 0);
    c.g[RA][0] = sx(rets[0]);
    c.g[A1][0] = 0x100;
    call_ptr(c, 0x003e_c180, rets[0])?;
    c.g[A2][0] = 0x4a_0000;
    c.g[A3][0] = 0x4a_0000;
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = sx(0x004a_2f68);
    c.g[A3][0] = sx(tag);
    c.g[RA][0] = sx(rets[1]);
    c.g[A1][0] = 0x100;
    call_ptr(c, 0x003e_c180, rets[1])?;
    c.g[V1][0] = 0x1;
    c.g[T0][0] = 0x27_0000;
    c.g[A2][0] = 0x6368_0000;
    c.g[AT][0] = 0x41a0_0000;
    c.mtc1(1, 12);
    Addr(r32(c, S0)).sw(c, c.g[V1][0]);
    c.g[A0][0] = c.g[S0][0];
    c.g[T0][0] = sx(0x0026_8ea0);
    c.g[A1][0] = kind;
    c.g[A2][0] = c.g[A2][0] | 0x616c;
    c.g[RA][0] = sx(rets[2]);
    c.g[A3][0] = c.g[SP][0];
    callee::LOBBY_MAN_SEND_REQUEST.call(c, rets[2])?;
    frame.restore(c, S0, 0x110);
    frame.restore_ra(c, 0x100);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Buddy manager.

/// `void BuddyMan_DeleteMessageAt(BuddyMan* self, int index)` (0x2635a8):
/// remove the message at `index` from the ring (moving the tail) and release
/// it.
pub fn buddy_man_delete_message_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx(r32(c, A1) << 2);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = sx(p(r32(c, S0), 0xa8));
        c.g[V0][0] = Addr(p(r32(c, S0), 0xac)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, S0), 0xb0)).lw(c);
        c.g[A0][0] = sx(p(r32(c, V0), r32(c, A1)));
        c.g[V0][0] = sx(p(r32(c, A0), 4));
        let last = c.g[V0][0] == c.g[V1][0];
        c.g[S1][0] = Addr(r32(c, A0)).lw(c);
        if !last {
            c.cycles += 6;
            c.g[A2][0] = Addr(p(r32(c, A2), 8)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, V0)));
            c.g[A2][0] = sx(((r32(c, A2) as i32) >> 2) as u32);
            c.g[RA][0] = sx(0x0026_35f4);
            c.g[A2][0] = sx(r32(c, A2) << 2);
            call_ptr(c, 0x0041_610c, 0x0026_35f4)?;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(p(r32(c, S0), 0xb0)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        let empty = c.g[S1][0] == 0;
        Addr(p(r32(c, S0), 0xb0)).sw(c, c.g[V0][0]);
        if !empty {
            c.cycles += 6;
            c.g[V0][0] = Addr(p(r32(c, S1), 0xc)).lw(c);
            c.g[A1][0] = 0x3;
            c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0026_361c);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, target, 0x0026_361c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// INI parser.

const INI_FIND_ENTRY: u32 = 0x002a_7c68;
const INI_FIND_END: u32 = 0x002a_7cf0;
/// Resumed after an event at the section walk's back edge.
const INI_FIND_SECTION: u32 = 0x002a_7c88;

/// `int IniParser_FindSectionPrefix(IniParser* self, const char* prefix)`
/// (0x2a7c68): walk the sections and return 1 when one is a prefix of the
/// other's name.
pub fn ini_parser_find_section_prefix(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                INI_FIND_ENTRY => {
                    c.cycles += 8;
                    let frame = Frame::open(c, 0x40);
                    frame.save(c, S1, 0x20);
                    frame.save(c, S2, 0x10);
                    c.g[S1][0] = c.g[A1][0];
                    frame.save(c, S0, 0x30);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(r32(c, S2), 8)).lw(c);
                    pc = 0x002a_7cb4;
                }
                0x002a_7cb4 => {
                    c.cycles += 3;
                    c.g[V1][0] = Addr(r32(c, V0)).lb(c);
                    let empty = c.g[V1][0] == 0;
                    c.g[V0][0] = 0;
                    if empty {
                        pc = 0x002a_7cd8;
                    } else {
                        pc = 0x002a_7cc0;
                    }
                }
                0x002a_7cc0 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x002a_7cc8);
                    c.g[A0][0] = c.g[S2][0];
                    callee::INI_PARSER_NEXT_SECTION.call(c, 0x002a_7cc8)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] != 0 {
                        c.g[A0][0] = c.g[S1][0];
                        if event_due(c) {
                            c.pc = INI_FIND_SECTION;
                            if c.event() {
                                return Err(crate::call::Unwound);
                            }
                        }
                        pc = INI_FIND_SECTION;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                        pc = 0x002a_7cd8;
                    }
                }
                INI_FIND_SECTION => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002a_7c90);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041_ad80, 0x002a_7c90)?;
                    let found = c.g[V0][0] == c.g[S1][0];
                    c.g[A0][0] = c.g[S0][0];
                    if found {
                        c.cycles += 2;
                        c.g[V0][0] = 0x1;
                        pc = 0x002a_7cd8;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x002a_7ca0);
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0041_ad80, 0x002a_7ca0)?;
                        if c.g[V0][0] != c.g[S0][0] {
                            c.g[V0][0] = Addr(p(r32(c, S2), 8)).lw(c);
                            pc = 0x002a_7cb4;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = 0x1;
                            pc = 0x002a_7cd8;
                        }
                    }
                }
                0x002a_7cd8 => {
                    c.cycles += 6;
                    frame_restore_ini(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, INI_FIND_ENTRY, INI_FIND_END);
                    return Ok(());
                }
            }
        }
    })
}

/// The shared epilogue of [`ini_parser_find_section_prefix`].
fn frame_restore_ini(c: &mut Cpu) {
    let frame = Frame { size: 0x40 };
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Script tables and objects.

/// `ScriptObjTable* ScriptEngine_ObjTableCtor(ScriptObjTable* self)`
/// (0x27cdd0): publish the vtable and clear the 20 free slots and the 30
/// 5-word records.
pub fn script_engine_obj_table_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x48_0000;
    c.g[V1][0] = 0x13;
    c.g[V0][0] = sx(0x0048_2528);
    Addr(p(r32(c, A0), 0x2a8)).sw(c, c.g[V0][0]);
    Addr(p(r32(c, GP), 0x3ec)).sw(c, c.g[A0][0]);
    c.g[V0][0] = sx(p(r32(c, A0), 0x4c));
    loop {
        c.cycles += 7;
        Addr(r32(c, V0)).sw(c, 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0027_cde8);
        }
    }
    c.cycles += 7;
    c.g[T1][0] = u64::MAX;
    c.g[V1][0] = sx(p(r32(c, A0), 0x60));
    c.g[T0][0] = sx(p(r32(c, A0), 0x5c));
    c.g[A3][0] = sx(p(r32(c, A0), 0x58));
    c.g[A2][0] = sx(p(r32(c, A0), 0x54));
    c.g[A1][0] = sx(p(r32(c, A0), 0x50));
    c.g[T2][0] = sx(p(r32(c, A0), 0x2b8));
    loop {
        c.cycles += 12;
        Addr(r32(c, A1)).sw(c, 0);
        Addr(r32(c, A2)).sw(c, c.g[T1][0]);
        c.g[A1][0] = sx(p(r32(c, A1), 0x14));
        Addr(r32(c, A3)).sw(c, c.g[T1][0]);
        c.g[A2][0] = sx(p(r32(c, A2), 0x14));
        Addr(r32(c, T0)).sw(c, 0);
        c.g[A3][0] = sx(p(r32(c, A3), 0x14));
        Addr(r32(c, V1)).sw(c, c.g[T1][0]);
        c.g[V1][0] = sx(p(r32(c, V1), 0x14));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T2][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[T0][0] = sx(p(r32(c, T0), 0x14));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0027_ce20);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `void ScriptObjCam_Activate(ScriptObjCam* self)` (0x27d850): activate the
/// script object and, when the world is running, build the matrix and start
/// the camera controller.
pub fn script_obj_cam_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0027_d864);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_OBJ_ACTIVATE.call(c, 0x0027_d864)?;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x84)).lw(c);
        let no_world = c.g[V1][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_world {
            c.cycles += 19;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[A1][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
            c.g[RA][0] = sx(0x0027_d888);
            c.swc1(0, p(r32(c, S0), 0x18));
            callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT.call(c, 0x0027_d888)?;
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[V1][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x84)).lw(c);
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = Addr(p(r32(c, A0), 0x84)).lw(c);
            c.g[V0][0] = sx(p(r32(c, V0), r32(c, V1)));
            c.g[S0][0] = Addr(p(r32(c, V0), 4)).lw(c);
            c.g[RA][0] = sx(0x0027_d8b0);
            c.g[A0][0] = Addr(p(r32(c, S0), 0xac)).lw(c);
            call_ptr(c, 0x0016_9190, 0x0027_d8b0)?;
            c.g[V0][0] = Addr(p(r32(c, S0), 0xac)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x18)).lw(c);
            let no_controller = c.g[V1][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !no_controller {
                c.cycles += 2;
                c.g[RA][0] = sx(0x0027_d8c8);
                c.g[A1][0] = 0x3;
                call_ptr(c, 0x0015_dfd8, 0x0027_d8c8)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptAudioObj* ScriptAudioObj_ctor(ScriptAudioObj* self)` (0x2805b8):
/// the binding/object constructors, the audio vtable and the four output
/// records.
pub fn script_audio_obj_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0028_05cc);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_BINDING_CTOR.call(c, 0x0028_05cc)?;
        c.g[A0][0] = sx(p(r32(c, S0), 0xc));
        c.g[RA][0] = sx(0x0028_05d8);
        c.g[A1][0] = 0x5;
        callee::SCRIPT_OBJ_CTOR.call(c, 0x0028_05d8)?;
        c.g[V0][0] = 0x48_0000;
        c.g[V1][0] = 0x48_0000;
        c.g[V0][0] = sx(0x0048_2298);
        c.g[V1][0] = sx(0x0048_2260);
        Addr(p(r32(c, S0), 0x18)).sw(c, c.g[V0][0]);
        c.g[A0][0] = u64::MAX;
        Addr(r32(c, S0)).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx(p(r32(c, S0), 0xbc));
        Addr(p(r32(c, S0), 0xec)).sw(c, 0);
        c.g[V1][0] = 0x3;
        Addr(p(r32(c, S0), 0xf0)).sw(c, 0);
        Addr(p(r32(c, S0), 0xf4)).sw(c, 0);
        Addr(p(r32(c, S0), 0xf8)).sw(c, 0);
        loop {
            c.cycles += 7;
            Addr(p(r32(c, V0), 8)).sw(c, c.g[A0][0]);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            Addr(r32(c, V0)).sw(c, c.g[A0][0]);
            Addr(p(r32(c, V0), 4)).sw(c, 0);
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(p(r32(c, V0), 0xc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_0610);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio.

/// `int PATH_PlatSetTrackLevel(int track, int level)` (0x2af580): lock the
/// sound system, set the level on the track's sound, and return the previous
/// lock depth.
pub fn path_plat_set_track_level(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x4d_0000;
        frame.save(c, S1, 0x20);
        c.g[A0][0] = sx(r32(c, A0) << 2);
        frame.save(c, S2, 0x10);
        c.g[V0][0] = sx(0x004d_3e98);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(r32(c, A0), r32(c, V0)));
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        c.g[S0][0] = Addr(r32(c, A0)).lw(c);
        let null = c.g[S0][0] == 0;
        c.g[S1][0] = 0xffff_ffff_ffff_fff8;
        if !null {
            c.cycles += 5;
            c.g[RA][0] = sx(0x002a_f5bc);
            call_ptr(c, 0x003b_58a0, 0x002a_f5bc)?;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x64)).lb(c);
            let streamed = c.g[V0][0] == 0;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            if streamed {
                c.cycles += 2;
                c.g[RA][0] = sx(0x002a_f5e0);
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x003b_8010, 0x002a_f5e0)?;
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002a_f5d0);
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x003b_8700, 0x002a_f5d0)?;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x002a_f5e8);
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x003b_58d8, 0x002a_f5e8)?;
            Addr(p(r32(c, S0), 0x14)).sw(c, c.g[S2][0]);
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// In-sequence channels.

/// `void* InSeq_ChannelSetListener(InSeqChannel* self, void* listener)`
/// (0x274240): release the previous listener, store the new one and hand it
/// its owner; returns the previous listener.
pub fn in_seq_channel_set_listener(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = Addr(p(r32(c, S0), 0x2c)).lw(c);
        let null = c.g[S2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !null {
            c.cycles += 8;
            c.g[V0][0] = Addr(r32(c, S2)).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[V1][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lh(c);
            c.g[A3][0] = Addr(p(r32(c, V0), 0x14)).lw(c);
            c.g[A2][0] = Addr(p(r32(c, V1), 3)).lbu(c);
            let target = r32(c, A3);
            c.g[RA][0] = sx(0x0027_4284);
            c.g[A0][0] = sx(p(r32(c, S2), r32(c, A0)));
            call_ptr(c, target, 0x0027_4284)?;
        }
        c.cycles += 2;
        let none = c.g[S1][0] == 0;
        Addr(p(r32(c, S0), 0x2c)).sw(c, c.g[S1][0]);
        if !none {
            c.cycles += 8;
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[V1][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
            c.g[A3][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            c.g[A2][0] = Addr(p(r32(c, V1), 3)).lbu(c);
            let target = r32(c, A3);
            c.g[RA][0] = sx(0x0027_42ac);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, target, 0x0027_42ac)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory card.

/// `void cPS2MemCard_Close(cPS2MemCard* self)` (0x2c6360): finish the pending
/// card request (if any) and clear the card state.
pub fn c_ps2_mem_card_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        let mut skip = false;
        if !idle {
            c.cycles += 8;
            Addr(gp(c, 0x850)).sw(c, 0);
            c.g[A2][0] = sx(gp(c, 0x850));
            c.g[RA][0] = sx(0x002c_638c);
            c.g[A1][0] = 0;
            call_ptr(c, 0x0040_a360, 0x002c_638c)?;
            c.g[V1][0] = Addr(gp(c, 0x850)).lw(c);
            c.g[V0][0] = u64::MAX;
            if c.g[V1][0] != c.g[V0][0] {
                Addr(p(r32(c, S0), 0x40)).sw(c, 0);
                skip = true;
            } else {
                c.cycles += 6;
                c.g[V0][0] = Addr(p(r32(c, S0), 0xc)).lw(c);
                c.g[V1][0] = 0x14;
                c.g[A0][0] = 0x1;
                c.mult(0, 5, 2, 3);
                c.g[V0][0] = sx(p(r32(c, A1), r32(c, S0)));
                Addr(p(r32(c, V0), 0x18c)).sw(c, c.g[A0][0]);
            }
        }
        if !skip {
            c.cycles += 1;
            Addr(p(r32(c, S0), 0x40)).sw(c, 0);
        }
        c.cycles += 12;
        c.g[A0][0] = c.g[S0][0];
        Addr(p(r32(c, S0), 0xc)).sw(c, 0);
        Addr(p(r32(c, S0), 0x17c)).sw(c, 0);
        Addr(p(r32(c, S0), 0x180)).sw(c, 0);
        Addr(p(r32(c, S0), 0x184)).sw(c, 0);
        Addr(p(r32(c, S0), 0x188)).sw(c, 0);
        c.g[RA][0] = sx(0x002c_63d8);
        Addr(p(r32(c, S0), 0x3c)).sw(c, 0);
        callee::PS2_MEM_CARD_CLEAR_DIR_CACHE.call(c, 0x002c_63d8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Chase-camera algorithms.

/// The `initFromRider` arguments of the three camera reset wrappers.
#[derive(Clone, Copy)]
enum InitArgs {
    /// `f12 = 100.0f`, `f13 = f12`.
    Hundred,
    /// `lwc1 f12, gp+off`, `f13 = 300.0f`.
    GpAnd300(i32),
}

/// Shared body of the three `*_ResetForRider` wrappers: if the rider's camera
/// target id matches, reset the chase algorithm for the rider and initialise
/// it, then notify the camera controller.
fn chase_camera_reset_for_rider(c: &mut Cpu, rets: [u32; 4], args: InitArgs) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x30)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 4)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, A0), 0x6c0)).lw(c);
        c.g[A0][0] = sx(p(r32(c, A0), 0x6c0));
        c.g[A1][0] = Addr(p(r32(c, V1), 0x38)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x3c)).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(rets[0]);
        c.g[A0][0] = sx(p(r32(c, A0), r32(c, A1)));
        call_ptr(c, target, rets[0])?;
        if c.g[S1][0] != c.g[V0][0] {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 14;
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(rets[1]);
            c.g[A0][0] = c.g[S0][0];
            callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER.call(c, rets[1])?;
            match args {
                InitArgs::Hundred => {
                    c.g[AT][0] = 0x42c8_0000;
                    c.mtc1(1, 12);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(rets[2]);
                    c.mov_s(13, 12);
                }
                InitArgs::GpAnd300(off) => {
                    c.lwc1(12, gp(c, off));
                    c.g[AT][0] = 0x4396_0000;
                    c.mtc1(1, 13);
                    c.g[RA][0] = sx(rets[2]);
                    c.g[A0][0] = c.g[S0][0];
                }
            }
            callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER.call(c, rets[2])?;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x10)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x28)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x2c)).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(rets[3]);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, target, rets[3])?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MatrixChaseCameraAlgorithm_ResetForRider(MatrixChaseCameraAlgorithm*
/// self, int id)` (0x160ae8).
pub fn matrix_chase_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    chase_camera_reset_for_rider(c, [0x0016_0b20, 0x0016_0b34, 0x0016_0b48, 0x0016_0b5c], InitArgs::Hundred);
}

/// `void LookBackChaseCameraAlgorithm_ResetForRider(LookBackChaseCameraAlgorithm*
/// self, int id)` (0x168940).
pub fn look_back_chase_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    chase_camera_reset_for_rider(c, [0x0016_8978, 0x0016_898c, 0x0016_89a0, 0x0016_89b4], InitArgs::GpAnd300(-0x66d4));
}

/// `void MidRightChaseCameraAlgorithm_ResetForRider(MidRightChaseCameraAlgorithm*
/// self, int id)` (0x168c50).
pub fn mid_right_chase_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    chase_camera_reset_for_rider(c, [0x0016_8c88, 0x0016_8c9c, 0x0016_8cb0, 0x0016_8cc4], InitArgs::GpAnd300(-0x6610));
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP, RA as TEST_RA};

    const SEEDS: u32 = 40;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }
    fn h(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }
    fn b(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }

    /// A vtable slot `{delta, fn}` at `vt + off`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, delta: i16, f: u32) {
        h(c, vt + off, delta as u16);
        w(c, vt + off + 4, f);
    }

    /// The zeroed BE-library singleton at `gp-0x1f04` (else the FE helpers walk
    /// into the lazy disc load).
    fn be_library(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    }

    /// The dummy world at `gp-0x848` with the race-manager chain.
    fn world(c: &mut Cpu) {
        let wld = DATA + 0x1000;
        w(c, GAME_GP.wrapping_sub(0x848), wld);
        w(c, wld + 0x84, DATA + 0x1200);
        w(c, DATA + 0x1200 + 0x20, DATA + 0x3000);
        w(c, DATA + 0x1200 + 0x28, DATA + 0x1400);
        w(c, DATA + 0x1200 + 0x78, 0);
        w(c, wld + 0x7c, DATA + 0x1500);
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, 0x40000)`: the game heap over test
    /// memory, for the paths that allocate.
    fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = TEST_RA as u64;
        c.pc = 0x0031_ff60;
        ssx3_ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, TEST_RA as u32, "DLHeap_InitArena did not return");
    }

    /// A NUL-terminated string at `a`.
    fn string(c: &mut Cpu, a: u32, s: &str) {
        for (i, ch) in s.bytes().enumerate() {
            b(c, a + i as u32, ch);
        }
        b(c, a + s.len() as u32, 0);
    }

    /// A child object `obj+field` whose vtable (at `child+vt_field`) dispatches
    /// slot 8 to a fake.
    fn fake_child(c: &mut Cpu, obj: u32, field: u32, vt_field: u32, n: u32) {
        let child = DATA + 0x400 + 0x40 * n;
        let vt = DATA + 0x800 + 0x40 * n;
        w(c, obj + field, child);
        w(c, child + vt_field, vt);
        slot(c, vt, 8, 0, FAKE + 0x10 * n);
    }

    // ------------------------------------------------------------------
    // Script list.

    #[test]
    fn script_list_poll_loading_matches() {
        check(ssx3_ee::s07::f_00277778__generated, script_list_poll_loading, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2777_78, 4);
            let list = DATA + 0x100;
            let engine = DATA + 0x400;
            w(c, list, engine);
            let state = if seed % 4 == 0 { 0 } else { 1 };
            w(c, list + 0xa4, state);
            // ScriptCmdQueue_At(list+0xc, 0): read cursor at +0x8c, 28-byte slots.
            let read = r.next_u32() % 5;
            w(c, list + 0xc + 0x8c, read);
            let entry = list + 0xc + read * 0x1c;
            // ScriptList_IsMovieEntry reads the first word of the entry.
            let movie = seed % 5 == 0;
            w(c, entry, if movie { 29 + r.next_u32() % 4 } else { r.next_u32() % 29 });
            // ScriptEngine_GetProcessState(engine, entry[+0xc]) indexes a
            // 0x24-byte process record at engine+0x2c0; present at +0x20.
            let id = (r.next_u32() % 3) as u32;
            w(c, entry + 0xc, id);
            let proc = engine + 0x2c0 + id * 0x24;
            w(c, proc + 0x20, 1);
            w(c, proc + 4, if seed % 2 == 0 { 2 } else { 3 });
        });
    }

    // ------------------------------------------------------------------
    // UI commands.

    #[test]
    fn ui_screen_cmd_play_anim_matches() {
        check(ssx3_ee::s13::f_0039cca8__generated, ui_screen_cmd_play_anim, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x39cca8, 4);
            let screen = DATA + 0x100;
            let p1 = DATA + 0x200;
            let msg = DATA + 0x300;
            c.g[A0][0] = screen as u64;
            c.g[A1][0] = p1 as u64;
            c.g[A2][0] = msg as u64;
            let hash = r.next_u32();
            w(c, msg + 8, hash);
            w(c, msg + 4, r.next_u32());
            b(c, msg + 0xc, (r.next_u32() % 4) as u8);
            h(c, p1 + 8, (r.next_u32() % 0x10000) as u16);
            // The screen's object array.
            if seed % 3 == 0 {
                w(c, screen + 0x38, 0);
                w(c, screen + 0x3c, 0);
            } else {
                let obj = DATA + 0x400;
                let objs = DATA + 0x500;
                w(c, objs, obj);
                w(c, screen + 0x38, objs);
                w(c, screen + 0x3c, 1);
                w(c, obj + 0x38, hash);
            }
            // The animation bank: an empty list makes CreateAnim return null.
            let anim_set = DATA + 0x600;
            let bank = DATA + 0x650;
            w(c, screen + 0xd0, anim_set);
            w(c, anim_set + 0x10, bank - 0x50);
            w(c, bank, DATA + 0x700);
            w(c, DATA + 0x700, 0);
        });
    }

    #[test]
    fn c_ui_scroll_dtor_matches() {
        check(ssx3_ee::s13::f_0039d8c0__generated, c_ui_scroll_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x39d8c0, 3);
            be_library(c);
            world(c);
            let obj = DATA + 0x100;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = ((seed % 2) * 2) as u64;
            w(c, obj + 0x74, 0);
            w(c, obj + 0x78, 0);
            if seed % 3 != 0 {
                fake_child(c, obj, 0x74, 8, 0);
            }
            if seed % 4 != 0 {
                fake_child(c, obj, 0x78, 8, 1);
            }
            let _ = r.next_u32();
        });
    }

        #[test]
    fn c_ui_progress_dtor_matches() {
        check(ssx3_ee::s13::f_0039c240__generated, c_ui_progress_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x39c240, 3);
            be_library(c);
            world(c);
            let obj = DATA + 0x100;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = ((seed % 2) * 2) as u64;
            if seed % 3 != 0 {
                fake_child(c, obj, 0x74, 8, 0);
            }
            if seed % 4 != 0 {
                fake_child(c, obj, 0x78, 8, 1);
            }
            let _ = r.next_u32();
        });
    }

    #[test]
    fn manual_cam_controller_dtor_matches() {
        check(ssx3_ee::s01::f_0015ce10__generated, manual_cam_controller_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x15ce10, 3);
            be_library(c);
            world(c);
            let obj = DATA + 0x100;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = ((seed % 2) * 2) as u64;
            if seed % 3 != 0 {
                fake_child(c, obj, 0x18, 0x10, 0);
            }
            if seed % 4 != 0 {
                fake_child(c, obj, 0x1c, 0x10, 1);
            }
            let _ = r.next_u32();
        });
    }

    // ------------------------------------------------------------------
    // BX strings.

    #[test]
    fn c_bx_string_ctor_cstr_n_matches() {
        check(ssx3_ee::s10::f_00318350__generated, c_bx_string_ctor_cstr_n, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x318350, 3);
            be_library(c);
            world(c);
            heap_init(c);
            let this = DATA + 0x100;
            c.g[A0][0] = this as u64;
            let s = DATA + 0x200;
            let len = (r.next_u32() % 8) as usize;
            for i in 0..0x20 {
                b(c, s + i as u32, b'a' + (i % 26) as u8);
            }
            b(c, s + len as u32, 0);
            c.g[A1][0] = s as u64;
            c.g[A2][0] = (r.next_u32() % 0x20) as u64;
        });
    }

    #[test]
    fn c_bx_string_concat_copy_matches() {
        check(ssx3_ee::s10::f_00318540__generated, c_bx_string_concat_copy, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x318540, 3);
            be_library(c);
            world(c);
            heap_init(c);
            let this = DATA + 0x100;
            let s1 = DATA + 0x200;
            let s2 = DATA + 0x280;
            for i in 0..0x40 {
                b(c, s1 + i, b'a' + (i % 26) as u8);
                b(c, s2 + i, b'A' + (i % 26) as u8);
            }
            c.g[A0][0] = this as u64;
            // (len1, s1, len2, s2) as the generated call order shows.
            c.g[A1][0] = (r.next_u32() % 0x20) as u64;
            c.g[A2][0] = s1 as u64;
            c.g[A3][0] = (r.next_u32() % 0x20) as u64;
            c.g[T0][0] = s2 as u64;
        });
    }

    // ------------------------------------------------------------------
    // Bolt-on effects.

    #[test]
    fn c_bolt_on_fx_interface_ctor_matches() {
        check(ssx3_ee::s09::f_002ec9e0__generated, c_bolt_on_fx_interface_ctor, SEEDS, &|c, seed| {
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
        check(ssx3_ee::s09::f_002eca68__generated, c_bolt_on_fx_interface_ctor_2, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2eca68, 0);
            be_library(c);
            world(c);
            let obj = DATA + 0x100;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    // ------------------------------------------------------------------
    // World memory manager and cache.

    #[test]
    fn world_memman_ctor_matches() {
        check(ssx3_ee::s13::f_003a7a20__generated, world_memman_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3a7a20, 3);
            be_library(c);
            world(c);
            heap_init(c);
            let obj = DATA + 0x100;
            c.g[A0][0] = obj as u64;
            // The block-pool table: {size, count} triples.
            const TABLE: u32 = 0x0044_c1e0;
            for i in 0..3 {
                let n = (r.next_u32() % 3) as u32;
                w(c, TABLE + 0x10 * i + 4, 0x18);
                w(c, TABLE + 0x10 * i + 8, n);
            }
        });
    }

    #[test]
    fn c_world_cache_create_section_table_matches() {
        check(
            ssx3_ee::s13::f_003a81a8__generated,
            c_world_cache_create_section_table,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a81a8, 6);
                be_library(c);
                world(c);
                heap_init(c);
                let cache = DATA + 0x100;
                let sections = DATA + 0x200;
                c.g[A0][0] = cache as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                w(c, cache, sections);
                w(c, cache + 8, DATA + 0x300);
                w(c, cache + 0xc, DATA + 0x400);
                w(c, sections + 0x14, DATA + 0x500);
            },
        );
    }

    // ------------------------------------------------------------------
    // Tag fields.

    #[test]
    fn tag_field_get_token_matches() {
        check(ssx3_ee::s14::f_003ed070__generated, tag_field_get_token, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3ed070, 0);
            let s = DATA + 0x200;
            if seed % 7 == 0 {
                c.g[A0][0] = 0;
            } else {
                c.g[A0][0] = s as u64;
                // A run of printable characters, a control character and a
                // high byte, at varying offsets.
                let run = (r.next_u32() % 6) as usize;
                for i in 0..0x10 {
                    let v = if i < run {
                        b'a' + (r.next_u32() % 26) as u8
                    } else {
                        [0u8, 0x20, 0x7f, 0x80, 0xff][(r.next_u32() % 5) as usize]
                    };
                    b(c, s + i as u32, v);
                }
            }
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    // ------------------------------------------------------------------
    // UI groups.

    #[test]
    fn c_ui_group_update_anim_matches() {
        check(ssx3_ee::s13::f_00398910__generated, c_ui_group_update_anim, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x398910, 4);
            be_library(c);
            world(c);
            let group = DATA + 0x100;
            c.g[A0][0] = group as u64;
            c.g[A1][0] = (r.next_u32() & 0xffff) as u64;
            c.pc = 0x0039_8910;
            let list = group + 0x74;
            if seed % 4 == 0 {
                // An empty list: the sentinel links back to itself.
                let sent = DATA + 0x300;
                w(c, list + 4, sent);
                w(c, sent, sent);
                w(c, sent + 4, sent);
            } else {
                let n1 = DATA + 0x300;
                let sent = DATA + 0x340;
                let vt = DATA + 0x500;
                w(c, list + 4, n1);
                w(c, n1, r.next_u32());
                w(c, n1 + 4, sent);
                w(c, n1 + 8, vt);
                slot(c, vt, 0x98, 0, FAKE);
                slot(c, vt, 0x9c, 0, FAKE + 0x10);
                w(c, sent, sent);
                w(c, sent + 4, sent);
            }
        });
    }

    #[test]
    fn modifier_block_remove_dead_particles_matches() {
        check(
            ssx3_ee::s11::f_00353278__generated,
            modifier_block_remove_dead_particles,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x353278, 6);
                be_library(c);
                world(c);
                let blk = DATA + 0x100;
                c.g[A0][0] = blk as u64;
                c.g[A1][0] = r.next_u32() as u64;
                if seed % 4 == 0 {
                    w(c, blk + 0x10, 0);
                } else {
                    let n1 = DATA + 0x300;
                    let n2 = DATA + 0x380;
                    let vt1 = DATA + 0x500;
                    let vt2 = DATA + 0x600;
                    w(c, blk + 0x10, n1);
                    w(c, n1, n2);
                    w(c, n1 + 8, vt1);
                    w(c, n2, 0);
                    w(c, n2 + 8, vt2);
                    slot(c, vt1, 0x10, 0, FAKE);
                    slot(c, vt1, 0x14, 0, FAKE + 0x10);
                    slot(c, vt2, 0x10, 0, FAKE + 0x20);
                    slot(c, vt2, 0x14, 0, FAKE + 0x30);
                    // The state getter returns 3 (dead) on some seeds.
                    if seed % 2 == 0 {
                        w(c, 0x0090_0010, 3);
                    }
                }
            },
        );
    }

            // ------------------------------------------------------------------
    // Game state.

    #[test]
    fn game_state_finish_enter_matches() {
        check(ssx3_ee::s06::f_00233c50__generated, game_state_finish_enter, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x233c50, 6);
            be_library(c);
            world(c);
            let st = DATA + 0x100;
            c.g[A0][0] = st as u64;
            // gp-0x488: the replay manager; +0x28 and +0x48 its objects.
            let replay = DATA + 0x200;
            w(c, GAME_GP.wrapping_sub(0x488), replay);
            w(c, replay + 0x28, 0);
            w(c, replay + 0x48, DATA + 0x280);
            // gp-0x84c: the script engine for ScriptEngine_ListStop.
            w(c, GAME_GP.wrapping_sub(0x84c), 0);
            // gp-0x204: the GameStateFinish instance (or none).
            if seed % 3 == 0 {
                w(c, GAME_GP.wrapping_sub(0x204), 0);
            } else {
                w(c, GAME_GP.wrapping_sub(0x204), DATA + 0x300);
            }
            let _ = r.next_u32();
        });
    }

        // ------------------------------------------------------------------
    // Luno VM.

    #[test]
    fn luno_vm_register_read_ptr_matches() {
        check(ssx3_ee::s05::f_00224c78__generated, luno_vm_register_read_ptr, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x224c78, 4);
            be_library(c);
            world(c);
            let reg = DATA + 0x100;
            let stream = DATA + 0x200;
            let out = DATA + 0x300;
            c.g[A0][0] = reg as u64;
            c.g[A1][0] = stream as u64;
            c.g[A2][0] = out as u64;
            let vt = DATA + 0x400;
            w(c, stream, vt);
            slot(c, vt, 0x10, 0, FAKE);
            slot(c, vt, 0x14, 0, FAKE + 0x10);
            w(c, out, r.next_u32());
            w(c, out + 4, r.next_u32());
        });
    }

    #[test]
    fn c_luno_table_entry_dtor_matches() {
        check(ssx3_ee::s05::f_00226768__generated, c_luno_table_entry_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x226768, 4);
            be_library(c);
            world(c);
            let entry = DATA + 0x100;
            c.g[A0][0] = entry as u64;
            c.g[A1][0] = ((seed % 2) * 2) as u64;
            if seed % 3 == 0 {
                w(c, entry + 0x18, 0);
            } else {
                let child = DATA + 0x400;
                let vt = DATA + 0x800;
                w(c, entry + 0x18, child);
                w(c, child + 0x1c, vt);
                slot(c, vt, 8, 0, FAKE);
            }
            let _ = r.next_u32();
        });
    }

    #[test]
    fn avalanche_release_instance_matches() {
        check(ssx3_ee::s09::f_002d7dd8__generated, avalanche_release_instance, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2d7dd8, 4);
            be_library(c);
            world(c);
            let av = DATA + 0x100;
            c.g[A0][0] = av as u64;
            c.g[A1][0] = (seed % 2) as u64;
            if seed % 4 == 0 {
                w(c, av + 0x2e0, 0);
            } else {
                let first = DATA + 0x300;
                let live = DATA + 0x400;
                let vt = DATA + 0x500;
                w(c, av + 0x2e0, first);
                w(c, first + 0xf0, 2);
                w(c, first + 0xf8, live);
                w(c, live + 0xf0, 0);
                w(c, live + 0xf8, 0);
                if seed % 3 == 0 {
                    w(c, live + 0xc, 0);
                } else {
                    let sub = DATA + 0x600;
                    w(c, live + 0xc, sub);
                    w(c, sub + 0xc, vt);
                    slot(c, vt, 8, 0, FAKE);
                }
            }
        });
    }

    // ------------------------------------------------------------------
    // UI objects.

    #[test]
    fn ui_pair_set_visible_matches() {
        check(ssx3_ee::s13::f_0039bb50__generated, ui_pair_set_visible, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x39bb50, 3);
            be_library(c);
            world(c);
            let pair = DATA + 0x100;
            c.g[A0][0] = pair as u64;
            c.g[A1][0] = (seed % 3) as u64;
            w(c, pair + 0x14, if seed % 2 == 0 { 0x40 } else { 0 });
            w(c, pair + 0x78, 0);
            w(c, pair + 0x7c, 0);
            if seed % 3 != 0 {
                let a = DATA + 0x300;
                let vt = DATA + 0x500;
                w(c, pair + 0x78, a);
                w(c, a + 8, vt);
                slot(c, vt, 0x48, 0, FAKE);
                slot(c, vt, 0x4c, 0, FAKE + 0x10);
            }
            if seed % 4 != 0 {
                let b = DATA + 0x400;
                let vt = DATA + 0x600;
                w(c, pair + 0x7c, b);
                w(c, b + 8, vt);
                slot(c, vt, 0x48, 0, FAKE + 0x20);
                slot(c, vt, 0x4c, 0, FAKE + 0x30);
            }
            let _ = r.next_u32();
        });
    }

    #[test]
    fn ov_screen_enter_lodge_on_activate_matches() {
        check(
            ssx3_ee::s04::f_001f7210__generated,
            ov_screen_enter_lodge_on_activate,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1f7210, 6);
                be_library(c);
                world(c);
                let st = DATA + 0x100;
                c.g[A0][0] = st as u64;
                c.g[A1][0] = r.next_u32() as u64;
                w(c, st + 0x40, DATA + 0x300);
                if seed % 3 != 0 {
                    let objs = DATA + 0x400;
                    let obj = DATA + 0x500;
                    w(c, DATA + 0x300 + 0x38, objs);
                    w(c, DATA + 0x300 + 0x3c, 1);
                    w(c, objs, obj);
                    w(c, obj + 0x38, 0);
                }
            },
        );
    }

    #[test]
    fn c_fe_state_mem_card_restart_scan_matches() {
        check(
            ssx3_ee::s02::f_001871a8__generated,
            c_fe_state_mem_card_restart_scan,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1871a8, 6);
                be_library(c);
                world(c);
                let mgr = DATA + 0x200;
                let vt = DATA + 0x400;
                w(c, GAME_GP.wrapping_sub(0x1c38), mgr);
                w(c, GAME_GP.wrapping_sub(0x1c34), 0);
                w(c, mgr + 0x748, vt);
                // The slot itself is a vtable: delta at +8, function at +0xc.
                slot(c, vt, 8, 0, FAKE);
                slot(c, vt, 0xc, 0, FAKE + 0x10);
                c.g[A0][0] = DATA as u64;
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn rider_model_show_part_variant_matches() {
        check(ssx3_ee::s10::f_0030ec18__generated, rider_model_show_part_variant, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x30ec18, 0);
            be_library(c);
            world(c);
            let model = DATA + 0x100;
            c.g[A0][0] = model as u64;
            let part = (r.next_u32() % 4) as u32;
            c.g[A1][0] = part as u64;
            c.g[A2][0] = r.next_u32() as u64;
            w(c, model + 0x14, (r.next_u32() % 8) as u32);
            if seed % 5 == 0 {
                w(c, model + 0x1c, 0);
            } else {
                let arr = DATA + 0x200;
                w(c, model + 0x1c, arr);
                for i in 0..4 {
                    // A negative id is a "no variant" slot.
                    let id = if seed % 3 == 0 && i == 1 {
                        0xffff_ffffu32
                    } else {
                        i
                    };
                    w(c, arr + 4 * i, id);
                }
                w(c, model + 0xc, DATA + 0x300);
                c.bus.write64(model + 0x150, r.next_u32() as u64);
            }
        });
    }

    // ------------------------------------------------------------------
    // Animation maps and camera triggers.

    #[test]
    fn game_anim_map_unregister_bank_matches() {
        check(
            ssx3_ee::s10::f_00314fe8__generated,
            game_anim_map_unregister_bank,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x314fe8, 3);
                be_library(c);
                world(c);
                let map = DATA + 0x100;
                let bank = DATA + 0x200;
                c.g[A0][0] = map as u64;
                c.g[A1][0] = bank as u64;
                w(c, GAME_GP.wrapping_add(0x2860), 0);
                let count = (r.next_u32() % 3) as i32;
                w(c, bank + 4, DATA + 0x300);
                h(c, DATA + 0x300 + 2, count as u16);
                w(c, bank + 8, DATA + 0x400);
                for i in 0..3 {
                    w(c, DATA + 0x400 + 0x14 * i, r.next_u32() & 0xffff);
                }
            },
        );
    }

    #[test]
    fn camera_trigger_man_ctor_matches() {
        check(ssx3_ee::s02::f_0016c800__generated, camera_trigger_man_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x16c800, 0);
            be_library(c);
            world(c);
            let man = DATA + 0x100;
            c.g[A0][0] = man as u64;
            let _ = r.next_u32();
        });
    }

    #[test]
    fn fe_async_manager_free_all_matches() {
        check(ssx3_ee::s03::f_001a3c50__generated, fe_async_manager_free_all, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1a3c50, 4);
            be_library(c);
            world(c);
            let mgr = DATA + 0x100;
            c.g[A0][0] = mgr as u64;
            // gp-0x854: the object whose vtable slot 50 releases a file.
            let obj = DATA + 0x200;
            let vt = DATA + 0x600;
            w(c, GAME_GP.wrapping_sub(0x854), obj);
            w(c, obj + 0x10d8, vt);
            slot(c, vt, 0x190, 0, FAKE);
            slot(c, vt, 0x194, 0, FAKE + 0x10);
            for i in 0..5 {
                let file = mgr + 0x100 + 0x11c * i;
                if seed % 3 != 0 || i == 0 {
                    w(c, file, r.next_u32() & 0xffff);
                    w(c, file + 8, r.next_u32());
                    w(c, file - 0x10, DATA + 0x4000 + 0x40 * i);
                }
            }
        });
    }

    // ------------------------------------------------------------------
    // Front-end network states.

    #[test]
    fn net_config_auth_popup_handle_input_matches() {
        check(
            ssx3_ee::s03::f_001b66b8__generated,
            net_config_auth_popup_handle_input,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b66b8, 4);
                be_library(c);
                world(c);
                let popup = DATA + 0x100;
                let obj = DATA + 0x200;
                c.g[A0][0] = popup as u64;
                c.g[A1][0] = obj as u64;
                w(c, popup + 0x6e4, if seed % 3 == 0 { 2 } else { 1 });
                w(c, popup + 0x6d0, DATA + 0x300);
                let vt = DATA + 0x500;
                w(c, obj + 8, vt);
                slot(c, vt, 0x28, 0, FAKE);
                slot(c, vt, 0x2c, 0, FAKE + 0x10);
                slot(c, vt, 0x30, 0, FAKE + 0x20);
                slot(c, vt, 0x34, 0, FAKE + 0x30);
            },
        );
    }

    #[test]
    fn c_fe_state_create_step1_init_matches() {
        check(ssx3_ee::s03::f_001b8960__generated, c_fe_state_create_step1_init, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1b8960, 4);
            be_library(c);
            world(c);
            let st = DATA + 0x100;
            c.g[A0][0] = st as u64;
            w(c, st + 0x6ec, if seed % 2 == 0 { 0 } else { 1 });
            w(c, st + 0x10, 0);
            w(c, GAME_GP.wrapping_sub(0xc8), DATA + 0x200);
            let _ = r.next_u32();
        });
    }

    #[test]
    fn c_fe_state_create_step1_detect_changes_matches() {
        check(
            ssx3_ee::s03::f_001ba358__generated,
            c_fe_state_create_step1_detect_changes,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1ba358, 4);
                be_library(c);
                world(c);
                let st = DATA + 0x100;
                let dev = DATA + 0x200;
                c.g[A0][0] = st as u64;
                let s1 = DATA + 0x300;
                let s2 = DATA + 0x380;
                string(c, s1, "abc");
                string(c, s2, if seed % 3 == 0 { "abc" } else { "xyz" });
                w(c, st + 0x6d0, dev);
                w(c, st + 0x6d4, s1);
                w(c, st + 0x6dc, s1);
                w(c, dev + 4, if seed % 2 == 0 { s1 } else { s2 });
                w(c, dev + 0xc, if seed % 3 == 0 { s1 } else { s2 });
                w(c, dev + 0x38, r.next_u32());
                w(c, dev + 0x3c, r.next_u32());
                w(c, st + 0x6e0, r.next_u32());
                w(c, st + 0x6e4, r.next_u32());
            },
        );
    }

    #[test]
    fn c_fe_state_create_duplicate_on_child_event_matches() {
        check(
            ssx3_ee::s03::f_001bc6a0__generated,
            c_fe_state_create_duplicate_on_child_event,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1bc6a0, 2);
                be_library(c);
                world(c);
                let st = DATA + 0x100;
                let ev = DATA + 0x200;
                c.g[A0][0] = st as u64;
                c.g[A1][0] = ev as u64;
                c.g[A2][0] = r.next_u32() as u64;
                let word = match seed % 5 {
                    0 => 0x81f,
                    1 => 0x80e,
                    2 => 0x82c,
                    3 => 0x800,
                    _ => 0x900,
                };
                w(c, ev + 0x18, word);
            },
        );
    }

        #[test]
    fn c_fe_state_online_agreement_update_matches() {
        check(
            ssx3_ee::s03::f_001c0a80__generated,
            c_fe_state_online_agreement_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c0a80, 6);
                be_library(c);
                world(c);
                let st = DATA + 0x100;
                c.g[A0][0] = st as u64;
                w(c, st + 0x6d8, (r.next_u32() % 4) as u32);
                w(c, st + 0x6d4, (r.next_u32() % 4) as u32);
                w(c, st + 0x6d0, if seed % 3 == 0 { 0 } else { DATA + 0x200 });
                w(c, st + 0x40, DATA + 0x300);
                if seed % 4 != 0 {
                    let objs = DATA + 0x400;
                    let obj = DATA + 0x500;
                    w(c, DATA + 0x300 + 0x38, objs);
                    w(c, DATA + 0x300 + 0x3c, 1);
                    w(c, objs, obj);
                    w(c, obj + 0x38, 0);
                }
            },
        );
    }

    #[test]
    fn c_fe_popup_select_multiplayer_mode_on_widget_notify_matches() {
        check(
            ssx3_ee::s04::f_001e0470__generated,
            c_fe_popup_select_multiplayer_mode_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e0470, 2);
                be_library(c);
                world(c);
                let st = DATA + 0x100;
                let menu = DATA + 0x200;
                c.g[A0][0] = st as u64;
                c.g[A1][0] = menu as u64;
                c.g[A2][0] = (seed % 9) as u64;
                c.g[A3][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    #[test]
    fn message_center_trigger_groups_matches() {
        check(
            ssx3_ee::s04::f_001e14c0__generated,
            message_center_trigger_groups,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e14c0, 4);
                be_library(c);
                world(c);
                let st = DATA + 0x100;
                c.g[A0][0] = (seed % 6) as u64;
                let _ = st;
                // The trigger table at 0x441630, 0x14 bytes per group.
                const TABLE: u32 = 0x0044_1630;
                for i in 0..0x40 {
                    let kind = (r.next_u32() % 7) as u32;
                    w(c, TABLE + 0x14 * i, kind);
                }
            },
        );
    }

    // ------------------------------------------------------------------
    // Lobby / buddy.

    #[test]
    fn lobby_man_chal_request_host_matches() {
        check(ssx3_ee::s06::f_0025fa48__generated, lobby_man_chal_request_host, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x25fa48, 3);
            be_library(c);
            world(c);
            let this = DATA + 0x100;
            c.g[A0][0] = this as u64;
            let name = DATA + 0x200;
            string(c, name, "rider");
            c.g[A1][0] = name as u64;
            let _ = r.next_u32();
        });
    }

    #[test]
    fn lobby_man_chal_request_join_matches() {
        check(ssx3_ee::s06::f_0025fad0__generated, lobby_man_chal_request_join, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x25fad0, 3);
            be_library(c);
            world(c);
            let this = DATA + 0x100;
            c.g[A0][0] = this as u64;
            let name = DATA + 0x200;
            string(c, name, "rider");
            c.g[A1][0] = name as u64;
            let _ = r.next_u32();
        });
    }

    #[test]
    fn buddy_man_delete_message_at_matches() {
        check(ssx3_ee::s07::f_002635a8__generated, buddy_man_delete_message_at, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2635a8, 4);
            be_library(c);
            world(c);
            let man = DATA + 0x100;
            let ring = DATA + 0x200;
            c.g[A0][0] = man as u64;
            let count = 1 + (r.next_u32() % 4) as u32;
            c.g[A1][0] = (r.next_u32() % count) as u64;
            w(c, man + 0xa8, ring);
            w(c, man + 0xac, ring);
            w(c, man + 0xb0, ring + 4 * count);
            for i in 0..count {
                let msg = DATA + 0x400 + 0x40 * i;
                w(c, ring + 4 * i, msg);
                let vt = DATA + 0x800 + 0x40 * i;
                w(c, msg + 0xc, vt);
                slot(c, vt, 8, 0, FAKE + 0x10 * i);
                slot(c, vt, 0xc, 0, FAKE + 0x10 * i + 4);
            }
        });
    }

    // ------------------------------------------------------------------
    // INI parser.

    #[test]
    fn ini_parser_find_section_prefix_matches() {
        check(ssx3_ee::s08::f_002a7c68__generated, ini_parser_find_section_prefix, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2a7c68, 4);
            be_library(c);
            world(c);
            let parser = DATA + 0x100;
            let prefix = DATA + 0x200;
            c.g[A0][0] = parser as u64;
            c.g[A1][0] = prefix as u64;
            c.pc = 0x002a_7c68;
            string(c, prefix, if seed % 2 == 0 { "abc" } else { "abd" });
            match seed % 3 {
                0 => w(c, parser + 8, 0),
                _ => {
                    let sect = DATA + 0x300;
                    string(c, sect, if seed % 2 == 0 { "abcdef" } else { "xyz" });
                    w(c, parser + 8, sect);
                }
            }
        });
    }

    // ------------------------------------------------------------------
    // Script tables and objects.

    #[test]
    fn script_engine_obj_table_ctor_matches() {
        check(ssx3_ee::s07::f_0027cdd0__generated, script_engine_obj_table_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x27cdd0, 0);
            be_library(c);
            world(c);
            c.g[A0][0] = (DATA + 0x100) as u64;
            let _ = r.next_u32();
        });
    }

    #[test]
    fn script_obj_cam_activate_matches() {
        check(ssx3_ee::s07::f_0027d850__generated, script_obj_cam_activate, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x27d850, 6);
            be_library(c);
            world(c);
            let cam = DATA + 0x100;
            c.g[A0][0] = cam as u64;
            if seed % 4 == 0 {
                // No world: the short path.
                w(c, GAME_GP.wrapping_sub(0x848), 0);
            } else {
                world(c);
                let wld = DATA + 0x1000;
                let race = DATA + 0x1200;
                w(c, wld + 0x84, race);
                let arr = DATA + 0x1600;
                w(c, race + 0x84, arr);
                w(c, cam + 0x7c, (r.next_u32() % 4) as u32);
                let slot_obj = DATA + 0x1700;
                let slot_index = c.bus.read32(cam + 0x7c);
                w(c, arr + 4 * slot_index, slot_obj);
                w(c, slot_obj + 4, 0);
                w(c, slot_obj + 0xac, DATA + 0x1800);
                w(c, cam + 0x14, r.next_u32() & 0x7);
                w(c, cam + 0x18, 0);
                // The controller's vtable slot at +0x18.
                let ctl = DATA + 0x1800;
                w(c, ctl + 0x18, if seed % 3 == 0 { 0 } else { DATA + 0x1900 });
                let _ = ctl;
            }
        });
    }

    #[test]
    fn script_audio_obj_ctor_matches() {
        check(ssx3_ee::s07::f_002805b8__generated, script_audio_obj_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2805b8, 0);
            be_library(c);
            world(c);
            c.g[A0][0] = (DATA + 0x100) as u64;
            let _ = r.next_u32();
        });
    }

    // ------------------------------------------------------------------
    // Audio.

    #[test]
    fn path_plat_set_track_level_matches() {
        check(ssx3_ee::s08::f_002af580__generated, path_plat_set_track_level, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2af580, 4);
            be_library(c);
            world(c);
            const TABLE: u32 = 0x004d_3e98;
            let track = (r.next_u32() % 8) as u32;
            c.g[A0][0] = track as u64;
            c.g[A1][0] = (r.next_u32() % 0x100) as u64;
            if seed % 4 == 0 {
                w(c, TABLE + 4 * track, 0);
            } else {
                let snd = DATA + 0x300;
                w(c, TABLE + 4 * track, snd);
                w(c, snd, DATA + 0x400);
                b(c, snd + 0x64, if seed % 2 == 0 { 0 } else { 1 });
            }
        });
    }

    // ------------------------------------------------------------------
    // In-sequence channels.

    #[test]
    fn in_seq_channel_set_listener_matches() {
        check(ssx3_ee::s07::f_00274240__generated, in_seq_channel_set_listener, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x274240, 4);
            be_library(c);
            world(c);
            let ch = DATA + 0x100;
            c.g[A0][0] = ch as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { (DATA + 0x300) as u64 };
            w(c, ch + 0x1c, DATA + 0x400);
            b(c, DATA + 0x400 + 3, (r.next_u32() % 4) as u8);
            if seed % 3 != 0 {
                let old = DATA + 0x500;
                let vt = DATA + 0x600;
                w(c, ch + 0x2c, old);
                w(c, old, vt);
                slot(c, vt, 0x10, 0, FAKE);
                slot(c, vt, 0x14, 0, FAKE + 0x10);
            } else {
                w(c, ch + 0x2c, 0);
            }
            let new = DATA + 0x300;
            w(c, new, if seed % 4 == 0 { 0 } else { DATA + 0x700 });
            let vt = DATA + 0x700;
            slot(c, vt, 8, 0, FAKE + 0x20);
            slot(c, vt, 0xc, 0, FAKE + 0x30);
        });
    }

    // ------------------------------------------------------------------
    // Memory card.

    #[test]
    fn c_ps2_mem_card_close_matches() {
        check(ssx3_ee::s08::f_002c6360__generated, c_ps2_mem_card_close, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2c6360, 4);
            be_library(c);
            world(c);
            let card = DATA + 0x100;
            c.g[A0][0] = card as u64;
            w(c, card + 0x40, if seed % 2 == 0 { 0 } else { 1 });
            w(c, card + 0xc, r.next_u32() & 0x7);
            w(c, GAME_GP.wrapping_add(0x850), if seed % 3 == 0 { 0xffff_ffff } else { 0 });
        });
    }

    // ------------------------------------------------------------------
    // Chase-camera algorithms.

    #[test]
    fn matrix_chase_camera_algorithm_reset_for_rider_matches() {
        check(
            ssx3_ee::s01::f_00160ae8__generated,
            matrix_chase_camera_algorithm_reset_for_rider,
            SEEDS,
            &|c, seed| camera_reset_setup(c, seed, 0x160ae8),
        );
    }

    #[test]
    fn look_back_chase_camera_algorithm_reset_for_rider_matches() {
        check(
            ssx3_ee::s02::f_00168940__generated,
            look_back_chase_camera_algorithm_reset_for_rider,
            SEEDS,
            &|c, seed| camera_reset_setup(c, seed, 0x168940),
        );
    }

    #[test]
    fn mid_right_chase_camera_algorithm_reset_for_rider_matches() {
        check(
            ssx3_ee::s02::f_00168c50__generated,
            mid_right_chase_camera_algorithm_reset_for_rider,
            SEEDS,
            &|c, seed| camera_reset_setup(c, seed, 0x168c50),
        );
    }

    /// Shared fixture of the three camera resets: the algorithm's target
    /// (self+0x30 -> +4 -> +0x6c0) with a matching id, and the controller's
    /// vtable.
    fn camera_reset_setup(c: &mut Cpu, seed: u32, salt: u32) {
        let mut r = machine(c, seed, salt as u64, 4);
        be_library(c);
        world(c);
        let algo = DATA + 0x100;
        let target = DATA + 0x200;
        let vt = DATA + 0x600;
        let id = r.next_u32();
        c.g[A0][0] = algo as u64;
        c.g[A1][0] = if seed % 3 == 0 { id.wrapping_add(1) as u64 } else { id as u64 };
        w(c, algo + 0x30, DATA + 0x300);
        w(c, DATA + 0x300 + 4, target);
        w(c, target + 0x6c0, vt);
        slot(c, vt, 0x38, 0, FAKE);
        slot(c, vt, 0x3c, 0, FAKE + 0x10);
        // gp-0x66d4 / gp-0x6610: the pitch constants the look-back and
        // mid-right variants load.
        w(c, GAME_GP.wrapping_sub(0x66d4), 0x3f80_0000);
        w(c, GAME_GP.wrapping_sub(0x6610), 0x3f80_0000);
        // The controller's virtual +0x28/+0x2c call.
        let ctl = DATA + 0x700;
        w(c, algo + 0x10, ctl);
        slot(c, ctl, 0x28, 0, FAKE + 0x20);
        slot(c, ctl, 0x2c, 0, FAKE + 0x30);
    }
}
