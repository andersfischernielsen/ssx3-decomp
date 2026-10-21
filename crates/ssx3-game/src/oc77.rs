//! Phase 2 wave 18 module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for 45 small script-engine, crowd, camera, UI, front-end,
//! memory-card and dirtysock-tag functions, each with a differential test
//! against the generated body.
//!
//! The bulk of them are small wrappers around a vtable slot or around one or
//! two engine calls:
//!
//! - **script engine** (`WScriptProcess_v06/v02`, `WScriptMan_StartProcess`,
//!   `WScriptMan_StopProcess`, `tWScriptCache_Free`): swap a process into a
//!   manager field, call its slot, swap it back; free a cached script buffer.
//! - **crowd/modifier nodes** (`CrowdMan3d_Reset`, `ModObj_v14`,
//!   `ModObj_v39`, `AnimDelta_ctor`): walk the five 0x1f4-byte crowd groups,
//!   or forward a virtual method to the modifier block.
//! - **camera** (`ManualCamController_ResetForRider`,
//!   `ChaseCamController_dtor`, `CameraTriggerMan_Reset`,
//!   `DebugMenuModule_Enter`): reset rider cameras, camera trigger lists and
//!   the chase camera, or run the debug-menu entry sequence.
//! - **UI** (`UIFontSet_FindFont`, `UIScreen_CmdBeginTransitionOut`,
//!   `UIState_OnActivate`, `UIPair_dtor`, `UIListBox_dtor`, `UIText_dtor`):
//!   linear font lookup, focus/notify/child iteration and the three derived
//!   UI destructors (vtable, body, then the UIObject base destructor).
//! - **front-end state machine** (`cFEStateOnlineCharacterSelect_ctor`,
//!   `cFEStateNetCreateRoomPopup_ctor`, the `cFEStateRewardGallery*` button
//!   handlers, `cFEPopupConfirm*`, `MemCardUI_ShowFileDisplay`): constructors
//!   that install a vtable and a class id, and small event/refresh wrappers.
//! - **dirtysock tags** (`TagFieldStrnzcpy`, `TagFieldGetFlags`): the
//!   bounded string copy and the flag lookup over the type table.
//!
//! Every function keeps the generated code's cycle charging (block by block,
//! including the cost of the instructions around calls), its stack frame,
//! every register the original leaves behind (including `v0`/`v1` scratch
//! values), and its call protocol ([`crate::call`]: the same `ra` per call,
//! `call_ptr` for the `jalr`/cross-shard calls, and the unwind check after
//! each call). Loops whose back edge checks events follow
//! [`crate::events`]: `deliver_event_here` in structured loops, and
//! `back_edge_unwinds` (with resume-at-`pc` support) in the two
//! dispatcher-shaped loops.

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002e_e6f0,
        world_painter_get_weather30,
        "WorldPainter_GetWeather30",
    ),
    (0x0030_7d30, w_script_process_v06, "WScriptProcess_v06"),
    (0x0030_7dc8, w_script_process_v02, "WScriptProcess_v02"),
    (
        0x0030_9750,
        w_script_man_start_process,
        "WScriptMan_StartProcess",
    ),
    (
        0x0030_9798,
        w_script_man_stop_process,
        "WScriptMan_StopProcess",
    ),
    (0x0034_3bc0, crowd_man3d_reset, "CrowdMan3d_Reset"),
    (0x0035_5420, mod_obj_v14, "ModObj_v14"),
    (0x0035_6a70, mod_obj_v39, "ModObj_v39"),
    (0x0035_9228, anim_delta_ctor, "AnimDelta_ctor"),
    (0x0036_17b8, floating_v27, "Floating_v27"),
    (
        0x0037_c7c0,
        ps2_graphics_man_reset_texture_slots,
        "PS2GraphicsMan_ResetTextureSlots",
    ),
    (
        0x0038_9118,
        ps2_graphics_man_remove_interrupt_handlers,
        "PS2GraphicsMan_RemoveInterruptHandlers",
    ),
    (0x0038_aeb0, ps2_bezier_man_dtor, "PS2BezierMan_dtor"),
    (0x0039_83f0, ui_font_set_find_font, "UIFontSet_FindFont"),
    (
        0x0039_ce48,
        ui_screen_cmd_begin_transition_out,
        "UIScreen_CmdBeginTransitionOut",
    ),
    (0x0039_e4c0, ui_state_on_activate, "UIState_OnActivate"),
    (0x003a_54b8, ui_pair_dtor, "UIPair_dtor"),
    (0x003a_5658, ui_list_box_dtor, "UIListBox_dtor"),
    (0x003a_5da8, ui_text_dtor, "UIText_dtor"),
    (
        0x003a_a3f0,
        c_ps2_chunk_resolve_base_v05,
        "cPS2ChunkResolveBase_v05",
    ),
    (0x003a_cc90, t_w_script_cache_free, "tWScriptCache_Free"),
    (0x003e_bb60, tag_field_strnzcpy, "TagFieldStrnzcpy"),
    (0x003e_cfb0, tag_field_get_flags, "TagFieldGetFlags"),
    (
        0x0015_8960,
        be_reward_int_get_event_record_value,
        "BERewardInt_GetEventRecordValue",
    ),
    (
        0x0015_ce98,
        manual_cam_controller_reset_for_rider,
        "ManualCamController_ResetForRider",
    ),
    (
        0x0016_1a10,
        chase_cam_controller_dtor,
        "ChaseCamController_dtor",
    ),
    (0x0016_c918, camera_trigger_man_reset, "CameraTriggerMan_Reset"),
    (0x0017_fa60, debug_menu_module_enter, "DebugMenuModule_Enter"),
    (
        0x0018_6f40,
        c_fe_state_mem_card_is_scan_done_with_card,
        "cFEStateMemCard_IsScanDoneWithCard",
    ),
    (
        0x0019_26f0,
        c_fe_state_bragging_rights_init_player_rows,
        "cFEStateBraggingRights_InitPlayerRows",
    ),
    (
        0x0019_a498,
        c_fe_state_char_equip_release_preview,
        "cFEStateCharEquip_ReleasePreview",
    ),
    (
        0x0019_cbe0,
        bolt_file_index_manager_get_model_slot,
        "BoltFileIndexManager_GetModelSlot",
    ),
    (
        0x0019_cce8,
        bolt_file_index_manager_get_texture_slot,
        "BoltFileIndexManager_GetTextureSlot",
    ),
    (
        0x001b_0f60,
        c_fe_state_online_character_select_ctor,
        "cFEStateOnlineCharacterSelect_ctor",
    ),
    (
        0x001b_e7d8,
        c_fe_state_net_create_room_popup_ctor,
        "cFEStateNetCreateRoomPopup_ctor",
    ),
    (
        0x001c_8628,
        c_fe_popup_confirm_get_widget_size,
        "cFEPopupConfirm_GetWidgetSize",
    ),
    (0x001c_a528, c_fe_popup_confirm_update, "cFEPopupConfirm_Update"),
    (
        0x001d_11a8,
        c_fe_state_reward_gallery_posters_v26,
        "cFEStateRewardGalleryPosters_v26",
    ),
    (
        0x001d_1340,
        c_fe_state_reward_gallery_trading_cards_v26,
        "cFEStateRewardGalleryTradingCards_v26",
    ),
    (
        0x001d_14d8,
        c_fe_state_reward_gallery_concept_arts_v26,
        "cFEStateRewardGalleryConceptArts_v26",
    ),
    (
        0x001d_1670,
        c_fe_state_reward_gallery_videos_v26,
        "cFEStateRewardGalleryVideos_v26",
    ),
    (
        0x001d_1808,
        c_fe_state_reward_gallery_toys_v26,
        "cFEStateRewardGalleryToys_v26",
    ),
    (
        0x001d_8358,
        mem_card_ui_show_file_display,
        "MemCardUI_ShowFileDisplay",
    ),
    (0x001d_86b0, mem_card_state_base_v45, "MemCardStateBase_v45"),
];

// ---------------------------------------------------------------------------
// Callees (the generated aliases, so a replaced callee is reached too).

mod callee {
    use super::{ee, Callee};

    // Script engine (shard 10).
    pub const WSCRIPT_PROCESS_TERMINATE: Callee = Callee {
        addr: 0x0030_7d78,
        f: ee::s10::f_00307d78,
    };
    pub const WSCRIPT_MAN_CALL_PROCESS_FUNC: Callee = Callee {
        addr: 0x0030_ac98,
        f: ee::s10::f_0030ac98,
    };
    pub const WSCRIPT_MAN_IS_PROCESS_ACTIVE: Callee = Callee {
        addr: 0x0030_b898,
        f: ee::s10::f_0030b898,
    };

    // Crowd / modifier nodes (shard 11).
    pub const CROWD_MAN3D_GROUP_FREE: Callee = Callee {
        addr: 0x0034_4348,
        f: ee::s11::f_00344348,
    };
    pub const CROWD_MAN3D_GROUP_RESET: Callee = Callee {
        addr: 0x0034_42e0,
        f: ee::s11::f_003442e0,
    };
    pub const ANIM_OBJECT_CTOR: Callee = Callee {
        addr: 0x0034_1aa0,
        f: ee::s11::f_00341aa0,
    };

    // Graphics (shard 12).
    pub const TEX_MAN_SET_ALLOC_FLAGS: Callee = Callee {
        addr: 0x0036_6fe0,
        f: ee::s12::f_00366fe0,
    };
    pub const PS2_BEZIER_MAN_FREE_CACHES: Callee = Callee {
        addr: 0x0038_b0f8,
        f: ee::s12::f_0038b0f8,
    };
    pub const BEZIER_MAN_BASE_DTOR: Callee = Callee {
        addr: 0x0037_4ca8,
        f: ee::s12::f_00374ca8,
    };

