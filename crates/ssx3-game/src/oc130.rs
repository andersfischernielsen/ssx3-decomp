//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path functions — message constructors, FE screen
//! helpers, BE interface accessors, script/path readers, render-slot resets —
//! each with a differential test against the generated body.
//!
//! Forty-seven are listed in [`OVERRIDES`]. Two more exact ports are kept
//! below without a differential test because their only interesting path
//! needs a live file cache or Luno VM (`cFEStateRewardGallery_RequestShapeLoad`,
//! `WScriptMan_CallFuncOnInstance`). `SSXAudioSystem_ReadPlaylistNames` is not
//! ported: it is a resumable dispatcher whose body also runs the `.ini`
//! parser, and its `EAFile_PathExists` path is not boundable in a unit test.
//!
//! Several generated bodies read callee-saved registers they never set
//! (`s1`/`s2` hold incoming scratch values from the caller in the original);
//! those reads are reproduced as they are, since the register is part of the
//! function's interface there.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0024_19f0,
        mc_manager_format_slot_total_space,
        "MCManager_FormatSlotTotalSpace",
    ),
    (
        0x001c_7040,
        cfe_popup_confirm_layout_post_text,
        "cFEPopupConfirm_LayoutPostText",
    ),
    (
        0x0017_3208,
        bounded_camera_algorithm_base_ctor,
        "BoundedCameraAlgorithmBase_ctor",
    ),
    (
        0x0017_a2c8,
        debug_menu_find_prev_item,
        "DebugMenu_FindPrevItem",
    ),
    (
        0x0030_9568,
        wscript_man_create_mission,
        "WScriptMan_CreateMission",
    ),
    (0x0035_7d28, multiparticle_man_load, "MultiParticleMan_Load"),
    (
        0x0026_9b90,
        csetup_man_client_recv_msg11,
        "cSetupMan_ClientRecvMsg11",
    ),
    (
        0x0026_9cf0,
        csetup_man_client_recv_msg13,
        "cSetupMan_ClientRecvMsg13",
    ),
    (
        0x0026_9ae0,
        csetup_man_client_recv_msg10,
        "cSetupMan_ClientRecvMsg10",
    ),
    (
        0x0026_9c40,
        csetup_man_client_recv_msg12,
        "cSetupMan_ClientRecvMsg12",
    ),(
        0x0018_d058,
        cfe_state_options_controller_update,
        "cFEStateOptionsController_Update",
    ),
    (
        0x001d_9258,
        memcard_state_base_sync_menu_selection,
        "MemCardStateBase_SyncMenuSelection",
    ),
    (
        0x0014_7170,
        benew_player_int_get_name,
        "BENewPlayerInt_GetName",
    ),
    (0x0023_8c80, gm_handler_start, "GMHandler_Start"),
    (0x0014_d558, bolts_find_first_child, "Bolts_FindFirstChild"),
    (
        0x0034_49f0,
        crowd_man3d_group_update,
        "CrowdMan3d_GroupUpdate",
    ),
    (
        0x0036_cef8,
        emitter_set_desc_path_coeffs,
        "Emitter_SetDescPathCoeffs",
    ),
    (0x003a_c2a8, cps2_chunk_resolve_v13, "cPS2ChunkResolve_v13"),
    (0x0023_4008, game_state_finish_exit, "GameStateFinish_Exit"),
    (
        0x0020_a430,
        cov_state_overlay_shutdown,
        "cOVStateOverlay_Shutdown",
    ),
    (
        0x002d_9c00,
        avalanche_get_object_matrix,
        "Avalanche_GetObjectMatrix",
    ),
    (
        0x0034_c378,
        cflag_manager_group_load,
        "cFlagManager_GroupLoad",
    ),
    (
        0x0019_7ad8,
        cfe_state_request_line_sort_song_list,
        "cFEStateRequestLine_SortSongList",
    ),
    (
        0x0018_f0b8,
        cfe_state_profile_load_v38,
        "cFEStateProfileLoad_v38",
    ),
    (
        0x0014_56a0,
        benew_race_int_get_event_record_target,
        "BENewRaceInt_GetEventRecordTarget",
    ),
    (
        0x0021_0d20,
        ov_screen_read_private_message_update,
        "OVScreenReadPrivateMessage_Update",
    ),
    (
        0x0015_4898,
        bemission_int_load_state,
        "BEMissionInt_LoadState",
    ),
    (
        0x0014_eb08,
        beoption_int_swap_player_flags,
        "BEOptionInt_SwapPlayerFlags",
    ),
    (
        0x0015_58f8,
        bescore_int_get_event_best_place,
        "BEScoreInt_GetEventBestPlace",
    ),
    (
        0x0031_4dd8,
        game_anim_map_build_name_index,
        "GameAnimMap_BuildNameIndex",
    ),
    (0x0039_71c8, uianim_eval_channel, "UIAnim_EvalChannel"),
    (0x0019_57d0, femodule_base_dtor, "FEModule_BaseDtor"),
    (
        0x0022_e298,
        section_man_queue_section,
        "SectionMan_QueueSection",
    ),
    (
        0x001b_b520,
        cfe_state_create_step2_fill_gender_list,
        "cFEStateCreateStep2_FillGenderList",
    ),
    (
        0x001b_ec78,
        net_create_room_popup_validate_room_name,
        "cFEStateNetCreateRoomPopup_ValidateRoomName",
    ),
    (
        0x001e_31b8,
        message_center_remove_message,
        "MessageCenter_RemoveMessage",
    ),
    (0x0026_80a8, chat_event_clone, "ChatEvent_Clone"),
    (
        0x002a_2cf0,
        ssx_audio_system_dj_say212e,
        "SSXAudioSystem_DJSay212E",
    ),
    (
        0x002a_2da0,
        ssx_audio_system_dj_say212f,
        "SSXAudioSystem_DJSay212F",
    ),
    (
        0x002a_7cf0,
        ini_parser_read_quoted_string,
        "IniParser_ReadQuotedString",
    ),
    (
        0x0012_ab20,
        ai_reset_computer_render_slots,
        "AI_ResetComputerRenderSlots",
    ),
    (
        0x0012_ac48,
        ai_reset_actor_render_slots,
        "AI_ResetActorRenderSlots",
    ),
    (
        0x002c_63f8,
        cps2_memcard_get_slot_name,
        "cPS2MemCard_GetSlotName",
    ),
    (
        0x001c_cf70,
        keyboard_popup_set_static_wide_text,
        "cKeyboardPopup_SetStaticWideText",
    ),
    (
        0x0017_8938,
        superpipe_chase_camera_algorithm_reset_for_rider,
        "SuperpipeChaseCameraAlgorithm_ResetForRider",
    ),
    (0x0026_a378, path_read_event, "Path_ReadEvent"),
];

// ---------------------------------------------------------------------------
// Callees (address + the generated shard function, which dispatches through
// its override slot). Entries for calls that the generator emitted as
// `call_indirect` are kept for reference next to the `call_ptr` sites.

#[allow(dead_code)]
mod callee {
    use super::{ee, Callee};

    pub const USTR_FROM_ASCII: Callee = Callee {
        addr: 0x002c_2540,
        f: ee::s08::f_002c2540,
    };
    pub const USTR_SPRINTF: Callee = Callee {
        addr: 0x002c_26d0,
        f: ee::s08::f_002c26d0,
    };
    pub const USTR_TO_ASCII: Callee = Callee {
        addr: 0x002c_2580,
        f: ee::s08::f_002c2580,
    };

    pub const POPUP_SET_WIDGET_ALIGN: Callee = Callee {
        addr: 0x001c_8930,
        f: ee::s03::f_001c8930,
    };
    pub const POPUP_SET_WRAPPED_TEXT: Callee = Callee {
        addr: 0x001c_97e0,
        f: ee::s03::f_001c97e0,
    };

    pub const RIDER_CAMERA_ALGORITHM_CTOR: Callee = Callee {
        addr: 0x0016_22b0,
        f: ee::s01::f_001622b0,
    };
    pub const DEBUG_MENU_DISPATCH_ITEM_OP: Callee = Callee {
        addr: 0x0017_a158,
        f: ee::s02::f_0017a158,
    };

    pub const WSCRIPT_MISSION_CTOR: Callee = Callee {
        addr: 0x0030_7128,
        f: ee::s10::f_00307128,
    };
    pub const EA_MEM_ALLOC_NAMED: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::f_00317d70,
    };
    pub const WSCRIPT_PROCESS_LIST_PUSH_FRONT: Callee = Callee {
        addr: 0x0030_b2c0,
        f: ee::s10::f_0030b2c0,
    };

    pub const MULTIPARTICLE_MAN_CLEAR: Callee = Callee {
        addr: 0x0035_7b38,
        f: ee::s11::f_00357b38,
    };
    pub const MULTIPARTICLE_CTOR: Callee = Callee {
        addr: 0x0035_7e80,
        f: ee::s11::f_00357e80,
    };

    pub const MUSIC_PLAYER_SET_EVENT_STATE: Callee = Callee {
        addr: 0x002b_3bc0,
        f: ee::s08::f_002b3bc0,
    };
    pub const MUSIC_PLAYER_GET_SONG_TAG: Callee = Callee {
        addr: 0x002b_49e0,
        f: ee::s08::f_002b49e0,
    };

    pub const UI_STATE_UPDATE: Callee = Callee {
        addr: 0x0039_e510,
        f: ee::s13::f_0039e510,
    };
    pub const OPTIONS_SET_VIBRATION: Callee = Callee {
        addr: 0x0018_d108,
        f: ee::s02::f_0018d108,
    };

    pub const STR_HASH: Callee = Callee {
        addr: 0x0031_7670,
        f: ee::s10::f_00317670,
    };
    pub const UI_SCREEN_FIND_OBJECT: Callee = Callee {
        addr: 0x0039_d860,
        f: ee::s13::f_0039d860,
    };
    pub const UI_MENU_FIND_FIRST_SELECTABLE: Callee = Callee {
        addr: 0x0039_b7b0,
        f: ee::s13::f_0039b7b0,
    };
    pub const UI_MENU_SET_SELECTION: Callee = Callee {
        addr: 0x0039_b960,
        f: ee::s13::f_0039b960,
    };

    pub const STRLEN: Callee = Callee {
        addr: 0x0041_6810,
        f: ee::s15::f_00416810,
    };

    pub const SECTION_MAN_DEQUEUE_SECTION: Callee = Callee {
        addr: 0x0022_e360,
        f: ee::s05::f_0022e360,
    };
    pub const SECTION_MAN_POP_FREE_NODE: Callee = Callee {
        addr: 0x0022_e278,
        f: ee::s05::f_0022e278,
    };

    pub const UI_LISTBOX_ADD_ITEM_STR_ID: Callee = Callee {
        addr: 0x0039_a568,
        f: ee::s13::f_0039a568,
    };
    pub const UI_LISTBOX_FIND_ITEM_BY_VALUE: Callee = Callee {
        addr: 0x0039_a768,
        f: ee::s13::f_0039a768,
    };
    pub const UI_LISTBOX_SET_SELECTION: Callee = Callee {
        addr: 0x0039_a7a8,
        f: ee::s13::f_0039a7a8,
    };

    pub const NET_FE_STATE_GET_POPUP: Callee = Callee {
        addr: 0x001a_9c50,
        f: ee::s03::f_001a9c50,
    };
    pub const NET_FE_STATE_SHOW_POPUP: Callee = Callee {
        addr: 0x001a_9bc0,
        f: ee::s03::f_001a9bc0,
    };
    pub const POPUP_CFG_SET_OPTION_TEXT_ID: Callee = Callee {
        addr: 0x001c_57c0,
        f: ee::s03::f_001c57c0,
    };

    pub const MESSAGE_CENTER_GET_INBOX: Callee = Callee {
        addr: 0x001e_39f8,
        f: ee::s04::f_001e39f8,
    };

    pub const CBXSTRING_CTOR_COPY: Callee = Callee {
        addr: 0x0031_7fe8,
        f: ee::s10::f_00317fe8,
    };
    pub const CBXSTRING_DTOR: Callee = Callee {
        addr: 0x0031_8260,
        f: ee::s10::f_00318260,
    };

    pub const SSX_AUDIO_IS_DJ_SPEECH_ON: Callee = Callee {
        addr: 0x0029_f0d8,
        f: ee::s08::f_0029f0d8,
    };
    pub const SPEECH_SYSTEM_SET_EVENT_CONTEXT: Callee = Callee {
        addr: 0x002b_1458,
        f: ee::s08::f_002b1458,
    };
    pub const SSX_AUDIO_GET_LANGUAGE_MASK: Callee = Callee {
        addr: 0x0029_f198,
        f: ee::s08::f_0029f198,
    };
    pub const SSX_AUDIO_GET_SPEECH_CONTEXT_MASK: Callee = Callee {
        addr: 0x002a_1bd8,
        f: ee::s08::f_002a1bd8,
    };

    pub const INI_PARSER_SKIP_WHITESPACE: Callee = Callee {
        addr: 0x002a_78f8,
        f: ee::s08::f_002a78f8,
    };
    pub const INI_PARSER_EXPECT: Callee = Callee {
        addr: 0x002a_7ef0,
        f: ee::s08::f_002a7ef0,
    };
    pub const INI_PARSER_GET_CHAR: Callee = Callee {
        addr: 0x002a_78e0,
        f: ee::s08::f_002a78e0,
    };

    pub const CHUNK_RESOLVE_RELOCATE_DMA: Callee = Callee {
        addr: 0x003a_c358,
        f: ee::s13::f_003ac358,
    };
    pub const FLAG_MANAGER_GROUP_BUILD_MESH: Callee = Callee {
        addr: 0x0034_b228,
        f: ee::s11::f_0034b228,
    };

    pub const BE_GET_RIDER_CHARACTER: Callee = Callee {
        addr: 0x0014_a080,
        f: ee::s01::f_0014a080,
    };
    pub const BE_GET_RIDER_PROFILE: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::f_0014a0e0,
    };
    pub const BE_GET_CHAR_PROFILE_EVENT_RECORD: Callee = Callee {
        addr: 0x0015_2398,
        f: ee::s01::f_00152398,
    };

    pub const OV_STATE_OVERLAY_UPDATE: Callee = Callee {
        addr: 0x0020_e900,
        f: ee::s05::f_0020e900,
    };
    pub const READ_PRIVATE_MESSAGE_SET_TEXT: Callee = Callee {
        addr: 0x0021_0bd8,
        f: ee::s05::f_00210bd8,
    };
    pub const NET_MANAGER: Callee = Callee {
        addr: 0x0025_8160,
        f: ee::s06::f_00258160,
    };
    pub const OV_SCREEN_PDA_TEMPLATE_CLOSE: Callee = Callee {
        addr: 0x0020_d190,
        f: ee::s05::f_0020d190,
    };

    pub const PATH_DATA_READ_INT: Callee = Callee {
        addr: 0x0026_ba68,
        f: ee::s07::f_0026ba68,
    };
    pub const PATH_DATA_READ_FLOAT: Callee = Callee {
        addr: 0x0026_ba88,
        f: ee::s07::f_0026ba88,
    };

    pub const SUPERPIPE_COMPUTE_DESIRED_EYE: Callee = Callee {
        addr: 0x0017_85f0,
        f: ee::s02::f_001785f0,
    };
    pub const SUPERPIPE_COMPUTE_LOOK_AT: Callee = Callee {
        addr: 0x0017_8758,
        f: ee::s02::f_00178758,
    };
    pub const CHASE_CAMERA_RESET_FOR_RIDER: Callee = Callee {
        addr: 0x0016_6c60,
        f: ee::s02::f_00166c60,
    };
    pub const CHASE_CAMERA_INIT_FROM_RIDER: Callee = Callee {
        addr: 0x0016_6550,
        f: ee::s02::f_00166550,
    };

    pub const MEMSET: Callee = Callee {
        addr: 0x0041_6210,
        f: ee::s15::f_00416210,
    };
    pub const QSORT: Callee = Callee {
        addr: 0x0041_8ef8,
        f: ee::s15::f_00418ef8,
    };
    pub const CROWD_MAN3D_GROUP_BUILD_MATRICES: Callee = Callee {
        addr: 0x0034_4aa0,
        f: ee::s11::f_00344aa0,
    };
    pub const CHASE_CAM_CONTROLLER_RESTORE_DEFAULT: Callee = Callee {
        addr: 0x0016_2290,
        f: ee::s01::f_00162290,
    };
    pub const SCRIPT_ENGINE_LIST_START: Callee = Callee {
        addr: 0x0027_8f68,
        f: ee::s07::f_00278f68,
    };
    pub const SCRIPT_ENGINE_SET_SKIP_DELAY: Callee = Callee {
        addr: 0x0027_8de8,
        f: ee::s07::f_00278de8,
    };
    pub const CUTSCENE_TRANSITION_MAN_FORCE_ON: Callee = Callee {
        addr: 0x002e_a860,
        f: ee::s09::f_002ea860,
    };
    pub const UI_STATE_MGR_FIND_STATE: Callee = Callee {
        addr: 0x0039_f9d8,
        f: ee::s13::f_0039f9d8,
    };
    pub const UI_STATE_SHUTDOWN: Callee = Callee {
        addr: 0x0039_e4a0,
        f: ee::s13::f_0039e4a0,
    };
    pub const UI_STATE_MGR_CLOSE_ALL_POPUPS: Callee = Callee {
        addr: 0x0039_f698,
        f: ee::s13::f_0039f698,
    };
    pub const GAME_COMM_RECV_GAME_PACKET: Callee = Callee {
        addr: 0x0025_8160,
        f: ee::s06::f_00258160,
    };

    pub const OPERATOR_NEW_NAMED: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::f_00317e30,
    };
    pub const EA_MEM_FREE: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::f_00317e98,
    };
    pub const USTR_LEN: Callee = Callee {
        addr: 0x003d_bca0,
        f: ee::s14::f_003dbca0,
    };
    pub const USTR_CPY: Callee = Callee {
        addr: 0x003d_be20,
        f: ee::s14::f_003dbe20,
    };
    pub const UITEXT_SET_WIDE_STRING: Callee = Callee {
        addr: 0x003a_0e90,
        f: ee::s13::f_003a0e90,
    };
}

