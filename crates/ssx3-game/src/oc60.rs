//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for fifty small functions — gcc's forwarding wrappers, table
//! and vtable dispatchers, and four fixed-count loops — each with a
//! differential test against the generated body.
//!
//! - **Music player** (`0x2b3a70`..): pause/resume the song instance the
//!   player holds at `+0x408`, and its tag (`+0x418` tag flag, song `+0x58`).
//! - **World triggers** (`0x2b4b48`): destroy the singleton trigger manager.
//! - **MemCard / menus** (`0x2c68c8`, `0x2cbf08`, `0x2d08e0`): the dir-entry
//!   name copy, the validated page selection index and the 2D item's focus.
//! - **WScript / PS2 exec / modifier blocks**: the small wrappers that
//!   forward to the script process, the exec manager's semaphores and the
//!   modifier-block methods, plus the command-queue constructor.
//! - **Render**: snapshot/free the graphics manager's texture table, the
//!   bezier/grid-mesh vtable thunks, free the light instances, write into the
//!   compiled-script cache and free a shared-header buffer.
//! - **Rider / AI / flags / BE singletons**: trick counters, press-spin,
//!   the 2-word sync block, the 64-bit flag move, the BE race/option/save
//!   field accessors.
//! - **Camera**: the heading ring reset and the three scripted-algorithm
//!   resets (a vtable call with `a1 = 0`).
//! - **Frontend**: the state `Update` chains (base update then the rider
//!   preview), the event-select activate/shutdown/update and the MemCard
//!   destructor.
//! - **Bolt file cache and rider previews**: cache construction (also
//!   publishes the singleton), the two preview transforms (`lq`/`sq` of the
//!   two `sQuadRM`s) and the preview slot indices.
//!
//! Every replacement keeps the frame layout, the register dance at each call
//! (`v0`/`v1`/`a0`..`a3`/`s0` as the original leaves them), the unwind check
//! after each call ([`crate::call`]) and the cycle counts of the generated
//! code; the loops deliver due events at the same back edge
//! ([`crate::events`]).

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_3a70, music_player_pause, "MusicPlayer_Pause"),
    (0x002b_3a98, music_player_resume, "MusicPlayer_Resume"),
    (0x002b_49e0, music_player_get_song_tag, "MusicPlayer_GetSongTag"),
    (0x002b_4b48, world_trigger_manager_destroy_instance, "WorldTriggerManager_DestroyInstance"),
    (0x002c_68c8, cps2_memcard_get_dir_entry_name, "cPS2MemCard_GetDirEntryName"),
    (0x002c_bf08, menu_sys_page_get_selected_index, "MenuSysPage_GetSelectedIndex"),
    (0x002d_08e0, menu_sys_point2d_item_on_focus, "MenuSysPoint2DItem_OnFocus"),
    (0x002d_1b30, mod_obj_call_slot7, "ModObj_CallSlot7"),
    (0x0030_a598, wscript_man_has_kill_script, "WScriptMan_HasKillScript"),
    (0x0030_a6d8, wscript_man_jump_to_task, "WScriptMan_JumpToTask"),
    (0x0030_a700, wscript_man_goto_task, "WScriptMan_GotoTask"),
    (0x0030_c738, wscript_man_cmd_queue_ctor, "WScriptMan_CmdQueueCtor"),
    (0x0031_aac8, ps2_exec_man_v05, "PS2ExecMan_v05"),
    (0x0031_acd8, ps2_exec_man_v09, "PS2ExecMan_v09"),
    (0x0035_6ab8, mod_obj_v43, "ModObj_v43"),
    (0x0035_6ae0, mod_obj_v42, "ModObj_v42"),
    (0x0035_6b08, mod_obj_v36, "ModObj_v36"),
    (0x0036_9ff8, graphics_man_snapshot_texture_table, "GraphicsMan_SnapshotTextureTable"),
    (0x0036_a020, graphics_man_free_all_textures, "GraphicsMan_FreeAllTextures"),
    (0x0038_d638, ps2_bezier_man_call_v17, "PS2BezierMan_CallV17"),
    (0x0039_4b98, grid_mesh_base_call_v22, "GridMeshBase_CallV22"),
    (0x0039_4ce8, light_man_free_instances, "LightMan_FreeInstances"),
    (0x003a_ca10, cwscript_cache_set_data, "cWScriptCache_SetData"),
    (0x003b_2b68, shp_free, "SHP_Free"),
    (0x0010_d170, computer_actor_rider_reset_trick_counts, "ComputerActorRider_ResetTrickCounts"),
    (0x0011_9ad8, rider_metrix_set_press_spin, "RiderMetrix_SetPressSpin"),
    (0x0012_bac0, ai_set_sync_words, "AI_SetSyncWords"),
    (0x0012_baf0, ai_get_sync_words, "AI_GetSyncWords"),
    (0x0014_4670, flags64_move_to_second, "Flags64_MoveToSecond"),
    (0x0014_55d0, benew_race_int_get_special_event_field4, "BENewRaceInt_GetSpecialEventField4"),
    (0x0014_5600, benew_race_int_get_special_event_field6, "BENewRaceInt_GetSpecialEventField6"),
    (0x0014_f648, beoption_int_reset_singleton, "BEOptionInt_ResetSingleton"),
    (0x0015_26d0, besave_int_get_save_buffer_end, "BESaveInt_GetSaveBufferEnd"),
    (0x0015_2728, besave_int_release_save_buffer, "BESaveInt_ReleaseSaveBuffer"),
    (0x0016_7db8, chase_camera_algorithm_clear_heading_ring, "ChaseCameraAlgorithm_ClearHeadingRing"),
    (0x0016_9540, manual_script_camera_algo_reset, "ManualScriptCameraAlgo_Reset"),
    (0x0016_9d80, target_script_camera_algo_reset, "TargetScriptCameraAlgo_Reset"),
    (0x0016_a4e0, subject_script_camera_algo_reset, "SubjectScriptCameraAlgo_Reset"),
    (0x0018_1420, cfe_state_char_select_update, "cFEStateCharSelect_Update"),
    (0x0018_2c38, cfe_state_char_setup_update, "cFEStateCharSetup_Update"),
    (0x0018_35c8, cfe_state_rider_detail_update, "cFEStateRiderDetail_Update"),
    (0x0018_63b8, cfe_options_credits_update, "cFEOptionsCredits_Update"),
    (0x0018_6748, cfe_state_event_select_on_activate, "cFEStateEventSelect_OnActivate"),
    (0x0018_6778, cfe_state_event_select_shutdown, "cFEStateEventSelect_Shutdown"),
    (0x0018_69d8, cfe_state_event_select_update, "cFEStateEventSelect_Update"),
    (0x0018_6b18, cfe_state_mem_card_dtor, "cFEStateMemCard_dtor"),
    (0x0019_cdf0, bolt_file_cache_ctor, "BoltFile_CacheCtor"),
    (0x0019_f3e8, femodule_rider_preview_set_transform, "FEModule_RiderPreviewSetTransform"),
    (0x0019_f548, femodule_rider_preview_set_transform2, "FEModule_RiderPreviewSetTransform2"),
    (0x001a_0498, femodule_set_rider_preview_indices, "FEModule_SetRiderPreviewIndices"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

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
    pub const STRLEN_COPY: u32 = 0x0041_66f4; // strcpy (newlib)
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
mod g {
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
// Helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn at(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global address.
#[inline(always)]
fn glo(c: &Cpu, off: i32) -> u32 {
    abi::r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Music player.

/// `void MusicPlayer_Pause(MusicPlayer* self)` (0x2b3a70): pause the current
/// song instance (`self->song` at +0x408), if one is playing.
pub fn music_player_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 0x408)).lw(c);
        let song = c.g[A0][0];
        frame.restore_ra(c, 0);
        if song != 0 {
            c.cycles += 3;
            callee::SONG_INSTANCE_PAUSE.call(c, 0x002b_3a8c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void MusicPlayer_Resume(MusicPlayer* self)` (0x2b3a98): resume the current
/// song instance, if one is loaded.
pub fn music_player_resume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 0x408)).lw(c);
        let song = c.g[A0][0];
        frame.restore_ra(c, 0);
        if song != 0 {
            c.cycles += 3;
            callee::SONG_INSTANCE_RESUME.call(c, 0x002b_3ab4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `int MusicPlayer_GetSongTag(MusicPlayer* self)` (0x2b49e0): the tag of the
/// current song (+0x58), or -1 while the player holds no tagged song
/// (+0x418 flag).
pub fn music_player_get_song_tag(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = Addr(at(self_, 0x418)).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[A0][0] = Addr(at(self_, 0x408)).lw(c);
        let song = c.g[A0][0];
        if song != 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(at(song as u32, 0x58)).lw(c);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World triggers.

/// `void WorldTriggerManager_DestroyInstance(void)` (0x2b4b48): destroy the
/// trigger manager singleton (with `a1 = 3`, the delete flavour) and clear
/// the pointer.
pub fn world_trigger_manager_destroy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(glo(c, g::WORLD_TRIGGER_MANAGER)).lw(c);
        let mgr = c.g[A0][0];
        frame.save_ra(c, 0);
        if mgr != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            callee::WORLD_TRIGGER_MANAGER_DTOR.call(c, 0x002b_4b60)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        Addr(glo(c, g::WORLD_TRIGGER_MANAGER)).sw(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// MemCard and menus.

/// `void cPS2MemCard_GetDirEntryName(self, int i, char* dst)` (0x2c68c8):
/// copy the `i`-th 64-byte directory entry name (at `+0x4e0`) into `dst`.
pub fn cps2_memcard_get_dir_entry_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(abi::arg(c, 0)));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A2][0];
        let src = c.g[A1][0] as u32;
        c.g[A1][0] = sx(at(src, 0x4e0));
        call_ptr(c, callee::STRLEN_COPY, 0x002c_68e4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int MenuSysPage_GetSelectedIndex(MenuSysPage* self)` (0x2cbf08): validate
/// the page's selection, then read the selected index (+0x4).
pub fn menu_sys_page_get_selected_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_PAGE_VALIDATE_SELECTION.call(c, 0x002c_bf1c)?;
        c.g[V0][0] = Addr(at(c.g[S0][0] as u32, 4)).lw(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MenuSysPoint2DItem_OnFocus(MenuSysPoint2DItem* self)` (0x2d08e0):
/// run the base item's focus handler and clear the item's focus flag (+0x40).
pub fn menu_sys_point2d_item_on_focus(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_ON_FOCUS.call(c, 0x002d_08f4)?;
        Addr(at(c.g[S0][0] as u32, 0x40)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Table and vtable dispatchers.

/// `void ModObj_CallSlot7(ModObj* obj)` (0x2d1b30): call the object's vtable
/// slot 7 (delta at `+0x38`, method at `+0x3c`) with the adjusted `this`.
pub fn mod_obj_call_slot7(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(abi::arg(c, 0)).lw(c);
        let vt = c.g[V1][0] as u32;
        c.g[A1][0] = Addr(at(vt, 0x38)).lh(c);
        c.g[V0][0] = Addr(at(vt, 0x3c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target, 0x002d_1b4c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void WScriptMan_HasKillScript(WScriptMan* self, int id)` (0x30a598): true
/// unless the object-script lookup (`self->slot at +0x28c`, class 4) returns
/// -1.
pub fn wscript_man_has_kill_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 4;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 0x28c)).lw(c);
        call_ptr(c, callee::WORLD_GET_OBJECT_SCRIPT_FUNC, 0x0030_a5ac)?;
        c.g[V0][0] = !c.g[V0][0];
        frame.restore_ra(c, 0);
        c.g[V0][0] = (c.g[V0][0] > 0) as u64;
        frame.close(c);
        Ok(())
    });
}