    // UI (shard 13).
    pub const C_UI_GROUP_SET_FOCUS: Callee = Callee {
        addr: 0x0039_8638,
        f: ee::s13::f_00398638,
    };
    pub const UI_LIST_FIRST: Callee = Callee {
        addr: 0x0039_7840,
        f: ee::s13::f_00397840,
    };
    pub const UI_PAIR_FREE_CHILDREN: Callee = Callee {
        addr: 0x0039_be70,
        f: ee::s13::f_0039be70,
    };
    pub const UI_OBJECT_DTOR: Callee = Callee {
        addr: 0x0039_fc48,
        f: ee::s13::f_0039fc48,
    };
    pub const UI_LIST_BOX_CLEAR: Callee = Callee {
        addr: 0x0039_a670,
        f: ee::s13::f_0039a670,
    };
    pub const UI_TEXT_FREE_STRINGS: Callee = Callee {
        addr: 0x003a_0b38,
        f: ee::s13::f_003a0b38,
    };

    // Camera / reward front end (shard 1..4).
    pub const BE_GET_CHAR_PROFILE_EVENT_RECORD: Callee = Callee {
        addr: 0x0015_2398,
        f: ee::s01::f_00152398,
    };
    pub const CAM_CONTROLLER_RESET_FOR_RIDER: Callee = Callee {
        addr: 0x0015_cc70,
        f: ee::s01::f_0015cc70,
    };
    pub const CAM_CONTROLLER_DTOR: Callee = Callee {
        addr: 0x0015_cc10,
        f: ee::s01::f_0015cc10,
    };
    pub const CAMERA_TRIGGER_MAN_CLEAR_DATA: Callee = Callee {
        addr: 0x0016_bf20,
        f: ee::s02::f_0016bf20,
    };
    pub const ACTIVE_CAMERA_TRIGGER_LIST_CLEAR: Callee = Callee {
        addr: 0x0016_c628,
        f: ee::s02::f_0016c628,
    };
    pub const CAMERA_TRIGGER_STACK_CLEAR: Callee = Callee {
        addr: 0x0016_d200,
        f: ee::s02::f_0016d200,
    };
    pub const BOLT_FILE_RELEASE: Callee = Callee {
        addr: 0x0019_dc20,
        f: ee::s03::f_0019dc20,
    };
    pub const BOLT_FILE_INDEX_MANAGER_FIND_NODE: Callee = Callee {
        addr: 0x0019_c938,
        f: ee::s03::f_0019c938,
    };
    pub const NET_FE_STATE_CTOR: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::f_001a8500,
    };
    pub const C_FE_STATE_REWARD_GALLERY_FIND_TEXTURE_BY_NAME: Callee = Callee {
        addr: 0x001c_fdd0,
        f: ee::s04::f_001cfdd0,
    };
    pub const MEM_CARD_STATE_BASE_SET_USE_SAVE_TITLE: Callee = Callee {
        addr: 0x001d_9308,
        f: ee::s04::f_001d9308,
    };
    pub const C_FE_STATE_VIDEO_PLAYER_START_PLAYBACK: Callee = Callee {
        addr: 0x001d_25e8,
        f: ee::s04::f_001d25e8,
    };
}

use callee::*;

// ---------------------------------------------------------------------------
// WorldPainter / lens effects.

/// `float WorldPainter_GetWeather30(int i)` (0x2ee6f0): the lens-effect slot
/// `i` (`D_004FA370 + i * 0xf0`) asks its object's virtual weather accessor
/// (vtable slot 0x180/0x184) for the current weather blend.
pub fn world_painter_get_weather30(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = 0xf0; // sizeof(sLensFxSlot)
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1); // a0 = i * 0xf0
        c.g[V0][0] = 0x0050_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x0050_0000u32.wrapping_sub(0x5c90)); // &D_004FA370
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let slot = c.g[A0][0] as u32;
        c.g[V1][0] = Addr(slot).off(0x20).lw(c); // s->p20
        let obj = Addr(c.g[V1][0] as u32).lw(c); // *s->p20
        c.g[A2][0] = obj;
        let vt = Addr((obj as u32).wrapping_add(4)).lw(c); // obj->vt
        c.g[A1][0] = vt;
        c.g[A0][0] = Addr((vt as u32).wrapping_add(0x180)).lh(c); // delta
        c.g[V0][0] = Addr((vt as u32).wrapping_add(0x184)).lw(c); // fn
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x002e_e728);
        c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002e_e728)?;
        c.lwc1(0, c.g[V0][0] as u32); // f0 = weather blend
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script processes.

/// `void WScriptProcess_FinishProcess(self)` (0x307d30): mark the process
/// finished (`*self = 3`), hand its script to the script manager if it has
/// one (`+0x30`), then terminate it.
pub fn w_script_process_v06(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 3;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = c.g[S0][0] as u32;
        Addr(this).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(this).off(0x30).lw(c);
        if c.g[A2][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(this).off(0x10).lw(c);
            c.g[A1][0] = c.g[S0][0];
            WSCRIPT_MAN_CALL_PROCESS_FUNC.call(c, 0x0030_7d60)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        WSCRIPT_PROCESS_TERMINATE.call(c, 0x0030_7d68)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptProcess_Deactivate(self)` (0x307dc8): if the process is still
/// in the manager's active list, call its `deactivate` slot
/// (vtable 0x30/0x34).
pub fn w_script_process_v02(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = Addr(this).off(0x10).lw(c);
        WSCRIPT_MAN_IS_PROCESS_ACTIVE.call(c, 0x0030_7de4)?;
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[V0][0] = Addr(this).off(0x5c).lw(c);
            let vt = c.g[V0][0] as u32;
            c.g[A0][0] = Addr(vt).off(0x30).lh(c);
            c.g[V1][0] = Addr(vt).off(0x34).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0030_7e00);
            c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0030_7e00)?;
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_StartProcess(man, process)` (0x309750): make `process` the
/// manager's current process (`+0x2a4`), call its `start` slot (0x20/0x24),
/// and restore the previous current process.
pub fn w_script_man_start_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        let man = c.g[S0][0] as u32;
        c.g[S1][0] = Addr(man).off(0x2a4).lw(c);
        Addr(man).off(0x2a4).sw(c, c.g[A1][0]);
        let process = c.g[A1][0] as u32;
        c.g[V0][0] = Addr(process).off(0x5c).lw(c);
        let vt = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vt).off(0x20).lh(c);
        c.g[V1][0] = Addr(vt).off(0x24).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0030_9780);
        c.g[A0][0] = sx(process.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0030_9780)?;
        Addr(man).off(0x2a4).sw(c, c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_StopProcess(man, process)` (0x309798): as
/// [`w_script_man_start_process`] but calls the process's `stop` slot
/// (vtable 0x30/0x34).
pub fn w_script_man_stop_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        let man = c.g[S0][0] as u32;
        c.g[S1][0] = Addr(man).off(0x2a4).lw(c);
        Addr(man).off(0x2a4).sw(c, c.g[A1][0]);
        let process = c.g[A1][0] as u32;
        c.g[V0][0] = Addr(process).off(0x5c).lw(c);
        let vt = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vt).off(0x30).lh(c);
        c.g[V1][0] = Addr(vt).off(0x34).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0030_97c8);
        c.g[A0][0] = sx(process.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0030_97c8)?;
        Addr(man).off(0x2a4).sw(c, c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Crowd / modifier objects.

/// `void CrowdMan3d_Reset(crowd)` (0x343bc0): free and reinitialise each of
/// the five 0x1f4-byte crowd groups (the loop from `crowd` to `crowd+0x9c4`).
pub fn crowd_man3d_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c4));
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S0][0];
            CROWD_MAN3D_GROUP_FREE.call(c, 0x0034_3be0)?;
            c.g[A0][0] = c.g[S0][0];
            CROWD_MAN3D_GROUP_RESET.call(c, 0x0034_3be8)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let again = c.g[V0][0] != 0;
            c.g[RA][0] = c.ld(frame.at(c, 0));
            if again {
                if event_due(c) {
                    deliver_event_here(c, 0x0034_3bd8);
                }
                continue;
            }
            c.cycles += 4;
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `int ModObj_v14(self)` (0x355420): the move-node block's
/// "is-section-removed" query (vtable slot 0x40/0x44 of `*(self+0x1c)`);
/// 1 when there is no block or no node.
pub fn mod_obj_v14(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        c.g[V1][0] = Addr(this).off(0x1c).lw(c);
        let no_block = c.g[V1][0] == 0;
        c.g[V0][0] = 1;
        let mut extra = 0;
        if no_block {
            extra = 1;
        } else {
            c.cycles += 3;
            c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
            if c.g[A1][0] != 0 {
                c.cycles += 7;
                let node = c.g[A1][0] as u32;
                c.g[V1][0] = Addr(node).lw(c);
                let vt = c.g[V1][0] as u32;
                c.g[A0][0] = Addr(vt).off(0x40).lh(c);
                c.g[V0][0] = Addr(vt).off(0x44).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0035_5454);
                c.g[A0][0] = sx(node.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0035_5454)?;
            }
        }
        c.cycles += extra + 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ModObj_v39(self)` (0x356a70): the move-node block's modifier-block
/// query (vtable slot 0xa8/0xac); 0 when there is no block or no node.
pub fn mod_obj_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        c.g[V1][0] = Addr(this).off(0x1c).lw(c);
        let no_block = c.g[V1][0] == 0;
        c.g[V0][0] = 0;
        let mut extra = 0;
        if no_block {
            extra = 1;
        } else {
            c.cycles += 3;
            c.g[T1][0] = Addr(c.g[V1][0] as u32).lw(c);
            if c.g[T1][0] != 0 {
                c.cycles += 7;
                let node = c.g[T1][0] as u32;
                c.g[V1][0] = Addr(node).lw(c);
                let vt = c.g[V1][0] as u32;
                c.g[A0][0] = Addr(vt).off(0xa8).lh(c);
                c.g[V0][0] = Addr(vt).off(0xac).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0035_6aa4);
                c.g[A0][0] = sx(node.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0035_6aa4)?;
            }
        }
        c.cycles += extra + 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AnimDelta* AnimDelta_ctor(this, a, b, c)` (0x359228): construct the
/// anim-object base (`AnimObject_ctor(this, a, 2, b, c)`), then install the
/// delta vtable and clear the delta state.
pub fn anim_delta_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[T0][0] = c.g[A3][0];
        frame.save(c, S0, 0x10);
        c.g[A3][0] = c.g[A2][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x2;
        ANIM_OBJECT_CTOR.call(c, 0x0035_9248)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_sub(0x1590)); // &D_0048EA70
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x6c).sw(c, 0);
        Addr(this).off(0x3c).sw(c, c.g[V1][0]);
        Addr(this).off(4).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int Floating_v27(self)` (0x3617b8): call the rail modifier's flush slot