// ---------------------------------------------------------------------------
// `cMCOverlayManager_GetDeviceTotalString` (0x2419f0): format one memory-card
// device's total space into a wide buffer. The device record is at
// `self + a1 * 0xe0`; a missing record, or one with any of three optional
// strings null, leaves the destination untouched.
pub fn mc_manager_format_slot_total_space(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x450);
        frame.save(c, S1, 0x430);
        c.g[V0][0] = 0xe0;
        c.mult(0, 17, 5, 2);
        frame.save(c, S2, 0x420);
        frame.save(c, S3, 0x410);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x440);
        frame.save_ra(c, 0x400);
        let dev = (c.g[S2][0] as u32).wrapping_add(c.g[S1][0] as u32);
        c.g[V0][0] = sx(dev);
        c.g[V1][0] = c.lw(dev.wrapping_add(0x350));
        let missing = c.g[V1][0] == 0;
        c.g[S3][0] = c.g[A2][0];
        // 7 cycles for the two skipped block tails, 6 for the middle one, 5
        // for the outer one (see the generated nesting).
        let mut tail = 7u64;
        'checks: {
            if missing {
                break 'checks;
            }
            c.cycles += 3;
            c.g[V1][0] = c.lw(dev.wrapping_add(0x344));
            let missing = c.g[V1][0] == 0;
            c.lq(16, frame.at(c, 0x440));
            if missing {
                tail = 6;
                break 'checks;
            }
            c.cycles += 3;
            c.g[V1][0] = c.lw(dev.wrapping_add(0x348));
            if c.g[V1][0] == 0 {
                c.lq(17, frame.at(c, 0x430));
                tail = 5;
                break 'checks;
            }
            c.cycles += 3;
            c.g[V1][0] = c.lw(dev.wrapping_add(0x34c));
            let missing = c.g[V1][0] == 0;
            c.g[A1][0] = 0x4a0000;
            if missing {
                tail = 6;
                break 'checks;
            }
            // Two wide strings then the formatted line.
            c.cycles += 14;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2ca0));
            call_ptr(c, 0x002c2540, 0x0024_1a54)?;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c98));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002c2540, 0x0024_1a68)?;
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S1][0] as u32)));
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x358));
            c.g[A1][0] = c.g[S0][0];
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, 0x002c26d0, 0x0024_1a80)?;
        }
        c.cycles += tail;
        frame.restore(c, S0, 0x440);
        frame.restore(c, S1, 0x430);
        frame.restore(c, S2, 0x420);
        frame.restore(c, S3, 0x410);
        frame.restore_ra(c, 0x400);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEPopupConfirm_LayoutPostText` (0x1c7040): re-lay a confirm popup's wrapped
// message text after its text object (`+0x78`) exists: copy the popup's
// position, bump y by `+0x264`, and update the label (and its alignment when
// `+0x2cc` is set). Called with the popup and a 3-vector source.
pub fn cfe_popup_confirm_layout_post_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        if c.g[V1][0] == 0 {
            c.lq(16, frame.at(c, 0x20));
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
            let empty = c.g[V0][0] == 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4c));
            if !empty {
                c.cycles += 25;
                // v = self->pos; v.y += self->+0x264; text->+0x44 = v.
                c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1bf));
                c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1b8));
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(2, (c.g[SP][0] as u32));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.g[AT][0] = 0x41200000;
                c.mtc1(1, 12);
                c.g[A0][0] = c.g[S0][0];
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x264));
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x260));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.add_s(0, 0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(8, (c.g[SP][0] as u32));
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x4b));
                c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4c), c.g[T1][0]);
                c.g[A3][0] = 0u64;
                callee::POPUP_SET_WRAPPED_TEXT.call(c, 0x001c_70c0)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2cc));
                let no_align = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if !no_align {
                    c.cycles += 4;
                    c.mtc1(0, 12);
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x78));
                    c.g[A2][0] = 0x11;
                    callee::POPUP_SET_WIDGET_ALIGN.call(c, 0x001c_70dc)?;
                }
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `BoundedCameraAlgorithmBase_ctor` (0x173208): run the rider-camera base
// constructor, install the vtable and the bounded-camera defaults, and return
// the object.
pub fn bounded_camera_algorithm_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001622b0, 0x0017_321c)?;
        c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x6434));
        c.g[V1][0] = 0x460000;
        c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x6430));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3b38));
        c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x642c));
        c.g[A0][0] = 0x500000;
        c.g[AT][0] = 0x40a00000;
        c.mtc1(1, 4);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xed0));
        c.g[AT][0] = 0x41200000;
        c.mtc1(1, 3);
        c.g[A1][0] = 0x5a;
        c.g[AT][0] = 0x428c0000;
        c.mtc1(1, 2);
        c.g[V0][0] = c.g[S0][0];
        c.g[AT][0] = 0x43c80000;
        c.mtc1(1, 0);
        c.g[AT][0] = 0x41a00000;
        c.mtc1(1, 1);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0x58));
        c.swc1(7, (c.g[S0][0] as u32).wrapping_add(0x5c));
        c.swc1(5, (c.g[S0][0] as u32).wrapping_add(0x60));
        c.swc1(4, (c.g[S0][0] as u32).wrapping_add(0x64));
        c.swc1(3, (c.g[S0][0] as u32).wrapping_add(0x68));
        c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0x6c));
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x70));
        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x78));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x74), 0u64);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.lq(3, (c.g[A0][0] as u32));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A1][0]);
        c.sq(3, (c.g[S0][0] as u32).wrapping_add(0x40));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x50), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `DebugMenu_FindPrevItem` (0x17a2c8): walk the menu ring backwards from
// `start + count - 1`, returning the first item the dispatch callback accepts
// (or -1).
pub fn debug_menu_find_prev_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0u64;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A2][0];
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.cycles += 5;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        c.div(0, 5, 3);
        c.g[V0][0] = c.hi[0];
        let nonempty = (c.g[V1][0] as i64) > 0;
        c.g[S0][0] = c.g[V0][0];
        let mut found = false;
        if nonempty {
            'walk: loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S3][0];
                callee::DEBUG_MENU_DISPATCH_ITEM_OP.call(c, 0x0017_a320)?;
                let hit = c.g[V0][0] != 0;
                c.g[V0][0] = c.g[S0][0];
                if hit {
                    found = true;
                    break 'walk;
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                c.g[A1][0] = ((c.g[S1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                c.div(0, 3, 4);
                c.g[V0][0] = c.hi[0];
                let again = c.g[A1][0] != 0;
                c.g[S0][0] = c.g[V0][0];
                if !again {
                    break 'walk;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_a310);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
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
// `WScriptMan_CreateMission` (0x309568): allocate and construct a mission
// script, push it on the front or back process list, and return it.
pub fn wscript_man_create_mission(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S4, 0x10);
        c.g[S4][0] = c.g[A0][0];
        c.g[A1][0] = 0x490000;
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[T0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x66e8));
        c.g[A0][0] = 0x78;
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x0030_95b0)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S4][0];
        callee::WSCRIPT_MISSION_CTOR.call(c, 0x0030_95c8)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S3][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x2bc));
            c.g[A1][0] = c.g[S0][0];
            callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT.call(c, 0x0030_95dc)?;
        } else {
            c.cycles += 4;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x2b4));
            c.g[A1][0] = c.g[S0][0];
            callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT.call(c, 0x0030_95f0)?;
        }
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 8;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `MultiParticleMan_Load` (0x357d28): clear the manager, then for four stream
// entries read a `(delta, fn)` pair; when the entry reports "present",
// allocate and construct the particle there, otherwise store null.
pub fn multiparticle_man_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[V0][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0x3;
        frame.save_ra(c, 0x10);
        c.g[S3][0] = 0x490000;
        callee::MULTIPARTICLE_MAN_CLEAR.call(c, 0x0035_7d58)?;
        loop {
            c.cycles += 14;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32));
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = 0x4;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, target, 0x0035_7d78)?;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32));
            c.g[A0][0] = 0x50;
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1600));
            c.g[A2][0] = 0x20000000;
            let absent = c.g[V0][0] == 0;
            c.g[A3][0] = 0u64;
            if absent {
                c.cycles += 1;
                c.sw((c.g[S0][0] as u32), 0u64);
            } else {
                c.cycles += 7;
                call_ptr(c, 0x00317d70, 0x0035_7d98)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S1][0];
                callee::MULTIPARTICLE_CTOR.call(c, 0x0035_7da4)?;
                c.sw((c.g[S0][0] as u32), c.g[V0][0]);
            }
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_7d58);
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

// ---------------------------------------------------------------------------
// `cSetupMan_ClientRecvMsg11` (0x269b90): store the message id in the caller's
// word, allocate a 12-byte message node, fill kind/vtable/extra, allocate and
// link a list node, and splice it in after the head stored at `gp-0xc8`+0xf0.
// The 0xFD/0xFE constructors differ only in those constants and the extra
// word (`cSetupMan_ClientRecvMsg10/12/13` are the same shape).
fn recv_msg_node(
    c: &mut Cpu,
    kind: u64,
    vt: u32,
    extra: u64,
    ret1: u32,
    ret2: u32,
) -> crate::call::Flow {
    c.sw((c.g[A0][0] as u32), c.g[A1][0]);
    c.g[A1][0] = 0x480000;
    c.g[A0][0] = 0xc;
    c.g[A2][0] = 0x20000000;
    c.g[A3][0] = 0u64;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf18));
    call_ptr(c, 0x0031_7d70, ret1)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V1][0] = kind;
    c.sw((c.g[S0][0] as u32), c.g[V1][0]);
    c.g[V0][0] = 0x480000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(vt));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), extra);
    c.g[A1][0] = 0x480000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
    c.g[A0][0] = 0xc;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    c.g[A2][0] = 0x20000000;
    c.g[A3][0] = 0u64;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xf0));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
    call_ptr(c, 0x0031_7d70, ret2)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
    if c.g[V0][0] != 0 {
        c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
    }
    // Splice the node after the head.
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
    c.sw((c.g[A1][0] as u32), c.g[V0][0]);
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
    c.sw((c.g[A0][0] as u32), c.g[A1][0]);
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
    c.sw((c.g[SP][0] as u32), c.g[A1][0]);
    Ok(())
}

/// `cSetupMan_ClientRecvMsg11` (0x269b90): queue message kind 0xFD.
pub fn csetup_man_client_recv_msg11(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        recv_msg_node(c, 0xfd, 0x13e0, 0, 0x0026_9bb8, 0x0026_9bf8)?;
        c.cycles += 13;
        c.lq(16, frame.at(c, 0x30));
        frame.close(c);
        Ok(())
    })
}

/// `cSetupMan_ClientRecvMsg13` (0x269cf0): queue message kind 0xFE.
pub fn csetup_man_client_recv_msg13(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        recv_msg_node(c, 0xfe, 0x13c0, 0, 0x0026_9d18, 0x0026_9d58)?;
        c.cycles += 13;
        c.lq(16, frame.at(c, 0x30));
        frame.close(c);
        Ok(())
    })
}

/// `cSetupMan_ClientRecvMsg10` (0x269ae0): queue message kind 0xFD with the
/// extra word 1.
pub fn csetup_man_client_recv_msg10(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        recv_msg_node(c, 0xfd, 0x13e0, 1, 0x0026_9b08, 0x0026_9b4c)?;
        c.cycles += 13;
        c.lq(16, frame.at(c, 0x30));
        frame.close(c);
        Ok(())
    })
}

/// `cSetupMan_ClientRecvMsg12` (0x269c40): queue message kind 0xFE with the
/// extra word 1.
pub fn csetup_man_client_recv_msg12(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        recv_msg_node(c, 0xfe, 0x13c0, 1, 0x0026_9c68, 0x0026_9cac)?;
        c.cycles += 13;
        c.lq(16, frame.at(c, 0x30));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `SSXAudioSystem_OnMusicSetEventState` (0x28f3c8): set the bank monitor's
// event state and, when the song tag is no longer -1 and the code is one of
// 10/11/18, optionally issue the monitor's `v3(8)` call and clear `+0x530`.
pub fn ssx_audio_system_on_music_set_event_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002b3bc0, 0x0028_f3f4)?;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x002b49e0, 0x0028_f400)?;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        let tagged = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = c.g[S3][0];
        if tagged {
            c.g[V0][0] = c.g[S3][0];
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xa));
            c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
            let interesting = c.g[V0][0] != 0;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x530));
            let mut act = true;
            if !interesting {
                c.cycles += 3;
                c.g[V0][0] = 0x12;
                let other = c.g[S0][0] != c.g[V0][0];
                c.g[V0][0] = c.g[S3][0];
                if other {
                    act = false;
                } else {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x530));
                }
            }
            if act {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                if c.g[V1][0] != c.g[V0][0] {
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x530), 0u64);
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5558));
                    c.g[A1][0] = 0x8;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, target, 0x0028_f450)?;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x530), 0u64);
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S3][0];
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
// `cFEStateOptionsController_Update` (0x18d058): update the screen, then tick
// down the two vibration timers; an expired timer clears its bit and tells
// the controller to stop vibrating that motor. The returned bits select the
// motors.
pub fn cfe_state_options_controller_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039e510, 0x0018_d06c)?;
        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = c.g[V1][0] & 0x1;
        let no_first = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[V1][0] & 0x2;
        if !no_first {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
            let expired = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            if !expired {
                c.cycles += 2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
            } else {
                c.cycles += 8;
                c.mtc1(0, 12);
                c.g[V0][0] = c.g[V1][0] & 0xfe;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.g[A0][0] = c.g[S0][0];
                c.sb((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                c.g[A1][0] = 0u64;
                c.mov_s(13, 12);
                callee::OPTIONS_SET_VIBRATION.call(c, 0x0018_d0b0)?;
            }
            c.cycles += 2;
            c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.g[V1][0] & 0x2;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.lq(16, frame.at(c, 0x10));
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
            let expired = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            if !expired {
                c.cycles += 2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
            } else {
                c.cycles += 8;
                c.mtc1(0, 12);
                c.g[V0][0] = c.g[V1][0] & 0xfd;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[A0][0] = c.g[S0][0];
                c.sb((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                c.g[A1][0] = 0x1;
                c.mov_s(13, 12);
                callee::OPTIONS_SET_VIBRATION.call(c, 0x0018_d0f4)?;
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x10));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `MemCardStateBase_SyncMenuSelection` (0x1d9258): look up the memory-card
// menu by name hash, choose the selected item (first selectable when there is
// no current item or it is disabled), apply it, and remember its index.
pub fn memcard_state_base_sync_menu_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x4a0000;
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1600));
        call_ptr(c, 0x00317670, 0x001d_927c)?;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x001d_9288)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
            let no_item = c.g[V0][0] == 0;
            c.g[S1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x95));
            let mut find_first = no_item;
            if !no_item {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                find_first = c.g[V0][0] == 0;
                c.g[V0][0] = 0xff;
            }
            if find_first {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0039b7b0, 0x001d_92c0)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V0][0] = 0xff;
            }
            c.cycles += 2;
            let current = c.g[S1][0] == c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            if !current {
                c.cycles += 4;
                c.g[A1][0] = c.g[S1][0] & 0xff;
                call_ptr(c, 0x0039b960, 0x001d_92d8)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x1c4), c.g[S1][0]);
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0039b960, 0x001d_92e8)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x1c4), 0u64);
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x30));
        } else {
            c.lq(16, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `BENewPlayerInt_GetName` (0x147170): the player-name interface: if the
// fixed 28-byte name slot for `id` is empty, format the profile's name into it
// and copy it to ASCII; either way return the slot.
pub fn benew_player_int_get_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x120);
        c.g[V1][0] = 0x1c;
        frame.save(c, S3, 0xe0);
        c.g[V0][0] = 0x530000;
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0xf0);
        c.mult(0, 3, 19, 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4fe0));
        frame.save(c, S0, 0x110);
        frame.save(c, S1, 0x100);
        frame.save_ra(c, 0xd0);
        let name = (c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32);
        c.g[S2][0] = sx(name);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x00416810, 0x0014_71a8)?;
        let has_name = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S2][0];
        if !has_name {
            c.cycles += 20;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5d98));
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            call_ptr(c, 0x00317670, 0x0014_71d0)?;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
            c.g[A0][0] = c.g[S0][0];
            let target = c.g[V1][0] as u32;
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, target, 0x0014_71e0)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
            call_ptr(c, 0x002c26d0, 0x0014_71f0)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x002c2580, 0x0014_71fc)?;
            c.g[V0][0] = c.g[S2][0];
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x110);
        frame.restore(c, S1, 0x100);
        frame.restore(c, S2, 0xf0);
        frame.restore(c, S3, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `GMHandler_Start` (0x238c80): reset the game-mode singleton's fields, clear
// its two 0x28-byte arrays and dispatch its entry vtable function.
pub fn gm_handler_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[S0][0] = c.g[A0][0];
        let gm = c.g[V0][0] as u32;
        c.sw(gm.wrapping_add(0x84), c.g[A1][0]);
        c.g[A0][0] = sx(gm.wrapping_add(0x18));
        c.sw(gm.wrapping_add(0x8), c.g[V1][0]);
        c.sw(gm.wrapping_add(0x4), c.g[V1][0]);
        c.g[A1][0] = 0u64;
        c.sw(gm.wrapping_add(0x68), c.g[V1][0]);
        c.g[A2][0] = 0x28;
        c.sw(gm.wrapping_add(0x6c), c.g[V1][0]);
        c.sw(gm.wrapping_add(0x98), 0u64);
        c.sw(gm, 0u64);
        c.sw(gm.wrapping_add(0x70), 0u64);
        c.sw(gm.wrapping_add(0x74), 0u64);
        c.sw(gm.wrapping_add(0x10), 0u64);
        c.sw(gm.wrapping_add(0x14), 0u64);
        c.sw(gm.wrapping_add(0x78), 0u64);
        c.sw(gm.wrapping_add(0x7c), 0u64);
        c.sw(gm.wrapping_add(0x80), 0u64);
        c.sw(gm.wrapping_add(0x88), 0u64);
        c.sw(gm.wrapping_add(0x8c), 0u64);
        c.sw(gm.wrapping_add(0x90), 0u64);
        c.sw(gm.wrapping_add(0x94), 0u64);
        call_ptr(c, 0x00416210, 0x0023_8cf4)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x28;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x00416210, 0x0023_8d08)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x0023_8d1c)?;
        c.lq(16, frame.at(c, 0x10));
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

// ---------------------------------------------------------------------------
// `CrowdMan3d_GroupUpdate` (0x3449f0): advance the group's debounce clock,
// wrapping past the period, update every item's matrix, then rebuild the
// group's matrices.
pub fn crowd_man3d_group_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        // time += step, wrap at period.
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x1dc));
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x1e4));
        c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x1e0));
        c.add_s(0, 1, 0);
        c.c_lt_s(2, 0);
        // fcc: period < time + step -> past the period, so wrap.
        let wrap = c.fcc();
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x1dc));
        if wrap {
            c.cycles += 2;
            c.sub_s(0, 0, 2);
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x1dc));
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10c));
        let no_items = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0u64;
        if !no_items {
            c.cycles += 3;
            c.g[S2][0] = 0u64;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1ec));
            loop {
                c.cycles += 13;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x1dc));
                c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xd0));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, target, 0x0034_4a6c)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10c));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1ec));
                if event_due(c) {
                    deliver_event_here(c, 0x0034_4a48);
                }
            }
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00344aa0, 0x0034_4a84)?;
        c.lq(16, frame.at(c, 0x30));
        c.lq(17, frame.at(c, 0x20));
        c.lq(18, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Emitter_SetDescPathCoeffs` (0x36cef8): build the emitter's three descriptor