/// `void WScriptMan_JumpToTask(WScriptMan* self)` (0x30a6d8): make the
/// manager's script process (`+0x2a4`) jump to its current task.
pub fn wscript_man_jump_to_task(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 0x2a4)).lw(c);
        let proc = c.g[A0][0];
        frame.restore_ra(c, 0);
        if proc != 0 {
            c.cycles += 3;
            callee::WSCRIPT_PROCESS_JUMP_TO_TASK.call(c, 0x0030_a6f4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void WScriptMan_GotoTask(WScriptMan* self)` (0x30a700): the `GotoTask`
/// flavour of the above.
pub fn wscript_man_goto_task(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 0x2a4)).lw(c);
        let proc = c.g[A0][0];
        frame.restore_ra(c, 0);
        if proc != 0 {
            c.cycles += 3;
            callee::WSCRIPT_PROCESS_GOTO_TASK.call(c, 0x0030_a71c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `WScriptMan* WScriptMan_CmdQueueCtor(WScriptMan* self)` (0x30c738): reset
/// the command queue and return `self`.
pub fn wscript_man_cmd_queue_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::WSCRIPT_MAN_RESET_COMMAND_QUEUE.call(c, 0x0030_c74c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void PS2ExecMan_v05(PS2ExecMan* self)` (0x31aac8): release the exec
/// manager's semaphore (`+0x18`) if it holds one (`+0x1c`).
pub fn ps2_exec_man_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(abi::arg(c, 0), 0x1c)).lw(c);
        let held = c.g[V0][0];
        frame.restore_ra(c, 0);
        if held != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(at(abi::arg(c, 0), 0x18)).lw(c);
            call_ptr(c, callee::I_SIGNAL_SEMA, 0x0031_aae4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void PS2ExecMan_v09(PS2ExecMan* self, int value)` (0x31acd8): store
/// `value` in the manager's +0x14048 word and release the semaphore at
/// +0x14044.
pub fn ps2_exec_man_v09(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x1_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = sx(abi::arg(c, 0).wrapping_add(0x1_0000));
        let base = c.g[V1][0] as u32;
        let value = c.g[A1][0];
        Addr(at(base, 0x4048)).sw(c, value);
        c.g[A0][0] = Addr(at(base, 0x4044)).lw(c);
        call_ptr(c, callee::I_SIGNAL_SEMA, 0x0031_acf4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ModObj_v43(ModObj* self)` (0x356ab8): forward to the modifier
/// block's v43.
pub fn mod_obj_v43(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 0x1c)).lw(c);
        let block = c.g[A0][0];
        frame.restore_ra(c, 0);
        if block != 0 {
            c.cycles += 3;
            callee::MODIFIER_BLOCK_FORWARD_V43.call(c, 0x0035_6ad4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void ModObj_v42(ModObj* self)` (0x356ae0): forward to v42.
pub fn mod_obj_v42(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 0x1c)).lw(c);
        let block = c.g[A0][0];
        frame.restore_ra(c, 0);
        if block != 0 {
            c.cycles += 3;
            callee::MODIFIER_BLOCK_FORWARD_V42.call(c, 0x0035_6afc)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void ModObj_v36(ModObj* self)` (0x356b08): forward an event to the
/// modifier block's event handler.
pub fn mod_obj_v36(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 0x1c)).lw(c);
        let block = c.g[A0][0];
        frame.restore_ra(c, 0);
        if block != 0 {
            c.cycles += 3;
            callee::MODIFIER_BLOCK_HANDLE_EVENT.call(c, 0x0035_6b24)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void GraphicsMan_SnapshotTextureTable(GraphicsMan* self)` (0x369ff8):
/// call slot 0xf2 of the current texture table (`+0x10d8`).
pub fn graphics_man_snapshot_texture_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(abi::arg(c, 0), 0x10d8)).lw(c);
        let table = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(at(table, 0x3c8)).lh(c);
        c.g[A1][0] = Addr(at(table, 0x3cc)).lw(c);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0036_a014)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void GraphicsMan_FreeAllTextures(GraphicsMan* self)` (0x36a020): call
/// slot 0xf1 of the texture table.
pub fn graphics_man_free_all_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(abi::arg(c, 0), 0x10d8)).lw(c);
        let table = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(at(table, 0x3c0)).lh(c);
        c.g[A1][0] = Addr(at(table, 0x3c4)).lw(c);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0036_a03c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void PS2BezierMan_CallV17(PS2BezierMan* self)` (0x38d638): call slot 0x11
/// (`+0x88` delta, `+0x8c` method) of the object at `self->+0x4`.
pub fn ps2_bezier_man_call_v17(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(abi::arg(c, 0), 4)).lw(c);
        let obj = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(at(obj, 0x88)).lh(c);
        c.g[A1][0] = Addr(at(obj, 0x8c)).lw(c);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0038_d654)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void GridMeshBase_CallV22(GridMeshBase* self, ...)` (0x394b98): call slot
/// 0x16 of the mesh object at `+0x4` with the remaining arguments.
pub fn grid_mesh_base_call_v22(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[T2][0] = Addr(at(abi::arg(c, 0), 4)).lw(c);
        let obj = c.g[T2][0] as u32;
        c.g[V1][0] = Addr(at(obj, 0xb0)).lh(c);
        c.g[V0][0] = Addr(at(obj, 0xb4)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0039_4bb4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void LightMan_FreeInstances(LightMan* self)` (0x394ce8): free the
/// instance array at `+0x4`, if any.
pub fn light_man_free_instances(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 4)).lw(c);
        let list = c.g[A0][0];
        frame.restore_ra(c, 0);
        if list != 0 {
            c.cycles += 3;
            call_ptr(c, callee::EA_MEM_FREE, 0x0039_4d04)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void cWScriptCache_SetData(cWScriptCache* self, int i, int value)`
/// (0x3aca10): write item `i` of the cache (`+0x4` base, 12 bytes each).
pub fn cwscript_cache_set_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xc;
        frame.save_ra(c, 0);
        c.mult(0, V0, A1, V0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 4)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        callee::WSCRIPT_CACHE_SET_DATA.call(c, 0x003a_ca2c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SHP_Free(char* buf)` (0x3b2b68): NUL-terminate the buffer and hand
/// it to the shared-header free routine (`D_0044C458`, newlib `free`).
pub fn shp_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        Addr(abi::arg(c, 0)).sb(c, 0);
        c.g[V1][0] = 0x0045_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(0x0044_c458).lw(c);
        let target = c.g[V0][0] as u32;
        frame.save_ra(c, 0);
        call_ptr(c, target, 0x003b_2b84)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Rider, AI, flags, BE singletons.

/// `void ComputerActorRider_ResetTrickCounts(Rider* self)` (0x10d170): zero
/// the fifteen trick-count triplets at `+0xef4-0x78`, `+0xef4-0x3c` and
/// `+0xef4`.
pub fn computer_actor_rider_reset_trick_counts(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0xef4));
    c.g[V0][0] = 0xe;
    loop {
        c.cycles += 7;
        let a = c.g[A0][0] as u32;
        Addr(a.wrapping_sub(0x78)).sw(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        Addr(a.wrapping_sub(0x3c)).sw(c, 0);
        Addr(a).sw(c, 0);
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[A0][0] = sx(a.wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0010_d178);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void RiderMetrix_SetPressSpin(RiderMetrix* self, int on)` (0x119ad8):
/// record the press flag (+0x28) and add pi to the spin amount (+0x34).
pub fn rider_metrix_set_press_spin(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let base = c.g[V0][0] as u32;
        c.lwc1(0, glo(c, -0x7a0c));
        c.lwc1(12, at(base, 0x34));
        let on = c.g[A1][0];
        Addr(at(base, 0x28)).sw(c, on);
        let sum = crate::ps2f::FpuF32(c.f[12]) + crate::ps2f::FpuF32(c.f[0]);
        c.f[12] = sum.0;
        callee::RIDER_METRIX_SET_SPIN_AMOUNT.call(c, 0x0011_9af8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void AI_SetSyncWords(AI* self, int* src)` (0x12bac0): copy two words into
/// the AI's sync block at `+0x20`.
pub fn ai_set_sync_words(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0x20));
    c.g[V1][0] = 1;
    loop {
        c.cycles += 7;
        let src = c.g[A1][0] as u32;
        c.g[V0][0] = Addr(src).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[A1][0] = sx(src.wrapping_add(4));
        let dst = c.g[A0][0] as u32;
        let v = c.g[V0][0];
        Addr(dst).sw(c, v);
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[A0][0] = sx(dst.wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0012_bac8);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void AI_GetSyncWords(AI* self, int* dst)` (0x12baf0): copy the AI's sync
/// block at `+0x20` into `dst`.
pub fn ai_get_sync_words(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0x20));
    c.g[V1][0] = 1;
    loop {
        c.cycles += 7;
        let src = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(src).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[A0][0] = sx(src.wrapping_add(4));
        let dst = c.g[A1][0] as u32;
        let v = c.g[V0][0];
        Addr(dst).sw(c, v);
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[A1][0] = sx(dst.wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0012_baf8);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void Flags64_MoveToSecond(Flags64* f, int bit)` (0x144670): clear `bit` in
/// the first word (moving it into the second word's same bit).
pub fn flags64_move_to_second(c: &mut Cpu) {
    c.cycles += 11;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 1;
    c.g[V1][0] = Addr(self_).ld(c);
    c.g[V0][0] = c.g[V0][0] << (c.g[A1][0] & 63);
    c.g[A1][0] = Addr(at(self_, 8)).ld(c);
    c.g[A2][0] = !c.g[V0][0];
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
    c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
    let first = c.g[V1][0];
    Addr(self_).sd(c, first);
    let second = c.g[A1][0];
    Addr(at(self_, 8)).sd(c, second);
    abi::ret(c);
}

/// `unsigned short BENewRaceInt_GetSpecialEventField4(self, int event,
/// int index)` (0x1455d0): field 4 (u16) of `D_00440D18[event - 6][index]`
/// (a 3-wide, 20-byte-stride table).
pub fn benew_race_int_get_special_event_field4(c: &mut Cpu) {
    c.cycles += 11;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(6));
    c.g[A0][0] = 0x14;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[V1][0] = 0x0044_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = sx(0x0044_0000u32.wrapping_add(0xd18));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.mult(0, A2, A2, A0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    let entry = c.g[V1][0] as u32;
    c.g[V0][0] = Addr(at(entry, 4)).lhu(c);
    abi::ret(c);
}

/// `unsigned short BENewRaceInt_GetSpecialEventField6(self, int event,
/// int index)` (0x145600): field 6 (u16) of the same table entry.
pub fn benew_race_int_get_special_event_field6(c: &mut Cpu) {
    c.cycles += 11;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(6));
    c.g[A0][0] = 0x14;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[V1][0] = 0x0044_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = sx(0x0044_0000u32.wrapping_add(0xd18));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.mult(0, A2, A2, A0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    let entry = c.g[V1][0] as u32;
    c.g[V0][0] = Addr(at(entry, 6)).lhu(c);
    abi::ret(c);
}

/// `void BEOptionInt_ResetSingleton(void)` (0x14f648): reset the option
/// singleton's current page (`D_004A120C -> +0xc` object, slot at `+8`).
pub fn beoption_int_reset_singleton(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = Addr(glo(c, g::BE_OPTION_SINGLETON)).lw(c);
        frame.save_ra(c, 0);
        let opt = c.g[A1][0] as u32;
        c.g[V0][0] = Addr(at(opt, 0xc)).lw(c);
        let page = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(at(page, 8)).lh(c);
        c.g[V1][0] = Addr(at(page, 0xc)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(opt.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0014_f668)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int BESaveInt_GetSaveBufferEnd(void)` (0x1526d0): the save buffer base
/// (`D_004A1224`) plus the library's save-data size.
pub fn besave_int_get_save_buffer_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x0015_26e0)?;
        c.g[A0][0] = c.g[V0][0];
        callee::BE_LIBRARY_GET_SAVE_DATA_SIZE.call(c, 0x0015_26e8)?;
        c.g[V1][0] = Addr(glo(c, g::SAVE_BUFFER)).lw(c);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.close(c);
        Ok(())
    });
}

/// `void BESaveInt_ReleaseSaveBuffer(void)` (0x152728): free the save buffer
/// (`D_004A1224`) and clear the buffer and its end.
pub fn besave_int_release_save_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(glo(c, g::SAVE_BUFFER)).lw(c);
        let buf = c.g[A0][0];
        frame.save_ra(c, 0);
        if buf != 0 {
            c.cycles += 2;
            call_ptr(c, callee::EA_MEM_FREE, 0x0015_2740)?;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        Addr(glo(c, g::SAVE_BUFFER)).sw(c, 0);
        Addr(glo(c, g::SAVE_BUFFER_END)).sw(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Camera.

/// `void ChaseCameraAlgorithm_ClearHeadingRing(CameraAlgo* self)` (0x167db8):
/// zero the five heading samples at `+0x00`, `+0x14` (as floats) and the
/// five `+0x28` words, then the cursor at `+0x3c`.
pub fn chase_camera_algorithm_clear_heading_ring(c: &mut Cpu) {
    c.cycles += 2;
    Addr(at(abi::arg(c, 0), 0x3c)).sw(c, 0);
    c.g[V0][0] = 4;
    loop {
        c.cycles += 7;
        let a = c.g[A0][0] as u32;
        c.swc1(12, a);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.swc1(12, at(a, 0x14));
        Addr(at(a, 0x28)).sw(c, 0);
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[A0][0] = sx(a.wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0016_7dc0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// One of the scripted camera algorithms' `Reset` (0x169540, 0x169d80,
/// 0x16a4e0): call the algorithm's `v03` (slot at `+0x18`, method at `+0x1c`
/// of the vtable at `self->+0x10`) with `a1 = 0`.
fn script_camera_algo_reset(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(abi::arg(c, 0), 0x10)).lw(c);
        let vt = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(at(vt, 0x18)).lh(c);
        c.g[A2][0] = Addr(at(vt, 0x1c)).lw(c);
        let target = c.g[A2][0] as u32;
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ManualScriptCameraAlgo_Reset(ManualScriptCameraAlgo* self)`
/// (0x169540).
pub fn manual_script_camera_algo_reset(c: &mut Cpu) {
    script_camera_algo_reset(c, 0x0016_9560);
}

/// `void TargetScriptCameraAlgo_Reset(TargetScriptCameraAlgo* self)`
/// (0x169d80).
pub fn target_script_camera_algo_reset(c: &mut Cpu) {
    script_camera_algo_reset(c, 0x0016_9da0);
}

/// `void SubjectScriptCameraAlgo_Reset(SubjectScriptCameraAlgo* self)`
/// (0x16a4e0).
pub fn subject_script_camera_algo_reset(c: &mut Cpu) {
    script_camera_algo_reset(c, 0x0016_a500);
}

// ---------------------------------------------------------------------------
// Frontend states.

/// `void cFEStateCharSelect_Update(cFEStateCharSelect* self)` (0x181420): the
/// rider-preview update, then the base state's update.
pub fn cfe_state_char_select_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CHAR_SELECT_UPDATE_RIDER_PREVIEW.call(c, 0x0018_1434)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::UI_STATE_V13, 0x0018_143c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateCharSetup_Update(cFEStateCharSetup* self)` (0x182c38): the
/// base state's update, then the setup state's rider-preview update.
pub fn cfe_state_char_setup_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_V13, 0x0018_2c4c)?;
        c.g[A0][0] = c.g[S0][0];
        callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW.call(c, 0x0018_2c54)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateRiderDetail_Update(cFEStateRiderDetail* self)` (0x1835c8):
/// same shape as the character-setup state.
pub fn cfe_state_rider_detail_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_V13, 0x0018_35dc)?;
        c.g[A0][0] = c.g[S0][0];
        callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW.call(c, 0x0018_35e4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEOptionsCredits_Update(cFEOptionsCredits* self)` (0x1863b8):
/// advance the credits scroll (`+0x60 += +0x64`), then the base update.
pub fn cfe_options_credits_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        let self_ = abi::arg(c, 0);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at(self_, 0x60)).lw(c);
        c.g[A1][0] = Addr(at(self_, 0x64)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
        let y = c.g[V1][0];
        Addr(at(self_, 0x60)).sw(c, y);
        call_ptr(c, callee::UI_STATE_V13, 0x0018_63d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
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

// ---------------------------------------------------------------------------
// Bolt file cache and rider previews.

/// `BoltFile_Cache* BoltFile_CacheCtor(BoltFile_Cache* self)` (0x19cdf0):
/// publish the cache singleton (`gp-0x1784`), initialise it and return it.
pub fn bolt_file_cache_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let self_ = c.g[S0][0];
        Addr(glo(c, g::BOLT_FILE_CACHE)).sw(c, self_);
        callee::BOLT_FILE_CACHE_INIT.call(c, 0x0019_ce08)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void FEModule_RiderPreviewSetTransform(FEModule* self, sQuadRM* src)`
/// (0x19f3e8): copy the two quads into `+0xc30`/`+0xc40` and apply them.
pub fn femodule_rider_preview_set_transform(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.lq(3, abi::arg(c, 1));
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A0][0];
        let dst = c.g[A2][0] as u32;
        c.sq(3, at(dst, 0xc30));
        c.lq(2, at(abi::arg(c, 1), 0x10));
        c.sq(2, at(dst, 0xc40));
        callee::RIDER_PREVIEW_APPLY_TRANSFORM.call(c, 0x0019_f408)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void FEModule_RiderPreviewSetTransform2(FEModule* self, sQuadRM* src)`
/// (0x19f548): the second pair of quads (`+0xc50`/`+0xc60`).
pub fn femodule_rider_preview_set_transform2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.lq(3, abi::arg(c, 1));
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A0][0];
        let dst = c.g[A2][0] as u32;
        c.sq(3, at(dst, 0xc50));
        c.lq(2, at(abi::arg(c, 1), 0x10));
        c.sq(2, at(dst, 0xc60));
        callee::RIDER_PREVIEW_APPLY_TRANSFORM.call(c, 0x0019_f568)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void FEModule_SetRiderPreviewIndices(FEModule* self)` (0x1a0498): number
/// the two rider preview slots (`FEModule` stride 0xce0, index at `+0x4`).
pub fn femodule_set_rider_preview_indices(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V1][0] = 0;
    loop {
        c.cycles += 7;
        let a = c.g[A0][0] as u32;
        let v = c.g[V1][0];
        Addr(at(a, 4)).sw(c, v);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx(a.wrapping_add(0xce0));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x001a_04a0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};
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

    /// Where test objects live (all within [`DATA`]).
    const OBJ: u32 = DATA;
    /// A vtable the object points at.
    const VT: u32 = DATA + 0x8000;
    /// The song instance / process / block a wrapper loads from its object.
    const CHILD: u32 = DATA + 0x1000;
    /// Scratch for the callees' own data.
    const SONG: u32 = DATA + 0x2000;

    /// A `{ short delta; short; void (*fn)(); }` slot pointing at [`FAKE`].
    fn slot(c: &mut Cpu, vt: u32, off: u32) {
        c.bus.write16(vt + off, 0);
        c.bus.write32(vt + off + 4, FAKE);
    }

    /// The behaviour library singleton (a zeroed dummy, so
    /// `BELibrary_GetInstance` neither constructs one nor reads the disc) and
    /// a world whose race manager holds one rider (rider loops in the sound
    /// and UI code would otherwise walk a garbage count and run away).
    fn globals(c: &mut Cpu) {
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
    fn clear(c: &mut Cpu, base: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(base + 4 * i, 0);
        }
    }

    // -----------------------------------------------------------------
    // Music player.

    #[test]
    fn music_player_pause_matches() {
        check(s08::f_002b3a70__generated, music_player_pause, 150, &|c, seed| {
            machine(c, seed, 0x2b3a70, 1);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            clear(c, SONG, 0x100);
            c.g[A0][0] = sx(DATA + 0x400);
            // No song / a song with no stream: both early-outs of the callee.
            let song = if seed % 3 == 0 { 0 } else { SONG };
            c.bus.write32(DATA + 0x400 + 0x408, song);
        });
    }

    #[test]
    fn music_player_resume_matches() {
        check(s08::f_002b3a98__generated, music_player_resume, 150, &|c, seed| {
            machine(c, seed, 0x2b3a98, 1);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            clear(c, SONG, 0x100);
            c.g[A0][0] = sx(DATA + 0x400);
            let song = if seed % 3 == 0 { 0 } else { SONG };
            c.bus.write32(DATA + 0x400 + 0x408, song);
        });
    }

    #[test]
    fn music_player_get_song_tag_matches() {
        check(s08::f_002b49e0__generated, music_player_get_song_tag, 200, &|c, seed| {
            let mut r = Rng(0x49e0_1234 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(OBJ);
            clear(c, OBJ, 0x40);
            let tag_flag = if seed % 3 == 0 { 0 } else { 1 };
            let song = if seed % 5 == 0 { 0 } else { CHILD };
            c.bus.write32(OBJ + 0x418, tag_flag);
            c.bus.write32(OBJ + 0x408, song);
            c.bus.write32(CHILD + 0x58, r.next_u32());
        });
    }

    // -----------------------------------------------------------------
    // World triggers.

    #[test]
    fn world_trigger_manager_destroy_instance_matches() {
        check(
            s08::f_002b4b48__generated,
            world_trigger_manager_destroy_instance,
            120,
            &|c, seed| {
                machine(c, seed, 0x2b4b48, 1);
                globals(c);
                c.g[A0][0] = sx(OBJ);
                clear(c, CHILD, 0x40);
                // A live singleton (destroyed, then cleared) or none.
                let mgr = if seed % 3 == 0 { 0 } else { CHILD };
                c.bus.write32(GAME_GP.wrapping_add(0x21e4), mgr);
            },
        );
    }

    // -----------------------------------------------------------------
    // MemCard and menus.

    #[test]
    fn cps2_memcard_get_dir_entry_name_matches() {
        check(s08::f_002c68c8__generated, cps2_memcard_get_dir_entry_name, 150, &|c, seed| {
            machine(c, seed, 0x2c68c8, 1);
            c.g[A0][0] = sx(OBJ);
            c.g[A1][0] = sx(seed % 8);
            c.g[A2][0] = sx(DATA + 0x4000);
            clear(c, OBJ, 0x200);
            clear(c, DATA + 0x4000, 0x40);
            // A short NUL-terminated name in each of the eight entries.
            for i in 0..8u32 {
                let name = b"mc0:dir\0";
                for (k, b) in name.iter().enumerate() {
                    c.bus.write8(OBJ + (i << 6) + 0x4e0 + k as u32, *b);
                }
            }
            // Scrambled destinations are fine, but keep strcpy on RAM: the
            // copy length comes from the source only.
            c.bus.write8(DATA + 0x4000 + 0x3f, 0);
        });
    }

    #[test]
    fn menu_sys_page_get_selected_index_matches() {
        check(s09::f_002cbf08__generated, menu_sys_page_get_selected_index, 150, &|c, seed| {
            machine(c, seed, 0x2cbf08, 1);
            c.g[A0][0] = sx(OBJ);
            clear(c, OBJ, 0x40);
            clear(c, CHILD, 0x40);
            // Selection state: no items selected / one item at +0x4.
            c.bus.write32(OBJ + 4, seed as u32 % 4);
            // MenuSysPage_ValidateSelection walks the page's item list at
            // +0xc; a null list is the "nothing to validate" path.
            c.bus.write32(OBJ + 0xc, 0);
        });
    }

    #[test]
    fn menu_sys_point2d_item_on_focus_matches() {
        check(s09::f_002d08e0__generated, menu_sys_point2d_item_on_focus, 150, &|c, seed| {
            machine(c, seed, 0x2d08e0, 1);
            c.g[A0][0] = sx(OBJ);
            clear(c, OBJ, 0x40);
            c.bus.write32(OBJ + 0x40, seed as u32 & 0xff);
        });
    }

    // -----------------------------------------------------------------
    // Table and vtable dispatchers.

    #[test]
    fn mod_obj_call_slot7_matches() {
        covered(s09::f_002d1b30__generated, mod_obj_call_slot7, 120, &|c, seed| {
            machine(c, seed, 0x2d1b30, 1);
            clear(c, OBJ, 0x20);
            clear(c, VT, 0x40);
            // `ModObj::v07`: the vtable lives at the object's first word.
            slot(c, VT, 0x38);
            c.bus.write32(OBJ, VT);
            c.g[A0][0] = sx(OBJ);
            c.g[A1][0] = sx(seed as u32);
            c.g[A2][0] = sx(seed as u32);
            c.g[A3][0] = sx(seed as u32);
        });
    }

    #[test]
    fn wscript_man_has_kill_script_matches() {
        check(s10::f_0030a598__generated, wscript_man_has_kill_script, 120, &|c, seed| {
            machine(c, seed, 0x30a598, 1);
            globals(c);
            c.g[A0][0] = sx(OBJ);
            c.g[A1][0] = sx(CHILD + 0x80);
            clear(c, OBJ, 0x80);
            clear(c, CHILD, 0x80);
            // The script cache object (`+0x28c`) is null: the lookup falls
            // through to the id-only path.
            c.bus.write32(OBJ + 0x28c, 0);
            let _ = seed;
        });
    }

    #[test]
    fn wscript_man_jump_to_task_matches() {
        check(s10::f_0030a6d8__generated, wscript_man_jump_to_task, 120, &|c, seed| {
            machine(c, seed, 0x30a6d8, 1);
            c.g[A1][0] = 0;
            clear(c, OBJ, 0x40);
            clear(c, CHILD, 0x100);
            c.g[A0][0] = sx(OBJ);
            // No process / a process with no commands (the callee loops over
            // the command count at +0x4).
            let proc = if seed % 3 == 0 { 0 } else { CHILD };
            c.bus.write32(OBJ + 0x2a4, proc);
        });
    }

    #[test]
    fn wscript_man_goto_task_matches() {
        check(s10::f_0030a700__generated, wscript_man_goto_task, 120, &|c, seed| {
            machine(c, seed, 0x30a700, 1);
            c.g[A1][0] = 0;
            clear(c, OBJ, 0x40);
            clear(c, CHILD, 0x100);
            c.g[A0][0] = sx(OBJ);
            let proc = if seed % 3 == 0 { 0 } else { CHILD };
            c.bus.write32(OBJ + 0x2a4, proc);
        });
    }

    #[test]
    fn wscript_man_cmd_queue_ctor_matches() {
        check(s10::f_0030c738__generated, wscript_man_cmd_queue_ctor, 120, &|c, seed| {
            machine(c, seed, 0x30c738, 1);
            clear(c, OBJ, 0x40);
            c.g[A0][0] = sx(OBJ);
            c.bus.write32(OBJ, seed as u32 & 0xff);
        });
    }

    #[test]
    fn ps2_exec_man_v05_matches() {
        check(s10::f_0031aac8__generated, ps2_exec_man_v05, 120, &|c, seed| {
            machine(c, seed, 0x31aac8, 1);
            clear(c, OBJ, 0x40);
            c.g[A0][0] = sx(OBJ);
            // A held semaphore / none.
            c.bus.write32(OBJ + 0x1c, if seed % 3 == 0 { 0 } else { 1 });
            c.bus.write32(OBJ + 0x18, seed as u32 & 3);
        });
    }

    #[test]
    fn ps2_exec_man_v09_matches() {
        check(s10::f_0031acd8__generated, ps2_exec_man_v09, 120, &|c, seed| {
            machine(c, seed, 0x31acd8, 1);
            clear(c, OBJ + 0x1_4000, 0x20);
            c.g[A0][0] = sx(OBJ);
            c.g[A1][0] = sx(seed as u32 & 0xff);
            c.bus.write32(OBJ + 0x1_4044, seed as u32 & 3);
        });
    }

    #[test]
    fn mod_obj_v43_matches() {
        check(s11::f_00356ab8__generated, mod_obj_v43, 120, &|c, seed| {
            machine(c, seed, 0x356ab8, 1);
            clear(c, OBJ, 0x40);
            clear(c, CHILD, 0x100);
            c.g[A0][0] = sx(OBJ);
            let block = if seed % 3 == 0 { 0 } else { CHILD };
            c.bus.write32(OBJ + 0x1c, block);
        });
    }

    #[test]
    fn mod_obj_v42_matches() {
        check(s11::f_00356ae0__generated, mod_obj_v42, 120, &|c, seed| {
            machine(c, seed, 0x356ae0, 1);
            clear(c, OBJ, 0x40);
            clear(c, CHILD, 0x100);
            c.g[A0][0] = sx(OBJ);
            let block = if seed % 3 == 0 { 0 } else { CHILD };
            c.bus.write32(OBJ + 0x1c, block);
        });
    }

    #[test]
    fn mod_obj_v36_matches() {
        check(s11::f_00356b08__generated, mod_obj_v36, 120, &|c, seed| {
            machine(c, seed, 0x356b08, 1);
            c.g[A1][0] = 0;
            clear(c, OBJ, 0x40);
            clear(c, CHILD, 0x100);
            c.g[A0][0] = sx(OBJ);
            let block = if seed % 3 == 0 { 0 } else { CHILD };
            c.bus.write32(OBJ + 0x1c, block);
        });
    }

    #[test]
    fn graphics_man_snapshot_texture_table_matches() {
        covered(s12::f_00369ff8__generated, graphics_man_snapshot_texture_table, 120, &|c, seed| {
            machine(c, seed, 0x369ff8, 1);
            clear(c, OBJ, 0x40);
            clear(c, VT, 0x40);
            slot(c, VT, 0x3c8);
            c.bus.write32(OBJ + 0x10d8, VT);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }

    #[test]
    fn graphics_man_free_all_textures_matches() {
        covered(s12::f_0036a020__generated, graphics_man_free_all_textures, 120, &|c, seed| {
            machine(c, seed, 0x36a020, 1);
            clear(c, OBJ, 0x40);
            clear(c, VT, 0x40);
            slot(c, VT, 0x3c0);
            c.bus.write32(OBJ + 0x10d8, VT);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }

    #[test]
    fn ps2_bezier_man_call_v17_matches() {
        covered(s12::f_0038d638__generated, ps2_bezier_man_call_v17, 120, &|c, seed| {
            machine(c, seed, 0x38d638, 1);
            clear(c, OBJ, 0x40);
            clear(c, VT, 0x200);
            slot(c, VT, 0x88);
            c.bus.write32(OBJ + 4, VT);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }

    #[test]
    fn grid_mesh_base_call_v22_matches() {
        covered(s13::f_00394b98__generated, grid_mesh_base_call_v22, 120, &|c, seed| {
            machine(c, seed, 0x394b98, 1);
            clear(c, OBJ, 0x40);
            clear(c, VT, 0x200);
            slot(c, VT, 0xb0);
            c.bus.write32(OBJ + 4, VT);
            c.g[A0][0] = sx(OBJ);
            c.g[A1][0] = sx(seed as u32);
            c.g[A2][0] = sx(seed.wrapping_mul(3));
            c.g[A3][0] = 0;
        });
    }

    #[test]
    fn light_man_free_instances_matches() {
        check(s13::f_00394ce8__generated, light_man_free_instances, 120, &|c, seed| {
            machine(c, seed, 0x394ce8, 1);
            clear(c, OBJ, 0x40);
            c.g[A0][0] = sx(OBJ);
            let list = if seed % 3 == 0 { 0 } else { CHILD };
            c.bus.write32(OBJ + 4, list);
        });
    }

    #[test]
    fn cwscript_cache_set_data_matches() {
        check_lohi(s13::f_003aca10__generated, cwscript_cache_set_data, 150, &|c, seed| {
            machine(c, seed, 0x3aca10, 1);
            clear(c, OBJ, 0x40);
            clear(c, CHILD, 0x80);
            c.g[A0][0] = sx(OBJ);
            c.g[A1][0] = sx(seed % 5);
            c.g[A2][0] = sx(seed as u32 ^ 0x5a5a);
            c.bus.write32(OBJ + 4, CHILD);
        });
    }

    #[test]
    fn shp_free_matches() {
        check(s13::f_003b2b68__generated, shp_free, 120, &|c, seed| {
            machine(c, seed, 0x3b2b68, 1);
            clear(c, OBJ, 0x80);
            // `D_0044C458` is newlib's `free` hook; the null slot is faked.
            c.bus.write32(0x0044_c458, 0);
            c.g[A0][0] = sx(OBJ + (seed as u32 & 0x3c));
        });
    }

    // -----------------------------------------------------------------
    // Rider, AI, flags, BE singletons.

    #[test]
    fn computer_actor_rider_reset_trick_counts_matches() {
        check(
            s00::f_0010d170__generated,
            computer_actor_rider_reset_trick_counts,
            200,
            &|c, seed| {
                let mut r = Rng(0x10d1_70aa ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OBJ);
                clear(c, OBJ, 0x1200);
                // Fifteen triplets of live counters.
                for i in 0..15u32 {
                    let base = OBJ + 0xef4 + 4 * i;
                    c.bus.write32(base - 0x78, r.next_u32());
                    c.bus.write32(base - 0x3c, r.next_u32());
                    c.bus.write32(base, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn rider_metrix_set_press_spin_matches() {
        check(s00::f_00119ad8__generated, rider_metrix_set_press_spin, 150, &|c, seed| {
            machine(c, seed, 0x119ad8, 1);
            clear(c, OBJ, 0x80);
            c.g[A0][0] = sx(OBJ);
            c.g[A1][0] = sx(seed as u32 & 1);
            // The pi constant lives at `gp-0x7a0c` in `.lit4`; both bodies load
            // it from there, so a zeroed page is fine (and deterministic).
            c.bus.write32(GAME_GP.wrapping_sub(0x7a0c), 0x4049_0fdb);
            c.bus.write32(OBJ + 0x34, seed.wrapping_mul(0x3dcc_cccd));
            c.bus.write32(OBJ + 0x14, 0);
        });
    }

    #[test]
    fn ai_set_sync_words_matches() {
        check(s00::f_0012bac0__generated, ai_set_sync_words, 200, &|c, seed| {
            let mut r = Rng(0x12ba_c0aa ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(OBJ);
            c.g[A1][0] = sx(CHILD);
            clear(c, OBJ, 0x40);
            c.bus.write32(CHILD, r.next_u32());
            c.bus.write32(CHILD + 4, r.next_u32());
        });
    }

    #[test]
    fn ai_get_sync_words_matches() {
        check(s00::f_0012baf0__generated, ai_get_sync_words, 200, &|c, seed| {
            let mut r = Rng(0x12ba_f0aa ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(OBJ);
            c.g[A1][0] = sx(CHILD);
            clear(c, OBJ, 0x40);
            clear(c, CHILD, 0x40);
            c.bus.write32(OBJ + 0x20, r.next_u32());
            c.bus.write32(OBJ + 0x24, r.next_u32());
        });
    }

    #[test]
    fn flags64_move_to_second_matches() {
        check(s01::f_00144670__generated, flags64_move_to_second, 200, &|c, seed| {
            let mut r = Rng(0x1446_70aa ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(OBJ);
            c.g[A1][0] = sx(seed);
            c.bus.write64(OBJ, r.next_u32() as u64 | (r.next_u32() as u64) << 32);
            c.bus.write64(OBJ + 8, r.next_u32() as u64 | (r.next_u32() as u64) << 32);
        });
    }

    #[test]
    fn benew_race_int_get_special_event_field4_matches() {
        check_lohi(s01::f_001455d0__generated, benew_race_int_get_special_event_field4, 200, &|c, seed| {
            let mut r = Rng(0x1455_d0aa ^ seed as u64);
            scramble(c, &mut r);
            let event = 6 + (seed % 4) as i32;
            let index = (seed % 3) as i32;
            c.g[A1][0] = sx(event as u32);
            c.g[A2][0] = sx(index as u32);
            // 12 table entries (4 events x 3 indices) of 20 bytes at 0x440d18.
            let base = 0x0044_0d18u32;
            for i in 0..36u32 {
                c.bus.write32(base + 0x14 * i, r.next_u32());
                c.bus.write16(base + 0x14 * i + 4, r.next_u32() as u16);
                c.bus.write16(base + 0x14 * i + 6, r.next_u32() as u16);
            }
        });
    }

    #[test]
    fn benew_race_int_get_special_event_field6_matches() {
        check_lohi(s01::f_00145600__generated, benew_race_int_get_special_event_field6, 200, &|c, seed| {
            let mut r = Rng(0x1456_00aa ^ seed as u64);
            scramble(c, &mut r);
            let event = 6 + (seed % 4) as i32;
            let index = (seed % 3) as i32;
            c.g[A1][0] = sx(event as u32);
            c.g[A2][0] = sx(index as u32);
            let base = 0x0044_0d18u32;
            for i in 0..36u32 {
                c.bus.write32(base + 0x14 * i, r.next_u32());
                c.bus.write16(base + 0x14 * i + 4, r.next_u32() as u16);
                c.bus.write16(base + 0x14 * i + 6, r.next_u32() as u16);
            }
        });
    }

    #[test]
    fn beoption_int_reset_singleton_matches() {
        covered(s01::f_0014f648__generated, beoption_int_reset_singleton, 120, &|c, seed| {
            machine(c, seed, 0x14f648, 1);
            clear(c, OBJ, 0x40);
            clear(c, VT, 0x40);
            clear(c, CHILD, 0x40);
            slot(c, VT, 0x8);
            // D_004A120C -> current page object (+0xc) -> vtable.
            c.bus.write32(GAME_GP.wrapping_sub(0x1ee4), OBJ);
            c.bus.write32(OBJ + 0xc, CHILD);
            c.bus.write32(CHILD, VT);
            let _ = seed;
        });
    }

    #[test]
    fn besave_int_get_save_buffer_end_matches() {
        check(s01::f_001526d0__generated, besave_int_get_save_buffer_end, 120, &|c, seed| {
            machine(c, seed, 0x1526d0, 2);
            globals(c);
            clear(c, DATA + 0x6_c000, 0x40);
            c.bus.write32(GAME_GP.wrapping_sub(0x1ecc), 0x0080_2000u32 + 0x40 * (seed & 7));
            let _ = seed;
        });
    }

    #[test]
    fn besave_int_release_save_buffer_matches() {
        check(s01::f_00152728__generated, besave_int_release_save_buffer, 120, &|c, seed| {
            machine(c, seed, 0x152728, 1);
            clear(c, OBJ, 0x40);
            // A buffer to free, or none.
            let buf = if seed % 3 == 0 { 0 } else { OBJ };
            c.bus.write32(GAME_GP.wrapping_sub(0x1ecc), buf);
            c.bus.write32(GAME_GP.wrapping_sub(0x1ec8), 0x1234);
        });
    }

    // -----------------------------------------------------------------
    // Camera.

    #[test]
    fn chase_camera_algorithm_clear_heading_ring_matches() {
        check(
            s02::f_00167db8__generated,
            chase_camera_algorithm_clear_heading_ring,
            200,
            &|c, seed| {
                let mut r = Rng(0x167d_b8aa ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OBJ);
                clear(c, OBJ, 0x60);
                // The five heading samples and their duplicates.
                for i in 0..5u32 {
                    c.bus.write32(OBJ + 4 * i, r.next_u32());
                    c.bus.write32(OBJ + 0x14 + 4 * i, r.next_u32());
                    c.bus.write32(OBJ + 0x28 + 4 * i, r.next_u32());
                }
                c.bus.write32(OBJ + 0x3c, r.next_u32());
            },
        );
    }

    #[test]
    fn manual_script_camera_algo_reset_matches() {
        covered(s02::f_00169540__generated, manual_script_camera_algo_reset, 120, &|c, seed| {
            machine(c, seed, 0x169540, 1);
            clear(c, OBJ, 0x40);
            clear(c, VT, 0x40);
            slot(c, VT, 0x18);
            c.bus.write32(OBJ + 0x10, VT);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }

    #[test]
    fn target_script_camera_algo_reset_matches() {
        covered(s02::f_00169d80__generated, target_script_camera_algo_reset, 120, &|c, seed| {
            machine(c, seed, 0x169d80, 1);
            clear(c, OBJ, 0x40);
            clear(c, VT, 0x40);
            slot(c, VT, 0x18);
            c.bus.write32(OBJ + 0x10, VT);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }

    #[test]
    fn subject_script_camera_algo_reset_matches() {
        covered(s02::f_0016a4e0__generated, subject_script_camera_algo_reset, 120, &|c, seed| {
            machine(c, seed, 0x16a4e0, 1);
            clear(c, OBJ, 0x40);
            clear(c, VT, 0x40);
            slot(c, VT, 0x18);
            c.bus.write32(OBJ + 0x10, VT);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }

    // -----------------------------------------------------------------
    // Frontend states.

    #[test]
    fn cfe_state_char_select_update_matches() {
        check(s02::f_00181420__generated, cfe_state_char_select_update, 120, &|c, seed| {
            machine(c, seed, 0x181420, 2);
            globals(c);
            clear(c, OBJ, 0x100);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }

    #[test]
    fn cfe_state_char_setup_update_matches() {
        check(s02::f_00182c38__generated, cfe_state_char_setup_update, 120, &|c, seed| {
            machine(c, seed, 0x182c38, 2);
            globals(c);
            clear(c, OBJ, 0x100);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }

    #[test]
    fn cfe_state_rider_detail_update_matches() {
        check(s02::f_001835c8__generated, cfe_state_rider_detail_update, 120, &|c, seed| {
            machine(c, seed, 0x1835c8, 2);
            globals(c);
            clear(c, OBJ, 0x100);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }

    #[test]
    fn cfe_options_credits_update_matches() {
        check(s02::f_001863b8__generated, cfe_options_credits_update, 120, &|c, seed| {
            machine(c, seed, 0x1863b8, 1);
            globals(c);
            clear(c, OBJ, 0x100);
            c.g[A0][0] = sx(OBJ);
            c.bus.write32(OBJ + 0x60, 0x1000 + seed as u32);
            c.bus.write32(OBJ + 0x64, seed as u32 & 0x3f);
        });
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

    // -----------------------------------------------------------------
    // Bolt file cache and rider previews.

    #[test]
    fn bolt_file_cache_ctor_matches() {
        check(s03::f_0019cdf0__generated, bolt_file_cache_ctor, 100, &|c, seed| {
            machine(c, seed, 0x19cdf0, 1);
            globals(c);
            clear(c, OBJ, 0x100);
            c.g[A0][0] = sx(OBJ);
            // An existing cache skips the allocation in `BoltFile_CacheInit`;
            // alternate so both paths run (the null path allocates from the
            // zeroed heap, which returns nothing).
            c.bus.write32(GAME_GP.wrapping_sub(0x1788), if seed % 3 == 0 { 0 } else { OBJ });
        });
    }

    #[test]
    fn femodule_rider_preview_set_transform_matches() {
        check(
            s03::f_0019f3e8__generated,
            femodule_rider_preview_set_transform,
            120,
            &|c, seed| {
                machine(c, seed, 0x19f3e8, 1);
                clear(c, OBJ, 0x100);
                clear(c, CHILD, 0x40);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(CHILD);
                for i in 0..16u32 {
                    c.bus.write32(CHILD + 4 * i, seed.wrapping_mul(0x0101_0101) ^ i);
                }
            },
        );
    }

    #[test]
    fn femodule_rider_preview_set_transform2_matches() {
        check(
            s03::f_0019f548__generated,
            femodule_rider_preview_set_transform2,
            120,
            &|c, seed| {
                machine(c, seed, 0x19f548, 1);
                clear(c, OBJ, 0x100);
                clear(c, CHILD, 0x40);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(CHILD);
                for i in 0..16u32 {
                    c.bus.write32(CHILD + 4 * i, seed.wrapping_mul(0x0101_0101) ^ i);
                }
            },
        );
    }

    #[test]
    fn femodule_set_rider_preview_indices_matches() {
        check(s03::f_001a0498__generated, femodule_set_rider_preview_indices, 200, &|c, seed| {
            let mut r = Rng(0x1a04_98aa ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(OBJ);
            clear(c, OBJ, 0x1c00);
            c.bus.write32(OBJ + 4, r.next_u32());
            c.bus.write32(OBJ + 0xce4, r.next_u32());
        });
    }
}