/// (0x198/0x19c) when bit 0 of `+0x12` is set, then return `+0x74`.
pub fn floating_v27(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(this).off(0x12).lhu(c);
        c.g[V0][0] &= 1;
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[V0][0] = Addr(this).off(0xc).lw(c);
            let vt = c.g[V0][0] as u32;
            c.g[A0][0] = Addr(vt).off(0x198).lh(c);
            c.g[V1][0] = Addr(vt).off(0x19c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0036_17ec);
            c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0036_17ec)?;
        }
        c.g[V0][0] = Addr(this).off(0x74).lw(c);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Graphics managers.

/// `void PS2GraphicsMan_ResetTextureSlots(this, flags)` (0x37c7c0): clear the
/// 0x1f40-byte particle slot array at `+0x3838` and pass `flags` to the
/// texture allocator.
pub fn ps2_graphics_man_reset_texture_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x1f40;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3838));
        c.g[A1][0] = 0;
        call_ptr(c, 0x003e_6448, 0x0037_c7e8)?;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x18f4)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        TEX_MAN_SET_ALLOC_FLAGS.call(c, 0x0037_c7f4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2GraphicsMan_RemoveInterruptHandlers(this)` (0x389118): detach the
/// two interrupt/DMA handlers registered for the graphics manager.
pub fn ps2_graphics_man_remove_interrupt_handlers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x2;
        call_ptr(c, 0x0042_4880, 0x0038_9130)?;
        c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x5ac0)).lw(c);
        c.g[A0][0] = 0x2;
        call_ptr(c, 0x0042_3aa0, 0x0038_913c)?;
        c.g[A0][0] = 0x1;
        call_ptr(c, 0x0042_4950, 0x0038_9144)?;
        c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x5ac4)).lw(c);
        c.g[A0][0] = 0;
        call_ptr(c, 0x0042_3ad0, 0x0038_9150)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* PS2BezierMan_dtor(this, flags)` (0x38aeb0): install the bezier
/// manager vtable, free its caches and run the bezier-base destructor.
pub fn ps2_bezier_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_add(0x3760));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(c.g[S0][0] as u32).off(4).sw(c, c.g[V0][0]);
        PS2_BEZIER_MAN_FREE_CACHES.call(c, 0x0038_aed8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        BEZIER_MAN_BASE_DTOR.call(c, 0x0038_aee4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI objects.

/// `int UIFontSet_FindFont(bank, key)` (0x3983f0): linear search of the
/// `u8`-counted array of 0xc-byte font entries at `bank+4`; returns the
/// entry's `+8` value, or 0.
pub fn ui_font_set_find_font(c: &mut Cpu) {
    c.cycles += 3;
    let bank = c.g[A0][0] as u32;
    c.g[V1][0] = Addr(bank).off(0x34).lb(c);
    c.g[A2][0] = c.g[V1][0];
    if (c.g[V1][0] as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        crate::abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[A0][0] = sx(bank.wrapping_add(4));
    c.g[V1][0] = 0x0100_0000;
    c.g[A3][0] = 0x0100_0000;
    loop {
        c.cycles += 3;
        let entry = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(entry).lw(c);
        let found = c.g[V0][0] == c.g[A1][0];
        c.g[V0][0] = c.g[V1][0];
        if !found {
            c.cycles += 5;
            c.g[A0][0] = sx(entry.wrapping_add(0xc));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            if c.g[V0][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0039_8408);
                }
                continue;
            }
            break;
        }
        c.cycles += 2;
        c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
        crate::abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    crate::abi::ret(c);
}

/// `void UIScreen_CmdBeginTransitionOut(screen)` (0x39ce48): clear the
/// screen's focus and set the "transition out" mode (6) on its owner
/// (`+0xd0`), if any.
pub fn ui_screen_cmd_begin_transition_out(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
        C_UI_GROUP_SET_FOCUS.call(c, 0x0039_ce64)?;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xd0)).lw(c);
        let owner = c.g[A0][0] as u32;
        c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
        if owner != 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(owner).off(0x1c).lw(c);
            c.g[V0][0] &= c.g[V1][0];
            c.g[V0][0] |= 0x600;
            Addr(owner).off(0x1c).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIState_OnActivate(state)` (0x39e4c0): notify the state's vtable slot
/// 0x68/0x6c of the first child widget (`list at +0x24`, child `+0x40`).
pub fn ui_state_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
        UI_LIST_FIRST.call(c, 0x0039_e4d8)?;
        c.g[A1][0] = c.g[V0][0];
        let found = c.g[A1][0] != 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
        if found {
            c.cycles += 5;
            let this = c.g[S0][0] as u32;
            c.g[V0][0] = Addr(this).off(8).lw(c);
            let vt = c.g[V0][0] as u32;
            c.g[A0][0] = Addr(vt).off(0x68).lh(c);
            c.g[V1][0] = Addr(vt).off(0x6c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039_e4f8);
            c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0039_e4f8)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the three derived UI destructors: install `vtable` at
/// `+8`, run the class's own teardown (`clear`), then `UIObject_dtor`.
fn ui_derived_dtor(c: &mut Cpu, vtable: u32, clear: Callee, ret1: u32, ret2: u32) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(vtable);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(c.g[S0][0] as u32).off(8).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        clear.call(c, ret1)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        UI_OBJECT_DTOR.call(c, ret2)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIPair_dtor(this, flags)` (0x3a54b8).
pub fn ui_pair_dtor(c: &mut Cpu) {
    ui_derived_dtor(
        c,
        0x0049_4868,
        UI_PAIR_FREE_CHILDREN,
        0x003a_54e0,
        0x003a_54ec,
    )
}

/// `void UIListBox_dtor(this, flags)` (0x3a5658).
pub fn ui_list_box_dtor(c: &mut Cpu) {
    ui_derived_dtor(
        c,
        0x0049_4798,
        UI_LIST_BOX_CLEAR,
        0x003a_5680,
        0x003a_568c,
    )
}

/// `void UIText_dtor(this, flags)` (0x3a5da8).
pub fn ui_text_dtor(c: &mut Cpu) {
    ui_derived_dtor(
        c,
        0x0049_4588,
        UI_TEXT_FREE_STRINGS,
        0x003a_5dd0,
        0x003a_5ddc,
    )
}

/// `int cPS2ChunkResolveBase_v05(self, chunk)` (0x3aa3f0): relocate the
/// chunk's two internal pointers to absolute addresses, hand the chunk to the
/// resolver's slot 0x30/0x34 and report success.
pub fn c_ps2_chunk_resolve_base_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let chunk = c.g[V0][0] as u32;
        c.g[A2][0] = Addr(chunk).off(8).lw(c);
        c.g[V1][0] = Addr(chunk).off(0xc).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(chunk));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(chunk));
        Addr(chunk).off(8).sw(c, c.g[A2][0]);
        Addr(chunk).off(0xc).sw(c, c.g[V1][0]);
        c.g[A2][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        let resolver = c.g[A2][0] as u32;
        c.g[V1][0] = Addr(resolver).off(0x30).lh(c);
        c.g[V0][0] = Addr(resolver).off(0x34).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x003a_a428);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x003a_a428)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

/// `void tWScriptCache_Free(entry)` (0x3acc90): free the cached script buffer
/// at `+8` unless it is a borrowed (static) buffer (`*(short*)(entry+2)`),
/// then clear the entry's flag word.
pub fn t_w_script_cache_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let entry = c.g[S0][0] as u32;
        c.g[A0][0] = Addr(entry).off(8).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(entry).off(2).lh(c);
            if c.g[V0][0] != 0 {
                Addr(entry).off(8).sw(c, 0);
            } else {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x003a_ccc0)?;
                Addr(entry).off(8).sw(c, 0);
            }
            c.cycles += 1;
        }
        Addr(entry).sh(c, 0);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Dirtysock tag fields.

/// `int TagFieldStrnzcpy(char* dst, int len, const char* src)` (0x3ebb60):
/// copy at most `len-1` characters, NUL-terminate, return the number copied.
///
/// The generated body is a `pc`-dispatch loop (it re-enters at the store
/// stage 0x3ebb70 after a thread switch), so this port accepts the same
/// resume addresses.
pub fn tag_field_strnzcpy(c: &mut Cpu) {
    loop {
        match c.pc {
            0x003e_bb60 => {
                c.cycles += 3;
                c.g[A3][0] = c.g[A1][0]; // left = len
                c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
                c.pc = 0x003e_bb84;
            }
            0x003e_bb6c => {
                c.cycles += 1;
                c.pc = 0x003e_bb70;
            }
            0x003e_bb84 => {
                c.cycles += 2;
                c.pc = if c.g[V0][0] != 0 {
                    0x003e_bb98
                } else {
                    0x003e_bb8c
                };
            }
            0x003e_bb8c => {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lb(c);
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lbu(c);
                if !more {
                    c.pc = 0x003e_bb98;
                    continue;
                }
                if back_edge_unwinds(c, 0x003e_bb70) {
                    return;
                }
                c.pc = 0x003e_bb70;
            }
            0x003e_bb70 => {
                c.cycles += 5;
                Addr(c.g[A0][0] as u32).sb(c, c.g[V1][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < 2) as u64;
                c.pc = 0x003e_bb84;
            }
            0x003e_bb98 => {
                c.cycles += 2;
                if (c.g[A3][0] as i64) > 0 {
                    Addr(c.g[A0][0] as u32).sb(c, 0);
                }
                c.pc = 0x003e_bba0;
            }
            0x003e_bba0 => {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A3][0] as u32));
                crate::abi::ret(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x003e_bb60, 0x003e_bba8);
                return;
            }
        }
    }
}