// vectors from the (b, c) pair — `a - 3*(b + c)`, `2*b`, `2*c` — with the
// original's COP2 macro operations.
pub fn emitter_set_desc_path_coeffs(c: &mut Cpu) {
    c.cycles += 43;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.g[AT][0] = 0x40400000;
    c.mtc1(1, 1);
    c.lqc2(3, c.g[A2][0] as u32);
    c.lqc2(4, c.g[A3][0] as u32);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[AT][0] = 0x40000000;
    c.mtc1(1, 0);
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.mfc1(3, 1);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lqc2(3, c.g[A1][0] as u32);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0xb0));
    c.sq(2, (c.g[SP][0] as u32));
    c.mfc1(3, 0);
    c.lqc2(4, c.g[A2][0] as u32);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0xc0));
    c.sq(2, (c.g[SP][0] as u32));
    c.mfc1(3, 0);
    c.lqc2(4, c.g[A3][0] as u32);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0xd0));
    c.sq(2, (c.g[SP][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// `cPS2ChunkResolve_v13` (0x3ac2a8): resolve a chunk instance through the
// two-level lookup tables, write the resolved pointers/flags into the
// instance and relocate its DMA chain. Returns 0 when the id has no
// resource, 1 when it does.
pub fn cps2_chunk_resolve_v13(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[A1][0];
        c.g[V1][0] = c.lw((c.g[T0][0] as u32));
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x80));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        let empty = c.g[V1][0] == 0;
        c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
        // The generated body has no early return: both misses charge one cycle,
        // zero V0 and fall into the relocate block below.
        if !empty {
            c.cycles += 7;
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let miss = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            if miss {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 21;
        c.sw((c.g[A3][0] as u32).wrapping_add(0x80), c.g[V0][0]);
        c.g[A2][0] = 0xffff_ffff_ffff_0000;
        c.g[A0][0] = c.g[T0][0];
        c.g[A1][0] = c.g[A3][0];
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.sw((c.g[A3][0] as u32).wrapping_add(0x8c), 0u64);
        c.sw((c.g[A3][0] as u32).wrapping_add(0x94), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
        c.sw((c.g[A3][0] as u32).wrapping_add(0x88), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
        c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x2;
        c.sw((c.g[A3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        callee::CHUNK_RESOLVE_RELOCATE_DMA.call(c, 0x003a_c344)?;
        c.g[V0][0] = 0x1;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
// ---------------------------------------------------------------------------
// `GameStateFinish_Exit` (0x234008): restore the default camera on every
// rider, then arm the script engine's skip delay/list and set the finish
// state.
pub fn game_state_finish_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[S1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
        let none = (c.g[S1][0] as i64) <= 0;
        c.g[S0][0] = 0u64;
        if !none {
            c.cycles += 2;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            loop {
                c.cycles += 11;
                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa8));
                call_ptr(c, 0x00162290, 0x0023_4060)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0023_4040);
                }
            }
        }
        c.cycles += 18;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
        c.g[V0][0] = 0x2;
        c.g[A2][0] = 0x1;
        c.g[A3][0] = 0u64;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x00278f68, 0x0023_4088)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x00278de8, 0x0023_4094)?;
        c.g[V0][0] = 0x7;
        c.lq(16, frame.at(c, 0x30));
        c.sw((c.g[S2][0] as u32), c.g[V0][0]);
        c.lq(17, frame.at(c, 0x20));
        c.lq(18, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cOVStateOverlay_Shutdown` (0x20a430): stop the overlay's cut scene, close
// its PDA template, find and shut down its UI state, then run the base
// shutdown.
pub fn cov_state_overlay_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x50));
        let no_cutscene = c.g[V0][0] == 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        if no_cutscene {
            c.cycles += 1;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            if c.g[V0][0] == 0 {
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
                call_ptr(c, 0x002ea860, 0x0020_a464)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x50), 0u64);
                c.cycles += 1;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            }
        }
        c.cycles += 10;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20d0));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x00317670, 0x0020_a47c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039f9d8, 0x0020_a488)?;
        c.g[A3][0] = c.g[V0][0];
        let no_state = c.g[A3][0] == 0;
        c.g[A1][0] = 0x4;
        if !no_state {
            c.cycles += 6;
            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[A2][0] = 0u64;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, target, 0x0020_a4ac)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
        let no_pda = c.g[V0][0] == 0;
        if !no_pda {
            c.cycles += 2;
            callee::OV_SCREEN_PDA_TEMPLATE_CLOSE.call(c, 0x0020_a4c0)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0039e4a0, 0x0020_a4c8)?;
        c.lq(16, frame.at(c, 0x20));
        c.lq(17, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Avalanche_GetObjectMatrix` (0x2d9c00): find the avalanche node whose object
// id matches, copy its rotation matrix, scale it and store its position in
// the output matrix.
pub fn avalanche_get_object_matrix(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x4f0000;
    c.g[V1][0] = 0xbc00;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1890));
    c.g[A2][0] = c.g[V0][0];
    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x70));
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x2e0));
        let matched = if c.g[V0][0] == 0 {
            false
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xf8));
            c.g[V0][0] == c.g[A0][0]
        };
        if matched {
            c.cycles += 26;
            c.lqc2(1, c.g[V1][0] as u32);
            c.lqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
            c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
            c.sqc2(1, c.g[A1][0] as u32);
            c.sqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.sqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
            c.sqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0xb0));
            c.mfc1(3, 0);
            c.lqc2(4, c.g[A1][0] as u32);
            c.qmtc2(3, 3);
            c.lqc2(5, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.lqc2(6, (c.g[A1][0] as u32).wrapping_add(0x20));
            c.lqc2(7, (c.g[A1][0] as u32).wrapping_add(0x30));
            c.vu0().mul(0b1111, 8, 4, Src::Bc(3, X));
            c.vu0().mul(0b1111, 9, 5, Src::Bc(3, X));
            c.vu0().mul(0b1111, 10, 6, Src::Bc(3, X));
            c.vu0().mul(0b1111, 11, 7, Src::Bc(3, X));
            c.sqc2(8, c.g[A1][0] as u32);
            c.sqc2(9, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.sqc2(10, (c.g[A1][0] as u32).wrapping_add(0x20));
            c.sqc2(11, (c.g[A1][0] as u32).wrapping_add(0x30));
            c.lq(2, (c.g[A2][0] as u32).wrapping_add(0x60));
            c.sq(2, (c.g[A1][0] as u32).wrapping_add(0x30));
            abi::ret(c);
            return;
        }
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2f0));
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2f0));
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002d_9c18);
        }
    }
}

// ---------------------------------------------------------------------------
// `cFlagManager_GroupLoad` (0x34c378): let the stream create the group's mesh
// data, fill each group item from the stream, then build the mesh for the
// first item.
pub fn cflag_manager_group_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0x60;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0u64;
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw((c.g[S3][0] as u32));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x0034_c3b8)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x94));
        if !empty {
            c.cycles += 1;
            loop {
                c.cycles += 11;
                c.g[V1][0] = c.lw((c.g[S3][0] as u32));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, target, 0x0034_c3e0)?;
                c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_c3c8);
                }
            }
        }
        c.cycles += 2;
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = c.g[S1][0];
        if !none {
            c.cycles += 2;
            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x94));
            callee::FLAG_MANAGER_GROUP_BUILD_MESH.call(c, 0x0034_c408)?;
        }
        c.cycles += 7;
        c.lq(16, frame.at(c, 0x40));
        c.lq(17, frame.at(c, 0x30));
        c.lq(18, frame.at(c, 0x20));
        c.lq(19, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateRequestLine_SortSongList` (0x197ad8): stable-partition the song
// list by the selection mask into `list`: first the selected indices, then
// the unselected ones.
pub fn cfe_state_request_line_sort_song_list(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14c));
    c.g[A1][0] = 0u64;
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[A2][0] = 0u64;
    if !empty {
        c.cycles += 2;
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4c));
        c.g[V0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x160));
        loop {
            c.cycles += 6;
            c.g[V0][0] = c.g[V0][0] >> (c.g[A1][0] & 63);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.sw((c.g[V1][0] as u32), c.g[A1][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            }
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14c));
            c.cycles += 4;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14c));
                break;
            }
            c.g[V0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x160));
            if event_due(c) {
                deliver_event_here(c, 0x0019_7af0);
            }
        }
    }
    c.cycles += 2;
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[V1][0] = 0u64;
    if !empty {
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4c));
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[V0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x160));
        loop {
            c.cycles += 6;
            c.g[V0][0] = c.g[V0][0] >> (c.g[V1][0] & 63);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.sw((c.g[A1][0] as u32), c.g[V1][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
            }
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14c));
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x160));
            if event_due(c) {
                deliver_event_here(c, 0x0019_7b48);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// `cFEStateProfileLoad_v38` (0x18f0b8): when the profile mode is not 2,
// toggle the two option widgets (`on` selects `!on`) and remember the state.
pub fn cfe_state_profile_load_v38(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1b08));
        frame.save(c, S1, 0x20);
        c.g[V0][0] = 0x2;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        let skip = c.g[V1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if !skip {
            c.cycles += 25;
            c.g[A0][0] = 0x4a0000;
            c.g[S0][0] = c.g[S2][0] ^ 0x1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1458));
            call_ptr(c, 0x00317670, 0x0018_f0f0)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x0018_f0fc)?;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, target, 0x0018_f114)?;
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1460));
            call_ptr(c, 0x00317670, 0x0018_f120)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x0018_f12c)?;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, target, 0x0018_f144)?;
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0] ^ 0x1;
        c.lq(16, frame.at(c, 0x30));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x220), c.g[V0][0]);
        c.lq(17, frame.at(c, 0x20));
        c.lq(18, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `BENewRaceInt_GetEventRecordTarget` (0x1456a0): map a (row, mode) pair to a
// score/record table value; modes 6..11 are invalid (-1). Rows below 14 use
// the first table, others the second; modes 4/5 use the neighbouring entries
// and mode 4 the base value while 5 scales by 100.
pub fn benew_race_int_get_event_record_target(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x9));
    c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
    let invalid = c.g[V0][0] != 0;
    c.g[V0][0] = 0x6;
    // -1 result shared by every early exit.
    let reject = |c: &mut Cpu| {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        abi::ret(c);
    };
    if invalid {
        reject(c);
        return;
    }
    c.cycles += 2;
    let is6 = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 0x7;
    if is6 {
        reject(c);
        return;
    }
    c.cycles += 2;
    let is7 = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 0x8;
    if is7 {
        reject(c);
        return;
    }
    c.cycles += 2;
    let not8 = c.g[A2][0] != c.g[V0][0];
    c.g[V0][0] = ((c.g[A1][0] as i64) < 14i64) as u64;
    if !not8 {
        reject(c);
        return;
    }
    c.cycles += 2;
    let row_below_14 = c.g[V0][0] != 0;
    let mut base_path = false;
    if !row_below_14 {
        // Row >= 14: only modes 4 and 5 are valid.
        c.g[V0][0] = 0x4;
        c.cycles += 2;
        let is4 = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 16);
        if is4 {
            base_path = true;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x5;
            let not5 = c.g[A2][0] != c.g[V0][0];
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 16);
            if not5 {
                reject(c);
                return;
            }
            c.cycles += 8;
            c.g[V0][0] = 0x440000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xe80));
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 14) as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = 0x64;
            c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xe));
            c.mult(0, 2, 2, 4);
            abi::ret(c);
            return;
        }
    } else {
        // Row < 14: mode 0 is the base value, anything else scales by 100.
        c.cycles += 2;
        let mode_not0 = c.g[A2][0] != 0u64;
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 16);
        if !mode_not0 {
            base_path = true;
        } else {
            c.cycles += 8;
            c.g[V1][0] = 0x440000;
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 14) as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xe80));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = 0x64;
            c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x2));
            c.mult(0, 2, 2, 4);
            abi::ret(c);
            return;
        }
    }
    if base_path {
        c.cycles += 6;
        c.g[V1][0] = 0x440000;
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 14) as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xe80));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x2));
        abi::ret(c);
    }
}

// ---------------------------------------------------------------------------
// `OVScreenReadPrivateMessage_Update` (0x210d20): run the overlay update; if
// the net object is idle, close its popups and set the message flag; when the
// screen has content, read the pending message into it.
pub fn ov_screen_read_private_message_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::OV_STATE_OVERLAY_UPDATE.call(c, 0x0021_0d38)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
        let no_net = c.g[V1][0] == 0;
        if no_net {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x68));
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = 0u64;
            let mut closed = idle;
            if !idle {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
                closed = c.g[V0][0] == 0;
            }
            if closed {
                c.cycles += 1;
                c.g[A0][0] = 0x1;
            }
            c.cycles += 2;
            if !closed {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            } else {
                c.cycles += 9;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039f698, 0x0021_0d74)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[V1][0] = c.g[V1][0] | 0x780;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            }
        }
        c.cycles += 2;
        let empty = c.g[V0][0] == 0;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
        if !empty {
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0x41;
            call_ptr(c, 0x00258160, 0x0021_0da4)?;
            let no_msg = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !no_msg {
                c.cycles += 3;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0u64;
                callee::READ_PRIVATE_MESSAGE_SET_TEXT.call(c, 0x0021_0db8)?;
            }
        }
        c.cycles += 5;
        c.lq(16, frame.at(c, 0x20));
        c.lq(17, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `BEMissionInt_LoadState` (0x154898): stream-load the selected character's
// two record blocks and the mission's own state into `self`.
pub fn bemission_int_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0u64;
        callee::BE_GET_RIDER_CHARACTER.call(c, 0x0015_48c0)?;
        c.g[V1][0] = 0xf88;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
        c.mult(0, 18, 2, 3);
        c.g[S0][0] = 0x4a0000;
        c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6cac));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
        c.g[A2][0] = 0x114;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        call_ptr(c, target, 0x0015_48ec)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x114));
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        c.g[A2][0] = 0x160;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x0015_490c)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32));
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
        c.g[A2][0] = 0x4;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x0015_4928)?;
        c.lq(16, frame.at(c, 0x40));
        c.lq(17, frame.at(c, 0x30));
        c.lq(18, frame.at(c, 0x20));
        c.lq(19, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `BEOptionInt_SwapPlayerFlags` (0x14eb08): exchange the two network player
// slots' flag words and ids (a hand-unrolled 6-word swap).
pub fn beoption_int_swap_player_flags(c: &mut Cpu) {
    c.cycles += 43;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[T1][0] = 0x530000;
    c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x8b8));
    c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
    c.ldr(2, (c.g[A1][0] as u32));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
    c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
    c.sdr(2, (c.g[SP][0] as u32));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.g[A2][0] = 0xffff_ffff_ffff_fff7;
    c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x8b8));
    c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x8b8));
    c.g[V0][0] = c.ld((c.g[SP][0] as u32));
    c.g[T2][0] = 0xffff_ffff_ffff_fffe;
    c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
    c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
    c.g[V0][0] = c.g[V0][0] << 29;
    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
    c.g[T3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.g[V0][0] & 0x1;
    c.g[A0][0] = c.g[A3][0] & c.g[A2][0];
    c.g[T0][0] = c.g[A3][0] & 0x8;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
    c.g[A2][0] = c.lw((c.g[SP][0] as u32));
    c.g[V1][0] = c.g[V1][0] | c.g[T0][0];
    c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
    c.g[V1][0] = c.g[V1][0] & c.g[T2][0];
    c.g[A0][0] = c.g[A0][0] & c.g[T2][0];
    c.g[A3][0] = c.g[A3][0] & 0x1;
    c.g[T2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
    c.g[A2][0] = c.g[A2][0] & 0x1;
    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.g[V1][0] | c.g[A3][0];
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = c.g[A0][0] | c.g[A2][0];
    c.sw((c.g[T1][0] as u32).wrapping_add(0x8b8), c.g[V1][0]);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T2][0]);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[T3][0]);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[T0][0]);
    c.sw((c.g[A1][0] as u32).wrapping_add(0xc), c.g[A0][0]);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// `BEScoreInt_GetEventBestPlace` (0x1558f8): return the event record's best
// place for a rider, or -1 when the profile is unavailable or out of range.
pub fn bescore_int_get_event_best_place(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x530000;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5f9));
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S0, 0x40);
        let blocked = c.g[V1][0] != 0;
        frame.save_ra(c, 0);
        if blocked {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            callee::BE_GET_RIDER_PROFILE.call(c, 0x0015_5930)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = (c.g[S0][0] < 0x3u64) as u64;
            let bad = c.g[V0][0] == 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if bad {
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            } else {
                c.cycles += 15;
                c.g[A0][0] = c.g[S1][0];
                callee::BE_GET_RIDER_CHARACTER.call(c, 0x0015_5948)?;
                c.g[V1][0] = 0xf88;
                c.g[A0][0] = 0x9b50;
                c.mult(0, 2, 2, 3);
                c.mult(0, 4, 16, 4);
                c.g[V1][0] = 0x4a0000;
                c.g[A1][0] = c.g[S2][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                c.g[A2][0] = c.g[S3][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                callee::BE_GET_CHAR_PROFILE_EVENT_RECORD.call(c, 0x0015_5974)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x1));
                }
            }
        }
        c.cycles += 7;
        c.lq(16, frame.at(c, 0x40));
        c.lq(17, frame.at(c, 0x30));
        c.lq(18, frame.at(c, 0x20));
        c.lq(19, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `GameAnimMap_BuildNameIndex` (0x314dd8): hash the 518 fixed animation names
// into `self->hash`, fill `self->order` with their indices, sort the order by
// hash and clear the map.
pub fn game_anim_map_build_name_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x490000;
        frame.save(c, S3, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[S3][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0u64;
        frame.save(c, S4, 0x10);
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3040));
        c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x818));
        frame.save_ra(c, 0);
        loop {
            c.cycles += 9;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32));
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x00317670, 0x0031_4e1c)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x818), c.g[S1][0]);
            c.sw((c.g[S0][0] as u32), c.g[V0][0]);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < 518i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_4e10);
            }
        }
        c.cycles += 20;
        c.g[A3][0] = 0x310000;
        c.g[A0][0] = c.g[S4][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4d60));
        c.g[A1][0] = 0x206;
        c.g[A2][0] = 0x4;
        c.sw((c.g[GP][0] as u32).wrapping_add(0xd88), c.g[S3][0]);
        call_ptr(c, 0x00418ef8, 0x0031_4e50)?;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1030));
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        c.g[A2][0] = 0x818;
        c.sw((c.g[GP][0] as u32).wrapping_add(0xd88), 0u64);
        call_ptr(c, 0x00416210, 0x0031_4e64)?;
        c.lq(16, frame.at(c, 0x50));
        c.lq(17, frame.at(c, 0x40));
        c.lq(18, frame.at(c, 0x30));
        c.lq(19, frame.at(c, 0x20));
        c.lq(20, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `UIAnim_EvalChannel` (0x3971c8): evaluate one animation channel at frame
// `t`: find the key interval containing it and interpolate; 0 outside.
pub fn uianim_eval_channel(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = c.lw((c.g[A0][0] as u32));
    if c.g[A0][0] == 0 {
        c.cycles += 3;
        c.mtc1(0, 0);
        abi::ret(c);
        return;
    }
    c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0xa));
    c.cycles += 10;
    c.g[T0][0] = 0x1;
    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x6));
    c.g[T2][0] = 0u64;
    c.mtc1(2, 3);
    c.cvt_s_w(3, 3);
    c.g[A3][0] = 0u64;
    c.mtc1(0, 1);
    c.g[V0][0] = (c.g[T0][0] < c.g[V1][0]) as u64;
    let single = c.g[V0][0] == 0;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
    if !single {
        c.cycles += 2;
        c.g[T1][0] = c.g[V1][0];
        loop {
            c.cycles += 8;
            c.g[A2][0] = c.lhu((c.g[A0][0] as u32));
            c.g[V1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x2));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A2][0] as u32)));
            c.mtc1(3, 1);
            c.cvt_s_w(1, 1);
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let past = c.g[V0][0] == 0;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
            if past {
                c.cycles += 10;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
                c.mtc1(6, 2);
                c.cvt_s_w(2, 2);
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.sub_s(1, 1, 3);
                c.div_s(0, 0, 2);
                c.mul_s(0, 0, 1);
                c.add_s(0, 0, 3);
                abi::ret(c);
                return;
            }
            c.cycles += 5;
            c.g[T2][0] = c.g[A3][0];
            c.mov_s(3, 1);
            c.g[V0][0] = (c.g[T0][0] < c.g[T1][0]) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_7238);
            }
        }
    }
    c.cycles += 2;
    c.mov_s(0, 1);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// `SectionMan_QueueSection` (0x22e298): remove every node of `id` from the
// live list (dequeuing each) and append a fresh node for it at the end.
pub fn section_man_queue_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x19c));
        let empty = c.g[S0][0] == 0;
        c.g[S3][0] = 0u64;
        if !empty {
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
            loop {
                if c.g[V0][0] != c.g[S2][0] {
                    c.cycles += 2;
                    c.g[S3][0] = c.g[S0][0];
                    c.cycles += 1;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                } else {
                    c.cycles += 2;
                    c.cycles += 7;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = c.g[S3][0];
                    callee::SECTION_MAN_DEQUEUE_SECTION.call(c, 0x0022_e2e4)?;
                }
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                if event_due(c) {
                    deliver_event_here(c, 0x0022_e2c8);
                }
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        callee::SECTION_MAN_POP_FREE_NODE.call(c, 0x0022_e300)?;
        c.g[S0][0] = c.g[V0][0];
        c.sw((c.g[S0][0] as u32), c.g[S2][0]);
        let no_prev = c.g[S3][0] == 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        if !no_prev {
            c.cycles += 2;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[S0][0]);
        } else {
            c.cycles += 1;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x19c), c.g[S0][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1a0));
        c.lq(16, frame.at(c, 0x40));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.lq(18, frame.at(c, 0x20));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1a0), c.g[V0][0]);
        c.lq(17, frame.at(c, 0x30));
        c.lq(19, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateCreateStep2_FillGenderList` (0x1bb520): add the two gender strings
// to the list box, select the entry matching `flag == 0`, and let the box
// apply its selection.
pub fn cfe_state_create_step2_fill_gender_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = 0xffff_ffff_ffff_fffe;
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x460000;
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4b10));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.sb((c.g[SP][0] as u32), 0u64);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x80;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        call_ptr(c, 0x00317670, 0x001b_b55c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x0039a568, 0x001b_b56c)?;
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4b20));
        call_ptr(c, 0x00317670, 0x001b_b578)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0x1;
        call_ptr(c, 0x0039a568, 0x001b_b588)?;
        c.g[A1][0] = (c.g[S1][0] < 0x1u64) as u64;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x0039a768, 0x001b_b598)?;
        c.g[A1][0] = c.lbu((c.g[SP][0] as u32));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039a7a8, 0x001b_b5a4)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0x1;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x001b_b5bc)?;
        c.lq(16, frame.at(c, 0x30));
        c.lq(17, frame.at(c, 0x20));
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateNetCreateRoomPopup_ValidateRoomName` (0x1bec78): if the room-name
// field is empty, configure the confirm popup for it and show it; return 1
// when a name is present.
pub fn net_create_room_popup_validate_room_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6dc));
        call_ptr(c, 0x00416810, 0x001b_ec9c)?;
        let has_name = c.g[V0][0] != 0;
        c.g[V0][0] = 0x1;
        if has_name {
            // return 1
        } else {
            c.cycles += 26;
            c.g[A0][0] = c.g[S3][0];
            c.g[S2][0] = 0x3;
            callee::NET_FE_STATE_GET_POPUP.call(c, 0x001b_ecb0)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 0x460000;
            c.g[V0][0] = 0x1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4fc0));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
            call_ptr(c, 0x00317670, 0x001b_eccc)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[S2][0]);
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
            call_ptr(c, 0x00317670, 0x001b_ece0)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0u64;
            callee::POPUP_CFG_SET_OPTION_TEXT_ID.call(c, 0x001b_ecf0)?;
            c.g[V0][0] = 0x847;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14c), c.g[S2][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S0][0];
            callee::NET_FE_STATE_SHOW_POPUP.call(c, 0x001b_ed08)?;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 7;
        c.lq(16, frame.at(c, 0x40));
        c.lq(17, frame.at(c, 0x30));
        c.lq(18, frame.at(c, 0x20));
        c.lq(19, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `MessageCenter_RemoveMessage` (0x1e31b8): shift the inbox entries after
// `idx` down by one, carry the selection bit down with them and decrement the
// count.
pub fn message_center_remove_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MESSAGE_CENTER_GET_INBOX.call(c, 0x001e_31cc)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xcc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A3][0] = 0x1;
            c.g[V0][0] = sx((c.g[S0][0] as u32) << 3);
            loop {
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[A2][0] = c.g[V1][0];
                c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
                c.ldr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.sdr(4, (c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[A3][0] as u32) << ((c.g[V1][0] as u32) & 31));
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc8));
                c.g[V1][0] = c.g[A0][0] & c.g[V1][0];
                let empty = c.g[V1][0] == 0;
                c.g[V0][0] = sx((c.g[A3][0] as u32) << ((c.g[S0][0] as u32) & 31));
                if !empty {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = !(0u64 | c.g[V0][0]);
                    c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                }
                c.cycles += 7;
                c.sw((c.g[A1][0] as u32).wrapping_add(0xc8), c.g[V0][0]);
                c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xcc));
                c.g[S0][0] = c.g[A2][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[S0][0] as u32) << 3);
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xcc));
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001e_31f0);
                }
            }
        } else {
            c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xcc));
        }
        c.cycles += 6;
        c.lq(16, frame.at(c, 0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.sb((c.g[A1][0] as u32).wrapping_add(0xcc), c.g[V0][0]);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `ChatEvent_Clone` (0x2680a8): deep-copy a chat event, cloning its two
// strings and returning the new object.
pub fn chat_event_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = 0x480000;
        frame.save(c, S2, 0x30);
        c.g[A2][0] = 0x20000000;
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[A3][0] = 0u64;
        frame.save_ra(c, 0x20);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x498));
        c.g[A0][0] = 0x10;
        call_ptr(c, 0x00317d70, 0x0026_80d8)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
        call_ptr(c, 0x00317fe8, 0x0026_80e8)?;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00317fe8, 0x0026_80f8)?;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = 0xde;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1230));
        c.g[A1][0] = c.g[SP][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
        c.sw((c.g[S0][0] as u32), c.g[V0][0]);
        call_ptr(c, 0x00317fe8, 0x0026_8118)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x00317fe8, 0x0026_8124)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x0026_8130)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x0026_813c)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, frame.at(c, 0x40));
        c.lq(16, frame.at(c, 0x50));
        c.lq(18, frame.at(c, 0x30));
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `SSXAudioSystem_DJSay212E`/`SSXAudioSystem_DJSay212F` (0x2a2cf0/0x2a2da0): if
// DJ speech is on, push speech context `id` (0x212e/0x212f) and hand the
// language and context masks to the speech system.
fn dj_say(c: &mut Cpu, id: u64, ret_base: u32) -> crate::call::Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S0, 0x30);
    frame.save(c, S2, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S1, 0x20);
    frame.save_ra(c, 0);
    c.g[S2][0] = c.g[A1][0];
    callee::SSX_AUDIO_IS_DJ_SPEECH_ON.call(c, ret_base)?;
    let off = c.g[V0][0] == 0;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
    if !off {
        c.cycles += 10;
        c.mtc1(0, 12);
        c.g[A1][0] = 0u64;
        c.g[A2][0] = id;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0xa;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT.call(c, ret_base + 0x28)?;
        if c.g[V0][0] != 0 {
            c.cycles += 18;
            c.g[A0][0] = c.g[S0][0];
            callee::SSX_AUDIO_GET_LANGUAGE_MASK.call(c, ret_base + 0x38)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = 0xffff_ffff_ffff_ffff;
            callee::SSX_AUDIO_GET_SPEECH_CONTEXT_MASK.call(c, ret_base + 0x4c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 0x1;
            c.g[A1][0] = 0u64;
            c.g[A2][0] = id;
            call_ptr(c, 0x003d_8008, ret_base + 0x60)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x173c));
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S0][0];
            let target = c.g[V1][0] as u32;
            c.g[A1][0] = 0x2;
            call_ptr(c, target, ret_base + 0x78)?;
        }
    }
    c.cycles += 1;
    c.lq(16, frame.at(c, 0x30));
    c.cycles += 5;
    c.lq(17, frame.at(c, 0x20));
    c.lq(18, frame.at(c, 0x10));
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `SSXAudioSystem_DJSay212E` (0x2a2cf0): queue DJ speech 0x212e.
pub fn ssx_audio_system_dj_say212e(c: &mut Cpu) {
    guest(c, |c| dj_say(c, 0x212e, 0x002a_2d10))
}

/// `SSXAudioSystem_DJSay212F` (0x2a2da0): queue DJ speech 0x212f.
pub fn ssx_audio_system_dj_say212f(c: &mut Cpu) {
    guest(c, |c| dj_say(c, 0x212f, 0x002a_2dc0))
}

// ---------------------------------------------------------------------------
// `IniParser_ReadQuotedString` (0x2a7cf0): skip whitespace, expect a quote,
// copy characters until the closing quote, expect a quote, terminate the
// string and return its buffer.
pub fn ini_parser_read_quoted_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save(c, S4, 0x10);
        c.g[S0][0] = 0x4a0000;
        frame.save(c, S3, 0x20);
        c.g[S4][0] = c.g[S0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0u64;
        callee::INI_PARSER_SKIP_WHITESPACE.call(c, 0x002a_7d20)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3728));
        callee::INI_PARSER_EXPECT.call(c, 0x002a_7d2c)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = 0x22;
        c.g[V0][0] = c.lb((c.g[V1][0] as u32));
        let quoted = c.g[V0][0] == c.g[A0][0];
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        if !quoted {
            c.cycles += 2;
            c.g[S3][0] = 0x22;
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                callee::INI_PARSER_GET_CHAR.call(c, 0x002a_7d50)?;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                c.sb((c.g[V1][0] as u32), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lb((c.g[V0][0] as u32));
                let at_quote = c.g[V1][0] == c.g[S3][0];
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                if at_quote {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002a_7d48);
                }
            }
        }
        c.cycles += 14;
        c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x3728));
        c.g[A0][0] = c.g[S1][0];
        callee::INI_PARSER_EXPECT.call(c, 0x002a_7d74)?;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        c.lq(16, frame.at(c, 0x50));
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
        c.lq(17, frame.at(c, 0x40));
        c.lq(18, frame.at(c, 0x30));
        c.lq(19, frame.at(c, 0x20));
        c.lq(20, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        c.sb((c.g[V1][0] as u32), 0u64);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `AI_ResetComputerRenderSlots`/`AI_ResetActorRenderSlots`: clear the "render
// slot taken" word of every rider of the AI's list, release the slot in the
// global graphics manager and store the slot number + 4.
fn ai_reset_render_slots(
    c: &mut Cpu,
    zero_off: u32,
    count_off: u32,
    items_off: u32,
    ret: u32,
) -> crate::call::Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    frame.save(c, S2, 0x30);
    frame.save(c, S3, 0x20);
    frame.save(c, S4, 0x10);
    frame.save_ra(c, 0);
    c.g[S3][0] = c.g[A0][0];
    c.sw((c.g[S3][0] as u32).wrapping_add(zero_off), 0u64);
    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(count_off));
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[S2][0] = 0u64;
    if !none {
        c.cycles += 2;
        c.g[S4][0] = 0xffff_ffff_ffff_ffff;
        c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(items_off));
        loop {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32));
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x884));
            if c.g[V1][0] != 0 {
                c.cycles += 13;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x888), c.g[S4][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x884), 0u64);
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[S0][0] = c.lw((c.g[S1][0] as u32));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x390));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x394));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, target, ret)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x888), c.g[V0][0]);
            }
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(count_off));
            c.cycles += 4;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, ret - 0x38);
            }
        }
    }
    c.cycles += 8;
    c.lq(16, frame.at(c, 0x50));
    c.lq(17, frame.at(c, 0x40));
    c.lq(18, frame.at(c, 0x30));
    c.lq(19, frame.at(c, 0x20));
    c.lq(20, frame.at(c, 0x10));
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `AI_ResetComputerRenderSlots` (0x12ab20): free every computer rider's
/// render slot.
pub fn ai_reset_computer_render_slots(c: &mut Cpu) {
    guest(c, |c| {
        ai_reset_render_slots(c, 0x14, 0x80, 0x48, 0x0012_ab90)
    })
}

/// `AI_ResetActorRenderSlots` (0x12ac48): free every actor rider's render slot.
pub fn ai_reset_actor_render_slots(c: &mut Cpu) {
    guest(c, |c| {
        ai_reset_render_slots(c, 0x8c, 0x84, 0x5c, 0x0012_acb8)
    })
}

// ---------------------------------------------------------------------------
// `FEModule_BaseDtor` (0x1957d0): the front-end module base destructor:
// release the async file, the world graphics manager, the rider previews, the
// bolt cache and the FX loader, install the base vtables and free the module
// when `flags & 1`.
pub fn femodule_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x470000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2788));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[S0][0] as u32), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        let no_file = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_file {
            c.cycles += 2;
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x0039_7b70, 0x0019_5804)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        call_ptr(c, 0x0036_a020, 0x0019_580c)?;
        c.g[V0][0] = 0xb0000;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5aa0));
        let none = c.g[A0][0] == 0;
        if none {
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1a70));
        } else {
            c.cycles += 3;
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x002d_bee0, 0x0019_5828)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1a70));
        }
        c.cycles += 13;
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0019_ce20, 0x0019_5834)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb0));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x001a_0420, 0x0019_5840)?;
        c.g[V0][0] = 0x460000;
        c.g[V1][0] = 0x470000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4750));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2690));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0] & 0x1;
        let keep = c.g[A0][0] == 0;
        c.sw((c.g[S0][0] as u32), c.g[V1][0]);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0019_5868)?;
        }
        c.cycles += 5;
        c.lq(16, frame.at(c, 0x20));
        c.lq(17, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cPS2MemCard_GetSlotName` (0x2c63f8): format the memory-card slot number