/// `int TagFieldGetFlags(const char* type, int defval)` (0x3ecfb0): OR together
/// the flag words `D_004965E0[c]` for every character of the tag type; the
/// default is returned for a null type.
pub fn tag_field_get_flags(c: &mut Cpu) {
    loop {
        match c.pc {
            0x003e_cfb0 => {
                c.cycles += 2;
                c.g[V0][0] = c.g[A1][0];
                c.pc = if c.g[A0][0] == 0 {
                    0x003e_cff0
                } else {
                    0x003e_cfb8
                };
            }
            0x003e_cfb8 => {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lb(c);
                c.g[V1][0] = 0x0049_0000;
                c.g[V1][0] = sx(0x0049_0000u32.wrapping_add(0x65e0)); // D_004965E0
                c.g[A1][0] = 0;
                c.pc = 0x003e_cfd8;
            }
            0x003e_cfcc => {
                c.cycles += 1;
                c.pc = 0x003e_cfd0;
            }
            0x003e_cfd0 => {
                c.cycles += 2;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lb(c);
                c.pc = 0x003e_cfd8;
            }
            0x003e_cfd8 => {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[A1][0] |= c.g[V0][0];
                    if back_edge_unwinds(c, 0x003e_cfd0) {
                        return;
                    }
                    c.pc = 0x003e_cfd0;
                } else {
                    c.pc = 0x003e_cfec;
                }
            }
            0x003e_cfec => {
                c.cycles += 1;
                c.g[V0][0] = c.g[A1][0];
                c.pc = 0x003e_cff0;
            }
            0x003e_cff0 => {
                c.cycles += 2;
                crate::abi::ret(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x003e_cfb0, 0x003e_cff8);
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// BE int rewards / camera.

/// `int BERewardInt_GetEventRecordValue(profile, event, a2, a3, t0, t1)`
/// (0x158960): look the profile's event record up in
/// `D_004A6CA8[...]` and return its second word.
pub fn be_reward_int_get_event_record_value(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        c.g[V0][0] = 0xf88;
        c.g[V1][0] = 0x9b50;
        c.mult(0, A0, A2, V0); // a0 = a2 * sizeof(sProfileSlot)
        c.mult(0, T1, A1, V1); // t1 = a1 * sizeof(record)
        c.g[V0][0] = 0x004a_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(0x004a_0000u32.wrapping_add(0x6ca8)); // D_004A6CA8
        c.g[A1][0] = c.g[T0][0];
        c.g[A2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        BE_GET_CHAR_PROFILE_EVENT_RECORD.call(c, 0x0015_8994)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ManualCamController_ResetForRider(cam, rider)` (0x15ce98): reset the
/// rider's camera controller and tell its algorithm list (slot 0x18/0x1c) to
/// follow the rider.
pub fn manual_cam_controller_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        CAM_CONTROLLER_RESET_FOR_RIDER.call(c, 0x0015_ceb4)?;
        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x18)).lw(c);
        let cam = c.g[A2][0] as u32;
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = Addr(cam).off(0x10).lw(c);
        let vt = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vt).off(0x18).lh(c);
        c.g[V1][0] = Addr(vt).off(0x1c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0015_ced0);
        c.g[A0][0] = sx(cam.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0015_ced0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ChaseCamController_dtor(this, flags)` (0x161a10): install the chase
/// camera vtable, release the camera object at `+0x20`, then run the
/// CamController destructor.
pub fn chase_cam_controller_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(0x46f8));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(c.g[S0][0] as u32).off(0x14).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x20)).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0016_1a3c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        CAM_CONTROLLER_DTOR.call(c, 0x0016_1a48)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraTriggerMan_Reset(man)` (0x16c918): clear the camera trigger
/// manager's header, its two active lists, its two trigger stacks and reset
/// the chase controller switches.
pub fn camera_trigger_man_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        CAMERA_TRIGGER_MAN_CLEAR_DATA.call(c, 0x0016_c92c)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
        ACTIVE_CAMERA_TRIGGER_LIST_CLEAR.call(c, 0x0016_c934)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        ACTIVE_CAMERA_TRIGGER_LIST_CLEAR.call(c, 0x0016_c93c)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
        CAMERA_TRIGGER_STACK_CLEAR.call(c, 0x0016_c944)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3c));
        CAMERA_TRIGGER_STACK_CLEAR.call(c, 0x0016_c94c)?;
        call_ptr(c, 0x0015_dc18, 0x0016_c954)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DebugMenuModule_Enter()` (0x17fa60): release the current game
/// object, clear the debug menu's active flag, then ask the game object's
/// vtable slot 0x78/0x7c for a view-space point and pass it on.
pub fn debug_menu_module_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
        frame.save_ra(c, 0x10);
        call_ptr(c, 0x0036_9ff8, 0x0017_fa74)?;
        Addr((c.g[GP][0] as u32).wrapping_add(0x1234)).sw(c, 0);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x10d8).lw(c);
        let game = c.g[V0][0] as u32;
        let vt = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(vt).off(0x78).lh(c);
        Addr(c.g[SP][0] as u32).sw(c, 0);
        Addr((c.g[SP][0] as u32).wrapping_add(4)).sw(c, 0);
        c.g[A0][0] = sx(game.wrapping_add(c.g[A0][0] as u32));
        Addr((c.g[SP][0] as u32).wrapping_add(8)).sw(c, 0);
        c.g[V0][0] = Addr(vt).off(0x7c).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0017_faa0);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, target, 0x0017_faa0)?;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        frame.close(c);
        Ok(())
    })
}

/// `bool cFEStateMemCard_IsScanDoneWithCard(self)` (0x186f40): false until the
/// memory-card manager is in a scan state (>= 4) and reports the card ready.
pub fn c_fe_state_mem_card_is_scan_done_with_card(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0022_7f80, 0x0018_6f54)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1c34)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 4) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 5;
            call_ptr(c, 0x0023_c8d0, 0x0018_6f70)?;
            c.g[V1][0] = c.g[V0][0];
            let ready = c.g[V1][0] != 0;
            c.g[V0][0] = 1;
            if !ready {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end states.

/// `cFEStateRulesSelect* cFEStateBraggingRights_InitPlayerRows(self)`
/// (0x1926f0): install the vtable, clear the eight player rows (0x8..0x24,
/// descending) and the selection state; return `self`.
pub fn c_fe_state_bragging_rights_init_player_rows(c: &mut Cpu) {
    c.cycles += 6;
    let this = c.g[A0][0] as u32;
    c.g[V0][0] = 0x0047_0000;
    c.g[V1][0] = 0x7;
    c.g[V0][0] = sx(0x0047_0000u32.wrapping_sub(0x4f88)); // &D_0046B078
    Addr(this).off(0x30).sw(c, c.g[V0][0]);
    c.g[V0][0] = sx(this.wrapping_add(0x24));
    loop {
        c.cycles += 7;
        Addr(c.g[V0][0] as u32).sw(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0019_2708);
        }
    }
    c.cycles += 6;
    c.g[V0][0] = c.g[A0][0];
    Addr(this).sw(c, 0);
    Addr(this).off(0x2c).sw(c, 0);
    Addr(this).off(0x28).sw(c, 0);
    Addr(this).off(4).sw(c, 0);
    crate::abi::ret(c);
}

/// `void cFEStateCharEquip_ReleasePreview(self)` (0x19a498): release the
/// previewed equipment's bolt file (if a slot is held) and clear the preview
/// state.
pub fn c_fe_state_char_equip_release_preview(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[A2][0] = Addr(this).off(0xa6c).lw(c);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        if (c.g[A2][0] as i64) >= 0 {
            c.cycles += 4;
            c.g[A1][0] = Addr(this).off(0xbc).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x7c).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
            BOLT_FILE_RELEASE.call(c, 0x0019_a4c4)?;
        }
        c.cycles += 8;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        Addr(this).off(0x960).sb(c, 0);
        Addr(this).off(0x95c).sw(c, c.g[V0][0]);
        Addr(this).off(0xa6c).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of `BoltFileIndexManager_GetModelSlot` /