// (n+1) for the card icon, with the slot's free-space suffix when it is one
// of the first two slots.
pub fn cps2_memcard_get_slot_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[A0][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[A1][0] = 0x4a0000;
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0x20);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3970));
        call_ptr(c, 0x002c_2540, 0x002c_6428)?;
        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        call_ptr(c, 0x002c_26d0, 0x002c_643c)?;
        c.g[V0][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            let negative = (c.g[S0][0] as i64) < 0;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if !negative {
                c.cycles += 15;
                c.g[A0][0] = 0x480000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x64e8));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                call_ptr(c, 0x0031_7670, 0x002c_646c)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let target = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, target, 0x002c_647c)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x002c_26d0, 0x002c_648c)?;
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x60));
        } else {
            c.lq(16, frame.at(c, 0x60));
        }
        c.cycles += 6;
        c.lq(17, frame.at(c, 0x50));
        c.lq(18, frame.at(c, 0x40));
        c.lq(19, frame.at(c, 0x30));
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cKeyboardPopup_SetStaticWideText` (0x1ccf70): cache the static wide text on
// the popup: either clone it through the popup's text object or allocate a
// private copy.
pub fn keyboard_popup_set_static_wide_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
        let has_screen = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !has_screen {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x428));
            let none = c.g[A0][0] == 0;
            if !none {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x001c_cfa4)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x428), 0u64);
            }
            c.cycles += 15;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003d_bca0, 0x001c_cfb0)?;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6590));
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x0031_7e30, 0x001c_cfcc)?;
            c.g[A1][0] = c.g[S1][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x428), c.g[V0][0]);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x003d_be20, 0x001c_cfdc)?;
            c.lq(16, frame.at(c, 0x20));
        } else {
            c.cycles += 8;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6580));
            call_ptr(c, 0x0031_7670, 0x001c_cff0)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001c_cffc)?;
            let no_text = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !no_text {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003a_0e90, 0x001c_d00c)?;
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `SuperpipeChaseCameraAlgorithm_ResetForRider` (0x178938): if the rider's
// camera target id matches, recompute the desired eye and look-at, reset the
// chase camera for the rider and initialise it with the superpipe distances.
pub fn superpipe_chase_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        call_ptr(c, target, 0x0017_8970)?;
        if c.g[S1][0] != c.g[V0][0] {
            c.lq(16, frame.at(c, 0x30));
        } else {
            c.cycles += 24;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[S0][0];
            callee::SUPERPIPE_COMPUTE_DESIRED_EYE.call(c, 0x0017_8984)?;
            c.lq(2, (c.g[SP][0] as u32));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[S0][0];
            c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x40));
            callee::SUPERPIPE_COMPUTE_LOOK_AT.call(c, 0x0017_8998)?;
            c.lq(2, (c.g[SP][0] as u32));
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
            callee::CHASE_CAMERA_RESET_FOR_RIDER.call(c, 0x0017_89ac)?;
            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5d84));
            c.g[AT][0] = 0x43960000;
            c.mtc1(1, 13);
            c.g[A0][0] = c.g[S0][0];
            callee::CHASE_CAMERA_INIT_FROM_RIDER.call(c, 0x0017_89c0)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, target, 0x0017_89d4)?;
            c.lq(16, frame.at(c, 0x30));
        }
        c.cycles += 4;
        c.lq(17, frame.at(c, 0x20));
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Path_ReadEvent` (0x26a378): read an event key, look it up in the fixed
// 21-entry key table into `dst`, then read the event's delta and two floats.
pub fn path_read_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[A0][0] = c.g[SP][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        callee::PATH_DATA_READ_INT.call(c, 0x0026_a39c)?;
        c.sw((c.g[S1][0] as u32), 0u64);
        c.g[V0][0] = 0x440000;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.lw((c.g[SP][0] as u32));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x53d0));
        let first_differs = c.g[V1][0] != c.g[A0][0];
        c.g[A1][0] = 0u64;
        if !first_differs {
            c.cycles += 2;
        } else {
            c.cycles += 1;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            loop {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < 21i64) as u64;
                let out_of_range = c.g[V0][0] == 0;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x53d0));
                if out_of_range {
                    break;
                }
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                if c.g[V1][0] != c.g[A0][0] {
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_a3c4);
                    }
                    continue;
                }
                c.cycles += 1;
                c.sw((c.g[S1][0] as u32), c.g[A1][0]);
                break;
            }
            c.cycles += 1;
        }
        c.g[A1][0] = c.g[S0][0];
        c.cycles += 14;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        callee::PATH_DATA_READ_INT.call(c, 0x0026_a3f8)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = c.g[S0][0];
        callee::PATH_DATA_READ_FLOAT.call(c, 0x0026_a408)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        callee::PATH_DATA_READ_FLOAT.call(c, 0x0026_a414)?;
        c.lq(16, frame.at(c, 0x30));
        c.lq(17, frame.at(c, 0x20));
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Ports kept for later (no differential test yet; not in OVERRIDES).

/// `cFEStateRewardGallery_RequestShapeLoad` (0x1cfe60): ask the bolt manager
/// for the gallery's shape, or mark the request failed. Not listed: the
/// acquired path runs `BoltFile_Acquire`, which is not boundable on test data.
pub fn cfe_state_reward_gallery_request_shape_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0xc8));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xcc));
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1a70));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x001c_fe94)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
            c.cycles += 4;
            c.g[V1][0] = 0xffff_ffff_ffff_fffb;
            c.g[V0][0] = c.g[V0][0] | 0x8;
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
        } else {
            c.cycles += 19;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xcc));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, target, 0x001c_feb0)?;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A3][0] = 0x9;
            c.g[T0][0] = 0x1;
            c.g[T1][0] = 0u64;
            call_ptr(c, 0x0019_da10, 0x001c_fecc)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
            c.g[A0][0] = 0xffff_ffff_ffff_fff7;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8ec), c.g[V0][0]);
            c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
            c.g[V1][0] = c.g[V1][0] | 0x4;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V1][0]);
        }
        c.cycles += 5;
        c.lq(16, frame.at(c, 0x20));
        c.lq(17, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptMan_CallFuncOnInstance` (0x309d20): call a script function on the
/// world section's instance by name, with an empty argument and result value.
/// Not listed: it needs a live Luno VM to run, which the test fixtures cannot
/// provide cheaply.
pub fn wscript_man_call_func_on_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        frame.save_ra(c, 0x30);
        frame.save(c, S4, 0x40);
        c.g[S4][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x290));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28c));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x290), c.g[A2][0]);
        call_ptr(c, 0x003a_6d18, 0x0030_9d54)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x0022_6600, 0x0030_9d64)?;
        c.g[V0][0] = c.lbu((c.g[S1][0] as u32));
        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
        c.g[A0][0] = c.g[S3][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3cc));
        call_ptr(c, 0x0022_24a8, 0x0030_9d88)?;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0022_24b8, 0x0030_9da0)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x290), c.g[S4][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0022_5b90, 0x0030_9db0)?;
        c.lq(16, frame.at(c, 0x80));
        c.lq(17, frame.at(c, 0x70));
        c.lq(18, frame.at(c, 0x60));
        c.lq(19, frame.at(c, 0x50));
        c.lq(20, frame.at(c, 0x40));
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, fill, put_words, Rng, DATA, GAME_GP, RA as SENTINEL};
    use ssx3_ee as ee;

    const SEEDS: u32 = 120;

    const W: u32 = DATA + 0x5_0000;
    const RM: u32 = DATA + 0x5_1000;
    const RIDERS: u32 = DATA + 0x5_2000;
    const BE: u32 = DATA + 0x5_3000;
    const DB: u32 = DATA + 0x5_4000;
    const VTS: u32 = DATA + 0x5_5000;
    const MEM_HEAP: u32 = 0x004f_f250;

    fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, SENTINEL, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// A dummy world (`gp-0x848`), its player/bolt manager, the script DB and
    /// the BELibrary singleton (`gp-0x1f04`).
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.bus.write32(gp_at(-0x1f04), BE);
        c.bus.write32(gp_at(-0x848), W);
        c.bus.write32(W + 0x7c, RM);
        c.bus.write32(W + 0x84, RM);
        c.bus.write32(RM + 0x84, RIDERS);
        c.bus.write32(RIDERS + 0x10, 0);
        c.bus.write32(W + 0x8c, DB);
        c.bus.write32(DB + 4, VTS);
        r
    }

    /// [`base`] plus the size-class pool allocator and a dlmalloc arena.
    fn alloc_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = base(c, seed, salt);
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
            let block_base = nodes;
            for n in 0..16u32 {
                let node = block_base + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, block_base);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
        r
    }

    /// A `{delta = 0, fn = fake}` vtable pair at `off` in `vt`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, index: u32) {
        c.bus.write16(vt + off, 0);
        c.bus.write32(vt + off + 4, FAKE + 0x10 * index);
    }

    fn str_hash(c: &mut Cpu, s: u32) -> u32 {
        run(c, 0x0031_7670, ee::s10::f_00317670, &[s as u64]) as u32
    }

    /// A screen at `screen` whose object list holds `node` for the string at
    /// `name` (the zeroed game string). `+0x38` points at the object count and
    /// `+0x3c` at the object array, as `cUIScreen_getObjectByHashName` reads
    /// them; `node+0x38` is that string's hash.
    fn hash_screen(c: &mut Cpu, screen: u32, node: u32, name: u32) {
        let h = str_hash(c, name);
        let list = DATA + 0x5_6000;
        let count = DATA + 0x5_6010;
        c.bus.write32(screen + 0x38, count);
        c.bus.write32(screen + 0x3c, list);
        c.bus.write32(count, 1);
        c.bus.write32(list, node);
        c.bus.write32(node + 0x38, h);
    }

    fn wide(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write16(at + 2 * i as u32, *b as u16);
        }
    }

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    #[test]
    fn mc_manager_format_slot_total_space_matches() {
        check(
            ee::s06::f_002419f0__generated,
            mc_manager_format_slot_total_space,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x2419);
                let idx = (seed % 3) as u32;
                let dev = DATA + idx * 0xe0;
                let dst = DATA + 0x2000;
                let variant = seed / 3 % 5;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = sx(dst);
                c.bus.write32(dev + 0x350, if variant == 0 { 0 } else { 1 });
                c.bus.write32(dev + 0x344, if variant == 1 { 0 } else { 1 });
                c.bus.write32(dev + 0x348, if variant == 2 { 0 } else { 1 });
                c.bus.write32(dev + 0x34c, if variant == 3 { 0 } else { 1 });
                c.bus.write32(dev + 0x358, DATA + 0x3000);
                fill(c, &mut r, DATA + 0x3000, 0x40);
                fill(c, &mut r, dst, 0x100);
            },
        );
    }

    #[test]
    fn cfe_popup_confirm_layout_post_text_matches() {
        // No text object, or an empty source word (the full relayout calls
        // into the UI text engine).
        check(
            ee::s03::f_001c7040__generated,
            cfe_popup_confirm_layout_post_text,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x1c70);
                let src = DATA + 0x800;
                let text = DATA + 0x600;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(src);
                let no_text = seed % 2 == 0;
                c.bus.write32(DATA + 0x78, if no_text { 0 } else { text });
                c.bus.write32(src + 0x4c, if no_text { 1 } else { 0 });
                fill(c, &mut r, DATA + 0x1b8, 4);
                fill(c, &mut r, DATA + 0x260, 4);
                c.bus.write32(DATA + 0x264, f(1.5));
                fill(c, &mut r, text, 0x40);
            },
        );
    }

    #[test]
    fn bounded_camera_algorithm_base_ctor_matches() {
        check(
            ee::s02::f_00173208__generated,
            bounded_camera_algorithm_base_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x1732);
                c.g[A0][0] = sx(DATA);
                put_words(c, gp_at(-0x6434), &[f(0.8), f(10.3), f(1.1)]);
                fill(c, &mut r, DATA, 0x144);
            },
        );
    }

    #[test]
    fn debug_menu_find_prev_item_matches() {
        check(
            ee::s02::f_0017a2c8__generated,
            debug_menu_find_prev_item,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x17a2);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = 0x41;
                c.bus.write32(DATA, (seed % 4) as u32);
                for i in 0..4 {
                    c.bus.write32(DATA + 8 * i + 4, 0);
                }
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    #[test]
    fn wscript_man_create_mission_matches() {
        check(
            ee::s10::f_00309568__generated,
            wscript_man_create_mission,
            60,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x3095);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0x12;
                c.g[A2][0] = 0x34;
                c.g[A3][0] = 0x56;
                c.g[T0][0] = (seed % 2) as u64;
                c.bus.write32(DATA + 0x2b4, 0);
                c.bus.write32(DATA + 0x2bc, 0);
            },
        );
    }

    #[test]
    fn multiparticle_man_load_matches() {
        check(
            ee::s11::f_00357d28__generated,
            multiparticle_man_load,
            60,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x357d);
                let entries = DATA + 0x500;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                c.bus.write32(DATA + 0x400, entries);
                // the stream entry writes its 4-byte "present" flag at sp+0
                c.bus.write32(crate::difftest::SP - 0x60, 0);
                for i in 0..4 {
                    let e = entries + 8 * i;
                    c.bus.write16(e + 0x10, 0);
                    c.bus.write32(e + 0x14, FAKE + 0x10 * i);
                }
            },
        );
    }

    #[test]
    fn csetup_man_recv_msg11_matches() {
        check(
            ee::s07::f_00269b90__generated,
            csetup_man_client_recv_msg11,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2691);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed + 1) as u64;
                c.bus.write32(gp_at(-0xc8), DATA + 0x800);
                c.bus.write32(DATA + 0x8f0, DATA + 0x900);
                c.bus.write32(DATA + 0x900, 0);
            },
        );
    }

    #[test]
    fn csetup_man_recv_msg13_matches() {
        check(
            ee::s07::f_00269cf0__generated,
            csetup_man_client_recv_msg13,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2692);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed + 1) as u64;
                c.bus.write32(gp_at(-0xc8), DATA + 0x800);
                c.bus.write32(DATA + 0x8f0, DATA + 0x900);
            },
        );
    }

    #[test]
    fn csetup_man_recv_msg10_matches() {
        check(
            ee::s07::f_00269ae0__generated,
            csetup_man_client_recv_msg10,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2693);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed + 1) as u64;
                c.bus.write32(gp_at(-0xc8), DATA + 0x800);
                c.bus.write32(DATA + 0x8f0, DATA + 0x900);
            },
        );
    }

    #[test]
    fn csetup_man_recv_msg12_matches() {
        check(
            ee::s07::f_00269c40__generated,
            csetup_man_client_recv_msg12,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2694);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed + 1) as u64;
                c.bus.write32(gp_at(-0xc8), DATA + 0x800);
                c.bus.write32(DATA + 0x8f0, DATA + 0x900);
            },
        );
    }

        #[test]
    fn cfe_state_options_controller_update_matches() {
        check(
            ee::s02::f_0018d058__generated,
            cfe_state_options_controller_update,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x18d0);
                c.g[A0][0] = sx(DATA);
                c.bus.write8(DATA + 0x50, [0, 1, 2, 3][(seed % 4) as usize]);
                c.bus
                    .write32(DATA + 0x48, if seed % 3 == 0 { 0 } else { 2 });
                c.bus
                    .write32(DATA + 0x4c, if seed % 5 == 0 { 0 } else { 1 });
                fill(c, &mut r, DATA + 0x10, 0x40);
            },
        );
    }

    #[test]
    fn memcard_state_base_sync_menu_selection_matches() {
        check(
            ee::s04::f_001d9258__generated,
            memcard_state_base_sync_menu_selection,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x1d92);
                let screen = DATA + 0x1000;
                let node = DATA + 0x2000;
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0x40, if seed % 6 == 0 { 0 } else { screen });
                hash_screen(c, screen, node, 0x004a_1600);
                let item = DATA + 0x3000;
                let variant = seed % 4;
                c.bus
                    .write32(node + 0xa0, if variant == 0 { 0 } else { item });
                c.bus.write8(node + 0x95, (seed % 5) as u8);
                c.bus.write32(item + 0x14, variant << 5);
                c.bus.write32(node + 8, DATA + 0x4000);
                slot(c, DATA + 0x4000, 0x48, 1);
                fill(c, &mut r, DATA + 0x1c4, 0x10);
            },
        );
    }

    #[test]
    fn benew_player_int_get_name_matches() {
        check(
            ee::s01::f_00147170__generated,
            benew_player_int_get_name,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1471);
                let idx = (seed % 4) as u32;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = idx as u64;
                let name = 0x0053_4fe0 + idx * 0x1c;
                if seed % 3 == 0 {
                    c.bus.write8(name, 0);
                } else {
                    c.bus.write8(name, b'A');
                    c.bus.write8(name + 1, 0);
                }
                slot(c, VTS, 0x20, 0);
                fill(c, &mut r, DATA + 0x800, 0x40);
            },
        );
    }

    #[test]
    fn gm_handler_start_matches() {
        check(ee::s06::f_00238c80__generated, gm_handler_start, 40, &|c, seed| {
            let mut r = base(c, seed, 0x238c);
            let gm = DATA + 0x2000;
            c.g[A0][0] = sx(DATA);
            c.bus.write32(gp_at(-0x480), gm);
            c.bus.write32(gm, DATA + 0x3000);
            slot(c, DATA + 0x3000, 0x28, 1);
            fill(c, &mut r, gm, 0x100);
        });
    }

    #[test]
    fn bolts_find_first_child_matches() {
        check(
            ee::s01::f_0014d558__generated,
            bolts_find_first_child,
            150,
            &|c, seed| {
                let mut r = base(c, seed, 0x14d5);
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
    fn crowd_man3d_group_update_matches() {
        check(
            ee::s11::f_003449f0__generated,
            crowd_man3d_group_update,
            30,
            &|c, seed| {
                let _r = base(c, seed, 0x3449);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x10c, (seed % 3) as u32);
                c.bus.write32(DATA + 0x1ec, DATA + 0x1000);
                c.bus.write32(DATA + 0x1dc, f(1.0));
                c.bus.write32(DATA + 0x1e0, f(10.0));
                c.bus.write32(DATA + 0x1e4, f(0.5));
                for i in 0..3 {
                    let item = DATA + 0x1000 + 0xd0 * i;
                    c.bus.write32(item + 0x84, DATA + 0x2000 + 0x100 * i);
                    c.bus.write32(DATA + 0x2000 + 0x100 * i + 0x10, 0);
                    c.bus
                        .write32(DATA + 0x2000 + 0x100 * i + 0x14, FAKE + 0x10 * i);
                }
            },
        );
    }

    #[test]
    fn emitter_set_desc_path_coeffs_matches() {
        check(
            ee::s12::f_0036cef8__generated,
            emitter_set_desc_path_coeffs,
            100,
            &|c, seed| {
                let mut r = base(c, seed, 0x36ce);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] = sx(DATA + 0x300);
                for k in 0..4 {
                    c.bus.write32(DATA + 0x100 + 4 * k, r.next_u32());
                    c.bus.write32(DATA + 0x200 + 4 * k, r.next_u32());
                    c.bus.write32(DATA + 0x300 + 4 * k, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v13_matches() {
        check(
            ee::s13::f_003ac2a8__generated,
            cps2_chunk_resolve_v13,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x3ac2);
                let instance = DATA + 0x300;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(instance);
                c.bus.write32(DATA, DATA + 0x100);
                c.bus.write32(DATA + 0x100, DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 8, DATA + 0x400);
                let variant = seed % 4;
                let index = if variant == 0 { 0x55 } else { 0x34 };
                c.bus.write32(instance + 0x80, index);
                c.bus
                    .write32(DATA + 0x400 + (index & 0xff) * 4, DATA + 0x500);
                // The second-level value is a handle whose upper bits become a
                // small offset; keep the low RAM words plausible.
                c.bus.write32(0x20, 0x11);
                c.bus.write32(0x24, 0x22);
                fill(c, &mut r, instance + 0x8c, 0x10);
            },
        );
    }

    #[test]
    fn game_state_finish_exit_matches() {
        check(
            ee::s06::f_00234008__generated,
            game_state_finish_exit,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x2340);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(RIDERS + 0x10, (seed % 2) as u32);
                c.bus.write32(RIDERS + 4, DATA + 0x600);
                c.bus.write32(DATA + 0x600 + 0xa8, DATA + 0x700);
                c.bus.write32(gp_at(-0x84c), DATA + 0x800);
                fill(c, &mut r, DATA + 0x700, 0x40);
                fill(c, &mut r, DATA + 0x800, 0x40);
                c.bus.write32(DATA + 0x10, 0);
            },
        );
    }

    #[test]
    fn cov_state_overlay_shutdown_matches() {
        check(
            ee::s05::f_0020a430__generated,
            cov_state_overlay_shutdown,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x20a4);
                let screen = DATA + 0x1000;
                let node = DATA + 0x2000;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x50, (seed % 2) as u32);
                c.bus.write32(DATA + 0x54, (seed / 2 % 2) as u32);
                c.bus.write32(DATA + 0x10, screen - 0x18);
                hash_screen(c, screen, node, 0x004a_20d0);
                c.bus.write32(node + 8, DATA + 0x3000);
                slot(c, DATA + 0x3000, 0xc0, 1);
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    #[test]
    fn avalanche_get_object_matrix_matches() {
        check(
            ee::s09::f_002d9c00__generated,
            avalanche_get_object_matrix,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x2d9c);
                let out = DATA + 0x400;
                let node0 = 0x004e_e770u32;
                let hit = (seed % 3 != 0) as u32;
                c.g[A0][0] = if hit == 1 { (seed % 64) as u64 } else { 0x1234 };
                c.g[A1][0] = sx(out);
                let node = node0 + hit * 0x2f0;
                c.bus.write32(node + 0x2e0, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0xf8, seed as u32 % 64);
                for k in 0..4 {
                    c.bus.write32(node + 0x70 + 4 * k, r.next_u32());
                }
                c.bus.write32(node + 0xb0, f(2.0));
                for k in 0..4 {
                    c.bus.write32(node + 0x60 + 4 * k, r.next_u32());
                }
                fill(c, &mut r, out, 0x40);
            },
        );
    }

    #[test]
    fn flag_manager_group_load_matches() {
        check(
            ee::s11::f_0034c378__generated,
            cflag_manager_group_load,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x34c3);
                let stream = DATA + 0x400;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(stream);
                c.bus.write32(stream, DATA + 0x500);
                c.bus.write16(DATA + 0x500 + 0x10, 0);
                c.bus.write32(DATA + 0x500 + 0x14, FAKE);
                c.bus.write16(DATA + 0x500 + 0x18, 0);
                c.bus.write32(DATA + 0x500 + 0x1c, FAKE + 0x10);
                c.bus.write32(DATA + 0x5c, (seed % 2) as u32);
                fill(c, &mut r, DATA + 0x94, 8);
            },
        );
    }

    #[test]
    fn cfe_state_request_line_sort_song_list_matches() {
        check(
            ee::s02::f_00197ad8__generated,
            cfe_state_request_line_sort_song_list,
            120,
            &|c, seed| {
                let mut r = base(c, seed, 0x197a);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x14c, (seed % 4) as u32);
                c.bus
                    .write64(DATA + 0x160, if seed % 2 == 0 { 0x5 } else { 0xa });
                fill(c, &mut r, DATA + 0x4c, 8);
            },
        );
    }

    #[test]
    fn cfe_state_profile_load_v38_matches() {
        check(
            ee::s02::f_0018f0b8__generated,
            cfe_state_profile_load_v38,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x18f0);
                let screen = DATA + 0x1000;
                let node = DATA + 0x2000;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(gp_at(-0x1b08), (seed % 3) as u32);
                c.bus.write32(DATA + 0x40, screen);
                hash_screen(c, screen, node, 0x004a_1458);
                c.bus.write32(node + 8, DATA + 0x3000);
                slot(c, DATA + 0x3000, 0x48, 1);
                fill(c, &mut r, DATA + 0x220, 0x10);
            },
        );
    }

    #[test]
    fn benew_race_int_get_event_record_target_matches() {
        check(
            ee::s01::f_001456a0__generated,
            benew_race_int_get_event_record_target,
            150,
            &|c, seed| {
                let mut r = base(c, seed, 0x1456);
                c.g[A1][0] = (seed % 20) as u64;
                c.g[A2][0] = (seed / 20 % 13) as u64;
                for i in 0..24 {
                    c.bus
                        .write16(0x0044_0e80 + 2 * i, (r.next_u32() & 0x7fff) as u16);
                }
            },
        );
    }

    #[test]
    fn ov_screen_read_private_message_update_matches() {
        check(
            ee::s05::f_00210d20__generated,
            ov_screen_read_private_message_update,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x210d);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x40, (seed % 2) as u32);
                c.bus
                    .write32(gp_at(-0x204), if seed % 3 == 0 { 0 } else { DATA + 0x1000 });
                c.bus.write32(DATA + 0x1000 + 0x68, (seed / 2 % 2) as u32);
                c.bus.write32(DATA + 0x1000 + 0x64, (seed / 4 % 2) as u32);
                c.bus.write32(DATA + 0x10, DATA + 0x2000);
                fill(c, &mut r, DATA + 0x9c, 0x50);
                fill(c, &mut r, DATA + 0x2000, 0x40);
            },
        );
    }

    #[test]
    fn bemission_int_load_state_matches() {
        check(
            ee::s01::f_00154898__generated,
            bemission_int_load_state,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1548);
                let stream = DATA + 0x400;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(stream);
                c.bus.write32(stream, DATA + 0x500);
                c.bus.write16(DATA + 0x500 + 0x10, 0);
                c.bus.write32(DATA + 0x500 + 0x14, FAKE);
                fill(c, &mut r, DATA + 0x8, 0x10);
            },
        );
    }

    #[test]
    fn beoption_int_swap_player_flags_matches() {
        check(
            ee::s01::f_0014eb08__generated,
            beoption_int_swap_player_flags,
            100,
            &|c, seed| {
                let mut r = base(c, seed, 0x14eb);
                for k in 0..0x20 / 4 {
                    c.bus.write32(0x0053_08b8 + 4 * k, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn bescore_int_get_event_best_place_matches() {
        check(
            ee::s01::f_001558f8__generated,
            bescore_int_get_event_best_place,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x1558);
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (seed / 5 % 4) as u64;
                c.g[A3][0] = (seed / 20 % 4) as u64;
                c.bus.write8(0x0053_05f9, (seed % 3 == 0) as u8);
                fill(c, &mut r, 0x004a_6ca8, 0x1000);
            },
        );
    }

    #[test]
    fn game_anim_map_build_name_index_matches() {
        check(
            ee::s10::f_00314dd8__generated,
            game_anim_map_build_name_index,
            12,
            &|c, seed| {
                let _r = base(c, seed, 0x314d);
                c.g[A0][0] = sx(DATA);
                let empty = DATA + 0x1000;
                for i in 0..0x206u32 {
                    c.bus.write32(0x0048_cfc0 + 4 * i, empty);
                }
                c.bus.write8(empty, 0);
            },
        );
    }

    #[test]
    fn uianim_eval_channel_matches() {
        check(ee::s13::f_003971c8__generated, uianim_eval_channel, 120, &|c, seed| {
            let mut r = base(c, seed, 0x3971);
            let anim = DATA + 0x100;
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 40) as u64;
            c.bus.write32(DATA, if seed % 7 == 0 { 0 } else { anim });
            c.bus.write16(anim + 6, (seed % 5) as u16);
            c.bus.write16(anim + 0xa, (seed % 20) as u16);
            for i in 0..5u32 {
                c.bus.write16(anim + 0xc + 4 * i, (1 + i) as u16);
                c.bus
                    .write16(anim + 0xc + 4 * i + 2, (r.next_u32() & 0xff) as u16);
            }
        });
    }

    #[test]
    fn femodule_base_dtor_matches() {
        check(ee::s02::f_001957d0__generated, femodule_base_dtor, 30, &|c, seed| {
            let _r = alloc_base(c, seed, 0x1957);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 2) as u64;
            c.bus.write32(gp_at(-0x854), DATA + 0x3000);
            c.bus.write32(DATA + 0x3000 + 0x10d8, DATA + 0x4000);
            c.bus.write32(DATA + 0xc, 0);
            c.bus.write32(DATA + 0xb5aa0, 0);
        });
    }

    #[test]
    fn section_man_queue_section_matches() {
        check(
            ee::s05::f_0022e298__generated,
            section_man_queue_section,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x22e2);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                let n = (seed / 4 % 4) as u32;
                c.bus
                    .write32(DATA + 0x19c, if n == 0 { 0 } else { DATA + 0x1000 });
                for i in 0..3u32 {
                    let node = DATA + 0x1000 + 0x10 * i;
                    c.bus.write32(node, (i + 1) % 4);
                    c.bus
                        .write32(node + 4, if i + 1 < n { node + 0x10 } else { 0 });
                }
                c.bus.write32(DATA + 0x198, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, 0);
                c.bus.write32(DATA + 0x2000 + 4, 0);
                c.bus.write32(DATA + 0x1a0, 0);
                fill(c, &mut r, DATA + 0x2000, 0x20);
            },
        );
    }

    #[test]
    fn cfe_state_create_step2_fill_gender_list_matches() {
        check(
            ee::s03::f_001bb520__generated,
            cfe_state_create_step2_fill_gender_list,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1bb5);
                let box_ = DATA + 0x1000;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(box_);
                c.g[A2][0] = (seed % 2) as u64;
                c.bus.write32(box_ + 8, DATA + 0x2000);
                slot(c, DATA + 0x2000, 0x48, 1);
                fill(c, &mut r, box_, 0x40);
            },
        );
    }

    #[test]
    fn net_create_room_popup_validate_room_name_matches() {
        check(
            ee::s03::f_001bec78__generated,
            net_create_room_popup_validate_room_name,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1bec);
                let name = DATA + 0x1000;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x6dc, name);
                if seed % 2 == 0 {
                    c.bus.write8(name, 0);
                } else {
                    c.bus.write8(name, b'x');
                    c.bus.write8(name + 1, 0);
                }
                c.bus.write32(DATA + 0x100, DATA + 0x3000);
                fill(c, &mut r, DATA + 0x3000, 0x200);
            },
        );
    }

    #[test]
    fn message_center_remove_message_matches() {
        check(
            ee::s04::f_001e31b8__generated,
            message_center_remove_message,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x1e31);
                let inbox = DATA + 0x1000;
                c.g[A0][0] = (seed % 5) as u64;
                c.bus.write32(DATA + 0x2000, inbox);
                fill(c, &mut r, inbox, 0x100);
                c.bus.write8(inbox + 0xcc, (seed % 4) as u8);
                c.bus.write32(inbox + 0xc8, r.next_u32());
            },
        );
    }

    #[test]
    fn chat_event_clone_matches() {
        check(ee::s07::f_002680a8__generated, chat_event_clone, 40, &|c, seed| {
            let _r = alloc_base(c, seed, 0x2680);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA + 8, 0);
            c.bus.write32(DATA + 0xc, 0);
        });
    }

    #[test]
    fn ssx_audio_system_dj_say212e_matches() {
        check(
            ee::s08::f_002a2cf0__generated,
            ssx_audio_system_dj_say212e,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x2a2c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                fill(c, &mut r, DATA, 0x100);
                c.bus.write32(gp_at(0x173c), DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, FAKE);
            },
        );
    }

    #[test]
    fn ssx_audio_system_dj_say212f_matches() {
        check(
            ee::s08::f_002a2da0__generated,
            ssx_audio_system_dj_say212f,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x2a2d);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                fill(c, &mut r, DATA, 0x100);
                c.bus.write32(gp_at(0x173c), DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, FAKE);
            },
        );
    }

    #[test]
    fn ini_parser_read_quoted_string_matches() {
        check(
            ee::s08::f_002a7cf0__generated,
            ini_parser_read_quoted_string,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x2a7c);
                let text = DATA + 0x1000;
                c.g[A0][0] = sx(DATA);
                let s: &[u8] = match seed % 3 {
                    0 => b"\"ab\"",
                    1 => b"\"\"",
                    _ => b"\"hello world\"",
                };
                for (i, b) in s.iter().enumerate() {
                    c.bus.write8(text + i as u32, *b);
                }
                c.bus.write8(text + s.len() as u32, 0);
                c.bus.write32(DATA + 8, text);
                fill(c, &mut r, DATA + 0xc, 0x40);
            },
        );
    }

    #[test]
    fn ai_reset_computer_render_slots_matches() {
        check(
            ee::s00::f_0012ab20__generated,
            ai_reset_computer_render_slots,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x12ab);
                c.g[A0][0] = sx(DATA);
                let n = (seed % 3) as u32;
                c.bus.write32(DATA + 0x80, n);
                c.bus.write32(gp_at(-0x854), DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 0x10d8, DATA + 0x4000);
                slot(c, DATA + 0x4000, 0x390, 1);
                for i in 0..3u32 {
                    let rider = DATA + 0x2000 + 0x100 * i;
                    c.bus.write32(DATA + 0x48 + 4 * i, rider);
                    c.bus.write32(rider + 0x18, DATA + 0x1000 + 0x100 * i);
                    c.bus
                        .write32(DATA + 0x1000 + 0x100 * i + 0x884, (i + 1) as u32);
                }
                fill(c, &mut r, DATA + 0x1000, 0x300);
            },
        );
    }

    #[test]
    fn ai_reset_actor_render_slots_matches() {
        check(
            ee::s00::f_0012ac48__generated,
            ai_reset_actor_render_slots,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x12ac);
                c.g[A0][0] = sx(DATA);
                let n = (seed % 3) as u32;
                c.bus.write32(DATA + 0x84, n);
                c.bus.write32(gp_at(-0x854), DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 0x10d8, DATA + 0x4000);
                slot(c, DATA + 0x4000, 0x390, 1);
                for i in 0..3u32 {
                    let rider = DATA + 0x2000 + 0x100 * i;
                    c.bus.write32(DATA + 0x5c + 4 * i, rider);
                    c.bus.write32(rider + 0x18, DATA + 0x1000 + 0x100 * i);
                    c.bus
                        .write32(DATA + 0x1000 + 0x100 * i + 0x884, (i + 1) as u32);
                }
                fill(c, &mut r, DATA + 0x1000, 0x300);
            },
        );
    }

    #[test]
    fn cps2_memcard_get_slot_name_matches() {
        check(
            ee::s08::f_002c63f8__generated,
            cps2_memcard_get_slot_name,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x2c63);
                let dst = DATA + 0x2000;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(dst);
                slot(c, VTS, 0x20, 0);
                fill(c, &mut r, dst, 0x100);
                put_words(c, 0x004a_3970, &[0]);
            },
        );
    }

    #[test]
    fn keyboard_popup_set_static_wide_text_matches() {
        check(
            ee::s04::f_001ccf70__generated,
            keyboard_popup_set_static_wide_text,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x1ccf);
                let str_ = DATA + 0x1000;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(str_);
                wide(c, str_, b"abc");
                c.bus.write16(str_ + 8, 0);
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x40, 0);
                    c.bus
                        .write32(DATA + 0x428, if seed % 4 == 0 { 0 } else { DATA + 0x3000 });
                } else {
                    let screen = DATA + 0x2000;
                    let node = DATA + 0x2100;
                    c.bus.write32(DATA + 0x40, screen);
                    hash_screen(c, screen, node, 0x0046_6580);
                    c.bus.write32(node + 8, DATA + 0x2200);
                }
            },
        );
    }

    #[test]
    fn superpipe_chase_camera_reset_for_rider_matches() {
        check(
            ee::s02::f_00178938__generated,
            superpipe_chase_camera_algorithm_reset_for_rider,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1789);
                let rider = DATA + 0x1000;
                let id = 0x1234u32;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 { id as u64 } else { 0x77 };
                c.bus.write32(DATA + 0x30, DATA + 0x900);
                c.bus.write32(DATA + 0x900 + 4, rider);
                c.bus.write32(rider + 0x6c0, DATA + 0x2000);
                slot(c, DATA + 0x2000, 0x38, 1);
                c.bus.write32(crate::difftest::fakes::LOG_RESULTS, id);
                c.bus.write32(DATA + 0x10, DATA + 0x3100);
                fill(c, &mut r, DATA + 0x20, 0x80);
                fill(c, &mut r, DATA + 0x3000, 0x200);
            },
        );
    }

    #[test]
    fn path_read_event_matches() {
        check(ee::s07::f_0026a378__generated, path_read_event, 80, &|c, seed| {
            let mut r = base(c, seed, 0x26a3);
            let src = DATA + 0x1000;
            let dst = DATA + 0x2000;
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(src);
            c.g[A2][0] = sx(dst);
            c.bus.write32(src + 4, (seed % 25) as u32);
            for i in 0..21u32 {
                c.bus.write32(0x0044_53d0 + 4 * i, i * 3);
            }
            fill(c, &mut r, dst, 0x20);
            fill(c, &mut r, src + 8, 0x10);
        });
    }
}