/// `GetTextureSlot`: find the index node for `(slot << 16) | index`, test bit
/// `slot` of the node's availability mask at `mask_off`, and return the value
/// at `value_off` (or -1).
fn bolt_slot_lookup(c: &mut Cpu, shift: u32, mask_off: u32, value_off: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        BOLT_FILE_INDEX_MANAGER_FIND_NODE.call(c, ret)?;
        c.g[V1][0] = c.g[V0][0];
        let node = c.g[V1][0] as u32;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if node != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(node).off(mask_off).lh(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> (shift & 31)) as u32);
            c.g[V0][0] &= 1;
            let available = c.g[V0][0] != 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if available {
                c.cycles += 1;
                c.g[V0][0] = Addr(node).off(value_off).lh(c);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BoltFileIndexManager_GetModelSlot(this, slot, index)` (0x19cbe0).
pub fn bolt_file_index_manager_get_model_slot(c: &mut Cpu) {
    let shift = c.g[A1][0] as u32;
    bolt_slot_lookup(c, shift, 6, 0xa, 0x0019_cbf8)
}

/// `int BoltFileIndexManager_GetTextureSlot(this, slot, index)` (0x19cce8).
pub fn bolt_file_index_manager_get_texture_slot(c: &mut Cpu) {
    let shift = c.g[A1][0] as u32;
    bolt_slot_lookup(c, shift, 4, 8, 0x0019_cd00)
}

/// `cFEStateOnlineCharacterSelect* cFEStateOnlineCharacterSelect_ctor(this,
/// a1)` (0x1b0f60): construct the NetFE base (child-event handler in `a2 =
/// 0`), install the class vtable and id 0x41, and clear the selection state.
pub fn c_fe_state_online_character_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        NET_FE_STATE_CTOR.call(c, 0x001b_0f78)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[A1][0] = 0x1;
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x73f0)); // &D_00468C10
        c.g[A0][0] = 0x41;
        let this = c.g[S0][0] as u32;
        Addr(this).off(8).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(this).off(0x6dc).sw(c, c.g[A1][0]);
        Addr(this).off(0xc).sw(c, c.g[A0][0]);
        Addr(this).off(0x6d8).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetCreateRoomPopup* cFEStateNetCreateRoomPopup_ctor(this, a1)`
/// (0x1be7d8): construct the NetFE base, install the class vtable and id 0x47,
/// and copy the current player count into the three popup fields.
pub fn c_fe_state_net_create_room_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        NET_FE_STATE_CTOR.call(c, 0x001b_e7ec)?;
        c.g[V0][0] = 0x0047_0000;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xda0)).lw(c);
        c.g[V0][0] = sx(0x0047_0000u32.wrapping_sub(0x5918)); // &D_0046A6E8
        c.g[A0][0] = 0x47;
        let this = c.g[S0][0] as u32;
        Addr(this).off(8).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(this).off(0x6e4).sw(c, c.g[V1][0]);
        Addr(this).off(0xc).sw(c, c.g[A0][0]);
        Addr(this).off(0x6dc).sw(c, c.g[V1][0]);
        Addr(this).off(0x6e0).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEPopupConfirm_GetWidgetSize(self, widget, out)` (0x1c8628): ask the
/// widget for its size (vtable 0xa0/0xa4, written through the stack
/// argument), then store the two floats at `out+8`/`out+0xc`.
pub fn c_fe_popup_confirm_get_widget_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A2][0];
        let widget = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(widget).off(8).lw(c);
        let vt = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(vt).off(0xa0).lh(c);
        c.g[A2][0] = Addr(vt).off(0xa4).lw(c);
        let target = c.g[A2][0] as u32;
        c.g[RA][0] = sx(0x001c_8654);
        c.g[A0][0] = sx(widget.wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x001c_8654)?;
        c.lwc1(0, c.g[SP][0] as u32);
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(4));
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(8));
        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
        frame.restore(c, S0, 0x20);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        frame.close(c);
        Ok(())
    })
}

/// `void cFEPopupConfirm_Update(self)` (0x1ca528): if the popup has a body
/// widget (`+0x2c0`), let it lay itself out (vtable slot 0x60/0x64), then run
/// the base UI update.
pub fn c_fe_popup_confirm_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(this).off(0x2c0).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[A1][0] = Addr(this).off(0x20).lw(c);
            let widget = c.g[A1][0] as u32;
            c.g[V0][0] = Addr(widget).off(8).lw(c);
            let vt = c.g[V0][0] as u32;
            c.g[A0][0] = Addr(vt).off(0x60).lh(c);
            c.g[V1][0] = Addr(vt).off(0x64).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001c_a55c);
            c.g[A0][0] = sx(widget.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_a55c)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e510, 0x001c_a564)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the five `cFEStateRewardGallery*_InitButtons`
/// handlers: copy the first four characters of the `slot`-th entry's name
/// (`(self + slot*4 + 0xa28)` -> `+8`) into a stack buffer and look the
/// texture up by that name.
fn reward_gallery_init_button(c: &mut Cpu, slot: u32, ret1: u32, ret2: u32) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx(slot << 2);
        frame.save(c, S0, 0x20);
        c.g[A2][0] = 0x4;
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[A1][0] = sx(this.wrapping_add(slot << 2));
        c.g[A0][0] = c.g[SP][0];
        c.g[V0][0] = Addr((this.wrapping_add(slot << 2)).wrapping_add(0xa28)).lw(c);
        c.g[A1][0] = Addr(c.g[V0][0] as u32).off(8).lw(c);
        call_ptr(c, 0x0041_6ce0, ret1)?;
        c.g[A0][0] = c.g[S0][0];
        Addr((c.g[SP][0] as u32).wrapping_add(4)).sb(c, 0);
        c.g[A1][0] = c.g[SP][0];
        C_FE_STATE_REWARD_GALLERY_FIND_TEXTURE_BY_NAME.call(c, ret2)?;
        frame.restore(c, S0, 0x20);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRewardGalleryPosters_InitButtons(self, slot)` (0x1d11a8).
pub fn c_fe_state_reward_gallery_posters_v26(c: &mut Cpu) {
    let slot = c.g[A1][0] as u32;
    reward_gallery_init_button(c, slot, 0x001d_11d4, 0x001d_11e4);
}

/// `void cFEStateRewardGalleryTradingCards_InitButtons(self, slot)`
/// (0x1d1340).
pub fn c_fe_state_reward_gallery_trading_cards_v26(c: &mut Cpu) {
    let slot = c.g[A1][0] as u32;
    reward_gallery_init_button(c, slot, 0x001d_136c, 0x001d_137c);
}

/// `void cFEStateRewardGalleryConceptArts_InitButtons(self, slot)` (0x1d14d8).
pub fn c_fe_state_reward_gallery_concept_arts_v26(c: &mut Cpu) {
    let slot = c.g[A1][0] as u32;
    reward_gallery_init_button(c, slot, 0x001d_1504, 0x001d_1514);
}

/// `void cFEStateRewardGalleryVideos_InitButtons(self, slot)` (0x1d1670).
pub fn c_fe_state_reward_gallery_videos_v26(c: &mut Cpu) {
    let slot = c.g[A1][0] as u32;
    reward_gallery_init_button(c, slot, 0x001d_169c, 0x001d_16ac);
}

/// `void cFEStateRewardGalleryToys_InitButtons(self, slot)` (0x1d1808).
pub fn c_fe_state_reward_gallery_toys_v26(c: &mut Cpu) {
    let slot = c.g[A1][0] as u32;
    reward_gallery_init_button(c, slot, 0x001d_1834, 0x001d_1844);
}

/// `void cFEStateVideoPlayer_Init(self)` (0x1d23e0): allocate and construct
/// the 0x44-byte video engine (`0x461320`) into `+0x248`, then start playback.
pub fn c_fe_state_video_player_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x0046_0000;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx(0x0046_0000u32.wrapping_add(0x1320));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x44;
        call_ptr(c, 0x0031_7d70, 0x001d_2408)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0025_33c8, 0x001d_2410)?;
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x248).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        C_FE_STATE_VIDEO_PLAYER_START_PLAYBACK.call(c, 0x001d_241c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardUI_ShowFileDisplay(self)` (0x1d8358): move the front end into
/// the file-display state (mode 2) unless it is already there or a save is in
/// progress, and tell its vtable slot 0x118/0x11c.
pub fn mem_card_ui_show_file_display(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x10c8)).lw(c);
        frame.save_ra(c, 0);
        c.g[V1][0] = 0x2;
        let fe = c.g[A2][0] as u32;
        c.g[V0][0] = Addr(fe).off(0x214).lw(c);
        let in_mode = c.g[V0][0] == c.g[V1][0];
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        if !in_mode {
            c.cycles += 3;
            c.g[V0][0] = Addr(fe).off(0x1c0).lw(c);
            let busy = c.g[V0][0] != 0;
            c.g[A1][0] = 0x2;
            if !busy {
                c.cycles += 7;
                Addr(fe).off(0x214).sw(c, c.g[V1][0]);
                c.g[V0][0] = Addr(fe).off(8).lw(c);
                let vt = c.g[V0][0] as u32;
                c.g[A0][0] = Addr(vt).off(0x118).lh(c);
                c.g[V1][0] = Addr(vt).off(0x11c).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001d_8398);
                c.g[A0][0] = sx(fe.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001d_8398)?;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardStateBase_v45(self)` (0x1d86b0): enable the save title, set
/// the "check cards" mode (5) and call the state's vtable slot 0x178/0x17c.
pub fn mem_card_state_base_v45(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        MEM_CARD_STATE_BASE_SET_USE_SAVE_TITLE.call(c, 0x001d_86c8)?;
        c.g[V0][0] = 0x1;
        c.g[V1][0] = 0x5;
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x1a8).sw(c, c.g[V0][0]);
        c.g[V0][0] = Addr(this).off(8).lw(c);
        Addr(this).off(0x1c0).sw(c, c.g[V1][0]);
        let vt = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vt).off(0x178).lh(c);
        c.g[V1][0] = Addr(vt).off(0x17c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001d_86ec);
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_86ec)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, ptr, vtable};
    use crate::difftest::{check, DATA, GAME_GP};

    /// A nonzero BELibrary singleton, so the BE init path in `0x14dc80` does
    /// not try to create the library (and allocate) from disc.
    fn be_library(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
    }

    /// The engine's size-class pool allocator: `gp+0xdd8` ready plus 64 free
    /// nodes in each of the seven size classes, spaced by that class's block
    /// size (so a returned block never overlaps the next free node). Without
    /// it a class with an empty list recurses through `MEM_PoolAlloc`.
    fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        // (gp slot, free-list head offset in the pool, block stride)
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 0x40),
            (0x2a60, 0x2000, 0x40),
            (0x2a6c, 0x6000, 0x40),
            (0x2a7c, 0x32000, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut pool = DATA + 0x2_0000;
        let mut base = DATA + 0x6_0000;
        for (slot, head, stride) in POOLS {
            for n in 0..64u32 {
                let node = base + stride * n;
                c.bus.write32(node, if n == 63 { 0 } else { node + stride });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
            pool += 0x100;
            base += 0x8000;
        }
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)`: a real heap behind the
    /// pools, for the deep constructors whose allocations miss a pool.
    fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const ARENA: u32 = DATA + 0x20_0000;
        c.g[GP][0] = GAME_GP as u64;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = ARENA as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, crate::difftest::RA, "DLHeap_InitArena did not return");
    }

    /// Zero the test stack: `machine` fills it with random bytes, which the
    /// deep constructors otherwise read out of their uninitialised temporaries
    /// (sizes and pointers that then drive loops).
    fn clear_stack(c: &mut Cpu) {
        let sp = crate::difftest::SP;
        for a in (sp - 0x4000..sp + 0x40).step_by(4) {
            c.bus.write32(a, 0);
        }
    }

    // -----------------------------------------------------------------------
    // 0x2ee6f0 WorldPainter_GetWeather30.

    #[test]
    fn world_painter_get_weather30_matches() {
        check(ee::s09::f_002ee6f0__generated, world_painter_get_weather30, 32, &|c, seed| {
            let mut r = machine(c, seed, 0x2ee6_f0, 1);
            let i = seed % 8;
            c.g[A0][0] = i as u64;
            let slot = 0x004f_a370 + 0xf0 * i;
            let obj = DATA + 0x200;
            let vt = vtable(c, &mut r, &[0x180]);
            c.bus.write32(slot + 0x20, DATA + 0x100);
            c.bus.write32(DATA + 0x100, obj);
            c.bus.write32(obj + 4, vt);
        });
    }

    // -----------------------------------------------------------------------
    // 0x307d30 WScriptProcess_v06.

    #[test]
    fn w_script_process_v06_matches() {
        check(ee::s10::f_00307d30__generated, w_script_process_v06, 24, &|c, seed| {
            let _r = machine(c, seed, 0x307d_30, 2);
            pool_allocator(c);
            be_library(c);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            let proc = DATA + 0x400;
            let script = DATA + 0x500;
            c.bus.write32(this + 0x10, proc);
            c.bus.write32(this + 0x1c, 0);
            c.bus.write32(proc + 0x2a4, DATA + 0x600);
            c.bus.write32(proc + 0x290, 0);
            // The string helper reads a word from the caller's frame.
            c.bus.write32(crate::difftest::SP - 0xc0, 0);
            c.bus.write32(this + 0x30, if seed % 2 == 0 { 0 } else { script });
        });
    }

    // -----------------------------------------------------------------------
    // 0x307dc8 WScriptProcess_v02.

    #[test]
    fn w_script_process_v02_matches() {
        check(ee::s10::f_00307dc8__generated, w_script_process_v02, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x307d_c8, 1);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            if seed % 2 == 0 {
                c.bus.write32(this + 0x10, 0); // not in any list
            } else {
                let proc = DATA + 0x400;
                let node = DATA + 0x600;
                let id = r.next_u32() & 0xffff;
                c.bus.write32(this + 0x10, proc);
                c.bus.write32(proc + 0x2b8, node);
                c.bus.write32(node + 0x20, id);
                c.bus.write32(this + 0x20, id);
                let vt = vtable(c, &mut r, &[0x30]);
                c.bus.write32(this + 0x5c, vt);
            }
        });
    }

    // -----------------------------------------------------------------------
    // 0x309750 WScriptMan_StartProcess / 0x309798 WScriptMan_StopProcess.

    #[test]
    fn w_script_man_start_process_matches() {
        check(ee::s10::f_00309750__generated, w_script_man_start_process, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x3097_50, 1);
            let man = DATA + 0x100;
            let process = DATA + 0x300;
            c.g[A0][0] = sx(man);
            c.g[A1][0] = sx(process);
            c.bus.write32(man + 0x2a4, ptr(&mut r));
            let vt = vtable(c, &mut r, &[0x20]);
            c.bus.write32(process + 0x5c, vt);
        });
    }

    #[test]
    fn w_script_man_stop_process_matches() {
        check(ee::s10::f_00309798__generated, w_script_man_stop_process, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x3097_98, 1);
            let man = DATA + 0x100;
            let process = DATA + 0x300;
            c.g[A0][0] = sx(man);
            c.g[A1][0] = sx(process);
            c.bus.write32(man + 0x2a4, ptr(&mut r));
            let vt = vtable(c, &mut r, &[0x30]);
            c.bus.write32(process + 0x5c, vt);
        });
    }

    // -----------------------------------------------------------------------
    // 0x343bc0 CrowdMan3d_Reset.

    #[test]
    fn crowd_man3d_reset_matches() {
        check(ee::s11::f_00343bc0__generated, crowd_man3d_reset, 16, &|c, seed| {
            let _r = machine(c, seed, 0x343b_c0, 2);
            c.g[A0][0] = sx(DATA);
            // Five zeroed groups: GroupFree finds no mesh lists, GroupReset
            // only writes fields.
            for i in 0..5u32 {
                c.bus.write32(DATA + i * 0x1f4 + 0x1ec, 0);
                c.bus.write32(DATA + i * 0x1f4 + 0x1e8, 0);
            }
        });
    }

    // -----------------------------------------------------------------------
    // 0x355420 ModObj_v14 / 0x356a70 ModObj_v39.

    #[test]
    fn mod_obj_v14_matches() {
        check(ee::s11::f_00355420__generated, mod_obj_v14, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x3554_20, 1);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            match seed % 3 {
                0 => c.bus.write32(this + 0x1c, 0),
                1 => c.bus.write32(this + 0x1c, DATA + 0x200), // block, *block == 0
                _ => {
                    let node = DATA + 0x300;
                    c.bus.write32(this + 0x1c, DATA + 0x200);
                    c.bus.write32(DATA + 0x200, node);
                    let vt = vtable(c, &mut r, &[0x40]);
                    c.bus.write32(node, vt);
                }
            }
        });
    }

    #[test]
    fn mod_obj_v39_matches() {
        check(ee::s11::f_00356a70__generated, mod_obj_v39, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x356a_70, 1);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            match seed % 3 {
                0 => c.bus.write32(this + 0x1c, 0),
                1 => c.bus.write32(this + 0x1c, DATA + 0x200),
                _ => {
                    let node = DATA + 0x300;
                    c.bus.write32(this + 0x1c, DATA + 0x200);
                    c.bus.write32(DATA + 0x200, node);
                    let vt = vtable(c, &mut r, &[0xa8]);
                    c.bus.write32(node, vt);
                }
            }
        });
    }

    // -----------------------------------------------------------------------
    // 0x359228 AnimDelta_ctor.

    #[test]
    fn anim_delta_ctor_matches() {
        check(ee::s11::f_00359228__generated, anim_delta_ctor, 8, &|c, seed| {
            let _r = machine(c, seed, 0x3592_28, 0);
            pool_allocator(c);
            be_library(c);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(DATA + 0x400);
            c.g[A2][0] = sx(DATA + 0x800);
            c.g[A3][0] = sx(DATA + 0xc00);
        });
    }

    // -----------------------------------------------------------------------
    // 0x3617b8 Floating_v27.

    #[test]
    fn floating_v27_matches() {
        check(ee::s12::f_003617b8__generated, floating_v27, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x3617_b8, 1);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            c.bus.write16(this + 0x12, (r.next_u32() & 1) as u16);
            c.bus.write32(this + 0x74, r.next_u32());
            let vt = vtable(c, &mut r, &[0x198]);
            c.bus.write32(this + 0xc, vt);
        });
    }

    // -----------------------------------------------------------------------
    // 0x37c7c0 PS2GraphicsMan_ResetTextureSlots.

    #[test]
    fn ps2_graphics_man_reset_texture_slots_matches() {
        check(ee::s12::f_0037c7c0__generated, ps2_graphics_man_reset_texture_slots, 8, &|c, seed| {
            let _r = machine(c, seed, 0x37c7_c0, 0);
            c.g[A0][0] = sx(DATA + 0x1000);
            c.g[A1][0] = (seed as u64) << 4;
            c.bus.write32(DATA + 0x1000 + 0x18f4, DATA + 0x9000);
        });
    }

    // -----------------------------------------------------------------------
    // 0x389118 PS2GraphicsMan_RemoveInterruptHandlers.

    #[test]
    fn ps2_graphics_man_remove_interrupt_handlers_matches() {
        check(ee::s12::f_00389118__generated, ps2_graphics_man_remove_interrupt_handlers, 8, &|c, seed| {
            let mut r = machine(c, seed, 0x3891_18, 0);
            let this = DATA + 0x1000;
            c.g[A0][0] = sx(this);
            c.bus.write32(this + 0x5ac0, r.next_u32());
            c.bus.write32(this + 0x5ac4, r.next_u32());
        });
    }

    // -----------------------------------------------------------------------
    // 0x38aeb0 PS2BezierMan_dtor.

    #[test]
    fn ps2_bezier_man_dtor_matches() {
        check(ee::s12::f_0038aeb0__generated, ps2_bezier_man_dtor, 8, &|c, seed| {
            let _r = machine(c, seed, 0x38ae_b0, 0);
            c.g[A0][0] = sx(DATA + 0x1000);
            c.g[A1][0] = ((seed as u64) & 0xfe) << 1; // flags bit 0 clear: no delete
            for off in [0x440u32, 0x444, 0x448] {
                c.bus.write32(DATA + 0x1000 + off, 0);
            }
        });
    }

    // -----------------------------------------------------------------------
    // 0x3983f0 UIFontSet_FindFont.

    #[test]
    fn ui_font_set_find_font_matches() {
        check(ee::s13::f_003983f0__generated, ui_font_set_find_font, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x3983_f0, 0);
            let bank = DATA + 0x100;
            c.g[A0][0] = sx(bank);
            let count = (seed % 5) as u8;
            c.bus.write8(bank + 0x34, count);
            let key = r.next_u32();
            let hit = (seed % 3) as u32;
            for i in 0..4u32 {
                let entry = bank + 4 + 0xc * i;
                c.bus.write32(entry, if i == hit { key } else { r.next_u32() });
                c.bus.write32(entry + 8, r.next_u32());
            }
            c.g[A1][0] = key as u64;
        });
    }

    // -----------------------------------------------------------------------
    // 0x39ce48 UIScreen_CmdBeginTransitionOut.

    #[test]
    fn ui_screen_cmd_begin_transition_out_matches() {
        check(ee::s13::f_0039ce48__generated, ui_screen_cmd_begin_transition_out, 16, &|c, seed| {
            let mut r = machine(c, seed, 0x39ce_48, 1);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            for i in 0..8u32 {
                c.bus.write32(this + 0x40 + i * 4, r.next_u32());
            }
            if seed % 2 == 0 {
                c.bus.write32(this + 0xd0, 0);
            } else {
                let owner = DATA + 0x300;
                c.bus.write32(this + 0xd0, owner);
                c.bus.write32(owner + 0x1c, r.next_u32());
            }
        });
    }

    // -----------------------------------------------------------------------
    // 0x39e4c0 UIState_OnActivate.

    #[test]
    fn ui_state_on_activate_matches() {
        check(ee::s13::f_0039e4c0__generated, ui_state_on_activate, 16, &|c, seed| {
            let mut r = machine(c, seed, 0x39e4_c0, 1);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            if seed % 2 == 0 {
                c.bus.write32(this + 0x28, 0);
            } else {
                let node = DATA + 0x300;
                c.bus.write32(this + 0x28, node);
                c.bus.write32(node + 4, r.next_u32() | 1);
                let vt = vtable(c, &mut r, &[0x68]);
                c.bus.write32(this + 8, vt);
            }
        });
    }

    // -----------------------------------------------------------------------
    // 0x3a54b8 UIPair_dtor / 0x3a5658 UIListBox_dtor / 0x3a5da8 UIText_dtor.

    fn dtor_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let this = DATA + 0x100;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = ((seed as u64) & 0xfe) << 1; // flags bit 0 clear: no delete
        // A zeroed UI object with empty child lists.
        for off in [0x78u32, 0x7c, 0xc, 0xac, 0xb4] {
            c.bus.write32(this + off, 0);
        }
        c.bus.write32(this + 0x9c, 0);
        c.bus.write8(this + 0x318, 0);
        let owner = DATA + 0x800;
        c.bus.write32(this + 0x10, owner);
        c.bus.write32(owner + 0x28, 0);
        let _ = r.next_u32();
    }

    #[test]
    fn ui_pair_dtor_matches() {
        check(ee::s13::f_003a54b8__generated, ui_pair_dtor, 8, &|c, seed| dtor_setup(c, seed, 0x3a54_b8));
    }

    #[test]
    fn ui_list_box_dtor_matches() {
        check(ee::s13::f_003a5658__generated, ui_list_box_dtor, 8, &|c, seed| {
            dtor_setup(c, seed, 0x3a56_58)
        });
    }

    #[test]
    fn ui_text_dtor_matches() {
        check(ee::s13::f_003a5da8__generated, ui_text_dtor, 8, &|c, seed| dtor_setup(c, seed, 0x3a5d_a8));
    }

    // -----------------------------------------------------------------------
    // 0x3aa3f0 cPS2ChunkResolveBase_v05.

    #[test]
    fn c_ps2_chunk_resolve_base_v05_matches() {
        check(ee::s13::f_003aa3f0__generated, c_ps2_chunk_resolve_base_v05, 16, &|c, seed| {
            let mut r = machine(c, seed, 0x3aa3_f0, 1);
            let resolver = DATA + 0x100;
            let chunk = DATA + 0x200;
            c.g[A0][0] = sx(resolver);
            c.g[A1][0] = sx(chunk);
            c.bus.write32(resolver + 8, DATA + 0x300);
            c.bus.write32(chunk + 8, r.next_u32());
            c.bus.write32(chunk + 0xc, r.next_u32());
            let vt = vtable(c, &mut r, &[0x30]);
            c.bus.write32(DATA + 0x300, vt);
        });
    }

    // -----------------------------------------------------------------------
    // 0x3acc90 tWScriptCache_Free.

    #[test]
    fn t_w_script_cache_free_matches() {
        check(ee::s13::f_003acc90__generated, t_w_script_cache_free, 16, &|c, seed| {
            let _r = machine(c, seed, 0x3acc_90, 0);
            let entry = DATA + 0x100;
            c.g[A0][0] = sx(entry);
            if seed % 2 == 0 {
                c.bus.write32(entry + 8, 0);
            } else {
                c.bus.write32(entry + 8, DATA + 0x400);
                c.bus.write16(entry + 2, 1); // borrowed: never freed
            }
            c.bus.write16(entry, 0xffff);
        });
    }

    // -----------------------------------------------------------------------
    // 0x3ebb60 TagFieldStrnzcpy.

    #[test]
    fn tag_field_strnzcpy_matches() {
        check(ee::s14::f_003ebb60__generated, tag_field_strnzcpy, 32, &|c, seed| {
            let mut r = machine(c, seed, 0x3ebb_60, 0);
            c.pc = 0x003e_bb60;
            let src = DATA + 0x200;
            let dst = DATA + 0x400;
            c.g[A0][0] = sx(dst);
            c.g[A1][0] = (seed % 6) as u64;
            c.g[A2][0] = sx(src);
            let n = match seed % 4 {
                0 => 0,
                1 => 1,
                2 => 3,
                _ => 5,
            };
            for i in 0..n {
                c.bus.write8(src + i, b'a' + (r.next_u32() % 26) as u8);
            }
            c.bus.write8(src + n, 0);
        });
    }

    // -----------------------------------------------------------------------
    // 0x3ecfb0 TagFieldGetFlags.

    #[test]
    fn tag_field_get_flags_matches() {
        check(ee::s14::f_003ecfb0__generated, tag_field_get_flags, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x3ecf_b0, 0);
            c.pc = 0x003e_cfb0;
            let text = DATA + 0x200;
            c.g[A1][0] = r.next_u32() as u64;
            if seed % 4 == 0 {
                c.g[A0][0] = 0;
            } else {
                c.g[A0][0] = sx(text);
                let n = 1 + seed % 6;
                for i in 0..n {
                    c.bus.write8(text + i, (r.next_u32() % 0x30) as u8);
                }
                c.bus.write8(text + n, 0);
                // A table of nonzero flag words for the tag characters used.
                for i in 0..0x30u32 {
                    let (ch, flags) = (i as u8, (i * 3) as u32);
                    c.bus.write32(0x0049_65e0 + 4 * ch as u32, flags);
                }
            }
        });
    }

    // -----------------------------------------------------------------------
    // 0x158960 BERewardInt_GetEventRecordValue.

    #[test]
    fn be_reward_int_get_event_record_value_matches() {
        check(ee::s01::f_00158960__generated, be_reward_int_get_event_record_value, 16, &|c, seed| {
            let _r = machine(c, seed, 0x1589_60, 1);
            c.g[A0][0] = (seed % 10) as u64;
            c.g[A1][0] = (seed % 7) as u64;
            c.g[A2][0] = (seed % 5) as u64;
            c.g[A3][0] = if seed % 2 == 0 { 8 } else { 0x20 }; // record index / "missing"
            c.g[T0][0] = 20; // >= 17: the cheap index path
            c.g[T1][0] = (seed % 3) as u64;
            c.bus.write32(0x004a_6ca8, 1);
        });
    }

    // -----------------------------------------------------------------------
    // 0x15ce98 ManualCamController_ResetForRider.

    #[test]
    fn manual_cam_controller_reset_for_rider_matches() {
        check(ee::s01::f_0015ce98__generated, manual_cam_controller_reset_for_rider, 16, &|c, seed| {
            let mut r = machine(c, seed, 0x15ce_98, 2);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(DATA + 0x400);
            c.bus.write32(this + 8, 0); // empty algorithm list
            c.bus.write32(this + 0x10, DATA + 0x200);
            c.bus.write32(this + 0x18, DATA + 0x300);
            let vt = vtable(c, &mut r, &[0x18]);
            c.bus.write32(DATA + 0x300 + 0x10, vt);
        });
    }

    // -----------------------------------------------------------------------
    // 0x161a10 ChaseCamController_dtor.

    #[test]
    fn chase_cam_controller_dtor_matches() {
        check(ee::s01::f_00161a10__generated, chase_cam_controller_dtor, 8, &|c, seed| {
            let _r = machine(c, seed, 0x161a_10, 0);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = ((seed as u64) & 0xfe) << 1; // flags bit 0 clear: no delete
            c.bus.write32(this + 0x20, 0); // null camera: free is a no-op
            c.bus.write32(this + 8, 0); // no algorithm list
        });
    }

    // -----------------------------------------------------------------------
    // 0x16c918 CameraTriggerMan_Reset.

    #[test]
    fn camera_trigger_man_reset_matches() {
        check(ee::s02::f_0016c918__generated, camera_trigger_man_reset, 8, &|c, seed| {
            let _r = machine(c, seed, 0x16c9_18, 0);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            for off in [0x14u32, 0x18, 0x1c, 0x3c] {
                c.bus.write32(this + off, 0); // empty lists/stacks
            }
        });
    }

    // -----------------------------------------------------------------------
    // 0x17fa60 DebugMenuModule_Enter.

    #[test]
    fn debug_menu_module_enter_matches() {
        check(ee::s02::f_0017fa60__generated, debug_menu_module_enter, 8, &|c, seed| {
            let _r = machine(c, seed, 0x17fa_60, 1);
            c.bus.write32(GAME_GP.wrapping_sub(0x854), 0);
        });
    }

    // -----------------------------------------------------------------------
    // 0x186f40 cFEStateMemCard_IsScanDoneWithCard.

    #[test]
    fn c_fe_state_mem_card_is_scan_done_with_card_matches() {
        check(ee::s02::f_00186f40__generated, c_fe_state_mem_card_is_scan_done_with_card, 16, &|c, seed| {
            let _r = machine(c, seed, 0x186f_40, 0);
            let world = DATA + 0x1000;
            let mc = DATA + 0x2000;
            c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
            c.bus.write32(GAME_GP.wrapping_sub(0x47c), mc);
            c.bus.write32(mc + 0x43c, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(GAME_GP.wrapping_sub(0x1c34), (seed % 5) as u32);
        });
    }

    // -----------------------------------------------------------------------
    // 0x1926f0 cFEStateBraggingRights_InitPlayerRows.

    #[test]
    fn c_fe_state_bragging_rights_init_player_rows_matches() {
        check(ee::s02::f_001926f0__generated, c_fe_state_bragging_rights_init_player_rows, 8, &|c, seed| {
            let _r = machine(c, seed, 0x1926_f0, 0);
            c.g[A0][0] = sx(DATA + 0x100);
        });
    }

    // -----------------------------------------------------------------------
    // 0x19a498 cFEStateCharEquip_ReleasePreview.

    #[test]
    fn c_fe_state_char_equip_release_preview_matches() {
        check(ee::s03::f_0019a498__generated, c_fe_state_char_equip_release_preview, 16, &|c, seed| {
            let _r = machine(c, seed, 0x19a4_98, 1);
            let this = DATA + 0x100;
            let world = DATA + 0x1000;
            c.g[A0][0] = sx(this);
            c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
            c.bus.write32(world + 0x7c, DATA + 0x2000);
            let idx = if seed % 2 == 0 { 0xffff_ffffu32 } else { 2 };
            c.bus.write32(this + 0xa6c, idx);
            c.bus.write32(this + 0xbc, 0); // slot 0: offsets collapse to +0x120
            // A live refcount so BoltFile_Release only decrements it.
            let bolt = DATA + 0x2000 + 0x1a70;
            let node = bolt + 2 * 0x120;
            c.bus.write32(node + 0x114, 5);
            c.bus.write32(node, 0);
        });
    }

    // -----------------------------------------------------------------------
    // 0x19cbe0 / 0x19cce8 BoltFileIndexManager slot lookups.

    fn bolt_slot_setup(c: &mut Cpu, seed: u32, salt: u64, mask_off: u32, value_off: u32) {
        let mut r = machine(c, seed, salt, 1);
        let index = DATA + 0x100;
        let slot = seed % 8;
        c.g[A0][0] = sx(index);
        c.g[A1][0] = slot as u64;
        c.g[A2][0] = 0;
        c.bus.write32(index + 0x1f40, 0); // no "always create" flag
        let key = slot << 16; // (slot << 16) | 0
        let bucket = key % 400;
        let node = DATA + 0x300;
        c.bus.write32(index + 0x1900 + bucket * 4, node);
        c.bus.write32(node, key);
        let mask = if seed % 3 == 0 {
            0
        } else {
            1u32 << (slot & 31)
        };
        c.bus.write16(node + mask_off, mask as u16);
        c.bus.write16(node + value_off, r.next_u32() as u16);
    }

    #[test]
    fn bolt_file_index_manager_get_model_slot_matches() {
        check(ee::s03::f_0019cbe0__generated, bolt_file_index_manager_get_model_slot, 24, &|c, seed| {
            bolt_slot_setup(c, seed, 0x19cb_e0, 6, 0xa)
        });
    }

    #[test]
    fn bolt_file_index_manager_get_texture_slot_matches() {
        check(ee::s03::f_0019cce8__generated, bolt_file_index_manager_get_texture_slot, 24, &|c, seed| {
            bolt_slot_setup(c, seed, 0x19cc_e8, 4, 8)
        });
    }

    // -----------------------------------------------------------------------
    // 0x1b0f60 cFEStateOnlineCharacterSelect_ctor.

    #[test]
    fn c_fe_state_online_character_select_ctor_matches() {
        check(ee::s03::f_001b0f60__generated, c_fe_state_online_character_select_ctor, 8, &|c, seed| {
            let _r = machine(c, seed, 0x1b0f_60, 1);
            let this = DATA + 0x1000;
            pool_allocator(c);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 4) as u64;
        });
    }

    // -----------------------------------------------------------------------
    // 0x1be7d8 cFEStateNetCreateRoomPopup_ctor.

    #[test]
    fn c_fe_state_net_create_room_popup_ctor_matches() {
        check(ee::s03::f_001be7d8__generated, c_fe_state_net_create_room_popup_ctor, 8, &|c, seed| {
            let _r = machine(c, seed, 0x1be7_d8, 1);
            let this = DATA + 0x1000;
            pool_allocator(c);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 4) as u64;
            c.bus.write32(GAME_GP + 0xda0, seed.wrapping_mul(7));
        });
    }

    // -----------------------------------------------------------------------
    // 0x1c8628 cFEPopupConfirm_GetWidgetSize.

    #[test]
    fn c_fe_popup_confirm_get_widget_size_matches() {
        check(ee::s03::f_001c8628__generated, c_fe_popup_confirm_get_widget_size, 16, &|c, seed| {
            let mut r = machine(c, seed, 0x1c86_28, 1);
            let widget = DATA + 0x200;
            let out = DATA + 0x400;
            c.g[A0][0] = 0;
            c.g[A1][0] = sx(widget);
            c.g[A2][0] = sx(out);
            let vt = vtable(c, &mut r, &[0xa0]);
            c.bus.write32(widget + 8, vt);
            c.bus.write32(DATA + 0xff0, r.next_u32());
        });
    }

    // -----------------------------------------------------------------------
    // 0x1ca528 cFEPopupConfirm_Update.

    #[test]
    fn c_fe_popup_confirm_update_matches() {
        check(ee::s03::f_001ca528__generated, c_fe_popup_confirm_update, 16, &|c, seed| {
            let mut r = machine(c, seed, 0x1ca5_28, 2);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            c.bus.write32(this + 0x2c0, (seed % 2) as u32);
            c.bus.write32(this + 0x20, DATA + 0x300);
            let vt = vtable(c, &mut r, &[0x60]);
            c.bus.write32(DATA + 0x300 + 8, vt);
            // Make the base UI update find an empty object graph.
            c.bus.write32(this + 0x10, 0);
            c.bus.write32(this + 0xc, 0);
            c.bus.write32(this + 0x24, 0);
            c.bus.write8(this + 0x14, 0);
            c.bus.write8(this + 0x15, 0);
        });
    }

    // -----------------------------------------------------------------------
    // 0x1d11a8 .. 0x1d1808 reward gallery InitButtons.

    fn reward_button_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 1);
        let this = DATA + 0x1000;
        let slot = seed % 8;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = slot as u64;
        let entry = DATA + 0x300;
        let name = DATA + 0x400;
        c.bus.write32(this + 0xa28 + slot * 4, entry);
        c.bus.write32(entry + 8, name);
        for i in 0..4u32 {
            c.bus.write8(name + i, (r.next_u32() % 0x40 + 1) as u8);
        }
        c.bus.write8(name + 4, 0);
        c.bus.write32(this + 0x78, 0); // no texture entries: the lookup ends at -1
        let _ = ptr(&mut r);
    }

    #[test]
    fn c_fe_state_reward_gallery_posters_v26_matches() {
        check(ee::s04::f_001d11a8__generated, c_fe_state_reward_gallery_posters_v26, 8, &|c, seed| {
            reward_button_setup(c, seed, 0x1d11_a8)
        });
    }

    #[test]
    fn c_fe_state_reward_gallery_trading_cards_v26_matches() {
        check(
            ee::s04::f_001d1340__generated,
            c_fe_state_reward_gallery_trading_cards_v26,
            8,
            &|c, seed| reward_button_setup(c, seed, 0x1d13_40),
        );
    }

    #[test]
    fn c_fe_state_reward_gallery_concept_arts_v26_matches() {
        check(
            ee::s04::f_001d14d8__generated,
            c_fe_state_reward_gallery_concept_arts_v26,
            8,
            &|c, seed| reward_button_setup(c, seed, 0x1d14_d8),
        );
    }

    #[test]
    fn c_fe_state_reward_gallery_videos_v26_matches() {
        check(ee::s04::f_001d1670__generated, c_fe_state_reward_gallery_videos_v26, 8, &|c, seed| {
            reward_button_setup(c, seed, 0x1d16_70)
        });
    }

    #[test]
    fn c_fe_state_reward_gallery_toys_v26_matches() {
        check(ee::s04::f_001d1808__generated, c_fe_state_reward_gallery_toys_v26, 8, &|c, seed| {
            reward_button_setup(c, seed, 0x1d18_08)
        });
    }

    // -----------------------------------------------------------------------
    // 0x1d23e0 cFEStateVideoPlayer_Init.

    

    // -----------------------------------------------------------------------
    // 0x1d8358 MemCardUI_ShowFileDisplay.

    #[test]
    fn mem_card_ui_show_file_display_matches() {
        check(ee::s04::f_001d8358__generated, mem_card_ui_show_file_display, 16, &|c, seed| {
            let mut r = machine(c, seed, 0x1d83_58, 1);
            let fe = DATA + 0x100;
            c.bus.write32(GAME_GP.wrapping_sub(0x10c8), fe);
            match seed % 3 {
                0 => c.bus.write32(fe + 0x214, 2),      // already in mode 2
                1 => c.bus.write32(fe + 0x1c0, 1),      // a save is in progress
                _ => {
                    c.bus.write32(fe + 0x214, 0);
                    c.bus.write32(fe + 0x1c0, 0);
                    let vt = vtable(c, &mut r, &[0x118]);
                    c.bus.write32(fe + 8, vt);
                }
            }
        });
    }

    // -----------------------------------------------------------------------
    // 0x1d86b0 MemCardStateBase_v45.

    #[test]
    fn mem_card_state_base_v45_matches() {
        check(ee::s04::f_001d86b0__generated, mem_card_state_base_v45, 16, &|c, seed| {
            let mut r = machine(c, seed, 0x1d86_b0, 2);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            let vt = vtable(c, &mut r, &[0x178]);
            c.bus.write32(this + 8, vt);
            c.bus.write32(GAME_GP.wrapping_sub(0x848), 0);
            c.bus.write32(GAME_GP.wrapping_sub(0x47c), DATA + 0x2000);
        });
    }
}
